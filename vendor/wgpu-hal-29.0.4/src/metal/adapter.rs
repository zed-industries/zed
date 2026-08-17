use objc2::runtime::{AnyObject, ProtocolObject, Sel};
use objc2::{available, sel};
use objc2_foundation::{NSOperatingSystemVersion, NSProcessInfo};
use objc2_metal::{
    MTLArgumentBuffersTier, MTLCounterSamplingPoint, MTLDevice, MTLFeatureSet, MTLGPUFamily,
    MTLIndirectAccelerationStructureInstanceDescriptor, MTLLanguageVersion, MTLPixelFormat,
    MTLReadWriteTextureTier,
};
use wgt::{AstcBlock, AstcChannel};

use alloc::{string::ToString as _, sync::Arc, vec::Vec};
use core::sync::atomic;

use crate::metal::QueueShared;

use super::{OsFeatures, TimestampQuerySupport};

/// Check if a device's class has a given method in its method table.
///
/// This mirrors the check that `objc2` performs internally (in debug builds)
/// before sending a message. We use it to skip method calls that would panic
/// on proxy objects like Apple's `CaptureMTLDevice`, which forwards messages
/// at runtime but doesn't declare the methods in its class.
fn device_class_responds_to(device: &ProtocolObject<dyn MTLDevice>, sel: Sel) -> bool {
    AnyObject::class(device.as_ref()).responds_to(sel)
}

/// Maximum number of command buffers for `MTLCommandQueue`s that we create.
///
/// If a [new command buffer] is requested when Metal has run out of command
/// buffers, it waits indefinitely for one to become available. If the
/// outstanding command buffers are actively executing on the GPU, this will
/// happen relatively quickly. But if the outstanding command buffers will only
/// be recovered upon GC, and attempting to get a new command buffer prevents
/// forward progress towards that GC, there is a deadlock.
///
/// This is mostly a problem for the CTS, which frequently creates command
/// buffers that it does not submit. It is unclear how likely command buffer
/// exhaustion is in real applications.
///
/// This limit was increased from a previous value of 2048 for
/// <https://bugzilla.mozilla.org/show_bug.cgi?id=1971452>.
///
/// [new command buffer]: https://developer.apple.com/documentation/metal/mtlcommandqueue/makecommandbuffer()?language=objc
pub(super) const MAX_COMMAND_BUFFERS: usize = 4096;

unsafe impl Send for super::Adapter {}
unsafe impl Sync for super::Adapter {}

impl super::Adapter {
    pub(super) fn new(shared: Arc<super::AdapterShared>) -> Self {
        Self { shared }
    }
}

impl crate::Adapter for super::Adapter {
    type A = super::Api;

    unsafe fn open(
        &self,
        features: wgt::Features,
        _limits: &wgt::Limits,
        _memory_hints: &wgt::MemoryHints,
    ) -> Result<crate::OpenDevice<super::Api>, crate::DeviceError> {
        let queue = self
            .shared
            .device
            .newCommandQueueWithMaxCommandBufferCount(MAX_COMMAND_BUFFERS)
            .unwrap();

        // Acquiring the meaning of timestamp ticks is hard with Metal!
        // The only thing there is a method correlating cpu & gpu timestamps (`device.sample_timestamps`).
        // Users are supposed to call this method twice and calculate the difference,
        // see "Converting GPU Timestamps into CPU Time":
        // https://developer.apple.com/documentation/metal/gpu_counters_and_counter_sample_buffers/converting_gpu_timestamps_into_cpu_time
        // Not only does this mean we get an approximate value, this is as also *very slow*!
        // Chromium opted to solve this using a linear regression that they stop at some point
        // https://source.chromium.org/chromium/chromium/src/+/refs/heads/main:third_party/dawn/src/dawn/native/metal/DeviceMTL.mm;drc=76be2f9f117654f3fe4faa477b0445114fccedda;bpv=0;bpt=1;l=46
        // Generally, the assumption is that timestamp values aren't changing over time, after all all other APIs provide stable values.
        //
        // We should do as Chromium does for the general case, but this requires quite some state tracking
        // and doesn't even provide perfectly accurate values, especially at the start of the application when
        // we didn't have the chance to sample a lot of values just yet.
        //
        // So instead, we're doing the dangerous but easy thing and use our "knowledge" of timestamps
        // conversions on different devices, after all Metal isn't supported on that many ;)
        // Based on:
        // * https://github.com/gfx-rs/wgpu/pull/2528
        // * https://github.com/gpuweb/gpuweb/issues/1325#issuecomment-761041326
        let timestamp_period = if self.shared.device.name().to_string().starts_with("Intel") {
            83.333
        } else {
            // Known for Apple Silicon (at least M1 & M2, iPad Pro 2018) and AMD GPUs.
            1.0
        };

        Ok(crate::OpenDevice {
            device: super::Device {
                shared: Arc::clone(&self.shared),
                features,
                counters: Default::default(),
            },
            queue: super::Queue {
                shared: Arc::new(QueueShared {
                    raw: queue,
                    command_buffer_created_not_submitted: atomic::AtomicUsize::new(0),
                }),
                timestamp_period,
            },
        })
    }

