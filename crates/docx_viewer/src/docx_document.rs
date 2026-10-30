use std::collections::HashMap;
use std::io::{Cursor, Read as _};

use anyhow::{Context as _, Result};
use quick_xml::events::{BytesStart, Event};
use quick_xml::{Decoder, Reader, XmlVersion};

/// The style of one text run inside a DOCX paragraph.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct RunProps {
    pub bold: bool,
    pub italic: bool,
    pub underline: bool,
    pub strikethrough: bool,
}

impl RunProps {
    fn is_plain(&self) -> bool {
        !self.bold && !self.italic && !self.underline && !self.strikethrough
    }

    fn merge(&mut self, other: &RunProps) {
        self.bold |= other.bold;
        self.italic |= other.italic;
        self.underline |= other.underline;
        self.strikethrough |= other.strikethrough;
    }
}

/// One styled stretch of text. Adjacent runs with equal props are merged by the parser.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Run {
    pub text: String,
    pub props: RunProps,
}

/// One body-level element of a DOCX document.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Block {
    Heading {
        level: u8,
        runs: Vec<Run>,
    },
    Paragraph {
        runs: Vec<Run>,
    },
    ListItem {
        indent: u8,
        ordered: bool,
        marker: String,
        runs: Vec<Run>,
    },
    Table {
        rows: Vec<Vec<TableCell>>,
    },
}

/// A table cell holds the runs of each of its paragraphs.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TableCell {
    pub paragraphs: Vec<Vec<Run>>,
}

/// A parsed DOCX document: a sequence of flow blocks with word-level statistics.
///
/// This is intentionally a reading model, not a layout model: DOCX pagination is a
/// word-processor concern and no attempt is made to reproduce page geometry.
#[derive(Debug)]
pub struct DocxDocument {
    pub blocks: Vec<Block>,
    pub word_count: usize,
}

impl DocxDocument {
    pub fn parse(content: Vec<u8>) -> Result<Self> {
        let mut archive = zip::ZipArchive::new(Cursor::new(content))
            .context("reading DOCX container (not a valid zip archive)")?;
        let document_xml = read_entry(&mut archive, "word/document.xml")?
            .context("file is not a DOCX document (word/document.xml is missing)")?;
        let styles = read_entry(&mut archive, "word/styles.xml")?
            .map(|xml| parse_styles(&xml))
            .unwrap_or_default();
        let numbering = read_entry(&mut archive, "word/numbering.xml")?
            .map(|xml| parse_numbering(&xml))
            .unwrap_or_default();

        let mut parser = DocumentParser::new(&document_xml, styles, numbering);
        let mut blocks = parser.parse_blocks()?;

        // Padding paragraphs at the document edges carry no information.
        while blocks.last().is_some_and(block_is_empty) {
            blocks.pop();
        }
        let first_content = blocks.iter().position(|block| !block_is_empty(block));
        match first_content {
            Some(index) => {
                blocks.drain(..index);
            }
            None => blocks.clear(),
        }
        let word_count = blocks.iter().map(block_word_count).sum();
        Ok(Self { blocks, word_count })
    }

    /// The document text with its structure flattened to lines, for copying to the clipboard.
    pub fn extract_text(&self) -> String {
        let mut lines = Vec::new();
        for block in &self.blocks {
            match block {
                Block::Heading { runs, .. } | Block::Paragraph { runs } => {
                    lines.push(runs_text(runs));
                }
                Block::ListItem {
                    indent,
                    marker,
                    runs,
                    ..
                } => {
                    let mut line = "    ".repeat(*indent as usize);
                    line.push_str(marker);
                    line.push(' ');
                    line.push_str(&runs_text(runs));
                    lines.push(line);
                }
                Block::Table { rows } => {
                    for row in rows {
                        let cells = row
                            .iter()
                            .map(|cell| {
                                cell.paragraphs
                                    .iter()
                                    .map(|runs| runs_text(runs))
                                    .collect::<Vec<_>>()
                                    .join(" ")
                            })
                            .collect::<Vec<_>>();
                        lines.push(cells.join(" | "));
                    }
                }
            }
        }
        lines.join("\n")
    }
}

fn block_is_empty(block: &Block) -> bool {
    match block {
        Block::Heading { runs, .. } | Block::Paragraph { runs } => {
            runs_text(runs).trim().is_empty()
        }
        _ => false,
    }
}

