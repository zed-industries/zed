//! # logger
pub use log as log_impl;

pub const SCOPE_DEPTH_MAX: usize = 4;

/// because we are currently just wrapping the `log` crate in `zlog`,
/// we need to work around the fact that the `log` crate only provides a
/// single global level filter. In order to have more precise control until
/// we no longer wrap `log`, we bump up the priority of log level so that it
/// will be logged, even if the actual level is lower
/// This is fine for now, as we use a `info` level filter by default in releases,
/// which hopefully won't result in confusion like `warn` or `error` levels might.
pub fn min_printed_log_level(level: log_impl::Level) -> log_impl::Level {
    // this logic is defined based on the logic used in the `log` crate,
    // which checks that a logs level is <= both of these values,
    // so we take the minimum of the two values to ensure that check passes
    let level_min_static = log_impl::STATIC_MAX_LEVEL;
    let level_min_dynamic = log_impl::max_level();
    if level <= level_min_static && level <= level_min_dynamic {
        return level;
    }
    return log_impl::LevelFilter::min(level_min_static, level_min_dynamic)
        .to_level()
        .unwrap_or(level);
}

#[macro_export]
macro_rules! log {
    ($logger:expr, $level:expr, $($arg:tt)+) => {
        let level = $level;
        let logger = $logger;
        let (enabled, level) = $crate::scope_map::is_scope_enabled(&logger.scope, level);
        if enabled {
            $crate::log_impl::log!(level, "[{}]: {}", &logger.fmt_scope(), format!($($arg)+));
        }
    }
}

#[macro_export]
macro_rules! trace {
    ($logger:expr => $($arg:tt)+) => {
        $crate::log!($logger, $crate::log_impl::Level::Trace, $($arg)+);
    };
    ($($arg:tt)+) => {
        $crate::log!($crate::default_logger!(), $crate::log_impl::Level::Trace, $($arg)+);
    };
}

#[macro_export]
macro_rules! debug {
    ($logger:expr => $($arg:tt)+) => {
        $crate::log!($logger, $crate::log_impl::Level::Debug, $($arg)+);
    };
    ($($arg:tt)+) => {
        $crate::log!($crate::default_logger!(), $crate::log_impl::Level::Debug, $($arg)+);
    };
}

#[macro_export]
macro_rules! info {
    ($logger:expr => $($arg:tt)+) => {
        $crate::log!($logger, $crate::log_impl::Level::Info, $($arg)+);
    };
    ($($arg:tt)+) => {
        $crate::log!($crate::default_logger!(), $crate::log_impl::Level::Info, $($arg)+);
    };
}

#[macro_export]
macro_rules! warn {
    ($logger:expr => $($arg:tt)+) => {
        $crate::log!($logger, $crate::log_impl::Level::Warn, $($arg)+);
    };
    ($($arg:tt)+) => {
        $crate::log!($crate::default_logger!(), $crate::log_impl::Level::Warn, $($arg)+);
    };
}

#[macro_export]
macro_rules! error {
    ($logger:expr => $($arg:tt)+) => {
        $crate::log!($logger, $crate::log_impl::Level::Error, $($arg)+);
    };
    ($($arg:tt)+) => {
        $crate::log!($crate::default_logger!(), $crate::log_impl::Level::Error, $($arg)+);
    };
}

/// Creates a timer that logs the duration it was active for either when
/// it is dropped, or when explicitly stopped using the `end` method.
/// Logs at the `trace` level.
/// Note that it will include time spent across await points
/// (i.e. should not be used to measure the performance of async code)
/// However, this is a feature not a bug, as it allows for a more accurate
/// understanding of how long the action actually took to complete, including
/// interruptions, which can help explain why something may have timed out,
/// why it took longer to complete than it would had the await points resolved
/// immediately, etc.
#[macro_export]
macro_rules! time {
    ($logger:expr => $name:expr) => {
        $crate::Timer::new($logger, $name)
    };
    ($name:expr) => {
        time!($crate::default_logger!() => $name)
    };
}

