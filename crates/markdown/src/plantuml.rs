use anyhow::Context as _;
#[cfg(not(target_family = "wasm"))]
use async_lock::OnceCell as AsyncOnceCell;
use async_lock::Semaphore;
use collections::HashMap;
use flate2::{Compression, write::DeflateEncoder};
#[cfg(not(target_family = "wasm"))]
use futures::AsyncWriteExt as _;
use futures::{AsyncRead, AsyncReadExt as _};
use gpui::{
    AnyElement, Context, Entity, FutureExt as _, RenderImage, SMOOTH_SVG_SCALE_FACTOR, Task,
};
use http_client::{AsyncBody, HttpClient, Request};
use std::collections::BTreeMap;
use std::io::Write as _;
use std::ops::Range;
use std::path::Path;
use std::sync::{Arc, Mutex, OnceLock};
use std::time::Duration;
use ui::prelude::*;
use util::ResultExt as _;
#[cfg(not(target_family = "wasm"))]
use util::command::Stdio;

use crate::parser::{CodeBlockKind, MarkdownEvent, MarkdownTag};
use settings::PlantUmlRenderMode;

use super::{
    CopyButtonVisibility, Markdown, MarkdownStyle,
    diagram::{
        DiagramKind, DiagramRenderState, DiagramView, fenced_code_block_contents, truncate_message,
        update_diagram_cache,
    },
};

const PLANTUML_ALIASES: &[&str] = &["plantuml", "puml", "pu", "iuml", "wsd"];
const DEFAULT_SCALE: u32 = 100;
const MIN_SCALE: u32 = 10;
const MAX_SCALE: u32 = 500;
const RENDER_TIMEOUT: Duration = Duration::from_secs(15);
const MAX_CONCURRENT_RENDERS: usize = 2;
const MAX_DIAGRAMS_PER_DOCUMENT: usize = 16;
const PUBLIC_SERVER_BASE_URL: &str = "https://www.plantuml.com/plantuml";
const MAX_PUBLIC_SERVER_URL_BYTES: usize = 64 * 1024;
const MAX_SOURCE_BYTES: usize = 1024 * 1024;
const MAX_SVG_BYTES: usize = 16 * 1024 * 1024;
#[cfg(any(test, not(target_family = "wasm")))]
const MAX_DIAGNOSTIC_BYTES: usize = 64 * 1024;
const MAX_RASTER_PIXELS: f32 = (4 * 1024 * 1024) as f32;
const MAX_RASTER_DIMENSION: f32 = 4096.0;
#[cfg(any(test, not(target_family = "wasm")))]
const MIN_SECURE_PLANTUML_VERSION: (u32, u32, u32) = (1, 2020, 11);
static PLANTUML_RENDER_SEMAPHORE: Semaphore = Semaphore::new(MAX_CONCURRENT_RENDERS);

type PlantUmlDiagramCache =
    HashMap<ParsedMarkdownPlantUmlDiagramContents, Arc<CachedPlantUmlDiagram>>;

#[derive(Clone, Debug)]
pub(crate) struct ParsedMarkdownPlantUmlDiagram {
    pub(crate) content_range: Range<usize>,
    pub(crate) contents: ParsedMarkdownPlantUmlDiagramContents,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) struct ParsedMarkdownPlantUmlDiagramContents {
    pub(crate) contents: SharedString,
    pub(crate) scale: u32,
}

#[derive(Default, Clone)]
pub(crate) struct PlantUmlState {
    cache: PlantUmlDiagramCache,
    order: Vec<ParsedMarkdownPlantUmlDiagramContents>,
}

struct CachedPlantUmlDiagram {
    render_image: Arc<OnceLock<anyhow::Result<Arc<RenderImage>>>>,
    // Keep steady-state reads lock-free while allowing the completed render to release a large
    // fallback image immediately.
    pending_fallback_image: Arc<Mutex<Option<Arc<RenderImage>>>>,
    _task: Task<()>,
}

impl PlantUmlState {
    pub(crate) fn clear(&mut self) {
        self.cache.clear();
        self.order.clear();
    }

    pub(crate) fn update(
        &mut self,
        diagrams: &BTreeMap<usize, ParsedMarkdownPlantUmlDiagram>,
        render_mode: PlantUmlRenderMode,
        cx: &mut Context<Markdown>,
    ) {
        let new_order = diagrams
            .values()
            .map(|diagram| diagram.contents.clone())
            .collect::<Vec<_>>();

        update_diagram_cache(
            &mut self.cache,
            &mut self.order,
            new_order,
            |cached| {
                cached
                    .render_image
                    .get()
                    .and_then(|result| result.as_ref().ok().cloned())
                    .or_else(|| cached.fallback_image())
            },
            |contents, fallback_image| {
                CachedPlantUmlDiagram::new(contents, fallback_image, render_mode, cx)
            },
        );
    }
}

