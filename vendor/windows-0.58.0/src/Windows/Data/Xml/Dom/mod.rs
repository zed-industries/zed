windows_core::imp::define_interface!(IDtdEntity, IDtdEntity_Vtbl, 0x6a0b5ffc_63b4_480f_9e6a_8a92816aade4);
impl windows_core::RuntimeType for IDtdEntity {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDtdEntity_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PublicId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SystemId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub NotationName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IDtdNotation, IDtdNotation_Vtbl, 0x8cb4e04d_6d46_4edb_ab73_df83c51ad397);
impl windows_core::RuntimeType for IDtdNotation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IDtdNotation_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PublicId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SystemId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IXmlAttribute, IXmlAttribute_Vtbl, 0xac144aa4_b4f1_4db6_b206_8a22c308db0a);
impl windows_core::RuntimeType for IXmlAttribute {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IXmlAttribute_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Specified: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub Value: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetValue: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IXmlCDataSection, IXmlCDataSection_Vtbl, 0x4d04b46f_c8bd_45b4_8899_0400d7c2c60f);
impl windows_core::RuntimeType for IXmlCDataSection {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IXmlCDataSection_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IXmlCharacterData, IXmlCharacterData_Vtbl, 0x132e42ab_4e36_4df6_b1c8_0ce62fd88b26);
impl core::ops::Deref for IXmlCharacterData {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IXmlCharacterData, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(IXmlCharacterData, IXmlNode, IXmlNodeSelector, IXmlNodeSerializer);
impl IXmlCharacterData {
    pub fn Data(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Data)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetData(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetData)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn Length(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Length)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SubstringData(&self, offset: u32, count: u32) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SubstringData)(windows_core::Interface::as_raw(this), offset, count, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AppendData(&self, data: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AppendData)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(data)).ok() }
    }
    pub fn InsertData(&self, offset: u32, data: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).InsertData)(windows_core::Interface::as_raw(this), offset, core::mem::transmute_copy(data)).ok() }
    }
    pub fn DeleteData(&self, offset: u32, count: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).DeleteData)(windows_core::Interface::as_raw(this), offset, count).ok() }
    }
    pub fn ReplaceData(&self, offset: u32, count: u32, data: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).ReplaceData)(windows_core::Interface::as_raw(this), offset, count, core::mem::transmute_copy(data)).ok() }
    }
    pub fn NodeValue(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NodeValue)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetNodeValue<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetNodeValue)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn NodeType(&self) -> windows_core::Result<NodeType> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NodeType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn NodeName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NodeName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ParentNode(&self) -> windows_core::Result<IXmlNode> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ParentNode)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ChildNodes(&self) -> windows_core::Result<XmlNodeList> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ChildNodes)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn FirstChild(&self) -> windows_core::Result<IXmlNode> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FirstChild)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn LastChild(&self) -> windows_core::Result<IXmlNode> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LastChild)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn PreviousSibling(&self) -> windows_core::Result<IXmlNode> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousSibling)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn NextSibling(&self) -> windows_core::Result<IXmlNode> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NextSibling)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Attributes(&self) -> windows_core::Result<XmlNamedNodeMap> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Attributes)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn HasChildNodes(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HasChildNodes)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn OwnerDocument(&self) -> windows_core::Result<XmlDocument> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OwnerDocument)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn InsertBefore<P0, P1>(&self, newchild: P0, referencechild: P1) -> windows_core::Result<IXmlNode>
    where
        P0: windows_core::Param<IXmlNode>,
        P1: windows_core::Param<IXmlNode>,
    {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InsertBefore)(windows_core::Interface::as_raw(this), newchild.param().abi(), referencechild.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ReplaceChild<P0, P1>(&self, newchild: P0, referencechild: P1) -> windows_core::Result<IXmlNode>
    where
        P0: windows_core::Param<IXmlNode>,
        P1: windows_core::Param<IXmlNode>,
    {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReplaceChild)(windows_core::Interface::as_raw(this), newchild.param().abi(), referencechild.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RemoveChild<P0>(&self, childnode: P0) -> windows_core::Result<IXmlNode>
    where
        P0: windows_core::Param<IXmlNode>,
    {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RemoveChild)(windows_core::Interface::as_raw(this), childnode.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AppendChild<P0>(&self, newchild: P0) -> windows_core::Result<IXmlNode>
    where
        P0: windows_core::Param<IXmlNode>,
    {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AppendChild)(windows_core::Interface::as_raw(this), newchild.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CloneNode(&self, deep: bool) -> windows_core::Result<IXmlNode> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CloneNode)(windows_core::Interface::as_raw(this), deep, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn NamespaceUri(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NamespaceUri)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn LocalName(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LocalName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Prefix(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Prefix)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Normalize(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Normalize)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn SetPrefix<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetPrefix)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn SelectSingleNode(&self, xpath: &windows_core::HSTRING) -> windows_core::Result<IXmlNode> {
        let this = &windows_core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SelectSingleNode)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(xpath), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SelectNodes(&self, xpath: &windows_core::HSTRING) -> windows_core::Result<XmlNodeList> {
        let this = &windows_core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SelectNodes)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(xpath), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SelectSingleNodeNS<P0>(&self, xpath: &windows_core::HSTRING, namespaces: P0) -> windows_core::Result<IXmlNode>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SelectSingleNodeNS)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(xpath), namespaces.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SelectNodesNS<P0>(&self, xpath: &windows_core::HSTRING, namespaces: P0) -> windows_core::Result<XmlNodeList>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SelectNodesNS)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(xpath), namespaces.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetXml(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetXml)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn InnerText(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InnerText)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetInnerText(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetInnerText)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
}
impl windows_core::RuntimeType for IXmlCharacterData {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IXmlCharacterData_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Data: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetData: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Length: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SubstringData: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub AppendData: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub InsertData: unsafe extern "system" fn(*mut core::ffi::c_void, u32, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub DeleteData: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub ReplaceData: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IXmlComment, IXmlComment_Vtbl, 0xbca474d5_b61f_4611_9cac_2e92e3476d47);
impl windows_core::RuntimeType for IXmlComment {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IXmlComment_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IXmlDocument, IXmlDocument_Vtbl, 0xf7f3a506_1e87_42d6_bcfb_b8c809fa5494);
impl windows_core::RuntimeType for IXmlDocument {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IXmlDocument_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Doctype: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Implementation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DocumentElement: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateElement: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateDocumentFragment: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateTextNode: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateComment: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateProcessingInstruction: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateAttribute: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateEntityReference: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetElementsByTagName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateCDataSection: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DocumentUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub CreateAttributeNS: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateElementNS: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetElementById: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ImportNode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, bool, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IXmlDocumentFragment, IXmlDocumentFragment_Vtbl, 0xe2ea6a96_0c21_44a5_8bc9_9e4a262708ec);
impl windows_core::RuntimeType for IXmlDocumentFragment {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IXmlDocumentFragment_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IXmlDocumentIO, IXmlDocumentIO_Vtbl, 0x6cd0e74e_ee65_4489_9ebf_ca43e87ba637);
impl windows_core::RuntimeType for IXmlDocumentIO {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IXmlDocumentIO_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub LoadXml: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub LoadXmlWithSettings: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Storage")]
    pub SaveToFileAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    SaveToFileAsync: usize,
}
windows_core::imp::define_interface!(IXmlDocumentIO2, IXmlDocumentIO2_Vtbl, 0x5d034661_7bd8_4ad5_9ebf_81e6347263b1);
impl windows_core::RuntimeType for IXmlDocumentIO2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IXmlDocumentIO2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub LoadXmlFromBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    LoadXmlFromBuffer: usize,
    #[cfg(feature = "Storage_Streams")]
    pub LoadXmlFromBufferWithSettings: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    LoadXmlFromBufferWithSettings: usize,
}
windows_core::imp::define_interface!(IXmlDocumentStatics, IXmlDocumentStatics_Vtbl, 0x5543d254_d757_4b79_9539_232b18f50bf1);
impl windows_core::RuntimeType for IXmlDocumentStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IXmlDocumentStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub LoadFromUriAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub LoadFromUriWithSettingsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Storage")]
    pub LoadFromFileAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    LoadFromFileAsync: usize,
    #[cfg(feature = "Storage")]
    pub LoadFromFileWithSettingsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage"))]
    LoadFromFileWithSettingsAsync: usize,
}
windows_core::imp::define_interface!(IXmlDocumentType, IXmlDocumentType_Vtbl, 0xf7342425_9781_4964_8e94_9b1c6dfc9bc7);
impl windows_core::RuntimeType for IXmlDocumentType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IXmlDocumentType_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Entities: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Notations: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IXmlDomImplementation, IXmlDomImplementation_Vtbl, 0x6de58132_f11d_4fbb_8cc6_583cba93112f);
impl windows_core::RuntimeType for IXmlDomImplementation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IXmlDomImplementation_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub HasFeature: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IXmlElement, IXmlElement_Vtbl, 0x2dfb8a1f_6b10_4ef8_9f83_efcce8faec37);
impl windows_core::RuntimeType for IXmlElement {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IXmlElement_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub TagName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub GetAttribute: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetAttribute: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub RemoveAttribute: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub GetAttributeNode: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetAttributeNode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveAttributeNode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetElementsByTagName: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetAttributeNS: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub GetAttributeNS: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub RemoveAttributeNS: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetAttributeNodeNS: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetAttributeNodeNS: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IXmlEntityReference, IXmlEntityReference_Vtbl, 0x2e2f47bc_c3d0_4ccf_bb86_0ab8c36a61cf);
impl windows_core::RuntimeType for IXmlEntityReference {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IXmlEntityReference_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IXmlLoadSettings, IXmlLoadSettings_Vtbl, 0x58aa07a8_fed6_46f7_b4c5_fb1ba72108d6);
impl windows_core::RuntimeType for IXmlLoadSettings {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IXmlLoadSettings_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub MaxElementDepth: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetMaxElementDepth: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub ProhibitDtd: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetProhibitDtd: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub ResolveExternals: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetResolveExternals: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub ValidateOnParse: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetValidateOnParse: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub ElementContentWhiteSpace: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetElementContentWhiteSpace: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IXmlNamedNodeMap, IXmlNamedNodeMap_Vtbl, 0xb3a69eb0_aab0_4b82_a6fa_b1453f7c021b);
impl windows_core::RuntimeType for IXmlNamedNodeMap {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IXmlNamedNodeMap_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Length: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Item: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetNamedItem: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetNamedItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveNamedItem: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetNamedItemNS: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveNamedItemNS: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetNamedItemNS: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IXmlNode, IXmlNode_Vtbl, 0x1c741d59_2122_47d5_a856_83f3d4214875);
impl core::ops::Deref for IXmlNode {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IXmlNode, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(IXmlNode, IXmlNodeSelector, IXmlNodeSerializer);
impl IXmlNode {
    pub fn NodeValue(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NodeValue)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetNodeValue<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetNodeValue)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn NodeType(&self) -> windows_core::Result<NodeType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NodeType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn NodeName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NodeName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ParentNode(&self) -> windows_core::Result<IXmlNode> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ParentNode)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ChildNodes(&self) -> windows_core::Result<XmlNodeList> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ChildNodes)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn FirstChild(&self) -> windows_core::Result<IXmlNode> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FirstChild)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn LastChild(&self) -> windows_core::Result<IXmlNode> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LastChild)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn PreviousSibling(&self) -> windows_core::Result<IXmlNode> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousSibling)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn NextSibling(&self) -> windows_core::Result<IXmlNode> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NextSibling)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Attributes(&self) -> windows_core::Result<XmlNamedNodeMap> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Attributes)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn HasChildNodes(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HasChildNodes)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn OwnerDocument(&self) -> windows_core::Result<XmlDocument> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OwnerDocument)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn InsertBefore<P0, P1>(&self, newchild: P0, referencechild: P1) -> windows_core::Result<IXmlNode>
    where
        P0: windows_core::Param<IXmlNode>,
        P1: windows_core::Param<IXmlNode>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InsertBefore)(windows_core::Interface::as_raw(this), newchild.param().abi(), referencechild.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ReplaceChild<P0, P1>(&self, newchild: P0, referencechild: P1) -> windows_core::Result<IXmlNode>
    where
        P0: windows_core::Param<IXmlNode>,
        P1: windows_core::Param<IXmlNode>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReplaceChild)(windows_core::Interface::as_raw(this), newchild.param().abi(), referencechild.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RemoveChild<P0>(&self, childnode: P0) -> windows_core::Result<IXmlNode>
    where
        P0: windows_core::Param<IXmlNode>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RemoveChild)(windows_core::Interface::as_raw(this), childnode.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AppendChild<P0>(&self, newchild: P0) -> windows_core::Result<IXmlNode>
    where
        P0: windows_core::Param<IXmlNode>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AppendChild)(windows_core::Interface::as_raw(this), newchild.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CloneNode(&self, deep: bool) -> windows_core::Result<IXmlNode> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CloneNode)(windows_core::Interface::as_raw(this), deep, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn NamespaceUri(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NamespaceUri)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn LocalName(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LocalName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Prefix(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Prefix)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Normalize(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Normalize)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn SetPrefix<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetPrefix)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn SelectSingleNode(&self, xpath: &windows_core::HSTRING) -> windows_core::Result<IXmlNode> {
        let this = &windows_core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SelectSingleNode)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(xpath), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SelectNodes(&self, xpath: &windows_core::HSTRING) -> windows_core::Result<XmlNodeList> {
        let this = &windows_core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SelectNodes)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(xpath), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SelectSingleNodeNS<P0>(&self, xpath: &windows_core::HSTRING, namespaces: P0) -> windows_core::Result<IXmlNode>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SelectSingleNodeNS)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(xpath), namespaces.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SelectNodesNS<P0>(&self, xpath: &windows_core::HSTRING, namespaces: P0) -> windows_core::Result<XmlNodeList>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SelectNodesNS)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(xpath), namespaces.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetXml(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetXml)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn InnerText(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InnerText)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetInnerText(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetInnerText)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
}
impl windows_core::RuntimeType for IXmlNode {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IXmlNode_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub NodeValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetNodeValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub NodeType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut NodeType) -> windows_core::HRESULT,
    pub NodeName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub ParentNode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ChildNodes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FirstChild: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub LastChild: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub PreviousSibling: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub NextSibling: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Attributes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub HasChildNodes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub OwnerDocument: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub InsertBefore: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReplaceChild: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveChild: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AppendChild: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CloneNode: unsafe extern "system" fn(*mut core::ffi::c_void, bool, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub NamespaceUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub LocalName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Prefix: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Normalize: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPrefix: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IXmlNodeList, IXmlNodeList_Vtbl, 0x8c60ad77_83a4_4ec1_9c54_7ba429e13da6);
