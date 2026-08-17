macro_rules! droppable {
    () => {
        static COUNT: core::sync::atomic::AtomicI32 = core::sync::atomic::AtomicI32::new(0);

        #[derive(Eq, Ord, PartialEq, PartialOrd)]
        struct Droppable(i32);
        impl Droppable {
            fn new() -> Self {
                COUNT.fetch_add(1, core::sync::atomic::Ordering::Relaxed);
                Droppable(Self::count())
            }

            fn count() -> i32 {
                COUNT.load(core::sync::atomic::Ordering::Relaxed)
            }
        }
        impl Drop for Droppable {
            fn drop(&mut self) {
                COUNT.fetch_sub(1, core::sync::atomic::Ordering::Relaxed);
            }
        }
    };
}
