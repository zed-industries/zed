use core::{mem, slice};

use crate::{data::*, error::*, flag::*, number::*, scheme::*, CallerCtx, OpenResult};

pub trait SchemeBlockMut {
    fn handle(&mut self, packet: &Packet) -> Option<usize> {
        let res = match packet.a {
            SYS_OPEN => {
                if let Some(path) = unsafe { str_from_raw_parts(packet.b as *const u8, packet.c) } {
                    convert_in_scheme_handle_block(
                        packet,
                        self.xopen(path, packet.d, &CallerCtx::from_packet(&packet)),
                    )
                } else {
                    Err(Error::new(EINVAL))
                }
            }
            SYS_RMDIR => {
                if let Some(path) = unsafe { str_from_raw_parts(packet.b as *const u8, packet.c) } {
                    self.rmdir(path, packet.uid, packet.gid)
                } else {
                    Err(Error::new(EINVAL))
                }
            }
            SYS_UNLINK => {
                if let Some(path) = unsafe { str_from_raw_parts(packet.b as *const u8, packet.c) } {
                    self.unlink(path, packet.uid, packet.gid)
                } else {
                    Err(Error::new(EINVAL))
                }
            }

            SYS_DUP => convert_in_scheme_handle_block(
                packet,
                self.xdup(
                    packet.b,
                    unsafe { slice::from_raw_parts(packet.c as *const u8, packet.d) },
                    &CallerCtx::from_packet(&packet),
                ),
            ),
            SYS_READ => self.read(packet.b, unsafe {
                slice::from_raw_parts_mut(packet.c as *mut u8, packet.d)
            }),
            SYS_WRITE => self.write(packet.b, unsafe {
                slice::from_raw_parts(packet.c as *const u8, packet.d)
            }),
            SYS_LSEEK => self
                .seek(packet.b, packet.c as isize, packet.d)
                .map(|o| o.map(|o| o as usize)),
            SYS_FCHMOD => self.fchmod(packet.b, packet.c as u16),
            SYS_FCHOWN => self.fchown(packet.b, packet.c as u32, packet.d as u32),
            SYS_FCNTL => self.fcntl(packet.b, packet.c, packet.d),
            SYS_FEVENT => self
                .fevent(packet.b, EventFlags::from_bits_truncate(packet.c))
                .map(|f| f.map(|f| f.bits())),
            SYS_FLINK => {
                if let Some(path) = unsafe { str_from_raw_parts(packet.c as *const u8, packet.d) } {
                    self.flink(packet.b, path, packet.uid, packet.gid)
                } else {
                    Err(Error::new(EINVAL))
                }
            }
            SYS_FPATH => self.fpath(packet.b, unsafe {
                slice::from_raw_parts_mut(packet.c as *mut u8, packet.d)
            }),
            SYS_FRENAME => {
                if let Some(path) = unsafe { str_from_raw_parts(packet.c as *const u8, packet.d) } {
                    self.frename(packet.b, path, packet.uid, packet.gid)
                } else {
                    Err(Error::new(EINVAL))
                }
            }
            SYS_FSTAT => {
                if packet.d >= mem::size_of::<Stat>() {
                    self.fstat(packet.b, unsafe { &mut *(packet.c as *mut Stat) })
                } else {
                    Err(Error::new(EFAULT))
                }
            }
            SYS_FSTATVFS => {
                if packet.d >= mem::size_of::<StatVfs>() {
                    self.fstatvfs(packet.b, unsafe { &mut *(packet.c as *mut StatVfs) })
                } else {
                    Err(Error::new(EFAULT))
                }
            }
            SYS_FSYNC => self.fsync(packet.b),
            SYS_FTRUNCATE => self.ftruncate(packet.b, packet.c),
            SYS_FUTIMENS => {
                if packet.d >= mem::size_of::<TimeSpec>() {
                    self.futimens(packet.b, unsafe {
                        slice::from_raw_parts(
                            packet.c as *const TimeSpec,
                            packet.d / mem::size_of::<TimeSpec>(),
                        )
                    })
                } else {
                    Err(Error::new(EFAULT))
                }
            }
            SYS_CLOSE => self.close(packet.b),

            KSMSG_MMAP_PREP => self.mmap_prep(
                packet.b,
                u64::from(packet.uid) | (u64::from(packet.gid) << 32),
                packet.c,
                MapFlags::from_bits_truncate(packet.d),
            ),
            KSMSG_MUNMAP => self.munmap(
                packet.b,
                u64::from(packet.uid) | (u64::from(packet.gid) << 32),
                packet.c,
                MunmapFlags::from_bits_truncate(packet.d),
            ),

            _ => Err(Error::new(ENOSYS)),
        };

        res.transpose().map(Error::mux)
    }

    /* Scheme operations */

