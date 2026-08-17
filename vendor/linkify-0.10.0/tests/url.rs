mod common;

use crate::common::assert_linked_with;
use linkify::{LinkFinder, LinkKind};

#[test]
fn no_links() {
    assert_not_linked("");
    assert_not_linked("foo");
    assert_not_linked(":");
    assert_not_linked("://");
    assert_not_linked(":::");
}

#[test]
fn schemes() {
    assert_not_linked("://foo");
    assert_not_linked("1://foo");
    assert_not_linked("123://foo");
    assert_not_linked("+://foo");
    assert_not_linked("-://foo");
    assert_not_linked(".://foo");
    assert_not_linked("1abc://foo");
    assert_linked("a://foo", "|a://foo|");
    assert_linked("a123://foo", "|a123://foo|");
    assert_linked("a123b://foo", "|a123b://foo|");
    assert_linked("a+b://foo", "|a+b://foo|");
    assert_linked("a-b://foo", "|a-b://foo|");
    assert_linked("a.b://foo", "|a.b://foo|");
    assert_linked("ABC://foo", "|ABC://foo|");
    assert_linked(".http://example.org/", ".|http://example.org/|");
    assert_linked("1.http://example.org/", "1.|http://example.org/|");
}

#[test]
fn authority() {
    assert_not_linked("ab://");
    assert_not_linked("file://");
    assert_not_linked("file:// ");
    assert_not_linked("\"file://\"");
    assert_not_linked("\"file://...\", ");
    assert_linked("file://somefile", "|file://somefile|");
    assert_linked("file://../relative", "|file://../relative|");
    assert_linked("http://a.", "|http://a|.");
}

#[test]
fn local_links() {
    assert_linked("http://127.0.0.1", "|http://127.0.0.1|");
    assert_linked("http://127.0.0.1/", "|http://127.0.0.1/|");
}

#[test]
fn single_links() {
    assert_linked("ab://c", "|ab://c|");
    assert_linked("http://example.org/", "|http://example.org/|");
    assert_linked("http://example.org/123", "|http://example.org/123|");
    assert_linked(
        "http://example.org/?foo=test&bar=123",
        "|http://example.org/?foo=test&bar=123|",
    );
    assert_linked(
        "http://example.org/?foo=%20",
        "|http://example.org/?foo=%20|",
    );
    assert_linked("http://example.org/%3C", "|http://example.org/%3C|");
}

#[test]
fn single_links_without_protocol() {
    assert_urls_without_protocol("example.org/", "|example.org/|");
    assert_urls_without_protocol("example.org/123", "|example.org/123|");
    assert_urls_without_protocol(
        "example.org/?foo=test&bar=123",
        "|example.org/?foo=test&bar=123|",
    );
    assert_urls_without_protocol("example.org/?foo=%20", "|example.org/?foo=%20|");
    assert_urls_without_protocol("example.org/%3C", "|example.org/%3C|");
}

#[test]
fn space_characters_stop_url() {
    assert_linked("foo http://example.org/", "foo |http://example.org/|");
    assert_linked("http://example.org/ bar", "|http://example.org/| bar");
    assert_linked("http://example.org/\tbar", "|http://example.org/|\tbar");
    assert_linked("http://example.org/\nbar", "|http://example.org/|\nbar");
    assert_linked(
        "http://example.org/\u{0B}bar",
        "|http://example.org/|\u{0B}bar",
    );
    assert_linked(
        "http://example.org/\u{0C}bar",
        "|http://example.org/|\u{0C}bar",
    );
    assert_linked("http://example.org/\rbar", "|http://example.org/|\rbar");
}

