use std::marker::PhantomData;

use bytemuck::Pod;
use refineable::Refineable;

use crate::{
    App, Bounds, Element, ElementId, GlobalElementId, InspectorElementId, IntoElement, LayoutId,
    Pixels, Style, StyleRefinement, Styled, Window,
};

/// An element for custom rendering.
pub struct ShaderElement<T: Pod> {
    style: StyleRefinement,
    shader: CustomShader<T>,
    user_data: T,
}

/// Create a new shader element with `T` user-data.
pub fn custom_shader<T: Pod>(shader: CustomShader<T>, user_data: T) -> ShaderElement<T> {
    ShaderElement {
        style: Default::default(),
        shader,
        user_data,
    }
}

impl<T: Pod> IntoElement for ShaderElement<T> {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

impl<T: Pod> Element for ShaderElement<T> {
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
        window
            .paint_shader(bounds, &self.shader, &self.user_data)
            .unwrap();
    }
}

impl<T: Pod> Styled for ShaderElement<T> {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

/// An a shader which can be rendered by `shader`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct CustomShader<T: Pod> {
    pub(crate) source: String,
    _marker: PhantomData<T>,
}

impl<T: Pod> CustomShader<T> {
    /// Create a new fragment shader with per-instance user-data
    pub fn new_fragment(
        fragment_body: &str,
        user_data_definition: &str,
        extra_definitions: &str,
    ) -> Self {
        Self {
            source: format!(
                r#"
                struct GlobalParams {{
                    viewport_size: vec2<f32>,
                    pad1: u32,
                    pad2: u32,
                }}

                var<uniform> globals: GlobalParams;

                fn to_device_position_impl(position: vec2<f32>) -> vec4<f32> {{
                    let device_position = position / globals.viewport_size * vec2<f32>(2.0, -2.0) + vec2<f32>(-1.0, 1.0);
                    return vec4<f32>(device_position, 0.0, 1.0);
                }}

                fn to_device_position(unit_vertex: vec2<f32>, bounds: Bounds) -> vec4<f32> {{
                    let position = unit_vertex * vec2<f32>(bounds.size) + bounds.origin;
                    return to_device_position_impl(position);
                }}

                fn distance_from_clip_rect_impl(position: vec2<f32>, clip_bounds: Bounds) -> vec4<f32> {{
                    let tl = position - clip_bounds.origin;
                    let br = clip_bounds.origin + clip_bounds.size - position;
                    return vec4<f32>(tl.x, br.x, tl.y, br.y);
                }}

                fn distance_from_clip_rect(unit_vertex: vec2<f32>, bounds: Bounds, clip_bounds: Bounds) -> vec4<f32> {{
                    let position = unit_vertex * vec2<f32>(bounds.size) + bounds.origin;
                    return distance_from_clip_rect_impl(position, clip_bounds);
                }}

                struct Bounds {{
                    origin: vec2<f32>,
                    size: vec2<f32>,
                }}

                struct UserData {{
                    {user_data_definition}
                }}

                struct Instance {{
                    bounds: Bounds,
                    content_mask: Bounds,
                    user_data: UserData,
                }}

                struct Instances {{
                    instances: array<Instance>,
                }}

                var<storage, read> b_instances: Instances;

                struct VertexOut {{
                    @builtin(position) position: vec4<f32>,
                    @location(0) clip_distances: vec4<f32>,
                    @location(1) origin: vec2<f32>,
                    @location(2) size: vec2<f32>,
                    @location(3) instance_id: u32,
                }}

                @vertex
                fn vs(@builtin(vertex_index) vertex_id: u32, @builtin(instance_index) instance_id: u32) -> VertexOut {{
                    let unit_vertex = vec2<f32>(f32(vertex_id & 1u), 0.5 * f32(vertex_id & 2u));
                    let instance = b_instances.instances[instance_id];

                    var out = VertexOut();
                    out.position = to_device_position(unit_vertex, instance.bounds);
                    out.clip_distances = distance_from_clip_rect(unit_vertex, instance.bounds, instance.content_mask);
                    out.origin = instance.bounds.origin;
                    out.size = instance.bounds.size;
                    out.instance_id = instance_id;
                    return out;
                }}

                {extra_definitions}

                @fragment
                fn fs(input: VertexOut) -> @location(0) vec4<f32> {{
                    if (any(input.clip_distances < vec4<f32>(0.0))) {{
                        return vec4<f32>(0.0);
                    }}

                    let user_data = b_instances.instances[input.instance_id].user_data;
                    {fragment_body}
                }}
                "#
            ),
            _marker: PhantomData,
        }
    }
}
