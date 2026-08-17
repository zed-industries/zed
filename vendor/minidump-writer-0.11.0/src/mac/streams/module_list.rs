use super::*;

struct ImageLoadInfo {
    /// The preferred load address of the TEXT segment
    vm_addr: u64,
    /// The size of the TEXT segment
    vm_size: u64,
    /// The difference between the images preferred and actual load address
    slide: isize,
}

struct ImageDetails {
    /// Unique identifier for the module
    uuid: [u8; 16],
    /// The load info for the image indicating the range of addresses it covers
    load_info: ImageLoadInfo,
    /// Path to the module on the local filesystem. Note that as of MacOS 11.0.1
    /// for system libraries, this path won't actually exist on the filesystem.
    /// This data is more useful as human readable information in a minidump,
    /// but is not required, as the real identifier is the UUID
    file_path: Option<String>,
    /// Version information, not present for the main executable
    version: Option<u32>,
}

impl MinidumpWriter {
    /// Writes the [`MDStreamType::ModuleListStream`] to the minidump, which is
    /// the last of all loaded modules (images) in the process.
    ///
    /// Notably, this includes the UUID of the image which is needed to look up
    /// debug symbols for the module, as well as the address range covered by
    /// the module to know which debug symbols are used to resolve which instruction
    /// addresses
    pub(crate) fn write_module_list(
        &mut self,
        buffer: &mut DumpBuf,
        dumper: &TaskDumper,
    ) -> Result<MDRawDirectory, WriterError> {
        // The list of modules is pretty critical information, but there could
        // still be useful information in the minidump without them if we can't
        // retrieve them for some reason
        let modules = self
            .write_loaded_modules(buffer, dumper)
            .unwrap_or_default();

        let list_header = MemoryWriter::<u32>::alloc_with_val(buffer, modules.len() as u32)?;

        let mut dirent = MDRawDirectory {
            stream_type: MDStreamType::ModuleListStream as u32,
            location: list_header.location(),
        };

        if !modules.is_empty() {
            let mapping_list = MemoryArrayWriter::<MDRawModule>::alloc_from_iter(buffer, modules)?;
            dirent.location.data_size += mapping_list.location().data_size;
        }

        Ok(dirent)
    }

    fn write_loaded_modules(
        &self,
        buf: &mut DumpBuf,
        dumper: &TaskDumper,
    ) -> Result<Vec<MDRawModule>, WriterError> {
        let (all_images_info, mut images) = dumper.read_images()?;

        // Apparently MacOS will happily list the same image multiple times
        // for some reason, so sort the images by load address and remove all
        // of the duplicates
        images.sort();
        images.dedup();

        let mut modules = Vec::with_capacity(images.len());

        for image in images {
            if let Ok(image_details) = self.read_image(image, dumper) {
                let is_main_executable = image_details.version.is_none();

                if let Ok(module) = self.write_module(image_details, buf) {
                    // We want to keep the modules sorted by their load address except
                    // in the case of the main executable image which we want to put
                    // first, as it is most likely the culprit, or at least generally
                    // the most interesting module for human and machine inspectors
                    if is_main_executable {
                        modules.insert(0, module);
                    } else {
                        modules.push(module)
                    };
                }
            }
        }

        if !modules
            .get(0)
            .map(|rm| rm.version_info.signature != format::VS_FFI_SIGNATURE)
            .unwrap_or_default()
        {
            Err(TaskDumpError::NoExecutableImage.into())
        } else {
            // Crashpad also has code for loading the dyld info from the all images
            // array above, but AFAICT (and from crashpad's own comments) this will
            // never actually happen. It's more robust in the face of changes from
            // Apple, which considering their penchant for changings things often
            // and not actually documenting anything, is fair, but if that ever
            // happens we can just...change the code.
            if let Ok(dyld_image) = self.read_dyld(&all_images_info, dumper) {
                if let Ok(module) = self.write_module(dyld_image, buf) {
                    modules.push(module);
                }
            }

            Ok(modules)
        }
    }

