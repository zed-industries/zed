#![deny(warnings)]
use warp::Filter;

#[derive(Clone, Debug, PartialEq)]
struct Ext1(i32);

#[tokio::test]
async fn set_and_get() {
    let ext = warp::ext::get::<Ext1>();

    let extracted = warp::test::request()
        .extension(Ext1(55))
        .filter(&ext)
        .await
        .unwrap();

    assert_eq!(extracted, Ext1(55));
}

#[tokio::test]
async fn get_missing() {
    let ext = warp::ext::get().map(|e: Ext1| e.0.to_string());

    let res = warp::test::request().reply(&ext).await;

    assert_eq!(res.status(), 500);
    assert_eq!(res.body(), "Missing request extension");
}
