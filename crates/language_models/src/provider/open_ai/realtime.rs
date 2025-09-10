use anyhow::Result;
use audio::{CHANNEL_COUNT, SAMPLE_RATE, SamplesBufferExt, SourceExt};
use base64::engine::{Engine, general_purpose};
use futures::SinkExt;
use futures::channel::mpsc::{UnboundedReceiver, UnboundedSender};
use language_model::{
    LanguageModelRequestTool, LanguageModelToolResultContent, LanguageModelToolUse,
    RealtimeRequest, RealtimeResponse,
};
use open_ai::realtime::api::RealtimeClient;
use open_ai::realtime::client_event::{
    ConversationItemCreate, InputAudioBufferAppend, ResponseCreate, SessionUpdate,
};
use open_ai::realtime::server_event::ServerEvent;
use open_ai::realtime::types::{
    AudioFormat, CreateResponse, Item, ItemContent, ItemContentType, ItemType, SessionAudio,
    SessionAudioInput, SessionAudioOutput, TurnDetection,
};
use open_ai::realtime::types::{ResponseStatusDetail, Session};
use rodio::buffer::SamplesBuffer;
use rodio::{ChannelCount, SampleRate};
use smol::stream::StreamExt;

use std::str::FromStr;
use tokio_tungstenite::tungstenite::protocol::Message;

const OPENAI_CHANNEL_COUNT: ChannelCount = ChannelCount::new(1).unwrap();
const OPENAI_SAMPLE_RATE: SampleRate = SampleRate::new(24000).unwrap();

pub struct OpenAiRealtimeClient;

