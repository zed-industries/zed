use block2::RcBlock;
use futures::{StreamExt, channel::mpsc};
use gpui::{ForegroundExecutor, Task, ThermalState};
use objc2::{rc::Retained, runtime::ProtocolObject};
use objc2_foundation::{
    NSNotification, NSNotificationCenter, NSObjectProtocol, NSProcessInfo,
    NSProcessInfoThermalState, NSProcessInfoThermalStateDidChangeNotification,
};

pub fn thermal_state() -> ThermalState {
    map_thermal_state(NSProcessInfo::processInfo().thermalState())
}

fn map_thermal_state(state: NSProcessInfoThermalState) -> ThermalState {
    match state {
        NSProcessInfoThermalState::Fair => ThermalState::Fair,
        NSProcessInfoThermalState::Serious => ThermalState::Serious,
        NSProcessInfoThermalState::Critical => ThermalState::Critical,
        _ => ThermalState::Nominal,
    }
}

pub struct ThermalObserver {
    observer: Retained<ProtocolObject<dyn NSObjectProtocol>>,
    _task: Task<()>,
}

impl ThermalObserver {
    pub fn new(executor: &ForegroundExecutor, mut callback: Box<dyn FnMut()>) -> Self {
        let (sender, mut receiver) = mpsc::unbounded();
        let block = RcBlock::new(move |_: std::ptr::NonNull<NSNotification>| {
            // NotificationCenter can call synchronously or off-thread. Never enter
            // application code here: GPUI may already be borrowed on the main thread.
            if sender.unbounded_send(()).is_err() {
                log::debug!("thermal observer stopped before notification delivery");
            }
        });
        let observer = unsafe {
            NSNotificationCenter::defaultCenter().addObserverForName_object_queue_usingBlock(
                Some(NSProcessInfoThermalStateDidChangeNotification),
                Some(&NSProcessInfo::processInfo()),
                None,
                &block,
            )
        };
        let task = executor.spawn(async move {
            while receiver.next().await.is_some() {
                callback();
            }
        });
        Self {
            observer,
            _task: task,
        }
    }
}

impl Drop for ThermalObserver {
    fn drop(&mut self) {
        unsafe {
            NSNotificationCenter::defaultCenter().removeObserver(self.observer.as_ref());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{cell::Cell, rc::Rc};

    #[gpui::test]
    fn notifications_are_deferred_and_cancelled_on_drop(cx: &mut gpui::TestAppContext) {
        let calls = Rc::new(Cell::new(0));
        let observer = ThermalObserver::new(
            &cx.foreground_executor,
            Box::new({
                let calls = calls.clone();
                move || calls.set(calls.get() + 1)
            }),
        );
        let post_notification = || unsafe {
            NSNotificationCenter::defaultCenter().postNotificationName_object(
                NSProcessInfoThermalStateDidChangeNotification,
                Some(&NSProcessInfo::processInfo()),
            );
        };
        post_notification();
        assert_eq!(calls.get(), 0);
        cx.run_until_parked();
        assert_eq!(calls.get(), 1);

        post_notification();
        drop(observer);
        cx.run_until_parked();
        assert_eq!(calls.get(), 1);
        post_notification();
        cx.run_until_parked();
        assert_eq!(calls.get(), 1);
    }

    #[test]
    fn maps_native_thermal_states() {
        assert_eq!(
            map_thermal_state(NSProcessInfoThermalState::Nominal),
            ThermalState::Nominal
        );
        assert_eq!(
            map_thermal_state(NSProcessInfoThermalState::Fair),
            ThermalState::Fair
        );
        assert_eq!(
            map_thermal_state(NSProcessInfoThermalState::Serious),
            ThermalState::Serious
        );
        assert_eq!(
            map_thermal_state(NSProcessInfoThermalState::Critical),
            ThermalState::Critical
        );
        assert_eq!(
            map_thermal_state(NSProcessInfoThermalState(99)),
            ThermalState::Nominal
        );
    }
}
