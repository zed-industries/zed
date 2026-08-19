#![allow(clippy::disallowed_methods, reason = "tooling is exempt")]

use std::io::Write;
use std::path::Path;
use std::process::Command;

use anyhow::{Context as _, Result, bail};
use clap::Parser;

#[derive(Parser)]
pub struct WebExamplesArgs {
    #[arg(long)]
    pub release: bool,
    #[arg(long, default_value = "8080")]
    pub port: u16,
    #[arg(long)]
    pub no_serve: bool,
}

fn check_program(binary: &str, install_hint: &str) -> Result<()> {
    match Command::new(binary).arg("--version").output() {
        Ok(output) if output.status.success() => Ok(()),
        _ => bail!("`{binary}` not found. Install with: {install_hint}"),
    }
}

fn discover_examples() -> Result<Vec<String>> {
    let examples_dir = Path::new("crates/gpui/examples");
    let mut names = Vec::new();

    for entry in std::fs::read_dir(examples_dir).context("failed to read crates/gpui/examples")? {
        let path = entry?.path();
        if path.extension().and_then(|e| e.to_str()) == Some("rs") {
            if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                names.push(stem.to_string());
            }
        }
    }

    if names.is_empty() {
        bail!("no examples found in crates/gpui/examples");
    }

    names.sort();
    Ok(names)
}

pub fn run_web_examples(args: WebExamplesArgs) -> Result<()> {
    let cargo = std::env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
    let profile = if args.release { "release" } else { "debug" };
    let out_dir = "target/web-examples";

    check_program("wasm-bindgen", "cargo install wasm-bindgen-cli")?;

    let examples = discover_examples()?;
    eprintln!(
        "Building {} example(s) for wasm32-unknown-unknown ({profile})...\n",
        examples.len()
    );

    std::fs::create_dir_all(out_dir).context("failed to create output directory")?;

    eprintln!("Building all examples...");

    let mut cmd = Command::new(&cargo);
    cmd.args([
        "build",
        "--target",
        "wasm32-unknown-unknown",
        "-p",
        "gpui",
        "--keep-going",
    ]);
    // 🙈
    cmd.env("RUSTC_BOOTSTRAP", "1");
    for name in &examples {
        cmd.args(["--example", name]);
    }
    if args.release {
        cmd.arg("--release");
    }

    let _ = cmd.status().context("failed to run cargo build")?;

    // Run wasm-bindgen on each .wasm that was produced.
    let mut succeeded: Vec<String> = Vec::new();
    let mut failed: Vec<String> = Vec::new();

    for name in &examples {
        let wasm_path = format!("target/wasm32-unknown-unknown/{profile}/examples/{name}.wasm");
        if !Path::new(&wasm_path).exists() {
            eprintln!("[{name}] SKIPPED (build failed)");
            failed.push(name.clone());
            continue;
        }

        eprintln!("[{name}] Running wasm-bindgen...");

        let example_dir = format!("{out_dir}/{name}");
        std::fs::create_dir_all(&example_dir)
            .with_context(|| format!("failed to create {example_dir}"))?;

        let status = Command::new("wasm-bindgen")
            .args([
                &wasm_path,
                "--target",
                "web",
                "--no-typescript",
                "--out-dir",
                &example_dir,
                "--out-name",
                name,
            ])
            // 🙈
            .env("RUSTC_BOOTSTRAP", "1")
            .status()
            .context("failed to run wasm-bindgen")?;
        if !status.success() {
            eprintln!("[{name}] SKIPPED (wasm-bindgen failed)");
            failed.push(name.clone());
            continue;
        }

        // Write per-example index.html.
        let html_path = format!("{example_dir}/index.html");
        std::fs::File::create(&html_path)
            .and_then(|mut file| file.write_all(make_example_html(name).as_bytes()))
            .with_context(|| format!("failed to write {html_path}"))?;

        eprintln!("[{name}] OK");
        succeeded.push(name.clone());
    }

    if succeeded.is_empty() {
        bail!("all {} examples failed to build", examples.len());
    }

    let example_names: Vec<&str> = succeeded.iter().map(|s| s.as_str()).collect();
    let index_path = format!("{out_dir}/index.html");
    std::fs::File::create(&index_path)
        .and_then(|mut file| file.write_all(make_gallery_html(&example_names).as_bytes()))
        .context("failed to write index.html")?;

    if args.no_serve {
        return Ok(());
    }

    // Serve with COEP/COOP headers required for WebGPU / SharedArrayBuffer.
    eprintln!("Serving on http://127.0.0.1:{}...", args.port);

    let server_script = format!(
        r#"
import http.server
class Handler(http.server.SimpleHTTPRequestHandler):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, directory="{out_dir}", **kwargs)
    def end_headers(self):
        self.send_header("Cross-Origin-Embedder-Policy", "require-corp")
        self.send_header("Cross-Origin-Opener-Policy", "same-origin")
        super().end_headers()
http.server.HTTPServer(("127.0.0.1", {port}), Handler).serve_forever()
"#,
        port = args.port,
    );

    let status = Command::new("python3")
        .args(["-c", &server_script])
        .status()
        .context("failed to run python3 http server (is python3 installed?)")?;
    if !status.success() {
        bail!("python3 http server exited with: {status}");
    }

    Ok(())
}

