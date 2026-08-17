// Take a look at the license at the top of the repository in the LICENSE file.

use crate::Component;

pub(crate) struct ComponentInner;

impl ComponentInner {
    pub(crate) fn temperature(&self) -> f32 {
        0.0
    }

    pub(crate) fn max(&self) -> f32 {
        0.0
    }

    pub(crate) fn critical(&self) -> Option<f32> {
        None
    }

    pub(crate) fn label(&self) -> &str {
        ""
    }

    pub(crate) fn refresh(&mut self) {}
}

pub(crate) struct ComponentsInner {
    components: Vec<Component>,
}

impl ComponentsInner {
    pub(crate) fn new() -> Self {
        Self {
            components: Vec::new(),
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
        // Doesn't do anything.
    }
}
