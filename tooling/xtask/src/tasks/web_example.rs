#![allow(clippy::disallowed_methods, reason = "tooling is exempt")]

use std::collections::BTreeSet;
use std::io::Write;
use std::path::Path;
use std::process::Command;

use anyhow::{Context as _, Result, bail};
use clap::Parser;

#[derive(Parser)]
pub struct WebExampleArgs {
    /// Names of gpui examples to build and serve (e.g. `hello_world gradient`).
    /// If omitted, --all must be passed.
    pub names: Vec<String>,

    /// Build all discoverable gpui examples.
    #[arg(long)]
    pub all: bool,

    /// Build in release mode.
    #[arg(long)]
    pub release: bool,

    /// Port to serve on.
    #[arg(long, default_value = "8080")]
    pub port: u16,

    /// Only build, don't start the HTTP server.
    #[arg(long)]
    pub no_serve: bool,
}

/// Discover all example names from the gpui crate.
///
/// We look in two places:
/// 1. `crates/gpui/examples/*.rs` — auto-discovered by Cargo (file stem = name)
/// 2. `[[example]]` entries in `crates/gpui/Cargo.toml` — for subdirectory examples
///    like `image/image.rs` that need an explicit `name` field.
fn discover_all_examples() -> Result<BTreeSet<String>> {
    let mut names = BTreeSet::new();

    let examples_dir = Path::new("crates/gpui/examples");
    if examples_dir.is_dir() {
        for entry in std::fs::read_dir(examples_dir).context("failed to read examples dir")? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) == Some("rs") {
                if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                    names.insert(stem.to_string());
                }
            }
        }
    }

    let cargo_toml_path = Path::new("crates/gpui/Cargo.toml");
    if cargo_toml_path.is_file() {
        let content =
            std::fs::read_to_string(cargo_toml_path).context("failed to read gpui Cargo.toml")?;
        let doc: toml::Value = content.parse().context("failed to parse gpui Cargo.toml")?;
        if let Some(examples) = doc.get("example").and_then(|v| v.as_array()) {
            for example in examples {
                if let Some(name) = example.get("name").and_then(|v| v.as_str()) {
                    names.insert(name.to_string());
                }
            }
        }
    }

    Ok(names)
}

fn resolve_example_names(args: &WebExampleArgs) -> Result<Vec<String>> {
    if args.all {
        let all = discover_all_examples()?;
        if all.is_empty() {
            bail!("no examples found in crates/gpui/examples");
        }
        Ok(all.into_iter().collect())
    } else if args.names.is_empty() {
        bail!("provide one or more example names, or pass --all");
    } else {
        Ok(args.names.clone())
    }
}

struct BuildResult {
    name: String,
    success: bool,
    error_message: Option<String>,
}

fn build_single_example(name: &str, cargo: &str, release: bool) -> BuildResult {
    let profile = if release { "release" } else { "debug" };

    eprintln!("  [{name}] Building for wasm32-unknown-unknown ({profile})...");

    let mut build_command = Command::new(cargo);
    build_command
        .arg("build")
        .arg("-q")
        .arg("-p")
        .arg("gpui")
        .arg("--example")
        .arg(name)
        .arg("--target")
        .arg("wasm32-unknown-unknown");

    if release {
        build_command.arg("--release");
    }

    let output = match build_command.output() {
        Ok(output) => output,
        Err(err) => {
            return BuildResult {
                name: name.to_string(),
                success: false,
                error_message: Some(format!("failed to spawn cargo build: {err}")),
            };
        }
    };

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let truncated: String = stderr.lines().take(30).collect::<Vec<_>>().join("\n");
        return BuildResult {
            name: name.to_string(),
            success: false,
            error_message: Some(format!("cargo build failed:\n{truncated}")),
        };
    }

    let wasm_path = format!("target/wasm32-unknown-unknown/{profile}/examples/{name}.wasm",);
    let out_dir = format!("target/web-examples/{name}");
    let pkg_dir = format!("{out_dir}/pkg");

    if let Err(err) = std::fs::create_dir_all(&pkg_dir) {
        return BuildResult {
            name: name.to_string(),
            success: false,
            error_message: Some(format!("failed to create output dir: {err}")),
        };
    }

    eprintln!("  [{name}] Running wasm-bindgen...");

    let output = match Command::new("wasm-bindgen")
        .arg(&wasm_path)
        .arg("--out-dir")
        .arg(&pkg_dir)
        .arg("--target")
        .arg("web")
        .output()
    {
        Ok(output) => output,
        Err(err) => {
            return BuildResult {
                name: name.to_string(),
                success: false,
                error_message: Some(format!(
                    "failed to spawn wasm-bindgen (is it installed? cargo install wasm-bindgen-cli): {err}"
                )),
            };
        }
    };

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return BuildResult {
            name: name.to_string(),
            success: false,
            error_message: Some(format!("wasm-bindgen failed:\n{stderr}")),
        };
    }

    let html = make_example_html(name);
    let html_path = format!("{out_dir}/index.html");
    if let Err(err) =
        std::fs::File::create(&html_path).and_then(|mut f| f.write_all(html.as_bytes()))
    {
        return BuildResult {
            name: name.to_string(),
            success: false,
            error_message: Some(format!("failed to write index.html: {err}")),
        };
    }

    eprintln!("  [{name}] OK");
    BuildResult {
        name: name.to_string(),
        success: true,
        error_message: None,
    }
}

