//! Bindings for WASIp0 aka Preview 0 aka `wasi_unstable`.
//!
//! This module is purely here for backwards compatibility in the Wasmtime CLI.
//! You probably want to use [`preview1`](crate::preview1) instead.

use crate::preview0::types::Error;
use crate::preview1::WasiP1Ctx;
use crate::preview1::types as snapshot1_types;
use crate::preview1::wasi_snapshot_preview1::WasiSnapshotPreview1 as Snapshot1;
use wiggle::{GuestError, GuestMemory, GuestPtr};

pub fn add_to_linker_async<T: Send + 'static>(
    linker: &mut wasmtime::Linker<T>,
    f: impl Fn(&mut T) -> &mut WasiP1Ctx + Copy + Send + Sync + 'static,
) -> anyhow::Result<()> {
    wasi_unstable::add_to_linker(linker, f)
}

pub fn add_to_linker_sync<T: Send + 'static>(
    linker: &mut wasmtime::Linker<T>,
    f: impl Fn(&mut T) -> &mut WasiP1Ctx + Copy + Send + Sync + 'static,
) -> anyhow::Result<()> {
    sync::add_wasi_unstable_to_linker(linker, f)
}

wiggle::from_witx!({
    witx: ["witx/preview0/wasi_unstable.witx"],
    async: {
        wasi_unstable::{
            fd_advise, fd_close, fd_datasync, fd_fdstat_get, fd_filestat_get, fd_filestat_set_size,
            fd_filestat_set_times, fd_read, fd_pread, fd_seek, fd_sync, fd_readdir, fd_write,
            fd_pwrite, poll_oneoff, path_create_directory, path_filestat_get,
            path_filestat_set_times, path_link, path_open, path_readlink, path_remove_directory,
            path_rename, path_symlink, path_unlink_file, fd_renumber
        }
    },
    errors: { errno => trappable Error },
});

mod sync {
    use anyhow::Result;
    use std::future::Future;

    wiggle::wasmtime_integration!({
        witx: ["witx/preview0/wasi_unstable.witx"],
        target: super,
        block_on[in_tokio]: {
            wasi_unstable::{
                fd_advise, fd_close, fd_datasync, fd_fdstat_get, fd_filestat_get, fd_filestat_set_size,
                fd_filestat_set_times, fd_read, fd_pread, fd_seek, fd_sync, fd_readdir, fd_write,
                fd_pwrite, poll_oneoff, path_create_directory, path_filestat_get,
                path_filestat_set_times, path_link, path_open, path_readlink, path_remove_directory,
                path_rename, path_symlink, path_unlink_file, fd_renumber
            }
        },
        errors: { errno => trappable Error },
    });

    // Small wrapper around `in_tokio` to add a `Result` layer which is always
    // `Ok`
    fn in_tokio<F: Future>(future: F) -> Result<F::Output> {
        Ok(crate::runtime::in_tokio(future))
    }
}

impl wiggle::GuestErrorType for types::Errno {
    fn success() -> Self {
        Self::Success
    }
}

#[wiggle::async_trait]
impl<T: Snapshot1 + Send> wasi_unstable::WasiUnstable for T {
    fn set_hostcall_fuel(&mut self, fuel: usize) {
        Snapshot1::set_hostcall_fuel(self, fuel)
    }
    fn args_get(
        &mut self,
        memory: &mut GuestMemory<'_>,
        argv: GuestPtr<GuestPtr<u8>>,
        argv_buf: GuestPtr<u8>,
    ) -> Result<(), Error> {
        Snapshot1::args_get(self, memory, argv, argv_buf)?;
        Ok(())
    }

    fn args_sizes_get(
        &mut self,
        memory: &mut GuestMemory<'_>,
    ) -> Result<(types::Size, types::Size), Error> {
        let s = Snapshot1::args_sizes_get(self, memory)?;
        Ok(s)
    }

    fn environ_get(
        &mut self,
        memory: &mut GuestMemory<'_>,
        environ: GuestPtr<GuestPtr<u8>>,
        environ_buf: GuestPtr<u8>,
    ) -> Result<(), Error> {
        Snapshot1::environ_get(self, memory, environ, environ_buf)?;
        Ok(())
    }

