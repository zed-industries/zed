v4.0.0-alpha
==================
  * dbus message types are now hidden
  * rename Notification{Hint, Image, Urgency}
  * make `hint_from_key_val` a method
  * reorganize modules
  * make server an optional feature (was never stable)
  * remove `handle_actions`

v3.6.0 / 2019-05-16
==================

  * remove error-chain: use pure std::error implementation
  * update mac-notification-sys

v3.5.0 / 2018-10-21
===================

  * update dbus-rs to 0.6

v3.4.3 / 2018-10-13
===================

  * bump version to 3.4.3
  * update image and lazy-static
  * applied some more clippy
  * Complement requirements documentation and examples
  * applied some clippy
  * some manually assisted rustfmt
  * format examples
  * cleanups and modernizations
  * Depedency update
  * Updated error-chain
  * including readme in package
  * update docs link
  * Create LICENSE-MIT
  * Create LICENSE-Apache
  * added requirements to readme
  * formatting
  * made NotificationHandle::new() internally public
  * bumped version to 3.4.1
  * publicly using NotificationHandle again
  * Put mac-notificatio-sys errors as foreign links in error.rs
  * Fix new mac-notification-sys integration

v3.4.0 / 2017-05-21
===================

  * changed version to 3.4.0
  * exposing Error again
  * fixed build issues
  * feature gate all the things
  * image!
  * static spec version and Error
  * image_data
  * first working version of image data
  * badges!

