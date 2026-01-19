use crate::{Error, TrashContext, TrashItem, TrashItemMetadata, TrashItemSize};
use std::{
    borrow::Borrow,
    ffi::{c_void, OsStr, OsString},
    os::windows::{ffi::OsStrExt, prelude::*},
    path::PathBuf,
};
use windows::Win32::{
    Foundation::*, Storage::EnhancedStorage::*, System::Com::*, System::SystemServices::*,
    UI::Shell::PropertiesSystem::*, UI::Shell::*,
};
use windows::{
    core::{Interface, PCWSTR, PWSTR},
    Win32::System::Com::StructuredStorage::PropVariantToBSTR,
};

const SCID_ORIGINAL_LOCATION: PROPERTYKEY = PROPERTYKEY { fmtid: PSGUID_DISPLACED, pid: PID_DISPLACED_FROM };
const SCID_DATE_DELETED: PROPERTYKEY = PROPERTYKEY { fmtid: PSGUID_DISPLACED, pid: PID_DISPLACED_DATE };

impl From<windows::core::Error> for Error {
    fn from(err: windows::core::Error) -> Error {
        Error::Os { code: err.code().0, description: format!("windows error: {err}") }
    }
}

fn to_wide_path(path: impl AsRef<OsStr>) -> Vec<u16> {
    path.as_ref().encode_wide().chain(std::iter::once(0)).collect()
}

#[derive(Clone, Default, Debug)]
pub struct PlatformTrashContext;
impl PlatformTrashContext {
    pub const fn new() -> Self {
        PlatformTrashContext
    }
}
impl TrashContext {
    /// See https://docs.microsoft.com/en-us/windows/win32/api/shellapi/ns-shellapi-_shfileopstructa
    pub(crate) fn delete_specified_canonicalized(&self, full_paths: Vec<PathBuf>) -> Result<(), Error> {
        ensure_com_initialized();
        unsafe {
            let pfo: IFileOperation = CoCreateInstance(&FileOperation as *const _, None, CLSCTX_ALL).unwrap();

            pfo.SetOperationFlags(FOF_NO_UI | FOF_ALLOWUNDO | FOF_WANTNUKEWARNING)?;

            for full_path in full_paths.iter() {
                let path_prefix = ['\\' as u16, '\\' as u16, '?' as u16, '\\' as u16];
                let wide_path_container = to_wide_path(full_path);
                let wide_path_slice = if wide_path_container.starts_with(&path_prefix) {
                    &wide_path_container[path_prefix.len()..]
                } else {
                    &wide_path_container[0..]
                };

                let shi: IShellItem = SHCreateItemFromParsingName(PCWSTR(wide_path_slice.as_ptr()), None)?;

                pfo.DeleteItem(&shi, None)?;
            }
            pfo.PerformOperations()?;

            // https://learn.microsoft.com/en-us/windows/win32/api/shobjidl_core/nf-shobjidl_core-ifileoperation-performoperations
            // this method can still return a success code. Use the GetAnyOperationsAborted method to determine if this was the case.
            if pfo.GetAnyOperationsAborted()?.as_bool() {
                // TODO: return the reason why the operation was aborted.
                // We may retrieve reason from the IFileOperationProgressSink but
                // the list of HRESULT codes is not documented.
                return Err(Error::Unknown { description: "Some operations were aborted".into() });
            }
            Ok(())
        }
    }

    /// Removes all files and folder paths recursively.
    pub(crate) fn delete_all_canonicalized(&self, full_paths: Vec<PathBuf>) -> Result<(), Error> {
        self.delete_specified_canonicalized(full_paths)?;
        Ok(())
    }
}