impl CachedPlantUmlDiagram {
    fn new(
        contents: ParsedMarkdownPlantUmlDiagramContents,
        fallback_image: Option<Arc<RenderImage>>,
        render_mode: PlantUmlRenderMode,
        cx: &mut Context<Markdown>,
    ) -> Self {
        let render_image = Arc::new(OnceLock::<anyhow::Result<Arc<RenderImage>>>::new());
        let pending_fallback_image = Arc::new(Mutex::new(fallback_image));
        let svg_renderer = cx.svg_renderer();
        let executor = cx.background_executor().clone();
        let http_client = cx.http_client();
        let dark_mode = !cx.theme().appearance.is_light();

        let task = cx.spawn({
            let render_image = render_image.clone();
            let pending_fallback_image = pending_fallback_image.clone();
            async move |this, cx| {
                let render_result = cx
                    .background_spawn(async move {
                        let _permit = PLANTUML_RENDER_SEMAPHORE.acquire().await;
                        let svg = render_plantuml_svg(
                            contents.contents.clone(),
                            dark_mode,
                            render_mode,
                            http_client,
                        )
                        .with_timeout(RENDER_TIMEOUT, &executor)
                        .await
                        .map_err(|_| anyhow::anyhow!("PlantUML rendering timed out"))??;
                        let scale = bounded_raster_scale(&svg, contents.scale as f32 / 100.0)?;
                        svg_renderer
                            .render_single_frame(&svg, scale)
                            .map_err(|error| anyhow::anyhow!("{error}"))
                    })
                    .await;
                if render_image.set(render_result).is_err() {
                    log::error!("attempted to set a PlantUML render result more than once");
                }
                pending_fallback_image
                    .lock()
                    .unwrap_or_else(|poisoned| poisoned.into_inner())
                    .take();
                this.update(cx, |_, cx| cx.notify()).log_err();
            }
        });

        Self {
            render_image,
            pending_fallback_image,
            _task: task,
        }
    }

    #[cfg(test)]
    fn new_for_test(
        render_image: Option<Arc<RenderImage>>,
        fallback_image: Option<Arc<RenderImage>>,
    ) -> Self {
        let render_image = Arc::new(match render_image {
            Some(render_image) => OnceLock::from(Ok(render_image)),
            None => OnceLock::new(),
        });
        Self {
            render_image,
            pending_fallback_image: Arc::new(Mutex::new(fallback_image)),
            _task: Task::ready(()),
        }
    }

    #[cfg(test)]
    fn new_error_for_test(error: anyhow::Error) -> Self {
        Self {
            render_image: Arc::new(OnceLock::from(Err(error))),
            pending_fallback_image: Arc::new(Mutex::new(None)),
            _task: Task::ready(()),
        }
    }

    fn fallback_image(&self) -> Option<Arc<RenderImage>> {
        self.pending_fallback_image
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .clone()
    }
}

#[cfg(not(target_family = "wasm"))]
fn new_plantuml_command() -> util::command::Command {
    let mut command = util::command::new_command("plantuml");
    command
        .env("PLANTUML_SECURITY_PROFILE", "SANDBOX")
        .env("PLANTUML_LIMIT_SIZE", "4096")
        .env("LANG", "C")
        .env("LC_ALL", "C")
        .kill_on_drop(true);
    command
}

async fn render_plantuml_svg(
    contents: SharedString,
    dark_mode: bool,
    render_mode: PlantUmlRenderMode,
    http_client: Arc<dyn HttpClient>,
) -> anyhow::Result<Vec<u8>> {
    anyhow::ensure!(
        contents.len() <= MAX_SOURCE_BYTES,
        "PlantUML source exceeds the 1 MiB limit"
    );

    match render_mode {
        PlantUmlRenderMode::Local => render_plantuml_local_svg(contents, dark_mode).await,
        PlantUmlRenderMode::PublicServer => {
            render_plantuml_server_svg(contents, dark_mode, http_client).await
        }
    }
}

