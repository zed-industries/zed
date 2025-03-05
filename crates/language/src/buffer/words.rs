use std::{
    sync::{
        atomic::{self, AtomicBool},
        Arc,
    },
    time::Duration,
};

use collections::HashMap;
use fuzzy::{StringMatch, StringMatchCandidate};
use gpui::{App, AppContext, Task};
use text::{Edit, Point};
use theme::ActiveTheme;
use util::paths::PathMatcher;

use crate::{search::SearchQuery, Outline};

use super::{BufferRow, BufferSnapshot};

pub struct Words {
    words_to_query: Arc<Vec<StringMatchCandidate>>,
    unique: HashMap<String, ()>,

    update_tasks: HashMap<BufferRow, Task<()>>,
    buffer: BufferSnapshot,
    cancel_flag: Arc<AtomicBool>,
}

impl Words {
    pub fn new(buffer: BufferSnapshot, cx: &App) -> Self {
        let mut this = Self {
            words_to_query: Arc::default(),
            unique: HashMap::default(),
            update_tasks: HashMap::default(),
            buffer,
            cancel_flag: Arc::default(),
        };

        this.schedule_word_update(None, Vec::new(), false, cx);

        this
    }

    pub fn fuzzy_search_words(&self, query: String, cx: &App) -> Task<Vec<StringMatch>> {
        let words = Arc::clone(&self.words_to_query);
        let cancel_flag = Arc::clone(&self.cancel_flag);
        let executor = cx.background_executor().clone();
        cx.background_spawn(async move {
            fuzzy::match_strings(&words, &query, false, usize::MAX, &cancel_flag, executor).await
        })
    }

    pub fn schedule_word_update(
        &mut self,
        new_snapshot: Option<BufferSnapshot>,
        edits: Vec<Edit<Point>>,
        debounce: bool,
        cx: &App,
    ) {
        let new_snapshot = match new_snapshot {
            Some(new_snapshot) => {
                if self.buffer.version().changed_since(&new_snapshot.version()) {
                    return;
                }
                self.buffer = new_snapshot.clone();
                new_snapshot
            }
            None => self.buffer.clone(),
        };

        let buffer = new_snapshot.clone();
        let debounce = if debounce {
            Some(cx.background_executor().timer(Duration::from_millis(50)))
        } else {
            None
        };
        let search_results = cx
            .background_spawn(async move {
                if let Some(debounce) = debounce {
                    debounce.await;
                }
                let search_results = SearchQuery::regex(
                    r"\w+",
                    false,
                    false,
                    false,
                    PathMatcher::default(),
                    PathMatcher::default(),
                    None,
                )
                .expect("TODO kb")
                .search(&buffer, None)
                .await;
                dbg!(search_results.len());
                search_results
            })
            .detach();
    }

    pub fn schedule_symbol_update(&mut self, cx: &App) {
        let theme = cx.theme().syntax().clone();
        let buffer = self.buffer.clone();
        // TODO kb proper state management, query by cached ranges (500 lines hunks?)
        let outline_results = cx
            .background_spawn(async move {
                let outline = buffer
                    .outline_items_containing(0..buffer.len(), false, Some(&theme))
                    .map(Outline::new);
                dbg!(outline.as_ref().map(|o| o.path_candidates.len()));
                outline
            })
            .detach();
    }
}

impl Drop for Words {
    fn drop(&mut self) {
        self.cancel_flag.store(true, atomic::Ordering::Relaxed);
    }
}
