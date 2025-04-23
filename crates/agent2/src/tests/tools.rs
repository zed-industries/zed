use super::*;

#[derive(JsonSchema, Serialize, Deserialize)]
pub struct EchoToolInput {
    text: String,
}

pub struct EchoTool;

impl Tool for EchoTool {
    type Input = EchoToolInput;

    fn name(&self) -> String {
        "echo".to_string()
    }

    fn description(&self) -> String {
        "A tool that echoes its input".to_string()
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

    fn name(&self) -> String {
        "delay".to_string()
    }

    fn description(&self) -> String {
        "A tool that waits for a specified delay".to_string()
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
