use clap::Parser;
use core::panic;
use dotenv::dotenv;
use reqwest::header;
use serde::{Deserialize, Serialize};
use std::env;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  dotenv().ok();
  let args = Cli::parse();
  let bearer_token = args.get_oauth_token()?;
  let msg = Message { channel: args.get_channel()?, text: args.message };
  let slack = Client::new(&bearer_token);
  let res = slack.send_message(&msg).await?;
  if res.ok {
    println!("Message sent successfully");
  } else {
    eprintln!("{res:#?}");
    panic!("Error: Message not sent successfully");
  };
  Ok(())
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

  fn get_oauth_token(&self) -> Result<String, Box<dyn Error>> {
    if self.auth_token.is_some() {
      return Ok(self.auth_token.clone().unwrap());
    };
    match env::var("SLACK_TOKEN") {
      Ok(tok) => Ok(tok),
      Err(e) => {
        eprintln!("Couldn't find SLACK_TOKEN\n{e:?}");
        Err(e.into())
      }
    }
  }
}

#[derive(Debug, Serialize, Deserialize)]
struct Client {
  bearer_token: String,
}

impl Client {
  fn new(oauth_tok: &str) -> Self {
    Client { bearer_token: format!("Bearer {oauth_tok}") }
  }

  async fn send_message(
    self, message: &Message,
  ) -> Result<Response, Box<dyn Error>> {
    let body = create_request_body(message);
    let post_message_url = "https://slack.com/api/chat.postMessage";
    let resp = reqwest::Client::new()
      .post(post_message_url)
      .header(header::AUTHORIZATION, self.bearer_token)
      .header(header::CONTENT_TYPE, "application/json; charset=utf-8")
      .json(&body)
      .send()
      .await?;

    let resp_body = resp.text().await?;
    Response::parse(&resp_body)
  }
}

#[derive(Debug, Serialize, Deserialize)]
struct Message {
  channel: String,
  text: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Response {
  ok: bool,
  channel: String,
  ts: String,
  message: ResponseMessage,
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
struct ResponseMessage {
  user: String,
  #[serde(rename = "type")]
  _type: String,
  ts: String,
  bot_id: String,
  app_id: String,
  text: String,
  team: String,
}
