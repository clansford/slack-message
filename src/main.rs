use core::panic;
use std::env;

use clap::Parser;
use dotenv::dotenv;
use reqwest::{header, Client};
use serde::{Deserialize, Serialize};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  dotenv().ok();
  let args = Cli::parse();
  let channel = get_channel(&args);
  let bearer_token = get_bearer_token(&args);
  let slack_message = SlackMessage { channel, text: args.message };
  let body = create_request_body(slack_message);
  let resp = Client::new()
    .post("https://slack.com/api/chat.postMessage")
    .header(header::AUTHORIZATION, bearer_token)
    .header(header::CONTENT_TYPE, "application/json; charset=utf-8")
    .json(&body)
    .send()
    .await?;

  let resp_body = resp.text().await?;
  let slack_response = parse_res_body(&resp_body);
  if slack_response.ok {
    println!("Message sent successfully");
    Ok(())
  } else {
    panic!("Error: Message not sent successfully");
  }
}

fn parse_res_body(body: &str) -> SlackResponse {
  match serde_json::from_str::<SlackResponse>(body) {
    Ok(res) => res,
    Err(e) => panic!("{e}"),
  }
}

fn create_request_body(slack_message: SlackMessage) -> serde_json::Value {
  let val = serde_json::to_value(slack_message);
  match val {
    Ok(v) => v,
    Err(e) => panic!("{e}"),
  }
}

fn get_channel(cli: &Cli) -> String {
  if cli.channel.is_some() {
    return cli.channel.clone().unwrap();
  }
  match env::var("SLACK_CHANNEL") {
    Ok(c) => c,
    Err(e) => panic!("{e}"),
  }
}

fn get_bearer_token(cli: &Cli) -> String {
  let token = if cli.auth_token.is_some() {
    cli.auth_token.clone().unwrap()
  } else {
    match env::var("SLACK_TOKEN") {
      Ok(c) => c,
      Err(e) => panic!("{e}"),
    }
  };
  format!("Bearer {token}")
}

#[derive(Parser, Debug)]
struct Cli {
  #[arg(short, long)]
  channel: Option<String>,
  message: String,
  #[arg(short, long)]
  auth_token: Option<String>,
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
