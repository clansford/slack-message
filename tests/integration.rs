#[cfg(test)]
mod tests {
  use assert_cmd::Command;
  use std::time::{SystemTime, UNIX_EPOCH};

  const SLACK_MESSAGE: &str = "slack-message";
  const TEST_ICON: &str = ":test:";

  #[test]
  #[ignore]
  fn send_message() {
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let mut cmd = Command::cargo_bin(SLACK_MESSAGE).unwrap();
    let msg = format!("integration test message {now:?}");
    let assert = cmd.arg(msg).arg("--icon").arg(TEST_ICON).assert();
    assert.success().stdout("Message sent successfully\n");
  }

  #[test]
  #[ignore]
  fn send_message_name() {
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let mut cmd = Command::cargo_bin(SLACK_MESSAGE).unwrap();
    let msg = format!("integration test message {now:?}");
    let assert = cmd
      .arg(msg)
      .arg("--username")
      .arg("TEST-NAME")
      .arg("--icon")
      .arg(TEST_ICON)
      .assert();
    assert.success().stdout("Message sent successfully\n");
  }

  #[test]
  #[ignore]
  fn no_message_provided() {
    let mut cmd = Command::cargo_bin(SLACK_MESSAGE).unwrap();
    let assert = cmd.assert();
    let expected_out = "error: the following required arguments were not provided:\n  <MESSAGE>\n\nUsage: slack-message <MESSAGE>\n\nFor more information, try '--help'.\n";
    assert.failure().code(2).stderr(expected_out);
  }
}
