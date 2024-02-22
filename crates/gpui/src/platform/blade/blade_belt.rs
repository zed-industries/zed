use blade_graphics as gpu;
use std::mem;

struct ReusableBuffer {
    raw: gpu::Buffer,
    size: u64,
}

pub struct BladeBeltDescriptor {
    pub memory: gpu::Memory,
    pub min_chunk_size: u64,
    pub alignment: u64,
}

/// A belt of buffers, used by the BladeAtlas to cheaply
/// find staging space for uploads.
pub struct BladeBelt {
    desc: BladeBeltDescriptor,
    buffers: Vec<(ReusableBuffer, gpu::SyncPoint)>,
    active: Vec<(ReusableBuffer, u64)>,
}

impl BladeBelt {
    pub fn new(desc: BladeBeltDescriptor) -> Self {
        assert_ne!(desc.alignment, 0);
        Self {
            desc,
            buffers: Vec::new(),
            active: Vec::new(),
        }
    }

    pub fn destroy(&mut self, gpu: &gpu::Context) {
        for (buffer, _) in self.buffers.drain(..) {
            gpu.destroy_buffer(buffer.raw);
        }
        for (buffer, _) in self.active.drain(..) {
            gpu.destroy_buffer(buffer.raw);
        }
    }

    #[profiling::function]
    pub fn alloc(&mut self, size: u64, gpu: &gpu::Context) -> gpu::BufferPiece {
        for &mut (ref rb, ref mut offset) in self.active.iter_mut() {
            let aligned = offset.next_multiple_of(self.desc.alignment);
            if aligned + size <= rb.size {
                let piece = rb.raw.at(aligned);
                *offset = aligned + size;
                return piece;
            }
        }

        let index_maybe = self
            .buffers
            .iter()
            .position(|(rb, sp)| size <= rb.size && gpu.wait_for(sp, 0));
        if let Some(index) = index_maybe {
            let (rb, _) = self.buffers.remove(index);
            let piece = rb.raw.into();
            self.active.push((rb, size));
            return piece;
        }

        let chunk_index = self.buffers.len() + self.active.len();
        let chunk_size = size.max(self.desc.min_chunk_size);
        let chunk = gpu.create_buffer(gpu::BufferDesc {
            name: &format!("chunk-{}", chunk_index),
            size: chunk_size,
            memory: self.desc.memory,
        });
        let rb = ReusableBuffer {
            raw: chunk,
            size: chunk_size,
        };
        self.active.push((rb, size));
        chunk.into()
    }

    // SAFETY: T should be zeroable and ordinary data, no references, pointers, cells or other complicated data type.
    pub unsafe fn alloc_data<T>(&mut self, data: &[T], gpu: &gpu::Context) -> gpu::BufferPiece {
        assert!(!data.is_empty());
        let type_alignment = mem::align_of::<T>() as u64;
        debug_assert_eq!(
            self.desc.alignment % type_alignment,
            0,
            "Type alignment {} is too big",
            type_alignment
        );
        let total_bytes = std::mem::size_of_val(data);
        let bp = self.alloc(total_bytes as u64, gpu);
        unsafe {
            std::ptr::copy_nonoverlapping(data.as_ptr() as *const u8, bp.data(), total_bytes);
        }
        bp
    }

    pub fn flush(&mut self, sp: &gpu::SyncPoint) {
        self.buffers
            .extend(self.active.drain(..).map(|(rb, _)| (rb, sp.clone())));
    }
}
