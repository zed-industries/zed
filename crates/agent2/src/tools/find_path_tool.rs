use crate::AgentTool;

pub struct FindPathTool {}

// impl AgentTool for assistant_tools::FindPathTool {
//     type Input = assistant_tools::FindPathToolInput;

//     fn name(&self) -> ui::SharedString {
//         assistant_tool::Tool::name(self)
//     }

//     fn needs_authorization(&self, input: Self::Input, cx: &ui::App) -> bool {
//         todo!()
//     }

//     fn run(
//         self: std::sync::Arc<Self>,
//         input: Self::Input,
//         cx: &mut ui::App,
//     ) -> gpui::Task<gpui::Result<String>> {
//         todo!()
//     }
// }