    unsafe fn texture_format_capabilities(
        &self,
        format: wgt::TextureFormat,
    ) -> crate::TextureFormatCapabilities {
        use crate::TextureFormatCapabilities as Tfc;
        use wgt::TextureFormat as Tf;

        let msl_version = self.shared.private_caps.msl_version;
        let pc = &self.shared.private_texture_format_caps;
        // Affected formats documented at:
        // https://developer.apple.com/documentation/metal/mtlreadwritetexturetier/mtlreadwritetexturetier1?language=objc
        // https://developer.apple.com/documentation/metal/mtlreadwritetexturetier/mtlreadwritetexturetier2?language=objc
        let (read_write_tier1_if, read_write_tier2_if) = match pc.read_write_texture_tier {
            MTLReadWriteTextureTier::TierNone => (Tfc::empty(), Tfc::empty()),
            MTLReadWriteTextureTier::Tier1 => (Tfc::STORAGE_READ_WRITE, Tfc::empty()),
            MTLReadWriteTextureTier::Tier2 => (Tfc::STORAGE_READ_WRITE, Tfc::STORAGE_READ_WRITE),
            // Unknown levels of support are likely higher than Tier 2.
            _ => (Tfc::STORAGE_READ_WRITE, Tfc::STORAGE_READ_WRITE),
        };
        let msaa_count = pc.sample_count_mask;

        let msaa_resolve_desktop_if = if pc.msaa_desktop {
            Tfc::MULTISAMPLE_RESOLVE
        } else {
            Tfc::empty()
        };
        let msaa_resolve_apple3x_if = if pc.msaa_desktop | pc.msaa_apple3 {
            Tfc::MULTISAMPLE_RESOLVE
        } else {
            Tfc::empty()
        };
        let is_not_apple1x = super::CapabilitiesQuery::supports_any(
            self.shared.device.as_ref(),
            &[
                MTLFeatureSet::iOS_GPUFamily2_v1,
                MTLFeatureSet::macOS_GPUFamily1_v1,
                MTLFeatureSet::tvOS_GPUFamily1_v1,
            ],
        );

        let image_atomic_if = if msl_version >= MTLLanguageVersion::Version3_1 {
            Tfc::STORAGE_ATOMIC
        } else {
            Tfc::empty()
        };

        let image_64_atomic_if = if pc.int64_atomics {
            Tfc::STORAGE_ATOMIC
        } else {
            Tfc::empty()
        };

        // Metal defined pixel format capabilities
        let all_caps = Tfc::SAMPLED_LINEAR
            | Tfc::STORAGE_WRITE_ONLY
            | Tfc::COLOR_ATTACHMENT
            | Tfc::COLOR_ATTACHMENT_BLEND
            | msaa_count
            | Tfc::MULTISAMPLE_RESOLVE;

        let extra = match format {
            Tf::R8Unorm | Tf::R16Float | Tf::Rgba8Unorm | Tf::Rgba16Float => {
                read_write_tier2_if | all_caps
            }
            Tf::R8Snorm | Tf::Rg8Snorm | Tf::Rgba8Snorm => {
                let mut flags = all_caps;
                flags.set(Tfc::MULTISAMPLE_RESOLVE, is_not_apple1x);
                flags
            }
            Tf::R8Uint
            | Tf::R8Sint
            | Tf::R16Uint
            | Tf::R16Sint
            | Tf::Rgba8Uint
            | Tf::Rgba8Sint
            | Tf::Rgba16Uint
            | Tf::Rgba16Sint => {
                read_write_tier2_if | Tfc::STORAGE_WRITE_ONLY | Tfc::COLOR_ATTACHMENT | msaa_count
            }
            Tf::R16Unorm
            | Tf::R16Snorm
            | Tf::Rg16Unorm
            | Tf::Rg16Snorm
            | Tf::Rgba16Unorm
            | Tf::Rgba16Snorm => {
                Tfc::SAMPLED_LINEAR
                    | Tfc::STORAGE_WRITE_ONLY
                    | Tfc::COLOR_ATTACHMENT
                    | Tfc::COLOR_ATTACHMENT_BLEND
                    | msaa_count
                    | msaa_resolve_desktop_if
            }
            Tf::Rg8Unorm | Tf::Rg16Float | Tf::Bgra8Unorm => all_caps,
            Tf::Rg8Uint | Tf::Rg8Sint => {
                Tfc::STORAGE_WRITE_ONLY | Tfc::COLOR_ATTACHMENT | msaa_count
            }
            Tf::R32Uint | Tf::R32Sint => {
                read_write_tier1_if
                    | Tfc::STORAGE_WRITE_ONLY
                    | Tfc::COLOR_ATTACHMENT
                    | msaa_count
                    | image_atomic_if
            }
            Tf::R32Float => {
                let flags = if pc.format_r32float_all {
                    all_caps
                } else {
                    Tfc::STORAGE_WRITE_ONLY
                        | Tfc::COLOR_ATTACHMENT
                        | Tfc::COLOR_ATTACHMENT_BLEND
                        | msaa_count
                };
                read_write_tier1_if | flags
            }
            Tf::Rg16Uint | Tf::Rg16Sint => {
                Tfc::STORAGE_WRITE_ONLY | Tfc::COLOR_ATTACHMENT | msaa_count
            }
            Tf::Rgba8UnormSrgb | Tf::Bgra8UnormSrgb => {
                let mut flags = all_caps;
                flags.set(Tfc::STORAGE_WRITE_ONLY, pc.format_rgba8_srgb_all);
                flags
            }
            Tf::Rgb10a2Uint => {
                let mut flags = Tfc::COLOR_ATTACHMENT | msaa_count;
                flags.set(Tfc::STORAGE_WRITE_ONLY, pc.format_rgb10a2_uint_write);
                flags
            }
            Tf::Rgb10a2Unorm => {
                let mut flags = all_caps;
                flags.set(Tfc::STORAGE_WRITE_ONLY, pc.format_rgb10a2_unorm_all);
                flags
            }
            Tf::Rg11b10Ufloat => {
                let mut flags = all_caps;
                flags.set(Tfc::STORAGE_WRITE_ONLY, pc.format_rg11b10_all);
                flags
            }
            Tf::R64Uint => {
                Tfc::COLOR_ATTACHMENT
                    | Tfc::STORAGE_WRITE_ONLY
                    | image_64_atomic_if
                    | read_write_tier1_if
            }
            Tf::Rg32Uint | Tf::Rg32Sint => {
                Tfc::COLOR_ATTACHMENT | Tfc::STORAGE_WRITE_ONLY | msaa_count
            }
            Tf::Rg32Float => {
                if pc.format_rg32float_all {
                    all_caps
                } else {
                    Tfc::STORAGE_WRITE_ONLY
                        | Tfc::COLOR_ATTACHMENT
                        | Tfc::COLOR_ATTACHMENT_BLEND
                        | msaa_count
                }
            }
            Tf::Rgba32Uint | Tf::Rgba32Sint => {
                read_write_tier2_if | Tfc::STORAGE_WRITE_ONLY | Tfc::COLOR_ATTACHMENT | msaa_count
            }
            Tf::Rgba32Float => {
                let mut flags =
                    read_write_tier2_if | Tfc::STORAGE_WRITE_ONLY | Tfc::COLOR_ATTACHMENT;
                if pc.format_rgba32float_all {
                    flags |= all_caps
                } else if pc.msaa_apple7 {
                    flags |= msaa_count
                };
                flags
            }
            Tf::Stencil8 => {
                all_caps | Tfc::DEPTH_STENCIL_ATTACHMENT | msaa_count | msaa_resolve_apple3x_if
            }
            Tf::Depth16Unorm => {
                if pc.format_depth16unorm {
                    let mut flags =
                        Tfc::DEPTH_STENCIL_ATTACHMENT | msaa_count | msaa_resolve_apple3x_if;
                    if pc.format_depth16unorm_filter {
                        flags |= Tfc::SAMPLED_LINEAR;
                    }
                    flags
                } else {
                    return Tfc::empty();
                }
            }
            Tf::Depth32Float | Tf::Depth32FloatStencil8 => {
                let mut flags =
                    Tfc::DEPTH_STENCIL_ATTACHMENT | msaa_count | msaa_resolve_apple3x_if;
                if pc.format_depth32float_filter {
                    flags |= Tfc::SAMPLED_LINEAR
                }
                flags
            }
            Tf::Depth24Plus | Tf::Depth24PlusStencil8 => {
                let mut flags = Tfc::DEPTH_STENCIL_ATTACHMENT | msaa_count;
                if pc.format_depth24_stencil8 {
                    flags |= Tfc::SAMPLED_LINEAR | Tfc::MULTISAMPLE_RESOLVE
                } else {
                    flags |= msaa_resolve_apple3x_if;
                    if pc.format_depth32float_filter {
                        flags |= Tfc::SAMPLED_LINEAR
                    }
                }
                flags
            }
            Tf::NV12 => return Tfc::empty(),
            Tf::P010 => return Tfc::empty(),
            Tf::Rgb9e5Ufloat => {
                if pc.msaa_apple3 {
                    all_caps
                } else if pc.msaa_desktop {
                    Tfc::SAMPLED_LINEAR
                } else {
                    Tfc::SAMPLED_LINEAR
                        | Tfc::COLOR_ATTACHMENT
                        | Tfc::COLOR_ATTACHMENT_BLEND
                        | msaa_count
                        | Tfc::MULTISAMPLE_RESOLVE
                }
            }
            Tf::Bc1RgbaUnorm
            | Tf::Bc1RgbaUnormSrgb
            | Tf::Bc2RgbaUnorm
            | Tf::Bc2RgbaUnormSrgb
            | Tf::Bc3RgbaUnorm
            | Tf::Bc3RgbaUnormSrgb
            | Tf::Bc4RUnorm
            | Tf::Bc4RSnorm
            | Tf::Bc5RgUnorm
            | Tf::Bc5RgSnorm
            | Tf::Bc6hRgbUfloat
            | Tf::Bc6hRgbFloat
            | Tf::Bc7RgbaUnorm
            | Tf::Bc7RgbaUnormSrgb => {
                if pc.format_bc {
                    Tfc::SAMPLED_LINEAR
                } else {
                    Tfc::empty()
                }
            }
            Tf::Etc2Rgb8Unorm
            | Tf::Etc2Rgb8UnormSrgb
            | Tf::Etc2Rgb8A1Unorm
            | Tf::Etc2Rgb8A1UnormSrgb
            | Tf::Etc2Rgba8Unorm
            | Tf::Etc2Rgba8UnormSrgb
            | Tf::EacR11Unorm
            | Tf::EacR11Snorm
            | Tf::EacRg11Unorm
            | Tf::EacRg11Snorm => {
                if pc.format_eac_etc {
                    Tfc::SAMPLED_LINEAR
                } else {
                    Tfc::empty()
                }
            }
            Tf::Astc {
                block: _,
                channel: _,
            } => {
                if pc.format_astc || pc.format_astc_hdr {
                    Tfc::SAMPLED_LINEAR
                } else {
                    Tfc::empty()
                }
            }
        };

        Tfc::COPY_SRC | Tfc::COPY_DST | Tfc::SAMPLED | Tfc::STORAGE_READ_ONLY | extra
    }

    unsafe fn surface_capabilities(
        &self,
        surface: &super::Surface,
    ) -> Option<crate::SurfaceCapabilities> {
        let mut formats = vec![
            wgt::TextureFormat::Bgra8Unorm,
            wgt::TextureFormat::Bgra8UnormSrgb,
            wgt::TextureFormat::Rgba16Float,
        ];
        if self
            .shared
            .private_texture_format_caps
            .format_rgb10a2_unorm_all
        {
            formats.push(wgt::TextureFormat::Rgb10a2Unorm);
        }

        Some(crate::SurfaceCapabilities {
            formats,
            // We use this here to govern the maximum number of drawables + 1.
            // See https://developer.apple.com/documentation/quartzcore/cametallayer/maximumdrawablecount
            maximum_frame_latency: if available!(
                macos = 10.13.2,
                ios = 11.2,
                tvos = 11.2,
                visionos = 1.0
            ) {
                1..=2
            } else {
                // 3 is the default value for maximum drawables in `CAMetalLayer` documentation
                // iOS 10.3 was tested to use 3 on iphone5s
                2..=2
            },
            // We enable Immediate mode using `-[CAMetalLayer setDisplaySyncEnabled: false]`.
            present_modes: if OsFeatures::display_sync() {
                vec![wgt::PresentMode::Fifo, wgt::PresentMode::Immediate]
            } else {
                vec![wgt::PresentMode::Fifo]
            },
            composite_alpha_modes: vec![
                wgt::CompositeAlphaMode::Opaque,
                wgt::CompositeAlphaMode::PostMultiplied,
            ],

            current_extent: Some(surface.dimensions()),
            usage: wgt::TextureUses::COLOR_TARGET
                | wgt::TextureUses::COPY_SRC
                | wgt::TextureUses::COPY_DST
                | wgt::TextureUses::STORAGE_READ_ONLY
                | wgt::TextureUses::STORAGE_WRITE_ONLY
                | wgt::TextureUses::STORAGE_READ_WRITE,
        })
    }

    unsafe fn get_presentation_timestamp(&self) -> wgt::PresentationTimestamp {
        let timestamp = self.shared.presentation_timer.get_timestamp_ns();

        wgt::PresentationTimestamp(timestamp)
    }

    fn get_ordered_buffer_usages(&self) -> wgt::BufferUses {
        wgt::BufferUses::INCLUSIVE | wgt::BufferUses::MAP_WRITE
    }

    // Don't put barriers between inclusive uses
    fn get_ordered_texture_usages(&self) -> wgt::TextureUses {
        wgt::TextureUses::INCLUSIVE
            | wgt::TextureUses::COLOR_TARGET
            | wgt::TextureUses::DEPTH_STENCIL_WRITE
    }
}

