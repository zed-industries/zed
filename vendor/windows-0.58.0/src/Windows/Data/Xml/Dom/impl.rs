pub trait IXmlCharacterData_Impl: Sized + IXmlNode_Impl + IXmlNodeSelector_Impl + IXmlNodeSerializer_Impl {
    fn Data(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn SetData(&self, value: &windows_core::HSTRING) -> windows_core::Result<()>;
    fn Length(&self) -> windows_core::Result<u32>;
    fn SubstringData(&self, offset: u32, count: u32) -> windows_core::Result<windows_core::HSTRING>;
    fn AppendData(&self, data: &windows_core::HSTRING) -> windows_core::Result<()>;
    fn InsertData(&self, offset: u32, data: &windows_core::HSTRING) -> windows_core::Result<()>;
    fn DeleteData(&self, offset: u32, count: u32) -> windows_core::Result<()>;
    fn ReplaceData(&self, offset: u32, count: u32, data: &windows_core::HSTRING) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IXmlCharacterData {
    const NAME: &'static str = "Windows.Data.Xml.Dom.IXmlCharacterData";
}
impl IXmlCharacterData_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IXmlCharacterData_Vtbl
    where
        Identity: IXmlCharacterData_Impl,
    {
        unsafe extern "system" fn Data<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT
        where
            Identity: IXmlCharacterData_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXmlCharacterData_Impl::Data(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT
        where
            Identity: IXmlCharacterData_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXmlCharacterData_Impl::SetData(this, core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn Length<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut u32) -> windows_core::HRESULT
        where
            Identity: IXmlCharacterData_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXmlCharacterData_Impl::Length(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SubstringData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, offset: u32, count: u32, result__: *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT
        where
            Identity: IXmlCharacterData_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXmlCharacterData_Impl::SubstringData(this, offset, count) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AppendData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, data: core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT
        where
            Identity: IXmlCharacterData_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXmlCharacterData_Impl::AppendData(this, core::mem::transmute(&data)).into()
        }
        unsafe extern "system" fn InsertData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, offset: u32, data: core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT
        where
            Identity: IXmlCharacterData_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXmlCharacterData_Impl::InsertData(this, offset, core::mem::transmute(&data)).into()
        }
        unsafe extern "system" fn DeleteData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, offset: u32, count: u32) -> windows_core::HRESULT
        where
            Identity: IXmlCharacterData_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXmlCharacterData_Impl::DeleteData(this, offset, count).into()
        }
        unsafe extern "system" fn ReplaceData<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, offset: u32, count: u32, data: core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT
        where
            Identity: IXmlCharacterData_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXmlCharacterData_Impl::ReplaceData(this, offset, count, core::mem::transmute(&data)).into()
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IXmlCharacterData, OFFSET>(),
            Data: Data::<Identity, OFFSET>,
            SetData: SetData::<Identity, OFFSET>,
            Length: Length::<Identity, OFFSET>,
            SubstringData: SubstringData::<Identity, OFFSET>,
            AppendData: AppendData::<Identity, OFFSET>,
            InsertData: InsertData::<Identity, OFFSET>,
            DeleteData: DeleteData::<Identity, OFFSET>,
            ReplaceData: ReplaceData::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXmlCharacterData as windows_core::Interface>::IID
    }
}
pub trait IXmlNode_Impl: Sized + IXmlNodeSelector_Impl + IXmlNodeSerializer_Impl {
    fn NodeValue(&self) -> windows_core::Result<windows_core::IInspectable>;
    fn SetNodeValue(&self, value: Option<&windows_core::IInspectable>) -> windows_core::Result<()>;
    fn NodeType(&self) -> windows_core::Result<NodeType>;
    fn NodeName(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn ParentNode(&self) -> windows_core::Result<IXmlNode>;
    fn ChildNodes(&self) -> windows_core::Result<XmlNodeList>;
    fn FirstChild(&self) -> windows_core::Result<IXmlNode>;
    fn LastChild(&self) -> windows_core::Result<IXmlNode>;
    fn PreviousSibling(&self) -> windows_core::Result<IXmlNode>;
    fn NextSibling(&self) -> windows_core::Result<IXmlNode>;
    fn Attributes(&self) -> windows_core::Result<XmlNamedNodeMap>;
    fn HasChildNodes(&self) -> windows_core::Result<bool>;
    fn OwnerDocument(&self) -> windows_core::Result<XmlDocument>;
    fn InsertBefore(&self, newchild: Option<&IXmlNode>, referencechild: Option<&IXmlNode>) -> windows_core::Result<IXmlNode>;
    fn ReplaceChild(&self, newchild: Option<&IXmlNode>, referencechild: Option<&IXmlNode>) -> windows_core::Result<IXmlNode>;
    fn RemoveChild(&self, childnode: Option<&IXmlNode>) -> windows_core::Result<IXmlNode>;
    fn AppendChild(&self, newchild: Option<&IXmlNode>) -> windows_core::Result<IXmlNode>;
    fn CloneNode(&self, deep: bool) -> windows_core::Result<IXmlNode>;
    fn NamespaceUri(&self) -> windows_core::Result<windows_core::IInspectable>;
    fn LocalName(&self) -> windows_core::Result<windows_core::IInspectable>;
    fn Prefix(&self) -> windows_core::Result<windows_core::IInspectable>;
    fn Normalize(&self) -> windows_core::Result<()>;
    fn SetPrefix(&self, value: Option<&windows_core::IInspectable>) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IXmlNode {
    const NAME: &'static str = "Windows.Data.Xml.Dom.IXmlNode";
}
impl IXmlNode_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IXmlNode_Vtbl
    where
        Identity: IXmlNode_Impl,
    {
        unsafe extern "system" fn NodeValue<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXmlNode_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXmlNode_Impl::NodeValue(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetNodeValue<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXmlNode_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXmlNode_Impl::SetNodeValue(this, windows_core::from_raw_borrowed(&value)).into()
        }
        unsafe extern "system" fn NodeType<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut NodeType) -> windows_core::HRESULT
        where
            Identity: IXmlNode_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXmlNode_Impl::NodeType(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NodeName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT
        where
            Identity: IXmlNode_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXmlNode_Impl::NodeName(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ParentNode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXmlNode_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXmlNode_Impl::ParentNode(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ChildNodes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXmlNode_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXmlNode_Impl::ChildNodes(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FirstChild<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXmlNode_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXmlNode_Impl::FirstChild(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastChild<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXmlNode_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXmlNode_Impl::LastChild(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PreviousSibling<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXmlNode_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXmlNode_Impl::PreviousSibling(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NextSibling<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXmlNode_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXmlNode_Impl::NextSibling(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Attributes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXmlNode_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXmlNode_Impl::Attributes(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn HasChildNodes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut bool) -> windows_core::HRESULT
        where
            Identity: IXmlNode_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXmlNode_Impl::HasChildNodes(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OwnerDocument<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXmlNode_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXmlNode_Impl::OwnerDocument(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InsertBefore<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, newchild: *mut core::ffi::c_void, referencechild: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXmlNode_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXmlNode_Impl::InsertBefore(this, windows_core::from_raw_borrowed(&newchild), windows_core::from_raw_borrowed(&referencechild)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReplaceChild<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, newchild: *mut core::ffi::c_void, referencechild: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXmlNode_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXmlNode_Impl::ReplaceChild(this, windows_core::from_raw_borrowed(&newchild), windows_core::from_raw_borrowed(&referencechild)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveChild<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, childnode: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXmlNode_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXmlNode_Impl::RemoveChild(this, windows_core::from_raw_borrowed(&childnode)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AppendChild<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, newchild: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXmlNode_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXmlNode_Impl::AppendChild(this, windows_core::from_raw_borrowed(&newchild)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CloneNode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, deep: bool, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXmlNode_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXmlNode_Impl::CloneNode(this, deep) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NamespaceUri<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXmlNode_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXmlNode_Impl::NamespaceUri(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LocalName<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXmlNode_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXmlNode_Impl::LocalName(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Prefix<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXmlNode_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXmlNode_Impl::Prefix(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Normalize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXmlNode_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXmlNode_Impl::Normalize(this).into()
        }
        unsafe extern "system" fn SetPrefix<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXmlNode_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXmlNode_Impl::SetPrefix(this, windows_core::from_raw_borrowed(&value)).into()
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IXmlNode, OFFSET>(),
            NodeValue: NodeValue::<Identity, OFFSET>,
            SetNodeValue: SetNodeValue::<Identity, OFFSET>,
            NodeType: NodeType::<Identity, OFFSET>,
            NodeName: NodeName::<Identity, OFFSET>,
            ParentNode: ParentNode::<Identity, OFFSET>,
            ChildNodes: ChildNodes::<Identity, OFFSET>,
            FirstChild: FirstChild::<Identity, OFFSET>,
            LastChild: LastChild::<Identity, OFFSET>,
            PreviousSibling: PreviousSibling::<Identity, OFFSET>,
            NextSibling: NextSibling::<Identity, OFFSET>,
            Attributes: Attributes::<Identity, OFFSET>,
            HasChildNodes: HasChildNodes::<Identity, OFFSET>,
            OwnerDocument: OwnerDocument::<Identity, OFFSET>,
            InsertBefore: InsertBefore::<Identity, OFFSET>,
            ReplaceChild: ReplaceChild::<Identity, OFFSET>,
            RemoveChild: RemoveChild::<Identity, OFFSET>,
            AppendChild: AppendChild::<Identity, OFFSET>,
            CloneNode: CloneNode::<Identity, OFFSET>,
            NamespaceUri: NamespaceUri::<Identity, OFFSET>,
            LocalName: LocalName::<Identity, OFFSET>,
            Prefix: Prefix::<Identity, OFFSET>,
            Normalize: Normalize::<Identity, OFFSET>,
            SetPrefix: SetPrefix::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXmlNode as windows_core::Interface>::IID
    }
}
pub trait IXmlNodeSelector_Impl: Sized {
    fn SelectSingleNode(&self, xpath: &windows_core::HSTRING) -> windows_core::Result<IXmlNode>;
    fn SelectNodes(&self, xpath: &windows_core::HSTRING) -> windows_core::Result<XmlNodeList>;
    fn SelectSingleNodeNS(&self, xpath: &windows_core::HSTRING, namespaces: Option<&windows_core::IInspectable>) -> windows_core::Result<IXmlNode>;
    fn SelectNodesNS(&self, xpath: &windows_core::HSTRING, namespaces: Option<&windows_core::IInspectable>) -> windows_core::Result<XmlNodeList>;
}
impl windows_core::RuntimeName for IXmlNodeSelector {
    const NAME: &'static str = "Windows.Data.Xml.Dom.IXmlNodeSelector";
}
impl IXmlNodeSelector_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IXmlNodeSelector_Vtbl
    where
        Identity: IXmlNodeSelector_Impl,
    {
        unsafe extern "system" fn SelectSingleNode<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, xpath: core::mem::MaybeUninit<windows_core::HSTRING>, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXmlNodeSelector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXmlNodeSelector_Impl::SelectSingleNode(this, core::mem::transmute(&xpath)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectNodes<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, xpath: core::mem::MaybeUninit<windows_core::HSTRING>, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXmlNodeSelector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXmlNodeSelector_Impl::SelectNodes(this, core::mem::transmute(&xpath)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectSingleNodeNS<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, xpath: core::mem::MaybeUninit<windows_core::HSTRING>, namespaces: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXmlNodeSelector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXmlNodeSelector_Impl::SelectSingleNodeNS(this, core::mem::transmute(&xpath), windows_core::from_raw_borrowed(&namespaces)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SelectNodesNS<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, xpath: core::mem::MaybeUninit<windows_core::HSTRING>, namespaces: *mut core::ffi::c_void, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXmlNodeSelector_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXmlNodeSelector_Impl::SelectNodesNS(this, core::mem::transmute(&xpath), windows_core::from_raw_borrowed(&namespaces)) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IXmlNodeSelector, OFFSET>(),
            SelectSingleNode: SelectSingleNode::<Identity, OFFSET>,
            SelectNodes: SelectNodes::<Identity, OFFSET>,
            SelectSingleNodeNS: SelectSingleNodeNS::<Identity, OFFSET>,
            SelectNodesNS: SelectNodesNS::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXmlNodeSelector as windows_core::Interface>::IID
    }
}
pub trait IXmlNodeSerializer_Impl: Sized {
    fn GetXml(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn InnerText(&self) -> windows_core::Result<windows_core::HSTRING>;
    fn SetInnerText(&self, value: &windows_core::HSTRING) -> windows_core::Result<()>;
}
impl windows_core::RuntimeName for IXmlNodeSerializer {
    const NAME: &'static str = "Windows.Data.Xml.Dom.IXmlNodeSerializer";
}
impl IXmlNodeSerializer_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IXmlNodeSerializer_Vtbl
    where
        Identity: IXmlNodeSerializer_Impl,
    {
        unsafe extern "system" fn GetXml<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT
        where
            Identity: IXmlNodeSerializer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXmlNodeSerializer_Impl::GetXml(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InnerText<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT
        where
            Identity: IXmlNodeSerializer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXmlNodeSerializer_Impl::InnerText(this) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInnerText<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: core::mem::MaybeUninit<windows_core::HSTRING>) -> windows_core::HRESULT
        where
            Identity: IXmlNodeSerializer_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IXmlNodeSerializer_Impl::SetInnerText(this, core::mem::transmute(&value)).into()
        }
        Self {
            base__: windows_core::IInspectable_Vtbl::new::<Identity, IXmlNodeSerializer, OFFSET>(),
            GetXml: GetXml::<Identity, OFFSET>,
            InnerText: InnerText::<Identity, OFFSET>,
            SetInnerText: SetInnerText::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXmlNodeSerializer as windows_core::Interface>::IID
    }
}
pub trait IXmlText_Impl: Sized + IXmlCharacterData_Impl + IXmlNode_Impl + IXmlNodeSelector_Impl + IXmlNodeSerializer_Impl {
    fn SplitText(&self, offset: u32) -> windows_core::Result<IXmlText>;
}
impl windows_core::RuntimeName for IXmlText {
    const NAME: &'static str = "Windows.Data.Xml.Dom.IXmlText";
}
impl IXmlText_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IXmlText_Vtbl
    where
        Identity: IXmlText_Impl,
    {
        unsafe extern "system" fn SplitText<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, offset: u32, result__: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IXmlText_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IXmlText_Impl::SplitText(this, offset) {
                Ok(ok__) => {
                    result__.write(core::mem::transmute_copy(&ok__));
                    core::mem::forget(ok__);
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self { base__: windows_core::IInspectable_Vtbl::new::<Identity, IXmlText, OFFSET>(), SplitText: SplitText::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXmlText as windows_core::Interface>::IID
    }
}
