use futures_executor::block_on;

macro_rules! recurse {
    ($name:ident, $param:ty) => {
        #[::async_recursion::async_recursion]
        async fn $name<F>(param: $param, f: &F)
        where
            F: Fn($param) + Sync + Send,
        {
            f(param);
        }
    };
}

recurse!(owned, usize);
recurse!(by_ref, &usize);
recurse!(by_ref_mut, &mut usize);

#[test]
fn async_in_macro() {
    block_on(async move {
        owned(5, &|_| ()).await;
        by_ref(&5, &|_| ()).await;
        by_ref_mut(&mut 5, &|_| ()).await;
    });
}