const RESOURCE_HEAP_SUPPORT: &[MTLFeatureSet] = &[
    MTLFeatureSet::iOS_GPUFamily1_v3,
    MTLFeatureSet::tvOS_GPUFamily1_v2,
    MTLFeatureSet::macOS_GPUFamily1_v3,
];

const MUTABLE_COMPARISON_SAMPLER_SUPPORT: &[MTLFeatureSet] = &[
    MTLFeatureSet::iOS_GPUFamily3_v1,
    MTLFeatureSet::macOS_GPUFamily1_v1,
];

const SAMPLER_CLAMP_TO_BORDER_SUPPORT: &[MTLFeatureSet] = &[MTLFeatureSet::macOS_GPUFamily1_v2];

const ASTC_PIXEL_FORMAT_FEATURES: &[MTLFeatureSet] = &[
    MTLFeatureSet::iOS_GPUFamily2_v1,
    MTLFeatureSet::tvOS_GPUFamily1_v1,
];

const ANY8_UNORM_SRGB_ALL: &[MTLFeatureSet] = &[
    MTLFeatureSet::iOS_GPUFamily2_v3,
    MTLFeatureSet::tvOS_GPUFamily1_v2,
];

const ANY8_SNORM_RESOLVE: &[MTLFeatureSet] = &[
    MTLFeatureSet::iOS_GPUFamily2_v1,
    MTLFeatureSet::tvOS_GPUFamily1_v1,
    MTLFeatureSet::macOS_GPUFamily1_v1,
];

const RGBA8_SRGB: &[MTLFeatureSet] = &[
    MTLFeatureSet::iOS_GPUFamily2_v3,
    MTLFeatureSet::tvOS_GPUFamily1_v2,
];

const RGB10A2UNORM_ALL: &[MTLFeatureSet] = &[
    MTLFeatureSet::iOS_GPUFamily3_v1,
    MTLFeatureSet::tvOS_GPUFamily2_v1,
    MTLFeatureSet::macOS_GPUFamily1_v1,
];

const RGB10A2UINT_WRITE: &[MTLFeatureSet] = &[
    MTLFeatureSet::iOS_GPUFamily3_v1,
    MTLFeatureSet::tvOS_GPUFamily2_v1,
    MTLFeatureSet::macOS_GPUFamily1_v1,
];

const RG11B10FLOAT_ALL: &[MTLFeatureSet] = &[
    MTLFeatureSet::iOS_GPUFamily3_v1,
    MTLFeatureSet::tvOS_GPUFamily2_v1,
    MTLFeatureSet::macOS_GPUFamily1_v1,
];

const RGB9E5FLOAT_ALL: &[MTLFeatureSet] = &[
    MTLFeatureSet::iOS_GPUFamily3_v1,
    MTLFeatureSet::tvOS_GPUFamily2_v1,
];

const BGR10A2_ALL: &[MTLFeatureSet] = &[
    MTLFeatureSet::iOS_GPUFamily1_v4,
    MTLFeatureSet::tvOS_GPUFamily1_v3,
    MTLFeatureSet::macOS_GPUFamily2_v1,
];

/// "Indirect draw & dispatch arguments" in the Metal feature set tables
const INDIRECT_DRAW_DISPATCH_SUPPORT: &[MTLFeatureSet] = &[
    MTLFeatureSet::iOS_GPUFamily3_v1,
    MTLFeatureSet::tvOS_GPUFamily2_v1,
    MTLFeatureSet::macOS_GPUFamily1_v1,
];

/// "Base vertex/instance drawing" in the Metal feature set tables
///
/// in our terms, `base_vertex` and `first_instance` must be 0
const BASE_VERTEX_FIRST_INSTANCE_SUPPORT: &[MTLFeatureSet] = INDIRECT_DRAW_DISPATCH_SUPPORT;

const TEXTURE_CUBE_ARRAY_SUPPORT: &[MTLFeatureSet] = &[
    MTLFeatureSet::iOS_GPUFamily4_v1,
    MTLFeatureSet::tvOS_GPUFamily1_v2,
    MTLFeatureSet::macOS_GPUFamily1_v1,
];

const DUAL_SOURCE_BLEND_SUPPORT: &[MTLFeatureSet] = &[
    MTLFeatureSet::iOS_GPUFamily1_v4,
    MTLFeatureSet::tvOS_GPUFamily1_v3,
    MTLFeatureSet::macOS_GPUFamily1_v2,
];

const LAYERED_RENDERING_SUPPORT: &[MTLFeatureSet] = &[
    MTLFeatureSet::iOS_GPUFamily5_v1,
    MTLFeatureSet::macOS_GPUFamily1_v1,
    MTLFeatureSet::macOS_GPUFamily2_v1,
];

const FUNCTION_SPECIALIZATION_SUPPORT: &[MTLFeatureSet] = &[
    MTLFeatureSet::iOS_GPUFamily1_v3,
    MTLFeatureSet::tvOS_GPUFamily1_v2,
    MTLFeatureSet::macOS_GPUFamily1_v2,
];

const DEPTH_CLIP_MODE: &[MTLFeatureSet] = &[
    MTLFeatureSet::iOS_GPUFamily4_v1,
    MTLFeatureSet::tvOS_GPUFamily1_v3,
    MTLFeatureSet::macOS_GPUFamily1_v1,
];

impl super::CapabilitiesQuery {
    fn supports_any(raw: &ProtocolObject<dyn MTLDevice>, features_sets: &[MTLFeatureSet]) -> bool {
        features_sets
            .iter()
            .cloned()
            .any(|x| raw.supportsFeatureSet(x))
    }

