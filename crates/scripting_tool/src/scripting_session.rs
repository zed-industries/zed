use anyhow::anyhow;
use buffer_diff::BufferDiff;
use collections::{HashMap, HashSet};
use futures::{
    channel::{mpsc, oneshot},
    pin_mut, SinkExt, StreamExt,
};
use gpui::{AppContext, AsyncApp, Context, Entity, Task, WeakEntity};
use language::Buffer;
use mlua::{ExternalResult, Lua, MultiValue, ObjectLike, Table, UserData, UserDataMethods};
use parking_lot::Mutex;
use project::{search::SearchQuery, Fs, Project, ProjectPath, WorktreeId};
use regex::Regex;
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};
use util::{paths::PathMatcher, ResultExt};

struct ForegroundFn(Box<dyn FnOnce(WeakEntity<ScriptingSession>, AsyncApp) + Send>);

struct BufferChanges {
    diff: Entity<BufferDiff>,
    edit_ids: Vec<clock::Lamport>,
}

pub struct ScriptingSession {
    project: Entity<Project>,
    scripts: Vec<Script>,
    changes_by_buffer: HashMap<Entity<Buffer>, BufferChanges>,
    foreground_fns_tx: mpsc::Sender<ForegroundFn>,
    _invoke_foreground_fns: Task<()>,
}

impl ScriptingSession {
    pub fn new(project: Entity<Project>, cx: &mut Context<Self>) -> Self {
        let (foreground_fns_tx, mut foreground_fns_rx) = mpsc::channel(128);
        ScriptingSession {
            project,
            scripts: Vec::new(),
            changes_by_buffer: HashMap::default(),
            foreground_fns_tx,
            _invoke_foreground_fns: cx.spawn(|this, cx| async move {
                while let Some(foreground_fn) = foreground_fns_rx.next().await {
                    foreground_fn.0(this.clone(), cx.clone());
                }
            }),
        }
    }

    pub fn changed_buffers(&self) -> impl ExactSizeIterator<Item = &Entity<Buffer>> {
        self.changes_by_buffer.keys()
    }

    pub fn run_script(
        &mut self,
        script_src: String,
        cx: &mut Context<Self>,
    ) -> (ScriptId, Task<()>) {
        let id = ScriptId(self.scripts.len() as u32);

        let stdout = Arc::new(Mutex::new(String::new()));

        let script = Script {
            state: ScriptState::Running {
                stdout: stdout.clone(),
            },
        };
        self.scripts.push(script);

        let task = self.run_lua(script_src, stdout, cx);

        let task = cx.spawn(|session, mut cx| async move {
            let result = task.await;

            session
                .update(&mut cx, |session, _cx| {
                    let script = session.get_mut(id);
                    let stdout = script.stdout_snapshot();

                    script.state = match result {
                        Ok(()) => ScriptState::Succeeded { stdout },
                        Err(error) => ScriptState::Failed { stdout, error },
                    };
                })
                .log_err();
        });

        (id, task)
    }

    fn run_lua(
        &mut self,
        script: String,
        stdout: Arc<Mutex<String>>,
        cx: &mut Context<Self>,
    ) -> Task<anyhow::Result<()>> {
        const SANDBOX_PREAMBLE: &str = include_str!("sandbox_preamble.lua");

        // TODO Honor all worktrees instead of the first one
        let worktree_info = self
            .project
            .read(cx)
            .visible_worktrees(cx)
            .next()
            .map(|worktree| {
                let worktree = worktree.read(cx);
                (worktree.id(), worktree.abs_path())
            });

        let root_dir = worktree_info.as_ref().map(|(_, root)| root.clone());

        let fs = self.project.read(cx).fs().clone();
        let foreground_fns_tx = self.foreground_fns_tx.clone();

        let task = cx.background_spawn({
            let stdout = stdout.clone();

            async move {
                let lua = Lua::new();
                lua.set_memory_limit(2 * 1024 * 1024 * 1024)?; // 2 GB
                let globals = lua.globals();

                // Use the project root dir as the script's current working dir.
                if let Some(root_dir) = &root_dir {
                    if let Some(root_dir) = root_dir.to_str() {
                        globals.set("cwd", root_dir)?;
                    }
                }

                globals.set(
                    "sb_print",
                    lua.create_function({
                        let stdout = stdout.clone();
                        move |_, args: MultiValue| Self::print(args, &stdout)
                    })?,
                )?;
                globals.set(
                    "search",
                    lua.create_async_function({
                        let foreground_fns_tx = foreground_fns_tx.clone();
                        let fs = fs.clone();
                        move |lua, regex| {
                            let mut foreground_fns_tx = foreground_fns_tx.clone();
                            let fs = fs.clone();
                            async move {
                                Self::search(&lua, &mut foreground_fns_tx, fs, regex)
                                    .await
                                    .into_lua_err()
                            }
                        }
                    })?,
                )?;
                globals.set(
                    "outline",
                    lua.create_async_function({
                        let root_dir = root_dir.clone();
                        let foreground_fns_tx = foreground_fns_tx.clone();
                        move |_lua, path| {
                            let mut foreground_fns_tx = foreground_fns_tx.clone();
                            let root_dir = root_dir.clone();
                            async move {
                                Self::outline(root_dir, &mut foreground_fns_tx, path)
                                    .await
                                    .into_lua_err()
                            }
                        }
                    })?,
                )?;
                globals.set(
                    "sb_io_open",
                    lua.create_async_function({
                        let worktree_info = worktree_info.clone();
                        let foreground_fns_tx = foreground_fns_tx.clone();
                        move |lua, (path_str, mode)| {
                            let worktree_info = worktree_info.clone();
                            let mut foreground_fns_tx = foreground_fns_tx.clone();
                            let fs = fs.clone();
                            async move {
                                Self::io_open(
                                    &lua,
                                    worktree_info,
                                    &mut foreground_fns_tx,
                                    fs,
                                    path_str,
                                    mode,
                                )
                                .await
                            }
                        }
                    })?,
                )?;
                globals.set("user_script", script)?;

                lua.load(SANDBOX_PREAMBLE).exec_async().await?;

                anyhow::Ok(())
            }
        });

        task
    }

