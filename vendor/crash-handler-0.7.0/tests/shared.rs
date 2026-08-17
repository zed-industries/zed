#![allow(unsafe_code)]

use crash_handler as ch;

pub use sadness_generator::SadnessFlavor;

pub fn handles_crash(flavor: SadnessFlavor) {
    let mut _handler = None;

    unsafe {
        _handler = Some(
            ch::CrashHandler::attach(ch::make_crash_event(move |cc: &ch::CrashContext| {
                cfg_if::cfg_if! {
                    if #[cfg(any(target_os = "linux", target_os = "android"))] {
                        use ch::Signal;

                        assert_eq!(
                            cc.siginfo.ssi_signo,
                            match flavor {
                                SadnessFlavor::Abort => Signal::Abort,
                                SadnessFlavor::Bus => Signal::Bus,
                                SadnessFlavor::DivideByZero => Signal::Fpe,
                                SadnessFlavor::Illegal => Signal::Illegal,
                                SadnessFlavor::Segfault | SadnessFlavor::StackOverflow { .. } => {
                                    Signal::Segv
                                }
                                SadnessFlavor::Trap => Signal::Trap,
                            } as u32,
                        );

                        //assert_eq!(cc.tid, tid);

                        // At least on linux these...aren't set. Which is weird
                        //assert_eq!(cc.siginfo.ssi_pid, std::process::id());
                        //assert_eq!(cc.siginfo.ssi_tid, tid as u32);
                    } else if #[cfg(target_os = "macos")] {
                        use ch::ExceptionType;

                        let exc = cc.exception.expect("we should have an exception");

                        let expected = match flavor {
                            SadnessFlavor::Abort => {
                                assert_eq!(exc.code, 0x10003); // EXC_SOFT_SIGNAL
                                assert_eq!(exc.subcode.unwrap(), libc::SIGABRT as _);

                                ExceptionType::Software
                            }
                            SadnessFlavor::Bus
                            | SadnessFlavor::Segfault
                            | SadnessFlavor::StackOverflow { .. } => {
                                if flavor == SadnessFlavor::Segfault {
                                    // For EXC_BAD_ACCESS exceptions, the subcode will be the
                                    // bad address we tried to access
                                    assert_eq!(cc.exception.unwrap().subcode.unwrap(), sadness_generator::SEGFAULT_ADDRESS as _);
                                }

                                ExceptionType::BadAccess
                            },
                            SadnessFlavor::DivideByZero => ExceptionType::Arithmetic,
                            SadnessFlavor::Illegal => ExceptionType::BadInstruction,
                            SadnessFlavor::Trap => ExceptionType::Breakpoint,
                            SadnessFlavor::Guard => ExceptionType::Guard,
                        };

                        assert_eq!(exc.kind, expected as _);
                    } else if #[cfg(target_os = "windows")] {
                        use ch::ExceptionCode;

                        let ec = match flavor {
                            SadnessFlavor::Abort => ExceptionCode::Abort,
                            SadnessFlavor::DivideByZero => ExceptionCode::Fpe,
                            SadnessFlavor::Illegal => ExceptionCode::Illegal,
                            SadnessFlavor::InvalidParameter => ExceptionCode::InvalidParameter,
                            SadnessFlavor::Purecall => ExceptionCode::Purecall,
                            SadnessFlavor::Segfault => ExceptionCode::Segv,
                            SadnessFlavor::StackOverflow { .. }=> ExceptionCode::StackOverflow,
                            SadnessFlavor::Trap => ExceptionCode::Trap,
                            SadnessFlavor::HeapCorruption => ExceptionCode::HeapCorruption,
                        };

                        assert_eq!(
                            cc.exception_code, ec as i32,
                            "0x{:x} != 0x{:x}",
                            cc.exception_code, ec as i32
                        );
                    }
                }

                // Once we've verified we've received the exception we expected,
                // we exit with a success code to satisfy cargo test. While
                // we _could_ jump back on non-mac platforms (Mac can't since
                // exceptions are handled on a different thread) this is a
                // simpler approach that is consistent across platforms since
                // we only have 1 test per binary
                #[allow(clippy::exit)]
                std::process::exit(0);
            }))
            .unwrap(),
        );

        #[inline(never)]
        unsafe fn indirect(flavor: SadnessFlavor) {
            unsafe { flavor.make_sad() };
        }

        indirect(flavor);

        panic!("this should be impossible");
    }
}
