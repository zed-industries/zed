use core::iter::FusedIterator;

use crate::error;
use crate::pe::{data_directories, debug, optional_header, section_table, symbol};
use crate::strtab;
use alloc::vec::Vec;
use scroll::{ctx, IOread, IOwrite, Pread, Pwrite, SizeWith};

/// In `winnt.h` and `pe.h`, it's `IMAGE_DOS_HEADER`. It's a DOS header present in all PE binaries.
///
/// The DOS header is a relic from the MS-DOS era. It used to be useful to display an
/// error message if the binary is run in MS-DOS by utilizing the DOS stub.
///
/// Nowadays, only two fields from
/// the DOS header are used on Windows: [`signature` (aka `e_magic`)](DosHeader::signature)
/// and [`pe_pointer` (aka `e_lfanew`)](DosHeader::pe_pointer).
///
/// ## Position in a modern PE file
///
/// The DOS header is located at the beginning of the PE file and is usually followed by the [DosStub].
///
/// ## Note on the archaic "formatted header"
///
/// The subset of the structure spanning from its start to the [`overlay_number` (aka `e_ovno`)](DosHeader::overlay_number) field
/// included (i.e. till the offset 0x1C) used to be commonly known as "formatted header", since their position and contents were
/// fixed. Optional information used by overlay managers could have followed the formatted header. In the absence of optional
/// information, the formatted header was followed by the ["relocation pointer table"](https://www.tavi.co.uk/phobos/exeformat.html#reloctable).
///
/// Overlays were sections of a program that remained on disk until the program actually required them. Different overlays
/// could thus share the same memory area. The overlays were loaded and unloaded by special code provided by the program
/// or its run-time library.
///
/// [Source](https://www.tavi.co.uk/phobos/exeformat.html#:~:text=Format%20of%20the%20.EXE%20file%20header).
#[repr(C)]
#[derive(Debug, PartialEq, Copy, Clone, Default, Pwrite)]
#[doc(alias("IMAGE_DOS_HEADER"))]
pub struct DosHeader {
    /// Magic number: `[0x5A, 0x4D]`. In [little endian](https://en.wikipedia.org/wiki/Endianness)
    /// [ASCII](https://en.wikipedia.org/wiki/ASCII), it reads "MZ" for [Mark Zbikowski](https://en.wikipedia.org/wiki/Mark_Zbikowski)).
    ///
    /// ## Non-MZ DOS executables
    ///
    /// * For [IBM OS/2](https://www.britannica.com/technology/IBM-OS-2), the value was "NE".
    /// * For IBM OS/2 LE, the value was "LE".
    /// * For [NT](https://en.wikipedia.org/wiki/Windows_NT), the value was "PE00".
    ///
    /// Sources:
    ///
    /// * <https://stixproject.github.io/data-model/1.2/WinExecutableFileObj/DOSHeaderType/>
    /// * <https://learn.microsoft.com/en-us/archive/msdn-magazine/2002/february/inside-windows-win32-portable-executable-file-format-in-detail>
    #[doc(alias("e_magic"))]
    pub signature: u16,
    /// In `winnt.h` and `pe.h`, it's `e_cblp`.
    ///
    /// It used to specify the number of bytes actually used in the last "page".
    /// Page used to refer to a segment of memory, usually of 512 bytes size.
    ///
    /// The case of full page was represented by 0x0000 (since the last page is never empty).
    ///
    /// For example, assuming a page size of 512 bytes, this value would
    /// be 0x0000 for a 1024 byte file, and 0x0001 for a 1025 byte file
    /// (since it only contains one valid byte).
    /// [Source](https://stixproject.github.io/data-model/1.2/WinExecutableFileObj/DOSHeaderType/).
    ///
    /// Typically, this field is set to 0. [Source](https://offwhitesecurity.dev/malware-development/portable-executable-pe/dos-header/).
    #[doc(alias("e_cblp"))]
    pub bytes_on_last_page: u16,
    /// In `winnt.h` and `pe.h`, it's `e_cp`.
    ///
    /// It used to specify the number of pages required to hold a file. For example,
    /// if the file contained 1024 bytes, and the file had pages of a size of 512 bytes,
    /// this [word](https://en.wikipedia.org/wiki/Word_(computer_architecture)) would contain
    /// 0x0002 (2 pages); if the file contained 1025 bytes, this word would contain 0x0003 (3 pages).
    /// [Source](https://stixproject.github.io/data-model/1.2/WinExecutableFileObj/DOSHeaderType/).
    ///
    /// Typically, this field is set to 0. [Source](https://offwhitesecurity.dev/malware-development/portable-executable-pe/dos-header/).
    #[doc(alias("e_cp"))]
    pub pages_in_file: u16,
    /// In `winnt.h` and `pe.h`, it's `e_crlc`.
    ///
    /// It used to specify the number of "relocation items", i.e. the number of entries that
    /// existed in the ["relocation pointer table"](https://www.tavi.co.uk/phobos/exeformat.html#reloctable).
    /// If there were no relocations, this field would contain 0x0000.
    /// [Source](https://stixproject.github.io/data-model/1.2/WinExecutableFileObj/DOSHeaderType/).
    ///
    /// ## On relocation items and relocation pointer table
    ///
    /// When a program is compiled, memory addresses are often hard-coded into the binary code.
    /// These addresses are usually relative to the base address where the program expects to be loaded into memory.
    /// However, when the program is loaded into memory, it might not be loaded at its preferred base address due to
    /// various reasons such as memory fragmentation or other programs already occupying that space.
    ///
    /// Relocation items, also known as fixups or relocations, are pieces of data embedded within the executable file
    /// that indicate which memory addresses need to be adjusted when the program is loaded at a different base address.
    /// These relocations specify the location and type of adjustment needed.
    ///
    /// The relocation pointer table is a data structure that contains pointers to the locations within the executable file
    /// where relocations need to be applied. It allows the operating system's loader to efficiently locate and process the
    /// relocation data during the loading process.
    ///
    /// ---
    ///
    /// Typically, this field is set to 0. [Source](https://offwhitesecurity.dev/malware-development/portable-executable-pe/dos-header/).
    #[doc(alias("e_crlc"))]
    pub relocations: u16,
    /// In `winnt.h` and `pe.h`, it's `e_cparhdr`.
    ///
    /// It used to specify the size of the "executable header" in terms of "paragraphs" (16 byte chunks). It used to indicate
    /// the offset of the program's compiled/assembled and linked image (the [load module](https://www.tavi.co.uk/phobos/exeformat.html#loadmodule)) within the executable file. The size
    /// of the load module could have been deduced by substructing this value (converted to bytes) from the overall size that could
    /// have been derived from combining the value of [`pages_in_file` (aka `e_cp`)](DosHeader::pages_in_file) and the value of
    /// [`bytes_on_last_page` (aka `e_cblp)`](DosHeader::bytes_on_last_page). The header used to always span an even number of
    /// paragraphs.
    /// [Source](https://stixproject.github.io/data-model/1.2/WinExecutableFileObj/DOSHeaderType/).
    ///
    /// The "executable header" in this context refers to the DOS header itself.
    ///
    /// Typically, this field is set to 4. [Source](https://offwhitesecurity.dev/malware-development/portable-executable-pe/dos-header/).
    /// This is because the modern DOS header is 64 bytes long, and 64 / 16 = 4.
    #[doc(alias("e_cparhdr"))]
    pub size_of_header_in_paragraphs: u16,
    /// In `winnt.h` and `pe.h`, it's `e_minalloc`.
    ///
    /// It used to specify the minimum number of extra paragraphs needed to be allocated to begin execution. This is
    /// **in addition** to the memory required to hold the [load module](https://www.tavi.co.uk/phobos/exeformat.html#loadmodule). This value normally represented the total size
    /// of any uninitialized data and/or stack segments that were linked at the end of the program. This space was not
    /// directly included in the load module, since there were no particular initializing values and it would simply waste
    /// disk space.
    /// [Source](https://stixproject.github.io/data-model/1.2/WinExecutableFileObj/DOSHeaderType/).
    ///
    /// If both the [`minimum_extra_paragraphs_needed` (aka `e_minalloc`)](DosHeader::minimum_extra_paragraphs_needed) and
    /// [`maximum_extra_paragraphs_needed` (aka `e_maxalloc`)](DosHeader::maximum_extra_paragraphs_needed) fields were set to 0x0000,
    /// the program would be allocated as much memory as available. [Source](https://www.tavi.co.uk/phobos/exeformat.html)
    ///
    /// Typically, this field is set to 0x10. [Source](https://offwhitesecurity.dev/malware-development/portable-executable-pe/dos-header/).
    #[doc(alias("e_minalloc"))]
    pub minimum_extra_paragraphs_needed: u16,
    /// In `winnt.h` and `pe.h`, it's `e_maxalloc`.
    ///
    /// It used to specify the maximum number of extra paragraphs needed to be allocated by to begin execution. This indicated
    /// **additional** memory over and above that required by the [load module](https://www.tavi.co.uk/phobos/exeformat.html#loadmodule) and the value specified in
    /// [`minimum_extra_paragraphs_needed` (aka `e_minalloc`)](DosHeader::minimum_extra_paragraphs_needed).
    /// If the request could not be satisfied, the program would be allocated as much memory as available.
    /// [Source](https://stixproject.github.io/data-model/1.2/WinExecutableFileObj/DOSHeaderType/).
    ///
    /// If both the [`minimum_extra_paragraphs_needed` (aka `e_minalloc`)](DosHeader::minimum_extra_paragraphs_needed) and
    /// [`maximum_extra_paragraphs_needed` (aka `e_maxalloc`)](DosHeader::maximum_extra_paragraphs_needed) fields were set to 0x0000,
    /// the program would be allocated as much memory as available. [Source](https://www.tavi.co.uk/phobos/exeformat.html)
    ///
    /// Typically, this field is set to 0xFFFF. [Source](https://offwhitesecurity.dev/malware-development/portable-executable-pe/dos-header/).
    #[doc(alias("e_maxalloc"))]
    pub maximum_extra_paragraphs_needed: u16,
    /// In `winnt.h` and `pe.h`, it's `e_ss`.
    ///
    /// It used to specify the initial SS ("stack segment") value. SS value was a paragraph address of the stack segment
    /// relative to the start of the [load module](https://www.tavi.co.uk/phobos/exeformat.html#loadmodule). At load time, the value was relocated by adding the address of the
    /// start segment of the program to it, and the resulting value was placed in the SS register before the program is
    /// started. To read more about x86 memory segmentation and SS register, see the
    /// [wikipedia article](https://en.wikipedia.org/wiki/X86_memory_segmentation) on this topic. In DOS, the start segment
    /// boundary of the program was the first segment boundary in memory after
    /// [Program Segment Prefix (PSP)](https://en.wikipedia.org/wiki/Program_Segment_Prefix).
    /// [Source](https://stixproject.github.io/data-model/1.2/WinExecutableFileObj/DOSHeaderType/).
    ///
    /// The Program Segment Prefix (PSP) was a data structure used in DOS (Disk Operating System) environments.
    /// It was located at the beginning of the memory allocated for a running program and it contained various
    /// pieces of information about the program, including command-line arguments, environment variables,
    /// and pointers to various system resources.
    ///
    /// [According to Wikipedia](https://en.wikipedia.org/wiki/Data_segment#Stack), the stack segment contains the call stack,
    /// a LIFO structure, typically located in the higher parts of memory. A "stack pointer" register tracks the top of the
    /// stack; it is adjusted each time a value is "pushed" onto the stack. The set of values pushed for one function call
    /// is termed a "stack frame".
    ///
    /// Typically, this field is set to 0. [Source](https://offwhitesecurity.dev/malware-development/portable-executable-pe/dos-header/).
    #[doc(alias("e_ss"))]
    pub initial_relative_ss: u16,
    /// In `winnt.h` and `pe.h`, it's `e_sp`.
    ///
    /// It used to specify the initial SP ("stack pointer") value. SP value was the absolute value that must have been loaded
    /// into the SP register before the program is given control. Since the actual stack segment was determined by the loader,
    /// and this was merely a value within that segment, it didn't need to be relocated.
    ///
    /// [According to Wikipedia](https://en.wikipedia.org/wiki/Data_segment#Stack), the stack segment contains the call stack,
    /// a LIFO structure, typically located in the higher parts of memory. A "stack pointer" register tracks the top of the
    /// stack; it is adjusted each time a value is "pushed" onto the stack. The set of values pushed for one function call
    /// is termed a "stack frame".
    /// [Source](https://stixproject.github.io/data-model/1.2/WinExecutableFileObj/DOSHeaderType/).
    ///
    /// Typically, this field is set to 0xB8. [Source](https://offwhitesecurity.dev/malware-development/portable-executable-pe/dos-header/).
    // TODO: Clarify what exactly is meany by "this was merely a value within that segment".
    #[doc(alias("e_sp"))]
    pub initial_sp: u16,
    /// In `winnt.h` and `pe.h`, it's `e_csum`.
    ///
    /// It used to specify the checksum of the contents of the executable file It used to ensure the integrity of the data
    /// within the file. For full details on how this checksum was calculated, see <http://www.tavi.co.uk/phobos/exeformat.html#checksum>.
    /// [Source](https://stixproject.github.io/data-model/1.2/WinExecutableFileObj/DOSHeaderType/).
    ///
    /// Typically, this field is set to 0. [Source](https://offwhitesecurity.dev/malware-development/portable-executable-pe/dos-header/).
    #[doc(alias("e_csum"))]
    pub checksum: u16,
    /// In `winnt.h` and `pe.h`, it's `e_ip`.
    ///
    /// It used to specify the initial IP ("instruction pointer") value. IP value was the absolute value that must have been
    /// loaded into the IP register in order to transfer control to the program. Since the actual code segment was determined
    /// by the loader and, and this was merely a value within that segment, it didn't need to be relocated.
    /// [Source](https://stixproject.github.io/data-model/1.2/WinExecutableFileObj/DOSHeaderType/).
    ///
    /// Typically, this field is set to 0. [Source](https://offwhitesecurity.dev/malware-development/portable-executable-pe/dos-header/).
    // TODO: Clarify what exactly is meany by "this was merely a value within that segment".
    #[doc(alias("e_ip"))]
    pub initial_ip: u16,
    /// In `winnt.h` and `pe.h`, it's `e_cs`.
    ///
    /// It used to specify the pre-relocated initial CS ("code segment") value relative to the start of the [load module](https://www.tavi.co.uk/phobos/exeformat.html#loadmodule),
    /// that should have been placed in the CS register in order to transfer control to the program. At load time, this value
    /// was relocated by adding the address of the start segment of the program to it, and the resulting value was placed in
    /// the CS register when control is transferred.
    /// [Source](https://stixproject.github.io/data-model/1.2/WinExecutableFileObj/DOSHeaderType/).
    ///
    /// Typically, this field is set to 0. [Source](https://offwhitesecurity.dev/malware-development/portable-executable-pe/dos-header/).
    #[doc(alias("e_cs"))]
    pub initial_relative_cs: u16,
    /// In `winnt.h` and `pe.h`, it's `e_lfarlc`.
    ///
    /// It used to specify the logical file address of the relocation table, or more specifically, the offset from the start
    /// of the file to the [relocation pointer table](https://www.tavi.co.uk/phobos/exeformat.html#reloctable). This value
    /// must have been used to locate the relocation table (rather than assuming a fixed location) because variable-length
    /// information pertaining to program overlays could have occurred before this table, causing its position to vary.
    /// A value of 0x40 in this field generally indicated a different kind of executable, not a DOS 'MZ' type.
    /// [Source](https://stixproject.github.io/data-model/1.2/WinExecutableFileObj/DOSHeaderType/).
    ///
    /// Typically, this field is set to 0x40. [Source](https://offwhitesecurity.dev/malware-development/portable-executable-pe/dos-header/).
    #[doc(alias("e_lfarlc"))]
    pub file_address_of_relocation_table: u16,
    /// In `winnt.h` and `pe.h`, it's `e_ovno`.
    ///
    /// It used to specify the overlay number, which was normally set to 0x0000, because few programs actually had overlays.
    /// It changed only in files containing programs that used overlays.
    /// [Source](https://stixproject.github.io/data-model/1.2/WinExecutableFileObj/DOSHeaderType/).
    ///
    /// Overlays were sections of a program that remained on disk until the program actually required them. Different overlays
    /// could thus share the same memory area. The overlays were loaded and unloaded by special code provided by the program
    /// or its run-time library.
    ///
    /// Typically, this field is set to 0. [Source](https://offwhitesecurity.dev/malware-development/portable-executable-pe/dos-header/).
    #[doc(alias("e_ovno"))]
    pub overlay_number: u16,
    /// In `winnt.h` and `pe.h`, it's `e_res[4]`.
    ///
    /// It used to specify the reserved words for the program, i.e. an array reserved for future use.
    /// Usually, the array was zeroed by the linker.
    /// [Source](https://stixproject.github.io/data-model/1.2/WinExecutableFileObj/DOSHeaderType/).
    ///
    /// Typically, this field is set to 0. [Source](https://offwhitesecurity.dev/malware-development/portable-executable-pe/dos-header/).
    #[doc(alias("e_res"))]
    pub reserved: [u16; 4],
    /// In `winnt.h` and `pe.h`, it's `e_oemid`.
    ///
    /// It used to specify the identifier for the OEM ("Original Equipment Manufacturer") for [`oem_info` aka `e_oeminfo`](DosHeader::oem_info).
    /// [Source](https://stixproject.github.io/data-model/1.2/WinExecutableFileObj/DOSHeaderType/).
    ///
    /// More specifically, it used to specify the OEM of the system or hardware platform for which the executable file was created.
    /// This field was used to specify certain characteristics or requirements related to the hardware environment in which the
    /// executable was intended to run.
    ///
    /// Typically, this field is set to 0. [Source](https://offwhitesecurity.dev/malware-development/portable-executable-pe/dos-header/).
    #[doc(alias("e_oemid"))]
    pub oem_id: u16,
    /// In `winnt.h` and `pe.h`, it's `e_oeminfo`.
    ///
    /// It used to specify the extra information, the kind of which was specific to the OEM identified by [`oem_id` aka `e_oemid`](DosHeader::oem_id).
    #[doc(alias("e_oeminfo"))]
    pub oem_info: u16,
    /// In `winnt.h` and `pe.h`, it's `e_res2[10]`.
    ///
    /// It used to specify the reserved words for the program, i.e. an array reserved for future use.
    /// Usually, the array was zeroed by the linker.
    /// [Source](https://stixproject.github.io/data-model/1.2/WinExecutableFileObj/DOSHeaderType/).
    ///
    /// Typically, this field is set to 0. [Source](https://offwhitesecurity.dev/malware-development/portable-executable-pe/dos-header/).
    #[doc(alias("e_res2"))]
    pub reserved2: [u16; 10],
    /// In `winnt.h` and `pe.h`, it's `e_lfanew`.
    ///
    /// Today, it specifies the logcal file address of the of the new exe header. In particular, it is a 4-byte offset into
    /// the file where the PE file header is located. It is necessary to use this offset to locate the PE header in the file.
    ///
    /// Typically, this field is set to 0x3c ([`PE_POINTER_OFFSET`]).
    #[doc(alias("e_lfanew"))]
    pub pe_pointer: u32,
}

