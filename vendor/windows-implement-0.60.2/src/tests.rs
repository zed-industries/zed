//! These tests are just a way to quickly run the `#[implement]` macro and see its output.
//! They don't check the output in any way.
//!
//! This exists because of some difficulties of running `cargo expand` against the `#[implement]`
//! macro. It's also just really convenient. You can see the output by using `--nocapture` and
//! you'll probably want to restrict the output to a single thread:
//!
//! ```text
//! cargo test -p windows-implement --lib -- --nocapture --test-threads=1
//! ```

use std::io::{Read, Write};
use std::process::{Command, Stdio};

use proc_macro2::TokenStream;
use quote::quote;

fn implement(attributes: TokenStream, item_tokens: TokenStream) -> String {
    let out_tokens = crate::implement_core(attributes, item_tokens);
    let tokens_string = out_tokens.to_string();

    let out_string = rustfmt(&tokens_string);
    println!("// output of #[implement] :");
    println!();
    println!("{}", out_string);
    out_string
}

fn rustfmt(input: &str) -> String {
    let mut rustfmt = Command::new("rustfmt");

    rustfmt.stdin(Stdio::piped());
    rustfmt.stdout(Stdio::piped());
    rustfmt.stderr(Stdio::inherit());

    let mut child = match rustfmt.spawn() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("failed to spawn rustfmt: {e:?}");
            return input.to_string();
        }
    };

    let mut stdout = child.stdout.take().unwrap();

    // spawn thread to read stdout
    let stdout_thread = std::thread::spawn(move || {
        let mut buf = String::new();
        stdout.read_to_string(&mut buf).unwrap();
        buf
    });

    // write unformatted into stdin
    let mut stdin = child.stdin.take().unwrap();
    stdin.write_all(input.as_bytes()).unwrap();
    drop(stdin);

    let stdout_string: String = stdout_thread.join().unwrap();

    let exit = child.wait().unwrap();
    if !exit.success() {
        eprintln!("rustfmt terminated with failure status code");
        return input.to_string();
    }

    stdout_string
}

#[test]
fn simple_type() {
    implement(
        quote!(IFoo),
        quote! {
            struct Foo {
                x: u32,
            }
        },
    );
}

#[test]
fn zero_sized_type() {
    implement(
        quote!(IFoo),
        quote! {
            struct Foo;
        },
    );
}

#[test]
fn no_interfaces() {
    implement(
        quote!(),
        quote! {
            struct Foo {}
        },
    );
}

#[test]
fn generic_no_lifetime() {
    implement(
        quote!(IAsyncOperationWithProgress<T, P>, IAsyncInfo),
        quote! {
            struct OperationWithProgress<T, P>(SyncState<IAsyncOperationWithProgress<T, P>>)
            where
                T: RuntimeType + 'static,
                P: RuntimeType + 'static;

        },
    );
}

#[test]
fn generic_with_lifetime() {
    implement(
        quote!(),
        quote! {
            pub struct Foo<'a> {
                pub x: &'a [u8],
            }
        },
    );
}

#[test]
fn tuple_type() {
    implement(
        quote!(IFoo),
        quote! {
            struct Foo(pub i32);
        },
    );
}
