use worker::*;

const RESPONSE: &str = "All available routes\n
GET /               this page
GET /r/:id          redirects to the url associated with the given id
POST /shorten        returns a shortened url
";

pub async fn handle_request(_req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    Response::ok(RESPONSE)
}