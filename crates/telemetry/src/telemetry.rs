pub use serde_json;
pub use telemetry_events::FlexibleEvent as Event;

#[macro_export]
macro_rules! event {
    ($name:expr) => {
        {
            let _ = &$name;
        }
    };
    ($name:expr, $($key:ident $(= $value:expr)?),+ $(,)?) => {
        {
            let _ = &$name;
            $(
                let _ = &$crate::serialize_property!($key $(= $value)?);
            )+
        }
    };
}

#[macro_export]
macro_rules! serialize_property {
    ($key:ident) => { $key };
    ($key:ident = $value:expr) => { $value };
}

pub fn send_event(_event: Event) {}

pub fn init(_tx: futures::channel::mpsc::UnboundedSender<Event>) {}
