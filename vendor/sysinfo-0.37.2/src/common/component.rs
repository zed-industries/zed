// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{ComponentInner, ComponentsInner};

/// Interacting with components.
///
/// ```no_run
/// use sysinfo::Components;
///
/// let components = Components::new_with_refreshed_list();
/// for component in &components {
///     println!("{component:?}");
/// }
/// ```
pub struct Components {
    pub(crate) inner: ComponentsInner,
}

impl Default for Components {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Components> for Vec<Component> {
    fn from(components: Components) -> Self {
        components.inner.into_vec()
    }
}

impl From<Vec<Component>> for Components {
    fn from(components: Vec<Component>) -> Self {
        Self {
            inner: ComponentsInner::from_vec(components),
        }
    }
}

impl std::ops::Deref for Components {
    type Target = [Component];

    fn deref(&self) -> &Self::Target {
        self.list()
    }
}

impl std::ops::DerefMut for Components {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.list_mut()
    }
}

impl<'a> IntoIterator for &'a Components {
    type Item = &'a Component;
    type IntoIter = std::slice::Iter<'a, Component>;

    fn into_iter(self) -> Self::IntoIter {
        self.list().iter()
    }
}

impl<'a> IntoIterator for &'a mut Components {
    type Item = &'a mut Component;
    type IntoIter = std::slice::IterMut<'a, Component>;

    fn into_iter(self) -> Self::IntoIter {
        self.list_mut().iter_mut()
    }
}

impl Components {
    /// Creates a new empty [`Components`][crate::Components] type.
    ///
    /// If you want it to be filled directly, take a look at
    /// [`Components::new_with_refreshed_list`].
    ///
    /// ```no_run
    /// use sysinfo::Components;
    ///
    /// let mut components = Components::new();
    /// components.refresh(false);
    /// for component in &components {
    ///     println!("{component:?}");
    /// }
    /// ```
    pub fn new() -> Self {
        Self {
            inner: ComponentsInner::new(),
        }
    }

    /// Creates a new [`Components`][crate::Components] type with the components list
    /// loaded.
    ///
    /// ```no_run
    /// use sysinfo::Components;
    ///
    /// let mut components = Components::new_with_refreshed_list();
    /// for component in components.list() {
    ///     println!("{component:?}");
    /// }
    /// ```
    pub fn new_with_refreshed_list() -> Self {
        let mut components = Self::new();
        components.refresh(true);
        components
    }

    /// Returns the components list.
    ///
    /// ```no_run
    /// use sysinfo::Components;
    ///
    /// let components = Components::new_with_refreshed_list();
    /// for component in components.list() {
    ///     println!("{component:?}");
    /// }
    /// ```
    pub fn list(&self) -> &[Component] {
        self.inner.list()
    }

    /// Returns the components list.
    ///
    /// ```no_run
    /// use sysinfo::Components;
    ///
    /// let mut components = Components::new_with_refreshed_list();
    /// for component in components.list_mut() {
    ///     component.refresh();
    ///     println!("{component:?}");
    /// }
    /// ```
    pub fn list_mut(&mut self) -> &mut [Component] {
        self.inner.list_mut()
    }

    /// Refreshes the components list.
    ///
    /// ```no_run
    /// use sysinfo::Components;
    ///
    /// let mut components = Components::new_with_refreshed_list();
    /// // We wait some time...?
    /// components.refresh(false);
    /// ```
    pub fn refresh(&mut self, remove_not_listed_components: bool) {
        self.inner.refresh();
        if remove_not_listed_components {
            // Remove interfaces which are gone.
            self.inner.components.retain_mut(|c| {
                if !c.inner.updated {
                    return false;
                }
                c.inner.updated = false;
                true
            });
        }
    }
}

/// Getting a component temperature information.
///
/// ```no_run
/// use sysinfo::Components;
///
/// let components = Components::new_with_refreshed_list();
/// for component in &components {
///     if let Some(temperature) = component.temperature() {
///         println!("{} {temperature}°C", component.label());
///     } else {
///         println!("{} (unknown temperature)", component.label());
///     }
/// }
/// ```
pub struct Component {
    pub(crate) inner: ComponentInner,
}

impl Component {
    /// Returns the temperature of the component (in celsius degree).
    ///
    /// ## Linux
    ///
    /// Returns `f32::NAN` if it failed to retrieve it.
    ///
    /// ```no_run
    /// use sysinfo::Components;
    ///
    /// let components = Components::new_with_refreshed_list();
    /// for component in &components {
    ///     if let Some(temperature) = component.temperature() {
    ///         println!("{temperature}°C");
    ///     }
    /// }
    /// ```
    pub fn temperature(&self) -> Option<f32> {
        self.inner.temperature()
    }

