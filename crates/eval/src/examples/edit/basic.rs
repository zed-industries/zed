use std::{collections::HashSet, path::Path, sync::Arc};

use anyhow::Result;
use assistant_tools::{CreateFileToolInput, EditFileToolInput, ReadFileToolInput};
use async_trait::async_trait;
use buffer_diff::DiffHunkStatus;
use collections::HashMap;

use crate::example::{
    Example, ExampleContext, ExampleMetadata, FileEditHunk, FileEdits, JudgeAssertion,
    LanguageServer,
};

pub struct EditBasic;

#[async_trait(?Send)]
impl Example for EditBasic {
    fn meta(&self) -> ExampleMetadata {
        ExampleMetadata {
            name: "edit_basic".to_string(),
            url: "https://github.com/zed-industries/zed.git".to_string(),
            revision: "58604fba86ebbffaa01f7c6834253e33bcd38c0f".to_string(),
            language_server: None,
            max_assertions: None,
        }
    }

    async fn conversation(&self, cx: &mut ExampleContext) -> Result<()> {
        cx.push_user_message(format!(
            r#"
            Read the `crates/git/src/blame.rs` file and rename all occurrences of
            the `sha` field to `git_sha`. I want you to change just occurrences on this
            file. Be exhaustive.

            IMPORTANT: You are only allowed to use the `read_file` and `edit_file` tools!
            "#
        ));

        let response = cx.run_to_end().await?;
        let expected_edits = HashMap::from_iter([(
            Arc::from(Path::new("crates/git/src/blame.rs")),
            FileEdits {
                hunks: vec![
                    FileEditHunk {
                        base_text: "            unique_shas.insert(entry.sha);\n".into(),
                        text: "            unique_shas.insert(entry.git_sha);\n".into(),
                        status: DiffHunkStatus::modified_none(),
                    },
                    FileEditHunk {
                        base_text: "    pub sha: Oid,\n".into(),
                        text: "    pub git_sha: Oid,\n".into(),
                        status: DiffHunkStatus::modified_none(),
                    },
                    FileEditHunk {
                        base_text: "        let sha = parts\n".into(),
                        text: "        let git_sha = parts\n".into(),
                        status: DiffHunkStatus::modified_none(),
                    },
                    FileEditHunk {
                        base_text:
                            "            .ok_or_else(|| anyhow!(\"failed to parse sha\"))?;\n"
                                .into(),
                        text:
                            "            .ok_or_else(|| anyhow!(\"failed to parse git_sha\"))?;\n"
                                .into(),
                        status: DiffHunkStatus::modified_none(),
                    },
                    FileEditHunk {
                        base_text: "            sha,\n".into(),
                        text: "            git_sha,\n".into(),
                        status: DiffHunkStatus::modified_none(),
                    },
                    FileEditHunk {
                        base_text: "                    .get(&new_entry.sha)\n".into(),
                        text: "                    .get(&new_entry.git_sha)\n".into(),
                        status: DiffHunkStatus::modified_none(),
                    },
                    FileEditHunk {
                        base_text: "                let is_committed = !entry.sha.is_zero();\n"
                            .into(),
                        text: "                let is_committed = !entry.git_sha.is_zero();\n"
                            .into(),
                        status: DiffHunkStatus::modified_none(),
                    },
                    FileEditHunk {
                        base_text: "                index.insert(entry.sha, entries.len());\n"
                            .into(),
                        text: "                index.insert(entry.git_sha, entries.len());\n"
                            .into(),
                        status: DiffHunkStatus::modified_none(),
                    },
                    FileEditHunk {
                        base_text: "                if !entry.sha.is_zero() {\n".into(),
                        text: "                if !entry.git_sha.is_zero() {\n".into(),
                        status: DiffHunkStatus::modified_none(),
                    },
                ],
            },
        )]);
        let actual_edits = cx.edits();
        cx.assert_eq(&actual_edits, &expected_edits, "edits don't match")?;

        Ok(())
    }
}
