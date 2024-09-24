use serde_json::json;
use worker::*;

async fn random_string(length: u64) -> String {
    let mut req = match worker::Fetch::Url(format!("https://randomstring.bleep.workers.dev/?length={0}", length).parse().unwrap()).send().await {
        Ok(val) => val,
        Err(_) => Response::error("", 500).unwrap()
    };

    match req.text().await {
        Ok(val) => val,
        Err(_) => "err".to_string()
    }
}

async fn verify_url(url: String) -> bool {
    match worker::Fetch::Url(url.parse().unwrap_or("")).send().await {
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
            if verify_url(url.clone()).await {
                let db = match ctx.kv("db") {
                  Ok(val) => val,
                  Err(err) => return Response::error(err.to_string(), 500)
                };

                // The chance of a collision is really low, but we'll check anyway
                let mut key = random_string(10).await;
                loop {
                    match db.get(&key).await {
                        Ok(val) => {
                            match val {
                                Some(_) => {
                                    key = random_string(10).await;
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

                match db.put(key.as_str(), url).unwrap().execute().await {
                    Ok(_) => {
                        let res = json!({
                            "url": format!("{0}/r/{1}", req.headers().get("host").unwrap().unwrap(), key),
                        }).to_string();
                        Response::ok(res)
                    },
                    Err(err) => {
                        return Response::error(err.to_string(), 500)
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
