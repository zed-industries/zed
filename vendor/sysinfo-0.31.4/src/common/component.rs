// Take a look at the license at the top of the repository in the LICENSE file.

use crate::utils::into_iter_mut;
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
    /// components.refresh_list();
    /// for component in &components {
    ///     println!("{component:?}");
    /// }
    /// ```
    pub fn new() -> Self {
        Self {
            inner: ComponentsInner::new(),
        }
    }

    /// Creates a new [`Components`][crate::Components] type with the user list
    /// loaded. It is a combination of [`Components::new`] and
    /// [`Components::refresh_list`].
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
        components.refresh_list();
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

    /// Refreshes the listed components' information.
    ///
    /// ⚠️ If a component is added or removed, this method won't take it into account. Use
    /// [`Components::refresh_list`] instead.
    ///
    /// ⚠️ If you didn't call [`Components::refresh_list`] beforehand, this method will do
    /// nothing as the component list will be empty.
    ///
    /// ```no_run
    /// use sysinfo::Components;
    ///
    /// let mut components = Components::new_with_refreshed_list();
    /// // We wait some time...?
    /// components.refresh();
    /// ```
    pub fn refresh(&mut self) {
        #[cfg(all(
            feature = "multithread",
            not(feature = "unknown-ci"),
            not(all(target_os = "macos", feature = "apple-sandbox")),
        ))]
        use rayon::iter::ParallelIterator;
        into_iter_mut(self.list_mut()).for_each(|component| component.refresh());
    }

    /// The component list will be emptied then completely recomputed.
    ///
    /// ```no_run
    /// use sysinfo::Components;
    ///
    /// let mut components = Components::new();
    /// components.refresh_list();
    /// ```
    ///
    /// ⚠️ This function is not doing anything on macOS until
    /// [this issue](https://github.com/GuillaumeGomez/sysinfo/issues/1279) is fixed.
    pub fn refresh_list(&mut self) {
        self.inner.refresh_list()
    }
}

/// Getting a component temperature information.
///
/// ```no_run
/// use sysinfo::Components;
///
/// let components = Components::new_with_refreshed_list();
/// for component in &components {
///     println!("{} {}°C", component.label(), component.temperature());
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
    ///     println!("{}°C", component.temperature());
    /// }
    /// ```
    pub fn temperature(&self) -> f32 {
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
    ///     println!("{}°C", component.max());
    /// }
    /// ```
    pub fn max(&self) -> f32 {
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
    ///     println!("{:?}°C", component.critical());
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
    /// | ✓    | ✓    | ✓  | ✓ | `"{name} {label} {device_model} temp{id}"` |
    /// | ✓    | ✓    | ✗  | ✓ | `"{name} {label} {id}"` |
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
        components.refresh_list();
        components.refresh_list();
    }
}
