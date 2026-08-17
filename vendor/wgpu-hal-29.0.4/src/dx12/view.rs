use windows::Win32::Graphics::{Direct3D12, Dxgi};

use crate::auxil;

pub(super) struct ViewDescriptor {
    dimension: wgt::TextureViewDimension,
    pub aspects: crate::FormatAspects,
    pub rtv_dsv_format: Dxgi::Common::DXGI_FORMAT,
    srv_uav_format: Option<Dxgi::Common::DXGI_FORMAT>,
    multisampled: bool,
    array_layer_base: u32,
    array_layer_count: u32,
    mip_level_base: u32,
    mip_level_count: u32,
}

impl crate::TextureViewDescriptor<'_> {
    pub(super) fn to_internal(&self, texture: &super::Texture) -> ViewDescriptor {
        let aspects = crate::FormatAspects::new(texture.format, self.range.aspect);

        ViewDescriptor {
            dimension: self.dimension,
            aspects,
            rtv_dsv_format: auxil::dxgi::conv::map_texture_format(self.format),
            srv_uav_format: auxil::dxgi::conv::map_texture_format_for_srv_uav(self.format, aspects),
            multisampled: texture.sample_count > 1,
            mip_level_base: self.range.base_mip_level,
            mip_level_count: self.range.mip_level_count.unwrap_or(!0),
            array_layer_base: self.range.base_array_layer,
            array_layer_count: self.range.array_layer_count.unwrap_or(!0),
        }
    }
}

fn aspects_to_plane(aspects: crate::FormatAspects) -> u32 {
    match aspects {
        crate::FormatAspects::STENCIL => 1,
        crate::FormatAspects::PLANE_1 => 1,
        crate::FormatAspects::PLANE_2 => 2,
        _ => 0,
    }
}

