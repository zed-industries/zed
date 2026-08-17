use crate::p2::bindings::clocks::wall_clock;
use crate::p2::bindings::filesystem::preopens;
use crate::p2::bindings::filesystem::types::{
    self, ErrorCode, HostDescriptor, HostDirectoryEntryStream,
};
use crate::p2::filesystem::{
    Descriptor, Dir, File, FileInputStream, FileOutputStream, ReaddirIterator,
};
use crate::p2::{FsError, FsResult, WasiCtxView};
use crate::{DirPerms, FilePerms, OpenMode};
use anyhow::Context;
use wasmtime::component::Resource;
use wasmtime_wasi_io::streams::{DynInputStream, DynOutputStream};

mod sync;

impl preopens::Host for WasiCtxView<'_> {
    fn get_directories(
        &mut self,
    ) -> Result<Vec<(Resource<types::Descriptor>, String)>, anyhow::Error> {
        let mut results = Vec::new();
        for (dir, name) in self.ctx.preopens.clone() {
            let fd = self
                .table
                .push(Descriptor::Dir(dir))
                .with_context(|| format!("failed to push preopen {name}"))?;
            results.push((fd, name));
        }
        Ok(results)
    }
}

impl types::Host for WasiCtxView<'_> {
    fn convert_error_code(&mut self, err: FsError) -> anyhow::Result<ErrorCode> {
        err.downcast()
    }

    fn filesystem_error_code(
        &mut self,
        err: Resource<anyhow::Error>,
    ) -> anyhow::Result<Option<ErrorCode>> {
        let err = self.table.get(&err)?;

        // Currently `err` always comes from the stream implementation which
        // uses standard reads/writes so only check for `std::io::Error` here.
        if let Some(err) = err.downcast_ref::<std::io::Error>() {
            return Ok(Some(ErrorCode::from(err)));
        }

        Ok(None)
    }
}