#[macro_export]
macro_rules! scoped {
    ($parent:expr => $name:expr) => {{
        let parent = $parent;
        let name = $name;
        let mut scope = parent.scope;
        let mut index = 1; // always have crate/module name
        while index < scope.len() && !scope[index].is_empty() {
            index += 1;
        }
        if index >= scope.len() {
            #[cfg(debug_assertions)]
            {
                panic!("Scope overflow trying to add scope {}", name);
            }
            #[cfg(not(debug_assertions))]
            {
                $crate::warn!(
                    parent =>
                    "Scope overflow trying to add scope {}... ignoring scope",
                    name
                );
            }
        }
        scope[index] = name;
        $crate::Logger { scope }
    }};
    ($name:expr) => {
        $crate::scoped!($crate::default_logger!() => $name)
    };
}

#[macro_export]
macro_rules! default_logger {
    () => {
        $crate::Logger {
            scope: $crate::private::scope_new(&[$crate::crate_name!()]),
        }
    };
}

#[macro_export]
macro_rules! crate_name {
    () => {
        $crate::private::extract_crate_name_from_module_path(module_path!())
    };
}

/// functions that are used in macros, and therefore must be public,
/// but should not be used directly
pub mod private {
    use super::*;

    pub fn extract_crate_name_from_module_path(module_path: &'static str) -> &'static str {
        return module_path
            .split_once("::")
            .map(|(crate_name, _)| crate_name)
            .unwrap_or(module_path);
    }

    pub fn scope_new(scopes: &[&'static str]) -> Scope {
        assert!(scopes.len() <= SCOPE_DEPTH_MAX);
        let mut scope = [""; SCOPE_DEPTH_MAX];
        scope[0..scopes.len()].copy_from_slice(scopes);
        scope
    }

    pub fn scope_alloc_new(scopes: &[&str]) -> ScopeAlloc {
        assert!(scopes.len() <= SCOPE_DEPTH_MAX);
        let mut scope = [""; SCOPE_DEPTH_MAX];
        scope[0..scopes.len()].copy_from_slice(scopes);
        scope.map(|s| s.to_string())
    }

    pub fn scope_to_alloc(scope: &Scope) -> ScopeAlloc {
        return scope.map(|s| s.to_string());
    }
}

pub type Scope = [&'static str; SCOPE_DEPTH_MAX];
pub type ScopeAlloc = [String; SCOPE_DEPTH_MAX];
const SCOPE_STRING_SEP: &'static str = ".";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Logger {
    pub scope: Scope,
}

impl Logger {
    pub fn fmt_scope(&self) -> String {
        let mut last = 0;
        for s in self.scope {
            if s.is_empty() {
                break;
            }
            last += 1;
        }

        return self.scope[0..last].join(SCOPE_STRING_SEP);
    }
}

pub struct Timer {
    pub logger: Logger,
    pub start_time: std::time::Instant,
    pub name: &'static str,
    pub warn_if_longer_than: Option<std::time::Duration>,
    pub done: bool,
}

impl Drop for Timer {
    fn drop(&mut self) {
        self.finish();
    }
}

impl Timer {
    #[must_use = "Timer will stop when dropped, the result of this function should be saved in a variable prefixed with `_` if it should stop when dropped"]
    pub fn new(logger: Logger, name: &'static str) -> Self {
        return Self {
            logger,
            name,
            start_time: std::time::Instant::now(),
            warn_if_longer_than: None,
            done: false,
        };
    }
    pub fn warn_if_gt(mut self, warn_limit: std::time::Duration) -> Self {
        self.warn_if_longer_than = Some(warn_limit);
        return self;
    }

    pub fn end(mut self) {
        self.finish();
    }

    fn finish(&mut self) {
        if self.done {
            return;
        }
        let elapsed = self.start_time.elapsed();
        if let Some(warn_limit) = self.warn_if_longer_than {
            if elapsed > warn_limit {
                crate::warn!(
                    self.logger =>
                    "Timer '{}' took {:?}. Which was longer than the expected limit of {:?}",
                    self.name,
                    elapsed,
                    warn_limit
                );
                self.done = true;
                return;
            }
        }
        crate::trace!(
            self.logger =>
            "Timer '{}' finished in {:?}",
            self.name,
            elapsed
        );
        self.done = true;
    }
}

pub mod scope_map {
    use std::{
        collections::{HashMap, VecDeque},
        hash::{DefaultHasher, Hasher},
        sync::{
            RwLock,
            atomic::{AtomicU64, Ordering},
        },
        usize,
    };

    use super::*;

    static SCOPE_MAP: RwLock<Option<ScopeMap>> = RwLock::new(None);
    static SCOPE_MAP_HASH: AtomicU64 = AtomicU64::new(0);

    pub fn is_scope_enabled(scope: &Scope, level: log_impl::Level) -> (bool, log_impl::Level) {
        let level_min = min_printed_log_level(level);
        if level <= level_min {
            // [FAST PATH]
            // if the message is at or below the minimum printed log level
            // (where error < warn < info etc) then always enable
            return (true, level);
        }

        let Ok(map) = SCOPE_MAP.read() else {
            // on failure, default to enabled detection done by `log` crate
            return (true, level);
        };

        let Some(map) = map.as_ref() else {
            // on failure, default to enabled detection done by `log` crate
            return (true, level);
        };

        if map.is_empty() {
            // if no scopes are enabled, default to enabled detection done by `log` crate
            return (true, level);
        }
        let enabled_status = map.is_enabled(&scope, level);
        match enabled_status {
            EnabledStatus::NotConfigured => {
                // if this scope isn't configured, default to enabled detection done by `log` crate
                return (true, level);
            }
            EnabledStatus::Enabled => {
                // if this scope is enabled, enable logging
                // note: bumping level to min level that will be printed
                // to work around log crate limitations
                return (true, level_min);
            }
            EnabledStatus::Disabled => {
                // if the configured level is lower than the requested level, disable logging
                // note: err = 0, warn = 1, etc.
                return (false, level);
            }
        }
    }

    fn hash_scope_map_settings(map: &HashMap<String, String>) -> u64 {
        let mut hasher = DefaultHasher::new();
        let mut items = map.iter().collect::<Vec<_>>();
        items.sort();
        for (key, value) in items {
            Hasher::write(&mut hasher, key.as_bytes());
            Hasher::write(&mut hasher, value.as_bytes());
        }
        return hasher.finish();
    }

    pub fn refresh(settings: &HashMap<String, String>) {
        let hash_old = SCOPE_MAP_HASH.load(Ordering::Acquire);
        let hash_new = hash_scope_map_settings(settings);
        if hash_old == hash_new && hash_old != 0 {
            return;
        }
        let map_new = ScopeMap::new_from_settings(settings);

        if let Ok(_) = SCOPE_MAP_HASH.compare_exchange(
            hash_old,
            hash_new,
            Ordering::Release,
            Ordering::Relaxed,
        ) {
            let mut map = SCOPE_MAP.write().unwrap_or_else(|err| {
                SCOPE_MAP.clear_poison();
                err.into_inner()
            });
            *map = Some(map_new);
        }
    }

    fn level_from_level_str(level_str: &String) -> Option<log_impl::Level> {
        let level = match level_str.to_ascii_lowercase().as_str() {
            "" => log_impl::Level::Trace,
            "trace" => log_impl::Level::Trace,
            "debug" => log_impl::Level::Debug,
            "info" => log_impl::Level::Info,
            "warn" => log_impl::Level::Warn,
            "error" => log_impl::Level::Error,
            "off" | "disable" | "no" | "none" | "disabled" => {
                crate::warn!(
                    "Invalid log level \"{level_str}\", set to error to disable non-error logging. Defaulting to error"
                );
                log_impl::Level::Error
            }
            _ => {
                crate::warn!("Invalid log level \"{level_str}\", ignoring");
                return None;
            }
        };
        return Some(level);
    }

    fn scope_alloc_from_scope_str(scope_str: &String) -> Option<ScopeAlloc> {
        let mut scope_buf = [""; SCOPE_DEPTH_MAX];
        let mut index = 0;
        let mut scope_iter = scope_str.split(SCOPE_STRING_SEP);
        while index < SCOPE_DEPTH_MAX {
            let Some(scope) = scope_iter.next() else {
                break;
            };
            if scope == "" {
                continue;
            }
            scope_buf[index] = scope;
            index += 1;
        }
        if index == 0 {
            return None;
        }
        if let Some(_) = scope_iter.next() {
            crate::warn!(
                "Invalid scope key, too many nested scopes: '{scope_str}'. Max depth is {SCOPE_DEPTH_MAX}",
            );
            return None;
        }
        let scope = scope_buf.map(|s| s.to_string());
        return Some(scope);
    }

    pub struct ScopeMap {
        entries: Vec<ScopeMapEntry>,
        root_count: usize,
    }

    pub struct ScopeMapEntry {
        scope: String,
        enabled: Option<log_impl::Level>,
        descendants: std::ops::Range<usize>,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum EnabledStatus {
        Enabled,
        Disabled,
        NotConfigured,
    }

    impl ScopeMap {
        pub fn new_from_settings(items_input_map: &HashMap<String, String>) -> Self {
            let mut items = items_input_map
                .into_iter()
                .filter_map(|(scope_str, level_str)| {
                    let scope = scope_alloc_from_scope_str(&scope_str)?;
                    let level = level_from_level_str(&level_str)?;
                    return Some((scope, level));
                })
                .collect::<Vec<_>>();

            items.sort_by(|a, b| a.0.cmp(&b.0));

            let mut this = Self {
                entries: Vec::with_capacity(items.len() * SCOPE_DEPTH_MAX),
                root_count: 0,
            };

            let items_count = items.len();

            struct ProcessQueueEntry {
                parent_index: usize,
                depth: usize,
                items_range: std::ops::Range<usize>,
            }
            let mut process_queue = VecDeque::new();
            process_queue.push_back(ProcessQueueEntry {
                parent_index: usize::MAX,
                depth: 0,
                items_range: 0..items_count,
            });

            let empty_range = 0..0;

            while let Some(process_entry) = process_queue.pop_front() {
                let ProcessQueueEntry {
                    items_range,
                    depth,
                    parent_index,
                } = process_entry;
                let mut cursor = items_range.start;
                let res_entries_start = this.entries.len();
                while cursor < items_range.end {
                    let sub_items_start = cursor;
                    cursor += 1;
                    let scope_name = &items[sub_items_start].0[depth];
                    while cursor < items_range.end && &items[cursor].0[depth] == scope_name {
                        cursor += 1;
                    }
                    let sub_items_end = cursor;
                    if scope_name == "" {
                        assert_eq!(sub_items_start + 1, sub_items_end);
                        assert_ne!(depth, 0);
                        assert_ne!(parent_index, usize::MAX);
                        assert!(this.entries[parent_index].enabled.is_none());
                        this.entries[parent_index].enabled = Some(items[sub_items_start].1);
                        continue;
                    }
                    let is_valid_scope = scope_name != "";
                    let is_last = depth + 1 == SCOPE_DEPTH_MAX || !is_valid_scope;
                    let mut enabled = None;
                    if is_last {
                        assert_eq!(sub_items_start + 1, sub_items_end);
                        enabled = Some(items[sub_items_start].1);
                    } else {
                        let entry_index = this.entries.len();
                        process_queue.push_back(ProcessQueueEntry {
                            items_range: sub_items_start..sub_items_end,
                            parent_index: entry_index,
                            depth: depth + 1,
                        });
                    }
                    this.entries.push(ScopeMapEntry {
                        scope: scope_name.to_owned(),
                        enabled,
                        descendants: empty_range.clone(),
                    });
                }
                let res_entries_end = this.entries.len();
                if parent_index != usize::MAX {
                    this.entries[parent_index].descendants = res_entries_start..res_entries_end;
                } else {
                    this.root_count = res_entries_end;
                }
            }

            return this;
        }

        pub fn is_empty(&self) -> bool {
            self.entries.is_empty()
        }

        pub fn is_enabled<S>(
            &self,
            scope: &[S; SCOPE_DEPTH_MAX],
            level: log_impl::Level,
        ) -> EnabledStatus
        where
            S: AsRef<str>,
        {
            let mut enabled = None;
            let mut cur_range = &self.entries[0..self.root_count];
            let mut depth = 0;

            'search: while !cur_range.is_empty()
                && depth < SCOPE_DEPTH_MAX
                && scope[depth].as_ref() != ""
            {
                for entry in cur_range {
                    if entry.scope == scope[depth].as_ref() {
                        // note:
                        enabled = entry.enabled.or(enabled);
                        cur_range = &self.entries[entry.descendants.clone()];
                        depth += 1;
                        continue 'search;
                    }
                }
                break 'search;
            }

            return enabled.map_or(EnabledStatus::NotConfigured, |level_enabled| {
                if level <= level_enabled {
                    EnabledStatus::Enabled
                } else {
                    EnabledStatus::Disabled
                }
            });
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        fn scope_map_from_keys(kv: &[(&str, &str)]) -> ScopeMap {
            let hash_map: HashMap<String, String> = kv
                .iter()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect();
            ScopeMap::new_from_settings(&hash_map)
        }

        #[test]
        fn test_initialization() {
            let map = scope_map_from_keys(&[("a.b.c.d", "trace")]);
            assert_eq!(map.root_count, 1);
            assert_eq!(map.entries.len(), 4);

            let map = scope_map_from_keys(&[]);
            assert_eq!(map.root_count, 0);
            assert_eq!(map.entries.len(), 0);

            let map = scope_map_from_keys(&[("", "trace")]);
            assert_eq!(map.root_count, 0);
            assert_eq!(map.entries.len(), 0);

            let map = scope_map_from_keys(&[("foo..bar", "trace")]);
            assert_eq!(map.root_count, 1);
            assert_eq!(map.entries.len(), 2);

            let map = scope_map_from_keys(&[
                ("a.b.c.d", "trace"),
                ("e.f.g.h", "debug"),
                ("i.j.k.l", "info"),
                ("m.n.o.p", "warn"),
                ("q.r.s.t", "error"),
            ]);
            assert_eq!(map.root_count, 5);
            assert_eq!(map.entries.len(), 20);
            assert_eq!(map.entries[0].scope, "a");
            assert_eq!(map.entries[1].scope, "e");
            assert_eq!(map.entries[2].scope, "i");
            assert_eq!(map.entries[3].scope, "m");
            assert_eq!(map.entries[4].scope, "q");
        }

        fn scope_from_scope_str(scope_str: &'static str) -> Scope {
            let mut scope_buf = [""; SCOPE_DEPTH_MAX];
            let mut index = 0;
            let mut scope_iter = scope_str.split(SCOPE_STRING_SEP);
            while index < SCOPE_DEPTH_MAX {
                let Some(scope) = scope_iter.next() else {
                    break;
                };
                if scope == "" {
                    continue;
                }
                scope_buf[index] = scope;
                index += 1;
            }
            assert_ne!(index, 0);
            assert!(scope_iter.next().is_none());
            return scope_buf;
        }

        #[test]
        fn test_is_enabled() {
            let map = scope_map_from_keys(&[
                ("a.b.c.d", "trace"),
                ("e.f.g.h", "debug"),
                ("i.j.k.l", "info"),
                ("m.n.o.p", "warn"),
                ("q.r.s.t", "error"),
            ]);
            use log_impl::Level;
            assert_eq!(
                map.is_enabled(&scope_from_scope_str("a.b.c.d"), Level::Trace),
                EnabledStatus::Enabled
            );
            assert_eq!(
                map.is_enabled(&scope_from_scope_str("a.b.c.d"), Level::Debug),
                EnabledStatus::Enabled
            );

            assert_eq!(
                map.is_enabled(&scope_from_scope_str("e.f.g.h"), Level::Debug),
                EnabledStatus::Enabled
            );
            assert_eq!(
                map.is_enabled(&scope_from_scope_str("e.f.g.h"), Level::Info),
                EnabledStatus::Enabled
            );
            assert_eq!(
                map.is_enabled(&scope_from_scope_str("e.f.g.h"), Level::Trace),
                EnabledStatus::Disabled
            );

            assert_eq!(
                map.is_enabled(&scope_from_scope_str("i.j.k.l"), Level::Info),
                EnabledStatus::Enabled
            );
            assert_eq!(
                map.is_enabled(&scope_from_scope_str("i.j.k.l"), Level::Warn),
                EnabledStatus::Enabled
            );
            assert_eq!(
                map.is_enabled(&scope_from_scope_str("i.j.k.l"), Level::Debug),
                EnabledStatus::Disabled
            );

            assert_eq!(
                map.is_enabled(&scope_from_scope_str("m.n.o.p"), Level::Warn),
                EnabledStatus::Enabled
            );
            assert_eq!(
                map.is_enabled(&scope_from_scope_str("m.n.o.p"), Level::Error),
                EnabledStatus::Enabled
            );
            assert_eq!(
                map.is_enabled(&scope_from_scope_str("m.n.o.p"), Level::Info),
                EnabledStatus::Disabled
            );

            assert_eq!(
                map.is_enabled(&scope_from_scope_str("q.r.s.t"), Level::Error),
                EnabledStatus::Enabled
            );
            assert_eq!(
                map.is_enabled(&scope_from_scope_str("q.r.s.t"), Level::Warn),
                EnabledStatus::Disabled
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crate_name() {
        assert_eq!(crate_name!(), "zlog");
    }
}
