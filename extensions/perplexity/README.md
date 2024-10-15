# Zed Perplexity Extension

This example extension adds the `/perplexity` [slash command](https://zed.dev/docs/assistant/commands) to the Zed AI assistant.

## Usage

Open the AI Assistant panel (`cmd-r` or `ctrl-r`) and enter:

```
/perplexity What's the weather in Boulder, CO tomorrow evening?
```

## Development Setup

1. Install the Rust toolchain and clone the zed repo:

   ```
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

   mkdir -p ~/code
   cd ~/code
   git clone https://github.com/zed-industries/zed
   ```

1. Open Zed
1. Open Zed Extensions (`cmd-shift-x` / `ctrl-shift-x`)
1. Click "Install Dev Extension"
1. Navigate to the "extensions/perplexity" folder inside the zed git repo.
1. Ensure your `PERPLEXITY_API_KEY` environment variable is set (instructions below)

   ```sh
   env | grep PERPLEXITY_API_KEY
   ```

1. Quit and relaunch Zed

## PERPLEXITY_API_KEY

This extension requires a Perplexity API key to be available via the `PERPLEXITY_API_KEY` environment variable.

To onbtain a Perplexity.ai API token, login to your Perplexity.ai account and go [Settings->API](https://www.perplexity.ai/settings/api) and under "API Keys" click "Generate". This will require you to have [Perplexity Pro](https://www.perplexity.ai/pro) or to buy API credits. By default the extension uses `llama-3.1-sonar-small-128k-online`, currently cheapest model available which is roughly half a penny per request + a penny per 50,000 tokens. So most requests will cost less than $0.01 USD.

Take your API key and add it to your environment by adding `export PERPLEXITY_API_KEY="pplx-0123456789abcdef..."` to your `~/.zshrc` or `~/.bashrc`. Reload close and reopen your terminal session. Check with `env |grep PERPLEXITY_API_KEY`.
