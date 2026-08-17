#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Buffer(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(Buffer, windows_core::IUnknown, windows_core::IInspectable, IBuffer);
impl Buffer {
    pub fn Capacity(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Capacity)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Length(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Length)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetLength(&self, value: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetLength)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Create(capacity: u32) -> windows_core::Result<Buffer> {
        Self::IBufferFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), capacity, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateCopyFromMemoryBuffer<P0>(input: P0) -> windows_core::Result<Buffer>
    where
        P0: windows_core::Param<super::super::Foundation::IMemoryBuffer>,
    {
        Self::IBufferStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateCopyFromMemoryBuffer)(windows_core::Interface::as_raw(this), input.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateMemoryBufferOverIBuffer<P0>(input: P0) -> windows_core::Result<super::super::Foundation::MemoryBuffer>
    where
        P0: windows_core::Param<IBuffer>,
    {
        Self::IBufferStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateMemoryBufferOverIBuffer)(windows_core::Interface::as_raw(this), input.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IBufferFactory<R, F: FnOnce(&IBufferFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Buffer, IBufferFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn IBufferStatics<R, F: FnOnce(&IBufferStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<Buffer, IBufferStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for Buffer {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IBuffer>();
}
unsafe impl windows_core::Interface for Buffer {
    type Vtable = <IBuffer as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IBuffer as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for Buffer {
    const NAME: &'static str = "Windows.Storage.Streams.Buffer";
}
unsafe impl Send for Buffer {}
unsafe impl Sync for Buffer {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ByteOrder(pub i32);
impl ByteOrder {
    pub const LittleEndian: Self = Self(0i32);
    pub const BigEndian: Self = Self(1i32);
}
impl windows_core::TypeKind for ByteOrder {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for ByteOrder {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Storage.Streams.ByteOrder;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DataReader(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(DataReader, windows_core::IUnknown, windows_core::IInspectable, IDataReader);
windows_core::imp::required_hierarchy!(DataReader, super::super::Foundation::IClosable);
impl DataReader {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn UnconsumedBufferLength(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UnconsumedBufferLength)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn UnicodeEncoding(&self) -> windows_core::Result<UnicodeEncoding> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UnicodeEncoding)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetUnicodeEncoding(&self, value: UnicodeEncoding) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetUnicodeEncoding)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ByteOrder(&self) -> windows_core::Result<ByteOrder> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ByteOrder)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetByteOrder(&self, value: ByteOrder) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetByteOrder)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn InputStreamOptions(&self) -> windows_core::Result<InputStreamOptions> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InputStreamOptions)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetInputStreamOptions(&self, value: InputStreamOptions) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetInputStreamOptions)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ReadByte(&self) -> windows_core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReadByte)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ReadBytes(&self, value: &mut [u8]) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).ReadBytes)(windows_core::Interface::as_raw(this), value.len().try_into().unwrap(), value.as_mut_ptr()).ok() }
    }
    pub fn ReadBuffer(&self, length: u32) -> windows_core::Result<IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReadBuffer)(windows_core::Interface::as_raw(this), length, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ReadBoolean(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReadBoolean)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ReadGuid(&self) -> windows_core::Result<windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReadGuid)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ReadInt16(&self) -> windows_core::Result<i16> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReadInt16)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ReadInt32(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReadInt32)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ReadInt64(&self) -> windows_core::Result<i64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReadInt64)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ReadUInt16(&self) -> windows_core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReadUInt16)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ReadUInt32(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReadUInt32)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ReadUInt64(&self) -> windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReadUInt64)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ReadSingle(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReadSingle)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ReadDouble(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReadDouble)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ReadString(&self, codeunitcount: u32) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReadString)(windows_core::Interface::as_raw(this), codeunitcount, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn ReadDateTime(&self) -> windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReadDateTime)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ReadTimeSpan(&self) -> windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReadTimeSpan)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn LoadAsync(&self, count: u32) -> windows_core::Result<DataReaderLoadOperation> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LoadAsync)(windows_core::Interface::as_raw(this), count, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DetachBuffer(&self) -> windows_core::Result<IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DetachBuffer)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DetachStream(&self) -> windows_core::Result<IInputStream> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DetachStream)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateDataReader<P0>(inputstream: P0) -> windows_core::Result<DataReader>
    where
        P0: windows_core::Param<IInputStream>,
    {
        Self::IDataReaderFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateDataReader)(windows_core::Interface::as_raw(this), inputstream.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn FromBuffer<P0>(buffer: P0) -> windows_core::Result<DataReader>
    where
        P0: windows_core::Param<IBuffer>,
    {
        Self::IDataReaderStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FromBuffer)(windows_core::Interface::as_raw(this), buffer.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IDataReaderFactory<R, F: FnOnce(&IDataReaderFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<DataReader, IDataReaderFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn IDataReaderStatics<R, F: FnOnce(&IDataReaderStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<DataReader, IDataReaderStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for DataReader {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IDataReader>();
}
unsafe impl windows_core::Interface for DataReader {
    type Vtable = <IDataReader as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IDataReader as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for DataReader {
    const NAME: &'static str = "Windows.Storage.Streams.DataReader";
}
unsafe impl Send for DataReader {}
unsafe impl Sync for DataReader {}
pub type DataReaderLoadOperation = windows_future::IAsyncOperation<u32>;
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DataWriter(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(DataWriter, windows_core::IUnknown, windows_core::IInspectable, IDataWriter);
windows_core::imp::required_hierarchy!(DataWriter, super::super::Foundation::IClosable);
impl DataWriter {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<DataWriter, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn UnstoredBufferLength(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UnstoredBufferLength)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn UnicodeEncoding(&self) -> windows_core::Result<UnicodeEncoding> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UnicodeEncoding)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetUnicodeEncoding(&self, value: UnicodeEncoding) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetUnicodeEncoding)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ByteOrder(&self) -> windows_core::Result<ByteOrder> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ByteOrder)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetByteOrder(&self, value: ByteOrder) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetByteOrder)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn WriteByte(&self, value: u8) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).WriteByte)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn WriteBytes(&self, value: &[u8]) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).WriteBytes)(windows_core::Interface::as_raw(this), value.len().try_into().unwrap(), value.as_ptr()).ok() }
    }
    pub fn WriteBuffer<P0>(&self, buffer: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IBuffer>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).WriteBuffer)(windows_core::Interface::as_raw(this), buffer.param().abi()).ok() }
    }
    pub fn WriteBufferRange<P0>(&self, buffer: P0, start: u32, count: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IBuffer>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).WriteBufferRange)(windows_core::Interface::as_raw(this), buffer.param().abi(), start, count).ok() }
    }
    pub fn WriteBoolean(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).WriteBoolean)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn WriteGuid(&self, value: windows_core::GUID) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).WriteGuid)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn WriteInt16(&self, value: i16) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).WriteInt16)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn WriteInt32(&self, value: i32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).WriteInt32)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn WriteInt64(&self, value: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).WriteInt64)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn WriteUInt16(&self, value: u16) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).WriteUInt16)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn WriteUInt32(&self, value: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).WriteUInt32)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn WriteUInt64(&self, value: u64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).WriteUInt64)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn WriteSingle(&self, value: f32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).WriteSingle)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn WriteDouble(&self, value: f64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).WriteDouble)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn WriteDateTime(&self, value: super::super::Foundation::DateTime) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).WriteDateTime)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn WriteTimeSpan(&self, value: super::super::Foundation::TimeSpan) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).WriteTimeSpan)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn WriteString(&self, value: &windows_core::HSTRING) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).WriteString)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value), &mut result__).map(|| result__)
        }
    }
    pub fn MeasureString(&self, value: &windows_core::HSTRING) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MeasureString)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value), &mut result__).map(|| result__)
        }
    }
    pub fn StoreAsync(&self) -> windows_core::Result<DataWriterStoreOperation> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StoreAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn FlushAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FlushAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DetachBuffer(&self) -> windows_core::Result<IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DetachBuffer)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DetachStream(&self) -> windows_core::Result<IOutputStream> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DetachStream)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateDataWriter<P0>(outputstream: P0) -> windows_core::Result<DataWriter>
    where
        P0: windows_core::Param<IOutputStream>,
    {
        Self::IDataWriterFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateDataWriter)(windows_core::Interface::as_raw(this), outputstream.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IDataWriterFactory<R, F: FnOnce(&IDataWriterFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<DataWriter, IDataWriterFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for DataWriter {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IDataWriter>();
}
unsafe impl windows_core::Interface for DataWriter {
    type Vtable = <IDataWriter as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IDataWriter as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for DataWriter {
    const NAME: &'static str = "Windows.Storage.Streams.DataWriter";
}
unsafe impl Send for DataWriter {}
unsafe impl Sync for DataWriter {}
pub type DataWriterStoreOperation = windows_future::IAsyncOperation<u32>;
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FileInputStream(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(FileInputStream, windows_core::IUnknown, windows_core::IInspectable, IInputStream);
windows_core::imp::required_hierarchy!(FileInputStream, super::super::Foundation::IClosable);
impl FileInputStream {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn ReadAsync<P0>(&self, buffer: P0, count: u32, options: InputStreamOptions) -> windows_core::Result<windows_future::IAsyncOperationWithProgress<IBuffer, u32>>
    where
        P0: windows_core::Param<IBuffer>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReadAsync)(windows_core::Interface::as_raw(this), buffer.param().abi(), count, options, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for FileInputStream {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IInputStream>();
}
unsafe impl windows_core::Interface for FileInputStream {
    type Vtable = <IInputStream as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IInputStream as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for FileInputStream {
    const NAME: &'static str = "Windows.Storage.Streams.FileInputStream";
}
unsafe impl Send for FileInputStream {}
unsafe impl Sync for FileInputStream {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FileOpenDisposition(pub i32);
impl FileOpenDisposition {
    pub const OpenExisting: Self = Self(0i32);
    pub const OpenAlways: Self = Self(1i32);
    pub const CreateNew: Self = Self(2i32);
    pub const CreateAlways: Self = Self(3i32);
    pub const TruncateExisting: Self = Self(4i32);
}
impl windows_core::TypeKind for FileOpenDisposition {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for FileOpenDisposition {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Storage.Streams.FileOpenDisposition;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FileOutputStream(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(FileOutputStream, windows_core::IUnknown, windows_core::IInspectable, IOutputStream);
windows_core::imp::required_hierarchy!(FileOutputStream, super::super::Foundation::IClosable);
impl FileOutputStream {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn WriteAsync<P0>(&self, buffer: P0) -> windows_core::Result<windows_future::IAsyncOperationWithProgress<u32, u32>>
    where
        P0: windows_core::Param<IBuffer>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).WriteAsync)(windows_core::Interface::as_raw(this), buffer.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn FlushAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FlushAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for FileOutputStream {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IOutputStream>();
}
unsafe impl windows_core::Interface for FileOutputStream {
    type Vtable = <IOutputStream as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IOutputStream as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for FileOutputStream {
    const NAME: &'static str = "Windows.Storage.Streams.FileOutputStream";
}
unsafe impl Send for FileOutputStream {}
unsafe impl Sync for FileOutputStream {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FileRandomAccessStream(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(FileRandomAccessStream, windows_core::IUnknown, windows_core::IInspectable, IRandomAccessStream);
windows_core::imp::required_hierarchy!(FileRandomAccessStream, super::super::Foundation::IClosable, IInputStream, IOutputStream);
impl FileRandomAccessStream {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn OpenAsync(filepath: &windows_core::HSTRING, accessmode: super::FileAccessMode) -> windows_core::Result<windows_future::IAsyncOperation<IRandomAccessStream>> {
        Self::IFileRandomAccessStreamStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OpenAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(filepath), accessmode, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn OpenWithOptionsAsync(filepath: &windows_core::HSTRING, accessmode: super::FileAccessMode, sharingoptions: super::StorageOpenOptions, opendisposition: FileOpenDisposition) -> windows_core::Result<windows_future::IAsyncOperation<IRandomAccessStream>> {
        Self::IFileRandomAccessStreamStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OpenWithOptionsAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(filepath), accessmode, sharingoptions, opendisposition, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn OpenTransactedWriteAsync(filepath: &windows_core::HSTRING) -> windows_core::Result<windows_future::IAsyncOperation<super::StorageStreamTransaction>> {
        Self::IFileRandomAccessStreamStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OpenTransactedWriteAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(filepath), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn OpenTransactedWriteWithOptionsAsync(filepath: &windows_core::HSTRING, openoptions: super::StorageOpenOptions, opendisposition: FileOpenDisposition) -> windows_core::Result<windows_future::IAsyncOperation<super::StorageStreamTransaction>> {
        Self::IFileRandomAccessStreamStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OpenTransactedWriteWithOptionsAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(filepath), openoptions, opendisposition, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "System")]
    pub fn OpenForUserAsync<P0>(user: P0, filepath: &windows_core::HSTRING, accessmode: super::FileAccessMode) -> windows_core::Result<windows_future::IAsyncOperation<IRandomAccessStream>>
    where
        P0: windows_core::Param<super::super::System::User>,
    {
        Self::IFileRandomAccessStreamStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OpenForUserAsync)(windows_core::Interface::as_raw(this), user.param().abi(), core::mem::transmute_copy(filepath), accessmode, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "System")]
    pub fn OpenForUserWithOptionsAsync<P0>(user: P0, filepath: &windows_core::HSTRING, accessmode: super::FileAccessMode, sharingoptions: super::StorageOpenOptions, opendisposition: FileOpenDisposition) -> windows_core::Result<windows_future::IAsyncOperation<IRandomAccessStream>>
    where
        P0: windows_core::Param<super::super::System::User>,
    {
        Self::IFileRandomAccessStreamStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OpenForUserWithOptionsAsync)(windows_core::Interface::as_raw(this), user.param().abi(), core::mem::transmute_copy(filepath), accessmode, sharingoptions, opendisposition, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "System")]
    pub fn OpenTransactedWriteForUserAsync<P0>(user: P0, filepath: &windows_core::HSTRING) -> windows_core::Result<windows_future::IAsyncOperation<super::StorageStreamTransaction>>
    where
        P0: windows_core::Param<super::super::System::User>,
    {
        Self::IFileRandomAccessStreamStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OpenTransactedWriteForUserAsync)(windows_core::Interface::as_raw(this), user.param().abi(), core::mem::transmute_copy(filepath), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "System")]
    pub fn OpenTransactedWriteForUserWithOptionsAsync<P0>(user: P0, filepath: &windows_core::HSTRING, openoptions: super::StorageOpenOptions, opendisposition: FileOpenDisposition) -> windows_core::Result<windows_future::IAsyncOperation<super::StorageStreamTransaction>>
    where
        P0: windows_core::Param<super::super::System::User>,
    {
        Self::IFileRandomAccessStreamStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OpenTransactedWriteForUserWithOptionsAsync)(windows_core::Interface::as_raw(this), user.param().abi(), core::mem::transmute_copy(filepath), openoptions, opendisposition, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn ReadAsync<P0>(&self, buffer: P0, count: u32, options: InputStreamOptions) -> windows_core::Result<windows_future::IAsyncOperationWithProgress<IBuffer, u32>>
    where
        P0: windows_core::Param<IBuffer>,
    {
        let this = &windows_core::Interface::cast::<IInputStream>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReadAsync)(windows_core::Interface::as_raw(this), buffer.param().abi(), count, options, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn WriteAsync<P0>(&self, buffer: P0) -> windows_core::Result<windows_future::IAsyncOperationWithProgress<u32, u32>>
    where
        P0: windows_core::Param<IBuffer>,
    {
        let this = &windows_core::Interface::cast::<IOutputStream>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).WriteAsync)(windows_core::Interface::as_raw(this), buffer.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn FlushAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<bool>> {
        let this = &windows_core::Interface::cast::<IOutputStream>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FlushAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Size(&self) -> windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Size)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetSize(&self, value: u64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetSize)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn GetInputStreamAt(&self, position: u64) -> windows_core::Result<IInputStream> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetInputStreamAt)(windows_core::Interface::as_raw(this), position, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetOutputStreamAt(&self, position: u64) -> windows_core::Result<IOutputStream> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetOutputStreamAt)(windows_core::Interface::as_raw(this), position, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Position(&self) -> windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Position)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Seek(&self, position: u64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Seek)(windows_core::Interface::as_raw(this), position).ok() }
    }
    pub fn CloneStream(&self) -> windows_core::Result<IRandomAccessStream> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CloneStream)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CanRead(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CanRead)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CanWrite(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CanWrite)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    fn IFileRandomAccessStreamStatics<R, F: FnOnce(&IFileRandomAccessStreamStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<FileRandomAccessStream, IFileRandomAccessStreamStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for FileRandomAccessStream {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IRandomAccessStream>();
}
unsafe impl windows_core::Interface for FileRandomAccessStream {
    type Vtable = <IRandomAccessStream as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IRandomAccessStream as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for FileRandomAccessStream {
    const NAME: &'static str = "Windows.Storage.Streams.FileRandomAccessStream";
}
unsafe impl Send for FileRandomAccessStream {}
unsafe impl Sync for FileRandomAccessStream {}
windows_core::imp::define_interface!(IBuffer, IBuffer_Vtbl, 0x905a0fe0_bc53_11df_8c49_001e4fc686da);
impl windows_core::RuntimeType for IBuffer {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(IBuffer, windows_core::IUnknown, windows_core::IInspectable);
impl IBuffer {
    pub fn Capacity(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Capacity)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Length(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Length)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetLength(&self, value: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetLength)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeName for IBuffer {
    const NAME: &'static str = "Windows.Storage.Streams.IBuffer";
}
pub trait IBuffer_Impl: windows_core::IUnknownImpl {
    fn Capacity(&self) -> windows_core::Result<u32>;
    fn Length(&self) -> windows_core::Result<u32>;
    fn SetLength(&self, value: u32) -> windows_core::Result<()>;
}
impl IBuffer_Vtbl {
    pub const fn new<Identity: IBuffer_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Capacity<Identity: IBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBuffer_Impl::Capacity(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Length<Identity: IBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBuffer_Impl::Length(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetLength<Identity: IBuffer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBuffer_Impl::SetLength(this, value).into()
            }
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
#[repr(C)]
#[doc(hidden)]
pub struct IBuffer_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Capacity: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Length: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetLength: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBufferFactory, IBufferFactory_Vtbl, 0x71af914d_c10f_484b_bc50_14bc623b3a27);
impl windows_core::RuntimeType for IBufferFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IBufferFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IBufferStatics, IBufferStatics_Vtbl, 0xe901e65b_d716_475a_a90a_af7229b1e741);
impl windows_core::RuntimeType for IBufferStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IBufferStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateCopyFromMemoryBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateMemoryBufferOverIBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IContentTypeProvider, IContentTypeProvider_Vtbl, 0x97d098a5_3b99_4de9_88a5_e11d2f50c795);
impl windows_core::RuntimeType for IContentTypeProvider {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(IContentTypeProvider, windows_core::IUnknown, windows_core::IInspectable);
impl IContentTypeProvider {
    pub fn ContentType(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContentType)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
impl windows_core::RuntimeName for IContentTypeProvider {
    const NAME: &'static str = "Windows.Storage.Streams.IContentTypeProvider";
}
pub trait IContentTypeProvider_Impl: windows_core::IUnknownImpl {
    fn ContentType(&self) -> windows_core::Result<windows_core::HSTRING>;
}
impl IContentTypeProvider_Vtbl {
    pub const fn new<Identity: IContentTypeProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ContentType<Identity: IContentTypeProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
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
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IContentTypeProvider, OFFSET>(), ContentType: ContentType::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IContentTypeProvider as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IContentTypeProvider_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ContentType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDataReader, IDataReader_Vtbl, 0xe2b50029_b4c1_4314_a4b8_fb813a2f275e);
impl windows_core::RuntimeType for IDataReader {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(IDataReader, windows_core::IUnknown, windows_core::IInspectable);
impl IDataReader {
    pub fn UnconsumedBufferLength(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UnconsumedBufferLength)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn UnicodeEncoding(&self) -> windows_core::Result<UnicodeEncoding> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UnicodeEncoding)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetUnicodeEncoding(&self, value: UnicodeEncoding) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetUnicodeEncoding)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ByteOrder(&self) -> windows_core::Result<ByteOrder> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ByteOrder)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetByteOrder(&self, value: ByteOrder) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetByteOrder)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn InputStreamOptions(&self) -> windows_core::Result<InputStreamOptions> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InputStreamOptions)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetInputStreamOptions(&self, value: InputStreamOptions) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetInputStreamOptions)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ReadByte(&self) -> windows_core::Result<u8> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReadByte)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ReadBytes(&self, value: &mut [u8]) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).ReadBytes)(windows_core::Interface::as_raw(this), value.len().try_into().unwrap(), value.as_mut_ptr()).ok() }
    }
    pub fn ReadBuffer(&self, length: u32) -> windows_core::Result<IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReadBuffer)(windows_core::Interface::as_raw(this), length, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ReadBoolean(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReadBoolean)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ReadGuid(&self) -> windows_core::Result<windows_core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReadGuid)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ReadInt16(&self) -> windows_core::Result<i16> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReadInt16)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ReadInt32(&self) -> windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReadInt32)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ReadInt64(&self) -> windows_core::Result<i64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReadInt64)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ReadUInt16(&self) -> windows_core::Result<u16> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReadUInt16)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ReadUInt32(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReadUInt32)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ReadUInt64(&self) -> windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReadUInt64)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ReadSingle(&self) -> windows_core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReadSingle)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ReadDouble(&self) -> windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReadDouble)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ReadString(&self, codeunitcount: u32) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReadString)(windows_core::Interface::as_raw(this), codeunitcount, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn ReadDateTime(&self) -> windows_core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReadDateTime)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn ReadTimeSpan(&self) -> windows_core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReadTimeSpan)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn LoadAsync(&self, count: u32) -> windows_core::Result<DataReaderLoadOperation> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LoadAsync)(windows_core::Interface::as_raw(this), count, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DetachBuffer(&self) -> windows_core::Result<IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DetachBuffer)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DetachStream(&self) -> windows_core::Result<IInputStream> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DetachStream)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeName for IDataReader {
    const NAME: &'static str = "Windows.Storage.Streams.IDataReader";
}
pub trait IDataReader_Impl: windows_core::IUnknownImpl {
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
    fn ReadString(&self, codeUnitCount: u32) -> windows_core::Result<windows_core::HSTRING>;
    fn ReadDateTime(&self) -> windows_core::Result<super::super::Foundation::DateTime>;
    fn ReadTimeSpan(&self) -> windows_core::Result<super::super::Foundation::TimeSpan>;
    fn LoadAsync(&self, count: u32) -> windows_core::Result<DataReaderLoadOperation>;
    fn DetachBuffer(&self) -> windows_core::Result<IBuffer>;
    fn DetachStream(&self) -> windows_core::Result<IInputStream>;
}
impl IDataReader_Vtbl {
    pub const fn new<Identity: IDataReader_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn UnconsumedBufferLength<Identity: IDataReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataReader_Impl::UnconsumedBufferLength(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn UnicodeEncoding<Identity: IDataReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut UnicodeEncoding) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataReader_Impl::UnicodeEncoding(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetUnicodeEncoding<Identity: IDataReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: UnicodeEncoding) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataReader_Impl::SetUnicodeEncoding(this, value).into()
            }
        }
        unsafe extern "system" fn ByteOrder<Identity: IDataReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut ByteOrder) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataReader_Impl::ByteOrder(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetByteOrder<Identity: IDataReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: ByteOrder) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataReader_Impl::SetByteOrder(this, value).into()
            }
        }
        unsafe extern "system" fn InputStreamOptions<Identity: IDataReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut InputStreamOptions) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataReader_Impl::InputStreamOptions(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetInputStreamOptions<Identity: IDataReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: InputStreamOptions) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataReader_Impl::SetInputStreamOptions(this, value).into()
            }
        }
        unsafe extern "system" fn ReadByte<Identity: IDataReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataReader_Impl::ReadByte(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ReadBytes<Identity: IDataReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value_array_size: u32, value: *mut u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataReader_Impl::ReadBytes(this, core::slice::from_raw_parts_mut(core::mem::transmute_copy(&value), value_array_size as usize)).into()
            }
        }
        unsafe extern "system" fn ReadBuffer<Identity: IDataReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, length: u32, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
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
        }
        unsafe extern "system" fn ReadBoolean<Identity: IDataReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataReader_Impl::ReadBoolean(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ReadGuid<Identity: IDataReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataReader_Impl::ReadGuid(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ReadInt16<Identity: IDataReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut i16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataReader_Impl::ReadInt16(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ReadInt32<Identity: IDataReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataReader_Impl::ReadInt32(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ReadInt64<Identity: IDataReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataReader_Impl::ReadInt64(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ReadUInt16<Identity: IDataReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut u16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataReader_Impl::ReadUInt16(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ReadUInt32<Identity: IDataReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataReader_Impl::ReadUInt32(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ReadUInt64<Identity: IDataReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataReader_Impl::ReadUInt64(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ReadSingle<Identity: IDataReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataReader_Impl::ReadSingle(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ReadDouble<Identity: IDataReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataReader_Impl::ReadDouble(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ReadString<Identity: IDataReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, codeunitcount: u32, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
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
        }
        unsafe extern "system" fn ReadDateTime<Identity: IDataReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut super::super::Foundation::DateTime) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataReader_Impl::ReadDateTime(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ReadTimeSpan<Identity: IDataReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut super::super::Foundation::TimeSpan) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataReader_Impl::ReadTimeSpan(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn LoadAsync<Identity: IDataReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: u32, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
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
        }
        unsafe extern "system" fn DetachBuffer<Identity: IDataReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
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
        }
        unsafe extern "system" fn DetachStream<Identity: IDataReader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
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
#[repr(C)]
#[doc(hidden)]
pub struct IDataReader_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub UnconsumedBufferLength: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub UnicodeEncoding: unsafe extern "system" fn(*mut core::ffi::c_void, *mut UnicodeEncoding) -> windows_core::HRESULT,
    pub SetUnicodeEncoding: unsafe extern "system" fn(*mut core::ffi::c_void, UnicodeEncoding) -> windows_core::HRESULT,
    pub ByteOrder: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ByteOrder) -> windows_core::HRESULT,
    pub SetByteOrder: unsafe extern "system" fn(*mut core::ffi::c_void, ByteOrder) -> windows_core::HRESULT,
    pub InputStreamOptions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut InputStreamOptions) -> windows_core::HRESULT,
    pub SetInputStreamOptions: unsafe extern "system" fn(*mut core::ffi::c_void, InputStreamOptions) -> windows_core::HRESULT,
    pub ReadByte: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8) -> windows_core::HRESULT,
    pub ReadBytes: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u8) -> windows_core::HRESULT,
    pub ReadBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReadBoolean: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub ReadGuid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub ReadInt16: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i16) -> windows_core::HRESULT,
    pub ReadInt32: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub ReadInt64: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub ReadUInt16: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub ReadUInt32: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub ReadUInt64: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub ReadSingle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub ReadDouble: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub ReadString: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReadDateTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::DateTime) -> windows_core::HRESULT,
    pub ReadTimeSpan: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub LoadAsync: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DetachBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DetachStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDataReaderFactory, IDataReaderFactory_Vtbl, 0xd7527847_57da_4e15_914c_06806699a098);