pub fn list() -> Result<Vec<TrashItem>, Error> {
    ensure_com_initialized();
    unsafe {
        let mut item_vec = Vec::new();

        let recycle_bin: IShellItem =
            SHGetKnownFolderItem(&FOLDERID_RecycleBinFolder, KF_FLAG_DEFAULT, HANDLE::default())?;

        let pesi: IEnumShellItems = recycle_bin.BindToHandler(None, &BHID_EnumItems)?;

        loop {
            let mut fetched_count: u32 = 0;
            let mut arr = [None];
            pesi.Next(&mut arr, Some(&mut fetched_count as *mut u32))?;

            if fetched_count == 0 {
                break;
            }

            match &arr[0] {
                Some(item) => {
                    let id = get_display_name(item, SIGDN_DESKTOPABSOLUTEPARSING)?;
                    let name = get_display_name(item, SIGDN_PARENTRELATIVE)?;
                    let item2: IShellItem2 = item.cast()?;
                    let original_location_variant = item2.GetProperty(&SCID_ORIGINAL_LOCATION)?;
                    let original_location_bstr = PropVariantToBSTR(&original_location_variant)?;
                    let original_location = OsString::from_wide(original_location_bstr.as_wide());
                    let date_deleted = get_date_deleted_unix(&item2)?;

                    // NTFS paths are valid Unicode according to this chart:
                    // https://en.wikipedia.org/wiki/Filename#Comparison_of_filename_limitations
                    // Converting a String back to OsString doesn't do extra work
                    item_vec.push(TrashItem {
                        id,
                        name: name.into_string().map_err(|original| Error::ConvertOsString { original })?.into(),
                        original_parent: PathBuf::from(original_location),
                        time_deleted: date_deleted,
                    });
                }
                None => {
                    break;
                }
            }
        }

        Ok(item_vec)
    }
}

pub fn is_empty() -> Result<bool, Error> {
    ensure_com_initialized();
    unsafe {
        let recycle_bin: IShellItem =
            SHGetKnownFolderItem(&FOLDERID_RecycleBinFolder, KF_FLAG_DEFAULT, HANDLE::default())?;
        let pesi: IEnumShellItems = recycle_bin.BindToHandler(None, &BHID_EnumItems)?;

        let mut count = 0u32;
        let mut items = [None];
        pesi.Next(&mut items, Some(&mut count as *mut u32))?;

        Ok(count == 0)
    }
}

pub fn metadata(item: &TrashItem) -> Result<TrashItemMetadata, Error> {
    ensure_com_initialized();
    let id_as_wide = to_wide_path(&item.id);
    let parsing_name = PCWSTR(id_as_wide.as_ptr());
    let item: IShellItem = unsafe { SHCreateItemFromParsingName(parsing_name, None)? };
    let is_dir = unsafe { item.GetAttributes(SFGAO_FOLDER)? } == SFGAO_FOLDER;
    let size = if is_dir {
        let pesi: IEnumShellItems = unsafe { item.BindToHandler(None, &BHID_EnumItems)? };
        let mut size = 0;
        loop {
            let mut fetched_count: u32 = 0;
            let mut arr = [None];
            unsafe { pesi.Next(&mut arr, Some(&mut fetched_count as *mut u32))? };

            if fetched_count == 0 {
                break;
            }

            match &arr[0] {
                Some(_item) => {
                    size += 1;
                }
                None => {
                    break;
                }
            }
        }
        TrashItemSize::Entries(size)
    } else {
        let item2: IShellItem2 = item.cast()?;
        TrashItemSize::Bytes(unsafe { item2.GetUInt64(&PKEY_Size)? })
    };
    Ok(TrashItemMetadata { size })
}

pub fn purge_all<I>(items: I) -> Result<(), Error>
where
    I: IntoIterator,
    <I as IntoIterator>::Item: Borrow<TrashItem>,
{
    ensure_com_initialized();
    unsafe {
        let pfo: IFileOperation = CoCreateInstance(&FileOperation as *const _, None, CLSCTX_ALL)?;
        pfo.SetOperationFlags(FOF_NO_UI)?;
        let mut at_least_one = false;
        for item in items {
            at_least_one = true;
            let id_as_wide = to_wide_path(&item.borrow().id);
            let parsing_name = PCWSTR(id_as_wide.as_ptr());
            let trash_item: IShellItem = SHCreateItemFromParsingName(parsing_name, None)?;
            pfo.DeleteItem(&trash_item, None)?;
        }
        if at_least_one {
            pfo.PerformOperations()?;
        }
        Ok(())
    }
}

