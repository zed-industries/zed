#![forbid(unsafe_code)]

use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufReader, Read};
use std::{cmp, env};

use xml::reader::XmlEvent;
use xml::ParserConfig;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut file;
    let mut stdin;
    let source: &mut dyn Read = if let Some(file_name) = env::args().nth(1) {
        file = File::open(file_name).map_err(|e| format!("Cannot open input file: {e}"))?;
        &mut file
    } else {
        stdin = io::stdin();
        &mut stdin
    };

    let reader = ParserConfig::new()
        .whitespace_to_characters(true)
        .ignore_comments(false)
        .create_reader(BufReader::new(source));

    let mut processing_instructions = 0;
    let mut elements = 0;
    let mut character_blocks = 0;
    let mut cdata_blocks = 0;
    let mut characters = 0;
    let mut comment_blocks = 0;
    let mut comment_characters = 0;
    let mut namespaces = HashSet::new();
    let mut depth = 0;
    let mut max_depth = 0;

    for e in reader {
        let e = e.map_err(|e| format!("Error parsing XML document: {e}"))?;
        match e {
            XmlEvent::StartDocument { version, encoding, standalone } =>
                println!(
                    "XML document version {}, encoded in {}, {}standalone",
                    version, encoding, if standalone.unwrap_or(false) { "" } else { "not " }
                ),
            XmlEvent::EndDocument => println!("Document finished"),
            XmlEvent::ProcessingInstruction { .. } => processing_instructions += 1,
            XmlEvent::Whitespace(_) => {}, // can't happen due to configuration
            XmlEvent::Characters(s) => {
                character_blocks += 1;
                characters += s.len();
            },
            XmlEvent::CData(s) => {
                cdata_blocks += 1;
                characters += s.len();
            },
            XmlEvent::Comment(s) => {
                comment_blocks += 1;
                comment_characters += s.len();
            },
            XmlEvent::StartElement { namespace, .. } => {
                depth += 1;
                max_depth = cmp::max(max_depth, depth);
                elements += 1;
                namespaces.extend(namespace.0.into_values());
            },
            XmlEvent::EndElement { .. } => {
                depth -= 1;
            },
        };
    }

    namespaces.remove(xml::namespace::NS_EMPTY_URI);
    namespaces.remove(xml::namespace::NS_XMLNS_URI);
    namespaces.remove(xml::namespace::NS_XML_URI);

    println!("Elements: {elements}, maximum depth: {max_depth}");
    println!("Namespaces (excluding built-in): {}", namespaces.len());
    println!("Characters: {characters}, characters blocks: {character_blocks}, CDATA blocks: {cdata_blocks}");
    println!("Comment blocks: {comment_blocks}, comment characters: {comment_characters}");
    println!("Processing instructions (excluding built-in): {processing_instructions}");

    Ok(())
}
