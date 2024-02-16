pub trait ClientDispatcher: Send + Sync {
    fn dispatch_on_main_thread(&self);
}
