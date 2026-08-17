// Copyright 2018 the Resvg Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! SVG filter types.

use strict_num::PositiveF32;

use crate::{BlendMode, Color, Group, NonEmptyString, NonZeroF32, NonZeroRect, Opacity};

/// A filter element.
///
/// `filter` element in the SVG.
#[derive(Debug)]
pub struct Filter {
    pub(crate) id: NonEmptyString,
    pub(crate) rect: NonZeroRect,
    pub(crate) primitives: Vec<Primitive>,
}

impl Filter {
    /// Element's ID.
    ///
    /// Taken from the SVG itself.
    /// Used only during SVG writing. `resvg` doesn't rely on this property.
    pub fn id(&self) -> &str {
        self.id.get()
    }

    /// Filter region.
    ///
    /// `x`, `y`, `width` and `height` in the SVG.
    pub fn rect(&self) -> NonZeroRect {
        self.rect
    }

    /// A list of filter primitives.
    pub fn primitives(&self) -> &[Primitive] {
        &self.primitives
    }
}

/// A filter primitive element.
#[derive(Clone, Debug)]
pub struct Primitive {
    pub(crate) rect: NonZeroRect,
    pub(crate) color_interpolation: ColorInterpolation,
    pub(crate) result: String,
    pub(crate) kind: Kind,
}

impl Primitive {
    /// Filter subregion.
    ///
    /// `x`, `y`, `width` and `height` in the SVG.
    pub fn rect(&self) -> NonZeroRect {
        self.rect
    }

    /// Color interpolation mode.
    ///
    /// `color-interpolation-filters` in the SVG.
    pub fn color_interpolation(&self) -> ColorInterpolation {
        self.color_interpolation
    }

    /// Assigned name for this filter primitive.
    ///
    /// `result` in the SVG.
    pub fn result(&self) -> &str {
        &self.result
    }

    /// Filter primitive kind.
    pub fn kind(&self) -> &Kind {
        &self.kind
    }
}

/// A filter kind.
#[allow(missing_docs)]
#[derive(Clone, Debug)]
pub enum Kind {
    Blend(Blend),
    ColorMatrix(ColorMatrix),
    ComponentTransfer(ComponentTransfer),
    Composite(Composite),
    ConvolveMatrix(ConvolveMatrix),
    DiffuseLighting(DiffuseLighting),
    DisplacementMap(DisplacementMap),
    DropShadow(DropShadow),
    Flood(Flood),
    GaussianBlur(GaussianBlur),
    Image(Image),
    Merge(Merge),
    Morphology(Morphology),
    Offset(Offset),
    SpecularLighting(SpecularLighting),
    Tile(Tile),
    Turbulence(Turbulence),
}

impl Kind {
    /// Checks that `FilterKind` has a specific input.
    pub fn has_input(&self, input: &Input) -> bool {
        match self {
            Kind::Blend(fe) => fe.input1 == *input || fe.input2 == *input,
            Kind::ColorMatrix(fe) => fe.input == *input,
            Kind::ComponentTransfer(fe) => fe.input == *input,
            Kind::Composite(fe) => fe.input1 == *input || fe.input2 == *input,
            Kind::ConvolveMatrix(fe) => fe.input == *input,
            Kind::DiffuseLighting(fe) => fe.input == *input,
            Kind::DisplacementMap(fe) => fe.input1 == *input || fe.input2 == *input,
            Kind::DropShadow(fe) => fe.input == *input,
            Kind::Flood(_) => false,
            Kind::GaussianBlur(fe) => fe.input == *input,
            Kind::Image(_) => false,
            Kind::Merge(fe) => fe.inputs.iter().any(|i| i == input),
            Kind::Morphology(fe) => fe.input == *input,
            Kind::Offset(fe) => fe.input == *input,
            Kind::SpecularLighting(fe) => fe.input == *input,
            Kind::Tile(fe) => fe.input == *input,
            Kind::Turbulence(_) => false,
        }
    }
}

/// Identifies input for a filter primitive.
#[allow(missing_docs)]
#[derive(Clone, PartialEq, Debug)]
pub enum Input {
    SourceGraphic,
    SourceAlpha,
    Reference(String),
}

/// A color interpolation mode.
///
/// The default is `ColorInterpolation::LinearRGB`.
#[allow(missing_docs)]
#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub enum ColorInterpolation {
    SRGB,
    #[default]
    LinearRGB,
}

