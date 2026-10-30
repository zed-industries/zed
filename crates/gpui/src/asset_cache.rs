use crate::{App, EntityId, SharedString, SharedUri, Task};
use collections::FxHashSet;
use futures::{Future, TryFutureExt};

use std::cell::RefCell;
use std::fmt::Debug;
use std::hash::{BuildHasher, Hash};
use std::marker::PhantomData;
use std::mem;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::sync::Arc;

pub(crate) struct CachedLoad<T> {
    state: Rc<RefCell<CachedLoadState<T>>>,
    // Keep the owner outside the state so publishing a result cannot cancel its own task.
    _task: Task<()>,
}

enum CachedLoadState<T> {
    Loading(FxHashSet<EntityId>),
    Loaded(T),
}

impl<T: Clone + Send + 'static> CachedLoad<T> {
    pub(crate) fn new(future: impl Future<Output = T> + Send + 'static, cx: &App) -> Self {
        let state = Rc::new(RefCell::new(CachedLoadState::Loading(FxHashSet::default())));
        let task = cx.background_executor().spawn(future);
        let task = cx.spawn({
            let state = Rc::downgrade(&state);
            async move |cx| {
                let result = task.await;
                let Some(state) = state.upgrade() else {
                    return;
                };
                let previous =
                    mem::replace(&mut *state.borrow_mut(), CachedLoadState::Loaded(result));
                let CachedLoadState::Loading(views) = previous else {
                    unreachable!("a cached load completes only once");
                };
                cx.update(|cx| {
                    for view in views {
                        cx.notify(view);
                    }
                });
            }
        });
        Self { state, _task: task }
    }

    pub(crate) fn get(&self) -> Option<T> {
        match &*self.state.borrow() {
            CachedLoadState::Loading(_) => None,
            CachedLoadState::Loaded(result) => Some(result.clone()),
        }
    }

    pub(crate) fn use_by(&self, view: EntityId) -> Option<T> {
        match &mut *self.state.borrow_mut() {
            CachedLoadState::Loading(views) => {
                views.insert(view);
                None
            }
            CachedLoadState::Loaded(result) => Some(result.clone()),
        }
    }
}

/// An enum representing
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Resource {
    /// This resource is at a given URI
    Uri(SharedUri),
    /// This resource is at a given path in the file system
    Path(Arc<Path>),
    /// This resource is embedded in the application binary
    Embedded(SharedString),
}

impl From<SharedUri> for Resource {
    fn from(value: SharedUri) -> Self {
        Self::Uri(value)
    }
}

impl From<PathBuf> for Resource {
    fn from(value: PathBuf) -> Self {
        Self::Path(value.into())
    }
}

impl From<Arc<Path>> for Resource {
    fn from(value: Arc<Path>) -> Self {
        Self::Path(value)
    }
}

/// A trait for asynchronous asset loading.
pub trait Asset: 'static {
    /// The source of the asset.
    type Source: Clone + Hash + Send;

    /// The loaded asset
    type Output: Clone + Send;

    /// Load the asset asynchronously
    fn load(
        source: Self::Source,
        cx: &mut App,
    ) -> impl Future<Output = Self::Output> + Send + 'static;
}

/// An asset Loader which logs the [`Err`] variant of a [`Result`] during loading
pub enum AssetLogger<T> {
    #[doc(hidden)]
    _Phantom(PhantomData<T>, &'static dyn crate::seal::Sealed),
}

impl<T, R, E> Asset for AssetLogger<T>
where
    T: Asset<Output = Result<R, E>>,
    R: Clone + Send,
    E: Clone + Send + Debug,
{
    type Source = T::Source;

    type Output = T::Output;

    fn load(
        source: Self::Source,
        cx: &mut App,
    ) -> impl Future<Output = Self::Output> + Send + 'static {
        let load = T::load(source, cx);
        load.inspect_err(|e| log::error!("Failed to load asset: {:?}", e))
    }
}

