use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
  pub ok: bool,
  pub channel: String,
  pub ts: String,
  pub message: Message,
}

impl Response {
  pub fn parse(s: &str) -> Result<Self, serde_json::Error> {
    match serde_json::from_str::<Response>(s) {
      Ok(res) => Ok(res),
      Err(e) => {
        eprintln!("Error parsing SlackResponse\n{e:?}");
        Err(e)
      }
    }
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
  pub user: Option<String>,
  #[serde(rename = "type")]
  pub _type: String,
  pub ts: String,
  pub bot_id: String,
  pub app_id: String,
  pub text: String,
  pub team: Option<String>,
}