pub fn run_web_example(args: WebExampleArgs) -> Result<()> {
    let names = resolve_example_names(&args)?;
    let cargo = std::env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());

    eprintln!("Building {} example(s) for the web...\n", names.len());

    let mut results: Vec<BuildResult> = Vec::new();
    for name in &names {
        let result = build_single_example(name, &cargo, args.release);
        results.push(result);
    }

    let succeeded: Vec<&str> = results
        .iter()
        .filter(|r| r.success)
        .map(|r| r.name.as_str())
        .collect();
    let failed: Vec<&BuildResult> = results.iter().filter(|r| !r.success).collect();

    eprintln!();
    eprintln!("=== Build Summary ===");
    eprintln!("  Succeeded: {}", succeeded.len());
    eprintln!("  Failed:    {}", failed.len());

    for failure in &failed {
        eprintln!();
        eprintln!("  FAILED: {}", failure.name);
        if let Some(msg) = &failure.error_message {
            for line in msg.lines().take(10) {
                eprintln!("    {line}");
            }
        }
    }

    if succeeded.is_empty() {
        bail!("all examples failed to build");
    }

    let out_root = "target/web-examples";
    std::fs::create_dir_all(out_root).context("failed to create output root")?;

    let index_html = make_picker_html(&succeeded);
    let index_path = format!("{out_root}/index.html");
    std::fs::File::create(&index_path)
        .and_then(|mut f| f.write_all(index_html.as_bytes()))
        .context("failed to write top-level index.html")?;

    eprintln!();
    eprintln!(
        "Built {} example(s). Output in {out_root}/",
        succeeded.len()
    );

    if args.no_serve {
        eprintln!("Skipping HTTP server (--no-serve).");
        eprintln!(
            "To serve manually: cd {out_root} && python3 -m http.server {}",
            args.port
        );
        return Ok(());
    }

    eprintln!(
        "Serving on http://localhost:{}  (picker at root)",
        args.port
    );
    eprintln!("Press Ctrl+C to stop.\n");

    let exit_status = Command::new("python3")
        .arg("-m")
        .arg("http.server")
        .arg(args.port.to_string())
        .current_dir(out_root)
        .spawn()
        .context("failed to spawn python3 http server (is python3 installed?)")?
        .wait()
        .context("failed to wait for http server")?;

    if !exit_status.success() {
        bail!("HTTP server exited with: {}", exit_status);
    }

    Ok(())
}

// ---------------------------------------------------------------------------
// HTML generation
// ---------------------------------------------------------------------------

fn make_example_html(name: &str) -> String {
    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>GPUI Example: {name}</title>
    <style>
        * {{ margin: 0; padding: 0; box-sizing: border-box; }}
        html, body {{
            width: 100%; height: 100%; overflow: hidden;
            background: #1e1e2e; color: #cdd6f4;
            font-family: system-ui, -apple-system, sans-serif;
        }}
        canvas {{ display: block; width: 100%; height: 100%; }}
        #loading {{
            position: fixed; inset: 0;
            display: flex; align-items: center; justify-content: center;
            font-size: 1.25rem; opacity: 0.6;
        }}
        #loading.hidden {{ display: none; }}
        #error {{
            position: fixed; inset: 0; display: none;
            align-items: center; justify-content: center;
            flex-direction: column; gap: 1rem; padding: 2rem; text-align: center;
        }}
        #error.visible {{ display: flex; }}
        #error h2 {{ color: #f38ba8; }}
        #error pre {{
            max-width: 80ch; overflow: auto; background: #181825;
            padding: 1rem; border-radius: 0.5rem; font-size: 0.875rem; text-align: left;
        }}
    </style>
</head>
<body>
    <div id="loading">Loading GPUI example: {name}…</div>
    <div id="error">
        <h2>Failed to initialize</h2>
        <pre id="error-message"></pre>
    </div>
    <script type="module">
        const loadingEl = document.getElementById('loading');
        const errorEl = document.getElementById('error');
        const errorMsgEl = document.getElementById('error-message');
        function showError(message) {{
            loadingEl.classList.add('hidden');
            errorMsgEl.textContent = message;
            errorEl.classList.add('visible');
        }}
        async function run() {{
            try {{
                const {{ default: init }} = await import('./pkg/{name}.js');
                await init();
                loadingEl.classList.add('hidden');
            }} catch (err) {{
                console.error('GPUI Web initialization failed:', err);
                showError(String(err));
            }}
        }}
        run();
    </script>
