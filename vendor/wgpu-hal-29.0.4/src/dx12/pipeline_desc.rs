//! We try to use pipeline stream descriptors where possible, but this isn't allowed
//! on some older windows 10 versions. Therefore, we also must have some logic to
//! convert such descriptors to the "traditional" equivalent,
//! `D3D12_GRAPHICS_PIPELINE_STATE_DESC`.
//!
//! Stream descriptors allow extending the pipeline, enabling more advanced features,
//! including mesh shaders and multiview/view instancing. Using a stream descriptor
//! is like using a vulkan descriptor with a `pNext` chain. It doesn't have direct
//! benefits to all use cases, but allows new use cases.
//!
//! The code for pipeline stream descriptors is very complicated, and can have bad
//! consequences if it is written incorrectly. It has been isolated to this file for
//! that reason.

use core::{ffi::c_void, mem::ManuallyDrop, ptr::NonNull};

use alloc::vec::Vec;
use windows::Win32::Graphics::Direct3D12::*;
use windows::Win32::Graphics::Dxgi::Common::*;
use windows_core::Interface;

use crate::dx12::borrow_interface_temporarily;

// Wrapper newtypes for various pipeline subobjects which
// use complicated or non-unique representations.

#[repr(transparent)]
#[derive(Copy, Clone)]
// Option<NonNull<c_void>> is guaranteed to have the same representation as a raw pointer.
struct RootSignature(Option<NonNull<c_void>>);

#[repr(transparent)]
#[derive(Copy, Clone)]
struct VertexShader(D3D12_SHADER_BYTECODE);
#[repr(transparent)]
#[derive(Copy, Clone)]
struct PixelShader(D3D12_SHADER_BYTECODE);

#[repr(transparent)]
#[derive(Copy, Clone)]
struct MeshShader(D3D12_SHADER_BYTECODE);

#[repr(transparent)]
#[derive(Copy, Clone)]
struct TaskShader(D3D12_SHADER_BYTECODE);

#[repr(transparent)]
#[derive(Copy, Clone)]
struct SampleMask(u32);

#[repr(transparent)]
#[derive(Copy, Clone)]
struct NodeMask(u32);

/// Trait for types that can be used as subobjects in a pipeline state stream.
///
/// Safety:
/// - The type must be the correct alignment and size for the subobject it represents.
/// - The type must map to exactly one `D3D12_PIPELINE_STATE_SUBOBJECT_TYPE` variant.
/// - The variant must correctly represent the type's role in the pipeline state stream.
/// - The type must be `Copy` to ensure safe duplication in the stream.
/// - The type must be valid to memcpy into the pipeline state stream.
unsafe trait RenderPipelineStreamObject: Copy {
    const SUBOBJECT_TYPE: D3D12_PIPELINE_STATE_SUBOBJECT_TYPE;
}

macro_rules! implement_stream_object {
    (unsafe $ty:ty => $variant:expr) => {
        unsafe impl RenderPipelineStreamObject for $ty {
            const SUBOBJECT_TYPE: D3D12_PIPELINE_STATE_SUBOBJECT_TYPE = $variant;
        }
    };
}

