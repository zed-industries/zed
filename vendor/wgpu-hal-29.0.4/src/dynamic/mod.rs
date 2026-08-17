mod adapter;
mod command;
mod device;
mod instance;
mod queue;
mod surface;

pub use adapter::{DynAdapter, DynOpenDevice};
pub use command::DynCommandEncoder;
pub use device::DynDevice;
pub use instance::{DynExposedAdapter, DynInstance};
pub use queue::DynQueue;
pub use surface::{DynAcquiredSurfaceTexture, DynSurface};

use alloc::boxed::Box;
use core::{
    any::{Any, TypeId},
    fmt,
};

use wgt::WasmNotSendSync;

use crate::{
    AccelerationStructureAABBs, AccelerationStructureEntries, AccelerationStructureInstances,
    AccelerationStructureTriangleIndices, AccelerationStructureTriangleTransform,
    AccelerationStructureTriangles, BufferBinding, ExternalTextureBinding, ProgrammableStage,
    TextureBinding,
};

/// Base trait for all resources, allows downcasting via [`Any`].
pub trait DynResource: Any + WasmNotSendSync + 'static {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

/// Utility macro for implementing `DynResource` for a list of types.
macro_rules! impl_dyn_resource {
    ($($type:ty),*) => {
        $(
            impl crate::DynResource for $type {
                fn as_any(&self) -> &dyn ::core::any::Any {
                    self
                }

                fn as_any_mut(&mut self) -> &mut dyn ::core::any::Any {
                    self
                }
            }
        )*
    };
}
pub(crate) use impl_dyn_resource;

/// Extension trait for `DynResource` used by implementations of various dynamic resource traits.
trait DynResourceExt {
    /// # Panics
    ///
    /// - Panics if `self` is not downcastable to `T`.
    fn expect_downcast_ref<T: DynResource>(&self) -> &T;
    /// # Panics
    ///
    /// - Panics if `self` is not downcastable to `T`.
    fn expect_downcast_mut<T: DynResource>(&mut self) -> &mut T;

    /// Unboxes a `Box<dyn DynResource>` to a concrete type.
    ///
    /// # Safety
    ///
    /// - `self` must be the correct concrete type.
    unsafe fn unbox<T: DynResource + 'static>(self: Box<Self>) -> T;
}

impl<R: DynResource + ?Sized> DynResourceExt for R {
    fn expect_downcast_ref<'a, T: DynResource>(&'a self) -> &'a T {
        self.as_any()
            .downcast_ref()
            .expect("Resource doesn't have the expected backend type.")
    }

    fn expect_downcast_mut<'a, T: DynResource>(&'a mut self) -> &'a mut T {
        self.as_any_mut()
            .downcast_mut()
            .expect("Resource doesn't have the expected backend type.")
    }

    unsafe fn unbox<T: DynResource + 'static>(self: Box<Self>) -> T {
        debug_assert!(
            <Self as Any>::type_id(self.as_ref()) == TypeId::of::<T>(),
            "Resource doesn't have the expected type, expected {:?}, got {:?}",
            TypeId::of::<T>(),
            <Self as Any>::type_id(self.as_ref())
        );

        let casted_ptr = Box::into_raw(self).cast::<T>();
        // SAFETY: This is adheres to the safety contract of `Box::from_raw` because:
        //
        // - We are casting the value of a previously `Box`ed value, which guarantees:
        //   - `casted_ptr` is not null.
        //   - `casted_ptr` is valid for reads and writes, though by itself this does not mean
        //     valid reads and writes for `T` (read on for that).
        // - We don't change the allocator.
        // - The contract of `Box::from_raw` requires that an initialized and aligned `T` is stored
        //   within `casted_ptr`.
        *unsafe { Box::from_raw(casted_ptr) }
    }
}

