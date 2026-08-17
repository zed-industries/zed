pub trait IBuffer_Impl: Sized {
    fn Capacity(&self) -> windows_core::Result<u32>;
    fn Length(&self) -> windows_core::Result<u32>;
    fn SetLength(&self, value: u32) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IBuffer {
    const NAME: &'static str = "Windows.Storage.Streams.IBuffer";
}
impl IBuffer_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBuffer_Impl, const OFFSET: isize>() -> IBuffer_Vtbl {
        unsafe extern "system" fn Capacity<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IBuffer_Impl::Capacity(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Length<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IBuffer_Impl::Length(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLength<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IBuffer_Impl::SetLength(this, value).into()
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IBuffer, OFFSET>(),
            Capacity: Capacity::<Identity, Impl, OFFSET>,
            Length: Length::<Identity, Impl, OFFSET>,
            SetLength: SetLength::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IContentTypeProvider_Impl, const OFFSET: isize>() -> IContentTypeProvider_Vtbl {
        unsafe extern "system" fn ContentType<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IContentTypeProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IContentTypeProvider_Impl::ContentType(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IContentTypeProvider, OFFSET>(), ContentType: ContentType::<Identity, Impl, OFFSET> }
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDataReader_Impl, const OFFSET: isize>() -> IDataReader_Vtbl {
        unsafe extern "system" fn UnconsumedBufferLength<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDataReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDataReader_Impl::UnconsumedBufferLength(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnicodeEncoding<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDataReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut UnicodeEncoding) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDataReader_Impl::UnicodeEncoding(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUnicodeEncoding<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDataReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: UnicodeEncoding) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDataReader_Impl::SetUnicodeEncoding(this, value).into()
        }
        unsafe extern "system" fn ByteOrder<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDataReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut ByteOrder) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDataReader_Impl::ByteOrder(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetByteOrder<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDataReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: ByteOrder) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDataReader_Impl::SetByteOrder(this, value).into()
        }
        unsafe extern "system" fn InputStreamOptions<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDataReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut InputStreamOptions) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDataReader_Impl::InputStreamOptions(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInputStreamOptions<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDataReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: InputStreamOptions) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDataReader_Impl::SetInputStreamOptions(this, value).into()
        }
        unsafe extern "system" fn ReadByte<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDataReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut u8) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDataReader_Impl::ReadByte(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadBytes<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDataReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value_array_size: u32, value: *mut u8) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDataReader_Impl::ReadBytes(this, core::slice::from_raw_parts_mut(core::mem::transmute_copy(&value), value_array_size as usize)).into()
        }
        unsafe extern "system" fn ReadBuffer<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDataReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, length: u32, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDataReader_Impl::ReadBuffer(this, length) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadBoolean<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDataReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDataReader_Impl::ReadBoolean(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadGuid<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDataReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut windows_core::GUID) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDataReader_Impl::ReadGuid(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadInt16<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDataReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut i16) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDataReader_Impl::ReadInt16(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadInt32<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDataReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDataReader_Impl::ReadInt32(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadInt64<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDataReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut i64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDataReader_Impl::ReadInt64(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadUInt16<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDataReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut u16) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDataReader_Impl::ReadUInt16(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadUInt32<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDataReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDataReader_Impl::ReadUInt32(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadUInt64<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDataReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut u64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDataReader_Impl::ReadUInt64(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadSingle<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDataReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut f32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDataReader_Impl::ReadSingle(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadDouble<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDataReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut f64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDataReader_Impl::ReadDouble(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadString<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDataReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, codeunitcount: u32, result__: *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDataReader_Impl::ReadString(this, codeunitcount) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadDateTime<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDataReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDataReader_Impl::ReadDateTime(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadTimeSpan<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDataReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDataReader_Impl::ReadTimeSpan(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoadAsync<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDataReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: u32, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDataReader_Impl::LoadAsync(this, count) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DetachBuffer<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDataReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDataReader_Impl::DetachBuffer(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DetachStream<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDataReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDataReader_Impl::DetachStream(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IDataReader, OFFSET>(),
            UnconsumedBufferLength: UnconsumedBufferLength::<Identity, Impl, OFFSET>,
            UnicodeEncoding: UnicodeEncoding::<Identity, Impl, OFFSET>,
            SetUnicodeEncoding: SetUnicodeEncoding::<Identity, Impl, OFFSET>,
            ByteOrder: ByteOrder::<Identity, Impl, OFFSET>,
            SetByteOrder: SetByteOrder::<Identity, Impl, OFFSET>,
            InputStreamOptions: InputStreamOptions::<Identity, Impl, OFFSET>,
            SetInputStreamOptions: SetInputStreamOptions::<Identity, Impl, OFFSET>,
            ReadByte: ReadByte::<Identity, Impl, OFFSET>,
            ReadBytes: ReadBytes::<Identity, Impl, OFFSET>,
            ReadBuffer: ReadBuffer::<Identity, Impl, OFFSET>,
            ReadBoolean: ReadBoolean::<Identity, Impl, OFFSET>,
            ReadGuid: ReadGuid::<Identity, Impl, OFFSET>,
            ReadInt16: ReadInt16::<Identity, Impl, OFFSET>,
            ReadInt32: ReadInt32::<Identity, Impl, OFFSET>,
            ReadInt64: ReadInt64::<Identity, Impl, OFFSET>,
            ReadUInt16: ReadUInt16::<Identity, Impl, OFFSET>,
            ReadUInt32: ReadUInt32::<Identity, Impl, OFFSET>,
            ReadUInt64: ReadUInt64::<Identity, Impl, OFFSET>,
            ReadSingle: ReadSingle::<Identity, Impl, OFFSET>,
            ReadDouble: ReadDouble::<Identity, Impl, OFFSET>,
            ReadString: ReadString::<Identity, Impl, OFFSET>,
            ReadDateTime: ReadDateTime::<Identity, Impl, OFFSET>,
            ReadTimeSpan: ReadTimeSpan::<Identity, Impl, OFFSET>,
            LoadAsync: LoadAsync::<Identity, Impl, OFFSET>,
            DetachBuffer: DetachBuffer::<Identity, Impl, OFFSET>,
            DetachStream: DetachStream::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDataWriter_Impl, const OFFSET: isize>() -> IDataWriter_Vtbl {
        unsafe extern "system" fn UnstoredBufferLength<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDataWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDataWriter_Impl::UnstoredBufferLength(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnicodeEncoding<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDataWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut UnicodeEncoding) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDataWriter_Impl::UnicodeEncoding(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUnicodeEncoding<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDataWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: UnicodeEncoding) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDataWriter_Impl::SetUnicodeEncoding(this, value).into()
        }
        unsafe extern "system" fn ByteOrder<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDataWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut ByteOrder) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDataWriter_Impl::ByteOrder(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetByteOrder<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDataWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: ByteOrder) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDataWriter_Impl::SetByteOrder(this, value).into()
        }
        unsafe extern "system" fn WriteByte<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDataWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: u8) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDataWriter_Impl::WriteByte(this, value).into()
        }
        unsafe extern "system" fn WriteBytes<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDataWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value_array_size: u32, value: *const u8) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDataWriter_Impl::WriteBytes(this, core::slice::from_raw_parts(core::mem::transmute_copy(&value), value_array_size as usize)).into()
        }
        unsafe extern "system" fn WriteBuffer<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDataWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, buffer: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDataWriter_Impl::WriteBuffer(this, windows_core::from_raw_borrowed(&buffer)).into()
        }
        unsafe extern "system" fn WriteBufferRange<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDataWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, buffer: *mut core::ffi::c_void, start: u32, count: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDataWriter_Impl::WriteBufferRange(this, windows_core::from_raw_borrowed(&buffer), start, count).into()
        }
        unsafe extern "system" fn WriteBoolean<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDataWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: bool) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDataWriter_Impl::WriteBoolean(this, value).into()
        }
        unsafe extern "system" fn WriteGuid<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDataWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: windows_core::GUID) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDataWriter_Impl::WriteGuid(this, core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn WriteInt16<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDataWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i16) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDataWriter_Impl::WriteInt16(this, value).into()
        }
        unsafe extern "system" fn WriteInt32<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDataWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDataWriter_Impl::WriteInt32(this, value).into()
        }
        unsafe extern "system" fn WriteInt64<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDataWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDataWriter_Impl::WriteInt64(this, value).into()
        }
        unsafe extern "system" fn WriteUInt16<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDataWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: u16) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDataWriter_Impl::WriteUInt16(this, value).into()
        }
        unsafe extern "system" fn WriteUInt32<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDataWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDataWriter_Impl::WriteUInt32(this, value).into()
        }
        unsafe extern "system" fn WriteUInt64<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDataWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: u64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDataWriter_Impl::WriteUInt64(this, value).into()
        }
        unsafe extern "system" fn WriteSingle<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDataWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: f32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDataWriter_Impl::WriteSingle(this, value).into()
        }
        unsafe extern "system" fn WriteDouble<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDataWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: f64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDataWriter_Impl::WriteDouble(this, value).into()
        }
        unsafe extern "system" fn WriteDateTime<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDataWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::super::Foundation::DateTime) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDataWriter_Impl::WriteDateTime(this, core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn WriteTimeSpan<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDataWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IDataWriter_Impl::WriteTimeSpan(this, core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn WriteString<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDataWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: core::mem::MaybeUninit<windows_core::HSTRING>, result__: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDataWriter_Impl::WriteString(this, core::mem::transmute(&value)) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MeasureString<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDataWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: core::mem::MaybeUninit<windows_core::HSTRING>, result__: *mut u32) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDataWriter_Impl::MeasureString(this, core::mem::transmute(&value)) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StoreAsync<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDataWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDataWriter_Impl::StoreAsync(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FlushAsync<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDataWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDataWriter_Impl::FlushAsync(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DetachBuffer<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDataWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDataWriter_Impl::DetachBuffer(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DetachStream<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IDataWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IDataWriter_Impl::DetachStream(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IDataWriter, OFFSET>(),
            UnstoredBufferLength: UnstoredBufferLength::<Identity, Impl, OFFSET>,
            UnicodeEncoding: UnicodeEncoding::<Identity, Impl, OFFSET>,
            SetUnicodeEncoding: SetUnicodeEncoding::<Identity, Impl, OFFSET>,
            ByteOrder: ByteOrder::<Identity, Impl, OFFSET>,
            SetByteOrder: SetByteOrder::<Identity, Impl, OFFSET>,
            WriteByte: WriteByte::<Identity, Impl, OFFSET>,
            WriteBytes: WriteBytes::<Identity, Impl, OFFSET>,
            WriteBuffer: WriteBuffer::<Identity, Impl, OFFSET>,
            WriteBufferRange: WriteBufferRange::<Identity, Impl, OFFSET>,
            WriteBoolean: WriteBoolean::<Identity, Impl, OFFSET>,
            WriteGuid: WriteGuid::<Identity, Impl, OFFSET>,
            WriteInt16: WriteInt16::<Identity, Impl, OFFSET>,
            WriteInt32: WriteInt32::<Identity, Impl, OFFSET>,
            WriteInt64: WriteInt64::<Identity, Impl, OFFSET>,
            WriteUInt16: WriteUInt16::<Identity, Impl, OFFSET>,
            WriteUInt32: WriteUInt32::<Identity, Impl, OFFSET>,
            WriteUInt64: WriteUInt64::<Identity, Impl, OFFSET>,
            WriteSingle: WriteSingle::<Identity, Impl, OFFSET>,
            WriteDouble: WriteDouble::<Identity, Impl, OFFSET>,
            WriteDateTime: WriteDateTime::<Identity, Impl, OFFSET>,
            WriteTimeSpan: WriteTimeSpan::<Identity, Impl, OFFSET>,
            WriteString: WriteString::<Identity, Impl, OFFSET>,
            MeasureString: MeasureString::<Identity, Impl, OFFSET>,
            StoreAsync: StoreAsync::<Identity, Impl, OFFSET>,
            FlushAsync: FlushAsync::<Identity, Impl, OFFSET>,
            DetachBuffer: DetachBuffer::<Identity, Impl, OFFSET>,
            DetachStream: DetachStream::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IInputStream_Impl, const OFFSET: isize>() -> IInputStream_Vtbl {
        unsafe extern "system" fn ReadAsync<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IInputStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, buffer: *mut core::ffi::c_void, count: u32, options: InputStreamOptions, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IInputStream_Impl::ReadAsync(this, windows_core::from_raw_borrowed(&buffer), count, options) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IInputStream, OFFSET>(), ReadAsync: ReadAsync::<Identity, Impl, OFFSET> }
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IInputStreamReference_Impl, const OFFSET: isize>() -> IInputStreamReference_Vtbl {
        unsafe extern "system" fn OpenSequentialReadAsync<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IInputStreamReference_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IInputStreamReference_Impl::OpenSequentialReadAsync(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IInputStreamReference, OFFSET>(),
            OpenSequentialReadAsync: OpenSequentialReadAsync::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOutputStream_Impl, const OFFSET: isize>() -> IOutputStream_Vtbl {
        unsafe extern "system" fn WriteAsync<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOutputStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, buffer: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IOutputStream_Impl::WriteAsync(this, windows_core::from_raw_borrowed(&buffer)) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FlushAsync<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IOutputStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IOutputStream_Impl::FlushAsync(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IOutputStream, OFFSET>(),
            WriteAsync: WriteAsync::<Identity, Impl, OFFSET>,
            FlushAsync: FlushAsync::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertySetSerializer_Impl, const OFFSET: isize>() -> IPropertySetSerializer_Vtbl {
        unsafe extern "system" fn Serialize<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertySetSerializer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertyset: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IPropertySetSerializer_Impl::Serialize(this, windows_core::from_raw_borrowed(&propertyset)) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Deserialize<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IPropertySetSerializer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertyset: *mut core::ffi::c_void, buffer: *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IPropertySetSerializer_Impl::Deserialize(this, windows_core::from_raw_borrowed(&propertyset), windows_core::from_raw_borrowed(&buffer)).into()
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IPropertySetSerializer, OFFSET>(),
            Serialize: Serialize::<Identity, Impl, OFFSET>,
            Deserialize: Deserialize::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IRandomAccessStream_Impl, const OFFSET: isize>() -> IRandomAccessStream_Vtbl {
        unsafe extern "system" fn Size<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IRandomAccessStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut u64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IRandomAccessStream_Impl::Size(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSize<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IRandomAccessStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: u64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IRandomAccessStream_Impl::SetSize(this, value).into()
        }
        unsafe extern "system" fn GetInputStreamAt<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IRandomAccessStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, position: u64, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IRandomAccessStream_Impl::GetInputStreamAt(this, position) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOutputStreamAt<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IRandomAccessStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, position: u64, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IRandomAccessStream_Impl::GetOutputStreamAt(this, position) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Position<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IRandomAccessStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut u64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IRandomAccessStream_Impl::Position(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Seek<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IRandomAccessStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, position: u64) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            IRandomAccessStream_Impl::Seek(this, position).into()
        }
        unsafe extern "system" fn CloneStream<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IRandomAccessStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IRandomAccessStream_Impl::CloneStream(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanRead<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IRandomAccessStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IRandomAccessStream_Impl::CanRead(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanWrite<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IRandomAccessStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IRandomAccessStream_Impl::CanWrite(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IRandomAccessStream, OFFSET>(),
            Size: Size::<Identity, Impl, OFFSET>,
            SetSize: SetSize::<Identity, Impl, OFFSET>,
            GetInputStreamAt: GetInputStreamAt::<Identity, Impl, OFFSET>,
            GetOutputStreamAt: GetOutputStreamAt::<Identity, Impl, OFFSET>,
            Position: Position::<Identity, Impl, OFFSET>,
            Seek: Seek::<Identity, Impl, OFFSET>,
            CloneStream: CloneStream::<Identity, Impl, OFFSET>,
            CanRead: CanRead::<Identity, Impl, OFFSET>,
            CanWrite: CanWrite::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IRandomAccessStreamReference_Impl, const OFFSET: isize>() -> IRandomAccessStreamReference_Vtbl {
        unsafe extern "system" fn OpenReadAsync<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IRandomAccessStreamReference_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match IRandomAccessStreamReference_Impl::OpenReadAsync(this) {
                Ok(ok__) => {
                    core::ptr::write(result__, core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IRandomAccessStreamReference, OFFSET>(),
            OpenReadAsync: OpenReadAsync::<Identity, Impl, OFFSET>,
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
    pub const fn new<Identity: windows_core::IUnknownImpl<Impl = Impl>, Impl: IRandomAccessStreamWithContentType_Impl, const OFFSET: isize>() -> IRandomAccessStreamWithContentType_Vtbl {
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IRandomAccessStreamWithContentType, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRandomAccessStreamWithContentType as windows_core::Interface>::IID
    }
}