    pub fn get(&self, script_id: ScriptId) -> &Script {
        &self.scripts[script_id.0 as usize]
    }

    fn get_mut(&mut self, script_id: ScriptId) -> &mut Script {
        &mut self.scripts[script_id.0 as usize]
    }

    /// Sandboxed print() function in Lua.
    fn print(args: MultiValue, stdout: &Mutex<String>) -> mlua::Result<()> {
        for (index, arg) in args.into_iter().enumerate() {
            // Lua's `print()` prints tab characters between each argument.
            if index > 0 {
                stdout.lock().push('\t');
            }

            // If the argument's to_string() fails, have the whole function call fail.
            stdout.lock().push_str(&arg.to_string()?);
        }
        stdout.lock().push('\n');

        Ok(())
    }

    /// Sandboxed io.open() function in Lua.
    async fn io_open(
        lua: &Lua,
        worktree_info: Option<(WorktreeId, Arc<Path>)>,
        foreground_tx: &mut mpsc::Sender<ForegroundFn>,
        fs: Arc<dyn Fs>,
        path_str: String,
        mode: Option<String>,
    ) -> mlua::Result<(Option<Table>, String)> {
        let (worktree_id, root_dir) = worktree_info
            .ok_or_else(|| mlua::Error::runtime("cannot open file without a root directory"))?;

        let mode = mode.unwrap_or_else(|| "r".to_string());

        // Parse the mode string to determine read/write permissions
        let read_perm = mode.contains('r');
        let write_perm = mode.contains('w') || mode.contains('a') || mode.contains('+');
        let append = mode.contains('a');
        let truncate = mode.contains('w');

        // This will be the Lua value returned from the `open` function.
        let file = lua.create_table()?;

        // Store file metadata in the file
        file.set("__mode", mode.clone())?;
        file.set("__read_perm", read_perm)?;
        file.set("__write_perm", write_perm)?;

        let path = match Self::parse_abs_path_in_root_dir(&root_dir, &path_str) {
            Ok(path) => path,
            Err(err) => return Ok((None, format!("{err}"))),
        };

        let project_path = ProjectPath {
            worktree_id,
            path: Path::new(&path_str).into(),
        };

        // flush / close method
        let flush_fn = {
            let project_path = project_path.clone();
            let foreground_tx = foreground_tx.clone();
            lua.create_async_function(move |_lua, file_userdata: mlua::Table| {
                let project_path = project_path.clone();
                let mut foreground_tx = foreground_tx.clone();
                async move {
                    Self::io_file_flush(file_userdata, project_path, &mut foreground_tx).await
                }
            })?
        };
        file.set("flush", flush_fn.clone())?;
        // We don't really hold files open, so we only need to flush on close
        file.set("close", flush_fn)?;

        // If it's a directory, give it a custom read() and return early.
        if fs.is_dir(&path).await {
            return Self::io_file_dir(lua, fs, file, &path).await;
        }

        let mut file_content = Vec::new();

        if !truncate {
            // Try to read existing content if we're not truncating
            match Self::read_buffer(project_path.clone(), foreground_tx).await {
                Ok(content) => file_content = content.into_bytes(),
                Err(e) => return Ok((None, format!("Error reading file: {}", e))),
            }
        }

        // If in append mode, position should be at the end
        let position = if append { file_content.len() } else { 0 };
        file.set("__position", position)?;
        file.set(
            "__content",
            lua.create_userdata(FileContent(Arc::new(Mutex::new(file_content))))?,
        )?;

        // Create file methods

        // read method
        let read_fn = lua.create_function(Self::io_file_read)?;
        file.set("read", read_fn)?;

        // lines method
        let lines_fn = lua.create_function(Self::io_file_lines)?;
        file.set("lines", lines_fn)?;

        // write method
        let write_fn = lua.create_function(Self::io_file_write)?;
        file.set("write", write_fn)?;

        // If we got this far, the file was opened successfully
        Ok((Some(file), String::new()))
    }

