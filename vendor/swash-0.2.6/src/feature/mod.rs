mod aat;
mod at;
mod util;

use super::internal::{self, raw_tag, Bytes, RawFont};
use super::{FontRef, Tag};
use crate::text::{Language, Script};

const DFLT: u32 = raw_tag(b"DFLT");

#[derive(Copy, Clone)]
enum Kind {
    None,
    /// GSUB, GPOS offsets
    At(u32, u32),
    /// Morx offset, kerning available
    Aat(u32, bool),
}

impl Kind {
    fn from_font(font: &FontRef) -> Self {
        let gsub = font.table_offset(raw_tag(b"GSUB"));
        let gpos = font.table_offset(raw_tag(b"GPOS"));
        if gsub != 0 || gpos != 0 {
            return Self::At(gsub, gpos);
        }
        let morx = font.table_offset(raw_tag(b"morx"));
        if morx != 0 {
            let kern = font.table_offset(raw_tag(b"kern")) != 0
                || font.table_offset(raw_tag(b"kerx")) != 0;
            return Self::Aat(morx, kern);
        }
        Self::None
    }
}

#[derive(Copy, Clone)]
enum WritingSystemsKind<'a> {
    None,
    At(at::WritingSystems<'a>),
    Aat(aat::OnceItem<'a>),
}

/// Iterator over a collection of writing systems.
#[derive(Copy, Clone)]
pub struct WritingSystems<'a> {
    kind: WritingSystemsKind<'a>,
}

impl<'a> WritingSystems<'a> {
    pub(crate) fn from_font(font: &FontRef<'a>) -> Self {
        let kind = Kind::from_font(font);
        WritingSystems {
            kind: match kind {
                Kind::At(gsub, gpos) => WritingSystemsKind::At(at::WritingSystems::new(
                    at::Scripts::new(Bytes::new(font.data), gsub, gpos),
                )),
                Kind::Aat(morx, kern) => WritingSystemsKind::Aat(Some(aat::Item {
                    chains: aat::chains(font.data, morx),
                    kern,
                })),
                _ => WritingSystemsKind::None,
            },
        }
    }
}

#[derive(Copy, Clone)]
enum WritingSystemKind<'a> {
    At(at::WritingSystem<'a>),
    Aat(aat::Item<'a>),
}

impl<'a> Iterator for WritingSystems<'a> {
    type Item = WritingSystem<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.kind {
            WritingSystemsKind::At(iter) => {
                let item = iter.next()?;
                Some(WritingSystem {
                    kind: WritingSystemKind::At(item),
                    script_tag: item.script_tag(),
                    lang_tag: item.language_tag(),
                    lang: Language::from_opentype(item.language_tag()),
                })
            }
            WritingSystemsKind::Aat(iter) => {
                let item = iter.take()?;
                Some(WritingSystem {
                    kind: WritingSystemKind::Aat(item),
                    script_tag: DFLT,
                    lang_tag: DFLT,
                    lang: None,
                })
            }
            _ => None,
        }
    }
}

/// Script, language and associated typographic features available in a font.
#[derive(Copy, Clone)]
pub struct WritingSystem<'a> {
    kind: WritingSystemKind<'a>,
    script_tag: Tag,
    lang_tag: Tag,
    lang: Option<Language>,
}

impl<'a> WritingSystem<'a> {
    /// Returns the OpenType script tag for the writing system.
    pub fn script_tag(&self) -> Tag {
        self.script_tag
    }

    /// Returns the OpenType language tag for the writing system.
    pub fn language_tag(&self) -> Tag {
        self.lang_tag
    }

    /// Returns the script for the writing system.
    pub fn script(&self) -> Option<Script> {
        Script::from_opentype(self.script_tag)
    }

    /// Returns the language for the writing system.
    pub fn language(&self) -> Option<Language> {
        self.lang
    }

    /// Returns an iterator over the features provided by the writing
    /// system.
    pub fn features(&self) -> Features<'a> {
        Features {
            kind: match self.kind {
                WritingSystemKind::At(item) => FeaturesKind::At(item.features()),
                WritingSystemKind::Aat(item) => {
                    FeaturesKind::Aat(aat::Features::new(item.chains, item.kern))
                }
            },
        }
    }
}

#[derive(Copy, Clone)]
enum FeaturesKind<'a> {
    None,
    At(at::Features<'a>),
    AtAll(at::AllFeatures<'a>),
    Aat(aat::Features<'a>),
}

/// Typographic rule that produces modifications to a sequence of glyphs.
#[derive(Copy, Clone)]
pub struct Feature {
    tag: Tag,
    name: Option<&'static str>,
    action: Action,
}

impl Feature {
    fn from_tag(tag: Tag, action: Action) -> Self {
        Self {
            tag,
            name: util::desc_from_at(tag).map(|x| x.1),
            action,
        }
    }

    /// Returns the feature tag.
    pub fn tag(&self) -> Tag {
        self.tag
    }

    /// Returns the name of the feature, if available.
    pub fn name(&self) -> Option<&'static str> {
        self.name
    }

    /// Returns the action of the feature.
    pub fn action(&self) -> Action {
        self.action
    }
}

/// Modification performed by a feature.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Action {
    /// Replaces one or more glyphs such as in ligation.
    Substitution,
    /// Attaches one glyph to another such as in accent mark placement.
    Attachment,
    /// Adjusts the position of one or more glyphs such as in kerning.
    Adjustment,
}

/// Iterator over a collection of typographic features.
#[derive(Copy, Clone)]
pub struct Features<'a> {
    kind: FeaturesKind<'a>,
}

impl<'a> Features<'a> {
    pub(crate) fn from_font(font: &FontRef<'a>) -> Self {
        let kind = Kind::from_font(font);
        Self {
            kind: match kind {
                Kind::At(gsub, gpos) => {
                    FeaturesKind::AtAll(at::AllFeatures::new(Bytes::new(font.data), gsub, gpos))
                }
                Kind::Aat(morx, kern) => {
                    FeaturesKind::Aat(aat::Features::new(aat::chains(font.data, morx), kern))
                }
                _ => FeaturesKind::None,
            },
        }
    }
}

const MARK: u32 = raw_tag(b"mark");
const MKMK: u32 = raw_tag(b"mkmk");

impl<'a> Iterator for Features<'a> {
    type Item = Feature;

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.kind {
            FeaturesKind::At(iter) => {
                let item = iter.next()?;
                let action = if item.stage == 0 {
                    Action::Substitution
                } else {
                    match item.tag {
                        MARK | MKMK => Action::Attachment,
                        _ => Action::Adjustment,
                    }
                };
                Some(Feature::from_tag(item.tag, action))
            }
            FeaturesKind::AtAll(iter) => {
                let (stage, tag) = iter.next()?;
                let action = if stage == 0 {
                    Action::Substitution
                } else {
                    match tag {
                        MARK | MKMK => Action::Attachment,
                        _ => Action::Adjustment,
                    }
                };
                Some(Feature::from_tag(tag, action))
            }
            FeaturesKind::Aat(iter) => {
                let (tag, name) = iter.next()?;
                let action = if tag == raw_tag(b"kern") {
                    Action::Adjustment
                } else {
                    Action::Substitution
                };
                Some(Feature {
                    tag,
                    name: Some(name),
                    action,
                })
            }
            _ => None,
        }
    }
}
