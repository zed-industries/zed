use collections::HashMap;
use std::collections::VecDeque;
use std::sync::{
    OnceLock, RwLock,
    atomic::{AtomicU8, Ordering},
};

use crate::{SCOPE_DEPTH_MAX, SCOPE_STRING_SEP_STR, Scope, ScopeAlloc, env_config, private};

use log;

static ENV_FILTER: OnceLock<env_config::EnvFilter> = OnceLock::new();
static SCOPE_MAP: RwLock<Option<ScopeMap>> = RwLock::new(None);

pub const LEVEL_ENABLED_MAX_DEFAULT: log::LevelFilter = log::LevelFilter::Info;
/// The maximum log level of verbosity that is enabled by default.
/// All messages more verbose than this level will be discarded
/// by default unless specially configured.
///
/// This is used instead of the `log::max_level` as we need to tell the `log`
/// crate that the max level is everything, so that we can dynamically enable
/// logs that are more verbose than this level without the `log` crate throwing
/// them away before we see them
static LEVEL_ENABLED_MAX_STATIC: AtomicU8 = AtomicU8::new(LEVEL_ENABLED_MAX_DEFAULT as u8);

/// A cache of the true maximum log level that _could_ be printed. This is based
/// on the maximally verbose level that is configured by the user, and is used
/// to filter out logs more verbose than any configured level.
///
/// E.g. if `LEVEL_ENABLED_MAX_STATIC `is 'info' but a user has configured some
/// scope to print at a `debug` level, then this will be `debug`, and all
/// `trace` logs will be discarded.
/// Therefore, it should always be `>= LEVEL_ENABLED_MAX_STATIC`
// PERF: this doesn't need to be an atomic, we don't actually care about race conditions here
pub static LEVEL_ENABLED_MAX_CONFIG: AtomicU8 = AtomicU8::new(LEVEL_ENABLED_MAX_DEFAULT as u8);

const DEFAULT_FILTERS: &[(&str, log::LevelFilter)] = &[
    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
    ("zbus", log::LevelFilter::Warn),
    #[cfg(any(target_os = "linux", target_os = "freebsd", target_os = "windows"))]
    ("blade_graphics", log::LevelFilter::Warn),
    #[cfg(any(target_os = "linux", target_os = "freebsd", target_os = "windows"))]
    ("naga::back::spv::writer", log::LevelFilter::Warn),
    // usvg prints a lot of warnings on rendering an SVG with partial errors, which
    // can happen a lot with the SVG preview
    ("usvg::parser::style", log::LevelFilter::Error),
];

pub fn init_env_filter(filter: env_config::EnvFilter) {
    if let Some(level_max) = filter.level_global {
        LEVEL_ENABLED_MAX_STATIC.store(level_max as u8, Ordering::Release)
    }
    if ENV_FILTER.set(filter).is_err() {
        panic!("Environment filter cannot be initialized twice");
    }
}

pub fn is_possibly_enabled_level(level: log::Level) -> bool {
    level as u8 <= LEVEL_ENABLED_MAX_CONFIG.load(Ordering::Acquire)
}

pub fn is_scope_enabled(scope: &Scope, module_path: Option<&str>, level: log::Level) -> bool {
    // TODO: is_always_allowed_level that checks against LEVEL_ENABLED_MIN_CONFIG
    if !is_possibly_enabled_level(level) {
        // [FAST PATH]
        // if the message is above the maximum enabled log level
        // (where error < warn < info etc) then disable without checking
        // scope map
        return false;
    }
    let is_enabled_by_default = level as u8 <= LEVEL_ENABLED_MAX_STATIC.load(Ordering::Acquire);
    let global_scope_map = SCOPE_MAP.read().unwrap_or_else(|err| {
        SCOPE_MAP.clear_poison();
        err.into_inner()
    });

    let Some(map) = global_scope_map.as_ref() else {
        // on failure, return false because it's not <= LEVEL_ENABLED_MAX_STATIC
        return is_enabled_by_default;
    };

    if map.is_empty() {
        // if no scopes are enabled, return false because it's not <= LEVEL_ENABLED_MAX_STATIC
        return is_enabled_by_default;
    }
    let enabled_status = map.is_enabled(scope, module_path, level);
    match enabled_status {
        EnabledStatus::NotConfigured => is_enabled_by_default,
        EnabledStatus::Enabled => true,
        EnabledStatus::Disabled => false,
    }
}