/// Use a quick, non-cryptographically secure hash function to get an identifier from data
pub fn hash<T: Hash>(data: &T) -> u64 {
    collections::FxBuildHasher.hash_one(data)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{AppContext, TestAppContext};
    use futures::channel::oneshot;
    use std::{
        cell::Cell,
        collections::VecDeque,
        hash::{Hash, Hasher},
        rc::Rc,
        sync::{
            Arc, Mutex,
            atomic::{AtomicUsize, Ordering},
        },
    };

    #[gpui::test]
    fn cached_load_caches_success_and_error(cx: &mut TestAppContext) {
        let successful_load =
            cx.update(|cx| CachedLoad::new(async { Ok::<_, &'static str>(42) }, cx));
        let failed_load =
            cx.update(|cx| CachedLoad::new(async { Err::<i32, _>("load failed") }, cx));

        assert_eq!(successful_load.get(), None);
        assert_eq!(failed_load.get(), None);

        cx.run_until_parked();

        assert_eq!(successful_load.get(), Some(Ok(42)));
        assert_eq!(successful_load.get(), Some(Ok(42)));
        assert_eq!(failed_load.get(), Some(Err("load failed")));
        assert_eq!(failed_load.get(), Some(Err("load failed")));
    }

    #[gpui::test]
    fn completed_load_without_subscribers_is_cached_and_deduplicated(cx: &mut TestAppContext) {
        let (sender, receiver) = oneshot::channel();
        let source = TestAssetSource::new(1, [receiver]);

        assert_eq!(cx.update(|cx| cx.fetch_asset::<TestAsset>(&source)), None);
        assert_eq!(cx.update(|cx| cx.fetch_asset::<TestAsset>(&source)), None);
        assert_eq!(source.load_count(), 1);

        assert!(sender.send(Ok(42)).is_ok());
        cx.run_until_parked();

        assert_eq!(
            cx.update(|cx| cx.fetch_asset::<TestAsset>(&source)),
            Some(Ok(42))
        );
        assert_eq!(
            cx.update(|cx| cx.fetch_asset::<TestAsset>(&source)),
            Some(Ok(42))
        );
        assert_eq!(source.load_count(), 1);
    }

    #[gpui::test]
    fn load_completion_notifies_each_observing_entity_once(cx: &mut TestAppContext) {
        let (sender, receiver) = oneshot::channel::<i32>();
        let load = cx.update(|cx| {
            CachedLoad::new(
                async move { receiver.await.expect("test sends a result") },
                cx,
            )
        });
        let first_notification_count = Rc::new(Cell::new(0));
        let second_notification_count = Rc::new(Cell::new(0));
        let (first_entity, second_entity) = cx.update(|cx| {
            let first_entity = cx.new(|_| ());
            let second_entity = cx.new(|_| ());
            cx.observe(&first_entity, {
                let first_notification_count = first_notification_count.clone();
                move |_, _| first_notification_count.set(first_notification_count.get() + 1)
            })
            .detach();
            cx.observe(&second_entity, {
                let second_notification_count = second_notification_count.clone();
                move |_, _| second_notification_count.set(second_notification_count.get() + 1)
            })
            .detach();
            (first_entity, second_entity)
        });

        assert_eq!(load.use_by(first_entity.entity_id()), None);
        assert_eq!(load.use_by(first_entity.entity_id()), None);
        assert_eq!(load.use_by(second_entity.entity_id()), None);

        assert!(sender.send(42).is_ok());
        cx.run_until_parked();

        assert_eq!(first_notification_count.get(), 1);
        assert_eq!(second_notification_count.get(), 1);
        assert_eq!(load.use_by(first_entity.entity_id()), Some(42));
        assert_eq!(load.use_by(second_entity.entity_id()), Some(42));
    }

    #[gpui::test]
    fn removing_pending_asset_cancels_it_and_replacement_ignores_stale_completion(
        cx: &mut TestAppContext,
    ) {
        let (stale_sender, stale_receiver) = oneshot::channel();
        let (replacement_sender, replacement_receiver) = oneshot::channel();
        let source = TestAssetSource::new(2, [stale_receiver, replacement_receiver]);

        assert_eq!(cx.update(|cx| cx.fetch_asset::<TestAsset>(&source)), None);
        assert_eq!(source.load_count(), 1);
        cx.run_until_parked();

        cx.update(|cx| cx.remove_asset::<TestAsset>(&source));
        assert!(!cx.update(|cx| cx.has_asset::<TestAsset>(&source)));
        cx.run_until_parked();
        assert!(stale_sender.send(Ok(1)).is_err());

        assert_eq!(cx.update(|cx| cx.fetch_asset::<TestAsset>(&source)), None);
        assert_eq!(source.load_count(), 2);

        assert!(replacement_sender.send(Err("replacement failed")).is_ok());
        cx.run_until_parked();

        assert_eq!(
            cx.update(|cx| cx.fetch_asset::<TestAsset>(&source)),
            Some(Err("replacement failed"))
        );
        assert_eq!(source.load_count(), 2);
    }

    #[gpui::test]
    fn eviction_after_worker_completion_does_not_publish_into_replacement(cx: &mut TestAppContext) {
        let (sender, receiver) = oneshot::channel();
        let (replacement_sender, replacement_receiver) = oneshot::channel();
        let source = TestAssetSource::new(3, [receiver, replacement_receiver]);

        assert_eq!(cx.update(|cx| cx.fetch_asset::<TestAsset>(&source)), None);
        assert!(sender.send(Ok(1)).is_ok());
        while source.completion_count.load(Ordering::SeqCst) == 0 {
            assert!(cx.background_executor.tick());
        }
        assert_eq!(cx.update(|cx| cx.fetch_asset::<TestAsset>(&source)), None);

        cx.update(|cx| {
            cx.remove_asset::<TestAsset>(&source);
            assert_eq!(cx.fetch_asset::<TestAsset>(&source), None);
        });
        cx.run_until_parked();
        assert_eq!(cx.update(|cx| cx.fetch_asset::<TestAsset>(&source)), None);

        assert!(replacement_sender.send(Ok(2)).is_ok());
        cx.run_until_parked();
        assert_eq!(
            cx.update(|cx| cx.fetch_asset::<TestAsset>(&source)),
            Some(Ok(2))
        );
        assert_eq!(source.load_count(), 2);
    }

    struct TestAsset;

    #[derive(Clone)]
    struct TestAssetSource {
        id: usize,
        load_count: Arc<AtomicUsize>,
        completion_count: Arc<AtomicUsize>,
        receivers: Arc<Mutex<VecDeque<oneshot::Receiver<Result<i32, &'static str>>>>>,
    }

    impl TestAssetSource {
        fn new(
            id: usize,
            receivers: impl IntoIterator<Item = oneshot::Receiver<Result<i32, &'static str>>>,
        ) -> Self {
            Self {
                id,
                load_count: Arc::new(AtomicUsize::new(0)),
                completion_count: Arc::new(AtomicUsize::new(0)),
                receivers: Arc::new(Mutex::new(receivers.into_iter().collect())),
            }
        }

        fn load_count(&self) -> usize {
            self.load_count.load(Ordering::SeqCst)
        }
    }

    impl Hash for TestAssetSource {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.id.hash(state);
        }
    }

    impl Asset for TestAsset {
        type Source = TestAssetSource;
        type Output = Result<i32, &'static str>;

        fn load(
            source: Self::Source,
            _cx: &mut App,
        ) -> impl Future<Output = Self::Output> + Send + 'static {
            source.load_count.fetch_add(1, Ordering::SeqCst);
            let receiver = source
                .receivers
                .lock()
                .expect("test receiver mutex should not be poisoned")
                .pop_front()
                .expect("each test load should have a receiver");
            async move {
                let result = match receiver.await {
                    Ok(result) => result,
                    Err(_) => Err("cancelled"),
                };
                source.completion_count.fetch_add(1, Ordering::SeqCst);
                result
            }
        }
    }
}
