use super::*;

#[derive(JsonSchema, Serialize, Deserialize)]
pub struct EchoToolInput {
    text: String,
}

pub struct EchoTool;

impl Tool for EchoTool {
    type Input = EchoToolInput;

    fn name(&self) -> SharedString {
        "echo".into()
    }

    fn description(&self) -> SharedString {
        "A tool that echoes its input".into()
    }

    fn run(self: Arc<Self>, input: Self::Input, _cx: &mut App) -> Task<Result<String>> {
        Task::ready(Ok(input.text))
    }
}

#[derive(JsonSchema, Serialize, Deserialize)]
pub struct DelayToolInput {
    ms: u64,
}

pub struct DelayTool;

impl Tool for DelayTool {
    type Input = DelayToolInput;

    fn name(&self) -> SharedString {
        "delay".into()
    }

    fn description(&self) -> SharedString {
        "A tool that waits for a specified delay".into()
    }

    fn run(self: Arc<Self>, input: Self::Input, cx: &mut App) -> Task<Result<String>>
    where
        Self: Sized,
    {
        cx.foreground_executor().spawn(async move {
            smol::Timer::after(Duration::from_millis(input.ms)).await;
            Ok("Ding".to_string())
        })
    }
}
