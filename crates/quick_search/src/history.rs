use std::{
    collections::{HashMap, VecDeque},
    sync::{Arc, Mutex, OnceLock},
};

#[derive(Default)]
struct QueryHistory {
    by_source_id: HashMap<Arc<str>, VecDeque<String>>,
}

#[derive(Default)]
pub struct HistoryNavState {
    cursors: HashMap<Arc<str>, HistoryCursor>,
}

#[derive(Default)]
struct HistoryCursor {
    index: Option<usize>,
    prefix: Option<String>,
    suppress_reset_once: bool,
}

impl HistoryCursor {
    fn reset(&mut self) {
        self.index = None;
        self.prefix = None;
        self.suppress_reset_once = false;
    }
}

impl HistoryNavState {
    fn cursor_mut(&mut self, source_id: &Arc<str>) -> &mut HistoryCursor {
        self.cursors
            .entry(source_id.clone())
            .or_insert_with(HistoryCursor::default)
    }

    pub fn on_query_changed(&mut self, source_id: &Arc<str>) {
        let cursor = self.cursor_mut(source_id);
        if cursor.suppress_reset_once {
            cursor.suppress_reset_once = false;
            return;
        }
        cursor.reset();
    }

    pub fn suppress_reset_once(&mut self, source_id: &Arc<str>) {
        self.cursor_mut(source_id).suppress_reset_once = true;
    }

    pub fn reset(&mut self, source_id: &Arc<str>) {
        self.cursor_mut(source_id).reset();
    }

    pub fn prefix(&mut self, source_id: &Arc<str>) -> Option<&str> {
        self.cursor_mut(source_id).prefix.as_deref()
    }

    pub fn set_prefix(&mut self, source_id: &Arc<str>, prefix: String) {
        self.cursor_mut(source_id).prefix = Some(prefix);
    }

    pub fn take_prefix(&mut self, source_id: &Arc<str>) -> Option<String> {
        self.cursor_mut(source_id).prefix.take()
    }

    pub fn index(&mut self, source_id: &Arc<str>) -> Option<usize> {
        self.cursor_mut(source_id).index
    }

    pub fn set_index(&mut self, source_id: &Arc<str>, index: Option<usize>) {
        self.cursor_mut(source_id).index = index;
    }
}

static QUERY_HISTORY: OnceLock<Mutex<QueryHistory>> = OnceLock::new();

const MAX_QUERY_HISTORY: usize = 50;

fn history_bucket_mut<'a>(
    query_history: &'a mut QueryHistory,
    source_id: &Arc<str>,
) -> &'a mut VecDeque<String> {
    query_history
        .by_source_id
        .entry(source_id.clone())
        .or_insert_with(VecDeque::new)
}

pub fn push_query_history(source_id: &Arc<str>, query: &str) {
    let query = query.trim();
    if query.is_empty() {
        return;
    }

    let history = QUERY_HISTORY.get_or_init(|| Mutex::new(QueryHistory::default()));
    let mut history = history
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let bucket = history_bucket_mut(&mut history, source_id);

    if let Some(index) = bucket.iter().position(|q| q == query) {
        bucket.remove(index);
    }
    bucket.push_front(query.to_string());
    while bucket.len() > MAX_QUERY_HISTORY {
        bucket.pop_back();
    }
}

pub fn history_list_for_source_id(source_id: &Arc<str>) -> Vec<String> {
    let history = QUERY_HISTORY.get_or_init(|| Mutex::new(QueryHistory::default()));
    let history = history
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    history
        .by_source_id
        .get(source_id)
        .map(|bucket| bucket.iter().cloned().collect())
        .unwrap_or_default()
}

const DEFAULT_SOURCE_ID: &str = "grep";
static LAST_SOURCE_ID: OnceLock<Mutex<Arc<str>>> = OnceLock::new();

pub fn last_source_id() -> Arc<str> {
    let slot = LAST_SOURCE_ID.get_or_init(|| Mutex::new(Arc::from(DEFAULT_SOURCE_ID)));
    slot.lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
        .clone()
}

pub fn set_last_source_id(source_id: Arc<str>) {
    let slot = LAST_SOURCE_ID.get_or_init(|| Mutex::new(Arc::from(DEFAULT_SOURCE_ID)));
    *slot.lock().unwrap_or_else(|poisoned| poisoned.into_inner()) = source_id;
}
