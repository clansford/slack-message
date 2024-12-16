pub mod response;

use reqwest::{
  header::{AUTHORIZATION, CONTENT_TYPE},
  Client as HttpClient, Request,
};
use response::Response;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Message<'a> {
  pub channel: &'a str,
  pub icon_emoji: Option<&'a str>,
  pub text: &'a str,
  pub username: Option<&'a str>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, PartialOrd)]
pub struct Client<'a> {
  bearer_token: String,
  url: &'a str,
}

impl Client<'_> {
  pub fn new(oauth_tok: &str) -> Self {
    Client {
      bearer_token: format!("Bearer {oauth_tok}"),
      url: "https://slack.com/api/chat.postMessage",
    }
  }

  pub async fn send_message(
    &self, message: &Message<'_>,
  ) -> Result<Response, Box<dyn Error>> {
    let request = self.build_request(message)?;
    let response = HttpClient::new().execute(request).await?;
    let body = response.text().await?;
    let response = Response::parse(&body)?;
    Ok(response)
  }

  fn build_request(
    &self, message: &Message,
  ) -> Result<Request, Box<dyn Error>> {
    let body = serde_json::to_value(message)?;
    let req = HttpClient::new()
      .post(self.url)
      .header(AUTHORIZATION, self.bearer_token.clone())
      .header(CONTENT_TYPE, "application/json; charset=utf-8")
      .json(&body)
      .build()?;
    Ok(req)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::globals::{ENV_SLACK_CHANNEL, ENV_SLACK_TOKEN};
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
      channel: "testChannel",
      icon_emoji: None,
      text: "testMessageText",
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
      channel: "testChannel",
      icon_emoji: None,
      text: "testMessageText",
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
      channel: "testChannel",
      icon_emoji: Some(":test:"),
      text: "testMessageText",
      username: Some("testName"),
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
    let tok = env::var(ENV_SLACK_TOKEN)?;
    let channel = &env::var(ENV_SLACK_CHANNEL)?;
    let slack = Client::new(&tok);
    let msg = Message {
      channel,
      icon_emoji: Some(":test:"),
      text: "testMessageText",
      username: Some("TEST-NAME"),
    };
    slack.send_message(&msg).await?;
    Ok(())
  }
}
