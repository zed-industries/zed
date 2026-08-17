use std::{pin::Pin, task::Poll, time::Duration};

use futures_util::Stream;

use crate::{DbErr, QueryResult, Statement};

pub(crate) struct MetricStream<'a> {
    metric_callback: &'a Option<crate::metric::Callback>,
    stmt: &'a Statement,
    elapsed: Option<Duration>,
    stream: Pin<Box<dyn Stream<Item = Result<QueryResult, DbErr>> + 'a + Send>>,
}

impl<'a> MetricStream<'a> {
    #[allow(dead_code)]
    pub(crate) fn new<S>(
        metric_callback: &'a Option<crate::metric::Callback>,
        stmt: &'a Statement,
        elapsed: Option<Duration>,
        stream: S,
    ) -> Self
    where
        S: Stream<Item = Result<QueryResult, DbErr>> + 'a + Send,
    {
        MetricStream {
            metric_callback,
            stmt,
            elapsed,
            stream: Box::pin(stream),
        }
    }
}

impl Stream for MetricStream<'_> {
    type Item = Result<QueryResult, DbErr>;

    fn poll_next(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        let this = self.get_mut();
        let _start = this
            .metric_callback
            .is_some()
            .then(std::time::SystemTime::now);
        let res = Pin::new(&mut this.stream).poll_next(cx);
        if let (Some(_start), Some(elapsed)) = (_start, &mut this.elapsed) {
            *elapsed += _start.elapsed().unwrap_or_default();
        }
        res
    }
}

impl Drop for MetricStream<'_> {
    fn drop(&mut self) {
        if let (Some(callback), Some(elapsed)) = (self.metric_callback.as_deref(), self.elapsed) {
            let info = crate::metric::Info {
                elapsed,
                statement: self.stmt,
                failed: false,
            };
            callback(&info);
        }
    }
}
