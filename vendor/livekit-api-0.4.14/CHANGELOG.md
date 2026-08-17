# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.4.14](https://github.com/livekit/rust-sdks/compare/rust-sdks/livekit-api@0.4.13...rust-sdks/livekit-api@0.4.14) - 2026-02-16

### Fixed

- fix full_reconnect downgrade & don't ignore Leave messages ([#893](https://github.com/livekit/rust-sdks/pull/893))

### Other

- turn single peerconnection off by default ([#897](https://github.com/livekit/rust-sdks/pull/897))
- ensure signal connections times out properly and retries ([#895](https://github.com/livekit/rust-sdks/pull/895))
- added Single Peer Connection support to Rust ([#888](https://github.com/livekit/rust-sdks/pull/888))

## [0.4.13](https://github.com/livekit/rust-sdks/compare/rust-sdks/livekit-api@0.4.12...rust-sdks/livekit-api@0.4.13) - 2026-02-09

### Other

- update proto & fix CI ([#871](https://github.com/livekit/rust-sdks/pull/871))
- Use workspace dependencies & settings ([#856](https://github.com/livekit/rust-sdks/pull/856))
- Upgrade protocol to v1.44.0 ([#857](https://github.com/livekit/rust-sdks/pull/857))
- expose audio mixing as a configurable field ([#850](https://github.com/livekit/rust-sdks/pull/850))
- UniFFI interface ([#750](https://github.com/livekit/rust-sdks/pull/750))

## [0.4.12](https://github.com/livekit/rust-sdks/compare/rust-sdks/livekit-api@0.4.11...rust-sdks/livekit-api@0.4.12) - 2026-01-15

### Fixed

- fix incorrect 404 status code when WS connection fails ([#832](https://github.com/livekit/rust-sdks/pull/832))

### Other

- Add Wait Until Answered for CreateSip ([#833](https://github.com/livekit/rust-sdks/pull/833))

## [0.4.11](https://github.com/livekit/rust-sdks/compare/rust-sdks/livekit-api@0.4.10...rust-sdks/livekit-api@0.4.11) - 2025-12-04

### Added

- *(connector)* initial service impl ([#790](https://github.com/livekit/rust-sdks/pull/790))

## [0.4.10](https://github.com/livekit/rust-sdks/compare/rust-sdks/livekit-api@0.4.9...rust-sdks/livekit-api@0.4.10) - 2025-11-20

### Added

- add outbound trunk config for create_sip_participant. ([#771](https://github.com/livekit/rust-sdks/pull/771))

## [0.4.9](https://github.com/livekit/rust-sdks/compare/rust-sdks/livekit-api@0.4.8...rust-sdks/livekit-api@0.4.9) - 2025-10-22

### Other

- License check ([#746](https://github.com/livekit/rust-sdks/pull/746))
- Agent dispatch APIs ([#742](https://github.com/livekit/rust-sdks/pull/742))
- Access token via auth header ([#728](https://github.com/livekit/rust-sdks/pull/728))

## [0.4.8](https://github.com/livekit/rust-sdks/compare/rust-sdks/livekit-api@0.4.7...rust-sdks/livekit-api@0.4.8) - 2025-10-13

### Other

- Bump reqwest to 0.12 ([#711](https://github.com/livekit/rust-sdks/pull/711))

## [0.4.7](https://github.com/livekit/rust-sdks/compare/rust-sdks/livekit-api@0.4.6...rust-sdks/livekit-api@0.4.7) - 2025-09-29

### Other

- updated the following local packages: livekit-protocol

## [0.4.6](https://github.com/livekit/rust-sdks/compare/rust-sdks/livekit-api@0.4.5...rust-sdks/livekit-api@0.4.6) - 2025-09-03

### Other

- add attributes to Claims and AccessToken ([#693](https://github.com/livekit/rust-sdks/pull/693))

## [0.4.5](https://github.com/livekit/rust-sdks/compare/rust-sdks/livekit-api@0.4.4...rust-sdks/livekit-api@0.4.5) - 2025-07-31

### Other

- Expose departure_timeout in CreateRoomRequest ([#685](https://github.com/livekit/rust-sdks/pull/685))
- Handle region fetch errors ([#686](https://github.com/livekit/rust-sdks/pull/686))
# Changelog

## [0.4.4](https://github.com/livekit/rust-sdks/compare/rust-sdks/livekit-api@0.4.3...rust-sdks/livekit-api@0.4.4) - 2025-06-17

### Other

- Expose room updates, support MoveParticipant (protocol 15) ([#662](https://github.com/livekit/rust-sdks/pull/662))

## [0.4.2] - 2025-02-03

### Added

- Update protocol and add SendDataRequest nonce
