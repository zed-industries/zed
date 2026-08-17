// Take a look at the license at the top of the repository in the LICENSE file.

use crate::sys::{ffi, macos::utils::IOReleaser};
use crate::Component;

use libc::{c_char, c_int, c_void};

use std::mem;

const COMPONENTS_TEMPERATURE_IDS: &[(&str, &[i8])] = &[
    ("PECI CPU", &['T' as i8, 'C' as i8, 'X' as i8, 'C' as i8]), // PECI CPU "TCXC"
    ("PECI CPU", &['T' as i8, 'C' as i8, 'X' as i8, 'c' as i8]), // PECI CPU "TCXc"
    (
        "CPU Proximity",
        &['T' as i8, 'C' as i8, '0' as i8, 'P' as i8],
    ), // CPU Proximity (heat spreader) "TC0P"
    ("GPU", &['T' as i8, 'G' as i8, '0' as i8, 'P' as i8]),      // GPU "TG0P"
    ("Battery", &['T' as i8, 'B' as i8, '0' as i8, 'T' as i8]),  // Battery "TB0T"
];

pub(crate) struct ComponentFFI {
    input_structure: ffi::KeyData_t,
    val: ffi::Val_t,
    /// It is the `System::connection`. We need it to not require an extra argument
    /// in `ComponentInner::refresh`.
    connection: ffi::io_connect_t,
}

impl ComponentFFI {
    fn new(key: &[i8], connection: ffi::io_connect_t) -> Option<ComponentFFI> {
        unsafe {
            get_key_size(connection, key)
                .ok()
                .map(|(input_structure, val)| ComponentFFI {
                    input_structure,
                    val,
                    connection,
                })
        }
    }

    fn temperature(&self) -> Option<f32> {
        get_temperature_inner(self.connection, &self.input_structure, &self.val)
    }
}

// Used to get CPU information, not supported on iOS, or inside the default macOS sandbox.
pub(crate) struct ComponentsInner {
    components: Vec<Component>,
    connection: Option<IoService>,
}

impl ComponentsInner {
    pub(crate) fn new() -> Self {
        Self {
            components: Vec::with_capacity(2),
            connection: IoService::new_connection(),
        }
    }

    pub(crate) fn from_vec(components: Vec<Component>) -> Self {
        Self {
            components,
            connection: IoService::new_connection(),
        }
    }

    pub(crate) fn into_vec(self) -> Vec<Component> {
        self.components
    }

    pub(crate) fn list(&self) -> &[Component] {
        &self.components
    }

    pub(crate) fn list_mut(&mut self) -> &mut [Component] {
        &mut self.components
    }

    pub(crate) fn refresh_list(&mut self) {
        if let Some(ref connection) = self.connection {
            let connection = connection.inner();
            self.components.clear();
            // getting CPU critical temperature
            let critical_temp =
                get_temperature(connection, &['T' as i8, 'C' as i8, '0' as i8, 'D' as i8, 0]);

            for (id, v) in COMPONENTS_TEMPERATURE_IDS.iter() {
                if let Some(c) =
                    ComponentInner::new((*id).to_owned(), None, critical_temp, v, connection)
                {
                    self.components.push(Component { inner: c });
                }
            }
        }
    }
}

pub(crate) struct ComponentInner {
    temperature: f32,
    max: f32,
    critical: Option<f32>,
    label: String,
    ffi_part: ComponentFFI,
}

impl ComponentInner {
    /// Creates a new `ComponentInner` with the given information.
    pub(crate) fn new(
        label: String,
        max: Option<f32>,
        critical: Option<f32>,
        key: &[i8],
        connection: ffi::io_connect_t,
    ) -> Option<Self> {
        let ffi_part = ComponentFFI::new(key, connection)?;
        ffi_part.temperature().map(|temperature| Self {
            temperature,
            label,
            max: max.unwrap_or(temperature),
            critical,
            ffi_part,
        })
    }

    pub(crate) fn temperature(&self) -> f32 {
        self.temperature
    }

    pub(crate) fn max(&self) -> f32 {
        self.max
    }

    pub(crate) fn critical(&self) -> Option<f32> {
        self.critical
    }

    pub(crate) fn label(&self) -> &str {
        &self.label
    }

    pub(crate) fn refresh(&mut self) {
        if let Some(temp) = self.ffi_part.temperature() {
            self.temperature = temp;
            if self.temperature > self.max {
                self.max = self.temperature;
            }
        }
    }
}

unsafe fn perform_call(
    conn: ffi::io_connect_t,
    index: c_int,
    input_structure: *const ffi::KeyData_t,
    output_structure: *mut ffi::KeyData_t,
) -> i32 {
    let mut structure_output_size = mem::size_of::<ffi::KeyData_t>();

    ffi::IOConnectCallStructMethod(
        conn,
        index as u32,
        input_structure,
        mem::size_of::<ffi::KeyData_t>(),
        output_structure,
        &mut structure_output_size,
    )
}

// Adapted from https://github.com/lavoiesl/osx-cpu-temp/blob/master/smc.c#L28
#[inline]
fn strtoul(s: &[i8]) -> u32 {
    unsafe {
        ((*s.get_unchecked(0) as u32) << (3u32 << 3))
            + ((*s.get_unchecked(1) as u32) << (2u32 << 3))
            + ((*s.get_unchecked(2) as u32) << (1u32 << 3))
            + (*s.get_unchecked(3) as u32)
    }
}

