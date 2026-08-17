// Copyright 2014-2017 The html5ever Project Developers. See the
// COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::interface::{LimitedQuirks, NoQuirks, Quirks, QuirksMode};
use crate::tendril::StrTendril;
use crate::tokenizer::Doctype;

// These should all be lowercase, for ASCII-case-insensitive matching.
static QUIRKY_PUBLIC_PREFIXES: &[&str] = &[
    "-//advasoft ltd//dtd html 3.0 aswedit + extensions//",
    "-//as//dtd html 3.0 aswedit + extensions//",
    "-//ietf//dtd html 2.0 level 1//",
    "-//ietf//dtd html 2.0 level 2//",
    "-//ietf//dtd html 2.0 strict level 1//",
    "-//ietf//dtd html 2.0 strict level 2//",
    "-//ietf//dtd html 2.0 strict//",
    "-//ietf//dtd html 2.0//",
    "-//ietf//dtd html 2.1e//",
    "-//ietf//dtd html 3.0//",
    "-//ietf//dtd html 3.2 final//",
    "-//ietf//dtd html 3.2//",
    "-//ietf//dtd html 3//",
    "-//ietf//dtd html level 0//",
    "-//ietf//dtd html level 1//",
    "-//ietf//dtd html level 2//",
    "-//ietf//dtd html level 3//",
    "-//ietf//dtd html strict level 0//",
    "-//ietf//dtd html strict level 1//",
    "-//ietf//dtd html strict level 2//",
    "-//ietf//dtd html strict level 3//",
    "-//ietf//dtd html strict//",
    "-//ietf//dtd html//",
    "-//metrius//dtd metrius presentational//",
    "-//microsoft//dtd internet explorer 2.0 html strict//",
    "-//microsoft//dtd internet explorer 2.0 html//",
    "-//microsoft//dtd internet explorer 2.0 tables//",
    "-//microsoft//dtd internet explorer 3.0 html strict//",
    "-//microsoft//dtd internet explorer 3.0 html//",
    "-//microsoft//dtd internet explorer 3.0 tables//",
    "-//netscape comm. corp.//dtd html//",
    "-//netscape comm. corp.//dtd strict html//",
    "-//o'reilly and associates//dtd html 2.0//",
    "-//o'reilly and associates//dtd html extended 1.0//",
    "-//o'reilly and associates//dtd html extended relaxed 1.0//",
    "-//softquad software//dtd hotmetal pro 6.0::19990601::extensions to html 4.0//",
    "-//softquad//dtd hotmetal pro 4.0::19971010::extensions to html 4.0//",
    "-//spyglass//dtd html 2.0 extended//",
    "-//sq//dtd html 2.0 hotmetal + extensions//",
    "-//sun microsystems corp.//dtd hotjava html//",
    "-//sun microsystems corp.//dtd hotjava strict html//",
    "-//w3c//dtd html 3 1995-03-24//",
    "-//w3c//dtd html 3.2 draft//",
    "-//w3c//dtd html 3.2 final//",
    "-//w3c//dtd html 3.2//",
    "-//w3c//dtd html 3.2s draft//",
    "-//w3c//dtd html 4.0 frameset//",
    "-//w3c//dtd html 4.0 transitional//",
    "-//w3c//dtd html experimental 19960712//",
    "-//w3c//dtd html experimental 970421//",
    "-//w3c//dtd w3 html//",
    "-//w3o//dtd w3 html 3.0//",
    "-//webtechs//dtd mozilla html 2.0//",
    "-//webtechs//dtd mozilla html//",
];

static QUIRKY_PUBLIC_MATCHES: &[&str] = &[
    "-//w3o//dtd w3 html strict 3.0//en//",
    "-/w3c/dtd html 4.0 transitional/en",
    "html",
];

static QUIRKY_SYSTEM_MATCHES: &[&str] =
    &["http://www.ibm.com/data/dtd/v11/ibmxhtml1-transitional.dtd"];

static LIMITED_QUIRKY_PUBLIC_PREFIXES: &[&str] = &[
    "-//w3c//dtd xhtml 1.0 frameset//",
    "-//w3c//dtd xhtml 1.0 transitional//",
];

static HTML4_PUBLIC_PREFIXES: &[&str] = &[
    "-//w3c//dtd html 4.01 frameset//",
    "-//w3c//dtd html 4.01 transitional//",
];

pub(crate) fn doctype_error_and_quirks(
    doctype: &Doctype,
    iframe_srcdoc: bool,
) -> (bool, QuirksMode) {
    fn opt_string_as_slice(x: &Option<String>) -> Option<&str> {
        x.as_deref()
    }

    fn opt_tendril_as_slice(x: &Option<StrTendril>) -> Option<&str> {
        x.as_deref()
    }

    fn opt_to_ascii_lower(x: Option<&str>) -> Option<String> {
        x.map(|y| y.to_ascii_lowercase())
    }

    let name = opt_tendril_as_slice(&doctype.name);
    let public = opt_tendril_as_slice(&doctype.public_id);
    let system = opt_tendril_as_slice(&doctype.system_id);

    let err = !matches!(
        (name, public, system),
        (Some("html"), None, None)
            | (Some("html"), None, Some("about:legacy-compat"))
            | (Some("html"), Some("-//W3C//DTD HTML 4.0//EN"), None)
            | (
                Some("html"),
                Some("-//W3C//DTD HTML 4.0//EN"),
                Some("http://www.w3.org/TR/REC-html40/strict.dtd"),
            )
            | (Some("html"), Some("-//W3C//DTD HTML 4.01//EN"), None)
            | (
                Some("html"),
                Some("-//W3C//DTD HTML 4.01//EN"),
                Some("http://www.w3.org/TR/html4/strict.dtd"),
            )
            | (
                Some("html"),
                Some("-//W3C//DTD XHTML 1.0 Strict//EN"),
                Some("http://www.w3.org/TR/xhtml1/DTD/xhtml1-strict.dtd"),
            )
            | (
                Some("html"),
                Some("-//W3C//DTD XHTML 1.1//EN"),
                Some("http://www.w3.org/TR/xhtml11/DTD/xhtml11.dtd"),
            )
    );

    // FIXME: We could do something asymptotically faster here.
    // But there aren't many strings, and this happens at most once per parse.
    fn contains_pfx(haystack: &[&str], needle: &str) -> bool {
        haystack.iter().any(|&x| needle.starts_with(x))
    }

    // Quirks-mode matches are case-insensitive.
    let public = opt_to_ascii_lower(public);
    let system = opt_to_ascii_lower(system);

    let quirk = match (opt_string_as_slice(&public), opt_string_as_slice(&system)) {
        _ if doctype.force_quirks => Quirks,
        _ if name != Some("html") => Quirks,

        _ if iframe_srcdoc => NoQuirks,

        (Some(ref p), _) if QUIRKY_PUBLIC_MATCHES.contains(p) => Quirks,
        (_, Some(ref s)) if QUIRKY_SYSTEM_MATCHES.contains(s) => Quirks,

        (Some(p), _) if contains_pfx(QUIRKY_PUBLIC_PREFIXES, p) => Quirks,
        (Some(p), _) if contains_pfx(LIMITED_QUIRKY_PUBLIC_PREFIXES, p) => LimitedQuirks,

        (Some(p), s) if contains_pfx(HTML4_PUBLIC_PREFIXES, p) => match s {
            None => Quirks,
            Some(_) => LimitedQuirks,
        },

        _ => NoQuirks,
    };

    (err, quirk)
}
