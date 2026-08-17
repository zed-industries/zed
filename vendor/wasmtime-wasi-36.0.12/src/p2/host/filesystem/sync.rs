use crate::p2::bindings::filesystem::types as async_filesystem;
use crate::p2::bindings::sync::filesystem::types as sync_filesystem;
use crate::p2::bindings::sync::io::streams;
use crate::p2::{FsError, FsResult, WasiCtxView};
use crate::runtime::in_tokio;
use wasmtime::component::Resource;

impl sync_filesystem::Host for WasiCtxView<'_> {
    fn convert_error_code(&mut self, err: FsError) -> anyhow::Result<sync_filesystem::ErrorCode> {
        Ok(async_filesystem::Host::convert_error_code(self, err)?.into())
    }

    fn filesystem_error_code(
        &mut self,
        err: Resource<streams::Error>,
    ) -> anyhow::Result<Option<sync_filesystem::ErrorCode>> {
        Ok(async_filesystem::Host::filesystem_error_code(self, err)?.map(|e| e.into()))
    }
}

impl sync_filesystem::HostDescriptor for WasiCtxView<'_> {
    fn advise(
        &mut self,
        fd: Resource<sync_filesystem::Descriptor>,
        offset: sync_filesystem::Filesize,
        len: sync_filesystem::Filesize,
        advice: sync_filesystem::Advice,
    ) -> FsResult<()> {
        in_tokio(async {
            async_filesystem::HostDescriptor::advise(self, fd, offset, len, advice.into()).await
        })
    }

    fn sync_data(&mut self, fd: Resource<sync_filesystem::Descriptor>) -> FsResult<()> {
        in_tokio(async { async_filesystem::HostDescriptor::sync_data(self, fd).await })
    }

    fn get_flags(
        &mut self,
        fd: Resource<sync_filesystem::Descriptor>,
    ) -> FsResult<sync_filesystem::DescriptorFlags> {
        Ok(in_tokio(async { async_filesystem::HostDescriptor::get_flags(self, fd).await })?.into())
    }

    fn get_type(
        &mut self,
        fd: Resource<sync_filesystem::Descriptor>,
    ) -> FsResult<sync_filesystem::DescriptorType> {
        Ok(in_tokio(async { async_filesystem::HostDescriptor::get_type(self, fd).await })?.into())
    }

    fn set_size(
        &mut self,
        fd: Resource<sync_filesystem::Descriptor>,
        size: sync_filesystem::Filesize,
    ) -> FsResult<()> {
        in_tokio(async { async_filesystem::HostDescriptor::set_size(self, fd, size).await })
    }

    fn set_times(
        &mut self,
        fd: Resource<sync_filesystem::Descriptor>,
        atim: sync_filesystem::NewTimestamp,
        mtim: sync_filesystem::NewTimestamp,
    ) -> FsResult<()> {
        in_tokio(async {
            async_filesystem::HostDescriptor::set_times(self, fd, atim.into(), mtim.into()).await
        })
    }

    fn read(
        &mut self,
        fd: Resource<sync_filesystem::Descriptor>,
        len: sync_filesystem::Filesize,
        offset: sync_filesystem::Filesize,
    ) -> FsResult<(Vec<u8>, bool)> {
        in_tokio(async { async_filesystem::HostDescriptor::read(self, fd, len, offset).await })
    }

    fn write(
        &mut self,
        fd: Resource<sync_filesystem::Descriptor>,
        buf: Vec<u8>,
        offset: sync_filesystem::Filesize,
    ) -> FsResult<sync_filesystem::Filesize> {
        in_tokio(async { async_filesystem::HostDescriptor::write(self, fd, buf, offset).await })
    }

    fn read_directory(
        &mut self,
        fd: Resource<sync_filesystem::Descriptor>,
    ) -> FsResult<Resource<sync_filesystem::DirectoryEntryStream>> {
        in_tokio(async { async_filesystem::HostDescriptor::read_directory(self, fd).await })
    }

    fn sync(&mut self, fd: Resource<sync_filesystem::Descriptor>) -> FsResult<()> {
        in_tokio(async { async_filesystem::HostDescriptor::sync(self, fd).await })
    }

    fn create_directory_at(
        &mut self,
        fd: Resource<sync_filesystem::Descriptor>,
        path: String,
    ) -> FsResult<()> {
        in_tokio(async {
            async_filesystem::HostDescriptor::create_directory_at(self, fd, path).await
        })
    }

    fn stat(
        &mut self,
        fd: Resource<sync_filesystem::Descriptor>,
    ) -> FsResult<sync_filesystem::DescriptorStat> {
        Ok(in_tokio(async { async_filesystem::HostDescriptor::stat(self, fd).await })?.into())
    }

    fn stat_at(
        &mut self,
        fd: Resource<sync_filesystem::Descriptor>,
        path_flags: sync_filesystem::PathFlags,
        path: String,
    ) -> FsResult<sync_filesystem::DescriptorStat> {
        Ok(in_tokio(async {
            async_filesystem::HostDescriptor::stat_at(self, fd, path_flags.into(), path).await
        })?
        .into())
    }

    fn set_times_at(
        &mut self,
        fd: Resource<sync_filesystem::Descriptor>,
        path_flags: sync_filesystem::PathFlags,
        path: String,
        atim: sync_filesystem::NewTimestamp,
        mtim: sync_filesystem::NewTimestamp,
    ) -> FsResult<()> {
        in_tokio(async {
            async_filesystem::HostDescriptor::set_times_at(
                self,
                fd,
                path_flags.into(),
                path,
                atim.into(),
                mtim.into(),
            )
            .await
        })
    }

    fn link_at(
        &mut self,
        fd: Resource<sync_filesystem::Descriptor>,
        // TODO delete the path flags from this function
        old_path_flags: sync_filesystem::PathFlags,
        old_path: String,
        new_descriptor: Resource<sync_filesystem::Descriptor>,
        new_path: String,
    ) -> FsResult<()> {
        in_tokio(async {
            async_filesystem::HostDescriptor::link_at(
                self,
                fd,
                old_path_flags.into(),
                old_path,
                new_descriptor,
                new_path,
            )
            .await
        })
    }

    fn open_at(
        &mut self,
        fd: Resource<sync_filesystem::Descriptor>,
        path_flags: sync_filesystem::PathFlags,
        path: String,
        oflags: sync_filesystem::OpenFlags,
        flags: sync_filesystem::DescriptorFlags,
    ) -> FsResult<Resource<sync_filesystem::Descriptor>> {
        in_tokio(async {
            async_filesystem::HostDescriptor::open_at(
                self,
                fd,
                path_flags.into(),
                path,
                oflags.into(),
                flags.into(),
            )
            .await
        })
    }

    fn drop(&mut self, fd: Resource<sync_filesystem::Descriptor>) -> anyhow::Result<()> {
        async_filesystem::HostDescriptor::drop(self, fd)
    }

    fn readlink_at(
        &mut self,
        fd: Resource<sync_filesystem::Descriptor>,
        path: String,
    ) -> FsResult<String> {
        in_tokio(async { async_filesystem::HostDescriptor::readlink_at(self, fd, path).await })
    }

    fn remove_directory_at(
        &mut self,
        fd: Resource<sync_filesystem::Descriptor>,
        path: String,
    ) -> FsResult<()> {
        in_tokio(async {
            async_filesystem::HostDescriptor::remove_directory_at(self, fd, path).await
        })
    }

    fn rename_at(
        &mut self,
        fd: Resource<sync_filesystem::Descriptor>,
        old_path: String,
        new_fd: Resource<sync_filesystem::Descriptor>,
        new_path: String,
    ) -> FsResult<()> {
        in_tokio(async {
            async_filesystem::HostDescriptor::rename_at(self, fd, old_path, new_fd, new_path).await
        })
    }

    fn symlink_at(
        &mut self,
        fd: Resource<sync_filesystem::Descriptor>,
        src_path: String,
        dest_path: String,
    ) -> FsResult<()> {
        in_tokio(async {
            async_filesystem::HostDescriptor::symlink_at(self, fd, src_path, dest_path).await
        })
    }

    fn unlink_file_at(
        &mut self,
        fd: Resource<sync_filesystem::Descriptor>,
        path: String,
    ) -> FsResult<()> {
        in_tokio(async { async_filesystem::HostDescriptor::unlink_file_at(self, fd, path).await })
    }

    fn read_via_stream(
        &mut self,
        fd: Resource<sync_filesystem::Descriptor>,
        offset: sync_filesystem::Filesize,
    ) -> FsResult<Resource<streams::InputStream>> {
        Ok(async_filesystem::HostDescriptor::read_via_stream(
            self, fd, offset,
        )?)
    }

    fn write_via_stream(
        &mut self,
        fd: Resource<sync_filesystem::Descriptor>,
        offset: sync_filesystem::Filesize,
    ) -> FsResult<Resource<streams::OutputStream>> {
        Ok(async_filesystem::HostDescriptor::write_via_stream(
            self, fd, offset,
        )?)
    }

    fn append_via_stream(
        &mut self,
        fd: Resource<sync_filesystem::Descriptor>,
    ) -> FsResult<Resource<streams::OutputStream>> {
        Ok(async_filesystem::HostDescriptor::append_via_stream(
            self, fd,
        )?)
    }

    fn is_same_object(
        &mut self,
        a: Resource<sync_filesystem::Descriptor>,
        b: Resource<sync_filesystem::Descriptor>,
    ) -> anyhow::Result<bool> {
        in_tokio(async { async_filesystem::HostDescriptor::is_same_object(self, a, b).await })
    }
    fn metadata_hash(
        &mut self,
        fd: Resource<sync_filesystem::Descriptor>,
    ) -> FsResult<sync_filesystem::MetadataHashValue> {
        Ok(
            in_tokio(async { async_filesystem::HostDescriptor::metadata_hash(self, fd).await })?
                .into(),
        )
    }
    fn metadata_hash_at(
        &mut self,
        fd: Resource<sync_filesystem::Descriptor>,
        path_flags: sync_filesystem::PathFlags,
        path: String,
    ) -> FsResult<sync_filesystem::MetadataHashValue> {
        Ok(in_tokio(async {
            async_filesystem::HostDescriptor::metadata_hash_at(self, fd, path_flags.into(), path)
                .await
        })?
        .into())
    }
}