implement_stream_object! { unsafe RootSignature => D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_ROOT_SIGNATURE }
implement_stream_object! { unsafe VertexShader => D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_VS }
implement_stream_object! { unsafe PixelShader => D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_PS }
implement_stream_object! { unsafe MeshShader => D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_MS }
implement_stream_object! { unsafe TaskShader => D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_AS }
implement_stream_object! { unsafe D3D12_BLEND_DESC => D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_BLEND }
implement_stream_object! { unsafe SampleMask => D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_SAMPLE_MASK }
implement_stream_object! { unsafe D3D12_RASTERIZER_DESC => D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_RASTERIZER }
implement_stream_object! { unsafe D3D12_DEPTH_STENCIL_DESC => D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_DEPTH_STENCIL }
implement_stream_object! { unsafe D3D12_PRIMITIVE_TOPOLOGY_TYPE => D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_PRIMITIVE_TOPOLOGY }
implement_stream_object! { unsafe D3D12_RT_FORMAT_ARRAY => D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_RENDER_TARGET_FORMATS }
implement_stream_object! { unsafe DXGI_FORMAT => D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_DEPTH_STENCIL_FORMAT }
implement_stream_object! { unsafe DXGI_SAMPLE_DESC => D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_SAMPLE_DESC }
implement_stream_object! { unsafe NodeMask => D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_NODE_MASK }
implement_stream_object! { unsafe D3D12_CACHED_PIPELINE_STATE => D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_CACHED_PSO }
implement_stream_object! { unsafe D3D12_PIPELINE_STATE_FLAGS => D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_FLAGS }
implement_stream_object! { unsafe D3D12_INPUT_LAYOUT_DESC => D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_INPUT_LAYOUT }
implement_stream_object! { unsafe D3D12_INDEX_BUFFER_STRIP_CUT_VALUE => D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_IB_STRIP_CUT_VALUE }
implement_stream_object! { unsafe D3D12_STREAM_OUTPUT_DESC => D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_STREAM_OUTPUT }
implement_stream_object! { unsafe D3D12_VIEW_INSTANCING_DESC => D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_VIEW_INSTANCING }

/// Implementaation of a pipeline state stream, which is a sequence of subobjects put into
/// a byte array according to some basic alignment rules.
///
/// Each subobject must start on an 8 byte boundary. Each subobject contains a 32 bit
/// type identifier, followed by the actual subobject data, aligned as required by the
/// subobject's structure.
///
/// See <https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ns-d3d12-d3d12_pipeline_state_stream_desc>
/// for more information.
pub(super) struct RenderPipelineStateStream<'a> {
    bytes: Vec<u8>,
    _marker: core::marker::PhantomData<&'a ()>,
}

impl<'a> RenderPipelineStateStream<'a> {
    fn new() -> Self {
        // Dynamic allocation is used here because the resulting stream can become very large.
        // We pre-allocate the size based on an estimate of the size of the struct plus some extra space
        // per member for tags and alignment padding. In practice this will always be too big, as not
        // all members will be used.
        let size_of_stream_desc = size_of::<RenderPipelineStateStreamDesc>();
        let members = 20; // Approximate number of members we might push
        let capacity = size_of_stream_desc + members * 8; // Extra space for tags and alignment
        Self {
            bytes: Vec::with_capacity(capacity),
            _marker: core::marker::PhantomData,
        }
    }

    /// Align the internal byte buffer to the given alignment,
    /// padding with zeros as necessary.
    fn align_to(&mut self, alignment: usize) {
        let aligned_length = self.bytes.len().next_multiple_of(alignment);
        self.bytes.resize(aligned_length, 0);
    }

    /// Adds a subobject to the pipeline state stream.
    fn add_object<T: RenderPipelineStreamObject>(&mut self, object: T) {
        // Ensure 8-byte alignment for the subobject start.
        self.align_to(8);

        // Append the type tag (u32)
        let tag: u32 = T::SUBOBJECT_TYPE.0 as u32;
        self.bytes.extend_from_slice(&tag.to_ne_bytes());

        // Align the data to its natural alignment.
        self.align_to(align_of_val::<T>(&object));

        // Append the data itself, as raw bytes
        let data_ptr: *const T = &object;
        let data_u8_ptr: *const u8 = data_ptr.cast::<u8>();
        let data_size = size_of_val::<T>(&object);
        let slice = unsafe { core::slice::from_raw_parts::<u8>(data_u8_ptr, data_size) };
        self.bytes.extend_from_slice(slice);
    }

