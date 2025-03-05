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
use parking_lot::RwLock;
use text::{Anchor, Edit, Point};
use theme::ActiveTheme;
use util::{debug_panic, paths::PathMatcher};

use crate::{search::SearchQuery, Outline};

use super::{BufferRow, BufferSnapshot};

pub struct Words {
    unique_words: Arc<RwLock<UniqueWords>>,
    outlines: Arc<RwLock<Option<Arc<Outline<Anchor>>>>>,

    word_update_tasks: HashMap<BufferRow, Task<()>>,
    outline_update_task: Task<()>,
    buffer: BufferSnapshot,
    cancel_flag: Arc<AtomicBool>,
}

#[derive(Debug, Default)]
struct UniqueWords {
    words: HashMap<String, ()>,
    word_matches: Arc<Vec<StringMatchCandidate>>,
}

impl Words {
    pub fn new(buffer: BufferSnapshot, cx: &App) -> Self {
        let mut this = Self {
            unique_words: Arc::default(),
            outlines: Arc::default(),
            word_update_tasks: HashMap::default(),
            outline_update_task: Task::ready(()),
            buffer,
            cancel_flag: Arc::default(),
        };

        this.update_words(None, Vec::new(), false, cx);

        this
    }

    pub fn fuzzy_search_words(&self, query: String, cx: &App) -> Task<Vec<StringMatch>> {
        let words = self.unique_words.read().word_matches.clone();
        let cancel_flag = Arc::clone(&self.cancel_flag);
        let executor = cx.background_executor().clone();
        cx.background_spawn(async move {
            fuzzy::match_strings(&words, &query, false, usize::MAX, &cancel_flag, executor).await
        })
    }

    pub fn fuzzy_search_outlines(&self, query: String, cx: &App) -> Task<Vec<StringMatch>> {
        let Some(outlines) = self.outlines.read().clone() else {
            return Task::ready(Vec::new());
        };
        let cancel_flag = Arc::clone(&self.cancel_flag);
        let executor = cx.background_executor().clone();
        cx.background_spawn(async move {
            fuzzy::match_strings(
                &outlines.path_candidates,
                &query,
                false,
                usize::MAX,
                &cancel_flag,
                executor,
            )
            .await
        })
    }

    pub fn update_words(
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
        let unique_words = self.unique_words.clone();
        self.word_update_tasks.insert(
            0,
            cx.background_spawn(async move {
                if let Some(debounce) = debounce {
                    debounce.await;
                }
                let query = r"[\p{L}\p{N}_]+";
                let word_search_results = match SearchQuery::regex(
                    query,
                    false,
                    false,
                    false,
                    PathMatcher::default(),
                    PathMatcher::default(),
                    None,
                ) {
                    Ok(query) => query,
                    Err(e) => {
                        debug_panic!("Error creating search with query {query}: {e}");
                        return;
                    }
                }
                .search(&buffer, None)
                .await;
                let rope = buffer.as_rope();
                let mut cursor = rope.cursor(0);
                let matched_words = word_search_results.into_iter().map(|word_range| {
                    cursor.seek_forward(word_range.start);
                    cursor.slice(word_range.end).to_string()
                });
                let mut unique_words = unique_words.write();
                // TODO kb proper state management, query by cached ranges (500 lines hunks?)
                unique_words.word_matches = Arc::default();
                unique_words.words.clear();
                for matched_word in matched_words {
                    if unique_words
                        .words
                        .insert(matched_word.clone(), ())
                        .is_none()
                    {
                        let mut new_word_matches = unique_words.word_matches.as_slice().to_vec();
                        new_word_matches.push(StringMatchCandidate::new(
                            new_word_matches.len(),
                            &matched_word,
                        ));
                        unique_words.word_matches = Arc::new(new_word_matches);
                    }
                }

                dbg!(unique_words.word_matches.len());
            }),
        );
    }

    pub fn update_outlines(&mut self, cx: &App) {
        let theme = cx.theme().syntax().clone();
        let buffer = self.buffer.clone();
        let outlines = self.outlines.clone();
        self.outline_update_task = cx.background_spawn(async move {
            let outline = buffer
                .outline_items_containing(0..buffer.len(), false, Some(&theme))
                .map(Outline::new)
                .map(Arc::new);
            dbg!(outline.as_ref().map(|o| o.path_candidates.len()));
            *outlines.write() = outline;
        });
    }
}

impl Drop for Words {
    fn drop(&mut self) {
        self.cancel_flag.store(true, atomic::Ordering::Relaxed);
    }
}
