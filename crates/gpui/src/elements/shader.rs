use std::marker::PhantomData;

use crate::{
    App, Bounds, Element, ElementId, GlobalElementId, Hitbox, InspectorElementId,
    InteractiveElement, Interactivity, IntoElement, LayoutId, Pixels, StyleRefinement, Styled,
    Window,
};

/// Fragment shader which can be rendered using `shader_element` or `shader_element_with_data`.
#[derive(Clone)]
pub struct FragmentShader<T: ShaderUniform> {
    main_body: String,
    extra_items: Option<String>,
    _marker: PhantomData<T>,
}

impl<T: ShaderUniform> FragmentShader<T> {
    /// Create a new fragment shader
    pub fn new(main_body: &str) -> Self {
        Self {
            main_body: main_body.to_string(),
            extra_items: None,
            _marker: PhantomData,
        }
    }

    /// Adds an extra item (struct, function, etc.) to the WGSL source code
    pub fn with_item(mut self, item: &str) -> Self {
        if let Some(defs) = &mut self.extra_items {
            defs.push_str(item);
        } else {
            self.extra_items = Some(item.to_string());
        }

        self
    }
}

/// An element which can render an instance of a fragment shader. Use `shader_element` or `shader_element_with_data` to construct.
pub struct ShaderElement<T: ShaderUniform> {
    shader: FragmentShader<T>,
    instance_data: T,
    interactivity: Interactivity,
}

impl<T: ShaderUniform> Styled for ShaderElement<T> {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.interactivity.base_style
    }
}

impl<T: ShaderUniform> InteractiveElement for ShaderElement<T> {
    fn interactivity(&mut self) -> &mut Interactivity {
        &mut self.interactivity
    }
}

/// Constructs a `ShaderElement` which renders a shader which *doesn't* take instance-data.
pub fn shader_element(shader: FragmentShader<()>) -> ShaderElement<()> {
    ShaderElement {
        shader,
        instance_data: (),
        interactivity: Interactivity::new(),
    }
}

/// Constructs a `ShaderElement` which renders a shader with custom data for each instance.
pub fn shader_element_with_data<T: ShaderUniform>(
    shader: FragmentShader<T>,
    instance_data: T,
) -> ShaderElement<T> {
    ShaderElement {
        shader,
        instance_data,
        interactivity: Interactivity::new(),
    }
}

impl<T: ShaderUniform> IntoElement for ShaderElement<T> {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

impl<T: ShaderUniform> Element for ShaderElement<T> {
    type RequestLayoutState = ();
    type PrepaintState = Option<Hitbox>;

    fn id(&self) -> Option<ElementId> {
        self.interactivity.element_id.clone()
    }

    fn source_location(&self) -> Option<&'static std::panic::Location<'static>> {
        self.interactivity.source_location
    }

    fn request_layout(
        &mut self,
        global_id: Option<&GlobalElementId>,
        inspector_id: Option<&InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, Self::RequestLayoutState) {
        let layout_id = self.interactivity.request_layout(
            global_id,
            inspector_id,
            window,
            cx,
            |style, window, cx| window.request_layout(style, None, cx),
        );
        (layout_id, ())
    }

    fn prepaint(
        &mut self,
        global_id: Option<&GlobalElementId>,
        inspector_id: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        window: &mut Window,
        cx: &mut App,
    ) -> Self::PrepaintState {
        self.interactivity.prepaint(
            global_id,
            inspector_id,
            bounds,
            bounds.size,
            window,
            cx,
            |_, _, hitbox, _, _| hitbox,
        )
    }

