use std::sync::{
    atomic::{self, AtomicBool},
    Arc,
};

use collections::HashMap;
use fuzzy::{StringMatch, StringMatchCandidate};
use gpui::{App, AppContext, Task};
use text::{Edit, Point};
use util::paths::PathMatcher;

use crate::search::SearchQuery;

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

        this.schedule_update(None, Vec::new(), cx);

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

    pub fn schedule_update(
        &mut self,
        new_snapshot: Option<BufferSnapshot>,
        edits: Vec<Edit<Point>>,
        cx: &App,
    ) {
        // TODO kb need to search by words
        let buffer = self.buffer.clone();
        cx.spawn(|cx| async move {
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
            // TODO kb proper state management, query by cached ranges (500 lines hunks?)
            .search(&buffer, None)
            .await;
            dbg!(search_results.len());
            //
        })
        .detach();
    }
}

impl Drop for Words {
    fn drop(&mut self) {
        self.cancel_flag.store(true, atomic::Ordering::Relaxed);
    }
}
