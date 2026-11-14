//! This crate defines resources managed by Zed.
//!
//! In order to function properly, Zed has to fetch and invoke various binaries; language servers, node runtime, debuggers and a bunch of others.
//! In the past, it did so without consulting the user. Which binary was ran was not really clear to users, as discussed in e.g. #12589.
//!
//! This crate attempts to define a common entry point for all external entities we need to interface with in order to implement core Zed functionalities.
//! A Resource's definition is very lenient: it is any operation through which we might modify perform an operation on user's behalf that may affect their machine's state.
//! By that definition, sending a HTTP request is a resource.
//!
//! In order to interact with a resource, you ought to implement [ManagedResourceCommand].
//! One example of a resource is a language server binary. Zed may need to install it on user's behalf. The user may also want us to use the binary available on their system.
//! But then, whenever we want to invoke said binary, we should let the user know as well, as running arbitrary processes on their PC might affect their machine state.
use futures::{
    FutureExt,
    future::{LocalBoxFuture, Shared},
};
use gpui::{App, AsyncApp, Context, Subscription, Task, WeakEntity};
use serde::{Deserialize, Serialize};
pub use settings::{CommandPermissionRule, NpmResourceId, ResourceId, ResourcePermission};
use settings::{ResourcePermissionsContent, Settings, SettingsContent, SettingsStore};
use std::{
    any::{Any, TypeId},
    collections::HashMap,
    future::Future,
    rc::Rc,
    sync::Arc,
};

pub enum ExecutionBillRoot {
    Extension,
    Debugger,
    LanguageServer,
}

pub struct ResourceExecutionBill {
    pub root: ExecutionBillRoot,
    pub reasoning: Vec<()>,
}

pub type ResourceAction<Resource> =
    Box<dyn FnOnce() -> LocalBoxFuture<'static, anyhow::Result<Resource>>>;

pub enum ResourceAcquisition<Resource> {
    Ready {
        resource: Resource,
        action: Option<ResourceAction<Resource>>,
    },
    RequiresAction(ResourceAction<Resource>),
}

pub trait ManagedResource: Sized + Send + Sync + 'static {
    type Key: 'static;
    type Context: ?Sized + 'static;
    type Command: ManagedResourceCommand<Self>;
    type CommandKind: Serialize + PartialEq;

    /// Must be stable across restarts and unique to this resource type and key.
    /// Include configuration that changes what is authorized; context is not part of the cache key.
    fn id(key: &Self::Key) -> ResourceId;

    #[expect(async_fn_in_trait)]
    async fn obtain(
        key: &Self::Key,
        context: &Self::Context,
    ) -> anyhow::Result<ResourceAcquisition<Self>>;
}

pub trait ManagedResourceCommand<Resource: ManagedResource>: Sized + 'static {
    type Output: 'static;
    type PermissionSettings: serde::de::DeserializeOwned + Default;

    fn command_kind(&self) -> Resource::CommandKind;

    fn permission(&self, settings: &Self::PermissionSettings) -> Option<ResourcePermission>;

    /// Describe the specific operation for approval without performing it.
    fn permission_description(&self, resource: &Resource) -> String;

    fn run(self, resource: Arc<Resource>) -> impl Future<Output = Self::Output>;
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ResourcePermissionRequest {
    Obtain(ResourceId),
    ExecuteCommand {
        resource: ResourceId,
        description: String,
        payload: serde_json::Value,
    },
}

#[derive(
    Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, settings::RegisterSetting,
)]
#[serde(transparent)]
pub struct ResourcePermissions {
    content: ResourcePermissionsContent,
}

impl Settings for ResourcePermissions {
    fn from_settings(content: &SettingsContent) -> Self {
        Self {
            content: content
                .experimental_permissions
                .clone()
                .expect("missing default resource permissions"),
        }
    }
}

impl ResourcePermissions {
    fn permission(&self, id: &ResourceId) -> Option<ResourcePermission> {
        self.content
            .resource(id)
            .and_then(|resource| resource.obtain)
            .or(self.content.default)
    }

    fn command_permission<Resource: ManagedResource>(
        &self,
        id: &ResourceId,
        command: &Resource::Command,
    ) -> Result<Option<ResourcePermission>, ResourceAccessError> {
        let settings = if let Some(resource) = self.content.resource(id) {
            serde_json::from_value(serde_json::Value::Object(
                resource.settings.clone().into_iter().collect(),
            ))
            .map_err(|error| ResourceAccessError::CommandSettings(Arc::new(error)))?
        } else {
            Default::default()
        };
        Ok(command.permission(&settings).or(self.content.default))
    }

    fn set_permission(&mut self, id: ResourceId, permission: ResourcePermission) {
        self.content.set_obtain_permission(id, Some(permission));
    }
}

