use thiserror::Error;
use wgt::{
    error::{ErrorType, WebGpuError},
    AdapterInfo,
};

pub const HEADER_LENGTH: usize = size_of::<PipelineCacheHeader>();

#[derive(Debug, PartialEq, Eq, Clone, Error)]
#[non_exhaustive]
pub enum PipelineCacheValidationError {
    #[error("The pipeline cache data was truncated")]
    Truncated,
    #[error("The pipeline cache data was longer than recorded")]
    // TODO: Is it plausible that this would happen
    Extended,
    #[error("The pipeline cache data was corrupted (e.g. the hash didn't match)")]
    Corrupted,
    #[error("The pipeline cacha data was out of date and so cannot be safely used")]
    Outdated,
    #[error("The cache data was created for a different device")]
    DeviceMismatch,
    #[error("Pipeline cacha data was created for a future version of wgpu")]
    Unsupported,
}

impl PipelineCacheValidationError {
    /// Could the error have been avoided?
    /// That is, is there a mistake in user code interacting with the cache
    pub fn was_avoidable(&self) -> bool {
        match self {
            PipelineCacheValidationError::DeviceMismatch => true,
            PipelineCacheValidationError::Truncated
            | PipelineCacheValidationError::Unsupported
            | PipelineCacheValidationError::Extended
            // It's unusual, but not implausible, to be downgrading wgpu
            | PipelineCacheValidationError::Outdated
            | PipelineCacheValidationError::Corrupted => false,
        }
    }
}

impl WebGpuError for PipelineCacheValidationError {
    fn webgpu_error_type(&self) -> ErrorType {
        ErrorType::Validation
    }
}

/// Validate the data in a pipeline cache
pub fn validate_pipeline_cache<'d>(
    cache_data: &'d [u8],
    adapter: &AdapterInfo,
    validation_key: [u8; 16],
) -> Result<&'d [u8], PipelineCacheValidationError> {
    let adapter_key = adapter_key(adapter)?;
    let Some((header, remaining_data)) = PipelineCacheHeader::read(cache_data) else {
        return Err(PipelineCacheValidationError::Truncated);
    };
    if header.magic != MAGIC {
        return Err(PipelineCacheValidationError::Corrupted);
    }
    if header.header_version != HEADER_VERSION {
        return Err(PipelineCacheValidationError::Outdated);
    }
    if header.cache_abi != ABI {
        return Err(PipelineCacheValidationError::Outdated);
    }
    if header.backend != adapter.backend as u8 {
        return Err(PipelineCacheValidationError::DeviceMismatch);
    }
    if header.adapter_key != adapter_key {
        return Err(PipelineCacheValidationError::DeviceMismatch);
    }
    if header.validation_key != validation_key {
        // If the validation key is wrong, that means that this device has changed
        // in a way where the cache won't be compatible since the cache was made,
        // so it is outdated
        return Err(PipelineCacheValidationError::Outdated);
    }
    let data_size: usize = header
        .data_size
        .try_into()
        // If the data was previously more than 4GiB, and we're still on a 32 bit system (ABI check, above)
        // Then the data must be corrupted
        .map_err(|_| PipelineCacheValidationError::Corrupted)?;
    if remaining_data.len() < data_size {
        return Err(PipelineCacheValidationError::Truncated);
    }
    if remaining_data.len() > data_size {
        return Err(PipelineCacheValidationError::Extended);
    }
    if header.hash_space != HASH_SPACE_VALUE {
        return Err(PipelineCacheValidationError::Corrupted);
    }
    Ok(remaining_data)
}

pub fn add_cache_header(
    in_region: &mut [u8],
    data: &[u8],
    adapter: &AdapterInfo,
    validation_key: [u8; 16],
) {
    assert_eq!(in_region.len(), HEADER_LENGTH);
    let header = PipelineCacheHeader {
        adapter_key: adapter_key(adapter)
            .expect("Called add_cache_header for an adapter which doesn't support cache data. This is a wgpu internal bug"),
        backend: adapter.backend as u8,
        cache_abi: ABI,
        magic: MAGIC,
        header_version: HEADER_VERSION,
        validation_key,
        hash_space: HASH_SPACE_VALUE,
        data_size: data
            .len()
            .try_into()
            .expect("Cache larger than u64::MAX bytes"),
    };
    header.write(in_region);
}

