use anyhow::Result;
use serde::Serialize;
use std::env;
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize)]
struct ActionDef {
    name: &'static str,
    human_name: String,
    #[serde(skip_serializing_if = "<[_]>::is_empty")]
    deprecated_aliases: &'static [&'static str],
}

fn main() -> Result<()> {
    #[cfg(any(test, feature = "test-support"))]
    {
        // call a zed:: function so everything in `zed` crate is linked and
        // all actions in the actual app are registered
        zed::stdout_is_a_pty();

        let actions = dump_all_gpui_actions();

        let out_dir = env::var("CARGO_MANIFEST_DIR")?;
        let assets_path = Path::new(&out_dir)
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .join("assets/actions");

        // Create the actions directory if it doesn't exist
        fs::create_dir_all(&assets_path)?;

        let json_path = assets_path.join("actions.json");
        let json_content = serde_json::to_string_pretty(&actions)?;
        fs::write(&json_path, json_content)?;

        println!("cargo:rerun-if-changed=build.rs");
        // println!("cargo:rerun-if-changed=../../assets/actions/actions.json");
    }

    Ok(())
}

fn dump_all_gpui_actions() -> Vec<ActionDef> {
    let mut actions = gpui::generate_list_of_all_registered_actions()
        .into_iter()
        .map(|action| ActionDef {
            name: action.name,
            human_name: command_palette::humanize_action_name(action.name),
            deprecated_aliases: action.aliases,
        })
        .collect::<Vec<ActionDef>>();

    actions.sort_by_key(|a| a.name);

    actions
}
