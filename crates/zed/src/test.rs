use crate::{
    assets::Assets,
    channel::ChannelList,
    http::{HttpClient, Request, Response, ServerResponse},
    language,
    settings::{self, ThemeRegistry},
    user::UserStore,
    AppState,
};
use anyhow::Result;
use buffer::LanguageRegistry;
use futures::{future::BoxFuture, Future};
use gpui::{Entity, ModelHandle, MutableAppContext};
use parking_lot::Mutex;
use rpc_client as rpc;
use smol::channel;
use std::{fmt, marker::PhantomData, sync::Arc};
use worktree::fs::FakeFs;

#[cfg(test)]
#[ctor::ctor]
fn init_logger() {
    env_logger::init();
}

pub fn sample_text(rows: usize, cols: usize) -> String {
    let mut text = String::new();
    for row in 0..rows {
        let c: char = ('a' as u32 + row as u32) as u8 as char;
        let mut line = c.to_string().repeat(cols);
        if row < rows - 1 {
            line.push('\n');
        }
        text += &line;
    }
    text
}

pub fn test_app_state(cx: &mut MutableAppContext) -> Arc<AppState> {
    let (settings_tx, settings) = settings::test(cx);
    let mut languages = LanguageRegistry::new();
    languages.add(Arc::new(language::rust()));
    let themes = ThemeRegistry::new(Assets, cx.font_cache().clone());
    let rpc = rpc::Client::new();
    let http = FakeHttpClient::new(|_| async move { Ok(ServerResponse::new(404)) });
    let user_store = cx.add_model(|cx| UserStore::new(rpc.clone(), http, cx));
    Arc::new(AppState {
        settings_tx: Arc::new(Mutex::new(settings_tx)),
        settings,
        themes,
        languages: Arc::new(languages),
        channel_list: cx.add_model(|cx| ChannelList::new(user_store.clone(), rpc.clone(), cx)),
        rpc,
        user_store,
        fs: Arc::new(FakeFs::new()),
    })
}

pub struct Observer<T>(PhantomData<T>);

impl<T: 'static> Entity for Observer<T> {
    type Event = ();
}

impl<T: Entity> Observer<T> {
    pub fn new(
        handle: &ModelHandle<T>,
        cx: &mut gpui::TestAppContext,
    ) -> (ModelHandle<Self>, channel::Receiver<()>) {
        let (notify_tx, notify_rx) = channel::unbounded();
        let observer = cx.add_model(|cx| {
            cx.observe(handle, move |_, _, _| {
                let _ = notify_tx.try_send(());
            })
            .detach();
            Observer(PhantomData)
        });
        (observer, notify_rx)
    }
}

pub struct FakeHttpClient {
    handler:
        Box<dyn 'static + Send + Sync + Fn(Request) -> BoxFuture<'static, Result<ServerResponse>>>,
}

impl FakeHttpClient {
    pub fn new<Fut, F>(handler: F) -> Arc<dyn HttpClient>
    where
        Fut: 'static + Send + Future<Output = Result<ServerResponse>>,
        F: 'static + Send + Sync + Fn(Request) -> Fut,
    {
        Arc::new(Self {
            handler: Box::new(move |req| Box::pin(handler(req))),
        })
    }
}

impl fmt::Debug for FakeHttpClient {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FakeHttpClient").finish()
    }
}

impl HttpClient for FakeHttpClient {
    fn send<'a>(&'a self, req: Request) -> BoxFuture<'a, Result<Response>> {
        let future = (self.handler)(req);
        Box::pin(async move { future.await.map(Into::into) })
    }
}