    #[allow(unused_variables)]
    fn open(&mut self, path: &str, flags: usize, uid: u32, gid: u32) -> Result<Option<usize>> {
        Err(Error::new(ENOENT))
    }
    #[allow(unused_variables)]
    fn xopen(&mut self, path: &str, flags: usize, ctx: &CallerCtx) -> Result<Option<OpenResult>> {
        convert_to_this_scheme_block(self.open(path, flags, ctx.uid, ctx.gid))
    }

    #[allow(unused_variables)]
    fn chmod(&mut self, path: &str, mode: u16, uid: u32, gid: u32) -> Result<Option<usize>> {
        Err(Error::new(ENOENT))
    }

    #[allow(unused_variables)]
    fn rmdir(&mut self, path: &str, uid: u32, gid: u32) -> Result<Option<usize>> {
        Err(Error::new(ENOENT))
    }

    #[allow(unused_variables)]
    fn unlink(&mut self, path: &str, uid: u32, gid: u32) -> Result<Option<usize>> {
        Err(Error::new(ENOENT))
    }

    /* Resource operations */
    #[allow(unused_variables)]
    fn dup(&mut self, old_id: usize, buf: &[u8]) -> Result<Option<usize>> {
        Err(Error::new(EBADF))
    }

    #[allow(unused_variables)]
    fn xdup(&mut self, old_id: usize, buf: &[u8], ctx: &CallerCtx) -> Result<Option<OpenResult>> {
        convert_to_this_scheme_block(self.dup(old_id, buf))
    }

    #[allow(unused_variables)]
    fn read(&mut self, id: usize, buf: &mut [u8]) -> Result<Option<usize>> {
        Err(Error::new(EBADF))
    }

    #[allow(unused_variables)]
    fn write(&mut self, id: usize, buf: &[u8]) -> Result<Option<usize>> {
        Err(Error::new(EBADF))
    }

    #[allow(unused_variables)]
    fn seek(&mut self, id: usize, pos: isize, whence: usize) -> Result<Option<isize>> {
        Err(Error::new(EBADF))
    }

    #[allow(unused_variables)]
    fn fchmod(&mut self, id: usize, mode: u16) -> Result<Option<usize>> {
        Err(Error::new(EBADF))
    }

    #[allow(unused_variables)]
    fn fchown(&mut self, id: usize, uid: u32, gid: u32) -> Result<Option<usize>> {
        Err(Error::new(EBADF))
    }

    #[allow(unused_variables)]
    fn fcntl(&mut self, id: usize, cmd: usize, arg: usize) -> Result<Option<usize>> {
        Err(Error::new(EBADF))
    }

    #[allow(unused_variables)]
    fn fevent(&mut self, id: usize, flags: EventFlags) -> Result<Option<EventFlags>> {
        Err(Error::new(EBADF))
    }

    #[allow(unused_variables)]
    fn flink(&mut self, id: usize, path: &str, uid: u32, gid: u32) -> Result<Option<usize>> {
        Err(Error::new(EBADF))
    }

    #[allow(unused_variables)]
    fn fpath(&mut self, id: usize, buf: &mut [u8]) -> Result<Option<usize>> {
        Err(Error::new(EBADF))
    }

    #[allow(unused_variables)]
    fn frename(&mut self, id: usize, path: &str, uid: u32, gid: u32) -> Result<Option<usize>> {
        Err(Error::new(EBADF))
    }

    #[allow(unused_variables)]
    fn fstat(&mut self, id: usize, stat: &mut Stat) -> Result<Option<usize>> {
        Err(Error::new(EBADF))
    }

    #[allow(unused_variables)]
    fn fstatvfs(&mut self, id: usize, stat: &mut StatVfs) -> Result<Option<usize>> {
        Err(Error::new(EBADF))
    }

    #[allow(unused_variables)]
    fn fsync(&mut self, id: usize) -> Result<Option<usize>> {
        Err(Error::new(EBADF))
    }

    #[allow(unused_variables)]
    fn ftruncate(&mut self, id: usize, len: usize) -> Result<Option<usize>> {
        Err(Error::new(EBADF))
    }

    #[allow(unused_variables)]
    fn futimens(&mut self, id: usize, times: &[TimeSpec]) -> Result<Option<usize>> {
        Err(Error::new(EBADF))
    }

    #[allow(unused_variables)]
    fn close(&mut self, id: usize) -> Result<Option<usize>> {
        Err(Error::new(EBADF))
    }

    #[allow(unused_variables)]
    fn mmap_prep(&mut self, id: usize, offset: u64, size: usize, flags: MapFlags) -> Result<Option<usize>> {
        Err(Error::new(EOPNOTSUPP))
    }

    #[allow(unused_variables)]
    fn munmap(&mut self, id: usize, offset: u64, size: usize, flags: MunmapFlags) -> Result<Option<usize>> {
        Err(Error::new(EOPNOTSUPP))
    }
}
