use super::{run_randomized_test, RandomizedTest, TestClient, TestError, TestServer, UserTestPlan};
use anyhow::Result;
use async_trait::async_trait;
use gpui::{executor::Deterministic, TestAppContext};
use rand::prelude::*;
use serde_derive::{Deserialize, Serialize};
use std::{ops::Range, rc::Rc, sync::Arc};
use text::Bias;

#[gpui::test(
    iterations = 100,
    on_failure = "crate::tests::save_randomized_test_plan"
)]
async fn test_random_channel_buffers(
    cx: &mut TestAppContext,
    deterministic: Arc<Deterministic>,
    rng: StdRng,
) {
    run_randomized_test::<RandomChannelBufferTest>(cx, deterministic, rng).await;
}

struct RandomChannelBufferTest;

#[derive(Clone, Serialize, Deserialize)]
enum ChannelBufferOperation {
    JoinChannelNotes {
        channel_name: String,
    },
    LeaveChannelNotes {
        channel_name: String,
    },
    EditChannelNotes {
        channel_name: String,
        edits: Vec<(Range<usize>, Arc<str>)>,
    },
    Noop,
}

const CHANNEL_COUNT: usize = 3;

#[async_trait(?Send)]
impl RandomizedTest for RandomChannelBufferTest {
    type Operation = ChannelBufferOperation;

    async fn initialize(server: &mut TestServer, users: &[UserTestPlan]) {
        let db = &server.app_state.db;
        for ix in 0..CHANNEL_COUNT {
            let id = db
                .create_channel(&format!("channel-{ix}"), None, users[0].user_id)
                .await
                .unwrap();
            for user in &users[1..] {
                db.invite_channel_member(id, user.user_id, users[0].user_id, false)
                    .await
                    .unwrap();
                db.respond_to_channel_invite(id, user.user_id, true)
                    .await
                    .unwrap();
            }
        }
    }

    fn generate_operation(
        client: &TestClient,
        rng: &mut StdRng,
        _: &mut UserTestPlan,
        cx: &TestAppContext,
    ) -> ChannelBufferOperation {
        let channel_store = client.channel_store().clone();
        let channel_buffers = client.channel_buffers();

        // When signed out, we can't do anything unless a channel buffer is
        // already open.
        if channel_buffers.is_empty()
            && channel_store.read_with(cx, |store, _| store.channel_count() == 0)
        {
            return ChannelBufferOperation::Noop;
        }

        loop {
            match rng.gen_range(0..100_u32) {
                0..=29 => {
                    let channel_name = client.channel_store().read_with(cx, |store, cx| {
                        store.channel_dag_entries().find_map(|(_, channel)| {
                            if store.has_open_channel_buffer(channel.id, cx) {
                                None
                            } else {
                                Some(channel.name.clone())
                            }
                        })
                    });
                    if let Some(channel_name) = channel_name {
                        break ChannelBufferOperation::JoinChannelNotes { channel_name };
                    }
                }

                30..=40 => {
                    if let Some(buffer) = channel_buffers.iter().choose(rng) {
                        let channel_name = buffer.read_with(cx, |b, _| b.channel().name.clone());
                        break ChannelBufferOperation::LeaveChannelNotes { channel_name };
                    }
                }

                _ => {
                    if let Some(buffer) = channel_buffers.iter().choose(rng) {
                        break buffer.read_with(cx, |b, _| {
                            let channel_name = b.channel().name.clone();
                            let edits = b
                                .buffer()
                                .read_with(cx, |buffer, _| buffer.get_random_edits(rng, 3));
                            ChannelBufferOperation::EditChannelNotes {
                                channel_name,
                                edits,
                            }
                        });
                    }
                }
            }
        }
    }

