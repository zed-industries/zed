// SPDX-License-Identifier: Apache-2.0

use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::os::raw::c_int;
use std::os::unix::io::{FromRawFd, RawFd};

use ciborium::{de::from_reader, value::Value};
use rand::Rng;

const ITERATIONS: usize = 128 * 1024;

#[allow(non_camel_case_types)]
type pid_t = i32;

extern "C" {
    fn close(fd: RawFd) -> c_int;
    fn fork() -> pid_t;
    fn pipe(pipefd: &mut [RawFd; 2]) -> c_int;
    fn waitpid(pid: pid_t, wstatus: *mut c_int, options: c_int) -> pid_t;
}

#[test]
fn fuzz() {
    let mut fds: [RawFd; 2] = [0; 2];
    assert_eq!(unsafe { pipe(&mut fds) }, 0);

    let pid = unsafe { fork() };
    assert!(pid >= 0);

    match pid {
        0 => {
            let mut child = unsafe { File::from_raw_fd(fds[1]) };
            unsafe { close(fds[0]) };

            let mut rng = rand::thread_rng();
            let mut buffer = [0u8; 32];

            for _ in 0..ITERATIONS {
                let len = rng.gen_range(0..buffer.len());
                rng.fill(&mut buffer[..len]);

                writeln!(child, "{}", hex::encode(&buffer[..len])).unwrap();
                writeln!(child, "{:?}", from_reader::<Value, _>(&buffer[..len])).unwrap();
            }
        }

        pid => {
            let mut parent = unsafe { File::from_raw_fd(fds[0]) };
            unsafe { close(fds[1]) };

            let mut string = String::new();
            parent.read_to_string(&mut string).unwrap();
            eprint!("{}", string);

            let mut status = 0;
            assert_eq!(pid, unsafe { waitpid(pid, &mut status, 0) });

            eprintln!("exit status: {:?}", status);
            assert_eq!(0, status);
        }
    }
}
