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