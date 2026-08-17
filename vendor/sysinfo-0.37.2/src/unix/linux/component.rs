// Take a look at the license at the top of the repository in the LICENSE file.

// Information about values readable from `hwmon` sysfs.
//
// Values in /sys/class/hwmonN are `c_long` or `c_ulong`
// transposed to rust we only read `u32` or `i32` values.
use crate::Component;

use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs::{File, read_dir};
use std::io::Read;
use std::path::{Path, PathBuf};

#[derive(Default)]
pub(crate) struct ComponentInner {
    /// Optional associated device of a `Component`.
    device_model: Option<String>,

    /// ID of a `Component`.
    id: Option<String>,

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
    // /// Max threshold provided by the chip/kernel
    // /// - Read in:`temp[1-*]_max`
    // /// - Unit: read as millidegree Celsius converted to Celsius.
    // threshold_max: Option<f32>,
    // /// Min threshold provided by the chip/kernel.
    // /// - Read in:`temp[1-*]_min`
    // /// - Unit: read as millidegree Celsius converted to Celsius.
    // threshold_min: Option<f32>,
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
    /// ## Linux implementation details
    ///
    /// read n: `temp[1-*]_label` Suggested temperature channel label.
    /// Value: Text string
    ///
    /// Should only be created if the driver has hints about what
    /// this temperature channel is being used for, and user-space
    /// doesn't. In all other cases, the label is provided by user-space.
    label: String,
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
    pub(crate) updated: bool,
}

impl ComponentInner {
    fn update_from(
        &mut self,
        Component {
            inner:
                ComponentInner {
                    temperature,
                    max,
                    input_file,
                    highest_file,
                    ..
                },
        }: Component,
    ) {
        if let Some(temp) = temperature {
            self.temperature = Some(temp);
        }
        match (max, self.max) {
            (Some(new_max), Some(old_max)) => self.max = Some(new_max.max(old_max)),
            (Some(max), None) => self.max = Some(max),
            _ => {}
        }
        if input_file.is_some() && input_file != self.input_file {
            self.input_file = input_file;
        }
        if highest_file.is_some() && highest_file != self.highest_file {
            self.highest_file = highest_file;
        }
        self.updated = true;
    }
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
    /// Not all types are supported by all chips so we keep space for unknown sensors.
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
        // "max" => component.threshold_max = get_temperature_from_file(&hwmon_file),
        // "min" => component.threshold_min = get_temperature_from_file(&hwmon_file),
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
            if !entry.file_type().is_ok_and(|file_type| !file_type.is_dir()) {
                continue;
            }

            let entry = entry.path();
            let filename = entry.file_name().and_then(|x| x.to_str()).unwrap_or("");
            let Some((id, item)) = filename
                .strip_prefix("temp")
                .and_then(|f| f.split_once('_'))
                .and_then(|(id, item)| Some((id.parse::<u32>().ok()?, item)))
            else {
                continue;
            };