    async fn read_buffer(
        project_path: ProjectPath,
        foreground_tx: &mut mpsc::Sender<ForegroundFn>,
    ) -> anyhow::Result<String> {
        Self::run_foreground_fn(
            "read file from buffer",
            foreground_tx,
            Box::new(move |session, mut cx| {
                session.update(&mut cx, |session, cx| {
                    let open_buffer_task = session
                        .project
                        .update(cx, |project, cx| project.open_buffer(project_path, cx));

                    cx.spawn(|_, cx| async move {
                        let buffer = open_buffer_task.await?;

                        let text = buffer.read_with(&cx, |buffer, _cx| buffer.text())?;
                        Ok(text)
                    })
                })
            }),
        )
        .await??
        .await
    }

    async fn io_file_flush(
        file_userdata: mlua::Table,
        project_path: ProjectPath,
        foreground_tx: &mut mpsc::Sender<ForegroundFn>,
    ) -> mlua::Result<bool> {
        let write_perm = file_userdata.get::<bool>("__write_perm")?;

        if write_perm {
            let content = file_userdata.get::<mlua::AnyUserData>("__content")?;
            let content_ref = content.borrow::<FileContent>()?;
            let text = {
                let mut content_vec = content_ref.0.lock();
                let content_vec = std::mem::take(&mut *content_vec);
                String::from_utf8(content_vec).into_lua_err()?
            };

            Self::write_to_buffer(project_path, text, foreground_tx)
                .await
                .into_lua_err()?;
        }

        Ok(true)
    }

    async fn write_to_buffer(
        project_path: ProjectPath,
        text: String,
        foreground_tx: &mut mpsc::Sender<ForegroundFn>,
    ) -> anyhow::Result<()> {
        Self::run_foreground_fn(
            "write to buffer",
            foreground_tx,
            Box::new(move |session, mut cx| {
                session.update(&mut cx, |session, cx| {
                    let open_buffer_task = session
                        .project
                        .update(cx, |project, cx| project.open_buffer(project_path, cx));

                    cx.spawn(move |session, mut cx| async move {
                        let buffer = open_buffer_task.await?;

                        let diff = buffer
                            .update(&mut cx, |buffer, cx| buffer.diff(text, cx))?
                            .await;

                        let edit_ids = buffer.update(&mut cx, |buffer, cx| {
                            buffer.finalize_last_transaction();
                            buffer.apply_diff(diff, cx);
                            let transaction = buffer.finalize_last_transaction();
                            transaction
                                .map_or(Vec::new(), |transaction| transaction.edit_ids.clone())
                        })?;

                        session
                            .update(&mut cx, {
                                let buffer = buffer.clone();

                                |session, cx| {
                                    session
                                        .project
                                        .update(cx, |project, cx| project.save_buffer(buffer, cx))
                                }
                            })?
                            .await?;

                        let snapshot = buffer.read_with(&cx, |buffer, _| buffer.snapshot())?;

                        // If we saved successfully, mark buffer as changed
                        let buffer_without_changes =
                            buffer.update(&mut cx, |buffer, cx| buffer.branch(cx))?;
                        session
                            .update(&mut cx, |session, cx| {
                                let changed_buffer = session
                                    .changes_by_buffer
                                    .entry(buffer)
                                    .or_insert_with(|| BufferChanges {
                                        diff: cx.new(|cx| BufferDiff::new(&snapshot, cx)),
                                        edit_ids: Vec::new(),
                                    });
                                changed_buffer.edit_ids.extend(edit_ids);
                                let operations_to_undo = changed_buffer
                                    .edit_ids
                                    .iter()
                                    .map(|edit_id| (*edit_id, u32::MAX))
                                    .collect::<HashMap<_, _>>();
                                buffer_without_changes.update(cx, |buffer, cx| {
                                    buffer.undo_operations(operations_to_undo, cx);
                                });
                                changed_buffer.diff.update(cx, |diff, cx| {
                                    diff.set_base_text(buffer_without_changes, snapshot.text, cx)
                                })
                            })?
                            .await?;

                        Ok(())
                    })
                })
            }),
        )
        .await??
        .await
    }

