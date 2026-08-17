//! Sampler cache for Vulkan backend.
//!
//! Nearly identical to the DX12 sampler cache, without descriptor heap management.

use ash::vk;
use hashbrown::{hash_map::Entry, HashMap};
use ordered_float::OrderedFloat;

/// If the allowed sampler count is above this value, the sampler cache is disabled.
const ENABLE_SAMPLER_CACHE_CUTOFF: u32 = 1 << 20;

/// [`vk::SamplerCreateInfo`] is not hashable, so we wrap it in a newtype that is.
///
/// We use [`OrderedFloat`] to allow for floating point values to be compared and
/// hashed in a defined way.
#[derive(Copy, Clone)]
struct HashableSamplerCreateInfo(vk::SamplerCreateInfo<'static>);

impl PartialEq for HashableSamplerCreateInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0.flags == other.0.flags
            && self.0.mag_filter == other.0.mag_filter
            && self.0.min_filter == other.0.min_filter
            && self.0.mipmap_mode == other.0.mipmap_mode
            && self.0.address_mode_u == other.0.address_mode_u
            && self.0.address_mode_v == other.0.address_mode_v
            && self.0.address_mode_w == other.0.address_mode_w
            && OrderedFloat(self.0.mip_lod_bias) == OrderedFloat(other.0.mip_lod_bias)
            && self.0.anisotropy_enable == other.0.anisotropy_enable
            && OrderedFloat(self.0.max_anisotropy) == OrderedFloat(other.0.max_anisotropy)
            && self.0.compare_enable == other.0.compare_enable
            && self.0.compare_op == other.0.compare_op
            && OrderedFloat(self.0.min_lod) == OrderedFloat(other.0.min_lod)
            && OrderedFloat(self.0.max_lod) == OrderedFloat(other.0.max_lod)
            && self.0.border_color == other.0.border_color
            && self.0.unnormalized_coordinates == other.0.unnormalized_coordinates
    }
}

impl Eq for HashableSamplerCreateInfo {}

impl core::hash::Hash for HashableSamplerCreateInfo {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.0.flags.hash(state);
        self.0.mag_filter.hash(state);
        self.0.min_filter.hash(state);
        self.0.mipmap_mode.hash(state);
        self.0.address_mode_u.hash(state);
        self.0.address_mode_v.hash(state);
        self.0.address_mode_w.hash(state);
        OrderedFloat(self.0.mip_lod_bias).hash(state);
        self.0.anisotropy_enable.hash(state);
        OrderedFloat(self.0.max_anisotropy).hash(state);
        self.0.compare_enable.hash(state);
        self.0.compare_op.hash(state);
        OrderedFloat(self.0.min_lod).hash(state);
        OrderedFloat(self.0.max_lod).hash(state);
        self.0.border_color.hash(state);
        self.0.unnormalized_coordinates.hash(state);
    }
}

/// Entry in the sampler cache.
struct CacheEntry {
    sampler: vk::Sampler,
    ref_count: u32,
}

/// Global sampler cache.
///
/// As some devices have a low limit (4000) on the number of unique samplers that can be created,
/// we need to cache samplers to avoid running out if people eagerly create duplicate samplers.
pub(crate) struct SamplerCache {
    /// Mapping from the sampler description to sampler and reference count.
    samplers: HashMap<HashableSamplerCreateInfo, CacheEntry>,
    /// Maximum number of unique samplers that can be created.
    total_capacity: u32,
    /// If true, the sampler cache is disabled and all samplers are created on demand.
    passthrough: bool,
}

impl SamplerCache {
    pub fn new(total_capacity: u32) -> Self {
        let passthrough = total_capacity >= ENABLE_SAMPLER_CACHE_CUTOFF;
        Self {
            samplers: HashMap::new(),
            total_capacity,
            passthrough,
        }
    }

    /// Create a sampler, or return an existing one if it already exists.
    ///
    /// If the sampler already exists, the reference count is incremented.
    ///
    /// If the sampler does not exist, a new sampler is created and inserted into the cache.
    ///
    /// If the cache is full, an error is returned.
    pub fn create_sampler(
        &mut self,
        device: &ash::Device,
        create_info: vk::SamplerCreateInfo<'static>,
    ) -> Result<vk::Sampler, crate::DeviceError> {
        if self.passthrough {
            return unsafe { device.create_sampler(&create_info, None) }
                .map_err(super::map_host_device_oom_and_ioca_err);
        };

        // Get the number of used samplers. Needs to be done before to appease the borrow checker.
        let used_samplers = self.samplers.len();

        match self.samplers.entry(HashableSamplerCreateInfo(create_info)) {
            Entry::Occupied(occupied_entry) => {
                // We have found a match, so increment the refcount and return the index.
                let value = occupied_entry.into_mut();
                value.ref_count += 1;
                Ok(value.sampler)
            }
            Entry::Vacant(vacant_entry) => {
                // We need to create a new sampler.

                // We need to check if we can create more samplers.
                if used_samplers >= self.total_capacity as usize {
                    log::error!("There is no more room in the global sampler heap for more unique samplers. Your device supports a maximum of {} unique samplers.", self.samplers.len());
                    return Err(crate::DeviceError::OutOfMemory);
                }

                // Create the sampler.
                let sampler = unsafe { device.create_sampler(&create_info, None) }
                    .map_err(super::map_host_device_oom_and_ioca_err)?;

                // Insert the new sampler into the mapping.
                vacant_entry.insert(CacheEntry {
                    sampler,
                    ref_count: 1,
                });

                Ok(sampler)
            }
        }
    }

    /// Decrease the reference count of a sampler and destroy it if the reference count reaches 0.
    ///
    /// The provided sampler is checked against the sampler in the cache to ensure there is no clerical error.
    pub fn destroy_sampler(
        &mut self,
        device: &ash::Device,
        create_info: vk::SamplerCreateInfo<'static>,
        provided_sampler: vk::Sampler,
    ) {
        if self.passthrough {
            unsafe { device.destroy_sampler(provided_sampler, None) };
            return;
        };

        let Entry::Occupied(mut hash_map_entry) =
            self.samplers.entry(HashableSamplerCreateInfo(create_info))
        else {
            log::error!("Trying to destroy a sampler that does not exist.");
            return;
        };
        let cache_entry = hash_map_entry.get_mut();

        assert_eq!(
            cache_entry.sampler, provided_sampler,
            "Provided sampler does not match the sampler in the cache."
        );

        cache_entry.ref_count -= 1;

        if cache_entry.ref_count == 0 {
            unsafe { device.destroy_sampler(cache_entry.sampler, None) };
            hash_map_entry.remove();
        }
    }
}
