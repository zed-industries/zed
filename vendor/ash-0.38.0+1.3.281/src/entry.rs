use crate::instance::Instance;
#[cfg(doc)]
use crate::khr;
use crate::prelude::*;
use crate::vk;
use crate::RawPtr;
use alloc::vec::Vec;
use core::ffi;
use core::fmt;
use core::mem;
use core::ptr;

#[cfg(feature = "loaded")]
use libloading::Library;

/// Holds the Vulkan functions independent of a particular instance
#[derive(Clone)]
pub struct Entry {
    static_fn: crate::StaticFn,
    entry_fn_1_0: crate::EntryFnV1_0,
    entry_fn_1_1: crate::EntryFnV1_1,
    #[cfg(feature = "loaded")]
    _lib_guard: Option<alloc::sync::Arc<Library>>,
}

/// Vulkan core 1.0
impl Entry {
    /// Load default Vulkan library for the current platform
    ///
    /// Prefer this over [`linked()`][Self::linked()] when your application can gracefully handle
    /// environments that lack Vulkan support, and when the build environment might not have Vulkan
    /// development packages installed (e.g. the Vulkan SDK, or Ubuntu's `libvulkan-dev`).
    ///
    /// # Safety
    ///
    /// `dlopen`ing native libraries is inherently unsafe. The safety guidelines
    /// for [`Library::new()`] and [`Library::get()`] apply here.
    ///
    /// No Vulkan functions loaded directly or indirectly from this [`Entry`]
    /// may be called after it is [dropped][drop()].
    ///
    /// # Example
    ///
    /// ```no_run
    /// use ash::{vk, Entry};
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let entry = unsafe { Entry::load()? };
    /// let app_info = vk::ApplicationInfo {
    ///     api_version: vk::make_api_version(0, 1, 0, 0),
    ///     ..Default::default()
    /// };
    /// let create_info = vk::InstanceCreateInfo {
    ///     p_application_info: &app_info,
    ///     ..Default::default()
    /// };
    /// let instance = unsafe { entry.create_instance(&create_info, None)? };
    /// # Ok(()) }
    /// ```
    #[cfg(feature = "loaded")]
    #[cfg_attr(docsrs, doc(cfg(feature = "loaded")))]
    pub unsafe fn load() -> Result<Self, LoadingError> {
        #[cfg(windows)]
        const LIB_PATH: &str = "vulkan-1.dll";

        #[cfg(all(
            unix,
            not(any(
                target_os = "macos",
                target_os = "ios",
                target_os = "android",
                target_os = "fuchsia"
            ))
        ))]
        const LIB_PATH: &str = "libvulkan.so.1";

        #[cfg(any(target_os = "android", target_os = "fuchsia"))]
        const LIB_PATH: &str = "libvulkan.so";

        #[cfg(any(target_os = "macos", target_os = "ios"))]
        const LIB_PATH: &str = "libvulkan.dylib";

        Self::load_from(LIB_PATH)
    }

    /// Load entry points from a Vulkan loader linked at compile time
    ///
    /// Compared to [`load()`][Self::load()], this is infallible, but requires that the build
    /// environment have Vulkan development packages installed (e.g. the Vulkan SDK, or Ubuntu's
    /// `libvulkan-dev`), and prevents the resulting binary from starting in environments that do not
    /// support Vulkan.
    ///
    /// Note that instance/device functions are still fetched via `vkGetInstanceProcAddr` and
    /// `vkGetDeviceProcAddr` for maximum performance.
    ///
    /// Any Vulkan function acquired directly or indirectly from this [`Entry`] may be called after it
    /// is [dropped][drop()].
    ///
    /// # Example
    ///
    /// ```no_run
    /// use ash::{vk, Entry};
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let entry = Entry::linked();
    /// let app_info = vk::ApplicationInfo {
    ///     api_version: vk::make_api_version(0, 1, 0, 0),
    ///     ..Default::default()
    /// };
    /// let create_info = vk::InstanceCreateInfo {
    ///     p_application_info: &app_info,
    ///     ..Default::default()
    /// };
    /// let instance = unsafe { entry.create_instance(&create_info, None)? };
    /// # Ok(()) }
    /// ```
    #[cfg(feature = "linked")]
    #[cfg_attr(docsrs, doc(cfg(feature = "linked")))]
    pub fn linked() -> Self {
        // Sound because we're linking to Vulkan, which provides a vkGetInstanceProcAddr that has
        // defined behavior in this use.
        unsafe {
            Self::from_static_fn(crate::StaticFn {
                get_instance_proc_addr: vkGetInstanceProcAddr,
            })
        }
    }

    /// Load Vulkan library at `path`
    ///
    /// # Safety
    ///
    /// `dlopen`ing native libraries is inherently unsafe. The safety guidelines
    /// for [`Library::new()`] and [`Library::get()`] apply here.
    ///
    /// No Vulkan functions loaded directly or indirectly from this [`Entry`]
    /// may be called after it is [dropped][drop()].
    #[cfg(feature = "loaded")]
    #[cfg_attr(docsrs, doc(cfg(feature = "loaded")))]
    pub unsafe fn load_from(path: impl AsRef<std::ffi::OsStr>) -> Result<Self, LoadingError> {
        let lib = Library::new(path)
            .map_err(LoadingError::LibraryLoadFailure)
            .map(alloc::sync::Arc::new)?;

        let static_fn = crate::StaticFn::load_checked(|name| {
            lib.get(name.to_bytes_with_nul())
                .map(|symbol| *symbol)
                .unwrap_or(ptr::null_mut())
        })?;

        Ok(Self {
            _lib_guard: Some(lib),
            ..Self::from_static_fn(static_fn)
        })
    }

    /// Load entry points based on an already-loaded [`crate::StaticFn`]
    ///
    /// # Safety
    ///
    /// `static_fn` must contain valid function pointers that comply with the semantics specified
    /// by Vulkan 1.0, which must remain valid for at least the lifetime of the returned [`Entry`].
    pub unsafe fn from_static_fn(static_fn: crate::StaticFn) -> Self {
        let load_fn = move |name: &ffi::CStr| {
            mem::transmute((static_fn.get_instance_proc_addr)(
                vk::Instance::null(),
                name.as_ptr(),
            ))
        };

        Self::from_parts_1_1(
            static_fn,
            crate::EntryFnV1_0::load(load_fn),
            crate::EntryFnV1_1::load(load_fn),
        )
    }

    #[inline]
    pub fn from_parts_1_1(
        static_fn: crate::StaticFn,
        entry_fn_1_0: crate::EntryFnV1_0,
        entry_fn_1_1: crate::EntryFnV1_1,
    ) -> Self {
        Self {
            static_fn,
            entry_fn_1_0,
            entry_fn_1_1,
            #[cfg(feature = "loaded")]
            _lib_guard: None,
        }
    }

    #[inline]
    pub fn fp_v1_0(&self) -> &crate::EntryFnV1_0 {
        &self.entry_fn_1_0
    }

    #[inline]
    pub fn static_fn(&self) -> &crate::StaticFn {
        &self.static_fn
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkEnumerateInstanceVersion.html>
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ash::{Entry, vk};
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let entry = Entry::linked();
    /// match unsafe { entry.try_enumerate_instance_version() }? {
    ///     // Vulkan 1.1+
    ///     Some(version) => {
    ///         let major = vk::version_major(version);
    ///         let minor = vk::version_minor(version);
    ///         let patch = vk::version_patch(version);
    ///     },
    ///     // Vulkan 1.0
    ///     None => {},
    /// }
    /// # Ok(()) }
    /// ```
    #[inline]
    pub unsafe fn try_enumerate_instance_version(&self) -> VkResult<Option<u32>> {
        let enumerate_instance_version: Option<vk::PFN_vkEnumerateInstanceVersion> = {
            let name = ffi::CStr::from_bytes_with_nul_unchecked(b"vkEnumerateInstanceVersion\0");
            mem::transmute((self.static_fn.get_instance_proc_addr)(
                vk::Instance::null(),
                name.as_ptr(),
            ))
        };
        if let Some(enumerate_instance_version) = enumerate_instance_version {
            let mut api_version = mem::MaybeUninit::uninit();
            (enumerate_instance_version)(api_version.as_mut_ptr())
                .assume_init_on_success(api_version)
                .map(Some)
        } else {
            Ok(None)
        }
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkCreateInstance.html>
    ///
    /// # Safety
    ///
    /// The resulting [`Instance`] and any function-pointer objects (e.g. [`Device`][crate::Device]
    /// and extensions like [`khr::swapchain::Device`]) loaded from it may not be used after
    /// this [`Entry`] object is dropped, unless it was crated using [`Entry::linked()`] or
    /// [`Entry::from_parts_1_1()`].
    ///
    /// [`Instance`] does _not_ implement [drop][drop()] semantics and can only be destroyed via
    /// [`destroy_instance()`][Instance::destroy_instance()].
    #[inline]
    pub unsafe fn create_instance(
        &self,
        create_info: &vk::InstanceCreateInfo<'_>,
        allocation_callbacks: Option<&vk::AllocationCallbacks<'_>>,
    ) -> VkResult<Instance> {
        let mut instance = mem::MaybeUninit::uninit();
        let instance = (self.entry_fn_1_0.create_instance)(
            create_info,
            allocation_callbacks.as_raw_ptr(),
            instance.as_mut_ptr(),
        )
        .assume_init_on_success(instance)?;
        Ok(Instance::load(&self.static_fn, instance))
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkEnumerateInstanceLayerProperties.html>
    #[inline]
    pub unsafe fn enumerate_instance_layer_properties(&self) -> VkResult<Vec<vk::LayerProperties>> {
        read_into_uninitialized_vector(|count, data| {
            (self.entry_fn_1_0.enumerate_instance_layer_properties)(count, data)
        })
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkEnumerateInstanceExtensionProperties.html>
    #[inline]
    pub unsafe fn enumerate_instance_extension_properties(
        &self,
        layer_name: Option<&ffi::CStr>,
    ) -> VkResult<Vec<vk::ExtensionProperties>> {
        read_into_uninitialized_vector(|count, data| {
            (self.entry_fn_1_0.enumerate_instance_extension_properties)(
                layer_name.map_or(ptr::null(), |str| str.as_ptr()),
                count,
                data,
            )
        })
    }

    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkGetInstanceProcAddr.html>
    #[inline]
    pub unsafe fn get_instance_proc_addr(
        &self,
        instance: vk::Instance,
        p_name: *const ffi::c_char,
    ) -> vk::PFN_vkVoidFunction {
        (self.static_fn.get_instance_proc_addr)(instance, p_name)
    }
}

/// Vulkan core 1.1
impl Entry {
    #[inline]
    pub fn fp_v1_1(&self) -> &crate::EntryFnV1_1 {
        &self.entry_fn_1_1
    }

    #[deprecated = "This function is unavailable and therefore panics on Vulkan 1.0, please use `try_enumerate_instance_version()` instead"]
    /// <https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/vkEnumerateInstanceVersion.html>
    ///
    /// Please use [`try_enumerate_instance_version()`][Self::try_enumerate_instance_version()] instead.
    #[inline]
    pub unsafe fn enumerate_instance_version(&self) -> VkResult<u32> {
        let mut api_version = mem::MaybeUninit::uninit();
        (self.entry_fn_1_1.enumerate_instance_version)(api_version.as_mut_ptr())
            .assume_init_on_success(api_version)
    }
}

#[cfg(feature = "linked")]
#[cfg_attr(docsrs, doc(cfg(feature = "linked")))]
impl Default for Entry {
    #[inline]
    fn default() -> Self {
        Self::linked()
    }
}

impl crate::StaticFn {
    pub fn load_checked<F>(mut _f: F) -> Result<Self, MissingEntryPoint>
    where
        F: FnMut(&ffi::CStr) -> *const ffi::c_void,
    {
        Ok(Self {
            get_instance_proc_addr: unsafe {
                let cname = ffi::CStr::from_bytes_with_nul_unchecked(b"vkGetInstanceProcAddr\0");
                let val = _f(cname);
                if val.is_null() {
                    return Err(MissingEntryPoint);
                } else {
                    mem::transmute(val)
                }
            },
        })
    }
}

#[derive(Clone, Debug)]
pub struct MissingEntryPoint;
impl fmt::Display for MissingEntryPoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Cannot load `vkGetInstanceProcAddr` symbol from library")
    }
}
#[cfg(feature = "std")] // TODO: implement when error_in_core is stabilized
impl std::error::Error for MissingEntryPoint {}

#[cfg(feature = "linked")]
extern "system" {
    fn vkGetInstanceProcAddr(
        instance: vk::Instance,
        name: *const ffi::c_char,
    ) -> vk::PFN_vkVoidFunction;
}

#[cfg(feature = "loaded")]
mod loaded {

    use super::*;

    #[derive(Debug)]
    #[cfg_attr(docsrs, doc(cfg(feature = "loaded")))]
    pub enum LoadingError {
        LibraryLoadFailure(libloading::Error),
        MissingEntryPoint(MissingEntryPoint),
    }

    impl fmt::Display for LoadingError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Self::LibraryLoadFailure(err) => fmt::Display::fmt(err, f),
                Self::MissingEntryPoint(err) => fmt::Display::fmt(err, f),
            }
        }
    }

    #[cfg(feature = "std")]
    impl std::error::Error for LoadingError {
        fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
            Some(match self {
                Self::LibraryLoadFailure(err) => err,
                Self::MissingEntryPoint(err) => err,
            })
        }
    }

    impl From<MissingEntryPoint> for LoadingError {
        fn from(err: MissingEntryPoint) -> Self {
            Self::MissingEntryPoint(err)
        }
    }
}
#[cfg(feature = "loaded")]
pub use self::loaded::*;
