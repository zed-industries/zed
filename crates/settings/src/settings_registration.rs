use gpui::App;
pub use inventory;

pub struct RegisterSettingFn(fn(&mut App));

inventory::collect!(RegisterSettingFn);

pub fn load_registered_settings(cx: &mut App) {
    for register_function in inventory::iter::<RegisterSettingFn> {
        (register_function.0)(cx);
    }
}
