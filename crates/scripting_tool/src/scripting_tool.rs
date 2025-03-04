use anyhow::anyhow;
use assistant_tool::{Tool, ToolRegistry};
use gpui::{App, AppContext as _, Task, WeakEntity, Window};
use mlua::{Function, Lua, MultiValue, Result, UserData, UserDataMethods};
use schemars::JsonSchema;
use serde::Deserialize;
use std::{
    cell::RefCell,
    collections::HashMap,
    path::{Path, PathBuf},
    rc::Rc,
    sync::Arc,
};
use workspace::Workspace;

pub fn init(cx: &App) {
    let registry = ToolRegistry::global(cx);
    registry.register_tool(ScriptingTool);
}

#[derive(Debug, Deserialize, JsonSchema)]
struct ScriptingToolInput {
    lua_script: String,
}

struct ScriptingTool;

impl Tool for ScriptingTool {
    fn name(&self) -> String {
        "lua-interpreter".into()
    }

    fn description(&self) -> String {
        r#"You can write a Lua script and I'll run it on my code base and tell you what its output was,
including both stdout as well as the git diff of changes it made to the filesystem. That way,
you can get more information about the code base, or make changes to the code base directly.
The lua script will have access to `io` and it will run with the current working directory being in
the root of the code base, so you can use it to explore, search, make changes, etc. You can also have
the script print things, and I'll tell you what the output was. Note that `io` only has `open`, and
then the file it returns only has the methods read, write, and close - it doesn't have popen or
anything else. Also, I'm going to be putting this Lua script into JSON, so please don't use Lua's
double quote syntax for string literals - use one of Lua's other syntaxes for string literals, so I
don't have to escape the double quotes. There will be a global called `search` which accepts a regex
(it's implemented using Rust's regex crate, so use that regex syntax) and runs that regex on the contents
of every file in the code base (aside from gitignored files), then returns an array of tables with two
fields: "path" (the path to the file that had the matches) and "matches" (an array of strings, with each
string being a match that was found within the file)."#.into()
    }

    fn input_schema(&self) -> serde_json::Value {
        let schema = schemars::schema_for!(ScriptingToolInput);
        serde_json::to_value(&schema).unwrap()
    }

    fn run(
        self: Arc<Self>,
        input: serde_json::Value,
        workspace: WeakEntity<Workspace>,
        _window: &mut Window,
        cx: &mut App,
    ) -> Task<anyhow::Result<String>> {
        let root_dir = workspace.update(cx, |workspace, cx| {
            let first_worktree = workspace
                .visible_worktrees(cx)
                .next()
                .ok_or_else(|| anyhow!("no worktrees"))?;
            workspace
                .absolute_path_of_worktree(first_worktree.read(cx).id(), cx)
                .ok_or_else(|| anyhow!("no worktree root"))
        });
        let root_dir = match root_dir {
            Ok(root_dir) => root_dir,
            Err(err) => return Task::ready(Err(err)),
        };
        let root_dir = match root_dir {
            Ok(root_dir) => root_dir,
            Err(err) => return Task::ready(Err(err)),
        };
        let input = match serde_json::from_value::<ScriptingToolInput>(input) {
            Err(err) => return Task::ready(Err(err.into())),
            Ok(input) => input,
        };
        let lua_script = input.lua_script;
        cx.background_spawn(async move {
            let fs_changes = HashMap::new();
            let output = run_sandboxed_lua(&lua_script, fs_changes, root_dir)
                .map_err(|err| anyhow!(format!("{err}")))?;
            let output = output.printed_lines.join("\n");

            Ok(format!("The script output the following:\n{output}"))
        })
    }
}

const SANDBOX_PREAMBLE: &str = include_str!("sandbox_preamble.lua");

struct FileContent(RefCell<Vec<u8>>);

impl UserData for FileContent {
    fn add_methods<M: UserDataMethods<Self>>(_methods: &mut M) {
        // FileContent doesn't have any methods so far.
    }
}

/// Sandboxed print() function in Lua.
fn print(lua: &Lua, printed_lines: Rc<RefCell<Vec<String>>>) -> Result<Function> {
    lua.create_function(move |_, args: MultiValue| {
        let mut string = String::new();

        for arg in args.into_iter() {
            // Lua's `print()` prints tab characters between each argument.
            if !string.is_empty() {
                string.push('\t');
            }

            // If the argument's to_string() fails, have the whole function call fail.
            string.push_str(arg.to_string()?.as_str())
        }

        printed_lines.borrow_mut().push(string);

        Ok(())
    })
}