impl OpenAiRealtimeClient {
    pub async fn run(
        api_key: String,
        model: open_ai::Model,
        tools: Vec<LanguageModelRequestTool>,
        mut input_rx: UnboundedReceiver<RealtimeRequest>,
        mut output_tx: UnboundedSender<RealtimeResponse>,
    ) -> Result<()> {
        log::info!(
            "Starting OpenAI realtime client. Api key: {} Model: {}",
            api_key,
            model.id()
        );

        let tools: Vec<_> = tools
            .into_iter()
            .map(|tool| open_ai::realtime::types::ToolDefinition::Function {
                name: tool.name,
                description: tool.description,
                parameters: tool.input_schema,
            })
            .collect();

        let realtime_client = RealtimeClient::new(api_key, model.id().to_string());

        let (mut write, mut read) = realtime_client
            .connect()
            .await
            .map_err(|e| anyhow::anyhow!("Error connecting to OpenAI realtime API: {}", e))?;

        log::info!("WebSocket handshake complete");

        let update_session: Message = SessionUpdate {
            session: Session {
                tools: Some(tools),
                model: Some(model.id().to_string()),

                audio: Some(SessionAudio {
                    input: Some(SessionAudioInput {
                        format: Some(AudioFormat {
                            r#type: "audio/pcm".to_string(),
                            rate: 24000,
                        }),
                        turn_detection: Some(TurnDetection::ServerVAD {
                            threshold: 0.75,
                            prefix_padding_ms: 350,
                            silence_duration_ms: 500,
                            create_response: Some(true),
                            interrupt_response: Some(true),
                        }),
                    }),
                    output: Some(SessionAudioOutput {
                        format: Some(AudioFormat {
                            r#type: "audio/pcm".to_string(),
                            rate: 24000,
                        }),
                    }),
                }),
                ..Default::default()
            },
            ..Default::default()
        }
        .into();

        write.send(update_session).await?;

        let send_task = async move {
            while let Some(ev) = &mut input_rx.next().await {
                match ev {
                    RealtimeRequest::Audio(samples) => {
                        let i16_iter = samples
                            .convert_channels(OPENAI_CHANNEL_COUNT)
                            .convert_sample_rate(OPENAI_SAMPLE_RATE)
                            .convert_sample_type::<i16>();

                        let mut pcm_le =
                            Vec::with_capacity(i16_iter.size_hint().0.saturating_mul(2));

                        for s in i16_iter {
                            pcm_le.extend_from_slice(&s.to_le_bytes());
                        }

                        let b64 = general_purpose::STANDARD.encode(&pcm_le);

                        let append_audio: Message = InputAudioBufferAppend {
                            audio: b64,
                            ..Default::default()
                        }
                        .into();

                        write.send(append_audio).await?;
                    }
                    RealtimeRequest::Text(text) => {
                        let item_create_message: Message = ConversationItemCreate {
                            item: Item {
                                r#type: Some(ItemType::Message),
                                role: Some(open_ai::realtime::types::ItemRole::User),
                                content: Some(vec![ItemContent {
                                    r#type: open_ai::realtime::types::ItemContentType::InputText,
                                    text: Some(text.clone()),
                                    audio: None,
                                    transcript: None,
                                }]),
                                ..Default::default()
                            },
                            ..Default::default()
                        }
                        .into();

                        write.send(item_create_message).await?;
                    }
                    RealtimeRequest::ToolResult(tool_result) => {
                        log::debug!("Got tool result: {:?}", tool_result);

                        match &tool_result.content {
                            LanguageModelToolResultContent::Text(output) => {
                                let item_create_message: Message = ConversationItemCreate {
                                    item: Item {
                                        r#type: Some(ItemType::FunctionCallOutput),
                                        call_id: Some(tool_result.tool_use_id.to_string()),
                                        output: Some(output.to_string()),
                                        ..Default::default()
                                    },
                                    ..Default::default()
                                }
                                .into();

                                write.send(item_create_message).await?;

                                let response_create_message: Message = ResponseCreate {
                                        response: Some(CreateResponse {
                                            instructions:
                                            Some("If the function call output is useful to the user, summarize it briefly; otherwise, unless there is another reason to speak, don’t respond verbally. Call more functions if needed."
                                                .to_string()),
                                        }),
                                        ..Default::default()
                                    }
                                    .into();

                                write.send(response_create_message).await?;
                            }
                            _ => {}
                        };
                    }
                }
            }

            // Try to gracefully close the WebSocket so the receiver loop can terminate.
            if let Err(err) = write.send(Message::Close(None)).await {
                log::warn!("Failed to send WebSocket close frame: {}", err);
            }

            Ok::<(), anyhow::Error>(())
        };

        let receive_task = async move {
            while let Some(message) = read.next().await {
                let message = match message {
                    Ok(m) => m,
                    Err(err) => {
                        log::error!("WebSocket receive error: {}", err);
                        continue;
                    }
                };

                match message {
                    Message::Text(text) => {
                        let server_event: ServerEvent = match serde_json::de::from_str(&text) {
                            Ok(ev) => ev,
                            Err(err) => {
                                log::error!("Failed to deserialize server event: {}", err);
                                continue;
                            }
                        };

                        match server_event {
                            ServerEvent::ResponseOutputAudioDelta(ev) => {
                                let b64 = match general_purpose::STANDARD.decode(ev.delta) {
                                    Ok(b) => b,
                                    Err(err) => {
                                        log::error!("Failed to decode audio delta: {}", err);
                                        continue;
                                    }
                                };

                                let mut chunks = b64.chunks_exact(2);
                                let mut i16s = Vec::with_capacity(chunks.len());
                                for c in &mut chunks {
                                    i16s.push(i16::from_le_bytes([c[0], c[1]]));
                                }

                                let samples = SamplesBuffer::from_sample_type(
                                    i16s,
                                    OPENAI_SAMPLE_RATE,
                                    OPENAI_CHANNEL_COUNT,
                                )
                                .convert_sample_rate(SAMPLE_RATE)
                                .convert_channels(CHANNEL_COUNT);

                                output_tx.send(RealtimeResponse::Audio(samples)).await?;
                            }
                            ServerEvent::ConversationItemCreated(_) => {}
                            ServerEvent::ResponseOutputItemDone(ev) => {
                                if let (item, Some(item_type)) = (ev.item.clone(), ev.item.r#type) {
                                    match item_type {
                                        ItemType::Message => {
                                            for content in item.content.unwrap_or_default() {
                                                match content.r#type {
                                                    ItemContentType::Text => {
                                                        if let Some(text) = content.text {
                                                            log::info!(
                                                                "Text message created: {}",
                                                                text
                                                            );
                                                        } else {
                                                            log::warn!(
                                                                "Text content missing in message item"
                                                            );
                                                        }
                                                    }
                                                    _ => {}
                                                }
                                            }
                                        }
                                        ItemType::FunctionCall => {
                                            if let (Some(call_id), Some(name), Some(raw_input)) = (
                                                item.call_id.clone(),
                                                item.name.clone(),
                                                item.arguments.clone(),
                                            ) {
                                                let input =
                                                    serde_json::Value::from_str(&raw_input)?;

                                                output_tx
                                                    .send(RealtimeResponse::ToolUse(
                                                        LanguageModelToolUse {
                                                            id: call_id.into(),
                                                            name: name.as_str().into(),
                                                            is_input_complete: true,
                                                            input,
                                                            raw_input,
                                                        },
                                                    ))
                                                    .await?;
                                            } else {
                                                log::warn!(
                                                    "Missing fields in function call item (call_id/name/arguments)"
                                                );
                                            }
                                        }
                                        ItemType::FunctionCallOutput => {}
                                    }
                                }
                            }
                            ServerEvent::InputAudioBufferSpeechStarted(_) => {
                                output_tx.send(RealtimeResponse::AudioEnd).await?;
                            }
                            ServerEvent::ResponseOutputTextDelta(ev) => {
                                log::debug!("Text response: {}", ev.delta);
                                output_tx.send(RealtimeResponse::Text(ev.delta)).await?;
                            }
                            ServerEvent::ResponseOutputTextDone(ev) => {
                                log::debug!("Text response: {}", ev.text);
                            }
                            ServerEvent::ResponseOutputAudioTranscriptDelta(_ev) => {}
                            ServerEvent::ResponseOutputAudioTranscriptDone(ev) => {
                                log::debug!("{}", ev.transcript.trim());
                            }
                            ServerEvent::ResponseDone(ev) => {
                                if let Some(details) = ev.response.status_details {
                                    match details {
                                        ResponseStatusDetail::Incomplete { .. } => {}
                                        ResponseStatusDetail::Failed { error } => {
                                            if let Some(err) = error {
                                                let msg = err.message.unwrap_or_default();
                                                log::error!(
                                                    "Error getting realtime response: {}",
                                                    msg
                                                );
                                            } else {
                                                log::error!(
                                                    "Realtime response failed with no error details"
                                                );
                                            }
                                        }
                                        _ => {}
                                    }
                                }
                            }
                            ServerEvent::ResponseOutputAudioDone(_) => {}
                            ServerEvent::Error(e) => {
                                log::error!("{e:?}");
                            }
                            _ => {}
                        }
                    }
                    Message::Close(_) => {
                        log::info!("WebSocket closed");
                        break;
                    }
                    _ => {}
                }
            }
            Ok::<(), anyhow::Error>(())
        };

        tokio::try_join!(send_task, receive_task)?;

        Ok(())
    }
}