    async fn apply_operation(
        client: &TestClient,
        operation: ChannelBufferOperation,
        cx: &mut TestAppContext,
    ) -> Result<(), TestError> {
        match operation {
            ChannelBufferOperation::JoinChannelNotes { channel_name } => {
                let buffer = client.channel_store().update(cx, |store, cx| {
                    let channel_id = store
                        .channel_dag_entries()
                        .find(|(_, c)| c.name == channel_name)
                        .unwrap()
                        .1
                        .id;
                    if store.has_open_channel_buffer(channel_id, cx) {
                        Err(TestError::Inapplicable)
                    } else {
                        Ok(store.open_channel_buffer(channel_id, cx))
                    }
                })?;

                log::info!(
                    "{}: opening notes for channel {channel_name}",
                    client.username
                );
                client.channel_buffers().insert(buffer.await?);
            }

            ChannelBufferOperation::LeaveChannelNotes { channel_name } => {
                let buffer = cx.update(|cx| {
                    let mut left_buffer = Err(TestError::Inapplicable);
                    client.channel_buffers().retain(|buffer| {
                        if buffer.read(cx).channel().name == channel_name {
                            left_buffer = Ok(buffer.clone());
                            false
                        } else {
                            true
                        }
                    });
                    left_buffer
                })?;

                log::info!(
                    "{}: closing notes for channel {channel_name}",
                    client.username
                );
                cx.update(|_| drop(buffer));
            }

            ChannelBufferOperation::EditChannelNotes {
                channel_name,
                edits,
            } => {
                let channel_buffer = cx
                    .read(|cx| {
                        client
                            .channel_buffers()
                            .iter()
                            .find(|buffer| buffer.read(cx).channel().name == channel_name)
                            .cloned()
                    })
                    .ok_or_else(|| TestError::Inapplicable)?;

                log::info!(
                    "{}: editing notes for channel {channel_name} with {:?}",
                    client.username,
                    edits
                );

                channel_buffer.update(cx, |buffer, cx| {
                    let buffer = buffer.buffer();
                    buffer.update(cx, |buffer, cx| {
                        let snapshot = buffer.snapshot();
                        buffer.edit(
                            edits.into_iter().map(|(range, text)| {
                                let start = snapshot.clip_offset(range.start, Bias::Left);
                                let end = snapshot.clip_offset(range.end, Bias::Right);
                                (start..end, text)
                            }),
                            None,
                            cx,
                        );
                    });
                });
            }

            ChannelBufferOperation::Noop => Err(TestError::Inapplicable)?,
        }
        Ok(())
    }

    async fn on_client_added(client: &Rc<TestClient>, cx: &mut TestAppContext) {
        let channel_store = client.channel_store();
        while channel_store.read_with(cx, |store, _| store.channel_count() == 0) {
            channel_store.next_notification(cx).await;
        }
    }

    async fn on_quiesce(server: &mut TestServer, clients: &mut [(Rc<TestClient>, TestAppContext)]) {
        let channels = server.app_state.db.all_channels().await.unwrap();

        for (client, client_cx) in clients.iter_mut() {
            client_cx.update(|cx| {
                client
                    .channel_buffers()
                    .retain(|b| b.read(cx).is_connected());
            });
        }

        for (channel_id, channel_name) in channels {
            let mut prev_text: Option<(u64, String)> = None;

            let mut collaborator_user_ids = server
                .app_state
                .db
                .get_channel_buffer_collaborators(channel_id)
                .await
                .unwrap()
                .into_iter()
                .map(|id| id.to_proto())
                .collect::<Vec<_>>();
            collaborator_user_ids.sort();

            for (client, client_cx) in clients.iter() {
                let user_id = client.user_id().unwrap();
                client_cx.read(|cx| {
                    if let Some(channel_buffer) = client
                        .channel_buffers()
                        .iter()
                        .find(|b| b.read(cx).channel().id == channel_id.to_proto())
                    {
                        let channel_buffer = channel_buffer.read(cx);

                        // Assert that channel buffer's text matches other clients' copies.
                        let text = channel_buffer.buffer().read(cx).text();
                        if let Some((prev_user_id, prev_text)) = &prev_text {
                            assert_eq!(
                                &text,
                                prev_text,
                                "client {user_id} has different text than client {prev_user_id} for channel {channel_name}",
                            );
                        } else {
                            prev_text = Some((user_id, text.clone()));
                        }

                        // Assert that all clients and the server agree about who is present in the
                        // channel buffer.
                        let collaborators = channel_buffer.collaborators();
                        let mut user_ids =
                            collaborators.values().map(|c| c.user_id).collect::<Vec<_>>();
                        user_ids.sort();
                        assert_eq!(
                            user_ids,
                            collaborator_user_ids,
                            "client {user_id} has different user ids for channel {channel_name} than the server",
                        );
                    }
                });
            }
        }
    }
}
