# changelog for synchronoise

## [0.4.0] - 2017-11-02
### Changed
- `SignalEvent` and `CountdownEvent` have been updated to not use `Mutex`es internally
  - In the refactor, `wait_timeout` was changed to always wait for the full duration if the signal
    was never set, so the return value has been changed to reflect this. This is a **breaking
    change**
  - In the refactor, `CountdownEvent` was changed so that its counter is now is `usize` instead of
    an `isize`. Since it's an invalid state for the counter to be below zero, let's encode that in
    the type. This is a **breaking change** if you were directly using the counter value or its type
    before

## [0.3.0] - 2017-03-06
### Added
- `CountdownEvent::guard` and `CountdownGuard`, to provide scope-based increment/decrement
- `WriterReaderPhaser`, a primitive that allows multiple wait-free "writer critical sections"
  against a "reader phase flip" that waits for currently-active writers to finish
  - also the related structs `PhaserCriticalSection` and `PhaserReadLock`

## [0.2.0] - 2017-02-28
### Added
- `CountdownEvent::wait_timeout`, to wait but also have a timeout
- `SignalEvent`, a primitive that lets one or more threads wait for a signal from another one

## [0.1.0] - 2017-02-27
### Added
- `CountdownEvent`, a primitive that keeps a counter that allows you to block until it hits zero

<!-- vim: set tw=100 expandtab: -->