impl ViewDescriptor {
    pub(crate) unsafe fn to_srv(&self) -> Option<Direct3D12::D3D12_SHADER_RESOURCE_VIEW_DESC> {
        let mut desc = Direct3D12::D3D12_SHADER_RESOURCE_VIEW_DESC {
            Format: self.srv_uav_format?,
            ViewDimension: Direct3D12::D3D12_SRV_DIMENSION_UNKNOWN,
            Shader4ComponentMapping: Direct3D12::D3D12_DEFAULT_SHADER_4_COMPONENT_MAPPING,
            Anonymous: Default::default(),
        };

        match self.dimension {
            wgt::TextureViewDimension::D1 => {
                desc.ViewDimension = Direct3D12::D3D12_SRV_DIMENSION_TEXTURE1D;
                desc.Anonymous.Texture1D = Direct3D12::D3D12_TEX1D_SRV {
                    MostDetailedMip: self.mip_level_base,
                    MipLevels: self.mip_level_count,
                    ResourceMinLODClamp: 0.0,
                }
            }
            /*
            wgt::TextureViewDimension::D1Array => {
                desc.ViewDimension = Direct3D12::D3D12_SRV_DIMENSION_TEXTURE1DARRAY;
                desc.Anonymous.Texture1DArray = Direct3D12::D3D12_TEX1D_ARRAY_SRV {
                    MostDetailedMip: self.mip_level_base,
                    MipLevels: self.mip_level_count,
                    FirstArraySlice: self.array_layer_base,
                    ArraySize: self.array_layer_count,
                    ResourceMinLODClamp: 0.0,
                }
            }
            */
            wgt::TextureViewDimension::D2 if self.multisampled && self.array_layer_base == 0 => {
                desc.ViewDimension = Direct3D12::D3D12_SRV_DIMENSION_TEXTURE2DMS;
                desc.Anonymous.Texture2DMS = Direct3D12::D3D12_TEX2DMS_SRV {
                    UnusedField_NothingToDefine: 0,
                }
            }
            wgt::TextureViewDimension::D2 if self.array_layer_base == 0 => {
                desc.ViewDimension = Direct3D12::D3D12_SRV_DIMENSION_TEXTURE2D;
                desc.Anonymous.Texture2D = Direct3D12::D3D12_TEX2D_SRV {
                    MostDetailedMip: self.mip_level_base,
                    MipLevels: self.mip_level_count,
                    PlaneSlice: aspects_to_plane(self.aspects),
                    ResourceMinLODClamp: 0.0,
                }
            }
            wgt::TextureViewDimension::D2 | wgt::TextureViewDimension::D2Array
                if self.multisampled =>
            {
                desc.ViewDimension = Direct3D12::D3D12_SRV_DIMENSION_TEXTURE2DMSARRAY;
                desc.Anonymous.Texture2DMSArray = Direct3D12::D3D12_TEX2DMS_ARRAY_SRV {
                    FirstArraySlice: self.array_layer_base,
                    ArraySize: self.array_layer_count,
                }
            }
            wgt::TextureViewDimension::D2 | wgt::TextureViewDimension::D2Array => {
                desc.ViewDimension = Direct3D12::D3D12_SRV_DIMENSION_TEXTURE2DARRAY;
                desc.Anonymous.Texture2DArray = Direct3D12::D3D12_TEX2D_ARRAY_SRV {
                    MostDetailedMip: self.mip_level_base,
                    MipLevels: self.mip_level_count,
                    FirstArraySlice: self.array_layer_base,
                    ArraySize: self.array_layer_count,
                    PlaneSlice: aspects_to_plane(self.aspects),
                    ResourceMinLODClamp: 0.0,
                }
            }
            wgt::TextureViewDimension::D3 => {
                desc.ViewDimension = Direct3D12::D3D12_SRV_DIMENSION_TEXTURE3D;
                desc.Anonymous.Texture3D = Direct3D12::D3D12_TEX3D_SRV {
                    MostDetailedMip: self.mip_level_base,
                    MipLevels: self.mip_level_count,
                    ResourceMinLODClamp: 0.0,
                }
            }
            wgt::TextureViewDimension::Cube if self.array_layer_base == 0 => {
                desc.ViewDimension = Direct3D12::D3D12_SRV_DIMENSION_TEXTURECUBE;
                desc.Anonymous.TextureCube = Direct3D12::D3D12_TEXCUBE_SRV {
                    MostDetailedMip: self.mip_level_base,
                    MipLevels: self.mip_level_count,
                    ResourceMinLODClamp: 0.0,
                }
            }
            wgt::TextureViewDimension::Cube | wgt::TextureViewDimension::CubeArray => {
                desc.ViewDimension = Direct3D12::D3D12_SRV_DIMENSION_TEXTURECUBEARRAY;
                desc.Anonymous.TextureCubeArray = Direct3D12::D3D12_TEXCUBE_ARRAY_SRV {
                    MostDetailedMip: self.mip_level_base,
                    MipLevels: self.mip_level_count,
                    First2DArrayFace: self.array_layer_base,
                    NumCubes: if self.array_layer_count == !0 {
                        !0
                    } else {
                        self.array_layer_count / 6
                    },
                    ResourceMinLODClamp: 0.0,
                }
            }
        }

        Some(desc)
    }