    async fn io_file_dir(
        lua: &Lua,
        fs: Arc<dyn Fs>,
        file: Table,
        path: &Path,
    ) -> mlua::Result<(Option<Table>, String)> {
        // Create a special directory handle
        file.set("__is_directory", true)?;

        // Store directory entries
        let entries = match fs.read_dir(&path).await {
            Ok(entries) => {
                let mut entry_names = Vec::new();

                // Process the stream of directory entries
                pin_mut!(entries);
                while let Some(Ok(entry_result)) = entries.next().await {
                    if let Some(file_name) = entry_result.file_name() {
                        entry_names.push(file_name.to_string_lossy().into_owned());
                    }
                }

                entry_names
            }
            Err(e) => return Ok((None, format!("Error reading directory: {}", e))),
        };

        // Save the list of entries
        file.set("__dir_entries", entries)?;
        file.set("__dir_position", 0usize)?;

        // Create a directory-specific read function
        let read_fn = lua.create_function(|_lua, file_userdata: mlua::Table| {
            let position = file_userdata.get::<usize>("__dir_position")?;
            let entries = file_userdata.get::<Vec<String>>("__dir_entries")?;

            if position >= entries.len() {
                return Ok(None); // No more entries
            }

            let entry = entries[position].clone();
            file_userdata.set("__dir_position", position + 1)?;

            Ok(Some(entry))
        })?;
        file.set("read", read_fn)?;

        // If we got this far, the directory was opened successfully
        return Ok((Some(file), String::new()));
    }

    fn io_file_read(
        lua: &Lua,
        (file_userdata, format): (Table, Option<mlua::Value>),
    ) -> mlua::Result<Option<mlua::String>> {
        let read_perm = file_userdata.get::<bool>("__read_perm")?;
        if !read_perm {
            return Err(mlua::Error::runtime("File not open for reading"));
        }

        let content = file_userdata.get::<mlua::AnyUserData>("__content")?;
        let position = file_userdata.get::<usize>("__position")?;
        let content_ref = content.borrow::<FileContent>()?;
        let content = content_ref.0.lock();

        if position >= content.len() {
            return Ok(None); // EOF
        }

        let (result, new_position) = match Self::io_file_read_format(format)? {
            FileReadFormat::All => {
                // Read entire file from current position
                let result = content[position..].to_vec();
                (Some(result), content.len())
            }
            FileReadFormat::Line => {
                if let Some(next_newline_ix) = content[position..].iter().position(|c| *c == b'\n')
                {
                    let mut line = content[position..position + next_newline_ix].to_vec();
                    if line.ends_with(b"\r") {
                        line.pop();
                    }
                    (Some(line), position + next_newline_ix + 1)
                } else if position < content.len() {
                    let line = content[position..].to_vec();
                    (Some(line), content.len())
                } else {
                    (None, position) // EOF
                }
            }
            FileReadFormat::LineWithLineFeed => {
                if position < content.len() {
                    let next_line_ix = content[position..]
                        .iter()
                        .position(|c| *c == b'\n')
                        .map_or(content.len(), |ix| position + ix + 1);
                    let line = content[position..next_line_ix].to_vec();
                    (Some(line), next_line_ix)
                } else {
                    (None, position) // EOF
                }
            }
            FileReadFormat::Bytes(n) => {
                let end = std::cmp::min(position + n, content.len());
                let result = content[position..end].to_vec();
                (Some(result), end)
            }
        };

        // Update the position in the file userdata
        if new_position != position {
            file_userdata.set("__position", new_position)?;
        }

        // Convert the result to a Lua string
        match result {
            Some(bytes) => Ok(Some(lua.create_string(bytes)?)),
            None => Ok(None),
        }
    }

    fn io_file_lines(lua: &Lua, file_userdata: Table) -> mlua::Result<mlua::Function> {
        let read_perm = file_userdata.get::<bool>("__read_perm")?;
        if !read_perm {
            return Err(mlua::Error::runtime("File not open for reading"));
        }

        lua.create_function::<_, _, mlua::Value>(move |lua, _: ()| {
            file_userdata.call_method("read", lua.create_string("*l")?)
        })
    }

    fn io_file_read_format(format: Option<mlua::Value>) -> mlua::Result<FileReadFormat> {
        let format = match format {
            Some(mlua::Value::String(s)) => {
                let lossy_string = s.to_string_lossy();
                let format_str: &str = lossy_string.as_ref();

                // Only consider the first 2 bytes, since it's common to pass e.g. "*all"  instead of "*a"
                match &format_str[0..2] {
                    "*a" => FileReadFormat::All,
                    "*l" => FileReadFormat::Line,
                    "*L" => FileReadFormat::LineWithLineFeed,
                    "*n" => {
                        // Try to parse as a number (number of bytes to read)
                        match format_str.parse::<usize>() {
                            Ok(n) => FileReadFormat::Bytes(n),
                            Err(_) => {
                                return Err(mlua::Error::runtime(format!(
                                    "Invalid format: {}",
                                    format_str
                                )))
                            }
                        }
                    }
                    _ => {
                        return Err(mlua::Error::runtime(format!(
                            "Unsupported format: {}",
                            format_str
                        )))
                    }
                }
            }
            Some(mlua::Value::Number(n)) => FileReadFormat::Bytes(n as usize),
            Some(mlua::Value::Integer(n)) => FileReadFormat::Bytes(n as usize),
            Some(value) => {
                return Err(mlua::Error::runtime(format!(
                    "Invalid file format {:?}",
                    value
                )))
            }
            None => FileReadFormat::Line, // Default is to read a line
        };

        Ok(format)
    }