pub fn refresh_from_settings(settings: &HashMap<String, String>) {
    let env_config = ENV_FILTER.get();
    let map_new = ScopeMap::new_from_settings_and_env(settings, env_config, DEFAULT_FILTERS);
    let mut level_enabled_max = LEVEL_ENABLED_MAX_STATIC.load(Ordering::Acquire);
    for entry in &map_new.entries {
        if let Some(level) = entry.enabled {
            level_enabled_max = level_enabled_max.max(level as u8);
        }
    }
    LEVEL_ENABLED_MAX_CONFIG.store(level_enabled_max, Ordering::Release);

    {
        let mut global_map = SCOPE_MAP.write().unwrap_or_else(|err| {
            SCOPE_MAP.clear_poison();
            err.into_inner()
        });
        global_map.replace(map_new);
    }
    log::trace!("Log configuration updated");
}

fn level_filter_from_str(level_str: &str) -> Option<log::LevelFilter> {
    use log::LevelFilter::*;
    let level = match level_str.to_ascii_lowercase().as_str() {
        "" => Trace,
        "trace" => Trace,
        "debug" => Debug,
        "info" => Info,
        "warn" => Warn,
        "error" => Error,
        "off" => Off,
        "disable" | "no" | "none" | "disabled" => {
            crate::warn!(
                "Invalid log level \"{level_str}\", to disable logging set to \"off\". Defaulting to \"off\"."
            );
            Off
        }
        _ => {
            crate::warn!("Invalid log level \"{level_str}\", ignoring");
            return None;
        }
    };
    Some(level)
}

fn scope_alloc_from_scope_str(scope_str: &str) -> Option<ScopeAlloc> {
    let mut scope_buf = [""; SCOPE_DEPTH_MAX];
    let mut index = 0;
    let mut scope_iter = scope_str.split(SCOPE_STRING_SEP_STR);
    while index < SCOPE_DEPTH_MAX {
        let Some(scope) = scope_iter.next() else {
            break;
        };
        if scope.is_empty() {
            continue;
        }
        scope_buf[index] = scope;
        index += 1;
    }
    if index == 0 {
        return None;
    }
    if scope_iter.next().is_some() {
        crate::warn!(
            "Invalid scope key, too many nested scopes: '{scope_str}'. Max depth is {SCOPE_DEPTH_MAX}",
        );
        return None;
    }
    let scope = scope_buf.map(|s| s.to_string());
    Some(scope)
}

