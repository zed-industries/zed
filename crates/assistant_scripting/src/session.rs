use anyhow::anyhow;
use collections::{HashMap, HashSet};
use futures::{
    channel::{mpsc, oneshot},
    pin_mut, SinkExt, StreamExt,
};
use gpui::{AppContext, AsyncApp, Context, Entity, EventEmitter, SharedString, Task, WeakEntity};
use mlua::{ExternalResult, Lua, MultiValue, Table, UserData, UserDataMethods};
use parking_lot::Mutex;
use project::{search::SearchQuery, Fs, Project};
use regex::Regex;
use std::{
    cell::RefCell,
    path::{Path, PathBuf},
    sync::Arc,
};
use util::{paths::PathMatcher, ResultExt};

use crate::{SCRIPT_END_TAG, SCRIPT_START_TAG};

struct ForegroundFn(Box<dyn FnOnce(WeakEntity<ScriptSession>, AsyncApp) + Send>);

pub struct ScriptSession {
    project: Entity<Project>,
    // TODO Remove this
    fs_changes: Arc<Mutex<HashMap<PathBuf, Vec<u8>>>>,
    foreground_fns_tx: mpsc::Sender<ForegroundFn>,
    _invoke_foreground_fns: Task<()>,
    scripts: Vec<Script>,
}

impl ScriptSession {
    pub fn new(project: Entity<Project>, cx: &mut Context<Self>) -> Self {
        let (foreground_fns_tx, mut foreground_fns_rx) = mpsc::channel(128);
        ScriptSession {
            project,
            fs_changes: Arc::new(Mutex::new(HashMap::default())),
            foreground_fns_tx,
            _invoke_foreground_fns: cx.spawn(|this, cx| async move {
                while let Some(foreground_fn) = foreground_fns_rx.next().await {
                    foreground_fn.0(this.clone(), cx.clone());
                }
            }),
            scripts: Vec::new(),
        }
    }

    pub fn new_script(&mut self) -> ScriptId {
        let id = ScriptId(self.scripts.len() as u32);
        let script = Script {
            id,
            state: ScriptState::Generating,
            source: SharedString::new_static(""),
        };
        self.scripts.push(script);
        id
    }

    pub fn run_script(
        &mut self,
        script_id: ScriptId,
        script_src: String,
        cx: &mut Context<Self>,
    ) -> Task<anyhow::Result<()>> {
        let script = self.get_mut(script_id);

        let stdout = Arc::new(Mutex::new(String::new()));
        script.source = script_src.clone().into();
        script.state = ScriptState::Running {
            stdout: stdout.clone(),
        };

        let task = self.run_lua(script_src, stdout, cx);

        cx.emit(ScriptEvent::Spawned(script_id));

        cx.spawn(|session, mut cx| async move {
            let result = task.await;

            session.update(&mut cx, |session, cx| {
                let script = session.get_mut(script_id);
                let stdout = script.stdout_snapshot();

                script.state = match result {
                    Ok(()) => ScriptState::Succeeded { stdout },
                    Err(error) => ScriptState::Failed { stdout, error },
                };

                cx.emit(ScriptEvent::Exited(script_id))
            })
        })
    }