    fn io_file_write(
        _lua: &Lua,
        (file_userdata, text): (Table, mlua::String),
    ) -> mlua::Result<bool> {
        let write_perm = file_userdata.get::<bool>("__write_perm")?;
        if !write_perm {
            return Err(mlua::Error::runtime("File not open for writing"));
        }

        let content = file_userdata.get::<mlua::AnyUserData>("__content")?;
        let position = file_userdata.get::<usize>("__position")?;
        let content_ref = content.borrow::<FileContent>()?;
        let mut content_vec = content_ref.0.lock();

        let bytes = text.as_bytes();

        // Ensure the vector has enough capacity
        if position + bytes.len() > content_vec.len() {
            content_vec.resize(position + bytes.len(), 0);
        }

        // Write the bytes
        for (i, &byte) in bytes.iter().enumerate() {
            content_vec[position + i] = byte;
        }

        // Update position
        let new_position = position + bytes.len();
        file_userdata.set("__position", new_position)?;

        Ok(true)
    }

    async fn search(
        lua: &Lua,
        foreground_tx: &mut mpsc::Sender<ForegroundFn>,
        fs: Arc<dyn Fs>,
        regex: String,
    ) -> anyhow::Result<Table> {
        // TODO: Allow specification of these options.
        let search_query = SearchQuery::regex(
            &regex,
            false,
            false,
            false,
            PathMatcher::default(),
            PathMatcher::default(),
            None,
        );
        let search_query = match search_query {
            Ok(query) => query,
            Err(e) => return Err(anyhow!("Invalid search query: {}", e)),
        };

        // TODO: Should use `search_query.regex`. The tool description should also be updated,
        // as it specifies standard regex.
        let search_regex = match Regex::new(&regex) {
            Ok(re) => re,
            Err(e) => return Err(anyhow!("Invalid regex: {}", e)),
        };

        let mut abs_paths_rx = Self::find_search_candidates(search_query, foreground_tx).await?;

        let mut search_results: Vec<Table> = Vec::new();
        while let Some(path) = abs_paths_rx.next().await {
            // Skip files larger than 1MB
            if let Ok(Some(metadata)) = fs.metadata(&path).await {
                if metadata.len > 1_000_000 {
                    continue;
                }
            }

            // Attempt to read the file as text
            if let Ok(content) = fs.load(&path).await {
                let mut matches = Vec::new();

                // Find all regex matches in the content
                for capture in search_regex.find_iter(&content) {
                    matches.push(capture.as_str().to_string());
                }

                // If we found matches, create a result entry
                if !matches.is_empty() {
                    let result_entry = lua.create_table()?;
                    result_entry.set("path", path.to_string_lossy().to_string())?;

                    let matches_table = lua.create_table()?;
                    for (ix, m) in matches.iter().enumerate() {
                        matches_table.set(ix + 1, m.clone())?;
                    }
                    result_entry.set("matches", matches_table)?;

                    search_results.push(result_entry);
                }
            }
        }

        // Create a table to hold our results
        let results_table = lua.create_table()?;
        for (ix, entry) in search_results.into_iter().enumerate() {
            results_table.set(ix + 1, entry)?;
        }

        Ok(results_table)
    }

    async fn find_search_candidates(
        search_query: SearchQuery,
        foreground_tx: &mut mpsc::Sender<ForegroundFn>,
    ) -> anyhow::Result<mpsc::UnboundedReceiver<PathBuf>> {
        Self::run_foreground_fn(
            "finding search file candidates",
            foreground_tx,
            Box::new(move |session, mut cx| {
                session.update(&mut cx, |session, cx| {
                    session.project.update(cx, |project, cx| {
                        project.worktree_store().update(cx, |worktree_store, cx| {
                            // TODO: Better limit? For now this is the same as
                            // MAX_SEARCH_RESULT_FILES.
                            let limit = 5000;
                            // TODO: Providing non-empty open_entries can make this a bit more
                            // efficient as it can skip checking that these paths are textual.
                            let open_entries = HashSet::default();
                            let candidates = worktree_store.find_search_candidates(
                                search_query,
                                limit,
                                open_entries,
                                project.fs().clone(),
                                cx,
                            );
                            let (abs_paths_tx, abs_paths_rx) = mpsc::unbounded();
                            cx.spawn(|worktree_store, cx| async move {
                                pin_mut!(candidates);

                                while let Some(project_path) = candidates.next().await {
                                    worktree_store.read_with(&cx, |worktree_store, cx| {
                                        if let Some(worktree) = worktree_store
                                            .worktree_for_id(project_path.worktree_id, cx)
                                        {
                                            if let Some(abs_path) = worktree
                                                .read(cx)
                                                .absolutize(&project_path.path)
                                                .log_err()
                                            {
                                                abs_paths_tx.unbounded_send(abs_path)?;
                                            }
                                        }
                                        anyhow::Ok(())
                                    })??;
                                }
                                anyhow::Ok(())
                            })
                            .detach();
                            abs_paths_rx
                        })
                    })
                })
            }),
        )
        .await?
    }

