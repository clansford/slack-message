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
  let bearer_token = args.get_oauth_token()?;
  let msg = Message { channel: args.get_channel()?, text: args.message, username: args.username };
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