    /// Query the capabilities of the device.
    pub fn new(device: &ProtocolObject<dyn MTLDevice>) -> Self {
        // There are four different OSes we can target: macOS, iOS, tvOS and
        // visionOS. This can be detected using `cfg!(target_os = "ios")`, or
        // more conveniently using the `available!(...)` macro, which also
        // checks that the OS version that the binary is currently running on
        // is higher than or equal to the specified version.
        //
        // Along with the different OSes, there is also two other modes that
        // applications can run in: the Simulator, and Mac Catalyst. This can
        // be detected using `cfg!(target_env = "sim")` or
        // `cfg!(target_env = "macabi")`.
        //
        // Finally, iOS applications can be run on macOS and visionOS directly
        // using the "Designed for iPad" mode. This cannot be detected at
        // compile-time.
        //
        // All of this means that it only makes sense to use `cfg!(...)` and
        // `available!(...)` in here to check which Metal APIs are available;
        // we cannot rely on it for knowing properties of the device. For
        // that, we'll want to use `supportsFeatureSet` or `supportsFamily`.
        //
        // See the following link for further details:
        // https://developer.apple.com/documentation/metal/developing-metal-apps-that-run-in-simulator

        let version = NSProcessInfo::processInfo().operatingSystemVersion();
        let os_type = super::OsType::new(version, device);

        let family_check = available!(macos = 10.15, ios = 13.0, tvos = 13.0, visionos = 1.0);
        let metal3 = family_check && device.supportsFamily(MTLGPUFamily::Metal3);
        let metal4 = family_check && device.supportsFamily(MTLGPUFamily::Metal4);
        let mut sample_count_mask = crate::TextureFormatCapabilities::MULTISAMPLE_X4; // 1 and 4 samples are supported on all devices
        if device.supportsTextureSampleCount(2) {
            sample_count_mask |= crate::TextureFormatCapabilities::MULTISAMPLE_X2;
        }
        if device.supportsTextureSampleCount(8) {
            sample_count_mask |= crate::TextureFormatCapabilities::MULTISAMPLE_X8;
        }
        if device.supportsTextureSampleCount(16) {
            sample_count_mask |= crate::TextureFormatCapabilities::MULTISAMPLE_X16;
        }

        let rw_texture_tier = if available!(macos = 10.13, ios = 11.0, tvos = 11.0, visionos = 1.0)
        {
            device.readWriteTextureSupport()
        } else if available!(macos = 10.12) {
            if Self::supports_any(device, &[MTLFeatureSet::macOS_ReadWriteTextureTier2]) {
                MTLReadWriteTextureTier::Tier2
            } else {
                MTLReadWriteTextureTier::Tier1
            }
        } else {
            MTLReadWriteTextureTier::TierNone
        };

        let mut timestamp_query_support = TimestampQuerySupport::empty();
        if available!(macos = 11.0, ios = 14.0, tvos = 14.0, visionos = 1.0)
            && device.supportsCounterSampling(MTLCounterSamplingPoint::AtStageBoundary)
        {
            // If we don't support at stage boundary, don't support anything else.
            timestamp_query_support.insert(TimestampQuerySupport::STAGE_BOUNDARIES);

            if device.supportsCounterSampling(MTLCounterSamplingPoint::AtDrawBoundary) {
                timestamp_query_support.insert(TimestampQuerySupport::ON_RENDER_ENCODER);
            }
            if device.supportsCounterSampling(MTLCounterSamplingPoint::AtDispatchBoundary) {
                timestamp_query_support.insert(TimestampQuerySupport::ON_COMPUTE_ENCODER);
            }
            if device.supportsCounterSampling(MTLCounterSamplingPoint::AtBlitBoundary) {
                timestamp_query_support.insert(TimestampQuerySupport::ON_BLIT_ENCODER);
            }
            // `TimestampQuerySupport::INSIDE_WGPU_PASSES` emerges from the other flags.
        }

        let argument_buffers = available!(macos = 10.13, ios = 11.0, tvos = 11.0, visionos = 1.0)
            .then(|| device.argumentBuffersSupport());

        let is_virtual = device.name().to_string().to_lowercase().contains("virtual");

        let mesh_shaders = family_check
                && (device.supportsFamily(MTLGPUFamily::Metal3)
                    || device.supportsFamily(MTLGPUFamily::Apple7)
                    || device.supportsFamily(MTLGPUFamily::Mac2))
                    // Mesh shaders don't work on virtual devices even if they should be supported. CI thing
                && !is_virtual;

        let msl_version = if available!(macos = 15.0, ios = 18.0, tvos = 18.0, visionos = 2.0) {
            MTLLanguageVersion::Version3_2
        } else if available!(macos = 14.0, ios = 17.0, tvos = 17.0, visionos = 1.0) {
            MTLLanguageVersion::Version3_1
        } else if available!(macos = 13.0, ios = 16.0, tvos = 16.0, visionos = 1.0) {
            MTLLanguageVersion::Version3_0
        } else if available!(macos = 12.0, ios = 15.0, tvos = 15.0, visionos = 1.0) {
            MTLLanguageVersion::Version2_4
        } else if available!(macos = 11.0, ios = 14.0, tvos = 14.0, visionos = 1.0) {
            MTLLanguageVersion::Version2_3
        } else if available!(macos = 10.15, ios = 13.0, tvos = 13.0, visionos = 1.0) {
            MTLLanguageVersion::Version2_2
        } else if available!(macos = 10.14, ios = 12.0, tvos = 12.0, visionos = 1.0) {
            MTLLanguageVersion::Version2_1
        } else if available!(macos = 10.13, ios = 11.0, tvos = 11.0, visionos = 1.0) {
            MTLLanguageVersion::Version2_0
        } else if available!(macos = 10.12, ios = 10.0, tvos = 10.0, visionos = 1.0) {
            MTLLanguageVersion::Version1_2
        } else if available!(macos = 10.11, ios = 9.0, tvos = 9.0, visionos = 1.0) {
            MTLLanguageVersion::Version1_1
        } else {
            MTLLanguageVersion::Version1_0
        };

        Self {
            msl_version,
            // macOS 10.11 doesn't support read-write resources
            fragment_rw_storage: available!(macos = 10.12, ios = 8.0, tvos = 8.0, visionos = 1.0),
            read_write_texture_tier: rw_texture_tier,
            msaa_desktop: os_type == super::OsType::Macos,
            msaa_apple3: (family_check && device.supportsFamily(MTLGPUFamily::Apple3))
                || device.supportsFeatureSet(MTLFeatureSet::iOS_GPUFamily3_v4),
            msaa_apple7: family_check && device.supportsFamily(MTLGPUFamily::Apple7),
            resource_heaps: Self::supports_any(device, RESOURCE_HEAP_SUPPORT),
            argument_buffers,
            mutable_comparison_samplers: Self::supports_any(
                device,
                MUTABLE_COMPARISON_SAMPLER_SUPPORT,
            ),
            sampler_clamp_to_border: Self::supports_any(device, SAMPLER_CLAMP_TO_BORDER_SUPPORT),
            indirect_draw_dispatch: Self::supports_any(device, INDIRECT_DRAW_DISPATCH_SUPPORT),
            base_vertex_first_instance_drawing: Self::supports_any(
                device,
                BASE_VERTEX_FIRST_INSTANCE_SUPPORT,
            ),
            dual_source_blending: Self::supports_any(device, DUAL_SOURCE_BLEND_SUPPORT),
            low_power: os_type != super::OsType::Macos || device.isLowPower(),
            headless: os_type == super::OsType::Macos && device.isHeadless(),
            layered_rendering: Self::supports_any(device, LAYERED_RENDERING_SUPPORT),
            function_specialization: Self::supports_any(device, FUNCTION_SPECIALIZATION_SUPPORT),
            depth_clip_mode: Self::supports_any(device, DEPTH_CLIP_MODE),
            texture_cube_array: Self::supports_any(device, TEXTURE_CUBE_ARRAY_SUPPORT),
            supports_float_filtering: os_type == super::OsType::Macos
                || (available!(macos = 11.0, ios = 14.0, tvos = 16.0, visionos = 1.0)
                    && device_class_responds_to(device, sel!(supports32BitFloatFiltering))
                    && device.supports32BitFloatFiltering()),
            format_depth24_stencil8: os_type == super::OsType::Macos
                && device.isDepth24Stencil8PixelFormatSupported(),
            format_depth32_stencil8_filter: os_type == super::OsType::Macos,
            format_depth32_stencil8_none: os_type != super::OsType::Macos,
            format_min_srgb_channels: if os_type == super::OsType::Macos {
                4
            } else {
                1
            },
            format_b5: os_type != super::OsType::Macos,
            format_bc: os_type == super::OsType::Macos,
            format_eac_etc: os_type != super::OsType::Macos
                // M1 in macOS supports EAC/ETC2
                || (family_check && device.supportsFamily(MTLGPUFamily::Apple7)),
            // A8(Apple2) and later always support ASTC pixel formats
            format_astc: (family_check && device.supportsFamily(MTLGPUFamily::Apple2))
                || Self::supports_any(device, ASTC_PIXEL_FORMAT_FEATURES),
            // A13(Apple6) M1(Apple7) and later always support HDR ASTC pixel formats
            format_astc_hdr: family_check && device.supportsFamily(MTLGPUFamily::Apple6),
            // Apple3 and later supports compressed volume texture formats including ASTC Sliced 3D
            format_astc_3d: family_check && device.supportsFamily(MTLGPUFamily::Apple3),
            format_any8_unorm_srgb_all: Self::supports_any(device, ANY8_UNORM_SRGB_ALL),
            format_any8_unorm_srgb_no_write: !Self::supports_any(device, ANY8_UNORM_SRGB_ALL)
                && os_type != super::OsType::Macos,
            format_any8_snorm_all: Self::supports_any(device, ANY8_SNORM_RESOLVE),
            format_r16_norm_all: os_type == super::OsType::Macos,
            // No devices support r32's all capabilities
            format_r32_all: false,
            // All devices support r32's write capability
            format_r32_no_write: false,
            // iOS support r32float's write capability, macOS support r32float's all capabilities
            format_r32float_no_write_no_filter: false,
            // Only iOS doesn't support r32float's filter  capability
            format_r32float_no_filter: os_type != super::OsType::Macos,
            format_r32float_all: os_type == super::OsType::Macos,
            format_rgba8_srgb_all: Self::supports_any(device, RGBA8_SRGB),
            format_rgba8_srgb_no_write: !Self::supports_any(device, RGBA8_SRGB),
            format_rgb10a2_unorm_all: Self::supports_any(device, RGB10A2UNORM_ALL),
            format_rgb10a2_unorm_no_write: !Self::supports_any(device, RGB10A2UNORM_ALL),
            format_rgb10a2_uint_write: Self::supports_any(device, RGB10A2UINT_WRITE),
            format_rg11b10_all: Self::supports_any(device, RG11B10FLOAT_ALL),
            format_rg11b10_no_write: !Self::supports_any(device, RG11B10FLOAT_ALL),
            format_rgb9e5_all: Self::supports_any(device, RGB9E5FLOAT_ALL),
            format_rgb9e5_no_write: !Self::supports_any(device, RGB9E5FLOAT_ALL)
                && os_type != super::OsType::Macos,
            format_rgb9e5_filter_only: os_type == super::OsType::Macos,
            format_rg32_color: true,
            format_rg32_color_write: true,
            // Only macOS support rg32float's all capabilities
            format_rg32float_all: os_type == super::OsType::Macos,
            // All devices support rg32float's color + blend capabilities
            format_rg32float_color_blend: true,
            // Only iOS doesn't support rg32float's filter
            format_rg32float_no_filter: os_type != super::OsType::Macos,
            format_rgba32int_color: true,
            // All devices support rgba32uint and rgba32sint's color + write capabilities
            format_rgba32int_color_write: true,
            format_rgba32float_color: true,
            // All devices support rgba32float's color + write capabilities
            format_rgba32float_color_write: true,
            // Only macOS support rgba32float's all capabilities
            format_rgba32float_all: os_type == super::OsType::Macos,
            // https://developer.apple.com/documentation/metal/mtlpixelformat/depth16unorm
            format_depth16unorm: available!(macos = 10.12, ios = 13.0, tvos = 13.0, visionos = 1.0),
            // https://developer.apple.com/metal/Metal-Feature-Set-Tables.pdf#page=12
            format_depth16unorm_filter: family_check
                && (metal3
                    || device.supportsFamily(MTLGPUFamily::Apple3)
                    || device.supportsFamily(MTLGPUFamily::Mac2)),
            format_depth32float_filter: os_type == super::OsType::Macos,
            format_depth32float_none: os_type != super::OsType::Macos,
            format_bgr10a2_all: Self::supports_any(device, BGR10A2_ALL),
            format_bgr10a2_no_write: !Self::supports_any(device, BGR10A2_ALL),
            max_buffers_per_stage: 31,
            max_vertex_buffers: 31.min(crate::MAX_VERTEX_BUFFERS as u32), // duplicative of `apply_hal_limits`
            max_textures_per_stage: if os_type == super::OsType::Macos
                || (family_check && device.supportsFamily(MTLGPUFamily::Apple6))
            {
                128
            } else if family_check && device.supportsFamily(MTLGPUFamily::Apple4) {
                96
            } else {
                31
            },
            max_samplers_per_stage: 16,
            max_binding_array_elements: if argument_buffers == Some(MTLArgumentBuffersTier::Tier2) {
                1_000_000
            } else if family_check && device.supportsFamily(MTLGPUFamily::Apple4) {
                96
            } else {
                31
            },
            // https://developer.apple.com/metal/Metal-Feature-Set-Tables.pdf#page=7
            max_sampler_binding_array_elements: if family_check
                && device.supportsFamily(MTLGPUFamily::Apple9)
            {
                500_000
            } else if family_check
                && (device.supportsFamily(MTLGPUFamily::Apple7)
                    || device.supportsFamily(MTLGPUFamily::Mac2))
            {
                1024
            } else if family_check && device.supportsFamily(MTLGPUFamily::Apple6) {
                128
            } else {
                16
            },
            buffer_alignment: if matches!(os_type, super::OsType::Macos | super::OsType::VisionOs) {
                256
            } else {
                64
            },
            max_buffer_size: if available!(macos = 10.14, ios = 12.0, tvos = 12.0, visionos = 1.0) {
                device.maxBufferLength() as u64
            } else if os_type == super::OsType::Macos {
                1 << 30 // 1GB on macOS 10.11 and up
            } else {
                1 << 28 // 256MB on iOS 8.0+
            },
            max_texture_size: if Self::supports_any(
                device,
                &[
                    MTLFeatureSet::iOS_GPUFamily3_v1,
                    MTLFeatureSet::tvOS_GPUFamily2_v1,
                    MTLFeatureSet::macOS_GPUFamily1_v1,
                ],
            ) {
                16384
            } else {
                8192
            },
            max_texture_3d_size: 2048,
            max_texture_layers: 2048,
            max_fragment_input_components: if os_type == super::OsType::Macos
                || device.supportsFeatureSet(MTLFeatureSet::iOS_GPUFamily4_v1)
            {
                124
            } else {
                60
            },
            // https://developer.apple.com/metal/Metal-Feature-Set-Tables.pdf#page=7
            // 8 is supported on everything on that list
            max_color_render_targets: if Self::supports_any(
                device,
                &[
                    MTLFeatureSet::iOS_GPUFamily2_v1,
                    MTLFeatureSet::tvOS_GPUFamily1_v1,
                    MTLFeatureSet::macOS_GPUFamily1_v1,
                ],
            ) {
                8
            } else {
                4
            },
            // https://developer.apple.com/metal/Metal-Feature-Set-Tables.pdf#page=7
            max_color_attachment_bytes_per_sample: if family_check
                && device.supportsFamily(MTLGPUFamily::Apple7)
            {
                128
            } else if family_check && device.supportsFamily(MTLGPUFamily::Apple4) {
                64
            } else {
                32
            },
            max_varying_components: if device.supportsFeatureSet(MTLFeatureSet::macOS_GPUFamily1_v1)
            {
                124
            } else {
                60
            },
            // https://developer.apple.com/metal/Metal-Feature-Set-Tables.pdf#page=6
            // These are older checks but still hold true; no entry in this table supports
            // more than 1024 threads.
            max_threads_per_group: if Self::supports_any(
                device,
                &[
                    MTLFeatureSet::iOS_GPUFamily4_v2,
                    MTLFeatureSet::macOS_GPUFamily1_v1,
                ],
            ) {
                1024
            } else {
                512
            },
            // https://developer.apple.com/metal/Metal-Feature-Set-Tables.pdf#page=6
            // These are older checks but still hold true; no entry in this table supports
            // more than 32kb.
            max_total_threadgroup_memory: if Self::supports_any(
                device,
                &[
                    MTLFeatureSet::iOS_GPUFamily4_v1,
                    MTLFeatureSet::macOS_GPUFamily1_v2,
                ],
            ) {
                32 << 10
            } else {
                16 << 10
            },
            sample_count_mask,
            supports_debug_markers: Self::supports_any(
                device,
                &[
                    MTLFeatureSet::macOS_GPUFamily1_v2,
                    MTLFeatureSet::iOS_GPUFamily1_v3,
                    MTLFeatureSet::tvOS_GPUFamily1_v2,
                ],
            ),
            // https://developer.apple.com/metal/Metal-Feature-Set-Tables.pdf#page=4
            supports_binary_archives: family_check
                && (metal3
                    || device.supportsFamily(MTLGPUFamily::Apple3)
                    || device.supportsFamily(MTLGPUFamily::Mac2)),
            // This is just trusted blindly since docs referencing supports_any have been removed
            // but we don't want to remove feature support.
            supports_arrays_of_textures: Self::supports_any(
                device,
                &[
                    MTLFeatureSet::iOS_GPUFamily3_v2,
                    MTLFeatureSet::tvOS_GPUFamily2_v1,
                    MTLFeatureSet::macOS_GPUFamily1_v3,
                ],
            ),
            // https://developer.apple.com/metal/Metal-Feature-Set-Tables.pdf#page=3
            supports_arrays_of_textures_write: family_check
                && (metal3
                    || device.supportsFamily(MTLGPUFamily::Apple6)
                    || device.supportsFamily(MTLGPUFamily::Mac2)),
            // Depth clipping is supported on all macOS GPU families and iOS family 4 and later
            supports_depth_clip_control: os_type == super::OsType::Macos
                || device.supportsFeatureSet(MTLFeatureSet::iOS_GPUFamily4_v1),
            // https://developer.apple.com/metal/Metal-Feature-Set-Tables.pdf#page=4
            supports_shader_primitive_index: family_check
                && (metal3
                    || device.supportsFamily(MTLGPUFamily::Apple7)
                    || device.supportsFamily(MTLGPUFamily::Mac2)),
            // https://developer.apple.com/documentation/metal/mtldevice/hasunifiedmemory
            has_unified_memory: if available!(
                macos = 10.15,
                ios = 13.0,
                tvos = 13.0,
                visionos = 1.0
            ) && device_class_responds_to(device, sel!(hasUnifiedMemory))
            {
                Some(device.hasUnifiedMemory())
            } else {
                None
            },
            timestamp_query_support,
            // https://developer.apple.com/metal/Metal-Feature-Set-Tables.pdf#page=4
            supports_simd_scoped_operations: family_check
                && (metal3
                    || device.supportsFamily(MTLGPUFamily::Mac2)
                    || device.supportsFamily(MTLGPUFamily::Apple7)),
            // https://developer.apple.com/metal/Metal-Feature-Set-Tables.pdf#page=4
            int64: family_check && (metal3 || device.supportsFamily(MTLGPUFamily::Apple3)),
            // https://developer.apple.com/metal/Metal-Feature-Set-Tables.pdf#page=4
            // There is also a footnote that says
            // "Some GPU devices in the Apple8 family support 64-bit atomic minimum and maximum..."
            int64_atomics_min_max: family_check
                && (device.supportsFamily(MTLGPUFamily::Apple9)
                    || (device.supportsFamily(MTLGPUFamily::Apple8)
                        && device.supportsFamily(MTLGPUFamily::Mac2))),
            int64_atomics: family_check && device.supportsFamily(MTLGPUFamily::Apple9),
            // https://developer.apple.com/metal/Metal-Feature-Set-Tables.pdf#page=4
            float_atomics: family_check
                && (metal3
                    || device.supportsFamily(MTLGPUFamily::Apple7)
                    || device.supportsFamily(MTLGPUFamily::Mac2)),
            // https://developer.apple.com/metal/Metal-Feature-Set-Tables.pdf#page=5 (footnote)
            // Supported on some Metal4, Apple7, Mac2, and some other platforms can be queried with device.supportsShaderBarycentricCoordinates().
            shader_barycentrics: metal4
                || (family_check
                    && (device.supportsFamily(MTLGPUFamily::Apple7)
                        || device.supportsFamily(MTLGPUFamily::Mac2)))
                || (available!(macos = 10.15, ios = 14.0, tvos = 16.0, visionos = 1.0)
                    && device.supportsShaderBarycentricCoordinates()),
            // https://developer.apple.com/metal/Metal-Feature-Set-Tables.pdf#page=3
            // See https://github.com/gfx-rs/wgpu/pull/8725 for more details
            supports_memoryless_storage: metal4
                || if family_check {
                    // Apple A7 (MTLGPUFamily::Apple1) has been tested to have support.
                    device.supportsFamily(MTLGPUFamily::Apple1)
                } else {
                    // macOS: Always rely on family check
                    // iOS/tvOS: API added in 10.0
                    // visionOS: Always rely on family check
                    available!(ios = 10.0, tvos = 10.0)
                },
            supported_vertex_amplification_factor: {
                let mut factor = 1;
                // https://developer.apple.com/documentation/metal/mtldevice/supportsvertexamplificationcount(_:)
                if available!(macos = 10.15.4, ios = 13.0, tvos = 16.0, visionos = 1.0) {
                    // https://developer.apple.com/metal/Metal-Feature-Set-Tables.pdf#page=8
                    // The table specifies either none, 2, 8, or unsupported, implying it is a relatively small power of 2
                    // The bitmask only uses 32 bits, so it can't be higher even if the device for some reason claims to support that.
                    while factor < 32 && device.supportsVertexAmplificationCount(factor * 2) {
                        factor *= 2
                    }
                }
                factor as u32
            },
            // https://developer.apple.com/metal/Metal-Feature-Set-Tables.pdf#page=4
            mesh_shaders,
            max_mesh_task_workgroup_count: if mesh_shaders { 1024 } else { 0 },
            max_task_payload_size: if mesh_shaders { 16384 - 32 } else { 0 },
            supports_cooperative_matrix: family_check
                && (device.supportsFamily(MTLGPUFamily::Apple7)
                    || device.supportsFamily(MTLGPUFamily::Mac2)),
            // https://developer.apple.com/documentation/metal/mtlresidencyset
            supports_raytracing: if available!(
                macos = 15.0,
                ios = 18.0,
                tvos = 18.0,
                visionos = 2.0,
            ) {
                device_class_responds_to(device, sel!(supportsRaytracing))
                    && device.supportsRaytracing()
                    && device_class_responds_to(device, sel!(supportsRaytracingFromRender))
                    && device.supportsRaytracingFromRender()
            } else {
                false
            },
        }
    }

