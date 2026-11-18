use android_activity::AndroidApp;
use anyhow::Result;
use gpui::{Bounds, DisplayId, Pixels, PlatformDisplay, Point, Size, px};
use jni::objects::{JFloatArray, JObject};
use jni::{jni_sig, jni_str};
use std::cell::Cell;

#[derive(Debug)]
pub struct AndroidDisplay {
    id: DisplayId,
    uuid: uuid::Uuid,
    size: Cell<Size<Pixels>>,
    max_refresh_rate: Cell<Option<f32>>,
}

// Safety: the display is only accessed from the android_main thread; GPUI's
// PlatformDisplay trait requires Send + Sync but never shares it across threads.
unsafe impl Send for AndroidDisplay {}
unsafe impl Sync for AndroidDisplay {}

impl AndroidDisplay {
    pub fn new(app: &AndroidApp) -> Self {
        let display = AndroidDisplay {
            id: DisplayId::new(1),
            uuid: uuid::Uuid::new_v4(),
            size: Cell::new(Size {
                width: px(412.),
                height: px(915.),
            }),
            max_refresh_rate: Cell::new(None),
        };
        display.update_max_refresh_rate(app);
        display
    }

    /// The highest refresh rate, in hertz, the display supports at its current resolution.
    pub(crate) fn max_refresh_rate(&self) -> Option<f32> {
        self.max_refresh_rate.get()
    }

    /// Re-reads [`Self::max_refresh_rate`], which changes with the display's
    /// resolution and the user's smooth-display setting.
    pub(crate) fn update_max_refresh_rate(&self, app: &AndroidApp) {
        // SAFETY: android-activity guarantees `vm_as_ptr` returns the
        // process's valid `JavaVM` pointer for the lifetime of the app.
        let vm = unsafe { jni::JavaVM::from_raw(app.vm_as_ptr().cast()) };
        let activity_ptr = app.activity_as_ptr();
        let rates = vm.attach_current_thread(|env| {
            // SAFETY: `activity_as_ptr` is a global reference to the activity
            // that lives as long as the app. `JObject` does not delete the
            // reference on drop.
            let activity = unsafe { JObject::from_raw(env, activity_ptr as jni::sys::jobject) };
            let window_manager = env
                .call_method(
                    &activity,
                    jni_str!("getWindowManager"),
                    jni_sig!("()Landroid/view/WindowManager;"),
                    &[],
                )?
                .l()?;
            let display = env
                .call_method(
                    &window_manager,
                    jni_str!("getDefaultDisplay"),
                    jni_sig!("()Landroid/view/Display;"),
                    &[],
                )?
                .l()?;
            let rates = env
                .call_method(
                    &display,
                    jni_str!("getSupportedRefreshRates"),
                    jni_sig!("()[F"),
                    &[],
                )?
                .l()?;
            let rates = env.cast_local::<JFloatArray>(rates)?;
            let mut buffer = vec![0.0; rates.len(env)?];
            rates.get_region(env, 0, &mut buffer)?;
            anyhow::Ok(buffer)
        });
        match rates {
            Ok(rates) => self
                .max_refresh_rate
                .set(rates.into_iter().reduce(f32::max)),
            Err(error) => log::error!("failed to read display refresh rates: {error:#}"),
        }
    }

    pub(crate) fn set_size(&self, size: Size<Pixels>) {
        self.size.set(size);
    }
}

impl PlatformDisplay for AndroidDisplay {
    fn id(&self) -> DisplayId {
        self.id
    }

    fn refresh_interval(&self) -> Option<std::time::Duration> {
        self.max_refresh_rate()
            .and_then(|hertz| gpui::refresh_interval_from_hz(hertz as f64))
    }

    fn uuid(&self) -> Result<uuid::Uuid> {
        Ok(self.uuid)
    }

    fn bounds(&self) -> Bounds<Pixels> {
        Bounds {
            origin: Point::default(),
            size: self.size.get(),
        }
    }
}
