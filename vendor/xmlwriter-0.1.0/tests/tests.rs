use std::str;
use xmlwriter::{XmlWriter, Options};

#[derive(Clone, Copy, PartialEq)]
struct TStr<'a>(pub &'a str);

macro_rules! text_eq {
    ($result:expr, $expected:expr) => { assert_eq!($result, $expected) };
}


#[test]
fn write_element_01() {
    let mut w = XmlWriter::new(Options::default());
    w.start_element("svg");
    w.end_element();
    text_eq!(w.end_document(), "<svg/>\n");
}

#[test]
fn write_element_02() {
    let mut w = XmlWriter::new(Options::default());
    w.start_element("svg");
    w.start_element("rect");
    w.end_element();
    w.end_element();
    text_eq!(w.end_document(),
"<svg>
    <rect/>
</svg>
");
}

#[test]
fn write_element_03() {
    let mut w = XmlWriter::new(Options::default());
    w.start_element("svg");
    w.end_element();
    w.end_element(); // Should not panic.
    text_eq!(w.end_document(), "<svg/>\n");
}

#[test]
fn write_element_05() {
    let mut w = XmlWriter::new(Options::default());
    w.start_element("svg");
    // end_document() will call `close_element` automatically.
    text_eq!(w.end_document(), "<svg/>\n");
}

#[test]
fn write_element_06() {
    let mut w = XmlWriter::new(Options::default());
    w.start_element("svg");
    w.start_element("rect");
    w.start_element("rect");
    w.start_element("rect");
    w.start_element("rect");
    w.start_element("rect");
    text_eq!(w.end_document(),
"<svg>
    <rect>
        <rect>
            <rect>
                <rect>
                    <rect/>
                </rect>
            </rect>
        </rect>
    </rect>
</svg>
");
}

#[test]
#[should_panic]
fn write_attribute_01() {
    let mut w = XmlWriter::new(Options::default());
    // must be used only after write_element
    w.write_attribute("id", "q");
}

#[test]
fn write_attribute_02() {
    let mut w = XmlWriter::new(Options::default());
    w.start_element("svg");
    w.write_attribute("id", "q");
    w.end_element();
    text_eq!(w.end_document(), "<svg id=\"q\"/>\n");
}

#[test]
fn write_attribute_03() {
    let mut w = XmlWriter::new(Options::default());
    w.start_element("svg");
    w.write_attribute("id", "\"");
    w.end_element();
    text_eq!(w.end_document(), "<svg id=\"&quot;\"/>\n");
}

#[test]
fn write_attribute_04() {
    let opt = Options {
        use_single_quote: true,
        .. Options::default()
    };

    let mut w = XmlWriter::new(opt);
    w.start_element("svg");
    w.write_attribute("id", "'");
    w.end_element();
    text_eq!(w.end_document(), "<svg id='&apos;'/>\n");
}

#[test]
fn write_attribute_05() {
    let opt = Options {
        use_single_quote: true,
        .. Options::default()
    };

    let mut w = XmlWriter::new(opt);
    w.start_element("svg");
    w.write_attribute("id", "'''''");
    w.end_element();
    text_eq!(w.end_document(), "<svg id='&apos;&apos;&apos;&apos;&apos;'/>\n");
}

#[test]
fn write_attribute_06() {
    let opt = Options {
        use_single_quote: true,
        .. Options::default()
    };

    let mut w = XmlWriter::new(opt);
    w.start_element("svg");
    w.write_attribute("id", "'text'");
    w.end_element();
    text_eq!(w.end_document(), "<svg id='&apos;text&apos;'/>\n");
}

#[test]
fn write_attribute_07() {
    let mut w = XmlWriter::new(Options::default());
    w.start_element("svg");
    // TODO: looks we need specialization to remove &
    w.write_attribute("x", &5);
    w.end_element();
    text_eq!(w.end_document(), "<svg x=\"5\"/>\n");
}

#[test]
fn write_declaration_01() {
    let mut w = XmlWriter::new(Options::default());
    w.write_declaration();
    text_eq!(w.end_document(),
             "<?xml version=\"1.0\" encoding=\"UTF-8\" standalone=\"no\"?>\n");
}

#[test]
#[should_panic]
fn write_declaration_02() {
    let mut w = XmlWriter::new(Options::default());
    w.write_declaration();
    w.write_declaration();  // declaration must be written once
}

#[test]
#[should_panic]
fn write_declaration_03() {
    let mut w = XmlWriter::new(Options::default());
    w.write_comment("test");
    w.write_declaration(); // declaration must be written first
}

#[test]
fn write_single_quote_01() {
    let opt = Options {
        use_single_quote: true,
        .. Options::default()
    };

    let mut w = XmlWriter::new(opt);
    w.write_declaration();
    text_eq!(w.end_document(), "<?xml version='1.0' encoding='UTF-8' standalone='no'?>\n");
}

#[test]
fn write_single_quote_02() {
    let opt = Options {
        use_single_quote: true,
        .. Options::default()
    };

    let mut w = XmlWriter::new(opt);
    w.start_element("p");
    w.write_attribute("a", "b");
    w.end_element();
    text_eq!(w.end_document(), "<p a='b'/>\n");
}

#[test]
fn write_comment_01() {
    let mut w = XmlWriter::new(Options::default());
    w.write_comment("test");
    w.start_element("svg");
    text_eq!(w.end_document(),
"<!--test-->
<svg/>
");
}

#[test]
fn write_comment_02() {
    let mut w = XmlWriter::new(Options::default());
    w.start_element("svg");
    w.write_comment("test");
    text_eq!(w.end_document(),
"<svg>
    <!--test-->
</svg>
");
}

#[test]
fn write_comment_03() {
    let mut w = XmlWriter::new(Options::default());
    w.write_comment("test");
    w.start_element("svg");
    w.write_comment("test");
    text_eq!(w.end_document(),
"<!--test-->
<svg>
    <!--test-->
</svg>
");
}

#[test]
fn write_comment_04() {
    let mut w = XmlWriter::new(Options::default());
    w.write_comment("test");
    w.start_element("svg");
    w.start_element("rect");
    w.write_comment("test");
    text_eq!(w.end_document(),
"<!--test-->
<svg>
    <rect>
        <!--test-->
    </rect>
</svg>
");
}

#[test]
fn write_comment_05() {
    let mut w = XmlWriter::new(Options::default());
    w.write_comment("test");
    w.start_element("svg");
    w.write_comment("test");
    w.start_element("rect");
    w.end_element();
    text_eq!(w.end_document(),
"<!--test-->
<svg>
    <!--test-->
    <rect/>
</svg>
");
}

#[test]
fn write_comment_06() {
    let mut w = XmlWriter::new(Options::default());
    w.write_comment("test");
    w.start_element("svg");
    w.start_element("rect");
    w.end_element();
    w.write_comment("test");
    text_eq!(w.end_document(),
"<!--test-->
<svg>
    <rect/>
    <!--test-->
</svg>
");
}

#[test]
fn write_comment_07() {
    let mut w = XmlWriter::new(Options::default());
    w.start_element("svg");
    w.end_element();
    w.write_comment("test");
    text_eq!(w.end_document(),
"<svg/>
<!--test-->
");
}

#[test]
fn write_comment_08() {
    let mut w = XmlWriter::new(Options::default());
    w.write_comment("test");
    w.write_comment("test");
    w.write_comment("test");
    text_eq!(w.end_document(),
"<!--test-->
<!--test-->
<!--test-->
");
}

#[test]
#[should_panic]
fn write_text_01() {
    let mut w = XmlWriter::new(Options::default());
    w.write_text("text"); // Should be called after start_element()
}

#[test]
#[should_panic]
fn write_text_02() {
    let mut w = XmlWriter::new(Options::default());
    w.write_text("text"); // Should be called after start_element()
}

#[test]
#[should_panic]
fn write_text_03() {
    let mut w = XmlWriter::new(Options::default());
    w.start_element("p");
    w.end_element();
    w.write_text("text"); // Should be called after start_element()
}

#[test]
fn write_text_04() {
    let mut w = XmlWriter::new(Options::default());
    w.start_element("p");
    w.write_text("text");
    w.write_text("text");
    text_eq!(w.end_document(),
"<p>
    text
    text
</p>
");
}

#[test]
fn write_text_05() {
    let mut w = XmlWriter::new(Options::default());
    w.start_element("p");
    w.write_text("text");
    text_eq!(w.end_document(),
"<p>
    text
</p>
");
}

#[test]
fn write_text_06() {
    let mut w = XmlWriter::new(Options::default());
    w.start_element("p");
    w.write_text("text");
    w.start_element("p");
    w.write_text("text");
    text_eq!(w.end_document(),
"<p>
    text
    <p>
        text
    </p>
</p>
");
}

#[test]
fn write_text_07() {
    let mut w = XmlWriter::new(Options::default());
    w.start_element("div");
    w.start_element("p");
    w.write_text("text");
    w.start_element("p");
    w.write_text("text");
    text_eq!(w.end_document(),
"<div>
    <p>
        text
        <p>
            text
        </p>
    </p>
</div>
");
}

#[test]
fn write_text_08() {
    let mut w = XmlWriter::new(Options::default());
    w.start_element("p");
    w.write_text("<");
    text_eq!(w.end_document(),
"<p>
    &lt;
</p>
");
}

#[test]
fn write_text_09() {
    let mut w = XmlWriter::new(Options::default());
    w.start_element("p");
    w.write_text("<&>");
    text_eq!(w.end_document(),
"<p>
    &lt;&>
</p>
");
}

#[test]
fn write_text_10() {
    let mut w = XmlWriter::new(Options::default());
    w.start_element("p");
    w.write_text("&lt;");
    text_eq!(w.end_document(),
"<p>
    &lt;
</p>
");
}

#[test]
fn write_text_11() {
    let mut w = XmlWriter::new(Options::default());
    w.start_element("p");
    w.write_text("text");
    w.start_element("p");
    w.end_element();
    w.write_text("text");
    text_eq!(w.end_document(),
"<p>
    text
    <p/>
    text
</p>
");
}

#[test]
fn write_preserve_text_01() {
    let mut w = XmlWriter::new(Options::default());
    w.set_preserve_whitespaces(true);
    w.start_element("p");
    w.write_text("text");
    w.start_element("p");
    w.end_element();
    w.write_text("text");
    text_eq!(w.end_document(),
"<p>text<p/>text</p>");
}

#[test]
fn write_preserve_text_02() {
    let mut w = XmlWriter::new(Options::default());
    w.start_element("p");
    w.start_element("p");
    w.set_preserve_whitespaces(true);
    w.write_text("text");
    w.start_element("p");
    w.end_element();
    w.write_text("text");
    w.end_element();
    w.set_preserve_whitespaces(false);
    text_eq!(w.end_document(),
"<p>
    <p>text<p/>text</p>
</p>
");
}

#[test]
fn attrs_indent_01() {
    let opt = Options {
        attributes_indent: xmlwriter::Indent::Spaces(2),
        .. Options::default()
    };

    let mut w = XmlWriter::new(opt);
    w.start_element("rect");
    w.write_attribute("x", "5");
    w.start_element("rect");
    w.write_attribute("x", "10");
    w.write_attribute("y", "15");
    text_eq!(w.end_document(),
"<rect
  x=\"5\">
    <rect
      x=\"10\"
      y=\"15\"/>
</rect>
");
}
