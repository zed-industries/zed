use std::{fmt::Display, hash::Hash};

use bytemuck::{Pod, Zeroable};
use smallvec::SmallVec;

#[derive(Clone, PartialEq, Eq, Hash)]
pub(crate) struct CustomShaderInfo {
    pub main_body: &'static str,
    pub extra_items: SmallVec<[&'static str; 4]>,
    pub data_name: &'static str,
    pub data_definition: Option<&'static str>,
    pub data_size: usize,
    pub data_align: usize,
}

impl Display for CustomShaderInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let instance_data_definition = self.data_definition.unwrap_or("");
        let main_body = self.main_body;
        let extra_items = self.extra_items.join("");

        let (instance_data_field, instance_data_param, instance_data_arg) = if self.data_size != 0 {
            (
                format!("instance_data: {}", self.data_name),
                format!(", data: {}", self.data_name),
                ", b_instances.instances[input.instance_id].instance_data",
            )
        } else {
            (String::new(), String::new(), "")
        };

        write!(
            f,
            r#"
        struct GlobalParams {{
            viewport_size: vec2<f32>,
            premultiplied_alpha: u32,
            pad: u32,
        }}

        var<uniform> globals: GlobalParams;

        fn to_device_position(unit_vertex: vec2<f32>, bounds: Bounds) -> vec2<f32> {{
            let position = unit_vertex * bounds.size + bounds.origin;
            return position / globals.viewport_size * vec2<f32>(2.0, -2.0) + vec2<f32>(-1.0, 1.0);
        }}

        fn distance_from_clip_rect(unit_vertex: vec2<f32>, bounds: Bounds, clip_bounds: Bounds) -> vec4<f32> {{
            let position = unit_vertex * bounds.size + bounds.origin;
            let tl = position - clip_bounds.origin;
            let br = clip_bounds.origin + clip_bounds.size - position;
            return vec4<f32>(tl.x, br.x, tl.y, br.y);
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
            scale_factor: f32,
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
            @location(4) scale_factor: f32,
            @location(5) instance_id: u32,
        }}

        @vertex
        fn vs(@builtin(vertex_index) vertex_id: u32, @builtin(instance_index) instance_id: u32) -> VertexOut {{
            let unit_vertex = vec2<f32>(f32(vertex_id & 1u), 0.5 * f32(vertex_id & 2u));
            let instance = b_instances.instances[instance_id];

            var out = VertexOut();
            out.position = vec4<f32>(to_device_position(unit_vertex, instance.bounds), 0.0, 1.0);
            out.clip_distances = distance_from_clip_rect(unit_vertex, instance.bounds, instance.content_mask);
            out.origin = instance.bounds.origin / instance.scale_factor;
            out.size = instance.bounds.size / instance.scale_factor;
            out.opacity = instance.opacity;
            out.scale_factor = instance.scale_factor;
            out.instance_id = instance_id;

            return out;
        }}

        {extra_items}

        fn user_fs(position: vec2<f32>, bounds: Bounds, scale_factor: f32{instance_data_param}) -> vec4<f32> {{
            {main_body}
        }}

        @fragment
        fn fs(input: VertexOut) -> @location(0) vec4<f32> {{
            if (any(input.clip_distances < vec4<f32>(0.0))) {{
                return vec4<f32>(0.0);
            }}

            let color = user_fs(
                input.position.xy / input.scale_factor,
                Bounds(input.origin, input.size),
                input.scale_factor
                {instance_data_arg}
            );

            let alpha = color.a * input.opacity;
            let multiplier = select(1.0, alpha, globals.premultiplied_alpha != 0u);
            return vec4<f32>(color.rgb * multiplier, alpha);
        }}
        "#
        )
    }
}

#[cfg(not(any(target_os = "linux", all(target_os = "macos", feature = "macos-blade"))))]
use {
    crate::ShaderInstance,
    naga::{
        Module, Type, TypeInner,
        front::wgsl,
        valid::{Capabilities, ModuleInfo, ValidationFlags, Validator},
    },
};

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub(super) struct CustomShaderGlobalParams {
    pub viewport_size: [f32; 2],
    pub premultiplied_alpha: u32,
    pub pad: u32,
}

#[cfg(not(any(target_os = "linux", all(target_os = "macos", feature = "macos-blade"))))]
pub(super) fn naga_validate_custom_shader(
    source: &str,
    data_struct_name: Option<&str>,
    data_size: usize,
    data_align: usize,
) -> Result<(Module, ModuleInfo, usize), String> {
    let module = wgsl::parse_str(source).map_err(|err| err.to_string())?;
    let module_info = Validator::new(
        ValidationFlags::all() ^ ValidationFlags::BINDINGS,
        Capabilities::empty(),
    )
    .validate(&module)
    .map_err(|err| format!("naga validation failed: {err}"))?;

    if let Some(data_struct_name) = data_struct_name {
        check_struct_size(
            &module,
            data_struct_name,
            data_size.next_multiple_of(data_align),
        )?;
    }

    let (_, instance_size) = ShaderInstance::size_info(data_size, data_align);
    check_struct_size(&module, "Instance", instance_size)?;
    check_struct_size(
        &module,
        "GlobalParams",
        size_of::<CustomShaderGlobalParams>(),
    )?;

    Ok((module, module_info, instance_size))
}

#[cfg(not(any(target_os = "linux", all(target_os = "macos", feature = "macos-blade"))))]
fn check_struct_size(module: &Module, name: &str, expected_size: usize) -> Result<(), String> {
    match module
        .types
        .iter()
        .find(|(_, ty)| ty.name.as_deref() == Some(name))
    {
        Some((
            _,
            Type {
                inner: TypeInner::Struct { span, .. },
                ..
            },
        )) => {
            if *span as usize != expected_size {
                return Err(format!(
                    "`{name}` struct was the incorrect size. Expected {expected_size}, found {}",
                    *span
                ));
            }
        }
        _ => return Err(format!("`{name}` struct not found in shader")),
    }

    Ok(())
}