impl sync_filesystem::HostDirectoryEntryStream for WasiCtxView<'_> {
    fn read_directory_entry(
        &mut self,
        stream: Resource<sync_filesystem::DirectoryEntryStream>,
    ) -> FsResult<Option<sync_filesystem::DirectoryEntry>> {
        Ok(in_tokio(async {
            async_filesystem::HostDirectoryEntryStream::read_directory_entry(self, stream).await
        })?
        .map(|e| e.into()))
    }

    fn drop(
        &mut self,
        stream: Resource<sync_filesystem::DirectoryEntryStream>,
    ) -> anyhow::Result<()> {
        async_filesystem::HostDirectoryEntryStream::drop(self, stream)
    }
}

impl From<async_filesystem::ErrorCode> for sync_filesystem::ErrorCode {
    fn from(other: async_filesystem::ErrorCode) -> Self {
        use async_filesystem::ErrorCode;
        match other {
            ErrorCode::Access => Self::Access,
            ErrorCode::WouldBlock => Self::WouldBlock,
            ErrorCode::Already => Self::Already,
            ErrorCode::BadDescriptor => Self::BadDescriptor,
            ErrorCode::Busy => Self::Busy,
            ErrorCode::Deadlock => Self::Deadlock,
            ErrorCode::Quota => Self::Quota,
            ErrorCode::Exist => Self::Exist,
            ErrorCode::FileTooLarge => Self::FileTooLarge,
            ErrorCode::IllegalByteSequence => Self::IllegalByteSequence,
            ErrorCode::InProgress => Self::InProgress,
            ErrorCode::Interrupted => Self::Interrupted,
            ErrorCode::Invalid => Self::Invalid,
            ErrorCode::Io => Self::Io,
            ErrorCode::IsDirectory => Self::IsDirectory,
            ErrorCode::Loop => Self::Loop,
            ErrorCode::TooManyLinks => Self::TooManyLinks,
            ErrorCode::MessageSize => Self::MessageSize,
            ErrorCode::NameTooLong => Self::NameTooLong,
            ErrorCode::NoDevice => Self::NoDevice,
            ErrorCode::NoEntry => Self::NoEntry,
            ErrorCode::NoLock => Self::NoLock,
            ErrorCode::InsufficientMemory => Self::InsufficientMemory,
            ErrorCode::InsufficientSpace => Self::InsufficientSpace,
            ErrorCode::NotDirectory => Self::NotDirectory,
            ErrorCode::NotEmpty => Self::NotEmpty,
            ErrorCode::NotRecoverable => Self::NotRecoverable,
            ErrorCode::Unsupported => Self::Unsupported,
            ErrorCode::NoTty => Self::NoTty,
            ErrorCode::NoSuchDevice => Self::NoSuchDevice,
            ErrorCode::Overflow => Self::Overflow,
            ErrorCode::NotPermitted => Self::NotPermitted,
            ErrorCode::Pipe => Self::Pipe,
            ErrorCode::ReadOnly => Self::ReadOnly,
            ErrorCode::InvalidSeek => Self::InvalidSeek,
            ErrorCode::TextFileBusy => Self::TextFileBusy,
            ErrorCode::CrossDevice => Self::CrossDevice,
        }
    }
}

