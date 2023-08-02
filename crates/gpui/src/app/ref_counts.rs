#[cfg(any(test, feature = "test-support"))]
use std::sync::Arc;

use lazy_static::lazy_static;
#[cfg(any(test, feature = "test-support"))]
use parking_lot::Mutex;

use collections::{hash_map::Entry, HashMap, HashSet};

#[cfg(any(test, feature = "test-support"))]
use crate::util::post_inc;
use crate::ElementStateId;

lazy_static! {
    static ref LEAK_BACKTRACE: bool =
        std::env::var("LEAK_BACKTRACE").map_or(false, |b| !b.is_empty());
}

struct ElementStateRefCount {
    ref_count: usize,
    frame_id: usize,
}

#[derive(Default)]
pub struct RefCounts {
    entity_counts: HashMap<usize, usize>,
    element_state_counts: HashMap<ElementStateId, ElementStateRefCount>,
    dropped_windows: HashSet<usize>,
    dropped_models: HashSet<usize>,
    dropped_views: HashSet<(usize, usize)>,
    dropped_element_states: HashSet<ElementStateId>,

    #[cfg(any(test, feature = "test-support"))]
    pub leak_detector: Arc<Mutex<LeakDetector>>,
}

impl RefCounts {
    #[cfg(any(test, feature = "test-support"))]
    pub fn new(leak_detector: Arc<Mutex<LeakDetector>>) -> Self {
        Self {
            #[cfg(any(test, feature = "test-support"))]
            leak_detector,
            ..Default::default()
        }
    }

    pub fn inc_window(&mut self, window_id: usize) {
        match self.entity_counts.entry(window_id) {
            Entry::Occupied(mut entry) => {
                *entry.get_mut() += 1;
            }
            Entry::Vacant(entry) => {
                entry.insert(1);
                self.dropped_windows.remove(&window_id);
            }
        }
    }

    pub fn inc_model(&mut self, model_id: usize) {
        match self.entity_counts.entry(model_id) {
            Entry::Occupied(mut entry) => {
                *entry.get_mut() += 1;
            }
            Entry::Vacant(entry) => {
                entry.insert(1);
                self.dropped_models.remove(&model_id);
            }
        }
    }

    pub fn inc_view(&mut self, window_id: usize, view_id: usize) {
        match self.entity_counts.entry(view_id) {
            Entry::Occupied(mut entry) => *entry.get_mut() += 1,
            Entry::Vacant(entry) => {
                entry.insert(1);
                self.dropped_views.remove(&(window_id, view_id));
            }
        }
    }

    pub fn inc_element_state(&mut self, id: ElementStateId, frame_id: usize) {
        match self.element_state_counts.entry(id) {
            Entry::Occupied(mut entry) => {
                let entry = entry.get_mut();
                if entry.frame_id == frame_id || entry.ref_count >= 2 {
                    panic!("used the same element state more than once in the same frame");
                }
                entry.ref_count += 1;
                entry.frame_id = frame_id;
            }
            Entry::Vacant(entry) => {
                entry.insert(ElementStateRefCount {
                    ref_count: 1,
                    frame_id,
                });
                self.dropped_element_states.remove(&id);
            }
        }
    }

    pub fn dec_window(&mut self, window_id: usize) {
        let count = self.entity_counts.get_mut(&window_id).unwrap();
        *count -= 1;
        if *count == 0 {
            self.entity_counts.remove(&window_id);
            self.dropped_windows.insert(window_id);
        }
    }

    pub fn dec_model(&mut self, model_id: usize) {
        let count = self.entity_counts.get_mut(&model_id).unwrap();
        *count -= 1;
        if *count == 0 {
            self.entity_counts.remove(&model_id);
            self.dropped_models.insert(model_id);
        }
    }

    pub fn dec_view(&mut self, window_id: usize, view_id: usize) {
        let count = self.entity_counts.get_mut(&view_id).unwrap();
        *count -= 1;
        if *count == 0 {
            self.entity_counts.remove(&view_id);
            self.dropped_views.insert((window_id, view_id));
        }
    }

    pub fn dec_element_state(&mut self, id: ElementStateId) {
        let entry = self.element_state_counts.get_mut(&id).unwrap();
        entry.ref_count -= 1;
        if entry.ref_count == 0 {
            self.element_state_counts.remove(&id);
            self.dropped_element_states.insert(id);
        }
    }

    pub fn is_entity_alive(&self, entity_id: usize) -> bool {
        self.entity_counts.contains_key(&entity_id)
    }

    pub fn take_dropped(
        &mut self,
    ) -> (
        HashSet<usize>,
        HashSet<(usize, usize)>,
        HashSet<ElementStateId>,
    ) {
        (
            std::mem::take(&mut self.dropped_models),
            std::mem::take(&mut self.dropped_views),
            std::mem::take(&mut self.dropped_element_states),
        )
    }
}

#[cfg(any(test, feature = "test-support"))]
#[derive(Default)]
pub struct LeakDetector {
    next_handle_id: usize,
    #[allow(clippy::type_complexity)]
    handle_backtraces: HashMap<
        usize,
        (
            Option<&'static str>,
            HashMap<usize, Option<backtrace::Backtrace>>,
        ),
    >,
}

#[cfg(any(test, feature = "test-support"))]
impl LeakDetector {
    pub fn handle_created(&mut self, type_name: Option<&'static str>, entity_id: usize) -> usize {
        let handle_id = post_inc(&mut self.next_handle_id);
        let entry = self.handle_backtraces.entry(entity_id).or_default();
        let backtrace = if *LEAK_BACKTRACE {
            Some(backtrace::Backtrace::new_unresolved())
        } else {
            None
        };
        if let Some(type_name) = type_name {
            entry.0.get_or_insert(type_name);
        }
        entry.1.insert(handle_id, backtrace);
        handle_id
    }

    pub fn handle_dropped(&mut self, entity_id: usize, handle_id: usize) {
        if let Some((_, backtraces)) = self.handle_backtraces.get_mut(&entity_id) {
            assert!(backtraces.remove(&handle_id).is_some());
            if backtraces.is_empty() {
                self.handle_backtraces.remove(&entity_id);
            }
        }
    }

    pub fn assert_dropped(&mut self, entity_id: usize) {
        if let Some((type_name, backtraces)) = self.handle_backtraces.get_mut(&entity_id) {
            for trace in backtraces.values_mut().flatten() {
                trace.resolve();
                eprintln!("{:?}", crate::util::CwdBacktrace(trace));
            }

            let hint = if *LEAK_BACKTRACE {
                ""
            } else {
                " – set LEAK_BACKTRACE=1 for more information"
            };

            panic!(
                "{} handles to {} {} still exist{}",
                backtraces.len(),
                type_name.unwrap_or("entity"),
                entity_id,
                hint
            );
        }
    }

    pub fn detect(&mut self) {
        let mut found_leaks = false;
        for (id, (type_name, backtraces)) in self.handle_backtraces.iter_mut() {
            eprintln!(
                "leaked {} handles to {} {}",
                backtraces.len(),
                type_name.unwrap_or("entity"),
                id
            );
            for trace in backtraces.values_mut().flatten() {
                trace.resolve();
                eprintln!("{:?}", crate::util::CwdBacktrace(trace));
            }
            found_leaks = true;
        }

        let hint = if *LEAK_BACKTRACE {
            ""
        } else {
            " – set LEAK_BACKTRACE=1 for more information"
        };
        assert!(!found_leaks, "detected leaked handles{}", hint);
    }
}