#[inline]
unsafe fn ultostr(s: *mut c_char, val: u32) {
    *s.offset(0) = ((val >> 24) % 128) as i8;
    *s.offset(1) = ((val >> 16) % 128) as i8;
    *s.offset(2) = ((val >> 8) % 128) as i8;
    *s.offset(3) = (val % 128) as i8;
    *s.offset(4) = 0;
}

unsafe fn get_key_size(
    con: ffi::io_connect_t,
    key: &[i8],
) -> Result<(ffi::KeyData_t, ffi::Val_t), i32> {
    let mut input_structure: ffi::KeyData_t = mem::zeroed::<ffi::KeyData_t>();
    let mut output_structure: ffi::KeyData_t = mem::zeroed::<ffi::KeyData_t>();
    let mut val: ffi::Val_t = mem::zeroed::<ffi::Val_t>();

    input_structure.key = strtoul(key);
    input_structure.data8 = ffi::SMC_CMD_READ_KEYINFO;

    let result = perform_call(
        con,
        ffi::KERNEL_INDEX_SMC,
        &input_structure,
        &mut output_structure,
    );
    if result != ffi::KIO_RETURN_SUCCESS {
        return Err(result);
    }

    val.data_size = output_structure.key_info.data_size;
    ultostr(
        val.data_type.as_mut_ptr(),
        output_structure.key_info.data_type,
    );
    input_structure.key_info.data_size = val.data_size;
    input_structure.data8 = ffi::SMC_CMD_READ_BYTES;
    Ok((input_structure, val))
}

unsafe fn read_key(
    con: ffi::io_connect_t,
    input_structure: &ffi::KeyData_t,
    mut val: ffi::Val_t,
) -> Result<ffi::Val_t, i32> {
    let mut output_structure: ffi::KeyData_t = mem::zeroed::<ffi::KeyData_t>();

    match perform_call(
        con,
        ffi::KERNEL_INDEX_SMC,
        input_structure,
        &mut output_structure,
    ) {
        ffi::KIO_RETURN_SUCCESS => {
            libc::memcpy(
                val.bytes.as_mut_ptr() as *mut c_void,
                output_structure.bytes.as_mut_ptr() as *mut c_void,
                mem::size_of::<[u8; 32]>(),
            );
            Ok(val)
        }
        result => Err(result),
    }
}

fn get_temperature_inner(
    con: ffi::io_connect_t,
    input_structure: &ffi::KeyData_t,
    original_val: &ffi::Val_t,
) -> Option<f32> {
    unsafe {
        if let Ok(val) = read_key(con, input_structure, (*original_val).clone()) {
            if val.data_size > 0
                && libc::strcmp(val.data_type.as_ptr(), b"sp78\0".as_ptr() as *const i8) == 0
            {
                // convert sp78 value to temperature
                let x = (i32::from(val.bytes[0]) << 6) + (i32::from(val.bytes[1]) >> 2);
                return Some(x as f32 / 64f32);
            }
        }
    }
    None
}

fn get_temperature(con: ffi::io_connect_t, key: &[i8]) -> Option<f32> {
    unsafe {
        let (input_structure, val) = get_key_size(con, key).ok()?;
        get_temperature_inner(con, &input_structure, &val)
    }
}

pub(crate) struct IoService(ffi::io_connect_t);

impl IoService {
    fn new(obj: ffi::io_connect_t) -> Option<Self> {
        if obj == 0 {
            None
        } else {
            Some(Self(obj))
        }
    }

    pub(crate) fn inner(&self) -> ffi::io_connect_t {
        self.0
    }

    // code from https://github.com/Chris911/iStats
    // Not supported on iOS, or in the default macOS
    pub(crate) fn new_connection() -> Option<Self> {
        let mut iterator: ffi::io_iterator_t = 0;

        unsafe {
            let matching_dictionary = ffi::IOServiceMatching(b"AppleSMC\0".as_ptr() as *const i8);
            if matching_dictionary.is_null() {
                sysinfo_debug!("IOServiceMatching call failed, `AppleSMC` not found");
                return None;
            }
            let result = ffi::IOServiceGetMatchingServices(
                ffi::kIOMasterPortDefault,
                matching_dictionary,
                &mut iterator,
            );
            if result != ffi::KIO_RETURN_SUCCESS {
                sysinfo_debug!("Error: IOServiceGetMatchingServices() = {}", result);
                return None;
            }
            let iterator = match IOReleaser::new(iterator) {
                Some(i) => i,
                None => {
                    sysinfo_debug!("Error: IOServiceGetMatchingServices() succeeded but returned invalid descriptor");
                    return None;
                }
            };

            let device = match IOReleaser::new(ffi::IOIteratorNext(iterator.inner())) {
                Some(d) => d,
                None => {
                    sysinfo_debug!("Error: no SMC found");
                    return None;
                }
            };

            let mut conn = 0;
            let result = ffi::IOServiceOpen(device.inner(), libc::mach_task_self(), 0, &mut conn);
            if result != ffi::KIO_RETURN_SUCCESS {
                sysinfo_debug!("Error: IOServiceOpen() = {}", result);
                return None;
            }
            let conn = IoService::new(conn);
            if conn.is_none() {
                sysinfo_debug!(
                    "Error: IOServiceOpen() succeeded but returned invalid descriptor..."
                );
            }
            conn
        }
    }
}

impl Drop for IoService {
    fn drop(&mut self) {
        unsafe {
            ffi::IOServiceClose(self.0);
        }
    }
}
