//! Benchmarks for the typst preview pipeline.
//!
//! ## `bench_preview_loop` (cold compile)
//! Uses `tinymist compile --format svg` in a loop.  Each iteration is a
//! cold compile (no memoization).  Measures compile + rasterize.
//!
//! ## `bench_preview_lsp` (warm compile with comemo)
//! Drives tinymist LSP over stdin/stdout, opens the document, starts the
//! preview server, then sends `textDocument/didChange` edits over the LSP
//! and receives SVGs over the WebSocket data plane.  This exercises the
//! real incremental compilation path with comemo memoization — the same
//! path used when the user types in the editor.
//!
//! Run with:
//!   cargo test --package typst_viewer --release -- bench_preview --nocapture --ignored
//!
//! The tests are `#[ignore]` so they don't run in CI (requires tinymist
//! binary + a real .typ file on disk).
//!
//! Environment variables:
//!   TINYMIST_BIN      — path to tinymist binary (default: ~/src/semitenn/tinymist/target/release/tinymist)
//!   TYPST_BENCH_FILE  — path to .typ document (default: ~/Documents/Law/David/deepdives.typ)
//!   BENCH_ITERS       — number of edit-compile-rasterize iterations (default: 10)

#[cfg(test)]
mod tests {
    use std::io::Write;
    use std::path::{Path, PathBuf};
    use std::process::Command;
    use std::time::{Duration, Instant};

    use async_tungstenite::tungstenite::Message;
    use futures::StreamExt;

    use crate::typst_viewer_view::{
        inject_glyph_defs, parse_page_header, GLYPH_DEFS_OPEN, DEFS_CLOSE,
        rasterize_svg_to_image,
    };

    // -----------------------------------------------------------------------
    // Configuration
    // -----------------------------------------------------------------------

    fn tinymist_bin() -> PathBuf {
        if let Ok(p) = std::env::var("TINYMIST_BIN") {
            return PathBuf::from(p);
        }
        if let Some(home) = home_dir() {
            let candidate = home.join("src/semitenn/tinymist/target/release/tinymist");
            if candidate.exists() {
                return candidate;
            }
        }
        for dir in std::env::var("PATH").unwrap_or_default().split(':') {
            let candidate = PathBuf::from(dir).join("tinymist");
            if candidate.exists() {
                return candidate;
            }
        }
        panic!(
            "tinymist binary not found. Set TINYMIST_BIN env var, or ensure \
             ~/src/semitenn/tinymist/target/release/tinymist exists, or tinymist is in PATH."
        );
    }

    fn find_test_document() -> PathBuf {
        if let Ok(p) = std::env::var("TYPST_BENCH_FILE") {
            let path = PathBuf::from(p);
            assert!(path.exists(), "TYPST_BENCH_FILE does not exist: {path:?}");
            return path;
        }
        if let Some(home) = home_dir() {
            let candidates = [
                home.join("Documents/Law/David/deepdives.typ"),
                home.join("Documents/Law/FoL/Assignments/1.2 S26 NASA/newton-principia-acoustica.typ"),
            ];
            for c in &candidates {
                if c.exists() {
                    return c.clone();
                }
            }
        }
        panic!(
            "No test document found. Set TYPST_BENCH_FILE env var or place a .typ \
             file at ~/Documents/Law/David/deepdives.typ"
        );
    }

    fn home_dir() -> Option<PathBuf> {
        std::env::var("HOME").ok().map(PathBuf::from)
    }

    // -----------------------------------------------------------------------
    // Helpers
    // -----------------------------------------------------------------------

