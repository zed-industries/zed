use crate::load;
use futures_util::pin_mut;
use tokio_test::{assert_pending, assert_ready, assert_ready_ok, task};
use tower_test::{assert_request_eq, mock};

use super::*;

#[tokio::test]
async fn basic() {
    // start the pool
    let (mock, handle) = mock::pair::<(), load::Constant<mock::Mock<(), &'static str>, usize>>();
    pin_mut!(handle);

    let mut pool = mock::Spawn::new(Builder::new().build(mock, ()));
    assert_pending!(pool.poll_ready());

    // give the pool a backing service
    let (svc1_m, svc1) = mock::pair();
    pin_mut!(svc1);

    assert_request_eq!(handle, ()).send_response(load::Constant::new(svc1_m, 0));
    assert_ready_ok!(pool.poll_ready());

    // send a request to the one backing service
    let mut fut = task::spawn(pool.call(()));

    assert_pending!(fut.poll());
    assert_request_eq!(svc1, ()).send_response("foobar");
    assert_eq!(assert_ready_ok!(fut.poll()), "foobar");
}

#[tokio::test]
async fn high_load() {
    // start the pool
    let (mock, handle) = mock::pair::<(), load::Constant<mock::Mock<(), &'static str>, usize>>();
    pin_mut!(handle);

    let pool = Builder::new()
        .urgency(1.0) // so _any_ Pending will add a service
        .underutilized_below(0.0) // so no Ready will remove a service
        .max_services(Some(2))
        .build(mock, ());
    let mut pool = mock::Spawn::new(pool);
    assert_pending!(pool.poll_ready());

    // give the pool a backing service
    let (svc1_m, svc1) = mock::pair();
    pin_mut!(svc1);

    svc1.allow(1);
    assert_request_eq!(handle, ()).send_response(load::Constant::new(svc1_m, 0));
    assert_ready_ok!(pool.poll_ready());

    // make the one backing service not ready
    let mut fut1 = task::spawn(pool.call(()));

    // if we poll_ready again, pool should notice that load is increasing
    // since urgency == 1.0, it should immediately enter high load
    assert_pending!(pool.poll_ready());
    // it should ask the maker for another service, so we give it one
    let (svc2_m, svc2) = mock::pair();
    pin_mut!(svc2);

    svc2.allow(1);
    assert_request_eq!(handle, ()).send_response(load::Constant::new(svc2_m, 0));

    // the pool should now be ready again for one more request
    assert_ready_ok!(pool.poll_ready());
    let mut fut2 = task::spawn(pool.call(()));

    assert_pending!(pool.poll_ready());

    // the pool should _not_ try to add another service
    // sicen we have max_services(2)
    assert_pending!(handle.as_mut().poll_request());

    // let see that each service got one request
    assert_request_eq!(svc1, ()).send_response("foo");
    assert_request_eq!(svc2, ()).send_response("bar");
    assert_eq!(assert_ready_ok!(fut1.poll()), "foo");
    assert_eq!(assert_ready_ok!(fut2.poll()), "bar");
}

#[tokio::test]
async fn low_load() {
    // start the pool
    let (mock, handle) = mock::pair::<(), load::Constant<mock::Mock<(), &'static str>, usize>>();
    pin_mut!(handle);

    let pool = Builder::new()
        .urgency(1.0) // so any event will change the service count
        .build(mock, ());

    let mut pool = mock::Spawn::new(pool);

    assert_pending!(pool.poll_ready());

    // give the pool a backing service
    let (svc1_m, svc1) = mock::pair();
    pin_mut!(svc1);

    svc1.allow(1);
    assert_request_eq!(handle, ()).send_response(load::Constant::new(svc1_m, 0));
    assert_ready_ok!(pool.poll_ready());

    // cycling a request should now work
    let mut fut = task::spawn(pool.call(()));

    assert_request_eq!(svc1, ()).send_response("foo");
    assert_eq!(assert_ready_ok!(fut.poll()), "foo");
    // and pool should now not be ready (since svc1 isn't ready)
    // it should immediately try to add another service
    // which we give it
    assert_pending!(pool.poll_ready());
    let (svc2_m, svc2) = mock::pair();
    pin_mut!(svc2);

    svc2.allow(1);
    assert_request_eq!(handle, ()).send_response(load::Constant::new(svc2_m, 0));
    // pool is now ready
    // which (because of urgency == 1.0) should immediately cause it to drop a service
    // it'll drop svc1, so it'll still be ready
    assert_ready_ok!(pool.poll_ready());
    // and even with another ready, it won't drop svc2 since its now the only service
    assert_ready_ok!(pool.poll_ready());

    // cycling a request should now work on svc2
    let mut fut = task::spawn(pool.call(()));

    assert_request_eq!(svc2, ()).send_response("foo");
    assert_eq!(assert_ready_ok!(fut.poll()), "foo");

    // and again (still svc2)
    svc2.allow(1);
    assert_ready_ok!(pool.poll_ready());
    let mut fut = task::spawn(pool.call(()));

    assert_request_eq!(svc2, ()).send_response("foo");
    assert_eq!(assert_ready_ok!(fut.poll()), "foo");
}

#[tokio::test]
async fn failing_service() {
    // start the pool
    let (mock, handle) = mock::pair::<(), load::Constant<mock::Mock<(), &'static str>, usize>>();
    pin_mut!(handle);

    let pool = Builder::new()
        .urgency(1.0) // so _any_ Pending will add a service
        .underutilized_below(0.0) // so no Ready will remove a service
        .build(mock, ());

    let mut pool = mock::Spawn::new(pool);

    assert_pending!(pool.poll_ready());

    // give the pool a backing service
    let (svc1_m, svc1) = mock::pair();
    pin_mut!(svc1);

    svc1.allow(1);
    assert_request_eq!(handle, ()).send_response(load::Constant::new(svc1_m, 0));
    assert_ready_ok!(pool.poll_ready());

    // one request-response cycle
    let mut fut = task::spawn(pool.call(()));

    assert_request_eq!(svc1, ()).send_response("foo");
    assert_eq!(assert_ready_ok!(fut.poll()), "foo");

    // now make svc1 fail, so it has to be removed
    svc1.send_error("ouch");
    // polling now should recognize the failed service,
    // try to create a new one, and then realize the maker isn't ready
    assert_pending!(pool.poll_ready());
    // then we release another service
    let (svc2_m, svc2) = mock::pair();
    pin_mut!(svc2);

    svc2.allow(1);
    assert_request_eq!(handle, ()).send_response(load::Constant::new(svc2_m, 0));

    // the pool should now be ready again
    assert_ready_ok!(pool.poll_ready());
    // and a cycle should work (and go through svc2)
    let mut fut = task::spawn(pool.call(()));

    assert_request_eq!(svc2, ()).send_response("bar");
    assert_eq!(assert_ready_ok!(fut.poll()), "bar");
}