/// A blend filter primitive.
///
/// `feBlend` element in the SVG.
#[derive(Clone, Debug)]
pub struct Blend {
    pub(crate) input1: Input,
    pub(crate) input2: Input,
    pub(crate) mode: BlendMode,
}

impl Blend {
    /// Identifies input for the given filter primitive.
    ///
    /// `in` in the SVG.
    pub fn input1(&self) -> &Input {
        &self.input1
    }

    /// Identifies input for the given filter primitive.
    ///
    /// `in2` in the SVG.
    pub fn input2(&self) -> &Input {
        &self.input2
    }

    /// A blending mode.
    ///
    /// `mode` in the SVG.
    pub fn mode(&self) -> BlendMode {
        self.mode
    }
}

/// A color matrix filter primitive.
///
/// `feColorMatrix` element in the SVG.
#[derive(Clone, Debug)]
pub struct ColorMatrix {
    pub(crate) input: Input,
    pub(crate) kind: ColorMatrixKind,
}

impl ColorMatrix {
    /// Identifies input for the given filter primitive.
    ///
    /// `in` in the SVG.
    pub fn input(&self) -> &Input {
        &self.input
    }

    /// A matrix kind.
    ///
    /// `type` in the SVG.
    pub fn kind(&self) -> &ColorMatrixKind {
        &self.kind
    }
}

/// A color matrix filter primitive kind.
#[derive(Clone, Debug)]
#[allow(missing_docs)]
pub enum ColorMatrixKind {
    Matrix(Vec<f32>), // Guarantee to have 20 numbers.
    Saturate(PositiveF32),
    HueRotate(f32),
    LuminanceToAlpha,
}

impl Default for ColorMatrixKind {
    fn default() -> Self {
        ColorMatrixKind::Matrix(vec![
            1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0,
        ])
    }
}

/// A component-wise remapping filter primitive.
///
/// `feComponentTransfer` element in the SVG.
#[derive(Clone, Debug)]
pub struct ComponentTransfer {
    pub(crate) input: Input,
    pub(crate) func_r: TransferFunction,
    pub(crate) func_g: TransferFunction,
    pub(crate) func_b: TransferFunction,
    pub(crate) func_a: TransferFunction,
}

impl ComponentTransfer {
    /// Identifies input for the given filter primitive.
    ///
    /// `in` in the SVG.
    pub fn input(&self) -> &Input {
        &self.input
    }

    /// `feFuncR` in the SVG.
    pub fn func_r(&self) -> &TransferFunction {
        &self.func_r
    }

    /// `feFuncG` in the SVG.
    pub fn func_g(&self) -> &TransferFunction {
        &self.func_g
    }

    /// `feFuncB` in the SVG.
    pub fn func_b(&self) -> &TransferFunction {
        &self.func_b
    }

    /// `feFuncA` in the SVG.
    pub fn func_a(&self) -> &TransferFunction {
        &self.func_a
    }
}

/// A transfer function used by `FeComponentTransfer`.
///
/// <https://www.w3.org/TR/SVG11/filters.html#transferFuncElements>
#[derive(Clone, Debug)]
pub enum TransferFunction {
    /// Keeps a component as is.
    Identity,

    /// Applies a linear interpolation to a component.
    ///
    /// The number list can be empty.
    Table(Vec<f32>),

    /// Applies a step function to a component.
    ///
    /// The number list can be empty.
    Discrete(Vec<f32>),

    /// Applies a linear shift to a component.
    #[allow(missing_docs)]
    Linear { slope: f32, intercept: f32 },

    /// Applies an exponential shift to a component.
    #[allow(missing_docs)]
    Gamma {
        amplitude: f32,
        exponent: f32,
        offset: f32,
    },
}

/// A composite filter primitive.
///
/// `feComposite` element in the SVG.
#[derive(Clone, Debug)]
pub struct Composite {
    pub(crate) input1: Input,
    pub(crate) input2: Input,
    pub(crate) operator: CompositeOperator,
}

impl Composite {
    /// Identifies input for the given filter primitive.
    ///
    /// `in` in the SVG.
    pub fn input1(&self) -> &Input {
        &self.input1
    }

    /// Identifies input for the given filter primitive.
    ///
    /// `in2` in the SVG.
    pub fn input2(&self) -> &Input {
        &self.input2
    }

    /// A compositing operation.
    ///
    /// `operator` in the SVG.
    pub fn operator(&self) -> CompositeOperator {
        self.operator
    }
}

