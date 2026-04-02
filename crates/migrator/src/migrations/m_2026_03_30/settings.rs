use anyhow::Result;
use serde_json::Value;

use crate::migrations::migrate_settings;

pub fn make_play_sound_when_agent_done_an_enum(value: &mut Value) -> Result<()> {
    migrate_settings(value, &mut migrate_one)
}

fn migrate_one(obj: &mut serde_json::Map<String, Value>) -> Result<()> {
    let Some(play_sound) = obj
        .get_mut("agent")
        .and_then(|agent| agent.as_object_mut())
        .and_then(|agent| agent.get_mut("play_sound_when_agent_done"))
    else {
        return Ok(());
    };

    *play_sound = match play_sound {
        Value::Bool(true) => Value::String("always".to_string()),
        Value::Bool(false) => Value::String("never".to_string()),
        Value::String(s) if s == "never" || s == "when_hidden" || s == "always" => return Ok(()),
        _ => {
            anyhow::bail!("Expected play_sound_when_agent_done to be a boolean or valid enum value")
        }
    };

    Ok(())
}
