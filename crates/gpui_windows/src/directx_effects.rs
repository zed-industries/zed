//! Effect layers for the DirectX renderer.
//!
//! An effect layer is a subtree of the scene that the renderer must paint
//! with a blur, a colour matrix, a mask or a blend mode. The scene marks
//! the layer with `LayerBegin` and `LayerEnd`. Between the two marks the
//! renderer draws into a texture the size of the whole frame, because the
//! pixel shaders read `SV_Position` in render-target pixels and a smaller
//! texture would shift every rounded corner and gradient. At `LayerEnd`
//! this module copies the region the layer touches out of that texture and
//! out of the parent target, blurs one or both, and paints the result back
//! into the parent with the composite pipeline.
//!
//! Every texture holds premultiplied colour in the frame format. The
//! composite pipeline runs with blending off and mixes with the pixel under
//! it in the shader, so it needs a copy of the parent region.

use std::slice;

use anyhow::{Context, Result};
use windows::Win32::Graphics::{Direct3D11::*, Dxgi::Common::*};

use crate::directx_renderer::{
    PipelineState, RENDER_TARGET_FORMAT, shader_resources::ShaderModule,
};
use gpui::{BlurPlan, Bounds, DevicePixels, EffectLayer, LayerRegion, ScaledPixels, Size};

/// Textures kept for reuse between frames. Each open layer holds one
/// frame-sized texture plus a few region-sized ones, so a handful is
/// enough for deep nesting.
const POOL_LIMIT: usize = 16;

#[repr(C)]
#[derive(Clone, Copy)]
struct BlurParams {
    step: [f32; 2],
    sigma: f32,
    radius: i32,
}

/// One composite draw, laid out like the HLSL `LayerComposite`. Structured
/// buffers pack like C, so the field order matches the Metal one.
#[repr(C)]
#[derive(Clone, Copy)]
struct LayerComposite {
    layer: EffectLayer,
    region: Bounds<ScaledPixels>,
}

// A structured buffer packs its members at 4 bytes. The Rust structs match
// the HLSL ones field by field only while no member asks for more.
const _: () = assert!(std::mem::align_of::<BlurParams>() == 4);
const _: () = assert!(std::mem::size_of::<BlurParams>() == 16);
const _: () = assert!(std::mem::align_of::<LayerComposite>() == 4);

/// What one frame of layer work reads from the renderer.
pub(crate) struct EffectFrame<'a> {
    pub texture: &'a ID3D11Texture2D,
    pub view: &'a Option<ID3D11RenderTargetView>,
    pub viewport: D3D11_VIEWPORT,
    pub viewport_size: Size<DevicePixels>,
}

struct LayerTexture {
    texture: ID3D11Texture2D,
    view: Option<ID3D11RenderTargetView>,
    resource: Option<ID3D11ShaderResourceView>,
    width: u32,
    height: u32,
    format: DXGI_FORMAT,
}

impl LayerTexture {
    fn viewport(&self) -> D3D11_VIEWPORT {
        D3D11_VIEWPORT {
            TopLeftX: 0.0,
            TopLeftY: 0.0,
            Width: self.width as f32,
            Height: self.height as f32,
            MinDepth: 0.0,
            MaxDepth: 1.0,
        }
    }
}

/// A layer between its begin and end marks. `texture` is `None` when the
/// region is empty, in which case the content draws into the parent and
/// the end mark does nothing.
struct OpenLayer {
    region: LayerRegion,
    texture: Option<LayerTexture>,
}

/// The pixel format of the mask-weighted backdrop chain. The composite
/// divides these textures by their alpha, and an 8 bit level would step
/// visibly after the divide.
const MASKED_FORMAT: DXGI_FORMAT = DXGI_FORMAT_R16G16B16A16_FLOAT;

pub(crate) struct DirectXEffects {
    device: ID3D11Device,
    context: ID3D11DeviceContext,
    blur: PipelineState<BlurParams>,
    premask: PipelineState<LayerComposite>,
    composite: PipelineState<LayerComposite>,
    smooth_sampler: Option<ID3D11SamplerState>,
    exact_sampler: Option<ID3D11SamplerState>,
    pool: Vec<LayerTexture>,
    open: Vec<OpenLayer>,
}

