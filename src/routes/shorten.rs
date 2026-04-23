use rand::distr::{Alphanumeric, SampleString};
use rand::RngExt;
use serde_json::json;
use worker::*;

fn normalize_url(url: &str) -> Option<Url> {
    let trimmed = url.trim();
    if trimmed.is_empty() {
        return None;
    }

    match Url::parse(trimmed) {
        Ok(parsed) => Some(parsed),
        Err(_) => Url::parse(format!("https://{trimmed}").as_str()).ok(),
    }
}

fn random_string() -> String {
    let mut rng = rand::rng();
    let length = rng.random_range(6..=14);
    let mut res = Alphanumeric.sample_string(&mut rng, length);
    res.shrink_to_fit();
    res
}

async fn verify_url(url: String) -> bool {
    console_log!("verifying url: {0}", url);
    let parsed_url = match normalize_url(url.as_str()) {
        Some(parsed) => parsed,
        None => {
            console_log!("invalid url");
            return false;
        }
    };

    match Fetch::Url(parsed_url).send().await {
        Ok(mut val) => {
            match val.bytes().await {
                Ok(val) => val.len() > 0,
                Err(_) => false
            }
        },
        Err(_) => false
    }
}

pub async fn handle_request(mut req: Request, ctx: RouteContext<()>) -> Result<Response> {
    match req.text().await {
        Ok(url) => {
            let normalized_url = match normalize_url(url.as_str()) {
                Some(parsed) => parsed.to_string(),
                None => {
                    let res = json!({
                        "error": "invalid url"
                    })
                    .to_string();
                    return Response::error(res, 400);
                }
            };

            if verify_url(url.clone()).await {
                let db = match ctx.kv("db") {
                  Ok(val) => val,
                  Err(err) => return Response::error(err.to_string(), 500)
                };

                // The chance of a collision is really low, but we'll check anyway
                let mut key = random_string();
                loop {
                    match db.get(&key).text().await {
                        Ok(val) => {
                            match val {
                                Some(_) => {
                                    key = random_string();
                                },
                                None => {
                                    break;
                                }
                            }
                        },
                        Err(_) => {
                            break;
                        }
                    }
                };

                match db.put(key.as_str(), normalized_url)?.execute().await {
                    Ok(_) => {
                        let res = json!({
                            "url": format!("{0}/r/{1}", req.headers().get("host")?.unwrap(), key),
                        }).to_string();
                        Response::ok(res)
                    },
                    Err(err) => {
                        Response::error(err.to_string(), 500)
                    }
                }
            } else {
                let res = json!({
                        "error": "invalid url"
                    }).to_string();
                Response::error(res, 400)
            }
        },
        Err(_) => {
            let res = json!({
                        "error": "missing url"
                    }).to_string();
            Response::error(res, 400)
        }
    }
}
