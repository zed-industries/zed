#[cfg(all(test, feature = "unit-eval"))]
use futures::future::LocalBoxFuture;
#[cfg(all(test, feature = "unit-eval"))]
use gpui::TestAppContext;
#[cfg(all(test, feature = "unit-eval"))]
use std::fmt::Display;

#[cfg(all(test, feature = "unit-eval"))]
mod edit_file;
#[cfg(all(test, feature = "unit-eval"))]
mod terminal_tool;
#[cfg(all(test, feature = "unit-eval"))]
mod write_file;

#[cfg(all(test, feature = "unit-eval"))]
fn run_gpui_eval<T>(
    eval: impl for<'a> FnOnce(&'a mut TestAppContext) -> LocalBoxFuture<'a, anyhow::Result<T>>,
    outcome: impl FnOnce(&T) -> eval_utils::OutcomeKind,
) -> eval_utils::EvalOutput<()>
where
    T: Display,
{
    let dispatcher = gpui::TestDispatcher::new(rand::random());
    let mut cx = TestAppContext::build(dispatcher.clone(), None);
    let entity_refcounts = cx.app.borrow().ref_counts_drop_handle();
    let foreground_executor = cx.foreground_executor().clone();
    let result = foreground_executor.block_test(eval(&mut cx));

    cx.run_until_parked();
    cx.update(|cx| {
        cx.background_executor().forbid_parking();
        cx.quit();
    });
    cx.run_until_parked();
    drop(cx);
    dispatcher.drain_tasks();
    drop(dispatcher);
    drop(entity_refcounts);

    match result {
        Ok(output) => eval_utils::EvalOutput {
            data: output.to_string(),
            outcome: outcome(&output),
            metadata: (),
        },
        Err(err) => eval_utils::EvalOutput {
            data: format!("{err:?}"),
            outcome: eval_utils::OutcomeKind::Error,
            metadata: (),
        },
    }
}
