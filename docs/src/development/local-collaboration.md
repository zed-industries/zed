# Local Collaboration

First, make sure you've installed Zed's backend dependencies for your platform:

- [macOS](./macos.md#backend-dependencies)
- [Linux](./linux.md#backend-dependencies)
- [Windows](./windows.md#backend-dependencies)

## Database setup

Before you can run the `collab` server locally, you'll need to set up a `zed` Postgres database.

### On macOS and Linux

```sh
script/bootstrap
```

This script will set up the `zed` Postgres database, and populate it with some users. It requires internet access, because it fetches some users from the GitHub API.

The script will seed the database with various content defined by:

```sh
cat crates/collab/seed.default.json
```

To use a different set of admin users, you can create your own version of that json file and export the `SEED_PATH` environment variable. Note that the usernames listed in the admins list currently must correspond to valid Github users.

```json
{
  "admins": ["admin1", "admin2"],
  "channels": ["zed"]
}
```

### On Windows

```powershell
.\script\bootstrap.ps1
```

## Testing collaborative features locally

### On macOS and Linux

Ensure that Postgres is configured and running, then run Zed's collaboration server and the `livekit` dev server:

```sh
foreman start
# OR
cargo run -p collab -- serve all
```

In a new terminal, run two or more instances of Zed.

```sh
script/zed-local -2
```

This script starts one to four instances of Zed, depending on the `-2`, `-3` or `-4` flags. Each instance will be connected to the local `collab` server, signed in as a different user from `.admins.json` or `.admins.default.json`.

### On Windows

Since `foreman` is not available on Windows, you can run the following commands in separate terminals:

```powershell
cargo run --package=collab -- serve all
```

If you have added the `livekit-server` binary to your `PATH`, you can run:

```powershell
livekit-server --dev
```

Otherwise,

```powershell
.\path\to\livekit-serve.exe --dev
```

In a new terminal, run two or more instances of Zed.

```powershell
node .\script\zed-local -2
```

Note that this requires `node.exe` to be in your `PATH`.

## Running a local collab server

If you want to run your own version of the zed collaboration service, you can, but note that this is still under development, and there is no good support for authentication nor extensions.

Configuration is done through environment variables. By default it will read the configuration from [`.env.toml`](https://github.com/zed-industries/zed/blob/main/crates/collab/.env.toml) and you should use that as a guide for setting this up.

By default Zed assumes that the DATABASE_URL is a Postgres database, but you can make it use Sqlite by compiling with `--features sqlite` and using a sqlite DATABASE_URL with `?mode=rwc`.

To authenticate you must first configure the server by creating a seed.json file that contains at a minimum your github handle. This will be used to create the user on demand.

```json
{
  "admins": ["nathansobo"]
}
```

By default the collab server will seed the database when first creating it, but if you want to add more users you can explicitly reseed them with `SEED_PATH=./seed.json cargo run -p collab seed`

Then when running the zed client you must specify two environment variables, `ZED_ADMIN_API_TOKEN` (which should match the value of `API_TOKEN` in .env.toml) and `ZED_IMPERSONATE` (which should match one of the users in your seed.json)
