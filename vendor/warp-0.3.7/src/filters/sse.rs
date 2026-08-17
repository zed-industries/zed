//! Server-Sent Events (SSE)
//!
//! # Example
//!
//! ```
//!
//! use std::time::Duration;
//! use std::convert::Infallible;
//! use warp::{Filter, sse::Event};
//! use futures_util::{stream::iter, Stream};
//!
//! fn sse_events() -> impl Stream<Item = Result<Event, Infallible>> {
//!     iter(vec![
//!         Ok(Event::default().data("unnamed event")),
//!         Ok(
//!             Event::default().event("chat")
//!             .data("chat message")
//!         ),
//!         Ok(
//!             Event::default().id(13.to_string())
//!             .event("chat")
//!             .data("other chat message\nwith next line")
//!             .retry(Duration::from_millis(5000))
//!         )
//!     ])
//! }
//!
//! let app = warp::path("push-notifications")
//!     .and(warp::get())
//!     .map(|| {
//!         warp::sse::reply(warp::sse::keep_alive().stream(sse_events()))
//!     });
//! ```
//!
//! Each field already is event which can be sent to client.
//! The events with multiple fields can be created by combining fields using tuples.
//!
//! See also the [EventSource](https://developer.mozilla.org/en-US/docs/Web/API/EventSource) API,
//! which specifies the expected behavior of Server Sent Events.
//!

#![allow(rustdoc::invalid_html_tags)]

use serde::Serialize;
use std::borrow::Cow;
use std::error::Error as StdError;
use std::fmt::{self, Write};
use std::future::Future;
use std::pin::Pin;
use std::str::FromStr;
use std::task::{Context, Poll};
use std::time::Duration;

use futures_util::{future, Stream, TryStream, TryStreamExt};
use http::header::{HeaderValue, CACHE_CONTROL, CONTENT_TYPE};
use hyper::Body;
use pin_project::pin_project;
use serde_json::Error;
use tokio::time::{self, Sleep};

use self::sealed::SseError;
use super::header;
use crate::filter::One;
use crate::reply::Response;
use crate::{Filter, Rejection, Reply};

// Server-sent event data type
#[derive(Debug)]
enum DataType {
    Text(String),
    Json(String),
}

/// Server-sent event
#[derive(Default, Debug)]
pub struct Event {
    id: Option<String>,
    data: Option<DataType>,
    event: Option<String>,
    comment: Option<String>,
    retry: Option<Duration>,
}

impl Event {
    /// Set Server-sent event data
    /// data field(s) ("data:<content>")
    pub fn data<T: Into<String>>(mut self, data: T) -> Event {
        self.data = Some(DataType::Text(data.into()));
        self
    }

    /// Set Server-sent event data
    /// data field(s) ("data:<content>")
    pub fn json_data<T: Serialize>(mut self, data: T) -> Result<Event, Error> {
        self.data = Some(DataType::Json(serde_json::to_string(&data)?));
        Ok(self)
    }

    /// Set Server-sent event comment
    /// Comment field (":<comment-text>")
    pub fn comment<T: Into<String>>(mut self, comment: T) -> Event {
        self.comment = Some(comment.into());
        self
    }

    /// Set Server-sent event event
    /// Event name field ("event:<event-name>")
    pub fn event<T: Into<String>>(mut self, event: T) -> Event {
        self.event = Some(event.into());
        self
    }

    /// Set Server-sent event retry
    /// Retry timeout field ("retry:<timeout>")
    pub fn retry(mut self, duration: Duration) -> Event {
        self.retry = Some(duration);
        self
    }

    /// Set Server-sent event id
    /// Identifier field ("id:<identifier>")
    pub fn id<T: Into<String>>(mut self, id: T) -> Event {
        self.id = Some(id.into());
        self
    }
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(ref comment) = &self.comment {
            ":".fmt(f)?;
            comment.fmt(f)?;
            f.write_char('\n')?;
        }

        if let Some(ref event) = &self.event {
            "event:".fmt(f)?;
            event.fmt(f)?;
            f.write_char('\n')?;
        }

        match self.data {
            Some(DataType::Text(ref data)) => {
                for line in data.split('\n') {
                    "data:".fmt(f)?;
                    line.fmt(f)?;
                    f.write_char('\n')?;
                }
            }
            Some(DataType::Json(ref data)) => {
                "data:".fmt(f)?;
                data.fmt(f)?;
                f.write_char('\n')?;
            }
            None => {}
        }

        if let Some(ref id) = &self.id {
            "id:".fmt(f)?;
            id.fmt(f)?;
            f.write_char('\n')?;
        }