/// An images compositing operation.
#[allow(missing_docs)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum CompositeOperator {
    Over,
    In,
    Out,
    Atop,
    Xor,
    Arithmetic { k1: f32, k2: f32, k3: f32, k4: f32 },
}

/// A matrix convolution filter primitive.
///
/// `feConvolveMatrix` element in the SVG.
#[derive(Clone, Debug)]
pub struct ConvolveMatrix {
    pub(crate) input: Input,
    pub(crate) matrix: ConvolveMatrixData,
    pub(crate) divisor: NonZeroF32,
    pub(crate) bias: f32,
    pub(crate) edge_mode: EdgeMode,
    pub(crate) preserve_alpha: bool,
}

impl ConvolveMatrix {
    /// Identifies input for the given filter primitive.
    ///
    /// `in` in the SVG.
    pub fn input(&self) -> &Input {
        &self.input
    }

    /// A convolve matrix.
    pub fn matrix(&self) -> &ConvolveMatrixData {
        &self.matrix
    }

    /// A matrix divisor.
    ///
    /// `divisor` in the SVG.
    pub fn divisor(&self) -> NonZeroF32 {
        self.divisor
    }

    /// A kernel matrix bias.
    ///
    /// `bias` in the SVG.
    pub fn bias(&self) -> f32 {
        self.bias
    }

    /// An edges processing mode.
    ///
    /// `edgeMode` in the SVG.
    pub fn edge_mode(&self) -> EdgeMode {
        self.edge_mode
    }

    /// An alpha preserving flag.
    ///
    /// `preserveAlpha` in the SVG.
    pub fn preserve_alpha(&self) -> bool {
        self.preserve_alpha
    }
}

/// A convolve matrix representation.
///
/// Used primarily by [`ConvolveMatrix`].
#[derive(Clone, Debug)]
pub struct ConvolveMatrixData {
    pub(crate) target_x: u32,
    pub(crate) target_y: u32,
    pub(crate) columns: u32,
    pub(crate) rows: u32,
    pub(crate) data: Vec<f32>,
}

impl ConvolveMatrixData {
    /// Returns a matrix's X target.
    ///
    /// `targetX` in the SVG.
    pub fn target_x(&self) -> u32 {
        self.target_x
    }

    /// Returns a matrix's Y target.
    ///
    /// `targetY` in the SVG.
    pub fn target_y(&self) -> u32 {
        self.target_y
    }

    /// Returns a number of columns in the matrix.
    ///
    /// Part of the `order` attribute in the SVG.
    pub fn columns(&self) -> u32 {
        self.columns
    }

    /// Returns a number of rows in the matrix.
    ///
    /// Part of the `order` attribute in the SVG.
    pub fn rows(&self) -> u32 {
        self.rows
    }

    /// The actual matrix.
    pub fn data(&self) -> &[f32] {
        &self.data
    }
}

impl ConvolveMatrixData {
    /// Creates a new `ConvolveMatrixData`.
    ///
    /// Returns `None` when:
    ///
    /// - `columns` * `rows` != `data.len()`
    /// - `target_x` >= `columns`
    /// - `target_y` >= `rows`
    pub(crate) fn new(
        target_x: u32,
        target_y: u32,
        columns: u32,
        rows: u32,
        data: Vec<f32>,
    ) -> Option<Self> {
        if (columns * rows) as usize != data.len() || target_x >= columns || target_y >= rows {
            return None;
        }

        Some(ConvolveMatrixData {
            target_x,
            target_y,
            columns,
            rows,
            data,
        })
    }

    /// Returns a matrix value at the specified position.
    ///
    /// # Panics
    ///
    /// - When position is out of bounds.
    pub fn get(&self, x: u32, y: u32) -> f32 {
        self.data[(y * self.columns + x) as usize]
    }
}

/// An edges processing mode.
#[allow(missing_docs)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum EdgeMode {
    None,
    Duplicate,
    Wrap,
}

/// A displacement map filter primitive.
///
/// `feDisplacementMap` element in the SVG.
#[derive(Clone, Debug)]
pub struct DisplacementMap {
    pub(crate) input1: Input,
    pub(crate) input2: Input,
    pub(crate) scale: f32,
    pub(crate) x_channel_selector: ColorChannel,
    pub(crate) y_channel_selector: ColorChannel,
}

impl DisplacementMap {
    /// Identifies input for the given filter primitive.
    ///
    /// `in` in the SVG.
    pub fn input1(&self) -> &Input {
        &self.input1
    }

