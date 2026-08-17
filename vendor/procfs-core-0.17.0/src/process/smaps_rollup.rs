use super::MemoryMaps;
use crate::ProcResult;

#[derive(Debug)]
pub struct SmapsRollup {
    pub memory_map_rollup: MemoryMaps,
}

impl crate::FromBufRead for SmapsRollup {
    fn from_buf_read<R: std::io::BufRead>(r: R) -> ProcResult<Self> {
        MemoryMaps::from_buf_read(r).map(|m| SmapsRollup { memory_map_rollup: m })
    }
}
