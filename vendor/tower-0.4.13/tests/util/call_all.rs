use super::support;
use futures_core::Stream;
use futures_util::{
    future::{ready, Ready},
    pin_mut,
};
use std::task::{Context, Poll};
use std::{cell::Cell, rc::Rc};
use tokio_test::{assert_pending, assert_ready, task};
use tower::util::ServiceExt;
use tower_service::*;
use tower_test::{assert_request_eq, mock};

type Error = Box<dyn std::error::Error + Send + Sync>;

#[derive(Debug, Eq, PartialEq)]
struct Srv {
    admit: Rc<Cell<bool>>,
    count: Rc<Cell<usize>>,
}
impl Service<&'static str> for Srv {
    type Response = &'static str;
    type Error = Error;
    type Future = Ready<Result<Self::Response, Error>>;

    fn poll_ready(&mut self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        if !self.admit.get() {
            return Poll::Pending;
        }

        self.admit.set(false);
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: &'static str) -> Self::Future {
        self.count.set(self.count.get() + 1);
        ready(Ok(req))
    }
}

#[test]
fn ordered() {
    let _t = support::trace_init();

    let mut mock = task::spawn(());

    let admit = Rc::new(Cell::new(false));
    let count = Rc::new(Cell::new(0));
    let srv = Srv {
        count: count.clone(),
        admit: admit.clone(),
    };
    let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
    let ca = srv.call_all(support::IntoStream::new(rx));
    pin_mut!(ca);

    assert_pending!(mock.enter(|cx, _| ca.as_mut().poll_next(cx)));
    tx.send("one").unwrap();
    mock.is_woken();
    assert_pending!(mock.enter(|cx, _| ca.as_mut().poll_next(cx)));
    admit.set(true);
    let v = assert_ready!(mock.enter(|cx, _| ca.as_mut().poll_next(cx)))
        .transpose()
        .unwrap();
    assert_eq!(v, Some("one"));
    assert_pending!(mock.enter(|cx, _| ca.as_mut().poll_next(cx)));
    admit.set(true);
    tx.send("two").unwrap();
    mock.is_woken();
    tx.send("three").unwrap();
    let v = assert_ready!(mock.enter(|cx, _| ca.as_mut().poll_next(cx)))
        .transpose()
        .unwrap();
    assert_eq!(v, Some("two"));
    assert_pending!(mock.enter(|cx, _| ca.as_mut().poll_next(cx)));
    admit.set(true);
    let v = assert_ready!(mock.enter(|cx, _| ca.as_mut().poll_next(cx)))
        .transpose()
        .unwrap();
    assert_eq!(v, Some("three"));
    admit.set(true);
    assert_pending!(mock.enter(|cx, _| ca.as_mut().poll_next(cx)));
    admit.set(true);
    tx.send("four").unwrap();
    mock.is_woken();
    let v = assert_ready!(mock.enter(|cx, _| ca.as_mut().poll_next(cx)))
        .transpose()
        .unwrap();
    assert_eq!(v, Some("four"));
    assert_pending!(mock.enter(|cx, _| ca.as_mut().poll_next(cx)));

    // need to be ready since impl doesn't know it'll get EOF
    admit.set(true);

    // When we drop the request stream, CallAll should return None.
    drop(tx);
    mock.is_woken();
    let v = assert_ready!(mock.enter(|cx, _| ca.as_mut().poll_next(cx)))
        .transpose()
        .unwrap();
    assert!(v.is_none());
    assert_eq!(count.get(), 4);

    // We should also be able to recover the wrapped Service.
    assert_eq!(ca.take_service(), Srv { count, admit });
}

#[tokio::test(flavor = "current_thread")]
async fn unordered() {
    let _t = support::trace_init();

    let (mock, handle) = mock::pair::<_, &'static str>();
    pin_mut!(handle);

    let mut task = task::spawn(());
    let requests = futures_util::stream::iter(&["one", "two"]);

    let svc = mock.call_all(requests).unordered();
    pin_mut!(svc);

    assert_pending!(task.enter(|cx, _| svc.as_mut().poll_next(cx)));

    let resp1 = assert_request_eq!(handle, &"one");
    let resp2 = assert_request_eq!(handle, &"two");

    resp2.send_response("resp 1");

    let v = assert_ready!(task.enter(|cx, _| svc.as_mut().poll_next(cx)))
        .transpose()
        .unwrap();
    assert_eq!(v, Some("resp 1"));
    assert_pending!(task.enter(|cx, _| svc.as_mut().poll_next(cx)));

    resp1.send_response("resp 2");

    let v = assert_ready!(task.enter(|cx, _| svc.as_mut().poll_next(cx)))
        .transpose()
        .unwrap();
    assert_eq!(v, Some("resp 2"));

    let v = assert_ready!(task.enter(|cx, _| svc.as_mut().poll_next(cx)))
        .transpose()
        .unwrap();
    assert!(v.is_none());
}

#[tokio::test]
async fn pending() {
    let _t = support::trace_init();

    let (mock, mut handle) = mock::pair::<_, &'static str>();

    let mut task = task::spawn(());

    let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
    let ca = mock.call_all(support::IntoStream::new(rx));
    pin_mut!(ca);

    assert_pending!(task.enter(|cx, _| ca.as_mut().poll_next(cx)));
    tx.send("req").unwrap();
    assert_pending!(task.enter(|cx, _| ca.as_mut().poll_next(cx)));
    assert_request_eq!(handle, "req").send_response("res");
    let res = assert_ready!(task.enter(|cx, _| ca.as_mut().poll_next(cx)));
    assert_eq!(res.transpose().unwrap(), Some("res"));
    assert_pending!(task.enter(|cx, _| ca.as_mut().poll_next(cx)));
}