fn search(
    lua: &Lua,
    _fs_changes: Rc<RefCell<HashMap<PathBuf, Vec<u8>>>>,
    root_dir: PathBuf,
) -> Result<Function> {
    lua.create_function(move |lua, regex: String| {
        use mlua::Table;
        use regex::Regex;
        use std::fs;

        // Function to recursively search directory
        let search_regex = match Regex::new(&regex) {
            Ok(re) => re,
            Err(e) => return Err(mlua::Error::runtime(format!("Invalid regex: {}", e))),
        };

        let mut search_results: Vec<Result<Table>> = Vec::new();

        // Create an explicit stack for directories to process
        let mut dir_stack = vec![root_dir.clone()];

        while let Some(current_dir) = dir_stack.pop() {
            // Process each entry in the current directory
            let entries = match fs::read_dir(&current_dir) {
                Ok(entries) => entries,
                Err(e) => return Err(e.into()),
            };

            for entry_result in entries {
                let entry = match entry_result {
                    Ok(e) => e,
                    Err(e) => return Err(e.into()),
                };

                let path = entry.path();

                if path.is_dir() {
                    // Skip .git directory and other common directories to ignore
                    let dir_name = path.file_name().unwrap_or_default().to_string_lossy();
                    if !dir_name.starts_with('.')
                        && dir_name != "node_modules"
                        && dir_name != "target"
                    {
                        // Instead of recursive call, add to stack
                        dir_stack.push(path);
                    }
                } else if path.is_file() {
                    // Skip binary files and very large files
                    if let Ok(metadata) = fs::metadata(&path) {
                        if metadata.len() > 1_000_000 {
                            // Skip files larger than 1MB
                            continue;
                        }
                    }

                    // Attempt to read the file as text
                    if let Ok(content) = fs::read_to_string(&path) {
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
                            for (i, m) in matches.iter().enumerate() {
                                matches_table.set(i + 1, m.clone())?;
                            }
                            result_entry.set("matches", matches_table)?;

                            search_results.push(Ok(result_entry));
                        }
                    }
                }
            }
        }

        // Create a table to hold our results
        let results_table = lua.create_table()?;
        for (i, result) in search_results.into_iter().enumerate() {
            match result {
                Ok(entry) => results_table.set(i + 1, entry)?,
                Err(e) => return Err(e),
            }
        }

        Ok(results_table)
    })
}

/// Sandboxed io.open() function in Lua.
fn io_open(
    lua: &Lua,
    fs_changes: Rc<RefCell<HashMap<PathBuf, Vec<u8>>>>,
    root_dir: PathBuf,
) -> Result<Function> {
    lua.create_function(move |lua, (path_str, mode): (String, Option<String>)| {
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

        // Sandbox the path; it must be within root_dir
        let path: PathBuf = {
            let rust_path = Path::new(&path_str);

            // Get absolute path
            if rust_path.is_absolute() {
                // Check if path starts with root_dir prefix without resolving symlinks
                if !rust_path.starts_with(&root_dir) {
                    return Ok((
                        None,
                        format!(
                            "Error: Absolute path {} is outside the current working directory",
                            path_str
                        ),
                    ));
                }
                rust_path.to_path_buf()
            } else {
                // Make relative path absolute relative to cwd
                root_dir.join(rust_path)
            }
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
                        .borrow_mut()
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

        let is_in_changes = fs_changes.borrow().contains_key(&path);
        let file_exists = is_in_changes || path.exists();
        let mut file_content = Vec::new();

        if file_exists && !truncate {
            if is_in_changes {
                file_content = fs_changes.borrow().get(&path).unwrap().clone();
            } else {
                // Try to read existing content if file exists and we're not truncating
                match std::fs::read(&path) {
                    Ok(content) => file_content = content,
                    Err(e) => return Ok((None, format!("Error reading file: {}", e))),
                }
            }
        }

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
                fs_changes
                    .borrow_mut()
                    .insert(path_buf, content_vec.clone());

                Ok(true)
            })?
        };
        file.set("write", write_fn)?;

        // If we got this far, the file was opened successfully
        Ok((Some(file), String::new()))
    })
}

/// Runs a Lua script in a sandboxed environment and returns the printed lines
pub fn run_sandboxed_lua(
    script: &str,
    fs_changes: HashMap<PathBuf, Vec<u8>>,
    root_dir: PathBuf,
) -> Result<ScriptOutput> {
    let lua = Lua::new();
    lua.set_memory_limit(2 * 1024 * 1024 * 1024)?; // 2 GB
    let globals = lua.globals();

    // Track the lines the Lua script prints out.
    let printed_lines = Rc::new(RefCell::new(Vec::new()));
    let fs = Rc::new(RefCell::new(fs_changes));

    globals.set("sb_print", print(&lua, printed_lines.clone())?)?;
    globals.set("search", search(&lua, fs.clone(), root_dir.clone())?)?;
    globals.set("sb_io_open", io_open(&lua, fs.clone(), root_dir)?)?;
    globals.set("user_script", script)?;

    lua.load(SANDBOX_PREAMBLE).exec()?;

    drop(lua); // Necessary so the Rc'd values get decremented.

    Ok(ScriptOutput {
        printed_lines: Rc::try_unwrap(printed_lines)
            .expect("There are still other references to printed_lines")
            .into_inner(),
        fs_changes: Rc::try_unwrap(fs)
            .expect("There are still other references to fs_changes")
            .into_inner(),
    })
}

