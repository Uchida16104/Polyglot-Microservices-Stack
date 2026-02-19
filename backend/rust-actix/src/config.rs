#[derive(Clone)]
pub struct AppState {
    pub python_url: String,
    pub http_client: reqwest::Client,
}
