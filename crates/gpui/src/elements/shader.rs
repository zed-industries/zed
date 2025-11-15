use refineable::Refineable;

use crate::{
    App, Bounds, Element, ElementId, GlobalElementId, InspectorElementId, IntoElement, LayoutId,
    Pixels, Style, StyleRefinement, Styled, Window,
};

/// An element for custom rendering.
pub struct FragmentShader {
    style: StyleRefinement,
}

/// Create a new shader element.
pub fn shader() -> FragmentShader {
    FragmentShader {
        style: Default::default(),
    }
}

impl IntoElement for FragmentShader {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

impl Element for FragmentShader {
    type RequestLayoutState = ();

    type PrepaintState = ();

    fn id(&self) -> Option<ElementId> {
        None
    }

    fn source_location(&self) -> Option<&'static std::panic::Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, Self::RequestLayoutState) {
        let mut style = Style::default();
        style.refine(&self.style);

        let layout_id = window.request_layout(style, [], cx);
        (layout_id, ())
    }

    fn prepaint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        _bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        _window: &mut Window,
        _cx: &mut App,
    ) -> Self::PrepaintState {
    }

    fn paint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        _prepaint: &mut Self::PrepaintState,
        window: &mut Window,
        _cx: &mut App,
    ) {
        let shader = CustomShader {
            source: r#"
            struct GlobalParams {
                viewport_size: vec2<f32>,
                premultiplied_alpha: u32,
                pad: u32,
            }

            var<uniform> globals: GlobalParams;

            fn to_device_position_impl(position: vec2<f32>) -> vec4<f32> {
                let device_position = position / globals.viewport_size * vec2<f32>(2.0, -2.0) + vec2<f32>(-1.0, 1.0);
                return vec4<f32>(device_position, 0.0, 1.0);
            }

            fn to_device_position(unit_vertex: vec2<f32>, bounds: Bounds) -> vec4<f32> {
                let position = unit_vertex * vec2<f32>(bounds.size) + bounds.origin;
                return to_device_position_impl(position);
            }

            fn distance_from_clip_rect_impl(position: vec2<f32>, clip_bounds: Bounds) -> vec4<f32> {
                let tl = position - clip_bounds.origin;
                let br = clip_bounds.origin + clip_bounds.size - position;
                return vec4<f32>(tl.x, br.x, tl.y, br.y);
            }

            fn distance_from_clip_rect(unit_vertex: vec2<f32>, bounds: Bounds, clip_bounds: Bounds) -> vec4<f32> {
                let position = unit_vertex * vec2<f32>(bounds.size) + bounds.origin;
                return distance_from_clip_rect_impl(position, clip_bounds);
            }

            struct Bounds {
                origin: vec2<f32>,
                size: vec2<f32>,
            }

            struct PaintShader {
                order: u32,
                shader_id: u32,
                bounds: Bounds,
                content_mask: Bounds,
            }
            var<storage, read> b_shaders: array<PaintShader>;

            struct PaintShaderVarying {
                @builtin(position) position: vec4<f32>,
                @location(0) clip_distances: vec4<f32>,
            }

            @vertex
            fn vs(@builtin(vertex_index) vertex_id: u32, @builtin(instance_index) instance_id: u32) -> PaintShaderVarying {
                let unit_vertex = vec2<f32>(f32(vertex_id & 1u), 0.5 * f32(vertex_id & 2u));
                let view = b_shaders[instance_id];

                var out = PaintShaderVarying();
                out.position = to_device_position(unit_vertex, view.bounds);
                out.clip_distances = distance_from_clip_rect(unit_vertex, view.bounds, view.content_mask);
                return out;
            }

            @fragment
            fn fs(input: PaintShaderVarying) -> @location(0) vec4<f32> {
                if (any(input.clip_distances < vec4<f32>(0.0))) {
                    return vec4<f32>(0.0);
                }

                return vec4<f32>(input.position.x / globals.viewport_size.x, input.position.y / globals.viewport_size.y, 0.0, 1.0);
            }
            "#.to_string(),
        };

        // TODO: Better error type
        window.paint_shader(bounds, &shader).unwrap();
    }
}

impl Styled for FragmentShader {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

/// A custom rendering pipeline which can be executed as a primitive.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct CustomShader {
    /// The source code of the shader written in WGSL
    pub source: String,
}
