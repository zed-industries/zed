use crate::ChannelId;
use anyhow::Result;
use client::Client;
use gpui::{AppContext, AsyncAppContext, Entity, ModelContext, ModelHandle, Task};
use rpc::{proto, TypedEnvelope};
use std::sync::Arc;
use util::ResultExt;

// Open the channel document
// ChannelDocumentView { ChannelDocument, Editor } -> On clone, clones internal ChannelDocument handle, instantiates new editor
// Produces a view which is: (ChannelDocument, Editor), ChannelDocument manages subscriptions
// ChannelDocuments -> Buffers -> Editor with that buffer

// ChannelDocuments {
//     ChannleBuffers: HashMap<bufferId, ModelHandle<language::Buffer>>
// }

type BufferId = u64;

pub struct ChannelBuffer {
    channel_id: ChannelId,
    buffer_id: BufferId,
    buffer: ModelHandle<language::Buffer>,
    client: Arc<Client>,
}

impl Entity for ChannelBuffer {
    type Event = ();
}

impl ChannelBuffer {
    pub fn for_channel(
        channel_id: ChannelId,
        client: Arc<Client>,
        cx: &mut AppContext,
    ) -> Task<Result<ModelHandle<Self>>> {
        cx.spawn(|mut cx| async move {
            let response = client
                .request(proto::JoinChannelBuffer { channel_id })
                .await?;

            let base_text = response.base_text;
            let operations = response
                .operations
                .into_iter()
                .map(language::proto::deserialize_operation)
                .collect::<Result<Vec<_>, _>>()?;
            let buffer_id = response.buffer_id;

            let buffer = cx.add_model(|cx| language::Buffer::new(0, base_text, cx));
            buffer.update(&mut cx, |buffer, cx| buffer.apply_ops(operations, cx))?;

            anyhow::Ok(cx.add_model(|cx| {
                cx.subscribe(&buffer, Self::on_buffer_update).detach();
                client.add_model_message_handler(Self::handle_update_channel_buffer);
                Self {
                    buffer_id,
                    buffer,
                    client,
                    channel_id,
                }
            }))
        })
    }

    async fn handle_update_channel_buffer(
        this: ModelHandle<Self>,
        update_channel_buffer: TypedEnvelope<proto::UpdateChannelBuffer>,
        _: Arc<Client>,
        mut cx: AsyncAppContext,
    ) -> Result<()> {
        let ops = update_channel_buffer
            .payload
            .operations
            .into_iter()
            .map(language::proto::deserialize_operation)
            .collect::<Result<Vec<_>, _>>()?;

        this.update(&mut cx, |this, cx| {
            this.buffer
                .update(cx, |buffer, cx| buffer.apply_ops(ops, cx))
        })?;

        Ok(())
    }

    fn on_buffer_update(
        &mut self,
        _: ModelHandle<language::Buffer>,
        event: &language::Event,
        _: &mut ModelContext<Self>,
    ) {
        if let language::Event::Operation(operation) = event {
            let operation = language::proto::serialize_operation(operation);
            self.client
                .send(proto::UpdateChannelBuffer {
                    buffer_id: self.buffer_id,
                    operations: vec![operation],
                })
                .log_err();
        }
    }

    pub fn buffer(&self) -> ModelHandle<language::Buffer> {
        self.buffer.clone()
    }
}
