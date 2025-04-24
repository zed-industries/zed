use super::*;

/// A tool that echoes its input
#[derive(JsonSchema, Serialize, Deserialize)]
pub struct EchoToolInput {
    /// The text to echo.
    text: String,
}

pub struct EchoTool;

impl Tool for EchoTool {
    type Input = EchoToolInput;

    fn name(&self) -> SharedString {
        "echo".into()
    }

    fn run(self: Arc<Self>, input: Self::Input, _cx: &mut App) -> Task<Result<String>> {
        Task::ready(Ok(input.text))
    }
}

/// A tool that waits for a specified delay
#[derive(JsonSchema, Serialize, Deserialize)]
pub struct DelayToolInput {
    /// The delay in milliseconds.
    ms: u64,
}

pub struct DelayTool;

impl Tool for DelayTool {
    type Input = DelayToolInput;

    fn name(&self) -> SharedString {
        "delay".into()
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

/// A tool that takes an object with map from letters to random words starting with that letter.
/// All fiealds are required! Pass a word for every letter!
#[derive(JsonSchema, Serialize, Deserialize)]
pub struct WordListInput {
    /// Provide a random word that starts with A.
    a: Option<String>,
    /// Provide a random word that starts with B.
    b: Option<String>,
    /// Provide a random word that starts with C.
    c: Option<String>,
    /// Provide a random word that starts with D.
    d: Option<String>,
    /// Provide a random word that starts with E.
    e: Option<String>,
    /// Provide a random word that starts with F.
    f: Option<String>,
    /// Provide a random word that starts with G.
    g: Option<String>,
}

pub struct WordListTool;

impl Tool for WordListTool {
    type Input = WordListInput;

    fn name(&self) -> SharedString {
        "word_list".into()
    }

    fn run(self: Arc<Self>, _input: Self::Input, _cx: &mut App) -> Task<Result<String>> {
        Task::ready(Ok("ok".to_string()))
    }
}
