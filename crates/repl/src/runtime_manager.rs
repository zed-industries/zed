use collections::HashMap;
use editor::Editor;
#[allow(unused)]
use gpui::{AppContext, EntityId, Global, Model, ModelContext, Task, View, WeakView};
use project::Fs;
#[allow(unused)]
use runtimelib::JupyterMessageContent;

#[allow(unused)]
use std::{
    sync::Arc,
    time::{Duration, Instant},
};

#[allow(unused)]
use crate::{
    outputs::ExecutionView,
    runtimes::{get_runtime_specifications, Request, RunningKernel, RuntimeSpecification},
    EditorRuntimeState, Kernel,
};

// Per workspace
pub struct RuntimeManager {
    pub fs: Arc<dyn Fs>,
    pub runtime_specifications: Vec<RuntimeSpecification>,

    instances: HashMap<EntityId, Kernel>,
    pub editors: HashMap<WeakView<Editor>, EditorRuntimeState>,
    // todo!(): Next
    // To reduce the number of open tasks and channels we have, let's feed the response
    // messages by ID over to the paired ExecutionView
    _execution_views_by_id: HashMap<String, View<ExecutionView>>,
}

#[derive(Clone)]
pub struct RuntimeManagerGlobal(Model<RuntimeManager>);

impl Global for RuntimeManagerGlobal {}

impl RuntimeManager {
    pub fn new(fs: Arc<dyn Fs>, _cx: &mut AppContext) -> Self {
        Self {
            fs,
            runtime_specifications: Default::default(),
            instances: Default::default(),
            editors: Default::default(),
            _execution_views_by_id: Default::default(),
        }
    }

    pub fn load(&mut self, cx: &mut ModelContext<Self>) {
        let task = get_runtime_specifications(self.fs.clone());

        cx.spawn(|this, mut cx| async move {
            let runtime_specs = task.await?;
            this.update(&mut cx, |this, _cx| {
                this.runtime_specifications = runtime_specs;
            })
        })
        .detach_and_log_err(cx);
    }

    // fn get_or_launch_kernel(
    //     &mut self,
    //     entity_id: EntityId,
    //     language_name: Arc<str>,
    //     cx: &mut ModelContext<Self>,
    // ) -> Task<Result<UnboundedSender<Request>>> {
    //     let kernel = self.instances.get(&entity_id);
    //     let pending_kernel_start = match kernel {
    //         Some(Kernel::RunningKernel(running_kernel)) => {
    //             return Task::ready(anyhow::Ok(running_kernel.request_tx.clone()));
    //         }
    //         Some(Kernel::StartingKernel(task)) => task.clone(),
    //         Some(Kernel::FailedLaunch) | None => {
    //             self.instances.remove(&entity_id);

    //             let kernel = self.launch_kernel(entity_id, language_name, cx);
    //             let pending_kernel = cx
    //                 .spawn(|this, mut cx| async move {
    //                     let running_kernel = kernel.await;

    //                     match running_kernel {
    //                         Ok(running_kernel) => {
    //                             let _ = this.update(&mut cx, |this, _cx| {
    //                                 this.instances
    //                                     .insert(entity_id, Kernel::RunningKernel(running_kernel));
    //                             });
    //                         }
    //                         Err(_err) => {
    //                             let _ = this.update(&mut cx, |this, _cx| {
    //                                 this.instances.insert(entity_id, Kernel::FailedLaunch);
    //                             });
    //                         }
    //                     }
    //                 })
    //                 .shared();

    //             self.instances
    //                 .insert(entity_id, Kernel::StartingKernel(pending_kernel.clone()));

    //             pending_kernel
    //         }
    //     };

    //     cx.spawn(|this, mut cx| async move {
    //         pending_kernel_start.await;

    //         this.update(&mut cx, |this, _cx| {
    //             let kernel = this
    //                 .instances
    //                 .get(&entity_id)
    //                 .ok_or(anyhow!("unable to get a running kernel"))?;

    //             match kernel {
    //                 Kernel::RunningKernel(running_kernel) => Ok(running_kernel.request_tx.clone()),
    //                 _ => Err(anyhow!("unable to get a running kernel")),
    //             }
    //         })?
    //     })
    // }