    /// Identifies input for the given filter primitive.
    ///
    /// `in2` in the SVG.
    pub fn input2(&self) -> &Input {
        &self.input2
    }

    /// Scale factor.
    ///
    /// `scale` in the SVG.
    pub fn scale(&self) -> f32 {
        self.scale
    }

    /// Indicates a source color channel along the X-axis.
    ///
    /// `xChannelSelector` in the SVG.
    pub fn x_channel_selector(&self) -> ColorChannel {
        self.x_channel_selector
    }

    /// Indicates a source color channel along the Y-axis.
    ///
    /// `yChannelSelector` in the SVG.
    pub fn y_channel_selector(&self) -> ColorChannel {
        self.y_channel_selector
    }
}

/// A color channel.
#[allow(missing_docs)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ColorChannel {
    R,
    G,
    B,
    A,
}

/// A drop shadow filter primitive.
///
/// This is essentially `feGaussianBlur`, `feOffset` and `feFlood` joined together.
///
/// `feDropShadow` element in the SVG.
#[derive(Clone, Debug)]
pub struct DropShadow {
    pub(crate) input: Input,
    pub(crate) dx: f32,
    pub(crate) dy: f32,
    pub(crate) std_dev_x: PositiveF32,
    pub(crate) std_dev_y: PositiveF32,
    pub(crate) color: Color,
    pub(crate) opacity: Opacity,
}

impl DropShadow {
    /// Identifies input for the given filter primitive.
    ///
    /// `in` in the SVG.
    pub fn input(&self) -> &Input {
        &self.input
    }

    /// The amount to offset the input graphic along the X-axis.
    pub fn dx(&self) -> f32 {
        self.dx
    }

    /// The amount to offset the input graphic along the Y-axis.
    pub fn dy(&self) -> f32 {
        self.dy
    }

    /// A standard deviation along the X-axis.
    ///
    /// `stdDeviation` in the SVG.
    pub fn std_dev_x(&self) -> PositiveF32 {
        self.std_dev_x
    }

    /// A standard deviation along the Y-axis.
    ///
    /// `stdDeviation` in the SVG.
    pub fn std_dev_y(&self) -> PositiveF32 {
        self.std_dev_y
    }

    /// A flood color.
    ///
    /// `flood-color` in the SVG.
    pub fn color(&self) -> Color {
        self.color
    }

    /// A flood opacity.
    ///
    /// `flood-opacity` in the SVG.
    pub fn opacity(&self) -> Opacity {
        self.opacity
    }
}

/// A flood filter primitive.
///
/// `feFlood` element in the SVG.
#[derive(Clone, Copy, Debug)]
pub struct Flood {
    pub(crate) color: Color,
    pub(crate) opacity: Opacity,
}

impl Flood {
    /// A flood color.
    ///
    /// `flood-color` in the SVG.
    pub fn color(&self) -> Color {
        self.color
    }

    /// A flood opacity.
    ///
    /// `flood-opacity` in the SVG.
    pub fn opacity(&self) -> Opacity {
        self.opacity
    }
}

/// A Gaussian blur filter primitive.
///
/// `feGaussianBlur` element in the SVG.
#[derive(Clone, Debug)]
pub struct GaussianBlur {
    pub(crate) input: Input,
    pub(crate) std_dev_x: PositiveF32,
    pub(crate) std_dev_y: PositiveF32,
}

impl GaussianBlur {
    /// Identifies input for the given filter primitive.
    ///
    /// `in` in the SVG.
    pub fn input(&self) -> &Input {
        &self.input
    }

    /// A standard deviation along the X-axis.
    ///
    /// `stdDeviation` in the SVG.
    pub fn std_dev_x(&self) -> PositiveF32 {
        self.std_dev_x
    }

    /// A standard deviation along the Y-axis.
    ///
    /// `stdDeviation` in the SVG.
    pub fn std_dev_y(&self) -> PositiveF32 {
        self.std_dev_y
    }
}

/// An image filter primitive.
///
/// `feImage` element in the SVG.
#[derive(Clone, Debug)]
pub struct Image {
    pub(crate) root: Group,
}

impl Image {
    /// `feImage` children.
    pub fn root(&self) -> &Group {
        &self.root
    }
}

/// A diffuse lighting filter primitive.
///
/// `feDiffuseLighting` element in the SVG.
#[derive(Clone, Debug)]
pub struct DiffuseLighting {
    pub(crate) input: Input,
    pub(crate) surface_scale: f32,
    pub(crate) diffuse_constant: f32,
    pub(crate) lighting_color: Color,
    pub(crate) light_source: LightSource,
}