impl windows_core::RuntimeType for IDataReaderFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataReaderFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateDataReader: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDataReaderStatics, IDataReaderStatics_Vtbl, 0x11fcbfc8_f93a_471b_b121_f379e349313c);
impl windows_core::RuntimeType for IDataReaderStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataReaderStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub FromBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDataWriter, IDataWriter_Vtbl, 0x64b89265_d341_4922_b38a_dd4af8808c4e);
impl windows_core::RuntimeType for IDataWriter {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(IDataWriter, windows_core::IUnknown, windows_core::IInspectable);
impl IDataWriter {
    pub fn UnstoredBufferLength(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UnstoredBufferLength)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn UnicodeEncoding(&self) -> windows_core::Result<UnicodeEncoding> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).UnicodeEncoding)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetUnicodeEncoding(&self, value: UnicodeEncoding) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetUnicodeEncoding)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ByteOrder(&self) -> windows_core::Result<ByteOrder> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ByteOrder)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetByteOrder(&self, value: ByteOrder) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetByteOrder)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn WriteByte(&self, value: u8) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).WriteByte)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn WriteBytes(&self, value: &[u8]) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).WriteBytes)(windows_core::Interface::as_raw(this), value.len().try_into().unwrap(), value.as_ptr()).ok() }
    }
    pub fn WriteBuffer<P0>(&self, buffer: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IBuffer>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).WriteBuffer)(windows_core::Interface::as_raw(this), buffer.param().abi()).ok() }
    }
    pub fn WriteBufferRange<P0>(&self, buffer: P0, start: u32, count: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IBuffer>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).WriteBufferRange)(windows_core::Interface::as_raw(this), buffer.param().abi(), start, count).ok() }
    }
    pub fn WriteBoolean(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).WriteBoolean)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn WriteGuid(&self, value: windows_core::GUID) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).WriteGuid)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn WriteInt16(&self, value: i16) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).WriteInt16)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn WriteInt32(&self, value: i32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).WriteInt32)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn WriteInt64(&self, value: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).WriteInt64)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn WriteUInt16(&self, value: u16) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).WriteUInt16)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn WriteUInt32(&self, value: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).WriteUInt32)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn WriteUInt64(&self, value: u64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).WriteUInt64)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn WriteSingle(&self, value: f32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).WriteSingle)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn WriteDouble(&self, value: f64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).WriteDouble)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn WriteDateTime(&self, value: super::super::Foundation::DateTime) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).WriteDateTime)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn WriteTimeSpan(&self, value: super::super::Foundation::TimeSpan) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).WriteTimeSpan)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn WriteString(&self, value: &windows_core::HSTRING) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).WriteString)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value), &mut result__).map(|| result__)
        }
    }
    pub fn MeasureString(&self, value: &windows_core::HSTRING) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MeasureString)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value), &mut result__).map(|| result__)
        }
    }
    pub fn StoreAsync(&self) -> windows_core::Result<DataWriterStoreOperation> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StoreAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn FlushAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FlushAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DetachBuffer(&self) -> windows_core::Result<IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DetachBuffer)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DetachStream(&self) -> windows_core::Result<IOutputStream> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DetachStream)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeName for IDataWriter {
    const NAME: &'static str = "Windows.Storage.Streams.IDataWriter";
}
pub trait IDataWriter_Impl: windows_core::IUnknownImpl {
    fn UnstoredBufferLength(&self) -> windows_core::Result<u32>;
    fn UnicodeEncoding(&self) -> windows_core::Result<UnicodeEncoding>;
    fn SetUnicodeEncoding(&self, value: UnicodeEncoding) -> windows_core::Result<()>;
    fn ByteOrder(&self) -> windows_core::Result<ByteOrder>;
    fn SetByteOrder(&self, value: ByteOrder) -> windows_core::Result<()>;
    fn WriteByte(&self, value: u8) -> windows_core::Result<()>;
    fn WriteBytes(&self, value: &[u8]) -> windows_core::Result<()>;
    fn WriteBuffer(&self, buffer: windows_core::Ref<IBuffer>) -> windows_core::Result<()>;
    fn WriteBufferRange(&self, buffer: windows_core::Ref<IBuffer>, start: u32, count: u32) -> windows_core::Result<()>;
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
    fn FlushAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<bool>>;
    fn DetachBuffer(&self) -> windows_core::Result<IBuffer>;
    fn DetachStream(&self) -> windows_core::Result<IOutputStream>;
}
impl IDataWriter_Vtbl {
    pub const fn new<Identity: IDataWriter_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn UnstoredBufferLength<Identity: IDataWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataWriter_Impl::UnstoredBufferLength(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn UnicodeEncoding<Identity: IDataWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut UnicodeEncoding) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataWriter_Impl::UnicodeEncoding(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetUnicodeEncoding<Identity: IDataWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: UnicodeEncoding) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataWriter_Impl::SetUnicodeEncoding(this, value).into()
            }
        }
        unsafe extern "system" fn ByteOrder<Identity: IDataWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut ByteOrder) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataWriter_Impl::ByteOrder(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetByteOrder<Identity: IDataWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: ByteOrder) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataWriter_Impl::SetByteOrder(this, value).into()
            }
        }
        unsafe extern "system" fn WriteByte<Identity: IDataWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataWriter_Impl::WriteByte(this, value).into()
            }
        }
        unsafe extern "system" fn WriteBytes<Identity: IDataWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value_array_size: u32, value: *const u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataWriter_Impl::WriteBytes(this, core::slice::from_raw_parts(core::mem::transmute_copy(&value), value_array_size as usize)).into()
            }
        }
        unsafe extern "system" fn WriteBuffer<Identity: IDataWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, buffer: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataWriter_Impl::WriteBuffer(this, core::mem::transmute_copy(&buffer)).into()
            }
        }
        unsafe extern "system" fn WriteBufferRange<Identity: IDataWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, buffer: *mut core::ffi::c_void, start: u32, count: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataWriter_Impl::WriteBufferRange(this, core::mem::transmute_copy(&buffer), start, count).into()
            }
        }
        unsafe extern "system" fn WriteBoolean<Identity: IDataWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataWriter_Impl::WriteBoolean(this, value).into()
            }
        }
        unsafe extern "system" fn WriteGuid<Identity: IDataWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataWriter_Impl::WriteGuid(this, core::mem::transmute(&value)).into()
            }
        }
        unsafe extern "system" fn WriteInt16<Identity: IDataWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataWriter_Impl::WriteInt16(this, value).into()
            }
        }
        unsafe extern "system" fn WriteInt32<Identity: IDataWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataWriter_Impl::WriteInt32(this, value).into()
            }
        }
        unsafe extern "system" fn WriteInt64<Identity: IDataWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataWriter_Impl::WriteInt64(this, value).into()
            }
        }
        unsafe extern "system" fn WriteUInt16<Identity: IDataWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: u16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataWriter_Impl::WriteUInt16(this, value).into()
            }
        }
        unsafe extern "system" fn WriteUInt32<Identity: IDataWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataWriter_Impl::WriteUInt32(this, value).into()
            }
        }
        unsafe extern "system" fn WriteUInt64<Identity: IDataWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataWriter_Impl::WriteUInt64(this, value).into()
            }
        }
        unsafe extern "system" fn WriteSingle<Identity: IDataWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataWriter_Impl::WriteSingle(this, value).into()
            }
        }
        unsafe extern "system" fn WriteDouble<Identity: IDataWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataWriter_Impl::WriteDouble(this, value).into()
            }
        }
        unsafe extern "system" fn WriteDateTime<Identity: IDataWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::super::Foundation::DateTime) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataWriter_Impl::WriteDateTime(this, core::mem::transmute(&value)).into()
            }
        }
        unsafe extern "system" fn WriteTimeSpan<Identity: IDataWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::super::Foundation::TimeSpan) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDataWriter_Impl::WriteTimeSpan(this, core::mem::transmute(&value)).into()
            }
        }
        unsafe extern "system" fn WriteString<Identity: IDataWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void, result__: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataWriter_Impl::WriteString(this, core::mem::transmute(&value)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MeasureString<Identity: IDataWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void, result__: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDataWriter_Impl::MeasureString(this, core::mem::transmute(&value)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn StoreAsync<Identity: IDataWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
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
        }
        unsafe extern "system" fn FlushAsync<Identity: IDataWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
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
        }
        unsafe extern "system" fn DetachBuffer<Identity: IDataWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
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
        }
        unsafe extern "system" fn DetachStream<Identity: IDataWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
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
#[repr(C)]
#[doc(hidden)]
pub struct IDataWriter_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub UnstoredBufferLength: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub UnicodeEncoding: unsafe extern "system" fn(*mut core::ffi::c_void, *mut UnicodeEncoding) -> windows_core::HRESULT,
    pub SetUnicodeEncoding: unsafe extern "system" fn(*mut core::ffi::c_void, UnicodeEncoding) -> windows_core::HRESULT,
    pub ByteOrder: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ByteOrder) -> windows_core::HRESULT,
    pub SetByteOrder: unsafe extern "system" fn(*mut core::ffi::c_void, ByteOrder) -> windows_core::HRESULT,
    pub WriteByte: unsafe extern "system" fn(*mut core::ffi::c_void, u8) -> windows_core::HRESULT,
    pub WriteBytes: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u8) -> windows_core::HRESULT,
    pub WriteBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub WriteBufferRange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub WriteBoolean: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub WriteGuid: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID) -> windows_core::HRESULT,
    pub WriteInt16: unsafe extern "system" fn(*mut core::ffi::c_void, i16) -> windows_core::HRESULT,
    pub WriteInt32: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub WriteInt64: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub WriteUInt16: unsafe extern "system" fn(*mut core::ffi::c_void, u16) -> windows_core::HRESULT,
    pub WriteUInt32: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub WriteUInt64: unsafe extern "system" fn(*mut core::ffi::c_void, u64) -> windows_core::HRESULT,
    pub WriteSingle: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub WriteDouble: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub WriteDateTime: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::DateTime) -> windows_core::HRESULT,
    pub WriteTimeSpan: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
    pub WriteString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub MeasureString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub StoreAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FlushAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DetachBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DetachStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDataWriterFactory, IDataWriterFactory_Vtbl, 0x338c67c2_8b84_4c2b_9c50_7b8767847a1f);
