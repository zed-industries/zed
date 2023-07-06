[⬅ Back to Index](./index.md)

# Developing Zed's Backend

Zed's backend consists of the following components:

- The Zed.dev web site
  - implemented in the [`zed.dev`](https://github.com/zed-industries/zed.dev) repository
  - hosted on [Vercel](https://vercel.com/zed-industries/zed-dev).
- The Zed Collaboration server
  - implemented in the [`crates/collab`](https://github.com/zed-industries/zed/tree/main/crates/collab) directory of the main `zed` repository
  - hosted on [DigitalOcean](https://cloud.digitalocean.com/projects/6c680a82-9d3b-4f1a-91e5-63a6ca4a8611), using Kubernetes
- The Zed Postgres database
  - defined via migrations in the [`crates/collab/migrations`](https://github.com/zed-industries/zed/tree/main/crates/collab/migrations) directory
  - hosted on DigitalOcean

---

## Local Development

Here's some things you need to develop backend code locally.

### Dependencies

- **Postgres** - download [Postgres.app](https://postgresapp.com).

### Setup

1. Check out the `zed` and `zed.dev` repositories into a common parent directory
2. Set the `GITHUB_TOKEN` environment variable to one of your GitHub personal access tokens (PATs).

   - You can create a PAT [here](https://github.com/settings/tokens).
   - You may want to add something like this to your `~/.zshrc`:

     ```
     export GITHUB_TOKEN=<the personal access token>
     ```

3. In the `zed.dev` directory, run `npm install` to install dependencies.
4. In the `zed directory`, run `script/bootstrap` to set up the database
5. In the `zed directory`, run `foreman start` to start both servers

---

## Production Debugging

### Datadog

Zed uses Datadog to collect metrics and logs from backend services. The Zed organization lives within Datadog's _US5_ [site](https://docs.datadoghq.com/getting_started/site/), so it can be accessed at [us5.datadoghq.com](https://us5.datadoghq.com). Useful things to look at in Datadog:

- The [Logs](https://us5.datadoghq.com/logs) page shows logs from Zed.dev and the Collab server, and the internals of Zed's Kubernetes cluster.
- The [collab metrics dashboard](https://us5.datadoghq.com/dashboard/y2d-gxz-h4h/collab?from_ts=1660517946462&to_ts=1660604346462&live=true) shows metrics about the running collab server
