# Slack-Message

Send a slack message from command line.

## External requirements

- Slack app and bot token.
- rust to build/install it.
- [Just](https://github.com/casey/just) (optional)

## Build, Test, Install

- Integration tests are ignored because they utilize an actual slack workspace.
- Tests that utilize mocks are suffixed with "mock".

### With [Just](https://github.com/casey/just)

```shell
just build
just test
just test_integration
just install
```

### Without [Just](https://github.com/casey/just)

```shell
cargo build --workspace
cargo test --workspace
cargo test --workspace -- --include-ignored
cargo install --path .
```

## Setup

1. Create environment variable 'SLACK_MESSAGE_TOKEN' and 'SLACK_MESSAGE_CHANNEL'
    - SLACK_MESSAGE_TOKEN is the slack app's OAuth token
    - SLACK_MESSAGE_CHANNEL is the slack channel id to send the message in.
1. (Note) supports .env file for development.