    pub fn features(&self) -> wgt::Features {
        use wgt::Features as F;

        let mut features = F::empty()
            | F::MAPPABLE_PRIMARY_BUFFERS
            | F::VERTEX_WRITABLE_STORAGE
            | F::TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES
            | F::IMMEDIATES
            | F::POLYGON_MODE_LINE
            | F::CLEAR_TEXTURE
            | F::TEXTURE_FORMAT_16BIT_NORM
            | F::SHADER_F16
            | F::DEPTH32FLOAT_STENCIL8
            | F::BGRA8UNORM_STORAGE
            | F::PASSTHROUGH_SHADERS
            | F::EXTERNAL_TEXTURE;

        features.set(F::FLOAT32_FILTERABLE, self.supports_float_filtering);
        features.set(F::FLOAT32_BLENDABLE, true);
        features.set(F::INDIRECT_FIRST_INSTANCE, self.indirect_draw_dispatch);
        features.set(
            F::TIMESTAMP_QUERY | F::TIMESTAMP_QUERY_INSIDE_ENCODERS,
            self.timestamp_query_support
                .contains(TimestampQuerySupport::STAGE_BOUNDARIES),
        );
        features.set(
            F::TIMESTAMP_QUERY_INSIDE_PASSES,
            self.timestamp_query_support
                .contains(TimestampQuerySupport::INSIDE_WGPU_PASSES),
        );
        features.set(
            F::DUAL_SOURCE_BLENDING,
            self.msl_version >= MTLLanguageVersion::Version1_2 && self.dual_source_blending,
        );
        features.set(F::TEXTURE_COMPRESSION_ASTC, self.format_astc);
        features.set(F::TEXTURE_COMPRESSION_ASTC_HDR, self.format_astc_hdr);
        features.set(F::TEXTURE_COMPRESSION_ASTC_SLICED_3D, self.format_astc_3d);
        features.set(F::TEXTURE_COMPRESSION_BC, self.format_bc);
        features.set(F::TEXTURE_COMPRESSION_BC_SLICED_3D, self.format_bc); // BC guarantees Sliced 3D
        features.set(F::TEXTURE_COMPRESSION_ETC2, self.format_eac_etc);

        features.set(F::DEPTH_CLIP_CONTROL, self.supports_depth_clip_control);
        features.set(F::PRIMITIVE_INDEX, self.supports_shader_primitive_index);

        features.set(
            F::TEXTURE_BINDING_ARRAY
                | F::SAMPLED_TEXTURE_AND_STORAGE_BUFFER_ARRAY_NON_UNIFORM_INDEXING
                | F::STORAGE_TEXTURE_ARRAY_NON_UNIFORM_INDEXING
                | F::PARTIALLY_BOUND_BINDING_ARRAY,
            self.msl_version >= MTLLanguageVersion::Version3_0
                && self.supports_arrays_of_textures
                && self
                    .argument_buffers
                    .unwrap_or(MTLArgumentBuffersTier::Tier1)
                    >= MTLArgumentBuffersTier::Tier2,
        );
        features.set(
            F::STORAGE_RESOURCE_BINDING_ARRAY,
            self.msl_version >= MTLLanguageVersion::Version3_0
                && self.supports_arrays_of_textures
                && self.supports_arrays_of_textures_write
                && self
                    .argument_buffers
                    .unwrap_or(MTLArgumentBuffersTier::Tier1)
                    >= MTLArgumentBuffersTier::Tier2,
        );
        features.set(
            F::SHADER_INT64,
            self.int64 && self.msl_version >= MTLLanguageVersion::Version2_3,
        );
        features.set(
            F::SHADER_INT64_ATOMIC_MIN_MAX,
            self.int64_atomics_min_max && self.msl_version >= MTLLanguageVersion::Version2_4,
        );
        features.set(
            F::TEXTURE_INT64_ATOMIC,
            self.int64_atomics && self.msl_version >= MTLLanguageVersion::Version3_1,
        );
        features.set(
            F::TEXTURE_ATOMIC,
            self.msl_version >= MTLLanguageVersion::Version3_1,
        );
        features.set(
            F::SHADER_FLOAT32_ATOMIC,
            self.float_atomics && self.msl_version >= MTLLanguageVersion::Version3_0,
        );
        features.set(
            F::MEMORY_DECORATION_COHERENT,
            self.msl_version >= MTLLanguageVersion::Version3_2,
        );

        features.set(
            F::ADDRESS_MODE_CLAMP_TO_BORDER,
            self.sampler_clamp_to_border,
        );
        features.set(F::ADDRESS_MODE_CLAMP_TO_ZERO, true);

        features.set(F::RG11B10UFLOAT_RENDERABLE, self.format_rg11b10_all);

        features.set(
            F::SHADER_BARYCENTRICS,
            self.shader_barycentrics && self.msl_version >= MTLLanguageVersion::Version2_2,
        );

        if self.supports_simd_scoped_operations {
            features.insert(F::SUBGROUP | F::SUBGROUP_BARRIER);
        }

        features.set(F::EXPERIMENTAL_MESH_SHADER, self.mesh_shaders);

        // Cooperative matrix (simdgroup matrix) requires MSL 2.3+
        features.set(
            F::EXPERIMENTAL_COOPERATIVE_MATRIX,
            self.supports_cooperative_matrix && self.msl_version >= MTLLanguageVersion::Version2_3,
        );

        if self.supported_vertex_amplification_factor > 1 {
            features.insert(F::MULTIVIEW);
        }

        features.set(F::EXPERIMENTAL_RAY_QUERY, self.supports_raytracing);

        features
    }