3.3.1 / 2017-05-06
==================

  * bumped to version 3.3.1
  * readme update
  * add sound example
  * fix target_os
  * documented platform compatibility
  * updating history
  * examples compile on macOS
  * Fix some style issues given by clippy (#27)
  * Minimize number of Timeout <-> i32 convertors (only single pair needed) * Make `dbus::FromMessageItem<'a> for Timeout` produce correct enum for Default and Never cases
  * removed placeholder `Error`
  * a little more macOS documentation
  * simplyfied error handling in show (macOS)
  * fix missing field in server.rs add missing documentation
  * updating to mac-notification-sys 0.1.1
  * split notification summary into title and subtitle
  * fixing mac-notification-sys usage
  * satisfied lints
  * more moving things around
  * documentation, tests and deduplication
  * testing platform specific documentation
  * moved xdg specifics into extra module
  * target specific deps
  * first minimal notification on macOS
  * deprecated .actions
  * added an extra type for timeout
  * 3... 2... 1... NEW RELEASE! \0/

v3.2.1 / 2016-09-07
===================

  * 3... 2... 1... NEW RELEASE! \0/
  * updated to dbus ~0.4 and fixed warnings
  * ready to release v3.2.0
  * example to show notification in bg thread, good enough to fix #18
  * documentation: more examples
  * added fn auto_icon()
  * added on_close(F) method to NotificationHandle
  * `.show(&mut self)` -> `.show(&self)`
  * chore: applied clippy
  * added use of id when sending
  * pubbed NotificaionHint const strings and added hint_from_key_val
  * removing outdated commandline help from readme
  * fix clippy version again

v3.1.1 / 2016-03-03
===================

  * changed version to v3.1.1
  * exposed dbus Error because its returned in public methods

v3.1.0 / 2016-03-01
===================

  * changed version to v3.1.0
  * made clippy happy
  * removed redundant example
  * info example print a valid yaml
  * updated examples
  * added examples for custom_int and reuse of notifications
  * added custom int option
  * updated examples to current api

v3.0.4 / 2016-02-15
===================

  * updated dbus-rs dependency
  * use cfg unix instead of linux
  * updated to dbus ~0.3
  * minimal preparations for work on macos
  * reduced the TCB
  * fuzzier crate versions

v3.0.3 / 2015-11-03
===================

  * made a clippy a dev dependency only

v3.0.2 / 2015-11-03
===================

  * removed bin from crate
  * applied "good practices" checks
  * readme examples
  * building against stable
  * no longer testing with 1.0.0, those times are over

v3.0.1 / 2015-10-23
===================

  * bin moved in with lib
  * added examples to README
  * works with dbus 0.2.0
  * Add missing docs for ServerInformation
  * Add missing docs for NotificationUrgency
  * Add missing docs to Hint
  * Add missing documentation to NotificationServer
  * Document NotificationHandle::wait_for_action
  * why do I always forget the milk? no, the README :D

v3.0.0 / 2015-10-01
===================

  * changed version to 3.0.0
  * test example file for Hints
  * adjusted to new Urgencies
  * Tidying up
  * more documentation
  * added: Urgency::from(&str) for convenience
  * updated changelog and history
  * fixed: hint types other than just strings
  * added convenience from::<str>() for Urgency
  * renamed urgencies: Low, Normal and Critical (as in both standards)
  * resolved timeout TODO
  * fixed doc examples
  * tests and examples
  * added internal `stop_server()`
  * fixed urgency in example
  * removed redundant Urgency field
  * serializing and deserializing Notifications
  * introducing actual error handling, lots of breaking changes
  * renamed get_information example into info.rs
  * more idiomatic implementation of Hints
  * added clippy as dev-dependency
  * Cli: defaulting to --help if not command is issued
  * tracking nightly dbus-rs
  * updated changelog

v2.1.0 / 2015-09-27
===================

  * fixed: Hint Types are not only strings

v2.0.0 / 2015-08-04
===================

  * changed version to 2.0.0
  * fixed example code to match new api
  * Handle unused_must_use case
  * Update examples and tests
  * Remove old API
  * Impl update, close, etc. for NotificationHandle
  * Set transient hint in wait_for_closing example

v1.1.0 / 2015-08-03
===================

  * changed version to 1.1.0
  * small refactoring
  * Add NotificationHandle to keep dbus conns alive
  * added: notify-cli as child project
  * added: GetCapabilities() and GetServerinformation() to server
  * added a few possible panic messages
  * updated readme
  * using src/server for testing

v1.0.1 / 2015-07-19
===================

  * fixed: actions and hints were not passed along

v1.0.0 / 2015-07-13
===================

  * releasing 1.0.0
  * added hints to `show_debug()`
  * added urgency to `show_debug()`
  * how could I have forgotten `urgency`?
  * building agains dbus 0.1.1
  * improved documentation
  * shields
  * took a note from clippy
  * travis always test against 1.0.0, stable and nightly

v0.9.0 / 2015-07-01
===================

  * added: updateing notifications ( see example )
  * replace `.to_string()` with `.to_owned()`
  * updated readme
  * `if let` is nicer than `match`

v0.8.0 / 2015-06-19
===================

  * corrected versioning scheme

v0.0.8 / 2015-06-19
===================

  * changed version to v0.0.8
  * added: extra example for closing signal alone
  * added `"__closed"` action that is 'invoked' when the Notification closes
  * added doc about show_and_wait_for_action()
  * sacrificed storing closure approach for simpler construct
  * added interface for adding action alongside closure
  * checking for a certain action invoked
  * signal listener breaks on `NotificationClosed` too
  * checking the notificatoin id when waiting for a signal
  * can listen for ActionInvoked signals
  * added waiting_for_action_signal stub
  * removed examples from README, doc should be up to date

v0.0.7 / 2015-06-13
===================

  * ready for v0.0.7
  * implemented `GetServerInformation()`
  * Capitalize all the headlines!
  * Urgency is now an enum
  * pack methods return actual empty arrays now (builds with 1.0.0)
  * moved to dbus-rs 0.1.0
  * removed ruby script
  * moved examples into tests
  * added: close_notification
  * is this worth commiting?
  * made exe_name() private
  * elaborated documentation
  * displaying generic version in readme
  * corrected build status image
  * removed doc submodule
  * doc up to date

v0.0.6 / 2015-06-08
===================

  * writing history
  * added documentation link
  * changed submodule url
  * added my own gh-pages branch as submodule as branch as submodule
  * added note about Notification::hint()
  * added link to doc to README
  * added: `Notification.action(identifer, label)` for your convenience
  * previous commit only compiles on rust-nightly :(
  * pack methods return actual empty arrays now
  * pack methods return MessageItem directly
  * fixed: index out of bounds when no hints here used
  * added ruby script for testing
  * integrated hints, removed `NotificationCategory`
  * added backend to hints()
  * added: Hints and Categories API frontend
  * renaming crates is generally a bad idea
  * removed travis batch
  * pointed travis batch into right direction
  * removed left over send()s
  * changed dependency notice in readme
  * removed send()
  * updated readme
  * renamed into notify-rust
  * renamed send() into show()
  * added paragraph example
  * changed default timeout -1
  * working on documentation
  * added my ultimate goal
  * added some bookmarks for myself to read
  * wrote a little documentation
  * moved tests out of ./src
  * modified server output
  * added NotificationServer (barely works)
  * implemented Notification::actions()
  * implemented GetCapabilities()
  * send() and send_debug() now returns Notifcation ID
  * travis: build only
  * build dependencies for travis
  * whitespace removal
  * README.md: add travic-ci batch
  * Add support for travis-ci
  * added send_debug to readme

v0.0.4 / 2015-05-30
===================

  * changed version to v0.0.4
  * added more examples
  * added: formatting example
  * added: loop_test()
  * added: appname() and send_debug()

v0.0.3 / 2015-05-24
===================

  * changed API: using builder pattern now
  * getting ready for 0.0.3
  * removed duplication
  * updated example in README
  * added icon() and timeout() to builder pattern
  * implemented builder pattern
  * trying out a hole lot of design patterns

v0.0.2 / 2015-05-22
===================