#[doc(alias("IMAGE_DOS_SIGNATURE"))]
pub const DOS_MAGIC: u16 = 0x5a4d;
pub const PE_POINTER_OFFSET: u32 = 0x3c;
pub const DOS_STUB_OFFSET: u32 = PE_POINTER_OFFSET + (core::mem::size_of::<u32>() as u32);

impl DosHeader {
    pub fn parse(bytes: &[u8]) -> error::Result<Self> {
        let mut offset = 0;
        let signature = bytes.gread_with(&mut offset, scroll::LE).map_err(|_| {
            error::Error::Malformed(format!("cannot parse DOS signature (offset {:#x})", 0))
        })?;
        if signature != DOS_MAGIC {
            return Err(error::Error::Malformed(format!(
                "DOS header is malformed (signature {:#x})",
                signature
            )));
        }

        let bytes_on_last_page = bytes.gread_with(&mut offset, scroll::LE)?;
        let pages_in_file = bytes.gread_with(&mut offset, scroll::LE)?;
        let relocations = bytes.gread_with(&mut offset, scroll::LE)?;
        let size_of_header_in_paragraphs = bytes.gread_with(&mut offset, scroll::LE)?;
        let minimum_extra_paragraphs_needed = bytes.gread_with(&mut offset, scroll::LE)?;
        let maximum_extra_paragraphs_needed = bytes.gread_with(&mut offset, scroll::LE)?;
        let initial_relative_ss = bytes.gread_with(&mut offset, scroll::LE)?;
        let initial_sp = bytes.gread_with(&mut offset, scroll::LE)?;
        let checksum = bytes.gread_with(&mut offset, scroll::LE)?;
        let initial_ip = bytes.gread_with(&mut offset, scroll::LE)?;
        let initial_relative_cs = bytes.gread_with(&mut offset, scroll::LE)?;
        let file_address_of_relocation_table = bytes.gread_with(&mut offset, scroll::LE)?;
        let overlay_number = bytes.gread_with(&mut offset, scroll::LE)?;
        let reserved = bytes.gread_with(&mut offset, scroll::LE)?; // 4
        let oem_id = bytes.gread_with(&mut offset, scroll::LE)?;
        let oem_info = bytes.gread_with(&mut offset, scroll::LE)?;
        let reserved2 = bytes.gread_with(&mut offset, scroll::LE)?; // 10

        debug_assert!(
            offset == PE_POINTER_OFFSET as usize,
            "expected offset ({:#x}) after reading DOS header to be at 0x3C",
            offset
        );

        let pe_pointer: u32 = bytes
            .pread_with(PE_POINTER_OFFSET as usize, scroll::LE)
            .map_err(|_| {
                error::Error::Malformed(format!(
                    "cannot parse PE header pointer (offset {:#x})",
                    PE_POINTER_OFFSET
                ))
            })?;

        let pe_signature: u32 =
            bytes
                .pread_with(pe_pointer as usize, scroll::LE)
                .map_err(|_| {
                    error::Error::Malformed(format!(
                        "cannot parse PE header signature (offset {:#x})",
                        pe_pointer
                    ))
                })?;
        if pe_signature != PE_MAGIC {
            return Err(error::Error::Malformed(format!(
                "PE header is malformed (signature {:#x})",
                pe_signature
            )));
        }

        Ok(DosHeader {
            signature,
            bytes_on_last_page,
            pages_in_file,
            relocations,
            size_of_header_in_paragraphs,
            minimum_extra_paragraphs_needed,
            maximum_extra_paragraphs_needed,
            initial_relative_ss,
            initial_sp,
            checksum,
            initial_ip,
            initial_relative_cs,
            file_address_of_relocation_table,
            overlay_number,
            reserved,
            oem_id,
            oem_info,
            reserved2,
            pe_pointer,
        })
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
/// The DOS stub program which should be executed in DOS mode. It prints the message "This program cannot be run in DOS mode" and exits.
///
/// ## Position in a modern PE file
///
/// The [DosStub] is usually located immediately after the [DosHeader] and...
///
/// * De facto, can be followed by a non-standard ["Rich header"](https://0xrick.github.io/win-internals/pe3/#rich-header).
/// * According to the standard, is followed by the  [Header::signature] and then the [CoffHeader].
pub struct DosStub<'a> {
    pub data: &'a [u8],
}
impl<'a> Default for DosStub<'a> {
    /// This is the very basic DOS program bytecode representation embedded in MSVC linker.
    ///
    /// An equivalent (Not a equal) DOS program can be follows:
    ///
    /// ```asm
    ///     push cs           ; 0E         Push the code segment onto the stack
    ///     pop ds            ; 1F         Pop the top of the stack into the data segment register
    ///
    /// _start:
    ///     mov dx, aMessage  ; BA 0E 00   Load the address of the message to the DX register
    ///     mov ah, 09h       ; B4 09      DOS function 09h (display string) to print the message at DS:DX
    ///     int 21h           ; CD 21      Call DOS interrupt 21h for displaying the message
    ///
    ///     mov ax, 4C01h     ; B8 01 4C   DOS function 4Ch (terminate program) with return code 1
    ///     int 21h           ; CD 21      Call DOS interrupt 21h for program termination
    ///
    /// aMessage db 'This program cannot be run in DOS mode.'
    /// ```
    #[rustfmt::skip]
    fn default() -> Self {
        Self {
            data: &[
                0x0E,                   // push cs: Setup segment registers
                0x1F,                   // pop ds: Setup segment registers
                0xBA, 0x0E, 0x00,       // mov dx, 0x000E: Load the message address into the DX register
                0xB4, 0x09,             // mov ah, 0x09: DOS function to print a string
                0xCD, 0x21,             // int 0x21: Trigger DOS interrupt 21h to print the message
                0xB8, 0x01, 0x4C,       // mov ax, 0x4C01: Prepare to terminate the program (DOS function 4Ch)
                0xCD, 0x21,             // int 0x21: Trigger DOS interrupt 21h to terminate the program
                0x54, 0x68, 0x69, 0x73, // "This" ASCII string "This program cannot be run in DOS mode."
                0x20, 0x70, 0x72, 0x6F, // " pro" Continuation of the ASCII string,
                0x67, 0x72, 0x61, 0x6D, // "gram" Continuation of the ASCII string,
                0x20, 0x63, 0x61, 0x6E, // " can" Continuation of the ASCII string,
                0x6E, 0x6F, 0x74, 0x20, // "not " Continuation of the ASCII string,
                0x62, 0x65, 0x20, 0x72, // "be r" Continuation of the ASCII string,
                0x75, 0x6E, 0x20, 0x69, // "un i" Continuation of the ASCII string,
                0x6E, 0x20, 0x44, 0x4F, // "n DO" Continuation of the ASCII string,
                0x53, 0x20, 0x6D, 0x6F, // "S mo" Continuation of the ASCII string,
                0x64, 0x65, 0x2E,       // "DE." Continuation of the ASCII string, ending with a period.
                0x0D, 0x0D, 0x0A,       // Carriage return (CR `0x0D, 0x0D`) and line feed (LF `0x0A`)
                0x24,                   // '$' (End of string marker for DOS function 09h)
                0x00, 0x00, 0x00, 0x00, // Padding bytes (8-byte alignment)
                0x00, 0x00, 0x00,       // Padding bytes (8-byte alignment)
            ],
        }
    }
}
impl<'a> ctx::TryIntoCtx<scroll::Endian> for DosStub<'a> {
    type Error = error::Error;