#[test]
fn space_characters_stop_url_without_protocol() {
    assert_urls_without_protocol("foo example.org/", "foo |example.org/|");
    assert_urls_without_protocol("example.org/ bar", "|example.org/| bar");
    assert_urls_without_protocol("example.org/\tbar", "|example.org/|\tbar");
    assert_urls_without_protocol("example.org/\nbar", "|example.org/|\nbar");
    assert_urls_without_protocol("example.org/\u{0B}bar", "|example.org/|\u{0B}bar");
    assert_urls_without_protocol("example.org/\u{0C}bar", "|example.org/|\u{0C}bar");
    assert_urls_without_protocol("example.org/\rbar", "|example.org/|\rbar");
}

#[test]
fn illegal_characters_stop_url() {
    assert_linked("http://example.org/<", "|http://example.org/|<");
    assert_linked("http://example.org/>", "|http://example.org/|>");
    assert_linked("http://example.org/<>", "|http://example.org/|<>");
    assert_linked("http://example.org/\u{00}", "|http://example.org/|\u{00}");
    assert_linked("http://example.org/\u{0E}", "|http://example.org/|\u{0E}");
    assert_linked("http://example.org/\u{7F}", "|http://example.org/|\u{7F}");
    assert_linked("http://example.org/\u{9F}", "|http://example.org/|\u{9F}");
    assert_linked("http://example.org/foo|bar", "|http://example.org/foo||bar");
}

#[test]
fn illegal_characters_stop_url_without_protocol() {
    assert_urls_without_protocol("example.org/<", "|example.org/|<");
    assert_urls_without_protocol("example.org/>", "|example.org/|>");
    assert_urls_without_protocol("example.org/<>", "|example.org/|<>");
    assert_urls_without_protocol("example.org/\u{00}", "|example.org/|\u{00}");
    assert_urls_without_protocol("example.org/\u{0E}", "|example.org/|\u{0E}");
    assert_urls_without_protocol("example.org/\u{7F}", "|example.org/|\u{7F}");
    assert_urls_without_protocol("example.org/\u{9F}", "|example.org/|\u{9F}");
}

#[test]
fn delimiter_at_end() {
    assert_linked("http://example.org/.", "|http://example.org/|.");
    assert_linked("http://example.org/..", "|http://example.org/|..");
    assert_linked("http://example.org/,", "|http://example.org/|,");
    assert_linked("http://example.org/:", "|http://example.org/|:");
    assert_linked("http://example.org/?", "|http://example.org/|?");
    assert_linked("http://example.org/!", "|http://example.org/|!");
    assert_linked("http://example.org/;", "|http://example.org/|;");
}

#[test]
fn delimiter_at_end_no_protocol() {
    assert_urls_without_protocol("example.org/.", "|example.org/|.");
    assert_urls_without_protocol("example.org/..", "|example.org/|..");
    assert_urls_without_protocol("example.org/,", "|example.org/|,");
    assert_urls_without_protocol("example.org/:", "|example.org/|:");
    assert_urls_without_protocol("example.org/?", "|example.org/|?");
    assert_urls_without_protocol("example.org/!", "|example.org/|!");
    assert_urls_without_protocol("example.org/;", "|example.org/|;");
}

#[test]
fn matching_punctuation() {
    assert_linked("http://example.org/a(b)", "|http://example.org/a(b)|");
    assert_linked("http://example.org/a[b]", "|http://example.org/a[b]|");
    assert_linked("http://example.org/a{b}", "|http://example.org/a{b}|");
    assert_linked("http://example.org/a'b'", "|http://example.org/a'b'|");
    assert_linked("(http://example.org/)", "(|http://example.org/|)");
    assert_linked("[http://example.org/]", "[|http://example.org/|]");
    assert_linked("{http://example.org/}", "{|http://example.org/|}");
    assert_linked("\"http://example.org/\"", "\"|http://example.org/|\"");
    assert_linked("'http://example.org/'", "'|http://example.org/|'");
}
#[test]
fn matching_punctuation_without_protocol() {
    assert_urls_without_protocol("example.org/a(b)", "|example.org/a(b)|");
    assert_urls_without_protocol("example.org/a[b]", "|example.org/a[b]|");
    assert_urls_without_protocol("example.org/a{b}", "|example.org/a{b}|");
    assert_urls_without_protocol("example.org/a'b'", "|example.org/a'b'|");
    assert_urls_without_protocol("(example.org/)", "(|example.org/|)");
    assert_urls_without_protocol("[example.org/]", "[|example.org/|]");
    assert_urls_without_protocol("{example.org/}", "{|example.org/|}");
    assert_urls_without_protocol("\"example.org/\"", "\"|example.org/|\"");
    assert_urls_without_protocol("'example.org/'", "'|example.org/|'");
}