pub struct ScriptOutput {
    printed_lines: Vec<String>,
    #[allow(dead_code)]
    fs_changes: HashMap<PathBuf, Vec<u8>>,
}

#[allow(dead_code)]
impl ScriptOutput {
    fn fs_diff(&self) -> HashMap<PathBuf, String> {
        let mut diff_map = HashMap::new();
        for (path, content) in &self.fs_changes {
            let diff = if path.exists() {
                // Read the current file content
                match std::fs::read(path) {
                    Ok(current_content) => {
                        // Convert both to strings for diffing
                        let new_content = String::from_utf8_lossy(content).to_string();
                        let old_content = String::from_utf8_lossy(&current_content).to_string();

                        // Generate a git-style diff
                        let new_lines: Vec<&str> = new_content.lines().collect();
                        let old_lines: Vec<&str> = old_content.lines().collect();

                        let path_str = path.to_string_lossy();
                        let mut diff = format!("diff --git a/{} b/{}\n", path_str, path_str);
                        diff.push_str(&format!("--- a/{}\n", path_str));
                        diff.push_str(&format!("+++ b/{}\n", path_str));

                        // Very basic diff algorithm - this is simplified
                        let mut i = 0;
                        let mut j = 0;

                        while i < old_lines.len() || j < new_lines.len() {
                            if i < old_lines.len()
                                && j < new_lines.len()
                                && old_lines[i] == new_lines[j]
                            {
                                i += 1;
                                j += 1;
                                continue;
                            }

                            // Find next matching line
                            let mut next_i = i;
                            let mut next_j = j;
                            let mut found = false;

                            // Look ahead for matches
                            for look_i in i..std::cmp::min(i + 10, old_lines.len()) {
                                for look_j in j..std::cmp::min(j + 10, new_lines.len()) {
                                    if old_lines[look_i] == new_lines[look_j] {
                                        next_i = look_i;
                                        next_j = look_j;
                                        found = true;
                                        break;
                                    }
                                }
                                if found {
                                    break;
                                }
                            }

                            // Output the hunk header
                            diff.push_str(&format!(
                                "@@ -{},{} +{},{} @@\n",
                                i + 1,
                                if found {
                                    next_i - i
                                } else {
                                    old_lines.len() - i
                                },
                                j + 1,
                                if found {
                                    next_j - j
                                } else {
                                    new_lines.len() - j
                                }
                            ));

                            // Output removed lines
                            for k in i..next_i {
                                diff.push_str(&format!("-{}\n", old_lines[k]));
                            }

                            // Output added lines
                            for k in j..next_j {
                                diff.push_str(&format!("+{}\n", new_lines[k]));
                            }

                            i = next_i;
                            j = next_j;

                            if found {
                                i += 1;
                                j += 1;
                            } else {
                                break;
                            }
                        }

                        diff
                    }
                    Err(_) => format!("Error reading current file: {}", path.display()),
                }
            } else {
                // New file
                let content_str = String::from_utf8_lossy(content).to_string();
                let path_str = path.to_string_lossy();
                let mut diff = format!("diff --git a/{} b/{}\n", path_str, path_str);
                diff.push_str("new file mode 100644\n");
                diff.push_str("--- /dev/null\n");
                diff.push_str(&format!("+++ b/{}\n", path_str));

                let lines: Vec<&str> = content_str.lines().collect();
                diff.push_str(&format!("@@ -0,0 +1,{} @@\n", lines.len()));

                for line in lines {
                    diff.push_str(&format!("+{}\n", line));
                }

                diff
            };

            diff_map.insert(path.clone(), diff);
        }

        diff_map
    }

    fn diff_to_string(&self) -> String {
        let mut answer = String::new();
        let diff_map = self.fs_diff();

        if diff_map.is_empty() {
            return "No changes to files".to_string();
        }

        // Sort the paths for consistent output
        let mut paths: Vec<&PathBuf> = diff_map.keys().collect();
        paths.sort();

        for path in paths {
            if !answer.is_empty() {
                answer.push_str("\n");
            }
            answer.push_str(&diff_map[path]);
        }

        answer
    }
}