    fn replace_line(content: &str, line_idx: usize, new_line: &str) -> String {
        content
            .lines()
            .enumerate()
            .map(|(i, l)| if i == line_idx { new_line } else { l })
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn find_heading_line(content: &str) -> Option<(usize, String)> {
        for (i, line) in content.lines().enumerate() {
            let trimmed = line.trim();
            if trimmed.starts_with('=') && trimmed.len() > 5 {
                return Some((i, line.to_string()));
            }
        }
        for (i, line) in content.lines().enumerate() {
            let trimmed = line.trim();
            if trimmed.len() > 20
                && !trimmed.starts_with("//")
                && !trimmed.starts_with('#')
                && trimmed.chars().any(|c| c.is_alphabetic())
            {
                return Some((i, line.to_string()));
            }
        }
        None
    }

    fn copy_dir_shallow(src: &Path, dst: &Path) {
        std::fs::create_dir_all(dst).expect("create dest dir");
        let entries = std::fs::read_dir(src).expect("read source dir");
        for entry in entries {
            let entry = entry.expect("dir entry");
            let file_type = entry.file_type().expect("file type");
            if file_type.is_file() {
                let dest = dst.join(entry.file_name());
                std::fs::copy(entry.path(), &dest).expect("copy file");
            }
        }
    }

    /// Extract and cache glyph defs from an SVG (single-cache, for benchmarks).
    fn cache_defs(svg_bytes: &[u8], cache: &mut Option<String>) {
        let svg_str = String::from_utf8_lossy(svg_bytes);
        if let Some(start) = svg_str.find(GLYPH_DEFS_OPEN) {
            if let Some(end_offset) = svg_str[start..].find(DEFS_CLOSE) {
                let defs_end = start + end_offset + DEFS_CLOSE.len();
                *cache = Some(svg_str[start..defs_end].to_string());
            }
        }
    }

    fn rasterize_full(svg_bytes: &[u8], scale: f32) -> anyhow::Result<Duration> {
        let start = Instant::now();
        let _image = rasterize_svg_to_image(svg_bytes, scale)?;
        Ok(start.elapsed())
    }

    /// Compile a .typ file to SVG using tinymist and return page 1's SVG bytes.
    fn compile_to_svg(
        bin: &Path,
        doc_path: &Path,
        work_dir: &Path,
    ) -> anyhow::Result<(Vec<u8>, Duration)> {
        let svg_output = work_dir.join("__bench_output_{p}.svg");
        let start = Instant::now();
        let output = Command::new(bin)
            .arg("compile")
            .arg("--root")
            .arg(work_dir)
            .arg("--format")
            .arg("svg")
            .arg(doc_path)
            .arg(svg_output.to_str().unwrap())
            .current_dir(work_dir)
            .output()
            .map_err(|e| anyhow::anyhow!("failed to run tinymist compile: {e}"))?;
        let compile_dur = start.elapsed();

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("tinymist compile failed ({}): {stderr}", output.status);
        }

        let page1_path = work_dir.join("__bench_output_1.svg");
        let svg_bytes = std::fs::read(&page1_path)
            .map_err(|e| anyhow::anyhow!("failed to read page 1 SVG at {page1_path:?}: {e}"))?;

        // Clean up SVG files.
        for entry in std::fs::read_dir(work_dir).into_iter().flatten() {
            if let Ok(entry) = entry {
                let name = entry.file_name();
                let name = name.to_string_lossy();
                if name.starts_with("__bench_output_") && name.ends_with(".svg") {
                    let _ = std::fs::remove_file(entry.path());
                }
            }
        }

        Ok((svg_bytes, compile_dur))
    }

    /// Receive the next SVG from the WebSocket, skipping binary/non-SVG messages.
    async fn receive_ws_svg(
        ws: &mut crate::svg_stream::PreviewSocket,
    ) -> anyhow::Result<Vec<u8>> {
        let deadline = Instant::now() + Duration::from_secs(30);
        loop {
            if Instant::now() > deadline {
                anyhow::bail!("timeout waiting for SVG (30s)");
            }
            match ws.next().await {
                Some(Ok(Message::Text(text))) => {
                    let svg_text = if let Some((_, svg)) = parse_page_header(&text) {
                        svg.to_string()
                    } else if text.contains("<svg") {
                        text.to_string()
                    } else {
                        continue;
                    };
                    if svg_text.contains("<svg") {
                        return Ok(svg_text.into_bytes());
                    }
                }
                Some(Ok(Message::Binary(_))) => continue,
                Some(Ok(Message::Close(f))) => anyhow::bail!("WebSocket closed: {f:?}"),
                Some(Ok(_)) => continue,
                Some(Err(e)) => anyhow::bail!("WebSocket error: {e}"),
                None => anyhow::bail!("WebSocket stream ended"),
            }
        }
    }

