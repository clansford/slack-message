pub mod response;

use crate::globals::POST_MES_URL;
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
    Client { bearer_token: format!("Bearer {oauth_tok}"), url: POST_MES_URL }
  }

  pub async fn send_message(
    &self, message: &Message<'_>,
  ) -> Result<Response, Box<dyn Error>> {
    let request = self.build_request(message)?;
    assert_eq!(
      request.headers().get("authorization").unwrap(),
      &self.bearer_token,
      "Request authorization header does not match bearer token."
    );
    assert_eq!(
      request.headers().get("content-type").unwrap(),
      "application/json; charset=utf-8",
      "Request header 'content-type' is not 'application/json; charset=utf-8'."
    );
    let response = HttpClient::new().execute(request).await?;
    let res = Response::parse(response).await?;
    Ok(res)
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
  use wiremock::matchers::{body_json, method, path};
  use wiremock::{Mock, MockServer, ResponseTemplate};

  #[test]
  fn new() -> Result<(), Box<dyn Error>> {
    let auth_tok = "testToken";
    let actual = Client::new(auth_tok);
    let expected =
      Client { url: POST_MES_URL, bearer_token: format!("Bearer {auth_tok}") };
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
  async fn send_message_mock() -> Result<(), Box<dyn Error>> {
    let ts = "1734376519.228539";
    let channel = "test-channel";
    let icon_emoji = Some(":test:");
    let text = "testMessageText";
    let username = Some("TEST-USERNAME");
    let route = "/api/chat.postMessage";
    let msg = Message { channel, icon_emoji, text, username };
    let mock_server = setup_mock_server(&msg, route, ts).await?;
    let client = Client {
      bearer_token: String::from("test-token"),
      url: &format!("{}{route}", mock_server.uri()),
    };
    let actual = client.send_message(&msg).await?;
    assert!(actual.ok);
    assert_eq!(channel, actual.channel);
    assert_eq!(text, actual.message.text);
    assert_eq!(ts, actual.ts);
    assert_eq!(username.unwrap(), actual.message.username);
    Ok(())
  }

  #[tokio::test]
  #[ignore = "Actually sends slack message"]
  async fn send_message() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let tok = env::var(ENV_SLACK_TOKEN)?;
    let channel = &env::var(ENV_SLACK_CHANNEL)?;
    let slack = Client::new(&tok);
    let text = "testMessageText";
    let username = Some("TEST-NAME");
    let icon_emoji = Some(":test:");
    let msg = Message { channel, icon_emoji, text, username };
    let actual = slack.send_message(&msg).await?;
    println!("actual:\n{actual:?}");
    assert!(actual.ok);
    assert!(actual.error.is_none());
    assert_eq!(channel, &actual.channel);
    assert_eq!(text, &actual.message.text);
    assert_eq!(username.unwrap(), &actual.message.username);
    assert_eq!(icon_emoji.unwrap(), &actual.message.icons.unwrap().emoji);
    Ok(())
  }

  async fn setup_mock_server(
    msg: &Message<'_>, route: &str, ts: &str,
  ) -> Result<MockServer, Box<dyn Error>> {
    let mock_server = MockServer::start().await;
    let mock_body = format!(
      r#"{{"ok":true,"channel":"{channel}","ts":"{ts}","message":{{"subtype":"bot_message","text":"{text}","username":"{username}","icons":{{"emoji":"{icon}"}},"type":"message","ts":"{ts}","bot_id":"B12345ABCDE","app_id":"A12345ABCDE","blocks":[{{"type":"rich_text","block_id":"Cy=M","elements":[{{"type":"rich_text_section","elements":[{{"type":"text","text":"{text}"}}]}}]}}]}}}}"#,
      channel = msg.channel,
      text = msg.text,
      username = msg.username.unwrap(),
      icon = msg.icon_emoji.unwrap()
    );
    let template =
      ResponseTemplate::new(200).set_body_raw(mock_body, "application/json");
    Mock::given(method("POST"))
      .and(path(route))
      .and(body_json(&msg))
      .respond_with(template)
      .mount(&mock_server)
      .await;

    Ok(mock_server)
  }
}