    fn run_lua(
        &mut self,
        script: String,
        stdout: Arc<Mutex<String>>,
        cx: &mut Context<Self>,
    ) -> Task<anyhow::Result<()>> {
        const SANDBOX_PREAMBLE: &str = include_str!("sandbox_preamble.lua");

        // TODO Remove fs_changes
        let fs_changes = self.fs_changes.clone();
        // TODO Honor all worktrees instead of the first one
        let root_dir = self
            .project
            .read(cx)
            .visible_worktrees(cx)
            .next()
            .map(|worktree| worktree.read(cx).abs_path());

        let fs = self.project.read(cx).fs().clone();
        let foreground_fns_tx = self.foreground_fns_tx.clone();

        let task = cx.background_spawn({
            let stdout = stdout.clone();

            async move {
                let lua = Lua::new();
                lua.set_memory_limit(2 * 1024 * 1024 * 1024)?; // 2 GB
                let globals = lua.globals();
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
                    lua.create_function({
                        let fs_changes = fs_changes.clone();
                        let root_dir = root_dir.clone();
                        move |lua, (path_str, mode)| {
                            Self::io_open(&lua, &fs_changes, root_dir.as_ref(), path_str, mode)
                        }
                    })?,
                )?;
                globals.set("user_script", script)?;

                lua.load(SANDBOX_PREAMBLE).exec_async().await?;

                // Drop Lua instance to decrement reference count.
                drop(lua);

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
    fn io_open(
        lua: &Lua,
        fs_changes: &Arc<Mutex<HashMap<PathBuf, Vec<u8>>>>,
        root_dir: Option<&Arc<Path>>,
        path_str: String,
        mode: Option<String>,
    ) -> mlua::Result<(Option<Table>, String)> {
        let root_dir = root_dir
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
        file.set("__path", path_str.clone())?;
        file.set("__mode", mode.clone())?;
        file.set("__read_perm", read_perm)?;
        file.set("__write_perm", write_perm)?;

        let path = match Self::parse_abs_path_in_root_dir(&root_dir, &path_str) {
            Ok(path) => path,
            Err(err) => return Ok((None, format!("{err}"))),
        };

        // close method
        let close_fn = {
            let fs_changes = fs_changes.clone();
            lua.create_function(move |_lua, file_userdata: mlua::Table| {
                let write_perm = file_userdata.get::<bool>("__write_perm")?;
                let path = file_userdata.get::<String>("__path")?;

                if write_perm {
                    // When closing a writable file, record the content
                    let content = file_userdata.get::<mlua::AnyUserData>("__content")?;
                    let content_ref = content.borrow::<FileContent>()?;
                    let content_vec = content_ref.0.borrow();

                    // Don't actually write to disk; instead, just update fs_changes.
                    let path_buf = PathBuf::from(&path);
                    fs_changes
                        .lock()
                        .insert(path_buf.clone(), content_vec.clone());
                }

                Ok(true)
            })?
        };
        file.set("close", close_fn)?;

        // If it's a directory, give it a custom read() and return early.
        if path.is_dir() {
            // TODO handle the case where we changed it in the in-memory fs

            // Create a special directory handle
            file.set("__is_directory", true)?;

            // Store directory entries
            let entries = match std::fs::read_dir(&path) {
                Ok(entries) => {
                    let mut entry_names = Vec::new();
                    for entry in entries.flatten() {
                        entry_names.push(entry.file_name().to_string_lossy().into_owned());
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

        let fs_changes_map = fs_changes.lock();

        let is_in_changes = fs_changes_map.contains_key(&path);
        let file_exists = is_in_changes || path.exists();
        let mut file_content = Vec::new();

        if file_exists && !truncate {
            if is_in_changes {
                file_content = fs_changes_map.get(&path).unwrap().clone();
            } else {
                // Try to read existing content if file exists and we're not truncating
                match std::fs::read(&path) {
                    Ok(content) => file_content = content,
                    Err(e) => return Ok((None, format!("Error reading file: {}", e))),
                }
            }
        }

        drop(fs_changes_map); // Unlock the fs_changes mutex.

        // If in append mode, position should be at the end
        let position = if append && file_exists {
            file_content.len()
        } else {
            0
        };
        file.set("__position", position)?;
        file.set(
            "__content",
            lua.create_userdata(FileContent(RefCell::new(file_content)))?,
        )?;

        // Create file methods

        // read method
        let read_fn = {
            lua.create_function(
                |_lua, (file_userdata, format): (mlua::Table, Option<mlua::Value>)| {
                    let read_perm = file_userdata.get::<bool>("__read_perm")?;
                    if !read_perm {
                        return Err(mlua::Error::runtime("File not open for reading"));
                    }

                    let content = file_userdata.get::<mlua::AnyUserData>("__content")?;
                    let mut position = file_userdata.get::<usize>("__position")?;
                    let content_ref = content.borrow::<FileContent>()?;
                    let content_vec = content_ref.0.borrow();

                    if position >= content_vec.len() {
                        return Ok(None); // EOF
                    }

                    match format {
                        Some(mlua::Value::String(s)) => {
                            let lossy_string = s.to_string_lossy();
                            let format_str: &str = lossy_string.as_ref();

                            // Only consider the first 2 bytes, since it's common to pass e.g. "*all"  instead of "*a"
                            match &format_str[0..2] {
                                "*a" => {
                                    // Read entire file from current position
                                    let result = String::from_utf8_lossy(&content_vec[position..])
                                        .to_string();
                                    position = content_vec.len();
                                    file_userdata.set("__position", position)?;
                                    Ok(Some(result))
                                }
                                "*l" => {
                                    // Read next line
                                    let mut line = Vec::new();
                                    let mut found_newline = false;

                                    while position < content_vec.len() {
                                        let byte = content_vec[position];
                                        position += 1;

                                        if byte == b'\n' {
                                            found_newline = true;
                                            break;
                                        }

                                        // Skip \r in \r\n sequence but add it if it's alone
                                        if byte == b'\r' {
                                            if position < content_vec.len()
                                                && content_vec[position] == b'\n'
                                            {
                                                position += 1;
                                                found_newline = true;
                                                break;
                                            }
                                        }

                                        line.push(byte);
                                    }

                                    file_userdata.set("__position", position)?;

                                    if !found_newline
                                        && line.is_empty()
                                        && position >= content_vec.len()
                                    {
                                        return Ok(None); // EOF
                                    }

                                    let result = String::from_utf8_lossy(&line).to_string();
                                    Ok(Some(result))
                                }
                                "*n" => {
                                    // Try to parse as a number (number of bytes to read)
                                    match format_str.parse::<usize>() {
                                        Ok(n) => {
                                            let end =
                                                std::cmp::min(position + n, content_vec.len());
                                            let bytes = &content_vec[position..end];
                                            let result = String::from_utf8_lossy(bytes).to_string();
                                            position = end;
                                            file_userdata.set("__position", position)?;
                                            Ok(Some(result))
                                        }
                                        Err(_) => Err(mlua::Error::runtime(format!(
                                            "Invalid format: {}",
                                            format_str
                                        ))),
                                    }
                                }
                                "*L" => {
                                    // Read next line keeping the end of line
                                    let mut line = Vec::new();

                                    while position < content_vec.len() {
                                        let byte = content_vec[position];
                                        position += 1;

                                        line.push(byte);

                                        if byte == b'\n' {
                                            break;
                                        }

                                        // If we encounter a \r, add it and check if the next is \n
                                        if byte == b'\r'
                                            && position < content_vec.len()
                                            && content_vec[position] == b'\n'
                                        {
                                            line.push(content_vec[position]);
                                            position += 1;
                                            break;
                                        }
                                    }

                                    file_userdata.set("__position", position)?;

                                    if line.is_empty() && position >= content_vec.len() {
                                        return Ok(None); // EOF
                                    }

                                    let result = String::from_utf8_lossy(&line).to_string();
                                    Ok(Some(result))
                                }
                                _ => Err(mlua::Error::runtime(format!(
                                    "Unsupported format: {}",
                                    format_str
                                ))),
                            }
                        }
                        Some(mlua::Value::Number(n)) => {
                            // Read n bytes
                            let n = n as usize;
                            let end = std::cmp::min(position + n, content_vec.len());
                            let bytes = &content_vec[position..end];
                            let result = String::from_utf8_lossy(bytes).to_string();
                            position = end;
                            file_userdata.set("__position", position)?;
                            Ok(Some(result))
                        }
                        Some(_) => Err(mlua::Error::runtime("Invalid format")),
                        None => {
                            // Default is to read a line
                            let mut line = Vec::new();
                            let mut found_newline = false;

                            while position < content_vec.len() {
                                let byte = content_vec[position];
                                position += 1;

                                if byte == b'\n' {
                                    found_newline = true;
                                    break;
                                }

                                // Handle \r\n
                                if byte == b'\r' {
                                    if position < content_vec.len()
                                        && content_vec[position] == b'\n'
                                    {
                                        position += 1;
                                        found_newline = true;
                                        break;
                                    }
                                }

                                line.push(byte);
                            }

                            file_userdata.set("__position", position)?;

                            if !found_newline && line.is_empty() && position >= content_vec.len() {
                                return Ok(None); // EOF
                            }

                            let result = String::from_utf8_lossy(&line).to_string();
                            Ok(Some(result))
                        }
                    }
                },
            )?
        };
        file.set("read", read_fn)?;

        // write method
        let write_fn = {
            let fs_changes = fs_changes.clone();

            lua.create_function(move |_lua, (file_userdata, text): (mlua::Table, String)| {
                let write_perm = file_userdata.get::<bool>("__write_perm")?;
                if !write_perm {
                    return Err(mlua::Error::runtime("File not open for writing"));
                }

                let content = file_userdata.get::<mlua::AnyUserData>("__content")?;
                let position = file_userdata.get::<usize>("__position")?;
                let content_ref = content.borrow::<FileContent>()?;
                let mut content_vec = content_ref.0.borrow_mut();

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

                // Update fs_changes
                let path = file_userdata.get::<String>("__path")?;
                let path_buf = PathBuf::from(path);
                fs_changes.lock().insert(path_buf, content_vec.clone());

                Ok(true)
            })?
        };
        file.set("write", write_fn)?;

        // If we got this far, the file was opened successfully
        Ok((Some(file), String::new()))
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

struct FileContent(RefCell<Vec<u8>>);

impl UserData for FileContent {
    fn add_methods<M: UserDataMethods<Self>>(_methods: &mut M) {
        // FileContent doesn't have any methods so far.
    }
}

#[derive(Debug)]
pub enum ScriptEvent {
    Spawned(ScriptId),
    Exited(ScriptId),
}

impl EventEmitter<ScriptEvent> for ScriptSession {}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ScriptId(u32);

pub struct Script {
    pub id: ScriptId,
    pub state: ScriptState,
    pub source: SharedString,
}

pub enum ScriptState {
    Generating,
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
    pub fn source_tag(&self) -> String {
        format!("{}{}{}", SCRIPT_START_TAG, self.source, SCRIPT_END_TAG)
    }

    /// If exited, returns a message with the output for the LLM
    pub fn output_message_for_llm(&self) -> Option<String> {
        match &self.state {
            ScriptState::Generating { .. } => None,
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
            ScriptState::Generating { .. } => String::new(),
            ScriptState::Running { stdout } => stdout.lock().clone(),
            ScriptState::Succeeded { stdout } => stdout.clone(),
            ScriptState::Failed { stdout, .. } => stdout.clone(),
        }
    }

    /// Returns the error if the script failed, otherwise None
    pub fn error(&self) -> Option<&anyhow::Error> {
        match &self.state {
            ScriptState::Generating { .. } => None,
            ScriptState::Running { .. } => None,
            ScriptState::Succeeded { .. } => None,
            ScriptState::Failed { error, .. } => Some(error),
        }
    }
}

#[cfg(test)]
mod tests {
    use gpui::TestAppContext;
    use project::FakeFs;
    use serde_json::json;
    use settings::SettingsStore;

    use super::*;

    #[gpui::test]
    async fn test_print(cx: &mut TestAppContext) {
        let script = r#"
            print("Hello", "world!")
            print("Goodbye", "moon!")
        "#;

        let output = test_script(script, cx).await.unwrap();
        assert_eq!(output, "Hello\tworld!\nGoodbye\tmoon!\n");
    }

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

        let output = test_script(script, cx).await.unwrap();
        assert_eq!(output, "File: /file1.txt\nMatches:\n  world\n");
    }

    async fn test_script(source: &str, cx: &mut TestAppContext) -> anyhow::Result<String> {
        init_test(cx);
        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            "/",
            json!({
                "file1.txt": "Hello world!",
                "file2.txt": "Goodbye moon!"
            }),
        )
        .await;

        let project = Project::test(fs, [Path::new("/")], cx).await;
        let session = cx.new(|cx| ScriptSession::new(project, cx));

        let (script_id, task) = session.update(cx, |session, cx| {
            let script_id = session.new_script();
            let task = session.run_script(script_id, source.to_string(), cx);

            (script_id, task)
        });

        task.await?;

        Ok(session.read_with(cx, |session, _cx| session.get(script_id).stdout_snapshot()))
    }

    fn init_test(cx: &mut TestAppContext) {
        let settings_store = cx.update(SettingsStore::test);
        cx.set_global(settings_store);
        cx.update(Project::init_settings);
    }
}