#[derive(Debug, PartialEq, Eq)]
pub struct ScopeMap {
    entries: Vec<ScopeMapEntry>,
    modules: Vec<(String, log::LevelFilter)>,
    root_count: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ScopeMapEntry {
    scope: String,
    enabled: Option<log::LevelFilter>,
    descendants: std::ops::Range<usize>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnabledStatus {
    Enabled,
    Disabled,
    NotConfigured,
}

impl ScopeMap {
    pub fn new_from_settings_and_env(
        items_input_map: &HashMap<String, String>,
        env_config: Option<&env_config::EnvFilter>,
        default_filters: &[(&str, log::LevelFilter)],
    ) -> Self {
        let mut items = Vec::<(ScopeAlloc, log::LevelFilter)>::with_capacity(
            items_input_map.len()
                + env_config.map_or(0, |c| c.directive_names.len())
                + default_filters.len(),
        );
        let mut modules = Vec::with_capacity(4);

        let env_filters = env_config.iter().flat_map(|env_filter| {
            env_filter
                .directive_names
                .iter()
                .zip(env_filter.directive_levels.iter())
                .map(|(scope_str, level_filter)| (scope_str.as_str(), *level_filter))
        });

        let new_filters = items_input_map.iter().filter_map(|(scope_str, level_str)| {
            let level_filter = level_filter_from_str(level_str)?;
            Some((scope_str.as_str(), level_filter))
        });

        let all_filters = default_filters
            .iter()
            .cloned()
            .chain(env_filters)
            .chain(new_filters);

        for (scope_str, level_filter) in all_filters {
            if scope_str.contains("::") {
                if let Some(idx) = modules.iter().position(|(module, _)| module == scope_str) {
                    modules[idx].1 = level_filter;
                } else {
                    modules.push((scope_str.to_string(), level_filter));
                }
                continue;
            }
            let Some(scope) = scope_alloc_from_scope_str(scope_str) else {
                continue;
            };
            if let Some(idx) = items
                .iter()
                .position(|(scope_existing, _)| scope_existing == &scope)
            {
                items[idx].1 = level_filter;
            } else {
                items.push((scope, level_filter));
            }
        }

        items.sort_by(|a, b| a.0.cmp(&b.0));
        modules.sort_by(|(a_name, _), (b_name, _)| a_name.cmp(b_name));

        let mut this = Self {
            entries: Vec::with_capacity(items.len() * SCOPE_DEPTH_MAX),
            modules,
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
                if scope_name.is_empty() {
                    assert_eq!(sub_items_start + 1, sub_items_end);
                    assert_ne!(depth, 0);
                    assert_ne!(parent_index, usize::MAX);
                    assert!(this.entries[parent_index].enabled.is_none());
                    this.entries[parent_index].enabled = Some(items[sub_items_start].1);
                    continue;
                }
                let is_valid_scope = !scope_name.is_empty();
                let is_last = depth + 1 == SCOPE_DEPTH_MAX || !is_valid_scope;
                let mut enabled = None;
                if is_last {
                    assert_eq!(
                        sub_items_start + 1,
                        sub_items_end,
                        "Expected one item: got: {:?}",
                        &items[items_range]
                    );
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

        this
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty() && self.modules.is_empty()
    }

    pub fn is_enabled<S>(
        &self,
        scope: &[S; SCOPE_DEPTH_MAX],
        module_path: Option<&str>,
        level: log::Level,
    ) -> EnabledStatus
    where
        S: AsRef<str>,
    {
        fn search<S>(map: &ScopeMap, scope: &[S; SCOPE_DEPTH_MAX]) -> Option<log::LevelFilter>
        where
            S: AsRef<str>,
        {
            let mut enabled = None;
            let mut cur_range = &map.entries[0..map.root_count];
            let mut depth = 0;
            'search: while !cur_range.is_empty()
                && depth < SCOPE_DEPTH_MAX
                && scope[depth].as_ref() != ""
            {
                for entry in cur_range {
                    if entry.scope == scope[depth].as_ref() {
                        enabled = entry.enabled.or(enabled);
                        cur_range = &map.entries[entry.descendants.clone()];
                        depth += 1;
                        continue 'search;
                    }
                }
                break 'search;
            }
            enabled
        }

        let mut enabled = search(self, scope);

        if let Some(module_path) = module_path {
            let scope_is_empty = scope[0].as_ref().is_empty();

            if enabled.is_none() && scope_is_empty {
                let crate_name = private::extract_crate_name_from_module_path(module_path);
                let mut crate_name_scope = [""; SCOPE_DEPTH_MAX];
                crate_name_scope[0] = crate_name;
                enabled = search(self, &crate_name_scope);
            }

            if !self.modules.is_empty() {
                let crate_name = private::extract_crate_name_from_module_path(module_path);
                let is_scope_just_crate_name =
                    scope[0].as_ref() == crate_name && scope[1].as_ref() == "";
                if enabled.is_none() || is_scope_just_crate_name {
                    for (module, filter) in &self.modules {
                        if module == module_path {
                            enabled.replace(*filter);
                            break;
                        }
                    }
                }
            }
        }

        if let Some(enabled_filter) = enabled {
            if level <= enabled_filter {
                return EnabledStatus::Enabled;
            }
            return EnabledStatus::Disabled;
        }
        EnabledStatus::NotConfigured
    }
}

#[cfg(test)]
mod tests {
    use log::LevelFilter;

    use crate::private::scope_new;

    use super::*;

    fn scope_map_from_keys(kv: &[(&str, &str)]) -> ScopeMap {
        let hash_map: HashMap<String, String> = kv
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();
        ScopeMap::new_from_settings_and_env(&hash_map, None, &[])
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
        let mut scope_iter = scope_str.split(SCOPE_STRING_SEP_STR);
        while index < SCOPE_DEPTH_MAX {
            let Some(scope) = scope_iter.next() else {
                break;
            };
            if scope.is_empty() {
                continue;
            }
            scope_buf[index] = scope;
            index += 1;
        }
        assert_ne!(index, 0);
        assert!(scope_iter.next().is_none());
        scope_buf
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
        use log::Level;
        assert_eq!(
            map.is_enabled(&scope_from_scope_str("a.b.c.d"), None, Level::Trace),
            EnabledStatus::Enabled
        );
        assert_eq!(
            map.is_enabled(&scope_from_scope_str("a.b.c.d"), None, Level::Debug),
            EnabledStatus::Enabled
        );

        assert_eq!(
            map.is_enabled(&scope_from_scope_str("e.f.g.h"), None, Level::Debug),
            EnabledStatus::Enabled
        );
        assert_eq!(
            map.is_enabled(&scope_from_scope_str("e.f.g.h"), None, Level::Info),
            EnabledStatus::Enabled
        );
        assert_eq!(
            map.is_enabled(&scope_from_scope_str("e.f.g.h"), None, Level::Trace),
            EnabledStatus::Disabled
        );

        assert_eq!(
            map.is_enabled(&scope_from_scope_str("i.j.k.l"), None, Level::Info),
            EnabledStatus::Enabled
        );
        assert_eq!(
            map.is_enabled(&scope_from_scope_str("i.j.k.l"), None, Level::Warn),
            EnabledStatus::Enabled
        );
        assert_eq!(
            map.is_enabled(&scope_from_scope_str("i.j.k.l"), None, Level::Debug),
            EnabledStatus::Disabled
        );

        assert_eq!(
            map.is_enabled(&scope_from_scope_str("m.n.o.p"), None, Level::Warn),
            EnabledStatus::Enabled
        );
        assert_eq!(
            map.is_enabled(&scope_from_scope_str("m.n.o.p"), None, Level::Error),
            EnabledStatus::Enabled
        );
        assert_eq!(
            map.is_enabled(&scope_from_scope_str("m.n.o.p"), None, Level::Info),
            EnabledStatus::Disabled
        );

        assert_eq!(
            map.is_enabled(&scope_from_scope_str("q.r.s.t"), None, Level::Error),
            EnabledStatus::Enabled
        );
        assert_eq!(
            map.is_enabled(&scope_from_scope_str("q.r.s.t"), None, Level::Warn),
            EnabledStatus::Disabled
        );
    }

    #[test]
    fn test_is_enabled_module() {
        let mut map = scope_map_from_keys(&[("a", "trace")]);
        map.modules = [("a::b::c", "trace"), ("a::b::d", "debug")]
            .map(|(k, v)| (k.to_string(), v.parse().unwrap()))
            .to_vec();
        use log::Level;
        assert_eq!(
            map.is_enabled(
                &scope_from_scope_str("__unused__"),
                Some("a::b::c"),
                Level::Trace
            ),
            EnabledStatus::Enabled
        );
        assert_eq!(
            map.is_enabled(
                &scope_from_scope_str("__unused__"),
                Some("a::b::d"),
                Level::Debug
            ),
            EnabledStatus::Enabled
        );
        assert_eq!(
            map.is_enabled(
                &scope_from_scope_str("__unused__"),
                Some("a::b::d"),
                Level::Trace,
            ),
            EnabledStatus::Disabled
        );
        assert_eq!(
            map.is_enabled(
                &scope_from_scope_str("__unused__"),
                Some("a::e"),
                Level::Info
            ),
            EnabledStatus::NotConfigured
        );
        // when scope is just crate name, more specific module path overrides it
        assert_eq!(
            map.is_enabled(&scope_from_scope_str("a"), Some("a::b::d"), Level::Trace),
            EnabledStatus::Disabled,
        );
        // but when it is scoped, the scope overrides the module path
        assert_eq!(
            map.is_enabled(
                &scope_from_scope_str("a.scope"),
                Some("a::b::d"),
                Level::Trace
            ),
            EnabledStatus::Enabled,
        );
    }

    fn scope_map_from_keys_and_env(kv: &[(&str, &str)], env: &env_config::EnvFilter) -> ScopeMap {
        let hash_map: HashMap<String, String> = kv
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();
        ScopeMap::new_from_settings_and_env(&hash_map, Some(env), &[])
    }

    #[test]
    fn test_initialization_with_env() {
        let env_filter = env_config::parse("a.b=debug,u=error").unwrap();
        let map = scope_map_from_keys_and_env(&[], &env_filter);
        assert_eq!(map.root_count, 2);
        assert_eq!(map.entries.len(), 3);
        assert_eq!(
            map.is_enabled(&scope_new(&["a"]), None, log::Level::Debug),
            EnabledStatus::NotConfigured
        );
        assert_eq!(
            map.is_enabled(&scope_new(&["a", "b"]), None, log::Level::Debug),
            EnabledStatus::Enabled
        );
        assert_eq!(
            map.is_enabled(&scope_new(&["a", "b", "c"]), None, log::Level::Trace),
            EnabledStatus::Disabled
        );

        let env_filter = env_config::parse("a.b=debug,e.f.g.h=trace,u=error").unwrap();
        let map = scope_map_from_keys_and_env(
            &[
                ("a.b.c.d", "trace"),
                ("e.f.g.h", "debug"),
                ("i.j.k.l", "info"),
                ("m.n.o.p", "warn"),
                ("q.r.s.t", "error"),
            ],
            &env_filter,
        );
        assert_eq!(map.root_count, 6);
        assert_eq!(map.entries.len(), 21);
        assert_eq!(map.entries[0].scope, "a");
        assert_eq!(map.entries[1].scope, "e");
        assert_eq!(map.entries[2].scope, "i");
        assert_eq!(map.entries[3].scope, "m");
        assert_eq!(map.entries[4].scope, "q");
        assert_eq!(map.entries[5].scope, "u");
        assert_eq!(
            map.is_enabled(&scope_new(&["a", "b", "c", "d"]), None, log::Level::Trace),
            EnabledStatus::Enabled
        );
        assert_eq!(
            map.is_enabled(&scope_new(&["a", "b", "c"]), None, log::Level::Trace),
            EnabledStatus::Disabled
        );
        assert_eq!(
            map.is_enabled(&scope_new(&["u", "v"]), None, log::Level::Warn),
            EnabledStatus::Disabled
        );
        // settings override env
        assert_eq!(
            map.is_enabled(&scope_new(&["e", "f", "g", "h"]), None, log::Level::Trace),
            EnabledStatus::Disabled,
        );
    }

    fn scope_map_from_all(
        kv: &[(&str, &str)],
        env: &env_config::EnvFilter,
        default_filters: &[(&str, log::LevelFilter)],
    ) -> ScopeMap {
        let hash_map: HashMap<String, String> = kv
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();
        ScopeMap::new_from_settings_and_env(&hash_map, Some(env), default_filters)
    }

    #[test]
    fn precedence() {
        // Test precedence: kv > env > default

        // Default filters - these should be overridden by env and kv when they overlap
        let default_filters = &[
            ("a.b.c", log::LevelFilter::Debug), // Should be overridden by env
            ("p.q.r", log::LevelFilter::Info),  // Should be overridden by kv
            ("x.y.z", log::LevelFilter::Warn),  // Not overridden
            ("crate::module::default", log::LevelFilter::Error), // Module in default
            ("crate::module::user", log::LevelFilter::Off), // Module disabled in default
        ];

        // Environment filters - these should override default but be overridden by kv
        let env_filter =
            env_config::parse("a.b.c=trace,p.q=debug,m.n.o=error,crate::module::env=debug")
                .unwrap();

        // Key-value filters (highest precedence) - these should override everything
        let kv_filters = &[
            ("p.q.r", "trace"),              // Overrides default
            ("m.n.o", "warn"),               // Overrides env
            ("j.k.l", "info"),               // New filter
            ("crate::module::env", "trace"), // Overrides env for module
            ("crate::module::kv", "trace"),  // New module filter
        ];

        let map = scope_map_from_all(kv_filters, &env_filter, default_filters);

        // Test scope precedence
        use log::Level;

        // KV overrides all for scopes
        assert_eq!(
            map.is_enabled(&scope_from_scope_str("p.q.r"), None, Level::Trace),
            EnabledStatus::Enabled,
            "KV should override default filters for scopes"
        );
        assert_eq!(
            map.is_enabled(&scope_from_scope_str("m.n.o"), None, Level::Warn),
            EnabledStatus::Enabled,
            "KV should override env filters for scopes"
        );
        assert_eq!(
            map.is_enabled(&scope_from_scope_str("m.n.o"), None, Level::Debug),
            EnabledStatus::Disabled,
            "KV correctly limits log level"
        );

        // ENV overrides default but not KV for scopes
        assert_eq!(
            map.is_enabled(&scope_from_scope_str("a.b.c"), None, Level::Trace),
            EnabledStatus::Enabled,
            "ENV should override default filters for scopes"
        );

        // Default is used when no override exists for scopes
        assert_eq!(
            map.is_enabled(&scope_from_scope_str("x.y.z"), None, Level::Warn),
            EnabledStatus::Enabled,
            "Default filters should work when not overridden"
        );
        assert_eq!(
            map.is_enabled(&scope_from_scope_str("x.y.z"), None, Level::Info),
            EnabledStatus::Disabled,
            "Default filters correctly limit log level"
        );

        // KV overrides all for modules
        assert_eq!(
            map.is_enabled(&scope_new(&[""]), Some("crate::module::env"), Level::Trace),
            EnabledStatus::Enabled,
            "KV should override env filters for modules"
        );
        assert_eq!(
            map.is_enabled(&scope_new(&[""]), Some("crate::module::kv"), Level::Trace),
            EnabledStatus::Enabled,
            "KV module filters should work"
        );

        // ENV overrides default for modules
        assert_eq!(
            map.is_enabled(&scope_new(&[""]), Some("crate::module::env"), Level::Debug),
            EnabledStatus::Enabled,
            "ENV should override default for modules"
        );

        // Default is used when no override exists for modules
        assert_eq!(
            map.is_enabled(
                &scope_new(&[""]),
                Some("crate::module::default"),
                Level::Error
            ),
            EnabledStatus::Enabled,
            "Default filters should work for modules"
        );
        assert_eq!(
            map.is_enabled(
                &scope_new(&[""]),
                Some("crate::module::default"),
                Level::Warn
            ),
            EnabledStatus::Disabled,
            "Default filters correctly limit log level for modules"
        );

        assert_eq!(
            map.is_enabled(&scope_new(&[""]), Some("crate::module::user"), Level::Error),
            EnabledStatus::Disabled,
            "Module turned off in default filters is not enabled"
        );

        assert_eq!(
            map.is_enabled(
                &scope_new(&["crate"]),
                Some("crate::module::user"),
                Level::Error
            ),
            EnabledStatus::Disabled,
            "Module turned off in default filters is not enabled, even with crate name as scope"
        );

        // Test non-conflicting but similar paths

        // Test that "a.b" and "a.b.c" don't conflict (different depth)
        assert_eq!(
            map.is_enabled(&scope_from_scope_str("a.b.c.d"), None, Level::Trace),
            EnabledStatus::Enabled,
            "Scope a.b.c should inherit from a.b env filter"
        );
        assert_eq!(
            map.is_enabled(&scope_from_scope_str("a.b.c"), None, Level::Trace),
            EnabledStatus::Enabled,
            "Scope a.b.c.d should use env filter level (trace)"
        );

        // Test that similar module paths don't conflict
        assert_eq!(
            map.is_enabled(&scope_new(&[""]), Some("crate::module"), Level::Error),
            EnabledStatus::NotConfigured,
            "Module crate::module should not be affected by crate::module::default filter"
        );
        assert_eq!(
            map.is_enabled(
                &scope_new(&[""]),
                Some("crate::module::default::sub"),
                Level::Error
            ),
            EnabledStatus::NotConfigured,
            "Module crate::module::default::sub should not be affected by crate::module::default filter"
        );
    }

    #[test]
    fn default_filter_crate() {
        let default_filters = &[("crate", LevelFilter::Off)];
        let map = scope_map_from_all(&[], &env_config::parse("").unwrap(), default_filters);

        use log::Level;
        assert_eq!(
            map.is_enabled(&scope_new(&[""]), Some("crate::submodule"), Level::Error),
            EnabledStatus::Disabled,
            "crate::submodule should be disabled by disabling `crate` filter"
        );
    }
}