impl HostDescriptor for WasiCtxView<'_> {
    async fn advise(
        &mut self,
        fd: Resource<types::Descriptor>,
        offset: types::Filesize,
        len: types::Filesize,
        advice: types::Advice,
    ) -> FsResult<()> {
        use system_interface::fs::{Advice as A, FileIoExt};
        use types::Advice;

        let advice = match advice {
            Advice::Normal => A::Normal,
            Advice::Sequential => A::Sequential,
            Advice::Random => A::Random,
            Advice::WillNeed => A::WillNeed,
            Advice::DontNeed => A::DontNeed,
            Advice::NoReuse => A::NoReuse,
        };

        let f = self.table.get(&fd)?.file()?;
        f.run_blocking(move |f| f.advise(offset, len, advice))
            .await?;
        Ok(())
    }

    async fn sync_data(&mut self, fd: Resource<types::Descriptor>) -> FsResult<()> {
        let descriptor = self.table.get(&fd)?;

        match descriptor {
            Descriptor::File(f) => {
                match f.run_blocking(|f| f.sync_data()).await {
                    Ok(()) => Ok(()),
                    // On windows, `sync_data` uses `FileFlushBuffers` which fails with
                    // `ERROR_ACCESS_DENIED` if the file is not upen for writing. Ignore
                    // this error, for POSIX compatibility.
                    #[cfg(windows)]
                    Err(e)
                        if e.raw_os_error()
                            == Some(windows_sys::Win32::Foundation::ERROR_ACCESS_DENIED as _) =>
                    {
                        Ok(())
                    }
                    Err(e) => Err(e.into()),
                }
            }
            Descriptor::Dir(d) => {
                d.run_blocking(|d| Ok(d.open(std::path::Component::CurDir)?.sync_data()?))
                    .await
            }
        }
    }

    async fn get_flags(
        &mut self,
        fd: Resource<types::Descriptor>,
    ) -> FsResult<types::DescriptorFlags> {
        use system_interface::fs::{FdFlags, GetSetFdFlags};
        use types::DescriptorFlags;

        fn get_from_fdflags(flags: FdFlags) -> DescriptorFlags {
            let mut out = DescriptorFlags::empty();
            if flags.contains(FdFlags::DSYNC) {
                out |= DescriptorFlags::REQUESTED_WRITE_SYNC;
            }
            if flags.contains(FdFlags::RSYNC) {
                out |= DescriptorFlags::DATA_INTEGRITY_SYNC;
            }
            if flags.contains(FdFlags::SYNC) {
                out |= DescriptorFlags::FILE_INTEGRITY_SYNC;
            }
            out
        }

        let descriptor = self.table.get(&fd)?;
        match descriptor {
            Descriptor::File(f) => {
                let flags = f.run_blocking(|f| f.get_fd_flags()).await?;
                let mut flags = get_from_fdflags(flags);
                if f.open_mode.contains(OpenMode::READ) {
                    flags |= DescriptorFlags::READ;
                }
                if f.open_mode.contains(OpenMode::WRITE) {
                    flags |= DescriptorFlags::WRITE;
                }
                Ok(flags)
            }
            Descriptor::Dir(d) => {
                let flags = d.run_blocking(|d| d.get_fd_flags()).await?;
                let mut flags = get_from_fdflags(flags);
                if d.open_mode.contains(OpenMode::READ) {
                    flags |= DescriptorFlags::READ;
                }
                if d.open_mode.contains(OpenMode::WRITE) {
                    flags |= DescriptorFlags::MUTATE_DIRECTORY;
                }
                Ok(flags)
            }
        }
    }

    async fn get_type(
        &mut self,
        fd: Resource<types::Descriptor>,
    ) -> FsResult<types::DescriptorType> {
        let descriptor = self.table.get(&fd)?;

        match descriptor {
            Descriptor::File(f) => {
                let meta = f.run_blocking(|f| f.metadata()).await?;
                Ok(descriptortype_from(meta.file_type()))
            }
            Descriptor::Dir(_) => Ok(types::DescriptorType::Directory),
        }
    }

    async fn set_size(
        &mut self,
        fd: Resource<types::Descriptor>,
        size: types::Filesize,
    ) -> FsResult<()> {
        let f = self.table.get(&fd)?.file()?;
        if !f.perms.contains(FilePerms::WRITE) {
            Err(ErrorCode::NotPermitted)?;
        }
        f.run_blocking(move |f| f.set_len(size)).await?;
        Ok(())
    }

    async fn set_times(
        &mut self,
        fd: Resource<types::Descriptor>,
        atim: types::NewTimestamp,
        mtim: types::NewTimestamp,
    ) -> FsResult<()> {
        use fs_set_times::SetTimes;

        let descriptor = self.table.get(&fd)?;
        match descriptor {
            Descriptor::File(f) => {
                if !f.perms.contains(FilePerms::WRITE) {
                    return Err(ErrorCode::NotPermitted.into());
                }
                let atim = systemtimespec_from(atim)?;
                let mtim = systemtimespec_from(mtim)?;
                f.run_blocking(|f| f.set_times(atim, mtim)).await?;
                Ok(())
            }
            Descriptor::Dir(d) => {
                if !d.perms.contains(DirPerms::MUTATE) {
                    return Err(ErrorCode::NotPermitted.into());
                }
                let atim = systemtimespec_from(atim)?;
                let mtim = systemtimespec_from(mtim)?;
                d.run_blocking(|d| d.set_times(atim, mtim)).await?;
                Ok(())
            }
        }
    }

    async fn read(
        &mut self,
        fd: Resource<types::Descriptor>,
        len: types::Filesize,
        offset: types::Filesize,
    ) -> FsResult<(Vec<u8>, bool)> {
        use std::io::IoSliceMut;
        use system_interface::fs::FileIoExt;

        let f = self.table.get(&fd)?.file()?;
        if !f.perms.contains(FilePerms::READ) {
            return Err(ErrorCode::NotPermitted.into());
        }

        let (mut buffer, r) = f
            .run_blocking(move |f| {
                let mut buffer = vec![
                    0;
                    len.try_into()
                        .unwrap_or(usize::MAX)
                        .min(crate::MAX_READ_SIZE_ALLOC)
                ];
                let r = f.read_vectored_at(&mut [IoSliceMut::new(&mut buffer)], offset);
                (buffer, r)
            })
            .await;

        let (bytes_read, state) = match r? {
            0 => (0, true),
            n => (n, false),
        };

        buffer.truncate(bytes_read);

        Ok((buffer, state))
    }

    async fn write(
        &mut self,
        fd: Resource<types::Descriptor>,
        buf: Vec<u8>,
        offset: types::Filesize,
    ) -> FsResult<types::Filesize> {
        use std::io::IoSlice;
        use system_interface::fs::FileIoExt;

        let f = self.table.get(&fd)?.file()?;
        if !f.perms.contains(FilePerms::WRITE) {
            return Err(ErrorCode::NotPermitted.into());
        }

        let bytes_written = f
            .run_blocking(move |f| f.write_vectored_at(&[IoSlice::new(&buf)], offset))
            .await?;

        Ok(types::Filesize::try_from(bytes_written).expect("usize fits in Filesize"))
    }

    async fn read_directory(
        &mut self,
        fd: Resource<types::Descriptor>,
    ) -> FsResult<Resource<types::DirectoryEntryStream>> {
        let d = self.table.get(&fd)?.dir()?;
        if !d.perms.contains(DirPerms::READ) {
            return Err(ErrorCode::NotPermitted.into());
        }

        enum ReaddirError {
            Io(std::io::Error),
            IllegalSequence,
        }
        impl From<std::io::Error> for ReaddirError {
            fn from(e: std::io::Error) -> ReaddirError {
                ReaddirError::Io(e)
            }
        }

        let entries = d
            .run_blocking(|d| {
                // Both `entries` and `metadata` perform syscalls, which is why they are done
                // within this `block` call, rather than delay calculating the metadata
                // for entries when they're demanded later in the iterator chain.
                Ok::<_, std::io::Error>(
                    d.entries()?
                        .map(|entry| {
                            let entry = entry?;
                            let meta = entry.metadata()?;
                            let type_ = descriptortype_from(meta.file_type());
                            let name = entry
                                .file_name()
                                .into_string()
                                .map_err(|_| ReaddirError::IllegalSequence)?;
                            Ok(types::DirectoryEntry { type_, name })
                        })
                        .collect::<Vec<Result<types::DirectoryEntry, ReaddirError>>>(),
                )
            })
            .await?
            .into_iter();

        // On windows, filter out files like `C:\DumpStack.log.tmp` which we
        // can't get full metadata for.
        #[cfg(windows)]
        let entries = entries.filter(|entry| {
            use windows_sys::Win32::Foundation::{ERROR_ACCESS_DENIED, ERROR_SHARING_VIOLATION};
            if let Err(ReaddirError::Io(err)) = entry {
                if err.raw_os_error() == Some(ERROR_SHARING_VIOLATION as i32)
                    || err.raw_os_error() == Some(ERROR_ACCESS_DENIED as i32)
                {
                    return false;
                }
            }
            true
        });
        let entries = entries.map(|r| match r {
            Ok(r) => Ok(r),
            Err(ReaddirError::Io(e)) => Err(e.into()),
            Err(ReaddirError::IllegalSequence) => Err(ErrorCode::IllegalByteSequence.into()),
        });
        Ok(self.table.push(ReaddirIterator::new(entries))?)
    }

    async fn sync(&mut self, fd: Resource<types::Descriptor>) -> FsResult<()> {
        let descriptor = self.table.get(&fd)?;

        match descriptor {
            Descriptor::File(f) => {
                match f.run_blocking(|f| f.sync_all()).await {
                    Ok(()) => Ok(()),
                    // On windows, `sync_data` uses `FileFlushBuffers` which fails with
                    // `ERROR_ACCESS_DENIED` if the file is not upen for writing. Ignore
                    // this error, for POSIX compatibility.
                    #[cfg(windows)]
                    Err(e)
                        if e.raw_os_error()
                            == Some(windows_sys::Win32::Foundation::ERROR_ACCESS_DENIED as _) =>
                    {
                        Ok(())
                    }
                    Err(e) => Err(e.into()),
                }
            }
            Descriptor::Dir(d) => {
                d.run_blocking(|d| Ok(d.open(std::path::Component::CurDir)?.sync_all()?))
                    .await
            }
        }
    }

    async fn create_directory_at(
        &mut self,
        fd: Resource<types::Descriptor>,
        path: String,
    ) -> FsResult<()> {
        let d = self.table.get(&fd)?.dir()?;
        if !d.perms.contains(DirPerms::MUTATE) {
            return Err(ErrorCode::NotPermitted.into());
        }
        d.run_blocking(move |d| d.create_dir(&path)).await?;
        Ok(())
    }

    async fn stat(&mut self, fd: Resource<types::Descriptor>) -> FsResult<types::DescriptorStat> {
        let descriptor = self.table.get(&fd)?;
        match descriptor {
            Descriptor::File(f) => {
                // No permissions check on stat: if opened, allowed to stat it
                let meta = f.run_blocking(|f| f.metadata()).await?;
                Ok(descriptorstat_from(meta))
            }
            Descriptor::Dir(d) => {
                // No permissions check on stat: if opened, allowed to stat it
                let meta = d.run_blocking(|d| d.dir_metadata()).await?;
                Ok(descriptorstat_from(meta))
            }
        }
    }

    async fn stat_at(
        &mut self,
        fd: Resource<types::Descriptor>,
        path_flags: types::PathFlags,
        path: String,
    ) -> FsResult<types::DescriptorStat> {
        let d = self.table.get(&fd)?.dir()?;
        if !d.perms.contains(DirPerms::READ) {
            return Err(ErrorCode::NotPermitted.into());
        }

        let meta = if symlink_follow(path_flags) {
            d.run_blocking(move |d| d.metadata(&path)).await?
        } else {
            d.run_blocking(move |d| d.symlink_metadata(&path)).await?
        };
        Ok(descriptorstat_from(meta))
    }

    async fn set_times_at(
        &mut self,
        fd: Resource<types::Descriptor>,
        path_flags: types::PathFlags,
        path: String,
        atim: types::NewTimestamp,
        mtim: types::NewTimestamp,
    ) -> FsResult<()> {
        use cap_fs_ext::DirExt;

        let d = self.table.get(&fd)?.dir()?;
        if !d.perms.contains(DirPerms::MUTATE) {
            return Err(ErrorCode::NotPermitted.into());
        }
        let atim = systemtimespec_from(atim)?;
        let mtim = systemtimespec_from(mtim)?;
        if symlink_follow(path_flags) {
            d.run_blocking(move |d| {
                d.set_times(
                    &path,
                    atim.map(cap_fs_ext::SystemTimeSpec::from_std),
                    mtim.map(cap_fs_ext::SystemTimeSpec::from_std),
                )
            })
            .await?;
        } else {
            d.run_blocking(move |d| {
                d.set_symlink_times(
                    &path,
                    atim.map(cap_fs_ext::SystemTimeSpec::from_std),
                    mtim.map(cap_fs_ext::SystemTimeSpec::from_std),
                )
            })
            .await?;
        }
        Ok(())
    }

    async fn link_at(
        &mut self,
        fd: Resource<types::Descriptor>,
        // TODO delete the path flags from this function
        old_path_flags: types::PathFlags,
        old_path: String,
        new_descriptor: Resource<types::Descriptor>,
        new_path: String,
    ) -> FsResult<()> {
        let old_dir = self.table.get(&fd)?.dir()?;
        if !old_dir.perms.contains(DirPerms::MUTATE) {
            return Err(ErrorCode::NotPermitted.into());
        }
        let new_dir = self.table.get(&new_descriptor)?.dir()?;
        if !new_dir.perms.contains(DirPerms::MUTATE) {
            return Err(ErrorCode::NotPermitted.into());
        }
        if symlink_follow(old_path_flags) {
            return Err(ErrorCode::Invalid.into());
        }
        if old_dir.perms != new_dir.perms || old_dir.file_perms != new_dir.file_perms {
            return Err(ErrorCode::NotPermitted.into());
        }
        let new_dir_handle = std::sync::Arc::clone(&new_dir.dir);
        old_dir
            .run_blocking(move |d| d.hard_link(&old_path, &new_dir_handle, &new_path))
            .await?;
        Ok(())
    }

    async fn open_at(
        &mut self,
        fd: Resource<types::Descriptor>,
        path_flags: types::PathFlags,
        path: String,
        oflags: types::OpenFlags,
        flags: types::DescriptorFlags,
    ) -> FsResult<Resource<types::Descriptor>> {
        use cap_fs_ext::{FollowSymlinks, OpenOptionsFollowExt, OpenOptionsMaybeDirExt};
        use system_interface::fs::{FdFlags, GetSetFdFlags};
        use types::{DescriptorFlags, OpenFlags};

        let allow_blocking_current_thread = self.ctx.allow_blocking_current_thread;
        let d = self.table.get(&fd)?.dir()?;
        if !d.perms.contains(DirPerms::READ) {
            Err(ErrorCode::NotPermitted)?;
        }

        if !d.perms.contains(DirPerms::MUTATE) {
            if oflags.contains(OpenFlags::CREATE) || oflags.contains(OpenFlags::TRUNCATE) {
                Err(ErrorCode::NotPermitted)?;
            }
            if flags.contains(DescriptorFlags::WRITE) {
                Err(ErrorCode::NotPermitted)?;
            }
        }

        // Track whether we are creating file, for permission check:
        let mut create = false;
        // Track open mode, for permission check and recording in created descriptor:
        let mut open_mode = OpenMode::empty();
        // Construct the OpenOptions to give the OS:
        let mut opts = cap_std::fs::OpenOptions::new();
        opts.maybe_dir(true);

        if oflags.contains(OpenFlags::CREATE) {
            if oflags.contains(OpenFlags::EXCLUSIVE) {
                opts.create_new(true);
            } else {
                opts.create(true);
            }
            create = true;
            opts.write(true);
            open_mode |= OpenMode::WRITE;
        }

        if oflags.contains(OpenFlags::TRUNCATE) {
            opts.truncate(true).write(true);
            open_mode |= OpenMode::WRITE;
        }
        if flags.contains(DescriptorFlags::READ) {
            opts.read(true);
            open_mode |= OpenMode::READ;
        }
        if flags.contains(DescriptorFlags::WRITE) {
            opts.write(true);
            open_mode |= OpenMode::WRITE;
        } else {
            // If not opened write, open read. This way the OS lets us open
            // the file, but we can use perms to reject use of the file later.
            opts.read(true);
            open_mode |= OpenMode::READ;
        }
        if symlink_follow(path_flags) {
            opts.follow(FollowSymlinks::Yes);
        } else {
            opts.follow(FollowSymlinks::No);
        }

        // These flags are not yet supported in cap-std:
        if flags.contains(DescriptorFlags::FILE_INTEGRITY_SYNC)
            || flags.contains(DescriptorFlags::DATA_INTEGRITY_SYNC)
            || flags.contains(DescriptorFlags::REQUESTED_WRITE_SYNC)
        {
            Err(ErrorCode::Unsupported)?;
        }

        if oflags.contains(OpenFlags::DIRECTORY) {
            if oflags.contains(OpenFlags::CREATE)
                || oflags.contains(OpenFlags::EXCLUSIVE)
                || oflags.contains(OpenFlags::TRUNCATE)
            {
                Err(ErrorCode::Invalid)?;
            }
        }

        // Now enforce this WasiCtx's permissions before letting the OS have
        // its shot:
        if !d.perms.contains(DirPerms::MUTATE) && create {
            Err(ErrorCode::NotPermitted)?;
        }
        if !d.file_perms.contains(FilePerms::WRITE) && open_mode.contains(OpenMode::WRITE) {
            Err(ErrorCode::NotPermitted)?;
        }

        // Represents each possible outcome from the spawn_blocking operation.
        // This makes sure we don't have to give spawn_blocking any way to
        // manipulate the table.
        enum OpenResult {
            Dir(cap_std::fs::Dir),
            File(cap_std::fs::File),
            NotDir,
        }

        let opened = d
            .run_blocking::<_, std::io::Result<OpenResult>>(move |d| {
                let mut opened = d.open_with(&path, &opts)?;
                if opened.metadata()?.is_dir() {
                    Ok(OpenResult::Dir(cap_std::fs::Dir::from_std_file(
                        opened.into_std(),
                    )))
                } else if oflags.contains(OpenFlags::DIRECTORY) {
                    Ok(OpenResult::NotDir)
                } else {
                    // FIXME cap-std needs a nonblocking open option so that files reads and writes
                    // are nonblocking. Instead we set it after opening here:
                    let set_fd_flags = opened.new_set_fd_flags(FdFlags::NONBLOCK)?;
                    opened.set_fd_flags(set_fd_flags)?;
                    Ok(OpenResult::File(opened))
                }
            })
            .await?;

        match opened {
            OpenResult::Dir(dir) => Ok(self.table.push(Descriptor::Dir(Dir::new(
                dir,
                d.perms,
                d.file_perms,
                open_mode,
                allow_blocking_current_thread,
            )))?),

            OpenResult::File(file) => Ok(self.table.push(Descriptor::File(File::new(
                file,
                d.file_perms,
                open_mode,
                allow_blocking_current_thread,
            )))?),

            OpenResult::NotDir => Err(ErrorCode::NotDirectory.into()),
        }
    }

    fn drop(&mut self, fd: Resource<types::Descriptor>) -> anyhow::Result<()> {
        // The Drop will close the file/dir, but if the close syscall
        // blocks the thread, I will face god and walk backwards into hell.
        // tokio::fs::File just uses std::fs::File's Drop impl to close, so
        // it doesn't appear anyone else has found this to be a problem.
        // (Not that they could solve it without async drop...)
        self.table.delete(fd)?;

        Ok(())
    }

    async fn readlink_at(
        &mut self,
        fd: Resource<types::Descriptor>,
        path: String,
    ) -> FsResult<String> {
        let d = self.table.get(&fd)?.dir()?;
        if !d.perms.contains(DirPerms::READ) {
            return Err(ErrorCode::NotPermitted.into());
        }
        let link = d.run_blocking(move |d| d.read_link(&path)).await?;
        Ok(link
            .into_os_string()
            .into_string()
            .map_err(|_| ErrorCode::IllegalByteSequence)?)
    }

    async fn remove_directory_at(
        &mut self,
        fd: Resource<types::Descriptor>,
        path: String,
    ) -> FsResult<()> {
        let d = self.table.get(&fd)?.dir()?;
        if !d.perms.contains(DirPerms::MUTATE) {
            return Err(ErrorCode::NotPermitted.into());
        }
        Ok(d.run_blocking(move |d| d.remove_dir(&path)).await?)
    }

    async fn rename_at(
        &mut self,
        fd: Resource<types::Descriptor>,
        old_path: String,
        new_fd: Resource<types::Descriptor>,
        new_path: String,
    ) -> FsResult<()> {
        let old_dir = self.table.get(&fd)?.dir()?;
        if !old_dir.perms.contains(DirPerms::MUTATE) {
            return Err(ErrorCode::NotPermitted.into());
        }
        let new_dir = self.table.get(&new_fd)?.dir()?;
        if !new_dir.perms.contains(DirPerms::MUTATE) {
            return Err(ErrorCode::NotPermitted.into());
        }
        if old_dir.perms != new_dir.perms || old_dir.file_perms != new_dir.file_perms {
            return Err(ErrorCode::NotPermitted.into());
        }
        let new_dir_handle = std::sync::Arc::clone(&new_dir.dir);
        Ok(old_dir
            .run_blocking(move |d| d.rename(&old_path, &new_dir_handle, &new_path))
            .await?)
    }

    async fn symlink_at(
        &mut self,
        fd: Resource<types::Descriptor>,
        src_path: String,
        dest_path: String,
    ) -> FsResult<()> {
        // On windows, Dir.symlink is provided by DirExt
        #[cfg(windows)]
        use cap_fs_ext::DirExt;

        let d = self.table.get(&fd)?.dir()?;
        if !d.perms.contains(DirPerms::MUTATE) {
            return Err(ErrorCode::NotPermitted.into());
        }
        Ok(d.run_blocking(move |d| d.symlink(&src_path, &dest_path))
            .await?)
    }

    async fn unlink_file_at(
        &mut self,
        fd: Resource<types::Descriptor>,
        path: String,
    ) -> FsResult<()> {
        use cap_fs_ext::DirExt;

        let d = self.table.get(&fd)?.dir()?;
        if !d.perms.contains(DirPerms::MUTATE) {
            return Err(ErrorCode::NotPermitted.into());
        }
        Ok(d.run_blocking(move |d| d.remove_file_or_symlink(&path))
            .await?)
    }

    fn read_via_stream(
        &mut self,
        fd: Resource<types::Descriptor>,
        offset: types::Filesize,
    ) -> FsResult<Resource<DynInputStream>> {
        // Trap if fd lookup fails:
        let f = self.table.get(&fd)?.file()?;

        if !f.perms.contains(FilePerms::READ) {
            Err(types::ErrorCode::BadDescriptor)?;
        }

        // Create a stream view for it.
        let reader: DynInputStream = Box::new(FileInputStream::new(f, offset));

        // Insert the stream view into the table. Trap if the table is full.
        let index = self.table.push(reader)?;

        Ok(index)
    }

    fn write_via_stream(
        &mut self,
        fd: Resource<types::Descriptor>,
        offset: types::Filesize,
    ) -> FsResult<Resource<DynOutputStream>> {
        // Trap if fd lookup fails:
        let f = self.table.get(&fd)?.file()?;

        if !f.perms.contains(FilePerms::WRITE) {
            Err(types::ErrorCode::BadDescriptor)?;
        }

        // Create a stream view for it.
        let writer = FileOutputStream::write_at(f, offset);
        let writer: DynOutputStream = Box::new(writer);

        // Insert the stream view into the table. Trap if the table is full.
        let index = self.table.push(writer)?;

        Ok(index)
    }

    fn append_via_stream(
        &mut self,
        fd: Resource<types::Descriptor>,
    ) -> FsResult<Resource<DynOutputStream>> {
        // Trap if fd lookup fails:
        let f = self.table.get(&fd)?.file()?;

        if !f.perms.contains(FilePerms::WRITE) {
            Err(types::ErrorCode::BadDescriptor)?;
        }

        // Create a stream view for it.
        let appender = FileOutputStream::append(f);
        let appender: DynOutputStream = Box::new(appender);

        // Insert the stream view into the table. Trap if the table is full.
        let index = self.table.push(appender)?;

        Ok(index)
    }

    async fn is_same_object(
        &mut self,
        a: Resource<types::Descriptor>,
        b: Resource<types::Descriptor>,
    ) -> anyhow::Result<bool> {
        use cap_fs_ext::MetadataExt;
        let descriptor_a = self.table.get(&a)?;
        let meta_a = get_descriptor_metadata(descriptor_a).await?;
        let descriptor_b = self.table.get(&b)?;
        let meta_b = get_descriptor_metadata(descriptor_b).await?;
        if meta_a.dev() == meta_b.dev() && meta_a.ino() == meta_b.ino() {
            // MetadataHashValue does not derive eq, so use a pair of
            // comparisons to check equality:
            debug_assert_eq!(
                calculate_metadata_hash(&meta_a).upper,
                calculate_metadata_hash(&meta_b).upper
            );
            debug_assert_eq!(
                calculate_metadata_hash(&meta_a).lower,
                calculate_metadata_hash(&meta_b).lower
            );
            Ok(true)
        } else {
            // Hash collisions are possible, so don't assert the negative here
            Ok(false)
        }
    }
    async fn metadata_hash(
        &mut self,
        fd: Resource<types::Descriptor>,
    ) -> FsResult<types::MetadataHashValue> {
        let descriptor_a = self.table.get(&fd)?;
        let meta = get_descriptor_metadata(descriptor_a).await?;
        Ok(calculate_metadata_hash(&meta))
    }
    async fn metadata_hash_at(
        &mut self,
        fd: Resource<types::Descriptor>,
        path_flags: types::PathFlags,
        path: String,
    ) -> FsResult<types::MetadataHashValue> {
        let d = self.table.get(&fd)?.dir()?;
        // No permissions check on metadata: if dir opened, allowed to stat it
        let meta = d
            .run_blocking(move |d| {
                if symlink_follow(path_flags) {
                    d.metadata(path)
                } else {
                    d.symlink_metadata(path)
                }
            })
            .await?;
        Ok(calculate_metadata_hash(&meta))
    }
}