    fn try_into_ctx(self, bytes: &mut [u8], _: scroll::Endian) -> Result<usize, Self::Error> {
        let offset = &mut 0;
        bytes.gwrite_with(&*self.data, offset, ())?;
        Ok(*offset)
    }
}

impl<'a> DosStub<'a> {
    /// Parse the DOS stub.
    ///
    /// The DOS stub is a small program that prints the message "This program cannot be run in DOS mode" and exits; and
    /// is not really read for the PECOFF file format. It's a relic from the MS-DOS era.
    pub fn parse(bytes: &'a [u8], pe_pointer: u32) -> error::Result<Self> {
        let start_offset = DOS_STUB_OFFSET as usize;
        let end_offset = pe_pointer as usize;

        // Check if the end_offset is less than or equal to start_offset
        if end_offset <= start_offset {
            return Err(error::Error::Malformed(format!(
                "PE pointer ({:#x}) must be greater than the start offset ({:#x})",
                pe_pointer, start_offset
            )));
        }

        if bytes.len() < end_offset as usize {
            return Err(error::Error::Malformed(format!(
                "DOS stub is too short ({} bytes) to contain the PE header pointer ({:#x})",
                bytes.len(),
                end_offset
            )));
        }

        let dos_stub_area = &bytes[start_offset..end_offset];
        Ok(Self {
            data: dos_stub_area,
        })
    }
}

/// In `winnt.h`, it's `IMAGE_FILE_HEADER`. COFF Header.
///
/// Together with the [Header::signature] and the [Header::optional_header], it forms the
/// [`IMAGE_NT_HEADERS`](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-image_nt_headers32).
///
/// ## Position in a modern PE file
///
/// The COFF header is located after the [Header::signature], which in turn is located after the
/// non-standard ["Rich header"](https://0xrick.github.io/win-internals/pe3/#rich-header), if present,
/// and after the [DosStub], according to the standard.
///
/// COFF header is followed by the [Header::optional_header].
#[repr(C)]
#[derive(Debug, PartialEq, Copy, Clone, Default, Pread, Pwrite, IOread, IOwrite, SizeWith)]
#[doc(alias("IMAGE_FILE_HEADER"))]
pub struct CoffHeader {
    /// The architecture type of the computer. An image file can only be run
    /// on the specified computer or a system that emulates the specified computer.
    ///
    /// Can be one of the following values:
    ///
    /// * [`COFF_MACHINE_UNKNOWN`],
    /// * [`COFF_MACHINE_ALPHA`],
    /// * [`COFF_MACHINE_ALPHA64`],
    /// * [`COFF_MACHINE_AM33`],
    /// * [`COFF_MACHINE_X86_64`],
    /// * [`COFF_MACHINE_ARM`],
    /// * [`COFF_MACHINE_ARM64`],
    /// * [`COFF_MACHINE_ARMNT`],
    /// * [`COFF_MACHINE_EBC`],
    /// * [`COFF_MACHINE_X86`],
    /// * [`COFF_MACHINE_IA64`],
    /// * [`COFF_MACHINE_LOONGARCH32`],
    /// * [`COFF_MACHINE_LOONGARCH64`],
    /// * [`COFF_MACHINE_M32R`],
    /// * [`COFF_MACHINE_MIPS16`],
    /// * [`COFF_MACHINE_MIPSFPU`],
    /// * [`COFF_MACHINE_MIPSFPU16`],
    /// * [`COFF_MACHINE_POWERPC`],
    /// * [`COFF_MACHINE_POWERPCFP`],
    /// * [`COFF_MACHINE_R4000`],
    /// * [`COFF_MACHINE_RISCV32`],
    /// * [`COFF_MACHINE_RISCV64`],
    /// * [`COFF_MACHINE_RISCV128`],
    /// * [`COFF_MACHINE_SH3`],
    /// * [`COFF_MACHINE_SH3DSP`],
    /// * [`COFF_MACHINE_SH4`],
    /// * [`COFF_MACHINE_SH5`],
    /// * [`COFF_MACHINE_THUMB`],
    /// * [`COFF_MACHINE_WCEMIPSV2`],
    ///
    /// or any other value that is not listed here.
    ///
    /// The constants above are sourced from <https://learn.microsoft.com/en-us/windows/win32/debug/pe-format#machine-types>.
    /// If there's a missing constant, please open an issue or a pull request.
    // TODO: insert the values names with a macro
    #[doc(alias("Machine"))]
    pub machine: u16,
    /// The number of sections. This indicates the size of the section table, which immediately follows the headers.
    /// Note that the Windows loader limits the number of sections to 96.
    /// [Source](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-image_file_header).
    #[doc(alias("NumberOfSections"))]
    pub number_of_sections: u16,
    /// The low 32 bits of the time stamp of the image. This represents the date and time the image was created by the linker.
    /// The value is represented in the number of seconds elapsed since midnight (00:00:00), January 1, 1970, Universal
    /// Coordinated Time, according to the system clock.
    #[doc(alias("TimeDateStamp"))]
    pub time_date_stamp: u32,
    /// The offset of the symbol table, in bytes, or zero if no COFF symbol table exists.
    ///
    /// Typically, this field is set to 0 because COFF debugging information is deprecated.
    /// [Source](https://0xrick.github.io/win-internals/pe4/#file-header-image_file_header).
    // TODO: further explain the COFF symbol table. This seems to be a nuanced topic.
    #[doc(alias("PointerToSymbolTable"))]
    pub pointer_to_symbol_table: u32,
    /// The number of symbols in the symbol table.
    ///
    /// Typically, this field is set to 0 because COFF debugging information is deprecated.
    /// [Source](https://0xrick.github.io/win-internals/pe4/#file-header-image_file_header).
    // Q (JohnScience): Why is the name `number_of_symbol_table` and not `number_of_symbols`?
    #[doc(alias("NumberOfSymbols"))]
    pub number_of_symbol_table: u32,
    /// The size of the optional header, in bytes. This value should be zero for object files.
    ///
    /// The [`goblin::pe::optional_header::OptionalHeader`](crate::pe::optional_header::OptionalHeader) is meant to
    /// represent either the 32-bit or the 64-bit optional header. The size of the optional header is used to determine
    /// which one it is.
    #[doc(alias("SizeOfOptionalHeader"))]
    pub size_of_optional_header: u16,
    /// The [characteristics](https://learn.microsoft.com/en-us/windows/win32/debug/pe-format#characteristics) of the image.
    ///
    /// The constants for the characteristics are available in the [`goblin::pe::characteristic`](crate::pe::characteristic) module.
    #[doc(alias("Characteristics"))]
    pub characteristics: u16,
}

pub const SIZEOF_COFF_HEADER: usize = 20;
/// PE\0\0, little endian
pub const PE_MAGIC: u32 = 0x0000_4550;
pub const SIZEOF_PE_MAGIC: usize = 4;

// Q (JohnScience): doesn't it make sense to move all these constants to a dedicated module
// and then re-export them from here? This way, the module will be more organized.
//
// Also, don't we want to declare them in a macro to remove the boilerplate and make the implementation
// of `machine_to_str` more future-proof and concise? For example, addition of...
//
// * `IMAGE_FILE_MACHINE_LOONGARCH32`,
// * `IMAGE_FILE_MACHINE_LOONGARCH64`,
// * `IMAGE_FILE_MACHINE_ALPHA`,
// * `IMAGE_FILE_MACHINE_ALPHA64`
//
// didn't trigger the exhaustiveness check because there was a necessary default case.
//
// This way, we can also generate a test that would parse <https://learn.microsoft.com/en-us/windows/win32/debug/pe-format#machine-types>
// and check that there are no missing constants.

/// The contents of this field are assumed to be applicable to any machine type.
///
/// One of the possible values for [`CoffHeader::machine`].
#[doc(alias("IMAGE_FILE_MACHINE_UNKNOWN"))]
pub const COFF_MACHINE_UNKNOWN: u16 = 0x0;

/// Alpha AXP, 32-bit address space.
///
/// One of the possible values for [`CoffHeader::machine`].
#[doc(alias("IMAGE_FILE_MACHINE_ALPHA"))]
pub const COFF_MACHINE_ALPHA: u16 = 0x184;

/// Alpha AXP, 64-bit address space.
///
/// One of the possible values for [`CoffHeader::machine`].
#[doc(alias("IMAGE_FILE_MACHINE_ALPHA64"))]
#[doc(alias("IMAGE_FILE_MACHINE_AXP64"))]
pub const COFF_MACHINE_ALPHA64: u16 = 0x284;

/// Matsushita AM33.
///
/// One of the possible values for [`CoffHeader::machine`].
#[doc(alias("IMAGE_FILE_MACHINE_AM33"))]
pub const COFF_MACHINE_AM33: u16 = 0x1d3;

/// x64 aka amd64.
///
/// One of the possible values for [`CoffHeader::machine`].
#[doc(alias("IMAGE_FILE_MACHINE_AMD64"))]
// Q (JohnScience): why is this `COFF_MACHINE_X86_64` and not `COFF_MACHINE_AMD64`?
// Should we deprecate the former and use the latter instead?
pub const COFF_MACHINE_X86_64: u16 = 0x8664;

/// ARM little endian.
///
/// One of the possible values for [`CoffHeader::machine`].
#[doc(alias("IMAGE_FILE_MACHINE_ARM"))]
pub const COFF_MACHINE_ARM: u16 = 0x1c0;

/// ARM64 little endian.
///
/// One of the possible values for [`CoffHeader::machine`].
#[doc(alias("IMAGE_FILE_MACHINE_ARM64"))]
pub const COFF_MACHINE_ARM64: u16 = 0xaa64;

/// ARM Thumb-2 little endian.
///
/// One of the possible values for [`CoffHeader::machine`].
#[doc(alias("IMAGE_FILE_MACHINE_ARMNT"))]
pub const COFF_MACHINE_ARMNT: u16 = 0x1c4;

/// EFI byte code.
///
/// One of the possible values for [`CoffHeader::machine`].
#[doc(alias("IMAGE_FILE_MACHINE_EBC"))]
pub const COFF_MACHINE_EBC: u16 = 0xebc;

/// Intel 386 or later processors and compatible processors.
///
/// One of the possible values for [`CoffHeader::machine`].
// Q (JohnScience): why is this `COFF_MACHINE_X86` and not `COFF_MACHINE_I386`?
// Should we deprecate the former and use the latter instead?
#[doc(alias("IMAGE_FILE_MACHINE_I386"))]
pub const COFF_MACHINE_X86: u16 = 0x14c;

/// Intel Itanium processor family.
///
/// One of the possible values for [`CoffHeader::machine`].
#[doc(alias("IMAGE_FILE_MACHINE_IA64"))]
pub const COFF_MACHINE_IA64: u16 = 0x200;

/// LoongArch 32-bit processor family.
///
/// One of the possible values for [`CoffHeader::machine`].
#[doc(alias("IMAGE_FILE_MACHINE_LOONGARCH32"))]
pub const COFF_MACHINE_LOONGARCH32: u16 = 0x6232;

/// LoongArch 64-bit processor family.
///
/// One of the possible values for [`CoffHeader::machine`].
#[doc(alias("IMAGE_FILE_MACHINE_LOONGARCH64"))]
pub const COFF_MACHINE_LOONGARCH64: u16 = 0x6264;

/// Mitsubishi M32R little endian.
///
/// One of the possible values for [`CoffHeader::machine`].
#[doc(alias("IMAGE_FILE_MACHINE_M32R"))]
pub const COFF_MACHINE_M32R: u16 = 0x9041;

/// MIPS16.
///
/// One of the possible values for [`CoffHeader::machine`].
#[doc(alias("IMAGE_FILE_MACHINE_MIPS16"))]
pub const COFF_MACHINE_MIPS16: u16 = 0x266;

/// MIPS with FPU.
///
/// One of the possible values for [`CoffHeader::machine`].
#[doc(alias("IMAGE_FILE_MACHINE_MIPSFPU"))]
pub const COFF_MACHINE_MIPSFPU: u16 = 0x366;

/// MIPS16 with FPU.
///
/// One of the possible values for [`CoffHeader::machine`].
#[doc(alias("IMAGE_FILE_MACHINE_MIPSFPU16"))]
pub const COFF_MACHINE_MIPSFPU16: u16 = 0x466;

/// Power PC little endian.
///
/// One of the possible values for [`CoffHeader::machine`].
#[doc(alias("IMAGE_FILE_MACHINE_POWERPC"))]
pub const COFF_MACHINE_POWERPC: u16 = 0x1f0;

/// Power PC with floating point support.
///
/// One of the possible values for [`CoffHeader::machine`].
#[doc(alias("IMAGE_FILE_MACHINE_POWERPCFP"))]
pub const COFF_MACHINE_POWERPCFP: u16 = 0x1f1;

/// MIPS little endian.
///
/// One of the possible values for [`CoffHeader::machine`].
#[doc(alias("IMAGE_FILE_MACHINE_R4000"))]
pub const COFF_MACHINE_R4000: u16 = 0x166;

/// RISC-V 32-bit address space.
///
/// One of the possible values for [`CoffHeader::machine`].
#[doc(alias("IMAGE_FILE_MACHINE_RISCV32"))]
pub const COFF_MACHINE_RISCV32: u16 = 0x5032;

/// RISC-V 64-bit address space.
///
/// One of the possible values for [`CoffHeader::machine`].
#[doc(alias("IMAGE_FILE_MACHINE_RISCV64"))]
pub const COFF_MACHINE_RISCV64: u16 = 0x5064;

/// RISC-V 128-bit address space
///
/// One of the possible values for [`CoffHeader::machine`].
#[doc(alias("IMAGE_FILE_MACHINE_RISCV128"))]
pub const COFF_MACHINE_RISCV128: u16 = 0x5128;

/// Hitachi SH3.
///
/// One of the possible values for [`CoffHeader::machine`].
#[doc(alias("IMAGE_FILE_MACHINE_SH3"))]
pub const COFF_MACHINE_SH3: u16 = 0x1a2;

/// Hitachi SH3 DSP.
///
/// One of the possible values for [`CoffHeader::machine`].
#[doc(alias("IMAGE_FILE_MACHINE_SH3DSP"))]
pub const COFF_MACHINE_SH3DSP: u16 = 0x1a3;

/// Hitachi SH4.
///
/// One of the possible values for [`CoffHeader::machine`].
#[doc(alias("IMAGE_FILE_MACHINE_SH4"))]
pub const COFF_MACHINE_SH4: u16 = 0x1a6;

/// Hitachi SH5.
///
/// One of the possible values for [`CoffHeader::machine`].
#[doc(alias("IMAGE_FILE_MACHINE_SH5"))]
pub const COFF_MACHINE_SH5: u16 = 0x1a8;

/// Thumb.
///
/// One of the possible values for [`CoffHeader::machine`].
#[doc(alias("IMAGE_FILE_MACHINE_THUMB"))]
pub const COFF_MACHINE_THUMB: u16 = 0x1c2;

/// MIPS little-endian WCE v2.
///
/// One of the possible values for [`CoffHeader::machine`].
#[doc(alias("IMAGE_FILE_MACHINE_WCEMIPSV2"))]
pub const COFF_MACHINE_WCEMIPSV2: u16 = 0x169;

impl CoffHeader {
    pub fn parse(bytes: &[u8], offset: &mut usize) -> error::Result<Self> {
        Ok(bytes.gread_with(offset, scroll::LE)?)
    }

