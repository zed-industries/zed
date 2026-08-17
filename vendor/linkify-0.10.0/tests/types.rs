use linkify::Link;
use linkify::LinkFinder;
use linkify::LinkKind;

#[test]
fn send_and_sync() {
    check_send::<LinkFinder>();
    check_sync::<LinkFinder>();

    check_send::<Link<'_>>();
    check_sync::<Link<'_>>();
}

#[test]
fn equality() {
    let finder = LinkFinder::new();
    let first = finder.links("http://example.org").next();
    assert!(first.is_some());

    let link = first.unwrap();
    // Check that link has Debug
    println!("{:?}", link);

    assert_eq!(link.kind(), &LinkKind::Url);
}

fn check_send<T: Send>() {}

fn check_sync<T: Sync>() {}
