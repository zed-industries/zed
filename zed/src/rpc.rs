use postage::prelude::Stream;
use std::{future::Future, sync::Arc};
use zed_rpc::{proto, Peer, TypedEnvelope};

pub trait MessageHandler<'a, M: proto::EnvelopedMessage> {
    type Output: 'a + Future<Output = anyhow::Result<()>>;

    fn handle(
        &self,
        message: TypedEnvelope<M>,
        rpc: &'a Arc<Peer>,
        cx: &'a mut gpui::AsyncAppContext,
    ) -> Self::Output;
}

impl<'a, M, F, Fut> MessageHandler<'a, M> for F
where
    M: proto::EnvelopedMessage,
    F: Fn(TypedEnvelope<M>, &'a Arc<Peer>, &'a mut gpui::AsyncAppContext) -> Fut,
    Fut: 'a + Future<Output = anyhow::Result<()>>,
{
    type Output = Fut;

    fn handle(
        &self,
        message: TypedEnvelope<M>,
        rpc: &'a Arc<Peer>,
        cx: &'a mut gpui::AsyncAppContext,
    ) -> Self::Output {
        (self)(message, rpc, cx)
    }
}

pub trait PeerExt {
    fn on_message<H, M>(&self, handler: H, cx: &mut gpui::MutableAppContext)
    where
        H: 'static + for<'a> MessageHandler<'a, M>,
        M: proto::EnvelopedMessage;
}

impl PeerExt for Arc<Peer> {
    fn on_message<H, M>(&self, handler: H, cx: &mut gpui::MutableAppContext)
    where
        H: 'static + for<'a> MessageHandler<'a, M>,
        M: proto::EnvelopedMessage,
    {
        let rpc = self.clone();
        let mut messages = smol::block_on(self.add_message_handler::<M>());
        cx.spawn(|mut cx| async move {
            while let Some(message) = messages.recv().await {
                if let Err(err) = handler.handle(message, &rpc, &mut cx).await {
                    log::error!("error handling message: {:?}", err);
                }
            }
        })
        .detach();
    }
}
