use anyhow::Result;
use fs::Fs;
use std::sync::Arc;
use wasi::{
    filesystem::{
        preopens,
        types::{self, ErrorCode, HostDescriptor, HostDirectoryEntryStream},
    },
    io::streams,
};
use wasmtime::component::{Linker, Resource, ResourceTable};

pub trait WasiFsView: Send {
    fn table(&mut self) -> &mut ResourceTable;
    fn fs(&self) -> &Arc<dyn Fs>;
}

wasmtime::component::bindgen!({
    path: "wit",
    world: "wasi:filesystem/imports",
    trappable_imports: true,
    async: {
        only_imports: [
            "[method]descriptor.access-at",
            "[method]descriptor.advise",
            "[method]descriptor.change-directory-permissions-at",
            "[method]descriptor.change-file-permissions-at",
            "[method]descriptor.create-directory-at",
            "[method]descriptor.get-flags",
            "[method]descriptor.get-type",
            "[method]descriptor.is-same-object",
            "[method]descriptor.link-at",
            "[method]descriptor.lock-exclusive",
            "[method]descriptor.lock-shared",
            "[method]descriptor.metadata-hash",
            "[method]descriptor.metadata-hash-at",
            "[method]descriptor.open-at",
            "[method]descriptor.read",
            "[method]descriptor.read-directory",
            "[method]descriptor.readlink-at",
            "[method]descriptor.remove-directory-at",
            "[method]descriptor.rename-at",
            "[method]descriptor.set-size",
            "[method]descriptor.set-times",
            "[method]descriptor.set-times-at",
            "[method]descriptor.stat",
            "[method]descriptor.stat-at",
            "[method]descriptor.symlink-at",
            "[method]descriptor.sync",
            "[method]descriptor.sync-data",
            "[method]descriptor.try-lock-exclusive",
            "[method]descriptor.try-lock-shared",
            "[method]descriptor.unlink-file-at",
            "[method]descriptor.unlock",
            "[method]descriptor.write",
            "[method]input-stream.read",
            "[method]input-stream.blocking-read",
            "[method]input-stream.blocking-skip",
            "[method]input-stream.skip",
            "[method]output-stream.forward",
            "[method]output-stream.splice",
            "[method]output-stream.blocking-splice",
            "[method]output-stream.blocking-flush",
            "[method]output-stream.blocking-write",
            "[method]output-stream.blocking-write-and-flush",
            "[method]output-stream.blocking-write-zeroes-and-flush",
            "[method]directory-entry-stream.read-directory-entry",
            "poll",
            "[method]pollable.block",
            "[method]pollable.ready",
        ],
    },
    trappable_error_type: {
        "wasi:io/streams/stream-error" => StreamError,
        "wasi:filesystem/types/error-code" => FsError,
    },
    with: {
        "wasi:filesystem/types/directory-entry-stream": ReaddirIterator,
        "wasi:filesystem/types/descriptor": Descriptor,
        "wasi:io/streams/input-stream": InputStream,
        "wasi:io/streams/output-stream": OutputStream,
        "wasi:io/error/error": StreamError,
        "wasi:io/poll/pollable": Pollable,
    },
    skip_mut_forwarding_impls: true,
});

pub fn add_to_linker<T: WasiFsView + 'static>(linker: &mut Linker<T>) -> Result<()> {
    fn id<'a, T>(state: &'a mut T) -> &'a mut T {
        state
    }

    wasi::filesystem::types::add_to_linker_get_host(linker, id)?;
    wasi::filesystem::preopens::add_to_linker_get_host(linker, id)?;
    wasi::io::streams::add_to_linker_get_host(linker, id)?;

    Ok(())
}

impl<T: WasiFsView> WasiFsView for &mut T {
    fn table(&mut self) -> &mut ResourceTable {
        T::table(self)
    }

    fn fs(&self) -> &Arc<dyn Fs> {
        T::fs(self)
    }
}

#[async_trait::async_trait]
impl<T: WasiFsView> HostDescriptor for T {
    async fn advise(
        &mut self,
        _fd: Resource<Descriptor>,
        _offset: types::Filesize,
        _len: types::Filesize,
        _advice: types::Advice,
    ) -> FsResult<()> {
        unimplemented!()
    }

    async fn sync_data(&mut self, _fd: Resource<types::Descriptor>) -> FsResult<()> {
        unimplemented!()
    }

    async fn get_flags(&mut self, _fd: Resource<Descriptor>) -> FsResult<types::DescriptorFlags> {
        unimplemented!()
    }

    async fn get_type(&mut self, _fd: Resource<Descriptor>) -> FsResult<types::DescriptorType> {
        unimplemented!()
    }

    async fn set_size(
        &mut self,
        _fd: Resource<Descriptor>,
        _size: types::Filesize,
    ) -> FsResult<()> {
        unimplemented!()
    }

    async fn set_times(
        &mut self,
        _fd: Resource<Descriptor>,
        _atim: types::NewTimestamp,
        _mtim: types::NewTimestamp,
    ) -> FsResult<()> {
        unimplemented!()
    }

    async fn read(
        &mut self,
        _fd: Resource<Descriptor>,
        _len: types::Filesize,
        _offset: types::Filesize,
    ) -> FsResult<(Vec<u8>, bool)> {
        unimplemented!()
    }

    async fn write(
        &mut self,
        _fd: Resource<Descriptor>,
        _buf: Vec<u8>,
        _offset: types::Filesize,
    ) -> FsResult<types::Filesize> {
        unimplemented!()
    }

    async fn read_directory(
        &mut self,
        _fd: Resource<Descriptor>,
    ) -> FsResult<Resource<types::DirectoryEntryStream>> {
        unimplemented!()
    }