impl From<sync_filesystem::Advice> for async_filesystem::Advice {
    fn from(other: sync_filesystem::Advice) -> Self {
        use sync_filesystem::Advice;
        match other {
            Advice::Normal => Self::Normal,
            Advice::Sequential => Self::Sequential,
            Advice::Random => Self::Random,
            Advice::WillNeed => Self::WillNeed,
            Advice::DontNeed => Self::DontNeed,
            Advice::NoReuse => Self::NoReuse,
        }
    }
}

impl From<async_filesystem::DescriptorFlags> for sync_filesystem::DescriptorFlags {
    fn from(other: async_filesystem::DescriptorFlags) -> Self {
        let mut out = Self::empty();
        if other.contains(async_filesystem::DescriptorFlags::READ) {
            out |= Self::READ;
        }
        if other.contains(async_filesystem::DescriptorFlags::WRITE) {
            out |= Self::WRITE;
        }
        if other.contains(async_filesystem::DescriptorFlags::FILE_INTEGRITY_SYNC) {
            out |= Self::FILE_INTEGRITY_SYNC;
        }
        if other.contains(async_filesystem::DescriptorFlags::DATA_INTEGRITY_SYNC) {
            out |= Self::DATA_INTEGRITY_SYNC;
        }
        if other.contains(async_filesystem::DescriptorFlags::REQUESTED_WRITE_SYNC) {
            out |= Self::REQUESTED_WRITE_SYNC;
        }
        if other.contains(async_filesystem::DescriptorFlags::MUTATE_DIRECTORY) {
            out |= Self::MUTATE_DIRECTORY;
        }
        out
    }
}

