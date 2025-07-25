mod cli;
mod globals;
mod slack;

use crate::cli::Cli;
use crate::slack::{Client, Message};
use clap::{CommandFactory, Parser};
use core::panic;
use dotenvy::dotenv;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  dotenv().ok();
  let args = Cli::parse();
  if let Some(shell) = args.completion {
    cli::print_completions(shell, &mut Cli::command());
    return Ok(());
  };

  let token = args.get_oauth_token()?;
  let msg = Message {
    channel: &args.get_channel()?,
    icon_emoji: args.icon.as_deref(),
    text: &args.message,
    thread_ts: args.timestamp.as_deref(),
    username: args.username.as_deref(),
  };
  let slack = Client::new(&token);
  let res = slack.send_message(&msg).await?;
  if res.ok {
    println!("Message sent, timestamp: {}", res.ts);
  } else {
    eprintln!("{res:#?}");
    panic!("Error: Message not sent");
  };
  Ok(())
}
