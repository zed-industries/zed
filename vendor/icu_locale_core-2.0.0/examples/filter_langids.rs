// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// A sample application which takes a comma separated list of language identifiers,
// filters out identifiers with language subtags different than `en` and serializes
// the list back into a comma separated list in canonical syntax.
//
// Note: This is an example of the API use, and is not a good base for language matching.
// For language matching, please consider algorithms such as Locale Matcher.

#![no_main] // https://github.com/unicode-org/icu4x/issues/395
icu_benchmark_macros::instrument!();
use icu_benchmark_macros::println;

use std::env;

use icu_locale_core::{subtags, LanguageIdentifier};

const DEFAULT_INPUT: &str =
    "de, en-us, zh-hant, sr-cyrl, fr-ca, es-cl, pl, en-latn-us, ca-valencia, und-arab";

fn main() {
    for input in env::args()
        .nth(1)
        .as_deref()
        .unwrap_or(DEFAULT_INPUT)
        .split(',')
        .map(str::trim)
    {
        // 1. Parse the input string into a language identifier.
        let Ok(langid) = LanguageIdentifier::try_from_str(input) else {
            continue;
        };
        // 2. Filter for LanguageIdentifiers with Language subtag `en`.
        if langid.language == subtags::language!("en") {
            println!("✅ {}", langid)
        } else {
            println!("❌ {}", langid)
        }
    }
}