    pub fn capabilities(&self) -> crate::Capabilities {
        let mut downlevel = wgt::DownlevelCapabilities::default();
        downlevel.flags.set(
            wgt::DownlevelFlags::FRAGMENT_WRITABLE_STORAGE,
            self.fragment_rw_storage,
        );
        downlevel.flags.set(
            wgt::DownlevelFlags::CUBE_ARRAY_TEXTURES,
            self.texture_cube_array,
        );
        // TODO: separate the mutable comparisons from immutable ones
        downlevel.flags.set(
            wgt::DownlevelFlags::COMPARISON_SAMPLERS,
            self.mutable_comparison_samplers,
        );
        downlevel.flags.set(
            wgt::DownlevelFlags::INDIRECT_EXECUTION,
            self.indirect_draw_dispatch,
        );
        // TODO: add another flag for `first_instance`
        downlevel.flags.set(
            wgt::DownlevelFlags::BASE_VERTEX,
            self.base_vertex_first_instance_drawing,
        );
        downlevel
            .flags
            .set(wgt::DownlevelFlags::ANISOTROPIC_FILTERING, true);

        let base = wgt::Limits::default();
        // Be careful adjusting limits here. The `AdapterShared` stores the
        // original `PrivateCapabilities`, so code could accidentally use
        // the wrong value. See <https://github.com/gfx-rs/wgpu/issues/8715>.

        let limits = wgt::Limits {
            max_texture_dimension_1d: self.max_texture_size as u32,
            max_texture_dimension_2d: self.max_texture_size as u32,
            max_texture_dimension_3d: self.max_texture_3d_size as u32,
            max_texture_array_layers: self.max_texture_layers as u32,
            max_bind_groups: 8,
            max_bindings_per_bind_group: 65535,
            max_dynamic_uniform_buffers_per_pipeline_layout: base
                .max_dynamic_uniform_buffers_per_pipeline_layout,
            max_dynamic_storage_buffers_per_pipeline_layout: base
                .max_dynamic_storage_buffers_per_pipeline_layout,
            max_sampled_textures_per_shader_stage: self.max_textures_per_stage,
            max_samplers_per_shader_stage: self.max_samplers_per_stage,
            max_storage_buffers_per_shader_stage: self.max_buffers_per_stage,
            max_storage_textures_per_shader_stage: self.max_textures_per_stage,
            max_uniform_buffers_per_shader_stage: self.max_buffers_per_stage,
            max_binding_array_elements_per_shader_stage: self.max_binding_array_elements,
            max_binding_array_sampler_elements_per_shader_stage: self
                .max_sampler_binding_array_elements,
            max_binding_array_acceleration_structure_elements_per_shader_stage: 0,
            // Note: any adjustment here will not be reflected in the stored `PrivateCapabilities`.
            max_uniform_buffer_binding_size: self.max_buffer_size.min(!0u32 as u64),
            max_storage_buffer_binding_size: self.max_buffer_size.min(!0u32 as u64)
                & !(wgt::STORAGE_BINDING_SIZE_ALIGNMENT as u64 - 1),
            max_vertex_buffers: self.max_vertex_buffers,
            max_vertex_attributes: 31,
            max_vertex_buffer_array_stride: base.max_vertex_buffer_array_stride,
            max_immediate_size: 0x1000,
            max_inter_stage_shader_variables: self.max_varying_components / 4,
            min_uniform_buffer_offset_alignment: self.buffer_alignment as u32,
            min_storage_buffer_offset_alignment: self.buffer_alignment as u32,
            max_color_attachments: self.max_color_render_targets as u32,
            max_color_attachment_bytes_per_sample: self.max_color_attachment_bytes_per_sample
                as u32,
            max_compute_workgroup_storage_size: self.max_total_threadgroup_memory,
            max_compute_invocations_per_workgroup: self.max_threads_per_group,
            max_compute_workgroup_size_x: self.max_threads_per_group,
            max_compute_workgroup_size_y: self.max_threads_per_group,
            max_compute_workgroup_size_z: self.max_threads_per_group,
            max_compute_workgroups_per_dimension: 0xFFFF,
            max_buffer_size: self.max_buffer_size,
            max_non_sampler_bindings: u32::MAX,

            // from https://developer.apple.com/documentation/metal/mtlaccelerationstructureusage/extendedlimits
            max_blas_primitive_count: 1 << 28,
            max_blas_geometry_count: 1 << 24,
            max_tlas_instance_count: 1 << 24,
            // From 2.17.7 in https://developer.apple.com/metal/Metal-Shading-Language-Specification.pdf
            // > [Acceleration structures] are opaque objects that can be bound directly using
            // buffer binding points or via argument buffers
            max_acceleration_structures_per_shader_stage: self.max_buffers_per_stage,

            max_multiview_view_count: if self.supported_vertex_amplification_factor > 1 {
                self.supported_vertex_amplification_factor
            } else {
                0
            },

            // Should be not too large
            max_task_mesh_workgroup_total_count: self.max_mesh_task_workgroup_count,
            max_task_mesh_workgroups_per_dimension: self.max_mesh_task_workgroup_count,
            max_task_invocations_per_workgroup: if self.mesh_shaders { 1024 } else { 0 },
            max_task_invocations_per_dimension: if self.mesh_shaders { 1024 } else { 0 },
            max_mesh_invocations_per_workgroup: if self.mesh_shaders { 1024 } else { 0 },
            max_mesh_invocations_per_dimension: if self.mesh_shaders { 1024 } else { 0 },
            // Using certain variables or debuggers can reduce the size by 32 bytes
            max_task_payload_size: self.max_task_payload_size,
            max_mesh_output_vertices: 256,
            max_mesh_output_primitives: 256,
            max_mesh_output_layers: self.max_texture_layers as u32,
            max_mesh_multiview_view_count: 0,
        };

        crate::Capabilities {
            limits,
            alignments: crate::Alignments {
                buffer_copy_offset: wgt::BufferSize::new(self.buffer_alignment).unwrap(),
                buffer_copy_pitch: wgt::BufferSize::new(4).unwrap(),
                // This backend has Naga incorporate bounds checks into the
                // Metal Shading Language it generates, so from `wgpu_hal`'s
                // users' point of view, references are tightly checked.
                uniform_bounds_check_alignment: wgt::BufferSize::new(1).unwrap(),
                raw_tlas_instance_size: size_of::<MTLIndirectAccelerationStructureInstanceDescriptor>(
                ),
                ray_tracing_scratch_buffer_alignment: 1,
            },
            downlevel,
            cooperative_matrix_properties: self.cooperative_matrix_properties(),
        }
    }

