use std::cell::{Cell, RefCell};
use std::path::Path;
use std::time::Duration;

use anyhow::{Context as _, Result, anyhow};
use windows::Win32::Foundation::{HMODULE, RECT, S_OK};
use windows::Win32::Graphics::Direct3D::D3D_DRIVER_TYPE_HARDWARE;
use windows::Win32::Graphics::Direct3D11::{
    D3D11_BIND_RENDER_TARGET, D3D11_BIND_SHADER_RESOURCE, D3D11_CPU_ACCESS_READ,
    D3D11_CREATE_DEVICE_BGRA_SUPPORT, D3D11_CREATE_DEVICE_VIDEO_SUPPORT, D3D11_MAP_READ,
    D3D11_MAPPED_SUBRESOURCE, D3D11_SDK_VERSION, D3D11_TEXTURE2D_DESC, D3D11_USAGE_DEFAULT,
    D3D11_USAGE_STAGING, D3D11CreateDevice, ID3D11Device, ID3D11DeviceContext, ID3D11Multithread,
    ID3D11Texture2D,
};
use windows::Win32::Graphics::Dxgi::Common::{DXGI_FORMAT_B8G8R8A8_UNORM, DXGI_SAMPLE_DESC};
use windows::Win32::Media::MediaFoundation::{
    CLSID_MFMediaEngineClassFactory, IMFAttributes, IMFDXGIDeviceManager, IMFMediaEngine,
    IMFMediaEngineClassFactory, IMFMediaEngineNotify, IMFMediaEngineNotify_Impl,
    MF_MEDIA_ENGINE_CALLBACK, MF_MEDIA_ENGINE_DXGI_MANAGER, MF_MEDIA_ENGINE_READY_HAVE_METADATA,
    MF_MEDIA_ENGINE_VIDEO_OUTPUT_FORMAT, MF_VERSION, MFCreateAttributes, MFCreateDXGIDeviceManager,
    MFSTARTUP_FULL, MFShutdown, MFStartup,
};
use windows::Win32::System::Com::{CLSCTX_INPROC_SERVER, CoCreateInstance};
use windows::core::{BSTR, Interface, implement};

use crate::PlayerState;

/// A decoded frame in GPUI's image layout: BGRA rows of `width * 4` bytes.
pub struct Frame {
    pub width: u32,
    pub height: u32,
    pub bgra: Vec<u8>,
}

/// Plays a video file with the Media Foundation media engine in frame-server mode: the engine
/// decodes on the GPU and outputs the audio, and each new frame is copied out for GPUI, which
/// has no way to draw a Direct3D texture directly.
pub struct Player {
    engine: IMFMediaEngine,
    device: ID3D11Device,
    context: ID3D11DeviceContext,
    _device_manager: IMFDXGIDeviceManager,
    textures: RefCell<Option<FrameTextures>>,
    last_frame_time: Cell<Option<i64>>,
}

struct FrameTextures {
    width: u32,
    height: u32,
    render_target: ID3D11Texture2D,
    staging: ID3D11Texture2D,
}

/// The engine requires a callback; state is polled instead, so events are ignored.
#[implement(IMFMediaEngineNotify)]
struct IgnoreEvents;

impl IMFMediaEngineNotify_Impl for IgnoreEvents_Impl {
    fn EventNotify(&self, _event: u32, _param1: usize, _param2: u32) -> windows::core::Result<()> {
        Ok(())
    }
}

impl Player {
    pub fn open(path: &Path) -> Result<Self> {
        let url = url::Url::from_file_path(path)
            .map_err(|()| anyhow!("{} is not an absolute path", path.display()))?;
        // SAFETY: every out-pointer passed below points to a live local, and the COM objects are
        // used only through the interfaces the media engine documents for frame-server mode.
        unsafe {
            MFStartup(MF_VERSION, MFSTARTUP_FULL).context("starting Media Foundation")?;
            let result = Self::create(&url);
            if result.is_err() {
                log_error(MFShutdown().context("stopping Media Foundation"));
            }
            result
        }
    }