impl windows_core::RuntimeType for IXmlNodeList {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IXmlNodeList_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Length: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Item: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IXmlNodeSelector, IXmlNodeSelector_Vtbl, 0x63dbba8b_d0db_4fe1_b745_f9433afdc25b);
impl core::ops::Deref for IXmlNodeSelector {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IXmlNodeSelector, windows_core::IUnknown, windows_core::IInspectable);
impl IXmlNodeSelector {
    pub fn SelectSingleNode(&self, xpath: &windows_core::HSTRING) -> windows_core::Result<IXmlNode> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SelectSingleNode)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(xpath), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SelectNodes(&self, xpath: &windows_core::HSTRING) -> windows_core::Result<XmlNodeList> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SelectNodes)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(xpath), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SelectSingleNodeNS<P0>(&self, xpath: &windows_core::HSTRING, namespaces: P0) -> windows_core::Result<IXmlNode>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SelectSingleNodeNS)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(xpath), namespaces.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SelectNodesNS<P0>(&self, xpath: &windows_core::HSTRING, namespaces: P0) -> windows_core::Result<XmlNodeList>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SelectNodesNS)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(xpath), namespaces.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for IXmlNodeSelector {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IXmlNodeSelector_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SelectSingleNode: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SelectNodes: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SelectSingleNodeNS: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SelectNodesNS: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IXmlNodeSerializer, IXmlNodeSerializer_Vtbl, 0x5cc5b382_e6dd_4991_abef_06d8d2e7bd0c);
impl core::ops::Deref for IXmlNodeSerializer {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IXmlNodeSerializer, windows_core::IUnknown, windows_core::IInspectable);
impl IXmlNodeSerializer {
    pub fn GetXml(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetXml)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn InnerText(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InnerText)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetInnerText(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetInnerText)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
}
impl windows_core::RuntimeType for IXmlNodeSerializer {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IXmlNodeSerializer_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetXml: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub InnerText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetInnerText: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IXmlProcessingInstruction, IXmlProcessingInstruction_Vtbl, 0x2707fd1e_1e92_4ece_b6f4_26f069078ddc);
impl windows_core::RuntimeType for IXmlProcessingInstruction {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IXmlProcessingInstruction_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Target: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub Data: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
    pub SetData: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IXmlText, IXmlText_Vtbl, 0xf931a4cb_308d_4760_a1d5_43b67450ac7e);
