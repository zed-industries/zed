mod html_minifier;
pub(crate) mod html_parser;
mod html_rendering;

use html5ever::{ParseOpts, parse_document, tendril::TendrilSink};
use markup5ever_rcdom::RcDom;
use std::io::{self, Read};

#[inline(never)]
fn parse_html(mut reader: &mut dyn Read) -> io::Result<RcDom> {
    parse_document(RcDom::default(), ParseOpts::default())
        .from_utf8()
        .read_from(&mut reader)
}
