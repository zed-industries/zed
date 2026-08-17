mod builtin;
mod context;
mod preset;

pub(crate) use builtin::GitGraphBranchLabelBaselinePostprocessor;
pub use builtin::{
    CssOverridePolicy, CssOverridePostprocessor, DropNativeDuplicateFallbacksPostprocessor,
    ForeignObjectFallbackPostprocessor, RootBackgroundPostprocessor, SanitizeCssPostprocessor,
    SanitizeSvgAttributesPostprocessor, ScopedCssPostprocessor, StripForeignObjectPostprocessor,
};
pub use context::{SvgPostprocessContext, SvgPostprocessMetadata};
pub use preset::{SvgPipelinePreset, resvg_safe_svg};

use crate::{Error, Result};
use std::borrow::Cow;
use std::fmt;
use std::sync::Arc;

pub trait SvgPostprocessor: Send + Sync {
    fn name(&self) -> &'static str;

    fn process<'a>(
        &self,
        svg: Cow<'a, str>,
        ctx: &SvgPostprocessContext<'_>,
    ) -> Result<Cow<'a, str>>;
}

#[derive(Clone)]
pub struct SvgPipeline {
    preset: SvgPipelinePreset,
    postprocessors: Vec<Arc<dyn SvgPostprocessor>>,
}

impl fmt::Debug for SvgPipeline {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let names = self
            .postprocessors
            .iter()
            .map(|pass| pass.name())
            .collect::<Vec<_>>();

        f.debug_struct("SvgPipeline")
            .field("preset", &self.preset)
            .field("postprocessors", &names)
            .finish()
    }
}

impl Default for SvgPipeline {
    fn default() -> Self {
        Self::parity()
    }
}

impl SvgPipeline {
    pub fn parity() -> Self {
        Self::from_preset(SvgPipelinePreset::Parity)
    }

    pub fn readable() -> Self {
        Self::from_preset(SvgPipelinePreset::Readable)
    }

    pub fn resvg_safe() -> Self {
        Self::from_preset(SvgPipelinePreset::ResvgSafe)
    }

    pub fn from_preset(preset: SvgPipelinePreset) -> Self {
        Self {
            preset,
            postprocessors: Vec::new(),
        }
    }

    pub fn preset(&self) -> SvgPipelinePreset {
        self.preset
    }

    pub fn with_postprocessor<P>(mut self, postprocessor: P) -> Self
    where
        P: SvgPostprocessor + 'static,
    {
        self.postprocessors.push(Arc::new(postprocessor));
        self
    }

    pub fn with_shared_postprocessor(mut self, postprocessor: Arc<dyn SvgPostprocessor>) -> Self {
        self.postprocessors.push(postprocessor);
        self
    }

    pub fn push_postprocessor<P>(&mut self, postprocessor: P)
    where
        P: SvgPostprocessor + 'static,
    {
        self.postprocessors.push(Arc::new(postprocessor));
    }

