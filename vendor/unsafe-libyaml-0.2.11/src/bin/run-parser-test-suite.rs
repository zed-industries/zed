#![warn(clippy::pedantic)]
#![allow(
    clippy::cast_lossless,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss,
    clippy::items_after_statements,
    clippy::let_underscore_untyped,
    clippy::missing_errors_doc,
    clippy::missing_safety_doc,
    clippy::too_many_lines
)]

mod cstr;

use self::cstr::CStr;
use std::env;
use std::error::Error;
use std::ffi::c_void;
use std::fmt::Write as _;
use std::fs::File;
use std::io::{self, Read, Write};
use std::mem::MaybeUninit;
use std::process::{self, ExitCode};
use std::ptr::addr_of_mut;
use std::slice;
use unsafe_libyaml::{
    yaml_event_delete, yaml_event_t, yaml_event_type_t, yaml_parser_delete, yaml_parser_initialize,
    yaml_parser_parse, yaml_parser_set_input, yaml_parser_t, YAML_ALIAS_EVENT,
    YAML_DOCUMENT_END_EVENT, YAML_DOCUMENT_START_EVENT, YAML_DOUBLE_QUOTED_SCALAR_STYLE,
    YAML_FOLDED_SCALAR_STYLE, YAML_LITERAL_SCALAR_STYLE, YAML_MAPPING_END_EVENT,
    YAML_MAPPING_START_EVENT, YAML_NO_EVENT, YAML_PLAIN_SCALAR_STYLE, YAML_SCALAR_EVENT,
    YAML_SEQUENCE_END_EVENT, YAML_SEQUENCE_START_EVENT, YAML_SINGLE_QUOTED_SCALAR_STYLE,
    YAML_STREAM_END_EVENT, YAML_STREAM_START_EVENT,
};