    pub fn kernelspec(&self, language_name: Arc<str>) -> Option<RuntimeSpecification> {
        self.runtime_specifications
            .iter()
            .find(|runtime_specification| {
                runtime_specification.kernelspec.language == language_name.to_string()
            })
            .cloned()
    }

    // fn launch_kernel(
    //     &mut self,
    //     entity_id: EntityId,
    //     language_name: Arc<str>,
    //     cx: &mut ModelContext<Self>,
    // ) -> Task<Result<RunningKernel>> {
    //     let runtime_specification = match self.kernelspec(language_name.clone()) {
    //         Some(runtime_specification) => runtime_specification,
    //         None => {
    //             return Task::ready(Err(anyhow::anyhow!(
    //                 "No runtime found for language {}",
    //                 language_name
    //             )));
    //         }
    //     };

    //     let runtime_specification = runtime_specification.clone();

    //     let fs = self.fs.clone();

    //     cx.spawn(|_, cx| async move {
    //         let running_kernel =
    //             RunningKernel::new(runtime_specification, entity_id, fs.clone(), cx);

    //         let running_kernel = running_kernel.await?;

    //         let mut request_tx = running_kernel.request_tx.clone();

    //         let overall_timeout_duration = Duration::from_secs(10);

    //         let start_time = Instant::now();

    //         loop {
    //             if start_time.elapsed() > overall_timeout_duration {
    //                 // todo!(): Kill the kernel
    //                 return Err(anyhow::anyhow!("Kernel did not respond in time"));
    //             }

    //             let (tx, rx) = mpsc::unbounded();
    //             match request_tx
    //                 .send(Request {
    //                     request: runtimelib::KernelInfoRequest {}.into(),
    //                     responses_rx: tx,
    //                 })
    //                 .await
    //             {
    //                 Ok(_) => {}
    //                 Err(_err) => {
    //                     break;
    //                 }
    //             };

    //             let mut rx = rx.fuse();

    //             let kernel_info_timeout = Duration::from_secs(1);

    //             let mut got_kernel_info = false;
    //             while let Ok(Some(message)) = timeout(kernel_info_timeout, rx.next()).await {
    //                 match message {
    //                     JupyterMessageContent::KernelInfoReply(_) => {
    //                         got_kernel_info = true;
    //                     }
    //                     _ => {}
    //                 }
    //             }

    //             if got_kernel_info {
    //                 break;
    //             }
    //         }

    //         anyhow::Ok(running_kernel)
    //     })
    // }

    // pub fn execute_code(
    //     &mut self,
    //     entity_id: EntityId,
    //     language_name: Arc<str>,
    //     code: String,
    //     cx: &mut ModelContext<Self>,
    // ) -> impl Future<Output = Result<mpsc::UnboundedReceiver<JupyterMessageContent>>> {
    //     let (tx, rx) = mpsc::unbounded();

    //     let request_tx = self.get_or_launch_kernel(entity_id, language_name, cx);

    //     async move {
    //         let request_tx = request_tx.await?;

    //         request_tx
    //             .unbounded_send(Request {
    //                 request: runtimelib::ExecuteRequest {
    //                     code,
    //                     allow_stdin: false,
    //                     silent: false,
    //                     store_history: true,
    //                     stop_on_error: true,
    //                     ..Default::default()
    //                 }
    //                 .into(),
    //                 responses_rx: tx,
    //             })
    //             .context("Failed to send execution request")?;

    //         Ok(rx)
    //     }
    // }

    pub fn global(cx: &AppContext) -> Option<Model<Self>> {
        cx.try_global::<RuntimeManagerGlobal>()
            .map(|runtime_manager| runtime_manager.0.clone())
    }

    pub fn set_global(runtime_manager: Model<Self>, cx: &mut AppContext) {
        cx.set_global(RuntimeManagerGlobal(runtime_manager));
    }

    pub fn remove_global(cx: &mut AppContext) {
        if RuntimeManager::global(cx).is_some() {
            cx.remove_global::<RuntimeManagerGlobal>();
        }
    }
}