        if let Some(ref duration) = &self.retry {
            "retry:".fmt(f)?;

            let secs = duration.as_secs();
            let millis = duration.subsec_millis();

            if secs > 0 {
                // format seconds
                secs.fmt(f)?;

                // pad milliseconds
                if millis < 10 {
                    f.write_str("00")?;
                } else if millis < 100 {
                    f.write_char('0')?;
                }
            }

            // format milliseconds
            millis.fmt(f)?;

            f.write_char('\n')?;
        }

        f.write_char('\n')?;
        Ok(())
    }
}

/// Gets the optional last event id from request.
/// Typically this identifier represented as number or string.
///
/// ```
/// let app = warp::sse::last_event_id::<u32>();
///
/// // The identifier is present
/// async {
///     assert_eq!(
///         warp::test::request()
///            .header("Last-Event-ID", "12")
///            .filter(&app)
///            .await
///            .unwrap(),
///         Some(12)
///     );
///
///     // The identifier is missing
///     assert_eq!(
///        warp::test::request()
///            .filter(&app)
///            .await
///            .unwrap(),
///         None
///     );
///
///     // The identifier is not a valid
///     assert!(
///        warp::test::request()
///            .header("Last-Event-ID", "abc")
///            .filter(&app)
///            .await
///            .is_err(),
///     );
///};
/// ```
pub fn last_event_id<T>() -> impl Filter<Extract = One<Option<T>>, Error = Rejection> + Copy
where
    T: FromStr + Send + Sync + 'static,
{
    header::optional("last-event-id")
}

/// Server-sent events reply
///
/// This function converts stream of server events into a `Reply` with:
///
/// - Status of `200 OK`
/// - Header `content-type: text/event-stream`
/// - Header `cache-control: no-cache`.
///
/// # Example
///
/// ```
///
/// use std::time::Duration;
/// use futures_util::Stream;
/// use futures_util::stream::iter;
/// use std::convert::Infallible;
/// use warp::{Filter, sse::Event};
/// use serde_derive::Serialize;
///
/// #[derive(Serialize)]
/// struct Msg {
///     from: u32,
///     text: String,
/// }
///
/// fn event_stream() -> impl Stream<Item = Result<Event, Infallible>> {
///         iter(vec![
///             // Unnamed event with data only
///             Ok(Event::default().data("payload")),
///             // Named event with ID and retry timeout
///             Ok(
///                 Event::default().data("other message\nwith next line")
///                 .event("chat")
///                 .id(1.to_string())
///                 .retry(Duration::from_millis(15000))
///             ),
///             // Event with JSON data
///             Ok(
///                 Event::default().id(2.to_string())
///                 .json_data(Msg {
///                     from: 2,
///                     text: "hello".into(),
///                 }).unwrap(),
///             )
///         ])
/// }
///
/// async {
///     let app = warp::path("sse").and(warp::get()).map(|| {
///        warp::sse::reply(event_stream())
///     });
///
///     let res = warp::test::request()
///         .method("GET")
///         .header("Connection", "Keep-Alive")
///         .path("/sse")
///         .reply(&app)
///         .await
///         .into_body();
///
///     assert_eq!(
///         res,
///         r#"data:payload
///
/// event:chat
/// data:other message
/// data:with next line
/// id:1
/// retry:15000
///
/// data:{"from":2,"text":"hello"}
/// id:2
///
/// "#
///     );
/// };
/// ```
pub fn reply<S>(event_stream: S) -> impl Reply
where
    S: TryStream<Ok = Event> + Send + 'static,
    S::Error: StdError + Send + Sync + 'static,
{
    SseReply { event_stream }
}

#[allow(missing_debug_implementations)]
struct SseReply<S> {
    event_stream: S,
}

impl<S> Reply for SseReply<S>
where
    S: TryStream<Ok = Event> + Send + 'static,
    S::Error: StdError + Send + Sync + 'static,
{
    #[inline]
    fn into_response(self) -> Response {
        let body_stream = self
            .event_stream
            .map_err(|error| {
                // FIXME: error logging
                log::error!("sse stream error: {}", error);
                SseError
            })
            .into_stream()
            .and_then(|event| future::ready(Ok(event.to_string())));

        let mut res = Response::new(Body::wrap_stream(body_stream));
        // Set appropriate content type
        res.headers_mut()
            .insert(CONTENT_TYPE, HeaderValue::from_static("text/event-stream"));
        // Disable response body caching
        res.headers_mut()
            .insert(CACHE_CONTROL, HeaderValue::from_static("no-cache"));
        res
    }
}

