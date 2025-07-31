#[macro_export]
macro_rules! onboarding_event {
    ($name:expr) => {
        telemetry::event!($name, source = "Edit Prediction Onboarding");
    };
    ($name:expr, $($key:ident $(= $value:expr)?),+ $(,)?) => {
        telemetry::event!($name, source = "Edit Prediction Onboarding", $($key $(= $value)?),+);
    };
}
