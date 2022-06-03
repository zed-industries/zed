use plugin::prelude::*;

#[bind]
pub fn name(_: ()) -> &'static str {
    "vscode-json-languageserver"
}

#[bind]
pub fn server_args(_: ()) -> Vec<String> {
    vec!["--stdio".into()]
}