    // -----------------------------------------------------------------------
    // Result + summary
    // -----------------------------------------------------------------------

    struct IterResult {
        compile_ms: f64,
        svg_bytes: usize,
        raster_ms: f64,
        has_defs: bool,
    }

    impl std::fmt::Display for IterResult {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "compile={:6.1}ms  svg={:7}B  raster={:6.1}ms  defs={}",
                self.compile_ms,
                self.svg_bytes,
                self.raster_ms,
                if self.has_defs { "Y" } else { "N" },
            )
        }
    }

    fn print_summary(results: &[IterResult], label: &str) {
        eprintln!();
        eprintln!("=== Summary: {label} ({} iterations) ===", results.len());
        if results.is_empty() {
            return;
        }
        let avg = |f: fn(&IterResult) -> f64| -> f64 {
            results.iter().map(f).sum::<f64>() / results.len() as f64
        };
        let sorted = |f: fn(&IterResult) -> f64| -> Vec<f64> {
            let mut v: Vec<f64> = results.iter().map(f).collect();
            v.sort_by(|a, b| a.partial_cmp(b).unwrap());
            v
        };
        let p50 = |f: fn(&IterResult) -> f64| sorted(f)[results.len() / 2];
        let p95 = |f: fn(&IterResult) -> f64| sorted(f)[(results.len() as f64 * 0.95) as usize];
        let min = |f: fn(&IterResult) -> f64| -> f64 {
            results.iter().map(f).fold(f64::INFINITY, f64::min)
        };

        eprintln!("              {:>8} {:>8} {:>8} {:>8}", "avg", "p50", "p95", "min");
        eprintln!(
            "compile:      {:8.1} {:8.1} {:8.1} {:8.1} ms",
            avg(|r| r.compile_ms), p50(|r| r.compile_ms),
            p95(|r| r.compile_ms), min(|r| r.compile_ms),
        );
        eprintln!(
            "raster:       {:8.1} {:8.1} {:8.1} {:8.1} ms",
            avg(|r| r.raster_ms), p50(|r| r.raster_ms),
            p95(|r| r.raster_ms), min(|r| r.raster_ms),
        );
        eprintln!(
            "total:        {:8.1} {:8.1} {:8.1} {:8.1} ms",
            avg(|r| r.compile_ms + r.raster_ms),
            p50(|r| r.compile_ms + r.raster_ms),
            p95(|r| r.compile_ms + r.raster_ms),
            min(|r| r.compile_ms + r.raster_ms),
        );
        let avg_svg_kb = results.iter().map(|r| r.svg_bytes as f64).sum::<f64>()
            / results.len() as f64
            / 1024.0;
        eprintln!("avg SVG size: {avg_svg_kb:.1} KB");
        let defs_count = results.iter().filter(|r| r.has_defs).count();
        eprintln!(
            "defs present: {defs_count}/{} frames ({:.0}%)",
            results.len(),
            defs_count as f64 / results.len() as f64 * 100.0,
        );
    }

    // -----------------------------------------------------------------------
    // Shared edit loop logic
    // -----------------------------------------------------------------------

    fn chop_heading(current: &mut String, original: &str) {
        let trimmed = current.trim_end();
        if trimmed.len() <= 3 {
            *current = original.to_string();
        } else {
            let mut chars: Vec<char> = trimmed.chars().collect();
            chars.pop();
            *current = chars.into_iter().collect();
        }
    }

    // ===================================================================
    // bench_preview_loop — cold compile (no memoization)
    // ===================================================================

    #[test]
    #[ignore]
    fn bench_preview_loop() {
        let iterations = std::env::var("BENCH_ITERS")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(10usize);
        let scale = 2.0_f32;
        let bin = tinymist_bin();
        let doc_path = find_test_document();

        eprintln!("=== Typst Preview Benchmark (cold compile) ===");
        eprintln!("tinymist:   {}", bin.display());
        eprintln!("document:   {}", doc_path.display());
        eprintln!("iterations: {iterations}");
        eprintln!();

        let tmp_dir = std::env::temp_dir().join(format!("typst_bench_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&tmp_dir);
        let src_dir = doc_path.parent().expect("document has no parent dir");
        copy_dir_shallow(src_dir, &tmp_dir);
        let work_doc = tmp_dir.join(doc_path.file_name().unwrap());
        assert!(work_doc.exists(), "working copy not found: {work_doc:?}");

        let original_content = std::fs::read_to_string(&work_doc).expect("read document");
        let (heading_line_idx, heading_line) =
            find_heading_line(&original_content).expect("document has no heading to mutate");
        eprintln!("mutating line {heading_line_idx}: {heading_line}");

        // Warmup compile.
        let (initial_svg, initial_dur) =
            compile_to_svg(&bin, &work_doc, &tmp_dir).expect("initial compile");
        eprintln!(
            "initial compile: {:.1}ms, SVG: {} bytes",
            initial_dur.as_secs_f64() * 1000.0,
            initial_svg.len(),
        );
        let warmup = rasterize_full(&initial_svg, scale);
        eprintln!(
            "warmup rasterize: {:.1}ms",
            warmup.map(|d| d.as_secs_f64() * 1000.0).unwrap_or(-1.0),
        );
        eprintln!();

        let mut results: Vec<IterResult> = Vec::with_capacity(iterations);
        let mut current_heading = heading_line.clone();
        let mut cached_defs: Option<String> = None;
        cache_defs(&initial_svg, &mut cached_defs);

        for i in 0..iterations {
            chop_heading(&mut current_heading, &heading_line);
            let new_content = replace_line(&original_content, heading_line_idx, &current_heading);
            {
                let mut f = std::fs::OpenOptions::new()
                    .write(true)
                    .truncate(true)
                    .open(&work_doc)
                    .expect("open document for writing");
                f.write_all(new_content.as_bytes()).expect("write document");
                f.sync_all().expect("fsync document");
            }

            let (svg_bytes, compile_dur) = match compile_to_svg(&bin, &work_doc, &tmp_dir) {
                Ok(r) => r,
                Err(e) => {
                    eprintln!("iter {i}: compile error: {e}");
                    break;
                }
            };

            let has_defs = std::str::from_utf8(&svg_bytes)
                .map(|s| s.contains(r#"<defs id="glyph">"#))
                .unwrap_or(false);

            let mut raster_svg = svg_bytes.clone();
            if has_defs {
                cache_defs(&svg_bytes, &mut cached_defs);
            } else if let Some(ref defs) = cached_defs {
                raster_svg = inject_glyph_defs(&raster_svg, defs);
            }

            let raster_dur = rasterize_full(&raster_svg, scale).unwrap_or(Duration::ZERO);

            let result = IterResult {
                compile_ms: compile_dur.as_secs_f64() * 1000.0,
                svg_bytes: raster_svg.len(),
                raster_ms: raster_dur.as_secs_f64() * 1000.0,
                has_defs,
            };
            eprintln!("iter {i:2}: {result}");
            results.push(result);
        }

        print_summary(&results, "cold compile");
        let _ = std::fs::remove_dir_all(&tmp_dir);
    }

    // ===================================================================
    // bench_preview_lsp — warm compile with comemo memoization
    // ===================================================================

    #[test]
    #[ignore]
    fn bench_preview_lsp() {
        let iterations = std::env::var("BENCH_ITERS")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(10usize);
        let scale = 2.0_f32;
        let bin = tinymist_bin();
        let doc_path = find_test_document();

        eprintln!("=== Typst Preview Benchmark (LSP + comemo) ===");
        eprintln!("tinymist:   {}", bin.display());
        eprintln!("document:   {}", doc_path.display());
        eprintln!("iterations: {iterations}");
        eprintln!();

        let tmp_dir = std::env::temp_dir().join(format!("typst_bench_lsp_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&tmp_dir);
        let src_dir = doc_path.parent().expect("document has no parent dir");
        copy_dir_shallow(src_dir, &tmp_dir);
        let work_doc = tmp_dir.join(doc_path.file_name().unwrap());
        assert!(work_doc.exists(), "working copy not found: {work_doc:?}");

        let original_content = std::fs::read_to_string(&work_doc).expect("read document");
        let (heading_line_idx, heading_line) =
            find_heading_line(&original_content).expect("document has no heading to mutate");
        eprintln!("mutating line {heading_line_idx}: {heading_line}");

        let root_uri = format!("file://{}", tmp_dir.display());
        let doc_uri = format!("file://{}", work_doc.display());

        smol::block_on(async {
            let mut lsp = LspProcess::start(&bin, &tmp_dir);

            // Initialize LSP.
            let init_resp = lsp.request(1, "initialize", serde_json::json!({
                "processId": std::process::id(),
                "rootUri": root_uri,
                "capabilities": {
                    "textDocument": {
                        "synchronization": {
                            "didSave": true,
                            "dynamicRegistration": false
                        }
                    }
                },
                "initializationOptions": {
                    "formatterMode": "disable"
                }
            }));
            assert!(init_resp.get("result").is_some(), "LSP init failed: {init_resp:?}");
            lsp.notify("initialized", serde_json::json!({}));

            // Open the document.
            lsp.notify("textDocument/didOpen", serde_json::json!({
                "textDocument": {
                    "uri": doc_uri,
                    "languageId": "typst",
                    "version": 1,
                    "text": original_content,
                }
            }));

            smol::Timer::after(Duration::from_millis(500)).await;

            // Start preview.
            let preview_resp = lsp.request(2, "workspace/executeCommand", serde_json::json!({
                "command": "tinymist.doStartPreview",
                "arguments": [[
                    "--server-svg",
                    "--strip-svg-glyph-defs",
                    "--data-plane-host=127.0.0.1:0",
                    work_doc.to_str().unwrap()
                ]]
            }));
            let preview_result = preview_resp.get("result")
                .expect("doStartPreview returned no result");
            let data_plane_port = preview_result.get("dataPlanePort")
                .and_then(|v| v.as_u64())
                .expect("no dataPlanePort in response");
            eprintln!("preview data plane port: {data_plane_port}");

            // Connect WebSocket.
            let ws_url = format!("ws://127.0.0.1:{data_plane_port}");
            let mut ws = crate::svg_stream::connect(&ws_url)
                .await
                .expect("WebSocket connect");

            use futures::SinkExt;
            ws.send(Message::text("current")).await.expect("send current");

            let initial_svg = receive_ws_svg(&mut ws).await.expect("receive initial SVG");
            eprintln!("initial SVG: {} bytes", initial_svg.len());

            let mut cached_defs: Option<String> = None;
            cache_defs(&initial_svg, &mut cached_defs);

            let warmup = rasterize_full(&initial_svg, scale);
            eprintln!(
                "warmup rasterize: {:.1}ms",
                warmup.map(|d| d.as_secs_f64() * 1000.0).unwrap_or(-1.0),
            );
            eprintln!();

            // --- Benchmark loop ---
            let mut results: Vec<IterResult> = Vec::with_capacity(iterations);
            let mut current_heading = heading_line.clone();
            let mut version = 2i64;

            for i in 0..iterations {
                chop_heading(&mut current_heading, &heading_line);
                let new_content =
                    replace_line(&original_content, heading_line_idx, &current_heading);

                let change_start = Instant::now();
                lsp.notify("textDocument/didChange", serde_json::json!({
                    "textDocument": {
                        "uri": doc_uri,
                        "version": version,
                    },
                    "contentChanges": [{
                        "text": new_content,
                    }]
                }));
                version += 1;

                let svg_bytes = receive_ws_svg(&mut ws).await
                    .unwrap_or_else(|e| panic!("iter {i}: SVG receive error: {e}"));
                let compile_dur = change_start.elapsed();

                let has_defs = std::str::from_utf8(&svg_bytes)
                    .map(|s| s.contains(r#"<defs id="glyph">"#))
                    .unwrap_or(false);

                let mut raster_svg = svg_bytes.clone();
                if has_defs {
                    cache_defs(&svg_bytes, &mut cached_defs);
                } else if let Some(ref defs) = cached_defs {
                    raster_svg = inject_glyph_defs(&raster_svg, defs);
                }

                let raster_dur = rasterize_full(&raster_svg, scale).unwrap_or(Duration::ZERO);

                let result = IterResult {
                    compile_ms: compile_dur.as_secs_f64() * 1000.0,
                    svg_bytes: raster_svg.len(),
                    raster_ms: raster_dur.as_secs_f64() * 1000.0,
                    has_defs,
                };
                eprintln!("iter {i:2}: {result}");
                results.push(result);
            }

            // Shutdown.
            let _ = lsp.request(99, "shutdown", serde_json::json!(null));
            lsp.notify("exit", serde_json::json!(null));
            drop(lsp);

            print_summary(&results, "LSP + comemo");
        });

        let _ = std::fs::remove_dir_all(&tmp_dir);
    }

    // ===================================================================
    // Minimal LSP client over stdin/stdout
    // ===================================================================

    struct LspProcess {
        child: std::process::Child,
        stdin: std::io::BufWriter<std::process::ChildStdin>,
        stdout: std::io::BufReader<std::process::ChildStdout>,
    }

    impl LspProcess {
        fn start(bin: &Path, cwd: &Path) -> Self {
            let mut child = std::process::Command::new(bin)
                .arg("lsp")
                .current_dir(cwd)
                .stdin(std::process::Stdio::piped())
                .stdout(std::process::Stdio::piped())
                .stderr(std::process::Stdio::piped())
                .spawn()
                .expect("failed to start tinymist lsp");

            let stdin = std::io::BufWriter::new(child.stdin.take().expect("no stdin"));
            let stdout = std::io::BufReader::new(child.stdout.take().expect("no stdout"));

            let stderr = child.stderr.take().expect("no stderr");
            std::thread::spawn(move || {
                use std::io::BufRead;
                let reader = std::io::BufReader::new(stderr);
                for line in reader.lines() {
                    if let Ok(line) = line {
                        if line.contains("ERROR") || line.contains("WARN") {
                            eprintln!("  lsp stderr: {line}");
                        }
                    }
                }
            });

            Self { child, stdin, stdout }
        }

        fn send_raw(&mut self, msg: &serde_json::Value) {
            let body = serde_json::to_string(msg).expect("serialize JSON-RPC");
            let header = format!("Content-Length: {}\r\n\r\n", body.len());
            self.stdin.write_all(header.as_bytes()).expect("write header");
            self.stdin.write_all(body.as_bytes()).expect("write body");
            self.stdin.flush().expect("flush stdin");
        }

        fn read_msg(&mut self) -> serde_json::Value {
            use std::io::BufRead;
            let mut content_length: Option<usize> = None;
            loop {
                let mut line = String::new();
                self.stdout.read_line(&mut line).expect("read header line");
                let trimmed = line.trim();
                if trimmed.is_empty() {
                    break;
                }
                if let Some(val) = trimmed.strip_prefix("Content-Length: ") {
                    content_length = Some(val.trim().parse().expect("parse Content-Length"));
                }
            }
            let length = content_length.expect("no Content-Length header");
            let mut body = vec![0u8; length];
            std::io::Read::read_exact(&mut self.stdout, &mut body).expect("read body");
            serde_json::from_slice(&body).expect("parse JSON-RPC response")
        }

        fn request(
            &mut self,
            id: i64,
            method: &str,
            params: serde_json::Value,
        ) -> serde_json::Value {
            self.send_raw(&serde_json::json!({
                "jsonrpc": "2.0",
                "id": id,
                "method": method,
                "params": params,
            }));
            loop {
                let msg = self.read_msg();
                if msg.get("id").and_then(|v| v.as_i64()) == Some(id) {
                    return msg;
                }
            }
        }

        fn notify(&mut self, method: &str, params: serde_json::Value) {
            self.send_raw(&serde_json::json!({
                "jsonrpc": "2.0",
                "method": method,
                "params": params,
            }));
        }
    }

    impl Drop for LspProcess {
        fn drop(&mut self) {
            let _ = self.child.kill();
        }
    }
}