impl core::ops::Deref for IXmlText {
    type Target = windows_core::IInspectable;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IXmlText, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(IXmlText, IXmlCharacterData, IXmlNode, IXmlNodeSelector, IXmlNodeSerializer);
impl IXmlText {
    pub fn SplitText(&self, offset: u32) -> windows_core::Result<IXmlText> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplitText)(windows_core::Interface::as_raw(this), offset, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Data(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Data)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetData(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetData)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn Length(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Length)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SubstringData(&self, offset: u32, count: u32) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SubstringData)(windows_core::Interface::as_raw(this), offset, count, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AppendData(&self, data: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe { (windows_core::Interface::vtable(this).AppendData)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(data)).ok() }
    }
    pub fn InsertData(&self, offset: u32, data: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe { (windows_core::Interface::vtable(this).InsertData)(windows_core::Interface::as_raw(this), offset, core::mem::transmute_copy(data)).ok() }
    }
    pub fn DeleteData(&self, offset: u32, count: u32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe { (windows_core::Interface::vtable(this).DeleteData)(windows_core::Interface::as_raw(this), offset, count).ok() }
    }
    pub fn ReplaceData(&self, offset: u32, count: u32, data: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe { (windows_core::Interface::vtable(this).ReplaceData)(windows_core::Interface::as_raw(this), offset, count, core::mem::transmute_copy(data)).ok() }
    }
    pub fn NodeValue(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NodeValue)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetNodeValue<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetNodeValue)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn NodeType(&self) -> windows_core::Result<NodeType> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NodeType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn NodeName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NodeName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ParentNode(&self) -> windows_core::Result<IXmlNode> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ParentNode)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ChildNodes(&self) -> windows_core::Result<XmlNodeList> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ChildNodes)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn FirstChild(&self) -> windows_core::Result<IXmlNode> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FirstChild)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn LastChild(&self) -> windows_core::Result<IXmlNode> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LastChild)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn PreviousSibling(&self) -> windows_core::Result<IXmlNode> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousSibling)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn NextSibling(&self) -> windows_core::Result<IXmlNode> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NextSibling)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Attributes(&self) -> windows_core::Result<XmlNamedNodeMap> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Attributes)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn HasChildNodes(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HasChildNodes)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn OwnerDocument(&self) -> windows_core::Result<XmlDocument> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OwnerDocument)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn InsertBefore<P0, P1>(&self, newchild: P0, referencechild: P1) -> windows_core::Result<IXmlNode>
    where
        P0: windows_core::Param<IXmlNode>,
        P1: windows_core::Param<IXmlNode>,
    {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InsertBefore)(windows_core::Interface::as_raw(this), newchild.param().abi(), referencechild.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ReplaceChild<P0, P1>(&self, newchild: P0, referencechild: P1) -> windows_core::Result<IXmlNode>
    where
        P0: windows_core::Param<IXmlNode>,
        P1: windows_core::Param<IXmlNode>,
    {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReplaceChild)(windows_core::Interface::as_raw(this), newchild.param().abi(), referencechild.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RemoveChild<P0>(&self, childnode: P0) -> windows_core::Result<IXmlNode>
    where
        P0: windows_core::Param<IXmlNode>,
    {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RemoveChild)(windows_core::Interface::as_raw(this), childnode.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AppendChild<P0>(&self, newchild: P0) -> windows_core::Result<IXmlNode>
    where
        P0: windows_core::Param<IXmlNode>,
    {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AppendChild)(windows_core::Interface::as_raw(this), newchild.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CloneNode(&self, deep: bool) -> windows_core::Result<IXmlNode> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CloneNode)(windows_core::Interface::as_raw(this), deep, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn NamespaceUri(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NamespaceUri)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn LocalName(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LocalName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Prefix(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Prefix)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Normalize(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Normalize)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn SetPrefix<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetPrefix)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn SelectSingleNode(&self, xpath: &windows_core::HSTRING) -> windows_core::Result<IXmlNode> {
        let this = &windows_core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SelectSingleNode)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(xpath), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SelectNodes(&self, xpath: &windows_core::HSTRING) -> windows_core::Result<XmlNodeList> {
        let this = &windows_core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SelectNodes)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(xpath), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SelectSingleNodeNS<P0>(&self, xpath: &windows_core::HSTRING, namespaces: P0) -> windows_core::Result<IXmlNode>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SelectSingleNodeNS)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(xpath), namespaces.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SelectNodesNS<P0>(&self, xpath: &windows_core::HSTRING, namespaces: P0) -> windows_core::Result<XmlNodeList>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SelectNodesNS)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(xpath), namespaces.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetXml(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetXml)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn InnerText(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InnerText)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetInnerText(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetInnerText)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
}
impl windows_core::RuntimeType for IXmlText {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IXmlText_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SplitText: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct DtdEntity(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(DtdEntity, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(DtdEntity, IXmlNode, IXmlNodeSelector, IXmlNodeSerializer);
impl DtdEntity {
    pub fn PublicId(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PublicId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SystemId(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SystemId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn NotationName(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NotationName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn NodeValue(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NodeValue)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetNodeValue<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetNodeValue)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn NodeType(&self) -> windows_core::Result<NodeType> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NodeType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn NodeName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NodeName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ParentNode(&self) -> windows_core::Result<IXmlNode> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ParentNode)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ChildNodes(&self) -> windows_core::Result<XmlNodeList> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ChildNodes)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn FirstChild(&self) -> windows_core::Result<IXmlNode> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FirstChild)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn LastChild(&self) -> windows_core::Result<IXmlNode> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LastChild)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn PreviousSibling(&self) -> windows_core::Result<IXmlNode> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousSibling)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn NextSibling(&self) -> windows_core::Result<IXmlNode> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NextSibling)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Attributes(&self) -> windows_core::Result<XmlNamedNodeMap> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Attributes)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn HasChildNodes(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HasChildNodes)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn OwnerDocument(&self) -> windows_core::Result<XmlDocument> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OwnerDocument)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn InsertBefore<P0, P1>(&self, newchild: P0, referencechild: P1) -> windows_core::Result<IXmlNode>
    where
        P0: windows_core::Param<IXmlNode>,
        P1: windows_core::Param<IXmlNode>,
    {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InsertBefore)(windows_core::Interface::as_raw(this), newchild.param().abi(), referencechild.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ReplaceChild<P0, P1>(&self, newchild: P0, referencechild: P1) -> windows_core::Result<IXmlNode>
    where
        P0: windows_core::Param<IXmlNode>,
        P1: windows_core::Param<IXmlNode>,
    {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReplaceChild)(windows_core::Interface::as_raw(this), newchild.param().abi(), referencechild.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RemoveChild<P0>(&self, childnode: P0) -> windows_core::Result<IXmlNode>
    where
        P0: windows_core::Param<IXmlNode>,
    {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RemoveChild)(windows_core::Interface::as_raw(this), childnode.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AppendChild<P0>(&self, newchild: P0) -> windows_core::Result<IXmlNode>
    where
        P0: windows_core::Param<IXmlNode>,
    {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AppendChild)(windows_core::Interface::as_raw(this), newchild.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CloneNode(&self, deep: bool) -> windows_core::Result<IXmlNode> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CloneNode)(windows_core::Interface::as_raw(this), deep, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn NamespaceUri(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NamespaceUri)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn LocalName(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LocalName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Prefix(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Prefix)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Normalize(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Normalize)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn SetPrefix<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetPrefix)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn SelectSingleNode(&self, xpath: &windows_core::HSTRING) -> windows_core::Result<IXmlNode> {
        let this = &windows_core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SelectSingleNode)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(xpath), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SelectNodes(&self, xpath: &windows_core::HSTRING) -> windows_core::Result<XmlNodeList> {
        let this = &windows_core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SelectNodes)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(xpath), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SelectSingleNodeNS<P0>(&self, xpath: &windows_core::HSTRING, namespaces: P0) -> windows_core::Result<IXmlNode>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SelectSingleNodeNS)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(xpath), namespaces.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SelectNodesNS<P0>(&self, xpath: &windows_core::HSTRING, namespaces: P0) -> windows_core::Result<XmlNodeList>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SelectNodesNS)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(xpath), namespaces.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetXml(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetXml)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn InnerText(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InnerText)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetInnerText(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetInnerText)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
}
impl windows_core::RuntimeType for DtdEntity {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IDtdEntity>();
}
unsafe impl windows_core::Interface for DtdEntity {
    type Vtable = IDtdEntity_Vtbl;
    const IID: windows_core::GUID = <IDtdEntity as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for DtdEntity {
    const NAME: &'static str = "Windows.Data.Xml.Dom.DtdEntity";
}
unsafe impl Send for DtdEntity {}
unsafe impl Sync for DtdEntity {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct DtdNotation(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(DtdNotation, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(DtdNotation, IXmlNode, IXmlNodeSelector, IXmlNodeSerializer);
impl DtdNotation {
    pub fn PublicId(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PublicId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SystemId(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SystemId)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn NodeValue(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NodeValue)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetNodeValue<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetNodeValue)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn NodeType(&self) -> windows_core::Result<NodeType> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NodeType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn NodeName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NodeName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ParentNode(&self) -> windows_core::Result<IXmlNode> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ParentNode)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ChildNodes(&self) -> windows_core::Result<XmlNodeList> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ChildNodes)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn FirstChild(&self) -> windows_core::Result<IXmlNode> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FirstChild)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn LastChild(&self) -> windows_core::Result<IXmlNode> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LastChild)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn PreviousSibling(&self) -> windows_core::Result<IXmlNode> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousSibling)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn NextSibling(&self) -> windows_core::Result<IXmlNode> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NextSibling)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Attributes(&self) -> windows_core::Result<XmlNamedNodeMap> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Attributes)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn HasChildNodes(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HasChildNodes)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn OwnerDocument(&self) -> windows_core::Result<XmlDocument> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OwnerDocument)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn InsertBefore<P0, P1>(&self, newchild: P0, referencechild: P1) -> windows_core::Result<IXmlNode>
    where
        P0: windows_core::Param<IXmlNode>,
        P1: windows_core::Param<IXmlNode>,
    {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InsertBefore)(windows_core::Interface::as_raw(this), newchild.param().abi(), referencechild.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ReplaceChild<P0, P1>(&self, newchild: P0, referencechild: P1) -> windows_core::Result<IXmlNode>
    where
        P0: windows_core::Param<IXmlNode>,
        P1: windows_core::Param<IXmlNode>,
    {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReplaceChild)(windows_core::Interface::as_raw(this), newchild.param().abi(), referencechild.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RemoveChild<P0>(&self, childnode: P0) -> windows_core::Result<IXmlNode>
    where
        P0: windows_core::Param<IXmlNode>,
    {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RemoveChild)(windows_core::Interface::as_raw(this), childnode.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AppendChild<P0>(&self, newchild: P0) -> windows_core::Result<IXmlNode>
    where
        P0: windows_core::Param<IXmlNode>,
    {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AppendChild)(windows_core::Interface::as_raw(this), newchild.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CloneNode(&self, deep: bool) -> windows_core::Result<IXmlNode> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CloneNode)(windows_core::Interface::as_raw(this), deep, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn NamespaceUri(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NamespaceUri)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn LocalName(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LocalName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Prefix(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Prefix)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Normalize(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Normalize)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn SetPrefix<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetPrefix)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn SelectSingleNode(&self, xpath: &windows_core::HSTRING) -> windows_core::Result<IXmlNode> {
        let this = &windows_core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SelectSingleNode)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(xpath), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SelectNodes(&self, xpath: &windows_core::HSTRING) -> windows_core::Result<XmlNodeList> {
        let this = &windows_core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SelectNodes)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(xpath), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SelectSingleNodeNS<P0>(&self, xpath: &windows_core::HSTRING, namespaces: P0) -> windows_core::Result<IXmlNode>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SelectSingleNodeNS)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(xpath), namespaces.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SelectNodesNS<P0>(&self, xpath: &windows_core::HSTRING, namespaces: P0) -> windows_core::Result<XmlNodeList>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SelectNodesNS)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(xpath), namespaces.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetXml(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetXml)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn InnerText(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InnerText)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetInnerText(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetInnerText)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
}
impl windows_core::RuntimeType for DtdNotation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IDtdNotation>();
}
unsafe impl windows_core::Interface for DtdNotation {
    type Vtable = IDtdNotation_Vtbl;
    const IID: windows_core::GUID = <IDtdNotation as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for DtdNotation {
    const NAME: &'static str = "Windows.Data.Xml.Dom.DtdNotation";
}
unsafe impl Send for DtdNotation {}
unsafe impl Sync for DtdNotation {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct XmlAttribute(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(XmlAttribute, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(XmlAttribute, IXmlNode, IXmlNodeSelector, IXmlNodeSerializer);
impl XmlAttribute {
    pub fn Name(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Name)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Specified(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Specified)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Value(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Value)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetValue(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetValue)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn NodeValue(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NodeValue)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetNodeValue<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetNodeValue)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn NodeType(&self) -> windows_core::Result<NodeType> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NodeType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn NodeName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NodeName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ParentNode(&self) -> windows_core::Result<IXmlNode> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ParentNode)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ChildNodes(&self) -> windows_core::Result<XmlNodeList> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ChildNodes)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn FirstChild(&self) -> windows_core::Result<IXmlNode> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FirstChild)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn LastChild(&self) -> windows_core::Result<IXmlNode> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LastChild)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn PreviousSibling(&self) -> windows_core::Result<IXmlNode> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousSibling)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn NextSibling(&self) -> windows_core::Result<IXmlNode> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NextSibling)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Attributes(&self) -> windows_core::Result<XmlNamedNodeMap> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Attributes)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn HasChildNodes(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HasChildNodes)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn OwnerDocument(&self) -> windows_core::Result<XmlDocument> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OwnerDocument)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn InsertBefore<P0, P1>(&self, newchild: P0, referencechild: P1) -> windows_core::Result<IXmlNode>
    where
        P0: windows_core::Param<IXmlNode>,
        P1: windows_core::Param<IXmlNode>,
    {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InsertBefore)(windows_core::Interface::as_raw(this), newchild.param().abi(), referencechild.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ReplaceChild<P0, P1>(&self, newchild: P0, referencechild: P1) -> windows_core::Result<IXmlNode>
    where
        P0: windows_core::Param<IXmlNode>,
        P1: windows_core::Param<IXmlNode>,
    {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReplaceChild)(windows_core::Interface::as_raw(this), newchild.param().abi(), referencechild.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RemoveChild<P0>(&self, childnode: P0) -> windows_core::Result<IXmlNode>
    where
        P0: windows_core::Param<IXmlNode>,
    {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RemoveChild)(windows_core::Interface::as_raw(this), childnode.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AppendChild<P0>(&self, newchild: P0) -> windows_core::Result<IXmlNode>
    where
        P0: windows_core::Param<IXmlNode>,
    {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AppendChild)(windows_core::Interface::as_raw(this), newchild.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CloneNode(&self, deep: bool) -> windows_core::Result<IXmlNode> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CloneNode)(windows_core::Interface::as_raw(this), deep, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn NamespaceUri(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NamespaceUri)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn LocalName(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LocalName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Prefix(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Prefix)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Normalize(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Normalize)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn SetPrefix<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetPrefix)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn SelectSingleNode(&self, xpath: &windows_core::HSTRING) -> windows_core::Result<IXmlNode> {
        let this = &windows_core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SelectSingleNode)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(xpath), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SelectNodes(&self, xpath: &windows_core::HSTRING) -> windows_core::Result<XmlNodeList> {
        let this = &windows_core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SelectNodes)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(xpath), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SelectSingleNodeNS<P0>(&self, xpath: &windows_core::HSTRING, namespaces: P0) -> windows_core::Result<IXmlNode>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SelectSingleNodeNS)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(xpath), namespaces.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SelectNodesNS<P0>(&self, xpath: &windows_core::HSTRING, namespaces: P0) -> windows_core::Result<XmlNodeList>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SelectNodesNS)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(xpath), namespaces.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetXml(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetXml)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn InnerText(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InnerText)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetInnerText(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetInnerText)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
}
impl windows_core::RuntimeType for XmlAttribute {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IXmlAttribute>();
}
unsafe impl windows_core::Interface for XmlAttribute {
    type Vtable = IXmlAttribute_Vtbl;
    const IID: windows_core::GUID = <IXmlAttribute as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for XmlAttribute {
    const NAME: &'static str = "Windows.Data.Xml.Dom.XmlAttribute";
}
unsafe impl Send for XmlAttribute {}
unsafe impl Sync for XmlAttribute {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct XmlCDataSection(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(XmlCDataSection, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(XmlCDataSection, IXmlCharacterData, IXmlNode, IXmlNodeSelector, IXmlNodeSerializer, IXmlText);
impl XmlCDataSection {
    pub fn Data(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Data)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetData(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetData)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn Length(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Length)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SubstringData(&self, offset: u32, count: u32) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SubstringData)(windows_core::Interface::as_raw(this), offset, count, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AppendData(&self, data: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe { (windows_core::Interface::vtable(this).AppendData)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(data)).ok() }
    }
    pub fn InsertData(&self, offset: u32, data: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe { (windows_core::Interface::vtable(this).InsertData)(windows_core::Interface::as_raw(this), offset, core::mem::transmute_copy(data)).ok() }
    }
    pub fn DeleteData(&self, offset: u32, count: u32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe { (windows_core::Interface::vtable(this).DeleteData)(windows_core::Interface::as_raw(this), offset, count).ok() }
    }
    pub fn ReplaceData(&self, offset: u32, count: u32, data: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe { (windows_core::Interface::vtable(this).ReplaceData)(windows_core::Interface::as_raw(this), offset, count, core::mem::transmute_copy(data)).ok() }
    }
    pub fn NodeValue(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NodeValue)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetNodeValue<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetNodeValue)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn NodeType(&self) -> windows_core::Result<NodeType> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NodeType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn NodeName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NodeName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ParentNode(&self) -> windows_core::Result<IXmlNode> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ParentNode)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ChildNodes(&self) -> windows_core::Result<XmlNodeList> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ChildNodes)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn FirstChild(&self) -> windows_core::Result<IXmlNode> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FirstChild)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn LastChild(&self) -> windows_core::Result<IXmlNode> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LastChild)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn PreviousSibling(&self) -> windows_core::Result<IXmlNode> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousSibling)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn NextSibling(&self) -> windows_core::Result<IXmlNode> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NextSibling)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Attributes(&self) -> windows_core::Result<XmlNamedNodeMap> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Attributes)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn HasChildNodes(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HasChildNodes)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn OwnerDocument(&self) -> windows_core::Result<XmlDocument> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OwnerDocument)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn InsertBefore<P0, P1>(&self, newchild: P0, referencechild: P1) -> windows_core::Result<IXmlNode>
    where
        P0: windows_core::Param<IXmlNode>,
        P1: windows_core::Param<IXmlNode>,
    {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InsertBefore)(windows_core::Interface::as_raw(this), newchild.param().abi(), referencechild.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ReplaceChild<P0, P1>(&self, newchild: P0, referencechild: P1) -> windows_core::Result<IXmlNode>
    where
        P0: windows_core::Param<IXmlNode>,
        P1: windows_core::Param<IXmlNode>,
    {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReplaceChild)(windows_core::Interface::as_raw(this), newchild.param().abi(), referencechild.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RemoveChild<P0>(&self, childnode: P0) -> windows_core::Result<IXmlNode>
    where
        P0: windows_core::Param<IXmlNode>,
    {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RemoveChild)(windows_core::Interface::as_raw(this), childnode.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AppendChild<P0>(&self, newchild: P0) -> windows_core::Result<IXmlNode>
    where
        P0: windows_core::Param<IXmlNode>,
    {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AppendChild)(windows_core::Interface::as_raw(this), newchild.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CloneNode(&self, deep: bool) -> windows_core::Result<IXmlNode> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CloneNode)(windows_core::Interface::as_raw(this), deep, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn NamespaceUri(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NamespaceUri)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn LocalName(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LocalName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Prefix(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Prefix)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Normalize(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Normalize)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn SetPrefix<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetPrefix)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn SelectSingleNode(&self, xpath: &windows_core::HSTRING) -> windows_core::Result<IXmlNode> {
        let this = &windows_core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SelectSingleNode)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(xpath), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SelectNodes(&self, xpath: &windows_core::HSTRING) -> windows_core::Result<XmlNodeList> {
        let this = &windows_core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SelectNodes)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(xpath), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SelectSingleNodeNS<P0>(&self, xpath: &windows_core::HSTRING, namespaces: P0) -> windows_core::Result<IXmlNode>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SelectSingleNodeNS)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(xpath), namespaces.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SelectNodesNS<P0>(&self, xpath: &windows_core::HSTRING, namespaces: P0) -> windows_core::Result<XmlNodeList>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SelectNodesNS)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(xpath), namespaces.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetXml(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetXml)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn InnerText(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InnerText)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetInnerText(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetInnerText)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn SplitText(&self, offset: u32) -> windows_core::Result<IXmlText> {
        let this = &windows_core::Interface::cast::<IXmlText>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplitText)(windows_core::Interface::as_raw(this), offset, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for XmlCDataSection {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IXmlCDataSection>();
}
unsafe impl windows_core::Interface for XmlCDataSection {
    type Vtable = IXmlCDataSection_Vtbl;
    const IID: windows_core::GUID = <IXmlCDataSection as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for XmlCDataSection {
    const NAME: &'static str = "Windows.Data.Xml.Dom.XmlCDataSection";
}
unsafe impl Send for XmlCDataSection {}
unsafe impl Sync for XmlCDataSection {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct XmlComment(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(XmlComment, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(XmlComment, IXmlCharacterData, IXmlNode, IXmlNodeSelector, IXmlNodeSerializer);
impl XmlComment {
    pub fn Data(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Data)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetData(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetData)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn Length(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Length)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SubstringData(&self, offset: u32, count: u32) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SubstringData)(windows_core::Interface::as_raw(this), offset, count, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AppendData(&self, data: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe { (windows_core::Interface::vtable(this).AppendData)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(data)).ok() }
    }
    pub fn InsertData(&self, offset: u32, data: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe { (windows_core::Interface::vtable(this).InsertData)(windows_core::Interface::as_raw(this), offset, core::mem::transmute_copy(data)).ok() }
    }
    pub fn DeleteData(&self, offset: u32, count: u32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe { (windows_core::Interface::vtable(this).DeleteData)(windows_core::Interface::as_raw(this), offset, count).ok() }
    }
    pub fn ReplaceData(&self, offset: u32, count: u32, data: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe { (windows_core::Interface::vtable(this).ReplaceData)(windows_core::Interface::as_raw(this), offset, count, core::mem::transmute_copy(data)).ok() }
    }
    pub fn NodeValue(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NodeValue)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetNodeValue<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetNodeValue)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn NodeType(&self) -> windows_core::Result<NodeType> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NodeType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn NodeName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NodeName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ParentNode(&self) -> windows_core::Result<IXmlNode> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ParentNode)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ChildNodes(&self) -> windows_core::Result<XmlNodeList> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ChildNodes)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn FirstChild(&self) -> windows_core::Result<IXmlNode> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FirstChild)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn LastChild(&self) -> windows_core::Result<IXmlNode> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LastChild)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn PreviousSibling(&self) -> windows_core::Result<IXmlNode> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousSibling)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn NextSibling(&self) -> windows_core::Result<IXmlNode> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NextSibling)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Attributes(&self) -> windows_core::Result<XmlNamedNodeMap> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Attributes)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn HasChildNodes(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HasChildNodes)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn OwnerDocument(&self) -> windows_core::Result<XmlDocument> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OwnerDocument)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn InsertBefore<P0, P1>(&self, newchild: P0, referencechild: P1) -> windows_core::Result<IXmlNode>
    where
        P0: windows_core::Param<IXmlNode>,
        P1: windows_core::Param<IXmlNode>,
    {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InsertBefore)(windows_core::Interface::as_raw(this), newchild.param().abi(), referencechild.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ReplaceChild<P0, P1>(&self, newchild: P0, referencechild: P1) -> windows_core::Result<IXmlNode>
    where
        P0: windows_core::Param<IXmlNode>,
        P1: windows_core::Param<IXmlNode>,
    {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReplaceChild)(windows_core::Interface::as_raw(this), newchild.param().abi(), referencechild.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RemoveChild<P0>(&self, childnode: P0) -> windows_core::Result<IXmlNode>
    where
        P0: windows_core::Param<IXmlNode>,
    {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RemoveChild)(windows_core::Interface::as_raw(this), childnode.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AppendChild<P0>(&self, newchild: P0) -> windows_core::Result<IXmlNode>
    where
        P0: windows_core::Param<IXmlNode>,
    {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AppendChild)(windows_core::Interface::as_raw(this), newchild.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CloneNode(&self, deep: bool) -> windows_core::Result<IXmlNode> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CloneNode)(windows_core::Interface::as_raw(this), deep, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn NamespaceUri(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NamespaceUri)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn LocalName(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LocalName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Prefix(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Prefix)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Normalize(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Normalize)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn SetPrefix<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetPrefix)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn SelectSingleNode(&self, xpath: &windows_core::HSTRING) -> windows_core::Result<IXmlNode> {
        let this = &windows_core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SelectSingleNode)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(xpath), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SelectNodes(&self, xpath: &windows_core::HSTRING) -> windows_core::Result<XmlNodeList> {
        let this = &windows_core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SelectNodes)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(xpath), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SelectSingleNodeNS<P0>(&self, xpath: &windows_core::HSTRING, namespaces: P0) -> windows_core::Result<IXmlNode>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SelectSingleNodeNS)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(xpath), namespaces.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SelectNodesNS<P0>(&self, xpath: &windows_core::HSTRING, namespaces: P0) -> windows_core::Result<XmlNodeList>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SelectNodesNS)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(xpath), namespaces.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetXml(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetXml)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn InnerText(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InnerText)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetInnerText(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetInnerText)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
}
impl windows_core::RuntimeType for XmlComment {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IXmlComment>();
}
unsafe impl windows_core::Interface for XmlComment {
    type Vtable = IXmlComment_Vtbl;
    const IID: windows_core::GUID = <IXmlComment as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for XmlComment {
    const NAME: &'static str = "Windows.Data.Xml.Dom.XmlComment";
}
unsafe impl Send for XmlComment {}
unsafe impl Sync for XmlComment {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct XmlDocument(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(XmlDocument, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(XmlDocument, IXmlNode, IXmlNodeSelector, IXmlNodeSerializer);
impl XmlDocument {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<XmlDocument, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Doctype(&self) -> windows_core::Result<XmlDocumentType> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Doctype)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Implementation(&self) -> windows_core::Result<XmlDomImplementation> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Implementation)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DocumentElement(&self) -> windows_core::Result<XmlElement> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DocumentElement)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateElement(&self, tagname: &windows_core::HSTRING) -> windows_core::Result<XmlElement> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateElement)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(tagname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateDocumentFragment(&self) -> windows_core::Result<XmlDocumentFragment> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateDocumentFragment)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateTextNode(&self, data: &windows_core::HSTRING) -> windows_core::Result<XmlText> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateTextNode)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(data), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateComment(&self, data: &windows_core::HSTRING) -> windows_core::Result<XmlComment> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateComment)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(data), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateProcessingInstruction(&self, target: &windows_core::HSTRING, data: &windows_core::HSTRING) -> windows_core::Result<XmlProcessingInstruction> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateProcessingInstruction)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(target), core::mem::transmute_copy(data), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateAttribute(&self, name: &windows_core::HSTRING) -> windows_core::Result<XmlAttribute> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateAttribute)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateEntityReference(&self, name: &windows_core::HSTRING) -> windows_core::Result<XmlEntityReference> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateEntityReference)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetElementsByTagName(&self, tagname: &windows_core::HSTRING) -> windows_core::Result<XmlNodeList> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetElementsByTagName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(tagname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateCDataSection(&self, data: &windows_core::HSTRING) -> windows_core::Result<XmlCDataSection> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateCDataSection)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(data), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn DocumentUri(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DocumentUri)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateAttributeNS<P0>(&self, namespaceuri: P0, qualifiedname: &windows_core::HSTRING) -> windows_core::Result<XmlAttribute>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateAttributeNS)(windows_core::Interface::as_raw(this), namespaceuri.param().abi(), core::mem::transmute_copy(qualifiedname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CreateElementNS<P0>(&self, namespaceuri: P0, qualifiedname: &windows_core::HSTRING) -> windows_core::Result<XmlElement>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateElementNS)(windows_core::Interface::as_raw(this), namespaceuri.param().abi(), core::mem::transmute_copy(qualifiedname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetElementById(&self, elementid: &windows_core::HSTRING) -> windows_core::Result<XmlElement> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetElementById)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(elementid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ImportNode<P0>(&self, node: P0, deep: bool) -> windows_core::Result<IXmlNode>
    where
        P0: windows_core::Param<IXmlNode>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ImportNode)(windows_core::Interface::as_raw(this), node.param().abi(), deep, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn LoadXml(&self, xml: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IXmlDocumentIO>(self)?;
        unsafe { (windows_core::Interface::vtable(this).LoadXml)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(xml)).ok() }
    }
    pub fn LoadXmlWithSettings<P0>(&self, xml: &windows_core::HSTRING, loadsettings: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<XmlLoadSettings>,
    {
        let this = &windows_core::Interface::cast::<IXmlDocumentIO>(self)?;
        unsafe { (windows_core::Interface::vtable(this).LoadXmlWithSettings)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(xml), loadsettings.param().abi()).ok() }
    }
    #[cfg(feature = "Storage")]
    pub fn SaveToFileAsync<P0>(&self, file: P0) -> windows_core::Result<super::super::super::Foundation::IAsyncAction>
    where
        P0: windows_core::Param<super::super::super::Storage::IStorageFile>,
    {
        let this = &windows_core::Interface::cast::<IXmlDocumentIO>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SaveToFileAsync)(windows_core::Interface::as_raw(this), file.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn LoadXmlFromBuffer<P0>(&self, buffer: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Storage::Streams::IBuffer>,
    {
        let this = &windows_core::Interface::cast::<IXmlDocumentIO2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).LoadXmlFromBuffer)(windows_core::Interface::as_raw(this), buffer.param().abi()).ok() }
    }
    #[cfg(feature = "Storage_Streams")]
    pub fn LoadXmlFromBufferWithSettings<P0, P1>(&self, buffer: P0, loadsettings: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Storage::Streams::IBuffer>,
        P1: windows_core::Param<XmlLoadSettings>,
    {
        let this = &windows_core::Interface::cast::<IXmlDocumentIO2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).LoadXmlFromBufferWithSettings)(windows_core::Interface::as_raw(this), buffer.param().abi(), loadsettings.param().abi()).ok() }
    }
    pub fn LoadFromUriAsync<P0>(uri: P0) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<XmlDocument>>
    where
        P0: windows_core::Param<super::super::super::Foundation::Uri>,
    {
        Self::IXmlDocumentStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LoadFromUriAsync)(windows_core::Interface::as_raw(this), uri.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn LoadFromUriWithSettingsAsync<P0, P1>(uri: P0, loadsettings: P1) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<XmlDocument>>
    where
        P0: windows_core::Param<super::super::super::Foundation::Uri>,
        P1: windows_core::Param<XmlLoadSettings>,
    {
        Self::IXmlDocumentStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LoadFromUriWithSettingsAsync)(windows_core::Interface::as_raw(this), uri.param().abi(), loadsettings.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Storage")]
    pub fn LoadFromFileAsync<P0>(file: P0) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<XmlDocument>>
    where
        P0: windows_core::Param<super::super::super::Storage::IStorageFile>,
    {
        Self::IXmlDocumentStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LoadFromFileAsync)(windows_core::Interface::as_raw(this), file.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    #[cfg(feature = "Storage")]
    pub fn LoadFromFileWithSettingsAsync<P0, P1>(file: P0, loadsettings: P1) -> windows_core::Result<super::super::super::Foundation::IAsyncOperation<XmlDocument>>
    where
        P0: windows_core::Param<super::super::super::Storage::IStorageFile>,
        P1: windows_core::Param<XmlLoadSettings>,
    {
        Self::IXmlDocumentStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LoadFromFileWithSettingsAsync)(windows_core::Interface::as_raw(this), file.param().abi(), loadsettings.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn NodeValue(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NodeValue)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetNodeValue<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetNodeValue)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn NodeType(&self) -> windows_core::Result<NodeType> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NodeType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn NodeName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NodeName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ParentNode(&self) -> windows_core::Result<IXmlNode> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ParentNode)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ChildNodes(&self) -> windows_core::Result<XmlNodeList> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ChildNodes)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn FirstChild(&self) -> windows_core::Result<IXmlNode> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FirstChild)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn LastChild(&self) -> windows_core::Result<IXmlNode> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LastChild)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn PreviousSibling(&self) -> windows_core::Result<IXmlNode> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousSibling)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn NextSibling(&self) -> windows_core::Result<IXmlNode> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NextSibling)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Attributes(&self) -> windows_core::Result<XmlNamedNodeMap> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Attributes)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn HasChildNodes(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HasChildNodes)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn OwnerDocument(&self) -> windows_core::Result<XmlDocument> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OwnerDocument)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn InsertBefore<P0, P1>(&self, newchild: P0, referencechild: P1) -> windows_core::Result<IXmlNode>
    where
        P0: windows_core::Param<IXmlNode>,
        P1: windows_core::Param<IXmlNode>,
    {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InsertBefore)(windows_core::Interface::as_raw(this), newchild.param().abi(), referencechild.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ReplaceChild<P0, P1>(&self, newchild: P0, referencechild: P1) -> windows_core::Result<IXmlNode>
    where
        P0: windows_core::Param<IXmlNode>,
        P1: windows_core::Param<IXmlNode>,
    {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReplaceChild)(windows_core::Interface::as_raw(this), newchild.param().abi(), referencechild.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RemoveChild<P0>(&self, childnode: P0) -> windows_core::Result<IXmlNode>
    where
        P0: windows_core::Param<IXmlNode>,
    {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RemoveChild)(windows_core::Interface::as_raw(this), childnode.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AppendChild<P0>(&self, newchild: P0) -> windows_core::Result<IXmlNode>
    where
        P0: windows_core::Param<IXmlNode>,
    {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AppendChild)(windows_core::Interface::as_raw(this), newchild.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CloneNode(&self, deep: bool) -> windows_core::Result<IXmlNode> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CloneNode)(windows_core::Interface::as_raw(this), deep, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn NamespaceUri(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NamespaceUri)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn LocalName(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LocalName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Prefix(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Prefix)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Normalize(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Normalize)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn SetPrefix<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetPrefix)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn SelectSingleNode(&self, xpath: &windows_core::HSTRING) -> windows_core::Result<IXmlNode> {
        let this = &windows_core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SelectSingleNode)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(xpath), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SelectNodes(&self, xpath: &windows_core::HSTRING) -> windows_core::Result<XmlNodeList> {
        let this = &windows_core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SelectNodes)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(xpath), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SelectSingleNodeNS<P0>(&self, xpath: &windows_core::HSTRING, namespaces: P0) -> windows_core::Result<IXmlNode>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SelectSingleNodeNS)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(xpath), namespaces.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SelectNodesNS<P0>(&self, xpath: &windows_core::HSTRING, namespaces: P0) -> windows_core::Result<XmlNodeList>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SelectNodesNS)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(xpath), namespaces.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetXml(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetXml)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn InnerText(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InnerText)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetInnerText(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetInnerText)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    #[doc(hidden)]
    pub fn IXmlDocumentStatics<R, F: FnOnce(&IXmlDocumentStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<XmlDocument, IXmlDocumentStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for XmlDocument {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IXmlDocument>();
}
unsafe impl windows_core::Interface for XmlDocument {
    type Vtable = IXmlDocument_Vtbl;
    const IID: windows_core::GUID = <IXmlDocument as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for XmlDocument {
    const NAME: &'static str = "Windows.Data.Xml.Dom.XmlDocument";
}
unsafe impl Send for XmlDocument {}
unsafe impl Sync for XmlDocument {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct XmlDocumentFragment(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(XmlDocumentFragment, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(XmlDocumentFragment, IXmlNode, IXmlNodeSelector, IXmlNodeSerializer);
impl XmlDocumentFragment {
    pub fn NodeValue(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NodeValue)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetNodeValue<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetNodeValue)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn NodeType(&self) -> windows_core::Result<NodeType> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NodeType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn NodeName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NodeName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ParentNode(&self) -> windows_core::Result<IXmlNode> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ParentNode)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ChildNodes(&self) -> windows_core::Result<XmlNodeList> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ChildNodes)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn FirstChild(&self) -> windows_core::Result<IXmlNode> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FirstChild)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn LastChild(&self) -> windows_core::Result<IXmlNode> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LastChild)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn PreviousSibling(&self) -> windows_core::Result<IXmlNode> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousSibling)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn NextSibling(&self) -> windows_core::Result<IXmlNode> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NextSibling)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Attributes(&self) -> windows_core::Result<XmlNamedNodeMap> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Attributes)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn HasChildNodes(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HasChildNodes)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn OwnerDocument(&self) -> windows_core::Result<XmlDocument> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OwnerDocument)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn InsertBefore<P0, P1>(&self, newchild: P0, referencechild: P1) -> windows_core::Result<IXmlNode>
    where
        P0: windows_core::Param<IXmlNode>,
        P1: windows_core::Param<IXmlNode>,
    {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InsertBefore)(windows_core::Interface::as_raw(this), newchild.param().abi(), referencechild.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ReplaceChild<P0, P1>(&self, newchild: P0, referencechild: P1) -> windows_core::Result<IXmlNode>
    where
        P0: windows_core::Param<IXmlNode>,
        P1: windows_core::Param<IXmlNode>,
    {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReplaceChild)(windows_core::Interface::as_raw(this), newchild.param().abi(), referencechild.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RemoveChild<P0>(&self, childnode: P0) -> windows_core::Result<IXmlNode>
    where
        P0: windows_core::Param<IXmlNode>,
    {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RemoveChild)(windows_core::Interface::as_raw(this), childnode.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AppendChild<P0>(&self, newchild: P0) -> windows_core::Result<IXmlNode>
    where
        P0: windows_core::Param<IXmlNode>,
    {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AppendChild)(windows_core::Interface::as_raw(this), newchild.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CloneNode(&self, deep: bool) -> windows_core::Result<IXmlNode> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CloneNode)(windows_core::Interface::as_raw(this), deep, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn NamespaceUri(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NamespaceUri)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn LocalName(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LocalName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Prefix(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Prefix)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Normalize(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Normalize)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn SetPrefix<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetPrefix)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn SelectSingleNode(&self, xpath: &windows_core::HSTRING) -> windows_core::Result<IXmlNode> {
        let this = &windows_core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SelectSingleNode)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(xpath), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SelectNodes(&self, xpath: &windows_core::HSTRING) -> windows_core::Result<XmlNodeList> {
        let this = &windows_core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SelectNodes)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(xpath), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SelectSingleNodeNS<P0>(&self, xpath: &windows_core::HSTRING, namespaces: P0) -> windows_core::Result<IXmlNode>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SelectSingleNodeNS)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(xpath), namespaces.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SelectNodesNS<P0>(&self, xpath: &windows_core::HSTRING, namespaces: P0) -> windows_core::Result<XmlNodeList>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SelectNodesNS)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(xpath), namespaces.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetXml(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetXml)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn InnerText(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InnerText)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetInnerText(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetInnerText)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
}
impl windows_core::RuntimeType for XmlDocumentFragment {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IXmlDocumentFragment>();
}
unsafe impl windows_core::Interface for XmlDocumentFragment {
    type Vtable = IXmlDocumentFragment_Vtbl;
    const IID: windows_core::GUID = <IXmlDocumentFragment as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for XmlDocumentFragment {
    const NAME: &'static str = "Windows.Data.Xml.Dom.XmlDocumentFragment";
}
unsafe impl Send for XmlDocumentFragment {}
unsafe impl Sync for XmlDocumentFragment {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct XmlDocumentType(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(XmlDocumentType, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(XmlDocumentType, IXmlNode, IXmlNodeSelector, IXmlNodeSerializer);
impl XmlDocumentType {
    pub fn Name(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Name)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Entities(&self) -> windows_core::Result<XmlNamedNodeMap> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Entities)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Notations(&self) -> windows_core::Result<XmlNamedNodeMap> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Notations)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn NodeValue(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NodeValue)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetNodeValue<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetNodeValue)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn NodeType(&self) -> windows_core::Result<NodeType> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NodeType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn NodeName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NodeName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ParentNode(&self) -> windows_core::Result<IXmlNode> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ParentNode)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ChildNodes(&self) -> windows_core::Result<XmlNodeList> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ChildNodes)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn FirstChild(&self) -> windows_core::Result<IXmlNode> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FirstChild)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn LastChild(&self) -> windows_core::Result<IXmlNode> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LastChild)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn PreviousSibling(&self) -> windows_core::Result<IXmlNode> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousSibling)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn NextSibling(&self) -> windows_core::Result<IXmlNode> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NextSibling)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Attributes(&self) -> windows_core::Result<XmlNamedNodeMap> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Attributes)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn HasChildNodes(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HasChildNodes)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn OwnerDocument(&self) -> windows_core::Result<XmlDocument> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OwnerDocument)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn InsertBefore<P0, P1>(&self, newchild: P0, referencechild: P1) -> windows_core::Result<IXmlNode>
    where
        P0: windows_core::Param<IXmlNode>,
        P1: windows_core::Param<IXmlNode>,
    {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InsertBefore)(windows_core::Interface::as_raw(this), newchild.param().abi(), referencechild.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ReplaceChild<P0, P1>(&self, newchild: P0, referencechild: P1) -> windows_core::Result<IXmlNode>
    where
        P0: windows_core::Param<IXmlNode>,
        P1: windows_core::Param<IXmlNode>,
    {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReplaceChild)(windows_core::Interface::as_raw(this), newchild.param().abi(), referencechild.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RemoveChild<P0>(&self, childnode: P0) -> windows_core::Result<IXmlNode>
    where
        P0: windows_core::Param<IXmlNode>,
    {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RemoveChild)(windows_core::Interface::as_raw(this), childnode.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AppendChild<P0>(&self, newchild: P0) -> windows_core::Result<IXmlNode>
    where
        P0: windows_core::Param<IXmlNode>,
    {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AppendChild)(windows_core::Interface::as_raw(this), newchild.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CloneNode(&self, deep: bool) -> windows_core::Result<IXmlNode> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CloneNode)(windows_core::Interface::as_raw(this), deep, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn NamespaceUri(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NamespaceUri)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn LocalName(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LocalName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Prefix(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Prefix)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Normalize(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Normalize)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn SetPrefix<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetPrefix)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn SelectSingleNode(&self, xpath: &windows_core::HSTRING) -> windows_core::Result<IXmlNode> {
        let this = &windows_core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SelectSingleNode)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(xpath), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SelectNodes(&self, xpath: &windows_core::HSTRING) -> windows_core::Result<XmlNodeList> {
        let this = &windows_core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SelectNodes)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(xpath), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SelectSingleNodeNS<P0>(&self, xpath: &windows_core::HSTRING, namespaces: P0) -> windows_core::Result<IXmlNode>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SelectSingleNodeNS)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(xpath), namespaces.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SelectNodesNS<P0>(&self, xpath: &windows_core::HSTRING, namespaces: P0) -> windows_core::Result<XmlNodeList>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SelectNodesNS)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(xpath), namespaces.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetXml(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetXml)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn InnerText(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InnerText)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetInnerText(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetInnerText)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
}
impl windows_core::RuntimeType for XmlDocumentType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IXmlDocumentType>();
}
unsafe impl windows_core::Interface for XmlDocumentType {
    type Vtable = IXmlDocumentType_Vtbl;
    const IID: windows_core::GUID = <IXmlDocumentType as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for XmlDocumentType {
    const NAME: &'static str = "Windows.Data.Xml.Dom.XmlDocumentType";
}
unsafe impl Send for XmlDocumentType {}
unsafe impl Sync for XmlDocumentType {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct XmlDomImplementation(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(XmlDomImplementation, windows_core::IUnknown, windows_core::IInspectable);
impl XmlDomImplementation {
    pub fn HasFeature<P0>(&self, feature: &windows_core::HSTRING, version: P0) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HasFeature)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(feature), version.param().abi(), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for XmlDomImplementation {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IXmlDomImplementation>();
}
unsafe impl windows_core::Interface for XmlDomImplementation {
    type Vtable = IXmlDomImplementation_Vtbl;
    const IID: windows_core::GUID = <IXmlDomImplementation as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for XmlDomImplementation {
    const NAME: &'static str = "Windows.Data.Xml.Dom.XmlDomImplementation";
}
unsafe impl Send for XmlDomImplementation {}
unsafe impl Sync for XmlDomImplementation {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct XmlElement(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(XmlElement, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(XmlElement, IXmlNode, IXmlNodeSelector, IXmlNodeSerializer);
impl XmlElement {
    pub fn TagName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TagName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetAttribute(&self, attributename: &windows_core::HSTRING) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAttribute)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(attributename), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetAttribute(&self, attributename: &windows_core::HSTRING, attributevalue: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetAttribute)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(attributename), core::mem::transmute_copy(attributevalue)).ok() }
    }
    pub fn RemoveAttribute(&self, attributename: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveAttribute)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(attributename)).ok() }
    }
    pub fn GetAttributeNode(&self, attributename: &windows_core::HSTRING) -> windows_core::Result<XmlAttribute> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAttributeNode)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(attributename), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetAttributeNode<P0>(&self, newattribute: P0) -> windows_core::Result<XmlAttribute>
    where
        P0: windows_core::Param<XmlAttribute>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SetAttributeNode)(windows_core::Interface::as_raw(this), newattribute.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RemoveAttributeNode<P0>(&self, attributenode: P0) -> windows_core::Result<XmlAttribute>
    where
        P0: windows_core::Param<XmlAttribute>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RemoveAttributeNode)(windows_core::Interface::as_raw(this), attributenode.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetElementsByTagName(&self, tagname: &windows_core::HSTRING) -> windows_core::Result<XmlNodeList> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetElementsByTagName)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(tagname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetAttributeNS<P0>(&self, namespaceuri: P0, qualifiedname: &windows_core::HSTRING, value: &windows_core::HSTRING) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetAttributeNS)(windows_core::Interface::as_raw(this), namespaceuri.param().abi(), core::mem::transmute_copy(qualifiedname), core::mem::transmute_copy(value)).ok() }
    }
    pub fn GetAttributeNS<P0>(&self, namespaceuri: P0, localname: &windows_core::HSTRING) -> windows_core::Result<windows_core::HSTRING>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAttributeNS)(windows_core::Interface::as_raw(this), namespaceuri.param().abi(), core::mem::transmute_copy(localname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RemoveAttributeNS<P0>(&self, namespaceuri: P0, localname: &windows_core::HSTRING) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveAttributeNS)(windows_core::Interface::as_raw(this), namespaceuri.param().abi(), core::mem::transmute_copy(localname)).ok() }
    }
    pub fn SetAttributeNodeNS<P0>(&self, newattribute: P0) -> windows_core::Result<XmlAttribute>
    where
        P0: windows_core::Param<XmlAttribute>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SetAttributeNodeNS)(windows_core::Interface::as_raw(this), newattribute.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetAttributeNodeNS<P0>(&self, namespaceuri: P0, localname: &windows_core::HSTRING) -> windows_core::Result<XmlAttribute>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAttributeNodeNS)(windows_core::Interface::as_raw(this), namespaceuri.param().abi(), core::mem::transmute_copy(localname), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn NodeValue(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NodeValue)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetNodeValue<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetNodeValue)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn NodeType(&self) -> windows_core::Result<NodeType> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NodeType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn NodeName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NodeName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ParentNode(&self) -> windows_core::Result<IXmlNode> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ParentNode)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ChildNodes(&self) -> windows_core::Result<XmlNodeList> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ChildNodes)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn FirstChild(&self) -> windows_core::Result<IXmlNode> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FirstChild)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn LastChild(&self) -> windows_core::Result<IXmlNode> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LastChild)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn PreviousSibling(&self) -> windows_core::Result<IXmlNode> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousSibling)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn NextSibling(&self) -> windows_core::Result<IXmlNode> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NextSibling)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Attributes(&self) -> windows_core::Result<XmlNamedNodeMap> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Attributes)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn HasChildNodes(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HasChildNodes)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn OwnerDocument(&self) -> windows_core::Result<XmlDocument> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OwnerDocument)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn InsertBefore<P0, P1>(&self, newchild: P0, referencechild: P1) -> windows_core::Result<IXmlNode>
    where
        P0: windows_core::Param<IXmlNode>,
        P1: windows_core::Param<IXmlNode>,
    {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InsertBefore)(windows_core::Interface::as_raw(this), newchild.param().abi(), referencechild.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ReplaceChild<P0, P1>(&self, newchild: P0, referencechild: P1) -> windows_core::Result<IXmlNode>
    where
        P0: windows_core::Param<IXmlNode>,
        P1: windows_core::Param<IXmlNode>,
    {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReplaceChild)(windows_core::Interface::as_raw(this), newchild.param().abi(), referencechild.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RemoveChild<P0>(&self, childnode: P0) -> windows_core::Result<IXmlNode>
    where
        P0: windows_core::Param<IXmlNode>,
    {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RemoveChild)(windows_core::Interface::as_raw(this), childnode.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AppendChild<P0>(&self, newchild: P0) -> windows_core::Result<IXmlNode>
    where
        P0: windows_core::Param<IXmlNode>,
    {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AppendChild)(windows_core::Interface::as_raw(this), newchild.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CloneNode(&self, deep: bool) -> windows_core::Result<IXmlNode> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CloneNode)(windows_core::Interface::as_raw(this), deep, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn NamespaceUri(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NamespaceUri)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn LocalName(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LocalName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Prefix(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Prefix)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Normalize(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Normalize)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn SetPrefix<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetPrefix)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn SelectSingleNode(&self, xpath: &windows_core::HSTRING) -> windows_core::Result<IXmlNode> {
        let this = &windows_core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SelectSingleNode)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(xpath), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SelectNodes(&self, xpath: &windows_core::HSTRING) -> windows_core::Result<XmlNodeList> {
        let this = &windows_core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SelectNodes)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(xpath), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SelectSingleNodeNS<P0>(&self, xpath: &windows_core::HSTRING, namespaces: P0) -> windows_core::Result<IXmlNode>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SelectSingleNodeNS)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(xpath), namespaces.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SelectNodesNS<P0>(&self, xpath: &windows_core::HSTRING, namespaces: P0) -> windows_core::Result<XmlNodeList>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SelectNodesNS)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(xpath), namespaces.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetXml(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetXml)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn InnerText(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InnerText)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetInnerText(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetInnerText)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
}
impl windows_core::RuntimeType for XmlElement {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IXmlElement>();
}
unsafe impl windows_core::Interface for XmlElement {
    type Vtable = IXmlElement_Vtbl;
    const IID: windows_core::GUID = <IXmlElement as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for XmlElement {
    const NAME: &'static str = "Windows.Data.Xml.Dom.XmlElement";
}
unsafe impl Send for XmlElement {}
unsafe impl Sync for XmlElement {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct XmlEntityReference(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(XmlEntityReference, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(XmlEntityReference, IXmlNode, IXmlNodeSelector, IXmlNodeSerializer);
impl XmlEntityReference {
    pub fn NodeValue(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NodeValue)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetNodeValue<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetNodeValue)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn NodeType(&self) -> windows_core::Result<NodeType> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NodeType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn NodeName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NodeName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ParentNode(&self) -> windows_core::Result<IXmlNode> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ParentNode)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ChildNodes(&self) -> windows_core::Result<XmlNodeList> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ChildNodes)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn FirstChild(&self) -> windows_core::Result<IXmlNode> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FirstChild)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn LastChild(&self) -> windows_core::Result<IXmlNode> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LastChild)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn PreviousSibling(&self) -> windows_core::Result<IXmlNode> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousSibling)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn NextSibling(&self) -> windows_core::Result<IXmlNode> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NextSibling)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Attributes(&self) -> windows_core::Result<XmlNamedNodeMap> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Attributes)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn HasChildNodes(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HasChildNodes)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn OwnerDocument(&self) -> windows_core::Result<XmlDocument> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OwnerDocument)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn InsertBefore<P0, P1>(&self, newchild: P0, referencechild: P1) -> windows_core::Result<IXmlNode>
    where
        P0: windows_core::Param<IXmlNode>,
        P1: windows_core::Param<IXmlNode>,
    {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InsertBefore)(windows_core::Interface::as_raw(this), newchild.param().abi(), referencechild.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ReplaceChild<P0, P1>(&self, newchild: P0, referencechild: P1) -> windows_core::Result<IXmlNode>
    where
        P0: windows_core::Param<IXmlNode>,
        P1: windows_core::Param<IXmlNode>,
    {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReplaceChild)(windows_core::Interface::as_raw(this), newchild.param().abi(), referencechild.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RemoveChild<P0>(&self, childnode: P0) -> windows_core::Result<IXmlNode>
    where
        P0: windows_core::Param<IXmlNode>,
    {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RemoveChild)(windows_core::Interface::as_raw(this), childnode.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AppendChild<P0>(&self, newchild: P0) -> windows_core::Result<IXmlNode>
    where
        P0: windows_core::Param<IXmlNode>,
    {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AppendChild)(windows_core::Interface::as_raw(this), newchild.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CloneNode(&self, deep: bool) -> windows_core::Result<IXmlNode> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CloneNode)(windows_core::Interface::as_raw(this), deep, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn NamespaceUri(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NamespaceUri)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn LocalName(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LocalName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Prefix(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Prefix)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Normalize(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Normalize)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn SetPrefix<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetPrefix)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn SelectSingleNode(&self, xpath: &windows_core::HSTRING) -> windows_core::Result<IXmlNode> {
        let this = &windows_core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SelectSingleNode)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(xpath), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SelectNodes(&self, xpath: &windows_core::HSTRING) -> windows_core::Result<XmlNodeList> {
        let this = &windows_core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SelectNodes)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(xpath), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SelectSingleNodeNS<P0>(&self, xpath: &windows_core::HSTRING, namespaces: P0) -> windows_core::Result<IXmlNode>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SelectSingleNodeNS)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(xpath), namespaces.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SelectNodesNS<P0>(&self, xpath: &windows_core::HSTRING, namespaces: P0) -> windows_core::Result<XmlNodeList>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SelectNodesNS)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(xpath), namespaces.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetXml(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetXml)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn InnerText(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InnerText)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetInnerText(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetInnerText)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
}
impl windows_core::RuntimeType for XmlEntityReference {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IXmlEntityReference>();
}
unsafe impl windows_core::Interface for XmlEntityReference {
    type Vtable = IXmlEntityReference_Vtbl;
    const IID: windows_core::GUID = <IXmlEntityReference as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for XmlEntityReference {
    const NAME: &'static str = "Windows.Data.Xml.Dom.XmlEntityReference";
}
unsafe impl Send for XmlEntityReference {}
unsafe impl Sync for XmlEntityReference {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct XmlLoadSettings(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(XmlLoadSettings, windows_core::IUnknown, windows_core::IInspectable);
impl XmlLoadSettings {
    pub fn new() -> windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&windows_core::imp::IGenericFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<XmlLoadSettings, windows_core::imp::IGenericFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn MaxElementDepth(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxElementDepth)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetMaxElementDepth(&self, value: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetMaxElementDepth)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ProhibitDtd(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ProhibitDtd)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetProhibitDtd(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetProhibitDtd)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ResolveExternals(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ResolveExternals)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetResolveExternals(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetResolveExternals)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ValidateOnParse(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ValidateOnParse)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetValidateOnParse(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetValidateOnParse)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn ElementContentWhiteSpace(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ElementContentWhiteSpace)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetElementContentWhiteSpace(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetElementContentWhiteSpace)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for XmlLoadSettings {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IXmlLoadSettings>();
}
unsafe impl windows_core::Interface for XmlLoadSettings {
    type Vtable = IXmlLoadSettings_Vtbl;
    const IID: windows_core::GUID = <IXmlLoadSettings as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for XmlLoadSettings {
    const NAME: &'static str = "Windows.Data.Xml.Dom.XmlLoadSettings";
}
unsafe impl Send for XmlLoadSettings {}
unsafe impl Sync for XmlLoadSettings {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct XmlNamedNodeMap(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(XmlNamedNodeMap, windows_core::IUnknown, windows_core::IInspectable);
#[cfg(feature = "Foundation_Collections")]
windows_core::imp::required_hierarchy!(XmlNamedNodeMap, super::super::super::Foundation::Collections::IIterable::<IXmlNode>, super::super::super::Foundation::Collections::IVectorView::<IXmlNode>);
impl XmlNamedNodeMap {
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IIterator<IXmlNode>> {
        let this = &windows_core::Interface::cast::<super::super::super::Foundation::Collections::IIterable<IXmlNode>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).First)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAt(&self, index: u32) -> windows_core::Result<IXmlNode> {
        let this = &windows_core::Interface::cast::<super::super::super::Foundation::Collections::IVectorView<IXmlNode>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAt)(windows_core::Interface::as_raw(this), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<super::super::super::Foundation::Collections::IVectorView<IXmlNode>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Size)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn IndexOf<P0>(&self, value: P0, index: &mut u32) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<IXmlNode>,
    {
        let this = &windows_core::Interface::cast::<super::super::super::Foundation::Collections::IVectorView<IXmlNode>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IndexOf)(windows_core::Interface::as_raw(this), value.param().abi(), index, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMany(&self, startindex: u32, items: &mut [Option<IXmlNode>]) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<super::super::super::Foundation::Collections::IVectorView<IXmlNode>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetMany)(windows_core::Interface::as_raw(this), startindex, items.len().try_into().unwrap(), core::mem::transmute_copy(&items), &mut result__).map(|| result__)
        }
    }
    pub fn Length(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Length)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Item(&self, index: u32) -> windows_core::Result<IXmlNode> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Item)(windows_core::Interface::as_raw(this), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetNamedItem(&self, name: &windows_core::HSTRING) -> windows_core::Result<IXmlNode> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetNamedItem)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetNamedItem<P0>(&self, node: P0) -> windows_core::Result<IXmlNode>
    where
        P0: windows_core::Param<IXmlNode>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SetNamedItem)(windows_core::Interface::as_raw(this), node.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RemoveNamedItem(&self, name: &windows_core::HSTRING) -> windows_core::Result<IXmlNode> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RemoveNamedItem)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetNamedItemNS<P0>(&self, namespaceuri: P0, name: &windows_core::HSTRING) -> windows_core::Result<IXmlNode>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetNamedItemNS)(windows_core::Interface::as_raw(this), namespaceuri.param().abi(), core::mem::transmute_copy(name), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RemoveNamedItemNS<P0>(&self, namespaceuri: P0, name: &windows_core::HSTRING) -> windows_core::Result<IXmlNode>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RemoveNamedItemNS)(windows_core::Interface::as_raw(this), namespaceuri.param().abi(), core::mem::transmute_copy(name), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetNamedItemNS<P0>(&self, node: P0) -> windows_core::Result<IXmlNode>
    where
        P0: windows_core::Param<IXmlNode>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SetNamedItemNS)(windows_core::Interface::as_raw(this), node.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for XmlNamedNodeMap {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IXmlNamedNodeMap>();
}
unsafe impl windows_core::Interface for XmlNamedNodeMap {
    type Vtable = IXmlNamedNodeMap_Vtbl;
    const IID: windows_core::GUID = <IXmlNamedNodeMap as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for XmlNamedNodeMap {
    const NAME: &'static str = "Windows.Data.Xml.Dom.XmlNamedNodeMap";
}
#[cfg(feature = "Foundation_Collections")]
impl IntoIterator for XmlNamedNodeMap {
    type Item = IXmlNode;
    type IntoIter = super::super::super::Foundation::Collections::VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl IntoIterator for &XmlNamedNodeMap {
    type Item = IXmlNode;
    type IntoIter = super::super::super::Foundation::Collections::VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        super::super::super::Foundation::Collections::VectorViewIterator::new(windows_core::Interface::cast(self).ok())
    }
}
unsafe impl Send for XmlNamedNodeMap {}
unsafe impl Sync for XmlNamedNodeMap {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct XmlNodeList(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(XmlNodeList, windows_core::IUnknown, windows_core::IInspectable);
#[cfg(feature = "Foundation_Collections")]
windows_core::imp::required_hierarchy!(XmlNodeList, super::super::super::Foundation::Collections::IIterable::<IXmlNode>, super::super::super::Foundation::Collections::IVectorView::<IXmlNode>);
impl XmlNodeList {
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> windows_core::Result<super::super::super::Foundation::Collections::IIterator<IXmlNode>> {
        let this = &windows_core::Interface::cast::<super::super::super::Foundation::Collections::IIterable<IXmlNode>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).First)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAt(&self, index: u32) -> windows_core::Result<IXmlNode> {
        let this = &windows_core::Interface::cast::<super::super::super::Foundation::Collections::IVectorView<IXmlNode>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetAt)(windows_core::Interface::as_raw(this), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<super::super::super::Foundation::Collections::IVectorView<IXmlNode>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Size)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn IndexOf<P0>(&self, value: P0, index: &mut u32) -> windows_core::Result<bool>
    where
        P0: windows_core::Param<IXmlNode>,
    {
        let this = &windows_core::Interface::cast::<super::super::super::Foundation::Collections::IVectorView<IXmlNode>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IndexOf)(windows_core::Interface::as_raw(this), value.param().abi(), index, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMany(&self, startindex: u32, items: &mut [Option<IXmlNode>]) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<super::super::super::Foundation::Collections::IVectorView<IXmlNode>>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetMany)(windows_core::Interface::as_raw(this), startindex, items.len().try_into().unwrap(), core::mem::transmute_copy(&items), &mut result__).map(|| result__)
        }
    }
    pub fn Length(&self) -> windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Length)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Item(&self, index: u32) -> windows_core::Result<IXmlNode> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Item)(windows_core::Interface::as_raw(this), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for XmlNodeList {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IXmlNodeList>();
}
unsafe impl windows_core::Interface for XmlNodeList {
    type Vtable = IXmlNodeList_Vtbl;
    const IID: windows_core::GUID = <IXmlNodeList as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for XmlNodeList {
    const NAME: &'static str = "Windows.Data.Xml.Dom.XmlNodeList";
}
#[cfg(feature = "Foundation_Collections")]
impl IntoIterator for XmlNodeList {
    type Item = IXmlNode;
    type IntoIter = super::super::super::Foundation::Collections::VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl IntoIterator for &XmlNodeList {
    type Item = IXmlNode;
    type IntoIter = super::super::super::Foundation::Collections::VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        super::super::super::Foundation::Collections::VectorViewIterator::new(windows_core::Interface::cast(self).ok())
    }
}
unsafe impl Send for XmlNodeList {}
unsafe impl Sync for XmlNodeList {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct XmlProcessingInstruction(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(XmlProcessingInstruction, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(XmlProcessingInstruction, IXmlNode, IXmlNodeSelector, IXmlNodeSerializer);
impl XmlProcessingInstruction {
    pub fn NodeValue(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NodeValue)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetNodeValue<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetNodeValue)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn NodeType(&self) -> windows_core::Result<NodeType> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NodeType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn NodeName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NodeName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ParentNode(&self) -> windows_core::Result<IXmlNode> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ParentNode)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ChildNodes(&self) -> windows_core::Result<XmlNodeList> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ChildNodes)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn FirstChild(&self) -> windows_core::Result<IXmlNode> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FirstChild)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn LastChild(&self) -> windows_core::Result<IXmlNode> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LastChild)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn PreviousSibling(&self) -> windows_core::Result<IXmlNode> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousSibling)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn NextSibling(&self) -> windows_core::Result<IXmlNode> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NextSibling)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Attributes(&self) -> windows_core::Result<XmlNamedNodeMap> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Attributes)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn HasChildNodes(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HasChildNodes)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn OwnerDocument(&self) -> windows_core::Result<XmlDocument> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OwnerDocument)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn InsertBefore<P0, P1>(&self, newchild: P0, referencechild: P1) -> windows_core::Result<IXmlNode>
    where
        P0: windows_core::Param<IXmlNode>,
        P1: windows_core::Param<IXmlNode>,
    {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InsertBefore)(windows_core::Interface::as_raw(this), newchild.param().abi(), referencechild.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ReplaceChild<P0, P1>(&self, newchild: P0, referencechild: P1) -> windows_core::Result<IXmlNode>
    where
        P0: windows_core::Param<IXmlNode>,
        P1: windows_core::Param<IXmlNode>,
    {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReplaceChild)(windows_core::Interface::as_raw(this), newchild.param().abi(), referencechild.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RemoveChild<P0>(&self, childnode: P0) -> windows_core::Result<IXmlNode>
    where
        P0: windows_core::Param<IXmlNode>,
    {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RemoveChild)(windows_core::Interface::as_raw(this), childnode.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AppendChild<P0>(&self, newchild: P0) -> windows_core::Result<IXmlNode>
    where
        P0: windows_core::Param<IXmlNode>,
    {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AppendChild)(windows_core::Interface::as_raw(this), newchild.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CloneNode(&self, deep: bool) -> windows_core::Result<IXmlNode> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CloneNode)(windows_core::Interface::as_raw(this), deep, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn NamespaceUri(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NamespaceUri)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn LocalName(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LocalName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Prefix(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Prefix)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Normalize(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Normalize)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn SetPrefix<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetPrefix)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn SelectSingleNode(&self, xpath: &windows_core::HSTRING) -> windows_core::Result<IXmlNode> {
        let this = &windows_core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SelectSingleNode)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(xpath), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SelectNodes(&self, xpath: &windows_core::HSTRING) -> windows_core::Result<XmlNodeList> {
        let this = &windows_core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SelectNodes)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(xpath), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SelectSingleNodeNS<P0>(&self, xpath: &windows_core::HSTRING, namespaces: P0) -> windows_core::Result<IXmlNode>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SelectSingleNodeNS)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(xpath), namespaces.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SelectNodesNS<P0>(&self, xpath: &windows_core::HSTRING, namespaces: P0) -> windows_core::Result<XmlNodeList>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SelectNodesNS)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(xpath), namespaces.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetXml(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetXml)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn InnerText(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InnerText)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetInnerText(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetInnerText)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn Target(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Target)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Data(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Data)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetData(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetData)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
}
impl windows_core::RuntimeType for XmlProcessingInstruction {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IXmlProcessingInstruction>();
}
unsafe impl windows_core::Interface for XmlProcessingInstruction {
    type Vtable = IXmlProcessingInstruction_Vtbl;
    const IID: windows_core::GUID = <IXmlProcessingInstruction as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for XmlProcessingInstruction {
    const NAME: &'static str = "Windows.Data.Xml.Dom.XmlProcessingInstruction";
}
unsafe impl Send for XmlProcessingInstruction {}
unsafe impl Sync for XmlProcessingInstruction {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct XmlText(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(XmlText, windows_core::IUnknown, windows_core::IInspectable);
windows_core::imp::required_hierarchy!(XmlText, IXmlCharacterData, IXmlNode, IXmlNodeSelector, IXmlNodeSerializer, IXmlText);
impl XmlText {
    pub fn Data(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Data)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetData(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetData)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn Length(&self) -> windows_core::Result<u32> {
        let this = &windows_core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Length)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SubstringData(&self, offset: u32, count: u32) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SubstringData)(windows_core::Interface::as_raw(this), offset, count, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AppendData(&self, data: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe { (windows_core::Interface::vtable(this).AppendData)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(data)).ok() }
    }
    pub fn InsertData(&self, offset: u32, data: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe { (windows_core::Interface::vtable(this).InsertData)(windows_core::Interface::as_raw(this), offset, core::mem::transmute_copy(data)).ok() }
    }
    pub fn DeleteData(&self, offset: u32, count: u32) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe { (windows_core::Interface::vtable(this).DeleteData)(windows_core::Interface::as_raw(this), offset, count).ok() }
    }
    pub fn ReplaceData(&self, offset: u32, count: u32, data: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IXmlCharacterData>(self)?;
        unsafe { (windows_core::Interface::vtable(this).ReplaceData)(windows_core::Interface::as_raw(this), offset, count, core::mem::transmute_copy(data)).ok() }
    }
    pub fn NodeValue(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NodeValue)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetNodeValue<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetNodeValue)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn NodeType(&self) -> windows_core::Result<NodeType> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NodeType)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn NodeName(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NodeName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ParentNode(&self) -> windows_core::Result<IXmlNode> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ParentNode)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ChildNodes(&self) -> windows_core::Result<XmlNodeList> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ChildNodes)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn FirstChild(&self) -> windows_core::Result<IXmlNode> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FirstChild)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn LastChild(&self) -> windows_core::Result<IXmlNode> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LastChild)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn PreviousSibling(&self) -> windows_core::Result<IXmlNode> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PreviousSibling)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn NextSibling(&self) -> windows_core::Result<IXmlNode> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NextSibling)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Attributes(&self) -> windows_core::Result<XmlNamedNodeMap> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Attributes)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn HasChildNodes(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HasChildNodes)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn OwnerDocument(&self) -> windows_core::Result<XmlDocument> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OwnerDocument)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn InsertBefore<P0, P1>(&self, newchild: P0, referencechild: P1) -> windows_core::Result<IXmlNode>
    where
        P0: windows_core::Param<IXmlNode>,
        P1: windows_core::Param<IXmlNode>,
    {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InsertBefore)(windows_core::Interface::as_raw(this), newchild.param().abi(), referencechild.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn ReplaceChild<P0, P1>(&self, newchild: P0, referencechild: P1) -> windows_core::Result<IXmlNode>
    where
        P0: windows_core::Param<IXmlNode>,
        P1: windows_core::Param<IXmlNode>,
    {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).ReplaceChild)(windows_core::Interface::as_raw(this), newchild.param().abi(), referencechild.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn RemoveChild<P0>(&self, childnode: P0) -> windows_core::Result<IXmlNode>
    where
        P0: windows_core::Param<IXmlNode>,
    {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RemoveChild)(windows_core::Interface::as_raw(this), childnode.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn AppendChild<P0>(&self, newchild: P0) -> windows_core::Result<IXmlNode>
    where
        P0: windows_core::Param<IXmlNode>,
    {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AppendChild)(windows_core::Interface::as_raw(this), newchild.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn CloneNode(&self, deep: bool) -> windows_core::Result<IXmlNode> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CloneNode)(windows_core::Interface::as_raw(this), deep, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn NamespaceUri(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).NamespaceUri)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn LocalName(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).LocalName)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Prefix(&self) -> windows_core::Result<windows_core::IInspectable> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Prefix)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Normalize(&self) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).Normalize)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn SetPrefix<P0>(&self, value: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<IXmlNode>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetPrefix)(windows_core::Interface::as_raw(this), value.param().abi()).ok() }
    }
    pub fn SelectSingleNode(&self, xpath: &windows_core::HSTRING) -> windows_core::Result<IXmlNode> {
        let this = &windows_core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SelectSingleNode)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(xpath), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SelectNodes(&self, xpath: &windows_core::HSTRING) -> windows_core::Result<XmlNodeList> {
        let this = &windows_core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SelectNodes)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(xpath), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SelectSingleNodeNS<P0>(&self, xpath: &windows_core::HSTRING, namespaces: P0) -> windows_core::Result<IXmlNode>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SelectSingleNodeNS)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(xpath), namespaces.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SelectNodesNS<P0>(&self, xpath: &windows_core::HSTRING, namespaces: P0) -> windows_core::Result<XmlNodeList>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        let this = &windows_core::Interface::cast::<IXmlNodeSelector>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SelectNodesNS)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(xpath), namespaces.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn GetXml(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetXml)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn InnerText(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).InnerText)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn SetInnerText(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<IXmlNodeSerializer>(self)?;
        unsafe { (windows_core::Interface::vtable(this).SetInnerText)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(value)).ok() }
    }
    pub fn SplitText(&self, offset: u32) -> windows_core::Result<IXmlText> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SplitText)(windows_core::Interface::as_raw(this), offset, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for XmlText {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IXmlText>();
}
unsafe impl windows_core::Interface for XmlText {
    type Vtable = IXmlText_Vtbl;
    const IID: windows_core::GUID = <IXmlText as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for XmlText {
    const NAME: &'static str = "Windows.Data.Xml.Dom.XmlText";
}
unsafe impl Send for XmlText {}
unsafe impl Sync for XmlText {}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct NodeType(pub i32);
impl NodeType {
    pub const Invalid: Self = Self(0i32);
    pub const ElementNode: Self = Self(1i32);
    pub const AttributeNode: Self = Self(2i32);
    pub const TextNode: Self = Self(3i32);
    pub const DataSectionNode: Self = Self(4i32);
    pub const EntityReferenceNode: Self = Self(5i32);
    pub const EntityNode: Self = Self(6i32);
    pub const ProcessingInstructionNode: Self = Self(7i32);
    pub const CommentNode: Self = Self(8i32);
    pub const DocumentNode: Self = Self(9i32);
    pub const DocumentTypeNode: Self = Self(10i32);
    pub const DocumentFragmentNode: Self = Self(11i32);
    pub const NotationNode: Self = Self(12i32);
}
impl windows_core::TypeKind for NodeType {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for NodeType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("NodeType").field(&self.0).finish()
    }
}
impl windows_core::RuntimeType for NodeType {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Data.Xml.Dom.NodeType;i4)");
}
#[cfg(feature = "implement")]
core::include!("impl.rs");
