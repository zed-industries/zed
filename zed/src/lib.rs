use futures::Future;
use gpui::MutableAppContext;
use rpc_client::RpcClient;
use std::sync::Arc;
use zed_rpc::proto::RequestMessage;

pub mod assets;
pub mod editor;
pub mod file_finder;
pub mod language;
pub mod menus;
mod operation_queue;
pub mod rpc_client;
pub mod settings;
mod sum_tree;
#[cfg(test)]
mod test;
mod time;
mod util;
pub mod workspace;
mod worktree;

#[derive(Clone)]
pub struct AppState {
    pub settings: postage::watch::Receiver<settings::Settings>,
    pub language_registry: std::sync::Arc<language::LanguageRegistry>,
    pub rpc_client: Arc<RpcClient>,
}

impl AppState {
    pub async fn on_rpc_request<Req, F, Fut>(
        &self,
        cx: &mut MutableAppContext,
        handler: F,
    ) where
        Req: RequestMessage,
        F: 'static + Send + Sync + Fn(Req, &AppState, &mut MutableAppContext) -> Fut,
        Fut: 'static + Send + Sync + Future<Output = Req::Response>,
    {
        let app_state = self.clone();
        let cx = cx.to_background();
        app_state
            .rpc_client
            .on_request(move |req| cx.update(|cx| async move {
                handler(req, &app_state, cx)
            })
            .await
    }
}

pub fn init(cx: &mut gpui::MutableAppContext) {
    cx.add_global_action("app:quit", quit);
}

fn quit(_: &(), cx: &mut gpui::MutableAppContext) {
    cx.platform().quit();
}
