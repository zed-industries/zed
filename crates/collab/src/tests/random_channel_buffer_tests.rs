use crate::tests::{run_randomized_test, RandomizedTest, TestClient, TestError, UserTestPlan};
use anyhow::Result;
use async_trait::async_trait;
use gpui::{executor::Deterministic, TestAppContext};
use rand::rngs::StdRng;
use serde_derive::{Deserialize, Serialize};
use std::{rc::Rc, sync::Arc};

#[gpui::test]
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
    Join,
}

#[async_trait(?Send)]
impl RandomizedTest for RandomChannelBufferTest {
    type Operation = ChannelBufferOperation;

    fn generate_operation(
        client: &TestClient,
        rng: &mut StdRng,
        plan: &mut UserTestPlan,
        cx: &TestAppContext,
    ) -> ChannelBufferOperation {
        ChannelBufferOperation::Join
    }

    async fn apply_operation(
        client: &TestClient,
        operation: ChannelBufferOperation,
        cx: &mut TestAppContext,
    ) -> Result<(), TestError> {
        Ok(())
    }

    async fn on_client_added(client: &Rc<TestClient>) {}

    fn on_clients_quiesced(clients: &[(Rc<TestClient>, TestAppContext)]) {}
}