#[test]
fn matching_punctuation_tricky() {
    assert_linked("((http://example.org/))", "((|http://example.org/|))");
    assert_linked(
        "((http://example.org/a(b)))",
        "((|http://example.org/a(b)|))",
    );
    assert_linked("[(http://example.org/)]", "[(|http://example.org/|)]");
    assert_linked("(http://example.org/).", "(|http://example.org/|).");
    assert_linked("(http://example.org/.)", "(|http://example.org/|.)");
    assert_linked("http://example.org/>", "|http://example.org/|>");
    // not sure about these
    assert_linked("http://example.org/(", "|http://example.org/|(");
    assert_linked("http://example.org/(.", "|http://example.org/|(.");
    assert_linked("http://example.org/]()", "|http://example.org/|]()");
}

#[test]
fn matching_punctuation_tricky_without_protocol() {
    assert_urls_without_protocol("((example.org/))", "((|example.org/|))");
    assert_urls_without_protocol("((example.org/a(b)))", "((|example.org/a(b)|))");
    assert_urls_without_protocol("[(example.org/)]", "[(|example.org/|)]");
    assert_urls_without_protocol("(example.org/).", "(|example.org/|).");
    assert_urls_without_protocol("(example.org/.)", "(|example.org/|.)");
    assert_urls_without_protocol("example.org/>", "|example.org/|>");
    // not sure about these
    assert_urls_without_protocol("example.org/(", "|example.org/|(");
    assert_urls_without_protocol("example.org/(.", "|example.org/|(.");
    assert_urls_without_protocol("example.org/]()", "|example.org/|]()");
}

#[test]
fn single_quote() {
    assert_linked("'https://example.org'", "'|https://example.org|'");
    assert_linked("\"https://example.org\"", "\"|https://example.org|\"");
    assert_linked("''https://example.org''", "''|https://example.org|''");
    assert_linked("'https://example.org''", "'|https://example.org|''");
    assert_linked("'https://example.org", "'|https://example.org|");
    assert_linked(
        "http://example.org/\'_(foo)",
        "|http://example.org/\'_(foo)|",
    );
    assert_linked(
        "http://example.org/\'_(foo)\'",
        "|http://example.org/\'_(foo)\'|",
    );
    assert_linked("http://example.org/\'\'", "|http://example.org/\'\'|");
    assert_linked("http://example.org/\'\'\'", "|http://example.org/\'\'|\'");
    assert_linked("http://example.org/\'.", "|http://example.org/|\'.");
    assert_linked("http://example.org/\'a", "|http://example.org/\'a|");
    assert_linked("http://example.org/it's", "|http://example.org/it's|");
}

#[test]
fn single_quote_without_protocol() {
    assert_urls_without_protocol("example.org/\'_(foo)", "|example.org/\'_(foo)|");
    assert_urls_without_protocol("example.org/\'_(foo)\'", "|example.org/\'_(foo)\'|");
    assert_urls_without_protocol("example.org/\'\'", "|example.org/\'\'|");
    assert_urls_without_protocol("example.org/\'\'\'", "|example.org/\'\'|\'");
    assert_urls_without_protocol("example.org/\'.", "|example.org/|\'.");
    assert_urls_without_protocol("example.org/\'a", "|example.org/\'a|");
    assert_urls_without_protocol("example.org/it's", "|example.org/it's|");
}

