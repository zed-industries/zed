use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::process::Command;

use jobserver::Client;

macro_rules! t {
    ($e:expr) => {
        match $e {
            Ok(e) => e,
            Err(e) => panic!("{} failed with {}", stringify!($e), e),
        }
    };
}

fn main() {
    if env::var("_DO_THE_TEST").is_ok() {
        std::process::exit(
            Command::new(env::var_os("MAKE").unwrap())
                .env("MAKEFLAGS", env::var_os("CARGO_MAKEFLAGS").unwrap())
                .env_remove("_DO_THE_TEST")
                .args(&env::args_os().skip(1).collect::<Vec<_>>())
                .status()
                .unwrap()
                .code()
                .unwrap_or(1),
        );
    }

    if let Ok(s) = env::var("TEST_ADDR") {
        let mut contents = Vec::new();
        t!(t!(TcpStream::connect(&s)).read_to_end(&mut contents));
        return;
    }

    let c = t!(Client::new(1));
    let td = tempfile::tempdir().unwrap();

    let prog = env::var("MAKE").unwrap_or_else(|_| "make".to_string());

    let me = t!(env::current_exe());
    let me = me.to_str().unwrap();

    let mut cmd = Command::new(&me);
    cmd.current_dir(td.path());
    cmd.env("MAKE", prog);
    cmd.env("_DO_THE_TEST", "1");

    t!(t!(File::create(td.path().join("Makefile"))).write_all(
        format!(
            "\
all: foo bar
foo:
\t{0}
bar:
\t{0}
",
            me
        )
        .as_bytes()
    ));

    // We're leaking one extra token to `make` sort of violating the makefile
    // jobserver protocol. It has the desired effect though.
    c.configure(&mut cmd);

    let listener = t!(TcpListener::bind("127.0.0.1:0"));
    let addr = t!(listener.local_addr());
    cmd.env("TEST_ADDR", addr.to_string());
    let mut child = t!(cmd.spawn());

    // We should get both connections as the two programs should be run
    // concurrently.
    let a = t!(listener.accept());
    let b = t!(listener.accept());
    drop((a, b));

    assert!(t!(child.wait()).success());
}