    /// Creates a pipeline state object from the stream.
    ///
    /// Safety:
    /// - All unsafety invariants required by [`ID3D12Device2::CreatePipelineState`] must be upheld by the caller.
    pub unsafe fn create_pipeline_state(
        &mut self,
        device: &ID3D12Device2,
    ) -> windows::core::Result<ID3D12PipelineState> {
        let stream_desc = D3D12_PIPELINE_STATE_STREAM_DESC {
            SizeInBytes: self.bytes.len(),
            pPipelineStateSubobjectStream: self.bytes.as_mut_ptr().cast(),
        };

        // Safety: lifetime on Self preserved the contents
        // of the stream. Other unsafety invariants are upheld by the caller.
        unsafe { device.CreatePipelineState(&stream_desc) }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct RenderPipelineStateStreamDesc<'a> {
    pub root_signature: Option<&'a ID3D12RootSignature>,
    pub pixel_shader: D3D12_SHADER_BYTECODE,
    pub blend_state: D3D12_BLEND_DESC,
    pub sample_mask: u32,
    pub rasterizer_state: D3D12_RASTERIZER_DESC,
    pub depth_stencil_state: D3D12_DEPTH_STENCIL_DESC,
    pub primitive_topology_type: D3D12_PRIMITIVE_TOPOLOGY_TYPE,
    pub rtv_formats: D3D12_RT_FORMAT_ARRAY,
    pub dsv_format: DXGI_FORMAT,
    pub sample_desc: DXGI_SAMPLE_DESC,
    pub node_mask: u32,
    pub cached_pso: D3D12_CACHED_PIPELINE_STATE,
    pub flags: D3D12_PIPELINE_STATE_FLAGS,
    pub view_instancing: Option<D3D12_VIEW_INSTANCING_DESC>,

    // Vertex pipeline specific
    pub vertex_shader: D3D12_SHADER_BYTECODE,
    pub input_layout: D3D12_INPUT_LAYOUT_DESC,
    pub index_buffer_strip_cut_value: D3D12_INDEX_BUFFER_STRIP_CUT_VALUE,
    pub stream_output: D3D12_STREAM_OUTPUT_DESC,

    // Mesh pipeline specific
    pub task_shader: D3D12_SHADER_BYTECODE,
    pub mesh_shader: D3D12_SHADER_BYTECODE,
}

impl RenderPipelineStateStreamDesc<'_> {
    pub fn to_stream(&self) -> RenderPipelineStateStream<'_> {
        let mut stream = RenderPipelineStateStream::new();

        // Importantly here, the ID3D12RootSignature _itself_ is the pointer we're
        // trying to serialize into the stream, not a pointer to the pointer.
        //
        // This is correct because as_raw() returns turns that smart object into the raw
        // pointer that _is_ the com object handle.
        let root_sig_pointer = self
            .root_signature
            .map(|a| NonNull::new(a.as_raw()).unwrap());
        // Because the stream object borrows from self for its entire lifetime,
        // it is safe to store the pointer into it.
        stream.add_object(RootSignature(root_sig_pointer));

        stream.add_object(self.blend_state);
        stream.add_object(SampleMask(self.sample_mask));
        stream.add_object(self.rasterizer_state);
        stream.add_object(self.depth_stencil_state);
        stream.add_object(self.primitive_topology_type);
        if self.rtv_formats.NumRenderTargets != 0 {
            stream.add_object(self.rtv_formats);
        }
        if self.dsv_format != DXGI_FORMAT_UNKNOWN {
            stream.add_object(self.dsv_format);
        }
        stream.add_object(self.sample_desc);
        if self.node_mask != 0 {
            stream.add_object(NodeMask(self.node_mask));
        }
        if !self.cached_pso.pCachedBlob.is_null() {
            stream.add_object(self.cached_pso);
        }
        stream.add_object(self.flags);
        if let Some(view_instancing) = self.view_instancing {
            stream.add_object(view_instancing);
        }
        if !self.pixel_shader.pShaderBytecode.is_null() {
            stream.add_object(PixelShader(self.pixel_shader));
        }
        if !self.vertex_shader.pShaderBytecode.is_null() {
            stream.add_object(VertexShader(self.vertex_shader));
            stream.add_object(self.input_layout);
            stream.add_object(self.index_buffer_strip_cut_value);
            stream.add_object(self.stream_output);
        }
        if !self.task_shader.pShaderBytecode.is_null() {
            stream.add_object(TaskShader(self.task_shader));
        }
        if !self.mesh_shader.pShaderBytecode.is_null() {
            stream.add_object(MeshShader(self.mesh_shader));
        }

