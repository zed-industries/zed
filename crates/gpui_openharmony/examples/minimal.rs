fn main() {
    #[cfg(target_env = "ohos")]
    {
        use gpui::Platform;
        use gpui_openharmony::OpenHarmonyPlatform;
        use std::rc::Rc;

        let platform: Rc<OpenHarmonyPlatform> = OpenHarmonyPlatform::new();
        let displays = platform.displays();
        println!("OpenHarmony platform created; displays: {}", displays.len());
    }
    #[cfg(not(target_env = "ohos"))]
    {
        println!("This example only runs on OpenHarmony");
    }
}