impl From<async_filesystem::DescriptorType> for sync_filesystem::DescriptorType {
    fn from(other: async_filesystem::DescriptorType) -> Self {
        use async_filesystem::DescriptorType;
        match other {
            DescriptorType::RegularFile => Self::RegularFile,
            DescriptorType::Directory => Self::Directory,
            DescriptorType::BlockDevice => Self::BlockDevice,
            DescriptorType::CharacterDevice => Self::CharacterDevice,
            DescriptorType::Fifo => Self::Fifo,
            DescriptorType::Socket => Self::Socket,
            DescriptorType::SymbolicLink => Self::SymbolicLink,
            DescriptorType::Unknown => Self::Unknown,
        }
    }
}

impl From<async_filesystem::DirectoryEntry> for sync_filesystem::DirectoryEntry {
    fn from(other: async_filesystem::DirectoryEntry) -> Self {
        Self {
            type_: other.type_.into(),
            name: other.name,
        }
    }
}

impl From<async_filesystem::DescriptorStat> for sync_filesystem::DescriptorStat {
    fn from(other: async_filesystem::DescriptorStat) -> Self {
        Self {
            type_: other.type_.into(),
            link_count: other.link_count,
            size: other.size,
            data_access_timestamp: other.data_access_timestamp,
            data_modification_timestamp: other.data_modification_timestamp,
            status_change_timestamp: other.status_change_timestamp,
        }
    }
}