    /// Parse the COFF section headers.
    ///
    /// For COFF, these immediately follow the COFF header. For PE, these immediately follow the
    /// optional header.
    pub fn sections(
        &self,
        bytes: &[u8],
        offset: &mut usize,
    ) -> error::Result<Vec<section_table::SectionTable>> {
        let nsections = self.number_of_sections as usize;

        // a section table is at least 40 bytes
        if nsections > bytes.len() / 40 {
            return Err(error::Error::BufferTooShort(nsections, "sections"));
        }

        let mut sections = Vec::with_capacity(nsections);
        // Note that if we are handling a BigCoff, the size of the symbol will be different!
        let string_table_offset = self.pointer_to_symbol_table as usize
            + symbol::SymbolTable::size(self.number_of_symbol_table as usize);
        for i in 0..nsections {
            let section =
                section_table::SectionTable::parse(bytes, offset, string_table_offset as usize)?;
            debug!("({}) {:#?}", i, section);
            sections.push(section);
        }
        Ok(sections)
    }

    /// Return the COFF symbol table.
    pub fn symbols<'a>(&self, bytes: &'a [u8]) -> error::Result<Option<symbol::SymbolTable<'a>>> {
        let offset = self.pointer_to_symbol_table as usize;
        let number = self.number_of_symbol_table as usize;
        if offset == 0 {
            Ok(None)
        } else {
            symbol::SymbolTable::parse(bytes, offset, number).map(Some)
        }
    }

    /// Return the COFF string table.
    pub fn strings<'a>(&self, bytes: &'a [u8]) -> error::Result<Option<strtab::Strtab<'a>>> {
        // > The file offset of the COFF symbol table, or zero if no COFF symbol table is present.
        // > This value should be zero for an image because COFF debugging information is deprecated.
        if self.pointer_to_symbol_table == 0 {
            return Ok(None);
        }

        let mut offset = self.pointer_to_symbol_table as usize
            + symbol::SymbolTable::size(self.number_of_symbol_table as usize);

        let length_field_size = core::mem::size_of::<u32>();
        let length = bytes.pread_with::<u32>(offset, scroll::LE)? as usize - length_field_size;

        // The offset needs to be advanced in order to read the strings.
        offset += length_field_size;

        Ok(Some(strtab::Strtab::parse(bytes, offset, length, 0)?))
    }
}