const MAGIC: [u8; 8] = *b"WGPUPLCH";
const HEADER_VERSION: u32 = 1;
const ABI: u32 = size_of::<*const ()>() as u32;

/// The value used to fill [`PipelineCacheHeader::hash_space`]
///
/// If we receive reports of pipeline cache data corruption which is not otherwise caught
/// on a real device, it would be worth modifying this
///
/// Note that wgpu does not protect against malicious writes to e.g. a file used
/// to store a pipeline cache.
/// That is the responsibility of the end application, such as by using a
/// private space.
const HASH_SPACE_VALUE: u64 = 0xFEDCBA9_876543210;

#[repr(C)]
#[derive(PartialEq, Eq)]
struct PipelineCacheHeader {
    /// The magic header to ensure that we have the right file format
    /// Has a value of MAGIC, as above
    magic: [u8; 8],
    // /// The total size of this header, in bytes
    // header_size: u32,
    /// The version of this wgpu header
    /// Should be equal to HEADER_VERSION above
    ///
    /// This must always be the second item, after the value above
    header_version: u32,
    /// The number of bytes in the pointers of this ABI, because some drivers
    /// have previously not distinguished between their 32 bit and 64 bit drivers
    /// leading to Vulkan data corruption
    cache_abi: u32,
    /// The id for the backend in use, from [wgt::Backend]
    backend: u8,
    /// The key which identifiers the device/adapter.
    /// This is used to validate that this pipeline cache (probably) was produced for
    /// the expected device.
    /// On Vulkan: it is a combination of vendor ID and device ID
    adapter_key: [u8; 15],
    /// A key used to validate that this device is still compatible with the cache
    ///
    /// This should e.g. contain driver version and/or intermediate compiler versions
    validation_key: [u8; 16],
    /// The length of the data which is sent to/recieved from the backend
    data_size: u64,
    /// Space reserved for a hash of the data in future
    ///
    /// We assume that your cache storage system will be relatively robust, and so
    /// do not validate this hash
    ///
    /// Therefore, this will always have a value of [`HASH_SPACE_VALUE`]
    hash_space: u64,
}

impl PipelineCacheHeader {
    fn read(data: &[u8]) -> Option<(PipelineCacheHeader, &[u8])> {
        let mut reader = Reader {
            data,
            total_read: 0,
        };
        let magic = reader.read_array()?;
        let header_version = reader.read_u32()?;
        let cache_abi = reader.read_u32()?;
        let backend = reader.read_byte()?;
        let adapter_key = reader.read_array()?;
        let validation_key = reader.read_array()?;
        let data_size = reader.read_u64()?;
        let data_hash = reader.read_u64()?;

        assert_eq!(reader.total_read, size_of::<PipelineCacheHeader>());

        Some((
            PipelineCacheHeader {
                magic,
                header_version,
                cache_abi,
                backend,
                adapter_key,
                validation_key,
                data_size,
                hash_space: data_hash,
            },
            reader.data,
        ))
    }

    fn write(&self, into: &mut [u8]) -> Option<()> {
        let mut writer = Writer { data: into };
        writer.write_array(&self.magic)?;
        writer.write_u32(self.header_version)?;
        writer.write_u32(self.cache_abi)?;
        writer.write_byte(self.backend)?;
        writer.write_array(&self.adapter_key)?;
        writer.write_array(&self.validation_key)?;
        writer.write_u64(self.data_size)?;
        writer.write_u64(self.hash_space)?;

        assert_eq!(writer.data.len(), 0);
        Some(())
    }
}

fn adapter_key(adapter: &AdapterInfo) -> Result<[u8; 15], PipelineCacheValidationError> {
    match adapter.backend {
        wgt::Backend::Vulkan => {
            // If these change size, the header format needs to change
            // We set the type explicitly so this won't compile in that case
            let v: [u8; 4] = adapter.vendor.to_be_bytes();
            let d: [u8; 4] = adapter.device.to_be_bytes();
            let adapter = [
                255, 255, 255, v[0], v[1], v[2], v[3], d[0], d[1], d[2], d[3], 255, 255, 255, 255,
            ];
            Ok(adapter)
        }
        _ => Err(PipelineCacheValidationError::Unsupported),
    }
}

