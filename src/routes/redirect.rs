use std::str::FromStr;
use serde_json::json;
use worker::*;

pub async fn handle_request(_req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let id = match ctx.param("id") {
        Some(val) => val,
        None => {
            let res = json!({
                        "error": "missing url id"
                    }).to_string();
            return Response::error(res, 400)
        }
    };

    let db = match ctx.kv("db") {
        Ok(val) => val,
        Err(err) => return Response::error(err.to_string(), 500)
    };

    match db.get(id.as_str()).await {
        Ok(val) => {
            match val {
                Some(val) => {
                    Response::redirect(Url::from_str(&val.as_string()).unwrap())
                },
                None => Response::error("url not found".to_string(), 404)
            }
        },
        Err(err) => Response::error(err.to_string(), 500)
    }
}