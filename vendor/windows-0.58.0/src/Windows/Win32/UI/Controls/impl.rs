#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IImageList_Impl: Sized {
    fn Add(&self, hbmimage: super::super::Graphics::Gdi::HBITMAP, hbmmask: super::super::Graphics::Gdi::HBITMAP) -> windows_core::Result<i32>;
    fn ReplaceIcon(&self, i: i32, hicon: super::WindowsAndMessaging::HICON) -> windows_core::Result<i32>;
    fn SetOverlayImage(&self, iimage: i32, ioverlay: i32) -> windows_core::Result<()>;
    fn Replace(&self, i: i32, hbmimage: super::super::Graphics::Gdi::HBITMAP, hbmmask: super::super::Graphics::Gdi::HBITMAP) -> windows_core::Result<()>;
    fn AddMasked(&self, hbmimage: super::super::Graphics::Gdi::HBITMAP, crmask: super::super::Foundation::COLORREF) -> windows_core::Result<i32>;
    fn Draw(&self, pimldp: *const IMAGELISTDRAWPARAMS) -> windows_core::Result<()>;
    fn Remove(&self, i: i32) -> windows_core::Result<()>;
    fn GetIcon(&self, i: i32, flags: u32) -> windows_core::Result<super::WindowsAndMessaging::HICON>;
    fn GetImageInfo(&self, i: i32, pimageinfo: *mut IMAGEINFO) -> windows_core::Result<()>;
    fn Copy(&self, idst: i32, punksrc: Option<&windows_core::IUnknown>, isrc: i32, uflags: u32) -> windows_core::Result<()>;
    fn Merge(&self, i1: i32, punk2: Option<&windows_core::IUnknown>, i2: i32, dx: i32, dy: i32, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn Clone(&self, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetImageRect(&self, i: i32) -> windows_core::Result<super::super::Foundation::RECT>;
    fn GetIconSize(&self, cx: *mut i32, cy: *mut i32) -> windows_core::Result<()>;
    fn SetIconSize(&self, cx: i32, cy: i32) -> windows_core::Result<()>;
    fn GetImageCount(&self) -> windows_core::Result<i32>;
    fn SetImageCount(&self, unewcount: u32) -> windows_core::Result<()>;
    fn SetBkColor(&self, clrbk: super::super::Foundation::COLORREF) -> windows_core::Result<super::super::Foundation::COLORREF>;
    fn GetBkColor(&self) -> windows_core::Result<super::super::Foundation::COLORREF>;
    fn BeginDrag(&self, itrack: i32, dxhotspot: i32, dyhotspot: i32) -> windows_core::Result<()>;
    fn EndDrag(&self) -> windows_core::Result<()>;
    fn DragEnter(&self, hwndlock: super::super::Foundation::HWND, x: i32, y: i32) -> windows_core::Result<()>;
    fn DragLeave(&self, hwndlock: super::super::Foundation::HWND) -> windows_core::Result<()>;
    fn DragMove(&self, x: i32, y: i32) -> windows_core::Result<()>;
    fn SetDragCursorImage(&self, punk: Option<&windows_core::IUnknown>, idrag: i32, dxhotspot: i32, dyhotspot: i32) -> windows_core::Result<()>;
    fn DragShowNolock(&self, fshow: super::super::Foundation::BOOL) -> windows_core::Result<()>;
    fn GetDragImage(&self, ppt: *mut super::super::Foundation::POINT, ppthotspot: *mut super::super::Foundation::POINT, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetItemFlags(&self, i: i32) -> windows_core::Result<IMAGE_LIST_ITEM_FLAGS>;
    fn GetOverlayImage(&self, ioverlay: i32) -> windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl windows_core::RuntimeName for IImageList {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl IImageList_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IImageList_Vtbl
    where
        Identity: IImageList_Impl,
    {
        unsafe extern "system" fn Add<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hbmimage: super::super::Graphics::Gdi::HBITMAP, hbmmask: super::super::Graphics::Gdi::HBITMAP, pi: *mut i32) -> windows_core::HRESULT
        where
            Identity: IImageList_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IImageList_Impl::Add(this, core::mem::transmute_copy(&hbmimage), core::mem::transmute_copy(&hbmmask)) {
                Ok(ok__) => {
                    pi.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReplaceIcon<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, i: i32, hicon: super::WindowsAndMessaging::HICON, pi: *mut i32) -> windows_core::HRESULT
        where
            Identity: IImageList_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IImageList_Impl::ReplaceIcon(this, core::mem::transmute_copy(&i), core::mem::transmute_copy(&hicon)) {
                Ok(ok__) => {
                    pi.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOverlayImage<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, iimage: i32, ioverlay: i32) -> windows_core::HRESULT
        where
            Identity: IImageList_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IImageList_Impl::SetOverlayImage(this, core::mem::transmute_copy(&iimage), core::mem::transmute_copy(&ioverlay)).into()
        }
        unsafe extern "system" fn Replace<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, i: i32, hbmimage: super::super::Graphics::Gdi::HBITMAP, hbmmask: super::super::Graphics::Gdi::HBITMAP) -> windows_core::HRESULT
        where
            Identity: IImageList_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IImageList_Impl::Replace(this, core::mem::transmute_copy(&i), core::mem::transmute_copy(&hbmimage), core::mem::transmute_copy(&hbmmask)).into()
        }
        unsafe extern "system" fn AddMasked<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hbmimage: super::super::Graphics::Gdi::HBITMAP, crmask: super::super::Foundation::COLORREF, pi: *mut i32) -> windows_core::HRESULT
        where
            Identity: IImageList_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IImageList_Impl::AddMasked(this, core::mem::transmute_copy(&hbmimage), core::mem::transmute_copy(&crmask)) {
                Ok(ok__) => {
                    pi.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Draw<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pimldp: *const IMAGELISTDRAWPARAMS) -> windows_core::HRESULT
        where
            Identity: IImageList_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IImageList_Impl::Draw(this, core::mem::transmute_copy(&pimldp)).into()
        }
        unsafe extern "system" fn Remove<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, i: i32) -> windows_core::HRESULT
        where
            Identity: IImageList_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IImageList_Impl::Remove(this, core::mem::transmute_copy(&i)).into()
        }
        unsafe extern "system" fn GetIcon<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, i: i32, flags: u32, picon: *mut super::WindowsAndMessaging::HICON) -> windows_core::HRESULT
        where
            Identity: IImageList_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IImageList_Impl::GetIcon(this, core::mem::transmute_copy(&i), core::mem::transmute_copy(&flags)) {
                Ok(ok__) => {
                    picon.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetImageInfo<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, i: i32, pimageinfo: *mut IMAGEINFO) -> windows_core::HRESULT
        where
            Identity: IImageList_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IImageList_Impl::GetImageInfo(this, core::mem::transmute_copy(&i), core::mem::transmute_copy(&pimageinfo)).into()
        }
        unsafe extern "system" fn Copy<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, idst: i32, punksrc: *mut core::ffi::c_void, isrc: i32, uflags: u32) -> windows_core::HRESULT
        where
            Identity: IImageList_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IImageList_Impl::Copy(this, core::mem::transmute_copy(&idst), windows_core::from_raw_borrowed(&punksrc), core::mem::transmute_copy(&isrc), core::mem::transmute_copy(&uflags)).into()
        }
        unsafe extern "system" fn Merge<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, i1: i32, punk2: *mut core::ffi::c_void, i2: i32, dx: i32, dy: i32, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IImageList_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IImageList_Impl::Merge(this, core::mem::transmute_copy(&i1), windows_core::from_raw_borrowed(&punk2), core::mem::transmute_copy(&i2), core::mem::transmute_copy(&dx), core::mem::transmute_copy(&dy), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn Clone<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IImageList_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IImageList_Impl::Clone(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn GetImageRect<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, i: i32, prc: *mut super::super::Foundation::RECT) -> windows_core::HRESULT
        where
            Identity: IImageList_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IImageList_Impl::GetImageRect(this, core::mem::transmute_copy(&i)) {
                Ok(ok__) => {
                    prc.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetIconSize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cx: *mut i32, cy: *mut i32) -> windows_core::HRESULT
        where
            Identity: IImageList_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IImageList_Impl::GetIconSize(this, core::mem::transmute_copy(&cx), core::mem::transmute_copy(&cy)).into()
        }
        unsafe extern "system" fn SetIconSize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cx: i32, cy: i32) -> windows_core::HRESULT
        where
            Identity: IImageList_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IImageList_Impl::SetIconSize(this, core::mem::transmute_copy(&cx), core::mem::transmute_copy(&cy)).into()
        }
        unsafe extern "system" fn GetImageCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pi: *mut i32) -> windows_core::HRESULT
        where
            Identity: IImageList_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IImageList_Impl::GetImageCount(this) {
                Ok(ok__) => {
                    pi.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetImageCount<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, unewcount: u32) -> windows_core::HRESULT
        where
            Identity: IImageList_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IImageList_Impl::SetImageCount(this, core::mem::transmute_copy(&unewcount)).into()
        }
        unsafe extern "system" fn SetBkColor<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, clrbk: super::super::Foundation::COLORREF, pclr: *mut super::super::Foundation::COLORREF) -> windows_core::HRESULT
        where
            Identity: IImageList_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IImageList_Impl::SetBkColor(this, core::mem::transmute_copy(&clrbk)) {
                Ok(ok__) => {
                    pclr.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBkColor<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pclr: *mut super::super::Foundation::COLORREF) -> windows_core::HRESULT
        where
            Identity: IImageList_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IImageList_Impl::GetBkColor(this) {
                Ok(ok__) => {
                    pclr.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BeginDrag<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, itrack: i32, dxhotspot: i32, dyhotspot: i32) -> windows_core::HRESULT
        where
            Identity: IImageList_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IImageList_Impl::BeginDrag(this, core::mem::transmute_copy(&itrack), core::mem::transmute_copy(&dxhotspot), core::mem::transmute_copy(&dyhotspot)).into()
        }
        unsafe extern "system" fn EndDrag<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IImageList_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IImageList_Impl::EndDrag(this).into()
        }
        unsafe extern "system" fn DragEnter<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndlock: super::super::Foundation::HWND, x: i32, y: i32) -> windows_core::HRESULT
        where
            Identity: IImageList_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IImageList_Impl::DragEnter(this, core::mem::transmute_copy(&hwndlock), core::mem::transmute_copy(&x), core::mem::transmute_copy(&y)).into()
        }
        unsafe extern "system" fn DragLeave<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwndlock: super::super::Foundation::HWND) -> windows_core::HRESULT
        where
            Identity: IImageList_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IImageList_Impl::DragLeave(this, core::mem::transmute_copy(&hwndlock)).into()
        }
        unsafe extern "system" fn DragMove<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, x: i32, y: i32) -> windows_core::HRESULT
        where
            Identity: IImageList_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IImageList_Impl::DragMove(this, core::mem::transmute_copy(&x), core::mem::transmute_copy(&y)).into()
        }
        unsafe extern "system" fn SetDragCursorImage<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, punk: *mut core::ffi::c_void, idrag: i32, dxhotspot: i32, dyhotspot: i32) -> windows_core::HRESULT
        where
            Identity: IImageList_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IImageList_Impl::SetDragCursorImage(this, windows_core::from_raw_borrowed(&punk), core::mem::transmute_copy(&idrag), core::mem::transmute_copy(&dxhotspot), core::mem::transmute_copy(&dyhotspot)).into()
        }
        unsafe extern "system" fn DragShowNolock<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, fshow: super::super::Foundation::BOOL) -> windows_core::HRESULT
        where
            Identity: IImageList_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IImageList_Impl::DragShowNolock(this, core::mem::transmute_copy(&fshow)).into()
        }
        unsafe extern "system" fn GetDragImage<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppt: *mut super::super::Foundation::POINT, ppthotspot: *mut super::super::Foundation::POINT, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IImageList_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IImageList_Impl::GetDragImage(this, core::mem::transmute_copy(&ppt), core::mem::transmute_copy(&ppthotspot), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn GetItemFlags<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, i: i32, dwflags: *mut IMAGE_LIST_ITEM_FLAGS) -> windows_core::HRESULT
        where
            Identity: IImageList_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IImageList_Impl::GetItemFlags(this, core::mem::transmute_copy(&i)) {
                Ok(ok__) => {
                    dwflags.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOverlayImage<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ioverlay: i32, piindex: *mut i32) -> windows_core::HRESULT
        where
            Identity: IImageList_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            match IImageList_Impl::GetOverlayImage(this, core::mem::transmute_copy(&ioverlay)) {
                Ok(ok__) => {
                    piindex.write(core::mem::transmute(ok__));
                    windows_core::HRESULT(0)
                }
                Err(err) => err.into(),
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Add: Add::<Identity, OFFSET>,
            ReplaceIcon: ReplaceIcon::<Identity, OFFSET>,
            SetOverlayImage: SetOverlayImage::<Identity, OFFSET>,
            Replace: Replace::<Identity, OFFSET>,
            AddMasked: AddMasked::<Identity, OFFSET>,
            Draw: Draw::<Identity, OFFSET>,
            Remove: Remove::<Identity, OFFSET>,
            GetIcon: GetIcon::<Identity, OFFSET>,
            GetImageInfo: GetImageInfo::<Identity, OFFSET>,
            Copy: Copy::<Identity, OFFSET>,
            Merge: Merge::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
            GetImageRect: GetImageRect::<Identity, OFFSET>,
            GetIconSize: GetIconSize::<Identity, OFFSET>,
            SetIconSize: SetIconSize::<Identity, OFFSET>,
            GetImageCount: GetImageCount::<Identity, OFFSET>,
            SetImageCount: SetImageCount::<Identity, OFFSET>,
            SetBkColor: SetBkColor::<Identity, OFFSET>,
            GetBkColor: GetBkColor::<Identity, OFFSET>,
            BeginDrag: BeginDrag::<Identity, OFFSET>,
            EndDrag: EndDrag::<Identity, OFFSET>,
            DragEnter: DragEnter::<Identity, OFFSET>,
            DragLeave: DragLeave::<Identity, OFFSET>,
            DragMove: DragMove::<Identity, OFFSET>,
            SetDragCursorImage: SetDragCursorImage::<Identity, OFFSET>,
            DragShowNolock: DragShowNolock::<Identity, OFFSET>,
            GetDragImage: GetDragImage::<Identity, OFFSET>,
            GetItemFlags: GetItemFlags::<Identity, OFFSET>,
            GetOverlayImage: GetOverlayImage::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IImageList as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
pub trait IImageList2_Impl: Sized + IImageList_Impl {
    fn Resize(&self, cxnewiconsize: i32, cynewiconsize: i32) -> windows_core::Result<()>;
    fn GetOriginalSize(&self, iimage: i32, dwflags: u32, pcx: *mut i32, pcy: *mut i32) -> windows_core::Result<()>;
    fn SetOriginalSize(&self, iimage: i32, cx: i32, cy: i32) -> windows_core::Result<()>;
    fn SetCallback(&self, punk: Option<&windows_core::IUnknown>) -> windows_core::Result<()>;
    fn GetCallback(&self, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn ForceImagePresent(&self, iimage: i32, dwflags: u32) -> windows_core::Result<()>;
    fn DiscardImages(&self, ifirstimage: i32, ilastimage: i32, dwflags: u32) -> windows_core::Result<()>;
    fn PreloadImages(&self, pimldp: *const IMAGELISTDRAWPARAMS) -> windows_core::Result<()>;
    fn GetStatistics(&self, pils: *mut IMAGELISTSTATS) -> windows_core::Result<()>;
    fn Initialize(&self, cx: i32, cy: i32, flags: IMAGELIST_CREATION_FLAGS, cinitial: i32, cgrow: i32) -> windows_core::Result<()>;
    fn Replace2(&self, i: i32, hbmimage: super::super::Graphics::Gdi::HBITMAP, hbmmask: super::super::Graphics::Gdi::HBITMAP, punk: Option<&windows_core::IUnknown>, dwflags: u32) -> windows_core::Result<()>;
    fn ReplaceFromImageList(&self, i: i32, pil: Option<&IImageList>, isrc: i32, punk: Option<&windows_core::IUnknown>, dwflags: u32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl windows_core::RuntimeName for IImageList2 {}
#[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
impl IImageList2_Vtbl {
    pub const fn new<Identity: windows_core::IUnknownImpl, const OFFSET: isize>() -> IImageList2_Vtbl
    where
        Identity: IImageList2_Impl,
    {
        unsafe extern "system" fn Resize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cxnewiconsize: i32, cynewiconsize: i32) -> windows_core::HRESULT
        where
            Identity: IImageList2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IImageList2_Impl::Resize(this, core::mem::transmute_copy(&cxnewiconsize), core::mem::transmute_copy(&cynewiconsize)).into()
        }
        unsafe extern "system" fn GetOriginalSize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, iimage: i32, dwflags: u32, pcx: *mut i32, pcy: *mut i32) -> windows_core::HRESULT
        where
            Identity: IImageList2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IImageList2_Impl::GetOriginalSize(this, core::mem::transmute_copy(&iimage), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&pcx), core::mem::transmute_copy(&pcy)).into()
        }
        unsafe extern "system" fn SetOriginalSize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, iimage: i32, cx: i32, cy: i32) -> windows_core::HRESULT
        where
            Identity: IImageList2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IImageList2_Impl::SetOriginalSize(this, core::mem::transmute_copy(&iimage), core::mem::transmute_copy(&cx), core::mem::transmute_copy(&cy)).into()
        }
        unsafe extern "system" fn SetCallback<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, punk: *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IImageList2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IImageList2_Impl::SetCallback(this, windows_core::from_raw_borrowed(&punk)).into()
        }
        unsafe extern "system" fn GetCallback<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
        where
            Identity: IImageList2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IImageList2_Impl::GetCallback(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
        }
        unsafe extern "system" fn ForceImagePresent<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, iimage: i32, dwflags: u32) -> windows_core::HRESULT
        where
            Identity: IImageList2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IImageList2_Impl::ForceImagePresent(this, core::mem::transmute_copy(&iimage), core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn DiscardImages<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, ifirstimage: i32, ilastimage: i32, dwflags: u32) -> windows_core::HRESULT
        where
            Identity: IImageList2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IImageList2_Impl::DiscardImages(this, core::mem::transmute_copy(&ifirstimage), core::mem::transmute_copy(&ilastimage), core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn PreloadImages<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pimldp: *const IMAGELISTDRAWPARAMS) -> windows_core::HRESULT
        where
            Identity: IImageList2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IImageList2_Impl::PreloadImages(this, core::mem::transmute_copy(&pimldp)).into()
        }
        unsafe extern "system" fn GetStatistics<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, pils: *mut IMAGELISTSTATS) -> windows_core::HRESULT
        where
            Identity: IImageList2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IImageList2_Impl::GetStatistics(this, core::mem::transmute_copy(&pils)).into()
        }
        unsafe extern "system" fn Initialize<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, cx: i32, cy: i32, flags: IMAGELIST_CREATION_FLAGS, cinitial: i32, cgrow: i32) -> windows_core::HRESULT
        where
            Identity: IImageList2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IImageList2_Impl::Initialize(this, core::mem::transmute_copy(&cx), core::mem::transmute_copy(&cy), core::mem::transmute_copy(&flags), core::mem::transmute_copy(&cinitial), core::mem::transmute_copy(&cgrow)).into()
        }
        unsafe extern "system" fn Replace2<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, i: i32, hbmimage: super::super::Graphics::Gdi::HBITMAP, hbmmask: super::super::Graphics::Gdi::HBITMAP, punk: *mut core::ffi::c_void, dwflags: u32) -> windows_core::HRESULT
        where
            Identity: IImageList2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IImageList2_Impl::Replace2(this, core::mem::transmute_copy(&i), core::mem::transmute_copy(&hbmimage), core::mem::transmute_copy(&hbmmask), windows_core::from_raw_borrowed(&punk), core::mem::transmute_copy(&dwflags)).into()
        }
        unsafe extern "system" fn ReplaceFromImageList<Identity: windows_core::IUnknownImpl, const OFFSET: isize>(this: *mut core::ffi::c_void, i: i32, pil: *mut core::ffi::c_void, isrc: i32, punk: *mut core::ffi::c_void, dwflags: u32) -> windows_core::HRESULT
        where
            Identity: IImageList2_Impl,
        {
            let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
            IImageList2_Impl::ReplaceFromImageList(this, core::mem::transmute_copy(&i), windows_core::from_raw_borrowed(&pil), core::mem::transmute_copy(&isrc), windows_core::from_raw_borrowed(&punk), core::mem::transmute_copy(&dwflags)).into()
        }
        Self {
            base__: IImageList_Vtbl::new::<Identity, OFFSET>(),
            Resize: Resize::<Identity, OFFSET>,
            GetOriginalSize: GetOriginalSize::<Identity, OFFSET>,
            SetOriginalSize: SetOriginalSize::<Identity, OFFSET>,
            SetCallback: SetCallback::<Identity, OFFSET>,
            GetCallback: GetCallback::<Identity, OFFSET>,
            ForceImagePresent: ForceImagePresent::<Identity, OFFSET>,
            DiscardImages: DiscardImages::<Identity, OFFSET>,
            PreloadImages: PreloadImages::<Identity, OFFSET>,
            GetStatistics: GetStatistics::<Identity, OFFSET>,
            Initialize: Initialize::<Identity, OFFSET>,
            Replace2: Replace2::<Identity, OFFSET>,
            ReplaceFromImageList: ReplaceFromImageList::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IImageList2 as windows_core::Interface>::IID || iid == &<IImageList as windows_core::Interface>::IID
    }
}