struct Reader<'a> {
    data: &'a [u8],
    total_read: usize,
}

impl<'a> Reader<'a> {
    fn read_byte(&mut self) -> Option<u8> {
        let res = *self.data.first()?;
        self.total_read += 1;
        self.data = &self.data[1..];
        Some(res)
    }
    fn read_array<const N: usize>(&mut self) -> Option<[u8; N]> {
        // Only greater than because we're indexing fenceposts, not items
        if N > self.data.len() {
            return None;
        }
        let (start, data) = self.data.split_at(N);
        self.total_read += N;
        self.data = data;
        Some(start.try_into().expect("off-by-one-error in array size"))
    }

    // fn read_u16(&mut self) -> Option<u16> {
    //     self.read_array().map(u16::from_be_bytes)
    // }
    fn read_u32(&mut self) -> Option<u32> {
        self.read_array().map(u32::from_be_bytes)
    }
    fn read_u64(&mut self) -> Option<u64> {
        self.read_array().map(u64::from_be_bytes)
    }
}

struct Writer<'a> {
    data: &'a mut [u8],
}

impl<'a> Writer<'a> {
    fn write_byte(&mut self, byte: u8) -> Option<()> {
        self.write_array(&[byte])
    }
    fn write_array<const N: usize>(&mut self, array: &[u8; N]) -> Option<()> {
        // Only greater than because we're indexing fenceposts, not items
        if N > self.data.len() {
            return None;
        }
        let data = core::mem::take(&mut self.data);
        let (start, data) = data.split_at_mut(N);
        self.data = data;
        start.copy_from_slice(array);
        Some(())
    }

    // fn write_u16(&mut self, value: u16) -> Option<()> {
    //     self.write_array(&value.to_be_bytes())
    // }
    fn write_u32(&mut self, value: u32) -> Option<()> {
        self.write_array(&value.to_be_bytes())
    }
    fn write_u64(&mut self, value: u64) -> Option<()> {
        self.write_array(&value.to_be_bytes())
    }
}

#[cfg(test)]
mod tests {
    use alloc::{string::String, vec::Vec};
    use wgt::AdapterInfo;

    use crate::pipeline_cache::{PipelineCacheValidationError as E, HEADER_LENGTH};

    use super::ABI;

    // Assert the correct size
    const _: [(); HEADER_LENGTH] = [(); 64];

    const ADAPTER: AdapterInfo = AdapterInfo {
        name: String::new(),
        vendor: 0x0002_FEED,
        device: 0xFEFE_FEFE,
        device_type: wgt::DeviceType::Other,
        device_pci_bus_id: String::new(),
        driver: String::new(),
        driver_info: String::new(),
        backend: wgt::Backend::Vulkan,
        subgroup_min_size: 32,
        subgroup_max_size: 32,
        transient_saves_memory: true,
    };

    // IMPORTANT: If these tests fail, then you MUST increment HEADER_VERSION
    const VALIDATION_KEY: [u8; 16] = u128::to_be_bytes(0xFFFFFFFF_FFFFFFFF_88888888_88888888);
    #[test]
    fn written_header() {
        let mut result = [0; HEADER_LENGTH];
        super::add_cache_header(&mut result, &[], &ADAPTER, VALIDATION_KEY);
        let cache: [[u8; 8]; HEADER_LENGTH / 8] = [
            *b"WGPUPLCH",                                 // MAGIC
            [0, 0, 0, 1, 0, 0, 0, ABI as u8],             // Version and ABI
            [1, 255, 255, 255, 0, 2, 0xFE, 0xED],         // Backend and Adapter key
            [0xFE, 0xFE, 0xFE, 0xFE, 255, 255, 255, 255], // Backend and Adapter key
            0xFFFFFFFF_FFFFFFFFu64.to_be_bytes(),         // Validation key
            0x88888888_88888888u64.to_be_bytes(),         // Validation key
            0x0u64.to_be_bytes(),                         // Data size
            0xFEDCBA9_876543210u64.to_be_bytes(),         // Hash
        ];
        let expected = cache.into_iter().flatten().collect::<Vec<u8>>();

        assert_eq!(result.as_slice(), expected.as_slice());
    }