impl DirectXEffects {
    pub fn new(device: &ID3D11Device, context: &ID3D11DeviceContext) -> Result<Self> {
        let blur = PipelineState::new(
            device,
            "layer_blur_pipeline",
            ShaderModule::Blur,
            1,
            create_blend_state_without_blending(device)?,
        )?;
        let premask = PipelineState::new(
            device,
            "layer_premask_pipeline",
            ShaderModule::Premask,
            1,
            create_blend_state_without_blending(device)?,
        )?;
        let composite = PipelineState::new(
            device,
            "layer_composite_pipeline",
            ShaderModule::LayerComposite,
            1,
            create_blend_state_without_blending(device)?,
        )?;
        Ok(Self {
            device: device.clone(),
            context: context.clone(),
            blur,
            premask,
            composite,
            smooth_sampler: create_clamp_sampler(device, D3D11_FILTER_MIN_MAG_MIP_LINEAR)?,
            exact_sampler: create_clamp_sampler(device, D3D11_FILTER_MIN_MAG_MIP_POINT)?,
            pool: Vec::new(),
            open: Vec::new(),
        })
    }

    /// Call once per frame before the first layer. A frame that stopped on
    /// an error leaves its layers open.
    pub fn begin_frame(&mut self) {
        self.open.clear();
    }

    /// Drop every pooled texture, for example after a resize.
    pub fn forget_textures(&mut self) {
        self.pool.clear();
        self.open.clear();
    }

    pub fn has_open_layers(&self) -> bool {
        !self.open.is_empty()
    }

