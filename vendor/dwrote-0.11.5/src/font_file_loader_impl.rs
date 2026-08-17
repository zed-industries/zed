#![allow(non_snake_case, non_upper_case_globals)]

use std::collections::HashMap;
use std::marker::Send;
use std::sync::atomic::AtomicUsize;
use std::sync::{atomic, Arc, Mutex};
use std::{mem, ptr};
use winapi::ctypes::c_void;
use winapi::shared::basetsd::{UINT32, UINT64};
use winapi::shared::guiddef::REFIID;
use winapi::shared::minwindef::ULONG;
use winapi::shared::winerror::{E_FAIL, E_INVALIDARG, E_NOTIMPL, S_OK};
use winapi::um::dwrite::IDWriteFontFile;
use winapi::um::dwrite::{IDWriteFontFileLoader, IDWriteFontFileLoaderVtbl};
use winapi::um::dwrite::{IDWriteFontFileStream, IDWriteFontFileStreamVtbl};
use winapi::um::unknwnbase::{IUnknown, IUnknownVtbl};
use winapi::um::winnt::HRESULT;
use wio::com::ComPtr;

use super::DWriteFactory;
use crate::com_helpers::*;

struct FontFileLoader;

const FontFileLoaderVtbl: &IDWriteFontFileLoaderVtbl = &IDWriteFontFileLoaderVtbl {
    parent: implement_iunknown!(static IDWriteFontFileLoader, FontFileLoader),
    CreateStreamFromKey: {
        unsafe extern "system" fn CreateStreamFromKey(
            _This: *mut IDWriteFontFileLoader,
            fontFileReferenceKey: *const c_void,
            fontFileReferenceKeySize: UINT32,
            fontFileStream: *mut *mut IDWriteFontFileStream,
        ) -> HRESULT {
            if fontFileReferenceKey.is_null() || fontFileStream.is_null() {
                return E_INVALIDARG;
            }
            assert!(fontFileReferenceKeySize == mem::size_of::<usize>() as UINT32);
            let key = *(fontFileReferenceKey as *const usize);
            let stream = match FONT_FILE_STREAM_MAP.lock().unwrap().get(&key) {
                None => {
                    *fontFileStream = ptr::null_mut();
                    return E_FAIL;
                }
                Some(&FontFileStreamPtr(file_stream)) => file_stream,
            };

            // This is an addref getter, so make sure to do that!
            (*stream).AddRef();

            *fontFileStream = stream;
            S_OK
        }
        CreateStreamFromKey
    },
};

impl Com<IDWriteFontFileLoader> for FontFileLoader {
    type Vtbl = IDWriteFontFileLoaderVtbl;
    fn vtbl() -> &'static IDWriteFontFileLoaderVtbl {
        FontFileLoaderVtbl
    }
}

impl Com<IUnknown> for FontFileLoader {
    type Vtbl = IUnknownVtbl;
    fn vtbl() -> &'static IUnknownVtbl {
        &FontFileLoaderVtbl.parent
    }
}

impl FontFileLoader {
    pub fn new() -> FontFileLoader {
        FontFileLoader
    }
}

unsafe impl Send for FontFileLoader {}
unsafe impl Sync for FontFileLoader {}

struct FontFileStream {
    refcount: atomic::AtomicUsize,
    key: usize,
    data: Arc<dyn AsRef<[u8]> + Sync + Send>,
}

const FontFileStreamVtbl: &IDWriteFontFileStreamVtbl = &IDWriteFontFileStreamVtbl {
    parent: implement_iunknown!(IDWriteFontFileStream, FontFileStream),
    ReadFileFragment: {
        unsafe extern "system" fn ReadFileFragment(
            This: *mut IDWriteFontFileStream,
            fragmentStart: *mut *const c_void,
            fileOffset: UINT64,
            fragmentSize: UINT64,
            fragmentContext: *mut *mut c_void,
        ) -> HRESULT {
            let this = FontFileStream::from_interface(This);
            *fragmentContext = ptr::null_mut();
            let data = (*this.data).as_ref();
            if (fileOffset + fragmentSize) as usize > data.len() {
                return E_INVALIDARG;
            }
            let index = fileOffset as usize;
            *fragmentStart = data[index..].as_ptr() as *const c_void;
            S_OK
        }
        ReadFileFragment
    },
    ReleaseFileFragment: {
        unsafe extern "system" fn ReleaseFileFragment(
            _This: *mut IDWriteFontFileStream,
            _fragmentContext: *mut c_void,
        ) {
        }
        ReleaseFileFragment
    },
    GetFileSize: {
        unsafe extern "system" fn GetFileSize(
            This: *mut IDWriteFontFileStream,
            fileSize: *mut UINT64,
        ) -> HRESULT {
            let this = FontFileStream::from_interface(This);
            *fileSize = (*this.data).as_ref().len() as UINT64;
            S_OK
        }
        GetFileSize
    },
    GetLastWriteTime: {
        unsafe extern "system" fn GetLastWriteTime(
            _This: *mut IDWriteFontFileStream,
            _lastWriteTime: *mut UINT64,
        ) -> HRESULT {
            E_NOTIMPL
        }
        GetLastWriteTime
    },
};

