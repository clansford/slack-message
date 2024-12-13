pub mod response;

use core::panic;
use reqwest::header;
use reqwest::{Client as HttpClient, Request};
use response::Response;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Message {
  pub channel: String,
  pub icon_emoji: Option<String>,
  pub text: String,
  pub username: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, PartialOrd)]
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
    let request = self.build_request(message)?;
    let http = reqwest::Client::new();
    let response = http.execute(request).await?;
    let body = response.text().await?;
    Ok(Response::parse(&body)?)
  }

  fn build_request(
    &self, message: &Message,
  ) -> Result<Request, reqwest::Error> {
    let body = create_request_body(message);
    HttpClient::new()
      .post(self.url)
      .header(header::AUTHORIZATION, self.bearer_token.clone())
      .header(header::CONTENT_TYPE, "application/json; charset=utf-8")
      .json(&body)
      .build()
  }
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

#[cfg(test)]
mod tests {
  use super::*;
  use core::str;
  use dotenv::dotenv;
  use std::env;

  #[test]
  fn new() -> Result<(), Box<dyn Error>> {
    let auth_tok = "testToken";
    let actual = Client::new(auth_tok);
    let expected = Client {
      url: "https://slack.com/api/chat.postMessage",
      bearer_token: format!("Bearer {auth_tok}"),
    };
    assert_eq!(
      expected, actual,
      "\n  expected: {expected:#?}\n  actual: {actual:#?}"
    );
    Ok(())
  }

  #[test]
  fn build_request_method() -> Result<(), Box<dyn Error>> {
    let client = Client::new("testToken");
    let msg = Message {
      channel: String::from("testChannel"),
      icon_emoji: None,
      text: String::from("testMessageText"),
      username: None,
    };
    let actual = client.build_request(&msg)?;
    assert_eq!(reqwest::Method::POST, actual.method());
    Ok(())
  }

  #[test]
  fn build_request_headers() -> Result<(), Box<dyn Error>> {
    let tok = "testToken";
    let client = Client::new(tok);
    let msg = Message {
      channel: String::from("testChannel"),
      icon_emoji: None,
      text: String::from("testMessageText"),
      username: None,
    };
    let req = client.build_request(&msg)?;
    let headers = req.headers();
    let auth_header = "authorization";
    let content_header = "content-type";
    assert!(headers.contains_key(auth_header));
    assert!(headers.contains_key(content_header));
    assert_eq!(
      &format!("Bearer {tok}"),
      headers.get(auth_header).unwrap().to_str()?
    );
    assert_eq!(
      "application/json; charset=utf-8",
      headers.get(content_header).unwrap().to_str()?
    );
    Ok(())
  }

  #[test]
  fn build_request_body() -> Result<(), Box<dyn Error>> {
    let client = Client::new("testToken");
    let msg = Message {
      channel: String::from("testChannel"),
      icon_emoji: Some(String::from(":test:")),
      text: String::from("testMessageText"),
      username: Some(String::from("testName")),
    };
    let req = client.build_request(&msg)?;
    let body = req.body().unwrap().as_bytes().unwrap();
    let actual = str::from_utf8(body)?;
    let expected = r#"{"channel":"testChannel","icon_emoji":":test:","text":"testMessageText","username":"testName"}"#;
    assert_eq!(expected, actual, "\nexpected: {expected}\nactual:{actual}");
    Ok(())
  }

  #[tokio::test]
  #[ignore = "Actually sends slack message"]
  async fn send_message() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let tok = env::var("SLACK_MESSAGE_TOKEN")?;
    let channel = env::var("SLACK_MESSAGE_CHANNEL")?;
    let slack = Client::new(&tok);
    let msg = Message {
      channel,
      icon_emoji: Some(String::from(":test:")),
      text: String::from("testMessageText"),
      username: Some(String::from("TEST-NAME")),
    };
    slack.send_message(&msg).await?;
    Ok(())
  }
}
