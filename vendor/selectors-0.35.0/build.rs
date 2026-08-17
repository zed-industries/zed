/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

extern crate phf_codegen;

use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

fn main() {
    let path = Path::new(&env::var_os("OUT_DIR").unwrap())
        .join("ascii_case_insensitive_html_attributes.rs");
    let mut file = BufWriter::new(File::create(&path).unwrap());

    let mut set = phf_codegen::Set::new();
    for name in ASCII_CASE_INSENSITIVE_HTML_ATTRIBUTES.split_whitespace() {
        set.entry(name);
    }
    write!(
        &mut file,
        "{{ static SET: ::phf::Set<&'static str> = {}; &SET }}",
        set.build(),
    )
    .unwrap();
}

/// <https://html.spec.whatwg.org/multipage/#selectors>
static ASCII_CASE_INSENSITIVE_HTML_ATTRIBUTES: &str = r#"
    accept
    accept-charset
    align
    alink
    axis
    bgcolor
    charset
    checked
    clear
    codetype
    color
    compact
    declare
    defer
    dir
    direction
    disabled
    enctype
    face
    frame
    hreflang
    http-equiv
    lang
    language
    link
    media
    method
    multiple
    nohref
    noresize
    noshade
    nowrap
    readonly
    rel
    rev
    rules
    scope
    scrolling
    selected
    shape
    target
    text
    type
    valign
    valuetype
    vlink
"#;
