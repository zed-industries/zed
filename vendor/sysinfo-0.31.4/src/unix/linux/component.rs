// Take a look at the license at the top of the repository in the LICENSE file.

// Information about values readable from `hwmon` sysfs.
//
// Values in /sys/class/hwmonN are `c_long` or `c_ulong`
// transposed to rust we only read `u32` or `i32` values.
use crate::Component;

use std::collections::HashMap;
use std::fs::{read_dir, File};
use std::io::Read;
use std::path::{Path, PathBuf};

#[derive(Default)]
pub(crate) struct ComponentInner {
    /// Optional associated device of a `Component`.
    device_model: Option<String>,
    /// The chip name.
    ///
    /// Kernel documentation extract:
    ///
    /// ```txt
    /// This should be a short, lowercase string, not containing
    /// whitespace, dashes, or the wildcard character '*'.
    /// This attribute represents the chip name. It is the only
    /// mandatory attribute.
    /// I2C devices get this attribute created automatically.
    /// ```
    name: String,
    /// Temperature current value
    /// - Read in: `temp[1-*]_input`.
    /// - Unit: read as millidegree Celsius converted to Celsius.
    temperature: Option<f32>,
    /// Maximum value computed by `sysinfo`.
    max: Option<f32>,
    /// Max threshold provided by the chip/kernel
    /// - Read in:`temp[1-*]_max`
    /// - Unit: read as millidegree Celsius converted to Celsius.
    threshold_max: Option<f32>,
    /// Min threshold provided by the chip/kernel.
    /// - Read in:`temp[1-*]_min`
    /// - Unit: read as millidegree Celsius converted to Celsius.
    threshold_min: Option<f32>,
    /// Critical threshold provided by the chip/kernel previous user write.
    /// Read in `temp[1-*]_crit`:
    /// Typically greater than corresponding temp_max values.
    /// - Unit: read as millidegree Celsius converted to Celsius.
    threshold_critical: Option<f32>,
    /// Sensor type, not common but can exist!
    ///
    /// Read in: `temp[1-*]_type` Sensor type selection.
    /// Values integer:
    ///
    /// - 1: CPU embedded diode
    /// - 2: 3904 transistor
    /// - 3: thermal diode
    /// - 4: thermistor
    /// - 5: AMD AMDSI
    /// - 6: Intel PECI
    ///
    /// Not all types are supported by all chips.
    sensor_type: Option<ThermalSensorType>,
    /// Component Label
    ///
    /// For formatting detail see `Component::label` function docstring.
    ///
    /// ## Linux implementation details
    ///
    /// read n: `temp[1-*]_label` Suggested temperature channel label.
    /// Value: Text string
    ///
    /// Should only be created if the driver has hints about what
    /// this temperature channel is being used for, and user-space
    /// doesn't. In all other cases, the label is provided by user-space.
    label: String,
    // TODO: not used now.
    // Historical minimum temperature
    // - Read in:`temp[1-*]_lowest
    // - Unit: millidegree Celsius
    //
    // Temperature critical min value, typically lower than
    // corresponding temp_min values.
    // - Read in:`temp[1-*]_lcrit`
    // - Unit: millidegree Celsius
    //
    // Temperature emergency max value, for chips supporting more than
    // two upper temperature limits. Must be equal or greater than
    // corresponding temp_crit values.
    // - temp[1-*]_emergency
    // - Unit: millidegree Celsius
    /// File to read current temperature shall be `temp[1-*]_input`
    /// It may be absent but we don't continue if absent.
    input_file: Option<PathBuf>,
    /// `temp[1-*]_highest file` to read if available highest value.
    highest_file: Option<PathBuf>,
}

// Read arbitrary data from sysfs.
fn get_file_line(file: &Path, capacity: usize) -> Option<String> {
    let mut reader = String::with_capacity(capacity);
    let mut f = File::open(file).ok()?;
    f.read_to_string(&mut reader).ok()?;
    reader.truncate(reader.trim_end().len());
    Some(reader)
}

/// Designed at first for reading an `i32` or `u32` aka `c_long`
/// from a `/sys/class/hwmon` sysfs file.
fn read_number_from_file<N>(file: &Path) -> Option<N>
where
    N: std::str::FromStr,
{
    let mut reader = [0u8; 32];
    let mut f = File::open(file).ok()?;
    let n = f.read(&mut reader).ok()?;
    // parse and trim would complain about `\0`.
    let number = &reader[..n];
    let number = std::str::from_utf8(number).ok()?;
    let number = number.trim();
    // Assert that we cleaned a little bit that string.
    if cfg!(feature = "debug") {
        assert!(!number.contains('\n') && !number.contains('\0'));
    }
    number.parse().ok()
}

