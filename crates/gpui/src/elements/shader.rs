use std::marker::PhantomData;

use smallvec::SmallVec;

use crate::{
    App, Bounds, CursorStyle, Element, ElementId, GlobalElementId, Hitbox, InspectorElementId,
    InteractiveElement, Interactivity, IntoElement, LayoutId, Pixels, SharedString,
    StyleRefinement, Window, fill, point, rgb,
};

/// Fragment shader which can be rendered using `shader_element` or `shader_element_with_data`.
#[derive(Clone)]
pub struct FragmentShader<T: ShaderUniform> {
    main_body: SharedString,
    extra_items: SmallVec<[SharedString; 4]>,
    _marker: PhantomData<T>,
}

impl<T: ShaderUniform> FragmentShader<T> {
    /// Create a new fragment shader
    pub fn new(main_body: &'static str) -> Self {
        Self {
            main_body: SharedString::new_static(main_body),
            extra_items: SmallVec::new(),
            _marker: PhantomData,
        }
    }

    /// Adds an extra item (struct, function, etc.) to the WGSL source code
    pub fn with_item(mut self, item: &'static str) -> Self {
        self.extra_items.push(SharedString::new_static(item));
        self
    }
}

/// An element which can render an instance of a fragment shader. Use `shader_element` or `shader_element_with_data` to construct.
pub struct ShaderElement<T: ShaderUniform> {
    shader: FragmentShader<T>,
    instance_data: T,
    interactivity: Interactivity,
}

impl<T: ShaderUniform> ShaderElement<T> {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.interactivity.base_style
    }

    gpui::visibility_style_methods!({
        visibility: pub
    });

    gpui::margin_style_methods!({
        visibility: pub
    });

    gpui::position_style_methods!({
        visibility: pub
    });

    gpui::size_style_methods!({
        visibility: pub
    });

    gpui::cursor_style_methods!({
        visibility: pub
    });
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
        self.interactivity.paint(
            global_id,
            inspector_id,
            bounds,
            hitbox.as_ref(),
            window,
            cx,
            |_style, window, _cx| match window.paint_shader(
                bounds,
                self.shader.main_body.clone(),
                self.shader.extra_items.clone(),
                &self.instance_data,
            ) {
                Ok(_) => {}
                Err((msg, first_err)) => {
                    for x in 0..5 {
                        for y in 0..5 {
                            window.paint_quad(fill(
                                Bounds {
                                    origin: bounds.origin
                                        + point(
                                            bounds.size.width / 5.0 * x,
                                            bounds.size.height / 5.0 * y,
                                        ),
                                    size: bounds.size / 5.0,
                                },
                                if (x + y) & 1 == 0 {
                                    rgb(0xff00ff)
                                } else {
                                    rgb(0x000000)
                                },
                            ));
                        }
                    }

                    if first_err {
                        eprintln!("Shader compile error: {msg}");
                    }
                }
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
