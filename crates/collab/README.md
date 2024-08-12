# Zed Server

This crate is what we run at https://collab.zed.dev.

It contains our back-end logic for collaboration, to which we connect from the Zed client via a websocket after authenticating via https://zed.dev, which is a separate repo running on Vercel.

# Local Development

## Database setup

Before you can run the collab server locally, you'll need to set up a zed Postgres database.

```sh
script/bootstrap
```

This script will set up the `zed` Postgres database, and populate it with some users. It requires internet access, because it fetches some users from the GitHub API.

The script will create several _admin_ users, who you'll sign in as by default when developing locally. The GitHub logins for the default users are specified in the `seed.default.json` file.

To use a different set of admin users, create `crates/collab/seed.json`.

```json
{
  "admins": ["yourgithubhere"],
  "channels": ["zed"],
  "number_of_users": 20
}
```

## Testing collaborative features locally

In one terminal, run Zed's collaboration server and the livekit dev server:

```sh
foreman start
```

In a second terminal, run two or more instances of Zed.

```sh
script/zed-local -2
```

This script starts one to four instances of Zed, depending on the `-2`, `-3` or `-4` flags. Each instance will be connected to the local `collab` server, signed in as a different user from `seed.json` or `seed.default.json`.

# Deployment

We run two instances of collab:

- Staging (https://staging-collab.zed.dev)
- Production (https://collab.zed.dev)

Both of these run on the Kubernetes cluster hosted in Digital Ocean.

Deployment is triggered by pushing to the `collab-staging` (or `collab-production`) tag in Github. The best way to do this is:

- `./script/deploy-collab staging`
- `./script/deploy-collab production`

You can tell what is currently deployed with `./script/what-is-deployed`.

# Database Migrations

To create a new migration:

```sh
./script/create-migration <name>
```

Migrations are run automatically on service start, so run `foreman start` again. The service will crash if the migrations fail.

When you create a new migration, you also need to update the [SQLite schema](./migrations.sqlite/20221109000000_test_schema.sql) that is used for testing.