#[cfg(not(target_family = "wasm"))]
async fn render_plantuml_local_svg(
    contents: SharedString,
    dark_mode: bool,
) -> anyhow::Result<Vec<u8>> {
    ensure_supported_plantuml().await?;

    let mut command = new_plantuml_command();
    command
        .args([
            "-pipe",
            "-tsvg",
            "-nometadata",
            "-failfast2",
            "-charset",
            "UTF-8",
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    if dark_mode {
        command.arg("-darkmode");
    }

    let mut child = command
        .spawn()
        .context("failed to start `plantuml`; install PlantUML and ensure it is on PATH")?;
    let mut stdin = child
        .stdin
        .take()
        .context("failed to open PlantUML stdin")?;
    let stdout = child
        .stdout
        .take()
        .context("failed to open PlantUML stdout")?;
    let stderr = child
        .stderr
        .take()
        .context("failed to open PlantUML stderr")?;
    let write_stdin = async move {
        stdin
            .write_all(contents.as_bytes())
            .await
            .context("failed to write PlantUML source")?;
        stdin
            .close()
            .await
            .context("failed to close PlantUML stdin")?;
        Ok::<(), anyhow::Error>(())
    };
    let wait_for_status = async {
        child
            .status()
            .await
            .context("failed while waiting for PlantUML")
    };
    let ((), stdout, stderr, status) = futures::try_join!(
        write_stdin,
        read_stream_limited(stdout, MAX_SVG_BYTES, "PlantUML SVG output"),
        read_stream_limited(stderr, MAX_DIAGNOSTIC_BYTES, "PlantUML diagnostics"),
        wait_for_status,
    )?;

    if !status.success() {
        let stderr =
            concise_stderr(&stderr).unwrap_or_else(|| format!("process exited with {status}"));
        anyhow::bail!("PlantUML: {stderr}");
    }
    anyhow::ensure!(!stdout.is_empty(), "PlantUML returned an empty SVG");
    Ok(stdout)
}

#[cfg(target_family = "wasm")]
async fn render_plantuml_local_svg(
    _contents: SharedString,
    _dark_mode: bool,
) -> anyhow::Result<Vec<u8>> {
    anyhow::bail!("PlantUML rendering is unavailable on this platform")
}

async fn render_plantuml_server_svg(
    contents: SharedString,
    dark_mode: bool,
    http_client: Arc<dyn HttpClient>,
) -> anyhow::Result<Vec<u8>> {
    let encoded = encode_plantuml_source(contents.as_bytes())?;
    let format = if dark_mode { "dsvg" } else { "svg" };
    let url = format!("{PUBLIC_SERVER_BASE_URL}/{format}/{encoded}");
    anyhow::ensure!(
        url.len() <= MAX_PUBLIC_SERVER_URL_BYTES,
        "PlantUML source is too large for the public server"
    );

    let request = Request::builder()
        .uri(url)
        .header("Accept", "image/svg+xml")
        .body(AsyncBody::default())?;
    let mut response = http_client
        .send(request)
        .await
        .context("failed to contact the public PlantUML server")?;
    anyhow::ensure!(
        response.status().is_success(),
        "public PlantUML server returned {}",
        response.status()
    );
    let content_type = response
        .headers()
        .get("content-type")
        .context("public PlantUML server response is missing Content-Type")?
        .to_str()
        .context("public PlantUML server returned an invalid Content-Type")?;
    anyhow::ensure!(
        content_type.starts_with("image/svg+xml"),
        "public PlantUML server returned {content_type} instead of SVG"
    );

    let svg = read_stream_limited(
        response.body_mut(),
        MAX_SVG_BYTES,
        "public PlantUML server response",
    )
    .await?;
    anyhow::ensure!(
        !svg.is_empty(),
        "public PlantUML server returned an empty SVG"
    );
    Ok(svg)
}

fn encode_plantuml_source(source: &[u8]) -> anyhow::Result<String> {
    let mut encoder = DeflateEncoder::new(Vec::new(), Compression::best());
    encoder
        .write_all(source)
        .context("failed to compress PlantUML source")?;
    let compressed = encoder
        .finish()
        .context("failed to finish compressing PlantUML source")?;

    let mut encoded = String::with_capacity(compressed.len().div_ceil(3) * 4);
    for chunk in compressed.chunks(3) {
        let first = chunk
            .first()
            .copied()
            .context("failed to encode compressed PlantUML source")?;
        let second = chunk.get(1).copied().unwrap_or_default();
        let third = chunk.get(2).copied().unwrap_or_default();
        encoded.push(encode_plantuml_six_bits(first >> 2));
        encoded.push(encode_plantuml_six_bits(
            ((first & 0x03) << 4) | (second >> 4),
        ));
        encoded.push(encode_plantuml_six_bits(
            ((second & 0x0f) << 2) | (third >> 6),
        ));
        encoded.push(encode_plantuml_six_bits(third & 0x3f));
    }
    Ok(encoded)
}

fn encode_plantuml_six_bits(value: u8) -> char {
    match value {
        0..=9 => char::from(b'0' + value),
        10..=35 => char::from(b'A' + value - 10),
        36..=61 => char::from(b'a' + value - 36),
        62 => '-',
        _ => '_',
    }
}

#[cfg(not(target_family = "wasm"))]
async fn ensure_supported_plantuml() -> anyhow::Result<()> {
    static PREFLIGHT: AsyncOnceCell<()> = AsyncOnceCell::new();
    PREFLIGHT
        .get_or_try_init(probe_plantuml_version)
        .await
        .map(|_| ())
}

#[cfg(not(target_family = "wasm"))]
async fn probe_plantuml_version() -> anyhow::Result<()> {
    let mut command = new_plantuml_command();
    command
        .arg("-version")
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    let mut child = command
        .spawn()
        .context("failed to start `plantuml`; install PlantUML and ensure it is on PATH")?;
    let stdout = child
        .stdout
        .take()
        .context("failed to open PlantUML version output")?;
    let stderr = child
        .stderr
        .take()
        .context("failed to open PlantUML version diagnostics")?;
    let wait_for_status = async {
        child
            .status()
            .await
            .context("failed while checking the PlantUML version")
    };
    let (stdout, stderr, status) = futures::try_join!(
        read_stream_limited(stdout, MAX_DIAGNOSTIC_BYTES, "PlantUML version output"),
        read_stream_limited(stderr, MAX_DIAGNOSTIC_BYTES, "PlantUML version diagnostics"),
        wait_for_status,
    )?;

    if !status.success() {
        let stderr =
            concise_stderr(&stderr).unwrap_or_else(|| format!("process exited with {status}"));
        anyhow::bail!("PlantUML version check failed: {stderr}");
    }

    let version = parse_plantuml_version(&stdout)
        .context("could not determine the installed PlantUML version")?;
    anyhow::ensure!(
        version >= MIN_SECURE_PLANTUML_VERSION,
        "PlantUML {}.{}.{} is too old; version 1.2020.11 or newer is required",
        version.0,
        version.1,
        version.2,
    );
    Ok(())
}

#[cfg(any(test, not(target_family = "wasm")))]
fn parse_plantuml_version(output: &[u8]) -> Option<(u32, u32, u32)> {
    let output = String::from_utf8_lossy(output);
    let version = output.lines().find_map(|line| {
        line.trim()
            .strip_prefix("PlantUML version ")
            .and_then(|suffix| suffix.split_whitespace().next())
    })?;
    let mut components = version.split('.');
    Some((
        parse_version_component(components.next()?)?,
        parse_version_component(components.next()?)?,
        parse_version_component(components.next()?)?,
    ))
}

#[cfg(any(test, not(target_family = "wasm")))]
fn parse_version_component(component: &str) -> Option<u32> {
    let digits = component
        .chars()
        .take_while(char::is_ascii_digit)
        .collect::<String>();
    digits.parse().ok()
}

fn bounded_raster_scale(svg: &[u8], requested_scale: f32) -> anyhow::Result<f32> {
    let (width, height) =
        parse_plantuml_svg_size(svg).context("PlantUML returned an SVG without a valid size")?;
    let requested_raster_scale = requested_scale * SMOOTH_SVG_SCALE_FACTOR;
    let area_scale = (MAX_RASTER_PIXELS / (width * height)).sqrt();
    let dimension_scale = (MAX_RASTER_DIMENSION / width).min(MAX_RASTER_DIMENSION / height);
    let raster_scale = requested_raster_scale.min(area_scale).min(dimension_scale);
    anyhow::ensure!(
        raster_scale.is_finite() && raster_scale > 0.0,
        "PlantUML returned an SVG with an invalid size"
    );
    Ok(raster_scale / SMOOTH_SVG_SCALE_FACTOR)
}

fn parse_plantuml_svg_size(svg: &[u8]) -> Option<(f32, f32)> {
    let prefix = &svg[..svg.len().min(4096)];
    let tag_start = prefix
        .windows(b"<svg".len())
        .position(|window| window == b"<svg")?;
    let tag_end = tag_start + prefix[tag_start..].iter().position(|byte| *byte == b'>')?;
    let tag = std::str::from_utf8(&prefix[tag_start..=tag_end]).ok()?;

    if let Some(view_box) = quoted_svg_attribute(tag, "viewBox") {
        let components = view_box
            .split(|character: char| character.is_ascii_whitespace() || character == ',')
            .filter(|component| !component.is_empty())
            .collect::<Vec<_>>();
        if let [_, _, width, height] = components.as_slice() {
            let width = width.parse::<f32>().ok()?;
            let height = height.parse::<f32>().ok()?;
            if valid_svg_dimension(width) && valid_svg_dimension(height) {
                return Some((width, height));
            }
        }
    }

    let width = parse_svg_length(quoted_svg_attribute(tag, "width")?)?;
    let height = parse_svg_length(quoted_svg_attribute(tag, "height")?)?;
    (valid_svg_dimension(width) && valid_svg_dimension(height)).then_some((width, height))
}

fn quoted_svg_attribute<'a>(tag: &'a str, name: &str) -> Option<&'a str> {
    let marker = format!("{name}=\"");
    tag.match_indices(&marker).find_map(|(attribute_start, _)| {
        let has_name_boundary = attribute_start == 0
            || tag
                .get(..attribute_start)?
                .chars()
                .next_back()
                .is_some_and(|character| character.is_ascii_whitespace() || character == '<');
        if !has_name_boundary {
            return None;
        }

        let value = tag.get(attribute_start + marker.len()..)?;
        value.get(..value.find('"')?)
    })
}

