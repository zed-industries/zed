use gpui::TestAppContext;
use windows::{
    Win32::{
        Foundation::{HMODULE, HWND},
        Graphics::{
            Direct3D::{D3D_DRIVER_TYPE_WARP, D3D_FEATURE_LEVEL_10_1},
            Direct3D11::{D3D11_CREATE_DEVICE_BGRA_SUPPORT, D3D11_SDK_VERSION, D3D11CreateDevice},
            Dxgi::{IDXGIDevice, IDXGIFactory6},
        },
        UI::WindowsAndMessaging::{CreateWindowExW, DestroyWindow, WINDOW_EX_STYLE, WS_POPUP},
    },
    core::{Interface as _, w},
};

#[path = "../../gpui/src/rounded_clip_fixtures.rs"]
mod fixtures;

fn pixel(pixels: &[u8], width: usize, x: usize, y: usize) -> [u8; 4] {
    pixels[(y * width + x) * 4..(y * width + x + 1) * 4]
        .try_into()
        .unwrap()
}
fn assert_same_pixels(actual: &[u8], expected: &[u8], _width: usize) {
    assert_eq!(actual, expected);
}

struct HiddenWindow(HWND);
impl Drop for HiddenWindow {
    fn drop(&mut self) {
        if let Err(error) = unsafe { DestroyWindow(self.0) } {
            log::error!("Destroying rounded overflow test window: {error}");
        }
    }
}

fn renderer() -> (crate::directx_renderer::DirectXRenderer, HiddenWindow) {
    let mut device = None;
    let mut device_context = None;
    unsafe {
        D3D11CreateDevice(
            None,
            D3D_DRIVER_TYPE_WARP,
            HMODULE::default(),
            D3D11_CREATE_DEVICE_BGRA_SUPPORT,
            Some(&[D3D_FEATURE_LEVEL_10_1]),
            D3D11_SDK_VERSION,
            Some(&mut device),
            None,
            Some(&mut device_context),
        )
        .expect("rounded overflow tests require WARP");
    }
    let device = device.unwrap();
    let dxgi_device: IDXGIDevice = device.cast().unwrap();
    let adapter = unsafe { dxgi_device.GetAdapter() }.unwrap();
    let factory: IDXGIFactory6 = unsafe { adapter.GetParent() }.unwrap();
    let devices = crate::directx_devices::DirectXDevices {
        adapter: adapter.cast().unwrap(),
        dxgi_factory: factory,
        device,
        device_context: device_context.unwrap(),
    };
    let window = HiddenWindow(
        unsafe {
            CreateWindowExW(
                WINDOW_EX_STYLE::default(),
                w!("STATIC"),
                w!("Rounded overflow pixel test"),
                WS_POPUP,
                0,
                0,
                128,
                128,
                None,
                None,
                None,
                None,
            )
        }
        .expect("creating hidden test window"),
    );
    let renderer = crate::directx_renderer::DirectXRenderer::new(window.0, &devices, true).unwrap();
    (renderer, window)
}
fn render_div_fixture(cx: &mut TestAppContext, build: fn() -> gpui::Div) -> (Vec<u8>, usize) {
    let (mut renderer, _window) = renderer();
    fixtures::render_with(cx, build, |scene, size| {
        renderer.resize(size).unwrap();
        renderer
            .render_to_image(scene, gpui::WindowBackgroundAppearance::Opaque)
            .unwrap()
            .into_raw()
    })
}

#[gpui::test]
fn rounded_overflow_cached_child_follows_ancestor_clip(cx: &mut TestAppContext) {
    let (mut renderer, _window) = renderer();
    fixtures::retained_frames(cx, |scene, size| {
        renderer.resize(size).unwrap();
        renderer
            .render_to_image(scene, gpui::WindowBackgroundAppearance::Opaque)
            .unwrap()
            .into_raw()
    });
}