    async fn outline(
        root_dir: Option<Arc<Path>>,
        foreground_tx: &mut mpsc::Sender<ForegroundFn>,
        path_str: String,
    ) -> anyhow::Result<String> {
        let root_dir = root_dir
            .ok_or_else(|| mlua::Error::runtime("cannot get outline without a root directory"))?;
        let path = Self::parse_abs_path_in_root_dir(&root_dir, &path_str)?;
        let outline = Self::run_foreground_fn(
            "getting code outline",
            foreground_tx,
            Box::new(move |session, cx| {
                cx.spawn(move |mut cx| async move {
                    // TODO: This will not use file content from `fs_changes`. It will also reflect
                    // user changes that have not been saved.
                    let buffer = session
                        .update(&mut cx, |session, cx| {
                            session
                                .project
                                .update(cx, |project, cx| project.open_local_buffer(&path, cx))
                        })?
                        .await?;
                    buffer.update(&mut cx, |buffer, _cx| {
                        if let Some(outline) = buffer.snapshot().outline(None) {
                            Ok(outline)
                        } else {
                            Err(anyhow!("No outline for file {path_str}"))
                        }
                    })
                })
            }),
        )
        .await?
        .await??;

        Ok(outline
            .items
            .into_iter()
            .map(|item| {
                if item.text.contains('\n') {
                    log::error!("Outline item unexpectedly contains newline");
                }
                format!("{}{}", "  ".repeat(item.depth), item.text)
            })
            .collect::<Vec<String>>()
            .join("\n"))
    }

    async fn run_foreground_fn<R: Send + 'static>(
        description: &str,
        foreground_tx: &mut mpsc::Sender<ForegroundFn>,
        function: Box<dyn FnOnce(WeakEntity<Self>, AsyncApp) -> R + Send>,
    ) -> anyhow::Result<R> {
        let (response_tx, response_rx) = oneshot::channel();
        let send_result = foreground_tx
            .send(ForegroundFn(Box::new(move |this, cx| {
                response_tx.send(function(this, cx)).ok();
            })))
            .await;
        match send_result {
            Ok(()) => (),
            Err(err) => {
                return Err(anyhow::Error::new(err).context(format!(
                    "Internal error while enqueuing work for {description}"
                )));
            }
        }
        match response_rx.await {
            Ok(result) => Ok(result),
            Err(oneshot::Canceled) => Err(anyhow!(
                "Internal error: response oneshot was canceled while {description}."
            )),
        }
    }

    fn parse_abs_path_in_root_dir(root_dir: &Path, path_str: &str) -> anyhow::Result<PathBuf> {
        let path = Path::new(&path_str);
        if path.is_absolute() {
            // Check if path starts with root_dir prefix without resolving symlinks
            if path.starts_with(&root_dir) {
                Ok(path.to_path_buf())
            } else {
                Err(anyhow!(
                    "Error: Absolute path {} is outside the current working directory",
                    path_str
                ))
            }
        } else {
            // TODO: Does use of `../` break sandbox - is path canonicalization needed?
            Ok(root_dir.join(path))
        }
    }
}

enum FileReadFormat {
    All,
    Line,
    LineWithLineFeed,
    Bytes(usize),
}

struct FileContent(Arc<Mutex<Vec<u8>>>);