fn block_word_count(block: &Block) -> usize {
    fn runs_words(runs: &[Run]) -> usize {
        runs.iter()
            .map(|run| run.text.split_whitespace().count())
            .sum()
    }
    match block {
        Block::Heading { runs, .. } | Block::Paragraph { runs } | Block::ListItem { runs, .. } => {
            runs_words(runs)
        }
        Block::Table { rows } => rows
            .iter()
            .flatten()
            .flat_map(|cell| cell.paragraphs.iter())
            .map(|runs| runs_words(runs))
            .sum(),
    }
}

fn runs_text(runs: &[Run]) -> String {
    let mut text = String::new();
    for run in runs {
        text.push_str(&run.text);
    }
    text
}

const LIST_BULLETS: [&str; 3] = ["\u{2022}", "\u{25e6}", "\u{25aa}"];

/// Character/paragraph style information needed to classify and style runs.
#[derive(Default)]
struct Styles {
    /// style id -> heading level, for paragraph styles named "heading N".
    heading_levels: HashMap<String, u8>,
    /// style id -> run properties, for character styles such as Strong or Emphasis.
    run_props: HashMap<String, RunProps>,
}

/// Numbering definitions needed to label list items.
#[derive(Default)]
struct Numbering {
    /// num id -> abstract num id
    abstract_for_num: HashMap<u32, u32>,
    /// (abstract num id, level) -> whether the level is a numbered sequence
    ordered: HashMap<(u32, u8), bool>,
}

/// Reads one entry of the DOCX zip container; missing entries yield `None`.
fn read_entry(
    archive: &mut zip::ZipArchive<Cursor<Vec<u8>>>,
    name: &str,
) -> Result<Option<String>> {
    let mut entry = match archive.by_name(name) {
        Ok(entry) => entry,
        Err(zip::result::ZipError::FileNotFound) => return Ok(None),
        Err(error) => return Err(error.into()),
    };
    let mut xml = String::new();
    entry
        .read_to_string(&mut xml)
        .with_context(|| format!("reading {name}"))?;
    Ok(Some(xml))
}

/// The heading level for a style id or style name like "Heading1" or "heading 2", if any.
fn heading_level(style_id_or_name: &str) -> Option<u8> {
    let normalized: String = style_id_or_name
        .trim()
        .to_ascii_lowercase()
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect();
    normalized
        .strip_prefix("heading")?
        .parse::<u8>()
        .ok()
        .filter(|level| (1..=6).contains(level))
}

fn parse_styles(xml: &str) -> Styles {
    let mut styles = Styles::default();
    let mut reader = Reader::from_reader(xml.as_bytes());
    /// (style id, style name, accumulated run props)
    struct StyleInProgress {
        id: String,
        name: Option<String>,
        props: RunProps,
    }
    let mut current_style: Option<StyleInProgress> = None;
    let mut in_rpr = false;
    loop {
        match reader.read_event() {
            Ok(Event::Start(element)) | Ok(Event::Empty(element)) => {
                match element.local_name().as_ref() {
                    b"style" => {
                        if let Some(id) = attr_string(&element, b"styleId", reader.decoder()) {
                            current_style = Some(StyleInProgress {
                                id,
                                name: None,
                                props: RunProps::default(),
                            });
                        }
                    }
                    b"name" => {
                        if let Some(style) = &mut current_style {
                            style.name = attr_string(&element, b"val", reader.decoder());
                        }
                    }
                    b"rPr" => in_rpr = true,
                    b"b" | b"i" | b"u" | b"strike" if in_rpr => {
                        if let Some(style) = &mut current_style {
                            apply_flag(&mut style.props, &element, reader.decoder());
                        }
                    }
                    _ => {}
                }
            }
            Ok(Event::End(element)) => match element.local_name().as_ref() {
                b"rPr" => in_rpr = false,
                b"style" => {
                    if let Some(style) = current_style.take() {
                        let level = style
                            .name
                            .as_deref()
                            .and_then(heading_level)
                            .or_else(|| heading_level(&style.id));
                        if let Some(level) = level {
                            styles.heading_levels.insert(style.id.clone(), level);
                        }
                        if !style.props.is_plain() {
                            styles.run_props.insert(style.id, style.props);
                        }
                    }
                }
                _ => {}
            },
            Ok(Event::Eof) | Err(_) => break,
            _ => {}
        }
    }
    styles
}