    fn environ_sizes_get(
        &mut self,
        memory: &mut GuestMemory<'_>,
    ) -> Result<(types::Size, types::Size), Error> {
        let s = Snapshot1::environ_sizes_get(self, memory)?;
        Ok(s)
    }

    fn clock_res_get(
        &mut self,
        memory: &mut GuestMemory<'_>,
        id: types::Clockid,
    ) -> Result<types::Timestamp, Error> {
        let t = Snapshot1::clock_res_get(self, memory, id.into())?;
        Ok(t)
    }

    fn clock_time_get(
        &mut self,
        memory: &mut GuestMemory<'_>,
        id: types::Clockid,
        precision: types::Timestamp,
    ) -> Result<types::Timestamp, Error> {
        let t = Snapshot1::clock_time_get(self, memory, id.into(), precision)?;
        Ok(t)
    }

    async fn fd_advise(
        &mut self,
        memory: &mut GuestMemory<'_>,
        fd: types::Fd,
        offset: types::Filesize,
        len: types::Filesize,
        advice: types::Advice,
    ) -> Result<(), Error> {
        Snapshot1::fd_advise(self, memory, fd.into(), offset, len, advice.into()).await?;
        Ok(())
    }

    fn fd_allocate(
        &mut self,
        memory: &mut GuestMemory<'_>,
        fd: types::Fd,
        offset: types::Filesize,
        len: types::Filesize,
    ) -> Result<(), Error> {
        Snapshot1::fd_allocate(self, memory, fd.into(), offset, len)?;
        Ok(())
    }

    async fn fd_close(&mut self, memory: &mut GuestMemory<'_>, fd: types::Fd) -> Result<(), Error> {
        Snapshot1::fd_close(self, memory, fd.into()).await?;
        Ok(())
    }

    async fn fd_datasync(
        &mut self,
        memory: &mut GuestMemory<'_>,
        fd: types::Fd,
    ) -> Result<(), Error> {
        Snapshot1::fd_datasync(self, memory, fd.into()).await?;
        Ok(())
    }

    async fn fd_fdstat_get(
        &mut self,
        memory: &mut GuestMemory<'_>,
        fd: types::Fd,
    ) -> Result<types::Fdstat, Error> {
        Ok(Snapshot1::fd_fdstat_get(self, memory, fd.into())
            .await?
            .into())
    }

    fn fd_fdstat_set_flags(
        &mut self,
        memory: &mut GuestMemory<'_>,
        fd: types::Fd,
        flags: types::Fdflags,
    ) -> Result<(), Error> {
        Snapshot1::fd_fdstat_set_flags(self, memory, fd.into(), flags.into())?;
        Ok(())
    }

    fn fd_fdstat_set_rights(
        &mut self,
        memory: &mut GuestMemory<'_>,
        fd: types::Fd,
        fs_rights_base: types::Rights,
        fs_rights_inheriting: types::Rights,
    ) -> Result<(), Error> {
        Snapshot1::fd_fdstat_set_rights(
            self,
            memory,
            fd.into(),
            fs_rights_base.into(),
            fs_rights_inheriting.into(),
        )?;
        Ok(())
    }

    async fn fd_filestat_get(
        &mut self,
        memory: &mut GuestMemory<'_>,
        fd: types::Fd,
    ) -> Result<types::Filestat, Error> {
        Ok(Snapshot1::fd_filestat_get(self, memory, fd.into())
            .await?
            .into())
    }

    async fn fd_filestat_set_size(
        &mut self,
        memory: &mut GuestMemory<'_>,
        fd: types::Fd,
        size: types::Filesize,
    ) -> Result<(), Error> {
        Snapshot1::fd_filestat_set_size(self, memory, fd.into(), size).await?;
        Ok(())
    }

    async fn fd_filestat_set_times(
        &mut self,
        memory: &mut GuestMemory<'_>,
        fd: types::Fd,
        atim: types::Timestamp,
        mtim: types::Timestamp,
        fst_flags: types::Fstflags,
    ) -> Result<(), Error> {
        Snapshot1::fd_filestat_set_times(self, memory, fd.into(), atim, mtim, fst_flags.into())
            .await?;
        Ok(())
    }

