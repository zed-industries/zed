use super::*;

#[derive(JsonSchema, Serialize, Deserialize)]
pub struct EchoToolInput {
    text: String,
}

pub struct EchoTool;

impl Tool for EchoTool {
    fn name(&self) -> String {
        "echo".to_string()
    }

    fn description(&self) -> String {
        "A tool that echoes its input".to_string()
    }

    fn icon(&self) -> IconName {
        IconName::Ai
    }

    fn needs_confirmation(&self, _input: &serde_json::Value, _cx: &gpui::App) -> bool {
        false
    }

    fn ui_text(&self, _input: &serde_json::Value) -> String {
        "Echo".to_string()
    }

    fn run(
        self: Arc<Self>,
        input: serde_json::Value,
        _messages: &[LanguageModelRequestMessage],
        _project: gpui::Entity<Project>,
        _action_log: gpui::Entity<assistant_tool::ActionLog>,
        cx: &mut gpui::App,
    ) -> ToolResult {
        ToolResult {
            output: cx.foreground_executor().spawn(async move {
                let input: EchoToolInput = serde_json::from_value(input)?;
                Ok(input.text)
            }),
            card: None,
        }
    }

    fn input_schema(&self, format: LanguageModelToolSchemaFormat) -> Result<serde_json::Value> {
        assistant_tools::json_schema_for::<EchoToolInput>(format)
    }
}

#[derive(JsonSchema, Serialize, Deserialize)]
pub struct DelayToolInput {
    ms: u64,
}

pub struct DelayTool;

impl Tool for DelayTool {
    fn name(&self) -> String {
        "delay".to_string()
    }

    fn description(&self) -> String {
        "A tool that waits for a specified delay".to_string()
    }

    fn icon(&self) -> IconName {
        IconName::Cog
    }

    fn needs_confirmation(&self, _input: &serde_json::Value, _cx: &gpui::App) -> bool {
        false
    }

    fn ui_text(&self, _input: &serde_json::Value) -> String {
        "Delay".to_string()
    }

    fn run(
        self: Arc<Self>,
        input: serde_json::Value,
        _messages: &[LanguageModelRequestMessage],
        _project: gpui::Entity<Project>,
        _action_log: gpui::Entity<assistant_tool::ActionLog>,
        cx: &mut gpui::App,
    ) -> ToolResult {
        ToolResult {
            output: cx.foreground_executor().spawn(async move {
                let input: DelayToolInput = serde_json::from_value(input)?;
                smol::Timer::after(Duration::from_millis(input.ms)).await;
                Ok("Ding".to_string())
            }),
            card: None,
        }
    }

    fn input_schema(&self, format: LanguageModelToolSchemaFormat) -> Result<serde_json::Value> {
        assistant_tools::json_schema_for::<DelayToolInput>(format)
    }
}
