#[macro_use]
extern crate log;
extern crate env_logger;

use std::env;
use std::process;
use std::str;
use std::thread;

struct DropMe;

impl Drop for DropMe {
    fn drop(&mut self) {
        debug!("Dropping now");
    }
}

fn run() {
    // Use multiple thread local values to increase the chance that our TLS
    // value will get destroyed after the FORMATTER key in the library
    thread_local! {
        static DROP_ME_0: DropMe = DropMe;
        static DROP_ME_1: DropMe = DropMe;
        static DROP_ME_2: DropMe = DropMe;
        static DROP_ME_3: DropMe = DropMe;
        static DROP_ME_4: DropMe = DropMe;
        static DROP_ME_5: DropMe = DropMe;
        static DROP_ME_6: DropMe = DropMe;
        static DROP_ME_7: DropMe = DropMe;
        static DROP_ME_8: DropMe = DropMe;
        static DROP_ME_9: DropMe = DropMe;
    }
    DROP_ME_0.with(|_| {});
    DROP_ME_1.with(|_| {});
    DROP_ME_2.with(|_| {});
    DROP_ME_3.with(|_| {});
    DROP_ME_4.with(|_| {});
    DROP_ME_5.with(|_| {});
    DROP_ME_6.with(|_| {});
    DROP_ME_7.with(|_| {});
    DROP_ME_8.with(|_| {});
    DROP_ME_9.with(|_| {});
}

fn main() {
    env_logger::init();
    if env::var("YOU_ARE_TESTING_NOW").is_ok() {
        // Run on a separate thread because TLS values on the main thread
        // won't have their destructors run if pthread is used.
        // https://doc.rust-lang.org/std/thread/struct.LocalKey.html#platform-specific-behavior
        thread::spawn(run).join().unwrap();
    } else {
        let exe = env::current_exe().unwrap();
        let out = process::Command::new(exe)
            .env("YOU_ARE_TESTING_NOW", "1")
            .env("RUST_LOG", "debug")
            .output()
            .unwrap_or_else(|e| panic!("Unable to start child process: {}", e));
        if !out.status.success() {
            println!("test failed: {}", out.status);
            println!("--- stdout\n{}", str::from_utf8(&out.stdout).unwrap());
            println!("--- stderr\n{}", str::from_utf8(&out.stderr).unwrap());
            process::exit(1);
        }
    }
}
