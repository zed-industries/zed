pub trait IBuffer_Impl: Sized {
    fn Capacity(&self) -> windows_core::Result<u32>;
    fn Length(&self) -> windows_core::Result<u32>;
    fn SetLength(&self, value: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IBuffer {
    const NAME: &'static str = "Windows.Storage.Streams.IBuffer";
}
impl IBuffer_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IBuffer_Vtbl
    where
        Identity: IBuffer_Impl,
    {
        unsafe extern "system" fn Capacity<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut u32) -> windows_core::HRESULT
        where
            Identity: IBuffer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IBuffer_Impl::Capacity(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Length<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut u32) -> windows_core::HRESULT
        where
            Identity: IBuffer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IBuffer_Impl::Length(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLength<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: u32) -> windows_core::HRESULT
        where
            Identity: IBuffer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IBuffer_Impl::SetLength(this, value).into()
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IBuffer, OFFSET>(),
            Capacity: Capacity::<Identity, OFFSET>,
            Length: Length::<Identity, OFFSET>,
            SetLength: SetLength::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBuffer as windows_core::Interface>::IID
    }
}
pub trait IContentTypeProvider_Impl: Sized {
    fn ContentType(&self) -> windows_core::Result<windows_core::HSTRING>;
}
impl windows_core::RuntimeName for IContentTypeProvider {
    const NAME: &'static str = "Windows.Storage.Streams.IContentTypeProvider";
}
impl IContentTypeProvider_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IContentTypeProvider_Vtbl
    where
        Identity: IContentTypeProvider_Impl,
    {
        unsafe extern "system" fn ContentType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT
        where
            Identity: IContentTypeProvider_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IContentTypeProvider_Impl::ContentType(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IContentTypeProvider, OFFSET>(), ContentType: ContentType::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IContentTypeProvider as windows_core::Interface>::IID
    }
}
pub trait IDataReader_Impl: Sized {
    fn UnconsumedBufferLength(&self) -> windows_core::Result<u32>;
    fn UnicodeEncoding(&self) -> windows_core::Result<UnicodeEncoding>;
    fn SetUnicodeEncoding(&self, value: UnicodeEncoding) -> windows_core::Result<()>;
    fn ByteOrder(&self) -> windows_core::Result<ByteOrder>;
    fn SetByteOrder(&self, value: ByteOrder) -> windows_core::Result<()>;
    fn InputStreamOptions(&self) -> windows_core::Result<InputStreamOptions>;
    fn SetInputStreamOptions(&self, value: InputStreamOptions) -> windows_core::Result<()>;
    fn ReadByte(&self) -> windows_core::Result<u8>;
    fn ReadBytes(&self, value: &mut [u8]) -> windows_core::Result<()>;
    fn ReadBuffer(&self, length: u32) -> windows_core::Result<IBuffer>;
    fn ReadBoolean(&self) -> windows_core::Result<bool>;
    fn ReadGuid(&self) -> windows_core::Result<windows_core::GUID>;
    fn ReadInt16(&self) -> windows_core::Result<i16>;
    fn ReadInt32(&self) -> windows_core::Result<i32>;
    fn ReadInt64(&self) -> windows_core::Result<i64>;
    fn ReadUInt16(&self) -> windows_core::Result<u16>;
    fn ReadUInt32(&self) -> windows_core::Result<u32>;
    fn ReadUInt64(&self) -> windows_core::Result<u64>;
    fn ReadSingle(&self) -> windows_core::Result<f32>;
    fn ReadDouble(&self) -> windows_core::Result<f64>;
    fn ReadString(&self, codeunitcount: u32) -> windows_core::Result<windows_core::HSTRING>;
    fn ReadDateTime(&self) -> windows_core::Result<super::super::Foundation::DateTime>;
    fn ReadTimeSpan(&self) -> windows_core::Result<super::super::Foundation::TimeSpan>;
    fn LoadAsync(&self, count: u32) -> windows_core::Result<DataReaderLoadOperation>;
    fn DetachBuffer(&self) -> windows_core::Result<IBuffer>;
    fn DetachStream(&self) -> windows_core::Result<IInputStream>;
}
impl windows_core::RuntimeName for IDataReader {
    const NAME: &'static str = "Windows.Storage.Streams.IDataReader";
}
impl IDataReader_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDataReader_Vtbl
    where
        Identity: IDataReader_Impl,
    {
        unsafe extern "system" fn UnconsumedBufferLength<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut u32) -> windows_core::HRESULT
        where
            Identity: IDataReader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDataReader_Impl::UnconsumedBufferLength(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnicodeEncoding<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut UnicodeEncoding) -> windows_core::HRESULT
        where
            Identity: IDataReader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDataReader_Impl::UnicodeEncoding(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUnicodeEncoding<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: UnicodeEncoding) -> windows_core::HRESULT
        where
            Identity: IDataReader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDataReader_Impl::SetUnicodeEncoding(this, value).into()
        }
        unsafe extern "system" fn ByteOrder<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut ByteOrder) -> windows_core::HRESULT
        where
            Identity: IDataReader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDataReader_Impl::ByteOrder(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetByteOrder<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: ByteOrder) -> windows_core::HRESULT
        where
            Identity: IDataReader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDataReader_Impl::SetByteOrder(this, value).into()
        }
        unsafe extern "system" fn InputStreamOptions<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut InputStreamOptions) -> windows_core::HRESULT
        where
            Identity: IDataReader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDataReader_Impl::InputStreamOptions(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInputStreamOptions<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: InputStreamOptions) -> windows_core::HRESULT
        where
            Identity: IDataReader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDataReader_Impl::SetInputStreamOptions(this, value).into()
        }
        unsafe extern "system" fn ReadByte<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut u8) -> windows_core::HRESULT
        where
            Identity: IDataReader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDataReader_Impl::ReadByte(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadBytes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value_array_size: u32, value: *mut u8) -> windows_core::HRESULT
        where
            Identity: IDataReader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDataReader_Impl::ReadBytes(this, core::slice::from_raw_parts_mut(core::mem::transmute_copy(&value), value_array_size as usize)).into()
        }
        unsafe extern "system" fn ReadBuffer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, length: u32, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDataReader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDataReader_Impl::ReadBuffer(this, length) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadBoolean<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT
        where
            Identity: IDataReader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDataReader_Impl::ReadBoolean(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadGuid<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IDataReader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDataReader_Impl::ReadGuid(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadInt16<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut i16) -> windows_core::HRESULT
        where
            Identity: IDataReader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDataReader_Impl::ReadInt16(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadInt32<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut i32) -> windows_core::HRESULT
        where
            Identity: IDataReader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDataReader_Impl::ReadInt32(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadInt64<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut i64) -> windows_core::HRESULT
        where
            Identity: IDataReader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDataReader_Impl::ReadInt64(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadUInt16<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut u16) -> windows_core::HRESULT
        where
            Identity: IDataReader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDataReader_Impl::ReadUInt16(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadUInt32<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut u32) -> windows_core::HRESULT
        where
            Identity: IDataReader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDataReader_Impl::ReadUInt32(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadUInt64<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut u64) -> windows_core::HRESULT
        where
            Identity: IDataReader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDataReader_Impl::ReadUInt64(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadSingle<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut f32) -> windows_core::HRESULT
        where
            Identity: IDataReader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDataReader_Impl::ReadSingle(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadDouble<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut f64) -> windows_core::HRESULT
        where
            Identity: IDataReader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDataReader_Impl::ReadDouble(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadString<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, codeunitcount: u32, result__: *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT
        where
            Identity: IDataReader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDataReader_Impl::ReadString(this, codeunitcount) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadDateTime<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> windows_core::HRESULT
        where
            Identity: IDataReader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDataReader_Impl::ReadDateTime(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadTimeSpan<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> windows_core::HRESULT
        where
            Identity: IDataReader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDataReader_Impl::ReadTimeSpan(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoadAsync<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: u32, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDataReader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDataReader_Impl::LoadAsync(this, count) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DetachBuffer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDataReader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDataReader_Impl::DetachBuffer(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DetachStream<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDataReader_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDataReader_Impl::DetachStream(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IDataReader, OFFSET>(),
            UnconsumedBufferLength: UnconsumedBufferLength::<Identity, OFFSET>,
            UnicodeEncoding: UnicodeEncoding::<Identity, OFFSET>,
            SetUnicodeEncoding: SetUnicodeEncoding::<Identity, OFFSET>,
            ByteOrder: ByteOrder::<Identity, OFFSET>,
            SetByteOrder: SetByteOrder::<Identity, OFFSET>,
            InputStreamOptions: InputStreamOptions::<Identity, OFFSET>,
            SetInputStreamOptions: SetInputStreamOptions::<Identity, OFFSET>,
            ReadByte: ReadByte::<Identity, OFFSET>,
            ReadBytes: ReadBytes::<Identity, OFFSET>,
            ReadBuffer: ReadBuffer::<Identity, OFFSET>,
            ReadBoolean: ReadBoolean::<Identity, OFFSET>,
            ReadGuid: ReadGuid::<Identity, OFFSET>,
            ReadInt16: ReadInt16::<Identity, OFFSET>,
            ReadInt32: ReadInt32::<Identity, OFFSET>,
            ReadInt64: ReadInt64::<Identity, OFFSET>,
            ReadUInt16: ReadUInt16::<Identity, OFFSET>,
            ReadUInt32: ReadUInt32::<Identity, OFFSET>,
            ReadUInt64: ReadUInt64::<Identity, OFFSET>,
            ReadSingle: ReadSingle::<Identity, OFFSET>,
            ReadDouble: ReadDouble::<Identity, OFFSET>,
            ReadString: ReadString::<Identity, OFFSET>,
            ReadDateTime: ReadDateTime::<Identity, OFFSET>,
            ReadTimeSpan: ReadTimeSpan::<Identity, OFFSET>,
            LoadAsync: LoadAsync::<Identity, OFFSET>,
            DetachBuffer: DetachBuffer::<Identity, OFFSET>,
            DetachStream: DetachStream::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDataReader as windows_core::Interface>::IID
    }
}
pub trait IDataWriter_Impl: Sized {
    fn UnstoredBufferLength(&self) -> windows_core::Result<u32>;
    fn UnicodeEncoding(&self) -> windows_core::Result<UnicodeEncoding>;
    fn SetUnicodeEncoding(&self, value: UnicodeEncoding) -> windows_core::Result<()>;
    fn ByteOrder(&self) -> windows_core::Result<ByteOrder>;
    fn SetByteOrder(&self, value: ByteOrder) -> windows_core::Result<()>;
    fn WriteByte(&self, value: u8) -> windows_core::Result<()>;
    fn WriteBytes(&self, value: &[u8]) -> windows_core::Result<()>;
    fn WriteBuffer(&self, buffer: Option<&IBuffer>) -> windows_core::Result<()>;
    fn WriteBufferRange(&self, buffer: Option<&IBuffer>, start: u32, count: u32) -> windows_core::Result<()>;
    fn WriteBoolean(&self, value: bool) -> windows_core::Result<()>;
    fn WriteGuid(&self, value: &windows_core::GUID) -> windows_core::Result<()>;
    fn WriteInt16(&self, value: i16) -> windows_core::Result<()>;
    fn WriteInt32(&self, value: i32) -> windows_core::Result<()>;
    fn WriteInt64(&self, value: i64) -> windows_core::Result<()>;
    fn WriteUInt16(&self, value: u16) -> windows_core::Result<()>;
    fn WriteUInt32(&self, value: u32) -> windows_core::Result<()>;
    fn WriteUInt64(&self, value: u64) -> windows_core::Result<()>;
    fn WriteSingle(&self, value: f32) -> windows_core::Result<()>;
    fn WriteDouble(&self, value: f64) -> windows_core::Result<()>;
    fn WriteDateTime(&self, value: &super::super::Foundation::DateTime) -> windows_core::Result<()>;
    fn WriteTimeSpan(&self, value: &super::super::Foundation::TimeSpan) -> windows_core::Result<()>;
    fn WriteString(&self, value: &windows_core::HSTRING) -> windows_core::Result<u32>;
    fn MeasureString(&self, value: &windows_core::HSTRING) -> windows_core::Result<u32>;
    fn StoreAsync(&self) -> windows_core::Result<DataWriterStoreOperation>;
    fn FlushAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<bool>>;
    fn DetachBuffer(&self) -> windows_core::Result<IBuffer>;
    fn DetachStream(&self) -> windows_core::Result<IOutputStream>;
}
impl windows_core::RuntimeName for IDataWriter {
    const NAME: &'static str = "Windows.Storage.Streams.IDataWriter";
}
impl IDataWriter_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IDataWriter_Vtbl
    where
        Identity: IDataWriter_Impl,
    {
        unsafe extern "system" fn UnstoredBufferLength<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut u32) -> windows_core::HRESULT
        where
            Identity: IDataWriter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDataWriter_Impl::UnstoredBufferLength(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnicodeEncoding<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut UnicodeEncoding) -> windows_core::HRESULT
        where
            Identity: IDataWriter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDataWriter_Impl::UnicodeEncoding(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUnicodeEncoding<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: UnicodeEncoding) -> windows_core::HRESULT
        where
            Identity: IDataWriter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDataWriter_Impl::SetUnicodeEncoding(this, value).into()
        }
        unsafe extern "system" fn ByteOrder<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut ByteOrder) -> windows_core::HRESULT
        where
            Identity: IDataWriter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDataWriter_Impl::ByteOrder(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetByteOrder<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: ByteOrder) -> windows_core::HRESULT
        where
            Identity: IDataWriter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDataWriter_Impl::SetByteOrder(this, value).into()
        }
        unsafe extern "system" fn WriteByte<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: u8) -> windows_core::HRESULT
        where
            Identity: IDataWriter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDataWriter_Impl::WriteByte(this, value).into()
        }
        unsafe extern "system" fn WriteBytes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value_array_size: u32, value: *const u8) -> windows_core::HRESULT
        where
            Identity: IDataWriter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDataWriter_Impl::WriteBytes(this, core::slice::from_raw_parts(core::mem::transmute_copy(&value), value_array_size as usize)).into()
        }
        unsafe extern "system" fn WriteBuffer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, buffer: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDataWriter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDataWriter_Impl::WriteBuffer(this, windows_core::from_raw_borrowed(&buffer)).into()
        }
        unsafe extern "system" fn WriteBufferRange<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, buffer: *mut core::ffi::c_void, start: u32, count: u32) -> windows_core::HRESULT
        where
            Identity: IDataWriter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDataWriter_Impl::WriteBufferRange(this, windows_core::from_raw_borrowed(&buffer), start, count).into()
        }
        unsafe extern "system" fn WriteBoolean<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: bool) -> windows_core::HRESULT
        where
            Identity: IDataWriter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDataWriter_Impl::WriteBoolean(this, value).into()
        }
        unsafe extern "system" fn WriteGuid<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: windows_core::GUID) -> windows_core::HRESULT
        where
            Identity: IDataWriter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDataWriter_Impl::WriteGuid(this, core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn WriteInt16<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i16) -> windows_core::HRESULT
        where
            Identity: IDataWriter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDataWriter_Impl::WriteInt16(this, value).into()
        }
        unsafe extern "system" fn WriteInt32<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT
        where
            Identity: IDataWriter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDataWriter_Impl::WriteInt32(this, value).into()
        }
        unsafe extern "system" fn WriteInt64<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i64) -> windows_core::HRESULT
        where
            Identity: IDataWriter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDataWriter_Impl::WriteInt64(this, value).into()
        }
        unsafe extern "system" fn WriteUInt16<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: u16) -> windows_core::HRESULT
        where
            Identity: IDataWriter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDataWriter_Impl::WriteUInt16(this, value).into()
        }
        unsafe extern "system" fn WriteUInt32<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: u32) -> windows_core::HRESULT
        where
            Identity: IDataWriter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDataWriter_Impl::WriteUInt32(this, value).into()
        }
        unsafe extern "system" fn WriteUInt64<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: u64) -> windows_core::HRESULT
        where
            Identity: IDataWriter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDataWriter_Impl::WriteUInt64(this, value).into()
        }
        unsafe extern "system" fn WriteSingle<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: f32) -> windows_core::HRESULT
        where
            Identity: IDataWriter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDataWriter_Impl::WriteSingle(this, value).into()
        }
        unsafe extern "system" fn WriteDouble<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: f64) -> windows_core::HRESULT
        where
            Identity: IDataWriter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDataWriter_Impl::WriteDouble(this, value).into()
        }
        unsafe extern "system" fn WriteDateTime<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::super::Foundation::DateTime) -> windows_core::HRESULT
        where
            Identity: IDataWriter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDataWriter_Impl::WriteDateTime(this, core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn WriteTimeSpan<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> windows_core::HRESULT
        where
            Identity: IDataWriter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IDataWriter_Impl::WriteTimeSpan(this, core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn WriteString<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: core::mem::MaybeUninit<windows_core::HSTRING>, result__: *mut u32) -> windows_core::HRESULT
        where
            Identity: IDataWriter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDataWriter_Impl::WriteString(this, core::mem::transmute(&value)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MeasureString<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: core::mem::MaybeUninit<windows_core::HSTRING>, result__: *mut u32) -> windows_core::HRESULT
        where
            Identity: IDataWriter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDataWriter_Impl::MeasureString(this, core::mem::transmute(&value)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StoreAsync<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDataWriter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDataWriter_Impl::StoreAsync(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FlushAsync<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDataWriter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDataWriter_Impl::FlushAsync(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DetachBuffer<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDataWriter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDataWriter_Impl::DetachBuffer(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DetachStream<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IDataWriter_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IDataWriter_Impl::DetachStream(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IDataWriter, OFFSET>(),
            UnstoredBufferLength: UnstoredBufferLength::<Identity, OFFSET>,
            UnicodeEncoding: UnicodeEncoding::<Identity, OFFSET>,
            SetUnicodeEncoding: SetUnicodeEncoding::<Identity, OFFSET>,
            ByteOrder: ByteOrder::<Identity, OFFSET>,
            SetByteOrder: SetByteOrder::<Identity, OFFSET>,
            WriteByte: WriteByte::<Identity, OFFSET>,
            WriteBytes: WriteBytes::<Identity, OFFSET>,
            WriteBuffer: WriteBuffer::<Identity, OFFSET>,
            WriteBufferRange: WriteBufferRange::<Identity, OFFSET>,
            WriteBoolean: WriteBoolean::<Identity, OFFSET>,
            WriteGuid: WriteGuid::<Identity, OFFSET>,
            WriteInt16: WriteInt16::<Identity, OFFSET>,
            WriteInt32: WriteInt32::<Identity, OFFSET>,
            WriteInt64: WriteInt64::<Identity, OFFSET>,
            WriteUInt16: WriteUInt16::<Identity, OFFSET>,
            WriteUInt32: WriteUInt32::<Identity, OFFSET>,
            WriteUInt64: WriteUInt64::<Identity, OFFSET>,
            WriteSingle: WriteSingle::<Identity, OFFSET>,
            WriteDouble: WriteDouble::<Identity, OFFSET>,
            WriteDateTime: WriteDateTime::<Identity, OFFSET>,
            WriteTimeSpan: WriteTimeSpan::<Identity, OFFSET>,
            WriteString: WriteString::<Identity, OFFSET>,
            MeasureString: MeasureString::<Identity, OFFSET>,
            StoreAsync: StoreAsync::<Identity, OFFSET>,
            FlushAsync: FlushAsync::<Identity, OFFSET>,
            DetachBuffer: DetachBuffer::<Identity, OFFSET>,
            DetachStream: DetachStream::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDataWriter as windows_core::Interface>::IID
    }
}
pub trait IInputStream_Impl: Sized + super::super::Foundation::IClosable_Impl {
    fn ReadAsync(&self, buffer: Option<&IBuffer>, count: u32, options: InputStreamOptions) -> windows_core::Result<super::super::Foundation::IAsyncOperationWithProgress<IBuffer, u32>>;
}
impl windows_core::RuntimeName for IInputStream {
    const NAME: &'static str = "Windows.Storage.Streams.IInputStream";
}
impl IInputStream_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IInputStream_Vtbl
    where
        Identity: IInputStream_Impl,
    {
        unsafe extern "system" fn ReadAsync<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, buffer: *mut core::ffi::c_void, count: u32, options: InputStreamOptions, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInputStream_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInputStream_Impl::ReadAsync(this, windows_core::from_raw_borrowed(&buffer), count, options) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IInputStream, OFFSET>(), ReadAsync: ReadAsync::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInputStream as windows_core::Interface>::IID
    }
}
pub trait IInputStreamReference_Impl: Sized {
    fn OpenSequentialReadAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<IInputStream>>;
}
impl windows_core::RuntimeName for IInputStreamReference {
    const NAME: &'static str = "Windows.Storage.Streams.IInputStreamReference";
}
impl IInputStreamReference_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IInputStreamReference_Vtbl
    where
        Identity: IInputStreamReference_Impl,
    {
        unsafe extern "system" fn OpenSequentialReadAsync<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IInputStreamReference_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IInputStreamReference_Impl::OpenSequentialReadAsync(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IInputStreamReference, OFFSET>(),
            OpenSequentialReadAsync: OpenSequentialReadAsync::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInputStreamReference as windows_core::Interface>::IID
    }
}
pub trait IOutputStream_Impl: Sized + super::super::Foundation::IClosable_Impl {
    fn WriteAsync(&self, buffer: Option<&IBuffer>) -> windows_core::Result<super::super::Foundation::IAsyncOperationWithProgress<u32, u32>>;
    fn FlushAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<bool>>;
}
impl windows_core::RuntimeName for IOutputStream {
    const NAME: &'static str = "Windows.Storage.Streams.IOutputStream";
}
impl IOutputStream_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IOutputStream_Vtbl
    where
        Identity: IOutputStream_Impl,
    {
        unsafe extern "system" fn WriteAsync<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, buffer: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IOutputStream_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOutputStream_Impl::WriteAsync(this, windows_core::from_raw_borrowed(&buffer)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FlushAsync<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IOutputStream_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IOutputStream_Impl::FlushAsync(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IOutputStream, OFFSET>(),
            WriteAsync: WriteAsync::<Identity, OFFSET>,
            FlushAsync: FlushAsync::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IOutputStream as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Foundation_Collections")]
pub trait IPropertySetSerializer_Impl: Sized {
    fn Serialize(&self, propertyset: Option<&super::super::Foundation::Collections::IPropertySet>) -> windows_core::Result<IBuffer>;
    fn Deserialize(&self, propertyset: Option<&super::super::Foundation::Collections::IPropertySet>, buffer: Option<&IBuffer>) -> windows_core::Result<()>;
}
#[cfg(feature = "Foundation_Collections")]
impl windows_core::RuntimeName for IPropertySetSerializer {
    const NAME: &'static str = "Windows.Storage.Streams.IPropertySetSerializer";
}
#[cfg(feature = "Foundation_Collections")]
impl IPropertySetSerializer_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IPropertySetSerializer_Vtbl
    where
        Identity: IPropertySetSerializer_Impl,
    {
        unsafe extern "system" fn Serialize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertyset: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPropertySetSerializer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IPropertySetSerializer_Impl::Serialize(this, windows_core::from_raw_borrowed(&propertyset)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Deserialize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertyset: *mut core::ffi::c_void, buffer: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IPropertySetSerializer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IPropertySetSerializer_Impl::Deserialize(this, windows_core::from_raw_borrowed(&propertyset), windows_core::from_raw_borrowed(&buffer)).into()
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IPropertySetSerializer, OFFSET>(),
            Serialize: Serialize::<Identity, OFFSET>,
            Deserialize: Deserialize::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IPropertySetSerializer as windows_core::Interface>::IID
    }
}
pub trait IRandomAccessStream_Impl: Sized + super::super::Foundation::IClosable_Impl + IInputStream_Impl + IOutputStream_Impl {
    fn Size(&self) -> windows_core::Result<u64>;
    fn SetSize(&self, value: u64) -> windows_core::Result<()>;
    fn GetInputStreamAt(&self, position: u64) -> windows_core::Result<IInputStream>;
    fn GetOutputStreamAt(&self, position: u64) -> windows_core::Result<IOutputStream>;
    fn Position(&self) -> windows_core::Result<u64>;
    fn Seek(&self, position: u64) -> windows_core::Result<()>;
    fn CloneStream(&self) -> windows_core::Result<IRandomAccessStream>;
    fn CanRead(&self) -> windows_core::Result<bool>;
    fn CanWrite(&self) -> windows_core::Result<bool>;
}
impl windows_core::RuntimeName for IRandomAccessStream {
    const NAME: &'static str = "Windows.Storage.Streams.IRandomAccessStream";
}
impl IRandomAccessStream_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRandomAccessStream_Vtbl
    where
        Identity: IRandomAccessStream_Impl,
    {
        unsafe extern "system" fn Size<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut u64) -> windows_core::HRESULT
        where
            Identity: IRandomAccessStream_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRandomAccessStream_Impl::Size(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: u64) -> windows_core::HRESULT
        where
            Identity: IRandomAccessStream_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRandomAccessStream_Impl::SetSize(this, value).into()
        }
        unsafe extern "system" fn GetInputStreamAt<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, position: u64, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRandomAccessStream_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRandomAccessStream_Impl::GetInputStreamAt(this, position) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOutputStreamAt<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, position: u64, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRandomAccessStream_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRandomAccessStream_Impl::GetOutputStreamAt(this, position) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Position<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut u64) -> windows_core::HRESULT
        where
            Identity: IRandomAccessStream_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRandomAccessStream_Impl::Position(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Seek<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, position: u64) -> windows_core::HRESULT
        where
            Identity: IRandomAccessStream_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IRandomAccessStream_Impl::Seek(this, position).into()
        }
        unsafe extern "system" fn CloneStream<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRandomAccessStream_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRandomAccessStream_Impl::CloneStream(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanRead<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT
        where
            Identity: IRandomAccessStream_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRandomAccessStream_Impl::CanRead(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanWrite<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT
        where
            Identity: IRandomAccessStream_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRandomAccessStream_Impl::CanWrite(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IRandomAccessStream, OFFSET>(),
            Size: Size::<Identity, OFFSET>,
            SetSize: SetSize::<Identity, OFFSET>,
            GetInputStreamAt: GetInputStreamAt::<Identity, OFFSET>,
            GetOutputStreamAt: GetOutputStreamAt::<Identity, OFFSET>,
            Position: Position::<Identity, OFFSET>,
            Seek: Seek::<Identity, OFFSET>,
            CloneStream: CloneStream::<Identity, OFFSET>,
            CanRead: CanRead::<Identity, OFFSET>,
            CanWrite: CanWrite::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRandomAccessStream as windows_core::Interface>::IID
    }
}
pub trait IRandomAccessStreamReference_Impl: Sized {
    fn OpenReadAsync(&self) -> windows_core::Result<super::super::Foundation::IAsyncOperation<IRandomAccessStreamWithContentType>>;
}
impl windows_core::RuntimeName for IRandomAccessStreamReference {
    const NAME: &'static str = "Windows.Storage.Streams.IRandomAccessStreamReference";
}
impl IRandomAccessStreamReference_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRandomAccessStreamReference_Vtbl
    where
        Identity: IRandomAccessStreamReference_Impl,
    {
        unsafe extern "system" fn OpenReadAsync<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IRandomAccessStreamReference_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IRandomAccessStreamReference_Impl::OpenReadAsync(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IRandomAccessStreamReference, OFFSET>(),
            OpenReadAsync: OpenReadAsync::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRandomAccessStreamReference as windows_core::Interface>::IID
    }
}
pub trait IRandomAccessStreamWithContentType_Impl: Sized + super::super::Foundation::IClosable_Impl + IContentTypeProvider_Impl + IInputStream_Impl + IOutputStream_Impl + IRandomAccessStream_Impl {}
impl windows_core::RuntimeName for IRandomAccessStreamWithContentType {
    const NAME: &'static str = "Windows.Storage.Streams.IRandomAccessStreamWithContentType";
}
impl IRandomAccessStreamWithContentType_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IRandomAccessStreamWithContentType_Vtbl
    where
        Identity: IRandomAccessStreamWithContentType_Impl,
    {
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IRandomAccessStreamWithContentType, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRandomAccessStreamWithContentType as windows_core::Interface>::IID
    }
}
