use {super::mach, mach2::mach_types as mt, thiserror::Error};

#[derive(Error, Debug)]
pub enum TaskDumpError {
    #[error("kernel error {syscall} {error})")]
    Kernel {
        syscall: &'static str,
        error: mach::KernelError,
    },
    #[error("detected an invalid mach image header")]
    InvalidMachHeader,
    #[error(transparent)]
    NonUtf8String(#[from] std::string::FromUtf8Error),
    #[error("unable to find the main executable image for the process")]
    NoExecutableImage,
    #[error("expected load command {name}({id:?}) was not found for an image")]
    MissingLoadCommand {
        name: &'static str,
        id: mach::LoadCommandKind,
    },
}

/// Wraps a mach call in a Result
macro_rules! mach_call {
    ($call:expr) => {{
        // SAFETY: syscall
        let kr = unsafe { $call };
        if kr == mach::KERN_SUCCESS {
            Ok(())
        } else {
            // This is ugly, improvements to the macro welcome!
            let mut syscall = stringify!($call);
            if let Some(i) = syscall.find('(') {
                syscall = &syscall[..i];
            }
            Err(TaskDumpError::Kernel {
                syscall,
                error: kr.into(),
            })
        }
    }};
}

/// `dyld_all_image_infos` from <usr/include/mach-o/dyld_images.h>
///
/// This struct is truncated as we only need a couple of fields at the beginning
/// of the struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct AllImagesInfo {
    // VERSION 1
    pub version: u32,
    /// The number of [`ImageInfo`] structs at that following address
    info_array_count: u32,
    /// The address in the process where the array of [`ImageInfo`] structs is
    info_array_addr: u64,
    /// A function pointer, unused
    _notification: u64,
    /// Unused
    _process_detached_from_shared_region: bool,
    // VERSION 2
    lib_system_initialized: bool,
    // Note that crashpad adds a 32-bit int here to get proper alignment when
    // building on 32-bit targets...but we explicitly don't care about 32-bit
    // targets since Apple doesn't
    pub dyld_image_load_address: u64,
}

/// `dyld_image_info` from <usr/include/mach-o/dyld_images.h>
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ImageInfo {
    /// The address in the process where the image is loaded
    pub load_address: u64,
    /// The address in the process where the image's file path can be read
    pub file_path: u64,
    /// Timestamp for when the image's file was last modified
    pub file_mod_date: u64,
}

impl PartialEq for ImageInfo {
    fn eq(&self, o: &Self) -> bool {
        self.load_address == o.load_address
    }
}

impl Eq for ImageInfo {}

impl Ord for ImageInfo {
    fn cmp(&self, o: &Self) -> std::cmp::Ordering {
        self.load_address.cmp(&o.load_address)
    }
}

impl PartialOrd for ImageInfo {
    fn partial_cmp(&self, o: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(o))
    }
}

/// Describes a region of virtual memory
pub struct VMRegionInfo {
    pub info: mach::vm_region_submap_info_64,
    pub range: std::ops::Range<u64>,
}

/// Similarly to PtraceDumper for Linux, this provides access to information
/// for a task (MacOS process)
pub struct TaskDumper {
    task: mt::task_t,
    page_size: i64,
}

impl TaskDumper {
    /// Constructs a [`TaskDumper`] for the specified task
    pub fn new(task: mt::task_t) -> Self {
        Self {
            task,
            // SAFETY: syscall
            page_size: unsafe { libc::sysconf(libc::_SC_PAGESIZE) } as i64,
        }
    }

    /// Reads a block of memory from the task
    ///
    /// # Errors
    ///
    /// The syscall to read the task's memory fails for some reason, eg bad address.
    pub fn read_task_memory<T>(&self, address: u64, count: usize) -> Result<Vec<T>, TaskDumpError>
    where
        T: Sized + Clone,
    {
        let length = (count * std::mem::size_of::<T>()) as u64;

        // use the negative of the page size for the mask to find the page address
        let page_address = address & (-self.page_size as u64);
        let last_page_address =
            (address + length + (self.page_size - 1) as u64) & (-self.page_size as u64);

        let page_size = last_page_address - page_address;
        let mut local_start = 0;
        let mut local_length = 0;

        mach_call!(mach::mach_vm_read(
            self.task,
            page_address,
            page_size,
            &mut local_start,
            &mut local_length
        ))?;

        let mut buffer = Vec::with_capacity(count);

        // SAFETY: this is safe as long as the kernel has not lied to us
        let task_buffer = unsafe {
            std::slice::from_raw_parts(
                (local_start as *const u8)
                    .offset((address - page_address) as isize)
                    .cast(),
                count,
            )
        };
        buffer.extend_from_slice(task_buffer);

        // Don't worry about the return here, if something goes wrong there's probably
        // not much we can do about it, and we have what we want anyways
        let _res = mach_call!(mach::mach_vm_deallocate(
            mach::mach_task_self(),
            local_start as u64, // vm_read returns a pointer, but vm_deallocate takes a integer address :-/
            local_length as u64, // vm_read and vm_deallocate use different sizes :-/
        ));

        Ok(buffer)
    }