            let component = matchings.entry(id).or_insert_with(|| Component {
                inner: ComponentInner::default(),
            });
            let component = &mut component.inner;
            let name = get_file_line(&folder.join("name"), 16);
            let component_id = folder
                .file_name()
                .and_then(OsStr::to_str)
                .map(|f| format!("{f}_{id}"));
            component.name = name.unwrap_or_default();
            component.id = component_id;
            let device_model = get_file_line(&folder.join("device/model"), 16);
            component.device_model = device_model;
            fill_component(component, item, folder, filename);
        }
        for (id, mut new_comp) in matchings
            .into_iter()
            // Remove components without `tempN_input` file termal. `Component` doesn't support this
            // kind of sensors yet
            .filter(|(_, c)| c.inner.input_file.is_some())
        {
            // compute label from known data
            new_comp.inner.label = new_comp.inner.format_label("temp", id);
            if let Some(comp) = components
                .iter_mut()
                .find(|comp| comp.inner.label == new_comp.inner.label)
            {
                comp.inner.update_from(new_comp);
            } else {
                new_comp.inner.updated = true;
                components.push(new_comp);
            }
        }

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
                format!("{name} {label} {device_model}")
            }
            (true, None) => format!("{name} {label}"),
            (false, Some(device_model)) => format!("{name} {device_model}"),
            (false, None) => format!("{name} {class}{id}"),
        }
    }

    pub(crate) fn temperature(&self) -> Option<f32> {
        self.temperature
    }

    pub(crate) fn max(&self) -> Option<f32> {
        self.max
    }

    pub(crate) fn critical(&self) -> Option<f32> {
        self.threshold_critical
    }

    pub(crate) fn label(&self) -> &str {
        &self.label
    }

    pub(crate) fn id(&self) -> Option<&str> {
        self.id.as_deref()
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

fn read_temp_dir<F: FnMut(PathBuf)>(path: &Path, starts_with: &str, mut f: F) {
    if let Ok(dir) = read_dir(path) {
        for entry in dir.flatten() {
            if !entry
                .file_name()
                .to_str()
                .unwrap_or("")
                .starts_with(starts_with)
            {
                continue;
            }
            let path = entry.path();
            if !path.is_file() {
                f(path);
            }
        }
    }
}

pub(crate) struct ComponentsInner {
    pub(crate) components: Vec<Component>,
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

    pub(crate) fn refresh(&mut self) {
        self.refresh_from_sys_class_path(Path::new("/sys/class"));
    }

    fn refresh_from_sys_class_path(&mut self, path: &Path) {
        read_temp_dir(&path.join("hwmon"), "hwmon", |path| {
            ComponentInner::from_hwmon(&mut self.components, &path);
        });
        if self.components.is_empty() {
            // Normally should only be used by raspberry pi.
            read_temp_dir(&path.join("thermal"), "thermal_", |path| {
                let temp = path.join("temp");
                if temp.exists() {
                    let Some(name) = get_file_line(&path.join("type"), 16) else {
                        return;
                    };
                    let component_id = path.file_name().and_then(OsStr::to_str).map(str::to_string);
                    let mut component = ComponentInner {
                        name,
                        id: component_id,
                        ..Default::default()
                    };
                    fill_component(&mut component, "input", &path, "temp");
                    self.components.push(Component { inner: component });
                }
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile;

    #[test]
    fn test_component_refresh_simple() {
        let temp_dir = tempfile::tempdir().expect("failed to create temporary directory");
        let hwmon0_dir = temp_dir.path().join("hwmon/hwmon0");

        fs::create_dir_all(temp_dir.path().join("hwmon/hwmon0"))
            .expect("failed to create hwmon/hwmon0 directory");

        fs::write(hwmon0_dir.join("name"), "test_name").expect("failed to write to name file");
        fs::write(hwmon0_dir.join("temp1_input"), "1234")
            .expect("failed to write to temp1_input file");

        let mut components = ComponentsInner::new();
        components.refresh_from_sys_class_path(temp_dir.path());
        let components = components.into_vec();

        assert_eq!(components.len(), 1);
        assert_eq!(components[0].inner.name, "test_name");
        assert_eq!(components[0].label(), "test_name temp1");
        assert_eq!(components[0].temperature(), Some(1.234));
        assert_eq!(components[0].id(), Some("hwmon0_1"));
    }

    #[test]
    fn test_component_refresh_with_more_data() {
        let temp_dir = tempfile::tempdir().expect("failed to create temporary directory");
        let hwmon0_dir = temp_dir.path().join("hwmon/hwmon0");

        // create hwmon0 file including device/model file
        fs::create_dir_all(hwmon0_dir.join("device"))
            .expect("failed to create hwmon/hwmon0 directory");

        fs::write(hwmon0_dir.join("name"), "test_name").expect("failed to write to name file");
        fs::write(hwmon0_dir.join("device/model"), "test_model")
            .expect("failed to write to model file");

        fs::write(hwmon0_dir.join("temp1_label"), "test_label1")
            .expect("failed to write to temp1_label file");
        fs::write(hwmon0_dir.join("temp1_input"), "1234")
            .expect("failed to write to temp1_input file");
        fs::write(hwmon0_dir.join("temp1_crit"), "100").expect("failed to write to temp1_min file");

        let mut components = ComponentsInner::new();
        components.refresh_from_sys_class_path(temp_dir.path());
        let components = components.into_vec();

        assert_eq!(components.len(), 1);
        assert_eq!(components[0].inner.name, "test_name");
        assert_eq!(components[0].label(), "test_name test_label1 test_model");
        assert_eq!(components[0].temperature(), Some(1.234));
        assert_eq!(components[0].max(), Some(1.234));
        assert_eq!(components[0].critical(), Some(0.1));
        assert_eq!(components[0].id(), Some("hwmon0_1"));
    }

    #[test]
    fn test_component_refresh_multiple_sensors() {
        let temp_dir = tempfile::tempdir().expect("failed to create temporary directory");
        let hwmon0_dir = temp_dir.path().join("hwmon/hwmon0");
        fs::create_dir_all(&hwmon0_dir).expect("failed to create hwmon/hwmon0 directory");

        fs::write(hwmon0_dir.join("name"), "test_name").expect("failed to write to name file");

        fs::write(hwmon0_dir.join("temp1_label"), "test_label1")
            .expect("failed to write to temp1_label file");
        fs::write(hwmon0_dir.join("temp1_input"), "1234")
            .expect("failed to write to temp1_input file");
        fs::write(hwmon0_dir.join("temp1_crit"), "100").expect("failed to write to temp1_min file");

        fs::write(hwmon0_dir.join("temp2_label"), "test_label2")
            .expect("failed to write to temp2_label file");
        fs::write(hwmon0_dir.join("temp2_input"), "5678")
            .expect("failed to write to temp2_input file");
        fs::write(hwmon0_dir.join("temp2_crit"), "200").expect("failed to write to temp2_min file");

        let mut components = ComponentsInner::new();
        components.refresh_from_sys_class_path(temp_dir.path());
        let mut components = components.into_vec();
        components.sort_by_key(|c| c.inner.label.clone());

        assert_eq!(components.len(), 2);
        assert_eq!(components[0].inner.name, "test_name");
        assert_eq!(components[0].label(), "test_name test_label1");
        assert_eq!(components[0].temperature(), Some(1.234));
        assert_eq!(components[0].max(), Some(1.234));
        assert_eq!(components[0].id(), Some("hwmon0_1"));
        assert_eq!(components[0].critical(), Some(0.1));

        assert_eq!(components[1].inner.name, "test_name");
        assert_eq!(components[1].label(), "test_name test_label2");
        assert_eq!(components[1].temperature(), Some(5.678));
        assert_eq!(components[1].max(), Some(5.678));
        assert_eq!(components[1].id(), Some("hwmon0_2"));
        assert_eq!(components[1].critical(), Some(0.2));
    }

    #[test]
    fn test_component_refresh_multiple_sensors_with_device_model() {
        let temp_dir = tempfile::tempdir().expect("failed to create temporary directory");
        let hwmon0_dir = temp_dir.path().join("hwmon/hwmon0");

        // create hwmon0 file including device/model file
        fs::create_dir_all(hwmon0_dir.join("device"))
            .expect("failed to create hwmon/hwmon0 directory");

        fs::write(hwmon0_dir.join("name"), "test_name").expect("failed to write to name file");
        fs::write(hwmon0_dir.join("device/model"), "test_model")
            .expect("failed to write to model file");

        fs::write(hwmon0_dir.join("temp1_label"), "test_label1")
            .expect("failed to write to temp1_label file");
        fs::write(hwmon0_dir.join("temp1_input"), "1234")
            .expect("failed to write to temp1_input file");
        fs::write(hwmon0_dir.join("temp1_crit"), "100").expect("failed to write to temp1_min file");

        fs::write(hwmon0_dir.join("temp2_label"), "test_label2")
            .expect("failed to write to temp2_label file");
        fs::write(hwmon0_dir.join("temp2_input"), "5678")
            .expect("failed to write to temp2_input file");
        fs::write(hwmon0_dir.join("temp2_crit"), "200").expect("failed to write to temp2_min file");

        let mut components = ComponentsInner::new();
        components.refresh_from_sys_class_path(temp_dir.path());
        let mut components = components.into_vec();
        components.sort_by_key(|c| c.inner.label.clone());

        assert_eq!(components.len(), 2);
        assert_eq!(components[0].inner.name, "test_name");
        assert_eq!(components[0].label(), "test_name test_label1 test_model");
        assert_eq!(components[0].temperature(), Some(1.234));
        assert_eq!(components[0].max(), Some(1.234));
        assert_eq!(components[0].critical(), Some(0.1));
        assert_eq!(components[0].id(), Some("hwmon0_1"));

        assert_eq!(components[1].inner.name, "test_name");
        assert_eq!(components[1].label(), "test_name test_label2 test_model");
        assert_eq!(components[1].temperature(), Some(5.678));
        assert_eq!(components[1].max(), Some(5.678));
        assert_eq!(components[1].critical(), Some(0.2));
        assert_eq!(components[1].id(), Some("hwmon0_2"));
    }

    #[test]
    fn test_thermal_zone() {
        let temp_dir = tempfile::tempdir().expect("failed to create temporary directory");
        let thermal_zone0_dir = temp_dir.path().join("thermal/thermal_zone0");
        let thermal_zone1_dir = temp_dir.path().join("thermal/thermal_zone1");

        // create thermal zone files
        fs::create_dir_all(thermal_zone0_dir.join("device"))
            .expect("failed to create thermal/thermal_zone0 directory");

        fs::write(thermal_zone0_dir.join("type"), "test_name")
            .expect("failed to write to name file");
        fs::write(thermal_zone0_dir.join("temp"), "1234").expect("failed to write to temp file");

        // create thermal zone files
        fs::create_dir_all(thermal_zone1_dir.join("device"))
            .expect("failed to create thermal/thermal_zone1 directory");

        fs::write(thermal_zone1_dir.join("type"), "test_name2")
            .expect("failed to write to name file");
        fs::write(thermal_zone1_dir.join("temp"), "5678").expect("failed to write to temp file");

        let mut components = ComponentsInner::new();
        components.refresh_from_sys_class_path(temp_dir.path());
        let mut components = components.into_vec();
        components.sort_by_key(|c| c.inner.name.clone());

        assert_eq!(components.len(), 2);
        assert_eq!(components[0].inner.name, "test_name");
        assert_eq!(components[0].label(), "");
        assert_eq!(components[0].temperature(), Some(1.234));
        assert_eq!(components[0].max(), Some(1.234));
        assert_eq!(components[0].id(), Some("thermal_zone0"));

        assert_eq!(components[1].inner.name, "test_name2");
        assert_eq!(components[1].label(), "");
        assert_eq!(components[1].temperature(), Some(5.678));
        assert_eq!(components[1].max(), Some(5.678));
        assert_eq!(components[1].id(), Some("thermal_zone1"));
    }
}