    async fn fd_read(
        &mut self,
        memory: &mut GuestMemory<'_>,
        fd: types::Fd,
        iovs: types::IovecArray,
    ) -> Result<types::Size, Error> {
        assert_iovec_array_same();
        let result = Snapshot1::fd_read(self, memory, fd.into(), iovs.cast()).await?;
        Ok(result)
    }

    async fn fd_pread(
        &mut self,
        memory: &mut GuestMemory<'_>,
        fd: types::Fd,
        iovs: types::IovecArray,
        offset: types::Filesize,
    ) -> Result<types::Size, Error> {
        assert_iovec_array_same();
        let result = Snapshot1::fd_pread(self, memory, fd.into(), iovs.cast(), offset).await?;
        Ok(result)
    }

    async fn fd_write(
        &mut self,
        memory: &mut GuestMemory<'_>,
        fd: types::Fd,
        ciovs: types::CiovecArray,
    ) -> Result<types::Size, Error> {
        assert_ciovec_array_same();
        let result = Snapshot1::fd_write(self, memory, fd.into(), ciovs.cast()).await?;
        Ok(result)
    }

    async fn fd_pwrite(
        &mut self,
        memory: &mut GuestMemory<'_>,
        fd: types::Fd,
        ciovs: types::CiovecArray,
        offset: types::Filesize,
    ) -> Result<types::Size, Error> {
        assert_ciovec_array_same();
        let result = Snapshot1::fd_pwrite(self, memory, fd.into(), ciovs.cast(), offset).await?;
        Ok(result)
    }

    fn fd_prestat_get(
        &mut self,
        memory: &mut GuestMemory<'_>,
        fd: types::Fd,
    ) -> Result<types::Prestat, Error> {
        Ok(Snapshot1::fd_prestat_get(self, memory, fd.into())?.into())
    }

    fn fd_prestat_dir_name(
        &mut self,
        memory: &mut GuestMemory<'_>,
        fd: types::Fd,
        path: GuestPtr<u8>,
        path_max_len: types::Size,
    ) -> Result<(), Error> {
        Snapshot1::fd_prestat_dir_name(self, memory, fd.into(), path, path_max_len)?;
        Ok(())
    }

    async fn fd_renumber(
        &mut self,
        memory: &mut GuestMemory<'_>,
        from: types::Fd,
        to: types::Fd,
    ) -> Result<(), Error> {
        Snapshot1::fd_renumber(self, memory, from.into(), to.into()).await?;
        Ok(())
    }

    async fn fd_seek(
        &mut self,
        memory: &mut GuestMemory<'_>,
        fd: types::Fd,
        offset: types::Filedelta,
        whence: types::Whence,
    ) -> Result<types::Filesize, Error> {
        Ok(Snapshot1::fd_seek(self, memory, fd.into(), offset, whence.into()).await?)
    }

    async fn fd_sync(&mut self, memory: &mut GuestMemory<'_>, fd: types::Fd) -> Result<(), Error> {
        Snapshot1::fd_sync(self, memory, fd.into()).await?;
        Ok(())
    }

    fn fd_tell(
        &mut self,
        memory: &mut GuestMemory<'_>,
        fd: types::Fd,
    ) -> Result<types::Filesize, Error> {
        Ok(Snapshot1::fd_tell(self, memory, fd.into())?)
    }

    async fn fd_readdir(
        &mut self,
        memory: &mut GuestMemory<'_>,
        fd: types::Fd,
        buf: GuestPtr<u8>,
        buf_len: types::Size,
        cookie: types::Dircookie,
    ) -> Result<types::Size, Error> {
        Ok(Snapshot1::fd_readdir(self, memory, fd.into(), buf, buf_len, cookie).await?)
    }

    async fn path_create_directory(
        &mut self,
        memory: &mut GuestMemory<'_>,
        dirfd: types::Fd,
        path: GuestPtr<str>,
    ) -> Result<(), Error> {
        Snapshot1::path_create_directory(self, memory, dirfd.into(), path).await?;
        Ok(())
    }

    async fn path_filestat_get(
        &mut self,
        memory: &mut GuestMemory<'_>,
        dirfd: types::Fd,
        flags: types::Lookupflags,
        path: GuestPtr<str>,
    ) -> Result<types::Filestat, Error> {
        Ok(
            Snapshot1::path_filestat_get(self, memory, dirfd.into(), flags.into(), path)
                .await?
                .into(),
        )
    }