    /// Returns the supported cooperative matrix configurations for Metal.
    ///
    /// Metal's simdgroup_matrix supports 8x8 tiles with f16 and f32 element types.
    fn cooperative_matrix_properties(&self) -> Vec<wgt::CooperativeMatrixProperties> {
        if !self.supports_cooperative_matrix || self.msl_version < MTLLanguageVersion::Version2_3 {
            return Vec::new();
        }

        vec![
            // 8x8 f32 configuration
            wgt::CooperativeMatrixProperties {
                m_size: 8,
                n_size: 8,
                k_size: 8,
                ab_type: wgt::CooperativeScalarType::F32,
                cr_type: wgt::CooperativeScalarType::F32,
                saturating_accumulation: false,
            },
            // 8x8 f16 configuration
            wgt::CooperativeMatrixProperties {
                m_size: 8,
                n_size: 8,
                k_size: 8,
                ab_type: wgt::CooperativeScalarType::F16,
                cr_type: wgt::CooperativeScalarType::F16,
                saturating_accumulation: false,
            },
            // Mixed precision: f16 inputs, f32 accumulator
            wgt::CooperativeMatrixProperties {
                m_size: 8,
                n_size: 8,
                k_size: 8,
                ab_type: wgt::CooperativeScalarType::F16,
                cr_type: wgt::CooperativeScalarType::F32,
                saturating_accumulation: false,
            },
        ]
    }

    pub fn private_capabilities(&self) -> super::PrivateCapabilities {
        super::PrivateCapabilities {
            msl_version: self.msl_version,
            low_power: self.low_power,
            headless: self.headless,
            has_unified_memory: self.has_unified_memory,
            timestamp_query_support: self.timestamp_query_support,
            supports_memoryless_storage: self.supports_memoryless_storage,
            mesh_shaders: self.mesh_shaders,
            max_buffers_per_stage: self.max_buffers_per_stage,
            max_vertex_buffers: self.max_vertex_buffers,
            max_textures_per_stage: self.max_textures_per_stage,
            max_samplers_per_stage: self.max_samplers_per_stage,
        }
    }

    pub fn private_texture_format_capabilities(&self) -> super::PrivateTextureFormatCapabilities {
        super::PrivateTextureFormatCapabilities {
            read_write_texture_tier: self.read_write_texture_tier,
            sample_count_mask: self.sample_count_mask,
            int64_atomics: self.int64_atomics,
            msaa_desktop: self.msaa_desktop,
            msaa_apple3: self.msaa_apple3,
            msaa_apple7: self.msaa_apple7,
            format_r32float_all: self.format_r32float_all,
            format_rgba8_srgb_all: self.format_rgba8_srgb_all,
            format_rgb10a2_uint_write: self.format_rgb10a2_uint_write,
            format_rgb10a2_unorm_all: self.format_rgb10a2_unorm_all,
            format_rg11b10_all: self.format_rg11b10_all,
            format_rg32float_all: self.format_rg32float_all,
            format_rgba32float_all: self.format_rgba32float_all,
            format_depth16unorm: self.format_depth16unorm,
            format_depth16unorm_filter: self.format_depth16unorm_filter,
            format_depth32float_filter: self.format_depth32float_filter,
            format_depth24_stencil8: self.format_depth24_stencil8,
            format_bc: self.format_bc,
            format_eac_etc: self.format_eac_etc,
            format_astc: self.format_astc,
            format_astc_hdr: self.format_astc_hdr,
        }
    }
}

impl super::PrivateCapabilities {
    pub fn device_type(&self) -> wgt::DeviceType {
        if self.has_unified_memory.unwrap_or(self.low_power) {
            wgt::DeviceType::IntegratedGpu
        } else {
            wgt::DeviceType::DiscreteGpu
        }
    }
}