fn parse_numbering(xml: &str) -> Numbering {
    let mut numbering = Numbering::default();
    let mut reader = Reader::from_reader(xml.as_bytes());
    let mut current_num: Option<u32> = None;
    let mut current_abstract: Option<u32> = None;
    let mut current_level: Option<u8> = None;
    loop {
        match reader.read_event() {
            Ok(Event::Start(element)) | Ok(Event::Empty(element)) => {
                match element.local_name().as_ref() {
                    b"num" => current_num = attr_u32(&element, b"numId", reader.decoder()),
                    b"abstractNum" => {
                        current_abstract = attr_u32(&element, b"abstractNumId", reader.decoder());
                        current_level = None;
                    }
                    b"abstractNumId" => {
                        if let (Some(num), Some(abstract_num)) =
                            (current_num, attr_u32(&element, b"val", reader.decoder()))
                        {
                            numbering.abstract_for_num.insert(num, abstract_num);
                        }
                    }
                    b"lvl" => {
                        current_level =
                            attr_u32(&element, b"ilvl", reader.decoder()).map(|v| v as u8)
                    }
                    b"numFmt" => {
                        if let (Some(abstract_num), Some(level)) = (current_abstract, current_level)
                        {
                            let format = attr_string(&element, b"val", reader.decoder());
                            numbering
                                .ordered
                                .insert((abstract_num, level), format.as_deref() != Some("bullet"));
                        }
                    }
                    _ => {}
                }
            }
            Ok(Event::End(element)) => match element.local_name().as_ref() {
                b"num" => current_num = None,
                b"abstractNum" => {
                    current_abstract = None;
                    current_level = None;
                }
                b"lvl" => current_level = None,
                _ => {}
            },
            Ok(Event::Eof) | Err(_) => break,
            _ => {}
        }
    }
    numbering
}

fn attr_string(element: &BytesStart<'_>, local_name: &[u8], decoder: Decoder) -> Option<String> {
    for attr in element.attributes().flatten() {
        if attr.key.local_name().as_ref() == local_name {
            return attr
                .decoded_and_normalized_value(XmlVersion::Implicit1_0, decoder)
                .ok()
                .map(|value| value.into_owned());
        }
    }
    None
}

fn attr_u32(element: &BytesStart<'_>, local_name: &[u8], decoder: Decoder) -> Option<u32> {
    attr_string(element, local_name, decoder)?.parse().ok()
}

/// Applies a run-property flag element (b/i/u/strike) to the accumulated properties.
/// `<w:b/>` enables the flag; `<w:b w:val="false"/>` disables it.
fn apply_flag(props: &mut RunProps, element: &BytesStart<'_>, decoder: Decoder) {
    let enabled = !matches!(
        attr_string(element, b"val", decoder).as_deref(),
        Some("false") | Some("0") | Some("none") | Some("off")
    );
    match element.local_name().as_ref() {
        b"b" => props.bold = enabled,
        b"i" => props.italic = enabled,
        b"u" => props.underline = enabled,
        b"strike" => props.strikethrough = enabled,
        _ => {}
    }
}

struct RunInProgress {
    props: RunProps,
    style_id: Option<String>,
    text: String,
}

struct ParagraphState {
    style_id: Option<String>,
    num_pr: Option<(u8, u32)>,
    runs: Vec<Run>,
}

struct TableState {
    rows: Vec<Vec<TableCell>>,
    row: Vec<TableCell>,
    cell_paragraphs: Vec<Vec<Run>>,
    in_cell: bool,
}

struct DocumentParser<'a> {
    reader: Reader<&'a [u8]>,
    styles: Styles,
    numbering: Numbering,
    blocks: Vec<Block>,
    paragraph: Option<ParagraphState>,
    run: Option<RunInProgress>,
    in_run_rpr: bool,
    table: Option<TableState>,
    /// (num id, level) -> counter for numbered sequences
    list_counters: HashMap<(u32, u8), u32>,
}

impl<'a> DocumentParser<'a> {
    fn new(xml: &'a str, styles: Styles, numbering: Numbering) -> Self {
        Self {
            reader: Reader::from_reader(xml.as_bytes()),
            styles,
            numbering,
            blocks: Vec::new(),
            paragraph: None,
            run: None,
            in_run_rpr: false,
            table: None,
            list_counters: HashMap::new(),
        }
    }