impl HostDirectoryEntryStream for WasiCtxView<'_> {
    async fn read_directory_entry(
        &mut self,
        stream: Resource<types::DirectoryEntryStream>,
    ) -> FsResult<Option<types::DirectoryEntry>> {
        let readdir = self.table.get(&stream)?;
        readdir.next()
    }

    fn drop(&mut self, stream: Resource<types::DirectoryEntryStream>) -> anyhow::Result<()> {
        self.table.delete(stream)?;
        Ok(())
    }
}

async fn get_descriptor_metadata(fd: &types::Descriptor) -> FsResult<cap_std::fs::Metadata> {
    match fd {
        Descriptor::File(f) => {
            // No permissions check on metadata: if opened, allowed to stat it
            Ok(f.run_blocking(|f| f.metadata()).await?)
        }
        Descriptor::Dir(d) => {
            // No permissions check on metadata: if opened, allowed to stat it
            Ok(d.run_blocking(|d| d.dir_metadata()).await?)
        }
    }
}

fn calculate_metadata_hash(meta: &cap_std::fs::Metadata) -> types::MetadataHashValue {
    use cap_fs_ext::MetadataExt;
    // Without incurring any deps, std provides us with a 64 bit hash
    // function:
    use std::hash::Hasher;
    // Note that this means that the metadata hash (which becomes a preview1 ino) may
    // change when a different rustc release is used to build this host implementation:
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    hasher.write_u64(meta.dev());
    hasher.write_u64(meta.ino());
    let lower = hasher.finish();
    // MetadataHashValue has a pair of 64-bit members for representing a
    // single 128-bit number. However, we only have 64 bits of entropy. To
    // synthesize the upper 64 bits, lets xor the lower half with an arbitrary
    // constant, in this case the 64 bit integer corresponding to the IEEE
    // double representation of (a number as close as possible to) pi.
    // This seems better than just repeating the same bits in the upper and
    // lower parts outright, which could make folks wonder if the struct was
    // mangled in the ABI, or worse yet, lead to consumers of this interface
    // expecting them to be equal.
    let upper = lower ^ 4614256656552045848u64;
    types::MetadataHashValue { lower, upper }
}

