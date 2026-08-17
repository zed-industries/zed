use rustc_hash::FxHashMap;
use std::sync::{Arc, Mutex, OnceLock};

#[derive(Debug, Default, Clone)]
pub(super) struct StateRenderDetails {
    pub(super) root_calls: u32,
    pub(super) clusters: web_time::Duration,
    pub(super) edge_paths: web_time::Duration,
    pub(super) edge_labels: web_time::Duration,
    pub(super) leaf_nodes: web_time::Duration,
    pub(super) leaf_nodes_style_parse: web_time::Duration,
    pub(super) leaf_nodes_roughjs: web_time::Duration,
    pub(super) leaf_roughjs_calls: u32,
    pub(super) leaf_roughjs_unique: std::collections::HashSet<StateRoughCacheKey>,
    pub(super) leaf_nodes_measure: web_time::Duration,
    pub(super) leaf_nodes_label_html: web_time::Duration,
    pub(super) leaf_nodes_emit: web_time::Duration,
    pub(super) nested_roots: web_time::Duration,
    pub(super) self_loop_placeholders: web_time::Duration,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(super) struct StateRoughCacheKey {
    pub(super) tag: u8,
    pub(super) a: u64,
    pub(super) b: u64,
    pub(super) seed: u64,
}

type StateRoughCircleCache = FxHashMap<StateRoughCacheKey, Arc<String>>;
type StateRoughPathsCache = FxHashMap<StateRoughCacheKey, (Arc<String>, Arc<String>)>;

const STATE_ROUGH_TLS_CACHE_LIMIT: usize = 4096;

pub(super) fn state_global_rough_circle_cache() -> &'static Mutex<StateRoughCircleCache> {
    static CACHE: OnceLock<Mutex<StateRoughCircleCache>> = OnceLock::new();
    CACHE.get_or_init(|| Mutex::new(FxHashMap::default()))
}

pub(super) fn state_global_rough_paths_cache() -> &'static Mutex<StateRoughPathsCache> {
    static CACHE: OnceLock<Mutex<StateRoughPathsCache>> = OnceLock::new();
    CACHE.get_or_init(|| Mutex::new(FxHashMap::default()))
}

thread_local! {
    static STATE_TLS_ROUGH_CIRCLE_CACHE: std::cell::RefCell<StateRoughCircleCache> =
        std::cell::RefCell::new(FxHashMap::default());
    static STATE_TLS_ROUGH_PATHS_CACHE: std::cell::RefCell<StateRoughPathsCache> =
        std::cell::RefCell::new(FxHashMap::default());
}

#[inline]
pub(super) fn state_tls_get_circle(key: StateRoughCacheKey) -> Option<Arc<String>> {
    STATE_TLS_ROUGH_CIRCLE_CACHE.with(|cache| cache.borrow().get(&key).cloned())
}

#[inline]
pub(super) fn state_tls_put_circle(key: StateRoughCacheKey, value: Arc<String>) {
    STATE_TLS_ROUGH_CIRCLE_CACHE.with(|cache| {
        let mut map = cache.borrow_mut();
        if map.len() >= STATE_ROUGH_TLS_CACHE_LIMIT {
            // Best-effort bound. This cache only exists to avoid global mutex overhead on
            // repeated renders within the same thread; eviction does not affect correctness.
            map.clear();
        }
        map.insert(key, value);
    });
}

#[inline]
pub(super) fn state_tls_get_paths(key: StateRoughCacheKey) -> Option<(Arc<String>, Arc<String>)> {
    STATE_TLS_ROUGH_PATHS_CACHE.with(|cache| cache.borrow().get(&key).cloned())
}

#[inline]
pub(super) fn state_tls_put_paths(key: StateRoughCacheKey, value: (Arc<String>, Arc<String>)) {
    STATE_TLS_ROUGH_PATHS_CACHE.with(|cache| {
        let mut map = cache.borrow_mut();
        if map.len() >= STATE_ROUGH_TLS_CACHE_LIMIT {
            // Best-effort bound. See `state_tls_put_circle` for rationale.
            map.clear();
        }
        map.insert(key, value);
    });
}

#[inline]
pub(super) fn detail_guard<'a>(
    enabled: bool,
    dst: &'a mut web_time::Duration,
) -> Option<super::timing::TimingGuard<'a>> {
    enabled.then(|| super::timing::TimingGuard::new(dst))
}