    unsafe fn create(url: &url::Url) -> Result<Self> {
        // SAFETY: see `open`.
        unsafe {
            let mut device = None;
            let mut context = None;
            D3D11CreateDevice(
                None,
                D3D_DRIVER_TYPE_HARDWARE,
                HMODULE::default(),
                D3D11_CREATE_DEVICE_VIDEO_SUPPORT | D3D11_CREATE_DEVICE_BGRA_SUPPORT,
                None,
                D3D11_SDK_VERSION,
                Some(&mut device),
                None,
                Some(&mut context),
            )
            .context("creating a Direct3D 11 device for video decoding")?;
            let device: ID3D11Device = device.context("Direct3D returned no device")?;
            let context: ID3D11DeviceContext = context.context("Direct3D returned no context")?;
            // Media Foundation decodes on its own threads with the same device.
            let _was_protected = device
                .cast::<ID3D11Multithread>()?
                .SetMultithreadProtected(true);

            let mut reset_token = 0;
            let mut device_manager = None;
            MFCreateDXGIDeviceManager(&mut reset_token, &mut device_manager)?;
            let device_manager: IMFDXGIDeviceManager =
                device_manager.context("Media Foundation returned no device manager")?;
            device_manager.ResetDevice(&device, reset_token)?;

            let mut attributes = None;
            MFCreateAttributes(&mut attributes, 3)?;
            let attributes: IMFAttributes =
                attributes.context("Media Foundation returned no attributes")?;
            let callback: IMFMediaEngineNotify = IgnoreEvents.into();
            attributes.SetUnknown(&MF_MEDIA_ENGINE_CALLBACK, &callback)?;
            attributes.SetUnknown(&MF_MEDIA_ENGINE_DXGI_MANAGER, &device_manager)?;
            attributes.SetUINT32(
                &MF_MEDIA_ENGINE_VIDEO_OUTPUT_FORMAT,
                DXGI_FORMAT_B8G8R8A8_UNORM.0 as u32,
            )?;

            let factory: IMFMediaEngineClassFactory =
                CoCreateInstance(&CLSID_MFMediaEngineClassFactory, None, CLSCTX_INPROC_SERVER)?;
            let engine = factory.CreateInstance(0, &attributes)?;
            engine.SetSource(&BSTR::from(url.as_str()))?;
            Ok(Self {
                engine,
                device,
                context,
                _device_manager: device_manager,
                textures: RefCell::new(None),
                last_frame_time: Cell::new(None),
            })
        }
    }

    pub fn state(&self) -> PlayerState {
        // SAFETY: the engine is a live COM object.
        unsafe {
            if let Ok(error) = self.engine.GetError() {
                return PlayerState::Failed(match error.GetExtendedErrorCode() {
                    Err(extended) => {
                        format!("The video cannot be played: {}", extended.message())
                    }
                    Ok(()) => format!(
                        "The video cannot be played (media engine error {})",
                        error.GetErrorCode()
                    ),
                });
            }
            if self.engine.GetReadyState() >= MF_MEDIA_ENGINE_READY_HAVE_METADATA.0 as u16 {
                PlayerState::Ready
            } else {
                PlayerState::Loading
            }
        }
    }

    pub fn duration(&self) -> Option<Duration> {
        // SAFETY: the engine is a live COM object.
        let seconds = unsafe { self.engine.GetDuration() };
        (seconds.is_finite() && seconds >= 0.0).then(|| Duration::from_secs_f64(seconds))
    }

    pub fn has_video(&self) -> bool {
        // SAFETY: the engine is a live COM object.
        unsafe { self.engine.HasVideo().as_bool() }
    }

    pub fn position(&self) -> Duration {
        // SAFETY: the engine is a live COM object.
        let seconds = unsafe { self.engine.GetCurrentTime() };
        Duration::from_secs_f64(seconds.max(0.0))
    }

    pub fn is_playing(&self) -> bool {
        // SAFETY: the engine is a live COM object.
        unsafe { !self.engine.IsPaused().as_bool() && !self.engine.IsEnded().as_bool() }
    }

    pub fn is_finished(&self) -> bool {
        // SAFETY: the engine is a live COM object.
        unsafe { self.engine.IsEnded().as_bool() }
    }

    pub fn play(&self) {
        if self.is_finished() {
            self.seek(Duration::ZERO);
        }
        // SAFETY: the engine is a live COM object.
        log_error(unsafe { self.engine.Play() }.context("starting video playback"));
    }

    pub fn pause(&self) {
        // SAFETY: the engine is a live COM object.
        log_error(unsafe { self.engine.Pause() }.context("pausing video playback"));
    }

    pub fn seek(&self, position: Duration) {
        // SAFETY: the engine is a live COM object.
        log_error(
            unsafe { self.engine.SetCurrentTime(position.as_secs_f64()) }
                .context("seeking in the video"),
        );
    }

