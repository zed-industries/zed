# Local Collaboration

First, make sure you've installed Zed's [backend dependencies](./developing_zed__building_zed.md#backend-dependencies).

## Database setup

Before you can run the `collab` server locally, you'll need to set up a `zed` Postgres database.

```
script/bootstrap
```

This script will set up the `zed` Postgres database, and populate it with some users. It requires internet access, because it fetches some users from the GitHub API.

The script will create several _admin_ users, who you'll sign in as by default when developing locally. The GitHub logins for these default admin users are specified in this file:

```
cat crates/collab/.admins.default.json
```

To use a different set of admin users, you can create a file called `.admins.json` in the same directory:

```
cat > crates/collab/.admins.json <<JSON
[
  "your-github-login",
  "another-github-login"
]
JSON
```

## Testing collaborative features locally

In one terminal, run Zed's collaboration server and the `livekit` dev server:

```
foreman start
```

In a second terminal, run two or more instances of Zed.

```
script/zed-local -2
```

This script starts one to four instances of Zed, depending on the `-2`, `-3` or `-4` flags. Each instance will be connected to the local `collab` server, signed in as a different user from `.admins.json` or `.admins.default.json`.
