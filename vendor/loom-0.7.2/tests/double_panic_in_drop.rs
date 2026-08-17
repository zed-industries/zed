#[test]
#[should_panic]
fn double_panic_at_branch_max() {
    let mut builder = loom::model::Builder::new();
    builder.max_branches = 2;

    builder.check(|| {
        let _arc = loom::sync::Arc::new(());
        loom::thread::yield_now();
        loom::thread::yield_now();
        loom::thread::yield_now();
    });
}