impl UserData for FileContent {
    fn add_methods<M: UserDataMethods<Self>>(_methods: &mut M) {
        // FileContent doesn't have any methods so far.
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ScriptId(u32);

pub struct Script {
    pub state: ScriptState,
}

#[derive(Debug)]
pub enum ScriptState {
    Running {
        stdout: Arc<Mutex<String>>,
    },
    Succeeded {
        stdout: String,
    },
    Failed {
        stdout: String,
        error: anyhow::Error,
    },
}

impl Script {
    /// If exited, returns a message with the output for the LLM
    pub fn output_message_for_llm(&self) -> Option<String> {
        match &self.state {
            ScriptState::Running { .. } => None,
            ScriptState::Succeeded { stdout } => {
                format!("Here's the script output:\n{}", stdout).into()
            }
            ScriptState::Failed { stdout, error } => format!(
                "The script failed with:\n{}\n\nHere's the output it managed to print:\n{}",
                error, stdout
            )
            .into(),
        }
    }

    /// Get a snapshot of the script's stdout
    pub fn stdout_snapshot(&self) -> String {
        match &self.state {
            ScriptState::Running { stdout } => stdout.lock().clone(),
            ScriptState::Succeeded { stdout } => stdout.clone(),
            ScriptState::Failed { stdout, .. } => stdout.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use gpui::TestAppContext;
    use project::FakeFs;
    use serde_json::json;
    use settings::SettingsStore;
    use util::path;

    use super::*;

    #[gpui::test]
    async fn test_print(cx: &mut TestAppContext) {
        let script = r#"
            print("Hello", "world!")
            print("Goodbye", "moon!")
        "#;

        let test_session = TestSession::init(cx).await;
        let output = test_session.test_success(script, cx).await;
        assert_eq!(output, "Hello\tworld!\nGoodbye\tmoon!\n");
    }

    // search

    #[gpui::test]
    async fn test_search(cx: &mut TestAppContext) {
        let script = r#"
            local results = search("world")
            for i, result in ipairs(results) do
                print("File: " .. result.path)
                print("Matches:")
                for j, match in ipairs(result.matches) do
                    print("  " .. match)
                end
            end
        "#;

        let test_session = TestSession::init(cx).await;
        let output = test_session.test_success(script, cx).await;
        assert_eq!(
            output,
            concat!("File: ", path!("/file1.txt"), "\nMatches:\n  world\n")
        );
    }

    // io.open

    #[gpui::test]
    async fn test_open_and_read_file(cx: &mut TestAppContext) {
        let script = r#"
            local file = io.open("file1.txt", "r")
            local content = file:read()
            print("Content:", content)
            file:close()
        "#;

        let test_session = TestSession::init(cx).await;
        let output = test_session.test_success(script, cx).await;
        assert_eq!(output, "Content:\tHello world!\n");
        assert_eq!(test_session.diff(cx), Vec::new());
    }

    #[gpui::test]
    async fn test_lines_iterator(cx: &mut TestAppContext) {
        let script = r#"
            -- Create a test file with multiple lines
            local file = io.open("lines_test.txt", "w")
            file:write("Line 1\nLine 2\nLine 3\nLine 4\nLine 5")
            file:close()

            -- Read it back using the lines iterator
            local read_file = io.open("lines_test.txt", "r")
            local count = 0
            for line in read_file:lines() do
                count = count + 1
                print(count .. ": " .. line)
            end
            read_file:close()

            print("Total lines:", count)
        "#;

        let test_session = TestSession::init(cx).await;
        let output = test_session.test_success(script, cx).await;
        assert_eq!(
            output,
            "1: Line 1\n2: Line 2\n3: Line 3\n4: Line 4\n5: Line 5\nTotal lines:\t5\n"
        );
    }

    #[gpui::test]
    async fn test_read_write_roundtrip(cx: &mut TestAppContext) {
        let script = r#"
            local file = io.open("file1.txt", "w")
            file:write("This is new content")
            file:close()

            -- Read back to verify
            local read_file = io.open("file1.txt", "r")
            local content = read_file:read("*a")
            print("Written content:", content)
            read_file:close()
        "#;

        let test_session = TestSession::init(cx).await;
        let output = test_session.test_success(script, cx).await;
        assert_eq!(output, "Written content:\tThis is new content\n");
        assert_eq!(
            test_session.diff(cx),
            vec![(
                PathBuf::from("file1.txt"),
                vec![(
                    "Hello world!\n".to_string(),
                    "This is new content".to_string()
                )]
            )]
        );
    }

    #[gpui::test]
    async fn test_multiple_writes(cx: &mut TestAppContext) {
        let script = r#"
            -- Test writing to a file multiple times
            local file = io.open("multiwrite.txt", "w")
            file:write("First line\n")
            file:write("Second line\n")
            file:write("Third line")
            file:close()

            -- Read back to verify
            local read_file = io.open("multiwrite.txt", "r")
            if read_file then
                local content = read_file:read("*a")
                print("Full content:", content)
                read_file:close()
            end
        "#;

        let test_session = TestSession::init(cx).await;
        let output = test_session.test_success(script, cx).await;
        assert_eq!(
            output,
            "Full content:\tFirst line\nSecond line\nThird line\n"
        );
        assert_eq!(
            test_session.diff(cx),
            vec![(
                PathBuf::from("multiwrite.txt"),
                vec![(
                    "".to_string(),
                    "First line\nSecond line\nThird line".to_string()
                )]
            )]
        );
    }

    #[gpui::test]
    async fn test_multiple_writes_diff_handles(cx: &mut TestAppContext) {
        let script = r#"
            -- Write to a file
            local file1 = io.open("multi_open.txt", "w")
            file1:write("Content written by first handle\n")
            file1:close()

            -- Open it again and add more content
            local file2 = io.open("multi_open.txt", "w")
            file2:write("Content written by second handle\n")
            file2:close()

            -- Open it a third time and read
            local file3 = io.open("multi_open.txt", "r")
            local content = file3:read("*a")
            print("Final content:", content)
            file3:close()
        "#;

        let test_session = TestSession::init(cx).await;
        let output = test_session.test_success(script, cx).await;
        assert_eq!(
            output,
            "Final content:\tContent written by second handle\n\n"
        );
        assert_eq!(
            test_session.diff(cx),
            vec![(
                PathBuf::from("multi_open.txt"),
                vec![(
                    "".to_string(),
                    "Content written by second handle\n".to_string()
                )]
            )]
        );
    }

    #[gpui::test]
    async fn test_append_mode(cx: &mut TestAppContext) {
        let script = r#"
            -- Append more content
            file = io.open("file1.txt", "a")
            file:write("Appended content\n")
            file:close()

            -- Add even more
            file = io.open("file1.txt", "a")
            file:write("More appended content")
            file:close()

            -- Read back to verify
            local read_file = io.open("file1.txt", "r")
            local content = read_file:read("*a")
            print("Content after appends:", content)
            read_file:close()
        "#;

        let test_session = TestSession::init(cx).await;
        let output = test_session.test_success(script, cx).await;
        assert_eq!(
            output,
            "Content after appends:\tHello world!\nAppended content\nMore appended content\n"
        );
        assert_eq!(
            test_session.diff(cx),
            vec![(
                PathBuf::from("file1.txt"),
                vec![(
                    "".to_string(),
                    "Appended content\nMore appended content".to_string()
                )]
            )]
        );
    }

    #[gpui::test]
    async fn test_read_formats(cx: &mut TestAppContext) {
        let script = r#"
            local file = io.open("multiline.txt", "w")
            file:write("Line 1\nLine 2\nLine 3")
            file:close()

            -- Test "*a" (all)
            local f = io.open("multiline.txt", "r")
            local all = f:read("*a")
            print("All:", all)
            f:close()

            -- Test "*l" (line)
            f = io.open("multiline.txt", "r")
            local line1 = f:read("*l")
            local line2 = f:read("*l")
            local line3 = f:read("*l")
            print("Line 1:", line1)
            print("Line 2:", line2)
            print("Line 3:", line3)
            f:close()

            -- Test "*L" (line with newline)
            f = io.open("multiline.txt", "r")
            local line_with_nl = f:read("*L")
            print("Line with newline length:", #line_with_nl)
            print("Last char:", string.byte(line_with_nl, #line_with_nl))
            f:close()

            -- Test number of bytes
            f = io.open("multiline.txt", "r")
            local bytes5 = f:read(5)
            print("5 bytes:", bytes5)
            f:close()
        "#;

        let test_session = TestSession::init(cx).await;
        let output = test_session.test_success(script, cx).await;
        println!("{}", &output);
        assert!(output.contains("All:\tLine 1\nLine 2\nLine 3"));
        assert!(output.contains("Line 1:\tLine 1"));
        assert!(output.contains("Line 2:\tLine 2"));
        assert!(output.contains("Line 3:\tLine 3"));
        assert!(output.contains("Line with newline length:\t7"));
        assert!(output.contains("Last char:\t10")); // LF
        assert!(output.contains("5 bytes:\tLine "));
        assert_eq!(
            test_session.diff(cx),
            vec![(
                PathBuf::from("multiline.txt"),
                vec![("".to_string(), "Line 1\nLine 2\nLine 3".to_string())]
            )]
        );
    }

    // helpers

    struct TestSession {
        session: Entity<ScriptingSession>,
    }

    impl TestSession {
        async fn init(cx: &mut TestAppContext) -> Self {
            let settings_store = cx.update(SettingsStore::test);
            cx.set_global(settings_store);
            cx.update(Project::init_settings);
            cx.update(language::init);

            let fs = FakeFs::new(cx.executor());
            fs.insert_tree(
                path!("/"),
                json!({
                    "file1.txt": "Hello world!\n",
                    "file2.txt": "Goodbye moon!\n"
                }),
            )
            .await;

            let project = Project::test(fs.clone(), [Path::new(path!("/"))], cx).await;
            let session = cx.new(|cx| ScriptingSession::new(project, cx));

            TestSession { session }
        }

        async fn test_success(&self, source: &str, cx: &mut TestAppContext) -> String {
            let script_id = self.run_script(source, cx).await;

            self.session.read_with(cx, |session, _cx| {
                let script = session.get(script_id);
                let stdout = script.stdout_snapshot();

                if let ScriptState::Failed { error, .. } = &script.state {
                    panic!("Script failed:\n{}\n\n{}", error, stdout);
                }

                stdout
            })
        }

        fn diff(&self, cx: &mut TestAppContext) -> Vec<(PathBuf, Vec<(String, String)>)> {
            self.session.read_with(cx, |session, cx| {
                session
                    .changes_by_buffer
                    .iter()
                    .map(|(buffer, changes)| {
                        let snapshot = buffer.read(cx).snapshot();
                        let diff = changes.diff.read(cx);
                        let hunks = diff.hunks(&snapshot, cx);
                        let path = buffer.read(cx).file().unwrap().path().clone();
                        let diffs = hunks
                            .map(|hunk| {
                                let old_text = diff
                                    .base_text()
                                    .text_for_range(hunk.diff_base_byte_range)
                                    .collect::<String>();
                                let new_text =
                                    snapshot.text_for_range(hunk.range).collect::<String>();
                                (old_text, new_text)
                            })
                            .collect();
                        (path.to_path_buf(), diffs)
                    })
                    .collect()
            })
        }

        async fn run_script(&self, source: &str, cx: &mut TestAppContext) -> ScriptId {
            let (script_id, task) = self
                .session
                .update(cx, |session, cx| session.run_script(source.to_string(), cx));

            task.await;

            script_id
        }
    }
}
