/// Prints the current time zone, e.g. "Europe/Paris".

use android_system_properties::AndroidSystemProperties;

fn main() {
    let android_system_properties = AndroidSystemProperties::new();
    let tz = android_system_properties.get("persist.sys.timezone");
    println!("Your time zone is: {}", tz.as_deref().unwrap_or("<unknown>"));
}
