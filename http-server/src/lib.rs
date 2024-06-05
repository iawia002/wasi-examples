use waki::{handler, Request, Response};

#[handler]
fn hello(req: Request) -> Response {
    let query = req.query();
    Response::new().body(
        format!(
            "Hello, {}!",
            query.get("name").unwrap_or(&"WASI".to_string())
        )
        .as_bytes(),
    )
}
