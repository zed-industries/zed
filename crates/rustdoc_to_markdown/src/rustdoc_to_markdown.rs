use html5ever::driver::ParseOpts;
use html5ever::parse_document;
use html5ever::tendril::TendrilSink;
use html5ever::tree_builder::TreeBuilderOpts;
use markup5ever_rcdom::{Handle, NodeData, RcDom};

pub fn convert_rustdoc_to_markdown() {
    let parse_options = ParseOpts {
        tree_builder: TreeBuilderOpts {
            drop_doctype: true,
            ..Default::default()
        },
        ..Default::default()
    };
    let html = include_str!("/Users/maxdeviant/projects/zed/target/doc/gpui/index.html");

    let dom = parse_document(RcDom::default(), parse_options)
        .from_utf8()
        .read_from(&mut html.as_bytes())
        .unwrap();
    walk(&dom.document);

    dbg!(&dom.errors);
}

fn walk(input: &Handle) {
    match input.data {
        NodeData::Document | NodeData::Doctype { .. } | NodeData::ProcessingInstruction { .. } => {}
        NodeData::Comment { .. } => {}
        NodeData::Text { ref contents } => {
            let mut text = contents.borrow().to_string();
            println!("{text}");
        }
        NodeData::Element { ref name, .. } => {
            //
        }
        _ => {}
    }

    for child in input.children.borrow().iter() {
        walk(child);
    }
}