    /// Obtains important image metadata by traversing the image's load commands
    ///
    /// # Errors
    ///
    /// The image's load commands cannot be traversed, or a required load command
    /// is missing
    fn read_image(
        &self,
        image: ImageInfo,
        dumper: &TaskDumper,
    ) -> Result<ImageDetails, TaskDumpError> {
        let mut load_info = None;
        let mut version = None;
        let mut uuid = None;

        {
            let load_commands = dumper.read_load_commands(&image)?;

            for lc in load_commands.iter() {
                match lc {
                    mach::LoadCommand::Segment(seg) if load_info.is_none() => {
                        if &seg.segment_name[..7] == b"__TEXT\0" {
                            let slide = image.load_address as isize - seg.vm_addr as isize;

                            load_info = Some(ImageLoadInfo {
                                vm_addr: seg.vm_addr,
                                vm_size: seg.vm_size,
                                slide,
                            });
                        }
                    }
                    mach::LoadCommand::Dylib(dylib) if version.is_none() => {
                        version = Some(dylib.dylib.current_version);
                    }
                    mach::LoadCommand::Uuid(img_id) if uuid.is_none() => {
                        uuid = Some(img_id.uuid);
                    }
                    _ => {}
                }

                if load_info.is_some() && version.is_some() && uuid.is_some() {
                    break;
                }
            }
        }

        let load_info = load_info.ok_or(TaskDumpError::MissingLoadCommand {
            name: "LC_SEGMENT_64",
            id: mach::LoadCommandKind::Segment,
        })?;
        let uuid = uuid.ok_or(TaskDumpError::MissingLoadCommand {
            name: "LC_UUID",
            id: mach::LoadCommandKind::Uuid,
        })?;

        let file_path = if image.file_path != 0 {
            dumper
                .read_string(image.file_path, None)
                .unwrap_or_default()
        } else {
            None
        };

        Ok(ImageDetails {
            uuid,
            load_info,
            file_path,
            version,
        })
    }

    /// Reads the dynamic linker, which is similar but
    fn read_dyld(
        &self,
        all_images: &task_dumper::AllImagesInfo,
        dumper: &TaskDumper,
    ) -> Result<ImageDetails, TaskDumpError> {
        let image = ImageInfo {
            load_address: all_images.dyld_image_load_address,
            file_path: 0,
            file_mod_date: 0,
        };

        let mut load_info = None;
        let mut version = None;
        let mut uuid = None;
        let mut file_path = None;

        {
            let load_commands = dumper.read_load_commands(&image)?;

            for lc in load_commands.iter() {
                match lc {
                    mach::LoadCommand::Segment(seg) if load_info.is_none() => {
                        if &seg.segment_name[..7] == b"__TEXT\0" {
                            let slide = image.load_address as isize - seg.vm_addr as isize;

                            load_info = Some(ImageLoadInfo {
                                vm_addr: seg.vm_addr,
                                vm_size: seg.vm_size,
                                slide,
                            });
                        }
                    }
                    mach::LoadCommand::Dylib(dylib) if version.is_none() => {
                        version = Some(dylib.dylib.current_version);
                    }
                    mach::LoadCommand::Uuid(img_id) if uuid.is_none() => {
                        uuid = Some(img_id.uuid);
                    }
                    mach::LoadCommand::DylinkerCommand(dy_cmd) if file_path.is_none() => {
                        file_path = Some(dy_cmd.name.to_owned());
                    }
                    _ => {}
                }

                if load_info.is_some() && version.is_some() && uuid.is_some() && file_path.is_some()
                {
                    break;
                }
            }
        }

        let load_info = load_info.ok_or(TaskDumpError::MissingLoadCommand {
            name: "LC_SEGMENT_64",
            id: mach::LoadCommandKind::Segment,
        })?;
        let uuid = uuid.ok_or(TaskDumpError::MissingLoadCommand {
            name: "LC_UUID",
            id: mach::LoadCommandKind::Uuid,
        })?;

        Ok(ImageDetails {
            uuid,
            load_info,
            file_path,
            version,
        })
    }

