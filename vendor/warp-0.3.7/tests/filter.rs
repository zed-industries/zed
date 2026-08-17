#![deny(warnings)]
use std::convert::Infallible;
use warp::Filter;

#[tokio::test]
async fn flattens_tuples() {
    let _ = pretty_env_logger::try_init();

    let str1 = warp::any().map(|| "warp");
    let true1 = warp::any().map(|| true);
    let unit1 = warp::any();

    // just 1 value
    let ext = warp::test::request().filter(&str1).await.unwrap();
    assert_eq!(ext, "warp");

    // just 1 unit
    let ext = warp::test::request().filter(&unit1).await.unwrap();
    assert_eq!(ext, ());

    // combine 2 values
    let and = str1.and(true1);
    let ext = warp::test::request().filter(&and).await.unwrap();
    assert_eq!(ext, ("warp", true));

    // combine 2 reversed
    let and = true1.and(str1);
    let ext = warp::test::request().filter(&and).await.unwrap();
    assert_eq!(ext, (true, "warp"));

    // combine 1 with unit
    let and = str1.and(unit1);
    let ext = warp::test::request().filter(&and).await.unwrap();
    assert_eq!(ext, "warp");

    let and = unit1.and(str1);
    let ext = warp::test::request().filter(&and).await.unwrap();
    assert_eq!(ext, "warp");

    // combine 3 values
    let and = str1.and(str1).and(true1);
    let ext = warp::test::request().filter(&and).await.unwrap();
    assert_eq!(ext, ("warp", "warp", true));

    // combine 2 with unit
    let and = str1.and(unit1).and(true1);
    let ext = warp::test::request().filter(&and).await.unwrap();
    assert_eq!(ext, ("warp", true));

    let and = unit1.and(str1).and(true1);
    let ext = warp::test::request().filter(&and).await.unwrap();
    assert_eq!(ext, ("warp", true));

    let and = str1.and(true1).and(unit1);
    let ext = warp::test::request().filter(&and).await.unwrap();
    assert_eq!(ext, ("warp", true));

    // nested tuples
    let str_true_unit = str1.and(true1).and(unit1);
    let unit_str_true = unit1.and(str1).and(true1);

    let and = str_true_unit.and(unit_str_true);
    let ext = warp::test::request().filter(&and).await.unwrap();
    assert_eq!(ext, ("warp", true, "warp", true));

    let and = unit_str_true.and(unit1).and(str1).and(str_true_unit);
    let ext = warp::test::request().filter(&and).await.unwrap();
    assert_eq!(ext, ("warp", true, "warp", "warp", true));
}

#[tokio::test]
async fn map() {
    let _ = pretty_env_logger::try_init();

    let ok = warp::any().map(warp::reply);

    let req = warp::test::request();
    let resp = req.reply(&ok).await;
    assert_eq!(resp.status(), 200);
}

#[tokio::test]
async fn or() {
    let _ = pretty_env_logger::try_init();

    // Or can be combined with an infallible filter
    let a = warp::path::param::<u32>();
    let b = warp::any().map(|| 41i32);
    let f = a.or(b);

    let _: Result<_, Infallible> = warp::test::request().filter(&f).await;
}

#[tokio::test]
async fn or_else() {
    let _ = pretty_env_logger::try_init();

    let a = warp::path::param::<u32>();
    let f = a.or_else(|_| async { Ok::<_, warp::Rejection>((44u32,)) });

    assert_eq!(
        warp::test::request().path("/33").filter(&f).await.unwrap(),
        33,
    );
    assert_eq!(warp::test::request().filter(&f).await.unwrap(), 44,);

    // OrElse can be combined with an infallible filter
    let a = warp::path::param::<u32>();
    let f = a.or_else(|_| async { Ok::<_, Infallible>((44u32,)) });

    let _: Result<_, Infallible> = warp::test::request().filter(&f).await;
}

#[tokio::test]
async fn recover() {
    let _ = pretty_env_logger::try_init();

    let a = warp::path::param::<String>();
    let f = a.recover(|err| async move { Err::<String, _>(err) });

    // not rejected
    let resp = warp::test::request().path("/hi").reply(&f).await;
    assert_eq!(resp.status(), 200);
    assert_eq!(resp.body(), "hi");

    // rejected, recovered, re-rejected
    let resp = warp::test::request().reply(&f).await;
    assert_eq!(resp.status(), 404);

    // Recover can be infallible
    let f = a.recover(|_| async move { Ok::<_, Infallible>("shh") });

    let _: Result<_, Infallible> = warp::test::request().filter(&f).await;
}

#[tokio::test]
async fn unify() {
    let _ = pretty_env_logger::try_init();

    let a = warp::path::param::<u32>();
    let b = warp::path::param::<u32>();
    let f = a.or(b).unify();

    let ex = warp::test::request().path("/1").filter(&f).await.unwrap();

    assert_eq!(ex, 1);
}

#[should_panic]
#[tokio::test]
async fn nested() {
    let f = warp::any().and_then(|| async {
        let p = warp::path::param::<u32>();
        warp::test::request().filter(&p).await
    });

    let _ = warp::test::request().filter(&f).await;
}