impl From<sync_filesystem::PathFlags> for async_filesystem::PathFlags {
    fn from(other: sync_filesystem::PathFlags) -> Self {
        let mut out = Self::empty();
        if other.contains(sync_filesystem::PathFlags::SYMLINK_FOLLOW) {
            out |= Self::SYMLINK_FOLLOW;
        }
        out
    }
}

impl From<sync_filesystem::NewTimestamp> for async_filesystem::NewTimestamp {
    fn from(other: sync_filesystem::NewTimestamp) -> Self {
        use sync_filesystem::NewTimestamp;
        match other {
            NewTimestamp::NoChange => Self::NoChange,
            NewTimestamp::Now => Self::Now,
            NewTimestamp::Timestamp(datetime) => Self::Timestamp(datetime),
        }
    }
}

impl From<sync_filesystem::OpenFlags> for async_filesystem::OpenFlags {
    fn from(other: sync_filesystem::OpenFlags) -> Self {
        let mut out = Self::empty();
        if other.contains(sync_filesystem::OpenFlags::CREATE) {
            out |= Self::CREATE;
        }
        if other.contains(sync_filesystem::OpenFlags::DIRECTORY) {
            out |= Self::DIRECTORY;
        }
        if other.contains(sync_filesystem::OpenFlags::EXCLUSIVE) {
            out |= Self::EXCLUSIVE;
        }
        if other.contains(sync_filesystem::OpenFlags::TRUNCATE) {
            out |= Self::TRUNCATE;
        }
        out
    }
}
impl From<sync_filesystem::DescriptorFlags> for async_filesystem::DescriptorFlags {
    fn from(other: sync_filesystem::DescriptorFlags) -> Self {
        let mut out = Self::empty();
        if other.contains(sync_filesystem::DescriptorFlags::READ) {
            out |= Self::READ;
        }
        if other.contains(sync_filesystem::DescriptorFlags::WRITE) {
            out |= Self::WRITE;
        }
        if other.contains(sync_filesystem::DescriptorFlags::FILE_INTEGRITY_SYNC) {
            out |= Self::FILE_INTEGRITY_SYNC;
        }
        if other.contains(sync_filesystem::DescriptorFlags::DATA_INTEGRITY_SYNC) {
            out |= Self::DATA_INTEGRITY_SYNC;
        }
        if other.contains(sync_filesystem::DescriptorFlags::REQUESTED_WRITE_SYNC) {
            out |= Self::REQUESTED_WRITE_SYNC;
        }
        if other.contains(sync_filesystem::DescriptorFlags::MUTATE_DIRECTORY) {
            out |= Self::MUTATE_DIRECTORY;
        }
        out
    }
}
impl From<async_filesystem::MetadataHashValue> for sync_filesystem::MetadataHashValue {
    fn from(other: async_filesystem::MetadataHashValue) -> Self {
        Self {
            lower: other.lower,
            upper: other.upper,
        }
    }
}