    async fn path_filestat_set_times(
        &mut self,
        memory: &mut GuestMemory<'_>,
        dirfd: types::Fd,
        flags: types::Lookupflags,
        path: GuestPtr<str>,
        atim: types::Timestamp,
        mtim: types::Timestamp,
        fst_flags: types::Fstflags,
    ) -> Result<(), Error> {
        Snapshot1::path_filestat_set_times(
            self,
            memory,
            dirfd.into(),
            flags.into(),
            path,
            atim,
            mtim,
            fst_flags.into(),
        )
        .await?;
        Ok(())
    }

    async fn path_link(
        &mut self,
        memory: &mut GuestMemory<'_>,
        src_fd: types::Fd,
        src_flags: types::Lookupflags,
        src_path: GuestPtr<str>,
        target_fd: types::Fd,
        target_path: GuestPtr<str>,
    ) -> Result<(), Error> {
        Snapshot1::path_link(
            self,
            memory,
            src_fd.into(),
            src_flags.into(),
            src_path,
            target_fd.into(),
            target_path,
        )
        .await?;
        Ok(())
    }

    async fn path_open(
        &mut self,
        memory: &mut GuestMemory<'_>,
        dirfd: types::Fd,
        dirflags: types::Lookupflags,
        path: GuestPtr<str>,
        oflags: types::Oflags,
        fs_rights_base: types::Rights,
        fs_rights_inheriting: types::Rights,
        fdflags: types::Fdflags,
    ) -> Result<types::Fd, Error> {
        Ok(Snapshot1::path_open(
            self,
            memory,
            dirfd.into(),
            dirflags.into(),
            path,
            oflags.into(),
            fs_rights_base.into(),
            fs_rights_inheriting.into(),
            fdflags.into(),
        )
        .await?
        .into())
    }

    async fn path_readlink(
        &mut self,
        memory: &mut GuestMemory<'_>,
        dirfd: types::Fd,
        path: GuestPtr<str>,
        buf: GuestPtr<u8>,
        buf_len: types::Size,
    ) -> Result<types::Size, Error> {
        Ok(Snapshot1::path_readlink(self, memory, dirfd.into(), path, buf, buf_len).await?)
    }

    async fn path_remove_directory(
        &mut self,
        memory: &mut GuestMemory<'_>,
        dirfd: types::Fd,
        path: GuestPtr<str>,
    ) -> Result<(), Error> {
        Snapshot1::path_remove_directory(self, memory, dirfd.into(), path).await?;
        Ok(())
    }

    async fn path_rename(
        &mut self,
        memory: &mut GuestMemory<'_>,
        src_fd: types::Fd,
        src_path: GuestPtr<str>,
        dest_fd: types::Fd,
        dest_path: GuestPtr<str>,
    ) -> Result<(), Error> {
        Snapshot1::path_rename(
            self,
            memory,
            src_fd.into(),
            src_path,
            dest_fd.into(),
            dest_path,
        )
        .await?;
        Ok(())
    }

    async fn path_symlink(
        &mut self,
        memory: &mut GuestMemory<'_>,
        src_path: GuestPtr<str>,
        dirfd: types::Fd,
        dest_path: GuestPtr<str>,
    ) -> Result<(), Error> {
        Snapshot1::path_symlink(self, memory, src_path, dirfd.into(), dest_path).await?;
        Ok(())
    }

    async fn path_unlink_file(
        &mut self,
        memory: &mut GuestMemory<'_>,
        dirfd: types::Fd,
        path: GuestPtr<str>,
    ) -> Result<(), Error> {
        Snapshot1::path_unlink_file(self, memory, dirfd.into(), path).await?;
        Ok(())
    }