#[test]
fn double_quote() {
    // " not allowed in URLs
    assert_linked("http://example.org/\"a", "|http://example.org/|\"a");
    assert_linked("http://example.org/\"a\"", "|http://example.org/|\"a\"");
}

#[test]
fn grave_quote() {
    // ` not allowed in URLs
    assert_linked("http://example.org/`a", "|http://example.org/|`a");
    assert_linked("http://example.org/`a`", "|http://example.org/|`a`");
}

#[test]
fn asterisk() {
    assert_linked("https://example.org*", "|https://example.org|*");
    assert_linked("https://example.org/*", "|https://example.org/|*");
    assert_linked("https://example.org/**", "|https://example.org/|**");
    assert_linked("https://example.org/*/a", "|https://example.org/*/a|");
}

#[test]
fn grave_quote_without_protocol() {
    // ` not allowed in URLs
    assert_urls_without_protocol("example.org/`a", "|example.org/|`a");
    assert_urls_without_protocol("example.org/`a`", "|example.org/|`a`");
}

#[test]
fn html() {
    assert_linked("http://example.org\">", "|http://example.org|\">");
    assert_linked("http://example.org'>", "|http://example.org|'>");
    assert_linked("http://example.org\"/>", "|http://example.org|\"/>");
    assert_linked("http://example.org'/>", "|http://example.org|'/>");
    assert_linked("http://example.org<p>", "|http://example.org|<p>");
    assert_linked("http://example.org</p>", "|http://example.org|</p>");
}

#[test]
fn html_no_protocol() {
    assert_urls_without_protocol("example.org\">", "|example.org|\">");
    assert_urls_without_protocol("example.org'>", "|example.org|'>");
    assert_urls_without_protocol("example.org\"/>", "|example.org|\"/>");
    assert_urls_without_protocol("example.org'/>", "|example.org|'/>");
    assert_urls_without_protocol("example.org<p>", "|example.org|<p>");
    assert_urls_without_protocol("example.org</p>", "|example.org|</p>");
}

#[test]
fn css() {
    assert_linked("http://example.org\");", "|http://example.org|\");");
    assert_linked("http://example.org');", "|http://example.org|');");
}

#[test]
fn images() {
    assert_linked(
        r#"<img src="http://example.org/test.svg">"#,
        r#"<img src="|http://example.org/test.svg|">"#,
    );
}

#[test]
fn complex_html() {
    assert_linked(
        r#"<div><a href="http://example.org"></a></div>"#,
        r#"<div><a href="|http://example.org|"></a></div>"#,
    );

    assert_linked(
        r#"<div><a href="http://example.org"
        ></a></div>"#,
        r#"<div><a href="|http://example.org|"
        ></a></div>"#,
    );

    assert_linked(
        r#"<div>
       <img
         src="http://example.org/test3.jpg" />
     </div>"#,
        r#"<div>
       <img
         src="|http://example.org/test3.jpg|" />
     </div>"#,
    )
}

#[test]
fn css_without_protocol() {
    assert_urls_without_protocol("example.org\");", "|example.org|\");");
    assert_urls_without_protocol("example.org');", "|example.org|');");
}

#[test]
fn slash() {
    assert_linked("http://example.org/", "|http://example.org/|");
    assert_linked("http://example.org/a/", "|http://example.org/a/|");
    assert_linked("http://example.org//", "|http://example.org//|");
}

#[test]
fn slash_without_protocol() {
    assert_urls_without_protocol("example.org/", "|example.org/|");
    assert_urls_without_protocol("example.org/a/", "|example.org/a/|");
    assert_urls_without_protocol("example.org//", "|example.org//|");
}