    fn paint(
        &mut self,
        global_id: Option<&GlobalElementId>,
        inspector_id: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        hitbox: &mut Option<Hitbox>,
        window: &mut Window,
        cx: &mut App,
    ) {
        let (instance_data_field, instance_data_param, instance_data_arg) = if size_of::<T>() != 0 {
            (
                format!("instance_data: {},", T::NAME),
                format!(", data: {}", T::NAME),
                ", b_instances.instances[input.instance_id].instance_data",
            )
        } else {
            (String::new(), String::new(), "")
        };
        let instance_data_definition = T::DEFINITION.unwrap_or("");

        let source = format!(
            r#"
            struct GlobalParams {{
                viewport_size: vec2<f32>,
                premultiplied_alpha: u32,
                opacity: f32,
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

            {instance_data_definition}

            struct Instance {{
                bounds: Bounds,
                content_mask: Bounds,
                opacity: f32,
                {instance_data_field}
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
                @location(3) opacity: f32,
                @location(4) instance_id: u32,
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
                out.opacity = instance.opacity;
                out.instance_id = instance_id;
                return out;
            }}

            {}

            fn user_fs(input: VertexOut{instance_data_param}) -> vec4<f32> {{
                {}
            }}

            @fragment
            fn fs(input: VertexOut) -> @location(0) vec4<f32> {{
                if (any(input.clip_distances < vec4<f32>(0.0))) {{
                    return vec4<f32>(0.0);
                }}

                let color = user_fs(input{instance_data_arg});

                let alpha = color.a * input.opacity;
                let multiplier = select(1.0, alpha, globals.premultiplied_alpha != 0u);
                return vec4<f32>(color.rgb * multiplier, alpha);
            }}
            "#,
            self.shader.extra_items.as_deref().unwrap_or(""),
            self.shader.main_body
        );

        self.interactivity.paint(
            global_id,
            inspector_id,
            bounds,
            hitbox.as_ref(),
            window,
            cx,
            |_style, window, _cx| {
                window
                    .paint_shader(bounds, &source, &self.instance_data)
                    .unwrap();
            },
        );
    }
}

/// Marker trait for data which can be passed to custom WGSL shaders.
///
/// To create a custom structure, derive this trait:
///
/// ```rust
/// #[repr(C)]
/// #[derive(gpui::ShaderUniform, Clone, Copy)]
/// struct MyStruct {
///     a_vec4_field: [f32; 4],
///     some_other_field: u32,
///     // an_invalid_field: [f32; 2], // ERROR! This field can't be here since its offset is 20, which is not a multiple of 8
/// }
/// ```
pub unsafe trait ShaderUniform: Clone + Copy + 'static {
    /// The name of the type in wgsl (ie. `f32`, `InstanceData`)
    const NAME: &str;

    /// The type's definition, if it is a struct.
    const DEFINITION: Option<&str>;

    /// The wgsl alignment of this type in bytes
    const ALIGN: usize;
}

// Used to mark instance-data as unused. It is not allowed in instance-data structs.
unsafe impl ShaderUniform for () {
    const NAME: &str = "This shouldn't ever be emitted";
    const DEFINITION: Option<&str> = None;
    const ALIGN: usize = 1;
}

macro_rules! impl_scalar {
    ($ty:ty, $name:literal) => {
        unsafe impl ShaderUniform for $ty {
            const NAME: &str = $name;
            const DEFINITION: Option<&str> = None;
            const ALIGN: usize = 4;
        }

        unsafe impl ShaderUniform for [$ty; 2] {
            const NAME: &str = concat!("vec2<", $name, ">");
            const DEFINITION: Option<&str> = None;
            const ALIGN: usize = 8;
        }

        unsafe impl ShaderUniform for [$ty; 3] {
            const NAME: &str = concat!("vec3<", $name, ">");
            const DEFINITION: Option<&str> = None;
            const ALIGN: usize = 16;
        }

        unsafe impl ShaderUniform for [$ty; 4] {
            const NAME: &str = concat!("vec4<", $name, ">");
            const DEFINITION: Option<&str> = None;
            const ALIGN: usize = 16;
        }
    };
}

impl_scalar!(u32, "u32");
impl_scalar!(i32, "i32");
impl_scalar!(f32, "f32");
