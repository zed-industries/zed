use super::super::Tag;
use super::internal::{at::*, *};
use super::util::*;

#[derive(Copy, Clone)]
pub struct Script<'a> {
    data: Bytes<'a>,
    gsub: u32,
    gpos: u32,
    gsub_offset: u32,
    gpos_offset: u32,
    tag: Tag,
}

impl<'a> Script<'a> {
    pub fn languages(&self) -> Languages<'a> {
        Languages::new(
            self.data,
            self.gsub,
            self.gpos,
            self.gsub_offset,
            self.gpos_offset,
        )
    }
}

#[derive(Copy, Clone)]
pub struct Scripts<'a> {
    data: Bytes<'a>,
    gsub: u32,
    gpos: u32,
    in_gsub: bool,
    len: u16,
    cur: u16,
    done: bool,
}

impl<'a> Scripts<'a> {
    pub fn new(data: Bytes<'a>, gsub: u32, gpos: u32) -> Self {
        Self {
            data,
            gsub,
            gpos,
            in_gsub: true,
            len: script_count(&data, gsub),
            cur: 0,
            done: false,
        }
    }

    fn get_next(&mut self) -> Option<Script<'a>> {
        if self.in_gsub {
            if self.cur < self.len {
                let index = self.cur;
                self.cur += 1;
                let (tag, offset) = script_at(&self.data, self.gsub, index)?;
                let gpos_offset = script_by_tag(&self.data, self.gpos, tag).unwrap_or(0);
                return Some(Script {
                    data: self.data,
                    gsub: self.gsub,
                    gpos: self.gpos,
                    gsub_offset: offset,
                    gpos_offset,
                    tag,
                });
            } else {
                self.in_gsub = false;
                self.cur = 0;
                self.len = script_count(&self.data, self.gpos);
            }
        } else if self.cur < self.len {
            let index = self.cur;
            self.cur += 1;
            let (tag, offset) = script_at(&self.data, self.gpos, index)?;
            if script_by_tag(&self.data, self.gsub, tag).is_some() {
                return None;
            }
            return Some(Script {
                data: self.data,
                gsub: self.gsub,
                gpos: self.gpos,
                gsub_offset: 0,
                gpos_offset: offset,
                tag,
            });
        } else {
            self.done = true;
        }
        None
    }
}

impl<'a> Iterator for Scripts<'a> {
    type Item = Script<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        while !self.done {
            let item = self.get_next();
            if item.is_some() {
                return item;
            }
        }
        None
    }
}

/// Specifies language system specific features used for shaping glyphs in
/// a particular script.
#[derive(Copy, Clone)]
pub struct Language<'a> {
    data: Bytes<'a>,
    gsub: u32,
    gpos: u32,
    gsub_offset: u32,
    gpos_offset: u32,
    tag: Tag,
}

/// An iterator over languages supported by a script.
#[derive(Copy, Clone)]
pub struct Languages<'a> {
    data: Bytes<'a>,
    gsub: u32,
    gpos: u32,
    gsub_script: u32,
    gpos_script: u32,
    in_gsub: bool,
    len: u16,
    cur: u16,
    done: bool,
}

impl<'a> Languages<'a> {
    fn new(data: Bytes<'a>, gsub: u32, gpos: u32, gsub_script: u32, gpos_script: u32) -> Self {
        Self {
            data,
            gsub,
            gpos,
            gsub_script,
            gpos_script,
            in_gsub: true,
            len: script_language_count(&data, gsub_script),
            cur: 0,
            done: false,
        }
    }

    fn get_next(&mut self) -> Option<Language<'a>> {
        if self.in_gsub {
            if self.cur < self.len {
                let index = self.cur;
                self.cur += 1;
                let (tag, offset) = script_language_at(&self.data, self.gsub_script, index)?;
                let gsub_default = tag == DFLT;
                let (gpos_offset, _) = if gsub_default {
                    (
                        script_default_language(&self.data, self.gpos_script).unwrap_or(0),
                        true,
                    )
                } else {
                    script_language_by_tag(&self.data, self.gpos_script, Some(tag))
                        .unwrap_or((0, false))
                };
                return Some(Language {
                    data: self.data,
                    gsub: self.gsub,
                    gpos: self.gpos,
                    gsub_offset: offset,
                    gpos_offset,
                    tag,
                });
            } else {
                self.in_gsub = false;
                self.cur = 0;
                self.len = script_language_count(&self.data, self.gpos_script);
            }
        } else if self.cur < self.len {
            let index = self.cur;
            self.cur += 1;
            let (tag, offset) = script_language_at(&self.data, self.gpos_script, index)?;
            if script_language_by_tag(&self.data, self.gsub_script, Some(tag)).is_some() {
                return None;
            }
            return Some(Language {
                data: self.data,
                gsub: self.gsub,
                gpos: self.gpos,
                gsub_offset: 0,
                gpos_offset: offset,
                tag,
            });
        } else {
            self.done = true;
        }
        None
    }
}

