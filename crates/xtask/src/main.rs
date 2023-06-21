mod cli;

use schemars::schema_for;

use theme::Theme;
fn main() {
    let theme = schema_for!(Theme);
    let output = serde_json::to_string_pretty(&theme).unwrap();
    std::fs::create_dir("schemas").ok();
    std::fs::write("schemas/theme.json", output).ok();
}