    pub(crate) unsafe fn to_uav(&self) -> Option<Direct3D12::D3D12_UNORDERED_ACCESS_VIEW_DESC> {
        let mut desc = Direct3D12::D3D12_UNORDERED_ACCESS_VIEW_DESC {
            Format: self.srv_uav_format?,
            ViewDimension: Direct3D12::D3D12_UAV_DIMENSION_UNKNOWN,
            Anonymous: Default::default(),
        };

        match self.dimension {
            wgt::TextureViewDimension::D1 => {
                desc.ViewDimension = Direct3D12::D3D12_UAV_DIMENSION_TEXTURE1D;
                desc.Anonymous.Texture1D = Direct3D12::D3D12_TEX1D_UAV {
                    MipSlice: self.mip_level_base,
                }
            }
            /*
            wgt::TextureViewDimension::D1Array => {
                desc.ViewDimension = Direct3D12::D3D12_UAV_DIMENSION_TEXTURE1DARRAY;
                desc.Anonymous.Texture1DArray = Direct3D12::D3D12_TEX1D_ARRAY_UAV {
                    MipSlice: self.mip_level_base,
                    FirstArraySlice: self.array_layer_base,
                    ArraySize,
                }
            }*/
            wgt::TextureViewDimension::D2 if self.array_layer_base == 0 => {
                desc.ViewDimension = Direct3D12::D3D12_UAV_DIMENSION_TEXTURE2D;
                desc.Anonymous.Texture2D = Direct3D12::D3D12_TEX2D_UAV {
                    MipSlice: self.mip_level_base,
                    PlaneSlice: aspects_to_plane(self.aspects),
                }
            }
            wgt::TextureViewDimension::D2 | wgt::TextureViewDimension::D2Array => {
                desc.ViewDimension = Direct3D12::D3D12_UAV_DIMENSION_TEXTURE2DARRAY;
                desc.Anonymous.Texture2DArray = Direct3D12::D3D12_TEX2D_ARRAY_UAV {
                    MipSlice: self.mip_level_base,
                    FirstArraySlice: self.array_layer_base,
                    ArraySize: self.array_layer_count,
                    PlaneSlice: aspects_to_plane(self.aspects),
                }
            }
            wgt::TextureViewDimension::D3 => {
                desc.ViewDimension = Direct3D12::D3D12_UAV_DIMENSION_TEXTURE3D;
                desc.Anonymous.Texture3D = Direct3D12::D3D12_TEX3D_UAV {
                    MipSlice: self.mip_level_base,
                    FirstWSlice: 0,
                    WSize: u32::MAX,
                }
            }
            wgt::TextureViewDimension::Cube | wgt::TextureViewDimension::CubeArray => {
                panic!("Unable to view texture as cube UAV")
            }
        }

        Some(desc)
    }

    pub(crate) unsafe fn to_rtv(&self) -> Direct3D12::D3D12_RENDER_TARGET_VIEW_DESC {
        let mut desc = Direct3D12::D3D12_RENDER_TARGET_VIEW_DESC {
            Format: self.rtv_dsv_format,
            ViewDimension: Direct3D12::D3D12_RTV_DIMENSION_UNKNOWN,
            Anonymous: Default::default(),
        };

        match self.dimension {
            wgt::TextureViewDimension::D1 => {
                desc.ViewDimension = Direct3D12::D3D12_RTV_DIMENSION_TEXTURE1D;
                desc.Anonymous.Texture1D = Direct3D12::D3D12_TEX1D_RTV {
                    MipSlice: self.mip_level_base,
                }
            }
            /*
            wgt::TextureViewDimension::D1Array => {
                desc.ViewDimension = Direct3D12::D3D12_RTV_DIMENSION_TEXTURE1DARRAY;
                desc.Anonymous.Texture1DArray = Direct3D12::D3D12_TEX1D_ARRAY_RTV {
                    MipSlice: self.mip_level_base,
                    FirstArraySlice: self.array_layer_base,
                    ArraySize,
                }
            }*/
            wgt::TextureViewDimension::D2 if self.multisampled && self.array_layer_base == 0 => {
                desc.ViewDimension = Direct3D12::D3D12_RTV_DIMENSION_TEXTURE2DMS;
                desc.Anonymous.Texture2DMS = Direct3D12::D3D12_TEX2DMS_RTV {
                    UnusedField_NothingToDefine: 0,
                }
            }
            wgt::TextureViewDimension::D2 if self.array_layer_base == 0 => {
                desc.ViewDimension = Direct3D12::D3D12_RTV_DIMENSION_TEXTURE2D;
                desc.Anonymous.Texture2D = Direct3D12::D3D12_TEX2D_RTV {
                    MipSlice: self.mip_level_base,
                    PlaneSlice: aspects_to_plane(self.aspects),
                }
            }
            wgt::TextureViewDimension::D2 | wgt::TextureViewDimension::D2Array
                if self.multisampled =>
            {
                desc.ViewDimension = Direct3D12::D3D12_RTV_DIMENSION_TEXTURE2DMSARRAY;
                desc.Anonymous.Texture2DMSArray = Direct3D12::D3D12_TEX2DMS_ARRAY_RTV {
                    FirstArraySlice: self.array_layer_base,
                    ArraySize: self.array_layer_count,
                }
            }
            wgt::TextureViewDimension::D2 | wgt::TextureViewDimension::D2Array => {
                desc.ViewDimension = Direct3D12::D3D12_RTV_DIMENSION_TEXTURE2DARRAY;
                desc.Anonymous.Texture2DArray = Direct3D12::D3D12_TEX2D_ARRAY_RTV {
                    MipSlice: self.mip_level_base,
                    FirstArraySlice: self.array_layer_base,
                    ArraySize: self.array_layer_count,
                    PlaneSlice: aspects_to_plane(self.aspects),
                }
            }
            wgt::TextureViewDimension::D3
            | wgt::TextureViewDimension::Cube
            | wgt::TextureViewDimension::CubeArray => {
                panic!("Unable to view texture as cube or 3D RTV")
            }
        }

        desc
    }

