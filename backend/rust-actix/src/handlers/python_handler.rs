use actix_web::{web, HttpResponse};
use std::time::Instant;
use crate::config::AppState;
use crate::models::LangResponse;

pub async fn handle(state: web::Data<AppState>) -> HttpResponse {
    let t = Instant::now();
    let url = format!("{}/compute", state.python_url);

    match state.http_client.get(&url).send().await {
        Ok(resp) => match resp.json::<serde_json::Value>().await {
            Ok(json) => {
                let result = json["result"]
                    .as_str()
                    .unwrap_or("(no result field)")
                    .to_owned();
                HttpResponse::Ok().json(LangResponse::ok(
                    "Python3/FastAPI",
                    result,
                    t.elapsed().as_millis(),
                ))
            }
            Err(e) => HttpResponse::Ok()
                .json(LangResponse::err("Python3/FastAPI", format!("JSON error: {e}"))),
        },
        Err(e) => HttpResponse::Ok()
            .json(LangResponse::err("Python3/FastAPI", format!("HTTP error: {e}"))),
    }
}