fn make_example_html(name: &str) -> String {
    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>GPUI Web: {name}</title>
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
    </style>
</head>
<body>
    <div id="loading">Loading {name}…</div>
    <script type="module">
        import init from './{name}.js';
        await init();
        document.getElementById('loading').classList.add('hidden');
    </script>
</body>
</html>
"#
    )
}

fn make_gallery_html(examples: &[&str]) -> String {
    let mut buttons = String::new();
    for name in examples {
        buttons.push_str(&format!(
            "                <button class=\"example-btn\" data-name=\"{name}\">{name}</button>\n"
        ));
    }

    let first = examples.first().copied().unwrap_or("hello_web");

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
        #app {{ display: flex; width: 100%; height: 100%; }}

        #sidebar {{
            width: 240px; min-width: 240px;
            background: #181825;
            border-right: 1px solid #313244;
            display: flex; flex-direction: column;
        }}
        #sidebar-header {{
            padding: 16px 14px 12px;
            font-size: 0.8rem; font-weight: 700;
            text-transform: uppercase; letter-spacing: 0.08em;
            color: #a6adc8; border-bottom: 1px solid #313244;
        }}
        #sidebar-header span {{
            font-size: 1rem; text-transform: none; letter-spacing: normal;
            color: #cdd6f4; display: block; margin-top: 2px;
        }}
        #example-list {{
            flex: 1; overflow-y: auto; padding: 8px 0;
        }}
        .example-btn {{
            display: block; width: 100%;
            padding: 8px 14px; border: none;
            background: transparent; color: #bac2de;
            font-size: 0.85rem; text-align: left;
            cursor: pointer;
            font-family: 'SF Mono', 'Fira Code', 'Cascadia Code', monospace;
        }}
        .example-btn:hover {{ background: #313244; color: #cdd6f4; }}
        .example-btn.active {{ background: #45475a; color: #f5e0dc; font-weight: 600; }}

        #main {{ flex: 1; display: flex; flex-direction: column; min-width: 0; }}
        #toolbar {{
            height: 40px; display: flex; align-items: center;
            padding: 0 16px; gap: 12px;
            background: #1e1e2e; border-bottom: 1px solid #313244;
            font-size: 0.8rem; color: #a6adc8;
        }}
        #current-name {{
            font-weight: 600; color: #cdd6f4;
            font-family: 'SF Mono', 'Fira Code', 'Cascadia Code', monospace;
        }}
        #open-tab {{
            margin-left: auto; padding: 4px 10px;
            border: 1px solid #585b70; border-radius: 4px;
            background: transparent; color: #a6adc8;
            font-size: 0.75rem; cursor: pointer;
            text-decoration: none;
        }}
        #open-tab:hover {{ background: #313244; color: #cdd6f4; }}
        #viewer {{ flex: 1; border: none; width: 100%; background: #11111b; }}
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
{buttons}            </div>
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