fn parse_svg_length(length: &str) -> Option<f32> {
    length
        .trim()
        .strip_suffix("px")
        .unwrap_or(length.trim())
        .parse()
        .ok()
}

fn valid_svg_dimension(dimension: f32) -> bool {
    dimension.is_finite() && dimension > 0.0
}

async fn read_stream_limited(
    mut stream: impl AsyncRead + Unpin,
    max_bytes: usize,
    description: &'static str,
) -> anyhow::Result<Vec<u8>> {
    let mut bytes = Vec::with_capacity(max_bytes.min(8192));
    let mut buffer = [0; 8192];
    let mut exceeded_limit = false;

    loop {
        let count = stream
            .read(&mut buffer)
            .await
            .with_context(|| format!("failed to read {description}"))?;
        if count == 0 {
            break;
        }

        let remaining = max_bytes.saturating_sub(bytes.len());
        let keep = remaining.min(count);
        bytes.extend_from_slice(&buffer[..keep]);
        exceeded_limit |= keep < count;
    }

    anyhow::ensure!(
        !exceeded_limit,
        "{description} exceeds the {max_bytes}-byte limit"
    );
    Ok(bytes)
}

#[cfg(any(test, not(target_family = "wasm")))]
fn concise_stderr(bytes: &[u8]) -> Option<String> {
    let message = String::from_utf8_lossy(bytes)
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(|line| {
            if line.chars().all(|character| character.is_ascii_digit()) {
                format!("line {line}")
            } else {
                line.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join(": ");
    (!message.is_empty()).then(|| truncate_message(&message, 200))
}

fn is_plantuml_alias(language: &str) -> bool {
    PLANTUML_ALIASES
        .iter()
        .any(|alias| language.eq_ignore_ascii_case(alias))
}

fn parse_plantuml_info(info: &str) -> Option<u32> {
    let mut parts = info.split_whitespace();
    if !is_plantuml_alias(parts.next()?) {
        return None;
    }

    Some(
        parts
            .next()
            .and_then(|scale| scale.parse().ok())
            .unwrap_or(DEFAULT_SCALE)
            .clamp(MIN_SCALE, MAX_SCALE),
    )
}

pub(crate) fn extract_plantuml_diagrams(
    source: &str,
    events: &[(Range<usize>, MarkdownEvent)],
) -> BTreeMap<usize, ParsedMarkdownPlantUmlDiagram> {
    let mut plantuml_diagrams = BTreeMap::default();

    for (source_range, event) in events {
        if plantuml_diagrams.len() >= MAX_DIAGRAMS_PER_DOCUMENT {
            break;
        }

        let MarkdownEvent::Start(MarkdownTag::CodeBlock { kind, metadata }) = event else {
            continue;
        };
        if !metadata.is_fenced_closed {
            continue;
        }

        let scale = match kind {
            CodeBlockKind::FencedLang(info) => match parse_plantuml_info(info.as_ref()) {
                Some(scale) => scale,
                None => continue,
            },
            CodeBlockKind::FencedSrc(path_range) => {
                let path = Path::new(path_range.path.as_ref());
                match path.extension().and_then(|extension| extension.to_str()) {
                    Some(extension) if is_plantuml_alias(extension) => DEFAULT_SCALE,
                    _ => continue,
                }
            }
            _ => continue,
        };

        let Some(contents) = fenced_code_block_contents(source, metadata.content_range.clone())
        else {
            continue;
        };
        plantuml_diagrams.insert(
            source_range.start,
            ParsedMarkdownPlantUmlDiagram {
                content_range: metadata.content_range.clone(),
                contents: ParsedMarkdownPlantUmlDiagramContents { contents, scale },
            },
        );
    }

    plantuml_diagrams
}

pub(crate) fn render_plantuml_diagram(
    parsed: &ParsedMarkdownPlantUmlDiagram,
    plantuml_state: &PlantUmlState,
    style: &MarkdownStyle,
    markdown: Entity<Markdown>,
    source_offset: usize,
    showing_code: bool,
    copy_button_visibility: CopyButtonVisibility,
) -> AnyElement {
    let cached = plantuml_state.cache.get(&parsed.contents);
    let render_state = DiagramRenderState::from_result(
        cached.and_then(|cached| cached.render_image.get()),
        || cached.and_then(|cached| cached.fallback_image()),
    );
    DiagramView {
        kind: DiagramKind::PlantUml,
        render_state,
        contents: &parsed.contents.contents,
        style,
        markdown,
        source_offset,
        showing_code,
        copy_button_visibility,
    }
    .render()
}

#[cfg(test)]
mod tests {
    use super::{
        CachedPlantUmlDiagram, MAX_DIAGRAMS_PER_DOCUMENT, MAX_RASTER_DIMENSION, MAX_RASTER_PIXELS,
        MAX_SOURCE_BYTES, PLANTUML_ALIASES, ParsedMarkdownPlantUmlDiagramContents,
        PlantUmlDiagramCache, bounded_raster_scale, encode_plantuml_six_bits,
        encode_plantuml_source, extract_plantuml_diagrams, parse_plantuml_info,
        parse_plantuml_svg_size, parse_plantuml_version, read_stream_limited, render_plantuml_svg,
    };
    use crate::{
        CodeBlockRenderer, CopyButtonVisibility, Markdown, MarkdownElement, MarkdownStyle,
        RenderedMarkdown, WrapButtonVisibility, diagram::fallback_image_for_edit,
    };
    use collections::HashMap;
    use gpui::{
        Context, Entity, IntoElement, Render, RenderImage, SMOOTH_SVG_SCALE_FACTOR, TestAppContext,
        Window, size,
    };
    use http_client::{AsyncBody, BlockedHttpClient, FakeHttpClient, Response};
    use settings::PlantUmlRenderMode;
    use std::sync::Arc;
    use ui::prelude::*;

    struct TestWindow;

    impl Render for TestWindow {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            div()
        }
    }

    fn ensure_theme_initialized(cx: &mut TestAppContext) {
        cx.update(|cx| {
            if !cx.has_global::<settings::SettingsStore>() {
                settings::init(cx);
            }
            if !cx.has_global::<theme::GlobalTheme>() {
                theme_settings::init(theme::LoadThemes::JustBase, cx);
            }
        });
    }

    fn mock_render_image(cx: &mut TestAppContext) -> Arc<RenderImage> {
        cx.update(|cx| {
            cx.svg_renderer()
                .render_single_frame(
                    br#"<svg xmlns="http://www.w3.org/2000/svg" width="1" height="1"></svg>"#,
                    1.0,
                )
                .expect("test SVG should render")
        })
    }

    fn draw_plantuml(
        source: &str,
        cached: CachedPlantUmlDiagram,
        cx: &mut TestAppContext,
    ) -> (RenderedMarkdown, Entity<Markdown>, bool, bool) {
        ensure_theme_initialized(cx);
        let (_, cx) = cx.add_window_view(|_, _| TestWindow);
        let markdown = cx.new(|cx| Markdown::new(source.into(), None, None, cx));
        cx.run_until_parked();

        markdown.update(cx, |markdown, _| {
            let diagrams = extract_plantuml_diagrams(
                markdown.parsed_markdown.source(),
                markdown.parsed_markdown.events(),
            );
            let contents = diagrams
                .values()
                .next()
                .expect("test Markdown should contain a PlantUML diagram")
                .contents
                .clone();
            markdown.parsed_markdown.plantuml_diagrams = diagrams;
            markdown.options.render_plantuml_diagrams = true;
            markdown
                .plantuml_state
                .cache
                .insert(contents.clone(), Arc::new(cached));
            markdown.plantuml_state.order = vec![contents];
        });

        let (rendered, _) = cx.draw(
            Default::default(),
            size(px(600.0), px(600.0)),
            |_window, _cx| {
                MarkdownElement::new(markdown.clone(), MarkdownStyle::default())
                    .code_block_renderer(CodeBlockRenderer::Default {
                        copy_button_visibility: CopyButtonVisibility::Hidden,
                        wrap_button_visibility: WrapButtonVisibility::Hidden,
                        border: false,
                    })
            },
        );

        let code_is_visible = cx.debug_bounds("plantuml-code").is_some();
        let error_is_visible = cx.debug_bounds("plantuml-error").is_some();
        (rendered, markdown, code_is_visible, error_is_visible)
    }

    fn rendered_text(rendered: &RenderedMarkdown) -> String {
        rendered
            .text
            .lines
            .iter()
            .map(|line| line.layout.wrapped_text())
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn plantuml_contents(contents: &str) -> ParsedMarkdownPlantUmlDiagramContents {
        ParsedMarkdownPlantUmlDiagramContents {
            contents: contents.to_string().into(),
            scale: 100,
        }
    }

    fn plantuml_sequence(diagrams: &[&str]) -> Vec<ParsedMarkdownPlantUmlDiagramContents> {
        diagrams
            .iter()
            .map(|diagram| plantuml_contents(diagram))
            .collect()
    }

    fn plantuml_fallback(
        new_diagram: &str,
        new_full_order: &[ParsedMarkdownPlantUmlDiagramContents],
        old_full_order: &[ParsedMarkdownPlantUmlDiagramContents],
        cache: &PlantUmlDiagramCache,
    ) -> Option<Arc<RenderImage>> {
        let new_content = plantuml_contents(new_diagram);
        let index = new_full_order
            .iter()
            .position(|diagram| diagram == &new_content)?;
        fallback_image_for_edit(
            index,
            old_full_order,
            new_full_order.len(),
            cache,
            |cached| {
                cached
                    .render_image
                    .get()
                    .and_then(|result| result.as_ref().ok().cloned())
                    .or_else(|| cached.fallback_image())
            },
        )
    }

    #[test]
    fn test_parse_plantuml_aliases_and_scale() {
        for alias in PLANTUML_ALIASES {
            assert_eq!(parse_plantuml_info(alias), Some(100));
            assert_eq!(parse_plantuml_info(&format!("{alias} 175")), Some(175));
        }
        assert_eq!(parse_plantuml_info("PLANTUML 150"), Some(150));
        assert_eq!(parse_plantuml_info("plantuml 5"), Some(10));
        assert_eq!(parse_plantuml_info("plantuml 999"), Some(500));
        assert_eq!(parse_plantuml_info("plantuml invalid"), Some(100));
        assert_eq!(parse_plantuml_info("mermaid"), None);
    }

    #[test]
    fn test_extract_plantuml_aliases_and_source_path() {
        let markdown = concat!(
            "```plantuml 150\n@startuml\nA -> B\n@enduml\n```\n\n",
            "```puml\n@startuml\nB -> C\n@enduml\n```\n\n",
            "```pu\n@startuml\nC -> D\n@enduml\n```\n\n",
            "```iuml\n@startuml\nD -> E\n@enduml\n```\n\n",
            "```wsd\n@startuml\nE -> F\n@enduml\n```\n\n",
            "```diagrams/example.puml\n@startuml\nF -> G\n@enduml\n```\n\n",
            "```rust\nfn main() {}\n```",
        );
        let events =
            crate::parser::parse_markdown_with_options(markdown, false, false, false).events;
        let diagrams = extract_plantuml_diagrams(markdown, &events);

        assert_eq!(diagrams.len(), 6);
        assert_eq!(
            diagrams
                .values()
                .next()
                .expect("at least one PlantUML diagram should be extracted")
                .contents
                .scale,
            150
        );
        assert!(
            diagrams
                .values()
                .any(|diagram| diagram.contents.contents.contains("F -> G"))
        );
    }

    #[test]
    fn test_extract_plantuml_requires_closed_fence() {
        let markdown = "```plantuml\n@startuml\nA -> B\n@enduml";
        let events =
            crate::parser::parse_markdown_with_options(markdown, false, false, false).events;

        assert!(extract_plantuml_diagrams(markdown, &events).is_empty());
    }

    #[test]
    fn test_extract_plantuml_skips_empty_fences() {
        let markdown = "```plantuml\n\n```\n\n```puml\n   \n```";
        let events =
            crate::parser::parse_markdown_with_options(markdown, false, false, false).events;

        assert!(extract_plantuml_diagrams(markdown, &events).is_empty());
    }

    #[test]
    fn test_extract_plantuml_limits_diagrams_per_document() {
        let markdown = (0..MAX_DIAGRAMS_PER_DOCUMENT + 1)
            .map(|index| format!("```plantuml\n@startuml\nA{index} -> B{index}\n@enduml\n```\n"))
            .collect::<String>();
        let events =
            crate::parser::parse_markdown_with_options(&markdown, false, false, false).events;

        assert_eq!(
            extract_plantuml_diagrams(&markdown, &events).len(),
            MAX_DIAGRAMS_PER_DOCUMENT
        );
    }

    #[test]
    fn test_parse_plantuml_version() {
        assert_eq!(
            parse_plantuml_version(b"PlantUML version 1.2026.4 / abc123\n"),
            Some((1, 2026, 4))
        );
        assert_eq!(
            parse_plantuml_version(b"PlantUML version 1.2020.11beta2\n"),
            Some((1, 2020, 11))
        );
        assert_eq!(parse_plantuml_version(b"unknown executable\n"), None);
    }

    #[test]
    fn test_bounded_raster_scale_caps_area_and_dimensions() {
        let large_svg = br#"<svg viewBox="0 0 4096 4096" width="4096px" height="4096px"></svg>"#;
        assert_eq!(parse_plantuml_svg_size(large_svg), Some((4096.0, 4096.0)));

        let scale = bounded_raster_scale(large_svg, 5.0)
            .expect("a sized PlantUML SVG should produce a raster scale");
        let raster_scale = scale * SMOOTH_SVG_SCALE_FACTOR;
        let raster_width = 4096.0 * raster_scale;
        let raster_height = 4096.0 * raster_scale;
        assert!(raster_width * raster_height <= MAX_RASTER_PIXELS);
        assert!(raster_width <= MAX_RASTER_DIMENSION);
        assert!(raster_height <= MAX_RASTER_DIMENSION);
        assert!(scale < 5.0);

        let small_svg = br#"<svg width="100px" height="50px"></svg>"#;
        assert_eq!(parse_plantuml_svg_size(small_svg), Some((100.0, 50.0)));
        assert_eq!(
            bounded_raster_scale(small_svg, 1.0)
                .expect("a small PlantUML SVG should preserve its requested scale"),
            1.0
        );
    }

    #[test]
    fn test_parse_svg_size_ignores_partial_utf8_after_root_tag() {
        let mut svg = br#"<svg viewBox="0 0 100 50">"#.to_vec();
        svg.resize(4095, b'a');
        svg.extend_from_slice("😀".as_bytes());

        assert_eq!(parse_plantuml_svg_size(&svg), Some((100.0, 50.0)));
    }

    #[test]
    fn test_parse_svg_size_matches_complete_attribute_names() {
        let svg = br#"<svg data-viewBox="0 0 900 800" stroke-width="700" width="100px" height="50px"></svg>"#;
        assert_eq!(parse_plantuml_svg_size(svg), Some((100.0, 50.0)));
    }

    #[test]
    fn test_plantuml_source_limit() {
        let oversized_source = "x".repeat(MAX_SOURCE_BYTES + 1);
        let error = futures::executor::block_on(render_plantuml_svg(
            oversized_source.into(),
            false,
            PlantUmlRenderMode::Local,
            Arc::new(BlockedHttpClient::new()),
        ))
        .expect_err("oversized PlantUML source should be rejected before rendering");
        assert!(error.to_string().contains("1 MiB limit"));
    }

    #[test]
    fn test_plantuml_server_encoding_uses_url_safe_alphabet() {
        let encoded = encode_plantuml_source(b"@startuml\nBob -> Alice : hello\n@enduml")
            .expect("PlantUML source should encode");
        assert!(!encoded.is_empty());
        assert!(
            encoded
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_'))
        );
        assert_eq!(
            (0..64).map(encode_plantuml_six_bits).collect::<String>(),
            "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz-_"
        );
    }

    #[test]
    fn test_public_server_render_uses_dark_svg_endpoint() {
        let http_client = FakeHttpClient::create(|request| async move {
            assert_eq!(request.uri().host(), Some("www.plantuml.com"));
            assert!(request.uri().path().starts_with("/plantuml/dsvg/"));
            Ok(Response::builder()
                .status(200)
                .header("Content-Type", "image/svg+xml")
                .body(AsyncBody::from(
                    br#"<svg viewBox="0 0 100 50"></svg>"#.as_slice(),
                ))
                .expect("test response should be valid"))
        });

        let svg = futures::executor::block_on(render_plantuml_svg(
            "@startuml\nAlice -> Bob\n@enduml".into(),
            true,
            PlantUmlRenderMode::PublicServer,
            http_client,
        ))
        .expect("public server response should render");
        assert_eq!(svg, br#"<svg viewBox="0 0 100 50"></svg>"#);
    }

    #[test]
    fn test_public_server_render_rejects_non_svg_response() {
        let http_client = FakeHttpClient::create(|_| async move {
            Ok(Response::builder()
                .status(200)
                .header("Content-Type", "text/html")
                .body(AsyncBody::from("server error"))
                .expect("test response should be valid"))
        });

        let error = futures::executor::block_on(render_plantuml_svg(
            "@startuml\nAlice -> Bob\n@enduml".into(),
            false,
            PlantUmlRenderMode::PublicServer,
            http_client,
        ))
        .expect_err("non-SVG public server response should be rejected");
        assert!(error.to_string().contains("instead of SVG"));
    }

    #[test]
    fn test_plantuml_output_limit() {
        let exact = futures::executor::block_on(read_stream_limited(
            futures::io::Cursor::new(b"1234"),
            4,
            "test output",
        ))
        .expect("output at the byte limit should be accepted");
        assert_eq!(exact, b"1234");

        let error = futures::executor::block_on(read_stream_limited(
            futures::io::Cursor::new(b"12345"),
            4,
            "test output",
        ))
        .expect_err("output above the byte limit should be rejected");
        assert!(error.to_string().contains("4-byte limit"));
    }

    #[test]
    fn test_concise_stderr_preserves_line_numbers() {
        assert_eq!(
            super::concise_stderr(b"ERROR\n2\nSyntax Error?\n"),
            Some("ERROR: line 2: Syntax Error?".to_string())
        );
        assert_eq!(
            super::concise_stderr(b"\n42\n"),
            Some("line 42".to_string())
        );
    }

    #[gpui::test]
    fn test_plantuml_fallback_on_edit(cx: &mut TestAppContext) {
        let old_full_order = plantuml_sequence(&["diagram A", "diagram B", "diagram C"]);
        let new_full_order = plantuml_sequence(&["diagram A", "diagram B modified", "diagram C"]);
        let diagram_b = mock_render_image(cx);

        let mut cache: PlantUmlDiagramCache = HashMap::default();
        cache.insert(
            plantuml_contents("diagram A"),
            Arc::new(CachedPlantUmlDiagram::new_for_test(
                Some(mock_render_image(cx)),
                None,
            )),
        );
        cache.insert(
            plantuml_contents("diagram B"),
            Arc::new(CachedPlantUmlDiagram::new_for_test(
                Some(diagram_b.clone()),
                None,
            )),
        );
        cache.insert(
            plantuml_contents("diagram C"),
            Arc::new(CachedPlantUmlDiagram::new_for_test(
                Some(mock_render_image(cx)),
                None,
            )),
        );

        let fallback = plantuml_fallback(
            "diagram B modified",
            &new_full_order,
            &old_full_order,
            &cache,
        );
        assert_eq!(fallback.as_ref().map(|image| image.id), Some(diagram_b.id));
    }

    #[gpui::test]
    fn test_plantuml_mock_render_replaces_code_and_maps_source(cx: &mut TestAppContext) {
        let source = "```plantuml\n@startuml\nAlice -> Bob\n@enduml\n```";
        let render_image = mock_render_image(cx);
        let (rendered, markdown, _, _) = draw_plantuml(
            source,
            CachedPlantUmlDiagram::new_for_test(Some(render_image), None),
            cx,
        );

        assert!(!rendered_text(&rendered).contains("Alice -> Bob"));

        let diagram = markdown.update(cx, |markdown, _| {
            markdown
                .parsed_markdown
                .plantuml_diagrams
                .values()
                .next()
                .expect("rendered Markdown should retain its PlantUML diagram")
                .clone()
        });
        assert!(
            rendered
                .text
                .position_for_source_index(diagram.content_range.start)
                .is_some()
        );
        assert!(
            rendered
                .text
                .position_for_source_index(diagram.content_range.end.saturating_sub(1))
                .is_some()
        );
    }

    #[gpui::test]
    fn test_plantuml_error_renders_source_and_message(cx: &mut TestAppContext) {
        let source = concat!(
            "```plantuml\n",
            "@startuml\nPLANTUML_SOURCE_SENTINEL\n@enduml\n",
            "```",
        );
        let (_, _, code_is_visible, error_is_visible) = draw_plantuml(
            source,
            CachedPlantUmlDiagram::new_error_for_test(anyhow::anyhow!("PlantUML: syntax error")),
            cx,
        );
        assert!(code_is_visible);
        assert!(error_is_visible);
    }
}
