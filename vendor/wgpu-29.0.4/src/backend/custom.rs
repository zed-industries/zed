//! Provides wrappers custom backend implementations

#![allow(ambiguous_wide_pointer_comparisons)]

pub use crate::dispatch::*;

use alloc::sync::Arc;

macro_rules! dyn_type {
    // cloning of arc forbidden
    // but we still use it to provide Eq,Ord,Hash implementations
    (pub mut struct $name:ident(dyn $interface:tt)) => {
        #[derive(Debug)]
        pub(crate) struct $name(Arc<dyn $interface>);
        crate::cmp::impl_eq_ord_hash_arc_address!($name => .0);

        impl $name {
            pub(crate) fn new<T: $interface>(t: T) -> Self {
                Self(Arc::new(t))
            }

            #[allow(clippy::allow_attributes, dead_code)]
            pub(crate) fn downcast<T: $interface>(&self) -> Option<&T> {
                self.0.as_ref().as_any().downcast_ref()
            }
        }

        impl core::ops::Deref for $name {
            type Target = dyn $interface;

            #[inline]
            fn deref(&self) -> &Self::Target {
                self.0.as_ref()
            }
        }

        impl core::ops::DerefMut for $name {
            #[inline]
            fn deref_mut(&mut self) -> &mut Self::Target {
                Arc::get_mut(&mut self.0).expect("")
            }
        }
    };
    // cloning of arc is allowed
    (pub ref struct $name:ident(dyn $interface:tt)) => {
        #[derive(Debug, Clone)]
        pub(crate) struct $name(Arc<dyn $interface>);
        crate::cmp::impl_eq_ord_hash_arc_address!($name => .0);

        impl $name {
            pub(crate) fn new<T: $interface>(t: T) -> Self {
                Self(Arc::new(t))
            }

            pub(crate) fn downcast<T: $interface>(&self) -> Option<&T> {
                self.0.as_ref().as_any().downcast_ref()
            }
        }

        impl core::ops::Deref for $name {
            type Target = dyn $interface;

            #[inline]
            fn deref(&self) -> &Self::Target {
                self.0.as_ref()
            }
        }
    };
}

dyn_type!(pub ref struct DynContext(dyn InstanceInterface));
dyn_type!(pub ref struct DynAdapter(dyn AdapterInterface));
dyn_type!(pub ref struct DynDevice(dyn DeviceInterface));
dyn_type!(pub ref struct DynQueue(dyn QueueInterface));
dyn_type!(pub ref struct DynShaderModule(dyn ShaderModuleInterface));
dyn_type!(pub ref struct DynBindGroupLayout(dyn BindGroupLayoutInterface));
dyn_type!(pub ref struct DynBindGroup(dyn BindGroupInterface));
dyn_type!(pub ref struct DynTextureView(dyn TextureViewInterface));
dyn_type!(pub ref struct DynSampler(dyn SamplerInterface));
dyn_type!(pub ref struct DynBuffer(dyn BufferInterface));
dyn_type!(pub ref struct DynTexture(dyn TextureInterface));
dyn_type!(pub ref struct DynExternalTexture(dyn ExternalTextureInterface));
dyn_type!(pub ref struct DynBlas(dyn BlasInterface));
dyn_type!(pub ref struct DynTlas(dyn TlasInterface));
dyn_type!(pub ref struct DynQuerySet(dyn QuerySetInterface));
dyn_type!(pub ref struct DynPipelineLayout(dyn PipelineLayoutInterface));
dyn_type!(pub ref struct DynRenderPipeline(dyn RenderPipelineInterface));
dyn_type!(pub ref struct DynComputePipeline(dyn ComputePipelineInterface));
dyn_type!(pub ref struct DynPipelineCache(dyn PipelineCacheInterface));
dyn_type!(pub mut struct DynCommandEncoder(dyn CommandEncoderInterface));
dyn_type!(pub mut struct DynComputePass(dyn ComputePassInterface));
dyn_type!(pub mut struct DynRenderPass(dyn RenderPassInterface));
dyn_type!(pub mut struct DynCommandBuffer(dyn CommandBufferInterface));
dyn_type!(pub mut struct DynRenderBundleEncoder(dyn RenderBundleEncoderInterface));
dyn_type!(pub ref struct DynRenderBundle(dyn RenderBundleInterface));
dyn_type!(pub ref struct DynSurface(dyn SurfaceInterface));
dyn_type!(pub ref struct DynSurfaceOutputDetail(dyn SurfaceOutputDetailInterface));
dyn_type!(pub mut struct DynQueueWriteBuffer(dyn QueueWriteBufferInterface));
dyn_type!(pub mut struct DynBufferMappedRange(dyn BufferMappedRangeInterface));
