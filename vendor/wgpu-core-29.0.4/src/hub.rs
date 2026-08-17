/*! Allocating resource ids, and tracking the resources they refer to.

The `wgpu_core` API uses identifiers of type [`Id<R>`] to refer to
resources of type `R`. For example, [`id::DeviceId`] is an alias for
`Id<markers::Device>`, and [`id::BufferId`] is an alias for
`Id<markers::Buffer>`. `Id` implements `Copy`, `Hash`, `Eq`, `Ord`, and
of course `Debug`.

[`id::DeviceId`]: crate::id::DeviceId
[`id::BufferId`]: crate::id::BufferId

Each `Id` contains not only an index for the resource it denotes but
also a Backend indicating which `wgpu` backend it belongs to.

`Id`s also incorporate a generation number, for additional validation.

The resources to which identifiers refer are freed explicitly.
Attempting to use an identifier for a resource that has been freed
elicits an error result.

Eventually, we would like to remove numeric IDs from wgpu-core.
See <https://github.com/gfx-rs/wgpu/issues/5121>.

## Assigning ids to resources

The users of `wgpu_core` generally want resource ids to be assigned
in one of two ways:

- Users like `wgpu` want `wgpu_core` to assign ids to resources itself.
  For example, `wgpu` expects to call `Global::device_create_buffer`
  and have the return value indicate the newly created buffer's id.

- Users like Firefox want to allocate ids themselves, and pass
  `Global::device_create_buffer` and friends the id to assign the new
  resource.

To accommodate either pattern, `wgpu_core` methods that create
resources all expect an `id_in` argument that the caller can use to
specify the id, and they all return the id used. For example, the
declaration of `Global::device_create_buffer` looks like this:

```ignore
impl Global {
    /* ... */
    pub fn device_create_buffer<A: HalApi>(
        &self,
        device_id: id::DeviceId,
        desc: &resource::BufferDescriptor,
        id_in: Input<G>,
    ) -> (id::BufferId, Option<resource::CreateBufferError>) {
        /* ... */
    }
    /* ... */
}
```

Users that want to assign resource ids themselves pass in the id they
want as the `id_in` argument, whereas users that want `wgpu_core`
itself to choose ids always pass `()`. In either case, the id
ultimately assigned is returned as the first element of the tuple.

Producing true identifiers from `id_in` values is the job of an
[`crate::identity::IdentityManager`] or ids will be received from outside through `Option<Id>` arguments.

## Id allocation and streaming

Perhaps surprisingly, allowing users to assign resource ids themselves
enables major performance improvements in some applications.

The `wgpu_core` API is designed for use by Firefox's [WebGPU]
implementation. For security, web content and GPU use must be kept
segregated in separate processes, with all interaction between them
mediated by an inter-process communication protocol. As web content uses
the WebGPU API, the content process sends messages to the GPU process,
which interacts with the platform's GPU APIs on content's behalf,
occasionally sending results back.

In a classic Rust API, a resource allocation function takes parameters
describing the resource to create, and if creation succeeds, it returns
the resource id in a `Result::Ok` value. However, this design is a poor
fit for the split-process design described above: content must wait for
the reply to its buffer-creation message (say) before it can know which
id it can use in the next message that uses that buffer. On a common
usage pattern, the classic Rust design imposes the latency of a full
cross-process round trip.

We can avoid incurring these round-trip latencies simply by letting the
content process assign resource ids itself. With this approach, content
can choose an id for the new buffer, send a message to create the
buffer, and then immediately send the next message operating on that
buffer, since it already knows its id. Allowing content and GPU process
activity to be pipelined greatly improves throughput.

To help propagate errors correctly in this style of usage, when resource
creation fails, the id supplied for that resource is marked to indicate
as much, allowing subsequent operations using that id to be properly
flagged as errors as well.

[`process`]: crate::identity::IdentityManager::process
[`Id<R>`]: crate::id::Id
[wrapped in a mutex]: trait.IdentityHandler.html#impl-IdentityHandler%3CI%3E-for-Mutex%3CIdentityManager%3E
[WebGPU]: https://www.w3.org/TR/webgpu/

## IDs and tracing

As of `wgpu` v27, commands are encoded all at once when
`CommandEncoder::finish` is called, not when the encoding methods are
called for each command. This implies storing a representation of the
commands in memory until `finish` is called.  `Arc`s are more suitable
for this purpose than numeric ids. Rather than redundantly store both
`Id`s and `Arc`s, tracing has been changed to work with `Arc`s. The
serialized trace identifies resources by the integer value of
`Arc::as_ptr`. These IDs have the type [`crate::id::PointerId`]. The
trace player uses hash maps to go from `PointerId`s to `Arc`s
when replaying a trace.

*/

use alloc::sync::Arc;
use core::fmt::Debug;

use crate::{
    binding_model::{BindGroup, BindGroupLayout, PipelineLayout},
    command::{CommandBuffer, CommandEncoder, RenderBundle},
    device::{queue::Queue, Device},
    instance::Adapter,
    pipeline::{ComputePipeline, PipelineCache, RenderPipeline, ShaderModule},
    registry::{Registry, RegistryReport},
    resource::{
        Blas, Buffer, ExternalTexture, Fallible, QuerySet, Sampler, StagingBuffer, Texture,
        TextureView, Tlas,
    },
};

#[derive(Debug, PartialEq, Eq)]
pub struct HubReport {
    pub adapters: RegistryReport,
    pub devices: RegistryReport,
    pub queues: RegistryReport,
    pub pipeline_layouts: RegistryReport,
    pub shader_modules: RegistryReport,
    pub bind_group_layouts: RegistryReport,
    pub bind_groups: RegistryReport,
    pub command_encoders: RegistryReport,
    pub command_buffers: RegistryReport,
    pub render_bundles: RegistryReport,
    pub render_pipelines: RegistryReport,
    pub compute_pipelines: RegistryReport,
    pub pipeline_caches: RegistryReport,
    pub query_sets: RegistryReport,
    pub buffers: RegistryReport,
    pub textures: RegistryReport,
    pub texture_views: RegistryReport,
    pub external_textures: RegistryReport,
    pub samplers: RegistryReport,
}

impl HubReport {
    pub fn is_empty(&self) -> bool {
        self.adapters.is_empty()
    }
}

#[allow(rustdoc::private_intra_doc_links)]
/// All the resources tracked by a [`crate::global::Global`].
///
/// ## Locking
///
/// Each field in `Hub` is a [`Registry`] holding all the values of a
/// particular type of resource, all protected by a single RwLock.
/// So for example, to access any [`Buffer`], you must acquire a read
/// lock on the `Hub`s entire buffers registry. The lock guard
/// gives you access to the `Registry`'s [`Storage`], which you can
/// then index with the buffer's id. (Yes, this design causes
/// contention; see [#2272].)
///
/// But most `wgpu` operations require access to several different
/// kinds of resource, so you often need to hold locks on several
/// different fields of your [`Hub`] simultaneously.
///
/// Inside the `Registry` there are `Arc<T>` where `T` is a Resource
/// Lock of `Registry` happens only when accessing to get the specific resource
///
/// [`Storage`]: crate::storage::Storage
pub struct Hub {
    pub(crate) adapters: Registry<Arc<Adapter>>,
    pub(crate) devices: Registry<Arc<Device>>,
    pub(crate) queues: Registry<Arc<Queue>>,
    pub(crate) pipeline_layouts: Registry<Fallible<PipelineLayout>>,
    pub(crate) shader_modules: Registry<Fallible<ShaderModule>>,
    pub(crate) bind_group_layouts: Registry<Fallible<BindGroupLayout>>,
    pub(crate) bind_groups: Registry<Fallible<BindGroup>>,
    pub(crate) command_encoders: Registry<Arc<CommandEncoder>>,
    pub(crate) command_buffers: Registry<Arc<CommandBuffer>>,
    pub(crate) render_bundles: Registry<Fallible<RenderBundle>>,
    pub(crate) render_pipelines: Registry<Fallible<RenderPipeline>>,
    pub(crate) compute_pipelines: Registry<Fallible<ComputePipeline>>,
    pub(crate) pipeline_caches: Registry<Fallible<PipelineCache>>,
    pub(crate) query_sets: Registry<Fallible<QuerySet>>,
    pub(crate) buffers: Registry<Fallible<Buffer>>,
    pub(crate) staging_buffers: Registry<StagingBuffer>,
    pub(crate) textures: Registry<Fallible<Texture>>,
    pub(crate) texture_views: Registry<Fallible<TextureView>>,
    pub(crate) external_textures: Registry<Fallible<ExternalTexture>>,
    pub(crate) samplers: Registry<Fallible<Sampler>>,
    pub(crate) blas_s: Registry<Fallible<Blas>>,
    pub(crate) tlas_s: Registry<Fallible<Tlas>>,
}

impl Hub {
    pub(crate) fn new() -> Self {
        Self {
            adapters: Registry::new(),
            devices: Registry::new(),
            queues: Registry::new(),
            pipeline_layouts: Registry::new(),
            shader_modules: Registry::new(),
            bind_group_layouts: Registry::new(),
            bind_groups: Registry::new(),
            command_encoders: Registry::new(),
            command_buffers: Registry::new(),
            render_bundles: Registry::new(),
            render_pipelines: Registry::new(),
            compute_pipelines: Registry::new(),
            pipeline_caches: Registry::new(),
            query_sets: Registry::new(),
            buffers: Registry::new(),
            staging_buffers: Registry::new(),
            textures: Registry::new(),
            texture_views: Registry::new(),
            external_textures: Registry::new(),
            samplers: Registry::new(),
            blas_s: Registry::new(),
            tlas_s: Registry::new(),
        }
    }

    pub fn generate_report(&self) -> HubReport {
        HubReport {
            adapters: self.adapters.generate_report(),
            devices: self.devices.generate_report(),
            queues: self.queues.generate_report(),
            pipeline_layouts: self.pipeline_layouts.generate_report(),
            shader_modules: self.shader_modules.generate_report(),
            bind_group_layouts: self.bind_group_layouts.generate_report(),
            bind_groups: self.bind_groups.generate_report(),
            command_encoders: self.command_encoders.generate_report(),
            command_buffers: self.command_buffers.generate_report(),
            render_bundles: self.render_bundles.generate_report(),
            render_pipelines: self.render_pipelines.generate_report(),
            compute_pipelines: self.compute_pipelines.generate_report(),
            pipeline_caches: self.pipeline_caches.generate_report(),
            query_sets: self.query_sets.generate_report(),
            buffers: self.buffers.generate_report(),
            textures: self.textures.generate_report(),
            texture_views: self.texture_views.generate_report(),
            external_textures: self.external_textures.generate_report(),
            samplers: self.samplers.generate_report(),
        }
    }
}
