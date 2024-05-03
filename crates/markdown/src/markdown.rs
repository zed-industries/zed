use gpui::HighlightStyle;

pub struct Markdown {
    parsed: Option<ParsedMarkdown>,
    pending_parse: Option<Task<()>>,
}

struct ParsedMarkdown {
    text: String,
    blocks: Vec<MarkdownBlock>,
    highlights: Vec<(Range<usize>, HighlightStyle)>,
}

enum MarkdownBlock {
    Heading { level: u8, range: Range<usize> },
    Text(Range<usize>),
    Code(Range<usize>),
}

impl Markdown {
    pub fn new(text: String) -> Self {
        Self {
            text,
            blocks: Vec::new(),
            highlights: Vec::new(),
        }
    }

    pub fn push_str(&mut self, text: &str) {}
}
