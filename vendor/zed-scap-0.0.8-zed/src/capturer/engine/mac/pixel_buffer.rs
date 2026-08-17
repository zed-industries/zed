use core::slice;
use core_video_sys::{
    CVPixelBufferGetBaseAddress, CVPixelBufferGetBaseAddressOfPlane, CVPixelBufferGetBytesPerRow,
    CVPixelBufferGetBytesPerRowOfPlane, CVPixelBufferGetHeight, CVPixelBufferGetHeightOfPlane,
    CVPixelBufferGetPlaneCount, CVPixelBufferGetWidth, CVPixelBufferGetWidthOfPlane,
    CVPixelBufferLockBaseAddress, CVPixelBufferRef, CVPixelBufferUnlockBaseAddress,
};
use screencapturekit::{cm_sample_buffer::CMSampleBuffer, sc_types::SCFrameStatus};
use screencapturekit_sys::cm_sample_buffer_ref::CMSampleBufferGetImageBuffer;
use std::{ops::Deref, sync::mpsc};

use crate::capturer::{engine::ChannelItem, RawCapturer};

pub struct PixelBuffer {
    display_time: u64,
    width: usize,
    height: usize,
    bytes_per_row: usize,
    buffer: CMSampleBuffer,
}

impl PixelBuffer {
    pub fn display_time(&self) -> u64 {
        self.display_time
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn buffer(&self) -> &CMSampleBuffer {
        &self.buffer
    }

    pub fn bytes_per_row(&self) -> usize {
        self.bytes_per_row
    }

    pub fn data(&self) -> PixelBufferData {
        unsafe {
            let pixel_buffer = sample_buffer_to_pixel_buffer(&self.buffer);

            CVPixelBufferLockBaseAddress(pixel_buffer, 0);

            let base_address = CVPixelBufferGetBaseAddress(pixel_buffer);

            PixelBufferData {
                buffer: pixel_buffer,
                data: slice::from_raw_parts(
                    base_address as *mut _,
                    self.bytes_per_row * self.height,
                ),
            }
        }
    }

    pub fn planes(&self) -> Vec<Plane> {
        unsafe {
            let pixel_buffer = sample_buffer_to_pixel_buffer(&self.buffer);
            let count = CVPixelBufferGetPlaneCount(pixel_buffer);

            CVPixelBufferLockBaseAddress(pixel_buffer, 0);

            (0..count)
                .map(|i| Plane {
                    buffer: pixel_buffer,
                    width: CVPixelBufferGetWidthOfPlane(pixel_buffer, i),
                    height: CVPixelBufferGetHeightOfPlane(pixel_buffer, i),
                    bytes_per_row: CVPixelBufferGetBytesPerRowOfPlane(pixel_buffer, i),
                    index: i,
                })
                .collect()
        }
    }

    pub(crate) fn new(item: ChannelItem) -> Option<Self> {
        unsafe {
            let display_time = pixel_buffer_display_time(&item.0);
            let pixel_buffer = sample_buffer_to_pixel_buffer(&item.0);
            let (width, height) = pixel_buffer_bounds(pixel_buffer);

            match item.0.frame_status {
                SCFrameStatus::Complete | SCFrameStatus::Started | SCFrameStatus::Idle => {
                    Some(Self {
                        display_time,
                        width,
                        height,
                        bytes_per_row: pixel_buffer_bytes_per_row(pixel_buffer),
                        buffer: item.0,
                    })
                }
                _ => None,
            }
        }
    }
}

impl Into<CMSampleBuffer> for PixelBuffer {
    fn into(self) -> CMSampleBuffer {
        self.buffer
    }
}

#[derive(Debug)]
pub struct Plane {
    buffer: CVPixelBufferRef,
    index: usize,
    width: usize,
    height: usize,
    bytes_per_row: usize,
}

impl Plane {
    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn bytes_per_row(&self) -> usize {
        self.bytes_per_row
    }

    pub fn data(&self) -> PixelBufferData {
        unsafe {
            CVPixelBufferLockBaseAddress(self.buffer, 0);

            let base_address = CVPixelBufferGetBaseAddressOfPlane(self.buffer, self.index);

            PixelBufferData {
                buffer: self.buffer,
                data: slice::from_raw_parts(
                    base_address as *mut _,
                    self.bytes_per_row * self.height,
                ),
            }
        }
    }
}

pub struct PixelBufferData<'a> {
    buffer: CVPixelBufferRef,
    data: &'a [u8],
}

impl<'a> Deref for PixelBufferData<'a> {
    type Target = [u8];

    fn deref(&self) -> &'a Self::Target {
        self.data
    }
}

impl<'a> Drop for PixelBufferData<'a> {
    fn drop(&mut self) {
        unsafe { CVPixelBufferUnlockBaseAddress(self.buffer, 0) };
    }
}

impl RawCapturer<'_> {
    #[cfg(target_os = "macos")]
    pub fn get_next_pixel_buffer(&self) -> Result<PixelBuffer, mpsc::RecvError> {
        use std::time::Duration;

        let capturer = &self.capturer;

        loop {
            let error_flag = capturer
                .engine
                .error_flag
                .load(std::sync::atomic::Ordering::Relaxed);
            if error_flag {
                return Err(mpsc::RecvError);
            }

            let res = match capturer.rx.recv_timeout(Duration::from_millis(10)) {
                Ok(v) => Ok(v),
                Err(mpsc::RecvTimeoutError::Timeout) => continue,
                Err(mpsc::RecvTimeoutError::Disconnected) => Err(mpsc::RecvError),
            }?
            .ok();

            if let Some(frame) = PixelBuffer::new(res.unwrap()) {
                return Ok(frame);
            }
        }
    }
}

pub unsafe fn sample_buffer_to_pixel_buffer(sample_buffer: &CMSampleBuffer) -> CVPixelBufferRef {
    let buffer_ref = &(*sample_buffer.sys_ref);

    CMSampleBufferGetImageBuffer(buffer_ref) as CVPixelBufferRef
}

pub unsafe fn pixel_buffer_bounds(pixel_buffer: CVPixelBufferRef) -> (usize, usize) {
    let width = CVPixelBufferGetWidth(pixel_buffer);
    let height = CVPixelBufferGetHeight(pixel_buffer);

    (width, height)
}

pub unsafe fn pixel_buffer_bytes_per_row(pixel_buffer: CVPixelBufferRef) -> usize {
    CVPixelBufferGetBytesPerRow(pixel_buffer)
}

pub unsafe fn pixel_buffer_display_time(sample_buffer: &CMSampleBuffer) -> u64 {
    sample_buffer.sys_ref.get_presentation_timestamp().value as u64
}
