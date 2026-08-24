#[cfg(all(test, feature = "unit-eval"))]
use futures::future::LocalBoxFuture;
#[cfg(all(test, feature = "unit-eval"))]
use gpui::TestAppContext;
#[cfg(all(test, feature = "unit-eval"))]
use std::{fmt::Display, time::Duration};

#[cfg(all(test, feature = "unit-eval"))]
mod edit_file;
#[cfg(all(test, feature = "unit-eval"))]
mod terminal_tool;
#[cfg(all(test, feature = "unit-eval"))]
mod write_file;

#[cfg(all(test, feature = "unit-eval"))]
#[test]
fn completion_retry_delay_uses_fixed_transient_provider_fallback() {
    for status in [
        http_client::StatusCode::TOO_MANY_REQUESTS,
        http_client::StatusCode::SERVICE_UNAVAILABLE,
        http_client::StatusCode::from_u16(529).expect("529 should be a valid status"),
    ] {
        let error = anyhow::Error::new(
            language_model::LanguageModelCompletionError::from_http_status(
                language_model::LanguageModelProviderName::new("test"),
                status,
                "transient provider failure".to_string(),
                None,
            ),
        );

        assert_eq!(
            completion_retry_delay(&error, 1),
            Some(Duration::from_secs(5))
        );
        assert_eq!(
            completion_retry_delay(&error, 10),
            Some(Duration::from_secs(5))
        );
    }
}

#[cfg(all(test, feature = "unit-eval"))]
fn completion_retry_delay(error: &anyhow::Error, attempt: usize) -> Option<Duration> {
    error
        .downcast_ref::<language_model::LanguageModelCompletionError>()?
        .retry_delay(
            attempt,
            Duration::from_secs(1),
            Duration::from_secs(30),
            Some(Duration::from_secs(5)),
        )
}

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
