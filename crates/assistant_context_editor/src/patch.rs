use anyhow::{anyhow, Context as _, Result};
use collections::HashMap;
use editor::ProposedChangesEditor;
use futures::{future, TryFutureExt as _};
use gpui::{App, AppContext as _, AsyncApp, Entity, SharedString};
use language::{AutoindentMode, Buffer, BufferSnapshot};
use project::{Project, ProjectPath};
use std::{cmp, ops::Range, path::Path, sync::Arc};
use text::{AnchorRangeExt as _, Bias, OffsetRangeExt as _, Point};

#[derive(Clone, Debug)]
pub struct AssistantPatch {
    pub range: Range<language::Anchor>,
    pub title: SharedString,
    pub edits: Arc<[Result<AssistantEdit>]>,
    pub status: AssistantPatchStatus,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum AssistantPatchStatus {
    Pending,
    Ready,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AssistantEdit {
    pub path: String,
    pub kind: AssistantEditKind,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AssistantEditKind {
    Update {
        old_text: String,
        new_text: String,
        description: Option<String>,
    },
    Create {
        new_text: String,
        description: Option<String>,
    },
    InsertBefore {
        old_text: String,
        new_text: String,
        description: Option<String>,
    },
    InsertAfter {
        old_text: String,
        new_text: String,
        description: Option<String>,
    },
    Delete {
        old_text: String,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResolvedPatch {
    pub edit_groups: HashMap<Entity<Buffer>, Vec<ResolvedEditGroup>>,
    pub errors: Vec<AssistantPatchResolutionError>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResolvedEditGroup {
    pub context_range: Range<language::Anchor>,
    pub edits: Vec<ResolvedEdit>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResolvedEdit {
    range: Range<language::Anchor>,
    new_text: String,
    description: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AssistantPatchResolutionError {
    pub edit_ix: usize,
    pub message: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum SearchDirection {
    Up,
    Left,
    Diagonal,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct SearchState {
    cost: u32,
    direction: SearchDirection,
}

impl SearchState {
    fn new(cost: u32, direction: SearchDirection) -> Self {
        Self { cost, direction }
    }
}

struct SearchMatrix {
    cols: usize,
    data: Vec<SearchState>,
}

impl SearchMatrix {
    fn new(rows: usize, cols: usize) -> Self {
        SearchMatrix {
            cols,
            data: vec![SearchState::new(0, SearchDirection::Diagonal); rows * cols],
        }
    }

    fn get(&self, row: usize, col: usize) -> SearchState {
        self.data[row * self.cols + col]
    }

    fn set(&mut self, row: usize, col: usize, cost: SearchState) {
        self.data[row * self.cols + col] = cost;
    }
}

impl ResolvedPatch {
    pub fn apply(&self, editor: &ProposedChangesEditor, cx: &mut App) {
        for (buffer, groups) in &self.edit_groups {
            let branch = editor.branch_buffer_for_base(buffer).unwrap();
            Self::apply_edit_groups(groups, &branch, cx);
        }
        editor.recalculate_all_buffer_diffs();
    }

    fn apply_edit_groups(groups: &Vec<ResolvedEditGroup>, buffer: &Entity<Buffer>, cx: &mut App) {
        let mut edits = Vec::new();
        for group in groups {
            for suggestion in &group.edits {
                edits.push((suggestion.range.clone(), suggestion.new_text.clone()));
            }
        }
        buffer.update(cx, |buffer, cx| {
            buffer.edit(
                edits,
                Some(AutoindentMode::Block {
                    original_start_columns: Vec::new(),
                }),
                cx,
            );
        });
    }
}

impl ResolvedEdit {
    pub fn try_merge(&mut self, other: &Self, buffer: &text::BufferSnapshot) -> bool {
        let range = &self.range;
        let other_range = &other.range;

        // Don't merge if we don't contain the other suggestion.
        if range.start.cmp(&other_range.start, buffer).is_gt()
            || range.end.cmp(&other_range.end, buffer).is_lt()
        {
            return false;
        }

        let other_offset_range = other_range.to_offset(buffer);
        let offset_range = range.to_offset(buffer);

        // If the other range is empty at the start of this edit's range, combine the new text
        if other_offset_range.is_empty() && other_offset_range.start == offset_range.start {
            self.new_text = format!("{}\n{}", other.new_text, self.new_text);
            self.range.start = other_range.start;

            if let Some((description, other_description)) =
                self.description.as_mut().zip(other.description.as_ref())
            {
                *description = format!("{}\n{}", other_description, description)
            }
        } else {
            if let Some((description, other_description)) =
                self.description.as_mut().zip(other.description.as_ref())
            {
                description.push('\n');
                description.push_str(other_description);
            }
        }

        true
    }
}

impl AssistantEdit {
    pub fn new(
        path: Option<String>,
        operation: Option<String>,
        old_text: Option<String>,
        new_text: Option<String>,
        description: Option<String>,
    ) -> Result<Self> {
        let path = path.ok_or_else(|| anyhow!("missing path"))?;
        let operation = operation.ok_or_else(|| anyhow!("missing operation"))?;

        let kind = match operation.as_str() {
            "update" => AssistantEditKind::Update {
                old_text: old_text.ok_or_else(|| anyhow!("missing old_text"))?,
                new_text: new_text.ok_or_else(|| anyhow!("missing new_text"))?,
                description,
            },
            "insert_before" => AssistantEditKind::InsertBefore {
                old_text: old_text.ok_or_else(|| anyhow!("missing old_text"))?,
                new_text: new_text.ok_or_else(|| anyhow!("missing new_text"))?,
                description,
            },
            "insert_after" => AssistantEditKind::InsertAfter {
                old_text: old_text.ok_or_else(|| anyhow!("missing old_text"))?,
                new_text: new_text.ok_or_else(|| anyhow!("missing new_text"))?,
                description,
            },
            "delete" => AssistantEditKind::Delete {
                old_text: old_text.ok_or_else(|| anyhow!("missing old_text"))?,
            },
            "create" => AssistantEditKind::Create {
                description,
                new_text: new_text.ok_or_else(|| anyhow!("missing new_text"))?,
            },
            _ => Err(anyhow!("unknown operation {operation:?}"))?,
        };

        Ok(Self { path, kind })
    }

    pub async fn resolve(
        &self,
        project: Entity<Project>,
        mut cx: AsyncApp,
    ) -> Result<(Entity<Buffer>, ResolvedEdit)> {
        let path = self.path.clone();
        let kind = self.kind.clone();
        let buffer = project
            .update(&mut cx, |project, cx| {
                let project_path = project
                    .find_project_path(Path::new(&path), cx)
                    .or_else(|| {
                        // If we couldn't find a project path for it, put it in the active worktree
                        // so that when we create the buffer, it can be saved.
                        let worktree = project
                            .active_entry()
                            .and_then(|entry_id| project.worktree_for_entry(entry_id, cx))
                            .or_else(|| project.worktrees(cx).next())?;
                        let worktree = worktree.read(cx);

                        Some(ProjectPath {
                            worktree_id: worktree.id(),
                            path: Arc::from(Path::new(&path)),
                        })
                    })
                    .with_context(|| format!("worktree not found for {:?}", path))?;
                anyhow::Ok(project.open_buffer(project_path, cx))
            })??
            .await?;

        let snapshot = buffer.update(&mut cx, |buffer, _| buffer.snapshot())?;
        let suggestion = cx
            .background_spawn(async move { kind.resolve(&snapshot) })
            .await;

        Ok((buffer, suggestion))
    }
}

impl AssistantEditKind {
    fn resolve(self, snapshot: &BufferSnapshot) -> ResolvedEdit {
        match self {
            Self::Update {
                old_text,
                new_text,
                description,
            } => {
                let range = Self::resolve_location(&snapshot, &old_text);
                ResolvedEdit {
                    range,
                    new_text,
                    description,
                }
            }
            Self::Create {
                new_text,
                description,
            } => ResolvedEdit {
                range: text::Anchor::MIN..text::Anchor::MAX,
                description,
                new_text,
            },
            Self::InsertBefore {
                old_text,
                mut new_text,
                description,
            } => {
                let range = Self::resolve_location(&snapshot, &old_text);
                new_text.push('\n');
                ResolvedEdit {
                    range: range.start..range.start,
                    new_text,
                    description,
                }
            }
            Self::InsertAfter {
                old_text,
                mut new_text,
                description,
            } => {
                let range = Self::resolve_location(&snapshot, &old_text);
                new_text.insert(0, '\n');
                ResolvedEdit {
                    range: range.end..range.end,
                    new_text,
                    description,
                }
            }
            Self::Delete { old_text } => {
                let range = Self::resolve_location(&snapshot, &old_text);
                ResolvedEdit {
                    range,
                    new_text: String::new(),
                    description: None,
                }
            }
        }
    }

    fn resolve_location(buffer: &text::BufferSnapshot, search_query: &str) -> Range<text::Anchor> {
        const INSERTION_COST: u32 = 3;
        const DELETION_COST: u32 = 10;
        const WHITESPACE_INSERTION_COST: u32 = 1;
        const WHITESPACE_DELETION_COST: u32 = 1;

        let buffer_len = buffer.len();
        let query_len = search_query.len();
        let mut matrix = SearchMatrix::new(query_len + 1, buffer_len + 1);
        let mut leading_deletion_cost = 0_u32;
        for (row, query_byte) in search_query.bytes().enumerate() {
            let deletion_cost = if query_byte.is_ascii_whitespace() {
                WHITESPACE_DELETION_COST
            } else {
                DELETION_COST
            };

            leading_deletion_cost = leading_deletion_cost.saturating_add(deletion_cost);
            matrix.set(
                row + 1,
                0,
                SearchState::new(leading_deletion_cost, SearchDirection::Diagonal),
            );

            for (col, buffer_byte) in buffer.bytes_in_range(0..buffer.len()).flatten().enumerate() {
                let insertion_cost = if buffer_byte.is_ascii_whitespace() {
                    WHITESPACE_INSERTION_COST
                } else {
                    INSERTION_COST
                };

                let up = SearchState::new(
                    matrix.get(row, col + 1).cost.saturating_add(deletion_cost),
                    SearchDirection::Up,
                );
                let left = SearchState::new(
                    matrix.get(row + 1, col).cost.saturating_add(insertion_cost),
                    SearchDirection::Left,
                );
                let diagonal = SearchState::new(
                    if query_byte == *buffer_byte {
                        matrix.get(row, col).cost
                    } else {
                        matrix
                            .get(row, col)
                            .cost
                            .saturating_add(deletion_cost + insertion_cost)
                    },
                    SearchDirection::Diagonal,
                );
                matrix.set(row + 1, col + 1, up.min(left).min(diagonal));
            }
        }

        // Traceback to find the best match
        let mut best_buffer_end = buffer_len;
        let mut best_cost = u32::MAX;
        for col in 1..=buffer_len {
            let cost = matrix.get(query_len, col).cost;
            if cost < best_cost {
                best_cost = cost;
                best_buffer_end = col;
            }
        }

        let mut query_ix = query_len;
        let mut buffer_ix = best_buffer_end;
        while query_ix > 0 && buffer_ix > 0 {
            let current = matrix.get(query_ix, buffer_ix);
            match current.direction {
                SearchDirection::Diagonal => {
                    query_ix -= 1;
                    buffer_ix -= 1;
                }
                SearchDirection::Up => {
                    query_ix -= 1;
                }
                SearchDirection::Left => {
                    buffer_ix -= 1;
                }
            }
        }

        let mut start = buffer.offset_to_point(buffer.clip_offset(buffer_ix, Bias::Left));
        start.column = 0;
        let mut end = buffer.offset_to_point(buffer.clip_offset(best_buffer_end, Bias::Right));
        if end.column > 0 {
            end.column = buffer.line_len(end.row);
        }

        buffer.anchor_after(start)..buffer.anchor_before(end)
    }
}

impl AssistantPatch {
    pub async fn resolve(&self, project: Entity<Project>, cx: &mut AsyncApp) -> ResolvedPatch {
        let mut resolve_tasks = Vec::new();
        for (ix, edit) in self.edits.iter().enumerate() {
            if let Ok(edit) = edit.as_ref() {
                resolve_tasks.push(
                    edit.resolve(project.clone(), cx.clone())
                        .map_err(move |error| (ix, error)),
                );
            }
        }

        let edits = future::join_all(resolve_tasks).await;
        let mut errors = Vec::new();
        let mut edits_by_buffer = HashMap::default();
        for entry in edits {
            match entry {
                Ok((buffer, edit)) => {
                    edits_by_buffer
                        .entry(buffer)
                        .or_insert_with(Vec::new)
                        .push(edit);
                }
                Err((edit_ix, error)) => errors.push(AssistantPatchResolutionError {
                    edit_ix,
                    message: error.to_string(),
                }),
            }
        }

        // Expand the context ranges of each edit and group edits with overlapping context ranges.
        let mut edit_groups_by_buffer = HashMap::default();
        for (buffer, edits) in edits_by_buffer {
            if let Ok(snapshot) = buffer.update(cx, |buffer, _| buffer.text_snapshot()) {
                edit_groups_by_buffer.insert(buffer, Self::group_edits(edits, &snapshot));
            }
        }

        ResolvedPatch {
            edit_groups: edit_groups_by_buffer,
            errors,
        }
    }

    fn group_edits(
        mut edits: Vec<ResolvedEdit>,
        snapshot: &text::BufferSnapshot,
    ) -> Vec<ResolvedEditGroup> {
        let mut edit_groups = Vec::<ResolvedEditGroup>::new();
        // Sort edits by their range so that earlier, larger ranges come first
        edits.sort_by(|a, b| a.range.cmp(&b.range, &snapshot));

        // Merge overlapping edits
        edits.dedup_by(|a, b| b.try_merge(a, &snapshot));

        // Create context ranges for each edit
        for edit in edits {
            let context_range = {
                let edit_point_range = edit.range.to_point(&snapshot);
                let start_row = edit_point_range.start.row.saturating_sub(5);
                let end_row = cmp::min(edit_point_range.end.row + 5, snapshot.max_point().row);
                let start = snapshot.anchor_before(Point::new(start_row, 0));
                let end = snapshot.anchor_after(Point::new(end_row, snapshot.line_len(end_row)));
                start..end
            };

            if let Some(last_group) = edit_groups.last_mut() {
                if last_group
                    .context_range
                    .end
                    .cmp(&context_range.start, &snapshot)
                    .is_ge()
                {
                    // Merge with the previous group if context ranges overlap
                    last_group.context_range.end = context_range.end;
                    last_group.edits.push(edit);
                } else {
                    // Create a new group
                    edit_groups.push(ResolvedEditGroup {
                        context_range,
                        edits: vec![edit],
                    });
                }
            } else {
                // Create the first group
                edit_groups.push(ResolvedEditGroup {
                    context_range,
                    edits: vec![edit],
                });
            }
        }

        edit_groups
    }

    pub fn path_count(&self) -> usize {
        self.paths().count()
    }

    pub fn paths(&self) -> impl '_ + Iterator<Item = &str> {
        let mut prev_path = None;
        self.edits.iter().filter_map(move |edit| {
            if let Ok(edit) = edit {
                let path = Some(edit.path.as_str());
                if path != prev_path {
                    prev_path = path;
                    return path;
                }
            }
            None
        })
    }
}

impl PartialEq for AssistantPatch {
    fn eq(&self, other: &Self) -> bool {
        self.range == other.range
            && self.title == other.title
            && Arc::ptr_eq(&self.edits, &other.edits)
    }
}

impl Eq for AssistantPatch {}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::App;
    use language::{
        language_settings::AllLanguageSettings, Language, LanguageConfig, LanguageMatcher,
    };
    use settings::SettingsStore;
    use ui::BorrowAppContext;
    use unindent::Unindent as _;
    use util::test::{generate_marked_text, marked_text_ranges};

    #[gpui::test]
    fn test_resolve_location(cx: &mut App) {
        assert_location_resolution(
            concat!(
                "    Lorem\n",
                "«    ipsum\n",
                "    dolor sit amet»\n",
                "    consecteur",
            ),
            "ipsum\ndolor",
            cx,
        );

        assert_location_resolution(
            &"
            «fn foo1(a: usize) -> usize {
                40
            }»

            fn foo2(b: usize) -> usize {
                42
            }
            "
            .unindent(),
            "fn foo1(b: usize) {\n40\n}",
            cx,
        );

        assert_location_resolution(
            &"
            fn main() {
            «    Foo
                    .bar()
                    .baz()
                    .qux()»
            }

            fn foo2(b: usize) -> usize {
                42
            }
            "
            .unindent(),
            "Foo.bar.baz.qux()",
            cx,
        );

        assert_location_resolution(
            &"
            class Something {
                one() { return 1; }
            «    two() { return 2222; }
                three() { return 333; }
                four() { return 4444; }
                five() { return 5555; }
                six() { return 6666; }
            »    seven() { return 7; }
                eight() { return 8; }
            }
            "
            .unindent(),
            &"
                two() { return 2222; }
                four() { return 4444; }
                five() { return 5555; }
                six() { return 6666; }
            "
            .unindent(),
            cx,
        );
    }

    #[gpui::test]
    fn test_resolve_edits(cx: &mut App) {
        init_test(cx);

        assert_edits(
            "
                /// A person
                struct Person {
                    name: String,
                    age: usize,
                }

                /// A dog
                struct Dog {
                    weight: f32,
                }

                impl Person {
                    fn name(&self) -> &str {
                        &self.name
                    }
                }
            "
            .unindent(),
            vec![
                AssistantEditKind::Update {
                    old_text: "
                        name: String,
                    "
                    .unindent(),
                    new_text: "
                        first_name: String,
                        last_name: String,
                    "
                    .unindent(),
                    description: None,
                },
                AssistantEditKind::Update {
                    old_text: "
                        fn name(&self) -> &str {
                            &self.name
                        }
                    "
                    .unindent(),
                    new_text: "
                        fn name(&self) -> String {
                            format!(\"{} {}\", self.first_name, self.last_name)
                        }
                    "
                    .unindent(),
                    description: None,
                },
            ],
            "
                /// A person
                struct Person {
                    first_name: String,
                    last_name: String,
                    age: usize,
                }

                /// A dog
                struct Dog {
                    weight: f32,
                }

                impl Person {
                    fn name(&self) -> String {
                        format!(\"{} {}\", self.first_name, self.last_name)
                    }
                }
            "
            .unindent(),
            cx,
        );

        // Ensure InsertBefore merges correctly with Update of the same text
        assert_edits(
            "
                fn foo() {

                }
            "
            .unindent(),
            vec![
                AssistantEditKind::InsertBefore {
                    old_text: "
                        fn foo() {"
                        .unindent(),
                    new_text: "
                        fn bar() {
                            qux();
                        }"
                    .unindent(),
                    description: Some("implement bar".into()),
                },
                AssistantEditKind::Update {
                    old_text: "
                        fn foo() {

                        }"
                    .unindent(),
                    new_text: "
                        fn foo() {
                            bar();
                        }"
                    .unindent(),
                    description: Some("call bar in foo".into()),
                },
                AssistantEditKind::InsertAfter {
                    old_text: "
                        fn foo() {

                        }
                    "
                    .unindent(),
                    new_text: "
                        fn qux() {
                            // todo
                        }
                    "
                    .unindent(),
                    description: Some("implement qux".into()),
                },
            ],
            "
                fn bar() {
                    qux();
                }

                fn foo() {
                    bar();
                }

                fn qux() {
                    // todo
                }
            "
            .unindent(),
            cx,
        );

        // Correctly indent new text when replacing multiple adjacent indented blocks.
        assert_edits(
            "
            impl Numbers {
                fn one() {
                    1
                }

                fn two() {
                    2
                }

                fn three() {
                    3
                }
            }
            "
            .unindent(),
            vec![
                AssistantEditKind::Update {
                    old_text: "
                        fn one() {
                            1
                        }
                    "
                    .unindent(),
                    new_text: "
                        fn one() {
                            101
                        }
                    "
                    .unindent(),
                    description: None,
                },
                AssistantEditKind::Update {
                    old_text: "
                        fn two() {
                            2
                        }
                    "
                    .unindent(),
                    new_text: "
                        fn two() {
                            102
                        }
                    "
                    .unindent(),
                    description: None,
                },
                AssistantEditKind::Update {
                    old_text: "
                        fn three() {
                            3
                        }
                    "
                    .unindent(),
                    new_text: "
                        fn three() {
                            103
                        }
                    "
                    .unindent(),
                    description: None,
                },
            ],
            "
                impl Numbers {
                    fn one() {
                        101
                    }

                    fn two() {
                        102
                    }

                    fn three() {
                        103
                    }
                }
            "
            .unindent(),
            cx,
        );

        assert_edits(
            "
            impl Person {
                fn set_name(&mut self, name: String) {
                    self.name = name;
                }

                fn name(&self) -> String {
                    return self.name;
                }
            }
            "
            .unindent(),
            vec![
                AssistantEditKind::Update {
                    old_text: "self.name = name;".unindent(),
                    new_text: "self._name = name;".unindent(),
                    description: None,
                },
                AssistantEditKind::Update {
                    old_text: "return self.name;\n".unindent(),
                    new_text: "return self._name;\n".unindent(),
                    description: None,
                },
            ],
            "
                impl Person {
                    fn set_name(&mut self, name: String) {
                        self._name = name;
                    }

                    fn name(&self) -> String {
                        return self._name;
                    }
                }
            "
            .unindent(),
            cx,
        );
    }

    fn init_test(cx: &mut App) {
        let settings_store = SettingsStore::test(cx);
        cx.set_global(settings_store);
        language::init(cx);
        cx.update_global::<SettingsStore, _>(|settings, cx| {
            settings.update_user_settings::<AllLanguageSettings>(cx, |_| {});
        });
    }

    #[track_caller]
    fn assert_location_resolution(text_with_expected_range: &str, query: &str, cx: &mut App) {
        let (text, _) = marked_text_ranges(text_with_expected_range, false);
        let buffer = cx.new(|cx| Buffer::local(text.clone(), cx));
        let snapshot = buffer.read(cx).snapshot();
        let range = AssistantEditKind::resolve_location(&snapshot, query).to_offset(&snapshot);
        let text_with_actual_range = generate_marked_text(&text, &[range], false);
        pretty_assertions::assert_eq!(text_with_actual_range, text_with_expected_range);
    }

    #[track_caller]
    fn assert_edits(
        old_text: String,
        edits: Vec<AssistantEditKind>,
        new_text: String,
        cx: &mut App,
    ) {
        let buffer =
            cx.new(|cx| Buffer::local(old_text, cx).with_language(Arc::new(rust_lang()), cx));
        let snapshot = buffer.read(cx).snapshot();
        let resolved_edits = edits
            .into_iter()
            .map(|kind| kind.resolve(&snapshot))
            .collect();
        let edit_groups = AssistantPatch::group_edits(resolved_edits, &snapshot);
        ResolvedPatch::apply_edit_groups(&edit_groups, &buffer, cx);
        let actual_new_text = buffer.read(cx).text();
        pretty_assertions::assert_eq!(actual_new_text, new_text);
    }

    fn rust_lang() -> Language {
        Language::new(
            LanguageConfig {
                name: "Rust".into(),
                matcher: LanguageMatcher {
                    path_suffixes: vec!["rs".to_string()],
                    ..Default::default()
                },
                ..Default::default()
            },
            Some(language::tree_sitter_rust::LANGUAGE.into()),
        )
        .with_indents_query(
            r#"
            (call_expression) @indent
            (field_expression) @indent
            (_ "(" ")" @end) @indent
            (_ "{" "}" @end) @indent
            "#,
        )
        .unwrap()
    }
}
