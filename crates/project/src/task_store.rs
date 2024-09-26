use gpui::{AsyncAppContext, Model, ModelContext, Task, WeakModel};
use language::Location;
use rpc::{proto, AnyProtoClient, TypedEnvelope};
use settings::WorktreeId;
use task::{TaskContext, TaskTemplate, TaskVariables};
use util::ResultExt;

use crate::{buffer_store::BufferStore, worktree_store::WorktreeStore, Inventory, TaskSourceKind};

/// TODO kb docs

pub struct TaskStore {
    state: Box<dyn TaskStoreImpl>,
    downstream_client: Option<(AnyProtoClient, u64)>,
}

struct LocalTaskStore {
    buffer_store: WeakModel<BufferStore>,
    worktree_store: Model<WorktreeStore>,
    task_inventory: Model<Inventory>,
    // TODO kb needs to track task.json changes (both local and remote) and update the task inventory accordingly
    // _subscription: Subscription,
}

struct RemoteTaskStore {
    upstream_client: AnyProtoClient,
    project_id: u64,
    buffer_store: WeakModel<BufferStore>,
    worktree_store: Model<WorktreeStore>,
}

impl TaskStore {
    pub async fn handle_task_context_for_location(
        store: Model<Self>,
        envelope: TypedEnvelope<proto::TaskContextForLocation>,
        mut cx: AsyncAppContext,
    ) -> anyhow::Result<proto::TaskContext> {
        todo!("TODO kb context calculation lies totally on the ssh client")
    }
}

trait TaskStoreImpl {
    fn task_context_for_location(
        &self,
        captured_variables: TaskVariables,
        location: Location,
        cx: &mut ModelContext<LocalTaskStore>,
    ) -> Task<Option<TaskContext>>;
}

impl TaskStoreImpl for Model<LocalTaskStore> {
    fn task_context_for_location(
        &self,
        captured_variables: TaskVariables,
        location: Location,
        cx: &mut ModelContext<LocalTaskStore>,
    ) -> Task<Option<TaskContext>> {
        todo!()
    }
}

impl TaskStoreImpl for Model<RemoteTaskStore> {
    fn task_context_for_location(
        &self,
        captured_variables: TaskVariables,
        location: Location,
        cx: &mut ModelContext<LocalTaskStore>,
    ) -> Task<Option<TaskContext>> {
        todo!()
    }
}