impl<'a> Iterator for Languages<'a> {
    type Item = Language<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        while !self.done {
            let item = self.get_next();
            if item.is_some() {
                return item;
            }
        }
        None
    }
}

#[derive(Copy, Clone)]
pub struct WritingSystem<'a> {
    lang: Language<'a>,
    script_tag: Tag,
}

impl<'a> WritingSystem<'a> {
    pub fn script_tag(&self) -> Tag {
        self.script_tag
    }

    pub fn language_tag(&self) -> Tag {
        self.lang.tag
    }

    pub fn features(&self) -> Features<'a> {
        Features::new(&self.lang)
    }
}

#[derive(Copy, Clone)]
pub struct WritingSystems<'a> {
    scripts: Scripts<'a>,
    langs: Option<Languages<'a>>,
    script_tag: Tag,
}

impl<'a> WritingSystems<'a> {
    pub fn new(scripts: Scripts<'a>) -> Self {
        Self {
            scripts,
            langs: None,
            script_tag: 0,
        }
    }
}

impl<'a> Iterator for WritingSystems<'a> {
    type Item = WritingSystem<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.langs.is_none() {
                let script = self.scripts.next()?;
                self.script_tag = script.tag;
                self.langs = Some(script.languages());
            }
            if let Some(lang) = self.langs.as_mut().unwrap().next() {
                return Some(WritingSystem {
                    lang,
                    script_tag: self.script_tag,
                });
            } else {
                self.langs = None;
            }
        }
    }
}

/// Represents a single typographic feature-- a substitution or positioning
/// action that is a component of text shaping.
#[derive(Copy, Clone)]
pub struct Feature {
    pub stage: u8,
    pub tag: Tag,
}

#[derive(Copy, Clone)]
pub struct Features<'a> {
    data: Bytes<'a>,
    gsub: u32,
    gpos: u32,
    gsub_language: u32,
    gpos_language: u32,
    stage: u8,
    len: u16,
    cur: u16,
    done: bool,
}

impl<'a> Features<'a> {
    fn new(language: &Language<'a>) -> Self {
        Self {
            data: language.data,
            gsub: language.gsub,
            gpos: language.gpos,
            gsub_language: language.gsub_offset,
            gpos_language: language.gpos_offset,
            stage: 0,
            len: language_feature_count(&language.data, language.gsub_offset),
            cur: 0,
            done: false,
        }
    }

    fn get_next(&mut self) -> Option<Feature> {
        let (gsubgpos, language) = match self.stage {
            0 => (self.gsub, self.gsub_language),
            _ => (self.gpos, self.gpos_language),
        };
        if self.cur < self.len {
            let index = self.cur;
            self.cur += 1;
            let feature = language_feature_at(&self.data, language, index)?;
            let (tag, _offset) = feature_at(&self.data, gsubgpos, feature)?;
            return Some(Feature {
                stage: self.stage,
                tag,
            });
        } else if self.stage == 0 {
            self.stage = 1;
            self.len = language_feature_count(&self.data, self.gpos_language);
            self.cur = 0;
        } else {
            self.done = true;
        }
        None
    }
}

impl<'a> Iterator for Features<'a> {
    type Item = Feature;

    fn next(&mut self) -> Option<Self::Item> {
        while !self.done {
            let item = self.get_next();
            if item.is_some() {
                return item;
            }
        }
        None
    }
}

#[derive(Copy, Clone)]
pub struct AllFeatures<'a> {
    data: Bytes<'a>,
    seen: SeenFeatures,
    table: u32,
    next_table: u32,
    stage: u8,
    len: u16,
    cur: u16,
}

impl<'a> AllFeatures<'a> {
    pub fn new(data: Bytes<'a>, gsub: u32, gpos: u32) -> Self {
        let len = feature_count(&data, gsub);
        Self {
            data,
            seen: SeenFeatures::new(),
            table: gsub,
            next_table: gpos,
            stage: 0,
            len,
            cur: 0,
        }
    }
}

impl<'a> Iterator for AllFeatures<'a> {
    type Item = (u8, Tag);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.cur >= self.len {
                if self.next_table == 0 {
                    return None;
                }
                self.table = self.next_table;
                self.next_table = 0;
                self.stage = 1;
                self.cur = 0;
                self.len = feature_count(&self.data, self.table);
                if self.len == 0 {
                    return None;
                }
            }
            let index = self.cur;
            self.cur += 1;
            if let Some((tag, _)) = feature_at(&self.data, self.table, index) {
                match FEATURES.binary_search_by(|pair| pair.0.cmp(&tag)) {
                    Ok(index) => {
                        if self.seen.mark(index) {
                            return Some((self.stage, tag));
                        }
                    }
                    _ => continue,
                }
            }
        }
    }
}