// Read a temperature from a `tempN_item` sensor form the sysfs.
// number returned will be in mili-celsius.
//
// Don't call it on `label`, `name` or `type` file.
#[inline]
fn get_temperature_from_file(file: &Path) -> Option<f32> {
    let temp = read_number_from_file(file);
    convert_temp_celsius(temp)
}

/// Takes a raw temperature in mili-celsius and convert it to celsius.
#[inline]
fn convert_temp_celsius(temp: Option<i32>) -> Option<f32> {
    temp.map(|n| (n as f32) / 1000f32)
}

/// Information about thermal sensor. It may be unavailable as it's
/// kernel module and chip dependent.
enum ThermalSensorType {
    /// 1: CPU embedded diode
    CPUEmbeddedDiode,
    /// 2: 3904 transistor
    Transistor3904,
    /// 3: thermal diode
    ThermalDiode,
    /// 4: thermistor
    Thermistor,
    /// 5: AMD AMDSI
    AMDAMDSI,
    /// 6: Intel PECI
    IntelPECI,
    /// Not all types are supported by all chips so we keep space for
    /// unknown sensors.
    #[allow(dead_code)]
    Unknown(u8),
}

impl From<u8> for ThermalSensorType {
    fn from(input: u8) -> Self {
        match input {
            0 => Self::CPUEmbeddedDiode,
            1 => Self::Transistor3904,
            3 => Self::ThermalDiode,
            4 => Self::Thermistor,
            5 => Self::AMDAMDSI,
            6 => Self::IntelPECI,
            n => Self::Unknown(n),
        }
    }
}

/// Check given `item` dispatch to read the right `file` with the right parsing and store data in
/// given `component`. `id` is provided for `label` creation.
fn fill_component(component: &mut ComponentInner, item: &str, folder: &Path, file: &str) {
    let hwmon_file = folder.join(file);
    match item {
        "type" => {
            component.sensor_type =
                read_number_from_file::<u8>(&hwmon_file).map(ThermalSensorType::from)
        }
        "input" => {
            let temperature = get_temperature_from_file(&hwmon_file);
            component.input_file = Some(hwmon_file);
            component.temperature = temperature;
            // Maximum know try to get it from `highest` if not available
            // use current temperature
            if component.max.is_none() {
                component.max = temperature;
            }
        }
        "label" => component.label = get_file_line(&hwmon_file, 10).unwrap_or_default(),
        "highest" => {
            component.max = get_temperature_from_file(&hwmon_file).or(component.temperature);
            component.highest_file = Some(hwmon_file);
        }
        "max" => component.threshold_max = get_temperature_from_file(&hwmon_file),
        "min" => component.threshold_min = get_temperature_from_file(&hwmon_file),
        "crit" => component.threshold_critical = get_temperature_from_file(&hwmon_file),
        _ => {
            sysinfo_debug!(
                "This hwmon-temp file is still not supported! Contributions are appreciated.;) {:?}",
                hwmon_file,
            );
        }
    }
}