    fn parse_blocks(&mut self) -> Result<Vec<Block>> {
        loop {
            match self.reader.read_event() {
                Ok(Event::Start(element)) => self.start_element(&element),
                Ok(Event::Empty(element)) => self.start_element(&element),
                Ok(Event::End(element)) => self.end_element(element.local_name().as_ref()),
                Ok(Event::Text(text)) => self.text(&text.decode()?),
                Ok(Event::CData(text)) => self.text(&text.decode()?),
                Ok(Event::GeneralRef(reference)) => self.entity_reference(&reference.decode()?),
                Ok(Event::Eof) => break,
                Err(error) => return Err(error).context("parsing word/document.xml"),
                _ => {}
            }
        }
        Ok(std::mem::take(&mut self.blocks))
    }

    /// Plain text content. Entity references arrive as `Event::GeneralRef`, so the
    /// text needs no unescaping.
    fn text(&mut self, text: &str) {
        self.push_text(text);
    }

    /// An entity or character reference like `&amp;` or `&#169;`; the reader supplies
    /// it without the delimiters. OOXML documents use no DTDs, so only the predefined
    /// XML entities and numeric references can occur.
    fn entity_reference(&mut self, reference: &str) {
        let resolved = match reference {
            "amp" => Some("&".to_string()),
            "lt" => Some("<".to_string()),
            "gt" => Some(">".to_string()),
            "quot" => Some("\"".to_string()),
            "apos" => Some("'".to_string()),
            _ => reference
                .strip_prefix("#x")
                .and_then(|hex| u32::from_str_radix(hex, 16).ok())
                .or_else(|| {
                    reference
                        .strip_prefix("#")
                        .and_then(|decimal| decimal.parse::<u32>().ok())
                })
                .and_then(char::from_u32)
                .map(|c| c.to_string()),
        };
        if let Some(text) = resolved {
            self.push_text(&text);
        }
    }

    fn start_element(&mut self, element: &BytesStart<'_>) {
        match element.local_name().as_ref() {
            b"p" => {
                self.paragraph = Some(ParagraphState {
                    style_id: None,
                    num_pr: None,
                    runs: Vec::new(),
                });
            }
            b"r" => {
                self.run = Some(RunInProgress {
                    props: RunProps::default(),
                    style_id: None,
                    text: String::new(),
                });
            }
            b"rPr" if self.run.is_some() => self.in_run_rpr = true,
            b"rStyle" if self.in_run_rpr => {
                if let Some(run) = &mut self.run {
                    run.style_id = attr_string(element, b"val", self.reader.decoder());
                }
            }
            b"b" | b"i" | b"u" | b"strike" if self.in_run_rpr => {
                if let Some(run) = &mut self.run {
                    apply_flag(&mut run.props, element, self.reader.decoder());
                }
            }
            b"pStyle" => {
                if let Some(paragraph) = &mut self.paragraph {
                    paragraph.style_id = attr_string(element, b"val", self.reader.decoder());
                }
            }
            b"ilvl" => {
                if let Some(paragraph) = &mut self.paragraph
                    && let Some(level) = attr_u32(element, b"val", self.reader.decoder())
                {
                    paragraph.num_pr.get_or_insert((0, 0)).0 = level as u8;
                }
            }
            b"numId" => {
                if let Some(paragraph) = &mut self.paragraph
                    && let Some(num_id) = attr_u32(element, b"val", self.reader.decoder())
                {
                    paragraph.num_pr.get_or_insert((0, 0)).1 = num_id;
                }
            }
            b"tab" => self.push_text("\t"),
            b"br" | b"cr" => self.push_text("\n"),
            b"noBreakHyphen" => self.push_text("\u{2011}"),
            b"softHyphen" => self.push_text("\u{00ad}"),
            b"tbl" => {
                if self.table.is_none() {
                    self.table = Some(TableState {
                        rows: Vec::new(),
                        row: Vec::new(),
                        cell_paragraphs: Vec::new(),
                        in_cell: false,
                    });
                }
            }
            b"tc" => {
                if let Some(table) = &mut self.table {
                    table.in_cell = true;
                    table.cell_paragraphs = Vec::new();
                }
            }
            _ => {}
        }
    }

    fn push_text(&mut self, text: &str) {
        if let Some(run) = &mut self.run {
            run.text.push_str(text);
        }
    }

