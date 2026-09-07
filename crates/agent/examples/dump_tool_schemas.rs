//! Zed built-in tool schema dump.
//!
//! This example serializes the same normalized built-in tool definitions used
//! to construct language model requests.

use anyhow::Result;

fn main() -> Result<()> {
    let mut tools = agent::built_in_tools().collect::<Vec<_>>();
    tools.sort_by(|left, right| left.name.cmp(&right.name));
    serde_json::to_writer_pretty(std::io::stdout().lock(), &tools)?;
    println!();
    Ok(())
}