#[cfg(unix)]
fn from_raw_os_error(err: Option<i32>) -> Option<ErrorCode> {
    use rustix::io::Errno as RustixErrno;
    if err.is_none() {
        return None;
    }
    Some(match RustixErrno::from_raw_os_error(err.unwrap()) {
        RustixErrno::PIPE => ErrorCode::Pipe,
        RustixErrno::PERM => ErrorCode::NotPermitted,
        RustixErrno::NOENT => ErrorCode::NoEntry,
        RustixErrno::NOMEM => ErrorCode::InsufficientMemory,
        RustixErrno::IO => ErrorCode::Io,
        RustixErrno::BADF => ErrorCode::BadDescriptor,
        RustixErrno::BUSY => ErrorCode::Busy,
        RustixErrno::ACCESS => ErrorCode::Access,
        RustixErrno::NOTDIR => ErrorCode::NotDirectory,
        RustixErrno::ISDIR => ErrorCode::IsDirectory,
        RustixErrno::INVAL => ErrorCode::Invalid,
        RustixErrno::EXIST => ErrorCode::Exist,
        RustixErrno::FBIG => ErrorCode::FileTooLarge,
        RustixErrno::NOSPC => ErrorCode::InsufficientSpace,
        RustixErrno::SPIPE => ErrorCode::InvalidSeek,
        RustixErrno::MLINK => ErrorCode::TooManyLinks,
        RustixErrno::NAMETOOLONG => ErrorCode::NameTooLong,
        RustixErrno::NOTEMPTY => ErrorCode::NotEmpty,
        RustixErrno::LOOP => ErrorCode::Loop,
        RustixErrno::OVERFLOW => ErrorCode::Overflow,
        RustixErrno::ILSEQ => ErrorCode::IllegalByteSequence,
        RustixErrno::NOTSUP => ErrorCode::Unsupported,
        RustixErrno::ALREADY => ErrorCode::Already,
        RustixErrno::INPROGRESS => ErrorCode::InProgress,
        RustixErrno::INTR => ErrorCode::Interrupted,

        #[allow(
            unreachable_patterns,
            reason = "on some platforms, these have the same value as other errno values"
        )]
        RustixErrno::OPNOTSUPP => ErrorCode::Unsupported,

        _ => return None,
    })
}
#[cfg(windows)]
fn from_raw_os_error(raw_os_error: Option<i32>) -> Option<ErrorCode> {
    use windows_sys::Win32::Foundation;
    Some(match raw_os_error.map(|code| code as u32) {
        Some(Foundation::ERROR_FILE_NOT_FOUND) => ErrorCode::NoEntry,
        Some(Foundation::ERROR_PATH_NOT_FOUND) => ErrorCode::NoEntry,
        Some(Foundation::ERROR_ACCESS_DENIED) => ErrorCode::Access,
        Some(Foundation::ERROR_SHARING_VIOLATION) => ErrorCode::Access,
        Some(Foundation::ERROR_PRIVILEGE_NOT_HELD) => ErrorCode::NotPermitted,
        Some(Foundation::ERROR_INVALID_HANDLE) => ErrorCode::BadDescriptor,
        Some(Foundation::ERROR_INVALID_NAME) => ErrorCode::NoEntry,
        Some(Foundation::ERROR_NOT_ENOUGH_MEMORY) => ErrorCode::InsufficientMemory,
        Some(Foundation::ERROR_OUTOFMEMORY) => ErrorCode::InsufficientMemory,
        Some(Foundation::ERROR_DIR_NOT_EMPTY) => ErrorCode::NotEmpty,
        Some(Foundation::ERROR_NOT_READY) => ErrorCode::Busy,
        Some(Foundation::ERROR_BUSY) => ErrorCode::Busy,
        Some(Foundation::ERROR_NOT_SUPPORTED) => ErrorCode::Unsupported,
        Some(Foundation::ERROR_FILE_EXISTS) => ErrorCode::Exist,
        Some(Foundation::ERROR_BROKEN_PIPE) => ErrorCode::Pipe,
        Some(Foundation::ERROR_BUFFER_OVERFLOW) => ErrorCode::NameTooLong,
        Some(Foundation::ERROR_NOT_A_REPARSE_POINT) => ErrorCode::Invalid,
        Some(Foundation::ERROR_NEGATIVE_SEEK) => ErrorCode::Invalid,
        Some(Foundation::ERROR_DIRECTORY) => ErrorCode::NotDirectory,
        Some(Foundation::ERROR_ALREADY_EXISTS) => ErrorCode::Exist,
        Some(Foundation::ERROR_STOPPED_ON_SYMLINK) => ErrorCode::Loop,
        Some(Foundation::ERROR_DIRECTORY_NOT_SUPPORTED) => ErrorCode::IsDirectory,
        _ => return None,
    })
}

