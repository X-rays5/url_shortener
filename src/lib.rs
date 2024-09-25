use worker::*;

mod utils;
mod routes;

fn log_request(req: &Request) {
    console_log!(
        "{} - [{}], located at: {:?}, within: {}",
        Date::now().to_string(),
        req.path(),
        req.cf().coordinates().unwrap_or_default(),
        req.cf().region().unwrap_or("unknown region".into())
    );
}

#[event(fetch)]
pub async fn main(req: Request, env: Env) -> Result<Response> {
    log_request(&req);

    // Optionally, get more helpful error messages written to the console in the case of a panic.
    utils::set_panic_hook();

    let router = Router::new();

    router
        .get_async("/", routes::index::handle_request)
        .on("/health", |_, _| async {
            let response = Response::ok("Health check OK");
            response
        })
        .post_async("/shorten", routes::shorten::handle_request)
        .get_async("/r/:id", routes::redirect::handle_request)
        .run(req, env)
        .await
}