impl DiffuseLighting {
    /// Identifies input for the given filter primitive.
    ///
    /// `in` in the SVG.
    pub fn input(&self) -> &Input {
        &self.input
    }

    /// A surface scale.
    ///
    /// `surfaceScale` in the SVG.
    pub fn surface_scale(&self) -> f32 {
        self.surface_scale
    }

    /// A diffuse constant.
    ///
    /// `diffuseConstant` in the SVG.
    pub fn diffuse_constant(&self) -> f32 {
        self.diffuse_constant
    }

    /// A lighting color.
    ///
    /// `lighting-color` in the SVG.
    pub fn lighting_color(&self) -> Color {
        self.lighting_color
    }

    /// A light source.
    pub fn light_source(&self) -> LightSource {
        self.light_source
    }
}

/// A specular lighting filter primitive.
///
/// `feSpecularLighting` element in the SVG.
#[derive(Clone, Debug)]
pub struct SpecularLighting {
    pub(crate) input: Input,
    pub(crate) surface_scale: f32,
    pub(crate) specular_constant: f32,
    pub(crate) specular_exponent: f32,
    pub(crate) lighting_color: Color,
    pub(crate) light_source: LightSource,
}

impl SpecularLighting {
    /// Identifies input for the given filter primitive.
    ///
    /// `in` in the SVG.
    pub fn input(&self) -> &Input {
        &self.input
    }

    /// A surface scale.
    ///
    /// `surfaceScale` in the SVG.
    pub fn surface_scale(&self) -> f32 {
        self.surface_scale
    }

    /// A specular constant.
    ///
    /// `specularConstant` in the SVG.
    pub fn specular_constant(&self) -> f32 {
        self.specular_constant
    }

    /// A specular exponent.
    ///
    /// Should be in 1..128 range.
    ///
    /// `specularExponent` in the SVG.
    pub fn specular_exponent(&self) -> f32 {
        self.specular_exponent
    }

    /// A lighting color.
    ///
    /// `lighting-color` in the SVG.
    pub fn lighting_color(&self) -> Color {
        self.lighting_color
    }

    /// A light source.
    pub fn light_source(&self) -> LightSource {
        self.light_source
    }
}

/// A light source kind.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug)]
pub enum LightSource {
    DistantLight(DistantLight),
    PointLight(PointLight),
    SpotLight(SpotLight),
}

/// A distant light source.
///
/// `feDistantLight` element in the SVG.
#[derive(Clone, Copy, Debug)]
pub struct DistantLight {
    /// Direction angle for the light source on the XY plane (clockwise),
    /// in degrees from the x axis.
    ///
    /// `azimuth` in the SVG.
    pub azimuth: f32,

    /// Direction angle for the light source from the XY plane towards the z axis, in degrees.
    ///
    /// `elevation` in the SVG.
    pub elevation: f32,
}

/// A point light source.
///
/// `fePointLight` element in the SVG.
#[derive(Clone, Copy, Debug)]
pub struct PointLight {
    /// X location for the light source.
    ///
    /// `x` in the SVG.
    pub x: f32,

    /// Y location for the light source.
    ///
    /// `y` in the SVG.
    pub y: f32,

    /// Z location for the light source.
    ///
    /// `z` in the SVG.
    pub z: f32,
}

/// A spot light source.
///
/// `feSpotLight` element in the SVG.
#[derive(Clone, Copy, Debug)]
pub struct SpotLight {
    /// X location for the light source.
    ///
    /// `x` in the SVG.
    pub x: f32,

    /// Y location for the light source.
    ///
    /// `y` in the SVG.
    pub y: f32,

    /// Z location for the light source.
    ///
    /// `z` in the SVG.
    pub z: f32,

    /// X point at which the light source is pointing.
    ///
    /// `pointsAtX` in the SVG.
    pub points_at_x: f32,

    /// Y point at which the light source is pointing.
    ///
    /// `pointsAtY` in the SVG.
    pub points_at_y: f32,

    /// Z point at which the light source is pointing.
    ///
    /// `pointsAtZ` in the SVG.
    pub points_at_z: f32,

    /// Exponent value controlling the focus for the light source.
    ///
    /// `specularExponent` in the SVG.
    pub specular_exponent: PositiveF32,

    /// A limiting cone which restricts the region where the light is projected.
    ///
    /// `limitingConeAngle` in the SVG.
    pub limiting_cone_angle: Option<f32>,
}

