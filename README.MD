# Slack-Message
Send a slack message from command line.

## Setup (optional)
1. Create environment variable 'SLACK_TOKEN' and 'SLACK_CHANNEL'
- Will check environment variables if they are not passed to  the command as flags.
- SLACK_TOKEN is the slack app's oauth token
- SLACK_CHANNEL is the slack channel id to send the message in.

## TODOS
- cleanup.
- analyze panics / better error messages if necessary to panic.
