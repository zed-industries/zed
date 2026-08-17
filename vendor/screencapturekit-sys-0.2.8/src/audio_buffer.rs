#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub(crate) struct AudioBuffer {
    pub number_channels: u32,
    pub data_bytes_size: u32,
    pub data: *mut u8,
}

const MAX_AUDIO_BUFFERS: usize = 8;

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub(crate) struct AudioBufferList {
    pub number_buffers: u32,
    pub buffers: [AudioBuffer; MAX_AUDIO_BUFFERS],
}

pub struct CopiedAudioBuffer {
    pub number_channels: u32,
    pub data: Vec<u8>,
}

#[allow(non_upper_case_globals)]
pub const kCMSampleBufferFlag_AudioBufferList_Assure16ByteAlignment: u32 = 1 << 0;