    // The representation of `SubscriptionClock` is different in preview0 and
    // preview1 so a bit of a hack is employed here. The change was to remove a
    // field from `SubscriptionClock` so to implement this without copying too
    // much the `subs` field is overwritten with preview1-compatible structures
    // and then the preview1 implementation is used. Before returning though
    // the old values are restored to pretend like we didn't overwrite them.
    //
    // Surely no one would pass overlapping pointers to this API right?
    async fn poll_oneoff(
        &mut self,
        memory: &mut GuestMemory<'_>,
        subs: GuestPtr<types::Subscription>,
        events: GuestPtr<types::Event>,
        nsubscriptions: types::Size,
    ) -> Result<types::Size, Error> {
        let subs_array = subs.as_array(nsubscriptions);
        let mut old_subs = Vec::new();
        for slot in subs_array.iter() {
            let slot = slot?;
            let sub = memory.read(slot)?;
            old_subs.push(sub.clone());
            memory.write(
                slot.cast(),
                snapshot1_types::Subscription {
                    userdata: sub.userdata,
                    u: match sub.u {
                        types::SubscriptionU::Clock(c) => {
                            snapshot1_types::SubscriptionU::Clock(c.into())
                        }
                        types::SubscriptionU::FdRead(c) => {
                            snapshot1_types::SubscriptionU::FdRead(c.into())
                        }
                        types::SubscriptionU::FdWrite(c) => {
                            snapshot1_types::SubscriptionU::FdWrite(c.into())
                        }
                    },
                },
            )?;
        }
        let ret = Snapshot1::poll_oneoff(self, memory, subs.cast(), events.cast(), nsubscriptions)
            .await?;
        for (sub, slot) in old_subs.into_iter().zip(subs_array.iter()) {
            memory.write(slot?, sub)?;
        }
        Ok(ret)
    }

    fn proc_exit(
        &mut self,
        memory: &mut GuestMemory<'_>,
        status: types::Exitcode,
    ) -> anyhow::Error {
        Snapshot1::proc_exit(self, memory, status)
    }

    fn proc_raise(
        &mut self,
        memory: &mut GuestMemory<'_>,
        sig: types::Signal,
    ) -> Result<(), Error> {
        Snapshot1::proc_raise(self, memory, sig.into())?;
        Ok(())
    }

    fn sched_yield(&mut self, memory: &mut GuestMemory<'_>) -> Result<(), Error> {
        Snapshot1::sched_yield(self, memory)?;
        Ok(())
    }

    fn random_get(
        &mut self,
        memory: &mut GuestMemory<'_>,
        buf: GuestPtr<u8>,
        buf_len: types::Size,
    ) -> Result<(), Error> {
        Snapshot1::random_get(self, memory, buf, buf_len)?;
        Ok(())
    }

    fn sock_recv(
        &mut self,
        _memory: &mut GuestMemory<'_>,
        _fd: types::Fd,
        _ri_data: types::IovecArray,
        _ri_flags: types::Riflags,
    ) -> Result<(types::Size, types::Roflags), Error> {
        Err(Error::trap(anyhow::Error::msg("sock_recv unsupported")))
    }

    fn sock_send(
        &mut self,
        _memory: &mut GuestMemory<'_>,
        _fd: types::Fd,
        _si_data: types::CiovecArray,
        _si_flags: types::Siflags,
    ) -> Result<types::Size, Error> {
        Err(Error::trap(anyhow::Error::msg("sock_send unsupported")))
    }

    fn sock_shutdown(
        &mut self,
        _memory: &mut GuestMemory<'_>,
        _fd: types::Fd,
        _how: types::Sdflags,
    ) -> Result<(), Error> {
        Err(Error::trap(anyhow::Error::msg("sock_shutdown unsupported")))
    }
}

fn assert_iovec_array_same() {
    // NB: this isn't enough to assert the types are the same, but it's
    // something. Additionally preview1 and preview0 aren't changing any more
    // and it's been manually verified that these two types are the same, so
    // it's ok to cast between them.
    assert_eq!(
        std::mem::size_of::<types::IovecArray>(),
        std::mem::size_of::<snapshot1_types::IovecArray>()
    );
}

fn assert_ciovec_array_same() {
    // NB: see above too
    assert_eq!(
        std::mem::size_of::<types::CiovecArray>(),
        std::mem::size_of::<snapshot1_types::CiovecArray>()
    );
}

impl From<snapshot1_types::Error> for Error {
    fn from(error: snapshot1_types::Error) -> Error {
        match error.downcast() {
            Ok(errno) => Error::from(types::Errno::from(errno)),
            Err(trap) => Error::trap(trap),
        }
    }
}

/// Fd is a newtype wrapper around u32. Unwrap and wrap it.
impl From<types::Fd> for snapshot1_types::Fd {
    fn from(fd: types::Fd) -> snapshot1_types::Fd {
        u32::from(fd).into()
    }
}

