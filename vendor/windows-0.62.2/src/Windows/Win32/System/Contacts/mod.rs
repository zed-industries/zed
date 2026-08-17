pub const CACO_DEFAULT: CONTACT_AGGREGATION_COLLECTION_OPTIONS = CONTACT_AGGREGATION_COLLECTION_OPTIONS(0i32);
pub const CACO_EXTERNAL_ONLY: CONTACT_AGGREGATION_COLLECTION_OPTIONS = CONTACT_AGGREGATION_COLLECTION_OPTIONS(2i32);
pub const CACO_INCLUDE_EXTERNAL: CONTACT_AGGREGATION_COLLECTION_OPTIONS = CONTACT_AGGREGATION_COLLECTION_OPTIONS(1i32);
pub const CA_CREATE_EXTERNAL: CONTACT_AGGREGATION_CREATE_OR_OPEN_OPTIONS = CONTACT_AGGREGATION_CREATE_OR_OPEN_OPTIONS(1i32);
pub const CA_CREATE_LOCAL: CONTACT_AGGREGATION_CREATE_OR_OPEN_OPTIONS = CONTACT_AGGREGATION_CREATE_OR_OPEN_OPTIONS(0i32);
pub const CGD_ARRAY_NODE: u32 = 8u32;
pub const CGD_BINARY_PROPERTY: u32 = 4u32;
pub const CGD_DATE_PROPERTY: u32 = 2u32;
pub const CGD_DEFAULT: u32 = 0u32;
pub const CGD_STRING_PROPERTY: u32 = 1u32;
pub const CGD_UNKNOWN_PROPERTY: u32 = 0u32;
pub const CLSID_ContactAggregationManager: windows_core::GUID = windows_core::GUID::from_u128(0x96c8ad95_c199_44de_b34e_ac33c442df39);
pub const CONTACTLABEL_PUB_AGENT: windows_core::PCWSTR = windows_core::w!("Agent");
pub const CONTACTLABEL_PUB_BBS: windows_core::PCWSTR = windows_core::w!("BBS");
pub const CONTACTLABEL_PUB_BUSINESS: windows_core::PCWSTR = windows_core::w!("Business");
pub const CONTACTLABEL_PUB_CAR: windows_core::PCWSTR = windows_core::w!("Car");
pub const CONTACTLABEL_PUB_CELLULAR: windows_core::PCWSTR = windows_core::w!("Cellular");
pub const CONTACTLABEL_PUB_DOMESTIC: windows_core::PCWSTR = windows_core::w!("Domestic");
pub const CONTACTLABEL_PUB_FAX: windows_core::PCWSTR = windows_core::w!("Fax");
pub const CONTACTLABEL_PUB_INTERNATIONAL: windows_core::PCWSTR = windows_core::w!("International");
pub const CONTACTLABEL_PUB_ISDN: windows_core::PCWSTR = windows_core::w!("ISDN");
pub const CONTACTLABEL_PUB_LOGO: windows_core::PCWSTR = windows_core::w!("Logo");
pub const CONTACTLABEL_PUB_MOBILE: windows_core::PCWSTR = windows_core::w!("Mobile");
pub const CONTACTLABEL_PUB_MODEM: windows_core::PCWSTR = windows_core::w!("Modem");
pub const CONTACTLABEL_PUB_OTHER: windows_core::PCWSTR = windows_core::w!("Other");
pub const CONTACTLABEL_PUB_PAGER: windows_core::PCWSTR = windows_core::w!("Pager");
pub const CONTACTLABEL_PUB_PARCEL: windows_core::PCWSTR = windows_core::w!("Parcel");
pub const CONTACTLABEL_PUB_PCS: windows_core::PCWSTR = windows_core::w!("PCS");
pub const CONTACTLABEL_PUB_PERSONAL: windows_core::PCWSTR = windows_core::w!("Personal");
pub const CONTACTLABEL_PUB_POSTAL: windows_core::PCWSTR = windows_core::w!("Postal");
pub const CONTACTLABEL_PUB_PREFERRED: windows_core::PCWSTR = windows_core::w!("Preferred");
pub const CONTACTLABEL_PUB_TTY: windows_core::PCWSTR = windows_core::w!("TTY");
pub const CONTACTLABEL_PUB_USERTILE: windows_core::PCWSTR = windows_core::w!("UserTile");
pub const CONTACTLABEL_PUB_VIDEO: windows_core::PCWSTR = windows_core::w!("Video");
pub const CONTACTLABEL_PUB_VOICE: windows_core::PCWSTR = windows_core::w!("Voice");
pub const CONTACTLABEL_WAB_ANNIVERSARY: windows_core::PCWSTR = windows_core::w!("wab:Anniversary");
pub const CONTACTLABEL_WAB_ASSISTANT: windows_core::PCWSTR = windows_core::w!("wab:Assistant");
pub const CONTACTLABEL_WAB_BIRTHDAY: windows_core::PCWSTR = windows_core::w!("wab:Birthday");
pub const CONTACTLABEL_WAB_CHILD: windows_core::PCWSTR = windows_core::w!("wab:Child");
pub const CONTACTLABEL_WAB_MANAGER: windows_core::PCWSTR = windows_core::w!("wab:Manager");
pub const CONTACTLABEL_WAB_SCHOOL: windows_core::PCWSTR = windows_core::w!("wab:School");
pub const CONTACTLABEL_WAB_SOCIALNETWORK: windows_core::PCWSTR = windows_core::w!("wab:SocialNetwork");
pub const CONTACTLABEL_WAB_SPOUSE: windows_core::PCWSTR = windows_core::w!("wab:Spouse");
pub const CONTACTLABEL_WAB_WISHLIST: windows_core::PCWSTR = windows_core::w!("wab:WishList");
pub const CONTACTPROP_PUB_CREATIONDATE: windows_core::PCWSTR = windows_core::w!("CreationDate");
pub const CONTACTPROP_PUB_GENDER: windows_core::PCWSTR = windows_core::w!("Gender");
pub const CONTACTPROP_PUB_GENDER_FEMALE: windows_core::PCWSTR = windows_core::w!("Female");
pub const CONTACTPROP_PUB_GENDER_MALE: windows_core::PCWSTR = windows_core::w!("Male");
pub const CONTACTPROP_PUB_GENDER_UNSPECIFIED: windows_core::PCWSTR = windows_core::w!("Unspecified");
pub const CONTACTPROP_PUB_L1_CERTIFICATECOLLECTION: windows_core::PCWSTR = windows_core::w!("CertificateCollection");
pub const CONTACTPROP_PUB_L1_CONTACTIDCOLLECTION: windows_core::PCWSTR = windows_core::w!("ContactIDCollection");
pub const CONTACTPROP_PUB_L1_DATECOLLECTION: windows_core::PCWSTR = windows_core::w!("DateCollection");
pub const CONTACTPROP_PUB_L1_EMAILADDRESSCOLLECTION: windows_core::PCWSTR = windows_core::w!("EmailAddressCollection");
pub const CONTACTPROP_PUB_L1_IMADDRESSCOLLECTION: windows_core::PCWSTR = windows_core::w!("IMAddressCollection");
pub const CONTACTPROP_PUB_L1_NAMECOLLECTION: windows_core::PCWSTR = windows_core::w!("NameCollection");
pub const CONTACTPROP_PUB_L1_PERSONCOLLECTION: windows_core::PCWSTR = windows_core::w!("PersonCollection");
pub const CONTACTPROP_PUB_L1_PHONENUMBERCOLLECTION: windows_core::PCWSTR = windows_core::w!("PhoneNumberCollection");
pub const CONTACTPROP_PUB_L1_PHOTOCOLLECTION: windows_core::PCWSTR = windows_core::w!("PhotoCollection");
pub const CONTACTPROP_PUB_L1_PHYSICALADDRESSCOLLECTION: windows_core::PCWSTR = windows_core::w!("PhysicalAddressCollection");
pub const CONTACTPROP_PUB_L1_POSITIONCOLLECTION: windows_core::PCWSTR = windows_core::w!("PositionCollection");
pub const CONTACTPROP_PUB_L1_URLCOLLECTION: windows_core::PCWSTR = windows_core::w!("UrlCollection");
pub const CONTACTPROP_PUB_L2_CERTIFICATE: windows_core::PCWSTR = windows_core::w!("/Certificate");
pub const CONTACTPROP_PUB_L2_CONTACTID: windows_core::PCWSTR = windows_core::w!("/ContactID");
pub const CONTACTPROP_PUB_L2_DATE: windows_core::PCWSTR = windows_core::w!("/Date");
pub const CONTACTPROP_PUB_L2_EMAILADDRESS: windows_core::PCWSTR = windows_core::w!("/EmailAddress");
pub const CONTACTPROP_PUB_L2_IMADDRESSENTRY: windows_core::PCWSTR = windows_core::w!("/IMAddress");
pub const CONTACTPROP_PUB_L2_NAME: windows_core::PCWSTR = windows_core::w!("/Name");
pub const CONTACTPROP_PUB_L2_PERSON: windows_core::PCWSTR = windows_core::w!("/Person");
pub const CONTACTPROP_PUB_L2_PHONENUMBER: windows_core::PCWSTR = windows_core::w!("/PhoneNumber");
pub const CONTACTPROP_PUB_L2_PHOTO: windows_core::PCWSTR = windows_core::w!("/Photo");
pub const CONTACTPROP_PUB_L2_PHYSICALADDRESS: windows_core::PCWSTR = windows_core::w!("/PhysicalAddress");
pub const CONTACTPROP_PUB_L2_POSITION: windows_core::PCWSTR = windows_core::w!("/Position");
pub const CONTACTPROP_PUB_L2_URL: windows_core::PCWSTR = windows_core::w!("/Url");
pub const CONTACTPROP_PUB_L3_ADDRESS: windows_core::PCWSTR = windows_core::w!("/Address");
pub const CONTACTPROP_PUB_L3_ADDRESSLABEL: windows_core::PCWSTR = windows_core::w!("/AddressLabel");
pub const CONTACTPROP_PUB_L3_ALTERNATE: windows_core::PCWSTR = windows_core::w!("/Alternate");
pub const CONTACTPROP_PUB_L3_COMPANY: windows_core::PCWSTR = windows_core::w!("/Company");
pub const CONTACTPROP_PUB_L3_COUNTRY: windows_core::PCWSTR = windows_core::w!("/Country");
pub const CONTACTPROP_PUB_L3_DEPARTMENT: windows_core::PCWSTR = windows_core::w!("/Department");
pub const CONTACTPROP_PUB_L3_EXTENDEDADDRESS: windows_core::PCWSTR = windows_core::w!("/ExtendedAddress");
pub const CONTACTPROP_PUB_L3_FAMILYNAME: windows_core::PCWSTR = windows_core::w!("/FamilyName");
pub const CONTACTPROP_PUB_L3_FORMATTEDNAME: windows_core::PCWSTR = windows_core::w!("/FormattedName");
pub const CONTACTPROP_PUB_L3_GENERATION: windows_core::PCWSTR = windows_core::w!("/Generation");
pub const CONTACTPROP_PUB_L3_GIVENNAME: windows_core::PCWSTR = windows_core::w!("/GivenName");
pub const CONTACTPROP_PUB_L3_JOB_TITLE: windows_core::PCWSTR = windows_core::w!("/JobTitle");
pub const CONTACTPROP_PUB_L3_LOCALITY: windows_core::PCWSTR = windows_core::w!("/Locality");
pub const CONTACTPROP_PUB_L3_MIDDLENAME: windows_core::PCWSTR = windows_core::w!("/MiddleName");
pub const CONTACTPROP_PUB_L3_NICKNAME: windows_core::PCWSTR = windows_core::w!("/NickName");
pub const CONTACTPROP_PUB_L3_NUMBER: windows_core::PCWSTR = windows_core::w!("/Number");
pub const CONTACTPROP_PUB_L3_OFFICE: windows_core::PCWSTR = windows_core::w!("/Office");
pub const CONTACTPROP_PUB_L3_ORGANIZATION: windows_core::PCWSTR = windows_core::w!("/Organization");
pub const CONTACTPROP_PUB_L3_PERSONID: windows_core::PCWSTR = windows_core::w!("/PersonID");
pub const CONTACTPROP_PUB_L3_PHONETIC: windows_core::PCWSTR = windows_core::w!("/Phonetic");
pub const CONTACTPROP_PUB_L3_POBOX: windows_core::PCWSTR = windows_core::w!("/POBox");
pub const CONTACTPROP_PUB_L3_POSTALCODE: windows_core::PCWSTR = windows_core::w!("/PostalCode");
pub const CONTACTPROP_PUB_L3_PREFIX: windows_core::PCWSTR = windows_core::w!("/Prefix");
pub const CONTACTPROP_PUB_L3_PROFESSION: windows_core::PCWSTR = windows_core::w!("/Profession");
pub const CONTACTPROP_PUB_L3_PROTOCOL: windows_core::PCWSTR = windows_core::w!("/Protocol");
pub const CONTACTPROP_PUB_L3_REGION: windows_core::PCWSTR = windows_core::w!("/Region");
pub const CONTACTPROP_PUB_L3_ROLE: windows_core::PCWSTR = windows_core::w!("/Role");
pub const CONTACTPROP_PUB_L3_STREET: windows_core::PCWSTR = windows_core::w!("/Street");
pub const CONTACTPROP_PUB_L3_SUFFIX: windows_core::PCWSTR = windows_core::w!("/Suffix");
pub const CONTACTPROP_PUB_L3_THUMBPRINT: windows_core::PCWSTR = windows_core::w!("/ThumbPrint");
pub const CONTACTPROP_PUB_L3_TITLE: windows_core::PCWSTR = windows_core::w!("/Title");
pub const CONTACTPROP_PUB_L3_TYPE: windows_core::PCWSTR = windows_core::w!("/Type");
pub const CONTACTPROP_PUB_L3_URL: windows_core::PCWSTR = windows_core::w!("/Url");
pub const CONTACTPROP_PUB_L3_VALUE: windows_core::PCWSTR = windows_core::w!("/Value");
pub const CONTACTPROP_PUB_MAILER: windows_core::PCWSTR = windows_core::w!("Mailer");
pub const CONTACTPROP_PUB_NOTES: windows_core::PCWSTR = windows_core::w!("Notes");
pub const CONTACTPROP_PUB_PROGID: windows_core::PCWSTR = windows_core::w!("ProgID");
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CONTACT_AGGREGATION_BLOB {
    pub dwCount: u32,
    pub lpb: *mut u8,
}
impl Default for CONTACT_AGGREGATION_BLOB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CONTACT_AGGREGATION_COLLECTION_OPTIONS(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CONTACT_AGGREGATION_CREATE_OR_OPEN_OPTIONS(pub i32);
pub const Contact: windows_core::GUID = windows_core::GUID::from_u128(0x61b68808_8eee_4fd1_acb8_3d804c8db056);
pub const ContactManager: windows_core::GUID = windows_core::GUID::from_u128(0x7165c8ab_af88_42bd_86fd_5310b4285a02);
windows_core::imp::define_interface!(IContact, IContact_Vtbl, 0xf941b671_bda7_4f77_884a_f46462f226a7);
windows_core::imp::interface_hierarchy!(IContact, windows_core::IUnknown);
impl IContact {
    pub unsafe fn GetContactID(&self, pszcontactid: &mut [u16], pdwcchcontactidrequired: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetContactID)(windows_core::Interface::as_raw(self), core::mem::transmute(pszcontactid.as_ptr()), pszcontactid.len().try_into().unwrap(), pdwcchcontactidrequired as _).ok() }
    }
    pub unsafe fn GetPath(&self, pszpath: &mut [u16], pdwcchpathrequired: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetPath)(windows_core::Interface::as_raw(self), core::mem::transmute(pszpath.as_ptr()), pszpath.len().try_into().unwrap(), pdwcchpathrequired as _).ok() }
    }
    pub unsafe fn CommitChanges(&self, dwcommitflags: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CommitChanges)(windows_core::Interface::as_raw(self), dwcommitflags).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IContact_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetContactID: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PWSTR, u32, *mut u32) -> windows_core::HRESULT,
    pub GetPath: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PWSTR, u32, *mut u32) -> windows_core::HRESULT,
    pub CommitChanges: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
