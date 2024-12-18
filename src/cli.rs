use crate::globals::{ENV_SLACK_CHANNEL, ENV_SLACK_TOKEN};
use clap::{builder::Styles, Command, Parser};
use clap_complete::aot::Shell;
use clap_complete::aot::{generate, Generator};
use env::VarError;
use std::env;
use std::io;

#[derive(Parser, Debug, Default)]
#[command(
  about = "Send a simple slack message.",
  after_help = "Info:\n--auth-token and --channel can be omitted if env vars are set.\nSLACK_MESSAGE_TOKEN='xoxb-123...'\nSLACK_MESSAGE_CHANNEL='channelName'",
  author = "Christian Lansford",
  name = "slack-message",
  styles = Styles::plain(),
  version,
)]
pub struct Cli {
  #[arg(short, long)]
  auth_token: Option<String>,
  #[arg(short, long)]
  pub channel: Option<String>,
  #[arg(long)]
  pub completion: Option<Shell>,
  #[arg(short, long)]
  pub icon: Option<String>,
  pub message: String,
  #[arg(short, long)]
  pub username: Option<String>,
}

impl Cli {
  pub fn get_channel(&self) -> Result<String, VarError> {
    find_arg_or_env(self.channel.as_ref(), ENV_SLACK_CHANNEL)
  }

  pub fn get_oauth_token(&self) -> Result<String, VarError> {
    find_arg_or_env(self.auth_token.as_ref(), ENV_SLACK_TOKEN)
  }
}

pub fn print_completions<G: Generator>(gen: G, cmd: &mut Command) {
  generate(gen, cmd, cmd.get_name().to_string(), &mut io::stdout());
}

