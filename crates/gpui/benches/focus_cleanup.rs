use std::{cell::RefCell, rc::Rc, time::Duration};

use gpui::{AppContext as _, BenchAppContext, Entity, EntityId, FocusHandle, WeakFocusHandle};

const ENTITY_COUNT: usize = 10_000;

#[gpui::bench(
    inputs = [0, 100, 1_000, 10_000],
    input_name = "handles",
    group = "focus_cleanup_no_drops_10000_effects",
    fps = 120
)]
fn no_drops(handle_count: &usize, cx: &mut BenchAppContext) {
    measure(*handle_count, false, 0, cx);
}

#[gpui::bench(
    inputs = [10_000],
    input_name = "handles",
    group = "focus_cleanup_final_drops_10000_effects",
    fps = 120
)]
fn final_drops(handle_count: &usize, cx: &mut BenchAppContext) {
    measure(*handle_count, true, 0, cx);
}

#[gpui::bench(
    inputs = [1, 10],
    input_name = "windows",
    group = "focus_cleanup_final_drops_with_windows_10000_handles_10000_effects",
    fps = 120
)]
fn final_drops_with_windows(window_count: &usize, cx: &mut BenchAppContext) {
    measure(10_000, true, *window_count, cx);
}

fn measure(handle_count: usize, final_drops: bool, window_count: usize, cx: &mut BenchAppContext) {
    assert!(!cfg!(debug_assertions), "use --profile release-fast");
    assert!(window_count <= handle_count);
    let windows = (0..window_count)
        .map(|_| cx.add_empty_window().window_handle())
        .collect::<Vec<_>>();
    let observed = Rc::new(RefCell::new(Vec::with_capacity(ENTITY_COUNT)));
    let _subscription = cx.update(|cx| {
        cx.observe_new::<CreatedEntity>({
            let observed = observed.clone();
            move |_, _, cx| observed.borrow_mut().push(cx.entity_id())
        })
    });
    let prepare = |cx: &mut BenchAppContext| {
        cx.update(|cx| {
            let handles = (0..handle_count)
                .map(|_| cx.focus_handle())
                .collect::<Vec<_>>();
            for (window, handle) in windows.iter().zip(&handles) {
                window
                    .update(cx, |_, window, cx| {
                        window.focus(handle, cx);
                        assert_eq!(window.focused(cx).as_ref(), Some(handle));
                    })
                    .expect("benchmark window missing during focus setup");
            }
            handles
        })
    };
    let run = |handles: Vec<FocusHandle>, cx: &mut BenchAppContext| {
        let observed = observed.clone();
        cx.read(|cx| {
            cx.spawn(async move |cx| {
                let dropped = handles.iter().map(FocusHandle::downgrade).collect();
                let entities = cx.update(|cx| {
                    drop(handles);
                    (0..ENTITY_COUNT)
                        .map(|_| cx.new(|_| CreatedEntity))
                        .collect::<Vec<_>>()
                });
                CompletedBatch {
                    entities,
                    observed,
                    dropped,
                }
            })
        })
    };
    if final_drops {
        cx.bench_batched_task(prepare, |handles, cx| run(std::mem::take(handles), cx));

        for window in windows {
            window
                .update(cx, |_, window, cx| assert_eq!(window.focused(cx), None))
                .expect("benchmark window missing after focus cleanup");
        }
    } else {
        let handles = prepare(cx);
        cx.bench_task(|cx| run(Vec::new(), cx));

        for handle in &handles {
            assert_eq!(handle.downgrade().upgrade().as_ref(), Some(handle));
        }
    }
}

struct CreatedEntity;

struct CompletedBatch {
    entities: Vec<Entity<CreatedEntity>>,
    observed: Rc<RefCell<Vec<EntityId>>>,
    dropped: Vec<WeakFocusHandle>,
}

impl Drop for CompletedBatch {
    fn drop(&mut self) {
        assert_eq!(self.entities.len(), ENTITY_COUNT);
        assert_eq!(
            *self.observed.borrow(),
            self.entities
                .iter()
                .map(Entity::entity_id)
                .collect::<Vec<_>>()
        );
        self.observed.borrow_mut().clear();
        assert_eq!(
            self.dropped
                .iter()
                .filter(|handle| handle.upgrade().is_some())
                .count(),
            0
        );
    }
}

gpui::bench_group! {
    name = benches;
    config = criterion::Criterion::default()
        .sample_size(10)
        .warm_up_time(Duration::from_secs(1))
        .measurement_time(Duration::from_secs(3))
        .without_plots();
    targets = no_drops, final_drops, final_drops_with_windows
}
gpui::bench_main!(benches);