/// Configure the interval between keep-alive messages, the content
/// of each message, and the associated stream.
#[derive(Debug)]
pub struct KeepAlive {
    comment_text: Cow<'static, str>,
    max_interval: Duration,
}

impl KeepAlive {
    /// Customize the interval between keep-alive messages.
    ///
    /// Default is 15 seconds.
    pub fn interval(mut self, time: Duration) -> Self {
        self.max_interval = time;
        self
    }

    /// Customize the text of the keep-alive message.
    ///
    /// Default is an empty comment.
    pub fn text(mut self, text: impl Into<Cow<'static, str>>) -> Self {
        self.comment_text = text.into();
        self
    }

    /// Wrap an event stream with keep-alive functionality.
    ///
    /// See [`keep_alive`] for more.
    pub fn stream<S>(
        self,
        event_stream: S,
    ) -> impl TryStream<Ok = Event, Error = impl StdError + Send + Sync + 'static> + Send + 'static
    where
        S: TryStream<Ok = Event> + Send + 'static,
        S::Error: StdError + Send + Sync + 'static,
    {
        let alive_timer = time::sleep(self.max_interval);
        SseKeepAlive {
            event_stream,
            comment_text: self.comment_text,
            max_interval: self.max_interval,
            alive_timer,
        }
    }
}

#[allow(missing_debug_implementations)]
#[pin_project]
struct SseKeepAlive<S> {
    #[pin]
    event_stream: S,
    comment_text: Cow<'static, str>,
    max_interval: Duration,
    #[pin]
    alive_timer: Sleep,
}

/// Keeps event source connection alive when no events sent over a some time.
///
/// Some proxy servers may drop HTTP connection after a some timeout of inactivity.
/// This function helps to prevent such behavior by sending comment events every
/// `keep_interval` of inactivity.
///
/// By default the comment is `:` (an empty comment) and the time interval between
/// events is 15 seconds. Both may be customized using the builder pattern
/// as shown below.
///
/// ```
/// use std::time::Duration;
/// use std::convert::Infallible;
/// use futures_util::StreamExt;
/// use tokio::time::interval;
/// use tokio_stream::wrappers::IntervalStream;
/// use warp::{Filter, Stream, sse::Event};
///
/// // create server-sent event
/// fn sse_counter(counter: u64) ->  Result<Event, Infallible> {
///     Ok(Event::default().data(counter.to_string()))
/// }
///
/// fn main() {
///     let routes = warp::path("ticks")
///         .and(warp::get())
///         .map(|| {
///             let mut counter: u64 = 0;
///             let interval = interval(Duration::from_secs(15));
///             let stream = IntervalStream::new(interval);
///             let event_stream = stream.map(move |_| {
///                 counter += 1;
///                 sse_counter(counter)
///             });
///             // reply using server-sent events
///             let stream = warp::sse::keep_alive()
///                 .interval(Duration::from_secs(5))
///                 .text("thump".to_string())
///                 .stream(event_stream);
///             warp::sse::reply(stream)
///         });
/// }
/// ```
///
/// See [notes](https://www.w3.org/TR/2009/WD-eventsource-20090421/#notes).
pub fn keep_alive() -> KeepAlive {
    KeepAlive {
        comment_text: Cow::Borrowed(""),
        max_interval: Duration::from_secs(15),
    }
}

impl<S> Stream for SseKeepAlive<S>
where
    S: TryStream<Ok = Event> + Send + 'static,
    S::Error: StdError + Send + Sync + 'static,
{
    type Item = Result<Event, SseError>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let mut pin = self.project();
        match pin.event_stream.try_poll_next(cx) {
            Poll::Pending => match Pin::new(&mut pin.alive_timer).poll(cx) {
                Poll::Pending => Poll::Pending,
                Poll::Ready(_) => {
                    // restart timer
                    pin.alive_timer
                        .reset(tokio::time::Instant::now() + *pin.max_interval);
                    let comment_str = pin.comment_text.clone();
                    let event = Event::default().comment(comment_str);
                    Poll::Ready(Some(Ok(event)))
                }
            },
            Poll::Ready(Some(Ok(event))) => {
                // restart timer
                pin.alive_timer
                    .reset(tokio::time::Instant::now() + *pin.max_interval);
                Poll::Ready(Some(Ok(event)))
            }
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Ready(Some(Err(error))) => {
                log::error!("sse::keep error: {}", error);
                Poll::Ready(Some(Err(SseError)))
            }
        }
    }
}

mod sealed {
    use super::*;

    /// SSE error type
    #[derive(Debug)]
    pub struct SseError;

    impl fmt::Display for SseError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "sse error")
        }
    }

    impl StdError for SseError {}
}
