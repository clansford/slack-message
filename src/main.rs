use clap::Parser;
use core::panic;
use dotenv::dotenv;
use reqwest::{header, Client};
use serde::{Deserialize, Serialize};
use std::env;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  dotenv().ok();
  let args = Cli::parse();
  let bearer_token = args.get_bearer_token()?;
  let slack_message =
    SlackMessage { channel: args.get_channel()?, text: args.message };
  let body = create_request_body(slack_message);
  let resp = Client::new()
    .post("https://slack.com/api/chat.postMessage")
    .header(header::AUTHORIZATION, bearer_token)
    .header(header::CONTENT_TYPE, "application/json; charset=utf-8")
    .json(&body)
    .send()
    .await?;

  let resp_body = resp.text().await?;
  let slack_response = SlackResponse::parse(&resp_body)?;
  if slack_response.ok {
    println!("Message sent successfully");
    Ok(())
  } else {
    panic!("Error: Message not sent successfully");
  }
}

fn create_request_body(slack_message: SlackMessage) -> serde_json::Value {
  let val = serde_json::to_value(slack_message);
  match val {
    Ok(v) => v,
    Err(e) => panic!("{e}"),
  }
}

#[derive(Parser, Debug)]
struct Cli {
  #[arg(short, long)]
  channel: Option<String>,
  message: String,
  #[arg(short, long)]
  auth_token: Option<String>,
}

impl Cli {
  fn get_channel(&self) -> Result<String, Box<dyn Error>> {
    if self.channel.is_some() {
      return Ok(self.channel.clone().unwrap());
    }
    match env::var("SLACK_CHANNEL") {
      Ok(c) => Ok(c),
      Err(e) => {
        eprintln!("Couldn't find SLACK_CHANNEL\n{e:?}");
        Err(e.into())
      }
    }
  }

  fn get_bearer_token(&self) -> Result<String, Box<dyn Error>> {
    let token = if self.auth_token.is_some() {
      self.auth_token.clone().unwrap()
    } else {
      match env::var("SLACK_TOKEN") {
        Ok(tok) => tok,
        Err(e) => {
          eprintln!("Couldn't find SLACK_TOKEN\n{e:?}");
          return Err(e.into());
        }
      }
    };
    Ok(format!("Bearer {token}"))
  }
}

#[derive(Debug, Serialize, Deserialize)]
struct SlackMessage {
  channel: String,
  text: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct SlackResponse {
  ok: bool,
  channel: String,
  ts: String,
  message: Message,
}

impl SlackResponse {
  fn parse(s: &str) -> Result<Self, Box<dyn Error>> {
    match serde_json::from_str::<SlackResponse>(s) {
      Ok(res) => Ok(res),
      Err(e) => {
        eprintln!("Error parsing SlackResponse\n{e:?}");
        Err(e.into())
      }
    }
  }
}

#[derive(Debug, Serialize, Deserialize)]
struct Message {
  user: String,
  #[serde(rename = "type")]
  _type: String,
  ts: String,
  bot_id: String,
  app_id: String,
  text: String,
  team: String,
}
