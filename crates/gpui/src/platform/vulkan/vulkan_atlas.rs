use std::borrow::Cow;

use anyhow::Result;
use ash::vk;
use collections::FxHashMap;
use etagere::BucketedAtlasAllocator;
use parking_lot::Mutex;

use crate::{
    AtlasKey, AtlasTextureId, AtlasTextureKind, AtlasTile, Bounds, DevicePixels, PlatformAtlas,
    Point, Size,
};

pub(crate) struct VulkanAtlas(Mutex<VulkanAtlasState>);

impl VulkanAtlas {
    pub fn new(device: ash::Device, memory_properties: vk::PhysicalDeviceMemoryProperties) -> Self {
        Self(Mutex::new(VulkanAtlasState {
            device,
            memory_properties,
            monochrome_textures: Default::default(),
            polychrome_textures: Default::default(),
            path_textures: Default::default(),
            tiles_by_key: Default::default(),
        }))
    }

    pub fn destroy(&self) {
        self.0.lock().destroy();
    }
}

impl PlatformAtlas for VulkanAtlas {
    fn get_or_insert_with<'a>(
        &self,
        key: &AtlasKey,
        build: &mut dyn FnMut() -> Result<(Size<DevicePixels>, Cow<'a, [u8]>)>,
    ) -> Result<AtlasTile> {
        let mut lock = self.0.lock();
        if let Some(tile) = lock.tiles_by_key.get(key) {
            Ok(tile.clone())
        } else {
            let (size, bytes) = build()?;
            let tile = lock.allocate(size, key.texture_kind());
            let texture = lock.texture(tile.texture_id);
            texture.upload(tile.bounds, &bytes);
            lock.tiles_by_key.insert(key.clone(), tile.clone());
            Ok(tile)
        }
    }
}

struct VulkanAtlasState {
    device: ash::Device,
    memory_properties: vk::PhysicalDeviceMemoryProperties,
    monochrome_textures: Vec<VulkanAtlasTexture>,
    polychrome_textures: Vec<VulkanAtlasTexture>,
    path_textures: Vec<VulkanAtlasTexture>,
    tiles_by_key: FxHashMap<AtlasKey, AtlasTile>,
}

impl VulkanAtlasState {
    fn allocate(&mut self, size: Size<DevicePixels>, texture_kind: AtlasTextureKind) -> AtlasTile {
        let textures = match texture_kind {
            AtlasTextureKind::Monochrome => &mut self.monochrome_textures,
            AtlasTextureKind::Polychrome => &mut self.polychrome_textures,
            AtlasTextureKind::Path => &mut self.path_textures,
        };

        textures
            .iter_mut()
            .rev()
            .find_map(|texture| texture.allocate(size))
            .unwrap_or_else(|| {
                let texture = self.push_texture(size, texture_kind);
                texture.allocate(size).unwrap()
            })
    }