/// Fd is a newtype wrapper around u32. Unwrap and wrap it.
impl From<snapshot1_types::Fd> for types::Fd {
    fn from(fd: snapshot1_types::Fd) -> types::Fd {
        u32::from(fd).into()
    }
}

/// Trivial conversion between two c-style enums that have the exact same set of variants.
/// Could we do something unsafe and not list all these variants out? Probably, but doing
/// it this way doesn't bother me much. I copy-pasted the list of variants out of the
/// rendered rustdocs.
/// LLVM ought to compile these From impls into no-ops, inshallah
macro_rules! convert_enum {
    ($from:ty, $to:ty, $($var:ident),+) => {
        impl From<$from> for $to {
            fn from(e: $from) -> $to {
                match e {
                    $( <$from>::$var => <$to>::$var, )+
                }
            }
        }
    }
}
convert_enum!(
    snapshot1_types::Errno,
    types::Errno,
    Success,
    TooBig,
    Acces,
    Addrinuse,
    Addrnotavail,
    Afnosupport,
    Again,
    Already,
    Badf,
    Badmsg,
    Busy,
    Canceled,
    Child,
    Connaborted,
    Connrefused,
    Connreset,
    Deadlk,
    Destaddrreq,
    Dom,
    Dquot,
    Exist,
    Fault,
    Fbig,
    Hostunreach,
    Idrm,
    Ilseq,
    Inprogress,
    Intr,
    Inval,
    Io,
    Isconn,
    Isdir,
    Loop,
    Mfile,
    Mlink,
    Msgsize,
    Multihop,
    Nametoolong,
    Netdown,
    Netreset,
    Netunreach,
    Nfile,
    Nobufs,
    Nodev,
    Noent,
    Noexec,
    Nolck,
    Nolink,
    Nomem,
    Nomsg,
    Noprotoopt,
    Nospc,
    Nosys,
    Notconn,
    Notdir,
    Notempty,
    Notrecoverable,
    Notsock,
    Notsup,
    Notty,
    Nxio,
    Overflow,
    Ownerdead,
    Perm,
    Pipe,
    Proto,
    Protonosupport,
    Prototype,
    Range,
    Rofs,
    Spipe,
    Srch,
    Stale,
    Timedout,
    Txtbsy,
    Xdev,
    Notcapable
);
convert_enum!(
    types::Clockid,
    snapshot1_types::Clockid,
    Realtime,
    Monotonic,
    ProcessCputimeId,
    ThreadCputimeId
);

convert_enum!(
    types::Advice,
    snapshot1_types::Advice,
    Normal,
    Sequential,
    Random,
    Willneed,
    Dontneed,
    Noreuse
);
convert_enum!(
    snapshot1_types::Filetype,
    types::Filetype,
    Directory,
    BlockDevice,
    CharacterDevice,
    RegularFile,
    SocketDgram,
    SocketStream,
    SymbolicLink,
    Unknown
);
convert_enum!(types::Whence, snapshot1_types::Whence, Cur, End, Set);

convert_enum!(
    types::Signal,
    snapshot1_types::Signal,
    None,
    Hup,
    Int,
    Quit,
    Ill,
    Trap,
    Abrt,
    Bus,
    Fpe,
    Kill,
    Usr1,
    Segv,
    Usr2,
    Pipe,
    Alrm,
    Term,
    Chld,
    Cont,
    Stop,
    Tstp,
    Ttin,
    Ttou,
    Urg,
    Xcpu,
    Xfsz,
    Vtalrm,
    Prof,
    Winch,
    Poll,
    Pwr,
    Sys
);

/// Prestat isn't a c-style enum, its a union where the variant has a payload. Its the only one of
/// those we need to convert, so write it by hand.
impl From<snapshot1_types::Prestat> for types::Prestat {
    fn from(p: snapshot1_types::Prestat) -> types::Prestat {
        match p {
            snapshot1_types::Prestat::Dir(d) => types::Prestat::Dir(d.into()),
        }
    }
}

/// Trivial conversion between two structs that have the exact same set of fields,
/// with recursive descent into the field types.
macro_rules! convert_struct {
    ($from:ty, $to:path, $($field:ident),+) => {
        impl From<$from> for $to {
            fn from(e: $from) -> $to {
                $to {
                    $( $field: e.$field.into(), )+
                }
            }
        }
    }
}

