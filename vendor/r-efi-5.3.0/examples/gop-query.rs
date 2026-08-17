// Example: Graphics Query
//
// This is a slightly more complex UEFI application than `hello-world`. It
// locates the graphics-output-protocol, queries its current mode and prints
// the current resolution to the UEFI console.
//
// This example should make everyone aware that UEFI programing in Rust really
// asks for helper layers. While the C/FFI/Spec API can be used directly, it
// is quite cumbersome. Especially the error handling is overly difficult.
//
// Nevertheless, this example shows how to find UEFI protocol and invoke
// their member functions.
//
// Like all the other r-efi examples, it is a standalone example. That is, no
// UTF-16 helpers are pulled in, nor any allocators or panic frameworks. For
// real world scenarios, you really should choose such helpers.

#![no_main]
#![no_std]

use r_efi::efi;

#[panic_handler]
fn panic_handler(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

fn fail(_r: efi::Status) -> ! {
    panic!();
}

// A simple `itoa()`-ish function that takes a u32 and turns it into a UTF-16
// string. It always prints exactly 10 characters, so leading zeroes are used
// for small numbers.
fn utoa(mut u: u32, a: &mut [u16]) {
    for i in 0..10 {
        a[9 - i] = 0x0030u16 + ((u % 10) as u16);
        u = u / 10;
    }
}

// A simple helper that takes two integers and prints them to the UEFI console
// with a short prefix. It uses a UTF-16 buffer and fills in the numbers before
// printing the entire buffer.
fn print_xy(st: *mut efi::SystemTable, x: u32, y: u32) {
    let mut s = [
        0x0058u16, 0x0059u16, 0x003au16, //            "XY:"
        0x0020u16, //                                  " "
        0x0020u16, 0x0020u16, 0x0020u16, 0x0020u16, // "    "
        0x0020u16, 0x0020u16, 0x0020u16, 0x0020u16, // "    "
        0x0020u16, 0x0020u16, //                       "    "
        0x0078u16, //                                  "x"
        0x0020u16, 0x0020u16, 0x0020u16, 0x0020u16, // "    "
        0x0020u16, 0x0020u16, 0x0020u16, 0x0020u16, // "    "
        0x0020u16, 0x0020u16, //                       "    "
        0x000au16, //                                  "\n"
        0x0000u16, //                                  NUL
    ];

    utoa(x, &mut s[4..14]);
    utoa(y, &mut s[15..25]);

    unsafe {
        let r = ((*(*st).con_out).output_string)((*st).con_out, s.as_ptr() as *mut efi::Char16);
        if r.is_error() {
            fail(r);
        }
    }
}

// This function locates singleton UEFI protocols. Those protocols do not
// require to register listener handles, but are globally available to all
// UEFI applications. It takes a GUID of the protocol to locate and returns
// the protocol pointer on success.
fn locate_singleton(
    st: *mut efi::SystemTable,
    guid: *const efi::Guid,
) -> Result<*mut core::ffi::c_void, efi::Status> {
    let mut interface: *mut core::ffi::c_void = core::ptr::null_mut();
    let mut handles: *mut efi::Handle = core::ptr::null_mut();
    let mut n_handles: usize = 0;
    let mut r: efi::Status;

    // Use `locate_handle_buffer()` to find all handles that support the
    // specified protocol.
    unsafe {
        if (*st).hdr.revision < efi::SYSTEM_TABLE_REVISION_1_10 {
            // We use `LocateHandleBuffer`, which was introduced in 1.10.
            return Err(efi::Status::UNSUPPORTED);
        }

        let r = ((*(*st).boot_services).locate_handle_buffer)(
            efi::BY_PROTOCOL,
            guid as *mut _,
            core::ptr::null_mut(),
            &mut n_handles,
            &mut handles,
        );
        match r {
            efi::Status::SUCCESS => {}
            efi::Status::NOT_FOUND => return Err(r),
            efi::Status::OUT_OF_RESOURCES => return Err(r),
            _ => panic!(),
        };
    }

    // Now that we have all handles with the specified protocol, query it for
    // the protocol interface. We loop here, even though every item should
    // succeed. Lets be on the safe side.
    // Secondly, we use `handle_protocol()` here, but really should be using
    // `open_protocol()`. But for singleton protocols, this does not matter,
    // so lets use the simple path for now.
    unsafe {
        r = efi::Status::NOT_FOUND;
        for i in 0..n_handles {
            r = ((*(*st).boot_services).handle_protocol)(
                *handles.offset(core::convert::TryFrom::<usize>::try_from(i).unwrap()),
                guid as *mut _,
                &mut interface,
            );
            match r {
                efi::Status::SUCCESS => break,
                efi::Status::UNSUPPORTED => continue,
                _ => panic!(),
            };
        }
    }

    // Free the allocated buffer memory of `handles`. This was allocated on the
    // pool by `locate_handle_buffer()`.
    unsafe {
        let r = ((*(*st).boot_services).free_pool)(handles as *mut core::ffi::c_void);
        assert!(!r.is_error());
    }

    // In case we found nothing, return `NOT_FOUND`, otherwise return the
    // interface identifier.
    match r {
        efi::Status::SUCCESS => Ok(interface),
        _ => Err(efi::Status::NOT_FOUND),
    }
}

// A simple helper that queries the current mode of the GraphicsOutputProtocol
// and returns the x and y dimensions on success.
fn query_gop(
    gop: *mut efi::protocols::graphics_output::Protocol,
) -> Result<(u32, u32), efi::Status> {
    let mut info: *mut efi::protocols::graphics_output::ModeInformation = core::ptr::null_mut();
    let mut z_info: usize = 0;

    unsafe {
        // We could just look at `gop->mode->info`, but lets query the mode
        // instead to show how to query other modes than the active one.
        let r = ((*gop).query_mode)(gop, (*(*gop).mode).mode, &mut z_info, &mut info);
        match r {
            efi::Status::SUCCESS => {}
            efi::Status::DEVICE_ERROR => return Err(r),
            _ => panic!(),
        };
        if z_info < core::mem::size_of_val(&*info) {
            return Err(efi::Status::UNSUPPORTED);
        }

        Ok(((*info).horizontal_resolution, (*info).vertical_resolution))
    }
}

// This is the UEFI application entrypoint. We use it to locate the GOP
// pointer, query the current mode, and then print it to the system console.
#[export_name = "efi_main"]
pub extern "C" fn main(_h: efi::Handle, st: *mut efi::SystemTable) -> efi::Status {
    let r = locate_singleton(st, &efi::protocols::graphics_output::PROTOCOL_GUID);
    let gop = match r {
        Ok(v) => v,
        Err(r) => fail(r),
    };

    let r = query_gop(gop as _);
    let v = match r {
        Ok(v) => v,
        Err(r) => fail(r),
    };

    print_xy(st, v.0, v.1);

    efi::Status::SUCCESS
}