pub trait DynAccelerationStructure: DynResource + fmt::Debug {}
pub trait DynBindGroup: DynResource + fmt::Debug {}
pub trait DynBindGroupLayout: DynResource + fmt::Debug {}
pub trait DynBuffer: DynResource + fmt::Debug {}
pub trait DynCommandBuffer: DynResource + fmt::Debug {}
pub trait DynComputePipeline: DynResource + fmt::Debug {}
pub trait DynFence: DynResource + fmt::Debug {}
pub trait DynPipelineCache: DynResource + fmt::Debug {}
pub trait DynPipelineLayout: DynResource + fmt::Debug {}
pub trait DynQuerySet: DynResource + fmt::Debug {}
pub trait DynRenderPipeline: DynResource + fmt::Debug {}
pub trait DynSampler: DynResource + fmt::Debug {}
pub trait DynShaderModule: DynResource + fmt::Debug {}
pub trait DynSurfaceTexture:
    DynResource + core::borrow::Borrow<dyn DynTexture> + fmt::Debug
{
}
pub trait DynTexture: DynResource + fmt::Debug {}
pub trait DynTextureView: DynResource + fmt::Debug {}

impl<'a> BufferBinding<'a, dyn DynBuffer> {
    pub fn expect_downcast<B: DynBuffer>(self) -> BufferBinding<'a, B> {
        BufferBinding {
            buffer: self.buffer.expect_downcast_ref(),
            offset: self.offset,
            size: self.size,
        }
    }
}

impl<'a> TextureBinding<'a, dyn DynTextureView> {
    pub fn expect_downcast<T: DynTextureView>(self) -> TextureBinding<'a, T> {
        TextureBinding {
            view: self.view.expect_downcast_ref(),
            usage: self.usage,
        }
    }
}

impl<'a> ExternalTextureBinding<'a, dyn DynBuffer, dyn DynTextureView> {
    pub fn expect_downcast<B: DynBuffer, T: DynTextureView>(
        self,
    ) -> ExternalTextureBinding<'a, B, T> {
        let planes = self.planes.map(|plane| plane.expect_downcast());
        let params = self.params.expect_downcast();
        ExternalTextureBinding { planes, params }
    }
}

impl<'a> ProgrammableStage<'a, dyn DynShaderModule> {
    fn expect_downcast<T: DynShaderModule>(self) -> ProgrammableStage<'a, T> {
        ProgrammableStage {
            module: self.module.expect_downcast_ref(),
            entry_point: self.entry_point,
            constants: self.constants,
            zero_initialize_workgroup_memory: self.zero_initialize_workgroup_memory,
        }
    }
}

impl<'a> AccelerationStructureEntries<'a, dyn DynBuffer> {
    fn expect_downcast<B: DynBuffer>(&self) -> AccelerationStructureEntries<'a, B> {
        match self {
            AccelerationStructureEntries::Instances(instances) => {
                AccelerationStructureEntries::Instances(AccelerationStructureInstances {
                    buffer: instances.buffer.map(|b| b.expect_downcast_ref()),
                    offset: instances.offset,
                    count: instances.count,
                })
            }
            AccelerationStructureEntries::Triangles(triangles) => {
                AccelerationStructureEntries::Triangles(
                    triangles
                        .iter()
                        .map(|t| AccelerationStructureTriangles {
                            vertex_buffer: t.vertex_buffer.map(|b| b.expect_downcast_ref()),
                            vertex_format: t.vertex_format,
                            first_vertex: t.first_vertex,
                            vertex_count: t.vertex_count,
                            vertex_stride: t.vertex_stride,
                            indices: t.indices.as_ref().map(|i| {
                                AccelerationStructureTriangleIndices {
                                    buffer: i.buffer.map(|b| b.expect_downcast_ref()),
                                    format: i.format,
                                    offset: i.offset,
                                    count: i.count,
                                }
                            }),
                            transform: t.transform.as_ref().map(|t| {
                                AccelerationStructureTriangleTransform {
                                    buffer: t.buffer.expect_downcast_ref(),
                                    offset: t.offset,
                                }
                            }),
                            flags: t.flags,
                        })
                        .collect(),
                )
            }
            AccelerationStructureEntries::AABBs(entries) => AccelerationStructureEntries::AABBs(
                entries
                    .iter()
                    .map(|e| AccelerationStructureAABBs {
                        buffer: e.buffer.map(|b| b.expect_downcast_ref()),
                        offset: e.offset,
                        count: e.count,
                        stride: e.stride,
                        flags: e.flags,
                    })
                    .collect(),
            ),
        }
    }
}