    /// The render target the scene draws into right now: the innermost
    /// open layer, or the frame.
    pub fn target_view<'a>(
        &'a self,
        frame_view: &'a Option<ID3D11RenderTargetView>,
    ) -> &'a Option<ID3D11RenderTargetView> {
        self.open
            .iter()
            .rev()
            .find_map(|layer| layer.texture.as_ref())
            .map_or(frame_view, |texture| &texture.view)
    }

    fn target<'a>(&'a self, frame: &EffectFrame<'a>) -> (&'a ID3D11Texture2D, LayerRegion) {
        self.open
            .iter()
            .rev()
            .find_map(|layer| layer.texture.as_ref().map(|t| (&t.texture, layer.region)))
            .unwrap_or((frame.texture, LayerRegion::of_viewport(frame.viewport_size)))
    }

    /// Opens a layer and points the output merger at its texture.
    pub fn begin_layer(&mut self, frame: &EffectFrame, layer: &EffectLayer) -> Result<()> {
        let region = layer.region(self.target(frame).1);
        let texture = if region.is_empty() {
            None
        } else {
            let texture = self.take_texture(
                LayerRegion::of_viewport(frame.viewport_size),
                RENDER_TARGET_FORMAT,
            )?;
            unsafe {
                self.context.ClearRenderTargetView(
                    texture.view.as_ref().context("missing layer view")?,
                    &[0.0; 4],
                );
                self.context
                    .OMSetRenderTargets(Some(slice::from_ref(&texture.view)), None);
            }
            Some(texture)
        };
        self.open.push(OpenLayer { region, texture });
        Ok(())
    }

    /// Closes the innermost layer, paints it into its parent and points the
    /// output merger back at the parent.
    pub fn end_layer(&mut self, frame: &EffectFrame, layer: &EffectLayer) -> Result<()> {
        let Some(open) = self.open.pop() else {
            return Ok(());
        };
        let Some(full) = open.texture else {
            return Ok(());
        };
        let region = open.region;
        let context = self.context.clone();
        // Nothing may be a render target and a copy source at once.
        unsafe { context.OMSetRenderTargets(None, None) };

        let content = self.take_texture(region, RENDER_TARGET_FORMAT)?;
        copy_region(&context, &full.texture, &content.texture, region);
        self.give_back(full);

        let under = self.take_texture(region, RENDER_TARGET_FORMAT)?;
        let (parent_texture, _) = self.target(frame);
        copy_region(&context, parent_texture, &under.texture, region);

        let composite_params = LayerComposite {
            layer: *layer,
            region: region.bounds(),
        };

        let blurred_content = if layer.blur > 0.0 {
            Some(self.blur(&content, region, layer.blur)?)
        } else {
            None
        };

        let backdrop_blurs = layer.has_backdrop != 0 && layer.backdrop_blur > 0.0;
        // A mask over a blurred backdrop asks for a blur that grows with
        // the mask. The blur must only read pixels the mask covers, or the
        // colours next to the mask bleed in. The premask pass multiplies
        // the backdrop by the mask and keeps the weight in alpha, so the
        // composite can divide it back out.
        let progressive = backdrop_blurs && layer.has_mask != 0;
        let masked = if progressive {
            let masked = self.take_texture(region, MASKED_FORMAT)?;
            self.premask_pass(&under, &masked, composite_params)?;
            Some(masked)
        } else {
            None
        };
        let backdrop_source = masked.as_ref().unwrap_or(&under);
        let blurred_under = if backdrop_blurs {
            Some(self.blur(backdrop_source, region, layer.backdrop_blur)?)
        } else {
            None
        };
        // Two much weaker blurs give the shader levels to mix, so the blur
        // amount can ramp over a wide range of the mask.
        let blurred_mid = if progressive {
            Some(self.blur(backdrop_source, region, layer.backdrop_blur * 0.25)?)
        } else {
            None
        };
        let blurred_low = if progressive {
            Some(self.blur(backdrop_source, region, layer.backdrop_blur * 0.0625)?)
        } else {
            None
        };
        if let Some(texture) = masked {
            self.give_back(texture);
        }

        let parent_view = self.target_view(frame.view).clone();
        self.composite
            .update_buffer(&self.device, &context, &[composite_params])?;
        unsafe {
            context.OMSetRenderTargets(Some(slice::from_ref(&parent_view)), None);
            context.RSSetViewports(Some(slice::from_ref(&frame.viewport)));
            let backdrop = blurred_under.as_ref().unwrap_or(&under);
            context.PSSetShaderResources(
                2,
                Some(&[
                    under.resource.clone(),
                    backdrop.resource.clone(),
                    blurred_mid.as_ref().unwrap_or(backdrop).resource.clone(),
                    blurred_low.as_ref().unwrap_or(backdrop).resource.clone(),
                ]),
            );
            context.PSSetSamplers(1, Some(slice::from_ref(&self.exact_sampler)));
        }
        self.composite.draw_with_texture(
            &context,
            slice::from_ref(&blurred_content.as_ref().unwrap_or(&content).resource),
            slice::from_ref(&self.smooth_sampler),
            1,
        )?;
        unbind_textures(&context);

        self.give_back(content);
        self.give_back(under);
        if let Some(texture) = blurred_content {
            self.give_back(texture);
        }
        for texture in [blurred_under, blurred_mid, blurred_low]
            .into_iter()
            .flatten()
        {
            self.give_back(texture);
        }
        Ok(())
    }

    /// Two separable gaussian passes over a shrunk copy of `source`. The
    /// result is a texture the size of the small region, which the
    /// composite samples with the linear sampler to scale it back up.
    fn blur(
        &mut self,
        source: &LayerTexture,
        region: LayerRegion,
        sigma: f32,
    ) -> Result<LayerTexture> {
        let plan = BlurPlan::new(sigma);
        let BlurPlan { sigma, radius, .. } = plan;

        // Halve the source until it is `plan.scale` times smaller. One tap
        // at the centre of a 2 by 2 block is an exact box average.
        let format = source.format;
        let mut size = LayerRegion::new(0, 0, region.width, region.height);
        let mut shrunk: Option<LayerTexture> = None;
        for _ in 0..plan.shrink_steps() {
            let next_size = size.halved();
            let next = self.take_texture(next_size, format)?;
            self.blur_pass(
                shrunk.as_ref().unwrap_or(source),
                &next,
                BlurParams {
                    step: [0.0, 0.0],
                    sigma: 1.0,
                    radius: 0,
                },
            )?;
            if let Some(texture) = shrunk {
                self.give_back(texture);
            }
            shrunk = Some(next);
            size = next_size;
        }

        let first = self.take_texture(size, format)?;
        self.blur_pass(
            shrunk.as_ref().unwrap_or(source),
            &first,
            BlurParams {
                step: [1.0 / size.width as f32, 0.0],
                sigma,
                radius,
            },
        )?;
        if let Some(texture) = shrunk {
            self.give_back(texture);
        }
        let second = self.take_texture(size, format)?;
        self.blur_pass(
            &first,
            &second,
            BlurParams {
                step: [0.0, 1.0 / size.height as f32],
                sigma,
                radius,
            },
        )?;
        self.give_back(first);
        Ok(second)
    }

    fn blur_pass(
        &mut self,
        source: &LayerTexture,
        target: &LayerTexture,
        params: BlurParams,
    ) -> Result<()> {
        let context = &self.context;
        self.blur.update_buffer(&self.device, context, &[params])?;
        unsafe {
            context.OMSetRenderTargets(Some(slice::from_ref(&target.view)), None);
            context.RSSetViewports(Some(&[target.viewport()]));
        }
        self.blur.draw_with_texture(
            context,
            slice::from_ref(&source.resource),
            slice::from_ref(&self.smooth_sampler),
            1,
        )?;
        unbind_textures(context);
        Ok(())
    }

    /// Writes `under` times the mask into `target`, with the mask weight in
    /// alpha. The composite divides the blurred result by that alpha, so
    /// the blur average only counts pixels the mask covers.
    fn premask_pass(
        &mut self,
        under: &LayerTexture,
        target: &LayerTexture,
        params: LayerComposite,
    ) -> Result<()> {
        let context = &self.context;
        self.premask
            .update_buffer(&self.device, context, &[params])?;
        unsafe {
            context.OMSetRenderTargets(Some(slice::from_ref(&target.view)), None);
            context.RSSetViewports(Some(&[target.viewport()]));
            context.PSSetShaderResources(2, Some(&[under.resource.clone()]));
            context.PSSetSamplers(1, Some(slice::from_ref(&self.exact_sampler)));
        }
        // The shader reads the backdrop from t2 alone, so t0 stays empty.
        self.premask.draw_with_texture(
            context,
            &[None],
            slice::from_ref(&self.exact_sampler),
            1,
        )?;
        unbind_textures(context);
        unsafe { context.OMSetRenderTargets(None, None) };
        Ok(())
    }

    fn take_texture(&mut self, region: LayerRegion, format: DXGI_FORMAT) -> Result<LayerTexture> {
        let width = region.width.max(1) as u32;
        let height = region.height.max(1) as u32;
        if let Some(index) = self
            .pool
            .iter()
            .position(|t| t.width == width && t.height == height && t.format == format)
        {
            return Ok(self.pool.swap_remove(index));
        }
        let desc = D3D11_TEXTURE2D_DESC {
            Width: width,
            Height: height,
            MipLevels: 1,
            ArraySize: 1,
            Format: format,
            SampleDesc: DXGI_SAMPLE_DESC {
                Count: 1,
                Quality: 0,
            },
            Usage: D3D11_USAGE_DEFAULT,
            BindFlags: (D3D11_BIND_RENDER_TARGET.0 | D3D11_BIND_SHADER_RESOURCE.0) as u32,
            CPUAccessFlags: 0,
            MiscFlags: 0,
        };
        let mut texture = None;
        unsafe {
            self.device
                .CreateTexture2D(&desc, None, Some(&mut texture))?
        };
        let texture = texture.context("layer texture was not created")?;
        let mut view = None;
        unsafe {
            self.device
                .CreateRenderTargetView(&texture, None, Some(&mut view))?
        };
        let mut resource = None;
        unsafe {
            self.device
                .CreateShaderResourceView(&texture, None, Some(&mut resource))?
        };
        Ok(LayerTexture {
            texture,
            view,
            resource,
            width,
            height,
            format,
        })
    }

    fn give_back(&mut self, texture: LayerTexture) {
        if self.pool.len() < POOL_LIMIT {
            self.pool.push(texture);
        }
    }
}

