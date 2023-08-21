use crate::ChannelId;
use anyhow::Result;
use client::Client;
use gpui::{Entity, ModelContext, ModelHandle, Task};
use rpc::proto::GetChannelBuffer;
use std::sync::Arc;

// Open the channel document
// ChannelDocumentView { ChannelDocument, Editor } -> On clone, clones internal ChannelDocument handle, instantiates new editor
// Produces a view which is: (ChannelDocument, Editor), ChannelDocument manages subscriptions
// ChannelDocuments -> Buffers -> Editor with that buffer

// ChannelDocuments {
//     ChannleBuffers: HashMap<bufferId, ModelHandle<language::Buffer>>
// }

pub struct ChannelBuffer {
    channel_id: ChannelId,
    buffer: Option<ModelHandle<language::Buffer>>,
    client: Arc<Client>,
}

impl Entity for ChannelBuffer {
    type Event = ();
}

impl ChannelBuffer {
    pub fn for_channel(
        channel_id: ChannelId,
        client: Arc<Client>,
        cx: &mut ModelContext<Self>,
    ) -> Self {
        Self {
            channel_id,
            client,
            buffer: None,
        }
    }

    fn on_buffer_update(
        &mut self,
        buffer: ModelHandle<language::Buffer>,
        event: &language::Event,
        cx: &mut ModelContext<Self>,
    ) {
        //
    }

    pub fn buffer(
        &mut self,
        cx: &mut ModelContext<Self>,
    ) -> Task<Result<ModelHandle<language::Buffer>>> {
        if let Some(buffer) = &self.buffer {
            Task::ready(Ok(buffer.clone()))
        } else {
            let channel_id = self.channel_id;
            let client = self.client.clone();
            cx.spawn(|this, mut cx| async move {
                let response = client.request(GetChannelBuffer { channel_id }).await?;

                let base_text = response.base_text;
                let operations = response
                    .operations
                    .into_iter()
                    .map(language::proto::deserialize_operation)
                    .collect::<Result<Vec<_>, _>>()?;

                this.update(&mut cx, |this, cx| {
                    let buffer = cx.add_model(|cx| language::Buffer::new(0, base_text, cx));
                    buffer.update(cx, |buffer, cx| buffer.apply_ops(operations, cx))?;

                    cx.subscribe(&buffer, Self::on_buffer_update).detach();

                    this.buffer = Some(buffer.clone());
                    anyhow::Ok(buffer)
                })
            })
        }
    }
}
