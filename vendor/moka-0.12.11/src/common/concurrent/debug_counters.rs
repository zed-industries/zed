use crossbeam_utils::atomic::AtomicCell;
use once_cell::sync::Lazy;

#[derive(Clone, Debug)]
pub struct GlobalDebugCounters {
    pub bucket_array_creation_count: u64,
    pub bucket_array_allocation_bytes: u64,
    pub bucket_array_drop_count: u64,
    pub bucket_array_release_bytes: u64,
    pub bucket_creation_count: u64,
    pub bucket_drop_count: u64,
    pub value_entry_creation_count: u64,
    pub value_entry_drop_count: u64,
    pub entry_info_creation_count: u64,
    pub entry_info_drop_count: u64,
    pub deq_node_creation_count: u64,
    pub deq_node_drop_count: u64,
}

impl GlobalDebugCounters {
    pub fn current() -> Self {
        InternalGlobalDebugCounters::current()
    }
}

static COUNTERS: Lazy<InternalGlobalDebugCounters> =
    Lazy::new(InternalGlobalDebugCounters::default);

#[derive(Default)]
pub(crate) struct InternalGlobalDebugCounters {
    bucket_array_creation_count: AtomicCell<u64>,
    bucket_array_allocation_bytes: AtomicCell<u64>,
    bucket_array_drop_count: AtomicCell<u64>,
    bucket_array_release_bytes: AtomicCell<u64>,
    bucket_creation_count: AtomicCell<u64>,
    bucket_drop_count: AtomicCell<u64>,
    value_entry_creation_count: AtomicCell<u64>,
    value_entry_drop_count: AtomicCell<u64>,
    entry_info_creation_count: AtomicCell<u64>,
    entry_info_drop_count: AtomicCell<u64>,
    deq_node_creation_count: AtomicCell<u64>,
    deq_node_drop_count: AtomicCell<u64>,
}

impl InternalGlobalDebugCounters {
    fn current() -> GlobalDebugCounters {
        let c = &COUNTERS;
        GlobalDebugCounters {
            bucket_array_creation_count: c.bucket_array_creation_count.load(),
            bucket_array_allocation_bytes: c.bucket_array_allocation_bytes.load(),
            bucket_array_drop_count: c.bucket_array_drop_count.load(),
            bucket_array_release_bytes: c.bucket_array_release_bytes.load(),
            bucket_creation_count: c.bucket_creation_count.load(),
            bucket_drop_count: c.bucket_drop_count.load(),
            value_entry_creation_count: c.value_entry_creation_count.load(),
            value_entry_drop_count: c.value_entry_drop_count.load(),
            entry_info_creation_count: c.entry_info_creation_count.load(),
            entry_info_drop_count: c.entry_info_drop_count.load(),
            deq_node_creation_count: c.deq_node_creation_count.load(),
            deq_node_drop_count: c.deq_node_drop_count.load(),
        }
    }

    pub(crate) fn bucket_array_created(byte_size: u64) {
        COUNTERS.bucket_array_creation_count.fetch_add(1);
        COUNTERS.bucket_array_allocation_bytes.fetch_add(byte_size);
    }

    pub(crate) fn bucket_array_dropped(byte_size: u64) {
        COUNTERS.bucket_array_drop_count.fetch_add(1);
        COUNTERS.bucket_array_release_bytes.fetch_add(byte_size);
    }

    pub(crate) fn bucket_created() {
        COUNTERS.bucket_creation_count.fetch_add(1);
    }

    pub(crate) fn bucket_dropped() {
        COUNTERS.bucket_drop_count.fetch_add(1);
    }

    pub(crate) fn value_entry_created() {
        COUNTERS.value_entry_creation_count.fetch_add(1);
    }

    pub(crate) fn value_entry_dropped() {
        COUNTERS.value_entry_drop_count.fetch_add(1);
    }

    pub(crate) fn entry_info_created() {
        COUNTERS.entry_info_creation_count.fetch_add(1);
    }

    pub(crate) fn entry_info_dropped() {
        COUNTERS.entry_info_drop_count.fetch_add(1);
    }

    pub(crate) fn deq_node_created() {
        COUNTERS.deq_node_creation_count.fetch_add(1);
    }

    pub(crate) fn deq_node_dropped() {
        COUNTERS.deq_node_drop_count.fetch_add(1);
    }
}

#[derive(Clone, Debug)]
pub struct CacheDebugStats {
    pub entry_count: u64,
    pub weighted_size: u64,
    // bytes
    pub freq_sketch_size: u64,
    // max entries
    pub hashmap_capacity: u64,
}

impl CacheDebugStats {
    pub(crate) fn new(
        entry_count: u64,
        weighted_size: u64,
        hashmap_capacity: u64,
        freq_sketch_size: u64,
    ) -> Self {
        Self {
            entry_count,
            weighted_size,
            freq_sketch_size,
            hashmap_capacity,
        }
    }
}