/// Copies `region` of `source` into the top left of `target`.
fn copy_region(
    context: &ID3D11DeviceContext,
    source: &ID3D11Texture2D,
    target: &ID3D11Texture2D,
    region: LayerRegion,
) {
    let source_box = D3D11_BOX {
        left: region.x as u32,
        top: region.y as u32,
        front: 0,
        right: (region.x + region.width) as u32,
        bottom: (region.y + region.height) as u32,
        back: 1,
    };
    unsafe {
        context.CopySubresourceRegion(target, 0, 0, 0, 0, source, 0, Some(&source_box));
    }
}

/// Clears t0 on both stages and t2 to t5, so a texture can become a render
/// target again.
fn unbind_textures(context: &ID3D11DeviceContext) {
    unsafe {
        context.VSSetShaderResources(0, Some(&[None]));
        context.PSSetShaderResources(0, Some(&[None]));
        context.PSSetShaderResources(2, Some(&[None, None, None, None]));
    }
}

fn create_clamp_sampler(
    device: &ID3D11Device,
    filter: D3D11_FILTER,
) -> Result<Option<ID3D11SamplerState>> {
    let desc = D3D11_SAMPLER_DESC {
        Filter: filter,
        AddressU: D3D11_TEXTURE_ADDRESS_CLAMP,
        AddressV: D3D11_TEXTURE_ADDRESS_CLAMP,
        AddressW: D3D11_TEXTURE_ADDRESS_CLAMP,
        MipLODBias: 0.0,
        MaxAnisotropy: 1,
        ComparisonFunc: D3D11_COMPARISON_ALWAYS,
        BorderColor: [0.0; 4],
        MinLOD: 0.0,
        MaxLOD: D3D11_FLOAT32_MAX,
    };
    let mut sampler = None;
    unsafe { device.CreateSamplerState(&desc, Some(&mut sampler))? };
    Ok(sampler)
}

fn create_blend_state_without_blending(device: &ID3D11Device) -> Result<ID3D11BlendState> {
    let mut desc = D3D11_BLEND_DESC::default();
    desc.RenderTarget[0].BlendEnable = false.into();
    desc.RenderTarget[0].RenderTargetWriteMask = D3D11_COLOR_WRITE_ENABLE_ALL.0 as u8;
    let mut state = None;
    unsafe { device.CreateBlendState(&desc, Some(&mut state))? };
    state.context("blend state was not created")
}
