use std::process;

#[tokio::main]
pub async fn main() -> Result<()> {
    let args = Args::parse();

    let source_dir = source_dir
        .canonicalize()
        .context("failed to canonicalize source_dir")?;
    let scratch_dir = scratch_dir
        .canonicalize()
        .context("failed to canonicalize scratch_dir")?;
    extension_cli::run(source_dir, scratch_dir).await;

    let output_dir = if args.output_dir.is_relative() {
        env::current_dir()?.join(&args.output_dir)
    } else {
        args.output_dir
    };

    let archive_dir = output_dir.join("archive");
    fs::remove_dir_all(&archive_dir).ok();
    copy_extension_resources(&manifest, &extension_path, &archive_dir, fs.clone())
        .await
        .context("failed to copy extension resources")?;

    let tar_output = Command::new("tar")
        .current_dir(&output_dir)
        .args(["-czvf", "archive.tar.gz", "-C", "archive", "."])
        .output()
        .context("failed to run tar")?;
    if !tar_output.status.success() {
        bail!(
            "failed to create archive.tar.gz: {}",
            String::from_utf8_lossy(&tar_output.stderr)
        );
    }

    let extension_provides = extension_provides(&manifest);

    let manifest_json = serde_json::to_string(&rpc::ExtensionApiManifest {
        name: manifest.name,
        version: manifest.version,
        description: manifest.description,
        authors: manifest.authors,
        schema_version: Some(manifest.schema_version.0),
        repository: manifest
            .repository
            .context("missing repository in extension manifest")?,
        wasm_api_version: manifest.lib.version.map(|version| version.to_string()),
        provides: extension_provides,
    })?;
    fs::remove_dir_all(&archive_dir)?;
    fs::write(output_dir.join("manifest.json"), manifest_json.as_bytes())?;
}
