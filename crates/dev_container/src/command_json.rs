// A small module for handling commands which output json, and can deserialize into a provided struct

use std::process::Output;

use serde::Deserialize;
use smol::process::Command;

use crate::DevContainerErrorV2;

pub(crate) async fn evaluate_json_command<T>(
    mut command: Command,
) -> Result<Option<T>, DevContainerErrorV2>
where
    T: for<'de> Deserialize<'de>,
{
    let output = command.output().await.map_err(|e| {
        log::error!("Error inspecting docker image: {e}");
        DevContainerErrorV2::UnmappedError
    })?;

    deserialize_json_output(output)
}

pub(crate) fn deserialize_json_output<T>(output: Output) -> Result<Option<T>, DevContainerErrorV2>
where
    T: for<'de> Deserialize<'de>,
{
    if output.status.success() {
        let raw = String::from_utf8_lossy(&output.stdout);
        if raw.is_empty() {
            return Ok(None);
        }
        let value = serde_json_lenient::from_str(&raw).map_err(|e| {
            log::error!("Error deserializing from raw json: {e}");
            DevContainerErrorV2::UnmappedError
        });
        value
    } else {
        let std_err = String::from_utf8_lossy(&output.stderr);
        log::error!("Sent non-successful output; cannot deserialize. StdErr: {std_err}");
        Err(DevContainerErrorV2::UnmappedError)
    }
}
