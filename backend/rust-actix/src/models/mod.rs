use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LangResponse {
    pub language:    String,
    pub result:      String,
    pub duration_ms: u128,
    pub status:      String,
}

impl LangResponse {
    pub fn ok(language: &str, result: impl Into<String>, duration_ms: u128) -> Self {
        Self {
            language:    language.to_owned(),
            result:      result.into(),
            duration_ms,
            status:      "ok".to_owned(),
        }
    }

    pub fn err(language: &str, msg: impl Into<String>) -> Self {
        Self {
            language:    language.to_owned(),
            result:      msg.into(),
            duration_ms: 0,
            status:      "error".to_owned(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HealthResponse {
    pub status:    String,
    pub timestamp: String,
    pub version:   String,
    pub langs:     Vec<String>,
}