/// The PE header.
///
/// ## Position in a modern PE file
///
/// The PE header is located at the very beginning of the file and
/// is followed by the section table and sections.
#[derive(Debug, PartialEq, Copy, Clone, Default)]
pub struct Header<'a> {
    pub dos_header: DosHeader,
    /// DOS program for legacy loaders
    pub dos_stub: DosStub<'a>,
    pub rich_header: Option<RichHeader<'a>>,

    /// PE Magic: PE\0\0, little endian
    pub signature: u32,
    pub coff_header: CoffHeader,
    pub optional_header: Option<optional_header::OptionalHeader>,
}

impl<'a> Header<'a> {
    fn parse_impl(
        bytes: &'a [u8],
        dos_header: DosHeader,
        dos_stub: DosStub<'a>,
    ) -> error::Result<Self> {
        let mut offset = dos_header.pe_pointer as usize;
        let rich_header = RichHeader::parse(&bytes)?;
        let signature = bytes.gread_with(&mut offset, scroll::LE).map_err(|_| {
            error::Error::Malformed(format!("cannot parse PE signature (offset {:#x})", offset))
        })?;
        let coff_header = CoffHeader::parse(&bytes, &mut offset)?;
        let optional_header = if coff_header.size_of_optional_header > 0 {
            Some(bytes.pread::<optional_header::OptionalHeader>(offset)?)
        } else {
            None
        };

        Ok(Header {
            dos_header,
            dos_stub,
            rich_header,
            signature,
            coff_header,
            optional_header,
        })
    }

    /// Parses PE header from the given bytes; this will fail if the DosHeader or DosStub is malformed or missing in some way
    pub fn parse(bytes: &'a [u8]) -> error::Result<Self> {
        let dos_header = DosHeader::parse(&bytes)?;
        let dos_stub = DosStub::parse(bytes, dos_header.pe_pointer)?;

        Header::parse_impl(bytes, dos_header, dos_stub)
    }

    /// Parses PE header from the given bytes, a default DosHeader and DosStub are generated, and any malformed header or stub is ignored
    pub fn parse_without_dos(bytes: &'a [u8]) -> error::Result<Self> {
        let dos_header = DosHeader::default();
        Header::parse_impl(bytes, dos_header, DosStub::default())
    }
}

impl<'a> ctx::TryIntoCtx<scroll::Endian> for Header<'a> {
    type Error = error::Error;

    fn try_into_ctx(self, bytes: &mut [u8], ctx: scroll::Endian) -> Result<usize, Self::Error> {
        let offset = &mut 0;
        bytes.gwrite_with(self.dos_header, offset, ctx)?;
        bytes.gwrite_with(self.dos_stub, offset, ctx)?;
        bytes.gwrite_with(self.signature, offset, scroll::LE)?;
        bytes.gwrite_with(self.coff_header, offset, ctx)?;
        if let Some(opt_header) = self.optional_header {
            bytes.gwrite_with(opt_header, offset, ctx)?;
        }
        Ok(*offset)
    }
}

/// The DANS marker is a XOR-decoded version of the string "DanS" and is used to identify the Rich header.
pub const DANS_MARKER: u32 = 0x536E6144;
/// Size of [DANS_MARKER] in bytes
pub const DANS_MARKER_SIZE: usize = core::mem::size_of::<u32>();
/// The Rich marker is a XOR-decoded version of the string "Rich" and is used to identify the Rich header.
pub const RICH_MARKER: u32 = 0x68636952;
/// Size of [RICH_MARKER] in bytes
pub const RICH_MARKER_SIZE: usize = core::mem::size_of::<u32>();

/// The Rich header is a undocumented header that is used to store information about the build environment.
///
/// The Rich Header first appeared in Visual Studio 6.0 and contains: a product identifier, build number, and the number of times it was used during the build process.
#[derive(Debug, PartialEq, Copy, Clone, Default)]
pub struct RichHeader<'a> {
    /// Key is 32-bit value used for XOR encrypt/decrypt fields
    pub key: u32,
    /// The Rich header data with the padding.
    pub data: &'a [u8],
    /// Padding bytes at the prologue of [Self::data]
    pub padding_size: usize,
    /// Start offset of the Rich header.
    pub start_offset: u32,
    /// End offset of the Rich header.
    pub end_offset: u32,
}

/// The Rich metadata is a pair of 32-bit values that store the tool version and the use count.
#[repr(C)]
#[derive(Debug, PartialEq, Copy, Clone, Default, Pread, Pwrite)]
pub struct RichMetadata {
    /// Build version is a 16-bit value that stores the version of the tool used to build the PE file.
    pub build: u16,
    /// Product identifier is a 16-bit value that stores the type of tool used to build the PE file.
    pub product: u16,
    /// The use count is a 32-bit value that stores the number of times the tool was used during the build process.
    pub use_count: u32,
}

impl RichMetadata {
    /// Parse [`RichMetadata`] from given bytes
    fn parse(bytes: &[u8], key: u32) -> error::Result<Self> {
        let mut offset = 0;
        let build_and_product = bytes.gread_with::<u32>(&mut offset, scroll::LE)? ^ key;
        let build = (build_and_product & 0xFFFF) as u16;
        let product = (build_and_product >> 16) as u16;
        let use_count = bytes.gread_with::<u32>(&mut offset, scroll::LE)? ^ key;
        Ok(Self {
            build,
            product,
            use_count,
        })
    }
}

/// Size of [`RichMetadata`] entries.
const RICH_METADATA_SIZE: usize = 8;

/// Iterator over [`RichMetadata`] in [`RichHeader`].
#[derive(Debug)]
pub struct RichMetadataIterator<'a> {
    /// The key of [RichHeader::key]
    key: u32,
    /// The raw data [RichHeader::data] without padding
    data: &'a [u8],
}

impl Iterator for RichMetadataIterator<'_> {
    type Item = error::Result<RichMetadata>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.data.is_empty() {
            return None;
        }

        // Data within this iterator should not have padding
        Some(match RichMetadata::parse(&self.data, self.key) {
            Ok(metadata) => {
                self.data = &self.data[RICH_METADATA_SIZE..];
                Ok(metadata)
            }
            Err(error) => {
                self.data = &[];
                Err(error.into())
            }
        })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.data.len() / RICH_METADATA_SIZE;
        (len, Some(len))
    }
}

impl FusedIterator for RichMetadataIterator<'_> {}
impl ExactSizeIterator for RichMetadataIterator<'_> {}

