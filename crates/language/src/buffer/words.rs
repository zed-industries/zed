use std::sync::{
    atomic::{self, AtomicBool},
    Arc,
};

use collections::HashMap;
use fuzzy::{StringMatch, StringMatchCandidate};
use gpui::{App, AppContext, SharedString, Task};
use text::{BufferSnapshot, Edit, Point};

use super::BufferRow;

pub struct Words {
    words_to_query: Arc<Vec<StringMatchCandidate>>,
    unique: HashMap<String, ()>,

    update_tasks: HashMap<BufferRow, Task<()>>,
    buffer: BufferSnapshot,
    cancel_flag: Arc<AtomicBool>,
}

impl Words {
    pub fn new(buffer: BufferSnapshot) -> Self {
        Self {
            words_to_query: Arc::default(),
            unique: HashMap::default(),
            update_tasks: HashMap::default(),
            buffer,
            cancel_flag: Arc::default(),
        }
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
        new_snapshot: BufferSnapshot,
        edits: Vec<Edit<Point>>,
        cx: &App,
    ) {
        // todo!("TODO kb")
        dbg!(edits);
    }
}

impl Drop for Words {
    fn drop(&mut self) {
        self.cancel_flag.store(true, atomic::Ordering::Relaxed);
    }
}
