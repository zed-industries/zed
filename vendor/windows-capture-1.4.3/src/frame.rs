use std::{
    fs::{self},
    io,
    path::Path,
    ptr, slice,
};

use rayon::iter::{IntoParallelIterator, ParallelIterator};
use windows::{
    Foundation::TimeSpan,
    Graphics::DirectX::Direct3D11::IDirect3DSurface,
    Win32::Graphics::{
        Direct3D11::{
            ID3D11Device, ID3D11DeviceContext, ID3D11Texture2D, D3D11_BOX, D3D11_CPU_ACCESS_READ,
            D3D11_CPU_ACCESS_WRITE, D3D11_MAPPED_SUBRESOURCE, D3D11_MAP_READ_WRITE,
            D3D11_TEXTURE2D_DESC, D3D11_USAGE_STAGING,
        },
        Dxgi::Common::{DXGI_FORMAT, DXGI_SAMPLE_DESC},
    },
};

use crate::{
    encoder::{self, ImageEncoder},
    settings::ColorFormat,
};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Invalid box size")]
    InvalidSize,
    #[error("This color format is not supported for saving as image")]
    UnsupportedFormat,
    #[error("Failed to encode image buffer to image bytes with specified format: {0}")]
    ImageEncoderError(#[from] encoder::ImageEncoderError),
    #[error("IO error: {0}")]
    IoError(#[from] io::Error),
    #[error("Windows API error: {0}")]
    WindowsError(#[from] windows::core::Error),
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum ImageFormat {
    Jpeg,
    Png,
    Gif,
    Tiff,
    Bmp,
    JpegXr,
}

/// Represents a frame captured from a graphics capture item.
///
/// # Example
/// ```ignore
/// // Get frame from capture the session
/// let mut buffer = frame.buffer()?;
/// buffer.save_as_image("screenshot.png", ImageFormat::Png)?;
/// ```
pub struct Frame<'a> {
    d3d_device: &'a ID3D11Device,
    frame_surface: IDirect3DSurface,
    frame_texture: ID3D11Texture2D,
    time: TimeSpan,
    context: &'a ID3D11DeviceContext,
    buffer: &'a mut Vec<u8>,
    width: u32,
    height: u32,
    color_format: ColorFormat,
}

impl<'a> Frame<'a> {
    /// Create a new Frame.
    ///
    /// # Arguments
    ///
    /// * `d3d_device` - The ID3D11Device used for creating the frame.
    /// * `frame_surface` - The IDirect3DSurface representing the frame surface.
    /// * `frame_texture` - The ID3D11Texture2D representing the frame texture.
    /// * `time` - The TimeSpan representing the frame time.
    /// * `context` - The ID3D11DeviceContext used for copying the texture.
    /// * `buffer` - The mutable Vec<u8> representing the frame buffer.
    /// * `width` - The width of the frame.
    /// * `height` - The height of the frame.
    /// * `color_format` - The ColorFormat of the frame.
    ///
    /// # Returns
    ///
    /// A new Frame instance.
    #[allow(clippy::too_many_arguments)]
    #[must_use]
    #[inline]
    pub fn new(
        d3d_device: &'a ID3D11Device,
        frame_surface: IDirect3DSurface,
        frame_texture: ID3D11Texture2D,
        time: TimeSpan,
        context: &'a ID3D11DeviceContext,
        buffer: &'a mut Vec<u8>,
        width: u32,
        height: u32,
        color_format: ColorFormat,
    ) -> Self {
        Self {
            d3d_device,
            frame_surface,
            frame_texture,
            time,
            context,
            buffer,
            width,
            height,
            color_format,
        }
    }

    /// Get the width of the frame.
    ///
    /// # Returns
    ///
    /// The width of the frame.
    #[must_use]
    #[inline]
    pub const fn width(&self) -> u32 {
        self.width
    }

    /// Get the height of the frame.
    ///
    /// # Returns
    ///
    /// The height of the frame.
    #[must_use]
    #[inline]
    pub const fn height(&self) -> u32 {
        self.height
    }

    /// Get the time of the frame.
    ///
    /// # Returns
    ///
    /// The time of the frame.
    #[must_use]
    #[inline]
    pub const fn timespan(&self) -> TimeSpan {
        self.time
    }

    /// Get the color format of the frame.
    ///
    /// # Returns
    ///
    /// The color format of the frame.
    #[must_use]
    #[inline]
    pub const fn color_format(&self) -> ColorFormat {
        self.color_format
    }

    /// Get the raw surface of the frame.
    ///
    /// # Returns
    ///
    /// The IDirect3DSurface representing the raw surface of the frame.
    ///
    /// # Safety
    ///
    /// This method is unsafe because it returns a raw pointer to the IDirect3DSurface.
    #[allow(clippy::missing_safety_doc)]
    #[must_use]
    #[inline]
    pub const unsafe fn as_raw_surface(&self) -> &IDirect3DSurface {
        &self.frame_surface
    }

    /// Get the raw texture of the frame.
    ///
    /// # Returns
    ///
    /// The ID3D11Texture2D representing the raw texture of the frame.
    #[allow(clippy::missing_safety_doc)]
    #[must_use]
    #[inline]
    pub const unsafe fn as_raw_texture(&self) -> &ID3D11Texture2D {
        &self.frame_texture
    }

    /// Get the frame buffer.
    ///
    /// # Returns
    ///
    /// The FrameBuffer containing the frame data.
    #[inline]
    pub fn buffer(&mut self) -> Result<FrameBuffer, Error> {
        // Texture Settings
        let texture_desc = D3D11_TEXTURE2D_DESC {
            Width: self.width,
            Height: self.height,
            MipLevels: 1,
            ArraySize: 1,
            Format: DXGI_FORMAT(self.color_format as i32),
            SampleDesc: DXGI_SAMPLE_DESC {
                Count: 1,
                Quality: 0,
            },
            Usage: D3D11_USAGE_STAGING,
            BindFlags: 0,
            CPUAccessFlags: D3D11_CPU_ACCESS_READ.0 as u32 | D3D11_CPU_ACCESS_WRITE.0 as u32,
            MiscFlags: 0,
        };

        // Create a texture that CPU can read
        let mut texture = None;
        unsafe {
            self.d3d_device
                .CreateTexture2D(&texture_desc, None, Some(&mut texture))?;
        };

        let texture = texture.unwrap();

        // Copy the real texture to copy texture
        unsafe {
            self.context.CopyResource(&texture, &self.frame_texture);
        };

        // Map the texture to enable CPU access
        let mut mapped_resource = D3D11_MAPPED_SUBRESOURCE::default();
        unsafe {
            self.context.Map(
                &texture,
                0,
                D3D11_MAP_READ_WRITE,
                0,
                Some(&mut mapped_resource),
            )?;
        };

        // Get the mapped resource data slice
        let mapped_frame_data = unsafe {
            slice::from_raw_parts_mut(
                mapped_resource.pData.cast(),
                (self.height * mapped_resource.RowPitch) as usize,
            )
        };

        // Create frame buffer from slice
        let frame_buffer = FrameBuffer::new(
            mapped_frame_data,
            self.buffer,
            self.width,
            self.height,
            mapped_resource.RowPitch,
            mapped_resource.DepthPitch,
            self.color_format,
        );

        Ok(frame_buffer)
    }

    /// Get a cropped frame buffer.
    ///
    /// # Arguments
    ///
    /// * `start_width` - The starting width of the cropped frame.
    /// * `start_height` - The starting height of the cropped frame.
    /// * `end_width` - The ending width of the cropped frame.
    /// * `end_height` - The ending height of the cropped frame.
    ///
    /// # Returns
    ///
    /// The FrameBuffer containing the cropped frame data.
    #[inline]
    pub fn buffer_crop(
        &mut self,
        start_width: u32,
        start_height: u32,
        end_width: u32,
        end_height: u32,
    ) -> Result<FrameBuffer, Error> {
        if start_width >= end_width || start_height >= end_height {
            return Err(Error::InvalidSize);
        }

        let texture_width = end_width - start_width;
        let texture_height = end_height - start_height;

        // Texture Settings
        let texture_desc = D3D11_TEXTURE2D_DESC {
            Width: texture_width,
            Height: texture_height,
            MipLevels: 1,
            ArraySize: 1,
            Format: DXGI_FORMAT(self.color_format as i32),
            SampleDesc: DXGI_SAMPLE_DESC {
                Count: 1,
                Quality: 0,
            },
            Usage: D3D11_USAGE_STAGING,
            BindFlags: 0,
            CPUAccessFlags: D3D11_CPU_ACCESS_READ.0 as u32 | D3D11_CPU_ACCESS_WRITE.0 as u32,
            MiscFlags: 0,
        };

        // Create a texture that CPU can read
        let mut texture = None;
        unsafe {
            self.d3d_device
                .CreateTexture2D(&texture_desc, None, Some(&mut texture))?;
        };
        let texture = texture.unwrap();

        // Box Settings
        let resource_box = D3D11_BOX {
            left: start_width,
            top: start_height,
            front: 0,
            right: end_width,
            bottom: end_height,
            back: 1,
        };

        // Copy the real texture to copy texture
        unsafe {
            self.context.CopySubresourceRegion(
                &texture,
                0,
                0,
                0,
                0,
                &self.frame_texture,
                0,
                Some(&resource_box),
            );
        };

        // Map the texture to enable CPU access
        let mut mapped_resource = D3D11_MAPPED_SUBRESOURCE::default();
        unsafe {
            self.context.Map(
                &texture,
                0,
                D3D11_MAP_READ_WRITE,
                0,
                Some(&mut mapped_resource),
            )?;
        };

        // Get the mapped resource data slice
        let mapped_frame_data = unsafe {
            slice::from_raw_parts_mut(
                mapped_resource.pData.cast(),
                (texture_height * mapped_resource.RowPitch) as usize,
            )
        };

        // Create frame buffer from slice
        let frame_buffer = FrameBuffer::new(
            mapped_frame_data,
            self.buffer,
            texture_width,
            texture_height,
            mapped_resource.RowPitch,
            mapped_resource.DepthPitch,
            self.color_format,
        );

        Ok(frame_buffer)
    }

    /// Save the frame buffer as an image to the specified path.
    ///
    /// # Arguments
    ///
    /// * `path` - The path where the image will be saved.
    /// * `format` - The ImageFormat of the saved image.
    ///
    /// # Returns
    ///
    /// An empty Result if successful, or an Error if there was an issue saving the image.
    #[inline]
    pub fn save_as_image<T: AsRef<Path>>(
        &mut self,
        path: T,
        format: ImageFormat,
    ) -> Result<(), Error> {
        let mut frame_buffer = self.buffer()?;

        frame_buffer.save_as_image(path, format)?;

        Ok(())
    }
}

/// Represents a frame buffer containing pixel data.
///
/// # Example
/// ```ignore
/// // Get frame from the capture session
/// let mut buffer = frame.buffer()?;
/// buffer.save_as_image("screenshot.png", ImageFormat::Png)?;
/// ```
pub struct FrameBuffer<'a> {
    raw_buffer: &'a mut [u8],
    buffer: &'a mut Vec<u8>,
    width: u32,
    height: u32,
    row_pitch: u32,
    depth_pitch: u32,
    color_format: ColorFormat,
}

impl<'a> FrameBuffer<'a> {
    /// Create a new Frame Buffer.
    ///
    /// # Arguments
    ///
    /// * `raw_buffer` - A mutable reference to the raw pixel data buffer.
    /// * `buffer` - A mutable reference to the buffer used for copying pixel data without padding.
    /// * `width` - The width of the frame buffer.
    /// * `height` - The height of the frame buffer.
    /// * `row_pitch` - The row pitch of the frame buffer.
    /// * `depth_pitch` - The depth pitch of the frame buffer.
    /// * `color_format` - The color format of the frame buffer.
    ///
    /// # Returns
    ///
    /// A new `FrameBuffer` instance.
    #[must_use]
    #[inline]
    pub fn new(
        raw_buffer: &'a mut [u8],
        buffer: &'a mut Vec<u8>,
        width: u32,
        height: u32,
        row_pitch: u32,
        depth_pitch: u32,
        color_format: ColorFormat,
    ) -> Self {
        Self {
            raw_buffer,
            buffer,
            width,
            height,
            row_pitch,
            depth_pitch,
            color_format,
        }
    }

    /// Get the width of the frame buffer.
    #[must_use]
    #[inline]
    pub const fn width(&self) -> u32 {
        self.width
    }

    /// Get the height of the frame buffer.
    #[must_use]
    #[inline]
    pub const fn height(&self) -> u32 {
        self.height
    }

    /// Get the row pitch of the frame buffer.
    #[must_use]
    #[inline]
    pub const fn row_pitch(&self) -> u32 {
        self.row_pitch
    }

    /// Get the depth pitch of the frame buffer.
    #[must_use]
    #[inline]
    pub const fn depth_pitch(&self) -> u32 {
        self.depth_pitch
    }

    /// Check if the buffer has padding.
    #[must_use]
    #[inline]
    pub const fn has_padding(&self) -> bool {
        self.width * 4 != self.row_pitch
    }

    /// Get the raw pixel data with possible padding.
    #[must_use]
    #[inline]
    pub fn as_raw_buffer(&mut self) -> &mut [u8] {
        self.raw_buffer
    }

    /// Get the raw pixel data without padding.
    ///
    /// # Returns
    ///
    /// A mutable reference to the buffer containing pixel data without padding.
    #[inline]
    pub fn as_nopadding_buffer(&mut self) -> Result<&mut [u8], Error> {
        if !self.has_padding() {
            return Ok(self.raw_buffer);
        }

        let multiplyer = match self.color_format {
            ColorFormat::Rgba16F => 8,
            ColorFormat::Rgba8 => 4,
            ColorFormat::Bgra8 => 4,
        };

        let frame_size = (self.width * self.height * multiplyer) as usize;
        if self.buffer.capacity() < frame_size {
            self.buffer.resize(frame_size, 0);
        }

        let width_size = (self.width * multiplyer) as usize;
        let buffer_address = self.buffer.as_mut_ptr() as isize;
        (0..self.height).into_par_iter().for_each(|y| {
            let index = (y * self.row_pitch) as usize;
            let ptr = buffer_address as *mut u8;

            unsafe {
                ptr::copy_nonoverlapping(
                    self.raw_buffer.as_ptr().add(index),
                    ptr.add(y as usize * width_size),
                    width_size,
                );
            }
        });

        Ok(&mut self.buffer[0..frame_size])
    }

    /// Save the frame buffer as an image to the specified path.
    ///
    /// # Arguments
    ///
    /// * `path` - The path where the image will be saved.
    /// * `format` - The image format to use for saving.
    ///
    /// # Returns
    ///
    /// An `Ok` result if the image is successfully saved, or an `Err` result if there was an error.
    #[inline]
    pub fn save_as_image<T: AsRef<Path>>(
        &mut self,
        path: T,
        format: ImageFormat,
    ) -> Result<(), Error> {
        let width = self.width;
        let height = self.height;

        let bytes = ImageEncoder::new(format, self.color_format).encode(
            self.as_nopadding_buffer()?,
            width,
            height,
        )?;

        fs::write(path, bytes)?;

        Ok(())
    }
}