pub fn restore_all<I>(items: I) -> Result<(), Error>
where
    I: IntoIterator<Item = TrashItem>,
{
    let items: Vec<_> = items.into_iter().collect();

    // Do a quick and dirty check if the target items already exist at the location
    // and if they do, return all of them, if they don't just go ahead with the processing
    // without giving a damn.
    // Note that this is not 'thread safe' meaning that if a paralell thread (or process)
    // does this operation the exact same time or creates files or folders right after this check,
    // then the files that would collide will not be detected and returned as part of an error.
    // Instead Windows will display a prompt to the user whether they want to replace or skip.
    for item in items.iter() {
        let path = item.original_path();
        if path.exists() {
            return Err(Error::RestoreCollision { path, remaining_items: items });
        }
    }
    ensure_com_initialized();
    unsafe {
        let pfo: IFileOperation = CoCreateInstance(&FileOperation as *const _, None, CLSCTX_ALL)?;
        pfo.SetOperationFlags(FOF_NO_UI | FOFX_EARLYFAILURE)?;
        for item in items.iter() {
            let id_as_wide = to_wide_path(&item.id);
            let parsing_name = PCWSTR(id_as_wide.as_ptr());
            let trash_item: IShellItem = SHCreateItemFromParsingName(parsing_name, None)?;
            let parent_path_wide = to_wide_path(&item.original_parent);
            let orig_folder_shi: IShellItem = SHCreateItemFromParsingName(PCWSTR(parent_path_wide.as_ptr()), None)?;
            let name_wstr = to_wide_path(&item.name);

            pfo.MoveItem(&trash_item, &orig_folder_shi, PCWSTR(name_wstr.as_ptr()), None)?;
        }
        if !items.is_empty() {
            pfo.PerformOperations()?;
        }
        Ok(())
    }
}

unsafe fn get_display_name(psi: &IShellItem, sigdnname: SIGDN) -> Result<OsString, Error> {
    let name = psi.GetDisplayName(sigdnname)?;
    let result = wstr_to_os_string(name);
    CoTaskMemFree(Some(name.0 as *const c_void));
    Ok(result)
}

unsafe fn wstr_to_os_string(wstr: PWSTR) -> OsString {
    let mut len = 0;
    while *(wstr.0.offset(len)) != 0 {
        len += 1;
    }
    let wstr_slice = std::slice::from_raw_parts(wstr.0, len as usize);
    OsString::from_wide(wstr_slice)
}

unsafe fn get_date_deleted_unix(item: &IShellItem2) -> Result<i64, Error> {
    /// January 1, 1970 as Windows file time
    const EPOCH_AS_FILETIME: u64 = 116444736000000000;
    const HUNDREDS_OF_NANOSECONDS: u64 = 10000000;

    let time = item.GetFileTime(&SCID_DATE_DELETED)?;
    let time_u64 = ((time.dwHighDateTime as u64) << 32) | (time.dwLowDateTime as u64);
    let rel_to_linux_epoch = time_u64 - EPOCH_AS_FILETIME;
    let seconds_since_unix_epoch = rel_to_linux_epoch / HUNDREDS_OF_NANOSECONDS;

    Ok(seconds_since_unix_epoch as i64)
}

struct CoInitializer {}
impl CoInitializer {
    fn new() -> CoInitializer {
        //let first = INITIALIZER_THREAD_COUNT.fetch_add(1, Ordering::SeqCst) == 0;
        #[cfg(all(not(feature = "coinit_multithreaded"), not(feature = "coinit_apartmentthreaded")))]
        {
            0 = "THIS IS AN ERROR ON PURPOSE. Either the `coinit_multithreaded` or the `coinit_apartmentthreaded` feature must be specified";
        }
        let mut init_mode;
        #[cfg(feature = "coinit_multithreaded")]
        {
            init_mode = COINIT_MULTITHREADED;
        }
        #[cfg(feature = "coinit_apartmentthreaded")]
        {
            init_mode = COINIT_APARTMENTTHREADED;
        }

        // These flags can be combined with either of coinit_multithreaded or coinit_apartmentthreaded.
        if cfg!(feature = "coinit_disable_ole1dde") {
            init_mode |= COINIT_DISABLE_OLE1DDE;
        }
        if cfg!(feature = "coinit_speed_over_memory") {
            init_mode |= COINIT_SPEED_OVER_MEMORY;
        }
        let hr = unsafe { CoInitializeEx(None, init_mode) };
        if hr.is_err() {
            panic!("Call to CoInitializeEx failed. HRESULT: {:?}. Consider using `trash` with the feature `coinit_multithreaded`", hr);
        }
        CoInitializer {}
    }
}
impl Drop for CoInitializer {
    fn drop(&mut self) {
        // TODO: This does not get called because it's a global static.
        // Is there an atexit in Win32?
        unsafe {
            CoUninitialize();
        }
    }
}
thread_local! {
    static CO_INITIALIZER: CoInitializer = CoInitializer::new();
}
fn ensure_com_initialized() {
    CO_INITIALIZER.with(|_| {});
}