impl<'a> RichHeader<'a> {
    /// Parse the rich header from the given bytes.
    ///
    /// To decode the Rich header,
    /// - First locate the Rich marker and the subsequent 32-bit encryption key.
    /// - Then, work backwards from the Rich marker, XORing the key with the stored 32-bit values until you decode the DanS marker.
    ///
    /// Between these markers, you'll find pairs of 32-bit values:
    ///
    /// - the first indicates the Microsoft tool used, and
    /// - the second shows the count of linked object files made with that tool.
    /// - The upper 16 bits of the tool ID describe the tool type,
    /// - while the lower 16 bits specify the tool’s build version.
    pub fn parse(bytes: &'a [u8]) -> error::Result<Option<Self>> {
        // Parse the DOS header; some fields are required to locate the Rich header.
        let dos_header = DosHeader::parse(bytes)?;
        let dos_header_end_offset = PE_POINTER_OFFSET as usize;
        let pe_header_start_offset = dos_header.pe_pointer as usize;

        // The Rich header is not present in all PE files.
        if (pe_header_start_offset - dos_header_end_offset) < 8 {
            return Ok(None);
        }

        // The Rich header is located between the DOS header and the PE header.
        let scan_start = dos_header_end_offset + 4;
        let scan_end = pe_header_start_offset;
        if scan_start > scan_end {
            return Err(error::Error::Malformed(format!(
                "Rich header scan start ({:#X}) is greater than scan end ({:#X})",
                scan_start, scan_end
            )));
        }
        let scan_stub = &bytes[scan_start..scan_end];

        // First locate the Rich marker and the subsequent 32-bit encryption key.
        let (rich_end_offset, key) = match scan_stub
            .windows(8)
            .enumerate()
            .filter_map(
                |(index, window)| match window.pread_with::<u32>(0, scroll::LE) {
                    // Marker matches, then return its index
                    Ok(marker) if marker == RICH_MARKER => Some(Ok(index)),
                    // Error reading with scroll
                    Err(e) => Some(Err(error::Error::from(e))),
                    // Marker did not match
                    _ => None,
                },
            )
            // Next is the very first element succeeded
            .next()
        {
            Some(Ok(rich_end_offset)) => {
                let rich_key =
                    scan_stub.pread_with::<u32>(rich_end_offset + RICH_MARKER_SIZE, scroll::LE)?;
                (rich_end_offset, rich_key)
            }
            // Something went wrong, e.g., reading with scroll
            Some(Err(e)) => return Err(e),
            // Marker did not found, rich header is assumed it does not exist
            None => return Ok(None),
        };

        // Ensure rich_end_offset is within bounds
        if rich_end_offset >= scan_stub.len() {
            return Err(error::Error::Malformed(format!(
                "Rich end offset ({:#X}) exceeds scan stub length ({:#X})",
                rich_end_offset,
                scan_stub.len()
            )));
        }
        // Scope the buffer
        let rich_header = &scan_stub[..rich_end_offset];

        // Look for DanS marker
        let rich_start_offset = match scan_stub
            .windows(4)
            .enumerate()
            .filter_map(
                |(index, window)| match window.pread_with::<u32>(0, scroll::LE) {
                    // If we do found the DanS marker, return the offset
                    Ok(value) if (value ^ key) == DANS_MARKER => Some(Ok(index + DANS_MARKER_SIZE)),
                    // This is scroll error, likely malformed rich header
                    Err(e) => Some(Err(error::Error::from(e))),
                    // No matching DanS marker found
                    _ => None,
                },
            )
            // Next is the very first element succeeded
            .next()
        {
            // Suceeded
            Some(Ok(offset)) => offset,
            // Errors such as from scroll reader
            Some(Err(e)) => return Err(e),
            // DanS marker did not found
            None => {
                return Err(error::Error::Malformed(format!(
                    "Rich header does not contain the DanS marker"
                )));
            }
        };

        // Ensure rich_start_offset is within bounds
        if rich_start_offset >= rich_header.len() {
            return Err(error::Error::Malformed(format!(
                "Rich start offset ({:#X}) exceeds rich header length ({:#X})",
                rich_start_offset,
                rich_header.len()
            )));
        }
        // Scope the buffer
        let rich_header = &rich_header[rich_start_offset..];

        // Skip padding bytes
        let padding_size = rich_header
            .chunks(4)
            .map(|chunk| chunk.pread_with::<u32>(0, scroll::LE))
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .take_while(|value| value == &key)
            .count()
            * core::mem::size_of_val(&key);

        // Extract the Rich header data without the padding
        let data = rich_header;

        // Subtract the sizeof DanS marker (u32, 4 bytes)
        let start_offset = scan_start as u32 + rich_start_offset as u32 - DANS_MARKER_SIZE as u32;
        let end_offset = scan_start as u32 + rich_end_offset as u32;

        Ok(Some(RichHeader {
            key,
            data,
            padding_size,
            start_offset,
            end_offset,
        }))
    }

    /// Returns [`RichMetadataIterator`] iterator for [`RichMetadata`]
    pub fn metadatas(&self) -> RichMetadataIterator<'a> {
        RichMetadataIterator {
            key: self.key,
            data: &self.data[self.padding_size..],
        }
    }
}

/// The TE header is a reduced PE32/PE32+ header containing only fields
/// required for execution in the Platform Initialization
/// ([PI](https://uefi.org/specs/PI/1.8/V1_Introduction.html)) architecture.
/// The TE header is described in this specification:
/// <https://uefi.org/specs/PI/1.8/V1_TE_Image.html#te-header>
#[cfg(feature = "te")]
#[repr(C)]
#[derive(Debug, Default, PartialEq, Copy, Clone, Pread, Pwrite)]
pub struct TeHeader {
    /// Te signature, always [TE_MAGIC]
    pub signature: u16,
    /// The machine type
    pub machine: u16,
    /// The number of sections
    pub number_of_sections: u8,
    /// The subsystem
    pub subsystem: u8,
    /// the amount of bytes stripped from the header when converting from a
    /// PE32/PE32+ header to a TE header. Used to resolve addresses
    pub stripped_size: u16,
    /// The entry point of the binary
    pub entry_point: u32,
    /// The base of the code section
    pub base_of_code: u32,
    /// The image base
    pub image_base: u64,
    /// The size and address of the relocation directory
    pub reloc_dir: data_directories::DataDirectory,
    /// The size and address of the debug directory
    pub debug_dir: data_directories::DataDirectory,
}

#[cfg(feature = "te")]
#[doc(alias("IMAGE_TE_SIGNATURE"))]
pub const TE_MAGIC: u16 = 0x5a56;

#[cfg(feature = "te")]
impl TeHeader {
    /// Parse the TE header from the given bytes.
    pub fn parse(bytes: &[u8], offset: &mut usize) -> error::Result<Self> {
        let mut header: TeHeader = bytes.gread_with(offset, scroll::LE)?;
        let adj_offset = header.stripped_size as u32 - core::mem::size_of::<TeHeader>() as u32;
        header.fixup_header(adj_offset);
        Ok(header)
    }

    /// Parse the sections from the TE header.
    pub fn sections(
        &self,
        bytes: &[u8],
        offset: &mut usize,
    ) -> error::Result<Vec<section_table::SectionTable>> {
        let adj_offset = self.stripped_size as u32 - core::mem::size_of::<TeHeader>() as u32;
        let nsections = self.number_of_sections as usize;

        // a section table is at least 40 bytes
        if nsections > bytes.len() / 40 {
            return Err(error::Error::BufferTooShort(nsections, "sections"));
        }

        let mut sections = Vec::with_capacity(nsections);
        for i in 0..nsections {
            let mut section = section_table::SectionTable::parse(bytes, offset, 0)?;
            TeHeader::fixup_section(&mut section, adj_offset);
            debug!("({}) {:#?}", i, section);
            sections.push(section);
        }
        Ok(sections)
    }

    // Adjust addresses in the header to account for the stripped size
    fn fixup_header(&mut self, adj_offset: u32) {
        debug!(
            "Entry point fixed up from: 0x{:x} to 0x{:X}",
            self.entry_point,
            self.entry_point.wrapping_sub(adj_offset)
        );
        self.entry_point = self.entry_point.wrapping_sub(adj_offset);

        debug!(
            "Base of code fixed up from: 0x{:x} to 0x{:X}",
            self.base_of_code,
            self.base_of_code.wrapping_sub(adj_offset)
        );
        self.base_of_code = self.base_of_code.wrapping_sub(adj_offset);

        debug!(
            "Relocation Directory fixed up from: 0x{:x} to 0x{:X}",
            self.reloc_dir.virtual_address,
            self.reloc_dir.virtual_address.wrapping_sub(adj_offset)
        );
        self.reloc_dir.virtual_address = self.reloc_dir.virtual_address.wrapping_sub(adj_offset);

        debug!(
            "Debug Directory fixed up from: 0x{:x} to 0x{:X}",
            self.debug_dir.virtual_address,
            self.debug_dir.virtual_address.wrapping_sub(adj_offset)
        );
        self.debug_dir.virtual_address = self.debug_dir.virtual_address.wrapping_sub(adj_offset);
    }

    // Adjust addresses in the section to account for the stripped size
    fn fixup_section(section: &mut section_table::SectionTable, adj_offset: u32) {
        debug!(
            "Section virtual address fixed up from: 0x{:X} to 0x{:X}",
            section.virtual_address,
            section.virtual_address.wrapping_sub(adj_offset)
        );
        section.virtual_address = section.virtual_address.wrapping_sub(adj_offset);

        if section.pointer_to_linenumbers > 0 {
            debug!(
                "Section pointer to line numbers fixed up from: 0x{:X} to 0x{:X}",
                section.pointer_to_linenumbers,
                section.pointer_to_linenumbers.wrapping_sub(adj_offset)
            );
            section.pointer_to_linenumbers =
                section.pointer_to_linenumbers.wrapping_sub(adj_offset);
        }

        if section.pointer_to_raw_data > 0 {
            debug!(
                "Section pointer to raw data fixed up from: 0x{:X} to 0x{:X}",
                section.pointer_to_raw_data,
                section.pointer_to_raw_data.wrapping_sub(adj_offset)
            );
            section.pointer_to_raw_data = section.pointer_to_raw_data.wrapping_sub(adj_offset);
        }

        if section.pointer_to_relocations > 0 {
            debug!(
                "Section pointer to relocations fixed up from: 0x{:X} to 0x{:X}",
                section.pointer_to_relocations,
                section.pointer_to_relocations.wrapping_sub(adj_offset)
            );
            section.pointer_to_relocations =
                section.pointer_to_relocations.wrapping_sub(adj_offset);
        }
    }
}

