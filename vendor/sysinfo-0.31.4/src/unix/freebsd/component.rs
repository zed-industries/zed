// Take a look at the license at the top of the repository in the LICENSE file.

use super::utils::get_sys_value_by_name;
use crate::Component;

pub(crate) struct ComponentInner {
    id: Vec<u8>,
    label: String,
    temperature: f32,
    max: f32,
}

impl ComponentInner {
    pub(crate) fn temperature(&self) -> f32 {
        self.temperature
    }

    pub(crate) fn max(&self) -> f32 {
        self.max
    }

    pub(crate) fn critical(&self) -> Option<f32> {
        None
    }

    pub(crate) fn label(&self) -> &str {
        &self.label
    }

    pub(crate) fn refresh(&mut self) {
        unsafe {
            if let Some(temperature) = refresh_component(&self.id) {
                self.temperature = temperature;
                if self.temperature > self.max {
                    self.max = self.temperature;
                }
            }
        }
    }
}

unsafe fn refresh_component(id: &[u8]) -> Option<f32> {
    let mut temperature: libc::c_int = 0;
    if !get_sys_value_by_name(id, &mut temperature) {
        None
    } else {
        // convert from Kelvin (x 10 -> 273.2 x 10) to Celsius
        Some((temperature - 2732) as f32 / 10.)
    }
}

pub(crate) struct ComponentsInner {
    nb_cpus: usize,
    components: Vec<Component>,
}

impl ComponentsInner {
    pub(crate) fn new() -> Self {
        let nb_cpus = unsafe { super::utils::get_nb_cpus() };
        Self {
            nb_cpus,
            components: Vec::with_capacity(nb_cpus),
        }
    }

    pub(crate) fn from_vec(components: Vec<Component>) -> Self {
        Self {
            nb_cpus: unsafe { super::utils::get_nb_cpus() },
            components,
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
        self.components.clear();
        for core in 0..self.nb_cpus {
            unsafe {
                let id = format!("dev.cpu.{core}.temperature\0").as_bytes().to_vec();
                if let Some(temperature) = refresh_component(&id) {
                    self.components.push(Component {
                        inner: ComponentInner {
                            id,
                            label: format!("CPU {}", core + 1),
                            temperature,
                            max: temperature,
                        },
                    });
                }
            }
        }
    }
}
