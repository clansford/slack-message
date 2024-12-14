use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
  pub channel: String,
  #[serde(rename = "token")]
  pub auth_token: String,
}

fn find_config() -> Result<PathBuf, Box<dyn Error>> {
  let paths = vec![
    "~/.slack-message.toml",
    "~/.config/slack-message/config.toml",
    "~/.config/slack-message/slack-message.toml",
  ];

  for path in &paths {
    if Path::new(path).exists() {
      return Ok(PathBuf::from(path));
    }
  }

  Err("Couldn't find config.toml".into())
}

fn read_config(path: &PathBuf) -> Result<Config, Box<dyn Error>> {
  let file = fs::read_to_string(&path)?;
  let config: Config = toml::from_str(&file)?;
  Ok(config)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn read_config() -> Result<(), Box<dyn Error>> {
    let path = PathBuf::from("./tests/test_config.toml");
    let cfg = super::read_config(&path)?;
    let expected_channel = "test-channel";
    let expected_token = "test-token";
    assert_eq!(expected_channel, cfg.channel);
    assert_eq!(expected_token, cfg.auth_token);
    Ok(())
  }

  #[test]
  fn read_config_bad_config() -> Result<(), Box<dyn Error>> {
    let path = PathBuf::from("./tests/bad_test_config.toml");
    let result = super::read_config(&path);
    match result {
      Ok(_) => unreachable!("read_config supposed to fail."),
      Err(e) => {
        let expected = "TOML parse error at line 1, column 1\n  |\n1 | key = \"value\"\n  | ^^^^^^^^^^^^^\nmissing field `channel`\n";
        assert_eq!(expected, e.to_string());
      }
    }
    Ok(())
  }

  #[test]
  fn find_config() -> Result<(), Box<dyn Error>> {
    // chris bookmark TODO finish find_config
    let cfg = super::find_config()?;
    println!("{cfg:?}");
    Ok(())
  }
}
