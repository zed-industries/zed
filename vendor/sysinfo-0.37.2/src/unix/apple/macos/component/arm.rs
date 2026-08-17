// Take a look at the license at the top of the repository in the LICENSE file.

use objc2_core_foundation::{
    CFArray, CFDictionary, CFNumber, CFRetained, CFString, kCFAllocatorDefault,
};
use objc2_io_kit::{IOHIDEventSystemClient, IOHIDServiceClient};

use crate::Component;
use crate::sys::inner::ffi::{
    HID_DEVICE_PROPERTY_PRIMARY_USAGE, HID_DEVICE_PROPERTY_PRIMARY_USAGE_PAGE,
    HID_DEVICE_PROPERTY_PRODUCT, IOHIDEventFieldBase, IOHIDEventGetFloatValue,
    IOHIDEventSystemClientCreate, IOHIDEventSystemClientSetMatching, IOHIDServiceClientCopyEvent,
    kHIDPage_AppleVendor, kHIDUsage_AppleVendor_TemperatureSensor, kIOHIDEventTypeTemperature,
    kIOHIDSerialNumberKey,
};

pub(crate) struct ComponentsInner {
    pub(crate) components: Vec<Component>,
    client: Option<CFRetained<IOHIDEventSystemClient>>,
}

// SAFETY: `ComponentsInner::client` is never updated in a `&self` context, so it's safe to
// make the type `Sync` and `Send`.
unsafe impl Send for ComponentsInner {}
unsafe impl Sync for ComponentsInner {}

impl ComponentsInner {
    pub(crate) fn new() -> Self {
        Self {
            components: vec![],
            client: None,
        }
    }

    pub(crate) fn from_vec(components: Vec<Component>) -> Self {
        Self {
            components,
            client: None,
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

    #[allow(unreachable_code)]
    pub(crate) fn refresh(&mut self) {
        let keys = [
            &*CFString::from_static_str(HID_DEVICE_PROPERTY_PRIMARY_USAGE_PAGE),
            &*CFString::from_static_str(HID_DEVICE_PROPERTY_PRIMARY_USAGE),
        ];

        let nums = [
            &*CFNumber::new_i32(kHIDPage_AppleVendor),
            &*CFNumber::new_i32(kHIDUsage_AppleVendor_TemperatureSensor),
        ];

        let matches = CFDictionary::from_slices(&keys, &nums);
        let matches = matches.as_opaque();

        unsafe {
            if self.client.is_none() {
                let client = match IOHIDEventSystemClientCreate(kCFAllocatorDefault) {
                    // SAFETY: `IOHIDEventSystemClientCreate` is a "create"
                    // function, so the client has +1 retain count.
                    Some(c) => CFRetained::from_raw(c),
                    None => return,
                };
                self.client = Some(client);
            }

            let client = self.client.as_ref().unwrap();

            let _ = IOHIDEventSystemClientSetMatching(client, matches);

            let services = match client.services() {
                Some(s) => s,
                None => return,
            };

            // SAFETY: Return type documented to be CFArray of IOHIDServiceClient.
            let services = CFRetained::cast_unchecked::<CFArray<IOHIDServiceClient>>(services);

            let key = CFString::from_static_str(HID_DEVICE_PROPERTY_PRODUCT);
            let serial_key = CFString::from_static_str(kIOHIDSerialNumberKey);

            for service in services {
                let Some(name) = service.property(&key) else {
                    continue;
                };
                let serial = service
                    .property(&serial_key)
                    .and_then(|value| value.downcast::<CFString>().ok())
                    .as_deref()
                    .map(CFString::to_string);
                let name = name.downcast::<CFString>().unwrap();
                let name_str = name.to_string();

                if let Some(c) = self
                    .components
                    .iter_mut()
                    .find(|c| c.inner.label == name_str)
                {
                    c.refresh();
                    c.inner.updated = true;
                    continue;
                }

                let mut component = ComponentInner::new(serial, name_str, None, None, service);
                component.refresh();

                self.components.push(Component { inner: component });
            }
        }
    }
}

pub(crate) struct ComponentInner {
    id: Option<String>,
    service: CFRetained<IOHIDServiceClient>,
    temperature: Option<f32>,
    label: String,
    max: f32,
    critical: Option<f32>,
    pub(crate) updated: bool,
}

// SAFETY: `ComponentsInner::service` is never updated, so it's safe to make the type `Sync`
// and `Send`.
unsafe impl Send for ComponentInner {}
unsafe impl Sync for ComponentInner {}

impl ComponentInner {
    pub(crate) fn new(
        id: Option<String>,
        label: String,
        max: Option<f32>,
        critical: Option<f32>,
        service: CFRetained<IOHIDServiceClient>,
    ) -> Self {
        Self {
            id,
            service,
            label,
            max: max.unwrap_or(0.),
            critical,
            temperature: None,
            updated: true,
        }
    }

    pub(crate) fn temperature(&self) -> Option<f32> {
        self.temperature
    }

    pub(crate) fn max(&self) -> Option<f32> {
        Some(self.max)
    }

    pub(crate) fn critical(&self) -> Option<f32> {
        self.critical
    }

    pub(crate) fn label(&self) -> &str {
        &self.label
    }

    pub(crate) fn id(&self) -> Option<&str> {
        self.id.as_deref()
    }

    pub(crate) fn refresh(&mut self) {
        unsafe {
            let Some(event) =
                IOHIDServiceClientCopyEvent(&self.service, kIOHIDEventTypeTemperature, 0, 0)
            else {
                self.temperature = None;
                return;
            };
            // SAFETY: `IOHIDServiceClientCopyEvent` is a "copy" function, so
            // the event has +1 retain count.
            let event = CFRetained::from_raw(event);

            let temperature =
                IOHIDEventGetFloatValue(&event, IOHIDEventFieldBase(kIOHIDEventTypeTemperature))
                    as _;
            self.temperature = Some(temperature);
            if temperature > self.max {
                self.max = temperature;
            }
        }
    }
}