impl From<std::io::Error> for ErrorCode {
    fn from(err: std::io::Error) -> ErrorCode {
        ErrorCode::from(&err)
    }
}

impl<'a> From<&'a std::io::Error> for ErrorCode {
    fn from(err: &'a std::io::Error) -> ErrorCode {
        match from_raw_os_error(err.raw_os_error()) {
            Some(errno) => errno,
            None => {
                tracing::debug!("unknown raw os error: {err}");
                match err.kind() {
                    std::io::ErrorKind::NotFound => ErrorCode::NoEntry,
                    std::io::ErrorKind::PermissionDenied => ErrorCode::NotPermitted,
                    std::io::ErrorKind::AlreadyExists => ErrorCode::Exist,
                    std::io::ErrorKind::InvalidInput => ErrorCode::Invalid,
                    _ => ErrorCode::Io,
                }
            }
        }
    }
}

impl From<cap_rand::Error> for ErrorCode {
    fn from(err: cap_rand::Error) -> ErrorCode {
        // I picked Error::Io as a 'reasonable default', FIXME dan is this ok?
        from_raw_os_error(err.raw_os_error()).unwrap_or(ErrorCode::Io)
    }
}

impl From<std::num::TryFromIntError> for ErrorCode {
    fn from(_err: std::num::TryFromIntError) -> ErrorCode {
        ErrorCode::Overflow
    }
}