convert_struct!(snapshot1_types::PrestatDir, types::PrestatDir, pr_name_len);
convert_struct!(
    snapshot1_types::Fdstat,
    types::Fdstat,
    fs_filetype,
    fs_rights_base,
    fs_rights_inheriting,
    fs_flags
);
convert_struct!(
    types::SubscriptionClock,
    snapshot1_types::SubscriptionClock,
    id,
    timeout,
    precision,
    flags
);
convert_struct!(
    types::SubscriptionFdReadwrite,
    snapshot1_types::SubscriptionFdReadwrite,
    file_descriptor
);

/// Snapshot1 Filestat is incompatible with Snapshot0 Filestat - the nlink
/// field is u32 on this Filestat, and u64 on theirs. If you've got more than
/// 2^32 links I don't know what to tell you
impl From<snapshot1_types::Filestat> for types::Filestat {
    fn from(f: snapshot1_types::Filestat) -> types::Filestat {
        types::Filestat {
            dev: f.dev,
            ino: f.ino,
            filetype: f.filetype.into(),
            nlink: f.nlink.try_into().unwrap_or(u32::MAX),
            size: f.size,
            atim: f.atim,
            mtim: f.mtim,
            ctim: f.ctim,
        }
    }
}

/// Trivial conversion between two bitflags that have the exact same set of flags.
macro_rules! convert_flags {
    ($from:ty, $to:ty, $($flag:ident),+) => {
        impl From<$from> for $to {
            fn from(f: $from) -> $to {
                let mut out = <$to>::empty();
                $(
                    if f.contains(<$from>::$flag) {
                        out |= <$to>::$flag;
                    }
                )+
                out
            }
        }
    }
}

/// Need to convert in both directions? This saves listing out the flags twice
macro_rules! convert_flags_bidirectional {
    ($from:ty, $to:ty, $($flag:tt)*) => {
        convert_flags!($from, $to, $($flag)*);
        convert_flags!($to, $from, $($flag)*);
    }
}

convert_flags_bidirectional!(
    snapshot1_types::Fdflags,
    types::Fdflags,
    APPEND,
    DSYNC,
    NONBLOCK,
    RSYNC,
    SYNC
);
convert_flags!(
    types::Lookupflags,
    snapshot1_types::Lookupflags,
    SYMLINK_FOLLOW
);
convert_flags!(
    types::Fstflags,
    snapshot1_types::Fstflags,
    ATIM,
    ATIM_NOW,
    MTIM,
    MTIM_NOW
);
convert_flags!(
    types::Oflags,
    snapshot1_types::Oflags,
    CREAT,
    DIRECTORY,
    EXCL,
    TRUNC
);
convert_flags_bidirectional!(
    types::Rights,
    snapshot1_types::Rights,
    FD_DATASYNC,
    FD_READ,
    FD_SEEK,
    FD_FDSTAT_SET_FLAGS,
    FD_SYNC,
    FD_TELL,
    FD_WRITE,
    FD_ADVISE,
    FD_ALLOCATE,
    PATH_CREATE_DIRECTORY,
    PATH_CREATE_FILE,
    PATH_LINK_SOURCE,
    PATH_LINK_TARGET,
    PATH_OPEN,
    FD_READDIR,
    PATH_READLINK,
    PATH_RENAME_SOURCE,
    PATH_RENAME_TARGET,
    PATH_FILESTAT_GET,
    PATH_FILESTAT_SET_SIZE,
    PATH_FILESTAT_SET_TIMES,
    FD_FILESTAT_GET,
    FD_FILESTAT_SET_SIZE,
    FD_FILESTAT_SET_TIMES,
    PATH_SYMLINK,
    PATH_REMOVE_DIRECTORY,
    PATH_UNLINK_FILE,
    POLL_FD_READWRITE,
    SOCK_SHUTDOWN
);
convert_flags!(
    types::Subclockflags,
    snapshot1_types::Subclockflags,
    SUBSCRIPTION_CLOCK_ABSTIME
);

impl From<GuestError> for types::Error {
    fn from(err: GuestError) -> Self {
        snapshot1_types::Error::from(err).into()
    }
}
