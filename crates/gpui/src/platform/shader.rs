use bytemuck::{Pod, Zeroable};

#[cfg(not(any(target_os = "linux", all(target_os = "macos", feature = "macos-blade"))))]
use {
    crate::ShaderInstance,
    anyhow::{Context, Result, anyhow},
    naga::{
        Module, Type, TypeInner,
        front::wgsl,
        valid::{Capabilities, ModuleInfo, ValidationFlags, Validator},
    },
};

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct CustomShaderGlobalParams {
    pub viewport_size: [f32; 2],
    pub pad: [u32; 2],
}

#[cfg(not(any(target_os = "linux", all(target_os = "macos", feature = "macos-blade"))))]
pub fn naga_validate_custom_shader(
    source: &str,
    data_struct_name: Option<&str>,
    data_size: usize,
    data_align: usize,
) -> Result<(Module, ModuleInfo, usize)> {
    let module = wgsl::parse_str(source)?;
    let module_info = Validator::new(
        ValidationFlags::all() ^ ValidationFlags::BINDINGS,
        Capabilities::empty(),
    )
    .validate(&module)
    .context("naga validation failed")?;

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
fn check_struct_size(module: &Module, name: &str, expected_size: usize) -> Result<()> {
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
                return Err(anyhow!(
                    "`{name}` struct was the incorrect size. Expected {expected_size}, found {}",
                    *span
                ));
            }
        }
        _ => return Err(anyhow!("`{name}` struct not found in shader")),
    }

    Ok(())
}
