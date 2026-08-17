// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// A sample application which takes a comma separated list of locales,
// makes them syntactically canonical and serializes the list back into a comma separated list.

#![no_main] // https://github.com/unicode-org/icu4x/issues/395
icu_benchmark_macros::instrument!();
use icu_benchmark_macros::println;

use std::env;

use icu_locale_core::Locale;

const DEFAULT_INPUT: &str = "sr-cyrL-rS, es-mx, und-arab-u-ca-Buddhist";

fn main() {
    for input in env::args()
        .nth(1)
        .as_deref()
        .unwrap_or(DEFAULT_INPUT)
        .split(',')
        .map(str::trim)
    {
        let output = Locale::normalize(input).unwrap();
        println!("{input} -> {output}");
    }
}