    fn write_module(
        &self,
        image: ImageDetails,
        buf: &mut DumpBuf,
    ) -> Result<MDRawModule, WriterError> {
        let file_path = image.file_path.as_deref().unwrap_or_default();
        let module_name = write_string_to_location(buf, file_path)?;

        let mut raw_module = MDRawModule {
            base_of_image: (image.load_info.vm_addr as isize + image.load_info.slide) as u64,
            size_of_image: image.load_info.vm_size as u32,
            module_name_rva: module_name.rva,
            ..Default::default()
        };

        // Version info is not available for the main executable image since
        // it doesn't issue a LC_ID_DYLIB load command
        if let Some(version) = image.version {
            raw_module.version_info.signature = format::VS_FFI_SIGNATURE;
            raw_module.version_info.struct_version = format::VS_FFI_STRUCVERSION;

            // Convert MAC dylib version format, which is a 32 bit number, to the
            // format used by minidump.
            raw_module.version_info.file_version_hi = version >> 16;
            raw_module.version_info.file_version_lo = ((version & 0xff00) << 8) | (version & 0xff);
        }

        let module_name = if let Some(sep_index) = file_path.rfind('/') {
            &file_path[sep_index + 1..]
        } else if file_path.is_empty() {
            "<Unknown>"
        } else {
            file_path
        };

        #[derive(scroll::Pwrite, scroll::SizeWith)]
        struct CvInfoPdb {
            cv_signature: u32,
            signature: format::GUID,
            age: u32,
        }

        let cv = MemoryWriter::alloc_with_val(
            buf,
            CvInfoPdb {
                cv_signature: format::CvSignature::Pdb70 as u32,
                age: 0,
                signature: image.uuid.into(),
            },
        )?;

        // Note that we don't use write_string_to_location here as the module
        // name is a simple 8-bit string, not 16-bit like most other strings
        // in the minidump, and is directly part of the record itself, not an rva
        buf.write_all(module_name.as_bytes());
        buf.write_all(&[0]); // null terminator

        let mut cv_location = cv.location();
        cv_location.data_size += module_name.len() as u32 + 1;
        raw_module.cv_record = cv_location;

        Ok(raw_module)
    }
}

#[cfg(test)]
// The libc functions used here are all marked as deprecated, saying you
// should use the mach2 crate, however, the mach2 crate does not expose
// any of these functions so...
#[allow(deprecated)]
mod test {
    use super::*;

    // This function isn't declared in libc nor mach2. And is also undocumented
    // by apple, I know, SHOCKING
    extern "C" {
        fn getsegmentdata(
            header: *const libc::mach_header,
            segname: *const u8,
            size: &mut u64,
        ) -> *const u8;
    }

    /// Tests that the images we write as modules to the minidump are consistent
    /// with those reported by the kernel. The kernel function used as the source
    /// of truth can only be used to obtain info for the current process, which
    /// is why they aren't used in the actual implementation as we want to handle
    /// both the local and intra-process scenarios
    #[test]
    fn images_match() {
        if std::env::var_os("CI").is_some() && cfg!(target_arch = "aarch64") {
            // https://github.com/rust-minidump/minidump-writer/issues/101
            println!(
                "this fails on github actions but works on a local aarch64-apple-darwin machine..."
            );
            return;
        }

        let mdw = MinidumpWriter::new(None, None);
        let td = TaskDumper::new(mdw.task);

        let (all_images, images) = td.read_images().unwrap();

        let actual_image_count = unsafe { libc::_dyld_image_count() } as u32;

        assert_eq!(actual_image_count, images.len() as u32);

        for index in 0..actual_image_count {
            let expected_img_hdr = unsafe { libc::_dyld_get_image_header(index) };

            let actual_img = &images[index as usize];

            assert_eq!(actual_img.load_address, expected_img_hdr as u64);

            let mut expect_segment_size = 0;
            let expect_segment_data = unsafe {
                getsegmentdata(
                    expected_img_hdr,
                    b"__TEXT\0".as_ptr(),
                    &mut expect_segment_size,
                )
            };

            let actual_img_details = mdw
                .read_image(*actual_img, &td)
                .expect("failed to get image details");

            let expected_image_name =
                unsafe { std::ffi::CStr::from_ptr(libc::_dyld_get_image_name(index)) };

            let expected_slide = unsafe { libc::_dyld_get_image_vmaddr_slide(index) };
            assert_eq!(
                expected_slide, actual_img_details.load_info.slide,
                "image {index}({expected_image_name:?}) slide is incorrect"
            );

            // The segment pointer has already been adjusted by the slide
            assert_eq!(
                expect_segment_data as u64,
                (actual_img_details.load_info.vm_addr as isize + actual_img_details.load_info.slide)
                    as u64,
                "image {index}({expected_image_name:?}) TEXT address is incorrect"
            );
            assert_eq!(
                expect_segment_size, actual_img_details.load_info.vm_size,
                "image {index}({expected_image_name:?}) TEXT size is incorrect"
            );

            assert_eq!(
                expected_image_name.to_str().unwrap(),
                actual_img_details.file_path.unwrap()
            );
        }

        let dyld = mdw
            .read_dyld(&all_images, &td)
            .expect("failed to read dyld");

        // If the user overrides the dynamic linker and runs this test it will
        // fail, but that's kind of on you, person reading this comment wondering
        // why the test fails. Or Apple changed the path in whatever MacOS version
        // in which case, please file a PR!
        assert_eq!("/usr/lib/dyld", dyld.file_path.as_deref().unwrap());
        assert!(dyld.load_info.vm_size > 0);
    }
}