impl ComponentInner {
    /// Read out `hwmon` info (hardware monitor) from `folder`
    /// to get values' path to be used on refresh as well as files containing `max`,
    /// `critical value` and `label`. Then we store everything into `components`.
    ///
    /// Note that a thermal [Component] must have a way to read its temperature.
    /// If not, it will be ignored and not added into `components`.
    ///
    /// ## What is read:
    ///
    /// - Mandatory: `name` the name of the `hwmon`.
    /// - Mandatory: `tempN_input` Drop [Component] if missing
    /// - Optional: sensor `label`, in the general case content of `tempN_label`
    ///   see below for special cases
    /// - Optional: `label`
    /// - Optional: `/device/model`
    /// - Optional: highest historic value in `tempN_highest`.
    /// - Optional: max threshold value defined in `tempN_max`
    /// - Optional: critical threshold value defined in `tempN_crit`
    ///
    /// Where `N` is a `u32` associated to a sensor like `temp1_max`, `temp1_input`.
    ///
    /// ## Doc to Linux kernel API.
    ///
    /// Kernel hwmon API: https://www.kernel.org/doc/html/latest/hwmon/hwmon-kernel-api.html
    /// DriveTemp kernel API: https://docs.kernel.org/gpu/amdgpu/thermal.html#hwmon-interfaces
    /// Amdgpu hwmon interface: https://www.kernel.org/doc/html/latest/hwmon/drivetemp.html
    fn from_hwmon(components: &mut Vec<Component>, folder: &Path) -> Option<()> {
        let dir = read_dir(folder).ok()?;
        let mut matchings: HashMap<u32, Component> = HashMap::with_capacity(10);
        for entry in dir.flatten() {
            let Ok(file_type) = entry.file_type() else {
                continue;
            };
            if file_type.is_dir() {
                continue;
            }

            let entry = entry.path();
            let filename = entry.file_name().and_then(|x| x.to_str()).unwrap_or("");
            if !filename.starts_with("temp") {
                continue;
            }

            let (id, item) = filename.split_once('_')?;
            let id = id.get(4..)?.parse::<u32>().ok()?;

            let component = matchings.entry(id).or_insert_with(|| Component {
                inner: ComponentInner::default(),
            });
            let component = &mut component.inner;
            let name = get_file_line(&folder.join("name"), 16);
            component.name = name.unwrap_or_default();
            let device_model = get_file_line(&folder.join("device/model"), 16);
            component.device_model = device_model;
            fill_component(component, item, folder, filename);
        }
        let compo = matchings
            .into_iter()
            .map(|(id, mut c)| {
                // sysinfo expose a generic interface with a `label`.
                // Problem: a lot of sensors don't have a label or a device model! ¯\_(ツ)_/¯
                // So let's pretend we have a unique label!
                // See the table in `Component::label` documentation for the table detail.
                c.inner.label = c.inner.format_label("temp", id);
                c
            })
            // Remove components without `tempN_input` file termal. `Component` doesn't support this kind of sensors yet
            .filter(|c| c.inner.input_file.is_some());

        components.extend(compo);
        Some(())
    }

    /// Compute a label out of available information.
    /// See the table in `Component::label`'s documentation.
    fn format_label(&self, class: &str, id: u32) -> String {
        let ComponentInner {
            device_model,
            name,
            label,
            ..
        } = self;
        let has_label = !label.is_empty();
        match (has_label, device_model) {
            (true, Some(device_model)) => {
                format!("{name} {label} {device_model} {class}{id}")
            }
            (true, None) => format!("{name} {label}"),
            (false, Some(device_model)) => format!("{name} {device_model}"),
            (false, None) => format!("{name} {class}{id}"),
        }
    }

    pub(crate) fn temperature(&self) -> f32 {
        self.temperature.unwrap_or(f32::NAN)
    }

    pub(crate) fn max(&self) -> f32 {
        self.max.unwrap_or(f32::NAN)
    }

    pub(crate) fn critical(&self) -> Option<f32> {
        self.threshold_critical
    }

    pub(crate) fn label(&self) -> &str {
        &self.label
    }

    pub(crate) fn refresh(&mut self) {
        let current = self
            .input_file
            .as_ref()
            .and_then(|file| get_temperature_from_file(file.as_path()));
        // tries to read out kernel highest if not compute something from temperature.
        let max = self
            .highest_file
            .as_ref()
            .and_then(|file| get_temperature_from_file(file.as_path()))
            .or_else(|| {
                let last = self.temperature?;
                let current = current?;
                Some(last.max(current))
            });
        self.max = max;
        self.temperature = current;
    }
}

pub(crate) struct ComponentsInner {
    components: Vec<Component>,
}

impl ComponentsInner {
    pub(crate) fn new() -> Self {
        Self {
            components: Vec::with_capacity(4),
        }
    }

    pub(crate) fn from_vec(components: Vec<Component>) -> Self {
        Self { components }
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
        self.components.clear();
        if let Ok(dir) = read_dir(Path::new("/sys/class/hwmon/")) {
            for entry in dir.flatten() {
                let Ok(file_type) = entry.file_type() else {
                    continue;
                };
                let entry = entry.path();
                if !file_type.is_file()
                    && entry
                        .file_name()
                        .and_then(|x| x.to_str())
                        .unwrap_or("")
                        .starts_with("hwmon")
                {
                    ComponentInner::from_hwmon(&mut self.components, &entry);
                }
            }
        }
        if self.components.is_empty() {
            // Specfic to raspberry pi.
            let thermal_path = Path::new("/sys/class/thermal/thermal_zone0/");
            if thermal_path.join("temp").exists() {
                let mut component = ComponentInner::default();
                fill_component(&mut component, "input", thermal_path, "temp");
                let name = get_file_line(&thermal_path.join("type"), 16);
                component.name = name.unwrap_or_default();
                self.components.push(Component { inner: component });
            }
        }
    }
}