/// Convert machine to str representation. Any case of "COFF_UNKNOWN"
/// should be expected to change to a more specific value.
pub fn machine_to_str(machine: u16) -> &'static str {
    // TODO: generate the branches with a macro
    match machine {
        COFF_MACHINE_UNKNOWN => "UNKNOWN",
        COFF_MACHINE_ALPHA => "ALPHA",
        COFF_MACHINE_ALPHA64 => "ALPHA64",
        COFF_MACHINE_AM33 => "AM33",
        // This is an outlier. In the C header, it's IMAGE_FILE_MACHINE_AMD64
        COFF_MACHINE_X86_64 => "X86_64",
        COFF_MACHINE_ARM => "ARM",
        COFF_MACHINE_ARM64 => "ARM64",
        COFF_MACHINE_ARMNT => "ARM_NT",
        COFF_MACHINE_EBC => "EBC",
        // This is an outlier. In the C header, it's IMAGE_FILE_MACHINE_I386
        COFF_MACHINE_X86 => "X86",
        COFF_MACHINE_IA64 => "IA64",
        COFF_MACHINE_LOONGARCH32 => "LOONGARCH32",
        COFF_MACHINE_LOONGARCH64 => "LOONGARCH64",
        COFF_MACHINE_M32R => "M32R",
        COFF_MACHINE_MIPS16 => "MIPS_16",
        COFF_MACHINE_MIPSFPU => "MIPS_FPU",
        COFF_MACHINE_MIPSFPU16 => "MIPS_FPU_16",
        COFF_MACHINE_POWERPC => "POWERPC",
        COFF_MACHINE_POWERPCFP => "POWERCFP",
        COFF_MACHINE_R4000 => "R4000",
        COFF_MACHINE_RISCV32 => "RISC-V_32",
        COFF_MACHINE_RISCV64 => "RISC-V_64",
        COFF_MACHINE_RISCV128 => "RISC-V_128",
        COFF_MACHINE_SH3 => "SH3",
        COFF_MACHINE_SH3DSP => "SH3DSP",
        COFF_MACHINE_SH4 => "SH4",
        COFF_MACHINE_SH5 => "SH5",
        COFF_MACHINE_THUMB => "THUMB",
        COFF_MACHINE_WCEMIPSV2 => "WCE_MIPS_V2",
        _ => "COFF_UNKNOWN",
    }
}

#[cfg(test)]
mod tests {
    use crate::{error, pe::header::DosStub};

    use super::{
        machine_to_str, Header, RichHeader, RichMetadata, COFF_MACHINE_X86, DOS_MAGIC, PE_MAGIC,
    };

    const CRSS_HEADER: [u8; 688] = [
        0x4d, 0x5a, 0x90, 0x00, 0x03, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0xff, 0xff, 0x00,
        0x00, 0xb8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x40, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0xd0, 0x00, 0x00, 0x00, 0x0e, 0x1f, 0xba, 0x0e, 0x00, 0xb4, 0x09, 0xcd, 0x21, 0xb8, 0x01,
        0x4c, 0xcd, 0x21, 0x54, 0x68, 0x69, 0x73, 0x20, 0x70, 0x72, 0x6f, 0x67, 0x72, 0x61, 0x6d,
        0x20, 0x63, 0x61, 0x6e, 0x6e, 0x6f, 0x74, 0x20, 0x62, 0x65, 0x20, 0x72, 0x75, 0x6e, 0x20,
        0x69, 0x6e, 0x20, 0x44, 0x4f, 0x53, 0x20, 0x6d, 0x6f, 0x64, 0x65, 0x2e, 0x0d, 0x0d, 0x0a,
        0x24, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xaa, 0x4a, 0xc3, 0xeb, 0xee, 0x2b, 0xad,
        0xb8, 0xee, 0x2b, 0xad, 0xb8, 0xee, 0x2b, 0xad, 0xb8, 0xee, 0x2b, 0xac, 0xb8, 0xfe, 0x2b,
        0xad, 0xb8, 0x33, 0xd4, 0x66, 0xb8, 0xeb, 0x2b, 0xad, 0xb8, 0x33, 0xd4, 0x63, 0xb8, 0xea,
        0x2b, 0xad, 0xb8, 0x33, 0xd4, 0x7a, 0xb8, 0xed, 0x2b, 0xad, 0xb8, 0x33, 0xd4, 0x64, 0xb8,
        0xef, 0x2b, 0xad, 0xb8, 0x33, 0xd4, 0x61, 0xb8, 0xef, 0x2b, 0xad, 0xb8, 0x52, 0x69, 0x63,
        0x68, 0xee, 0x2b, 0xad, 0xb8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x50, 0x45,
        0x00, 0x00, 0x4c, 0x01, 0x05, 0x00, 0xd9, 0x8f, 0x15, 0x52, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0xe0, 0x00, 0x02, 0x01, 0x0b, 0x01, 0x0b, 0x00, 0x00, 0x08, 0x00, 0x00,
        0x00, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x10, 0x11, 0x00, 0x00, 0x00, 0x10, 0x00,
        0x00, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x40, 0x00, 0x00, 0x10, 0x00, 0x00, 0x00, 0x02,
        0x00, 0x00, 0x06, 0x00, 0x03, 0x00, 0x06, 0x00, 0x03, 0x00, 0x06, 0x00, 0x03, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x60, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0xe4, 0xab, 0x00, 0x00,
        0x01, 0x00, 0x40, 0x05, 0x00, 0x00, 0x04, 0x00, 0x00, 0x30, 0x00, 0x00, 0x00, 0x00, 0x10,
        0x00, 0x00, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x3c, 0x30, 0x00, 0x00, 0x3c, 0x00, 0x00, 0x00, 0x00,
        0x40, 0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x1a, 0x00, 0x00, 0xb8, 0x22, 0x00, 0x00, 0x00, 0x50, 0x00, 0x00, 0x38, 0x00, 0x00,
        0x00, 0x10, 0x10, 0x00, 0x00, 0x38, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x68, 0x10, 0x00, 0x00, 0x5c, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x30, 0x00, 0x00, 0x3c, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x2e, 0x74, 0x65, 0x78, 0x74, 0x00, 0x00, 0x00, 0x24,
        0x06, 0x00, 0x00, 0x00, 0x10, 0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x20, 0x00, 0x00,
        0x60, 0x2e, 0x64, 0x61, 0x74, 0x61, 0x00, 0x00, 0x00, 0x3c, 0x03, 0x00, 0x00, 0x00, 0x20,
        0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x0c, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x40, 0x00, 0x00, 0xc0, 0x2e, 0x69, 0x64, 0x61,
        0x74, 0x61, 0x00, 0x00, 0xf8, 0x01, 0x00, 0x00, 0x00, 0x30, 0x00, 0x00, 0x00, 0x02, 0x00,
        0x00, 0x00, 0x0e, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x40, 0x00, 0x00, 0x40, 0x2e, 0x72, 0x73, 0x72, 0x63, 0x00, 0x00, 0x00, 0x00,
        0x08, 0x00, 0x00, 0x00, 0x40, 0x00, 0x00, 0x00, 0x08, 0x00, 0x00, 0x00, 0x10, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x40, 0x00, 0x00,
        0x42, 0x2e, 0x72, 0x65, 0x6c, 0x6f, 0x63, 0x00, 0x00, 0x86, 0x01, 0x00, 0x00, 0x00, 0x50,
        0x00, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00, 0x18, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x40, 0x00, 0x00, 0x42, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ];

    const NO_RICH_HEADER: [u8; 262] = [
        0x4D, 0x5A, 0x50, 0x00, 0x02, 0x00, 0x00, 0x00, 0x04, 0x00, 0x0F, 0x00, 0xFF, 0xFF, 0x00,
        0x00, 0xB8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x40, 0x00, 0x1A, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x01, 0x00, 0x00, 0xBA, 0x10, 0x00, 0x0E, 0x1F, 0xB4, 0x09, 0xCD, 0x21, 0xB8, 0x01,
        0x4C, 0xCD, 0x21, 0x90, 0x90, 0x54, 0x68, 0x69, 0x73, 0x20, 0x70, 0x72, 0x6F, 0x67, 0x72,
        0x61, 0x6D, 0x20, 0x6D, 0x75, 0x73, 0x74, 0x20, 0x62, 0x65, 0x20, 0x72, 0x75, 0x6E, 0x20,
        0x75, 0x6E, 0x64, 0x65, 0x72, 0x20, 0x57, 0x69, 0x6E, 0x33, 0x32, 0x0D, 0x0A, 0x24, 0x37,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x50, 0x45, 0x00, 0x00, 0x64, 0x86,
    ];

    const NO_RICH_HEADER_INVALID_PE_POINTER: [u8; 304] = [
        0x4D, 0x5A, 0x90, 0x00, 0x03, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0x00,
        0x00, 0xB8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x40, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x3C, 0xFF, 0x00, 0x00, 0x0E, 0x1F, 0xBA, 0x0E, 0x00, 0xB4, 0x09, 0xCD, 0x21, 0xB8, 0x01,
        0x4C, 0xCD, 0x21, 0x54, 0x68, 0x69, 0x73, 0x20, 0x70, 0x72, 0x6F, 0x67, 0x72, 0x61, 0x6D,
        0x20, 0x63, 0x61, 0x6E, 0x6E, 0x6F, 0x74, 0x20, 0x62, 0x65, 0x20, 0x72, 0x75, 0x6E, 0x20,
        0x69, 0x6E, 0x20, 0x44, 0x4F, 0x53, 0x20, 0x6D, 0x6F, 0x64, 0x65, 0x2E, 0x0D, 0x0D, 0x0A,
        0x24, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x8D, 0xC7, 0xEA, 0x07, 0xC9, 0xA6, 0x84,
        0x54, 0xC9, 0xA6, 0x84, 0x54, 0xC9, 0xA6, 0x84, 0x54, 0x10, 0xD2, 0x81, 0x55, 0xCB, 0xA6,
        0x84, 0x54, 0xC0, 0xDE, 0x17, 0x54, 0xC1, 0xA6, 0x84, 0x54, 0xDD, 0xCD, 0x80, 0x55, 0xC7,
        0xA6, 0x84, 0x54, 0xDD, 0xCD, 0x87, 0x55, 0xC1, 0xA6, 0x84, 0x54, 0xDD, 0xCD, 0x81, 0x55,
        0x7E, 0xA6, 0x84, 0x54, 0xB9, 0x27, 0x85, 0x55, 0xCA, 0xA6, 0x84, 0x54, 0xC9, 0xA6, 0x85,
        0x54, 0x08, 0xA6, 0x84, 0x54, 0xA5, 0xD2, 0x81, 0x55, 0xE8, 0xA6, 0x84, 0x54, 0xA5, 0xD2,
        0x80, 0x55, 0xD9, 0xA6, 0x84, 0x54, 0xA5, 0xD2, 0x87, 0x55, 0xC0, 0xA6, 0x84, 0x54, 0x10,
        0xD2, 0x80, 0x55, 0x49, 0xA6, 0x84, 0x54, 0x10, 0xD2, 0x84, 0x55, 0xC8, 0xA6, 0x84, 0x54,
        0x10, 0xD2, 0x7B, 0x54, 0xC8, 0xA6, 0x84, 0x54, 0x10, 0xD2, 0x86, 0x55, 0xC8, 0xA6, 0x84,
        0x54, 0x52, 0x69, 0x63, 0x68, 0xC9, 0xA6, 0x84, 0x54, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x50, 0x45, 0x00, 0x00, 0x64, 0x86, 0x07, 0x00, 0xEC, 0xA5, 0x5B, 0x66,
        0x00, 0x00, 0x00, 0x00,
    ];

