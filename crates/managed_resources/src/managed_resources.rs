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
use gpui::{
    App, AsyncApp, BackgroundExecutor, Context, EventEmitter, SharedString, Task, WeakEntity,
};
use serde::{Deserialize, Serialize};
use settings_content::ResourcePermissionsContent;
pub use settings_content::{CommandPermissionRule, NpmResourceId, ResourceId, ResourcePermission};
use std::{
    any::{Any, TypeId},
    collections::HashMap,
    future::Future,
    rc::Rc,
    sync::Arc,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ExecutionBillRoot {
    Extension { id: SharedString },
    Debugger { name: SharedString },
    LanguageServer { name: SharedString },
    Formatter { name: SharedString },
    AgentServer { name: SharedString },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ResourceExecutionBill {
    pub root: ExecutionBillRoot,
    pub reasoning: Vec<SharedString>,
}

impl ResourceExecutionBill {
    pub fn because(&self, reason: impl Into<SharedString>) -> Self {
        let mut bill = self.clone();
        bill.reasoning.push(reason.into());
        bill
    }
}

pub type ResourceAction<Resource> =
    Box<dyn FnOnce() -> LocalBoxFuture<'static, anyhow::Result<Resource>>>;

/// Discovery and verification must be local. Any network checks, installation,
/// or updates belong inside the deferred action, which returns a verified resource.
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
    type CommandKind: Serialize + PartialEq;

    /// Must be stable across restarts and unique to this resource type and key.
    /// Override `is_current` when configuration changes require replacing the cached resource.
    fn id(key: &Self::Key) -> ResourceId;

    fn is_current(&self, _context: &Self::Context) -> bool {
        true
    }

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

    /// Await background work so completion, errors, and cancellation follow this command.
    fn run(
        self,
        resource: &Resource,
        executor: &BackgroundExecutor,
    ) -> impl Future<Output = Self::Output>;
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ResourcePermissionRequest {
    Manage {
        resource: ResourceId,
        bill: ResourceExecutionBill,
    },
    ExecuteCommand {
        resource: ResourceId,
        description: String,
        payload: serde_json::Value,
        bill: ResourceExecutionBill,
    },
}

impl ResourcePermissionRequest {
    pub fn description(&self) -> String {
        let (resource, operation, bill) = match self {
            Self::Manage { resource, bill } => (
                resource,
                "Let Zed manage this binary (including network checks, downloads, and updates)",
                bill,
            ),
            Self::ExecuteCommand {
                resource,
                description,
                bill,
                ..
            } => (resource, description.as_str(), bill),
        };
        let ResourceId::Npm { payload } = resource;
        let requester = match &bill.root {
            ExecutionBillRoot::Extension { id } => format!("Extension: {id}"),
            ExecutionBillRoot::Debugger { name } => format!("Debugger: {name}"),
            ExecutionBillRoot::LanguageServer { name } => format!("Language server: {name}"),
            ExecutionBillRoot::Formatter { name } => format!("Formatter: {name}"),
            ExecutionBillRoot::AgentServer { name } => format!("Agent server: {name}"),
        };
        let mut description = format!("{operation}\n\nnpm runtime: {}\n{requester}", payload.name);
        for reason in &bill.reasoning {
            description.push('\n');
            description.push_str(reason);
        }
        description
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ResourcePermissions {
    content: ResourcePermissionsContent,
}

impl From<ResourcePermissionsContent> for ResourcePermissions {
    fn from(content: ResourcePermissionsContent) -> Self {
        Self { content }
    }
}

impl ResourcePermissions {
    pub fn into_content(self) -> ResourcePermissionsContent {
        self.content
    }

    fn permission(&self, id: &ResourceId) -> Option<ResourcePermission> {
        self.content
            .resource(id)
            .and_then(|resource| resource.management)
            .or(self.content.management)
            .map(Into::into)
    }

    fn command_permission<Resource: ManagedResource>(
        &self,
        id: &ResourceId,
        command: &impl ManagedResourceCommand<Resource>,
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
        self.content.set_management_permission(id, Some(permission));
    }
}

#[derive(Clone, Debug, thiserror::Error)]
pub enum ResourceAccessError {
    #[error("permission for Zed to manage this binary was denied")]
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
}

pub struct StateUpdated(pub ResourcePermissions);

impl EventEmitter<StateUpdated> for ManagedResourceStore {}

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
        }
    }

    /// Replaces persistent state without emitting an update back to its source.
    pub fn replace_state(&mut self, permissions: ResourcePermissions, cx: &mut Context<Self>) {
        if self.permissions == permissions {
            return;
        }
        self.permissions = permissions;
        for resource in self.resources.values_mut() {
            resource.permission_revision = resource.permission_revision.wrapping_add(1);
        }
        cx.notify();
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
        cx.emit(StateUpdated(self.permissions()));
        cx.notify();
    }

    pub fn clear_permission(&mut self, id: &ResourceId, cx: &mut Context<Self>) {
        if let Some(resource) = self.resources.get_mut(id) {
            resource.permission_revision = resource.permission_revision.wrapping_add(1);
        }
        self.permissions
            .content
            .set_management_permission(id.clone(), None);
        cx.emit(StateUpdated(self.permissions()));
        cx.notify();
    }

    fn acquire_resource<Resource: ManagedResource>(
        &mut self,
        key: Rc<Resource::Key>,
        context: Rc<Resource::Context>,
        bill: ResourceExecutionBill,
        cx: &mut Context<Self>,
    ) -> ResourceTask {
        let id = Resource::id(&key);
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
            return Task::ready(Err(ResourceAccessError::ConflictingResourceType)).shared();
        }

        if let Some(pending) = &entry.pending {
            pending.clone()
        } else {
            let value = entry.value.clone();
            let pending = cx
                .spawn({
                    let id = id;
                    let bill = bill;
                    let context = context;
                    let key = key;
                    async move |this, cx| {
                        let result = async {
                            if let Some(value) = value {
                                let resource = value
                                    .downcast::<Resource>()
                                    .map_err(|_| ResourceAccessError::ConflictingResourceType)?;
                                if resource.is_current(&context) {
                                    return Ok(resource as Arc<dyn Any + Send + Sync>);
                                }
                            }
                            let acquisition = Resource::obtain(&key, &context)
                                .await
                                .map_err(|error| ResourceAccessError::Obtain(Arc::new(error)))?;
                            let resource = match acquisition {
                                // Normal use never starts an optional management attempt.
                                ResourceAcquisition::Ready { resource, .. } => resource,
                                ResourceAcquisition::RequiresAction(action) => {
                                    Self::ensure_permission(&this, &id, &bill, cx).await?;
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
        }
    }

    /// Resource approval and acquisition are shared; commands may run concurrently.
    /// Dropping the returned task cancels that command, not shared acquisition.
    /// Management permission only gates deferred actions, never existing resources.
    /// Revocation cannot cancel commands or management attempts already running.
    /// Resource implementations are responsible for coordinating mutations with use.
    pub fn execute_command<Resource: ManagedResource, Output: 'static>(
        &mut self,
        key: Resource::Key,
        context: Rc<Resource::Context>,
        bill: ResourceExecutionBill,
        command: impl ManagedResourceCommand<Resource, Output = Output>,
        cx: &mut Context<Self>,
    ) -> Task<Result<Output, ResourceAccessError>> {
        let id = Resource::id(&key);
        let key = Rc::new(key);
        let mut acquisition =
            self.acquire_resource::<Resource>(key.clone(), context.clone(), bill.clone(), cx);

        cx.spawn(async move |this, cx| {
            let resource = loop {
                let resource = acquisition
                    .await?
                    .downcast::<Resource>()
                    .map_err(|_| ResourceAccessError::ConflictingResourceType)?;
                if resource.is_current(&context) {
                    break resource;
                }
                // A shared acquisition may have used another caller's configuration.
                acquisition = this
                    .update(cx, |this, cx| {
                        this.acquire_resource::<Resource>(
                            key.clone(),
                            context.clone(),
                            bill.clone(),
                            cx,
                        )
                    })
                    .map_err(|_| ResourceAccessError::StoreDropped)?;
            };
            let payload = serde_json::to_value(command.command_kind())
                .map_err(|error| ResourceAccessError::CommandPayload(Arc::new(error)))?;
            let request_approval = this
                .read_with(cx, |this, _| this.request_approval.clone())
                .map_err(|_| ResourceAccessError::StoreDropped)?;
            let request = ResourcePermissionRequest::ExecuteCommand {
                resource: id.clone(),
                description: command.permission_description(&resource),
                payload: payload.clone(),
                bill,
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
            Ok(command.run(&resource, cx.background_executor()).await)
        })
    }

    async fn ensure_permission(
        this: &WeakEntity<Self>,
        id: &ResourceId,
        bill: &ResourceExecutionBill,
        cx: &mut AsyncApp,
    ) -> Result<(), ResourceAccessError> {
        let approval = this
            .read_with(cx, |this, _| match this.permission(id) {
                Some(ResourcePermission::Allowed) | None => Ok(None),
                Some(ResourcePermission::Denied) => Err(ResourceAccessError::Denied),
                Some(ResourcePermission::Prompt) => Ok(Some((
                    this.resources
                        .get(id)
                        .map(|resource| resource.permission_revision),
                    this.request_approval.clone(),
                ))),
            })
            .map_err(|_| ResourceAccessError::StoreDropped)??;
        if let Some((revision, request_approval)) = approval {
            let permission = cx
                .update(|cx| {
                    request_approval(
                        ResourcePermissionRequest::Manage {
                            resource: id.clone(),
                            bill: bill.clone(),
                        },
                        cx,
                    )
                })
                .await;
            this.read_with(cx, |this, _| match this.permission(id) {
                Some(ResourcePermission::Allowed) | None => Ok(()),
                Some(ResourcePermission::Prompt)
                    if permission == ResourcePermission::Allowed
                        && this
                            .resources
                            .get(id)
                            .map(|resource| resource.permission_revision)
                            == revision =>
                {
                    Ok(())
                }
                _ => Err(ResourceAccessError::Denied),
            })
            .map_err(|_| ResourceAccessError::StoreDropped)??;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::{channel::oneshot, poll};
    use gpui::{AppContext as _, Entity, TestAppContext};
    use settings::{RootUserSettings as _, SettingsContent};
    use std::{
        cell::{Cell, RefCell},
        collections::VecDeque,
        sync::atomic::{AtomicUsize, Ordering::SeqCst},
    };

    type TestResource = Counter<0>;
    type OtherResource = Counter<1>;

    impl ResourcePermissions {
        fn ask_to_manage() -> Self {
            serde_json::from_value(serde_json::json!({"management":"ask"}))
                .expect("management policy")
        }

        fn from_settings(content: &SettingsContent) -> Self {
            content
                .experimental_permissions
                .clone()
                .expect("default permissions")
                .into()
        }
    }

    fn deny_obtain_allow_commands(
        request: ResourcePermissionRequest,
        _: &mut App,
    ) -> Task<ResourcePermission> {
        Task::ready(match request {
            ResourcePermissionRequest::Manage { .. } => ResourcePermission::Denied,
            ResourcePermissionRequest::ExecuteCommand { .. } => ResourcePermission::Allowed,
        })
    }

    fn new_store(cx: &mut TestAppContext) -> Entity<ManagedResourceStore> {
        cx.new(|_| {
            ManagedResourceStore::new(
                ResourcePermissions::ask_to_manage(),
                deny_obtain_allow_commands,
            )
        })
    }

    fn test_bill() -> ResourceExecutionBill {
        ResourceExecutionBill {
            root: ExecutionBillRoot::LanguageServer {
                name: "test-server".into(),
            },
            reasoning: vec![
                "Provide completions".into(),
                "Ensure the server is installed".into(),
            ],
        }
    }

    fn execute<const KIND: u8>(
        store: &Entity<ManagedResourceStore>,
        key: &'static str,
        context: &Rc<TestContext>,
        command: TestCommand,
        cx: &mut TestAppContext,
    ) -> Task<Result<Result<usize, TestError>, ResourceAccessError>> {
        store.update(cx, |store, cx| {
            store.execute_command::<Counter<KIND>, _>(
                key,
                context.clone(),
                test_bill(),
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
            ManagedResourceStore::new(ResourcePermissions::ask_to_manage(), move |request, cx| {
                let ResourcePermissionRequest::Manage { resource, .. } = request else {
                    return Task::ready(ResourcePermission::Allowed);
                };
                assert_eq!(resource, TestResource::id(&"first"));
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
            ManagedResourceStore::new(ResourcePermissions::ask_to_manage(), move |request, cx| {
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
        configuration: usize,
    }

    #[derive(Clone, Default)]
    struct TestContext {
        configuration: usize,
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
        type CommandKind = TestCommandKind;

        fn is_current(&self, context: &Self::Context) -> bool {
            self.configuration == context.configuration
        }

        fn id(key: &Self::Key) -> ResourceId {
            ResourceId::Npm {
                payload: NpmResourceId {
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
                            configuration: context.configuration,
                        })
                    }
                    .boxed_local()
                }
            });
            Ok(if let Some(value) = context.available_value {
                ResourceAcquisition::Ready {
                    resource: Self {
                        value: AtomicUsize::new(value),
                        configuration: context.configuration,
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

    #[gpui::test]
    async fn configuration_changes_do_not_wait_for_commands_and_preserve_resource_on_failure(
        cx: &mut TestAppContext,
    ) {
        let allow_management = Rc::new(Cell::new(true));
        let store = cx.new(|_| {
            let allow_management = allow_management.clone();
            ManagedResourceStore::new(ResourcePermissions::ask_to_manage(), move |request, _| {
                Task::ready(
                    if matches!(request, ResourcePermissionRequest::Manage { .. })
                        && !allow_management.get()
                    {
                        ResourcePermission::Denied
                    } else {
                        ResourcePermission::Allowed
                    },
                )
            })
        });
        let original = Rc::new(TestContext {
            available_value: Some(7),
            ..Default::default()
        });
        let (finish, ready) = oneshot::channel();
        let started = Rc::new(Cell::new(false));
        let running = execute::<0>(
            &store,
            "first",
            &original,
            TestCommand::Wait {
                started: started.clone(),
                ready,
                value: 2,
            },
            cx,
        );
        cx.run_until_parked();
        assert!(started.get());

        let replacement = Rc::new(TestContext {
            configuration: 1,
            update_value: Some(21),
            ..Default::default()
        });
        let refreshing = execute::<0>(&store, "first", &replacement, TestCommand::Read, cx);
        cx.run_until_parked();
        assert_eq!(refreshing.await.expect("permission"), Ok(21));
        assert_eq!(replacement.obtain_count.get(), 1);
        finish.send(()).expect("command still running");
        assert_eq!(running.await.expect("permission"), Ok(9));

        let broken = Rc::new(TestContext {
            configuration: 2,
            fail_next_obtain: Rc::new(Cell::new(true)),
            ..Default::default()
        });
        assert!(matches!(
            execute::<0>(&store, "first", &broken, TestCommand::Read, cx).await,
            Err(ResourceAccessError::Obtain(_))
        ));
        allow_management.set(false);
        assert!(
            execute::<0>(&store, "first", &broken, TestCommand::Read, cx)
                .await
                .is_err()
        );
        assert_eq!(
            broken.obtain_count.get(),
            1,
            "denied refresh must not run its action"
        );
        assert_eq!(
            execute::<0>(&store, "first", &replacement, TestCommand::Read, cx)
                .await
                .expect("cached resource"),
            Ok(21)
        );
        assert_eq!(replacement.obtain_count.get(), 1);
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

        async fn run(
            self,
            resource: &Counter<KIND>,
            executor: &BackgroundExecutor,
        ) -> Self::Output {
            match self {
                Self::Read => Ok(resource.value.load(SeqCst)),
                Self::Add(value) => Ok(resource.value.fetch_add(value, SeqCst) + value),
                Self::Wait {
                    started,
                    ready,
                    value,
                } => {
                    started.set(true);
                    executor
                        .spawn(ready)
                        .await
                        .map_err(|_| TestError::Cancelled)?;
                    Ok(resource.value.fetch_add(value, SeqCst) + value)
                }
                Self::Fail => Err(TestError::Command),
                Self::Unexpected => panic!("this command must not run"),
            }
        }
    }

    #[gpui::test]
    async fn acquisition_actions_are_lazy_and_return_resources(cx: &mut TestAppContext) {
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
                    assert_eq!(
                        TestCommand::Read
                            .run(&resource, &cx.background_executor)
                            .await,
                        Ok(17)
                    );
                    action.expect("optional update")
                }
                ResourceAcquisition::RequiresAction(action) => action,
            };
            let resource = action().await.expect("acquisition action");
            assert_eq!(
                TestCommand::Read
                    .run(&resource, &cx.background_executor)
                    .await,
                Ok(41)
            );
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
                store.set_permission(TestResource::id(&"first"), ResourcePermission::Denied, cx)
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

        for value in [
            serde_json::json!("npm"),
            serde_json::json!({"kind":"npm","payload":{"name":"test-runtime"}}),
        ] {
            let specifier: ResourceSpecifier =
                serde_json::from_value(value.clone()).expect("valid resource specifier");
            assert_eq!(
                serde_json::to_value(specifier).expect("serialize specifier"),
                value
            );
        }
        for value in [
            serde_json::json!("unknown"),
            serde_json::json!({"kind":"unknown","payload":{"name":"lldb"}}),
            serde_json::json!("debugger"),
            serde_json::json!("language_server"),
            serde_json::json!({"kind":"debugger","payload":{"name":"lldb"}}),
            serde_json::json!({"kind":"language_server","payload":{"name":"rust-analyzer"}}),
            serde_json::json!({"kind":"npm"}),
            serde_json::json!({"kind":"npm","payload":{}}),
            serde_json::json!({"kind":"npm","payload":{"name":17}}),
            serde_json::json!({"kind":"npm","payload":{"name":"test-runtime","typo":true}}),
        ] {
            assert!(serde_json::from_value::<ResourceSpecifier>(value).is_err());
        }
    }

    #[gpui::test]
    async fn resource_rules_match_kinds_payloads_and_order(cx: &mut TestAppContext) {
        let mut permissions: ResourcePermissions = serde_json::from_value(serde_json::json!({
            "default":"denied",
            "management":"never",
            "resources":[
                {
                    "resource":{"kind":"npm","payload":{"name":"counter-0-first"}},
                    "management":"automatic",
                    "commands":[{"permission":"allowed","command":"read"}]
                },
                {"resource":"npm","management":"never"}
            ]
        }))
        .expect("permission rules");
        assert_eq!(
            permissions.permission(&ResourceId::Npm {
                payload: NpmResourceId {
                    name: "other-runtime".into()
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
    }

    #[gpui::test]
    async fn settings_defaults_and_action_overrides(cx: &mut TestAppContext) {
        let context = Rc::new(TestContext::default());
        for (settings, allowed, acquired) in [
            (r#"{}"#, true, true),
            (
                r#"{"experimental.permissions":{"default":"denied","resources":[
                    {"resource":"npm","management":"automatic","commands":[{"permission":"allowed","command":"add"}]}
                ]}}"#,
                true,
                true,
            ),
            (
                r#"{"experimental.permissions":{"default":"allowed","resources":[
                    {"resource":"npm","commands":[{"permission":"denied","command":"add"}]}
                ]}}"#,
                false,
                true,
            ),
            (
                r#"{"experimental.permissions":{"management":"never"}}"#,
                false,
                false,
            ),
            (
                r#"{"experimental.permissions":{
                "default":"denied",
                "resources":[{"resource":{"kind":"npm","payload":{"name":"counter-0-first"}},"management":"automatic","commands":[{"permission":"allowed","command":"add"}]}]
            }}"#,
                true,
                true,
            ),
            (
                r#"{"experimental.permissions":{
                "default":"allowed",
                "resources":[{"resource":{"kind":"npm","payload":{"name":"counter-0-first"}},"management":"never"}]
            }}"#,
                false,
                false,
            ),
            (
                r#"{"experimental.permissions":{
                "default":"allowed",
                "resources":[{"resource":{"kind":"npm","payload":{"name":"counter-0-first"}},"commands":[{"permission":"denied","command":"add"}]}]
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
                    "resource": {"kind":"npm","payload":{"name":"counter-0-first"}},
                    "management": "automatic",
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
                "resources":[{"resource":{"kind":"npm","payload":{"name":"counter-0-first"}},"management":"ask"}]
            }
        }"#,
        )
        .expect("settings");
        let prompts = Rc::new(Cell::new(0));
        let store = cx.new(|_| {
            ManagedResourceStore::new(ResourcePermissions::from_settings(&content), {
                let prompts = prompts.clone();
                move |request, _| {
                    assert!(matches!(request, ResourcePermissionRequest::Manage { .. }));
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
        assert_eq!(prompts.get(), 1);
        assert_eq!(context.obtain_count.get(), 1);
        assert_eq!(
            store.read_with(cx, |store, _| store.permission(&TestResource::id(&"first"))),
            Some(ResourcePermission::Prompt)
        );
    }

    #[gpui::test]
    async fn state_snapshots_round_trip_without_echoing(cx: &mut TestAppContext) {
        let initial: ResourcePermissions = serde_json::from_value(serde_json::json!({
            "default":"allowed",
            "resources":[{"resource":{"kind":"npm","payload":{"name":"counter-0-first"}},"commands":[{"permission":"allowed","command":"add"}]}]
        })).expect("initial state");
        let store =
            cx.new(|_| ManagedResourceStore::new(initial, |_, _| panic!("must not prompt")));
        let snapshots = Rc::new(RefCell::new(Vec::new()));
        let _subscription = cx.update(|cx| {
            cx.subscribe(&store, {
                let snapshots = snapshots.clone();
                move |_, event: &StateUpdated, _| snapshots.borrow_mut().push(event.0.clone())
            })
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
        assert_eq!(snapshots.borrow().len(), 1);
        let persisted: ResourcePermissions = serde_json::from_str(
            &serde_json::to_string(snapshots.borrow().last().expect("snapshot"))
                .expect("serialize state"),
        )
        .expect("deserialize state");
        assert_eq!(
            persisted.permission(&TestResource::id(&"first")),
            Some(ResourcePermission::Denied)
        );
        assert_eq!(
            persisted
                .command_permission::<TestResource>(
                    &TestResource::id(&"first"),
                    &TestCommand::Add(7)
                )
                .expect("valid settings"),
            Some(ResourcePermission::Allowed)
        );
        let restored = cx.new(|_| {
            ManagedResourceStore::new(persisted.clone(), |_, _| panic!("must not prompt"))
        });
        assert!(matches!(
            execute::<0>(&restored, "first", &context, TestCommand::Unexpected, cx).await,
            Err(ResourceAccessError::Denied)
        ));

        store.update(cx, |store, cx| {
            let mut updated = persisted;
            updated.content.default = Some(ResourcePermission::Denied);
            let resource = updated
                .content
                .resources
                .first_mut()
                .expect("resource rule");
            resource.management = Some(ResourcePermission::Allowed.into());
            resource.settings.insert(
                "commands".into(),
                serde_json::json!([
                    {"permission":"denied","command":"unexpected"}
                ]),
            );
            store.replace_state(updated, cx);
        });
        cx.run_until_parked();
        assert_eq!(snapshots.borrow().len(), 1, "incoming state must not echo");
        assert!(matches!(
            execute::<0>(&store, "first", &context, TestCommand::Unexpected, cx).await,
            Err(ResourceAccessError::CommandDenied)
        ));
        assert!(matches!(
            execute::<0>(&store, "second", &context, TestCommand::Unexpected, cx).await,
            Err(ResourceAccessError::CommandDenied)
        ));

        store.update(cx, |store, cx| {
            store.clear_permission(&TestResource::id(&"first"), cx)
        });
        cx.run_until_parked();
        assert_eq!(snapshots.borrow().len(), 2);
        let snapshots = snapshots.borrow();
        let persisted = snapshots.last().expect("clear snapshot");
        let resource = persisted
            .content
            .resource(&TestResource::id(&"first"))
            .expect("command permissions retained");
        assert_eq!(resource.management, None);
        assert_eq!(
            resource.settings.get("commands"),
            Some(&serde_json::json!([{"permission":"denied","command":"unexpected"}]))
        );
        assert!(matches!(
            execute::<0>(&store, "first", &context, TestCommand::Unexpected, cx).await,
            Err(ResourceAccessError::CommandDenied)
        ));
    }

    #[gpui::test]
    async fn management_permissions_do_not_gate_cached_access(cx: &mut TestAppContext) {
        let approval_count = Rc::new(Cell::new(0));
        let store = cx.new(|_| {
            ManagedResourceStore::new(ResourcePermissions::ask_to_manage(), {
                let approval_count = approval_count.clone();
                move |request, cx| {
                    if matches!(request, ResourcePermissionRequest::Manage { .. }) {
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
            Some(ResourcePermission::Prompt)
        );
        assert_eq!(approval_count.get(), 2);
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
        let result = execute::<0>(&store, "first", &context, TestCommand::Read, cx).await;
        assert_eq!(result.expect("existing resource remains usable"), Ok(12));

        store.update(cx, |store, cx| store.clear_permission(&id, cx));
        let result = execute::<0>(&store, "first", &context, TestCommand::Read, cx).await;
        assert_eq!(
            result.expect("cached use needs no management approval"),
            Ok(12)
        );
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
            serde_json::json!({"kind":"npm","payload":{"name":"counter-0-first"}})
        );
        assert_eq!(
            serde_json::from_value::<ResourceId>(
                serde_json::json!({"kind":"npm","payload":{"name":"counter-0-first"}})
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
                "management": "ask",
                "resources": [
                    {"resource":{"kind":"npm","payload":{"name":"counter-0-first"}},"management":"automatic"},
                    {"resource":{"kind":"npm","payload":{"name":"counter-0-second"}},"management":"never"},
                    {"resource":{"kind":"npm","payload":{"name":"counter-1-first"}},"management":"never"},
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
    async fn concurrent_refreshes_share_acquisition_and_respect_configuration(
        cx: &mut TestAppContext,
    ) {
        let store = new_store(cx);
        store.update(cx, |store, cx| {
            store.set_permission(TestResource::id(&"first"), ResourcePermission::Allowed, cx);
        });
        let original = Rc::new(TestContext::default());
        assert_eq!(
            execute::<0>(&store, "first", &original, TestCommand::Read, cx)
                .await
                .expect("original"),
            Ok(5)
        );
        let (ready, wait) = oneshot::channel();
        let replacement = Rc::new(TestContext {
            configuration: 1,
            update_value: Some(23),
            obtain_ready: Rc::new(Cell::new(Some(wait))),
            ..Default::default()
        });
        let first = execute::<0>(&store, "first", &replacement, TestCommand::Read, cx);
        let second = execute::<0>(&store, "first", &replacement, TestCommand::Read, cx);
        cx.run_until_parked();
        assert_eq!(replacement.obtain_count.get(), 1);
        let different = Rc::new(TestContext {
            configuration: 2,
            update_value: Some(41),
            ..Default::default()
        });
        let third = execute::<0>(&store, "first", &different, TestCommand::Read, cx);
        ready.send(()).expect("refresh waiting");
        assert_eq!(first.await.expect("first refresh"), Ok(23));
        assert_eq!(second.await.expect("shared refresh"), Ok(23));
        assert_eq!(third.await.expect("different configuration"), Ok(41));
        assert_eq!(replacement.obtain_count.get(), 1);
        assert_eq!(different.obtain_count.get(), 1);
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
            let mut state = store.permissions();
            state.content.default = Some(ResourcePermission::Denied);
            store.replace_state(state, cx);
        });
        command_ready.send(()).expect("command pending");
        assert_eq!(running.await.expect("running command allowed"), Ok(7));
        assert!(matches!(
            waiting.await,
            Err(ResourceAccessError::CommandDenied)
        ));
    }

    #[gpui::test]
    async fn management_revocation_does_not_cancel_started_attempts(cx: &mut TestAppContext) {
        for permission in [Some(ResourcePermission::Denied), None] {
            let store = new_store(cx);
            let context = Rc::new(TestContext::default());
            let id = TestResource::id(&"first");
            store.update(cx, |store, cx| {
                store.set_permission(id.clone(), ResourcePermission::Allowed, cx)
            });
            let (obtain_ready, obtain_wait) = oneshot::channel();
            context.obtain_ready.set(Some(obtain_wait));
            let first = execute::<0>(&store, "first", &context, TestCommand::Read, cx);
            let second = execute::<0>(&store, "first", &context, TestCommand::Read, cx);
            cx.run_until_parked();
            assert_eq!(context.obtain_count.get(), 1);
            store.update(cx, |store, cx| match permission {
                Some(permission) => store.set_permission(id.clone(), permission, cx),
                None => store.clear_permission(&id, cx),
            });
            obtain_ready.send(()).expect("acquisition pending");
            assert_eq!(first.await.expect("started management can finish"), Ok(5));
            assert_eq!(
                second
                    .await
                    .expect("management policy does not gate commands"),
                Ok(5)
            );

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

            async fn run(self, _: &Self, _: &BackgroundExecutor) {
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
                store.execute_command::<ConflictingResource, _>(
                    (),
                    Rc::new(()),
                    test_bill(),
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
                Some(ResourcePermission::Prompt)
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
                Some(ResourcePermission::Prompt)
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
                    permission.or(Some(ResourcePermission::Prompt))
                );
            }
        }
    }

    #[gpui::test]
    async fn management_approval_does_not_change_settings(cx: &mut TestAppContext) {
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
        let pending = execute::<0>(&store, "first", &context, TestCommand::Read, cx);
        cx.run_until_parked();
        approve
            .send(ResourcePermission::Allowed)
            .expect("approval pending");
        assert_eq!(pending.await.expect("management approved once"), Ok(5));
        assert_eq!(context.obtain_count.get(), 1);
        assert_eq!(observed.get(), None);

        store.update(cx, |store, cx| {
            store.clear_permission(&TestResource::id(&"first"), cx)
        });
        assert_eq!(observed.get(), Some(ResourcePermission::Prompt));
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
                ManagedResourceStore::new(ResourcePermissions::ask_to_manage(), {
                    let requests = requests.clone();
                    let next_permission = next_permission.clone();
                    move |request, cx| {
                        requests.borrow_mut().push(request.clone());
                        match request {
                            ResourcePermissionRequest::Manage { .. } => {
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
                [ResourcePermissionRequest::Manage {
                    resource: TestResource::id(&"first"),
                    bill: test_bill(),
                }]
            );
            obtain_ready.send(()).expect("acquisition pending");
            cx.run_until_parked();
            assert!(poll!(&mut pending).is_pending());
            assert_eq!(
                *requests.borrow(),
                [
                    ResourcePermissionRequest::Manage {
                        resource: TestResource::id(&"first"),
                        bill: test_bill(),
                    },
                    ResourcePermissionRequest::ExecuteCommand {
                        resource: TestResource::id(&"first"),
                        description: "Add 7 (current value: 5)".into(),
                        payload: serde_json::json!("add"),
                        bill: test_bill(),
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
    async fn execution_bills_follow_requests_not_shared_resources(cx: &mut TestAppContext) {
        let base = ResourceExecutionBill {
            root: ExecutionBillRoot::LanguageServer {
                name: "example-server".into(),
            },
            reasoning: vec!["Provide completions".into()],
        };
        let first_bill = base.because("Read the installed server version");
        let second_bill = base.because("Install a newer server version");
        assert_eq!(
            base.reasoning,
            vec![SharedString::from("Provide completions")]
        );
        assert_eq!(
            first_bill.reasoning,
            vec![
                SharedString::from("Provide completions"),
                SharedString::from("Read the installed server version"),
            ]
        );
        assert_eq!(
            second_bill.reasoning,
            vec![
                SharedString::from("Provide completions"),
                SharedString::from("Install a newer server version"),
            ]
        );

        let requests = Rc::new(RefCell::new(Vec::new()));
        let store = cx.new(|_| {
            ManagedResourceStore::new(ResourcePermissions::ask_to_manage(), {
                let requests = requests.clone();
                move |request, _| {
                    requests.borrow_mut().push(request);
                    Task::ready(ResourcePermission::Allowed)
                }
            })
        });
        let context = Rc::new(TestContext::default());
        let (obtain_ready, obtain_wait) = oneshot::channel();
        context.obtain_ready.set(Some(obtain_wait));
        let first = store.update(cx, |store, cx| {
            store.execute_command::<TestResource, _>(
                "first",
                context.clone(),
                first_bill.clone(),
                TestCommand::Read,
                cx,
            )
        });
        cx.run_until_parked();
        let second = store.update(cx, |store, cx| {
            store.execute_command::<TestResource, _>(
                "first",
                context.clone(),
                second_bill.clone(),
                TestCommand::Add(7),
                cx,
            )
        });
        cx.run_until_parked();
        obtain_ready.send(()).expect("shared acquisition pending");
        assert!(matches!(first.await.expect("read approved"), Ok(5 | 12)));
        assert_eq!(second.await.expect("add approved"), Ok(12));
        assert_eq!(requests.borrow().len(), 3);
        assert_eq!(
            requests.borrow().first(),
            Some(&ResourcePermissionRequest::Manage {
                resource: TestResource::id(&"first"),
                bill: first_bill.clone(),
            })
        );
        for (command, expected_bill) in [("read", &first_bill), ("add", &second_bill)] {
            assert!(requests.borrow().iter().any(|request| matches!(
                request,
                ResourcePermissionRequest::ExecuteCommand { payload, bill, .. }
                    if payload == command && bill == expected_bill
            )));
        }

        let cached_bill = ResourceExecutionBill {
            root: ExecutionBillRoot::Extension {
                id: "another-extension".into(),
            },
            reasoning: vec!["Inspect the existing resource".into()],
        };
        let result = store.update(cx, |store, cx| {
            store.execute_command::<TestResource, _>(
                "first",
                context.clone(),
                cached_bill.clone(),
                TestCommand::Read,
                cx,
            )
        });
        assert_eq!(result.await.expect("cached access approved"), Ok(12));
        assert_eq!(context.obtain_count.get(), 1);
        assert_eq!(requests.borrow().len(), 4);
        assert!(matches!(
            requests.borrow().last(),
            Some(ResourcePermissionRequest::ExecuteCommand { bill, .. }) if bill == &cached_bill
        ));
    }

    #[gpui::test]
    async fn concurrent_commands_have_independent_approvals(cx: &mut TestAppContext) {
        let (approve_first, first_response) = oneshot::channel();
        let (approve_second, second_response) = oneshot::channel();
        let responses = RefCell::new(VecDeque::from([first_response, second_response]));
        let store = cx.new(|_| {
            ManagedResourceStore::new(ResourcePermissions::ask_to_manage(), move |request, cx| {
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
        for explicit_rule in [true, false] {
            let (store, approve) = pending_command_approval_store(cx);
            let context = Rc::new(TestContext::default());
            let pending = execute::<0>(&store, "first", &context, TestCommand::Unexpected, cx);
            cx.run_until_parked();
            assert_eq!(context.obtain_count.get(), 1);
            store.update(cx, |store, cx| {
                let state = if explicit_rule {
                    serde_json::json!({"resources":[{"resource":"npm","commands":[{"command":"unexpected","permission":"denied"}]}]})
                } else {
                    serde_json::json!({"default":"denied"})
                };
                store.replace_state(serde_json::from_value(state).expect("permissions"), cx);
            });
            approve
                .send(ResourcePermission::Allowed)
                .expect("command approval pending");
            assert!(matches!(
                pending.await,
                Err(ResourceAccessError::CommandDenied)
            ));
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
