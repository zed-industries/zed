# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.7.32](https://github.com/livekit/rust-sdks/compare/rust-sdks/livekit@0.7.31...rust-sdks/livekit@0.7.32) - 2026-02-16

### Fixed

- fix full_reconnect downgrade & don't ignore Leave messages ([#893](https://github.com/livekit/rust-sdks/pull/893))

### Other

- turn single peerconnection off by default ([#897](https://github.com/livekit/rust-sdks/pull/897))
- ensure signal connections times out properly and retries ([#895](https://github.com/livekit/rust-sdks/pull/895))
- added Single Peer Connection support to Rust ([#888](https://github.com/livekit/rust-sdks/pull/888))
- set the simulcast codec & layers ([#891](https://github.com/livekit/rust-sdks/pull/891))

## [0.7.31](https://github.com/livekit/rust-sdks/compare/rust-sdks/livekit@0.7.30...rust-sdks/livekit@0.7.31) - 2026-02-10

### Other

- don't use clamp as the ultimate_kbps can be lower than 300 ([#886](https://github.com/livekit/rust-sdks/pull/886))
- pre-connect the publisher PC when an RPC handler is registered ([#880](https://github.com/livekit/rust-sdks/pull/880))

## [0.7.30](https://github.com/livekit/rust-sdks/compare/rust-sdks/livekit@0.7.29...rust-sdks/livekit@0.7.30) - 2026-02-09

### Fixed

- fix the RPC race condition ([#865](https://github.com/livekit/rust-sdks/pull/865))

### Other

- update proto & fix CI ([#871](https://github.com/livekit/rust-sdks/pull/871))
- Use workspace dependencies & settings ([#856](https://github.com/livekit/rust-sdks/pull/856))
- Upgrade protocol to v1.44.0 ([#857](https://github.com/livekit/rust-sdks/pull/857))
- Expose participant's permission to ffi layer ([#824](https://github.com/livekit/rust-sdks/pull/824))

## [0.7.29](https://github.com/livekit/rust-sdks/compare/rust-sdks/livekit@0.7.28...rust-sdks/livekit@0.7.29) - 2026-01-15

### Fixed

- ensure Room.creation_time is ms ([#822](https://github.com/livekit/rust-sdks/pull/822))

### Other

- try setting x-google-start-bitrate for vp9 ([#820](https://github.com/livekit/rust-sdks/pull/820))

## [0.7.28](https://github.com/livekit/rust-sdks/compare/rust-sdks/livekit@0.7.27...rust-sdks/livekit@0.7.28) - 2025-12-19

### Added

- *(ParticipantInfo)* export kind details ([#813](https://github.com/livekit/rust-sdks/pull/813))

## [0.7.27](https://github.com/livekit/rust-sdks/compare/rust-sdks/livekit@0.7.26...rust-sdks/livekit@0.7.27) - 2025-12-17

### Other

- Handle server initiated mute request ([#812](https://github.com/livekit/rust-sdks/pull/812))

## [0.7.26](https://github.com/livekit/rust-sdks/compare/rust-sdks/livekit@0.7.25...rust-sdks/livekit@0.7.26) - 2025-12-04

### Added

- *(connector)* initial service impl ([#790](https://github.com/livekit/rust-sdks/pull/790))

### Fixed

- fix mute/unmute events for LocalTrack. ([#799](https://github.com/livekit/rust-sdks/pull/799))

### Other

- Add RoomEvent::TokenRefreshed ([#803](https://github.com/livekit/rust-sdks/pull/803))

## [0.7.25](https://github.com/livekit/rust-sdks/compare/rust-sdks/livekit@0.7.24...rust-sdks/livekit@0.7.25) - 2025-11-20

### Other

- perform full reconnect if resume fails ([#792](https://github.com/livekit/rust-sdks/pull/792))
- E2E RPC tests ([#769](https://github.com/livekit/rust-sdks/pull/769))
- Remove unused dependencies ([#761](https://github.com/livekit/rust-sdks/pull/761))

## [0.7.24](https://github.com/livekit/rust-sdks/compare/rust-sdks/livekit@0.7.23...rust-sdks/livekit@0.7.24) - 2025-10-27

### Other

- Expose set video quality ([#759](https://github.com/livekit/rust-sdks/pull/759))

## [0.7.23](https://github.com/livekit/rust-sdks/compare/rust-sdks/livekit@0.7.22...rust-sdks/livekit@0.7.23) - 2025-10-23

### Other

- add h265 codec support ([#762](https://github.com/livekit/rust-sdks/pull/762))

## [0.7.22](https://github.com/livekit/rust-sdks/compare/rust-sdks/livekit@0.7.21...rust-sdks/livekit@0.7.22) - 2025-10-22

### Other

- License check ([#746](https://github.com/livekit/rust-sdks/pull/746))
- Remove participant check for data packets ([#757](https://github.com/livekit/rust-sdks/pull/757))
- clamp connection timeout and fixed the comment ([#748](https://github.com/livekit/rust-sdks/pull/748))
- put examples in root Cargo workspace ([#731](https://github.com/livekit/rust-sdks/pull/731))

## [0.7.21](https://github.com/livekit/rust-sdks/compare/rust-sdks/livekit@0.7.20...rust-sdks/livekit@0.7.21) - 2025-10-13

### Added

- *(e2ee)* add data channel encryption ([#708](https://github.com/livekit/rust-sdks/pull/708))

### Fixed

- fix some potential audio issues, clean up the code a bit, and suppress some warnings  ([#737](https://github.com/livekit/rust-sdks/pull/737))
- do not log 'signal client closed: "stream closed"' on disconnect ([#727](https://github.com/livekit/rust-sdks/pull/727))

### Other

- Test participant disconnect ([#732](https://github.com/livekit/rust-sdks/pull/732))
- Increase RPC max RT time to 7s ([#729](https://github.com/livekit/rust-sdks/pull/729))
- E2E audio test ([#724](https://github.com/livekit/rust-sdks/pull/724))

## [0.7.20](https://github.com/livekit/rust-sdks/compare/rust-sdks/livekit@0.7.19...rust-sdks/livekit@0.7.20) - 2025-10-03

### Other

- updated the following local packages: libwebrtc

## [0.7.19](https://github.com/livekit/rust-sdks/compare/rust-sdks/livekit@0.7.18...rust-sdks/livekit@0.7.19) - 2025-09-29

### Fixed

- apply original participant fields in data messages ([#709](https://github.com/livekit/rust-sdks/pull/709))

### Other

- Implement Display and Error for RpcError ([#719](https://github.com/livekit/rust-sdks/pull/719))
- Fix intermittently failing E2E reliability test ([#718](https://github.com/livekit/rust-sdks/pull/718))
- Do not modify raw packets ([#714](https://github.com/livekit/rust-sdks/pull/714))
- Add send_bytes method ([#691](https://github.com/livekit/rust-sdks/pull/691))
- Disable opus red for e2ee enabled clients ([#706](https://github.com/livekit/rust-sdks/pull/706))
- Upgrade protocol to v1.41.0 ([#703](https://github.com/livekit/rust-sdks/pull/703))

## [0.7.18](https://github.com/livekit/rust-sdks/compare/rust-sdks/livekit@0.7.17...rust-sdks/livekit@0.7.18) - 2025-09-09

### Other

- Add option to enable audio preconnect buffer in SDK & FFI ([#700](https://github.com/livekit/rust-sdks/pull/700))
- Data channel reliability ([#688](https://github.com/livekit/rust-sdks/pull/688))

## [0.7.17](https://github.com/livekit/rust-sdks/compare/rust-sdks/livekit@0.7.16...rust-sdks/livekit@0.7.17) - 2025-09-03

### Other

- updated the following local packages: livekit-api, libwebrtc

## [0.7.16](https://github.com/livekit/rust-sdks/compare/rust-sdks/livekit@0.7.15...rust-sdks/livekit@0.7.16) - 2025-07-31

### Other

- updated the following local packages: livekit-api
# Changelog

## [0.7.15](https://github.com/livekit/rust-sdks/compare/rust-sdks/livekit@0.7.14...rust-sdks/livekit@0.7.15) - 2025-07-16

### Other

- remove published tracks when the room is closed ([#677](https://github.com/livekit/rust-sdks/pull/677))

## [0.7.14](https://github.com/livekit/rust-sdks/compare/rust-sdks/livekit@0.7.13...rust-sdks/livekit@0.7.14) - 2025-06-23

### Fixed

- `audio_frame_ms` didn't work expectedly ([#671](https://github.com/livekit/rust-sdks/pull/671))

## [0.7.13](https://github.com/livekit/rust-sdks/compare/rust-sdks/livekit@0.7.12...rust-sdks/livekit@0.7.13) - 2025-06-17

### Other

- Expose room updates, support MoveParticipant (protocol 15) ([#662](https://github.com/livekit/rust-sdks/pull/662))

## [0.7.12](https://github.com/livekit/rust-sdks/compare/rust-sdks/livekit@0.7.11...rust-sdks/livekit@0.7.12) - 2025-06-11

### Fixed

- fix duration overflow ([#654](https://github.com/livekit/rust-sdks/pull/654))

### Other

- Remove debouncer when fast_publish is enabled ([#649](https://github.com/livekit/rust-sdks/pull/649))

## [0.7.9] - 2025-04-08

### Added

- High-level data streams API

### Deprecated

- Low-level data stream packet events

## [0.7.8] - 2025-03-21

### Fixed

- Revert the RPC change, need more robust way

## [0.7.7] - 2025-03-18

### Fixed

- Move RPC handlers to room

## [0.7.6] - 2025-02-28

### Added

- Add audio filter support

## [0.7.5] - 2025-02-06

### Fixed

- Fix a dependency issue with an older version of the libwebrtc crate

## [0.7.4] - 2025-02-03

### Added

- Support for track subscription permissions

## [0.7.3] - 2025-01-17

### Added

- Add an API to set buffer_amount_low_threshold for DataChannel
- Update RoomInfo to contain buffer_amount_low_threshold for DataChannel

### Fixed

- Wait for the buffered amount to become low before sending data during publish_data for Reliable Data Channel

## [0.7.2] - 2025-01-04

### Added

- bump

### Fixed

- Fixed deadlock with nested RPC calls

## [0.7.1] - 2024-12-14

### Added

- bump libwebrtc to m125
