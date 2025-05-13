use anyhow::Result;
use assistant_settings::AgentProfileId;
use assistant_tools::EditFileToolInput;
use async_trait::async_trait;

use crate::example::{Example, ExampleContext, ExampleMetadata};

pub struct FileOverwriteExample;

/***
Results before the fix:
┌───────┬────────────────────────────────────────────┬──────────┐
│ Round │                 Assertion                  │  Result  │
├───────┼────────────────────────────────────────────┼──────────┤
│   0   │ File should be edited, not overwritten     │ ✗ Failed │
│   1   │ File should be edited, not overwritten     │ ✗ Failed │
│   2   │ File should be edited, not overwritten     │ ✗ Failed │
│   3   │ File should be edited, not overwritten     │ ✗ Failed │
│   4   │ File should be edited, not overwritten     │ ✗ Failed │
│   5   │ File should be edited, not overwritten     │ ✗ Failed │
│   6   │ File should be edited, not overwritten     │ ✗ Failed │
│   7   │ File should be edited, not overwritten     │ ✗ Failed │
│   8   │ File should be edited, not overwritten     │ ✗ Failed │
│   9   │ File should be edited, not overwritten     │ ✗ Failed │
│  10   │ File should be edited, not overwritten     │ ✔︎ Passed │
│  11   │ File should be edited, not overwritten     │ ✗ Failed │
│  12   │ File should be edited, not overwritten     │ ✗ Failed │
│  13   │ File should be edited, not overwritten     │ ✗ Failed │
│  14   │ File should be edited, not overwritten     │ ✗ Failed │
│  15   │ File should be edited, not overwritten     │ ✗ Failed │
│  16   │ File should be edited, not overwritten     │ ✔︎ Passed │
│  17   │ File should be edited, not overwritten     │ ✗ Failed │
│  18   │ File should be edited, not overwritten     │ ✗ Failed │
│  19   │ File should be edited, not overwritten     │ ✗ Failed │
├───────┼────────────────────────────────────────────┼──────────┤
│   0   │ total                                      │       0% │
│   1   │ total                                      │       0% │
│   2   │ total                                      │       0% │
│   3   │ total                                      │       0% │
│   4   │ total                                      │       0% │
│   5   │ total                                      │       0% │
│   6   │ total                                      │       0% │
│   7   │ total                                      │       0% │
│   8   │ total                                      │       0% │
│   9   │ total                                      │       0% │
│  10   │ total                                      │      33% │
│  11   │ total                                      │       0% │
│  12   │ total                                      │       0% │
│  13   │ total                                      │       0% │
│  14   │ total                                      │       0% │
│  15   │ total                                      │       0% │
│  16   │ total                                      │      33% │
│  17   │ total                                      │       0% │
│  18   │ total                                      │       0% │
│  19   │ total                                      │       0% │
├───────┼────────────────────────────────────────────┼──────────┤
│  avg  │ total                                      │       3% │
└───────┴────────────────────────────────────────────┴──────────┘
┌──────────────────────────────┬──────────┬──────────┬──────────┐
│             Tool             │   Uses   │ Failures │   Rate   │
├──────────────────────────────┼──────────┼──────────┼──────────┤
│find_path                     │    1     │    0     │    0%    │
└──────────────────────────────┴──────────┴──────────┴──────────┘

*/

#[async_trait(?Send)]
impl Example for FileOverwriteExample {
    fn meta(&self) -> ExampleMetadata {
        let thread_json = include_str!("threads/overwrite-file.json");

        ExampleMetadata {
            name: "file_overwrite".to_string(),
            url: "https://github.com/zed-industries/zed.git".to_string(),
            revision: "023a60806a8cc82e73bd8d88e63b4b07fc7a0040".to_string(),
            language_server: None,
            max_assertions: Some(3),
            profile_id: AgentProfileId::default(),
            existing_thread_json: Some(thread_json.to_string()),
        }
    }

    async fn conversation(&self, cx: &mut ExampleContext) -> Result<()> {
        let response = cx.run_turns(1).await?;
        let file_overwritten = if let Some(tool_use) = response.find_tool_call("edit_file") {
            let input = tool_use.parse_input::<EditFileToolInput>()?;
            input.create_or_overwrite
        } else {
            false
        };

        cx.assert(!file_overwritten, "File should be edited, not overwritten")
    }
}