    fn end_element(&mut self, local_name: &[u8]) {
        match local_name {
            b"r" => {
                if let Some(mut run) = self.run.take() {
                    if let Some(style_id) = &run.style_id
                        && let Some(style_props) = self.styles.run_props.get(style_id)
                    {
                        let mut props = *style_props;
                        props.merge(&run.props);
                        run.props = props;
                    }
                    if let Some(paragraph) = &mut self.paragraph {
                        push_merged(&mut paragraph.runs, run.props, run.text);
                    }
                }
            }
            b"rPr" => self.in_run_rpr = false,
            b"p" => {
                if let Some(paragraph) = self.paragraph.take() {
                    if let Some(table) = &mut self.table
                        && table.in_cell
                    {
                        table.cell_paragraphs.push(paragraph.runs);
                        return;
                    }
                    self.finish_paragraph(paragraph);
                }
            }
            b"tc" => {
                if let Some(table) = &mut self.table {
                    table.in_cell = false;
                    let paragraphs = std::mem::take(&mut table.cell_paragraphs);
                    table.row.push(TableCell { paragraphs });
                }
            }
            b"tr" => {
                if let Some(table) = &mut self.table {
                    let row = std::mem::take(&mut table.row);
                    table.rows.push(row);
                }
            }
            b"tbl" => {
                if let Some(table) = self.table.take() {
                    self.blocks.push(Block::Table { rows: table.rows });
                }
            }
            _ => {}
        }
    }

    fn finish_paragraph(&mut self, paragraph: ParagraphState) {
        let runs = paragraph.runs;
        if runs_text(&runs).trim().is_empty() {
            // Collapse runs of empty paragraphs into a single spacer paragraph.
            if !self.blocks.last().is_some_and(block_is_empty) {
                self.blocks.push(Block::Paragraph { runs: Vec::new() });
            }
            return;
        }
        let heading = paragraph.style_id.as_deref().and_then(|style_id| {
            self.styles
                .heading_levels
                .get(style_id)
                .copied()
                .or_else(|| heading_level(style_id))
        });
        if let Some(level) = heading {
            self.blocks.push(Block::Heading { level, runs });
            return;
        }
        if let Some((indent, num_id)) = paragraph.num_pr {
            let abstract_num = self.numbering.abstract_for_num.get(&num_id).copied();
            let ordered = abstract_num
                .and_then(|abstract_num| self.numbering.ordered.get(&(abstract_num, indent)))
                .copied()
                .unwrap_or(false);
            let marker = if ordered {
                let counter = self.list_counters.entry((num_id, indent)).or_insert(0);
                *counter += 1;
                format!("{counter}.")
            } else {
                LIST_BULLETS[indent as usize % LIST_BULLETS.len()].to_string()
            };
            self.blocks.push(Block::ListItem {
                indent,
                ordered,
                marker,
                runs,
            });
            return;
        }
        self.blocks.push(Block::Paragraph { runs });
    }
}