    pub fn process<'a>(&self, svg: &'a str) -> Result<Cow<'a, str>> {
        let metadata = SvgPostprocessMetadata::from_svg(svg);
        self.process_with_metadata(svg, &metadata)
    }

    pub fn process_with_metadata<'a>(
        &self,
        svg: &'a str,
        metadata: &SvgPostprocessMetadata,
    ) -> Result<Cow<'a, str>> {
        let mut current = preset::apply_preset(self.preset, svg);

        for (index, postprocessor) in self.postprocessors.iter().enumerate() {
            let ctx =
                SvgPostprocessContext::new(self.preset, index, postprocessor.name(), metadata);
            current = postprocessor
                .process(current, &ctx)
                .map_err(|err| Error::svg_postprocess(postprocessor.name(), err.to_string()))?;
        }

        Ok(current)
    }

    pub fn process_to_string(&self, svg: &str) -> Result<String> {
        Ok(self.process(svg)?.into_owned())
    }

    pub fn process_to_string_with_metadata(
        &self,
        svg: &str,
        metadata: &SvgPostprocessMetadata,
    ) -> Result<String> {
        Ok(self.process_with_metadata(svg, metadata)?.into_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parity_pipeline_preserves_svg_exactly() {
        let svg = r#"<svg><style>@keyframes a{to{opacity:1}}</style><rect width="10"/></svg>"#;
        let out = SvgPipeline::parity().process(svg).unwrap();
        assert!(matches!(out, Cow::Borrowed(_)));
        assert_eq!(out, svg);
    }

    #[test]
    fn readable_pipeline_matches_foreign_object_fallback() {
        let svg = r#"<svg xmlns="http://www.w3.org/2000/svg"><g transform="translate(10,20)"><foreignObject width="80" height="48"><div xmlns="http://www.w3.org/1999/xhtml"><p>Layer 7\nHTTP</p></div></foreignObject></g></svg>"#;

        let expected = super::builtin::foreign_object::foreign_object_fallback_svg(svg);
        let out = SvgPipeline::readable().process_to_string(svg).unwrap();

        assert_eq!(out, expected);
        assert!(out.contains(">Layer 7</text>"));
        assert!(out.contains(">HTTP</text>"));
    }

    #[test]
    fn resvg_safe_pipeline_strips_generic_raster_hazards() {
        let svg = r#"<svg id="test" xmlns="http://www.w3.org/2000/svg"><style type="text/css">@keyframes bounce { 0% { transform: scale(1); } 100% { transform: scale(1.1); } } #test :root { --bg: white; } .node rect { animation: dash 1s linear; transform: rotate(45deg); fill: red; }</style><g transform="translate(undefined,NaN)"><foreignObject width="10" height="10"><div xmlns="http://www.w3.org/1999/xhtml"><p>Hello</p></div></foreignObject><rect width="10px" height="12px" stroke="" style="fill: ; stroke: #333; transform: rotate(45deg); animation: dash 1s;"/><rect width="10px" height="" fill="hsl(240, 100%, NaN%)"/></g></svg>"#;

        let out = SvgPipeline::resvg_safe().process_to_string(svg).unwrap();

        assert!(!out.contains("<foreignObject"));
        assert!(!out.contains("@keyframes"));
        assert!(!out.contains(":root"));
        assert!(!out.contains("animation"));
        assert!(!out.contains("deg"));
        assert!(!out.contains("NaN"));
        assert!(!out.contains("undefined"));
        assert!(!out.contains(r#"height="""#));
        assert!(!out.contains(r#"fill="hsl"#));
        assert!(!out.contains(r#"stroke="""#));
        assert!(out.contains(r#"width="10""#));
        assert!(out.contains(r#"height="12""#));
        assert!(out.contains("stroke:#333"));
        assert!(out.contains(">Hello</text>"));
    }

    struct AppendPass(&'static str);

    impl SvgPostprocessor for AppendPass {
        fn name(&self) -> &'static str {
            self.0
        }

        fn process<'a>(
            &self,
            svg: Cow<'a, str>,
            ctx: &SvgPostprocessContext<'_>,
        ) -> Result<Cow<'a, str>> {
            Ok(Cow::Owned(format!(
                "{}<!--{}:{}:{:?}:{}:{}:{}-->",
                svg,
                ctx.pass_index(),
                ctx.pass_name(),
                ctx.preset(),
                ctx.diagram_type().unwrap_or("none"),
                ctx.diagram_title().unwrap_or("none"),
                ctx.svg_id().unwrap_or("none")
            )))
        }
    }

    #[test]
    fn custom_postprocessors_run_after_builtin_preset_in_order() {
        let svg = r#"<svg><foreignObject width="10" height="10"><div><p>Hello</p></div></foreignObject></svg>"#;
        let pipeline = SvgPipeline::readable()
            .with_postprocessor(AppendPass("first"))
            .with_postprocessor(AppendPass("second"));

        let out = pipeline.process_to_string(svg).unwrap();

        let fallback = out.find("data-merman-foreignobject").unwrap();
        let first = out.find("<!--0:first:Readable").unwrap();
        let second = out.find("<!--1:second:Readable").unwrap();
        assert!(fallback < first);
        assert!(first < second);
    }

    #[test]
    fn custom_postprocessor_context_exposes_metadata() {
        let svg = r#"<svg id="host-diagram"><rect width="10"/></svg>"#;
        let metadata = SvgPostprocessMetadata::from_svg(svg)
            .with_diagram_type("flowchart-v2")
            .with_diagram_title("Host Diagram");
        let pipeline = SvgPipeline::parity().with_postprocessor(AppendPass("meta"));

        let out = pipeline
            .process_to_string_with_metadata(svg, &metadata)
            .unwrap();

        assert!(out.contains("<!--0:meta:Parity:flowchart-v2:Host Diagram:host-diagram-->"));
    }

    struct ErrorPass;

    impl SvgPostprocessor for ErrorPass {
        fn name(&self) -> &'static str {
            "error-pass"
        }

        fn process<'a>(
            &self,
            _svg: Cow<'a, str>,
            _ctx: &SvgPostprocessContext<'_>,
        ) -> Result<Cow<'a, str>> {
            Err(Error::InvalidModel {
                message: "boom".to_string(),
            })
        }
    }

    #[test]
    fn custom_postprocessor_errors_surface_with_pass_name() {
        let err = SvgPipeline::parity()
            .with_postprocessor(ErrorPass)
            .process_to_string("<svg/>")
            .unwrap_err();

        let message = err.to_string();
        assert!(message.contains("error-pass"));
        assert!(message.contains("boom"));
    }
}
