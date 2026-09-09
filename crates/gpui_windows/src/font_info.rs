use anyhow::Result;
use gpui_software::FontCorrection;
use windows::Win32::Graphics::DirectWrite::{
    DWRITE_FACTORY_TYPE_SHARED, DWRITE_PIXEL_GEOMETRY_BGR, DWriteCreateFactory, IDWriteFactory5,
    IDWriteRenderingParams1,
};
use windows::core::Interface;

pub(crate) fn get_font_correction() -> Result<FontCorrection> {
    let factory: IDWriteFactory5 = unsafe { DWriteCreateFactory(DWRITE_FACTORY_TYPE_SHARED)? };
    let render_params: IDWriteRenderingParams1 =
        unsafe { factory.CreateRenderingParams()?.cast()? };
    Ok(FontCorrection {
        gamma_ratios: gpui::get_gamma_correction_ratios(unsafe { render_params.GetGamma() }),
        grayscale_enhanced_contrast: unsafe { render_params.GetGrayscaleEnhancedContrast() },
        subpixel_enhanced_contrast: unsafe { render_params.GetEnhancedContrast() },
        is_bgr: unsafe { render_params.GetPixelGeometry() } == DWRITE_PIXEL_GEOMETRY_BGR,
    })
}