    pub(crate) unsafe fn to_dsv(
        &self,
        read_only: bool,
    ) -> Direct3D12::D3D12_DEPTH_STENCIL_VIEW_DESC {
        let mut desc = Direct3D12::D3D12_DEPTH_STENCIL_VIEW_DESC {
            Format: self.rtv_dsv_format,
            ViewDimension: Direct3D12::D3D12_DSV_DIMENSION_UNKNOWN,
            Flags: {
                let mut flags = Direct3D12::D3D12_DSV_FLAG_NONE;
                if read_only {
                    if self.aspects.contains(crate::FormatAspects::DEPTH) {
                        flags |= Direct3D12::D3D12_DSV_FLAG_READ_ONLY_DEPTH;
                    }
                    if self.aspects.contains(crate::FormatAspects::STENCIL) {
                        flags |= Direct3D12::D3D12_DSV_FLAG_READ_ONLY_STENCIL;
                    }
                }
                flags
            },
            Anonymous: Default::default(),
        };

        match self.dimension {
            wgt::TextureViewDimension::D1 => {
                desc.ViewDimension = Direct3D12::D3D12_DSV_DIMENSION_TEXTURE1D;
                desc.Anonymous.Texture1D = Direct3D12::D3D12_TEX1D_DSV {
                    MipSlice: self.mip_level_base,
                }
            }
            /*
            wgt::TextureViewDimension::D1Array => {
                desc.ViewDimension = Direct3D12::D3D12_DSV_DIMENSION_TEXTURE1DARRAY;
                desc.Anonymous.Texture1DArray = Direct3D12::D3D12_TEX1D_ARRAY_DSV {
                    MipSlice: self.mip_level_base,
                    FirstArraySlice: self.array_layer_base,
                    ArraySize,
                }
            }*/
            wgt::TextureViewDimension::D2 if self.multisampled && self.array_layer_base == 0 => {
                desc.ViewDimension = Direct3D12::D3D12_DSV_DIMENSION_TEXTURE2DMS;
                desc.Anonymous.Texture2DMS = Direct3D12::D3D12_TEX2DMS_DSV {
                    UnusedField_NothingToDefine: 0,
                }
            }
            wgt::TextureViewDimension::D2 if self.array_layer_base == 0 => {
                desc.ViewDimension = Direct3D12::D3D12_DSV_DIMENSION_TEXTURE2D;

                desc.Anonymous.Texture2D = Direct3D12::D3D12_TEX2D_DSV {
                    MipSlice: self.mip_level_base,
                }
            }
            wgt::TextureViewDimension::D2 | wgt::TextureViewDimension::D2Array
                if self.multisampled =>
            {
                desc.ViewDimension = Direct3D12::D3D12_DSV_DIMENSION_TEXTURE2DMSARRAY;
                desc.Anonymous.Texture2DMSArray = Direct3D12::D3D12_TEX2DMS_ARRAY_DSV {
                    FirstArraySlice: self.array_layer_base,
                    ArraySize: self.array_layer_count,
                }
            }
            wgt::TextureViewDimension::D2 | wgt::TextureViewDimension::D2Array => {
                desc.ViewDimension = Direct3D12::D3D12_DSV_DIMENSION_TEXTURE2DARRAY;
                desc.Anonymous.Texture2DArray = Direct3D12::D3D12_TEX2D_ARRAY_DSV {
                    MipSlice: self.mip_level_base,
                    FirstArraySlice: self.array_layer_base,
                    ArraySize: self.array_layer_count,
                }
            }
            wgt::TextureViewDimension::D3
            | wgt::TextureViewDimension::Cube
            | wgt::TextureViewDimension::CubeArray => {
                panic!("Unable to view texture as cube or 3D DSV")
            }
        }

        desc
    }
}
