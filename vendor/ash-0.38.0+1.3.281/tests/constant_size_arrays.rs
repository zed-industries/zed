use ash::vk::{PhysicalDeviceProperties, PipelineColorBlendStateCreateInfo};

#[test]
fn assert_struct_field_is_array() {
    let pipeline_cache_uuid: [u8; 16] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

    let _ = PhysicalDeviceProperties::default().pipeline_cache_uuid(pipeline_cache_uuid);

    let _ = PhysicalDeviceProperties {
        pipeline_cache_uuid,
        ..Default::default()
    };

    let blend_constants: [f32; 4] = [0.0, 0.0, 0.0, 0.0];

    let _ = PipelineColorBlendStateCreateInfo::default().blend_constants(blend_constants);

    let _ = PipelineColorBlendStateCreateInfo {
        blend_constants,
        ..Default::default()
    };
}

#[test]
#[allow(dead_code)]
fn assert_ffi_array_param_is_pointer() {
    // don't run it, just make sure it compiles
    unsafe fn dummy(device: &ash::Device, cmd_buffer: ash::vk::CommandBuffer) {
        let blend_constants: [f32; 4] = [0.0, 0.0, 0.0, 0.0];

        device.cmd_set_blend_constants(cmd_buffer, &blend_constants);
    }
}
