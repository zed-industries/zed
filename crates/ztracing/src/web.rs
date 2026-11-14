use std::{
    cell::{Cell, OnceCell},
    collections::HashMap,
    sync::{
        Arc,
        atomic::{AtomicU32, AtomicUsize, Ordering},
    },
};

use tracing::{Id, Subscriber, span::Attributes};
use tracing_subscriber::{Layer, registry::LookupSpan};
use wasm_bindgen::{JsCast, JsValue, prelude::wasm_bindgen};
use web_sys::Performance;

const EVENT_BUFFER_CAPACITY: usize = 65_536;
const REPORT_BATCH_SIZE: usize = 4_096;

static NEXT_SOURCE_THREAD_ID: AtomicU32 = AtomicU32::new(0);

thread_local! {
    static PERFORMANCE: OnceCell<Option<Performance>> = const { OnceCell::new() };
    static SOURCE_THREAD_ID: Cell<Option<u32>> = const { Cell::new(None) };
}

#[wasm_bindgen]
extern "C" {
    type PerformanceMeasureApi;

    #[wasm_bindgen(catch, method, js_class = "Performance", js_name = measure)]
    fn measure_with_options(
        this: &PerformanceMeasureApi,
        measure_name: &str,
        options: &JsValue,
    ) -> Result<JsValue, JsValue>;
}

pub struct PerformanceLayer {
    event_sender: async_channel::Sender<Event>,
    dropped_event_count: Arc<AtomicUsize>,
}

pub struct PerformanceReporter {
    event_receiver: async_channel::Receiver<Event>,
    dropped_event_count: Arc<AtomicUsize>,
}

enum Event {
    NewSpan {
        span_id: u64,
        target: &'static str,
        name: &'static str,
    },
    Enter {
        span_id: u64,
        source_thread_id: u32,
        timestamp: f64,
    },
    Exit {
        span_id: u64,
        source_thread_id: u32,
        timestamp: f64,
    },
    Close {
        span_id: u64,
    },
}

struct SpanState {
    display_name: String,
    active_entries: HashMap<u32, Vec<f64>>,
}

struct CompletedMeasure {
    name: String,
    source_thread_id: u32,
    start: f64,
    duration: f64,
}

#[derive(Default)]
struct ReporterState {
    spans: HashMap<u64, SpanState>,
}

pub fn performance_layer() -> (PerformanceLayer, PerformanceReporter) {
    let (event_sender, event_receiver) = async_channel::bounded(EVENT_BUFFER_CAPACITY);
    let dropped_event_count = Arc::new(AtomicUsize::new(0));
    (
        PerformanceLayer {
            event_sender,
            dropped_event_count: dropped_event_count.clone(),
        },
        PerformanceReporter {
            event_receiver,
            dropped_event_count,
        },
    )
}

impl PerformanceLayer {
    pub fn dropped_event_count(&self) -> usize {
        self.dropped_event_count.load(Ordering::Relaxed)
    }

    fn send(&self, event: Event) {
        if self.event_sender.try_send(event).is_err() {
            self.dropped_event_count.fetch_add(1, Ordering::Relaxed);
        }
    }
}

impl<S> Layer<S> for PerformanceLayer
where
    S: Subscriber + for<'lookup> LookupSpan<'lookup>,
{
    fn on_new_span(
        &self,
        _attributes: &Attributes<'_>,
        id: &Id,
        context: tracing_subscriber::layer::Context<'_, S>,
    ) {
        let Some(span) = context.span(id) else {
            return;
        };
        let metadata = span.metadata();
        self.send(Event::NewSpan {
            span_id: id.into_u64(),
            target: metadata.target(),
            name: metadata.name(),
        });
    }

    fn on_enter(&self, id: &Id, _context: tracing_subscriber::layer::Context<'_, S>) {
        let Some(timestamp) = absolute_now() else {
            return;
        };
        self.send(Event::Enter {
            span_id: id.into_u64(),
            source_thread_id: source_thread_id(),
            timestamp,
        });
    }

    fn on_exit(&self, id: &Id, _context: tracing_subscriber::layer::Context<'_, S>) {
        let Some(timestamp) = absolute_now() else {
            return;
        };
        self.send(Event::Exit {
            span_id: id.into_u64(),
            source_thread_id: source_thread_id(),
            timestamp,
        });
    }

    fn on_close(&self, id: Id, _context: tracing_subscriber::layer::Context<'_, S>) {
        self.send(Event::Close {
            span_id: id.into_u64(),
        });
    }
}

