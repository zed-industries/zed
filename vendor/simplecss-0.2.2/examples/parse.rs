// Copyright 2019 the SimpleCSS Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Parse

use std::io::{Read, Write};

fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 2 {
        println!("Usage:\n\tparse style.css\n\tparse - 'p {{ color:red }}'");
        std::process::exit(1);
    }

    std::env::set_var("RUST_LOG", "simplecss=warn");
    env_logger::builder()
        .format(|buf, record| writeln!(buf, "{}: {}", record.level(), record.args()))
        .init();

    let text = if args[1] == "-" {
        let mut buffer = String::new();
        let stdin = std::io::stdin();
        let mut handle = stdin.lock();
        handle.read_to_string(&mut buffer).unwrap();
        buffer
    } else {
        std::fs::read_to_string(&args[1]).unwrap()
    };

    let style = simplecss::StyleSheet::parse(&text);
    println!("{:#?}", style);
}