    /// Returns the maximum temperature of the component (in celsius degree).
    ///
    /// Note: if `temperature` is higher than the current `max`,
    /// `max` value will be updated on refresh.
    ///
    /// ## Linux
    ///
    /// May be computed by `sysinfo` from kernel.
    /// Returns `f32::NAN` if it failed to retrieve it.
    ///
    /// ```no_run
    /// use sysinfo::Components;
    ///
    /// let components = Components::new_with_refreshed_list();
    /// for component in &components {
    ///     if let Some(max) = component.max() {
    ///         println!("{max}°C");
    ///     }
    /// }
    /// ```
    pub fn max(&self) -> Option<f32> {
        self.inner.max()
    }

    /// Returns the highest temperature before the component halts (in celsius degree).
    ///
    /// ## Linux
    ///
    /// Critical threshold defined by chip or kernel.
    ///
    /// ```no_run
    /// use sysinfo::Components;
    ///
    /// let components = Components::new_with_refreshed_list();
    /// for component in &components {
    ///     if let Some(critical) = component.critical() {
    ///         println!("{critical}°C");
    ///     }
    /// }
    /// ```
    pub fn critical(&self) -> Option<f32> {
        self.inner.critical()
    }

    /// Returns the label of the component.
    ///
    /// ## Linux
    ///
    /// Since components information is retrieved thanks to `hwmon`,
    /// the labels are generated as follows.
    /// Note: it may change and it was inspired by `sensors` own formatting.
    ///
    /// | name | label | device_model | id_sensor | Computed label by `sysinfo` |
    /// |---------|--------|------------|----------|----------------------|
    /// | ✓    | ✓    | ✓  | ✓ | `"{name} {label} {device_model}"` |
    /// | ✓    | ✓    | ✗  | ✓ | `"{name} {label}"` |
    /// | ✓    | ✗    | ✓  | ✓ | `"{name} {device_model}"` |
    /// | ✓    | ✗    | ✗  | ✓ | `"{name} temp{id}"` |
    ///
    /// ```no_run
    /// use sysinfo::Components;
    ///
    /// let components = Components::new_with_refreshed_list();
    /// for component in &components {
    ///     println!("{}", component.label());
    /// }
    /// ```
    pub fn label(&self) -> &str {
        self.inner.label()
    }

    /// Returns the identifier of the component.
    ///
    /// Note: The identifier should be reasonably unique but is provided by the kernel.
    /// It could change if the hardware changes or after a reboot.
    ///
    /// | OS | Computed ID by `sysinfo` | Example |
    /// |----|--------------------------|----------|
    /// | Linux/hwmon | hwmon file concatenated with the temp index. | ` hwmon0_1` if the temperature data comes from the `hwmon0/temp1_input` file. |
    /// | Linux/thermal | thermal file name | `thermal_zone0` |
    /// | FreeBSD | `cpu_` concatenated with the core index. | `cpu_1` for the first core. |
    /// | macOS/arm | Serial ID reported by the HID driver. | |
    /// | macOS/x86 | Technical ID sent to the OS (see below) | `TXCX` |
    /// | Windows | `Computer` (same as the label) | `Computer` |
    /// | appstore | Components are not available | None |
    /// | unknown | Components are not available | None |
    ///
    /// For macOS on X86 the following identifiers are possible:
    /// - `TXCX` or `TXCx` for PECI CPU (depending on if run on iMac or MacBook)
    /// - `TC0P` for CPU Proximity
    /// - `TG0P` for GPU
    /// - `TB0T` for Battery
    ///
    /// ```no_run
    /// use sysinfo::Components;
    ///
    /// let components = Components::new_with_refreshed_list();
    /// for component in &components {
    ///     if let Some(id) = component.id() {
    ///         println!("{id}");
    ///     }
    /// }
    /// ```
    pub fn id(&self) -> Option<&str> {
        self.inner.id()
    }

    /// Refreshes component.
    ///
    /// ```no_run
    /// use sysinfo::Components;
    ///
    /// let mut components = Components::new_with_refreshed_list();
    /// for component in components.iter_mut() {
    ///     component.refresh();
    /// }
    /// ```
    pub fn refresh(&mut self) {
        self.inner.refresh()
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_components_mac_m1() {
        let mut components = Components::new();
        components.refresh(false);
        components.refresh(false);
    }
}
