// For clippy.
#![allow(unknown_lints)]

extern crate cpp_demangle;

// For command line integration
extern crate clap;

use clap::Parser;
use cpp_demangle::{BorrowedSymbol, DemangleOptions};
use std::io::{self, BufRead, Cursor, Write};
use std::process;

/// Find the index of the first (potential) occurrence of a mangled C++ symbol
/// in the given `haystack`.
fn find_mangled(haystack: &[u8]) -> Option<usize> {
    if haystack.is_empty() {
        return None;
    }

    for i in 0..haystack.len() - 1 {
        if haystack[i] == b'_' {
            match (
                haystack[i + 1],
                haystack.get(i + 2),
                haystack.get(i + 3),
                haystack.get(i + 4),
            ) {
                (b'Z', _, _, _) | (b'_', Some(b'Z'), _, _) | (b'_', Some(b'_'), Some(b'Z'), _) => {
                    return Some(i)
                }
                (b'_', Some(b'_'), Some(b'_'), Some(b'Z')) => return Some(i),
                _ => (),
            }
        }
    }

    None
}

/// Print the given `line` to `out`, with all mangled C++ symbols replaced with
/// their demangled form.
fn demangle_line<W>(out: &mut W, line: &[u8], options: DemangleOptions) -> io::Result<()>
where
    W: Write,
{
    let mut line = line;

    while let Some(idx) = find_mangled(line) {
        write!(out, "{}", String::from_utf8_lossy(&line[..idx]))?;

        if let Ok((sym, tail)) = BorrowedSymbol::with_tail(&line[idx..]) {
            let demangled = sym
                .demangle(&options)
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
            write!(out, "{}", demangled)?;
            line = tail;
        } else {
            write!(out, "_Z")?;
            line = &line[2..];
        }
    }

    write!(out, "{}", String::from_utf8_lossy(line))
}

/// Print all the lines from the given `input` to `out`, with all mangled C++
/// symbols replaced with their demangled form.
fn demangle_all<R, W>(input: &mut R, out: &mut W, options: DemangleOptions) -> io::Result<()>
where
    R: BufRead,
    W: Write,
{
    let mut buf = vec![];

    while input.read_until(b'\n', &mut buf)? > 0 {
        let nl = buf.ends_with(&[b'\n']);
        if nl {
            buf.pop();
        }
        demangle_line(out, &buf[..], options)?;
        if nl {
            write!(out, "\n")?;
        }
        buf.clear();
    }

    Ok(())
}

/// A c++filt clone as an example of how to use the cpp_demangle crate!
#[derive(Parser)]
#[clap(version, author)]
struct Cli {
    #[clap(short = 'p', long)]
    /// Do not display function arguments.
    no_params: bool,
    /// Do not display function return types.
    #[clap(long)]
    no_return_type: bool,
    #[clap(long)]
    /// Hide types in template parameter expression literals
    hide_expression_literal_types: bool,
    mangled_names: Vec<String>,
}

fn main() {
    let cli = Cli::parse();

    let stdin = io::stdin();
    let mut stdin = stdin.lock();

    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    let stderr = io::stderr();
    let mut stderr = stderr.lock();

    let mut options = DemangleOptions::new();
    if cli.no_params {
        options = options.no_params();
    }
    if cli.hide_expression_literal_types {
        options = options.hide_expression_literal_types();
    }
    if cli.no_return_type {
        options = options.no_return_type();
    }

    let demangle_result = if !cli.mangled_names.is_empty() {
        let mut input = Cursor::new(cli.mangled_names.into_iter().fold(
            String::new(),
            |mut accumulated, name| {
                accumulated.push_str(&name);
                accumulated.push_str("\n");
                accumulated
            },
        ));
        demangle_all(&mut input, &mut stdout, options)
    } else {
        demangle_all(&mut stdin, &mut stdout, options)
    };

    let code = match demangle_result {
        Ok(_) => 0,
        Err(e) => {
            let _ = writeln!(&mut stderr, "error: {}", e);
            1
        }
    };

    process::exit(code);
}