#[test]
fn multiple() {
    assert_linked(
        "http://one.org/ http://two.org/",
        "|http://one.org/| |http://two.org/|",
    );
    assert_linked(
        "http://one.org/ : http://two.org/",
        "|http://one.org/| : |http://two.org/|",
    );
    assert_linked(
        "(http://one.org/)(http://two.org/)",
        "(|http://one.org/|)(|http://two.org/|)",
    );
}
#[test]
fn multiple_without_protocol() {
    assert_urls_without_protocol("one.org/ two.org/", "|one.org/| |two.org/|");
    assert_urls_without_protocol("one.org/ : two.org/", "|one.org/| : |two.org/|");
    assert_urls_without_protocol("(one.org/)(two.org/)", "(|one.org/|)(|two.org/|)");
}

#[test]
fn multiple_mixed_protocol() {
    assert_urls_without_protocol("http://one.org/ two.org/", "|http://one.org/| |two.org/|");
    assert_urls_without_protocol(
        "one.org/ : http://two.org/",
        "|one.org/| : |http://two.org/|",
    );
    assert_urls_without_protocol(
        "(http://one.org/)(two.org/)",
        "(|http://one.org/|)(|two.org/|)",
    );
}

#[test]
fn international() {
    assert_linked("http://üñîçøðé.com", "|http://üñîçøðé.com|");
    assert_linked("http://üñîçøðé.com/ä", "|http://üñîçøðé.com/ä|");
    assert_linked("http://example.org/\u{A1}", "|http://example.org/\u{A1}|");
    assert_linked("http://example.org/\u{A2}", "|http://example.org/\u{A2}|");
    assert_linked(
        "http://example.org/\u{1F600}",
        "|http://example.org/\u{1F600}|",
    );
    assert_linked("http://example.org/\u{A2}/", "|http://example.org/\u{A2}/|");
    assert_linked(
        "http://xn--c1h.example.com/",
        "|http://xn--c1h.example.com/|",
    );
}

#[test]
fn international_not_allowed() {
    let mut finder = LinkFinder::new();
    finder.url_can_be_iri(false);
    finder.url_must_have_scheme(true);
    finder.kinds(&[LinkKind::Url]);
    assert_linked_with(&finder, "http://üñîçøðé.com", "http://üñîçøðé.com");
    assert_linked_with(&finder, "http://üñîçøðé.com/ä", "http://üñîçøðé.com/ä");
    assert_linked_with(
        &finder,
        "http://example.org/\u{A1}",
        "|http://example.org/|\u{A1}",
    );
    assert_linked_with(
        &finder,
        "http://example.org/\u{A2}",
        "|http://example.org/|\u{A2}",
    );
    assert_linked_with(
        &finder,
        "http://example.org/\u{1F600}",
        "|http://example.org/|\u{1F600}",
    );
    assert_linked_with(
        &finder,
        "http://example.org/\u{A2}/",
        "|http://example.org/|\u{A2}/",
    );
    assert_linked_with(
        &finder,
        "http://xn--c1h.example.com/",
        "|http://xn--c1h.example.com/|",
    );
}

#[test]
fn international_not_allowed_without_protocol() {
    let mut finder = LinkFinder::new();
    finder.url_can_be_iri(false);
    finder.url_must_have_scheme(false);
    finder.kinds(&[LinkKind::Url]);
    assert_linked_with(&finder, "üñîçøðé.com", "üñîçøðé.com");
    assert_linked_with(&finder, "üñîçøðé.com/ä", "üñîçøðé.com/ä");
    assert_linked_with(&finder, "example.org/\u{A1}", "|example.org/|\u{A1}");
    assert_linked_with(&finder, "example.org/\u{A2}", "|example.org/|\u{A2}");
    assert_linked_with(&finder, "example.org/\u{1F600}", "|example.org/|\u{1F600}");
    assert_linked_with(&finder, "example.org/\u{A2}/", "|example.org/|\u{A2}/");
    assert_linked_with(&finder, "xn--c1h.example.com/", "|xn--c1h.example.com/|");
}

