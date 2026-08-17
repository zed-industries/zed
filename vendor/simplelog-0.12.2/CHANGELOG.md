## v0.12.0
 - Replaces the semingly unmainted chrono library with the time crate.
 - Addresses through this update 
     - RUSTSEC-2020-0159 (chrono)
     - RUSTSEC-2020-0071 (time)
 - `ConfigBuilder::set_time_to_local` is replaced by `ConfigBuilder::set_time_offset_to_local`.
     - This function requires the new (and by default activated) `local-offset` feature.
     - This function may fail, for more information read the docs.
     - simplelog will not determine the local offset dynamically anymore, but only once, when this config option is set.
         - Due to safety reasons there is no way to provide this property currently.
 - `ConfigBuilder::set_time_offset` now takes a `time::UtcOffset` instead of a `chrono::FixedOffset`.
 - `ConfigBuilder::set_time_format` has been replaced by three new variants
     - `ConfigBuilder::set_time_format_rfc2822` sets the time format to use the format described by rfc2822.
     - `ConfigBuilder::set_time_format_rfc3339` sets the time format to use the format described by rfc3339.
     - `ConfigBuilder::set_time_format_custom` sets the time format to a custom time format best created using `time::macros::format_description`.
         - Runtime provided time format configuration is possible, but difficult due to lifetime constraints.
         - Fixing this will require a solution to https://github.com/time-rs/time/issues/429
     - *Note*: The default format is unchanged "[hour]:[minute]:[second]"

## v0.11.0
 - Add colored log levels using `ansi_term` (PR #88, credits to @manio)
 - Add target padding (PR #85, credits to @bytebeamio)
 - Add optional color and style support using `paris` crate (PR #84, credits to @manio)

## v0.10.2
 - Fix flushing with `BufferedStandardStreams` (PR #82, credits to @mrkline)

## v0.10.1
 - Fix TermLogger performance using `termcolor::BufferedStandardStream` (PR #80, credits to @mrkline)
 - Add write thread name support for `termlog` (PR #76, credits to @zeroflaw)

## v0.10.0
 - Fix wrong argument naming (PR #70, credits to @scvalex)
 - *Breaking*: More color customization options (PR #72, credits to @mrkline)
 - Clarify docs on config levels

## v0.9.0
 - Add customizable level label colors (PR #69. credits to @JarrettBillingsley)
 - Remove unneeded TermLogError
 - Future-proof config by adding `#[non_exhaustive]`
 - Fix compiling with minimal-versions

## v0.8.0
 - Switch from `term` to `termcolor` (PR #59. credits to @raybritton)
 - Fix typo in docs (PR #58, credits to @anthonyjmartinez)
 - Switch default padding to `Off`. Padding is annoyingly controversal, just set it to whatever you prefer, if you want it.

## v0.7.6
 - Derive `Clone`,`Copy`,`PartialEq`,`Eq`,`Debug` and `Hash` for `TerminalMode`. (PR #56, credits to @panhania)

## v0.7.5
 - Use `target()` instead of `module_path()` for filtering as suggested by `log`s docs. (PR #52, credits to @tsidea)
 - Allow logging thread names and pad thread ids (defaults match the old behaviour). (PR #53, credits to @mulark)

## v0.7.4
 - Fixed time formats with 

## v0.7.3
 - Fixed `test` feature

## v0.7.2
 - Allow Level Padding to be configured

## v0.7.1
 - Fix `LevelFilter:Off` not turning off parts of the log messages

## v0.7.0
 - Add local time zone option
 - Change config to builder pattern avoiding future major version bumps
 - Add further documentation about dealing with `TermLogger` failing
 - `term` 0.6.* is now also supported

## v0.6.0
 - Add time offset parameter (defaults to UTC)
 - Add thread_id output (credits to @kurtlawrence)
 - Pad log level
 - Add TestLogger (credits to @AlexW-GH)
 - Add stream configuration to TermLogger
 - Implement allow- and ignore-filtering (credits to @ryankurte)

## v0.5.3
 - Fix minimal chrono version / update to chrono v0.4.1 (PR #27, credits to @samueltardieu)

## v0.5.2
 - Don't interleave stdout and stderr (PR #23, credits to @hansjorg)

## v0.5.1
 - Updated `simplelog` to `log` 0.4.1
 - Updated `simplelog` to `term` 0.5.1
 - Fixed compiler warning about unused `#[macro_use]`

## v0.5.0
 - Update `simplelog` to `log` 0.4

## v0.4.4
 - Fixed building non-default feature sets

## v0.4.3
 - Publically export TermLogger Error type

## v0.4.2
 - Removed a debug println! statement

## v0.4.1
 - Fix `None` config value

## v0.4.0
 - `Config` is not using `LogLevelFilter` anymore but `Option<LogLevel>`
     - `None` represents no logging of the `Config` parameter at all
     - `LogLevelFilter::Off` was supposed to provide this feature, but is actually
       ordered higher then `LogLevelFilter::Error`, and presents *no filtering* instead
       of the incorrectly assumed *filter everything*.

## v0.3.0
 - Merged PullRequest by *Antoni Boucher* - Avoid unwrapping in `TermLogger`:
     - `TermLogger::new` now returns an Option
     - `TermLogger::init` now has its own ErrorType
 - Move FileLogger to WriteLogger
     - More generic, accepts not only `File`, but everything that implements `Write`
 - Loggers now initialize with a `Config` struct, that list, what parts of the typical log message shall be printed, at which levels
     - Migrate from `::init(level, ...)` to `::init(level, Config::default(), ...)`
     - Migrate from `::new(level, ...)` or `::new(level, Config::default(), ...)`
     - Exception: `CombinedLogger` has no use for its own `Config` and does not take the new argument
     - Optionally use rust's `Default` syntax to override some settings
         - E.g. always print location: `Config { location: LogLevelFilter::Error, ..Default::default() }`
         - E.g. never print the target: `Config { target: LogLevelFilter::Off, ..Default::default() }`
 - Target does now by default only log for Debug and lower
 - Added CHANGELOG
 - Removed some internal code duplication

## v0.2.0
 - Local changes that (accidentially) made it to crates.io, but not git
     - Basically a worse version of *Antoni Boucher* 0.3.0 changes
     - Got noticed, when he made a Pull Request

 Sorry, what a mess. If you still have that version, here are the docs:
     https://docs.rs/simplelog/0.2.0/

## v0.1.0
 - Initial Release