</body>
</html>
"#
    )
}

fn make_picker_html(examples: &[&str]) -> String {
    let mut items = String::new();
    for name in examples {
        items.push_str(&format!(
            r#"                <button class="example-btn" data-name="{name}">{name}</button>
"#
        ));
    }

    let first = examples.first().copied().unwrap_or("hello_world");

    format!(
        r##"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>GPUI Web Examples</title>
    <style>
        * {{ margin: 0; padding: 0; box-sizing: border-box; }}
        html, body {{
            width: 100%; height: 100%; overflow: hidden;
            background: #1e1e2e; color: #cdd6f4;
            font-family: system-ui, -apple-system, sans-serif;
        }}
        #app {{
            display: flex; width: 100%; height: 100%;
        }}

        /* ---- sidebar ---- */
        #sidebar {{
            width: 240px; min-width: 240px;
            background: #181825;
            border-right: 1px solid #313244;
            display: flex; flex-direction: column;
            overflow: hidden;
        }}
        #sidebar-header {{
            padding: 16px 14px 12px;
            font-size: 0.8rem; font-weight: 700;
            text-transform: uppercase; letter-spacing: 0.08em;
            color: #a6adc8; border-bottom: 1px solid #313244;
            flex-shrink: 0;
        }}
        #sidebar-header span {{
            font-size: 1rem; text-transform: none; letter-spacing: normal;
            color: #cdd6f4; display: block; margin-top: 2px;
        }}
        #example-list {{
            flex: 1; overflow-y: auto; padding: 8px 0;
        }}
        #example-list::-webkit-scrollbar {{ width: 6px; }}
        #example-list::-webkit-scrollbar-track {{ background: transparent; }}
        #example-list::-webkit-scrollbar-thumb {{ background: #45475a; border-radius: 3px; }}

        .example-btn {{
            display: block; width: 100%;
            padding: 8px 14px; border: none;
            background: transparent; color: #bac2de;
            font-size: 0.85rem; text-align: left;
            cursor: pointer; transition: background 0.1s, color 0.1s;
            font-family: 'SF Mono', 'Fira Code', 'Cascadia Code', monospace;
        }}
        .example-btn:hover {{
            background: #313244; color: #cdd6f4;
        }}
        .example-btn.active {{
            background: #45475a; color: #f5e0dc;
            font-weight: 600;
        }}

        /* ---- main area ---- */
        #main {{
            flex: 1; display: flex; flex-direction: column;
            min-width: 0;
        }}
        #toolbar {{
            height: 40px; min-height: 40px;
            display: flex; align-items: center;
            padding: 0 16px; gap: 12px;
            background: #1e1e2e;
            border-bottom: 1px solid #313244;
            font-size: 0.8rem; color: #a6adc8;
        }}
        #current-name {{
            font-weight: 600; color: #cdd6f4;
            font-family: 'SF Mono', 'Fira Code', 'Cascadia Code', monospace;
        }}
        #open-tab {{
            margin-left: auto;
            padding: 4px 10px;
            border: 1px solid #585b70; border-radius: 4px;
            background: transparent; color: #a6adc8;
            font-size: 0.75rem; cursor: pointer;
            transition: background 0.1s, color 0.1s;
            text-decoration: none;
        }}
        #open-tab:hover {{
            background: #313244; color: #cdd6f4;
        }}
        #viewer {{
            flex: 1; border: none; width: 100%; background: #11111b;
        }}
    </style>
</head>
<body>
    <div id="app">
        <div id="sidebar">
            <div id="sidebar-header">
                GPUI Examples
                <span>{count} available</span>
            </div>
            <div id="example-list">
{items}            </div>
        </div>
        <div id="main">
            <div id="toolbar">
                <span id="current-name">{first}</span>
                <a id="open-tab" href="./{first}/" target="_blank">Open in new tab ↗</a>
            </div>
            <iframe id="viewer" src="./{first}/"></iframe>
        </div>
    </div>
    <script>
        const buttons = document.querySelectorAll('.example-btn');
        const viewer  = document.getElementById('viewer');
        const nameEl  = document.getElementById('current-name');
        const openEl  = document.getElementById('open-tab');

        function select(name) {{
            buttons.forEach(b => b.classList.toggle('active', b.dataset.name === name));
            viewer.src = './' + name + '/';
            nameEl.textContent = name;
            openEl.href = './' + name + '/';
            history.replaceState(null, '', '#' + name);
        }}

        buttons.forEach(b => b.addEventListener('click', () => select(b.dataset.name)));

        // restore from hash
        const hash = location.hash.slice(1);
        if (hash && [...buttons].some(b => b.dataset.name === hash)) {{
            select(hash);
        }} else {{
            select('{first}');
        }}
    </script>
</body>
</html>
"##,
        count = examples.len(),
    )
}