        stream
    }

    /// Returns a traditional D3D12_GRAPHICS_PIPELINE_STATE_DESC.
    ///
    /// Safety:
    /// - This returned struct must not outlive self.
    pub unsafe fn to_graphics_pipeline_descriptor(&self) -> D3D12_GRAPHICS_PIPELINE_STATE_DESC {
        D3D12_GRAPHICS_PIPELINE_STATE_DESC {
            pRootSignature: if let Some(rsig) = self.root_signature {
                unsafe { borrow_interface_temporarily(rsig) }
            } else {
                ManuallyDrop::new(None)
            },
            VS: self.vertex_shader,
            PS: self.pixel_shader,
            DS: D3D12_SHADER_BYTECODE::default(),
            HS: D3D12_SHADER_BYTECODE::default(),
            GS: D3D12_SHADER_BYTECODE::default(),
            StreamOutput: self.stream_output,
            BlendState: self.blend_state,
            SampleMask: self.sample_mask,
            RasterizerState: self.rasterizer_state,
            DepthStencilState: self.depth_stencil_state,
            InputLayout: self.input_layout,
            IBStripCutValue: self.index_buffer_strip_cut_value,
            PrimitiveTopologyType: self.primitive_topology_type,
            NumRenderTargets: self.rtv_formats.NumRenderTargets,
            RTVFormats: self.rtv_formats.RTFormats,
            DSVFormat: self.dsv_format,
            SampleDesc: self.sample_desc,
            NodeMask: self.node_mask,
            CachedPSO: self.cached_pso,
            Flags: self.flags,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wrappers() {
        assert_eq!(size_of::<RootSignature>(), size_of::<ID3D12RootSignature>());
        assert_eq!(
            align_of::<RootSignature>(),
            align_of::<ID3D12RootSignature>()
        )
    }

    implement_stream_object!(unsafe u16 => D3D12_PIPELINE_STATE_SUBOBJECT_TYPE(1));
    implement_stream_object!(unsafe u32 => D3D12_PIPELINE_STATE_SUBOBJECT_TYPE(2));
    implement_stream_object!(unsafe u64 => D3D12_PIPELINE_STATE_SUBOBJECT_TYPE(3));

    #[test]
    fn stream() {
        let mut stream = RenderPipelineStateStream::new();

        stream.add_object(42u16);
        stream.add_object(84u32);
        stream.add_object(168u64);

        assert_eq!(stream.bytes.len(), 32);

        // Object 1: u16

        // Tag at the beginning
        assert_eq!(&stream.bytes[0..4], &1u32.to_ne_bytes());
        // Data tucked in, aligned to the natural alignment of u16
        assert_eq!(&stream.bytes[4..6], &42u16.to_ne_bytes());
        // Padding to align the next subobject to an 8 byte boundary.
        assert_eq!(&stream.bytes[6..8], &[0, 0]);

        // Object 2: u32

        // Tag at the beginning
        assert_eq!(&stream.bytes[8..12], &2u32.to_ne_bytes());
        // Data tucked in, aligned to the natural alignment of u32
        assert_eq!(&stream.bytes[12..16], &84u32.to_ne_bytes());

        // Object 3: u64

        // Tag at the beginning
        assert_eq!(&stream.bytes[16..20], &3u32.to_ne_bytes());
        // Padding to align the u64 to an 8 byte boundary.
        assert_eq!(&stream.bytes[20..24], &[0, 0, 0, 0]);
        // Data tucked in, aligned to the natural alignment of u64
        assert_eq!(&stream.bytes[24..32], &168u64.to_ne_bytes());
    }
}