    #[test]
    fn valid_data() {
        let cache: [[u8; 8]; HEADER_LENGTH / 8] = [
            *b"WGPUPLCH",                                 // MAGIC
            [0, 0, 0, 1, 0, 0, 0, ABI as u8],             // Version and ABI
            [1, 255, 255, 255, 0, 2, 0xFE, 0xED],         // Backend and Adapter key
            [0xFE, 0xFE, 0xFE, 0xFE, 255, 255, 255, 255], // Backend and Adapter key
            0xFFFFFFFF_FFFFFFFFu64.to_be_bytes(),         // Validation key
            0x88888888_88888888u64.to_be_bytes(),         // Validation key
            0x0u64.to_be_bytes(),                         // Data size
            0xFEDCBA9_876543210u64.to_be_bytes(),         // Hash
        ];
        let cache = cache.into_iter().flatten().collect::<Vec<u8>>();
        let expected: &[u8] = &[];
        let validation_result = super::validate_pipeline_cache(&cache, &ADAPTER, VALIDATION_KEY);
        assert_eq!(validation_result, Ok(expected));
    }
    #[test]
    fn invalid_magic() {
        let cache: [[u8; 8]; HEADER_LENGTH / 8] = [
            *b"NOT_WGPU",                                 // (Wrong) MAGIC
            [0, 0, 0, 1, 0, 0, 0, ABI as u8],             // Version and ABI
            [1, 255, 255, 255, 0, 2, 0xFE, 0xED],         // Backend and Adapter key
            [0xFE, 0xFE, 0xFE, 0xFE, 255, 255, 255, 255], // Backend and Adapter key
            0xFFFFFFFF_FFFFFFFFu64.to_be_bytes(),         // Validation key
            0x88888888_88888888u64.to_be_bytes(),         // Validation key
            0x0u64.to_be_bytes(),                         // Data size
            0xFEDCBA9_876543210u64.to_be_bytes(),         // Hash
        ];
        let cache = cache.into_iter().flatten().collect::<Vec<u8>>();
        let validation_result = super::validate_pipeline_cache(&cache, &ADAPTER, VALIDATION_KEY);
        assert_eq!(validation_result, Err(E::Corrupted));
    }

    #[test]
    fn wrong_version() {
        let cache: [[u8; 8]; HEADER_LENGTH / 8] = [
            *b"WGPUPLCH",                                 // MAGIC
            [0, 0, 0, 2, 0, 0, 0, ABI as u8],             // (wrong) Version and ABI
            [1, 255, 255, 255, 0, 2, 0xFE, 0xED],         // Backend and Adapter key
            [0xFE, 0xFE, 0xFE, 0xFE, 255, 255, 255, 255], // Backend and Adapter key
            0xFFFFFFFF_FFFFFFFFu64.to_be_bytes(),         // Validation key
            0x88888888_88888888u64.to_be_bytes(),         // Validation key
            0x0u64.to_be_bytes(),                         // Data size
            0xFEDCBA9_876543210u64.to_be_bytes(),         // Hash
        ];
        let cache = cache.into_iter().flatten().collect::<Vec<u8>>();
        let validation_result = super::validate_pipeline_cache(&cache, &ADAPTER, VALIDATION_KEY);
        assert_eq!(validation_result, Err(E::Outdated));
    }
    #[test]
    fn wrong_abi() {
        let cache: [[u8; 8]; HEADER_LENGTH / 8] = [
            *b"WGPUPLCH", // MAGIC
            // a 14 bit ABI is improbable
            [0, 0, 0, 1, 0, 0, 0, 14],            // Version and (wrong) ABI
            [1, 255, 255, 255, 0, 2, 0xFE, 0xED], // Backend and Adapter key
            [0xFE, 0xFE, 0xFE, 0xFE, 255, 255, 255, 255], // Backend and Adapter key
            0xFFFFFFFF_FFFFFFFFu64.to_be_bytes(), // Validation key
            0x88888888_88888888u64.to_be_bytes(), // Validation key
            0x0u64.to_be_bytes(),                 // Data size
            0xFEDCBA9_876543210u64.to_be_bytes(), // Header
        ];
        let cache = cache.into_iter().flatten().collect::<Vec<u8>>();
        let validation_result = super::validate_pipeline_cache(&cache, &ADAPTER, VALIDATION_KEY);
        assert_eq!(validation_result, Err(E::Outdated));
    }