/// A merge filter primitive.
///
/// `feMerge` element in the SVG.
#[derive(Clone, Debug)]
pub struct Merge {
    pub(crate) inputs: Vec<Input>,
}

impl Merge {
    /// List of input layers that should be merged.
    ///
    /// List of `feMergeNode`'s in the SVG.
    pub fn inputs(&self) -> &[Input] {
        &self.inputs
    }
}

/// A morphology filter primitive.
///
/// `feMorphology` element in the SVG.
#[derive(Clone, Debug)]
pub struct Morphology {
    pub(crate) input: Input,
    pub(crate) operator: MorphologyOperator,
    pub(crate) radius_x: PositiveF32,
    pub(crate) radius_y: PositiveF32,
}

impl Morphology {
    /// Identifies input for the given filter primitive.
    ///
    /// `in` in the SVG.
    pub fn input(&self) -> &Input {
        &self.input
    }

    /// A filter operator.
    ///
    /// `operator` in the SVG.
    pub fn operator(&self) -> MorphologyOperator {
        self.operator
    }

    /// A filter radius along the X-axis.
    ///
    /// A value of zero disables the effect of the given filter primitive.
    ///
    /// `radius` in the SVG.
    pub fn radius_x(&self) -> PositiveF32 {
        self.radius_x
    }

    /// A filter radius along the Y-axis.
    ///
    /// A value of zero disables the effect of the given filter primitive.
    ///
    /// `radius` in the SVG.
    pub fn radius_y(&self) -> PositiveF32 {
        self.radius_y
    }
}

/// A morphology operation.
#[allow(missing_docs)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum MorphologyOperator {
    Erode,
    Dilate,
}

/// An offset filter primitive.
///
/// `feOffset` element in the SVG.
#[derive(Clone, Debug)]
pub struct Offset {
    pub(crate) input: Input,
    pub(crate) dx: f32,
    pub(crate) dy: f32,
}

impl Offset {
    /// Identifies input for the given filter primitive.
    ///
    /// `in` in the SVG.
    pub fn input(&self) -> &Input {
        &self.input
    }

    /// The amount to offset the input graphic along the X-axis.
    pub fn dx(&self) -> f32 {
        self.dx
    }

    /// The amount to offset the input graphic along the Y-axis.
    pub fn dy(&self) -> f32 {
        self.dy
    }
}

/// A tile filter primitive.
///
/// `feTile` element in the SVG.
#[derive(Clone, Debug)]
pub struct Tile {
    pub(crate) input: Input,
}

impl Tile {
    /// Identifies input for the given filter primitive.
    ///
    /// `in` in the SVG.
    pub fn input(&self) -> &Input {
        &self.input
    }
}

/// A turbulence generation filter primitive.
///
/// `feTurbulence` element in the SVG.
#[derive(Clone, Copy, Debug)]
pub struct Turbulence {
    pub(crate) base_frequency_x: PositiveF32,
    pub(crate) base_frequency_y: PositiveF32,
    pub(crate) num_octaves: u32,
    pub(crate) seed: i32,
    pub(crate) stitch_tiles: bool,
    pub(crate) kind: TurbulenceKind,
}

impl Turbulence {
    /// Identifies the base frequency for the noise function.
    ///
    /// `baseFrequency` in the SVG.
    pub fn base_frequency_x(&self) -> PositiveF32 {
        self.base_frequency_x
    }

    /// Identifies the base frequency for the noise function.
    ///
    /// `baseFrequency` in the SVG.
    pub fn base_frequency_y(&self) -> PositiveF32 {
        self.base_frequency_y
    }

    /// Identifies the number of octaves for the noise function.
    ///
    /// `numOctaves` in the SVG.
    pub fn num_octaves(&self) -> u32 {
        self.num_octaves
    }

    /// The starting number for the pseudo random number generator.
    ///
    /// `seed` in the SVG.
    pub fn seed(&self) -> i32 {
        self.seed
    }

    /// Smooth transitions at the border of tiles.
    ///
    /// `stitchTiles` in the SVG.
    pub fn stitch_tiles(&self) -> bool {
        self.stitch_tiles
    }

    /// Indicates whether the filter primitive should perform a noise or turbulence function.
    ///
    /// `type` in the SVG.
    pub fn kind(&self) -> TurbulenceKind {
        self.kind
    }
}

/// A turbulence kind for the `feTurbulence` filter.
#[allow(missing_docs)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum TurbulenceKind {
    FractalNoise,
    Turbulence,
}