/// Appends a run to a run list, merging with the previous run when the props match.
fn push_merged(runs: &mut Vec<Run>, props: RunProps, text: String) {
    if text.is_empty() {
        return;
    }
    if let Some(last) = runs.last_mut()
        && last.props == props
    {
        last.text.push_str(&text);
        return;
    }
    runs.push(Run { text, props });
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    /// Builds a minimal DOCX container holding the given archive entries.
    pub(crate) fn build_docx(parts: &[(&str, &str)]) -> Vec<u8> {
        use std::io::Write as _;
        let mut writer = zip::ZipWriter::new(Cursor::new(Vec::new()));
        for (name, content) in parts {
            writer
                .start_file(*name, zip::write::FileOptions::default())
                .expect("starting zip entry");
            writer
                .write_all(content.as_bytes())
                .expect("writing zip entry");
        }
        writer.finish().expect("finishing zip").into_inner()
    }

    pub(crate) fn document_xml(body: &str) -> String {
        format!(
            r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
  <w:body>{body}</w:body>
</w:document>"#
        )
    }

    fn parse_document_xml(body: &str) -> DocxDocument {
        DocxDocument::parse(build_docx(&[("word/document.xml", &document_xml(body))]))
            .expect("document should parse")
    }

    #[test]
    fn test_parse_rejects_invalid_data() {
        let error = DocxDocument::parse(b"not a docx".to_vec()).unwrap_err();
        assert!(format!("{error:#}").contains("zip"));

        let empty_archive = build_docx(&[]);
        let error = DocxDocument::parse(empty_archive).unwrap_err();
        assert!(format!("{error:#}").contains("word/document.xml"));
    }

    #[test]
    fn test_parse_paragraphs_and_run_props() {
        let document = parse_document_xml(
            r#"
<w:p><w:r><w:t>Hello </w:t><w:t>world</w:t></w:r></w:p>
<w:p>
  <w:r><w:rPr><w:b/></w:rPr><w:t>bold </w:t></w:r>
  <w:r><w:rPr><w:i/></w:rPr><w:t>italic </w:t></w:r>
  <w:r><w:rPr><w:u/></w:rPr><w:t>under</w:t></w:r>
  <w:r><w:rPr><w:strike w:val="true"/></w:rPr><w:t> struck</w:t></w:r>
  <w:r><w:rPr><w:b w:val="false"/></w:rPr><w:t> plain</w:t></w:r>
</w:p>"#,
        );
        assert_eq!(
            document.extract_text(),
            "Hello world\nbold italic under struck plain"
        );
        assert_eq!(document.word_count, 7);
        let [first, second] = &document.blocks[..] else {
            panic!("unexpected blocks: {:?}", document.blocks);
        };
        assert_eq!(
            first,
            &Block::Paragraph {
                runs: vec![Run {
                    text: "Hello world".into(),
                    props: RunProps::default(),
                }],
            }
        );
        let Block::Paragraph { runs } = second else {
            panic!("unexpected block: {second:?}");
        };
        let styles: Vec<(bool, bool, bool, bool)> = runs
            .iter()
            .map(|run| {
                (
                    run.props.bold,
                    run.props.italic,
                    run.props.underline,
                    run.props.strikethrough,
                )
            })
            .collect();
        assert_eq!(
            styles,
            [
                (true, false, false, false),
                (false, true, false, false),
                (false, false, true, false),
                (false, false, false, true),
                (false, false, false, false),
            ]
        );
    }

    #[test]
    fn test_parse_headings_from_style_id() {
        let document = parse_document_xml(
            r#"
<w:p><w:pPr><w:pStyle w:val="Heading1"/></w:pPr><w:r><w:t>Title</w:t></w:r></w:p>
<w:p><w:pPr><w:pStyle w:val="Heading3"/></w:pPr><w:r><w:t>Deep</w:t></w:r></w:p>
<w:p><w:r><w:t>Body</w:t></w:r></w:p>"#,
        );
        match &document.blocks[..] {
            [
                Block::Heading { level: 1, runs },
                Block::Heading { level: 3, .. },
                Block::Paragraph { .. },
            ] => {
                assert_eq!(runs_text(runs), "Title");
                assert_eq!(document.word_count, 3);
            }
            other => panic!("unexpected blocks: {other:?}"),
        }
    }

    #[test]
    fn test_parse_headings_from_styles_xml() {
        let styles = r#"<?xml version="1.0" encoding="UTF-8"?>
<w:styles xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
  <w:style w:type="paragraph" w:styleId="FooTitle">
    <w:name w:val="heading 2"/>
  </w:style>
</w:styles>"#;
        let document = document_xml(
            r#"<w:p><w:pPr><w:pStyle w:val="FooTitle"/></w:pPr><w:r><w:t>Named</w:t></w:r></w:p>"#,
        );
        let parsed = DocxDocument::parse(build_docx(&[
            ("word/document.xml", &document),
            ("word/styles.xml", styles),
        ]))
        .expect("document should parse");
        match &parsed.blocks[..] {
            [Block::Heading { level: 2, runs }] => assert_eq!(runs_text(runs), "Named"),
            other => panic!("unexpected blocks: {other:?}"),
        }
    }

    #[test]
    fn test_character_styles_from_styles_xml() {
        let styles = r#"<?xml version="1.0" encoding="UTF-8"?>
<w:styles xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
  <w:style w:type="character" w:styleId="Strong">
    <w:name w:val="Strong"/>
    <w:rPr><w:b/></w:rPr>
  </w:style>
</w:styles>"#;
        let document = document_xml(
            r#"<w:p><w:r><w:rPr><w:rStyle w:val="Strong"/></w:rPr><w:t>strong text</w:t></w:r></w:p>"#,
        );
        let parsed = DocxDocument::parse(build_docx(&[
            ("word/document.xml", &document),
            ("word/styles.xml", styles),
        ]))
        .expect("document should parse");
        match &parsed.blocks[..] {
            [Block::Paragraph { runs }] => {
                assert_eq!(runs.len(), 1);
                assert_eq!(runs[0].text, "strong text");
                assert!(runs[0].props.bold);
            }
            other => panic!("unexpected blocks: {other:?}"),
        }
    }

    #[test]
    fn test_parse_lists_with_numbering() {
        let numbering = r#"<?xml version="1.0" encoding="UTF-8"?>
<w:numbering xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
  <w:abstractNum w:abstractNumId="1">
    <w:lvl w:ilvl="0"><w:numFmt w:val="bullet"/></w:lvl>
  </w:abstractNum>
  <w:abstractNum w:abstractNumId="2">
    <w:lvl w:ilvl="0"><w:numFmt w:val="decimal"/></w:lvl>
  </w:abstractNum>
  <w:num w:numId="7"><w:abstractNumId w:val="1"/></w:num>
  <w:num w:numId="8"><w:abstractNumId w:val="2"/></w:num>
</w:numbering>"#;
        let document = document_xml(
            r#"
<w:p><w:pPr><w:numPr><w:ilvl w:val="0"/><w:numId w:val="7"/></w:numPr></w:pPr><w:r><w:t>bullet one</w:t></w:r></w:p>
<w:p><w:pPr><w:numPr><w:ilvl w:val="0"/><w:numId w:val="8"/></w:numPr></w:pPr><w:r><w:t>first</w:t></w:r></w:p>
<w:p><w:pPr><w:numPr><w:ilvl w:val="0"/><w:numId w:val="8"/></w:numPr></w:pPr><w:r><w:t>second</w:t></w:r></w:p>"#,
        );
        let parsed = DocxDocument::parse(build_docx(&[
            ("word/document.xml", &document),
            ("word/numbering.xml", numbering),
        ]))
        .expect("document should parse");
        let markers: Vec<(String, bool)> = parsed
            .blocks
            .iter()
            .filter_map(|block| match block {
                Block::ListItem {
                    marker, ordered, ..
                } => Some((marker.clone(), *ordered)),
                _ => None,
            })
            .collect();
        assert_eq!(markers.len(), 3);
        assert_eq!(markers[0], ("\u{2022}".to_string(), false));
        assert_eq!(markers[1], ("1.".to_string(), true));
        assert_eq!(markers[2], ("2.".to_string(), true));
    }

    #[test]
    fn test_parse_table() {
        let document = parse_document_xml(
            r#"
<w:tbl>
  <w:tr>
    <w:tc><w:p><w:r><w:t>a1</w:t></w:r></w:p></w:tc>
    <w:tc><w:p><w:r><w:t>b1</w:t></w:r></w:p></w:tc>
  </w:tr>
  <w:tr>
    <w:tc><w:p><w:r><w:t>b2</w:t></w:r></w:p></w:tc>
  </w:tr>
</w:tbl>"#,
        );
        match &document.blocks[..] {
            [Block::Table { rows }] => {
                assert_eq!(rows.len(), 2);
                assert_eq!(runs_text(&rows[0][0].paragraphs[0]), "a1");
                assert_eq!(runs_text(&rows[1][0].paragraphs[0]), "b2");
            }
            other => panic!("unexpected blocks: {other:?}"),
        }
        assert_eq!(document.extract_text(), "a1 | b1\nb2");
    }

    #[test]
    fn test_entities_and_breaks() {
        let document = parse_document_xml(
            r#"<w:p><w:r><w:t>fish &amp; chips</w:t><w:br/><w:t>second line</w:t></w:r></w:p>"#,
        );
        match &document.blocks[..] {
            [Block::Paragraph { runs }] => {
                assert_eq!(runs_text(runs), "fish & chips\nsecond line")
            }
            other => panic!("unexpected blocks: {other:?}"),
        }
    }

    #[test]
    fn test_padding_paragraphs_are_dropped() {
        let document = parse_document_xml(
            r#"
<w:p/>
<w:p><w:r><w:t></w:t></w:r></w:p>
<w:p><w:r><w:t>real</w:t></w:r></w:p>
<w:p/>
<w:p/>"#,
        );
        assert_eq!(
            document.blocks,
            vec![Block::Paragraph {
                runs: vec![Run {
                    text: "real".into(),
                    props: RunProps::default(),
                }],
            }]
        );
    }
}