    async fn sync(&mut self, _fd: Resource<Descriptor>) -> FsResult<()> {
        unimplemented!()
    }

    async fn create_directory_at(
        &mut self,
        _fd: Resource<Descriptor>,
        _path: String,
    ) -> FsResult<()> {
        unimplemented!()
    }

    async fn stat(&mut self, _fd: Resource<Descriptor>) -> FsResult<types::DescriptorStat> {
        unimplemented!()
    }

    async fn stat_at(
        &mut self,
        _fd: Resource<Descriptor>,
        _path_flags: types::PathFlags,
        _path: String,
    ) -> FsResult<types::DescriptorStat> {
        unimplemented!()
    }

    async fn set_times_at(
        &mut self,
        _fd: Resource<Descriptor>,
        _path_flags: types::PathFlags,
        _path: String,
        _atim: types::NewTimestamp,
        _mtim: types::NewTimestamp,
    ) -> FsResult<()> {
        unimplemented!()
    }

    async fn link_at(
        &mut self,
        _fd: Resource<Descriptor>,
        _old_path_flags: types::PathFlags,
        _old_path: String,
        _new_descriptor: Resource<Descriptor>,
        _new_path: String,
    ) -> FsResult<()> {
        unimplemented!()
    }

    async fn open_at(
        &mut self,
        _fd: Resource<Descriptor>,
        _path_flags: types::PathFlags,
        _path: String,
        _oflags: types::OpenFlags,
        _flags: types::DescriptorFlags,
    ) -> FsResult<Resource<types::Descriptor>> {
        unimplemented!()
    }

    fn drop(&mut self, _fd: Resource<types::Descriptor>) -> anyhow::Result<()> {
        unimplemented!()
    }

    async fn readlink_at(
        &mut self,
        _fd: Resource<types::Descriptor>,
        _path: String,
    ) -> FsResult<String> {
        unimplemented!()
    }

    async fn remove_directory_at(
        &mut self,
        _fd: Resource<types::Descriptor>,
        _path: String,
    ) -> FsResult<()> {
        unimplemented!()
    }

    async fn rename_at(
        &mut self,
        _fd: Resource<types::Descriptor>,
        _old_path: String,
        _new_fd: Resource<types::Descriptor>,
        _new_path: String,
    ) -> FsResult<()> {
        unimplemented!()
    }

    async fn symlink_at(
        &mut self,
        _fd: Resource<types::Descriptor>,
        _src_path: String,
        _dest_path: String,
    ) -> FsResult<()> {
        unimplemented!()
    }

    async fn unlink_file_at(
        &mut self,
        _fd: Resource<types::Descriptor>,
        _path: String,
    ) -> FsResult<()> {
        unimplemented!()
    }

    fn read_via_stream(
        &mut self,
        _fd: Resource<types::Descriptor>,
        _offset: types::Filesize,
    ) -> FsResult<Resource<InputStream>> {
        unimplemented!()
    }

    fn write_via_stream(
        &mut self,
        _fd: Resource<types::Descriptor>,
        _offset: types::Filesize,
    ) -> FsResult<Resource<OutputStream>> {
        unimplemented!()
    }

    fn append_via_stream(
        &mut self,
        _fd: Resource<types::Descriptor>,
    ) -> FsResult<Resource<OutputStream>> {
        unimplemented!()
    }

    async fn is_same_object(
        &mut self,
        _a: Resource<types::Descriptor>,
        _b: Resource<types::Descriptor>,
    ) -> anyhow::Result<bool> {
        unimplemented!()
    }

    async fn metadata_hash(
        &mut self,
        _fd: Resource<types::Descriptor>,
    ) -> FsResult<types::MetadataHashValue> {
        unimplemented!()
    }

    async fn metadata_hash_at(
        &mut self,
        _fd: Resource<types::Descriptor>,
        _path_flags: types::PathFlags,
        _path: String,
    ) -> FsResult<types::MetadataHashValue> {
        unimplemented!()
    }
}

#[async_trait::async_trait]
impl<T: WasiFsView> HostDirectoryEntryStream for T {
    async fn read_directory_entry(
        &mut self,
        _stream: Resource<types::DirectoryEntryStream>,
    ) -> FsResult<Option<types::DirectoryEntry>> {
        unimplemented!()
    }

    fn drop(&mut self, _stream: Resource<types::DirectoryEntryStream>) -> anyhow::Result<()> {
        unimplemented!()
    }
}

#[async_trait::async_trait]
impl<T: WasiFsView> types::Host for T {
    fn convert_error_code(&mut self, _err: FsError) -> anyhow::Result<ErrorCode> {
        unimplemented!()
    }

    fn filesystem_error_code(
        &mut self,
        _err: Resource<StreamError>,
    ) -> anyhow::Result<Option<ErrorCode>> {
        unimplemented!()
    }
}

#[async_trait::async_trait]
impl<T: WasiFsView> streams::Host for T {
    fn convert_stream_error(&mut self, err: StreamError) -> anyhow::Result<streams::StreamError> {
        unimplemented!()
    }
}

#[async_trait::async_trait]
impl<T: WasiFsView> streams::HostOutputStream for T {}

#[async_trait::async_trait]
impl<T: WasiFsView> streams::HostInputStream for T {}

#[async_trait::async_trait]
impl<T: WasiFsView> preopens::Host for T {
    fn get_directories(
        &mut self,
    ) -> Result<Vec<(Resource<types::Descriptor>, String)>, anyhow::Error> {
        unimplemented!()
    }
}

pub struct InputStream {}

pub struct OutputStream {}

pub struct Descriptor {}

pub struct ReaddirIterator {}

pub struct StreamError {}

pub struct IoError {}

pub struct Pollable {}

pub type FsResult<T> = Result<T, FsError>;

pub struct FsError {}
