mod session;

pub(crate) use session::*;

use assistant_tool::{Tool, ToolRegistry};
use gpui::{App, AppContext as _, Task, WeakEntity, Window};
use schemars::JsonSchema;
use serde::Deserialize;
use std::sync::Arc;
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
        let input = match serde_json::from_value::<ScriptingToolInput>(input) {
            Err(err) => return Task::ready(Err(err.into())),
            Ok(input) => input,
        };
        let Ok(project) = workspace.read_with(cx, |workspace, _cx| workspace.project().clone())
        else {
            return Task::ready(Err(anyhow::anyhow!("No project found")));
        };

        let session = cx.new(|cx| Session::new(project, cx));
        let lua_script = input.lua_script;
        let script = session.update(cx, |session, cx| session.run_script(lua_script, cx));
        cx.spawn(|_cx| async move {
            let output = script.await?.stdout;
            drop(session);
            Ok(format!("The script output the following:\n{output}"))
        })
    }
}
