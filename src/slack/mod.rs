use core::panic;
use reqwest::header;
use reqwest::{Client as HttpClient, RequestBuilder};
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct Client<'a> {
  bearer_token: String,
  url: &'a str,
}

impl Client<'_> {
  pub fn new(oauth_tok: &str) -> Self {
    let url = "https://slack.com/api/chat.postMessage";
    let bearer_token = format!("Bearer {oauth_tok}");
    Client { bearer_token, url }
  }

  pub async fn send_message(
    &self, message: &Message,
  ) -> Result<Response, Box<dyn Error>> {
    let req = self.build_request(message);
    send_request(req).await
  }

  fn build_request(&self, message: &Message) -> RequestBuilder {
    let body = create_request_body(message);
    HttpClient::new()
      .post(self.url)
      .header(header::AUTHORIZATION, self.bearer_token.clone())
      .header(header::CONTENT_TYPE, "application/json; charset=utf-8")
      .json(&body)
  }
}

async fn send_request(r: RequestBuilder) -> Result<Response, Box<dyn Error>> {
  let res = r.send().await?;
  let res_body = res.text().await?;
  Response::parse(&res_body)
}

fn create_request_body(slack_message: &Message) -> serde_json::Value {
  let val = serde_json::to_value(slack_message);
  match val {
    Ok(v) => v,
    Err(e) => {
      eprintln!("Error creating serde value for message.");
      panic!("{e}");
    }
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
  pub channel: String,
  pub text: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
  pub ok: bool,
  pub channel: String,
  pub ts: String,
  pub message: ResponseMessage,
}

impl Response {
  fn parse(s: &str) -> Result<Self, Box<dyn Error>> {
    match serde_json::from_str::<Response>(s) {
      Ok(res) => Ok(res),
      Err(e) => {
        eprintln!("Error parsing SlackResponse\n{e:?}");
        Err(e.into())
      }
    }
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseMessage {
  pub user: String,
  #[serde(rename = "type")]
  pub _type: String,
  pub ts: String,
  pub bot_id: String,
  pub app_id: String,
  pub text: String,
  pub team: String,
}