impl PerformanceReporter {
    pub async fn run(self) {
        let Some(performance) = performance() else {
            web_sys::console::error_1(&JsValue::from_str(
                "browser performance tracing requires globalThis.performance",
            ));
            return;
        };
        let reporter_time_origin = performance.time_origin();
        let mut state = ReporterState::default();
        let mut completed_measures = Vec::new();
        let mut reported_dropped_event_count = 0;

        while let Ok(event) = self.event_receiver.recv().await {
            state.process(event, &mut completed_measures);
            let mut processed_event_count = 1;
            while processed_event_count < REPORT_BATCH_SIZE {
                let Ok(event) = self.event_receiver.try_recv() else {
                    break;
                };
                state.process(event, &mut completed_measures);
                processed_event_count += 1;
            }

            for measure in completed_measures.drain(..) {
                report_measure(&performance, reporter_time_origin, measure);
            }

            let dropped_event_count = self.dropped_event_count.load(Ordering::Relaxed);
            if dropped_event_count != reported_dropped_event_count {
                web_sys::console::warn_1(&JsValue::from_str(&format!(
                    "browser performance tracing dropped {} events",
                    dropped_event_count - reported_dropped_event_count
                )));
                reported_dropped_event_count = dropped_event_count;
            }
        }
    }
}

impl ReporterState {
    fn process(&mut self, event: Event, completed_measures: &mut Vec<CompletedMeasure>) {
        match event {
            Event::NewSpan {
                span_id,
                target,
                name,
            } => {
                self.spans.insert(
                    span_id,
                    SpanState {
                        display_name: format!("{target}::{name}"),
                        active_entries: HashMap::new(),
                    },
                );
            }
            Event::Enter {
                span_id,
                source_thread_id,
                timestamp,
            } => {
                let Some(span) = self.spans.get_mut(&span_id) else {
                    return;
                };
                span.active_entries
                    .entry(source_thread_id)
                    .or_default()
                    .push(timestamp);
            }
            Event::Exit {
                span_id,
                source_thread_id,
                timestamp,
            } => {
                let Some(span) = self.spans.get_mut(&span_id) else {
                    return;
                };
                let Some(active_entries) = span.active_entries.get_mut(&source_thread_id) else {
                    return;
                };
                let Some(start) = active_entries.pop() else {
                    return;
                };
                if active_entries.is_empty() {
                    span.active_entries.remove(&source_thread_id);
                }
                completed_measures.push(CompletedMeasure {
                    name: span.display_name.clone(),
                    source_thread_id,
                    start,
                    duration: (timestamp - start).max(0.0),
                });
            }
            Event::Close { span_id } => {
                self.spans.remove(&span_id);
            }
        }
    }
}

fn absolute_now() -> Option<f64> {
    with_performance(|performance| performance.time_origin() + performance.now())
}

fn source_thread_id() -> u32 {
    SOURCE_THREAD_ID.with(|source_thread_id| {
        if let Some(source_thread_id) = source_thread_id.get() {
            source_thread_id
        } else {
            let new_source_thread_id = NEXT_SOURCE_THREAD_ID.fetch_add(1, Ordering::Relaxed);
            source_thread_id.set(Some(new_source_thread_id));
            new_source_thread_id
        }
    })
}

fn performance() -> Option<Performance> {
    with_performance(Clone::clone)
}

fn with_performance<T>(callback: impl FnOnce(&Performance) -> T) -> Option<T> {
    PERFORMANCE.with(|performance| {
        performance
            .get_or_init(|| {
                js_sys::Reflect::get(&js_sys::global(), &JsValue::from_str("performance"))
                    .ok()
                    .and_then(|performance| performance.dyn_into().ok())
            })
            .as_ref()
            .map(callback)
    })
}