impl super::PrivateTextureFormatCapabilities {
    pub fn map_format(&self, format: wgt::TextureFormat) -> MTLPixelFormat {
        use wgt::TextureFormat as Tf;
        use MTLPixelFormat as MTL;
        match format {
            Tf::R8Unorm => MTL::R8Unorm,
            Tf::R8Snorm => MTL::R8Snorm,
            Tf::R8Uint => MTL::R8Uint,
            Tf::R8Sint => MTL::R8Sint,
            Tf::R16Uint => MTL::R16Uint,
            Tf::R16Sint => MTL::R16Sint,
            Tf::R16Unorm => MTL::R16Unorm,
            Tf::R16Snorm => MTL::R16Snorm,
            Tf::R16Float => MTL::R16Float,
            Tf::Rg8Unorm => MTL::RG8Unorm,
            Tf::Rg8Snorm => MTL::RG8Snorm,
            Tf::Rg8Uint => MTL::RG8Uint,
            Tf::Rg8Sint => MTL::RG8Sint,
            Tf::Rg16Unorm => MTL::RG16Unorm,
            Tf::Rg16Snorm => MTL::RG16Snorm,
            Tf::R32Uint => MTL::R32Uint,
            Tf::R32Sint => MTL::R32Sint,
            Tf::R32Float => MTL::R32Float,
            Tf::Rg16Uint => MTL::RG16Uint,
            Tf::Rg16Sint => MTL::RG16Sint,
            Tf::Rg16Float => MTL::RG16Float,
            Tf::Rgba8Unorm => MTL::RGBA8Unorm,
            Tf::Rgba8UnormSrgb => MTL::RGBA8Unorm_sRGB,
            Tf::Bgra8UnormSrgb => MTL::BGRA8Unorm_sRGB,
            Tf::Rgba8Snorm => MTL::RGBA8Snorm,
            Tf::Bgra8Unorm => MTL::BGRA8Unorm,
            Tf::Rgba8Uint => MTL::RGBA8Uint,
            Tf::Rgba8Sint => MTL::RGBA8Sint,
            Tf::Rgb10a2Uint => MTL::RGB10A2Uint,
            Tf::Rgb10a2Unorm => MTL::RGB10A2Unorm,
            Tf::Rg11b10Ufloat => MTL::RG11B10Float,
            // Ruint64 textures are emulated on metal
            Tf::R64Uint => MTL::RG32Uint,
            Tf::Rg32Uint => MTL::RG32Uint,
            Tf::Rg32Sint => MTL::RG32Sint,
            Tf::Rg32Float => MTL::RG32Float,
            Tf::Rgba16Uint => MTL::RGBA16Uint,
            Tf::Rgba16Sint => MTL::RGBA16Sint,
            Tf::Rgba16Unorm => MTL::RGBA16Unorm,
            Tf::Rgba16Snorm => MTL::RGBA16Snorm,
            Tf::Rgba16Float => MTL::RGBA16Float,
            Tf::Rgba32Uint => MTL::RGBA32Uint,
            Tf::Rgba32Sint => MTL::RGBA32Sint,
            Tf::Rgba32Float => MTL::RGBA32Float,
            Tf::Stencil8 => MTL::Stencil8,
            Tf::Depth16Unorm => MTL::Depth16Unorm,
            Tf::Depth32Float => MTL::Depth32Float,
            Tf::Depth32FloatStencil8 => MTL::Depth32Float_Stencil8,
            Tf::Depth24Plus => {
                if self.format_depth24_stencil8 {
                    MTL::Depth24Unorm_Stencil8
                } else {
                    MTL::Depth32Float
                }
            }
            Tf::Depth24PlusStencil8 => {
                if self.format_depth24_stencil8 {
                    MTL::Depth24Unorm_Stencil8
                } else {
                    MTL::Depth32Float_Stencil8
                }
            }
            Tf::NV12 => unreachable!(),
            Tf::P010 => unreachable!(),
            Tf::Rgb9e5Ufloat => MTL::RGB9E5Float,
            Tf::Bc1RgbaUnorm => MTL::BC1_RGBA,
            Tf::Bc1RgbaUnormSrgb => MTL::BC1_RGBA_sRGB,
            Tf::Bc2RgbaUnorm => MTL::BC2_RGBA,
            Tf::Bc2RgbaUnormSrgb => MTL::BC2_RGBA_sRGB,
            Tf::Bc3RgbaUnorm => MTL::BC3_RGBA,
            Tf::Bc3RgbaUnormSrgb => MTL::BC3_RGBA_sRGB,
            Tf::Bc4RUnorm => MTL::BC4_RUnorm,
            Tf::Bc4RSnorm => MTL::BC4_RSnorm,
            Tf::Bc5RgUnorm => MTL::BC5_RGUnorm,
            Tf::Bc5RgSnorm => MTL::BC5_RGSnorm,
            Tf::Bc6hRgbFloat => MTL::BC6H_RGBFloat,
            Tf::Bc6hRgbUfloat => MTL::BC6H_RGBUfloat,
            Tf::Bc7RgbaUnorm => MTL::BC7_RGBAUnorm,
            Tf::Bc7RgbaUnormSrgb => MTL::BC7_RGBAUnorm_sRGB,
            Tf::Etc2Rgb8Unorm => MTL::ETC2_RGB8,
            Tf::Etc2Rgb8UnormSrgb => MTL::ETC2_RGB8_sRGB,
            Tf::Etc2Rgb8A1Unorm => MTL::ETC2_RGB8A1,
            Tf::Etc2Rgb8A1UnormSrgb => MTL::ETC2_RGB8A1_sRGB,
            Tf::Etc2Rgba8Unorm => MTL::EAC_RGBA8,
            Tf::Etc2Rgba8UnormSrgb => MTL::EAC_RGBA8_sRGB,
            Tf::EacR11Unorm => MTL::EAC_R11Unorm,
            Tf::EacR11Snorm => MTL::EAC_R11Snorm,
            Tf::EacRg11Unorm => MTL::EAC_RG11Unorm,
            Tf::EacRg11Snorm => MTL::EAC_RG11Snorm,
            Tf::Astc { block, channel } => match channel {
                AstcChannel::Unorm => match block {
                    AstcBlock::B4x4 => MTL::ASTC_4x4_LDR,
                    AstcBlock::B5x4 => MTL::ASTC_5x4_LDR,
                    AstcBlock::B5x5 => MTL::ASTC_5x5_LDR,
                    AstcBlock::B6x5 => MTL::ASTC_6x5_LDR,
                    AstcBlock::B6x6 => MTL::ASTC_6x6_LDR,
                    AstcBlock::B8x5 => MTL::ASTC_8x5_LDR,
                    AstcBlock::B8x6 => MTL::ASTC_8x6_LDR,
                    AstcBlock::B8x8 => MTL::ASTC_8x8_LDR,
                    AstcBlock::B10x5 => MTL::ASTC_10x5_LDR,
                    AstcBlock::B10x6 => MTL::ASTC_10x6_LDR,
                    AstcBlock::B10x8 => MTL::ASTC_10x8_LDR,
                    AstcBlock::B10x10 => MTL::ASTC_10x10_LDR,
                    AstcBlock::B12x10 => MTL::ASTC_12x10_LDR,
                    AstcBlock::B12x12 => MTL::ASTC_12x12_LDR,
                },
                AstcChannel::UnormSrgb => match block {
                    AstcBlock::B4x4 => MTL::ASTC_4x4_sRGB,
                    AstcBlock::B5x4 => MTL::ASTC_5x4_sRGB,
                    AstcBlock::B5x5 => MTL::ASTC_5x5_sRGB,
                    AstcBlock::B6x5 => MTL::ASTC_6x5_sRGB,
                    AstcBlock::B6x6 => MTL::ASTC_6x6_sRGB,
                    AstcBlock::B8x5 => MTL::ASTC_8x5_sRGB,
                    AstcBlock::B8x6 => MTL::ASTC_8x6_sRGB,
                    AstcBlock::B8x8 => MTL::ASTC_8x8_sRGB,
                    AstcBlock::B10x5 => MTL::ASTC_10x5_sRGB,
                    AstcBlock::B10x6 => MTL::ASTC_10x6_sRGB,
                    AstcBlock::B10x8 => MTL::ASTC_10x8_sRGB,
                    AstcBlock::B10x10 => MTL::ASTC_10x10_sRGB,
                    AstcBlock::B12x10 => MTL::ASTC_12x10_sRGB,
                    AstcBlock::B12x12 => MTL::ASTC_12x12_sRGB,
                },
                AstcChannel::Hdr => match block {
                    AstcBlock::B4x4 => MTL::ASTC_4x4_HDR,
                    AstcBlock::B5x4 => MTL::ASTC_5x4_HDR,
                    AstcBlock::B5x5 => MTL::ASTC_5x5_HDR,
                    AstcBlock::B6x5 => MTL::ASTC_6x5_HDR,
                    AstcBlock::B6x6 => MTL::ASTC_6x6_HDR,
                    AstcBlock::B8x5 => MTL::ASTC_8x5_HDR,
                    AstcBlock::B8x6 => MTL::ASTC_8x6_HDR,
                    AstcBlock::B8x8 => MTL::ASTC_8x8_HDR,
                    AstcBlock::B10x5 => MTL::ASTC_10x5_HDR,
                    AstcBlock::B10x6 => MTL::ASTC_10x6_HDR,
                    AstcBlock::B10x8 => MTL::ASTC_10x8_HDR,
                    AstcBlock::B10x10 => MTL::ASTC_10x10_HDR,
                    AstcBlock::B12x10 => MTL::ASTC_12x10_HDR,
                    AstcBlock::B12x12 => MTL::ASTC_12x12_HDR,
                },
            },
        }
    }

    pub fn map_view_format(
        &self,
        format: wgt::TextureFormat,
        aspects: crate::FormatAspects,
    ) -> MTLPixelFormat {
        use crate::FormatAspects as Fa;
        use wgt::TextureFormat as Tf;
        use MTLPixelFormat as MTL;
        match (format, aspects) {
            // map combined depth-stencil format to their stencil-only format
            // see https://developer.apple.com/library/archive/documentation/Miscellaneous/Conceptual/MetalProgrammingGuide/WhatsNewiniOS10tvOS10andOSX1012/WhatsNewiniOS10tvOS10andOSX1012.html#//apple_ref/doc/uid/TP40014221-CH14-DontLinkElementID_77
            (Tf::Depth24PlusStencil8, Fa::STENCIL) => {
                if self.format_depth24_stencil8 {
                    MTL::X24_Stencil8
                } else {
                    MTL::X32_Stencil8
                }
            }
            (Tf::Depth32FloatStencil8, Fa::STENCIL) => MTL::X32_Stencil8,

            _ => self.map_format(format),
        }
    }
}

impl super::PrivateDisabilities {
    pub fn new(device: &ProtocolObject<dyn MTLDevice>) -> Self {
        let is_intel = device.name().to_string().starts_with("Intel");
        Self {
            broken_viewport_near_depth: is_intel
                && !device.supportsFeatureSet(MTLFeatureSet::macOS_GPUFamily1_v4),
            broken_layered_clear_image: is_intel,
        }
    }
}

impl super::OsType {
    fn new(version: NSOperatingSystemVersion, device: &ProtocolObject<dyn MTLDevice>) -> Self {
        // Metal was first introduced in OS X 10.11 and iOS 8. The current version number of visionOS is 1.0.0. Additionally,
        // on the Simulator, Apple only provides the Apple2 GPU capability, and the Apple2+ GPU capability covers the capabilities of Apple2.
        // Therefore, the following conditions can be used to determine if it is visionOS.
        // https://developer.apple.com/documentation/metal/developing_metal_apps_that_run_in_simulator
        let os_is_vision = version.majorVersion < 8 && device.supportsFamily(MTLGPUFamily::Apple2);
        let os_is_mac = device.supportsFeatureSet(MTLFeatureSet::macOS_GPUFamily1_v1);
        let os_is_tvos = device.supportsFeatureSet(MTLFeatureSet::tvOS_GPUFamily1_v1);
        if os_is_vision {
            Self::VisionOs
        } else if os_is_mac {
            Self::Macos
        } else if os_is_tvos {
            Self::Tvos
        } else {
            Self::Ios
        }
    }
}
