use reqwest::Response as HttpResponse;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
  pub ok: bool,
  pub channel: String,
  pub ts: String,
  pub message: Message,
  pub error: Option<String>,
}

impl Response {
  pub fn parse_str(s: &str) -> Result<Self, serde_json::Error> {
    serde_json::from_str(s)
  }
  pub async fn parse(res: HttpResponse) -> Result<Self, Box<dyn Error>> {
    let body = res.text().await?;
    Ok(Self::parse_str(&body)?)
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
  #[serde(rename = "type")]
  pub _type: String,
  pub app_id: String,
  pub bot_id: String,
  pub team: Option<String>,
  pub text: String,
  pub ts: String,
  pub user: Option<String>,
  pub username: String,
  pub icons: Option<Icons>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Icons {
  pub emoji: String,
}