    /// Reads a null terminated string starting at the specified address. This
    /// is a specialization of [`read_task_memory`] since strings can span VM
    /// regions.
    ///
    /// If not specified, the string is capped at 8k which should never be close
    /// to being hit in normal scenarios, at least for "system" strings, which is
    /// all this interface is used to retrieve
    ///
    /// # Errors
    ///
    /// Fails if the address cannot be read for some reason, or the string is
    /// not utf-8.
    pub fn read_string(
        &self,
        addr: u64,
        expected_size: Option<usize>,
    ) -> Result<Option<String>, TaskDumpError> {
        // The problem is we don't know how much to read until we know how long
        // the string is. And we don't know how long the string is, until we've read
        // the memory!  So, we'll try to read kMaxStringLength bytes
        // (or as many bytes as we can until we reach the end of the vm region).
        let get_region_size = || -> Result<u64, TaskDumpError> {
            let region = self.get_vm_region(addr)?;

            let mut size_to_end = region.range.end - addr;

            // If the remaining is less than 4k, check if the next region is
            // contiguous, and extend the memory that could contain the string
            // to include it
            if size_to_end < 4 * 1024 {
                let maybe_adjacent = self.get_vm_region(region.range.end)?;

                if maybe_adjacent.range.start == region.range.end {
                    size_to_end += maybe_adjacent.range.end - maybe_adjacent.range.start;
                }
            }

            Ok(size_to_end)
        };

        if let Ok(size_to_end) = get_region_size() {
            let mut bytes = self.read_task_memory(
                addr,
                std::cmp::min(size_to_end as usize, expected_size.unwrap_or(8 * 1024)),
            )?;

            // Find the null terminator and truncate our string
            if let Some(null_pos) = bytes.iter().position(|c| *c == 0) {
                bytes.resize(null_pos, 0);
            }

            Ok(String::from_utf8(bytes).map(Some)?)
        } else {
            Ok(None)
        }
    }

    /// Retrives information on the virtual memory region the specified address
    /// is located within.
    ///
    /// # Errors
    ///
    /// The syscall to retrieve the VM region information fails for some reason,
    /// eg. a bad address.
    pub fn get_vm_region(&self, addr: u64) -> Result<VMRegionInfo, TaskDumpError> {
        let mut region_base = addr;
        let mut region_size = 0;
        let mut nesting_level = 0;
        let mut submap_info = std::mem::MaybeUninit::<mach::vm_region_submap_info_64>::uninit();

        // <user/include/mach/vm_region.h>
        const VM_REGION_SUBMAP_INFO_COUNT_64: u32 =
            (std::mem::size_of::<mach::vm_region_submap_info_64>() / std::mem::size_of::<u32>())
                as u32;

        let mut info_count = VM_REGION_SUBMAP_INFO_COUNT_64;

        mach_call!(mach::mach_vm_region_recurse(
            self.task,
            &mut region_base,
            &mut region_size,
            &mut nesting_level,
            submap_info.as_mut_ptr().cast(),
            &mut info_count,
        ))?;

        Ok(VMRegionInfo {
            // SAFETY: this will be valid if the syscall succeeded
            info: unsafe { submap_info.assume_init() },
            range: region_base..region_base + region_size,
        })
    }

    /// Retrieves the state of the specified thread. The state is an architecture
    /// specific block of CPU context ie register state.
    ///
    /// # Errors
    ///
    /// The specified thread id is invalid, or the thread is in a task that is
    /// compiled for a different architecture than this local task.
    pub fn read_thread_state(&self, tid: u32) -> Result<mach::ThreadState, TaskDumpError> {
        let mut thread_state = mach::ThreadState::default();

        mach_call!(mach::thread_get_state(
            tid,
            mach::THREAD_STATE_FLAVOR as i32,
            thread_state.state.as_mut_ptr(),
            &mut thread_state.state_size,
        ))?;

        Ok(thread_state)
    }

    /// Reads the specified task information.
    ///
    /// # Errors
    ///
    /// The syscall to receive the task information failed for some reason, eg.
    /// the specified type and the flavor are mismatched and considered invalid.
    pub fn task_info<T: mach::TaskInfo>(&self) -> Result<T, TaskDumpError> {
        let mut info = std::mem::MaybeUninit::<T>::uninit();
        let mut count = (std::mem::size_of::<T>() / std::mem::size_of::<u32>()) as u32;

        mach_call!(mach::task::task_info(
            self.task,
            T::FLAVOR,
            info.as_mut_ptr().cast(),
            &mut count
        ))?;

        // SAFETY: this will be initialized if the call succeeded
        unsafe { Ok(info.assume_init()) }
    }

