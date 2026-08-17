use crate::pipeline;

/// A blending mode.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub enum BlendMode {
    /// Replaces destination with zero: fully transparent.
    Clear,
    /// Replaces destination.
    Source,
    /// Preserves destination.
    Destination,
    /// Source over destination.
    SourceOver,
    /// Destination over source.
    DestinationOver,
    /// Source trimmed inside destination.
    SourceIn,
    /// Destination trimmed by source.
    DestinationIn,
    /// Source trimmed outside destination.
    SourceOut,
    /// Destination trimmed outside source.
    DestinationOut,
    /// Source inside destination blended with destination.
    SourceAtop,
    /// Destination inside source blended with source.
    DestinationAtop,
    /// Each of source and destination trimmed outside the other.
    Xor,
    /// Sum of colors.
    Plus,
    /// Product of premultiplied colors; darkens destination.
    Modulate,
    /// Multiply inverse of pixels, inverting result; brightens destination.
    Screen,
    /// Multiply or screen, depending on destination.
    Overlay,
    /// Darker of source and destination.
    Darken,
    /// Lighter of source and destination.
    Lighten,
    /// Brighten destination to reflect source.
    ColorDodge,
    /// Darken destination to reflect source.
    ColorBurn,
    /// Multiply or screen, depending on source.
    HardLight,
    /// Lighten or darken, depending on source.
    SoftLight,
    /// Subtract darker from lighter with higher contrast.
    Difference,
    /// Subtract darker from lighter with lower contrast.
    Exclusion,
    /// Multiply source with destination, darkening image.
    Multiply,
    /// Hue of source with saturation and luminosity of destination.
    Hue,
    /// Saturation of source with hue and luminosity of destination.
    Saturation,
    /// Hue and saturation of source with luminosity of destination.
    Color,
    /// Luminosity of source with hue and saturation of destination.
    Luminosity,
}

impl Default for BlendMode {
    fn default() -> Self {
        BlendMode::SourceOver
    }
}

impl BlendMode {
    pub(crate) fn should_pre_scale_coverage(self) -> bool {
        // The most important things we do here are:
        //   1) never pre-scale with rgb coverage if the blend mode involves a source-alpha term;
        //   2) always pre-scale Plus.
        //
        // When we pre-scale with rgb coverage, we scale each of source r,g,b, with a distinct value,
        // and source alpha with one of those three values. This process destructively updates the
        // source-alpha term, so we can't evaluate blend modes that need its original value.
        //
        // Plus always requires pre-scaling as a specific quirk of its implementation in
        // RasterPipeline. This lets us put the clamp inside the blend mode itself rather
        // than as a separate stage that'd come after the lerp.
        //
        // This function is a finer-grained breakdown of SkBlendMode_SupportsCoverageAsAlpha().
        matches!(
            self,
            BlendMode::Destination |        // d              --> no sa term, ok!
            BlendMode::DestinationOver |    // d + s*inv(da)  --> no sa term, ok!
            BlendMode::Plus |               // clamp(s+d)     --> no sa term, ok!
            BlendMode::DestinationOut |     // d * inv(sa)
            BlendMode::SourceAtop |         // s*da + d*inv(sa)
            BlendMode::SourceOver |         // s + d*inv(sa)
            BlendMode::Xor // s*inv(da) + d*inv(sa)
        )
    }

    pub(crate) fn to_stage(self) -> Option<pipeline::Stage> {
        match self {
            BlendMode::Clear => Some(pipeline::Stage::Clear),
            BlendMode::Source => None, // This stage is a no-op.
            BlendMode::Destination => Some(pipeline::Stage::MoveDestinationToSource),
            BlendMode::SourceOver => Some(pipeline::Stage::SourceOver),
            BlendMode::DestinationOver => Some(pipeline::Stage::DestinationOver),
            BlendMode::SourceIn => Some(pipeline::Stage::SourceIn),
            BlendMode::DestinationIn => Some(pipeline::Stage::DestinationIn),
            BlendMode::SourceOut => Some(pipeline::Stage::SourceOut),
            BlendMode::DestinationOut => Some(pipeline::Stage::DestinationOut),
            BlendMode::SourceAtop => Some(pipeline::Stage::SourceAtop),
            BlendMode::DestinationAtop => Some(pipeline::Stage::DestinationAtop),
            BlendMode::Xor => Some(pipeline::Stage::Xor),
            BlendMode::Plus => Some(pipeline::Stage::Plus),
            BlendMode::Modulate => Some(pipeline::Stage::Modulate),
            BlendMode::Screen => Some(pipeline::Stage::Screen),
            BlendMode::Overlay => Some(pipeline::Stage::Overlay),
            BlendMode::Darken => Some(pipeline::Stage::Darken),
            BlendMode::Lighten => Some(pipeline::Stage::Lighten),
            BlendMode::ColorDodge => Some(pipeline::Stage::ColorDodge),
            BlendMode::ColorBurn => Some(pipeline::Stage::ColorBurn),
            BlendMode::HardLight => Some(pipeline::Stage::HardLight),
            BlendMode::SoftLight => Some(pipeline::Stage::SoftLight),
            BlendMode::Difference => Some(pipeline::Stage::Difference),
            BlendMode::Exclusion => Some(pipeline::Stage::Exclusion),
            BlendMode::Multiply => Some(pipeline::Stage::Multiply),
            BlendMode::Hue => Some(pipeline::Stage::Hue),
            BlendMode::Saturation => Some(pipeline::Stage::Saturation),
            BlendMode::Color => Some(pipeline::Stage::Color),
            BlendMode::Luminosity => Some(pipeline::Stage::Luminosity),
        }
    }
}
