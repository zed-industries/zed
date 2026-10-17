use anyhow::{Context as _, Result};
use windows::Win32::Graphics::{
    Direct3D11::{
        D3D11_BIND_CONSTANT_BUFFER, D3D11_BIND_SHADER_RESOURCE, D3D11_BUFFER_DESC,
        D3D11_SUBRESOURCE_DATA, D3D11_TEXTURE2D_DESC, D3D11_USAGE_IMMUTABLE, ID3D11Buffer,
        ID3D11Device, ID3D11DeviceContext, ID3D11ShaderResourceView,
    },
    Dxgi::Common::{DXGI_FORMAT_R32G32B32A32_FLOAT, DXGI_SAMPLE_DESC},
};

pub(crate) struct Bindings {
    flags: Option<ID3D11Buffer>,
    views: Vec<Option<ID3D11ShaderResourceView>>,
}
impl Bindings {
    pub(crate) fn new(device: &ID3D11Device, stacks: &[Vec<gpui::RoundedClip>]) -> Result<Self> {
        let values = [1u32, 0, 0, 0];
        let desc = D3D11_BUFFER_DESC {
            ByteWidth: 16,
            Usage: D3D11_USAGE_IMMUTABLE,
            BindFlags: D3D11_BIND_CONSTANT_BUFFER.0 as u32,
            ..Default::default()
        };
        let data = D3D11_SUBRESOURCE_DATA {
            pSysMem: values.as_ptr().cast(),
            ..Default::default()
        };
        let mut flags = None;
        if stacks.iter().any(|clips| !clips.is_empty()) {
            unsafe {
                device.CreateBuffer(&desc, Some(&data), Some(&mut flags))?;
            }
        }
        let mut views = Vec::new();
        for clips in stacks {
            if clips.is_empty() {
                views.push(None);
                continue;
            }
            let geometry: Vec<_> = clips.iter().map(gpui::RoundedClip::gpu_data).collect();
            let desc = D3D11_TEXTURE2D_DESC {
                Width: 3,
                Height: u32::try_from(clips.len())?,
                MipLevels: 1,
                ArraySize: 1,
                Format: DXGI_FORMAT_R32G32B32A32_FLOAT,
                SampleDesc: DXGI_SAMPLE_DESC {
                    Count: 1,
                    Quality: 0,
                },
                Usage: D3D11_USAGE_IMMUTABLE,
                BindFlags: D3D11_BIND_SHADER_RESOURCE.0 as u32,
                ..Default::default()
            };
            let data = D3D11_SUBRESOURCE_DATA {
                pSysMem: geometry.as_ptr().cast(),
                SysMemPitch: 48,
                ..Default::default()
            };
            let mut texture = None;
            let mut view = None;
            unsafe {
                device.CreateTexture2D(&desc, Some(&data), Some(&mut texture))?;
                device.CreateShaderResourceView(
                    texture.as_ref().context("rounded clip texture")?,
                    None,
                    Some(&mut view),
                )?;
            }
            views.push(view);
        }
        Ok(Self { flags, views })
    }
    pub(crate) fn bind(&self, context: &ID3D11DeviceContext, index: usize) {
        let view = self.views.get(index).cloned().flatten();
        let flags = view.as_ref().and(self.flags.clone());
        unsafe {
            context.PSSetConstantBuffers(2, Some(&[flags]));
            context.PSSetShaderResources(15, Some(&[view]));
        }
    }
}
