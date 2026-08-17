//! Configuration file API
//!
//! For now, just contains functions regarding the global configuration
//! stored as a cache inside alsa-lib. Calling `update_free_global` might help
//! against valgrind reporting memory leaks.
use crate::{alsa};
use super::error::*;
use super::Output;
use std::ptr;

pub fn update() -> Result<bool> {
	acheck!(snd_config_update()).map(|x| x != 0)
}

pub fn update_free_global() -> Result<()> {
	acheck!(snd_config_update_free_global()).map(|_| ())
}

/// [snd_config_t](https://alsa-project.org/alsa-doc/alsa-lib/group___config.html) wrapper
pub struct Config(*mut alsa::snd_config_t);

impl Drop for Config {
    fn drop(&mut self) { unsafe { alsa::snd_config_unref(self.0) }; }
}

pub fn update_ref() -> Result<Config> {
	let mut top = ptr::null_mut();
	acheck!(snd_config_update_ref(&mut top)).map(|_| Config(top))
}

impl Config {
    pub fn save(&self, o: &mut Output) -> Result<()> {
        acheck!(snd_config_save(self.0, super::io::output_handle(o))).map(|_| ())
    }
}

#[test]
fn config_save() {
	let c = update_ref().unwrap();
    let mut outp = Output::buffer_open().unwrap();
	c.save(&mut outp).unwrap();
    println!("== Config save ==\n{}", outp);
}