pub(crate) unsafe fn unsafe_main(
    mut stdin: &mut dyn Read,
    stdout: &mut dyn Write,
) -> Result<(), Box<dyn Error>> {
    let mut parser = MaybeUninit::<yaml_parser_t>::uninit();
    let parser = parser.as_mut_ptr();
    if yaml_parser_initialize(parser).fail {
        return Err("Could not initialize the parser object".into());
    }

    unsafe fn read_from_stdio(
        data: *mut c_void,
        buffer: *mut u8,
        size: u64,
        size_read: *mut u64,
    ) -> i32 {
        let stdin: *mut &mut dyn Read = data.cast();
        let slice = slice::from_raw_parts_mut(buffer.cast(), size as usize);
        match (*stdin).read(slice) {
            Ok(n) => {
                *size_read = n as u64;
                1
            }
            Err(_) => 0,
        }
    }

    yaml_parser_set_input(parser, read_from_stdio, addr_of_mut!(stdin).cast());

    let mut event = MaybeUninit::<yaml_event_t>::uninit();
    let event = event.as_mut_ptr();
    loop {
        if yaml_parser_parse(parser, event).fail {
            let mut error = format!("Parse error: {}", CStr::from_ptr((*parser).problem));
            if (*parser).problem_mark.line != 0 || (*parser).problem_mark.column != 0 {
                let _ = write!(
                    error,
                    "\nLine: {} Column: {}",
                    ((*parser).problem_mark.line).wrapping_add(1_u64),
                    ((*parser).problem_mark.column).wrapping_add(1_u64),
                );
            }
            yaml_parser_delete(parser);
            return Err(error.into());
        }

        let type_: yaml_event_type_t = (*event).type_;
        if type_ == YAML_NO_EVENT {
            let _ = writeln!(stdout, "???");
        } else if type_ == YAML_STREAM_START_EVENT {
            let _ = writeln!(stdout, "+STR");
        } else if type_ == YAML_STREAM_END_EVENT {
            let _ = writeln!(stdout, "-STR");
        } else if type_ == YAML_DOCUMENT_START_EVENT {
            let _ = write!(stdout, "+DOC");
            if !(*event).data.document_start.implicit {
                let _ = write!(stdout, " ---");
            }
            let _ = writeln!(stdout);
        } else if type_ == YAML_DOCUMENT_END_EVENT {
            let _ = write!(stdout, "-DOC");
            if !(*event).data.document_end.implicit {
                let _ = write!(stdout, " ...");
            }
            let _ = writeln!(stdout);
        } else if type_ == YAML_MAPPING_START_EVENT {
            let _ = write!(stdout, "+MAP");
            if !(*event).data.mapping_start.anchor.is_null() {
                let _ = write!(
                    stdout,
                    " &{}",
                    CStr::from_ptr((*event).data.mapping_start.anchor as *const i8),
                );
            }
            if !(*event).data.mapping_start.tag.is_null() {
                let _ = write!(
                    stdout,
                    " <{}>",
                    CStr::from_ptr((*event).data.mapping_start.tag as *const i8),
                );
            }
            let _ = writeln!(stdout);
        } else if type_ == YAML_MAPPING_END_EVENT {
            let _ = writeln!(stdout, "-MAP");
        } else if type_ == YAML_SEQUENCE_START_EVENT {
            let _ = write!(stdout, "+SEQ");
            if !(*event).data.sequence_start.anchor.is_null() {
                let _ = write!(
                    stdout,
                    " &{}",
                    CStr::from_ptr((*event).data.sequence_start.anchor as *const i8),
                );
            }
            if !(*event).data.sequence_start.tag.is_null() {
                let _ = write!(
                    stdout,
                    " <{}>",
                    CStr::from_ptr((*event).data.sequence_start.tag as *const i8),
                );
            }
            let _ = writeln!(stdout);
        } else if type_ == YAML_SEQUENCE_END_EVENT {
            let _ = writeln!(stdout, "-SEQ");
        } else if type_ == YAML_SCALAR_EVENT {
            let _ = write!(stdout, "=VAL");
            if !(*event).data.scalar.anchor.is_null() {
                let _ = write!(
                    stdout,
                    " &{}",
                    CStr::from_ptr((*event).data.scalar.anchor as *const i8),
                );
            }
            if !(*event).data.scalar.tag.is_null() {
                let _ = write!(
                    stdout,
                    " <{}>",
                    CStr::from_ptr((*event).data.scalar.tag as *const i8),
                );
            }
            let _ = stdout.write_all(match (*event).data.scalar.style {
                YAML_PLAIN_SCALAR_STYLE => b" :",
                YAML_SINGLE_QUOTED_SCALAR_STYLE => b" '",
                YAML_DOUBLE_QUOTED_SCALAR_STYLE => b" \"",
                YAML_LITERAL_SCALAR_STYLE => b" |",
                YAML_FOLDED_SCALAR_STYLE => b" >",
                _ => process::abort(),
            });
            print_escaped(
                stdout,
                (*event).data.scalar.value,
                (*event).data.scalar.length,
            );
            let _ = writeln!(stdout);
        } else if type_ == YAML_ALIAS_EVENT {
            let _ = writeln!(
                stdout,
                "=ALI *{}",
                CStr::from_ptr((*event).data.alias.anchor as *const i8),
            );
        } else {
            process::abort();
        }

        yaml_event_delete(event);
        if type_ == YAML_STREAM_END_EVENT {
            break;
        }
    }
    yaml_parser_delete(parser);
    Ok(())
}

unsafe fn print_escaped(stdout: &mut dyn Write, mut str: *mut u8, length: u64) {
    let end = str.offset(length as isize);
    while str < end {
        let repr = match &*str {
            b'\\' => b"\\\\",
            b'\0' => b"\\0",
            b'\x08' => b"\\b",
            b'\n' => b"\\n",
            b'\r' => b"\\r",
            b'\t' => b"\\t",
            c => slice::from_ref(c),
        };
        let _ = stdout.write_all(repr);
        str = str.offset(1);
    }
}

fn main() -> ExitCode {
    let args = env::args_os().skip(1);
    if args.len() == 0 {
        let _ = writeln!(io::stderr(), "Usage: run-parser-test-suite <in.yaml>...");
        return ExitCode::FAILURE;
    }
    for arg in args {
        let mut stdin = File::open(arg).unwrap();
        let mut stdout = io::stdout();
        let result = unsafe { unsafe_main(&mut stdin, &mut stdout) };
        if let Err(err) = result {
            let _ = writeln!(io::stderr(), "{}", err);
            return ExitCode::FAILURE;
        }
    }
    ExitCode::SUCCESS
}