    fn push_texture(
        &mut self,
        min_size: Size<DevicePixels>,
        kind: AtlasTextureKind,
    ) -> &mut VulkanAtlasTexture {
        const DEFAULT_ATLAS_SIZE: Size<DevicePixels> = Size {
            width: DevicePixels(1024),
            height: DevicePixels(1024),
        };
        let size = min_size.max(&DEFAULT_ATLAS_SIZE);

        let (format, usage) = match kind {
            AtlasTextureKind::Monochrome => (vk::Format::R8_UNORM, vk::ImageUsageFlags::SAMPLED),
            AtlasTextureKind::Polychrome => {
                (vk::Format::B8G8R8A8_UNORM, vk::ImageUsageFlags::SAMPLED)
            }
            AtlasTextureKind::Path => (vk::Format::R16_SFLOAT, vk::ImageUsageFlags::SAMPLED),
        };

        // Create image
        let image_create_info = vk::ImageCreateInfo::builder()
            .image_type(vk::ImageType::TYPE_2D)
            .format(format)
            .extent(vk::Extent3D {
                width: size.width.into(),
                height: size.height.into(),
                depth: 1,
            })
            .mip_levels(1)
            .array_layers(1)
            .samples(vk::SampleCountFlags::TYPE_1)
            .tiling(vk::ImageTiling::OPTIMAL)
            .usage(usage)
            .sharing_mode(vk::SharingMode::EXCLUSIVE)
            .build();
        let image = unsafe { self.device.create_image(&image_create_info, None) }.unwrap();

        // Allocate memory
        let memory_requirements = unsafe { self.device.get_image_memory_requirements(image) };
        let mut memory_type_index = None;
        for (idx, memory_type) in self.memory_properties.memory_types
            [0..self.memory_properties.memory_type_count as usize]
            .iter()
            .enumerate()
        {
            // Ensure correct memory type
            let memory_type_bit_mask = 1u32 << idx;
            if memory_requirements.memory_type_bits & memory_type_bit_mask == 0 {
                continue;
            }

            // Ensure memory property requirements are met
            if (memory_type.property_flags & vk::MemoryPropertyFlags::DEVICE_LOCAL)
                != vk::MemoryPropertyFlags::DEVICE_LOCAL
            {
                continue;
            }

            memory_type_index = Some(idx);
            break;
        }

        let memory_allocate_info = vk::MemoryAllocateInfo::builder()
            .allocation_size(memory_requirements.size)
            .memory_type_index(match memory_type_index {
                Some(memory_type_index) => memory_type_index as u32,
                None => panic!("No valid memory type found"),
            })
            .build();
        let image_memory =
            unsafe { self.device.allocate_memory(&memory_allocate_info, None) }.unwrap();
        unsafe { self.device.bind_image_memory(image, image_memory, 0) }.unwrap();

        // Create image view
        let image_view_create_info = vk::ImageViewCreateInfo::builder()
            .image(image)
            .view_type(vk::ImageViewType::TYPE_2D)
            .format(format)
            .components(
                vk::ComponentMapping::builder()
                    .r(vk::ComponentSwizzle::IDENTITY)
                    .g(vk::ComponentSwizzle::IDENTITY)
                    .b(vk::ComponentSwizzle::IDENTITY)
                    .a(vk::ComponentSwizzle::IDENTITY)
                    .build(),
            )
            .subresource_range(
                vk::ImageSubresourceRange::builder()
                    .aspect_mask(vk::ImageAspectFlags::COLOR)
                    .base_mip_level(0)
                    .level_count(1)
                    .base_array_layer(0)
                    .layer_count(1)
                    .build(),
            )
            .build();
        let image_view =
            unsafe { self.device.create_image_view(&image_view_create_info, None) }.unwrap();

        let textures = match kind {
            AtlasTextureKind::Monochrome => &mut self.monochrome_textures,
            AtlasTextureKind::Polychrome => &mut self.polychrome_textures,
            AtlasTextureKind::Path => &mut self.path_textures,
        };
        let atlas_texture = VulkanAtlasTexture {
            id: AtlasTextureId {
                index: textures.len() as u32,
                kind,
            },
            allocator: etagere::BucketedAtlasAllocator::new(size.into()),
            image,
            image_memory,
            image_view,
        };
        textures.push(atlas_texture);
        textures.last_mut().unwrap()
    }

    fn texture(&self, id: AtlasTextureId) -> &VulkanAtlasTexture {
        let textures = match id.kind {
            AtlasTextureKind::Monochrome => &self.monochrome_textures,
            AtlasTextureKind::Polychrome => &self.polychrome_textures,
            AtlasTextureKind::Path => &self.path_textures,
        };
        &textures[id.index as usize]
    }

    fn destroy(&mut self) {
        self.monochrome_textures
            .drain(0..)
            .for_each(|texture| texture.destroy(&self.device));
        self.polychrome_textures
            .drain(0..)
            .for_each(|texture| texture.destroy(&self.device));
        self.path_textures
            .drain(0..)
            .for_each(|texture| texture.destroy(&self.device));
    }
}

struct VulkanAtlasTexture {
    id: AtlasTextureId,
    allocator: BucketedAtlasAllocator,
    image: vk::Image,
    image_memory: vk::DeviceMemory,
    image_view: vk::ImageView,
}

impl VulkanAtlasTexture {
    fn clear(&mut self) {
        self.allocator.clear();
    }

    fn allocate(&mut self, size: Size<DevicePixels>) -> Option<AtlasTile> {
        let allocation = self.allocator.allocate(size.into())?;
        let tile = AtlasTile {
            texture_id: self.id,
            tile_id: allocation.id.into(),
            bounds: Bounds {
                origin: allocation.rectangle.min.into(),
                size,
            },
            padding: 0,
        };
        Some(tile)
    }

    fn upload(&self, _bounds: Bounds<DevicePixels>, _bytes: &[u8]) {
        // todo!("vulkan")
    }

    fn destroy(&self, device: &ash::Device) {
        unsafe {
            device.destroy_image(self.image, None);
            device.destroy_image_view(self.image_view, None);
            device.free_memory(self.image_memory, None);
        }
    }
}

impl From<Size<DevicePixels>> for etagere::Size {
    fn from(size: Size<DevicePixels>) -> Self {
        etagere::Size::new(size.width.into(), size.height.into())
    }
}

impl From<etagere::Point> for Point<DevicePixels> {
    fn from(value: etagere::Point) -> Self {
        Point {
            x: DevicePixels::from(value.x),
            y: DevicePixels::from(value.y),
        }
    }
}