pub trait IContact_Impl: windows_core::IUnknownImpl {
    fn GetContactID(&self, pszcontactid: windows_core::PWSTR, cchcontactid: u32, pdwcchcontactidrequired: *mut u32) -> windows_core::Result<()>;
    fn GetPath(&self, pszpath: windows_core::PWSTR, cchpath: u32, pdwcchpathrequired: *mut u32) -> windows_core::Result<()>;
    fn CommitChanges(&self, dwcommitflags: u32) -> windows_core::Result<()>;
}
impl IContact_Vtbl {
    pub const fn new<Identity: IContact_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetContactID<Identity: IContact_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszcontactid: windows_core::PWSTR, cchcontactid: u32, pdwcchcontactidrequired: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IContact_Impl::GetContactID(this, core::mem::transmute_copy(&pszcontactid), core::mem::transmute_copy(&cchcontactid), core::mem::transmute_copy(&pdwcchcontactidrequired)).into()
            }
        }
        unsafe extern "system" fn GetPath<Identity: IContact_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpath: windows_core::PWSTR, cchpath: u32, pdwcchpathrequired: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IContact_Impl::GetPath(this, core::mem::transmute_copy(&pszpath), core::mem::transmute_copy(&cchpath), core::mem::transmute_copy(&pdwcchpathrequired)).into()
            }
        }
        unsafe extern "system" fn CommitChanges<Identity: IContact_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwcommitflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IContact_Impl::CommitChanges(this, core::mem::transmute_copy(&dwcommitflags)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetContactID: GetContactID::<Identity, OFFSET>,
            GetPath: GetPath::<Identity, OFFSET>,
            CommitChanges: CommitChanges::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IContact as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IContact {}
windows_core::imp::define_interface!(IContactAggregationAggregate, IContactAggregationAggregate_Vtbl, 0x7ed1c814_cd30_43c8_9b8d_2e489e53d54b);
windows_core::imp::interface_hierarchy!(IContactAggregationAggregate, windows_core::IUnknown);
impl IContactAggregationAggregate {
    pub unsafe fn Save(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Save)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn GetComponentItems(&self) -> windows_core::Result<IContactAggregationContactCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetComponentItems)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Link<P0>(&self, paggregateid: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).Link)(windows_core::Interface::as_raw(self), paggregateid.param().abi()).ok() }
    }
    pub unsafe fn get_Groups(&self, options: CONTACT_AGGREGATION_COLLECTION_OPTIONS) -> windows_core::Result<IContactAggregationGroupCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Groups)(windows_core::Interface::as_raw(self), options, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn AntiLink(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AntiLink)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetAntiLink<P0>(&self, pantilink: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetAntiLink)(windows_core::Interface::as_raw(self), pantilink.param().abi()).ok() }
    }
    pub unsafe fn FavoriteOrder(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FavoriteOrder)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetFavoriteOrder(&self, favoriteorder: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetFavoriteOrder)(windows_core::Interface::as_raw(self), favoriteorder).ok() }
    }
    pub unsafe fn Id(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Id)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactAggregationAggregate_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Save: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetComponentItems: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Link: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub get_Groups: unsafe extern "system" fn(*mut core::ffi::c_void, CONTACT_AGGREGATION_COLLECTION_OPTIONS, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AntiLink: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub SetAntiLink: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub FavoriteOrder: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetFavoriteOrder: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
}
pub trait IContactAggregationAggregate_Impl: windows_core::IUnknownImpl {
    fn Save(&self) -> windows_core::Result<()>;
    fn GetComponentItems(&self) -> windows_core::Result<IContactAggregationContactCollection>;
    fn Link(&self, paggregateid: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn get_Groups(&self, options: CONTACT_AGGREGATION_COLLECTION_OPTIONS) -> windows_core::Result<IContactAggregationGroupCollection>;
    fn AntiLink(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetAntiLink(&self, pantilink: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn FavoriteOrder(&self) -> windows_core::Result<u32>;
    fn SetFavoriteOrder(&self, favoriteorder: u32) -> windows_core::Result<()>;
    fn Id(&self) -> windows_core::Result<windows_core::PWSTR>;
}
impl IContactAggregationAggregate_Vtbl {
    pub const fn new<Identity: IContactAggregationAggregate_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Save<Identity: IContactAggregationAggregate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IContactAggregationAggregate_Impl::Save(this).into()
            }
        }
        unsafe extern "system" fn GetComponentItems<Identity: IContactAggregationAggregate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcomponentitems: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IContactAggregationAggregate_Impl::GetComponentItems(this) {
                    Ok(ok__) => {
                        pcomponentitems.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Link<Identity: IContactAggregationAggregate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, paggregateid: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IContactAggregationAggregate_Impl::Link(this, core::mem::transmute(&paggregateid)).into()
            }
        }
        unsafe extern "system" fn get_Groups<Identity: IContactAggregationAggregate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, options: CONTACT_AGGREGATION_COLLECTION_OPTIONS, ppgroups: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IContactAggregationAggregate_Impl::get_Groups(this, core::mem::transmute_copy(&options)) {
                    Ok(ok__) => {
                        ppgroups.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AntiLink<Identity: IContactAggregationAggregate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppantilink: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IContactAggregationAggregate_Impl::AntiLink(this) {
                    Ok(ok__) => {
                        ppantilink.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAntiLink<Identity: IContactAggregationAggregate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pantilink: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IContactAggregationAggregate_Impl::SetAntiLink(this, core::mem::transmute(&pantilink)).into()
            }
        }
        unsafe extern "system" fn FavoriteOrder<Identity: IContactAggregationAggregate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfavoriteorder: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IContactAggregationAggregate_Impl::FavoriteOrder(this) {
                    Ok(ok__) => {
                        pfavoriteorder.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetFavoriteOrder<Identity: IContactAggregationAggregate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, favoriteorder: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IContactAggregationAggregate_Impl::SetFavoriteOrder(this, core::mem::transmute_copy(&favoriteorder)).into()
            }
        }
        unsafe extern "system" fn Id<Identity: IContactAggregationAggregate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppitemid: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IContactAggregationAggregate_Impl::Id(this) {
                    Ok(ok__) => {
                        ppitemid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Save: Save::<Identity, OFFSET>,
            GetComponentItems: GetComponentItems::<Identity, OFFSET>,
            Link: Link::<Identity, OFFSET>,
            get_Groups: get_Groups::<Identity, OFFSET>,
            AntiLink: AntiLink::<Identity, OFFSET>,
            SetAntiLink: SetAntiLink::<Identity, OFFSET>,
            FavoriteOrder: FavoriteOrder::<Identity, OFFSET>,
            SetFavoriteOrder: SetFavoriteOrder::<Identity, OFFSET>,
            Id: Id::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IContactAggregationAggregate as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IContactAggregationAggregate {}
windows_core::imp::define_interface!(IContactAggregationAggregateCollection, IContactAggregationAggregateCollection_Vtbl, 0x2359f3a6_3a68_40af_98db_0f9eb143c3bb);
windows_core::imp::interface_hierarchy!(IContactAggregationAggregateCollection, windows_core::IUnknown);
impl IContactAggregationAggregateCollection {
    pub unsafe fn FindFirst(&self) -> windows_core::Result<IContactAggregationAggregate> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FindFirst)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn FindFirstByAntiLinkId<P0>(&self, pantilinkid: P0) -> windows_core::Result<IContactAggregationAggregate>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FindFirstByAntiLinkId)(windows_core::Interface::as_raw(self), pantilinkid.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn FindNext(&self) -> windows_core::Result<IContactAggregationAggregate> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FindNext)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactAggregationAggregateCollection_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub FindFirst: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FindFirstByAntiLinkId: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FindNext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
pub trait IContactAggregationAggregateCollection_Impl: windows_core::IUnknownImpl {
    fn FindFirst(&self) -> windows_core::Result<IContactAggregationAggregate>;
    fn FindFirstByAntiLinkId(&self, pantilinkid: &windows_core::PCWSTR) -> windows_core::Result<IContactAggregationAggregate>;
    fn FindNext(&self) -> windows_core::Result<IContactAggregationAggregate>;
    fn Count(&self) -> windows_core::Result<i32>;
}
impl IContactAggregationAggregateCollection_Vtbl {
    pub const fn new<Identity: IContactAggregationAggregateCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn FindFirst<Identity: IContactAggregationAggregateCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppaggregate: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IContactAggregationAggregateCollection_Impl::FindFirst(this) {
                    Ok(ok__) => {
                        ppaggregate.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn FindFirstByAntiLinkId<Identity: IContactAggregationAggregateCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pantilinkid: windows_core::PCWSTR, ppaggregate: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IContactAggregationAggregateCollection_Impl::FindFirstByAntiLinkId(this, core::mem::transmute(&pantilinkid)) {
                    Ok(ok__) => {
                        ppaggregate.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn FindNext<Identity: IContactAggregationAggregateCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppaggregate: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IContactAggregationAggregateCollection_Impl::FindNext(this) {
                    Ok(ok__) => {
                        ppaggregate.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Count<Identity: IContactAggregationAggregateCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IContactAggregationAggregateCollection_Impl::Count(this) {
                    Ok(ok__) => {
                        pcount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            FindFirst: FindFirst::<Identity, OFFSET>,
            FindFirstByAntiLinkId: FindFirstByAntiLinkId::<Identity, OFFSET>,
            FindNext: FindNext::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IContactAggregationAggregateCollection as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IContactAggregationAggregateCollection {}
windows_core::imp::define_interface!(IContactAggregationContact, IContactAggregationContact_Vtbl, 0x1eb22e86_4c86_41f0_9f9f_c251e9fda6c3);
windows_core::imp::interface_hierarchy!(IContactAggregationContact, windows_core::IUnknown);
impl IContactAggregationContact {
    pub unsafe fn Delete(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Delete)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Save(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Save)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn MoveToAggregate<P0>(&self, paggregateid: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).MoveToAggregate)(windows_core::Interface::as_raw(self), paggregateid.param().abi()).ok() }
    }
    pub unsafe fn Unlink(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Unlink)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn AccountId(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AccountId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetAccountId<P0>(&self, paccountid: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetAccountId)(windows_core::Interface::as_raw(self), paccountid.param().abi()).ok() }
    }
    pub unsafe fn AggregateId(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AggregateId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Id(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Id)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn IsMe(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsMe)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn IsExternal(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsExternal)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn NetworkSourceId(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).NetworkSourceId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetNetworkSourceId(&self, networksourceid: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetNetworkSourceId)(windows_core::Interface::as_raw(self), networksourceid).ok() }
    }
    pub unsafe fn NetworkSourceIdString(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).NetworkSourceIdString)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetNetworkSourceIdString<P0>(&self, pnetworksourceid: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetNetworkSourceIdString)(windows_core::Interface::as_raw(self), pnetworksourceid.param().abi()).ok() }
    }
    pub unsafe fn RemoteObjectId(&self) -> windows_core::Result<*mut CONTACT_AGGREGATION_BLOB> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RemoteObjectId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetRemoteObjectId(&self, premoteobjectid: *const CONTACT_AGGREGATION_BLOB) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetRemoteObjectId)(windows_core::Interface::as_raw(self), premoteobjectid).ok() }
    }
    pub unsafe fn SyncIdentityHash(&self) -> windows_core::Result<*mut CONTACT_AGGREGATION_BLOB> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SyncIdentityHash)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetSyncIdentityHash(&self, psyncidentityhash: *const CONTACT_AGGREGATION_BLOB) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetSyncIdentityHash)(windows_core::Interface::as_raw(self), psyncidentityhash).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactAggregationContact_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Delete: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Save: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MoveToAggregate: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub Unlink: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AccountId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub SetAccountId: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub AggregateId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub IsMe: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub IsExternal: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub NetworkSourceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetNetworkSourceId: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub NetworkSourceIdString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub SetNetworkSourceIdString: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub RemoteObjectId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut CONTACT_AGGREGATION_BLOB) -> windows_core::HRESULT,
    pub SetRemoteObjectId: unsafe extern "system" fn(*mut core::ffi::c_void, *const CONTACT_AGGREGATION_BLOB) -> windows_core::HRESULT,
    pub SyncIdentityHash: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut CONTACT_AGGREGATION_BLOB) -> windows_core::HRESULT,
    pub SetSyncIdentityHash: unsafe extern "system" fn(*mut core::ffi::c_void, *const CONTACT_AGGREGATION_BLOB) -> windows_core::HRESULT,
}
pub trait IContactAggregationContact_Impl: windows_core::IUnknownImpl {
    fn Delete(&self) -> windows_core::Result<()>;
    fn Save(&self) -> windows_core::Result<()>;
    fn MoveToAggregate(&self, paggregateid: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn Unlink(&self) -> windows_core::Result<()>;
    fn AccountId(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetAccountId(&self, paccountid: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn AggregateId(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn Id(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn IsMe(&self) -> windows_core::Result<windows_core::BOOL>;
    fn IsExternal(&self) -> windows_core::Result<windows_core::BOOL>;
    fn NetworkSourceId(&self) -> windows_core::Result<u32>;
    fn SetNetworkSourceId(&self, networksourceid: u32) -> windows_core::Result<()>;
    fn NetworkSourceIdString(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetNetworkSourceIdString(&self, pnetworksourceid: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn RemoteObjectId(&self) -> windows_core::Result<*mut CONTACT_AGGREGATION_BLOB>;
    fn SetRemoteObjectId(&self, premoteobjectid: *const CONTACT_AGGREGATION_BLOB) -> windows_core::Result<()>;
    fn SyncIdentityHash(&self) -> windows_core::Result<*mut CONTACT_AGGREGATION_BLOB>;
    fn SetSyncIdentityHash(&self, psyncidentityhash: *const CONTACT_AGGREGATION_BLOB) -> windows_core::Result<()>;
}
impl IContactAggregationContact_Vtbl {
    pub const fn new<Identity: IContactAggregationContact_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Delete<Identity: IContactAggregationContact_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IContactAggregationContact_Impl::Delete(this).into()
            }
        }
        unsafe extern "system" fn Save<Identity: IContactAggregationContact_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IContactAggregationContact_Impl::Save(this).into()
            }
        }
        unsafe extern "system" fn MoveToAggregate<Identity: IContactAggregationContact_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, paggregateid: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IContactAggregationContact_Impl::MoveToAggregate(this, core::mem::transmute(&paggregateid)).into()
            }
        }
        unsafe extern "system" fn Unlink<Identity: IContactAggregationContact_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IContactAggregationContact_Impl::Unlink(this).into()
            }
        }
        unsafe extern "system" fn AccountId<Identity: IContactAggregationContact_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppaccountid: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IContactAggregationContact_Impl::AccountId(this) {
                    Ok(ok__) => {
                        ppaccountid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAccountId<Identity: IContactAggregationContact_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, paccountid: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IContactAggregationContact_Impl::SetAccountId(this, core::mem::transmute(&paccountid)).into()
            }
        }
        unsafe extern "system" fn AggregateId<Identity: IContactAggregationContact_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppaggregateid: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IContactAggregationContact_Impl::AggregateId(this) {
                    Ok(ok__) => {
                        ppaggregateid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Id<Identity: IContactAggregationContact_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppitemid: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IContactAggregationContact_Impl::Id(this) {
                    Ok(ok__) => {
                        ppitemid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsMe<Identity: IContactAggregationContact_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pisme: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IContactAggregationContact_Impl::IsMe(this) {
                    Ok(ok__) => {
                        pisme.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsExternal<Identity: IContactAggregationContact_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pisexternal: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IContactAggregationContact_Impl::IsExternal(this) {
                    Ok(ok__) => {
                        pisexternal.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn NetworkSourceId<Identity: IContactAggregationContact_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnetworksourceid: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IContactAggregationContact_Impl::NetworkSourceId(this) {
                    Ok(ok__) => {
                        pnetworksourceid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetNetworkSourceId<Identity: IContactAggregationContact_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, networksourceid: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IContactAggregationContact_Impl::SetNetworkSourceId(this, core::mem::transmute_copy(&networksourceid)).into()
            }
        }
        unsafe extern "system" fn NetworkSourceIdString<Identity: IContactAggregationContact_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppnetworksourceid: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IContactAggregationContact_Impl::NetworkSourceIdString(this) {
                    Ok(ok__) => {
                        ppnetworksourceid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetNetworkSourceIdString<Identity: IContactAggregationContact_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnetworksourceid: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IContactAggregationContact_Impl::SetNetworkSourceIdString(this, core::mem::transmute(&pnetworksourceid)).into()
            }
        }
        unsafe extern "system" fn RemoteObjectId<Identity: IContactAggregationContact_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppremoteobjectid: *mut *mut CONTACT_AGGREGATION_BLOB) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IContactAggregationContact_Impl::RemoteObjectId(this) {
                    Ok(ok__) => {
                        ppremoteobjectid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetRemoteObjectId<Identity: IContactAggregationContact_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, premoteobjectid: *const CONTACT_AGGREGATION_BLOB) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IContactAggregationContact_Impl::SetRemoteObjectId(this, core::mem::transmute_copy(&premoteobjectid)).into()
            }
        }
        unsafe extern "system" fn SyncIdentityHash<Identity: IContactAggregationContact_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppsyncidentityhash: *mut *mut CONTACT_AGGREGATION_BLOB) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IContactAggregationContact_Impl::SyncIdentityHash(this) {
                    Ok(ok__) => {
                        ppsyncidentityhash.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSyncIdentityHash<Identity: IContactAggregationContact_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psyncidentityhash: *const CONTACT_AGGREGATION_BLOB) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IContactAggregationContact_Impl::SetSyncIdentityHash(this, core::mem::transmute_copy(&psyncidentityhash)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Delete: Delete::<Identity, OFFSET>,
            Save: Save::<Identity, OFFSET>,
            MoveToAggregate: MoveToAggregate::<Identity, OFFSET>,
            Unlink: Unlink::<Identity, OFFSET>,
            AccountId: AccountId::<Identity, OFFSET>,
            SetAccountId: SetAccountId::<Identity, OFFSET>,
            AggregateId: AggregateId::<Identity, OFFSET>,
            Id: Id::<Identity, OFFSET>,
            IsMe: IsMe::<Identity, OFFSET>,
            IsExternal: IsExternal::<Identity, OFFSET>,
            NetworkSourceId: NetworkSourceId::<Identity, OFFSET>,
            SetNetworkSourceId: SetNetworkSourceId::<Identity, OFFSET>,
            NetworkSourceIdString: NetworkSourceIdString::<Identity, OFFSET>,
            SetNetworkSourceIdString: SetNetworkSourceIdString::<Identity, OFFSET>,
            RemoteObjectId: RemoteObjectId::<Identity, OFFSET>,
            SetRemoteObjectId: SetRemoteObjectId::<Identity, OFFSET>,
            SyncIdentityHash: SyncIdentityHash::<Identity, OFFSET>,
            SetSyncIdentityHash: SetSyncIdentityHash::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IContactAggregationContact as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IContactAggregationContact {}
windows_core::imp::define_interface!(IContactAggregationContactCollection, IContactAggregationContactCollection_Vtbl, 0x826e66fa_81de_43ca_a6fb_8c785cd996c6);
windows_core::imp::interface_hierarchy!(IContactAggregationContactCollection, windows_core::IUnknown);
impl IContactAggregationContactCollection {
    pub unsafe fn FindFirst(&self) -> windows_core::Result<IContactAggregationContact> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FindFirst)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn FindNext(&self) -> windows_core::Result<IContactAggregationContact> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FindNext)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn FindFirstByIdentityHash<P0, P1>(&self, psourcetype: P0, paccountid: P1, pidentityhash: *const CONTACT_AGGREGATION_BLOB) -> windows_core::Result<IContactAggregationContact>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FindFirstByIdentityHash)(windows_core::Interface::as_raw(self), psourcetype.param().abi(), paccountid.param().abi(), pidentityhash, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn FindFirstByRemoteId<P0, P1>(&self, psourcetype: P0, paccountid: P1, premoteobjectid: *const CONTACT_AGGREGATION_BLOB) -> windows_core::Result<IContactAggregationContact>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FindFirstByRemoteId)(windows_core::Interface::as_raw(self), psourcetype.param().abi(), paccountid.param().abi(), premoteobjectid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactAggregationContactCollection_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub FindFirst: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FindNext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FindFirstByIdentityHash: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, *const CONTACT_AGGREGATION_BLOB, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub FindFirstByRemoteId: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, *const CONTACT_AGGREGATION_BLOB, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IContactAggregationContactCollection_Impl: windows_core::IUnknownImpl {
    fn FindFirst(&self) -> windows_core::Result<IContactAggregationContact>;
    fn FindNext(&self) -> windows_core::Result<IContactAggregationContact>;
    fn FindFirstByIdentityHash(&self, psourcetype: &windows_core::PCWSTR, paccountid: &windows_core::PCWSTR, pidentityhash: *const CONTACT_AGGREGATION_BLOB) -> windows_core::Result<IContactAggregationContact>;
    fn Count(&self) -> windows_core::Result<i32>;
    fn FindFirstByRemoteId(&self, psourcetype: &windows_core::PCWSTR, paccountid: &windows_core::PCWSTR, premoteobjectid: *const CONTACT_AGGREGATION_BLOB) -> windows_core::Result<IContactAggregationContact>;
}
impl IContactAggregationContactCollection_Vtbl {
    pub const fn new<Identity: IContactAggregationContactCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn FindFirst<Identity: IContactAggregationContactCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppitem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IContactAggregationContactCollection_Impl::FindFirst(this) {
                    Ok(ok__) => {
                        ppitem.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn FindNext<Identity: IContactAggregationContactCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppitem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IContactAggregationContactCollection_Impl::FindNext(this) {
                    Ok(ok__) => {
                        ppitem.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn FindFirstByIdentityHash<Identity: IContactAggregationContactCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psourcetype: windows_core::PCWSTR, paccountid: windows_core::PCWSTR, pidentityhash: *const CONTACT_AGGREGATION_BLOB, ppitem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IContactAggregationContactCollection_Impl::FindFirstByIdentityHash(this, core::mem::transmute(&psourcetype), core::mem::transmute(&paccountid), core::mem::transmute_copy(&pidentityhash)) {
                    Ok(ok__) => {
                        ppitem.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Count<Identity: IContactAggregationContactCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IContactAggregationContactCollection_Impl::Count(this) {
                    Ok(ok__) => {
                        pcount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn FindFirstByRemoteId<Identity: IContactAggregationContactCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psourcetype: windows_core::PCWSTR, paccountid: windows_core::PCWSTR, premoteobjectid: *const CONTACT_AGGREGATION_BLOB, ppitem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IContactAggregationContactCollection_Impl::FindFirstByRemoteId(this, core::mem::transmute(&psourcetype), core::mem::transmute(&paccountid), core::mem::transmute_copy(&premoteobjectid)) {
                    Ok(ok__) => {
                        ppitem.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            FindFirst: FindFirst::<Identity, OFFSET>,
            FindNext: FindNext::<Identity, OFFSET>,
            FindFirstByIdentityHash: FindFirstByIdentityHash::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
            FindFirstByRemoteId: FindFirstByRemoteId::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IContactAggregationContactCollection as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IContactAggregationContactCollection {}
windows_core::imp::define_interface!(IContactAggregationGroup, IContactAggregationGroup_Vtbl, 0xc93c545f_1284_499b_96af_07372af473e0);
windows_core::imp::interface_hierarchy!(IContactAggregationGroup, windows_core::IUnknown);
impl IContactAggregationGroup {
    pub unsafe fn Delete(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Delete)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Save(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Save)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Add<P0>(&self, paggregateid: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).Add)(windows_core::Interface::as_raw(self), paggregateid.param().abi()).ok() }
    }
    pub unsafe fn Remove<P0>(&self, paggregateid: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).Remove)(windows_core::Interface::as_raw(self), paggregateid.param().abi()).ok() }
    }
    pub unsafe fn Members(&self) -> windows_core::Result<IContactAggregationAggregateCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Members)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GlobalObjectId(&self) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GlobalObjectId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetGlobalObjectId(&self, pglobalobjectid: *const windows_core::GUID) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetGlobalObjectId)(windows_core::Interface::as_raw(self), pglobalobjectid).ok() }
    }
    pub unsafe fn Id(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Id)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetName<P0>(&self, pname: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetName)(windows_core::Interface::as_raw(self), pname.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactAggregationGroup_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Delete: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Save: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Add: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub Members: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GlobalObjectId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub SetGlobalObjectId: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub SetName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
}
pub trait IContactAggregationGroup_Impl: windows_core::IUnknownImpl {
    fn Delete(&self) -> windows_core::Result<()>;
    fn Save(&self) -> windows_core::Result<()>;
    fn Add(&self, paggregateid: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn Remove(&self, paggregateid: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn Members(&self) -> windows_core::Result<IContactAggregationAggregateCollection>;
    fn GlobalObjectId(&self) -> windows_core::Result<windows_core::GUID>;
    fn SetGlobalObjectId(&self, pglobalobjectid: *const windows_core::GUID) -> windows_core::Result<()>;
    fn Id(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn Name(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetName(&self, pname: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
impl IContactAggregationGroup_Vtbl {
    pub const fn new<Identity: IContactAggregationGroup_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Delete<Identity: IContactAggregationGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IContactAggregationGroup_Impl::Delete(this).into()
            }
        }
        unsafe extern "system" fn Save<Identity: IContactAggregationGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IContactAggregationGroup_Impl::Save(this).into()
            }
        }
        unsafe extern "system" fn Add<Identity: IContactAggregationGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, paggregateid: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IContactAggregationGroup_Impl::Add(this, core::mem::transmute(&paggregateid)).into()
            }
        }
        unsafe extern "system" fn Remove<Identity: IContactAggregationGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, paggregateid: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IContactAggregationGroup_Impl::Remove(this, core::mem::transmute(&paggregateid)).into()
            }
        }
        unsafe extern "system" fn Members<Identity: IContactAggregationGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppaggregatecontactcollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IContactAggregationGroup_Impl::Members(this) {
                    Ok(ok__) => {
                        ppaggregatecontactcollection.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GlobalObjectId<Identity: IContactAggregationGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pglobalobjectid: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IContactAggregationGroup_Impl::GlobalObjectId(this) {
                    Ok(ok__) => {
                        pglobalobjectid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetGlobalObjectId<Identity: IContactAggregationGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pglobalobjectid: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IContactAggregationGroup_Impl::SetGlobalObjectId(this, core::mem::transmute_copy(&pglobalobjectid)).into()
            }
        }
        unsafe extern "system" fn Id<Identity: IContactAggregationGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppitemid: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IContactAggregationGroup_Impl::Id(this) {
                    Ok(ok__) => {
                        ppitemid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Name<Identity: IContactAggregationGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppname: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IContactAggregationGroup_Impl::Name(this) {
                    Ok(ok__) => {
                        ppname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetName<Identity: IContactAggregationGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pname: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IContactAggregationGroup_Impl::SetName(this, core::mem::transmute(&pname)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Delete: Delete::<Identity, OFFSET>,
            Save: Save::<Identity, OFFSET>,
            Add: Add::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
            Members: Members::<Identity, OFFSET>,
            GlobalObjectId: GlobalObjectId::<Identity, OFFSET>,
            SetGlobalObjectId: SetGlobalObjectId::<Identity, OFFSET>,
            Id: Id::<Identity, OFFSET>,
            Name: Name::<Identity, OFFSET>,
            SetName: SetName::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IContactAggregationGroup as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IContactAggregationGroup {}
windows_core::imp::define_interface!(IContactAggregationGroupCollection, IContactAggregationGroupCollection_Vtbl, 0x20a19a9c_d2f3_4b83_9143_beffd2cc226d);
windows_core::imp::interface_hierarchy!(IContactAggregationGroupCollection, windows_core::IUnknown);
impl IContactAggregationGroupCollection {
    pub unsafe fn FindFirst(&self) -> windows_core::Result<IContactAggregationGroup> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FindFirst)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn FindFirstByGlobalObjectId(&self, pglobalobjectid: *const windows_core::GUID) -> windows_core::Result<IContactAggregationGroup> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FindFirstByGlobalObjectId)(windows_core::Interface::as_raw(self), pglobalobjectid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn FindNext(&self) -> windows_core::Result<IContactAggregationGroup> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FindNext)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Count(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactAggregationGroupCollection_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub FindFirst: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FindFirstByGlobalObjectId: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FindNext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
pub trait IContactAggregationGroupCollection_Impl: windows_core::IUnknownImpl {
    fn FindFirst(&self) -> windows_core::Result<IContactAggregationGroup>;
    fn FindFirstByGlobalObjectId(&self, pglobalobjectid: *const windows_core::GUID) -> windows_core::Result<IContactAggregationGroup>;
    fn FindNext(&self) -> windows_core::Result<IContactAggregationGroup>;
    fn Count(&self) -> windows_core::Result<u32>;
}
impl IContactAggregationGroupCollection_Vtbl {
    pub const fn new<Identity: IContactAggregationGroupCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn FindFirst<Identity: IContactAggregationGroupCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppgroup: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IContactAggregationGroupCollection_Impl::FindFirst(this) {
                    Ok(ok__) => {
                        ppgroup.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn FindFirstByGlobalObjectId<Identity: IContactAggregationGroupCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pglobalobjectid: *const windows_core::GUID, ppgroup: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IContactAggregationGroupCollection_Impl::FindFirstByGlobalObjectId(this, core::mem::transmute_copy(&pglobalobjectid)) {
                    Ok(ok__) => {
                        ppgroup.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn FindNext<Identity: IContactAggregationGroupCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppgroup: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IContactAggregationGroupCollection_Impl::FindNext(this) {
                    Ok(ok__) => {
                        ppgroup.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Count<Identity: IContactAggregationGroupCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IContactAggregationGroupCollection_Impl::Count(this) {
                    Ok(ok__) => {
                        pcount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            FindFirst: FindFirst::<Identity, OFFSET>,
            FindFirstByGlobalObjectId: FindFirstByGlobalObjectId::<Identity, OFFSET>,
            FindNext: FindNext::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IContactAggregationGroupCollection as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IContactAggregationGroupCollection {}
windows_core::imp::define_interface!(IContactAggregationLink, IContactAggregationLink_Vtbl, 0xb6813323_a183_4654_8627_79b30de3a0ec);
windows_core::imp::interface_hierarchy!(IContactAggregationLink, windows_core::IUnknown);
impl IContactAggregationLink {
    pub unsafe fn Delete(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Delete)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Save(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Save)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn AccountId(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AccountId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetAccountId<P0>(&self, paccountid: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetAccountId)(windows_core::Interface::as_raw(self), paccountid.param().abi()).ok() }
    }
    pub unsafe fn Id(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Id)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn IsLinkResolved(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsLinkResolved)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetIsLinkResolved(&self, islinkresolved: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetIsLinkResolved)(windows_core::Interface::as_raw(self), islinkresolved.into()).ok() }
    }
    pub unsafe fn NetworkSourceIdString(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).NetworkSourceIdString)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetNetworkSourceIdString<P0>(&self, pnetworksourceid: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetNetworkSourceIdString)(windows_core::Interface::as_raw(self), pnetworksourceid.param().abi()).ok() }
    }
    pub unsafe fn RemoteObjectId(&self) -> windows_core::Result<*mut CONTACT_AGGREGATION_BLOB> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RemoteObjectId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetRemoteObjectId(&self, premoteobjectid: *const CONTACT_AGGREGATION_BLOB) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetRemoteObjectId)(windows_core::Interface::as_raw(self), premoteobjectid).ok() }
    }
    pub unsafe fn ServerPerson(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ServerPerson)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetServerPerson<P0>(&self, pserverpersonid: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetServerPerson)(windows_core::Interface::as_raw(self), pserverpersonid.param().abi()).ok() }
    }
    pub unsafe fn ServerPersonBaseline(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ServerPersonBaseline)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetServerPersonBaseline<P0>(&self, pserverpersonid: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetServerPersonBaseline)(windows_core::Interface::as_raw(self), pserverpersonid.param().abi()).ok() }
    }
    pub unsafe fn SyncIdentityHash(&self) -> windows_core::Result<*mut CONTACT_AGGREGATION_BLOB> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SyncIdentityHash)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetSyncIdentityHash(&self, psyncidentityhash: *const CONTACT_AGGREGATION_BLOB) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetSyncIdentityHash)(windows_core::Interface::as_raw(self), psyncidentityhash).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactAggregationLink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Delete: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Save: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AccountId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub SetAccountId: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub IsLinkResolved: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub SetIsLinkResolved: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub NetworkSourceIdString: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub SetNetworkSourceIdString: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub RemoteObjectId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut CONTACT_AGGREGATION_BLOB) -> windows_core::HRESULT,
    pub SetRemoteObjectId: unsafe extern "system" fn(*mut core::ffi::c_void, *const CONTACT_AGGREGATION_BLOB) -> windows_core::HRESULT,
    pub ServerPerson: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub SetServerPerson: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub ServerPersonBaseline: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub SetServerPersonBaseline: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub SyncIdentityHash: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut CONTACT_AGGREGATION_BLOB) -> windows_core::HRESULT,
    pub SetSyncIdentityHash: unsafe extern "system" fn(*mut core::ffi::c_void, *const CONTACT_AGGREGATION_BLOB) -> windows_core::HRESULT,
}
pub trait IContactAggregationLink_Impl: windows_core::IUnknownImpl {
    fn Delete(&self) -> windows_core::Result<()>;
    fn Save(&self) -> windows_core::Result<()>;
    fn AccountId(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetAccountId(&self, paccountid: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn Id(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn IsLinkResolved(&self) -> windows_core::Result<windows_core::BOOL>;
    fn SetIsLinkResolved(&self, islinkresolved: windows_core::BOOL) -> windows_core::Result<()>;
    fn NetworkSourceIdString(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetNetworkSourceIdString(&self, pnetworksourceid: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn RemoteObjectId(&self) -> windows_core::Result<*mut CONTACT_AGGREGATION_BLOB>;
    fn SetRemoteObjectId(&self, premoteobjectid: *const CONTACT_AGGREGATION_BLOB) -> windows_core::Result<()>;
    fn ServerPerson(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetServerPerson(&self, pserverpersonid: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn ServerPersonBaseline(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetServerPersonBaseline(&self, pserverpersonid: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SyncIdentityHash(&self) -> windows_core::Result<*mut CONTACT_AGGREGATION_BLOB>;
    fn SetSyncIdentityHash(&self, psyncidentityhash: *const CONTACT_AGGREGATION_BLOB) -> windows_core::Result<()>;
}
impl IContactAggregationLink_Vtbl {
    pub const fn new<Identity: IContactAggregationLink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Delete<Identity: IContactAggregationLink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IContactAggregationLink_Impl::Delete(this).into()
            }
        }
        unsafe extern "system" fn Save<Identity: IContactAggregationLink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IContactAggregationLink_Impl::Save(this).into()
            }
        }
        unsafe extern "system" fn AccountId<Identity: IContactAggregationLink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppaccountid: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IContactAggregationLink_Impl::AccountId(this) {
                    Ok(ok__) => {
                        ppaccountid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAccountId<Identity: IContactAggregationLink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, paccountid: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IContactAggregationLink_Impl::SetAccountId(this, core::mem::transmute(&paccountid)).into()
            }
        }
        unsafe extern "system" fn Id<Identity: IContactAggregationLink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppitemid: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IContactAggregationLink_Impl::Id(this) {
                    Ok(ok__) => {
                        ppitemid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsLinkResolved<Identity: IContactAggregationLink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pislinkresolved: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IContactAggregationLink_Impl::IsLinkResolved(this) {
                    Ok(ok__) => {
                        pislinkresolved.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetIsLinkResolved<Identity: IContactAggregationLink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, islinkresolved: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IContactAggregationLink_Impl::SetIsLinkResolved(this, core::mem::transmute_copy(&islinkresolved)).into()
            }
        }
        unsafe extern "system" fn NetworkSourceIdString<Identity: IContactAggregationLink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppnetworksourceid: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IContactAggregationLink_Impl::NetworkSourceIdString(this) {
                    Ok(ok__) => {
                        ppnetworksourceid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetNetworkSourceIdString<Identity: IContactAggregationLink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnetworksourceid: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IContactAggregationLink_Impl::SetNetworkSourceIdString(this, core::mem::transmute(&pnetworksourceid)).into()
            }
        }
        unsafe extern "system" fn RemoteObjectId<Identity: IContactAggregationLink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppremoteobjectid: *mut *mut CONTACT_AGGREGATION_BLOB) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IContactAggregationLink_Impl::RemoteObjectId(this) {
                    Ok(ok__) => {
                        ppremoteobjectid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetRemoteObjectId<Identity: IContactAggregationLink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, premoteobjectid: *const CONTACT_AGGREGATION_BLOB) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IContactAggregationLink_Impl::SetRemoteObjectId(this, core::mem::transmute_copy(&premoteobjectid)).into()
            }
        }
        unsafe extern "system" fn ServerPerson<Identity: IContactAggregationLink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppserverpersonid: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IContactAggregationLink_Impl::ServerPerson(this) {
                    Ok(ok__) => {
                        ppserverpersonid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetServerPerson<Identity: IContactAggregationLink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pserverpersonid: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IContactAggregationLink_Impl::SetServerPerson(this, core::mem::transmute(&pserverpersonid)).into()
            }
        }
        unsafe extern "system" fn ServerPersonBaseline<Identity: IContactAggregationLink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppserverpersonid: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IContactAggregationLink_Impl::ServerPersonBaseline(this) {
                    Ok(ok__) => {
                        ppserverpersonid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetServerPersonBaseline<Identity: IContactAggregationLink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pserverpersonid: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IContactAggregationLink_Impl::SetServerPersonBaseline(this, core::mem::transmute(&pserverpersonid)).into()
            }
        }
        unsafe extern "system" fn SyncIdentityHash<Identity: IContactAggregationLink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppsyncidentityhash: *mut *mut CONTACT_AGGREGATION_BLOB) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IContactAggregationLink_Impl::SyncIdentityHash(this) {
                    Ok(ok__) => {
                        ppsyncidentityhash.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSyncIdentityHash<Identity: IContactAggregationLink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psyncidentityhash: *const CONTACT_AGGREGATION_BLOB) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IContactAggregationLink_Impl::SetSyncIdentityHash(this, core::mem::transmute_copy(&psyncidentityhash)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Delete: Delete::<Identity, OFFSET>,
            Save: Save::<Identity, OFFSET>,
            AccountId: AccountId::<Identity, OFFSET>,
            SetAccountId: SetAccountId::<Identity, OFFSET>,
            Id: Id::<Identity, OFFSET>,
            IsLinkResolved: IsLinkResolved::<Identity, OFFSET>,
            SetIsLinkResolved: SetIsLinkResolved::<Identity, OFFSET>,
            NetworkSourceIdString: NetworkSourceIdString::<Identity, OFFSET>,
            SetNetworkSourceIdString: SetNetworkSourceIdString::<Identity, OFFSET>,
            RemoteObjectId: RemoteObjectId::<Identity, OFFSET>,
            SetRemoteObjectId: SetRemoteObjectId::<Identity, OFFSET>,
            ServerPerson: ServerPerson::<Identity, OFFSET>,
            SetServerPerson: SetServerPerson::<Identity, OFFSET>,
            ServerPersonBaseline: ServerPersonBaseline::<Identity, OFFSET>,
            SetServerPersonBaseline: SetServerPersonBaseline::<Identity, OFFSET>,
            SyncIdentityHash: SyncIdentityHash::<Identity, OFFSET>,
            SetSyncIdentityHash: SetSyncIdentityHash::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IContactAggregationLink as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IContactAggregationLink {}
windows_core::imp::define_interface!(IContactAggregationLinkCollection, IContactAggregationLinkCollection_Vtbl, 0xf8bc0e93_fb55_4f28_b9fa_b1c274153292);
windows_core::imp::interface_hierarchy!(IContactAggregationLinkCollection, windows_core::IUnknown);
impl IContactAggregationLinkCollection {
    pub unsafe fn FindFirst(&self) -> windows_core::Result<IContactAggregationLink> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FindFirst)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn FindFirstByRemoteId<P0, P1>(&self, psourcetype: P0, paccountid: P1, premoteid: *const CONTACT_AGGREGATION_BLOB) -> windows_core::Result<IContactAggregationLink>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FindFirstByRemoteId)(windows_core::Interface::as_raw(self), psourcetype.param().abi(), paccountid.param().abi(), premoteid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn FindNext(&self) -> windows_core::Result<IContactAggregationLink> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FindNext)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Count(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactAggregationLinkCollection_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub FindFirst: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FindFirstByRemoteId: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, *const CONTACT_AGGREGATION_BLOB, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FindNext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
pub trait IContactAggregationLinkCollection_Impl: windows_core::IUnknownImpl {
    fn FindFirst(&self) -> windows_core::Result<IContactAggregationLink>;
    fn FindFirstByRemoteId(&self, psourcetype: &windows_core::PCWSTR, paccountid: &windows_core::PCWSTR, premoteid: *const CONTACT_AGGREGATION_BLOB) -> windows_core::Result<IContactAggregationLink>;
    fn FindNext(&self) -> windows_core::Result<IContactAggregationLink>;
    fn Count(&self) -> windows_core::Result<u32>;
}
impl IContactAggregationLinkCollection_Vtbl {
    pub const fn new<Identity: IContactAggregationLinkCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn FindFirst<Identity: IContactAggregationLinkCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppservercontactlink: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IContactAggregationLinkCollection_Impl::FindFirst(this) {
                    Ok(ok__) => {
                        ppservercontactlink.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn FindFirstByRemoteId<Identity: IContactAggregationLinkCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psourcetype: windows_core::PCWSTR, paccountid: windows_core::PCWSTR, premoteid: *const CONTACT_AGGREGATION_BLOB, ppservercontactlink: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IContactAggregationLinkCollection_Impl::FindFirstByRemoteId(this, core::mem::transmute(&psourcetype), core::mem::transmute(&paccountid), core::mem::transmute_copy(&premoteid)) {
                    Ok(ok__) => {
                        ppservercontactlink.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn FindNext<Identity: IContactAggregationLinkCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppservercontactlink: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IContactAggregationLinkCollection_Impl::FindNext(this) {
                    Ok(ok__) => {
                        ppservercontactlink.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Count<Identity: IContactAggregationLinkCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IContactAggregationLinkCollection_Impl::Count(this) {
                    Ok(ok__) => {
                        pcount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            FindFirst: FindFirst::<Identity, OFFSET>,
            FindFirstByRemoteId: FindFirstByRemoteId::<Identity, OFFSET>,
            FindNext: FindNext::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IContactAggregationLinkCollection as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IContactAggregationLinkCollection {}
windows_core::imp::define_interface!(IContactAggregationManager, IContactAggregationManager_Vtbl, 0x1d865989_4b1f_4b60_8f34_c2ad468b2b50);
windows_core::imp::interface_hierarchy!(IContactAggregationManager, windows_core::IUnknown);
impl IContactAggregationManager {
    pub unsafe fn GetVersionInfo(&self, plmajorversion: *mut i32, plminorversion: *mut i32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetVersionInfo)(windows_core::Interface::as_raw(self), plmajorversion as _, plminorversion as _).ok() }
    }
    pub unsafe fn CreateOrOpenGroup<P0>(&self, pgroupname: P0, options: CONTACT_AGGREGATION_CREATE_OR_OPEN_OPTIONS, pcreatedgroup: *mut windows_core::BOOL) -> windows_core::Result<IContactAggregationGroup>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateOrOpenGroup)(windows_core::Interface::as_raw(self), pgroupname.param().abi(), options, pcreatedgroup as _, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateExternalContact(&self) -> windows_core::Result<IContactAggregationContact> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateExternalContact)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateServerPerson(&self) -> windows_core::Result<IContactAggregationServerPerson> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateServerPerson)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateServerContactLink(&self) -> windows_core::Result<IContactAggregationLink> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateServerContactLink)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Flush(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Flush)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn OpenAggregateContact<P0>(&self, pitemid: P0) -> windows_core::Result<IContactAggregationAggregate>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OpenAggregateContact)(windows_core::Interface::as_raw(self), pitemid.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn OpenContact<P0>(&self, pitemid: P0) -> windows_core::Result<IContactAggregationContact>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OpenContact)(windows_core::Interface::as_raw(self), pitemid.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn OpenServerContactLink<P0>(&self, pitemid: P0) -> windows_core::Result<IContactAggregationLink>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OpenServerContactLink)(windows_core::Interface::as_raw(self), pitemid.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn OpenServerPerson<P0>(&self, pitemid: P0) -> windows_core::Result<IContactAggregationServerPerson>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OpenServerPerson)(windows_core::Interface::as_raw(self), pitemid.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_Contacts(&self, options: CONTACT_AGGREGATION_COLLECTION_OPTIONS) -> windows_core::Result<IContactAggregationContactCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Contacts)(windows_core::Interface::as_raw(self), options, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_AggregateContacts(&self, options: CONTACT_AGGREGATION_COLLECTION_OPTIONS) -> windows_core::Result<IContactAggregationAggregateCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_AggregateContacts)(windows_core::Interface::as_raw(self), options, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_Groups(&self, options: CONTACT_AGGREGATION_COLLECTION_OPTIONS) -> windows_core::Result<IContactAggregationGroupCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Groups)(windows_core::Interface::as_raw(self), options, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn ServerPersons(&self) -> windows_core::Result<IContactAggregationServerPersonCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ServerPersons)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_ServerContactLinks<P0>(&self, ppersonitemid: P0) -> windows_core::Result<IContactAggregationLinkCollection>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ServerContactLinks)(windows_core::Interface::as_raw(self), ppersonitemid.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactAggregationManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetVersionInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32, *mut i32) -> windows_core::HRESULT,
    pub CreateOrOpenGroup: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, CONTACT_AGGREGATION_CREATE_OR_OPEN_OPTIONS, *mut windows_core::BOOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateExternalContact: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateServerPerson: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateServerContactLink: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Flush: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OpenAggregateContact: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OpenContact: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OpenServerContactLink: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OpenServerPerson: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Contacts: unsafe extern "system" fn(*mut core::ffi::c_void, CONTACT_AGGREGATION_COLLECTION_OPTIONS, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_AggregateContacts: unsafe extern "system" fn(*mut core::ffi::c_void, CONTACT_AGGREGATION_COLLECTION_OPTIONS, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Groups: unsafe extern "system" fn(*mut core::ffi::c_void, CONTACT_AGGREGATION_COLLECTION_OPTIONS, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ServerPersons: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_ServerContactLinks: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IContactAggregationManager_Impl: windows_core::IUnknownImpl {
    fn GetVersionInfo(&self, plmajorversion: *mut i32, plminorversion: *mut i32) -> windows_core::Result<()>;
    fn CreateOrOpenGroup(&self, pgroupname: &windows_core::PCWSTR, options: CONTACT_AGGREGATION_CREATE_OR_OPEN_OPTIONS, pcreatedgroup: *mut windows_core::BOOL) -> windows_core::Result<IContactAggregationGroup>;
    fn CreateExternalContact(&self) -> windows_core::Result<IContactAggregationContact>;
    fn CreateServerPerson(&self) -> windows_core::Result<IContactAggregationServerPerson>;
    fn CreateServerContactLink(&self) -> windows_core::Result<IContactAggregationLink>;
    fn Flush(&self) -> windows_core::Result<()>;
    fn OpenAggregateContact(&self, pitemid: &windows_core::PCWSTR) -> windows_core::Result<IContactAggregationAggregate>;
    fn OpenContact(&self, pitemid: &windows_core::PCWSTR) -> windows_core::Result<IContactAggregationContact>;
    fn OpenServerContactLink(&self, pitemid: &windows_core::PCWSTR) -> windows_core::Result<IContactAggregationLink>;
    fn OpenServerPerson(&self, pitemid: &windows_core::PCWSTR) -> windows_core::Result<IContactAggregationServerPerson>;
    fn get_Contacts(&self, options: CONTACT_AGGREGATION_COLLECTION_OPTIONS) -> windows_core::Result<IContactAggregationContactCollection>;
    fn get_AggregateContacts(&self, options: CONTACT_AGGREGATION_COLLECTION_OPTIONS) -> windows_core::Result<IContactAggregationAggregateCollection>;
    fn get_Groups(&self, options: CONTACT_AGGREGATION_COLLECTION_OPTIONS) -> windows_core::Result<IContactAggregationGroupCollection>;
    fn ServerPersons(&self) -> windows_core::Result<IContactAggregationServerPersonCollection>;
    fn get_ServerContactLinks(&self, ppersonitemid: &windows_core::PCWSTR) -> windows_core::Result<IContactAggregationLinkCollection>;
}
impl IContactAggregationManager_Vtbl {
    pub const fn new<Identity: IContactAggregationManager_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetVersionInfo<Identity: IContactAggregationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plmajorversion: *mut i32, plminorversion: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IContactAggregationManager_Impl::GetVersionInfo(this, core::mem::transmute_copy(&plmajorversion), core::mem::transmute_copy(&plminorversion)).into()
            }
        }
        unsafe extern "system" fn CreateOrOpenGroup<Identity: IContactAggregationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pgroupname: windows_core::PCWSTR, options: CONTACT_AGGREGATION_CREATE_OR_OPEN_OPTIONS, pcreatedgroup: *mut windows_core::BOOL, ppgroup: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IContactAggregationManager_Impl::CreateOrOpenGroup(this, core::mem::transmute(&pgroupname), core::mem::transmute_copy(&options), core::mem::transmute_copy(&pcreatedgroup)) {
                    Ok(ok__) => {
                        ppgroup.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateExternalContact<Identity: IContactAggregationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppitem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IContactAggregationManager_Impl::CreateExternalContact(this) {
                    Ok(ok__) => {
                        ppitem.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateServerPerson<Identity: IContactAggregationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppserverperson: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IContactAggregationManager_Impl::CreateServerPerson(this) {
                    Ok(ok__) => {
                        ppserverperson.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateServerContactLink<Identity: IContactAggregationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppservercontactlink: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IContactAggregationManager_Impl::CreateServerContactLink(this) {
                    Ok(ok__) => {
                        ppservercontactlink.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Flush<Identity: IContactAggregationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IContactAggregationManager_Impl::Flush(this).into()
            }
        }
        unsafe extern "system" fn OpenAggregateContact<Identity: IContactAggregationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pitemid: windows_core::PCWSTR, ppitem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IContactAggregationManager_Impl::OpenAggregateContact(this, core::mem::transmute(&pitemid)) {
                    Ok(ok__) => {
                        ppitem.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn OpenContact<Identity: IContactAggregationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pitemid: windows_core::PCWSTR, ppitem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IContactAggregationManager_Impl::OpenContact(this, core::mem::transmute(&pitemid)) {
                    Ok(ok__) => {
                        ppitem.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn OpenServerContactLink<Identity: IContactAggregationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pitemid: windows_core::PCWSTR, ppitem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IContactAggregationManager_Impl::OpenServerContactLink(this, core::mem::transmute(&pitemid)) {
                    Ok(ok__) => {
                        ppitem.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn OpenServerPerson<Identity: IContactAggregationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pitemid: windows_core::PCWSTR, ppitem: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IContactAggregationManager_Impl::OpenServerPerson(this, core::mem::transmute(&pitemid)) {
                    Ok(ok__) => {
                        ppitem.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Contacts<Identity: IContactAggregationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, options: CONTACT_AGGREGATION_COLLECTION_OPTIONS, ppitems: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IContactAggregationManager_Impl::get_Contacts(this, core::mem::transmute_copy(&options)) {
                    Ok(ok__) => {
                        ppitems.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_AggregateContacts<Identity: IContactAggregationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, options: CONTACT_AGGREGATION_COLLECTION_OPTIONS, ppaggregates: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IContactAggregationManager_Impl::get_AggregateContacts(this, core::mem::transmute_copy(&options)) {
                    Ok(ok__) => {
                        ppaggregates.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Groups<Identity: IContactAggregationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, options: CONTACT_AGGREGATION_COLLECTION_OPTIONS, ppgroups: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IContactAggregationManager_Impl::get_Groups(this, core::mem::transmute_copy(&options)) {
                    Ok(ok__) => {
                        ppgroups.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ServerPersons<Identity: IContactAggregationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppserverpersoncollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IContactAggregationManager_Impl::ServerPersons(this) {
                    Ok(ok__) => {
                        ppserverpersoncollection.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_ServerContactLinks<Identity: IContactAggregationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppersonitemid: windows_core::PCWSTR, ppservercontactlinkcollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IContactAggregationManager_Impl::get_ServerContactLinks(this, core::mem::transmute(&ppersonitemid)) {
                    Ok(ok__) => {
                        ppservercontactlinkcollection.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetVersionInfo: GetVersionInfo::<Identity, OFFSET>,
            CreateOrOpenGroup: CreateOrOpenGroup::<Identity, OFFSET>,
            CreateExternalContact: CreateExternalContact::<Identity, OFFSET>,
            CreateServerPerson: CreateServerPerson::<Identity, OFFSET>,
            CreateServerContactLink: CreateServerContactLink::<Identity, OFFSET>,
            Flush: Flush::<Identity, OFFSET>,
            OpenAggregateContact: OpenAggregateContact::<Identity, OFFSET>,
            OpenContact: OpenContact::<Identity, OFFSET>,
            OpenServerContactLink: OpenServerContactLink::<Identity, OFFSET>,
            OpenServerPerson: OpenServerPerson::<Identity, OFFSET>,
            get_Contacts: get_Contacts::<Identity, OFFSET>,
            get_AggregateContacts: get_AggregateContacts::<Identity, OFFSET>,
            get_Groups: get_Groups::<Identity, OFFSET>,
            ServerPersons: ServerPersons::<Identity, OFFSET>,
            get_ServerContactLinks: get_ServerContactLinks::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IContactAggregationManager as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IContactAggregationManager {}
windows_core::imp::define_interface!(IContactAggregationServerPerson, IContactAggregationServerPerson_Vtbl, 0x7fdc3d4b_1b82_4334_85c5_25184ee5a5f2);
windows_core::imp::interface_hierarchy!(IContactAggregationServerPerson, windows_core::IUnknown);
impl IContactAggregationServerPerson {
    pub unsafe fn Delete(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Delete)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Save(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Save)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn AggregateId(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AggregateId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetAggregateId<P0>(&self, paggregateid: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetAggregateId)(windows_core::Interface::as_raw(self), paggregateid.param().abi()).ok() }
    }
    pub unsafe fn AntiLink(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AntiLink)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetAntiLink<P0>(&self, pantilink: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetAntiLink)(windows_core::Interface::as_raw(self), pantilink.param().abi()).ok() }
    }
    pub unsafe fn AntiLinkBaseline(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AntiLinkBaseline)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetAntiLinkBaseline<P0>(&self, pantilink: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetAntiLinkBaseline)(windows_core::Interface::as_raw(self), pantilink.param().abi()).ok() }
    }
    pub unsafe fn FavoriteOrder(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FavoriteOrder)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetFavoriteOrder(&self, favoriteorder: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetFavoriteOrder)(windows_core::Interface::as_raw(self), favoriteorder).ok() }
    }
    pub unsafe fn FavoriteOrderBaseline(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FavoriteOrderBaseline)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetFavoriteOrderBaseline(&self, favoriteorder: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetFavoriteOrderBaseline)(windows_core::Interface::as_raw(self), favoriteorder).ok() }
    }
    pub unsafe fn Groups(&self) -> windows_core::Result<*mut CONTACT_AGGREGATION_BLOB> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Groups)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetGroups(&self, pgroups: *const CONTACT_AGGREGATION_BLOB) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetGroups)(windows_core::Interface::as_raw(self), pgroups).ok() }
    }
    pub unsafe fn GroupsBaseline(&self) -> windows_core::Result<*mut CONTACT_AGGREGATION_BLOB> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GroupsBaseline)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetGroupsBaseline(&self, pgroups: *const CONTACT_AGGREGATION_BLOB) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetGroupsBaseline)(windows_core::Interface::as_raw(self), pgroups).ok() }
    }
    pub unsafe fn Id(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Id)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn IsTombstone(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsTombstone)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetIsTombstone(&self, istombstone: bool) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetIsTombstone)(windows_core::Interface::as_raw(self), istombstone.into()).ok() }
    }
    pub unsafe fn LinkedAggregateId(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LinkedAggregateId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetLinkedAggregateId<P0>(&self, plinkedaggregateid: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetLinkedAggregateId)(windows_core::Interface::as_raw(self), plinkedaggregateid.param().abi()).ok() }
    }
    pub unsafe fn ObjectId(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ObjectId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetObjectId<P0>(&self, pobjectid: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetObjectId)(windows_core::Interface::as_raw(self), pobjectid.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactAggregationServerPerson_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Delete: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Save: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AggregateId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub SetAggregateId: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub AntiLink: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub SetAntiLink: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub AntiLinkBaseline: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub SetAntiLinkBaseline: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub FavoriteOrder: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetFavoriteOrder: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub FavoriteOrderBaseline: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetFavoriteOrderBaseline: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Groups: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut CONTACT_AGGREGATION_BLOB) -> windows_core::HRESULT,
    pub SetGroups: unsafe extern "system" fn(*mut core::ffi::c_void, *const CONTACT_AGGREGATION_BLOB) -> windows_core::HRESULT,
    pub GroupsBaseline: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut CONTACT_AGGREGATION_BLOB) -> windows_core::HRESULT,
    pub SetGroupsBaseline: unsafe extern "system" fn(*mut core::ffi::c_void, *const CONTACT_AGGREGATION_BLOB) -> windows_core::HRESULT,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub IsTombstone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub SetIsTombstone: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub LinkedAggregateId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub SetLinkedAggregateId: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub ObjectId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub SetObjectId: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
}
pub trait IContactAggregationServerPerson_Impl: windows_core::IUnknownImpl {
    fn Delete(&self) -> windows_core::Result<()>;
    fn Save(&self) -> windows_core::Result<()>;
    fn AggregateId(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetAggregateId(&self, paggregateid: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn AntiLink(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetAntiLink(&self, pantilink: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn AntiLinkBaseline(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetAntiLinkBaseline(&self, pantilink: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn FavoriteOrder(&self) -> windows_core::Result<u32>;
    fn SetFavoriteOrder(&self, favoriteorder: u32) -> windows_core::Result<()>;
    fn FavoriteOrderBaseline(&self) -> windows_core::Result<u32>;
    fn SetFavoriteOrderBaseline(&self, favoriteorder: u32) -> windows_core::Result<()>;
    fn Groups(&self) -> windows_core::Result<*mut CONTACT_AGGREGATION_BLOB>;
    fn SetGroups(&self, pgroups: *const CONTACT_AGGREGATION_BLOB) -> windows_core::Result<()>;
    fn GroupsBaseline(&self) -> windows_core::Result<*mut CONTACT_AGGREGATION_BLOB>;
    fn SetGroupsBaseline(&self, pgroups: *const CONTACT_AGGREGATION_BLOB) -> windows_core::Result<()>;
    fn Id(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn IsTombstone(&self) -> windows_core::Result<windows_core::BOOL>;
    fn SetIsTombstone(&self, istombstone: windows_core::BOOL) -> windows_core::Result<()>;
    fn LinkedAggregateId(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetLinkedAggregateId(&self, plinkedaggregateid: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn ObjectId(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetObjectId(&self, pobjectid: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
impl IContactAggregationServerPerson_Vtbl {
    pub const fn new<Identity: IContactAggregationServerPerson_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Delete<Identity: IContactAggregationServerPerson_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IContactAggregationServerPerson_Impl::Delete(this).into()
            }
        }
        unsafe extern "system" fn Save<Identity: IContactAggregationServerPerson_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IContactAggregationServerPerson_Impl::Save(this).into()
            }
        }
        unsafe extern "system" fn AggregateId<Identity: IContactAggregationServerPerson_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppaggregateid: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IContactAggregationServerPerson_Impl::AggregateId(this) {
                    Ok(ok__) => {
                        ppaggregateid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAggregateId<Identity: IContactAggregationServerPerson_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, paggregateid: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IContactAggregationServerPerson_Impl::SetAggregateId(this, core::mem::transmute(&paggregateid)).into()
            }
        }
        unsafe extern "system" fn AntiLink<Identity: IContactAggregationServerPerson_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppantilink: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IContactAggregationServerPerson_Impl::AntiLink(this) {
                    Ok(ok__) => {
                        ppantilink.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAntiLink<Identity: IContactAggregationServerPerson_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pantilink: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IContactAggregationServerPerson_Impl::SetAntiLink(this, core::mem::transmute(&pantilink)).into()
            }
        }
        unsafe extern "system" fn AntiLinkBaseline<Identity: IContactAggregationServerPerson_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppantilink: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IContactAggregationServerPerson_Impl::AntiLinkBaseline(this) {
                    Ok(ok__) => {
                        ppantilink.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAntiLinkBaseline<Identity: IContactAggregationServerPerson_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pantilink: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IContactAggregationServerPerson_Impl::SetAntiLinkBaseline(this, core::mem::transmute(&pantilink)).into()
            }
        }
        unsafe extern "system" fn FavoriteOrder<Identity: IContactAggregationServerPerson_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfavoriteorder: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IContactAggregationServerPerson_Impl::FavoriteOrder(this) {
                    Ok(ok__) => {
                        pfavoriteorder.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetFavoriteOrder<Identity: IContactAggregationServerPerson_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, favoriteorder: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IContactAggregationServerPerson_Impl::SetFavoriteOrder(this, core::mem::transmute_copy(&favoriteorder)).into()
            }
        }
        unsafe extern "system" fn FavoriteOrderBaseline<Identity: IContactAggregationServerPerson_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfavoriteorder: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IContactAggregationServerPerson_Impl::FavoriteOrderBaseline(this) {
                    Ok(ok__) => {
                        pfavoriteorder.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetFavoriteOrderBaseline<Identity: IContactAggregationServerPerson_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, favoriteorder: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IContactAggregationServerPerson_Impl::SetFavoriteOrderBaseline(this, core::mem::transmute_copy(&favoriteorder)).into()
            }
        }
        unsafe extern "system" fn Groups<Identity: IContactAggregationServerPerson_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pgroups: *mut *mut CONTACT_AGGREGATION_BLOB) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IContactAggregationServerPerson_Impl::Groups(this) {
                    Ok(ok__) => {
                        pgroups.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetGroups<Identity: IContactAggregationServerPerson_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pgroups: *const CONTACT_AGGREGATION_BLOB) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IContactAggregationServerPerson_Impl::SetGroups(this, core::mem::transmute_copy(&pgroups)).into()
            }
        }
        unsafe extern "system" fn GroupsBaseline<Identity: IContactAggregationServerPerson_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppgroups: *mut *mut CONTACT_AGGREGATION_BLOB) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IContactAggregationServerPerson_Impl::GroupsBaseline(this) {
                    Ok(ok__) => {
                        ppgroups.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetGroupsBaseline<Identity: IContactAggregationServerPerson_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pgroups: *const CONTACT_AGGREGATION_BLOB) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IContactAggregationServerPerson_Impl::SetGroupsBaseline(this, core::mem::transmute_copy(&pgroups)).into()
            }
        }
        unsafe extern "system" fn Id<Identity: IContactAggregationServerPerson_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppid: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IContactAggregationServerPerson_Impl::Id(this) {
                    Ok(ok__) => {
                        ppid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsTombstone<Identity: IContactAggregationServerPerson_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pistombstone: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IContactAggregationServerPerson_Impl::IsTombstone(this) {
                    Ok(ok__) => {
                        pistombstone.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetIsTombstone<Identity: IContactAggregationServerPerson_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, istombstone: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IContactAggregationServerPerson_Impl::SetIsTombstone(this, core::mem::transmute_copy(&istombstone)).into()
            }
        }
        unsafe extern "system" fn LinkedAggregateId<Identity: IContactAggregationServerPerson_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pplinkedaggregateid: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IContactAggregationServerPerson_Impl::LinkedAggregateId(this) {
                    Ok(ok__) => {
                        pplinkedaggregateid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetLinkedAggregateId<Identity: IContactAggregationServerPerson_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plinkedaggregateid: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IContactAggregationServerPerson_Impl::SetLinkedAggregateId(this, core::mem::transmute(&plinkedaggregateid)).into()
            }
        }
        unsafe extern "system" fn ObjectId<Identity: IContactAggregationServerPerson_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppobjectid: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IContactAggregationServerPerson_Impl::ObjectId(this) {
                    Ok(ok__) => {
                        ppobjectid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetObjectId<Identity: IContactAggregationServerPerson_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pobjectid: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IContactAggregationServerPerson_Impl::SetObjectId(this, core::mem::transmute(&pobjectid)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Delete: Delete::<Identity, OFFSET>,
            Save: Save::<Identity, OFFSET>,
            AggregateId: AggregateId::<Identity, OFFSET>,
            SetAggregateId: SetAggregateId::<Identity, OFFSET>,
            AntiLink: AntiLink::<Identity, OFFSET>,
            SetAntiLink: SetAntiLink::<Identity, OFFSET>,
            AntiLinkBaseline: AntiLinkBaseline::<Identity, OFFSET>,
            SetAntiLinkBaseline: SetAntiLinkBaseline::<Identity, OFFSET>,
            FavoriteOrder: FavoriteOrder::<Identity, OFFSET>,
            SetFavoriteOrder: SetFavoriteOrder::<Identity, OFFSET>,
            FavoriteOrderBaseline: FavoriteOrderBaseline::<Identity, OFFSET>,
            SetFavoriteOrderBaseline: SetFavoriteOrderBaseline::<Identity, OFFSET>,
            Groups: Groups::<Identity, OFFSET>,
            SetGroups: SetGroups::<Identity, OFFSET>,
            GroupsBaseline: GroupsBaseline::<Identity, OFFSET>,
            SetGroupsBaseline: SetGroupsBaseline::<Identity, OFFSET>,
            Id: Id::<Identity, OFFSET>,
            IsTombstone: IsTombstone::<Identity, OFFSET>,
            SetIsTombstone: SetIsTombstone::<Identity, OFFSET>,
            LinkedAggregateId: LinkedAggregateId::<Identity, OFFSET>,
            SetLinkedAggregateId: SetLinkedAggregateId::<Identity, OFFSET>,
            ObjectId: ObjectId::<Identity, OFFSET>,
            SetObjectId: SetObjectId::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IContactAggregationServerPerson as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IContactAggregationServerPerson {}
windows_core::imp::define_interface!(IContactAggregationServerPersonCollection, IContactAggregationServerPersonCollection_Vtbl, 0x4f730a4a_6604_47b6_a987_669ecf1e5751);
windows_core::imp::interface_hierarchy!(IContactAggregationServerPersonCollection, windows_core::IUnknown);
impl IContactAggregationServerPersonCollection {
    pub unsafe fn FindFirst(&self) -> windows_core::Result<IContactAggregationServerPerson> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FindFirst)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn FindFirstByServerId<P0>(&self, pserverid: P0) -> windows_core::Result<IContactAggregationServerPerson>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FindFirstByServerId)(windows_core::Interface::as_raw(self), pserverid.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn FindFirstByAggregateId<P0>(&self, paggregateid: P0) -> windows_core::Result<IContactAggregationServerPerson>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FindFirstByAggregateId)(windows_core::Interface::as_raw(self), paggregateid.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn FindFirstByLinkedAggregateId<P0>(&self, paggregateid: P0) -> windows_core::Result<IContactAggregationServerPerson>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FindFirstByLinkedAggregateId)(windows_core::Interface::as_raw(self), paggregateid.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn FindNext(&self) -> windows_core::Result<IContactAggregationServerPerson> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FindNext)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Count(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactAggregationServerPersonCollection_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub FindFirst: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FindFirstByServerId: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FindFirstByAggregateId: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FindFirstByLinkedAggregateId: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FindNext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
pub trait IContactAggregationServerPersonCollection_Impl: windows_core::IUnknownImpl {
    fn FindFirst(&self) -> windows_core::Result<IContactAggregationServerPerson>;
    fn FindFirstByServerId(&self, pserverid: &windows_core::PCWSTR) -> windows_core::Result<IContactAggregationServerPerson>;
    fn FindFirstByAggregateId(&self, paggregateid: &windows_core::PCWSTR) -> windows_core::Result<IContactAggregationServerPerson>;
    fn FindFirstByLinkedAggregateId(&self, paggregateid: &windows_core::PCWSTR) -> windows_core::Result<IContactAggregationServerPerson>;
    fn FindNext(&self) -> windows_core::Result<IContactAggregationServerPerson>;
    fn Count(&self) -> windows_core::Result<u32>;
}
impl IContactAggregationServerPersonCollection_Vtbl {
    pub const fn new<Identity: IContactAggregationServerPersonCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn FindFirst<Identity: IContactAggregationServerPersonCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppserverperson: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IContactAggregationServerPersonCollection_Impl::FindFirst(this) {
                    Ok(ok__) => {
                        ppserverperson.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn FindFirstByServerId<Identity: IContactAggregationServerPersonCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pserverid: windows_core::PCWSTR, ppserverperson: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IContactAggregationServerPersonCollection_Impl::FindFirstByServerId(this, core::mem::transmute(&pserverid)) {
                    Ok(ok__) => {
                        ppserverperson.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn FindFirstByAggregateId<Identity: IContactAggregationServerPersonCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, paggregateid: windows_core::PCWSTR, ppserverperson: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IContactAggregationServerPersonCollection_Impl::FindFirstByAggregateId(this, core::mem::transmute(&paggregateid)) {
                    Ok(ok__) => {
                        ppserverperson.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn FindFirstByLinkedAggregateId<Identity: IContactAggregationServerPersonCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, paggregateid: windows_core::PCWSTR, ppserverperson: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IContactAggregationServerPersonCollection_Impl::FindFirstByLinkedAggregateId(this, core::mem::transmute(&paggregateid)) {
                    Ok(ok__) => {
                        ppserverperson.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn FindNext<Identity: IContactAggregationServerPersonCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppserverperson: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IContactAggregationServerPersonCollection_Impl::FindNext(this) {
                    Ok(ok__) => {
                        ppserverperson.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Count<Identity: IContactAggregationServerPersonCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IContactAggregationServerPersonCollection_Impl::Count(this) {
                    Ok(ok__) => {
                        pcount.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            FindFirst: FindFirst::<Identity, OFFSET>,
            FindFirstByServerId: FindFirstByServerId::<Identity, OFFSET>,
            FindFirstByAggregateId: FindFirstByAggregateId::<Identity, OFFSET>,
            FindFirstByLinkedAggregateId: FindFirstByLinkedAggregateId::<Identity, OFFSET>,
            FindNext: FindNext::<Identity, OFFSET>,
            Count: Count::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IContactAggregationServerPersonCollection as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IContactAggregationServerPersonCollection {}
windows_core::imp::define_interface!(IContactCollection, IContactCollection_Vtbl, 0xb6afa338_d779_11d9_8bde_f66bad1e3f3a);
windows_core::imp::interface_hierarchy!(IContactCollection, windows_core::IUnknown);
impl IContactCollection {
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Next(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn GetCurrent(&self) -> windows_core::Result<IContact> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCurrent)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactCollection_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCurrent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IContactCollection_Impl: windows_core::IUnknownImpl {
    fn Reset(&self) -> windows_core::Result<()>;
    fn Next(&self) -> windows_core::Result<()>;
    fn GetCurrent(&self) -> windows_core::Result<IContact>;
}
impl IContactCollection_Vtbl {
    pub const fn new<Identity: IContactCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Reset<Identity: IContactCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IContactCollection_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Next<Identity: IContactCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IContactCollection_Impl::Next(this).into()
            }
        }
        unsafe extern "system" fn GetCurrent<Identity: IContactCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcontact: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IContactCollection_Impl::GetCurrent(this) {
                    Ok(ok__) => {
                        ppcontact.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Reset: Reset::<Identity, OFFSET>,
            Next: Next::<Identity, OFFSET>,
            GetCurrent: GetCurrent::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IContactCollection as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IContactCollection {}
windows_core::imp::define_interface!(IContactManager, IContactManager_Vtbl, 0xad553d98_deb1_474a_8e17_fc0c2075b738);
windows_core::imp::interface_hierarchy!(IContactManager, windows_core::IUnknown);
impl IContactManager {
    pub unsafe fn Initialize<P0, P1>(&self, pszappname: P0, pszappversion: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), pszappname.param().abi(), pszappversion.param().abi()).ok() }
    }
    pub unsafe fn Load<P0>(&self, pszcontactid: P0) -> windows_core::Result<IContact>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Load)(windows_core::Interface::as_raw(self), pszcontactid.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn MergeContactIDs<P0, P1>(&self, psznewcontactid: P0, pszoldcontactid: P1) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).MergeContactIDs)(windows_core::Interface::as_raw(self), psznewcontactid.param().abi(), pszoldcontactid.param().abi()).ok() }
    }
    pub unsafe fn GetMeContact(&self) -> windows_core::Result<IContact> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMeContact)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetMeContact<P0>(&self, pmecontact: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IContact>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetMeContact)(windows_core::Interface::as_raw(self), pmecontact.param().abi()).ok() }
    }
    pub unsafe fn GetContactCollection(&self) -> windows_core::Result<IContactCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetContactCollection)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub Load: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MergeContactIDs: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetMeContact: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetMeContact: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetContactCollection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IContactManager_Impl: windows_core::IUnknownImpl {
    fn Initialize(&self, pszappname: &windows_core::PCWSTR, pszappversion: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn Load(&self, pszcontactid: &windows_core::PCWSTR) -> windows_core::Result<IContact>;
    fn MergeContactIDs(&self, psznewcontactid: &windows_core::PCWSTR, pszoldcontactid: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetMeContact(&self) -> windows_core::Result<IContact>;
    fn SetMeContact(&self, pmecontact: windows_core::Ref<IContact>) -> windows_core::Result<()>;
    fn GetContactCollection(&self) -> windows_core::Result<IContactCollection>;
}
impl IContactManager_Vtbl {
    pub const fn new<Identity: IContactManager_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Initialize<Identity: IContactManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszappname: windows_core::PCWSTR, pszappversion: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IContactManager_Impl::Initialize(this, core::mem::transmute(&pszappname), core::mem::transmute(&pszappversion)).into()
            }
        }
        unsafe extern "system" fn Load<Identity: IContactManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszcontactid: windows_core::PCWSTR, ppcontact: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IContactManager_Impl::Load(this, core::mem::transmute(&pszcontactid)) {
                    Ok(ok__) => {
                        ppcontact.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MergeContactIDs<Identity: IContactManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psznewcontactid: windows_core::PCWSTR, pszoldcontactid: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IContactManager_Impl::MergeContactIDs(this, core::mem::transmute(&psznewcontactid), core::mem::transmute(&pszoldcontactid)).into()
            }
        }
        unsafe extern "system" fn GetMeContact<Identity: IContactManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppmecontact: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IContactManager_Impl::GetMeContact(this) {
                    Ok(ok__) => {
                        ppmecontact.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetMeContact<Identity: IContactManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmecontact: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IContactManager_Impl::SetMeContact(this, core::mem::transmute_copy(&pmecontact)).into()
            }
        }
        unsafe extern "system" fn GetContactCollection<Identity: IContactManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcontactcollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IContactManager_Impl::GetContactCollection(this) {
                    Ok(ok__) => {
                        ppcontactcollection.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, OFFSET>,
            Load: Load::<Identity, OFFSET>,
            MergeContactIDs: MergeContactIDs::<Identity, OFFSET>,
            GetMeContact: GetMeContact::<Identity, OFFSET>,
            SetMeContact: SetMeContact::<Identity, OFFSET>,
            GetContactCollection: GetContactCollection::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IContactManager as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IContactManager {}
windows_core::imp::define_interface!(IContactProperties, IContactProperties_Vtbl, 0x70dd27dd_5cbd_46e8_bef0_23b6b346288f);
windows_core::imp::interface_hierarchy!(IContactProperties, windows_core::IUnknown);
impl IContactProperties {
    pub unsafe fn GetString<P0>(&self, pszpropertyname: P0, dwflags: u32, pszvalue: &mut [u16], pdwcchpropertyvaluerequired: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetString)(windows_core::Interface::as_raw(self), pszpropertyname.param().abi(), dwflags, core::mem::transmute(pszvalue.as_ptr()), pszvalue.len().try_into().unwrap(), pdwcchpropertyvaluerequired as _).ok() }
    }
    pub unsafe fn GetDate<P0>(&self, pszpropertyname: P0, dwflags: u32, pftdatetime: *mut super::super::Foundation::FILETIME) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetDate)(windows_core::Interface::as_raw(self), pszpropertyname.param().abi(), dwflags, pftdatetime as _).ok() }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetBinary<P0>(&self, pszpropertyname: P0, dwflags: u32, pszcontenttype: &mut [u16], pdwcchcontenttyperequired: *mut u32, ppstream: *mut Option<super::Com::IStream>) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetBinary)(windows_core::Interface::as_raw(self), pszpropertyname.param().abi(), dwflags, core::mem::transmute(pszcontenttype.as_ptr()), pszcontenttype.len().try_into().unwrap(), pdwcchcontenttyperequired as _, core::mem::transmute(ppstream)).ok() }
    }
    pub unsafe fn GetLabels<P0>(&self, pszarrayelementname: P0, dwflags: u32, pszlabels: &mut [u16], pdwcchlabelsrequired: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetLabels)(windows_core::Interface::as_raw(self), pszarrayelementname.param().abi(), dwflags, core::mem::transmute(pszlabels.as_ptr()), pszlabels.len().try_into().unwrap(), pdwcchlabelsrequired as _).ok() }
    }
    pub unsafe fn SetString<P0, P2>(&self, pszpropertyname: P0, dwflags: u32, pszvalue: P2) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetString)(windows_core::Interface::as_raw(self), pszpropertyname.param().abi(), dwflags, pszvalue.param().abi()).ok() }
    }
    pub unsafe fn SetDate<P0>(&self, pszpropertyname: P0, dwflags: u32, ftdatetime: super::super::Foundation::FILETIME) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetDate)(windows_core::Interface::as_raw(self), pszpropertyname.param().abi(), dwflags, core::mem::transmute(ftdatetime)).ok() }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetBinary<P0, P2, P3>(&self, pszpropertyname: P0, dwflags: u32, pszcontenttype: P2, pstream: P3) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<super::Com::IStream>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetBinary)(windows_core::Interface::as_raw(self), pszpropertyname.param().abi(), dwflags, pszcontenttype.param().abi(), pstream.param().abi()).ok() }
    }
    pub unsafe fn SetLabels<P0>(&self, pszarrayelementname: P0, dwflags: u32, ppszlabels: &[windows_core::PCWSTR]) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetLabels)(windows_core::Interface::as_raw(self), pszarrayelementname.param().abi(), dwflags, ppszlabels.len().try_into().unwrap(), core::mem::transmute(ppszlabels.as_ptr())).ok() }
    }
    pub unsafe fn CreateArrayNode<P0>(&self, pszarrayname: P0, dwflags: u32, fappend: bool, psznewarrayelementname: &mut [u16], pdwcchnewarrayelementnamerequired: *mut u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).CreateArrayNode)(windows_core::Interface::as_raw(self), pszarrayname.param().abi(), dwflags, fappend.into(), core::mem::transmute(psznewarrayelementname.as_ptr()), psznewarrayelementname.len().try_into().unwrap(), pdwcchnewarrayelementnamerequired as _).ok() }
    }
    pub unsafe fn DeleteProperty<P0>(&self, pszpropertyname: P0, dwflags: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).DeleteProperty)(windows_core::Interface::as_raw(self), pszpropertyname.param().abi(), dwflags).ok() }
    }
    pub unsafe fn DeleteArrayNode<P0>(&self, pszarrayelementname: P0, dwflags: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).DeleteArrayNode)(windows_core::Interface::as_raw(self), pszarrayelementname.param().abi(), dwflags).ok() }
    }
    pub unsafe fn DeleteLabels<P0>(&self, pszarrayelementname: P0, dwflags: u32) -> windows_core::Result<()>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).DeleteLabels)(windows_core::Interface::as_raw(self), pszarrayelementname.param().abi(), dwflags).ok() }
    }
    pub unsafe fn GetPropertyCollection<P2>(&self, pppropertycollection: *mut Option<IContactPropertyCollection>, dwflags: u32, pszmultivaluename: P2, ppszlabels: &[windows_core::PCWSTR], fanylabelmatches: bool) -> windows_core::Result<()>
    where
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetPropertyCollection)(windows_core::Interface::as_raw(self), core::mem::transmute(pppropertycollection), dwflags, pszmultivaluename.param().abi(), ppszlabels.len().try_into().unwrap(), core::mem::transmute(ppszlabels.as_ptr()), fanylabelmatches.into()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactProperties_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetString: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, windows_core::PWSTR, u32, *mut u32) -> windows_core::HRESULT,
    pub GetDate: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, *mut super::super::Foundation::FILETIME) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetBinary: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, windows_core::PWSTR, u32, *mut u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetBinary: usize,
    pub GetLabels: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, windows_core::PWSTR, u32, *mut u32) -> windows_core::HRESULT,
    pub SetString: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub SetDate: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, super::super::Foundation::FILETIME) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub SetBinary: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, windows_core::PCWSTR, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetBinary: usize,
    pub SetLabels: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, u32, *const windows_core::PCWSTR) -> windows_core::HRESULT,
    pub CreateArrayNode: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, windows_core::BOOL, windows_core::PWSTR, u32, *mut u32) -> windows_core::HRESULT,
    pub DeleteProperty: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32) -> windows_core::HRESULT,
    pub DeleteArrayNode: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32) -> windows_core::HRESULT,
    pub DeleteLabels: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32) -> windows_core::HRESULT,
    pub GetPropertyCollection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, u32, windows_core::PCWSTR, u32, *const windows_core::PCWSTR, windows_core::BOOL) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IContactProperties_Impl: windows_core::IUnknownImpl {
    fn GetString(&self, pszpropertyname: &windows_core::PCWSTR, dwflags: u32, pszvalue: windows_core::PWSTR, cchvalue: u32, pdwcchpropertyvaluerequired: *mut u32) -> windows_core::Result<()>;
    fn GetDate(&self, pszpropertyname: &windows_core::PCWSTR, dwflags: u32, pftdatetime: *mut super::super::Foundation::FILETIME) -> windows_core::Result<()>;
    fn GetBinary(&self, pszpropertyname: &windows_core::PCWSTR, dwflags: u32, pszcontenttype: windows_core::PWSTR, cchcontenttype: u32, pdwcchcontenttyperequired: *mut u32, ppstream: windows_core::OutRef<super::Com::IStream>) -> windows_core::Result<()>;
    fn GetLabels(&self, pszarrayelementname: &windows_core::PCWSTR, dwflags: u32, pszlabels: windows_core::PWSTR, cchlabels: u32, pdwcchlabelsrequired: *mut u32) -> windows_core::Result<()>;
    fn SetString(&self, pszpropertyname: &windows_core::PCWSTR, dwflags: u32, pszvalue: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetDate(&self, pszpropertyname: &windows_core::PCWSTR, dwflags: u32, ftdatetime: &super::super::Foundation::FILETIME) -> windows_core::Result<()>;
    fn SetBinary(&self, pszpropertyname: &windows_core::PCWSTR, dwflags: u32, pszcontenttype: &windows_core::PCWSTR, pstream: windows_core::Ref<super::Com::IStream>) -> windows_core::Result<()>;
    fn SetLabels(&self, pszarrayelementname: &windows_core::PCWSTR, dwflags: u32, dwlabelcount: u32, ppszlabels: *const windows_core::PCWSTR) -> windows_core::Result<()>;
    fn CreateArrayNode(&self, pszarrayname: &windows_core::PCWSTR, dwflags: u32, fappend: windows_core::BOOL, psznewarrayelementname: windows_core::PWSTR, cchnewarrayelementname: u32, pdwcchnewarrayelementnamerequired: *mut u32) -> windows_core::Result<()>;
    fn DeleteProperty(&self, pszpropertyname: &windows_core::PCWSTR, dwflags: u32) -> windows_core::Result<()>;
    fn DeleteArrayNode(&self, pszarrayelementname: &windows_core::PCWSTR, dwflags: u32) -> windows_core::Result<()>;
    fn DeleteLabels(&self, pszarrayelementname: &windows_core::PCWSTR, dwflags: u32) -> windows_core::Result<()>;
    fn GetPropertyCollection(&self, pppropertycollection: windows_core::OutRef<IContactPropertyCollection>, dwflags: u32, pszmultivaluename: &windows_core::PCWSTR, dwlabelcount: u32, ppszlabels: *const windows_core::PCWSTR, fanylabelmatches: windows_core::BOOL) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl IContactProperties_Vtbl {
    pub const fn new<Identity: IContactProperties_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetString<Identity: IContactProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpropertyname: windows_core::PCWSTR, dwflags: u32, pszvalue: windows_core::PWSTR, cchvalue: u32, pdwcchpropertyvaluerequired: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IContactProperties_Impl::GetString(this, core::mem::transmute(&pszpropertyname), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&pszvalue), core::mem::transmute_copy(&cchvalue), core::mem::transmute_copy(&pdwcchpropertyvaluerequired)).into()
            }
        }
        unsafe extern "system" fn GetDate<Identity: IContactProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpropertyname: windows_core::PCWSTR, dwflags: u32, pftdatetime: *mut super::super::Foundation::FILETIME) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IContactProperties_Impl::GetDate(this, core::mem::transmute(&pszpropertyname), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&pftdatetime)).into()
            }
        }
        unsafe extern "system" fn GetBinary<Identity: IContactProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpropertyname: windows_core::PCWSTR, dwflags: u32, pszcontenttype: windows_core::PWSTR, cchcontenttype: u32, pdwcchcontenttyperequired: *mut u32, ppstream: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IContactProperties_Impl::GetBinary(this, core::mem::transmute(&pszpropertyname), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&pszcontenttype), core::mem::transmute_copy(&cchcontenttype), core::mem::transmute_copy(&pdwcchcontenttyperequired), core::mem::transmute_copy(&ppstream)).into()
            }
        }
        unsafe extern "system" fn GetLabels<Identity: IContactProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszarrayelementname: windows_core::PCWSTR, dwflags: u32, pszlabels: windows_core::PWSTR, cchlabels: u32, pdwcchlabelsrequired: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IContactProperties_Impl::GetLabels(this, core::mem::transmute(&pszarrayelementname), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&pszlabels), core::mem::transmute_copy(&cchlabels), core::mem::transmute_copy(&pdwcchlabelsrequired)).into()
            }
        }
        unsafe extern "system" fn SetString<Identity: IContactProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpropertyname: windows_core::PCWSTR, dwflags: u32, pszvalue: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IContactProperties_Impl::SetString(this, core::mem::transmute(&pszpropertyname), core::mem::transmute_copy(&dwflags), core::mem::transmute(&pszvalue)).into()
            }
        }
        unsafe extern "system" fn SetDate<Identity: IContactProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpropertyname: windows_core::PCWSTR, dwflags: u32, ftdatetime: super::super::Foundation::FILETIME) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IContactProperties_Impl::SetDate(this, core::mem::transmute(&pszpropertyname), core::mem::transmute_copy(&dwflags), core::mem::transmute(&ftdatetime)).into()
            }
        }
        unsafe extern "system" fn SetBinary<Identity: IContactProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpropertyname: windows_core::PCWSTR, dwflags: u32, pszcontenttype: windows_core::PCWSTR, pstream: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IContactProperties_Impl::SetBinary(this, core::mem::transmute(&pszpropertyname), core::mem::transmute_copy(&dwflags), core::mem::transmute(&pszcontenttype), core::mem::transmute_copy(&pstream)).into()
            }
        }
        unsafe extern "system" fn SetLabels<Identity: IContactProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszarrayelementname: windows_core::PCWSTR, dwflags: u32, dwlabelcount: u32, ppszlabels: *const windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IContactProperties_Impl::SetLabels(this, core::mem::transmute(&pszarrayelementname), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&dwlabelcount), core::mem::transmute_copy(&ppszlabels)).into()
            }
        }
        unsafe extern "system" fn CreateArrayNode<Identity: IContactProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszarrayname: windows_core::PCWSTR, dwflags: u32, fappend: windows_core::BOOL, psznewarrayelementname: windows_core::PWSTR, cchnewarrayelementname: u32, pdwcchnewarrayelementnamerequired: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IContactProperties_Impl::CreateArrayNode(this, core::mem::transmute(&pszarrayname), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&fappend), core::mem::transmute_copy(&psznewarrayelementname), core::mem::transmute_copy(&cchnewarrayelementname), core::mem::transmute_copy(&pdwcchnewarrayelementnamerequired)).into()
            }
        }
        unsafe extern "system" fn DeleteProperty<Identity: IContactProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpropertyname: windows_core::PCWSTR, dwflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IContactProperties_Impl::DeleteProperty(this, core::mem::transmute(&pszpropertyname), core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn DeleteArrayNode<Identity: IContactProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszarrayelementname: windows_core::PCWSTR, dwflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IContactProperties_Impl::DeleteArrayNode(this, core::mem::transmute(&pszarrayelementname), core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn DeleteLabels<Identity: IContactProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszarrayelementname: windows_core::PCWSTR, dwflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IContactProperties_Impl::DeleteLabels(this, core::mem::transmute(&pszarrayelementname), core::mem::transmute_copy(&dwflags)).into()
            }
        }
        unsafe extern "system" fn GetPropertyCollection<Identity: IContactProperties_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pppropertycollection: *mut *mut core::ffi::c_void, dwflags: u32, pszmultivaluename: windows_core::PCWSTR, dwlabelcount: u32, ppszlabels: *const windows_core::PCWSTR, fanylabelmatches: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IContactProperties_Impl::GetPropertyCollection(this, core::mem::transmute_copy(&pppropertycollection), core::mem::transmute_copy(&dwflags), core::mem::transmute(&pszmultivaluename), core::mem::transmute_copy(&dwlabelcount), core::mem::transmute_copy(&ppszlabels), core::mem::transmute_copy(&fanylabelmatches)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetString: GetString::<Identity, OFFSET>,
            GetDate: GetDate::<Identity, OFFSET>,
            GetBinary: GetBinary::<Identity, OFFSET>,
            GetLabels: GetLabels::<Identity, OFFSET>,
            SetString: SetString::<Identity, OFFSET>,
            SetDate: SetDate::<Identity, OFFSET>,
            SetBinary: SetBinary::<Identity, OFFSET>,
            SetLabels: SetLabels::<Identity, OFFSET>,
            CreateArrayNode: CreateArrayNode::<Identity, OFFSET>,
            DeleteProperty: DeleteProperty::<Identity, OFFSET>,
            DeleteArrayNode: DeleteArrayNode::<Identity, OFFSET>,
            DeleteLabels: DeleteLabels::<Identity, OFFSET>,
            GetPropertyCollection: GetPropertyCollection::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IContactProperties as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IContactProperties {}
windows_core::imp::define_interface!(IContactPropertyCollection, IContactPropertyCollection_Vtbl, 0xffd3adf8_fa64_4328_b1b6_2e0db509cb3c);
windows_core::imp::interface_hierarchy!(IContactPropertyCollection, windows_core::IUnknown);
impl IContactPropertyCollection {
    pub unsafe fn Reset(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn Next(&self) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).Next)(windows_core::Interface::as_raw(self)).ok() }
    }
    pub unsafe fn GetPropertyName(&self, pszpropertyname: &mut [u16], pdwcchpropertynamerequired: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetPropertyName)(windows_core::Interface::as_raw(self), core::mem::transmute(pszpropertyname.as_ptr()), pszpropertyname.len().try_into().unwrap(), pdwcchpropertynamerequired as _).ok() }
    }
    pub unsafe fn GetPropertyType(&self, pdwtype: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetPropertyType)(windows_core::Interface::as_raw(self), pdwtype as _).ok() }
    }
    pub unsafe fn GetPropertyVersion(&self, pdwversion: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetPropertyVersion)(windows_core::Interface::as_raw(self), pdwversion as _).ok() }
    }
    pub unsafe fn GetPropertyModificationDate(&self, pftmodificationdate: *mut super::super::Foundation::FILETIME) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetPropertyModificationDate)(windows_core::Interface::as_raw(self), pftmodificationdate as _).ok() }
    }
    pub unsafe fn GetPropertyArrayElementID(&self, pszarrayelementid: &mut [u16], pdwccharrayelementidrequired: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetPropertyArrayElementID)(windows_core::Interface::as_raw(self), core::mem::transmute(pszarrayelementid.as_ptr()), pszarrayelementid.len().try_into().unwrap(), pdwccharrayelementidrequired as _).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactPropertyCollection_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Next: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetPropertyName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PWSTR, u32, *mut u32) -> windows_core::HRESULT,
    pub GetPropertyType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetPropertyVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetPropertyModificationDate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::Foundation::FILETIME) -> windows_core::HRESULT,
    pub GetPropertyArrayElementID: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PWSTR, u32, *mut u32) -> windows_core::HRESULT,
}
pub trait IContactPropertyCollection_Impl: windows_core::IUnknownImpl {
    fn Reset(&self) -> windows_core::Result<()>;
    fn Next(&self) -> windows_core::Result<()>;
    fn GetPropertyName(&self, pszpropertyname: windows_core::PWSTR, cchpropertyname: u32, pdwcchpropertynamerequired: *mut u32) -> windows_core::Result<()>;
    fn GetPropertyType(&self, pdwtype: *mut u32) -> windows_core::Result<()>;
    fn GetPropertyVersion(&self, pdwversion: *mut u32) -> windows_core::Result<()>;
    fn GetPropertyModificationDate(&self, pftmodificationdate: *mut super::super::Foundation::FILETIME) -> windows_core::Result<()>;
    fn GetPropertyArrayElementID(&self, pszarrayelementid: windows_core::PWSTR, ccharrayelementid: u32, pdwccharrayelementidrequired: *mut u32) -> windows_core::Result<()>;
}
impl IContactPropertyCollection_Vtbl {
    pub const fn new<Identity: IContactPropertyCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Reset<Identity: IContactPropertyCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IContactPropertyCollection_Impl::Reset(this).into()
            }
        }
        unsafe extern "system" fn Next<Identity: IContactPropertyCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IContactPropertyCollection_Impl::Next(this).into()
            }
        }
        unsafe extern "system" fn GetPropertyName<Identity: IContactPropertyCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpropertyname: windows_core::PWSTR, cchpropertyname: u32, pdwcchpropertynamerequired: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IContactPropertyCollection_Impl::GetPropertyName(this, core::mem::transmute_copy(&pszpropertyname), core::mem::transmute_copy(&cchpropertyname), core::mem::transmute_copy(&pdwcchpropertynamerequired)).into()
            }
        }
        unsafe extern "system" fn GetPropertyType<Identity: IContactPropertyCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwtype: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IContactPropertyCollection_Impl::GetPropertyType(this, core::mem::transmute_copy(&pdwtype)).into()
            }
        }
        unsafe extern "system" fn GetPropertyVersion<Identity: IContactPropertyCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwversion: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IContactPropertyCollection_Impl::GetPropertyVersion(this, core::mem::transmute_copy(&pdwversion)).into()
            }
        }
        unsafe extern "system" fn GetPropertyModificationDate<Identity: IContactPropertyCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pftmodificationdate: *mut super::super::Foundation::FILETIME) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IContactPropertyCollection_Impl::GetPropertyModificationDate(this, core::mem::transmute_copy(&pftmodificationdate)).into()
            }
        }
        unsafe extern "system" fn GetPropertyArrayElementID<Identity: IContactPropertyCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszarrayelementid: windows_core::PWSTR, ccharrayelementid: u32, pdwccharrayelementidrequired: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IContactPropertyCollection_Impl::GetPropertyArrayElementID(this, core::mem::transmute_copy(&pszarrayelementid), core::mem::transmute_copy(&ccharrayelementid), core::mem::transmute_copy(&pdwccharrayelementidrequired)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Reset: Reset::<Identity, OFFSET>,
            Next: Next::<Identity, OFFSET>,
            GetPropertyName: GetPropertyName::<Identity, OFFSET>,
            GetPropertyType: GetPropertyType::<Identity, OFFSET>,
            GetPropertyVersion: GetPropertyVersion::<Identity, OFFSET>,
            GetPropertyModificationDate: GetPropertyModificationDate::<Identity, OFFSET>,
            GetPropertyArrayElementID: GetPropertyArrayElementID::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IContactPropertyCollection as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IContactPropertyCollection {}