    pub fn set_volume(&self, volume: f32) {
        // SAFETY: the engine is a live COM object.
        log_error(
            unsafe { self.engine.SetVolume(f64::from(volume)) }.context("setting the video volume"),
        );
    }

    /// Returns the frame due now if it differs from the last returned frame.
    pub fn take_frame(&self) -> Option<Frame> {
        // SAFETY: the engine and textures are live COM objects; the mapped pointer is read only
        // within the bounds Direct3D reports for the staging texture and before `Unmap`.
        unsafe {
            if !self.has_video() {
                return None;
            }
            // The generated wrapper maps S_FALSE ("no new frame") to Ok with a zero time, so
            // the HRESULT is read from the vtable call directly.
            let mut frame_time = 0;
            let status = (Interface::vtable(&self.engine).OnVideoStreamTick)(
                Interface::as_raw(&self.engine),
                &mut frame_time,
            );
            if status != S_OK || self.last_frame_time.get() == Some(frame_time) {
                return None;
            }
            let mut width = 0;
            let mut height = 0;
            self.engine
                .GetNativeVideoSize(Some(&mut width), Some(&mut height))
                .ok()?;
            if width == 0 || height == 0 {
                return None;
            }
            let mut textures = self.textures.borrow_mut();
            if textures
                .as_ref()
                .is_none_or(|textures| (textures.width, textures.height) != (width, height))
            {
                *textures = Some(self.create_textures(width, height).ok()?);
            }
            let textures = textures.as_ref()?;
            let target = RECT {
                left: 0,
                top: 0,
                right: width as i32,
                bottom: height as i32,
            };
            self.engine
                .TransferVideoFrame(&textures.render_target, None, &target, None)
                .ok()?;
            self.last_frame_time.set(Some(frame_time));

            self.context
                .CopyResource(&textures.staging, &textures.render_target);
            let mut mapped = D3D11_MAPPED_SUBRESOURCE::default();
            self.context
                .Map(&textures.staging, 0, D3D11_MAP_READ, 0, Some(&mut mapped))
                .ok()?;
            let row_length = width as usize * 4;
            let mut bgra = Vec::with_capacity(row_length * height as usize);
            for row in 0..height as usize {
                let source = mapped
                    .pData
                    .cast::<u8>()
                    .add(row * mapped.RowPitch as usize);
                bgra.extend_from_slice(std::slice::from_raw_parts(source, row_length));
            }
            self.context.Unmap(&textures.staging, 0);
            // The engine leaves the alpha channel undefined; GPUI blends with it.
            for pixel in bgra.chunks_exact_mut(4) {
                pixel[3] = u8::MAX;
            }
            Some(Frame {
                width,
                height,
                bgra,
            })
        }
    }

    unsafe fn create_textures(&self, width: u32, height: u32) -> Result<FrameTextures> {
        let mut description = D3D11_TEXTURE2D_DESC {
            Width: width,
            Height: height,
            MipLevels: 1,
            ArraySize: 1,
            Format: DXGI_FORMAT_B8G8R8A8_UNORM,
            SampleDesc: DXGI_SAMPLE_DESC {
                Count: 1,
                Quality: 0,
            },
            Usage: D3D11_USAGE_DEFAULT,
            BindFlags: (D3D11_BIND_RENDER_TARGET.0 | D3D11_BIND_SHADER_RESOURCE.0) as u32,
            CPUAccessFlags: 0,
            MiscFlags: 0,
        };
        // SAFETY: the descriptions are fully initialized and the out-pointers are live locals.
        unsafe {
            let mut render_target = None;
            self.device
                .CreateTexture2D(&description, None, Some(&mut render_target))?;
            description.Usage = D3D11_USAGE_STAGING;
            description.BindFlags = 0;
            description.CPUAccessFlags = D3D11_CPU_ACCESS_READ.0 as u32;
            let mut staging = None;
            self.device
                .CreateTexture2D(&description, None, Some(&mut staging))?;
            Ok(FrameTextures {
                width,
                height,
                render_target: render_target.context("Direct3D returned no texture")?,
                staging: staging.context("Direct3D returned no texture")?,
            })
        }
    }
}

impl Drop for Player {
    fn drop(&mut self) {
        // SAFETY: the engine is a live COM object; MFShutdown balances the MFStartup in `open`.
        unsafe {
            log_error(
                self.engine
                    .Shutdown()
                    .context("shutting down the media engine"),
            );
            log_error(MFShutdown().context("stopping Media Foundation"));
        }
    }
}

fn log_error(result: Result<()>) {
    if let Err(error) = result {
        log::error!("{error:#}");
    }
}