    /// Reads the specified task information.
    ///
    /// # Errors
    ///
    /// The syscall to receive the task information failed for some reason, eg.
    /// the specified type and the flavor are mismatched and considered invalid,
    /// or the thread no longer exists
    pub fn thread_info<T: mach::ThreadInfo>(&self, tid: u32) -> Result<T, TaskDumpError> {
        let mut thread_info = std::mem::MaybeUninit::<T>::uninit();
        let mut count = (std::mem::size_of::<T>() / std::mem::size_of::<u32>()) as u32;

        mach_call!(mach::thread_info(
            tid,
            T::FLAVOR,
            thread_info.as_mut_ptr().cast(),
            &mut count,
        ))?;

        // SAFETY: this will be initialized if the call succeeded
        unsafe { Ok(thread_info.assume_init()) }
    }

    /// Retrieves all of the images loaded in the task.
    ///
    /// Note that there may be multiple images with the same load address.
    ///
    /// # Errors
    ///
    /// The syscall to retrieve the location of the loaded images fails, or
    /// the syscall to read the loaded images from the process memory fails
    pub fn read_images(&self) -> Result<(AllImagesInfo, Vec<ImageInfo>), TaskDumpError> {
        // Retrieve the address at which the list of loaded images is located
        // within the task
        let all_images_addr = {
            let dyld_info = self.task_info::<mach::task_info::task_dyld_info>()?;
            dyld_info.all_image_info_addr
        };

        // Here we make the assumption that dyld loaded at the same address in
        // the crashed process vs. this one.  This is an assumption made in
        // "dyld_debug.c" and is said to be nearly always valid.
        let dyld_all_info_buf =
            self.read_task_memory::<u8>(all_images_addr, std::mem::size_of::<AllImagesInfo>())?;
        // SAFETY: this is fine as long as the kernel isn't lying to us
        let all_images_info: &AllImagesInfo = unsafe { &*(dyld_all_info_buf.as_ptr().cast()) };

        let images = self.read_task_memory::<ImageInfo>(
            all_images_info.info_array_addr,
            all_images_info.info_array_count as usize,
        )?;

        Ok((*all_images_info, images))
    }

    /// Retrieves the main executable image for the task.
    ///
    /// Note that this method is currently only used for tests due to deficiencies
    /// in `otool`
    ///
    /// # Errors
    ///
    /// Any of the errors that apply to [`Self::read_images`] apply here, in
    /// addition to not being able to find the main executable image
    pub fn read_executable_image(&self) -> Result<ImageInfo, TaskDumpError> {
        let (_, images) = self.read_images()?;

        for img in images {
            let mach_header = self.read_task_memory::<mach::MachHeader>(img.load_address, 1)?;

            let header = &mach_header[0];

            if header.magic != mach::MH_MAGIC_64 {
                return Err(TaskDumpError::InvalidMachHeader);
            }

            if header.file_type == mach::MH_EXECUTE {
                return Ok(img);
            }
        }

        Err(TaskDumpError::NoExecutableImage)
    }

    /// Retrieves the load commands for the specified image
    ///
    /// # Errors
    ///
    /// We fail to read the image header for the specified image, the header we
    /// read is determined to be invalid, or we fail to read the block of memory
    /// containing the load commands themselves.
    pub fn read_load_commands(&self, img: &ImageInfo) -> Result<mach::LoadCommands, TaskDumpError> {
        let mach_header = self.read_task_memory::<mach::MachHeader>(img.load_address, 1)?;

        let header = &mach_header[0];

        if header.magic != mach::MH_MAGIC_64 {
            return Err(TaskDumpError::InvalidMachHeader);
        }

        // Read the load commands which immediately follow the image header from
        // the task memory. Note that load commands vary in size so we need to
        // retrieve the memory as a raw byte buffer that we can then iterate
        // through and step according to the size of each load command
        let load_commands_buf = self.read_task_memory::<u8>(
            img.load_address + std::mem::size_of::<mach::MachHeader>() as u64,
            header.size_commands as usize,
        )?;

        Ok(mach::LoadCommands {
            buffer: load_commands_buf,
            count: header.num_commands,
        })
    }

    /// Gets a list of all of the thread ids in the task
    ///
    /// # Errors
    ///
    /// The syscall to retrieve the list of threads fails
    pub fn read_threads(&self) -> Result<&'static [u32], TaskDumpError> {
        let mut threads = std::ptr::null_mut();
        let mut thread_count = 0;

        mach_call!(mach::task_threads(
            self.task,
            &mut threads,
            &mut thread_count
        ))?;

        Ok(
            // SAFETY: This should be valid if the call succeeded
            unsafe { std::slice::from_raw_parts(threads, thread_count as usize) },
        )
    }

    /// Retrieves the PID for the task
    ///
    /// # Errors
    ///
    /// Presumably the only way this would fail would be if the task we are
    /// dumping disappears.
    pub fn pid_for_task(&self) -> Result<i32, TaskDumpError> {
        let mut pid = 0;
        mach_call!(mach::pid_for_task(self.task, &mut pid))?;
        Ok(pid)
    }
}

impl mach::TaskInfo for mach::task_info::task_dyld_info {
    const FLAVOR: u32 = mach::task_info::TASK_DYLD_INFO;
}