#[derive(Clone, Debug, thiserror::Error)]
pub enum ResourceAccessError {
    #[error("access to the managed resource was denied")]
    Denied,
    #[error("permission to execute the managed resource command was denied")]
    CommandDenied,
    #[error("failed to identify the managed resource command: {0}")]
    CommandPayload(#[source] Arc<serde_json::Error>),
    #[error("invalid managed resource command settings: {0}")]
    CommandSettings(#[source] Arc<serde_json::Error>),
    #[error("failed to obtain the managed resource: {0}")]
    Obtain(#[source] Arc<anyhow::Error>),
    #[error("the managed resource identity is already in use by another resource type")]
    ConflictingResourceType,
    #[error("the managed resource store was dropped")]
    StoreDropped,
}

type RequestApproval = dyn Fn(ResourcePermissionRequest, &mut App) -> Task<ResourcePermission>;
type ResourceTask = Shared<Task<Result<Arc<dyn Any + Send + Sync>, ResourceAccessError>>>;

pub struct ManagedResourceStore {
    permissions: ResourcePermissions,
    resources: HashMap<ResourceId, ResourceEntry>,
    request_approval: Rc<RequestApproval>,
    fs: Option<Arc<dyn fs::Fs>>,
    _settings_subscription: Option<Subscription>,
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
        request_approval: impl Fn(ResourcePermissionRequest, &mut App) -> Task<ResourcePermission>
        + 'static,
    ) -> Self {
        Self {
            permissions,
            resources: HashMap::new(),
            request_approval: Rc::new(request_approval),
            fs: None,
            _settings_subscription: None,
        }
    }

    pub fn from_settings(
        fs: Arc<dyn fs::Fs>,
        request_approval: impl Fn(ResourcePermissionRequest, &mut App) -> Task<ResourcePermission>
        + 'static,
        cx: &mut Context<Self>,
    ) -> Self {
        let mut store = Self::new(
            ResourcePermissions::get_global(cx).clone(),
            request_approval,
        );
        store.fs = Some(fs);
        store._settings_subscription = Some(cx.observe_global::<SettingsStore>(|this, cx| {
            let permissions = ResourcePermissions::get_global(cx);
            if this.permissions != *permissions {
                this.permissions = permissions.clone();
                for resource in this.resources.values_mut() {
                    resource.permission_revision = resource.permission_revision.wrapping_add(1);
                }
                cx.notify();
            }
        }));
        store
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
        self.permissions.set_permission(id.clone(), permission);
        self.persist_permission(id, Some(permission), cx);
        cx.notify();
    }

    pub fn clear_permission(&mut self, id: &ResourceId, cx: &mut Context<Self>) {
        if let Some(resource) = self.resources.get_mut(id) {
            resource.permission_revision = resource.permission_revision.wrapping_add(1);
        }
        self.permissions
            .content
            .set_obtain_permission(id.clone(), None);
        self.persist_permission(id.clone(), None, cx);
        cx.notify();
    }

    fn persist_permission(&self, id: ResourceId, permission: Option<ResourcePermission>, cx: &App) {
        let Some(fs) = &self.fs else { return };
        cx.global::<SettingsStore>()
            .update_settings_file(fs.clone(), move |settings, _| {
                let permissions = settings.experimental_permissions.get_or_insert_default();
                permissions.set_obtain_permission(id, permission);
            });
    }

    /// Resource approval and acquisition are shared; commands may run concurrently.
    /// Commands without a configured permission require separate approval, which is not persisted.
    /// Dropping the returned task cancels that command, not shared acquisition.
    /// Resource permission is rechecked after command approval; revocation cannot
    /// cancel commands that have already started running.
    pub fn execute_command<Resource: ManagedResource>(
        &mut self,
        key: Resource::Key,
        context: Rc<Resource::Context>,
        _reason: ResourceExecutionBill,
        command: Resource::Command,
        cx: &mut Context<Self>,
    ) -> Task<
        Result<
            <Resource::Command as ManagedResourceCommand<Resource>>::Output,
            ResourceAccessError,
        >,
    > {
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
                            let acquisition = Resource::obtain(&key, &context)
                                .await
                                .map_err(|error| ResourceAccessError::Obtain(Arc::new(error)))?;
                            let resource = match acquisition {
                                // An available update is not a request to perform it.
                                ResourceAcquisition::Ready { resource, .. } => resource,
                                ResourceAcquisition::RequiresAction(action) => {
                                    this.read_with(cx, |this, _| {
                                        Self::check_permission(&this.permissions, &id)
                                    })
                                    .map_err(|_| ResourceAccessError::StoreDropped)??;
                                    action().await.map_err(|error| {
                                        ResourceAccessError::Obtain(Arc::new(error))
                                    })?
                                }
                            };
                            Ok(Arc::new(resource) as Arc<dyn Any + Send + Sync>)
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
            let payload = serde_json::to_value(command.command_kind())
                .map_err(|error| ResourceAccessError::CommandPayload(Arc::new(error)))?;
            let request_approval = this
                .read_with(cx, |this, _| {
                    Self::check_permission(&this.permissions, &id)?;
                    Ok(this.request_approval.clone())
                })
                .map_err(|_| ResourceAccessError::StoreDropped)??;
            let resource = resource
                .downcast::<Resource>()
                .map_err(|_| ResourceAccessError::ConflictingResourceType)?;
            let request = ResourcePermissionRequest::ExecuteCommand {
                resource: id.clone(),
                description: command.permission_description(&resource),
                payload: payload.clone(),
            };
            let permission = this
                .read_with(cx, |this, _| {
                    this.permissions
                        .command_permission::<Resource>(&id, &command)
                })
                .map_err(|_| ResourceAccessError::StoreDropped)??;
            let permission = match permission {
                Some(ResourcePermission::Allowed) => ResourcePermission::Allowed,
                Some(ResourcePermission::Denied) => ResourcePermission::Denied,
                Some(ResourcePermission::Prompt) | None => {
                    cx.update(|cx| request_approval(request, cx)).await
                }
            };
            this.read_with(cx, |this, _| Self::check_permission(&this.permissions, &id))
                .map_err(|_| ResourceAccessError::StoreDropped)??;
            let current_permission = this
                .read_with(cx, |this, _| {
                    this.permissions
                        .command_permission::<Resource>(&id, &command)
                })
                .map_err(|_| ResourceAccessError::StoreDropped)??;
            let permission = match current_permission {
                Some(ResourcePermission::Allowed) => ResourcePermission::Allowed,
                Some(ResourcePermission::Denied) => ResourcePermission::Denied,
                Some(ResourcePermission::Prompt) | None => permission,
            };
            if permission != ResourcePermission::Allowed {
                return Err(ResourceAccessError::CommandDenied);
            }
            Ok(command.run(resource).await)
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
                Some(ResourcePermission::Prompt) | None => Ok(Some((
                    this.resources
                        .get(id)
                        .map(|resource| resource.permission_revision),
                    this.request_approval.clone(),
                    this.permission(id).is_none(),
                ))),
            })
            .map_err(|_| ResourceAccessError::StoreDropped)??;
        if let Some((revision, request_approval, remember)) = approval {
            let permission = cx
                .update(|cx| request_approval(ResourcePermissionRequest::Obtain(id.clone()), cx))
                .await;
            if permission == ResourcePermission::Prompt {
                return Err(ResourceAccessError::Denied);
            }
            if !remember {
                if permission != ResourcePermission::Allowed {
                    return Err(ResourceAccessError::Denied);
                }
                return this
                    .read_with(cx, |this, _| {
                        if this
                            .resources
                            .get(id)
                            .map(|resource| resource.permission_revision)
                            == revision
                        {
                            Self::check_permission(&this.permissions, id)
                        } else if this.permission(id) == Some(ResourcePermission::Allowed) {
                            Ok(())
                        } else {
                            Err(ResourceAccessError::Denied)
                        }
                    })
                    .map_err(|_| ResourceAccessError::StoreDropped)?;
            }
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
            Some(ResourcePermission::Allowed | ResourcePermission::Prompt) => Ok(()),
            Some(ResourcePermission::Denied) | None => Err(ResourceAccessError::Denied),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::{channel::oneshot, poll};
    use gpui::{AppContext as _, Entity, TestAppContext};
    use settings::RootUserSettings as _;
    use std::{
        cell::{Cell, RefCell},
        collections::VecDeque,
        sync::atomic::{AtomicUsize, Ordering::SeqCst},
    };

    type TestResource = Counter<0>;
    type OtherResource = Counter<1>;

    fn deny_obtain_allow_commands(
        request: ResourcePermissionRequest,
        _: &mut App,
    ) -> Task<ResourcePermission> {
        Task::ready(match request {
            ResourcePermissionRequest::Obtain(_) => ResourcePermission::Denied,
            ResourcePermissionRequest::ExecuteCommand { .. } => ResourcePermission::Allowed,
        })
    }

    fn new_store(cx: &mut TestAppContext) -> Entity<ManagedResourceStore> {
        cx.new(|_| {
            ManagedResourceStore::new(ResourcePermissions::default(), deny_obtain_allow_commands)
        })
    }

    fn execute<const KIND: u8>(
        store: &Entity<ManagedResourceStore>,
        key: &'static str,
        context: &Rc<TestContext>,
        command: TestCommand,
        cx: &mut TestAppContext,
    ) -> Task<Result<Result<usize, TestError>, ResourceAccessError>> {
        store.update(cx, |store, cx| {
            store.execute_command::<Counter<KIND>>(
                key,
                context.clone(),
                ResourceExecutionBill {
                    root: ExecutionBillRoot::LanguageServer,
                    reasoning: Vec::new(),
                },
                command,
                cx,
            )
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
            ManagedResourceStore::new(ResourcePermissions::default(), move |request, cx| {
                let ResourcePermissionRequest::Obtain(id) = request else {
                    return Task::ready(ResourcePermission::Allowed);
                };
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

    fn pending_command_approval_store(
        cx: &mut TestAppContext,
    ) -> (
        Entity<ManagedResourceStore>,
        oneshot::Sender<ResourcePermission>,
    ) {
        let (response, receive_response) = oneshot::channel();
        let receive_response = Cell::new(Some(receive_response));
        let store = cx.new(|_| {
            ManagedResourceStore::new(ResourcePermissions::default(), move |request, cx| {
                if matches!(request, ResourcePermissionRequest::ExecuteCommand { .. })
                    && let Some(receive_response) = receive_response.take()
                {
                    return cx.spawn(async move |_| {
                        receive_response.await.expect("approval response cancelled")
                    });
                }
                Task::ready(ResourcePermission::Allowed)
            })
        });
        (store, response)
    }

    struct Counter<const KIND: u8> {
        value: AtomicUsize,
    }

    #[derive(Clone, Default)]
    struct TestContext {
        obtain_count: Rc<Cell<usize>>,
        fail_next_obtain: Rc<Cell<bool>>,
        obtain_ready: Rc<Cell<Option<oneshot::Receiver<()>>>>,
        available_value: Option<usize>,
        update_value: Option<usize>,
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
        type CommandKind = TestCommandKind;

        fn id(key: &Self::Key) -> ResourceId {
            ResourceId::LanguageServer {
                payload: settings::LanguageServerResourceId {
                    name: format!("counter-{KIND}-{key}"),
                },
            }
        }

        async fn obtain(
            key: &Self::Key,
            context: &Self::Context,
        ) -> anyhow::Result<ResourceAcquisition<Self>> {
            let value = context.update_value.unwrap_or(key.len());
            let action: ResourceAction<Self> = Box::new({
                let context = context.clone();
                move || {
                    async move {
                        context.obtain_count.set(context.obtain_count.get() + 1);
                        if let Some(obtain_ready) = context.obtain_ready.take() {
                            obtain_ready.await.map_err(|_| TestError::Cancelled)?;
                        }
                        if context.fail_next_obtain.replace(false) {
                            return Err(TestError::Obtain.into());
                        }
                        Ok(Self {
                            value: AtomicUsize::new(value),
                        })
                    }
                    .boxed_local()
                }
            });
            Ok(if let Some(value) = context.available_value {
                ResourceAcquisition::Ready {
                    resource: Self {
                        value: AtomicUsize::new(value),
                    },
                    action: context.update_value.map(|_| action),
                }
            } else {
                ResourceAcquisition::RequiresAction(action)
            })
        }
    }

    enum TestCommand {
        Read,
        Add(usize),
        Wait {
            started: Rc<Cell<bool>>,
            ready: oneshot::Receiver<()>,
            value: usize,
        },
        Fail,
        Unexpected,
    }

    #[derive(Serialize, Deserialize, PartialEq)]
    #[serde(rename_all = "snake_case")]
    enum TestCommandKind {
        Read,
        Add,
        Wait,
        Fail,
        Unexpected,
    }

    #[derive(Default, Deserialize)]
    #[serde(default)]
    struct TestPermissionSettings {
        commands: Vec<CommandPermissionRule<TestCommandKind>>,
    }

    impl<const KIND: u8> ManagedResourceCommand<Counter<KIND>> for TestCommand {
        type Output = Result<usize, TestError>;
        type PermissionSettings = TestPermissionSettings;

        fn command_kind(&self) -> TestCommandKind {
            match self {
                Self::Read => TestCommandKind::Read,
                Self::Add(_) => TestCommandKind::Add,
                Self::Wait { .. } => TestCommandKind::Wait,
                Self::Fail => TestCommandKind::Fail,
                Self::Unexpected => TestCommandKind::Unexpected,
            }
        }

        fn permission(&self, settings: &Self::PermissionSettings) -> Option<ResourcePermission> {
            let command = <Self as ManagedResourceCommand<Counter<KIND>>>::command_kind(self);
            settings
                .commands
                .iter()
                .find(|rule| rule.command == command)
                .map(|rule| rule.permission)
        }

        fn permission_description(&self, resource: &Counter<KIND>) -> String {
            let operation = match self {
                Self::Read => "Read".into(),
                Self::Add(value) => format!("Add {value}"),
                Self::Wait { value, .. } => format!("Wait and add {value}"),
                Self::Fail => "Fail".into(),
                Self::Unexpected => "Unexpected command".into(),
            };
            format!(
                "{operation} (current value: {})",
                resource.value.load(SeqCst)
            )
        }

        async fn run(self, resource: Arc<Counter<KIND>>) -> Self::Output {
            match self {
                Self::Read => Ok(resource.value.load(SeqCst)),
                Self::Add(value) => Ok(resource.value.fetch_add(value, SeqCst) + value),
                Self::Wait {
                    started,
                    ready,
                    value,
                } => {
                    started.set(true);
                    ready.await.map_err(|_| TestError::Cancelled)?;
                    Ok(resource.value.fetch_add(value, SeqCst) + value)
                }
                Self::Fail => Err(TestError::Command),
                Self::Unexpected => panic!("this command must not run"),
            }
        }
    }

    #[gpui::test]
    async fn acquisition_actions_are_lazy_and_return_resources(_cx: &mut TestAppContext) {
        for available_value in [None, Some(17)] {
            let context = TestContext {
                available_value,
                update_value: Some(41),
                ..Default::default()
            };
            let acquisition = TestResource::obtain(&"first", &context)
                .await
                .expect("discovery");
            assert_eq!(context.obtain_count.get(), 0);
            let action = match acquisition {
                ResourceAcquisition::Ready { resource, action } => {
                    assert_eq!(TestCommand::Read.run(Arc::new(resource)).await, Ok(17));
                    action.expect("optional update")
                }
                ResourceAcquisition::RequiresAction(action) => action,
            };
            let resource = action().await.expect("acquisition action");
            assert_eq!(TestCommand::Read.run(Arc::new(resource)).await, Ok(41));
            assert_eq!(context.obtain_count.get(), 1);
        }
    }

    #[gpui::test]
    async fn ready_resources_are_cached_without_running_optional_actions(cx: &mut TestAppContext) {
        for update_value in [None, Some(41)] {
            let context = Rc::new(TestContext {
                available_value: Some(17),
                update_value,
                ..Default::default()
            });
            let store = new_store(cx);
            store.update(cx, |store, cx| {
                store.set_permission(TestResource::id(&"first"), ResourcePermission::Allowed, cx)
            });
            assert_eq!(
                execute::<0>(&store, "first", &context, TestCommand::Add(7), cx)
                    .await
                    .expect("available resource"),
                Ok(24)
            );
            assert_eq!(
                execute::<0>(&store, "first", &context, TestCommand::Read, cx)
                    .await
                    .expect("cached resource"),
                Ok(24)
            );
            assert_eq!(context.obtain_count.get(), 0);
        }
    }

    #[test]
    fn resource_specifiers_are_closed_and_typed() {
        use settings::ResourceSpecifier;

        for kind in ["debugger", "language_server", "npm"] {
            for value in [
                serde_json::json!(kind),
                serde_json::json!({"kind":kind,"payload":{"name":"lldb"}}),
            ] {
                let specifier: ResourceSpecifier =
                    serde_json::from_value(value.clone()).expect("valid resource specifier");
                assert_eq!(
                    serde_json::to_value(specifier).expect("serialize specifier"),
                    value
                );
            }
        }
        for value in [
            serde_json::json!("unknown"),
            serde_json::json!({"kind":"unknown","payload":{"name":"lldb"}}),
            serde_json::json!({"kind":"debugger"}),
            serde_json::json!({"kind":"debugger","payload":{}}),
            serde_json::json!({"kind":"debugger","payload":{"name":17}}),
            serde_json::json!({"kind":"debugger","payload":{"name":"lldb","typo":true}}),
        ] {
            assert!(serde_json::from_value::<ResourceSpecifier>(value).is_err());
        }
    }

    #[gpui::test]
    async fn resource_rules_match_kinds_payloads_and_order(cx: &mut TestAppContext) {
        let mut permissions: ResourcePermissions = serde_json::from_value(serde_json::json!({
            "default":"denied",
            "resources":[
                {
                    "resource":{"kind":"language_server","payload":{"name":"counter-0-first"}},
                    "obtain":"allowed",
                    "commands":[{"permission":"allowed","command":"read"}]
                },
                {"resource":"language_server","obtain":"denied"},
                {"resource":"debugger","obtain":"allowed"}
            ]
        }))
        .expect("permission rules");
        let debugger = ResourceId::Debugger {
            payload: settings::DebuggerResourceId {
                name: "counter-0-first".into(),
            },
        };
        assert_eq!(
            permissions.permission(&debugger),
            Some(ResourcePermission::Allowed)
        );
        assert_eq!(
            permissions.permission(&ResourceId::Npm {
                payload: NpmResourceId {
                    name: "counter-0-first".into()
                },
            }),
            Some(ResourcePermission::Denied)
        );
        let context = Rc::new(TestContext::default());
        let store = cx.new(|_| {
            ManagedResourceStore::new(permissions.clone(), |_, _| {
                panic!("configured permissions must not prompt")
            })
        });
        assert_eq!(
            execute::<0>(&store, "first", &context, TestCommand::Read, cx)
                .await
                .expect("specific resource allowed"),
            Ok(5)
        );
        assert!(matches!(
            execute::<0>(&store, "second", &context, TestCommand::Unexpected, cx).await,
            Err(ResourceAccessError::Denied)
        ));

        permissions.content.resources.swap(0, 1);
        let store = cx.new(|_| {
            ManagedResourceStore::new(permissions, |_, _| {
                panic!("configured permissions must not prompt")
            })
        });
        assert!(matches!(
            execute::<0>(&store, "first", &context, TestCommand::Unexpected, cx).await,
            Err(ResourceAccessError::Denied)
        ));
        store.update(cx, |store, cx| {
            store.set_permission(TestResource::id(&"first"), ResourcePermission::Allowed, cx);
        });
        assert_eq!(
            execute::<0>(&store, "first", &context, TestCommand::Read, cx)
                .await
                .expect("explicit update overrides wildcard and preserves commands"),
            Ok(5)
        );
        store.update(cx, |store, cx| {
            store.set_permission(TestResource::id(&"third"), ResourcePermission::Allowed, cx);
            assert_eq!(
                store.permission(&TestResource::id(&"third")),
                Some(ResourcePermission::Allowed)
            );
            store.clear_permission(&TestResource::id(&"third"), cx);
        });
        assert_eq!(
            store.read_with(cx, |store, _| store.permission(&TestResource::id(&"third"))),
            Some(ResourcePermission::Denied)
        );
        assert_eq!(
            store.read_with(cx, |store, _| store
                .permission(&TestResource::id(&"second"))),
            Some(ResourcePermission::Denied)
        );
        assert_eq!(
            store.read_with(cx, |store, _| store.permission(&debugger)),
            Some(ResourcePermission::Allowed)
        );
    }

    #[gpui::test]
    async fn settings_defaults_and_action_overrides(cx: &mut TestAppContext) {
        let context = Rc::new(TestContext::default());
        for (settings, allowed, acquired) in [
            (r#"{}"#, true, true),
            (
                r#"{"experimental.permissions":{"default":"denied","resources":[
                    {"resource":"language_server","obtain":"allowed","commands":[{"permission":"allowed","command":"add"}]}
                ]}}"#,
                true,
                true,
            ),
            (
                r#"{"experimental.permissions":{"default":"allowed","resources":[
                    {"resource":"language_server","commands":[{"permission":"denied","command":"add"}]}
                ]}}"#,
                false,
                true,
            ),
            (
                r#"{"experimental.permissions":{"default":"denied"}}"#,
                false,
                false,
            ),
            (
                r#"{"experimental.permissions":{
                "default":"denied",
                "resources":[{"resource":{"kind":"language_server","payload":{"name":"counter-0-first"}},"obtain":"allowed","commands":[{"permission":"allowed","command":"add"}]}]
            }}"#,
                true,
                true,
            ),
            (
                r#"{"experimental.permissions":{
                "default":"allowed",
                "resources":[{"resource":{"kind":"language_server","payload":{"name":"counter-0-first"}},"obtain":"denied"}]
            }}"#,
                false,
                false,
            ),
            (
                r#"{"experimental.permissions":{
                "default":"allowed",
                "resources":[{"resource":{"kind":"language_server","payload":{"name":"counter-0-first"}},"commands":[{"permission":"denied","command":"add"}]}]
            }}"#,
                false,
                true,
            ),
        ] {
            let mut content = SettingsContent::parse_json_with_comments(include_str!(
                "../../../assets/settings/default.json"
            ))
            .expect("default settings");
            settings::MergeFromTrait::merge_from(
                &mut content,
                &SettingsContent::parse_json_with_comments(settings).expect("user settings"),
            );
            let store = cx.new(|_| {
                ManagedResourceStore::new(ResourcePermissions::from_settings(&content), |_, _| {
                    panic!("configured permissions must not prompt")
                })
            });
            let obtains_before = context.obtain_count.get();
            let result = execute::<0>(&store, "first", &context, TestCommand::Add(7), cx).await;
            assert_eq!(
                context.obtain_count.get() - obtains_before,
                usize::from(acquired)
            );
            if allowed {
                assert_eq!(result.expect("allowed by settings"), Ok(12));
            } else if acquired {
                assert!(matches!(result, Err(ResourceAccessError::CommandDenied)));
            } else {
                assert!(matches!(result, Err(ResourceAccessError::Denied)));
            }
        }
    }

    #[gpui::test]
    async fn command_rules_match_kinds_and_prompt_each_time(cx: &mut TestAppContext) {
        let content = SettingsContent::parse_json_with_comments(
            r#"{
            "experimental.permissions": {
                "default": "denied",
                "resources": [{
                    "resource": {"kind":"language_server","payload":{"name":"counter-0-first"}},
                    "obtain": "allowed",
                    "commands": [
                        {"permission":"prompt","command":"add"},
                        {"permission":"allowed","command":"add"},
                        {"permission":"allowed","command":"read"}
                    ]
                }]
            }
        }"#,
        )
        .expect("settings");
        let next_permission = Rc::new(Cell::new(ResourcePermission::Allowed));
        let prompts = Rc::new(Cell::new(0));
        let store = cx.new(|_| {
            ManagedResourceStore::new(ResourcePermissions::from_settings(&content), {
                let next_permission = next_permission.clone();
                let prompts = prompts.clone();
                move |request, _| {
                    assert!(matches!(
                        request,
                        ResourcePermissionRequest::ExecuteCommand { .. }
                    ));
                    prompts.set(prompts.get() + 1);
                    Task::ready(next_permission.get())
                }
            })
        });
        let context = Rc::new(TestContext::default());
        let permissions = store.read_with(cx, |store, _| store.permissions());
        assert_eq!(
            execute::<0>(&store, "first", &context, TestCommand::Add(7), cx)
                .await
                .expect("approved"),
            Ok(12)
        );
        next_permission.set(ResourcePermission::Denied);
        assert!(matches!(
            execute::<0>(&store, "first", &context, TestCommand::Add(7), cx).await,
            Err(ResourceAccessError::CommandDenied)
        ));
        assert!(matches!(
            execute::<0>(&store, "first", &context, TestCommand::Add(11), cx).await,
            Err(ResourceAccessError::CommandDenied)
        ));
        assert_eq!(
            execute::<0>(&store, "first", &context, TestCommand::Read, cx)
                .await
                .expect("read allowed"),
            Ok(12)
        );
        assert_eq!(prompts.get(), 3);
        assert_eq!(
            store.read_with(cx, |store, _| store.permissions()),
            permissions
        );
    }

    #[gpui::test]
    async fn acquisition_prompt_is_not_remembered(cx: &mut TestAppContext) {
        let content = SettingsContent::parse_json_with_comments(
            r#"{
            "experimental.permissions": {
                "default":"allowed",
                "resources":[{"resource":{"kind":"language_server","payload":{"name":"counter-0-first"}},"obtain":"prompt"}]
            }
        }"#,
        )
        .expect("settings");
        let prompts = Rc::new(Cell::new(0));
        let store = cx.new(|_| {
            ManagedResourceStore::new(ResourcePermissions::from_settings(&content), {
                let prompts = prompts.clone();
                move |request, _| {
                    assert!(matches!(request, ResourcePermissionRequest::Obtain(_)));
                    prompts.set(prompts.get() + 1);
                    Task::ready(ResourcePermission::Allowed)
                }
            })
        });
        let context = Rc::new(TestContext::default());
        for _ in 0..2 {
            assert_eq!(
                execute::<0>(&store, "first", &context, TestCommand::Read, cx)
                    .await
                    .expect("approved"),
                Ok(5)
            );
        }
        assert_eq!(prompts.get(), 2);
        assert_eq!(context.obtain_count.get(), 1);
        assert_eq!(
            store.read_with(cx, |store, _| store.permission(&TestResource::id(&"first"))),
            Some(ResourcePermission::Prompt)
        );
    }

    #[gpui::test]
    async fn settings_permissions_persist_and_update_live(cx: &mut TestAppContext) {
        use fs::Fs as _;

        let fs = fs::FakeFs::new(cx.background_executor.clone());
        fs.create_dir(paths::settings_file().parent().expect("settings directory"))
            .await
            .expect("create settings directory");
        let initial_settings = r#"{
            "proxy":"http://localhost:1234",
            "experimental.permissions":{
                "resources":[{"resource":{"kind":"language_server","payload":{"name":"counter-0-first"}},"commands":[{"permission":"allowed","command":"add"}]}]
            }
        }"#;
        fs.insert_file(paths::settings_file(), initial_settings.as_bytes().to_vec())
            .await;
        cx.update(|cx| {
            let mut store =
                SettingsStore::new(cx, include_str!("../../../assets/settings/default.json"));
            store
                .set_user_settings(initial_settings, cx)
                .result()
                .expect("initial settings");
            cx.set_global(store);
        });
        let store = cx.new(|cx| {
            ManagedResourceStore::from_settings(
                fs.clone(),
                |_, _| panic!("configured permissions must not prompt"),
                cx,
            )
        });
        let context = Rc::new(TestContext::default());
        assert_eq!(
            execute::<0>(&store, "first", &context, TestCommand::Add(7), cx)
                .await
                .expect("allowed by default"),
            Ok(12)
        );

        store.update(cx, |store, cx| {
            store.set_permission(TestResource::id(&"first"), ResourcePermission::Denied, cx)
        });
        cx.run_until_parked();
        let persisted = fs
            .load(paths::settings_file())
            .await
            .expect("persisted settings");
        let persisted =
            SettingsContent::parse_json_with_comments(&persisted).expect("valid settings");
        assert_eq!(persisted.proxy.as_deref(), Some("http://localhost:1234"));
        assert_eq!(
            ResourcePermissions::from_settings(&persisted).permission(&TestResource::id(&"first")),
            Some(ResourcePermission::Denied)
        );
        assert_eq!(
            ResourcePermissions::from_settings(&persisted)
                .command_permission::<TestResource>(
                    &TestResource::id(&"first"),
                    &TestCommand::Add(7)
                )
                .expect("valid settings"),
            Some(ResourcePermission::Allowed)
        );
        let restored = cx.new(|cx| {
            ManagedResourceStore::from_settings(fs.clone(), |_, _| panic!("must not prompt"), cx)
        });
        assert!(matches!(
            execute::<0>(&restored, "first", &context, TestCommand::Unexpected, cx).await,
            Err(ResourceAccessError::Denied)
        ));

        let updated = cx.update(|cx| {
            cx.global::<SettingsStore>()
                .update_settings_file_with_completion(fs.clone(), |settings, _| {
                    let permissions = settings.experimental_permissions.get_or_insert_default();
                    permissions.default = Some(ResourcePermission::Denied);
                    let resource = permissions
                        .resources
                        .iter_mut()
                        .find(|entry| entry.resource.matches(&TestResource::id(&"first")))
                        .expect("existing resource rule");
                    resource.obtain = Some(ResourcePermission::Allowed);
                    resource.settings.insert(
                        "commands".into(),
                        serde_json::json!([
                            {"permission":"denied","command":"unexpected"}
                        ]),
                    );
                })
        });
        updated
            .await
            .expect("settings update completed")
            .expect("settings written");
        assert!(matches!(
            execute::<0>(&store, "first", &context, TestCommand::Unexpected, cx).await,
            Err(ResourceAccessError::CommandDenied)
        ));
        assert!(matches!(
            execute::<0>(&store, "second", &context, TestCommand::Unexpected, cx).await,
            Err(ResourceAccessError::Denied)
        ));

        store.update(cx, |store, cx| {
            store.clear_permission(&TestResource::id(&"first"), cx)
        });
        cx.run_until_parked();
        let persisted = fs
            .load(paths::settings_file())
            .await
            .expect("persisted settings");
        let persisted =
            SettingsContent::parse_json_with_comments(&persisted).expect("valid settings");
        let resource = persisted
            .experimental_permissions
            .as_ref()
            .and_then(|permissions| permissions.resource(&TestResource::id(&"first")))
            .expect("command permissions retained");
        assert_eq!(resource.obtain, None);
        assert_eq!(
            resource.settings.get("commands"),
            Some(&serde_json::json!([{"permission":"denied","command":"unexpected"}]))
        );
        assert!(matches!(
            execute::<0>(&store, "first", &context, TestCommand::Unexpected, cx).await,
            Err(ResourceAccessError::Denied)
        ));
    }

    #[gpui::test]
    async fn permissions_gate_acquisition_and_cached_access(cx: &mut TestAppContext) {
        let approval_count = Rc::new(Cell::new(0));
        let store = cx.new(|_| {
            ManagedResourceStore::new(ResourcePermissions::default(), {
                let approval_count = approval_count.clone();
                move |request, cx| {
                    if matches!(request, ResourcePermissionRequest::Obtain(_)) {
                        approval_count.set(approval_count.get() + 1);
                    }
                    deny_obtain_allow_commands(request, cx)
                }
            })
        });
        let context = Rc::new(TestContext::default());
        let id = TestResource::id(&"first");

        for _ in 0..2 {
            let result = execute::<0>(&store, "first", &context, TestCommand::Unexpected, cx).await;
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
        assert_eq!(
            execute::<0>(&store, "first", &context, TestCommand::Add(7), cx)
                .await
                .expect("resource allowed"),
            Ok(12)
        );

        store.update(cx, |store, cx| {
            store.set_permission(id.clone(), ResourcePermission::Denied, cx)
        });
        let result = execute::<0>(&store, "first", &context, TestCommand::Unexpected, cx).await;
        assert!(matches!(result, Err(ResourceAccessError::Denied)));

        store.update(cx, |store, cx| store.clear_permission(&id, cx));
        let result = execute::<0>(&store, "first", &context, TestCommand::Unexpected, cx).await;
        assert!(matches!(result, Err(ResourceAccessError::Denied)));
        assert_eq!(approval_count.get(), 2);

        store.update(cx, |store, cx| {
            store.set_permission(id, ResourcePermission::Allowed, cx)
        });
        assert_eq!(
            execute::<0>(&store, "first", &context, TestCommand::Add(3), cx)
                .await
                .expect("resource allowed again"),
            Ok(15)
        );
        assert_eq!(context.obtain_count.get(), 1);
        assert_eq!(approval_count.get(), 2);
    }

    #[gpui::test]
    async fn permissions_round_trip_without_sharing_live_resources(cx: &mut TestAppContext) {
        let id = TestResource::id(&"first");
        assert_eq!(
            serde_json::to_value(&id).expect("serializable identity"),
            serde_json::json!({"kind":"language_server","payload":{"name":"counter-0-first"}})
        );
        assert_eq!(
            serde_json::from_value::<ResourceId>(
                serde_json::json!({"kind":"language_server","payload":{"name":"counter-0-first"}})
            )
            .expect("restorable identity"),
            id
        );
        let store = new_store(cx);
        let context = Rc::new(TestContext::default());
        store.update(cx, |store, cx| {
            store.set_permission(TestResource::id(&"first"), ResourcePermission::Allowed, cx);
            store.set_permission(TestResource::id(&"second"), ResourcePermission::Denied, cx);
            store.set_permission(OtherResource::id(&"first"), ResourcePermission::Denied, cx);
        });
        assert_eq!(
            execute::<0>(&store, "first", &context, TestCommand::Add(7), cx)
                .await
                .expect("resource allowed"),
            Ok(12)
        );

        let permissions = store.read_with(cx, |store, _| store.permissions());
        let serialized = serde_json::to_value(&permissions).expect("serializable permissions");
        assert_eq!(
            serialized,
            serde_json::json!({
                "default": null,
                "resources": [
                    {"resource":{"kind":"language_server","payload":{"name":"counter-0-first"}},"obtain":"allowed"},
                    {"resource":{"kind":"language_server","payload":{"name":"counter-0-second"}},"obtain":"denied"},
                    {"resource":{"kind":"language_server","payload":{"name":"counter-1-first"}},"obtain":"denied"},
                ],
            })
        );
        let restored = cx.new(|_| {
            ManagedResourceStore::new(
                serde_json::from_value(serialized).expect("restorable permissions"),
                deny_obtain_allow_commands,
            )
        });
        assert_eq!(
            restored.read_with(cx, |store, _| store.permissions()),
            permissions
        );
        assert_eq!(
            execute::<0>(&restored, "first", &context, TestCommand::Add(11), cx)
                .await
                .expect("restored permission"),
            Ok(16)
        );
        assert_eq!(context.obtain_count.get(), 2);

        for key in ["second", "third"] {
            let result = execute::<0>(&restored, key, &context, TestCommand::Unexpected, cx).await;
            assert!(matches!(result, Err(ResourceAccessError::Denied)));
        }
        let result = execute::<1>(&restored, "first", &context, TestCommand::Unexpected, cx).await;
        assert!(matches!(result, Err(ResourceAccessError::Denied)));
        assert_eq!(context.obtain_count.get(), 2);

        restored.update(cx, |store, cx| {
            store.set_permission(OtherResource::id(&"first"), ResourcePermission::Allowed, cx)
        });
        let value = execute::<1>(&restored, "first", &context, TestCommand::Read, cx)
            .await
            .expect("other resource allowed");
        assert_eq!(value, Ok(5));
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
        let first = execute::<0>(&store, "first", &context, TestCommand::Unexpected, cx);
        let second = execute::<0>(&store, "first", &context, TestCommand::Unexpected, cx);
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

        let value = execute::<0>(&store, "first", &context, TestCommand::Add(4), cx)
            .await
            .expect("acquisition retried");
        assert_eq!(value, Ok(9));
        let result = execute::<0>(&store, "first", &context, TestCommand::Fail, cx)
            .await
            .expect("resource cached");
        assert_eq!(result, Err(TestError::Command));
        assert_eq!(context.obtain_count.get(), 2);
    }

    #[gpui::test]
    async fn concurrent_commands_share_acquisition_without_serializing_execution(
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

        let mut first = execute::<0>(
            &store,
            "first",
            &context,
            TestCommand::Wait {
                started: command_started.clone(),
                ready: command_wait,
                value: 3,
            },
            cx,
        );
        let mut second = execute::<0>(&store, "first", &context, TestCommand::Add(7), cx);
        cx.run_until_parked();
        assert!(poll!(&mut first).is_pending());
        assert!(poll!(&mut second).is_pending());
        assert_eq!(context.obtain_count.get(), 1);
        assert!(!command_started.get());

        let different = execute::<0>(&store, "different", &context, TestCommand::Read, cx)
            .await
            .expect("other resource independent");
        assert_eq!(different, Ok(9));

        obtain_ready.send(()).expect("acquisition pending");
        cx.run_until_parked();
        assert!(command_started.get());
        assert!(poll!(&mut first).is_pending());
        let second = second
            .await
            .expect("second command does not wait for first");
        assert_eq!(second, Ok(12));
        command_ready.send(()).expect("first command pending");
        assert_eq!(first.await.expect("first command completed"), Ok(15));
        assert_eq!(context.obtain_count.get(), 2);
    }

    #[gpui::test]
    async fn revocation_blocks_new_commands_without_cancelling_running_commands(
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
        let running = execute::<0>(
            &store,
            "first",
            &context,
            TestCommand::Wait {
                started: started.clone(),
                ready: command_wait,
                value: 2,
            },
            cx,
        );
        cx.run_until_parked();
        assert!(started.get());
        let waiting = execute::<0>(&store, "first", &context, TestCommand::Unexpected, cx);
        store.update(cx, |store, cx| {
            store.set_permission(id, ResourcePermission::Denied, cx)
        });
        command_ready.send(()).expect("command pending");
        assert_eq!(running.await.expect("running command allowed"), Ok(7));
        assert!(matches!(waiting.await, Err(ResourceAccessError::Denied)));
    }

    #[gpui::test]
    async fn revocation_during_acquisition_prevents_commands(cx: &mut TestAppContext) {
        for permission in [Some(ResourcePermission::Denied), None] {
            let store = new_store(cx);
            let context = Rc::new(TestContext::default());
            let id = TestResource::id(&"first");
            store.update(cx, |store, cx| {
                store.set_permission(id.clone(), ResourcePermission::Allowed, cx)
            });
            let (obtain_ready, obtain_wait) = oneshot::channel();
            context.obtain_ready.set(Some(obtain_wait));
            let first = execute::<0>(&store, "first", &context, TestCommand::Unexpected, cx);
            let second = execute::<0>(&store, "first", &context, TestCommand::Unexpected, cx);
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
            let value = execute::<0>(&store, "first", &context, TestCommand::Read, cx)
                .await
                .expect("completed acquisition cached");
            assert_eq!(value, Ok(5));
            assert_eq!(context.obtain_count.get(), 1);
        }
    }

    #[gpui::test]
    async fn cancelled_command_does_not_cancel_shared_acquisition(cx: &mut TestAppContext) {
        let store = new_store(cx);
        let context = Rc::new(TestContext::default());
        store.update(cx, |store, cx| {
            store.set_permission(TestResource::id(&"first"), ResourcePermission::Allowed, cx)
        });
        let (obtain_ready, obtain_wait) = oneshot::channel();
        context.obtain_ready.set(Some(obtain_wait));
        let cancelled = execute::<0>(&store, "first", &context, TestCommand::Unexpected, cx);
        cx.run_until_parked();
        assert_eq!(context.obtain_count.get(), 1);
        drop(cancelled);
        cx.run_until_parked();
        obtain_ready
            .send(())
            .expect("shared acquisition still pending");
        cx.run_until_parked();
        let value = execute::<0>(&store, "first", &context, TestCommand::Read, cx)
            .await
            .expect("completed acquisition cached without waiters");
        assert_eq!(value, Ok(5));
        assert_eq!(context.obtain_count.get(), 1);
    }

    #[gpui::test]
    async fn conflicting_resource_types_cannot_reuse_an_identity(cx: &mut TestAppContext) {
        struct ConflictingResource;

        impl ManagedResource for ConflictingResource {
            type Key = ();
            type Context = ();
            type Command = Self;
            type CommandKind = TestCommandKind;

            fn id(_: &()) -> ResourceId {
                TestResource::id(&"first")
            }

            async fn obtain(_: &(), _: &()) -> anyhow::Result<ResourceAcquisition<Self>> {
                panic!("a conflicting identity must not be acquired")
            }
        }

        impl ManagedResourceCommand<Self> for ConflictingResource {
            type Output = ();
            type PermissionSettings = TestPermissionSettings;

            fn command_kind(&self) -> TestCommandKind {
                TestCommandKind::Unexpected
            }

            fn permission(&self, _: &Self::PermissionSettings) -> Option<ResourcePermission> {
                panic!("a conflicting resource must not check command permissions")
            }

            fn permission_description(&self, _: &Self) -> String {
                panic!("a conflicting resource must not request command approval")
            }

            async fn run(self, _: Arc<Self>) {
                panic!("a conflicting resource must not run commands")
            }
        }

        let store = new_store(cx);
        let context = Rc::new(TestContext::default());
        store.update(cx, |store, cx| {
            store.set_permission(TestResource::id(&"first"), ResourcePermission::Allowed, cx)
        });
        let first = execute::<0>(&store, "first", &context, TestCommand::Read, cx);
        let result = store
            .update(cx, |store, cx| {
                store.execute_command::<ConflictingResource>(
                    (),
                    Rc::new(()),
                    ResourceExecutionBill {
                        root: ExecutionBillRoot::LanguageServer,
                        reasoning: Vec::new(),
                    },
                    ConflictingResource,
                    cx,
                )
            })
            .await;
        assert!(matches!(
            result,
            Err(ResourceAccessError::ConflictingResourceType)
        ));
        assert_eq!(first.await.expect("original resource allowed"), Ok(5));
    }

    #[gpui::test]
    async fn approval_is_awaited_and_shared_before_acquisition(cx: &mut TestAppContext) {
        for permission in [ResourcePermission::Allowed, ResourcePermission::Denied] {
            let (store, approve) = pending_approval_store(cx);
            let context = Rc::new(TestContext::default());
            let mut first = execute::<0>(&store, "first", &context, TestCommand::Read, cx);
            let mut second = execute::<0>(&store, "first", &context, TestCommand::Read, cx);
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
            let different = execute::<0>(&store, "different", &context, TestCommand::Read, cx)
                .await
                .expect("other resource independent of approval");
            assert_eq!(different, Ok(9));

            approve.send(permission).expect("approval pending");
            match permission {
                ResourcePermission::Allowed => {
                    assert_eq!(first.await.expect("approval granted"), Ok(5));
                    assert_eq!(second.await.expect("approval shared"), Ok(5));
                    assert_eq!(context.obtain_count.get(), 2);
                }
                ResourcePermission::Denied => {
                    assert!(matches!(first.await, Err(ResourceAccessError::Denied)));
                    assert!(matches!(second.await, Err(ResourceAccessError::Denied)));
                    assert_eq!(context.obtain_count.get(), 1);
                }
                ResourcePermission::Prompt => panic!("approval requires a decision"),
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
                let pending = execute::<0>(&store, "first", &context, TestCommand::Read, cx);
                cx.run_until_parked();
                store.update(cx, |store, cx| match permission {
                    Some(permission) => store.set_permission(id.clone(), permission, cx),
                    None => store.clear_permission(&id, cx),
                });
                approve.send(response).expect("approval pending");
                if permission == Some(ResourcePermission::Allowed) {
                    assert_eq!(pending.await.expect("explicit approval wins"), Ok(5));
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
        let pending = execute::<0>(&store, "first", &context, TestCommand::Unexpected, cx);
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
    async fn cancelled_command_does_not_cancel_shared_approval(cx: &mut TestAppContext) {
        let (store, approve) = pending_approval_store(cx);
        let context = Rc::new(TestContext::default());
        let cancelled = execute::<0>(&store, "first", &context, TestCommand::Unexpected, cx);
        cx.run_until_parked();
        drop(cancelled);
        cx.run_until_parked();
        let pending = execute::<0>(&store, "first", &context, TestCommand::Read, cx);
        cx.run_until_parked();
        assert_eq!(context.obtain_count.get(), 0);
        approve
            .send(ResourcePermission::Allowed)
            .expect("approval still pending");
        assert_eq!(pending.await.expect("shared approval granted"), Ok(5));
        assert_eq!(context.obtain_count.get(), 1);
    }

    #[gpui::test]
    async fn dropping_store_prevents_pending_commands(cx: &mut TestAppContext) {
        for awaiting_command in [false, true] {
            let (store, approve) = if awaiting_command {
                pending_command_approval_store(cx)
            } else {
                pending_approval_store(cx)
            };
            let weak_store = store.downgrade();
            let context = Rc::new(TestContext::default());
            let pending = execute::<0>(&store, "first", &context, TestCommand::Unexpected, cx);
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
            assert_eq!(context.obtain_count.get(), usize::from(awaiting_command));
        }
    }

    #[gpui::test]
    async fn command_approval_is_awaited_after_obtain_and_not_remembered(cx: &mut TestAppContext) {
        for permission in [ResourcePermission::Allowed, ResourcePermission::Denied] {
            let context = Rc::new(TestContext::default());
            let (obtain_ready, obtain_wait) = oneshot::channel();
            context.obtain_ready.set(Some(obtain_wait));
            let (approve, response) = oneshot::channel();
            let response = Cell::new(Some(response));
            let requests = Rc::new(RefCell::new(Vec::new()));
            let next_permission = Rc::new(Cell::new(ResourcePermission::Denied));
            let store = cx.new(|_| {
                ManagedResourceStore::new(ResourcePermissions::default(), {
                    let requests = requests.clone();
                    let next_permission = next_permission.clone();
                    move |request, cx| {
                        requests.borrow_mut().push(request.clone());
                        match request {
                            ResourcePermissionRequest::Obtain(_) => {
                                Task::ready(ResourcePermission::Allowed)
                            }
                            ResourcePermissionRequest::ExecuteCommand { .. } => {
                                if let Some(response) = response.take() {
                                    cx.spawn(async move |_| {
                                        response.await.expect("approval pending")
                                    })
                                } else {
                                    Task::ready(next_permission.get())
                                }
                            }
                        }
                    }
                })
            });
            let mut pending = execute::<0>(&store, "first", &context, TestCommand::Add(7), cx);
            cx.run_until_parked();
            assert!(poll!(&mut pending).is_pending());
            assert_eq!(
                *requests.borrow(),
                [ResourcePermissionRequest::Obtain(TestResource::id(
                    &"first"
                ))]
            );
            obtain_ready.send(()).expect("acquisition pending");
            cx.run_until_parked();
            assert!(poll!(&mut pending).is_pending());
            assert_eq!(
                *requests.borrow(),
                [
                    ResourcePermissionRequest::Obtain(TestResource::id(&"first")),
                    ResourcePermissionRequest::ExecuteCommand {
                        resource: TestResource::id(&"first"),
                        description: "Add 7 (current value: 5)".into(),
                        payload: serde_json::json!("add"),
                    },
                ]
            );
            let permissions = store.read_with(cx, |store, _| store.permissions());
            approve.send(permission).expect("command approval pending");
            let expected_value = match permission {
                ResourcePermission::Allowed => {
                    assert_eq!(pending.await.expect("command approved"), Ok(12));
                    12
                }
                ResourcePermission::Denied => {
                    assert!(matches!(
                        pending.await,
                        Err(ResourceAccessError::CommandDenied)
                    ));
                    5
                }
                ResourcePermission::Prompt => panic!("approval requires a decision"),
            };

            for value in [7, 11] {
                assert!(matches!(
                    execute::<0>(&store, "first", &context, TestCommand::Add(value), cx).await,
                    Err(ResourceAccessError::CommandDenied)
                ));
            }
            next_permission.set(ResourcePermission::Allowed);
            assert_eq!(
                execute::<0>(&store, "first", &context, TestCommand::Read, cx)
                    .await
                    .expect("read approved"),
                Ok(expected_value)
            );
            assert_eq!(requests.borrow().len(), 5);
            assert_eq!(context.obtain_count.get(), 1);
            assert_eq!(
                store.read_with(cx, |store, _| store.permissions()),
                permissions
            );
        }
    }

    #[gpui::test]
    async fn concurrent_commands_have_independent_approvals(cx: &mut TestAppContext) {
        let (approve_first, first_response) = oneshot::channel();
        let (approve_second, second_response) = oneshot::channel();
        let responses = RefCell::new(VecDeque::from([first_response, second_response]));
        let store = cx.new(|_| {
            ManagedResourceStore::new(ResourcePermissions::default(), move |request, cx| {
                if matches!(request, ResourcePermissionRequest::ExecuteCommand { .. })
                    && let Some(response) = responses.borrow_mut().pop_front()
                {
                    return cx.spawn(async move |_| response.await.expect("approval pending"));
                }
                Task::ready(ResourcePermission::Allowed)
            })
        });
        let context = Rc::new(TestContext::default());
        let mut first = execute::<0>(&store, "first", &context, TestCommand::Add(7), cx);
        let mut second = execute::<0>(&store, "first", &context, TestCommand::Add(7), cx);
        cx.run_until_parked();
        assert!(poll!(&mut first).is_pending());
        assert!(poll!(&mut second).is_pending());
        approve_second
            .send(ResourcePermission::Allowed)
            .expect("independent approval pending");
        cx.run_until_parked();
        approve_first
            .send(ResourcePermission::Denied)
            .expect("other approval still pending");
        let results = [first.await, second.await];
        assert_eq!(
            results
                .iter()
                .filter(|result| matches!(result, Ok(Ok(12))))
                .count(),
            1
        );
        assert_eq!(
            results
                .iter()
                .filter(|result| matches!(result, Err(ResourceAccessError::CommandDenied)))
                .count(),
            1
        );
        assert_eq!(
            execute::<0>(&store, "first", &context, TestCommand::Read, cx)
                .await
                .expect("read approved"),
            Ok(12)
        );
        assert_eq!(context.obtain_count.get(), 1);
    }

    #[gpui::test]
    async fn revocation_during_command_approval_prevents_execution(cx: &mut TestAppContext) {
        for permission in [Some(ResourcePermission::Denied), None] {
            let (store, approve) = pending_command_approval_store(cx);
            let context = Rc::new(TestContext::default());
            let pending = execute::<0>(&store, "first", &context, TestCommand::Unexpected, cx);
            cx.run_until_parked();
            assert_eq!(context.obtain_count.get(), 1);
            store.update(cx, |store, cx| {
                let id = TestResource::id(&"first");
                match permission {
                    Some(permission) => store.set_permission(id, permission, cx),
                    None => store.clear_permission(&id, cx),
                }
            });
            approve
                .send(ResourcePermission::Allowed)
                .expect("command approval pending");
            assert!(matches!(pending.await, Err(ResourceAccessError::Denied)));
        }
    }

    #[gpui::test]
    async fn cancelled_command_drops_its_approval_but_keeps_the_resource(cx: &mut TestAppContext) {
        let (store, approve) = pending_command_approval_store(cx);
        let context = Rc::new(TestContext::default());
        let cancelled = execute::<0>(&store, "first", &context, TestCommand::Unexpected, cx);
        cx.run_until_parked();
        assert_eq!(context.obtain_count.get(), 1);
        drop(cancelled);
        cx.run_until_parked();
        assert!(approve.send(ResourcePermission::Allowed).is_err());
        assert_eq!(
            execute::<0>(&store, "first", &context, TestCommand::Read, cx)
                .await
                .expect("read approved"),
            Ok(5)
        );
        assert_eq!(context.obtain_count.get(), 1);
    }
}
