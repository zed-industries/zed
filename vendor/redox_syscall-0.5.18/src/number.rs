pub const SYS_CLASS: usize = 0xF000_0000;
pub const SYS_CLASS_PATH: usize = 0x1000_0000;
pub const SYS_CLASS_FILE: usize = 0x2000_0000;

pub const SYS_ARG: usize = 0x0F00_0000;
pub const SYS_ARG_SLICE: usize = 0x0100_0000;
pub const SYS_ARG_MSLICE: usize = 0x0200_0000;
pub const SYS_ARG_PATH: usize = 0x0300_0000;

pub const SYS_RET: usize = 0x00F0_0000;
pub const SYS_RET_FILE: usize = 0x0010_0000;

pub const SYS_OPEN: usize = SYS_CLASS_PATH | SYS_RET_FILE | 5;
pub const SYS_OPENAT: usize = SYS_CLASS_PATH | SYS_RET_FILE | 7;
pub const SYS_RMDIR: usize = SYS_CLASS_PATH | 84;
pub const SYS_UNLINK: usize = SYS_CLASS_PATH | 10;

pub const SYS_CLOSE: usize = SYS_CLASS_FILE | 6;
pub const SYS_DUP: usize = SYS_CLASS_FILE | SYS_RET_FILE | 41;
pub const SYS_DUP2: usize = SYS_CLASS_FILE | SYS_RET_FILE | 63;
pub const SYS_READ: usize = SYS_CLASS_FILE | SYS_ARG_MSLICE | 3;
pub const SYS_READ2: usize = SYS_CLASS_FILE | SYS_ARG_MSLICE | 35;
pub const SYS_WRITE: usize = SYS_CLASS_FILE | SYS_ARG_SLICE | 4;
pub const SYS_WRITE2: usize = SYS_CLASS_FILE | SYS_ARG_SLICE | 45;
pub const SYS_LSEEK: usize = SYS_CLASS_FILE | 19;
pub const SYS_FCHMOD: usize = SYS_CLASS_FILE | 94;
pub const SYS_FCHOWN: usize = SYS_CLASS_FILE | 207;
pub const SYS_FCNTL: usize = SYS_CLASS_FILE | 55;
pub const SYS_FEVENT: usize = SYS_CLASS_FILE | 927;

// SYS_CALL, fd, inout buf ptr, inout buf len, flags, metadata buf ptr, metadata buf len
pub const SYS_CALL: usize = SYS_CLASS_FILE | SYS_ARG_SLICE | SYS_ARG_MSLICE | 0xCA11;

pub const SYS_SENDFD: usize = SYS_CLASS_FILE | 34;
pub const SYS_GETDENTS: usize = SYS_CLASS_FILE | 43;

// TODO: Rename FMAP/FUNMAP to MMAP/MUNMAP
pub const SYS_FMAP_OLD: usize = SYS_CLASS_FILE | SYS_ARG_SLICE | 90;
pub const SYS_FMAP: usize = SYS_CLASS_FILE | SYS_ARG_SLICE | 900;
// TODO: SYS_FUNMAP should be SYS_CLASS_FILE
// TODO: Remove FMAP/FMAP_OLD
pub const SYS_FUNMAP_OLD: usize = SYS_CLASS_FILE | 91;
pub const SYS_FUNMAP: usize = SYS_CLASS_FILE | 92;
pub const SYS_MREMAP: usize = 155;

pub const SYS_FLINK: usize = SYS_CLASS_FILE | SYS_ARG_PATH | 9;
pub const SYS_FPATH: usize = SYS_CLASS_FILE | SYS_ARG_MSLICE | 928;
pub const SYS_FRENAME: usize = SYS_CLASS_FILE | SYS_ARG_PATH | 38;
pub const SYS_FSTAT: usize = SYS_CLASS_FILE | SYS_ARG_MSLICE | 28;
pub const SYS_FSTATVFS: usize = SYS_CLASS_FILE | SYS_ARG_MSLICE | 100;
pub const SYS_FSYNC: usize = SYS_CLASS_FILE | 118;
pub const SYS_FTRUNCATE: usize = SYS_CLASS_FILE | 93;
pub const SYS_FUTIMENS: usize = SYS_CLASS_FILE | SYS_ARG_SLICE | 320;

// b = file, c = flags, d = required_page_count, uid:gid = offset
pub const KSMSG_MMAP: usize = SYS_CLASS_FILE | 72;

// b = file, c = flags, d = page_count, uid:gid = offset
pub const KSMSG_MSYNC: usize = SYS_CLASS_FILE | 73;

// b = file, c = page_count, uid:gid = offset
pub const KSMSG_MUNMAP: usize = SYS_CLASS_FILE | 74;

// b = file, c = flags, d = page_count, uid:gid = offset
pub const KSMSG_MMAP_PREP: usize = SYS_CLASS_FILE | 75;

// b = target_packetid_lo32, c = target_packetid_hi32
pub const KSMSG_CANCEL: usize = SYS_CLASS_FILE | 76;

pub const SYS_CLOCK_GETTIME: usize = 265;
pub const SYS_FUTEX: usize = 240;
pub const SYS_MPROTECT: usize = 125;
pub const SYS_MKNS: usize = 984;
pub const SYS_NANOSLEEP: usize = 162;
pub const SYS_YIELD: usize = 158;
