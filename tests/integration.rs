#[cfg(test)]
mod tests {
  use assert_cmd::Command;
  use regex::Regex;
  use std::time::{SystemTime, UNIX_EPOCH};

  const SLACK_MESSAGE: &str = "slack-message";
  const TEST_ICON: &str = ":test:";

  fn parse_timestamp(v: Vec<u8>) -> String {
    let r = Regex::new(r"[+-]?([0-9]*[.])?[0-9]+").unwrap();
    let s = str::from_utf8(v.as_slice()).unwrap();
    r.find(s).unwrap().as_str().to_string()
  }

  #[test]
  #[ignore]
  fn send_message() {
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let mut cmd = Command::cargo_bin(SLACK_MESSAGE).unwrap();

    let msg = format!("integration test message {now:?}");
    let assert = cmd.arg(msg).arg("--icon").arg(TEST_ICON).assert();

    let cmd_output = assert.get_output().stdout.clone();
    let ts = parse_timestamp(cmd_output);
    let expected_output = format!("Message sent, timestamp: {ts}\n");
    assert.success().stdout(expected_output);
  }

  #[test]
  #[ignore]
  fn send_message_with_username() {
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

    let cmd_output = assert.get_output().stdout.clone();
    let ts = parse_timestamp(cmd_output);
    let expected_output = format!("Message sent, timestamp: {ts}\n");
    assert.success().stdout(expected_output);
  }

  #[test]
  #[ignore]
  fn send_message_as_reply() {
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let mut cmd1 = Command::cargo_bin(SLACK_MESSAGE).unwrap();
    let msg = format!("integration test reply message original {now:?}");
    let assert_cmd1 =
      cmd1.arg(msg).arg("--icon").arg(TEST_ICON).assert().success();
    let output_stdout = assert_cmd1.get_output().stdout.clone();
    let cmd1_ts = parse_timestamp(output_stdout);

    let reply_msg = format!("integration test reply message reply {now:?}");
    let mut cmd2 = Command::cargo_bin(SLACK_MESSAGE).unwrap();
    let assert = cmd2
      .arg(reply_msg)
      .arg("--icon")
      .arg(TEST_ICON)
      .arg("--timestamp")
      .arg(cmd1_ts)
      .assert();

    let cmd2_ts = parse_timestamp(assert.get_output().stdout.clone());
    let expected_stdout = format!("Message sent, timestamp: {}\n", cmd2_ts);
    assert.success().stdout(expected_stdout);
    //chris TODO extract timestamp from stdout str
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
