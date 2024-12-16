mod cli;
mod slack;

use crate::cli::Cli;
use crate::slack::{Client, Message};
use clap::Parser;
use core::panic;
use dotenv::dotenv;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
  dotenv().ok();
  let args = Cli::parse();
  let token = args.get_oauth_token()?;
  let msg = Message {
    channel: &args.get_channel()?,
    icon_emoji: args.icon.as_deref(),
    text: &args.message,
    username: args.username.as_deref(),
  };
  let slack = Client::new(&token);
  let res = slack.send_message(&msg).await?;
  if res.ok {
    println!("Message sent");
  } else {
    eprintln!("{res:#?}");
    panic!("Error: Message not sent");
  };
  Ok(())
}
