struct ReusableBuffer {
    raw: blade::Buffer,
    size: u64,
}

pub struct BladeBeltDescriptor {
    pub memory: blade::Memory,
    pub min_chunk_size: u64,
}

/// A belt of buffers, used by the BladeAtlas to cheaply
/// find staging space for uploads.
pub struct BladeBelt {
    desc: BladeBeltDescriptor,
    buffers: Vec<(ReusableBuffer, blade::SyncPoint)>,
    active: Vec<(ReusableBuffer, u64)>,
}

impl BladeBelt {
    pub fn new(desc: BladeBeltDescriptor) -> Self {
        Self {
            desc,
            buffers: Vec::new(),
            active: Vec::new(),
        }
    }

    pub fn destroy(&mut self, gpu: &blade::Context) {
        for (buffer, _) in self.buffers.drain(..) {
            gpu.destroy_buffer(buffer.raw);
        }
        for (buffer, _) in self.active.drain(..) {
            gpu.destroy_buffer(buffer.raw);
        }
    }

    pub fn alloc(&mut self, size: u64, gpu: &blade::Context) -> blade::BufferPiece {
        for &mut (ref rb, ref mut offset) in self.active.iter_mut() {
            if *offset + size <= rb.size {
                let piece = rb.raw.at(*offset);
                *offset += size;
                return piece;
            }
        }

        let index_maybe = self
            .buffers
            .iter()
            .position(|&(ref rb, ref sp)| size <= rb.size && gpu.wait_for(sp, 0));
        if let Some(index) = index_maybe {
            let (rb, _) = self.buffers.remove(index);
            let piece = rb.raw.into();
            self.active.push((rb, size));
            return piece;
        }

        let chunk_index = self.buffers.len() + self.active.len();
        let chunk_size = size.max(self.desc.min_chunk_size);
        let chunk = gpu.create_buffer(blade::BufferDesc {
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

    pub fn alloc_data(&mut self, data: &[u8], gpu: &blade::Context) -> blade::BufferPiece {
        let bp = self.alloc(data.len() as u64, gpu);
        unsafe {
            std::ptr::copy_nonoverlapping(data.as_ptr(), bp.data(), data.len());
        }
        bp
    }

    pub fn flush(&mut self, sp: &blade::SyncPoint) {
        self.buffers
            .extend(self.active.drain(..).map(|(rb, _)| (rb, sp.clone())));
    }
}
