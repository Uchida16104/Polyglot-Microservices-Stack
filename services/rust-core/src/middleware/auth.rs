use axum::{
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
    body::Body,
};
use uuid::Uuid;

pub async fn internal_auth(
    mut req: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    let user_id_header = req
        .headers()
        .get("x-user-id")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| Uuid::parse_str(s).ok());

    if req.uri().path() == "/health" {
        return Ok(next.run(req).await);
    }

    match user_id_header {
        Some(user_id) => {
            req.extensions_mut().insert(user_id);
            Ok(next.run(req).await)
        }
        None => Err(StatusCode::UNAUTHORIZED),
    }
}