fn report_measure(performance: &Performance, reporter_time_origin: f64, measure: CompletedMeasure) {
    let start = measure.start - reporter_time_origin;
    if start < 0.0 {
        web_sys::console::warn_1(&JsValue::from_str(&format!(
            "browser performance tracing dropped {} because it predates the reporter worker",
            measure.name
        )));
        return;
    }

    let options = js_sys::Object::new();
    if let Err(error) = js_sys::Reflect::set(
        &options,
        &JsValue::from_str("start"),
        &JsValue::from_f64(start),
    ) {
        log_performance_error("set measure start", error);
        return;
    }
    if let Err(error) = js_sys::Reflect::set(
        &options,
        &JsValue::from_str("duration"),
        &JsValue::from_f64(measure.duration),
    ) {
        log_performance_error("set measure duration", error);
        return;
    }
    let detail = js_sys::Object::new();
    if let Err(error) = js_sys::Reflect::set(
        &detail,
        &JsValue::from_str("sourceThreadId"),
        &JsValue::from_f64(f64::from(measure.source_thread_id)),
    ) {
        log_performance_error("set measure source thread", error);
        return;
    }
    if let Err(error) =
        js_sys::Reflect::set(&options, &JsValue::from_str("detail"), detail.as_ref())
    {
        log_performance_error("set measure detail", error);
        return;
    }
    let performance: &PerformanceMeasureApi = performance.unchecked_ref();
    if let Err(error) = performance.measure_with_options(&measure.name, options.as_ref()) {
        log_performance_error("measure", error);
    }
}

fn log_performance_error(operation: &str, error: JsValue) {
    web_sys::console::error_2(
        &JsValue::from_str(&format!("performance.{operation} failed")),
        &error,
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pairs_span_events() {
        let mut state = ReporterState::default();
        let mut completed_measures = Vec::new();

        state.process(
            Event::NewSpan {
                span_id: 1,
                target: "editor",
                name: "layout",
            },
            &mut completed_measures,
        );
        state.process(
            Event::Enter {
                span_id: 1,
                source_thread_id: 2,
                timestamp: 10.0,
            },
            &mut completed_measures,
        );
        state.process(
            Event::Exit {
                span_id: 1,
                source_thread_id: 2,
                timestamp: 15.0,
            },
            &mut completed_measures,
        );

        assert_eq!(completed_measures.len(), 1);
        let Some(measure) = completed_measures.first() else {
            return;
        };
        assert_eq!(measure.name, "editor::layout");
        assert_eq!(measure.source_thread_id, 2);
        assert_eq!(measure.start, 10.0);
        assert_eq!(measure.duration, 5.0);
    }

    #[test]
    fn pairs_reentrant_spans_in_stack_order() {
        let mut state = ReporterState::default();
        let mut completed_measures = Vec::new();

        state.process(
            Event::NewSpan {
                span_id: 1,
                target: "editor",
                name: "layout",
            },
            &mut completed_measures,
        );
        for timestamp in [10.0, 12.0] {
            state.process(
                Event::Enter {
                    span_id: 1,
                    source_thread_id: 2,
                    timestamp,
                },
                &mut completed_measures,
            );
        }
        for timestamp in [14.0, 16.0] {
            state.process(
                Event::Exit {
                    span_id: 1,
                    source_thread_id: 2,
                    timestamp,
                },
                &mut completed_measures,
            );
        }

        assert_eq!(completed_measures.len(), 2);
        let mut measures = completed_measures.iter();
        let Some(inner_measure) = measures.next() else {
            return;
        };
        let Some(outer_measure) = measures.next() else {
            return;
        };
        assert_eq!(inner_measure.start, 12.0);
        assert_eq!(inner_measure.duration, 2.0);
        assert_eq!(outer_measure.start, 10.0);
        assert_eq!(outer_measure.duration, 6.0);
    }
}
