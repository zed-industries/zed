# Local Collaboration

## Setting up the local collaboration server

### Setting up for the first time?

1. Make sure you have livekit installed (`brew install livekit`)
1. Install [Postgres](https://postgresapp.com) and run it.
1. Then, from the root of the repo, run `script/bootstrap`.

### Have a db that is out of date? / Need to migrate?

1. Make sure you have livekit installed (`brew install livekit`)
1. Try `cd crates/collab && cargo run -- migrate` from the root of the repo.
1. Run `script/seed-db`

## Testing collab locally

1. Run `foreman start` from the root of the repo.
1. In another terminal run `script/zed-local -2`.
1. Two copies of Zed will open. Add yourself as a contact in the one that is not you.
1. Start a collaboration session as normal with any open project.