impl FontFileStream {
    pub fn new(key: usize, data: Arc<dyn AsRef<[u8]> + Sync + Send>) -> FontFileStream {
        FontFileStream {
            refcount: AtomicUsize::new(1),
            key,
            data,
        }
    }
}

impl Drop for FontFileStream {
    fn drop(&mut self) {
        DataFontHelper::unregister_font_data(self.key);
    }
}

impl Com<IDWriteFontFileStream> for FontFileStream {
    type Vtbl = IDWriteFontFileStreamVtbl;
    fn vtbl() -> &'static IDWriteFontFileStreamVtbl {
        FontFileStreamVtbl
    }
}

impl Com<IUnknown> for FontFileStream {
    type Vtbl = IUnknownVtbl;
    fn vtbl() -> &'static IUnknownVtbl {
        &FontFileStreamVtbl.parent
    }
}

struct FontFileStreamPtr(*mut IDWriteFontFileStream);

unsafe impl Send for FontFileStreamPtr {}

static mut FONT_FILE_KEY: atomic::AtomicUsize = AtomicUsize::new(0);

#[derive(Clone)]
struct FontFileLoaderWrapper(ComPtr<IDWriteFontFileLoader>);

unsafe impl Send for FontFileLoaderWrapper {}
unsafe impl Sync for FontFileLoaderWrapper {}

lazy_static! {
    static ref FONT_FILE_STREAM_MAP: Mutex<HashMap<usize, FontFileStreamPtr>> =
        Mutex::new(HashMap::new());
    static ref FONT_FILE_LOADER: Mutex<FontFileLoaderWrapper> = {
        unsafe {
            let ffl_native = FontFileLoader::new();
            let ffl = ComPtr::<IDWriteFontFileLoader>::from_raw(ffl_native.into_interface());
            let hr = (*DWriteFactory()).RegisterFontFileLoader(ffl.as_raw());
            assert!(hr == 0);
            Mutex::new(FontFileLoaderWrapper(ffl))
        }
    };
}

pub(crate) struct DataFontHelper;

impl DataFontHelper {
    pub(crate) fn register_font_buffer(
        font_data: Arc<dyn AsRef<[u8]> + Sync + Send>,
    ) -> (
        ComPtr<IDWriteFontFile>,
        ComPtr<IDWriteFontFileStream>,
        usize,
    ) {
        unsafe {
            let key = FONT_FILE_KEY.fetch_add(1, atomic::Ordering::Relaxed);
            let font_file_stream_native = FontFileStream::new(key, font_data);
            let font_file_stream: ComPtr<IDWriteFontFileStream> =
                ComPtr::from_raw(font_file_stream_native.into_interface());

            {
                let mut map = FONT_FILE_STREAM_MAP.lock().unwrap();
                map.insert(key, FontFileStreamPtr(font_file_stream.as_raw()));
            }

            let mut font_file: *mut IDWriteFontFile = ptr::null_mut();
            {
                let loader = FONT_FILE_LOADER.lock().unwrap();
                let hr = (*DWriteFactory()).CreateCustomFontFileReference(
                    mem::transmute(&key),
                    mem::size_of::<usize>() as UINT32,
                    loader.0.as_raw(),
                    &mut font_file,
                );
                assert!(hr == S_OK);
            }
            let font_file = ComPtr::from_raw(font_file);

            (font_file, font_file_stream, key)
        }
    }

    fn unregister_font_data(key: usize) {
        let mut map = FONT_FILE_STREAM_MAP.lock().unwrap();
        if map.remove(&key).is_none() {
            panic!("unregister_font_data: trying to unregister key that is no longer registered");
        }
    }
}
