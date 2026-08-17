# Version 1.9.0

- Fix a bug where `WeakSender/WeakReceiver` could incorrectly return `Some` even if the channel is already closed (#60)
- Remove the unnecessary `T: Clone` bound from `WeakSender/WeakReceiver`'s `Clone` implementation (#62)

# Version 1.8.0

- Prevent deadlock if sender/receiver is forgotten (#49)
- Add weak sender and receiver (#51)
- Update `concurrent-queue` to v2 (#50)

# Version 1.7.1

- Work around MSRV increase due to a cargo bug.

# Version 1.7.0

- Add `send_blocking` and `recv_blocking` (#47)

# Version 1.6.1

- Make `send` return `Send` (#34)

# Version 1.6.0

- Added `Send` and `Recv` futures (#33)
- impl `FusedStream` for `Receiver` (#30)

# Version 1.5.1

- Fix typos in the docs.

# Version 1.5.0

- Add `receiver_count()` and `sender_count()`.

# Version 1.4.2

- Fix a bug that would sometime cause 100% CPU usage.

# Version 1.4.1

- Update dependencies.

# Version 1.4.0

- Update dependencies.

# Version 1.3.0

- Add `Sender::is_closed()` and `Receiver::is_closed()`.

# Version 1.2.0

- Add `Sender::close()` and `Receiver::close()`.

# Version 1.1.1

- Replace `usize::MAX` with `std::usize::MAX`.

# Version 1.1.0

- Add methods to error types.

# Version 1.0.0

- Initial version
