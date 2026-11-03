use futures::{FutureExt, future::Shared};
use gpui::{App, AsyncApp, Context, Task, WeakEntity};
use serde::{Deserialize, Serialize};
use std::{
    any::{Any, TypeId},
    collections::{BTreeMap, HashMap},
    future::Future,
    rc::Rc,
    sync::Arc,
};

pub trait ManagedResource: Sized + Send + Sync + 'static {
    type Key: 'static;
    type Context: ?Sized + 'static;
    type Command: ManagedResourceCommand<Self>;

    /// Must be stable across restarts and unique to this resource type and key.
    /// Include configuration that changes what is authorized; context is not part of the cache key.
    fn id(key: &Self::Key) -> ResourceId;

    fn obtain(
        key: &Self::Key,
        context: &Self::Context,
    ) -> impl Future<Output = anyhow::Result<Self>>;
}

pub trait ManagedResourceCommand<Resource: ManagedResource>: Sized {
    type Output;

    fn run(self, resource: Arc<Resource>) -> impl Future<Output = Self::Output>;
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ResourceId {
    pub kind: String,
    pub key: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResourcePermission {
    Allowed,
    Denied,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ResourcePermissions {
    entries: BTreeMap<String, BTreeMap<String, ResourcePermission>>,
}

impl ResourcePermissions {
    fn permission(&self, id: &ResourceId) -> Option<ResourcePermission> {
        self.entries.get(&id.kind)?.get(&id.key).copied()
    }

    fn set_permission(&mut self, id: ResourceId, permission: ResourcePermission) {
        self.entries
            .entry(id.kind)
            .or_default()
            .insert(id.key, permission);
    }
}

#[derive(Clone, Debug, thiserror::Error)]
pub enum ResourceAccessError {
    #[error("access to the managed resource was denied")]
    Denied,
    #[error("failed to obtain the managed resource: {0}")]
    Obtain(#[source] Arc<anyhow::Error>),
    #[error("the managed resource identity is already in use by another resource type")]
    ConflictingResourceType,
    #[error("the managed resource store was dropped")]
    StoreDropped,
}

type RequestApproval = dyn Fn(ResourceId, &mut App) -> Task<ResourcePermission>;
type ResourceTask = Shared<Task<Result<Arc<dyn Any + Send + Sync>, ResourceAccessError>>>;

pub struct ManagedResourceStore {
    permissions: ResourcePermissions,
    resources: HashMap<ResourceId, ResourceEntry>,
    request_approval: Rc<RequestApproval>,
}

struct ResourceEntry {
    resource_type: TypeId,
    value: Option<Arc<dyn Any + Send + Sync>>,
    pending: Option<ResourceTask>,
    permission_revision: u64,
}

impl ManagedResourceStore {
    pub fn new(
        permissions: ResourcePermissions,
        request_approval: impl Fn(ResourceId, &mut App) -> Task<ResourcePermission> + 'static,
    ) -> Self {
        Self {
            permissions,
            resources: HashMap::new(),
            request_approval: Rc::new(request_approval),
        }
    }

    pub fn permissions(&self) -> ResourcePermissions {
        self.permissions.clone()
    }

    pub fn permission(&self, id: &ResourceId) -> Option<ResourcePermission> {
        self.permissions.permission(id)
    }

    pub fn set_permission(
        &mut self,
        id: ResourceId,
        permission: ResourcePermission,
        cx: &mut Context<Self>,
    ) {
        if let Some(resource) = self.resources.get_mut(&id) {
            resource.permission_revision = resource.permission_revision.wrapping_add(1);
        }
        self.permissions.set_permission(id, permission);
        cx.notify();
    }

    pub fn clear_permission(&mut self, id: &ResourceId, cx: &mut Context<Self>) {
        if let Some(resource) = self.resources.get_mut(id) {
            resource.permission_revision = resource.permission_revision.wrapping_add(1);
        }
        if let Some(permissions) = self.permissions.entries.get_mut(&id.kind) {
            permissions.remove(&id.key);
            if permissions.is_empty() {
                self.permissions.entries.remove(&id.kind);
            }
        }
        cx.notify();
    }

    /// Approval and acquisition are shared; callbacks may run concurrently.
    /// Dropping the returned task cancels only that callback, not shared acquisition.
    /// Permission changes supersede a pending approval and are checked before each
    /// callback, but cannot revoke resource handles already handed out.
    pub fn with_managed_resource<Resource: ManagedResource, Ret: 'static>(
        &mut self,
        key: Resource::Key,
        context: Rc<Resource::Context>,
        cx: &mut Context<Self>,
        operation: impl AsyncFnOnce(Arc<Resource>) -> Ret + 'static,
    ) -> Task<Result<Ret, ResourceAccessError>> {
        let id = Resource::id(&key);
        if self.permission(&id) == Some(ResourcePermission::Denied) {
            return Task::ready(Err(ResourceAccessError::Denied));
        }
        let entry = self
            .resources
            .entry(id.clone())
            .or_insert_with(|| ResourceEntry {
                resource_type: TypeId::of::<Resource>(),
                value: None,
                pending: None,
                permission_revision: 0,
            });
        if entry.resource_type != TypeId::of::<Resource>() {
            return Task::ready(Err(ResourceAccessError::ConflictingResourceType));
        }

        let resource = if let Some(pending) = &entry.pending {
            pending.clone()
        } else {
            let value = entry.value.clone();
            let pending = cx
                .spawn({
                    let id = id.clone();
                    async move |this, cx| {
                        let result = async {
                            Self::ensure_permission(&this, &id, cx).await?;
                            if let Some(value) = value {
                                return Ok(value);
                            }
                            Resource::obtain(&key, &context)
                                .await
                                .map(|resource| Arc::new(resource) as Arc<dyn Any + Send + Sync>)
                                .map_err(|error| ResourceAccessError::Obtain(Arc::new(error)))
                        }
                        .await;
                        this.update(cx, |this, _| {
                            if let Some(entry) = this.resources.get_mut(&id) {
                                entry.pending = None;
                                if let Ok(value) = &result {
                                    entry.value = Some(value.clone());
                                }
                            }
                        })
                        .map_err(|_| ResourceAccessError::StoreDropped)?;
                        result
                    }
                })
                .shared();
            entry.pending = Some(pending.clone());
            pending
        };

        cx.spawn(async move |this, cx| {
            let resource = resource.await?;
            this.read_with(cx, |this, _| Self::check_permission(&this.permissions, &id))
                .map_err(|_| ResourceAccessError::StoreDropped)??;
            let resource = resource
                .downcast::<Resource>()
                .map_err(|_| ResourceAccessError::ConflictingResourceType)?;
            Ok(operation(resource).await)
        })
    }

    async fn ensure_permission(
        this: &WeakEntity<Self>,
        id: &ResourceId,
        cx: &mut AsyncApp,
    ) -> Result<(), ResourceAccessError> {
        let approval = this
            .read_with(cx, |this, _| match this.permission(id) {
                Some(ResourcePermission::Allowed) => Ok(None),
                Some(ResourcePermission::Denied) => Err(ResourceAccessError::Denied),
                None => Ok(Some((
                    this.resources
                        .get(id)
                        .map(|resource| resource.permission_revision),
                    this.request_approval.clone(),
                ))),
            })
            .map_err(|_| ResourceAccessError::StoreDropped)??;
        if let Some((revision, request_approval)) = approval {
            let permission = cx.update(|cx| request_approval(id.clone(), cx)).await;
            this.update(cx, |this, cx| {
                if this
                    .resources
                    .get(id)
                    .map(|resource| resource.permission_revision)
                    == revision
                {
                    this.set_permission(id.clone(), permission, cx);
                }
            })
            .map_err(|_| ResourceAccessError::StoreDropped)?;
            // Observers may revoke permission while the update flushes notifications.
            this.read_with(cx, |this, _| Self::check_permission(&this.permissions, id))
                .map_err(|_| ResourceAccessError::StoreDropped)??;
        }
        Ok(())
    }

    fn check_permission(
        permissions: &ResourcePermissions,
        id: &ResourceId,
    ) -> Result<(), ResourceAccessError> {
        match permissions.permission(id) {
            Some(ResourcePermission::Allowed) => Ok(()),
            Some(ResourcePermission::Denied) | None => Err(ResourceAccessError::Denied),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::{channel::oneshot, poll};
    use gpui::{AppContext as _, Entity, TestAppContext};
    use std::{
        cell::Cell,
        sync::atomic::{AtomicUsize, Ordering::SeqCst},
    };

    type TestResource = Counter<0>;
    type OtherResource = Counter<1>;

    fn deny_approval(_: ResourceId, _: &mut App) -> Task<ResourcePermission> {
        Task::ready(ResourcePermission::Denied)
    }

    fn new_store(cx: &mut TestAppContext) -> Entity<ManagedResourceStore> {
        cx.new(|_| ManagedResourceStore::new(ResourcePermissions::default(), deny_approval))
    }

    fn access<Resource: ManagedResource, Ret: 'static>(
        store: &Entity<ManagedResourceStore>,
        key: Resource::Key,
        context: &Rc<Resource::Context>,
        cx: &mut TestAppContext,
        operation: impl AsyncFnOnce(Arc<Resource>) -> Ret + 'static,
    ) -> Task<Result<Ret, ResourceAccessError>> {
        store.update(cx, |store, cx| {
            store.with_managed_resource(key, context.clone(), cx, operation)
        })
    }

    fn pending_approval_store(
        cx: &mut TestAppContext,
    ) -> (
        Entity<ManagedResourceStore>,
        oneshot::Sender<ResourcePermission>,
    ) {
        let (response, receive_response) = oneshot::channel();
        let receive_response = Cell::new(Some(receive_response));
        let store = cx.new(|_| {
            ManagedResourceStore::new(ResourcePermissions::default(), move |id, cx| {
                assert_eq!(id, TestResource::id(&"first"));
                let receive_response = receive_response
                    .take()
                    .expect("concurrent access must share the approval decision");
                cx.spawn(async move |_| {
                    receive_response.await.expect("approval response cancelled")
                })
            })
        });
        (store, response)
    }

    struct Counter<const KIND: u8> {
        value: AtomicUsize,
    }

    #[derive(Default)]
    struct TestContext {
        obtain_count: Cell<usize>,
        fail_next_obtain: Cell<bool>,
        obtain_ready: Cell<Option<oneshot::Receiver<()>>>,
    }

    #[derive(Debug, PartialEq, Eq, thiserror::Error)]
    enum TestError {
        #[error("acquisition failed")]
        Obtain,
        #[error("command failed")]
        Command,
        #[error("acquisition cancelled")]
        Cancelled,
    }

    impl<const KIND: u8> ManagedResource for Counter<KIND> {
        type Key = &'static str;
        type Context = TestContext;
        type Command = TestCommand;

        fn id(key: &Self::Key) -> ResourceId {
            ResourceId {
                kind: format!("counter-{KIND}"),
                key: key.to_string(),
            }
        }

        async fn obtain(key: &Self::Key, context: &Self::Context) -> anyhow::Result<Self> {
            context.obtain_count.set(context.obtain_count.get() + 1);
            if let Some(obtain_ready) = context.obtain_ready.take() {
                obtain_ready.await.map_err(|_| TestError::Cancelled)?;
            }
            if context.fail_next_obtain.replace(false) {
                return Err(TestError::Obtain.into());
            }
            Ok(Self {
                value: AtomicUsize::new(key.len()),
            })
        }
    }

    enum TestCommand {
        Add(usize),
        Fail,
    }

    impl<const KIND: u8> ManagedResourceCommand<Counter<KIND>> for TestCommand {
        type Output = Result<usize, TestError>;

        async fn run(self, resource: Arc<Counter<KIND>>) -> Self::Output {
            match self {
                Self::Add(value) => Ok(resource.value.fetch_add(value, SeqCst) + value),
                Self::Fail => Err(TestError::Command),
            }
        }
    }

    #[gpui::test]
    async fn permissions_gate_acquisition_and_cached_access(cx: &mut TestAppContext) {
        let approval_count = Rc::new(Cell::new(0));
        let store = cx.new(|_| {
            ManagedResourceStore::new(ResourcePermissions::default(), {
                let approval_count = approval_count.clone();
                move |_, _| {
                    approval_count.set(approval_count.get() + 1);
                    Task::ready(ResourcePermission::Denied)
                }
            })
        });
        let context = Rc::new(TestContext::default());
        let id = TestResource::id(&"first");

        for _ in 0..2 {
            let result = access::<TestResource, _>(&store, "first", &context, cx, async |_| {
                panic!("an unapproved resource must not reach the callback")
            })
            .await;
            assert!(matches!(result, Err(ResourceAccessError::Denied)));
        }
        assert_eq!(
            store.read_with(cx, |store, _| store.permission(&id)),
            Some(ResourcePermission::Denied)
        );
        assert_eq!(approval_count.get(), 1);
        assert_eq!(context.obtain_count.get(), 0);

        store.update(cx, |store, cx| {
            store.set_permission(id.clone(), ResourcePermission::Allowed, cx)
        });
        let retained = access::<TestResource, _>(&store, "first", &context, cx, async |resource| {
            assert_eq!(TestCommand::Add(7).run(resource.clone()).await, Ok(12));
            resource
        })
        .await
        .expect("resource allowed");

        store.update(cx, |store, cx| {
            store.set_permission(id.clone(), ResourcePermission::Denied, cx)
        });
        let result = access::<TestResource, _>(&store, "first", &context, cx, async |_| {
            panic!("a cached resource must not bypass denial")
        })
        .await;
        assert!(matches!(result, Err(ResourceAccessError::Denied)));
        assert_eq!(retained.value.load(SeqCst), 12);

        store.update(cx, |store, cx| store.clear_permission(&id, cx));
        let result = access::<TestResource, _>(&store, "first", &context, cx, async |_| {
            panic!("a cached resource must not bypass missing approval")
        })
        .await;
        assert!(matches!(result, Err(ResourceAccessError::Denied)));
        assert_eq!(approval_count.get(), 2);

        store.update(cx, |store, cx| {
            store.set_permission(id, ResourcePermission::Allowed, cx)
        });
        let cached =
            access::<TestResource, _>(&store, "first", &context, cx, async |resource| resource)
                .await
                .expect("resource allowed again");
        assert!(Arc::ptr_eq(&cached, &retained));
        let result = cx
            .background_executor
            .spawn(async move { TestCommand::Add(3).run(cached).await })
            .await;
        assert_eq!(result, Ok(15));
        assert_eq!(retained.value.load(SeqCst), 15);
        assert_eq!(context.obtain_count.get(), 1);
        assert_eq!(approval_count.get(), 2);
    }

    #[gpui::test]
    async fn permissions_round_trip_without_sharing_live_resources(cx: &mut TestAppContext) {
        let store = new_store(cx);
        let context = Rc::new(TestContext::default());
        store.update(cx, |store, cx| {
            store.set_permission(TestResource::id(&"first"), ResourcePermission::Allowed, cx);
            store.set_permission(TestResource::id(&"second"), ResourcePermission::Denied, cx);
            store.set_permission(OtherResource::id(&"first"), ResourcePermission::Denied, cx);
        });
        let original =
            access::<TestResource, _>(&store, "first", &context, cx, async |resource| resource)
                .await
                .expect("resource allowed");
        assert_eq!(TestCommand::Add(7).run(original.clone()).await, Ok(12));

        let permissions = store.read_with(cx, |store, _| store.permissions());
        let serialized = serde_json::to_value(&permissions).expect("serializable permissions");
        assert_eq!(
            serialized,
            serde_json::json!({
                "counter-0": { "first": "allowed", "second": "denied" },
                "counter-1": { "first": "denied" },
            })
        );
        let restored = cx.new(|_| {
            ManagedResourceStore::new(
                serde_json::from_value(serialized).expect("restorable permissions"),
                deny_approval,
            )
        });
        assert_eq!(
            restored.read_with(cx, |store, _| store.permissions()),
            permissions
        );
        let resource =
            access::<TestResource, _>(&restored, "first", &context, cx, async |resource| resource)
                .await
                .expect("restored permission");
        assert!(!Arc::ptr_eq(&original, &resource));
        assert_eq!(TestCommand::Add(11).run(resource).await, Ok(16));
        assert_eq!(context.obtain_count.get(), 2);

        for key in ["second", "third"] {
            let result = access::<TestResource, _>(&restored, key, &context, cx, async |_| {
                panic!("approval must not extend to another key")
            })
            .await;
            assert!(matches!(result, Err(ResourceAccessError::Denied)));
        }
        let result = access::<OtherResource, _>(&restored, "first", &context, cx, async |_| {
            panic!("approval must not extend to another resource kind")
        })
        .await;
        assert!(matches!(result, Err(ResourceAccessError::Denied)));
        assert_eq!(context.obtain_count.get(), 2);

        restored.update(cx, |store, cx| {
            store.set_permission(OtherResource::id(&"first"), ResourcePermission::Allowed, cx)
        });
        let value =
            access::<OtherResource, _>(&restored, "first", &context, cx, async |resource| {
                resource.value.load(SeqCst)
            })
            .await
            .expect("other resource allowed");
        assert_eq!(value, 5);
        assert_eq!(context.obtain_count.get(), 3);
    }

    #[gpui::test]
    async fn acquisition_failures_are_shared_and_retryable(cx: &mut TestAppContext) {
        let store = new_store(cx);
        let context = Rc::new(TestContext::default());
        context.fail_next_obtain.set(true);
        let (ready, wait) = oneshot::channel();
        context.obtain_ready.set(Some(wait));
        store.update(cx, |store, cx| {
            store.set_permission(TestResource::id(&"first"), ResourcePermission::Allowed, cx)
        });
        let first = access::<TestResource, _>(&store, "first", &context, cx, async |_| {
            panic!("failed acquisition must not reach the callback")
        });
        let second = access::<TestResource, _>(&store, "first", &context, cx, async |_| {
            panic!("failed acquisition must not reach another callback")
        });
        cx.run_until_parked();
        assert_eq!(context.obtain_count.get(), 1);
        ready.send(()).expect("acquisition pending");
        let Err(ResourceAccessError::Obtain(first_error)) = first.await else {
            panic!("expected acquisition failure");
        };
        let Err(ResourceAccessError::Obtain(second_error)) = second.await else {
            panic!("expected shared acquisition failure");
        };
        assert!(Arc::ptr_eq(&first_error, &second_error));
        assert_eq!(
            first_error.downcast_ref::<TestError>(),
            Some(&TestError::Obtain)
        );

        let value = access::<TestResource, _>(&store, "first", &context, cx, async |resource| {
            TestCommand::Add(4).run(resource).await
        })
        .await
        .expect("acquisition retried");
        assert_eq!(value, Ok(9));
        let result = access::<TestResource, _>(&store, "first", &context, cx, async |resource| {
            TestCommand::Fail.run(resource).await
        })
        .await
        .expect("resource cached");
        assert_eq!(result, Err(TestError::Command));
        assert_eq!(context.obtain_count.get(), 2);
    }

    #[gpui::test]
    async fn concurrent_access_shares_acquisition_without_serializing_callbacks(
        cx: &mut TestAppContext,
    ) {
        let store = new_store(cx);
        let context = Rc::new(TestContext::default());
        let (obtain_ready, obtain_wait) = oneshot::channel();
        context.obtain_ready.set(Some(obtain_wait));
        let (command_ready, command_wait) = oneshot::channel();
        let command_started = Rc::new(Cell::new(false));
        store.update(cx, |store, cx| {
            store.set_permission(TestResource::id(&"first"), ResourcePermission::Allowed, cx);
            store.set_permission(
                TestResource::id(&"different"),
                ResourcePermission::Allowed,
                cx,
            );
        });

        let mut first = access::<TestResource, _>(&store, "first", &context, cx, {
            let command_started = command_started.clone();
            async move |resource| {
                command_started.set(true);
                command_wait.await.expect("command released");
                resource
            }
        });
        let mut second =
            access::<TestResource, _>(&store, "first", &context, cx, async |resource| resource);
        cx.run_until_parked();
        assert!(poll!(&mut first).is_pending());
        assert!(poll!(&mut second).is_pending());
        assert_eq!(context.obtain_count.get(), 1);
        assert!(!command_started.get());

        let different =
            access::<TestResource, _>(&store, "different", &context, cx, async |resource| {
                resource.value.load(SeqCst)
            })
            .await
            .expect("other resource independent");
        assert_eq!(different, 9);

        obtain_ready.send(()).expect("acquisition pending");
        cx.run_until_parked();
        assert!(command_started.get());
        assert!(poll!(&mut first).is_pending());
        let second = second
            .await
            .expect("second callback does not wait for first");
        assert_eq!(TestCommand::Add(7).run(second.clone()).await, Ok(12));
        command_ready.send(()).expect("first callback pending");
        let first = first.await.expect("first callback completed");
        assert!(Arc::ptr_eq(&first, &second));
        assert_eq!(first.value.load(SeqCst), 12);
        assert_eq!(context.obtain_count.get(), 2);
    }

    #[gpui::test]
    async fn revocation_blocks_new_callbacks_without_cancelling_running_callbacks(
        cx: &mut TestAppContext,
    ) {
        let store = new_store(cx);
        let context = Rc::new(TestContext::default());
        let id = TestResource::id(&"first");
        store.update(cx, |store, cx| {
            store.set_permission(id.clone(), ResourcePermission::Allowed, cx)
        });
        let (command_ready, command_wait) = oneshot::channel();
        let started = Rc::new(Cell::new(false));
        let running = access::<TestResource, _>(&store, "first", &context, cx, {
            let started = started.clone();
            async move |resource| {
                started.set(true);
                command_wait.await.expect("command released");
                TestCommand::Add(2).run(resource).await
            }
        });
        cx.run_until_parked();
        assert!(started.get());
        let waiting = access::<TestResource, _>(&store, "first", &context, cx, async |_| {
            panic!("a callback not yet started must observe revocation")
        });
        store.update(cx, |store, cx| {
            store.set_permission(id, ResourcePermission::Denied, cx)
        });
        command_ready.send(()).expect("command pending");
        assert_eq!(running.await.expect("running callback allowed"), Ok(7));
        assert!(matches!(waiting.await, Err(ResourceAccessError::Denied)));
    }

    #[gpui::test]
    async fn revocation_during_acquisition_prevents_callbacks(cx: &mut TestAppContext) {
        for permission in [Some(ResourcePermission::Denied), None] {
            let store = new_store(cx);
            let context = Rc::new(TestContext::default());
            let id = TestResource::id(&"first");
            store.update(cx, |store, cx| {
                store.set_permission(id.clone(), ResourcePermission::Allowed, cx)
            });
            let (obtain_ready, obtain_wait) = oneshot::channel();
            context.obtain_ready.set(Some(obtain_wait));
            let first = access::<TestResource, _>(&store, "first", &context, cx, async |_| {
                panic!("revocation during acquisition must prevent the callback")
            });
            let second = access::<TestResource, _>(&store, "first", &context, cx, async |_| {
                panic!("revocation must also prevent concurrent callbacks")
            });
            cx.run_until_parked();
            assert_eq!(context.obtain_count.get(), 1);
            store.update(cx, |store, cx| match permission {
                Some(permission) => store.set_permission(id.clone(), permission, cx),
                None => store.clear_permission(&id, cx),
            });
            obtain_ready.send(()).expect("acquisition pending");
            assert!(matches!(first.await, Err(ResourceAccessError::Denied)));
            assert!(matches!(second.await, Err(ResourceAccessError::Denied)));

            store.update(cx, |store, cx| {
                store.set_permission(id, ResourcePermission::Allowed, cx)
            });
            let value =
                access::<TestResource, _>(&store, "first", &context, cx, async |resource| {
                    resource.value.load(SeqCst)
                })
                .await
                .expect("completed acquisition cached");
            assert_eq!(value, 5);
            assert_eq!(context.obtain_count.get(), 1);
        }
    }

    #[gpui::test]
    async fn cancelled_callback_does_not_cancel_shared_acquisition(cx: &mut TestAppContext) {
        let store = new_store(cx);
        let context = Rc::new(TestContext::default());
        store.update(cx, |store, cx| {
            store.set_permission(TestResource::id(&"first"), ResourcePermission::Allowed, cx)
        });
        let (obtain_ready, obtain_wait) = oneshot::channel();
        context.obtain_ready.set(Some(obtain_wait));
        let cancelled = access::<TestResource, _>(&store, "first", &context, cx, async |_| {
            panic!("cancelled callback must not run")
        });
        cx.run_until_parked();
        assert_eq!(context.obtain_count.get(), 1);
        drop(cancelled);
        cx.run_until_parked();
        obtain_ready
            .send(())
            .expect("shared acquisition still pending");
        cx.run_until_parked();
        let value = access::<TestResource, _>(&store, "first", &context, cx, async |resource| {
            resource.value.load(SeqCst)
        })
        .await
        .expect("completed acquisition cached without waiters");
        assert_eq!(value, 5);
        assert_eq!(context.obtain_count.get(), 1);
    }

    #[gpui::test]
    async fn conflicting_resource_types_cannot_reuse_an_identity(cx: &mut TestAppContext) {
        struct ConflictingResource;

        impl ManagedResource for ConflictingResource {
            type Key = ();
            type Context = ();
            type Command = Self;

            fn id(_: &()) -> ResourceId {
                TestResource::id(&"first")
            }

            async fn obtain(_: &(), _: &()) -> anyhow::Result<Self> {
                panic!("a conflicting identity must not be acquired")
            }
        }

        impl ManagedResourceCommand<Self> for ConflictingResource {
            type Output = ();

            async fn run(self, _: Arc<Self>) {}
        }

        let store = new_store(cx);
        let context = Rc::new(TestContext::default());
        store.update(cx, |store, cx| {
            store.set_permission(TestResource::id(&"first"), ResourcePermission::Allowed, cx)
        });
        let first = access::<TestResource, _>(&store, "first", &context, cx, async |_| ());
        let result = access::<ConflictingResource, _>(&store, (), &Rc::new(()), cx, async |_| {
            panic!("a conflicting resource must not reach the callback")
        })
        .await;
        assert!(matches!(
            result,
            Err(ResourceAccessError::ConflictingResourceType)
        ));
        first.await.expect("original resource allowed");
    }

    #[gpui::test]
    async fn approval_is_awaited_and_shared_before_acquisition(cx: &mut TestAppContext) {
        for permission in [ResourcePermission::Allowed, ResourcePermission::Denied] {
            let (store, approve) = pending_approval_store(cx);
            let context = Rc::new(TestContext::default());
            let mut first =
                access::<TestResource, _>(&store, "first", &context, cx, async |resource| resource);
            let mut second =
                access::<TestResource, _>(&store, "first", &context, cx, async |resource| resource);
            cx.run_until_parked();
            assert!(poll!(&mut first).is_pending());
            assert!(poll!(&mut second).is_pending());
            assert_eq!(context.obtain_count.get(), 0);
            assert_eq!(
                store.read_with(cx, |store, _| store.permission(&TestResource::id(&"first"))),
                None
            );

            store.update(cx, |store, cx| {
                store.set_permission(
                    TestResource::id(&"different"),
                    ResourcePermission::Allowed,
                    cx,
                )
            });
            let different =
                access::<TestResource, _>(&store, "different", &context, cx, async |resource| {
                    resource.value.load(SeqCst)
                })
                .await
                .expect("other resource independent of approval");
            assert_eq!(different, 9);

            approve.send(permission).expect("approval pending");
            match permission {
                ResourcePermission::Allowed => {
                    let first = first.await.expect("approval granted");
                    let second = second.await.expect("approval shared");
                    assert!(Arc::ptr_eq(&first, &second));
                    assert_eq!(first.value.load(SeqCst), 5);
                    assert_eq!(context.obtain_count.get(), 2);
                }
                ResourcePermission::Denied => {
                    assert!(matches!(first.await, Err(ResourceAccessError::Denied)));
                    assert!(matches!(second.await, Err(ResourceAccessError::Denied)));
                    assert_eq!(context.obtain_count.get(), 1);
                }
            }
            assert_eq!(
                store.read_with(cx, |store, _| store.permission(&TestResource::id(&"first"))),
                Some(permission)
            );
        }
    }

    #[gpui::test]
    async fn permission_changes_supersede_pending_approval(cx: &mut TestAppContext) {
        for response in [ResourcePermission::Allowed, ResourcePermission::Denied] {
            for permission in [
                Some(ResourcePermission::Allowed),
                Some(ResourcePermission::Denied),
                None,
            ] {
                let (store, approve) = pending_approval_store(cx);
                let context = Rc::new(TestContext::default());
                let id = TestResource::id(&"first");
                let pending =
                    access::<TestResource, _>(&store, "first", &context, cx, async |resource| {
                        resource.value.load(SeqCst)
                    });
                cx.run_until_parked();
                store.update(cx, |store, cx| match permission {
                    Some(permission) => store.set_permission(id.clone(), permission, cx),
                    None => store.clear_permission(&id, cx),
                });
                approve.send(response).expect("approval pending");
                if permission == Some(ResourcePermission::Allowed) {
                    assert_eq!(pending.await.expect("explicit approval wins"), 5);
                    assert_eq!(context.obtain_count.get(), 1);
                } else {
                    assert!(matches!(pending.await, Err(ResourceAccessError::Denied)));
                    assert_eq!(context.obtain_count.get(), 0);
                }
                assert_eq!(
                    store.read_with(cx, |store, _| store.permission(&id)),
                    permission
                );
            }
        }
    }

    #[gpui::test]
    async fn permission_observers_can_revoke_before_acquisition(cx: &mut TestAppContext) {
        let (store, approve) = pending_approval_store(cx);
        let context = Rc::new(TestContext::default());
        let observed = Rc::new(Cell::new(None));
        let _subscription = cx.update(|cx| {
            cx.observe(&store, {
                let observed = observed.clone();
                move |store, cx| {
                    let id = TestResource::id(&"first");
                    let permission = store.read(cx).permission(&id);
                    observed.set(permission);
                    if permission == Some(ResourcePermission::Allowed) {
                        store.update(cx, |store, cx| {
                            store.set_permission(id, ResourcePermission::Denied, cx);
                        });
                    }
                }
            })
        });
        let pending = access::<TestResource, _>(&store, "first", &context, cx, async |_| {
            panic!("observer revoked access")
        });
        cx.run_until_parked();
        approve
            .send(ResourcePermission::Allowed)
            .expect("approval pending");
        assert!(matches!(pending.await, Err(ResourceAccessError::Denied)));
        assert_eq!(context.obtain_count.get(), 0);
        assert_eq!(observed.get(), Some(ResourcePermission::Denied));

        store.update(cx, |store, cx| {
            store.clear_permission(&TestResource::id(&"first"), cx)
        });
        assert_eq!(observed.get(), None);
    }

    #[gpui::test]
    async fn cancelled_callback_does_not_cancel_shared_approval(cx: &mut TestAppContext) {
        let (store, approve) = pending_approval_store(cx);
        let context = Rc::new(TestContext::default());
        let cancelled = access::<TestResource, _>(&store, "first", &context, cx, async |_| {
            panic!("cancelled callback must not run")
        });
        cx.run_until_parked();
        drop(cancelled);
        cx.run_until_parked();
        let pending = access::<TestResource, _>(&store, "first", &context, cx, async |resource| {
            resource.value.load(SeqCst)
        });
        cx.run_until_parked();
        assert_eq!(context.obtain_count.get(), 0);
        approve
            .send(ResourcePermission::Allowed)
            .expect("approval still pending");
        assert_eq!(pending.await.expect("shared approval granted"), 5);
        assert_eq!(context.obtain_count.get(), 1);
    }

    #[gpui::test]
    async fn dropping_store_prevents_pending_callbacks(cx: &mut TestAppContext) {
        let (store, approve) = pending_approval_store(cx);
        let weak_store = store.downgrade();
        let context = Rc::new(TestContext::default());
        let pending = access::<TestResource, _>(&store, "first", &context, cx, async |_| {
            panic!("a dropped store must not start a callback")
        });
        cx.run_until_parked();
        drop(store);
        assert!(weak_store.upgrade().is_none());
        approve
            .send(ResourcePermission::Allowed)
            .expect("approval pending");
        assert!(matches!(
            pending.await,
            Err(ResourceAccessError::StoreDropped)
        ));
        assert_eq!(context.obtain_count.get(), 0);
    }
}