fn descriptortype_from(ft: cap_std::fs::FileType) -> types::DescriptorType {
    use cap_fs_ext::FileTypeExt;
    use types::DescriptorType;
    if ft.is_dir() {
        DescriptorType::Directory
    } else if ft.is_symlink() {
        DescriptorType::SymbolicLink
    } else if ft.is_block_device() {
        DescriptorType::BlockDevice
    } else if ft.is_char_device() {
        DescriptorType::CharacterDevice
    } else if ft.is_file() {
        DescriptorType::RegularFile
    } else {
        DescriptorType::Unknown
    }
}

fn systemtimespec_from(t: types::NewTimestamp) -> FsResult<Option<fs_set_times::SystemTimeSpec>> {
    use fs_set_times::SystemTimeSpec;
    use types::NewTimestamp;
    match t {
        NewTimestamp::NoChange => Ok(None),
        NewTimestamp::Now => Ok(Some(SystemTimeSpec::SymbolicNow)),
        NewTimestamp::Timestamp(st) => Ok(Some(SystemTimeSpec::Absolute(systemtime_from(st)?))),
    }
}

fn systemtime_from(t: wall_clock::Datetime) -> FsResult<std::time::SystemTime> {
    use std::time::{Duration, SystemTime};
    SystemTime::UNIX_EPOCH
        .checked_add(Duration::new(t.seconds, t.nanoseconds))
        .ok_or_else(|| ErrorCode::Overflow.into())
}