impl windows_core::RuntimeType for IDataWriterFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataWriterFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateDataWriter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IFileRandomAccessStreamStatics, IFileRandomAccessStreamStatics_Vtbl, 0x73550107_3b57_4b5d_8345_554d2fc621f0);
impl windows_core::RuntimeType for IFileRandomAccessStreamStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileRandomAccessStreamStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub OpenAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::FileAccessMode, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OpenWithOptionsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::FileAccessMode, super::StorageOpenOptions, FileOpenDisposition, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OpenTransactedWriteAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OpenTransactedWriteWithOptionsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::StorageOpenOptions, FileOpenDisposition, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "System")]
    pub OpenForUserAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, super::FileAccessMode, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    OpenForUserAsync: usize,
    #[cfg(feature = "System")]
    pub OpenForUserWithOptionsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, super::FileAccessMode, super::StorageOpenOptions, FileOpenDisposition, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    OpenForUserWithOptionsAsync: usize,
    #[cfg(feature = "System")]
    pub OpenTransactedWriteForUserAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    OpenTransactedWriteForUserAsync: usize,
    #[cfg(feature = "System")]
    pub OpenTransactedWriteForUserWithOptionsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, super::StorageOpenOptions, FileOpenDisposition, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    OpenTransactedWriteForUserWithOptionsAsync: usize,
}
windows_core::imp::define_interface!(IInputStream, IInputStream_Vtbl, 0x905a0fe2_bc53_11df_8c49_001e4fc686da);
impl windows_core::RuntimeType for IInputStream {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(IInputStream, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(IInputStream, super::super::Foundation::IClosable);
impl IInputStream {
    pub fn ReadAsync<P0>(&self, buffer: P0, count: u32, options: InputStreamOptions) -> windows_core::Result<windows_future::IAsyncOperationWithProgress<IBuffer, u32>>
    where
        P0: windows_core::Param<IBuffer>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReadAsync)(windows_core::Interface::as_raw(this), buffer.param().abi(), count, options, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
}
impl windows_core::RuntimeName for IInputStream {
    const NAME: &'static str = "Windows.Storage.Streams.IInputStream";
}
pub trait IInputStream_Impl: super::super::Foundation::IClosable_Impl {
    fn ReadAsync(&self, buffer: windows_core::Ref<IBuffer>, count: u32, options: InputStreamOptions) -> windows_core::Result<windows_future::IAsyncOperationWithProgress<IBuffer, u32>>;
}
impl IInputStream_Vtbl {
    pub const fn new<Identity: IInputStream_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ReadAsync<Identity: IInputStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, buffer: *mut core::ffi::c_void, count: u32, options: InputStreamOptions, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IInputStream_Impl::ReadAsync(this, core::mem::transmute_copy(&buffer), count, options) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IInputStream, OFFSET>(), ReadAsync: ReadAsync::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInputStream as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputStream_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ReadAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, InputStreamOptions, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IInputStreamReference, IInputStreamReference_Vtbl, 0x43929d18_5ec9_4b5a_919c_4205b0c804b6);
impl windows_core::RuntimeType for IInputStreamReference {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(IInputStreamReference, windows_core::IUnknown, windows_core::IInspectable);
impl IInputStreamReference {
    pub fn OpenSequentialReadAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<IInputStream>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OpenSequentialReadAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeName for IInputStreamReference {
    const NAME: &'static str = "Windows.Storage.Streams.IInputStreamReference";
}
pub trait IInputStreamReference_Impl: windows_core::IUnknownImpl {
    fn OpenSequentialReadAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<IInputStream>>;
}
impl IInputStreamReference_Vtbl {
    pub const fn new<Identity: IInputStreamReference_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OpenSequentialReadAsync<Identity: IInputStreamReference_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
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
#[repr(C)]
#[doc(hidden)]
pub struct IInputStreamReference_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub OpenSequentialReadAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IOutputStream, IOutputStream_Vtbl, 0x905a0fe6_bc53_11df_8c49_001e4fc686da);
impl windows_core::RuntimeType for IOutputStream {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(IOutputStream, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(IOutputStream, super::super::Foundation::IClosable);
impl IOutputStream {
    pub fn WriteAsync<P0>(&self, buffer: P0) -> windows_core::Result<windows_future::IAsyncOperationWithProgress<u32, u32>>
    where
        P0: windows_core::Param<IBuffer>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).WriteAsync)(windows_core::Interface::as_raw(this), buffer.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn FlushAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FlushAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
}
impl windows_core::RuntimeName for IOutputStream {
    const NAME: &'static str = "Windows.Storage.Streams.IOutputStream";
}
pub trait IOutputStream_Impl: super::super::Foundation::IClosable_Impl {
    fn WriteAsync(&self, buffer: windows_core::Ref<IBuffer>) -> windows_core::Result<windows_future::IAsyncOperationWithProgress<u32, u32>>;
    fn FlushAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<bool>>;
}
impl IOutputStream_Vtbl {
    pub const fn new<Identity: IOutputStream_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn WriteAsync<Identity: IOutputStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, buffer: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IOutputStream_Impl::WriteAsync(this, core::mem::transmute_copy(&buffer)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn FlushAsync<Identity: IOutputStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
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
#[repr(C)]
#[doc(hidden)]
pub struct IOutputStream_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub WriteAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FlushAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IPropertySetSerializer, IPropertySetSerializer_Vtbl, 0x6e8ebf1c_ef3d_4376_b20e_5be638aeac77);
impl windows_core::RuntimeType for IPropertySetSerializer {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(IPropertySetSerializer, windows_core::IUnknown, windows_core::IInspectable);
impl IPropertySetSerializer {
    #[cfg(feature = "Foundation_Collections")]
    pub fn Serialize<P0>(&self, propertyset: P0) -> windows_core::Result<IBuffer>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IPropertySet>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Serialize)(windows_core::Interface::as_raw(this), propertyset.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Deserialize<P0, P1>(&self, propertyset: P0, buffer: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::Foundation::Collections::IPropertySet>,
        P1: windows_core::Param<IBuffer>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Deserialize)(windows_core::Interface::as_raw(this), propertyset.param().abi(), buffer.param().abi()).ok() }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl windows_core::RuntimeName for IPropertySetSerializer {
    const NAME: &'static str = "Windows.Storage.Streams.IPropertySetSerializer";
}
#[cfg(feature = "Foundation_Collections")]
pub trait IPropertySetSerializer_Impl: windows_core::IUnknownImpl {
    fn Serialize(&self, propertySet: windows_core::Ref<super::super::Foundation::Collections::IPropertySet>) -> windows_core::Result<IBuffer>;
    fn Deserialize(&self, propertySet: windows_core::Ref<super::super::Foundation::Collections::IPropertySet>, buffer: windows_core::Ref<IBuffer>) -> windows_core::Result<()>;
}
#[cfg(feature = "Foundation_Collections")]
impl IPropertySetSerializer_Vtbl {
    pub const fn new<Identity: IPropertySetSerializer_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Serialize<Identity: IPropertySetSerializer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertyset: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IPropertySetSerializer_Impl::Serialize(this, core::mem::transmute_copy(&propertyset)) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        core::mem::forget(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Deserialize<Identity: IPropertySetSerializer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertyset: *mut core::ffi::c_void, buffer: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IPropertySetSerializer_Impl::Deserialize(this, core::mem::transmute_copy(&propertyset), core::mem::transmute_copy(&buffer)).into()
            }
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
#[repr(C)]
#[doc(hidden)]
pub struct IPropertySetSerializer_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Serialize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Serialize: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Deserialize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Deserialize: usize,
}
windows_core::imp::define_interface!(IRandomAccessStream, IRandomAccessStream_Vtbl, 0x905a0fe1_bc53_11df_8c49_001e4fc686da);
impl windows_core::RuntimeType for IRandomAccessStream {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(IRandomAccessStream, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(IRandomAccessStream, super::super::Foundation::IClosable, IInputStream, IOutputStream);
impl IRandomAccessStream {
    pub fn Size(&self) -> windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Size)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetSize(&self, value: u64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetSize)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn GetInputStreamAt(&self, position: u64) -> windows_core::Result<IInputStream> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetInputStreamAt)(windows_core::Interface::as_raw(this), position, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetOutputStreamAt(&self, position: u64) -> windows_core::Result<IOutputStream> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetOutputStreamAt)(windows_core::Interface::as_raw(this), position, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Position(&self) -> windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Position)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Seek(&self, position: u64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Seek)(windows_core::Interface::as_raw(this), position).ok() }
    }
    pub fn CloneStream(&self) -> windows_core::Result<IRandomAccessStream> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CloneStream)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CanRead(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CanRead)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CanWrite(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CanWrite)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn ReadAsync<P0>(&self, buffer: P0, count: u32, options: InputStreamOptions) -> windows_core::Result<windows_future::IAsyncOperationWithProgress<IBuffer, u32>>
    where
        P0: windows_core::Param<IBuffer>,
    {
        let this = &windows_core::Interface::cast::<IInputStream>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReadAsync)(windows_core::Interface::as_raw(this), buffer.param().abi(), count, options, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn WriteAsync<P0>(&self, buffer: P0) -> windows_core::Result<windows_future::IAsyncOperationWithProgress<u32, u32>>
    where
        P0: windows_core::Param<IBuffer>,
    {
        let this = &windows_core::Interface::cast::<IOutputStream>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).WriteAsync)(windows_core::Interface::as_raw(this), buffer.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn FlushAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<bool>> {
        let this = &windows_core::Interface::cast::<IOutputStream>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FlushAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeName for IRandomAccessStream {
    const NAME: &'static str = "Windows.Storage.Streams.IRandomAccessStream";
}
pub trait IRandomAccessStream_Impl: super::super::Foundation::IClosable_Impl + IInputStream_Impl + IOutputStream_Impl {
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
impl IRandomAccessStream_Vtbl {
    pub const fn new<Identity: IRandomAccessStream_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Size<Identity: IRandomAccessStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRandomAccessStream_Impl::Size(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSize<Identity: IRandomAccessStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRandomAccessStream_Impl::SetSize(this, value).into()
            }
        }
        unsafe extern "system" fn GetInputStreamAt<Identity: IRandomAccessStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, position: u64, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
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
        }
        unsafe extern "system" fn GetOutputStreamAt<Identity: IRandomAccessStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, position: u64, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
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
        }
        unsafe extern "system" fn Position<Identity: IRandomAccessStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRandomAccessStream_Impl::Position(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Seek<Identity: IRandomAccessStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, position: u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRandomAccessStream_Impl::Seek(this, position).into()
            }
        }
        unsafe extern "system" fn CloneStream<Identity: IRandomAccessStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
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
        }
        unsafe extern "system" fn CanRead<Identity: IRandomAccessStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRandomAccessStream_Impl::CanRead(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CanWrite<Identity: IRandomAccessStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRandomAccessStream_Impl::CanWrite(this) {
                    Ok(ok__) => {
                        result__.write(core::mem::transmute_copy(&ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
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
#[repr(C)]
#[doc(hidden)]
pub struct IRandomAccessStream_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Size: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub SetSize: unsafe extern "system" fn(*mut core::ffi::c_void, u64) -> windows_core::HRESULT,
    pub GetInputStreamAt: unsafe extern "system" fn(*mut core::ffi::c_void, u64, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetOutputStreamAt: unsafe extern "system" fn(*mut core::ffi::c_void, u64, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Position: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub Seek: unsafe extern "system" fn(*mut core::ffi::c_void, u64) -> windows_core::HRESULT,
    pub CloneStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CanRead: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub CanWrite: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRandomAccessStreamReference, IRandomAccessStreamReference_Vtbl, 0x33ee3134_1dd6_4e3a_8067_d1c162e8642b);
impl windows_core::RuntimeType for IRandomAccessStreamReference {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(IRandomAccessStreamReference, windows_core::IUnknown, windows_core::IInspectable);
impl IRandomAccessStreamReference {
    pub fn OpenReadAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<IRandomAccessStreamWithContentType>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OpenReadAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeName for IRandomAccessStreamReference {
    const NAME: &'static str = "Windows.Storage.Streams.IRandomAccessStreamReference";
}
pub trait IRandomAccessStreamReference_Impl: windows_core::IUnknownImpl {
    fn OpenReadAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<IRandomAccessStreamWithContentType>>;
}
impl IRandomAccessStreamReference_Vtbl {
    pub const fn new<Identity: IRandomAccessStreamReference_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OpenReadAsync<Identity: IRandomAccessStreamReference_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
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
#[repr(C)]
#[doc(hidden)]
pub struct IRandomAccessStreamReference_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub OpenReadAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRandomAccessStreamReferenceStatics, IRandomAccessStreamReferenceStatics_Vtbl, 0x857309dc_3fbf_4e7d_986f_ef3b1a07a964);
impl windows_core::RuntimeType for IRandomAccessStreamReferenceStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IRandomAccessStreamReferenceStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateFromFile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateFromUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateFromStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRandomAccessStreamStatics, IRandomAccessStreamStatics_Vtbl, 0x524cedcf_6e29_4ce5_9573_6b753db66c3a);
impl windows_core::RuntimeType for IRandomAccessStreamStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IRandomAccessStreamStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CopyAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CopySizeAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, u64, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CopyAndCloseAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IRandomAccessStreamWithContentType, IRandomAccessStreamWithContentType_Vtbl, 0xcc254827_4b3d_438f_9232_10c76bc7e038);
impl windows_core::RuntimeType for IRandomAccessStreamWithContentType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
windows_core::imp::interface_hierarchy!(IRandomAccessStreamWithContentType, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(IRandomAccessStreamWithContentType, super::super::Foundation::IClosable, IContentTypeProvider, IInputStream, IOutputStream, IRandomAccessStream);
impl IRandomAccessStreamWithContentType {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn ContentType(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IContentTypeProvider>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ContentType)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn ReadAsync<P0>(&self, buffer: P0, count: u32, options: InputStreamOptions) -> windows_core::Result<windows_future::IAsyncOperationWithProgress<IBuffer, u32>>
    where
        P0: windows_core::Param<IBuffer>,
    {
        let this = &windows_core::Interface::cast::<IInputStream>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReadAsync)(windows_core::Interface::as_raw(this), buffer.param().abi(), count, options, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn WriteAsync<P0>(&self, buffer: P0) -> windows_core::Result<windows_future::IAsyncOperationWithProgress<u32, u32>>
    where
        P0: windows_core::Param<IBuffer>,
    {
        let this = &windows_core::Interface::cast::<IOutputStream>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).WriteAsync)(windows_core::Interface::as_raw(this), buffer.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn FlushAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<bool>> {
        let this = &windows_core::Interface::cast::<IOutputStream>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FlushAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Size(&self) -> windows_core::Result<u64> {
        let this = &windows_core::Interface::cast::<IRandomAccessStream>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Size)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetSize(&self, value: u64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IRandomAccessStream>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetSize)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn GetInputStreamAt(&self, position: u64) -> windows_core::Result<IInputStream> {
        let this = &windows_core::Interface::cast::<IRandomAccessStream>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetInputStreamAt)(windows_core::Interface::as_raw(this), position, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetOutputStreamAt(&self, position: u64) -> windows_core::Result<IOutputStream> {
        let this = &windows_core::Interface::cast::<IRandomAccessStream>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetOutputStreamAt)(windows_core::Interface::as_raw(this), position, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Position(&self) -> windows_core::Result<u64> {
        let this = &windows_core::Interface::cast::<IRandomAccessStream>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Position)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Seek(&self, position: u64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IRandomAccessStream>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Seek)(windows_core::Interface::as_raw(this), position).ok() }
    }
    pub fn CloneStream(&self) -> windows_core::Result<IRandomAccessStream> {
        let this = &windows_core::Interface::cast::<IRandomAccessStream>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CloneStream)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CanRead(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IRandomAccessStream>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CanRead)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CanWrite(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IRandomAccessStream>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CanWrite)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeName for IRandomAccessStreamWithContentType {
    const NAME: &'static str = "Windows.Storage.Streams.IRandomAccessStreamWithContentType";
}
pub trait IRandomAccessStreamWithContentType_Impl: super::super::Foundation::IClosable_Impl + IContentTypeProvider_Impl + IInputStream_Impl + IOutputStream_Impl + IRandomAccessStream_Impl {}
impl IRandomAccessStreamWithContentType_Vtbl {
    pub const fn new<Identity: IRandomAccessStreamWithContentType_Impl, const OFFSET: isize>() -> Self {
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IRandomAccessStreamWithContentType, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRandomAccessStreamWithContentType as windows_core::Interface>::IID
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRandomAccessStreamWithContentType_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InMemoryRandomAccessStream(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(InMemoryRandomAccessStream, windows_core::IUnknown, windows_core::IInspectable, IRandomAccessStream);
windows_core::imp::required_hierarchy!(InMemoryRandomAccessStream, super::super::Foundation::IClosable, IInputStream, IOutputStream);
impl InMemoryRandomAccessStream {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<InMemoryRandomAccessStream, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn ReadAsync<P0>(&self, buffer: P0, count: u32, options: InputStreamOptions) -> windows_core::Result<windows_future::IAsyncOperationWithProgress<IBuffer, u32>>
    where
        P0: windows_core::Param<IBuffer>,
    {
        let this = &windows_core::Interface::cast::<IInputStream>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReadAsync)(windows_core::Interface::as_raw(this), buffer.param().abi(), count, options, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn WriteAsync<P0>(&self, buffer: P0) -> windows_core::Result<windows_future::IAsyncOperationWithProgress<u32, u32>>
    where
        P0: windows_core::Param<IBuffer>,
    {
        let this = &windows_core::Interface::cast::<IOutputStream>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).WriteAsync)(windows_core::Interface::as_raw(this), buffer.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn FlushAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<bool>> {
        let this = &windows_core::Interface::cast::<IOutputStream>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FlushAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Size(&self) -> windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Size)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetSize(&self, value: u64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetSize)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn GetInputStreamAt(&self, position: u64) -> windows_core::Result<IInputStream> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetInputStreamAt)(windows_core::Interface::as_raw(this), position, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetOutputStreamAt(&self, position: u64) -> windows_core::Result<IOutputStream> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetOutputStreamAt)(windows_core::Interface::as_raw(this), position, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Position(&self) -> windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Position)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Seek(&self, position: u64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Seek)(windows_core::Interface::as_raw(this), position).ok() }
    }
    pub fn CloneStream(&self) -> windows_core::Result<IRandomAccessStream> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CloneStream)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CanRead(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CanRead)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CanWrite(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CanWrite)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for InMemoryRandomAccessStream {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IRandomAccessStream>();
}
unsafe impl windows_core::Interface for InMemoryRandomAccessStream {
    type Vtable = <IRandomAccessStream as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IRandomAccessStream as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for InMemoryRandomAccessStream {
    const NAME: &'static str = "Windows.Storage.Streams.InMemoryRandomAccessStream";
}
unsafe impl Send for InMemoryRandomAccessStream {}
unsafe impl Sync for InMemoryRandomAccessStream {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct InputStreamOptions(pub u32);
impl InputStreamOptions {
    pub const None: Self = Self(0u32);
    pub const Partial: Self = Self(1u32);
    pub const ReadAhead: Self = Self(2u32);
}
impl windows_core::TypeKind for InputStreamOptions {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for InputStreamOptions {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Storage.Streams.InputStreamOptions;u4)");
}
impl InputStreamOptions {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for InputStreamOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for InputStreamOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for InputStreamOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for InputStreamOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for InputStreamOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InputStreamOverStream(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(InputStreamOverStream, windows_core::IUnknown, windows_core::IInspectable, IInputStream);
windows_core::imp::required_hierarchy!(InputStreamOverStream, super::super::Foundation::IClosable);
impl InputStreamOverStream {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn ReadAsync<P0>(&self, buffer: P0, count: u32, options: InputStreamOptions) -> windows_core::Result<windows_future::IAsyncOperationWithProgress<IBuffer, u32>>
    where
        P0: windows_core::Param<IBuffer>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReadAsync)(windows_core::Interface::as_raw(this), buffer.param().abi(), count, options, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for InputStreamOverStream {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IInputStream>();
}
unsafe impl windows_core::Interface for InputStreamOverStream {
    type Vtable = <IInputStream as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IInputStream as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for InputStreamOverStream {
    const NAME: &'static str = "Windows.Storage.Streams.InputStreamOverStream";
}
unsafe impl Send for InputStreamOverStream {}
unsafe impl Sync for InputStreamOverStream {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OutputStreamOverStream(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(OutputStreamOverStream, windows_core::IUnknown, windows_core::IInspectable, IOutputStream);
windows_core::imp::required_hierarchy!(OutputStreamOverStream, super::super::Foundation::IClosable);
impl OutputStreamOverStream {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn WriteAsync<P0>(&self, buffer: P0) -> windows_core::Result<windows_future::IAsyncOperationWithProgress<u32, u32>>
    where
        P0: windows_core::Param<IBuffer>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).WriteAsync)(windows_core::Interface::as_raw(this), buffer.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn FlushAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FlushAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for OutputStreamOverStream {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IOutputStream>();
}
unsafe impl windows_core::Interface for OutputStreamOverStream {
    type Vtable = <IOutputStream as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IOutputStream as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for OutputStreamOverStream {
    const NAME: &'static str = "Windows.Storage.Streams.OutputStreamOverStream";
}
unsafe impl Send for OutputStreamOverStream {}
unsafe impl Sync for OutputStreamOverStream {}
pub struct RandomAccessStream;
impl RandomAccessStream {
    pub fn CopyAsync<P0, P1>(source: P0, destination: P1) -> windows_core::Result<windows_future::IAsyncOperationWithProgress<u64, u64>>
    where
        P0: windows_core::Param<IInputStream>,
        P1: windows_core::Param<IOutputStream>,
    {
        Self::IRandomAccessStreamStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CopyAsync)(windows_core::Interface::as_raw(this), source.param().abi(), destination.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CopySizeAsync<P0, P1>(source: P0, destination: P1, bytestocopy: u64) -> windows_core::Result<windows_future::IAsyncOperationWithProgress<u64, u64>>
    where
        P0: windows_core::Param<IInputStream>,
        P1: windows_core::Param<IOutputStream>,
    {
        Self::IRandomAccessStreamStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CopySizeAsync)(windows_core::Interface::as_raw(this), source.param().abi(), destination.param().abi(), bytestocopy, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CopyAndCloseAsync<P0, P1>(source: P0, destination: P1) -> windows_core::Result<windows_future::IAsyncOperationWithProgress<u64, u64>>
    where
        P0: windows_core::Param<IInputStream>,
        P1: windows_core::Param<IOutputStream>,
    {
        Self::IRandomAccessStreamStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CopyAndCloseAsync)(windows_core::Interface::as_raw(this), source.param().abi(), destination.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IRandomAccessStreamStatics<R, F: FnOnce(&IRandomAccessStreamStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<RandomAccessStream, IRandomAccessStreamStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeName for RandomAccessStream {
    const NAME: &'static str = "Windows.Storage.Streams.RandomAccessStream";
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RandomAccessStreamOverStream(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(RandomAccessStreamOverStream, windows_core::IUnknown, windows_core::IInspectable, IRandomAccessStream);
windows_core::imp::required_hierarchy!(RandomAccessStreamOverStream, super::super::Foundation::IClosable, IInputStream, IOutputStream);
impl RandomAccessStreamOverStream {
    pub fn Close(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Close)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn ReadAsync<P0>(&self, buffer: P0, count: u32, options: InputStreamOptions) -> windows_core::Result<windows_future::IAsyncOperationWithProgress<IBuffer, u32>>
    where
        P0: windows_core::Param<IBuffer>,
    {
        let this = &windows_core::Interface::cast::<IInputStream>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReadAsync)(windows_core::Interface::as_raw(this), buffer.param().abi(), count, options, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn WriteAsync<P0>(&self, buffer: P0) -> windows_core::Result<windows_future::IAsyncOperationWithProgress<u32, u32>>
    where
        P0: windows_core::Param<IBuffer>,
    {
        let this = &windows_core::Interface::cast::<IOutputStream>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).WriteAsync)(windows_core::Interface::as_raw(this), buffer.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn FlushAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<bool>> {
        let this = &windows_core::Interface::cast::<IOutputStream>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FlushAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Size(&self) -> windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Size)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetSize(&self, value: u64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetSize)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn GetInputStreamAt(&self, position: u64) -> windows_core::Result<IInputStream> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetInputStreamAt)(windows_core::Interface::as_raw(this), position, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetOutputStreamAt(&self, position: u64) -> windows_core::Result<IOutputStream> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetOutputStreamAt)(windows_core::Interface::as_raw(this), position, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Position(&self) -> windows_core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Position)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Seek(&self, position: u64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Seek)(windows_core::Interface::as_raw(this), position).ok() }
    }
    pub fn CloneStream(&self) -> windows_core::Result<IRandomAccessStream> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CloneStream)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CanRead(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CanRead)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn CanWrite(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CanWrite)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for RandomAccessStreamOverStream {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IRandomAccessStream>();
}
unsafe impl windows_core::Interface for RandomAccessStreamOverStream {
    type Vtable = <IRandomAccessStream as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IRandomAccessStream as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for RandomAccessStreamOverStream {
    const NAME: &'static str = "Windows.Storage.Streams.RandomAccessStreamOverStream";
}
unsafe impl Send for RandomAccessStreamOverStream {}
unsafe impl Sync for RandomAccessStreamOverStream {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RandomAccessStreamReference(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(RandomAccessStreamReference, windows_core::IUnknown, windows_core::IInspectable, IRandomAccessStreamReference);
impl RandomAccessStreamReference {
    pub fn OpenReadAsync(&self) -> windows_core::Result<windows_future::IAsyncOperation<IRandomAccessStreamWithContentType>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OpenReadAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateFromFile<P0>(file: P0) -> windows_core::Result<RandomAccessStreamReference>
    where
        P0: windows_core::Param<super::IStorageFile>,
    {
        Self::IRandomAccessStreamReferenceStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromFile)(windows_core::Interface::as_raw(this), file.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateFromUri<P0>(uri: P0) -> windows_core::Result<RandomAccessStreamReference>
    where
        P0: windows_core::Param<super::super::Foundation::Uri>,
    {
        Self::IRandomAccessStreamReferenceStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromUri)(windows_core::Interface::as_raw(this), uri.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateFromStream<P0>(stream: P0) -> windows_core::Result<RandomAccessStreamReference>
    where
        P0: windows_core::Param<IRandomAccessStream>,
    {
        Self::IRandomAccessStreamReferenceStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateFromStream)(windows_core::Interface::as_raw(this), stream.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IRandomAccessStreamReferenceStatics<R, F: FnOnce(&IRandomAccessStreamReferenceStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<RandomAccessStreamReference, IRandomAccessStreamReferenceStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for RandomAccessStreamReference {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IRandomAccessStreamReference>();
}
unsafe impl windows_core::Interface for RandomAccessStreamReference {
    type Vtable = <IRandomAccessStreamReference as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IRandomAccessStreamReference as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for RandomAccessStreamReference {
    const NAME: &'static str = "Windows.Storage.Streams.RandomAccessStreamReference";
}
unsafe impl Send for RandomAccessStreamReference {}
unsafe impl Sync for RandomAccessStreamReference {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UnicodeEncoding(pub i32);
impl UnicodeEncoding {
    pub const Utf8: Self = Self(0i32);
    pub const Utf16LE: Self = Self(1i32);
    pub const Utf16BE: Self = Self(2i32);
}
impl windows_core::TypeKind for UnicodeEncoding {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for UnicodeEncoding {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Storage.Streams.UnicodeEncoding;i4)");
}
