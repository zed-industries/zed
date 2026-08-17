use std::rc::Rc;

use crate::sources::EventDispatcher;
use crate::token::TokenInner;

pub(crate) struct SourceEntry<'l, Data> {
    pub(crate) token: TokenInner,
    pub(crate) source: Option<Rc<dyn EventDispatcher<Data> + 'l>>,
}

pub(crate) struct SourceList<'l, Data> {
    sources: Vec<SourceEntry<'l, Data>>,
}

impl<'l, Data> SourceList<'l, Data> {
    pub(crate) fn new() -> Self {
        SourceList {
            sources: Vec::new(),
        }
    }

    pub(crate) fn vacant_entry(&mut self) -> &mut SourceEntry<'l, Data> {
        let opt_id = self.sources.iter().position(|slot| slot.source.is_none());
        match opt_id {
            Some(id) => {
                // we are reusing a slot
                let slot = &mut self.sources[id];
                // increment the slot version
                slot.token = slot.token.increment_version();
                slot
            }
            None => {
                // we are inserting a new slot
                let next_id = self.sources.len();
                self.sources.push(SourceEntry {
                    token: TokenInner::new(self.sources.len())
                        .expect("Trying to insert too many sources in an event loop."),
                    source: None,
                });
                &mut self.sources[next_id]
            }
        }
    }

    pub(crate) fn get(&self, token: TokenInner) -> crate::Result<&SourceEntry<'l, Data>> {
        let entry = self
            .sources
            .get(token.get_id())
            .ok_or(crate::Error::InvalidToken)?;
        if entry.token.same_source_as(token) {
            Ok(entry)
        } else {
            Err(crate::Error::InvalidToken)
        }
    }

    pub(crate) fn get_mut(
        &mut self,
        token: TokenInner,
    ) -> crate::Result<&mut SourceEntry<'l, Data>> {
        let entry = self
            .sources
            .get_mut(token.get_id())
            .ok_or(crate::Error::InvalidToken)?;
        if entry.token.same_source_as(token) {
            Ok(entry)
        } else {
            Err(crate::Error::InvalidToken)
        }
    }
}