fn datetime_from(t: std::time::SystemTime) -> wall_clock::Datetime {
    // FIXME make this infallible or handle errors properly
    wall_clock::Datetime::try_from(cap_std::time::SystemTime::from_std(t)).unwrap()
}

fn descriptorstat_from(meta: cap_std::fs::Metadata) -> types::DescriptorStat {
    use cap_fs_ext::MetadataExt;
    types::DescriptorStat {
        type_: descriptortype_from(meta.file_type()),
        link_count: meta.nlink(),
        size: meta.len(),
        data_access_timestamp: meta.accessed().map(|t| datetime_from(t.into_std())).ok(),
        data_modification_timestamp: meta.modified().map(|t| datetime_from(t.into_std())).ok(),
        status_change_timestamp: meta.created().map(|t| datetime_from(t.into_std())).ok(),
    }
}

fn symlink_follow(path_flags: types::PathFlags) -> bool {
    path_flags.contains(types::PathFlags::SYMLINK_FOLLOW)
}

#[cfg(test)]
mod test {
    use super::*;
    use wasmtime::component::ResourceTable;

    #[test]
    fn table_readdir_works() {
        let mut table = ResourceTable::new();
        let ix = table
            .push(ReaddirIterator::new(std::iter::empty()))
            .unwrap();
        let _ = table.get(&ix).unwrap();
        table.delete(ix).unwrap();
    }
}
