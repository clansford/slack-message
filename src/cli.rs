use clap::Parser;
use env::VarError;
use std::env;

#[derive(Parser, Debug)]
pub struct Cli {
  #[arg(short, long)]
  pub channel: Option<String>,
  pub message: String,
  #[arg(short, long)]
  auth_token: Option<String>,
  #[arg(long)]
  pub username: Option<String>,
}

impl Cli {
  pub fn get_channel(&self) -> Result<String, VarError> {
    find_arg_or_env(self.channel.as_ref(), "SLACK_CHANNEL")
  }

  pub fn get_oauth_token(&self) -> Result<String, VarError> {
    find_arg_or_env(self.auth_token.as_ref(), "SLACK_TOKEN")
  }
}

fn find_arg_or_env(
  arg: Option<&String>, env_var: &str,
) -> Result<String, VarError> {
  if let Some(val) = arg {
    return Ok(val.to_string());
  }
  match env::var(env_var) {
    Ok(tok) => Ok(tok),
    Err(e) => {
      eprintln!("Couldn't find {env_var}\n{e:?}");
      Err(e)
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use serial_test::serial;
  use std::error::Error;

  #[test]
  fn get_oauth_token_arg() -> Result<(), Box<dyn Error>> {
    let expected = String::from("testArgToken");
    let cli = Cli {
      channel: None,
      message: String::new(),
      auth_token: Some(expected.clone()),
      username: None,
    };
    let actual = cli.get_oauth_token()?;
    assert_eq!(
      expected, actual,
      "\n  expected: {expected}\n  actual: {actual}"
    );
    Ok(())
  }

  #[test]
  #[serial]
  fn get_oauth_token_env() -> Result<(), Box<dyn Error>> {
    let cli = Cli {
      channel: None,
      message: String::new(),
      auth_token: None,
      username: None,
    };
    let slack_token = "SLACK_TOKEN";
    let expected = String::from("testEnvToken");
    env::set_var(slack_token, expected.clone());
    let actual = cli.get_oauth_token()?;
    assert_eq!(
      expected, actual,
      "\n  expected: {expected}\n  actual: {actual}"
    );
    env::remove_var(slack_token);
    Ok(())
  }

  #[test]
  #[serial]
  fn get_oauth_token_fail() -> Result<(), Box<dyn Error>> {
    let cli = Cli {
      channel: None,
      message: String::new(),
      auth_token: None,
      username: None,
    };
    let actual = match cli.get_oauth_token() {
      Ok(_) => {
        eprintln!(
          "This shouldn't be reachable because the token shouldn't be set."
        );
        unreachable!()
      }
      Err(e) => handle_env_var_error(e)?,
    };
    let expected = "environment variable not found";
    assert_eq!(
      expected, actual,
      "\n  expected: {expected}\n  actual: {actual}"
    );
    Ok(())
  }

  #[test]
  fn get_channel_arg() -> Result<(), Box<dyn Error>> {
    let expected = String::from("testArgChannel");
    let cli = Cli {
      channel: Some(expected.clone()),
      message: String::new(),
      auth_token: None,
      username: None,
    };
    let actual = cli.get_channel()?;
    assert_eq!(
      expected, actual,
      "\n  expected: {expected}\n  actual: {actual}"
    );
    Ok(())
  }

  #[test]
  #[serial]
  fn get_channel_env() -> Result<(), Box<dyn Error>> {
    let cli = Cli {
      channel: None,
      message: String::new(),
      auth_token: None,
      username: None,
    };
    let slack_channel = "SLACK_CHANNEL";
    let expected = String::from("testEnvChannel");
    env::set_var(slack_channel, expected.clone());
    let actual = cli.get_channel()?;
    assert_eq!(
      expected, actual,
      "\n  expected: {expected}\n  actual: {actual}"
    );
    env::remove_var(slack_channel);
    Ok(())
  }

  #[test]
  #[serial]
  fn get_channel_fail() -> Result<(), Box<dyn Error>> {
    let cli = Cli {
      channel: None,
      message: String::new(),
      auth_token: None,
      username: None,
    };
    let actual = match cli.get_channel() {
      Ok(_) => {
        unreachable!(
          "This shouldn't be reachable because the channel shouldn't be set."
        )
      }
      Err(e) => handle_env_var_error(e)?,
    };
    let expected = "environment variable not found";
    assert_eq!(
      expected, actual,
      "\n  expected: {expected}\n  actual: {actual}"
    );
    Ok(())
  }

  fn handle_env_var_error(e: VarError) -> Result<String, Box<dyn Error>> {
    match e {
      env::VarError::NotPresent => Ok(e.to_string()),
      env::VarError::NotUnicode(_) => {
        eprintln!("NotUnicode is an unexpected state of the tests.");
        unreachable!()
      }
    }
  }
}