/// search precedence: provided arg, env var
fn find_arg_or_env(
  arg: Option<&String>, env_var: &str,
) -> Result<String, VarError> {
  if let Some(val) = arg {
    return Ok(val.to_string());
  }

  match env::var(env_var) {
    Ok(val) => Ok(val),
    Err(e) => {
      eprintln!("Couldn't find {env_var}\n{e:?}");
      Err(e)
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use clap::builder::{Str, StyledStr};
  use clap::{Command, CommandFactory, Id};
  use serial_test::serial;
  use std::error::Error;

  #[test]
  fn command() -> Result<(), Box<dyn Error>> {
    let cli = Cli::command();
    cli.clone().debug_assert();
    assert!(cli.get_long_flag().is_none());
    assert!(cli.get_short_flag().is_none());
    assert!(cli.get_long_about().is_none());
    assert!(cli.get_before_help().is_none());
    assert!(cli.get_before_long_help().is_none());
    assert!(cli.get_after_long_help().is_none());
    assert!(cli.get_display_name().is_none());
    assert!(cli.get_long_version().is_none());
    assert!(cli.get_long_version().is_none());
    assert!(cli.get_subcommands().collect::<Vec<&Command>>().is_empty());
    assert_eq!(Some("Christian Lansford"), cli.get_author());
    assert_eq!("slack-message", cli.get_name());
    Ok(())
  }

  #[test]
  fn command_about() -> Result<(), Box<dyn Error>> {
    let cli = Cli::command();
    cli.clone().debug_assert();
    let styled_str = StyledStr::from("Send a simple slack message.");
    let expected_about = Some(&styled_str);
    assert_eq!(expected_about, cli.get_about());
    Ok(())
  }

  #[test]
  fn command_after_help() -> Result<(), Box<dyn Error>> {
    let cli = Cli::command();
    cli.clone().debug_assert();
    let styled_str = StyledStr::from("Info:\n--auth-token and --channel can be omitted if env vars are set.\nSLACK_MESSAGE_TOKEN='xoxb-123...'\nSLACK_MESSAGE_CHANNEL='channelName'");
    let expected_after_help = Some(&styled_str);
    assert_eq!(expected_after_help, cli.get_after_help());
    Ok(())
  }

  #[test]
  fn command_groups() -> Result<(), Box<dyn Error>> {
    let cli = Cli::command();
    cli.clone().debug_assert();
    let mut contains_cli = false;
    for arg_group in cli.get_groups() {
      if "Cli" == arg_group.get_id().as_str() {
        contains_cli = true;
        let args = arg_group.get_args().collect::<Vec<&Id>>();
        let expected_args = vec![
          Id::from("auth_token"),
          Id::from("channel"),
          Id::from("completion"),
          Id::from("icon"),
          Id::from("message"),
          Id::from("username"),
        ];
        for expected_arg in expected_args {
          println!("arg: {expected_arg}");
          assert!(args.contains(&&expected_arg));
        }
      }
    }
    assert!(contains_cli);
    Ok(())
  }

  #[test]
  fn auth_token_flag() -> Result<(), Box<dyn Error>> {
    flag_test("auth_token", Some('a'), Some("auth-token"), "AUTH_TOKEN")?;
    Ok(())
  }

  #[test]
  fn channel_flag() -> Result<(), Box<dyn Error>> {
    flag_test("channel", Some('c'), Some("channel"), "CHANNEL")?;
    Ok(())
  }

  #[test]
  fn completion_flag() -> Result<(), Box<dyn Error>> {
    flag_test("completion", None, Some("completion"), "COMPLETION")?;
    Ok(())
  }

  #[test]
  fn icon_flag() -> Result<(), Box<dyn Error>> {
    flag_test("icon", Some('i'), Some("icon"), "ICON")?;
    Ok(())
  }

  #[test]
  fn username_flag() -> Result<(), Box<dyn Error>> {
    flag_test("username", Some('u'), Some("username"), "USERNAME")?;
    Ok(())
  }

  #[test]
  fn message_arg() -> Result<(), Box<dyn Error>> {
    flag_test("message", None, None, "MESSAGE")?;
    Ok(())
  }

  #[test]
  fn get_oauth_token_arg() -> Result<(), Box<dyn Error>> {
    let expected = String::from("testArgToken");
    let cli = Cli { auth_token: Some(expected.clone()), ..Default::default() };
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
    let cli = Cli { ..Default::default() };
    let expected = String::from("testEnvToken");
    env::set_var(ENV_SLACK_TOKEN, expected.clone());
    let actual = cli.get_oauth_token()?;
    assert_eq!(
      expected, actual,
      "\n  expected: {expected}\n  actual: {actual}"
    );
    env::remove_var(ENV_SLACK_TOKEN);
    Ok(())
  }

  #[test]
  #[serial]
  fn get_oauth_token_fail() -> Result<(), Box<dyn Error>> {
    env::remove_var(ENV_SLACK_TOKEN);
    let cli = Cli { ..Default::default() };
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
    let cli = Cli { channel: Some(expected.clone()), ..Default::default() };
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
    let cli = Cli { ..Default::default() };
    let expected = String::from("testEnvChannel");
    env::set_var(ENV_SLACK_CHANNEL, expected.clone());
    let actual = cli.get_channel()?;
    assert_eq!(
      expected, actual,
      "\n  expected: {expected}\n  actual: {actual}"
    );
    env::remove_var(ENV_SLACK_CHANNEL);
    Ok(())
  }

  #[test]
  #[serial]
  fn get_channel_fail() -> Result<(), Box<dyn Error>> {
    env::remove_var(ENV_SLACK_CHANNEL);
    let cli = Cli { ..Default::default() };
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

  fn flag_test(
    id: &str, short: Option<char>, long: Option<&str>, val_name: &'static str,
  ) -> Result<(), Box<dyn Error>> {
    let cli = Cli::command();
    cli.clone().debug_assert();
    let args = cli.get_arguments();
    let mut contains_arg = false;
    for arg in args {
      if id == arg.get_id().as_str() {
        contains_arg = true;
        match short {
          Some(_) => assert_eq!(short, arg.get_short()),
          None => assert!(arg.get_short().is_none()),
        };
        match long {
          Some(_) => assert_eq!(long, arg.get_long()),
          None => assert!(arg.get_long().is_none()),
        };
        assert!(arg.get_help().is_none());
        assert!(arg.get_long_help().is_none());
        assert!(arg.get_value_names().unwrap().contains(&Str::from(val_name)));
      };
    }
    assert!(contains_arg);
    Ok(())
  }
}
