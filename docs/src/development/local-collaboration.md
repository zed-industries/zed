# Local Collaboration

1. Ensure you have access to our cloud infrastructure. If you don't have access, you can't collaborate locally at this time.

2. Make sure you've installed Zed's dependencies for your platform:

- [macOS](#macos)
- [Linux](#linux)
- [Windows](#backend-windows)

Note that `collab` can be compiled only with MSVC toolchain on Windows

3. Clone down our cloud repository and follow the instructions in the cloud README

4. Setup the local database for your platform:

- [macOS & Linux](#database-unix)
- [Windows](#database-windows)

5. Run collab:

- [macOS & Linux](#run-collab-unix)
- [Windows](#run-collab-windows)

## Backend Dependencies

If you are developing collaborative features of Zed, you'll need to install the dependencies of zed's `collab` server:

- PostgreSQL
- LiveKit
- Foreman

You can install these dependencies natively or run them under Docker.

### macOS

1. Install [Postgres.app](https://postgresapp.com) or [postgresql via homebrew](https://formulae.brew.sh/formula/postgresql@15):

   ```sh
   brew install postgresql@15
   ```

2. Install [Livekit](https://formulae.brew.sh/formula/livekit) and [Foreman](https://formulae.brew.sh/formula/foreman)

   ```sh
   brew install livekit foreman
   ```

- Follow the steps in the [collab README](https://github.com/zed-industries/zed/blob/main/crates/collab/README.md) to configure the Postgres database for integration tests

Alternatively, if you have [Docker](https://www.docker.com/) installed you can bring up all the `collab` dependencies using Docker Compose.

### Linux

1. Install [Postgres](https://www.postgresql.org/download/linux/)

   ```sh
   sudo apt-get install postgresql                    # Ubuntu/Debian
   sudo pacman -S postgresql                          # Arch Linux
   sudo dnf install postgresql postgresql-server      # RHEL/Fedora
   sudo zypper install postgresql postgresql-server   # OpenSUSE
   ```

2. Install [Livekit](https://github.com/livekit/livekit-cli)

   ```sh
   curl -sSL https://get.livekit.io/cli | bash
   ```

3. Install [Foreman](https://theforeman.org/manuals/3.15/quickstart_guide.html)

### Windows {#backend-windows}

> This section is still in development. The instructions are not yet complete.

- Install [Postgres](https://www.postgresql.org/download/windows/)
- Install [Livekit](https://github.com/livekit/livekit), optionally you can add the `livekit-server` binary to your `PATH`.

Alternatively, if you have [Docker](https://www.docker.com/) installed you can bring up all the `collab` dependencies using Docker Compose.

### Docker {#Docker}

If you have docker or podman available, you can run the backend dependencies inside containers with Docker Compose:

```sh
docker compose up -d
```

## Database setup

Before you can run the `collab` server locally, you'll need to set up a `zed` Postgres database.

### On macOS and Linux {#database-unix}

```sh
script/bootstrap
```

This script will set up the `zed` Postgres database, and populate it with some users. It requires internet access, because it fetches some users from the GitHub API.

The script will seed the database with various content defined by:

```sh
cat crates/collab/seed.default.json
```

To use a different set of admin users, you can create your own version of that json file and export the `SEED_PATH` environment variable. Note that the usernames listed in the admins list currently must correspond to valid GitHub users.

```json [settings]
{
  "admins": ["admin1", "admin2"],
  "channels": ["zed"]
}
```

### On Windows {#database-windows}

```powershell
.\script\bootstrap.ps1
```

## Testing collaborative features locally

### On macOS and Linux {#run-collab-unix}

Ensure that Postgres is configured and running, then run Zed's collaboration server and the `livekit` dev server:

```sh
foreman start
# OR
docker compose up
```

Alternatively, if you're not testing voice and screenshare, you can just run `collab` and `cloud`, and not the `livekit` dev server:

```sh
cargo run -p collab -- serve all
```

```sh
cd ../cloud; cargo make dev
```

In a new terminal, run two or more instances of Zed.

```sh
script/zed-local -3
```

This script starts one to four instances of Zed, depending on the `-2`, `-3` or `-4` flags. Each instance will be connected to the local `collab` server, signed in as a different user from `.admins.json` or `.admins.default.json`.

### On Windows {#run-collab-windows}

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

You'll also need to start the cloud server:

```powershell
cd ..\cloud; cargo make dev
```

In a new terminal, run two or more instances of Zed.

```powershell
node .\script\zed-local -2
```

Note that this requires `node.exe` to be in your `PATH`.

## Running a local collab server

> [!NOTE]
> Because of recent changes to our authentication system, Zed will not be able to authenticate itself with, and therefore use, a local collab server.

If you want to run your own version of the zed collaboration service, you can, but note that this is still under development, and there is no support for authentication nor extensions.

Configuration is done through environment variables. By default it will read the configuration from [`.env.toml`](https://github.com/zed-industries/zed/blob/main/crates/collab/.env.toml) and you should use that as a guide for setting this up.

By default Zed assumes that the DATABASE_URL is a Postgres database, but you can make it use Sqlite by compiling with `--features sqlite` and using a sqlite DATABASE_URL with `?mode=rwc`.

To authenticate you must first configure the server by creating a seed.json file that contains at a minimum your github handle. This will be used to create the user on demand.

```json [settings]
{
  "admins": ["nathansobo"]
}
```

By default the collab server will seed the database when first creating it, but if you want to add more users you can explicitly reseed them with `SEED_PATH=./seed.json cargo run -p collab seed`

Then when running the zed client you must specify two environment variables, `ZED_ADMIN_API_TOKEN` (which should match the value of `API_TOKEN` in .env.toml) and `ZED_IMPERSONATE` (which should match one of the users in your seed.json)