    #[test]
    fn wrong_backend() {
        let cache: [[u8; 8]; HEADER_LENGTH / 8] = [
            *b"WGPUPLCH",                                 // MAGIC
            [0, 0, 0, 1, 0, 0, 0, ABI as u8],             // Version and ABI
            [2, 255, 255, 255, 0, 2, 0xFE, 0xED],         // (wrong) Backend and Adapter key
            [0xFE, 0xFE, 0xFE, 0xFE, 255, 255, 255, 255], // Backend and Adapter key
            0xFFFFFFFF_FFFFFFFFu64.to_be_bytes(),         // Validation key
            0x88888888_88888888u64.to_be_bytes(),         // Validation key
            0x0u64.to_be_bytes(),                         // Data size
            0xFEDCBA9_876543210u64.to_be_bytes(),         // Hash
        ];
        let cache = cache.into_iter().flatten().collect::<Vec<u8>>();
        let validation_result = super::validate_pipeline_cache(&cache, &ADAPTER, VALIDATION_KEY);
        assert_eq!(validation_result, Err(E::DeviceMismatch));
    }
    #[test]
    fn wrong_adapter() {
        let cache: [[u8; 8]; HEADER_LENGTH / 8] = [
            *b"WGPUPLCH",                                 // MAGIC
            [0, 0, 0, 1, 0, 0, 0, ABI as u8],             // Version and ABI
            [1, 255, 255, 255, 0, 2, 0xFE, 0x00],         // Backend and (wrong) Adapter key
            [0xFE, 0xFE, 0xFE, 0xFE, 255, 255, 255, 255], // Backend and Adapter key
            0xFFFFFFFF_FFFFFFFFu64.to_be_bytes(),         // Validation key
            0x88888888_88888888u64.to_be_bytes(),         // Validation key
            0x0u64.to_be_bytes(),                         // Data size
            0xFEDCBA9_876543210u64.to_be_bytes(),         // Hash
        ];
        let cache = cache.into_iter().flatten().collect::<Vec<u8>>();
        let validation_result = super::validate_pipeline_cache(&cache, &ADAPTER, VALIDATION_KEY);
        assert_eq!(validation_result, Err(E::DeviceMismatch));
    }
    #[test]
    fn wrong_validation() {
        let cache: [[u8; 8]; HEADER_LENGTH / 8] = [
            *b"WGPUPLCH",                                 // MAGIC
            [0, 0, 0, 1, 0, 0, 0, ABI as u8],             // Version and ABI
            [1, 255, 255, 255, 0, 2, 0xFE, 0xED],         // Backend and Adapter key
            [0xFE, 0xFE, 0xFE, 0xFE, 255, 255, 255, 255], // Backend and Adapter key
            0xFFFFFFFF_FFFFFFFFu64.to_be_bytes(),         // Validation key
            0x88888888_00000000u64.to_be_bytes(),         // (wrong) Validation key
            0x0u64.to_be_bytes(),                         // Data size
            0xFEDCBA9_876543210u64.to_be_bytes(),         // Hash
        ];
        let cache = cache.into_iter().flatten().collect::<Vec<u8>>();
        let validation_result = super::validate_pipeline_cache(&cache, &ADAPTER, VALIDATION_KEY);
        assert_eq!(validation_result, Err(E::Outdated));
    }
    #[test]
    fn too_little_data() {
        let cache: [[u8; 8]; HEADER_LENGTH / 8] = [
            *b"WGPUPLCH",                                 // MAGIC
            [0, 0, 0, 1, 0, 0, 0, ABI as u8],             // Version and ABI
            [1, 255, 255, 255, 0, 2, 0xFE, 0xED],         // Backend and Adapter key
            [0xFE, 0xFE, 0xFE, 0xFE, 255, 255, 255, 255], // Backend and Adapter key
            0xFFFFFFFF_FFFFFFFFu64.to_be_bytes(),         // Validation key
            0x88888888_88888888u64.to_be_bytes(),         // Validation key
            0x064u64.to_be_bytes(),                       // Data size
            0xFEDCBA9_876543210u64.to_be_bytes(),         // Hash
        ];
        let cache = cache.into_iter().flatten().collect::<Vec<u8>>();
        let validation_result = super::validate_pipeline_cache(&cache, &ADAPTER, VALIDATION_KEY);
        assert_eq!(validation_result, Err(E::Truncated));
    }
    #[test]
    fn not_no_data() {
        let cache: [[u8; 8]; HEADER_LENGTH / 8] = [
            *b"WGPUPLCH",                                 // MAGIC
            [0, 0, 0, 1, 0, 0, 0, ABI as u8],             // Version and ABI
            [1, 255, 255, 255, 0, 2, 0xFE, 0xED],         // Backend and Adapter key
            [0xFE, 0xFE, 0xFE, 0xFE, 255, 255, 255, 255], // Backend and Adapter key
            0xFFFFFFFF_FFFFFFFFu64.to_be_bytes(),         // Validation key
            0x88888888_88888888u64.to_be_bytes(),         // Validation key
            100u64.to_be_bytes(),                         // Data size
            0xFEDCBA9_876543210u64.to_be_bytes(),         // Hash
        ];
        let cache = cache
            .into_iter()
            .flatten()
            .chain(core::iter::repeat_n(0u8, 100))
            .collect::<Vec<u8>>();
        let validation_result = super::validate_pipeline_cache(&cache, &ADAPTER, VALIDATION_KEY);
        let expected: &[u8] = &[0; 100];
        assert_eq!(validation_result, Ok(expected));
    }
    #[test]
    fn too_much_data() {
        let cache: [[u8; 8]; HEADER_LENGTH / 8] = [
            *b"WGPUPLCH",                                 // MAGIC
            [0, 0, 0, 1, 0, 0, 0, ABI as u8],             // Version and ABI
            [1, 255, 255, 255, 0, 2, 0xFE, 0xED],         // Backend and Adapter key
            [0xFE, 0xFE, 0xFE, 0xFE, 255, 255, 255, 255], // Backend and Adapter key
            0xFFFFFFFF_FFFFFFFFu64.to_be_bytes(),         // Validation key
            0x88888888_88888888u64.to_be_bytes(),         // Validation key
            0x064u64.to_be_bytes(),                       // Data size
            0xFEDCBA9_876543210u64.to_be_bytes(),         // Hash
        ];
        let cache = cache
            .into_iter()
            .flatten()
            .chain(core::iter::repeat_n(0u8, 200))
            .collect::<Vec<u8>>();
        let validation_result = super::validate_pipeline_cache(&cache, &ADAPTER, VALIDATION_KEY);
        assert_eq!(validation_result, Err(E::Extended));
    }
    #[test]
    fn wrong_hash() {
        let cache: [[u8; 8]; HEADER_LENGTH / 8] = [
            *b"WGPUPLCH",                                 // MAGIC
            [0, 0, 0, 1, 0, 0, 0, ABI as u8],             // Version and ABI
            [1, 255, 255, 255, 0, 2, 0xFE, 0xED],         // Backend and Adapter key
            [0xFE, 0xFE, 0xFE, 0xFE, 255, 255, 255, 255], // Backend and Adapter key
            0xFFFFFFFF_FFFFFFFFu64.to_be_bytes(),         // Validation key
            0x88888888_88888888u64.to_be_bytes(),         // Validation key
            0x0u64.to_be_bytes(),                         // Data size
            0x00000000_00000000u64.to_be_bytes(),         // Hash
        ];
        let cache = cache.into_iter().flatten().collect::<Vec<u8>>();
        let validation_result = super::validate_pipeline_cache(&cache, &ADAPTER, VALIDATION_KEY);
        assert_eq!(validation_result, Err(E::Corrupted));
    }
}