    const CORRECT_RICH_HEADER: [u8; 256] = [
        0x4D, 0x5A, 0x90, 0x00, 0x03, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0x00,
        0x00, 0xB8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x40, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0xF8, 0x00, 0x00, 0x00, 0x0E, 0x1F, 0xBA, 0x0E, 0x00, 0xB4, 0x09, 0xCD, 0x21, 0xB8, 0x01,
        0x4C, 0xCD, 0x21, 0x54, 0x68, 0x69, 0x73, 0x20, 0x70, 0x72, 0x6F, 0x67, 0x72, 0x61, 0x6D,
        0x20, 0x63, 0x61, 0x6E, 0x6E, 0x6F, 0x74, 0x20, 0x62, 0x65, 0x20, 0x72, 0x75, 0x6E, 0x20,
        0x69, 0x6E, 0x20, 0x44, 0x4F, 0x53, 0x20, 0x6D, 0x6F, 0x64, 0x65, 0x2E, 0x0D, 0x0D, 0x0A,
        0x24, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x73, 0x4C, 0x5B, 0xB1, 0x37, 0x2D, 0x35,
        0xE2, 0x37, 0x2D, 0x35, 0xE2, 0x37, 0x2D, 0x35, 0xE2, 0x44, 0x4F, 0x31, 0xE3, 0x3D, 0x2D,
        0x35, 0xE2, 0x44, 0x4F, 0x36, 0xE3, 0x32, 0x2D, 0x35, 0xE2, 0x44, 0x4F, 0x30, 0xE3, 0x48,
        0x2D, 0x35, 0xE2, 0xEE, 0x4F, 0x36, 0xE3, 0x3E, 0x2D, 0x35, 0xE2, 0xEE, 0x4F, 0x30, 0xE3,
        0x14, 0x2D, 0x35, 0xE2, 0xEE, 0x4F, 0x31, 0xE3, 0x25, 0x2D, 0x35, 0xE2, 0x44, 0x4F, 0x34,
        0xE3, 0x3C, 0x2D, 0x35, 0xE2, 0x37, 0x2D, 0x34, 0xE2, 0xAF, 0x2D, 0x35, 0xE2, 0x37, 0x2D,
        0x35, 0xE2, 0x23, 0x2D, 0x35, 0xE2, 0xFC, 0x4E, 0x37, 0xE3, 0x36, 0x2D, 0x35, 0xE2, 0x52,
        0x69, 0x63, 0x68, 0x37, 0x2D, 0x35, 0xE2, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x50, 0x45, 0x00, 0x00, 0x64, 0x86, 0x05,
        0x00,
    ];

    const CORRUPTED_RICH_HEADER: [u8; 256] = [
        0x4D, 0x5A, 0x90, 0x00, 0x03, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0x00,
        0x00, 0xB8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x40, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0xF8, 0x00, 0x00, 0x00, 0x0E, 0x1F, 0xBA, 0x0E, 0x00, 0xB4, 0x09, 0xCD, 0x21, 0xB8, 0x01,
        0x4C, 0xCD, 0x21, 0x54, 0x68, 0x69, 0x73, 0x20, 0x70, 0x72, 0x6F, 0x67, 0x72, 0x61, 0x6D,
        0x20, 0x63, 0x61, 0x6E, 0x6E, 0x6F, 0x74, 0x20, 0x62, 0x65, 0x20, 0x72, 0x75, 0x6E, 0x20,
        0x69, 0x6E, 0x20, 0x44, 0x4F, 0x53, 0x20, 0x6D, 0x6F, 0x64, 0x65, 0x2E, 0x0D, 0x0D, 0x0A,
        0x24, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x12, 0x4C, 0x5B, 0xB1, 0x37, 0x2D, 0x35,
        0xE2, 0x37, 0x2D, 0x35, 0xE2, 0x37, 0x2D, 0x35, 0xE2, 0x44, 0x4F, 0x31, 0xE3, 0x3D, 0x2D,
        0x35, 0xE2, 0x44, 0x4F, 0x36, 0xE3, 0x32, 0x2D, 0x35, 0xE2, 0x44, 0x4F, 0x30, 0xE3, 0x48,
        0x2D, 0x35, 0xE2, 0xEE, 0x4F, 0x36, 0xE3, 0x3E, 0x2D, 0x35, 0xE2, 0xEE, 0x4F, 0x30, 0xE3,
        0x14, 0x2D, 0x35, 0xE2, 0xEE, 0x4F, 0x31, 0xE3, 0x25, 0x2D, 0x35, 0xE2, 0x44, 0x4F, 0x34,
        0xE3, 0x3C, 0x2D, 0x35, 0xE2, 0x37, 0x2D, 0x34, 0xE2, 0xAF, 0x2D, 0x35, 0xE2, 0x37, 0x2D,
        0x35, 0xE2, 0x23, 0x2D, 0x35, 0xE2, 0xFC, 0x4E, 0x37, 0xE3, 0x36, 0x2D, 0x35, 0xE2, 0x52,
        0x69, 0x63, 0x68, 0x37, 0x2D, 0x35, 0xE2, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x50, 0x45, 0x00, 0x00, 0x64, 0x86, 0x05,
        0x00,
    ];

    const BORLAND_PE32_VALID_NO_RICH_HEADER: [u8; 528] = [
        0x4D, 0x5A, 0x50, 0x00, 0x02, 0x00, 0x00, 0x00, 0x04, 0x00, 0x0F, 0x00, 0xFF, 0xFF, 0x00,
        0x00, 0xB8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x40, 0x00, 0x1A, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x02, 0x00, 0x00, 0xBA, 0x10, 0x00, 0x0E, 0x1F, 0xB4, 0x09, 0xCD, 0x21, 0xB8, 0x01,
        0x4C, 0xCD, 0x21, 0x90, 0x90, 0x54, 0x68, 0x69, 0x73, 0x20, 0x70, 0x72, 0x6F, 0x67, 0x72,
        0x61, 0x6D, 0x20, 0x6D, 0x75, 0x73, 0x74, 0x20, 0x62, 0x65, 0x20, 0x72, 0x75, 0x6E, 0x20,
        0x75, 0x6E, 0x64, 0x65, 0x72, 0x20, 0x57, 0x69, 0x6E, 0x33, 0x32, 0x0D, 0x0A, 0x24, 0x37,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, // PE
        0x50, 0x45, 0x00, 0x00, 0x4C, 0x01, 0x08, 0x00, 0xC0, 0x9C, 0x07, 0x67, 0x00, 0x00, 0x00,
        0x00,
    ];

    #[test]
    fn crss_header() {
        let header = Header::parse(&&CRSS_HEADER[..]).unwrap();
        assert!(header.dos_header.signature == DOS_MAGIC);
        assert!(header.signature == PE_MAGIC);
        assert!(header.coff_header.machine == COFF_MACHINE_X86);
        assert!(machine_to_str(header.coff_header.machine) == "X86");
        println!("header: {:?}", &header);
    }

    #[test]
    fn parse_without_dos() {
        let header = Header::parse_without_dos(&BORLAND_PE32_VALID_NO_RICH_HEADER).unwrap();
        assert_eq!(header.dos_stub, DosStub::default());
        assert_eq!(header.rich_header.is_none(), true);

        // DOS stub is default but rich parser still works
        let header = Header::parse_without_dos(&CORRECT_RICH_HEADER).unwrap();
        assert_eq!(header.dos_stub, DosStub::default());
        assert_eq!(header.rich_header.is_some(), true);
    }

    #[test]
    fn parse_borland_weird_dos_stub() {
        let dos_stub = DosStub::parse(&BORLAND_PE32_VALID_NO_RICH_HEADER, 0x200).unwrap();
        assert_ne!(dos_stub.data, BORLAND_PE32_VALID_NO_RICH_HEADER.to_vec());
    }

    #[test]
    fn parse_borland_no_rich_header() {
        let header = RichHeader::parse(&BORLAND_PE32_VALID_NO_RICH_HEADER).unwrap();
        assert_eq!(header, None);
    }

    #[test]
    fn parse_no_rich_header() {
        let header = RichHeader::parse(&NO_RICH_HEADER).unwrap();
        assert_eq!(header, None);
    }

    #[test]
    fn parse_no_rich_header_invalid_pe_pointer() {
        let header = RichHeader::parse(&NO_RICH_HEADER_INVALID_PE_POINTER);
        assert_eq!(header.is_err(), true);
        if let Err(error::Error::Malformed(msg)) = header {
            assert_eq!(msg, "cannot parse PE header signature (offset 0xff3c)");
        } else {
            panic!("Expected a Malformed error but got {:?}", header);
        }
    }

    #[test]
    fn parse_correct_rich_header() {
        let header = RichHeader::parse(&CORRECT_RICH_HEADER).unwrap();
        assert_ne!(header, None);
        let header = header.unwrap();
        let expected = vec![
            RichMetadata {
                build: 25203,
                product: 260,
                use_count: 10,
            },
            RichMetadata {
                build: 25203,
                product: 259,
                use_count: 5,
            },
            RichMetadata {
                build: 25203,
                product: 261,
                use_count: 127,
            },
            RichMetadata {
                build: 25305,
                product: 259,
                use_count: 9,
            },
            RichMetadata {
                build: 25305,
                product: 261,
                use_count: 35,
            },
            RichMetadata {
                build: 25305,
                product: 260,
                use_count: 18,
            },
            RichMetadata {
                build: 25203,
                product: 257,
                use_count: 11,
            },
            RichMetadata {
                build: 0,
                product: 1,
                use_count: 152,
            },
            RichMetadata {
                build: 0,
                product: 0,
                use_count: 20,
            },
            RichMetadata {
                build: 25547,
                product: 258,
                use_count: 1,
            },
        ];
        assert_eq!(
            header
                .metadatas()
                .filter_map(Result::ok)
                .collect::<Vec<RichMetadata>>(),
            expected
        );
    }

    #[test]
    fn parse_corrupted_rich_header() {
        let header_result = RichHeader::parse(&CORRUPTED_RICH_HEADER);
        assert_eq!(header_result.is_err(), true);
    }
}