#[test]
fn international_without_protocol() {
    assert_urls_without_protocol("üñîçøðé.com", "|üñîçøðé.com|");
    assert_urls_without_protocol("üñîçøðé.com/ä", "|üñîçøðé.com/ä|");
    assert_urls_without_protocol("example.org/\u{A1}", "|example.org/\u{A1}|");
    assert_urls_without_protocol("example.org/\u{A2}", "|example.org/\u{A2}|");
    assert_urls_without_protocol("example.org/\u{1F600}", "|example.org/\u{1F600}|");
    assert_urls_without_protocol("example.org/\u{A2}/", "|example.org/\u{A2}/|");
    assert_urls_without_protocol("xn--c1h.example.com/", "|xn--c1h.example.com/|");
}

#[test]
fn domain_tld_without_protocol_must_be_long() {
    assert_urls_without_protocol("example.", "example.");
    assert_urls_without_protocol("example./", "example./");
    assert_urls_without_protocol("foo.com.", "|foo.com|.");
    assert_urls_without_protocol("example.c", "example.c");
    assert_urls_without_protocol("example.co", "|example.co|");
    assert_urls_without_protocol("example.com", "|example.com|");
    assert_urls_without_protocol("e.com", "|e.com|");
    assert_urls_without_protocol("exampl.e.c", "exampl.e.c");
    assert_urls_without_protocol("exampl.e.co", "|exampl.e.co|");
    assert_urls_without_protocol("e.xample.c", "e.xample.c");
    assert_urls_without_protocol("e.xample.co", "|e.xample.co|");
    assert_urls_without_protocol("v1.1.1", "v1.1.1");
}

#[test]
fn skip_emails_without_protocol() {
    assert_not_linked_without_protocol("foo.bar@example.org");
    assert_not_linked_without_protocol("example.com@example.com");
}

#[test]
fn avoid_multiple_matches_without_protocol() {
    let mut finder = LinkFinder::new();
    finder.url_must_have_scheme(false);
    let links: Vec<_> = finder.links("http://example.com").collect();
    assert_eq!(links.len(), 1);
    assert_eq!(links[0].as_str(), "http://example.com");
}

#[test]
fn without_protocol_and_email() {
    let mut finder = LinkFinder::new();
    finder.url_must_have_scheme(false);

    assert_linked_with(
        &finder,
        "Look, no scheme: example.org/foo email@foo.com",
        "Look, no scheme: |example.org/foo| |email@foo.com|",
    );

    assert_linked_with(
        &finder,
        "Web:
www.foobar.co
E-Mail:
      bar@foobar.co (bla bla bla)",
        "Web:
|www.foobar.co|
E-Mail:
      |bar@foobar.co| (bla bla bla)",
    );
}

#[test]
fn uri_with_empty_path_and_with_query() {
    assert_linked(
        "upi://pay?pa=XXXXXXX&pn=XXXXX",
        "|upi://pay?pa=XXXXXXX&pn=XXXXX|",
    );
    assert_linked(
        "https://example.org?pa=XXXXXXX&pn=XXXXX",
        "|https://example.org?pa=XXXXXXX&pn=XXXXX|",
    );
}

#[test]
fn fuzz() {
    assert_not_linked("ab:/ϸ");
}

fn assert_not_linked(s: &str) {
    assert_linked(s, s);
}

/// Assert link with protocol
fn assert_linked(input: &str, expected: &str) {
    let finder = LinkFinder::new();
    assert_linked_with(&finder, input, expected);
}

fn assert_not_linked_without_protocol(s: &str) {
    assert_urls_without_protocol(s, s);
}

/// Assert link without protocol
fn assert_urls_without_protocol(input: &str, expected: &str) {
    let mut finder = LinkFinder::new();
    finder.url_must_have_scheme(false);
    finder.kinds(&[LinkKind::Url]);
    assert_linked_with(&finder, input, expected);
}
