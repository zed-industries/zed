ALSA bindings for Rust
=======================

Thin but safe wrappers for [ALSA](https://alsa-project.org), the most
common API for accessing audio devices on Linux.

[![crates.io](https://img.shields.io/crates/v/alsa.svg)](https://crates.io/crates/alsa)
[![API documentation](https://docs.rs/alsa/badge.svg)](https://docs.rs/alsa)
[![license](https://img.shields.io/crates/l/alsa.svg)](https://crates.io/crates/alsa)

The ALSA API is rather big, so everything is not covered yet, but expect the following to work:

 * Audio Playback (example in `pcm` module docs)

 * Audio Recording

 * Mixer controls

 * HCtl API (jack detection example in `hctl` module docs)

 * Raw midi

 * Midi sequencer (most of it)

 * Ctl API

 * Device name hints (example in `device_name` module docs)

 * Enumerations of all of the above

 * Poll and/or wait for all of the above

The following is not yet implemented (mostly because nobody asked for them) :

 * Separate timer API (snd_timer_*)

 * Config API (snd_config_*)

 * Plug-in API

Quickstart guide / API design:

 * Most functions map 1-to-1 to alsa-lib functions, e g, `ctl::CardInfo::get_id()` is a wrapper around
   `snd_ctl_card_info_get_id` and the [alsa-lib documentation](https://www.alsa-project.org/alsa-doc/alsa-lib/)
   can be consulted for additional information.

 * Structs are RAII and closed/freed on drop, e g, when a `PCM` struct is dropped, `snd_pcm_close` is called.

 * To read and write buffers, call the `io_*` methods. It will return a separate struct from which you can
   read or write, and which can also be used for mmap (if supported by the driver).

 * Error handling - most alsa-lib functions can return errors, so the return value from these is a `Result`.

 * Enumeration of cards, devices etc is done through structs implementing `Iterator`.

 * Many structs implement `poll::Descriptors`, to combine with your favorite async framework.
   Or just use `wait` if you don't need non-blocking functionality.
   
Notes:

 * To run the tests successfully, there must be a "default" sound card configured. This is usually not a problem when running on normal hardware, but some CI systems, docker images etc, might not have that configured by default. 

