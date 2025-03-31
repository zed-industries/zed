#[macro_export]
macro_rules! onboarding_event {
    ($name:expr_2021) => {
        telemetry::event!($name, source = "Edit Prediction Onboarding");
    };
    ($name:expr_2021, $($key:ident $(= $value:expr_2021)?),+ $(,)?) => {
        telemetry::event!($name, source = "Edit Prediction Onboarding", $($key $(= $value)?),+);
    };
}
