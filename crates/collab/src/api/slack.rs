use serde::{Deserialize, Serialize};

/// https://api.slack.com/reference/messaging/payload
#[derive(Default, Clone, Serialize, Deserialize)]
pub struct WebhookBody {
    text: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    blocks: Vec<Block>,
    #[serde(skip_serializing_if = "Option::is_none")]
    thread_ts: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mrkdwn: Option<bool>,
}

impl WebhookBody {
    pub fn new(f: impl FnOnce(Self) -> Self) -> Self {
        f(Self::default())
    }

    pub fn add_section(mut self, build: impl FnOnce(Section) -> Section) -> Self {
        self.blocks.push(Block::Section(build(Section::default())));
        self
    }

    pub fn add_rich_text(mut self, build: impl FnOnce(RichText) -> RichText) -> Self {
        self.blocks
            .push(Block::RichText(build(RichText::default())));
        self
    }
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
/// https://api.slack.com/reference/block-kit/blocks
pub enum Block {
    #[serde(rename = "section")]
    Section(Section),
    #[serde(rename = "rich_text")]
    RichText(RichText),
    // .... etc.
}

/// https://api.slack.com/reference/block-kit/blocks#section
#[derive(Default, Clone, Serialize, Deserialize)]
pub struct Section {
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<Text>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    fields: Vec<Text>,
    // fields, accessories...
}

impl Section {
    pub fn text(mut self, text: Text) -> Self {
        self.text = Some(text);
        self
    }

    pub fn add_field(mut self, field: Text) -> Self {
        self.fields.push(field);
        self
    }
}

/// https://api.slack.com/reference/block-kit/composition-objects#text
#[derive(Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Text {
    #[serde(rename = "plain_text")]
    PlainText { text: String, emoji: bool },
    #[serde(rename = "mrkdwn")]
    Markdown { text: String, verbatim: bool },
}

impl Text {
    pub fn plain(s: String) -> Self {
        Self::PlainText {
            text: s,
            emoji: true,
        }
    }

    pub fn markdown(s: String) -> Self {
        Self::Markdown {
            text: s,
            verbatim: false,
        }
    }
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct RichText {
    elements: Vec<RichTextObject>,
}

impl RichText {
    pub fn new(f: impl FnOnce(Self) -> Self) -> Self {
        f(Self::default())
    }

    pub fn add_preformatted(
        mut self,
        build: impl FnOnce(RichTextPreformatted) -> RichTextPreformatted,
    ) -> Self {
        self.elements.push(RichTextObject::Preformatted(build(
            RichTextPreformatted::default(),
        )));
        self
    }
}

/// https://api.slack.com/reference/block-kit/blocks#rich_text
#[derive(Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum RichTextObject {
    #[serde(rename = "rich_text_preformatted")]
    Preformatted(RichTextPreformatted),
    // etc.
}

/// https://api.slack.com/reference/block-kit/blocks#rich_text_preformatted
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct RichTextPreformatted {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    elements: Vec<RichTextElement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    border: Option<u8>,
}

impl RichTextPreformatted {
    pub fn add_text(mut self, text: String) -> Self {
        self.elements.push(RichTextElement::Text { text });
        self
    }
}

/// https://api.slack.com/reference/block-kit/blocks#element-types
#[derive(Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum RichTextElement {
    #[serde(rename = "text")]
    Text { text: String },
    // etc.
}
