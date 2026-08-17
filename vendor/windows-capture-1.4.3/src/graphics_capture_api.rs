use std::sync::{
    Arc,
    atomic::{self, AtomicBool},
};

use parking_lot::Mutex;
use windows::{
    Foundation::{Metadata::ApiInformation, TypedEventHandler},
    Graphics::{
        Capture::{Direct3D11CaptureFramePool, GraphicsCaptureItem, GraphicsCaptureSession},
        DirectX::{Direct3D11::IDirect3DDevice, DirectXPixelFormat},
    },
    Win32::{
        Foundation::{LPARAM, WPARAM},
        Graphics::Direct3D11::{
            D3D11_TEXTURE2D_DESC, ID3D11Device, ID3D11DeviceContext, ID3D11Texture2D,
        },
        System::WinRT::Direct3D11::IDirect3DDxgiInterfaceAccess,
        UI::WindowsAndMessaging::{PostThreadMessageW, WM_QUIT},
    },
    core::{HSTRING, IInspectable, Interface},
};

use crate::{
    capture::GraphicsCaptureApiHandler,
    d3d11::{self, SendDirectX, create_direct3d_device},
    frame::Frame,
    settings::{ColorFormat, CursorCaptureSettings, DrawBorderSettings},
};

#[derive(thiserror::Error, Eq, PartialEq, Clone, Debug)]
pub enum Error {
    #[error("Graphics capture API is not supported")]
    Unsupported,
    #[error("Graphics capture API toggling cursor capture is not supported")]
    CursorConfigUnsupported,
    #[error("Graphics capture API toggling border capture is not supported")]
    BorderConfigUnsupported,
    #[error("Already started")]
    AlreadyStarted,
    #[error("DirectX error: {0}")]
    DirectXError(#[from] d3d11::Error),
    #[error("Windows API error: {0}")]
    WindowsError(#[from] windows::core::Error),
}

/// Used to control the capture session
pub struct InternalCaptureControl {
    stop: Arc<AtomicBool>,
}

impl InternalCaptureControl {
    /// Create a new `InternalCaptureControl` struct.
    ///
    /// # Arguments
    ///
    /// * `stop` - An `Arc<AtomicBool>` indicating whether the capture should stop.
    ///
    /// # Returns
    ///
    /// A new instance of `InternalCaptureControl`.
    #[must_use]
    #[inline]
    pub const fn new(stop: Arc<AtomicBool>) -> Self {
        Self { stop }
    }

    /// Gracefully stop the capture thread.
    #[inline]
    pub fn stop(self) {
        self.stop.store(true, atomic::Ordering::Relaxed);
    }
}

/// Represents the GraphicsCaptureApi struct.
pub struct GraphicsCaptureApi {
    /// The GraphicsCaptureItem associated with the GraphicsCaptureApi.
    item: GraphicsCaptureItem,
    /// The ID3D11Device associated with the GraphicsCaptureApi.
    _d3d_device: ID3D11Device,
    /// The IDirect3DDevice associated with the GraphicsCaptureApi.
    _direct3d_device: IDirect3DDevice,
    /// The ID3D11DeviceContext associated with the GraphicsCaptureApi.
    _d3d_device_context: ID3D11DeviceContext,
    /// The optional Arc<Direct3D11CaptureFramePool> associated with the GraphicsCaptureApi.
    frame_pool: Option<Arc<Direct3D11CaptureFramePool>>,
    /// The optional GraphicsCaptureSession associated with the GraphicsCaptureApi.
    session: Option<GraphicsCaptureSession>,
    /// The Arc<AtomicBool> used to halt the GraphicsCaptureApi.
    halt: Arc<AtomicBool>,
    /// Indicates whether the GraphicsCaptureApi is active or not.
    active: bool,
    /// The EventRegistrationToken associated with the capture closed event.
    capture_closed_event_token: i64,
    /// The EventRegistrationToken associated with the frame arrived event.
    frame_arrived_event_token: i64,
}

impl GraphicsCaptureApi {
    /// Create a new Graphics Capture API struct.
    ///
    /// # Arguments
    ///
    /// * `d3d_device` - The ID3D11Device to use for the capture.
    /// * `d3d_device_context` - The ID3D11DeviceContext to use for the capture.
    /// * `item` - The graphics capture item to capture.
    /// * `callback` - The callback handler for capturing frames.
    /// * `capture_cursor` - Optional flag to capture the cursor.
    /// * `draw_border` - Optional flag to draw a border around the captured region.
    /// * `color_format` - The color format for the captured frames.
    /// * `thread_id` - The ID of the thread where the capture is running.
    /// * `result` - The result of the capture operation.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the new `GraphicsCaptureApi` struct if successful, or an `Error` if an error occurred.
    #[allow(clippy::too_many_arguments)]
    #[inline]
    pub fn new<
        T: GraphicsCaptureApiHandler<Error = E> + Send + 'static,
        E: Send + Sync + 'static,
    >(
        d3d_device: ID3D11Device,
        d3d_device_context: ID3D11DeviceContext,
        item: GraphicsCaptureItem,
        callback: Arc<Mutex<T>>,
        cursor_capture: CursorCaptureSettings,
        draw_border: DrawBorderSettings,
        color_format: ColorFormat,
        thread_id: u32,
        result: Arc<Mutex<Option<E>>>,
    ) -> Result<Self, Error> {
        // Check support
        if !Self::is_supported()? {
            return Err(Error::Unsupported);
        }

        if cursor_capture != CursorCaptureSettings::Default
            && !Self::is_cursor_settings_supported()?
        {
            return Err(Error::CursorConfigUnsupported);
        }

        if draw_border != DrawBorderSettings::Default && !Self::is_border_settings_supported()? {
            return Err(Error::BorderConfigUnsupported);
        }

        // Create DirectX devices
        let direct3d_device = create_direct3d_device(&d3d_device)?;

        let pixel_format = DirectXPixelFormat(color_format as i32);

        // Create frame pool
        let frame_pool =
            Direct3D11CaptureFramePool::Create(&direct3d_device, pixel_format, 1, item.Size()?)?;
        let frame_pool = Arc::new(frame_pool);

        // Create capture session
        let session = frame_pool.CreateCaptureSession(&item)?;

        // Preallocate memory
        let mut buffer = vec![0u8; 3840 * 2160 * 4];

        // Indicates if the capture is closed
        let halt = Arc::new(AtomicBool::new(false));

        // Set capture session closed event
        let capture_closed_event_token = item.Closed(&TypedEventHandler::<
            GraphicsCaptureItem,
            IInspectable,
        >::new({
            // Init
            let callback_closed = callback.clone();
            let halt_closed = halt.clone();
            let result_closed = result.clone();

            move |_, _| {
                halt_closed.store(true, atomic::Ordering::Relaxed);

                // Notify the struct that the capture session is closed
                let callback_closed = callback_closed.lock().on_closed();
                if let Err(e) = callback_closed {
                    *result_closed.lock() = Some(e);
                }

                // To stop message loop
                unsafe {
                    PostThreadMessageW(thread_id, WM_QUIT, WPARAM::default(), LPARAM::default())?;
                };

                Result::Ok(())
            }
        }))?;

        // Set frame pool frame arrived event
        let frame_arrived_event_token = frame_pool.FrameArrived(&TypedEventHandler::<
            Direct3D11CaptureFramePool,
            IInspectable,
        >::new({
            // Init
            let frame_pool_recreate = frame_pool.clone();
            let halt_frame_pool = halt.clone();
            let d3d_device_frame_pool = d3d_device.clone();
            let context = d3d_device_context.clone();
            let result_frame_pool = result;

            let mut last_size = item.Size()?;
            let callback_frame_pool = callback;
            let direct3d_device_recreate = SendDirectX::new(direct3d_device.clone());

            move |frame, _| {
                // Return early if the capture is closed
                if halt_frame_pool.load(atomic::Ordering::Relaxed) {
                    return Ok(());
                }

                // Get frame
                let frame = frame
                    .as_ref()
                    .expect("FrameArrived parameter was None this should never happen.")
                    .TryGetNextFrame()?;
                let timespan = frame.SystemRelativeTime()?;

                // Get frame content size
                let frame_content_size = frame.ContentSize()?;

                // Get frame surface
                let frame_surface = frame.Surface()?;

                // Convert surface to texture
                let frame_dxgi_interface = frame_surface.cast::<IDirect3DDxgiInterfaceAccess>()?;
                let frame_texture =
                    unsafe { frame_dxgi_interface.GetInterface::<ID3D11Texture2D>()? };

                // Get texture settings
                let mut desc = D3D11_TEXTURE2D_DESC::default();
                unsafe { frame_texture.GetDesc(&mut desc) }

                // Check if the size has been changed
                if frame_content_size.Width != last_size.Width
                    || frame_content_size.Height != last_size.Height
                {
                    let direct3d_device_recreate = &direct3d_device_recreate;
                    frame_pool_recreate.Recreate(
                        &direct3d_device_recreate.0,
                        pixel_format,
                        1,
                        frame_content_size,
                    )?;

                    last_size = frame_content_size;

                    return Ok(());
                }

                // Set width & height
                let texture_width = desc.Width;
                let texture_height = desc.Height;

                // Create a frame
                let mut frame = Frame::new(
                    &d3d_device_frame_pool,
                    frame_surface,
                    frame_texture,
                    timespan,
                    &context,
                    &mut buffer,
                    texture_width,
                    texture_height,
                    color_format,
                );

                // Init internal capture control
                let stop = Arc::new(AtomicBool::new(false));
                let internal_capture_control = InternalCaptureControl::new(stop.clone());

                // Send the frame to the callback struct
                let result = callback_frame_pool
                    .lock()
                    .on_frame_arrived(&mut frame, internal_capture_control);

                if stop.load(atomic::Ordering::Relaxed) || result.is_err() {
                    if let Err(e) = result {
                        *result_frame_pool.lock() = Some(e);
                    }

                    halt_frame_pool.store(true, atomic::Ordering::Relaxed);

                    // To stop the message loop
                    unsafe {
                        PostThreadMessageW(
                            thread_id,
                            WM_QUIT,
                            WPARAM::default(),
                            LPARAM::default(),
                        )?;
                    };
                }

                Result::Ok(())
            }
        }))?;

        if cursor_capture != CursorCaptureSettings::Default {
            if Self::is_cursor_settings_supported()? {
                match cursor_capture {
                    CursorCaptureSettings::Default => (),
                    CursorCaptureSettings::WithCursor => session.SetIsCursorCaptureEnabled(true)?,
                    CursorCaptureSettings::WithoutCursor => {
                        session.SetIsCursorCaptureEnabled(false)?
                    }
                };
            } else {
                return Err(Error::CursorConfigUnsupported);
            }
        }

        if draw_border != DrawBorderSettings::Default {
            if Self::is_border_settings_supported()? {
                match draw_border {
                    DrawBorderSettings::Default => (),
                    DrawBorderSettings::WithBorder => {
                        session.SetIsBorderRequired(true)?;
                    }
                    DrawBorderSettings::WithoutBorder => session.SetIsBorderRequired(false)?,
                }
            } else {
                return Err(Error::BorderConfigUnsupported);
            }
        }

        Ok(Self {
            item,
            _d3d_device: d3d_device,
            _direct3d_device: direct3d_device,
            _d3d_device_context: d3d_device_context,
            frame_pool: Some(frame_pool),
            session: Some(session),
            halt,
            active: false,
            frame_arrived_event_token,
            capture_closed_event_token,
        })
    }

    /// Start the capture.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the capture started successfully, or an `Error` if an error occurred.
    #[inline]
    pub fn start_capture(&mut self) -> Result<(), Error> {
        if self.active {
            return Err(Error::AlreadyStarted);
        }
        self.active = true;

        self.session.as_ref().unwrap().StartCapture()?;

        Ok(())
    }

    /// Stop the capture.
    #[inline]
    pub fn stop_capture(mut self) {
        if let Some(frame_pool) = self.frame_pool.take() {
            frame_pool
                .RemoveFrameArrived(self.frame_arrived_event_token)
                .expect("Failed to remove Frame Arrived event handler");

            frame_pool.Close().expect("Failed to Close Frame Pool");
        }

        if let Some(session) = self.session.take() {
            session.Close().expect("Failed to Close Capture Session");
        }

        self.item
            .RemoveClosed(self.capture_closed_event_token)
            .expect("Failed to remove Capture Session Closed event handler");
    }

    /// Get the halt handle.
    ///
    /// # Returns
    ///
    /// Returns an `Arc<AtomicBool>` representing the halt handle.
    #[must_use]
    #[inline]
    pub fn halt_handle(&self) -> Arc<AtomicBool> {
        self.halt.clone()
    }

    /// Check if the Windows Graphics Capture API is supported.
    ///
    /// # Returns
    ///
    /// Returns `Ok(true)` if the API is supported, `Ok(false)` if the API is not supported, or an `Error` if an error occurred.
    #[inline]
    pub fn is_supported() -> Result<bool, Error> {
        Ok(ApiInformation::IsApiContractPresentByMajor(
            &HSTRING::from("Windows.Foundation.UniversalApiContract"),
            8,
        )? && GraphicsCaptureSession::IsSupported()?)
    }

    /// Check if you can change the cursor capture setting.
    ///
    /// # Returns
    ///
    /// Returns `true` if toggling the cursor capture is supported, `false` otherwise.
    #[inline]
    pub fn is_cursor_settings_supported() -> Result<bool, Error> {
        Ok(ApiInformation::IsPropertyPresent(
            &HSTRING::from("Windows.Graphics.Capture.GraphicsCaptureSession"),
            &HSTRING::from("IsCursorCaptureEnabled"),
        )? && Self::is_supported()?)
    }

    /// Check if you can change the border capture setting.
    ///
    /// # Returns
    ///
    /// Returns `true` if toggling the border capture is supported, `false` otherwise.
    #[inline]
    pub fn is_border_settings_supported() -> Result<bool, Error> {
        Ok(ApiInformation::IsPropertyPresent(
            &HSTRING::from("Windows.Graphics.Capture.GraphicsCaptureSession"),
            &HSTRING::from("IsBorderRequired"),
        )? && Self::is_supported()?)
    }
}

impl Drop for GraphicsCaptureApi {
    fn drop(&mut self) {
        if let Some(frame_pool) = self.frame_pool.take() {
            frame_pool
                .RemoveFrameArrived(self.frame_arrived_event_token)
                .expect("Failed to remove Frame Arrived event handler");

            frame_pool.Close().expect("Failed to Close Frame Pool");
        }

        if let Some(session) = self.session.take() {
            session.Close().expect("Failed to Close Capture Session");
        }

        self.item
            .RemoveClosed(self.capture_closed_event_token)
            .expect("Failed to remove Capture Session Closed event handler");
    }
}
