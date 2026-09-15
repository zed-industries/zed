use std::{
    cell::{Cell, RefCell},
    rc::Rc,
    sync::{
        Arc, Weak,
        atomic::{AtomicBool, Ordering},
    },
};

use anyhow::{Context as _, Result, anyhow};
use futures::{
    channel::oneshot,
    future::{Either, select},
    lock::Mutex,
};
use gpui::ForegroundExecutor;
use gpui_util::ResultExt;
use windows::{
    Win32::{
        Foundation::*,
        System::{Com::*, Threading::GetCurrentThreadId},
        UI::{
            Controls::{TASKDIALOG_NOTIFICATIONS, TDN_BUTTON_CLICKED},
            WindowsAndMessaging::*,
        },
    },
    core::{BOOL, HRESULT, w},
};

use crate::SafeHwnd;

pub(crate) struct DialogOwner {
    hwnd: HWND,
    closed: Cell<bool>,
    pending: Cell<usize>,
    serialization: Mutex<()>,
    cancellations: RefCell<Vec<Weak<AtomicBool>>>,
    idle_waiters: RefCell<Vec<oneshot::Sender<()>>>,
}

impl DialogOwner {
    pub(crate) fn new(hwnd: HWND) -> Rc<Self> {
        Rc::new(Self {
            hwnd,
            closed: Cell::new(false),
            pending: Cell::new(0),
            serialization: Mutex::new(()),
            cancellations: RefCell::new(Vec::new()),
            idle_waiters: RefCell::new(Vec::new()),
        })
    }

    fn lease(self: &Rc<Self>, cancellation: &Arc<AtomicBool>) -> Result<DialogLease> {
        anyhow::ensure!(!self.closed.get(), "dialog owner has closed");
        self.pending.set(self.pending.get() + 1);
        let mut cancellations = self.cancellations.borrow_mut();
        cancellations.retain(|cancellation| cancellation.strong_count() != 0);
        cancellations.push(Arc::downgrade(cancellation));
        Ok(DialogLease(self.clone()))
    }

    pub(crate) fn close(&self) {
        self.closed.set(true);
        for cancellation in self.cancellations.borrow_mut().drain(..) {
            if let Some(cancellation) = cancellation.upgrade() {
                cancellation.store(true, Ordering::Release);
            }
        }
    }

    pub(crate) fn when_idle(&self) -> oneshot::Receiver<()> {
        let (sender, receiver) = oneshot::channel();
        if self.pending.get() == 0 {
            sender.send(()).ok();
        } else {
            self.idle_waiters.borrow_mut().push(sender);
        }
        receiver
    }
}

struct DialogLease(Rc<DialogOwner>);

impl Drop for DialogLease {
    fn drop(&mut self) {
        let owner = &self.0;
        owner.pending.set(owner.pending.get() - 1);
        if owner.pending.get() == 0 {
            for waiter in owner.idle_waiters.borrow_mut().drain(..) {
                waiter.send(()).ok();
            }
        }
    }
}

/// Native modal loops may synchronously message their owner, so the foreground
/// thread must keep pumping messages rather than waiting for the dialog thread.
/// The lifetime lease also delays destruction of the owner HWND until the native
/// loop exits; an Rc alone would not prevent WindowsWindow::drop destroying it.
pub(crate) fn show_dialog<T: Send + 'static>(
    owner: Option<Rc<DialogOwner>>,
    executor: &ForegroundExecutor,
    dialog: impl FnOnce(HWND) -> Result<T> + Send + 'static,
) -> oneshot::Receiver<Result<T>> {
    let (mut sender, receiver) = oneshot::channel();
    let cancellation = Arc::new(AtomicBool::new(false));
    let hwnd = owner.as_ref().map(|owner| SafeHwnd::from(owner.hwnd));
    let lease = if let Some(owner) = owner {
        match owner.lease(&cancellation) {
            Ok(lease) => Some(lease),
            Err(error) => {
                sender.send(Err(error)).ok();
                return receiver;
            }
        }
    } else {
        None
    };
    executor
        .spawn(async move {
            // Let Windows manage enabling and activating the owner. Overlapping
            // modal loops could otherwise re-enable it while another dialog is open.
            let serialization = if let Some(lease) = lease.as_ref() {
                match select(lease.0.serialization.lock(), sender.cancellation()).await {
                    Either::Left((guard, _)) => Some(guard),
                    Either::Right(((), _)) => return,
                }
            } else {
                None
            };
            if sender.is_canceled()
                || cancellation.load(Ordering::Acquire)
                || lease.as_ref().is_some_and(|lease| lease.0.closed.get())
            {
                return;
            }
            let (result_sender, mut result_receiver) = oneshot::channel();
            let thread = std::thread::Builder::new()
                .name("windows-native-dialog".into())
                .spawn({
                    let cancellation = cancellation.clone();
                    move || {
                        let result = (|| {
                            let _apartment = Apartment::new()?;
                            let _timer = CancellationTimer::new(cancellation.clone())?;
                            if cancellation.load(Ordering::Acquire) {
                                return Err(anyhow!("dialog cancelled"));
                            }
                            let result = dialog(hwnd.map(|hwnd| hwnd.as_raw()).unwrap_or_default());
                            if cancellation.load(Ordering::Acquire) {
                                return Err(anyhow!("dialog cancelled"));
                            }
                            result
                        })();
                        result_sender.send(result).ok();
                    }
                });
            if let Err(error) = thread {
                drop(serialization);
                drop(lease);
                sender.send(Err(error.into())).ok();
                return;
            }
            let result = match select(&mut result_receiver, sender.cancellation()).await {
                Either::Left((result, _)) => result,
                Either::Right(((), _)) => {
                    cancellation.store(true, Ordering::Release);
                    result_receiver.await
                }
            };
            // Neither launch another dialog nor destroy the owner until the
            // native loop exits, even if the caller no longer wants its result.
            drop(serialization);
            drop(lease);
            sender
                .send(result.unwrap_or_else(|error| Err(error.into())))
                .ok();
        })
        .detach();
    receiver
}

struct Apartment;

impl Apartment {
    fn new() -> Result<Self> {
        unsafe { CoInitializeEx(None, COINIT_APARTMENTTHREADED).ok()? };
        Ok(Self)
    }
}

impl Drop for Apartment {
    fn drop(&mut self) {
        unsafe { CoUninitialize() };
    }
}

thread_local! {
    static CANCELLATION: RefCell<Option<Arc<AtomicBool>>> = const { RefCell::new(None) };
}

struct CancellationTimer(usize);

impl CancellationTimer {
    fn new(cancellation: Arc<AtomicBool>) -> Result<Self> {
        let timer = unsafe { SetTimer(None, 0, 100, Some(cancel_dialog)) };
        if timer == 0 {
            return Err(windows::core::Error::from_thread())
                .context("starting native dialog cancellation timer");
        }
        CANCELLATION.with(|slot| slot.replace(Some(cancellation)));
        Ok(Self(timer))
    }
}

impl Drop for CancellationTimer {
    fn drop(&mut self) {
        CANCELLATION.with(|slot| slot.take());
        unsafe { KillTimer(None, self.0).log_err() };
    }
}

unsafe extern "system" fn cancel_dialog(_: HWND, _: u32, _: usize, _: u32) {
    if cancellation_requested() {
        unsafe {
            EnumThreadWindows(GetCurrentThreadId(), Some(close_dialog), LPARAM(0))
                .ok()
                .log_err();
        };
    }
}

fn cancellation_requested() -> bool {
    CANCELLATION.with(|slot| {
        slot.borrow()
            .as_ref()
            .is_some_and(|cancellation| cancellation.load(Ordering::Acquire))
    })
}

pub(crate) unsafe extern "system" fn task_dialog_callback(
    _: HWND,
    notification: TASKDIALOG_NOTIFICATIONS,
    button: WPARAM,
    _: LPARAM,
    has_cancel_button: isize,
) -> HRESULT {
    // WM_CLOSE requires TDF_ALLOW_DIALOG_CANCELLATION, which also enables Esc/X.
    // Only allow those user gestures when the prompt offered a Cancel button.
    if notification == TDN_BUTTON_CLICKED
        && button.0 == IDCANCEL.0 as usize
        && has_cancel_button == 0
        && !cancellation_requested()
    {
        S_FALSE
    } else {
        S_OK
    }
}

unsafe extern "system" fn close_dialog(hwnd: HWND, _: LPARAM) -> BOOL {
    // Shell dialogs can open further modal dialogs. Close only dialog windows
    // on this dedicated thread, not hidden COM infrastructure windows.
    let mut class_name = [0u16; 32];
    let length = unsafe { GetClassNameW(hwnd, &mut class_name) } as usize;
    let dialog_class = w!("#32770");
    if class_name.get(..length) == Some(unsafe { dialog_class.as_wide() }) {
        unsafe { PostMessageW(Some(hwnd), WM_CLOSE, WPARAM(0), LPARAM(0)).log_err() };
    }
    TRUE
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task_dialog_cancellation_requires_cancel_button_or_internal_request() {
        let cancellation = Arc::new(AtomicBool::new(false));
        CANCELLATION.with(|slot| slot.replace(Some(cancellation.clone())));
        for internally_cancelled in [false, true] {
            cancellation.store(internally_cancelled, Ordering::Release);
            for has_cancel_button in [false, true] {
                for button in [IDCANCEL.0, IDOK.0, -1] {
                    let result = unsafe {
                        task_dialog_callback(
                            HWND::default(),
                            TDN_BUTTON_CLICKED,
                            WPARAM(button as usize),
                            LPARAM(0),
                            has_cancel_button as isize,
                        )
                    };
                    let should_veto =
                        button == IDCANCEL.0 && !has_cancel_button && !internally_cancelled;
                    assert_eq!(result, if should_veto { S_FALSE } else { S_OK });
                }
            }
        }
        CANCELLATION.with(|slot| slot.take());
    }

    #[test]
    fn closing_owner_cancels_every_outstanding_dialog() {
        let owner = DialogOwner::new(HWND::default());
        let first = Arc::new(AtomicBool::new(false));
        let second = Arc::new(AtomicBool::new(false));
        let _first_lease = owner.lease(&first).expect("owner is open");
        let _second_lease = owner.lease(&second).expect("owner is open");

        owner.close();
        owner.close();

        assert!(owner.closed.get());
        assert!(first.load(Ordering::Acquire));
        assert!(second.load(Ordering::Acquire));
        assert!(owner.cancellations.borrow().is_empty());
        assert!(owner.lease(&Arc::new(AtomicBool::new(false))).is_err());
    }

    #[test]
    fn dialog_leases_outlive_closed_owner_without_touching_its_handle() {
        let owner = DialogOwner::new(HWND::default());
        let first = owner
            .lease(&Arc::new(AtomicBool::new(false)))
            .expect("owner is open");
        let second = owner
            .lease(&Arc::new(AtomicBool::new(false)))
            .expect("owner is open");
        let mut idle = owner.when_idle();

        owner.close();
        assert_eq!(idle.try_recv().expect("idle receiver is connected"), None);
        drop(first);
        assert_eq!(owner.pending.get(), 1);
        assert_eq!(idle.try_recv().expect("idle receiver is connected"), None);
        drop(second);
        assert_eq!(owner.pending.get(), 0);
        assert_eq!(
            idle.try_recv().expect("idle receiver is connected"),
            Some(())
        );
    }

    #[test]
    fn dropping_last_lease_notifies_idle() {
        let owner = DialogOwner::new(HWND::default());
        let lease = owner
            .lease(&Arc::new(AtomicBool::new(false)))
            .expect("owner is open");
        let mut idle = owner.when_idle();
        drop(lease);
        assert_eq!(owner.pending.get(), 0);
        assert_eq!(
            idle.try_recv().expect("idle receiver is connected"),
            Some(())
        );
    }

    #[test]
    fn dialogs_serialize_per_owner_while_queued_leases_keep_owner_alive() {
        let owner = DialogOwner::new(HWND::default());
        let other_owner = DialogOwner::new(HWND::default());
        let active = owner
            .lease(&Arc::new(AtomicBool::new(false)))
            .expect("owner is open");
        let queued = owner
            .lease(&Arc::new(AtomicBool::new(false)))
            .expect("owner is open");
        let guard = active.0.serialization.try_lock().expect("first dialog");
        let mut idle = owner.when_idle();

        assert!(queued.0.serialization.try_lock().is_none());
        assert!(other_owner.serialization.try_lock().is_some());
        drop(guard);
        drop(active);
        assert_eq!(idle.try_recv().expect("idle receiver is connected"), None);
        let guard = queued.0.serialization.try_lock().expect("next dialog");
        drop(guard);
        drop(queued);
        assert_eq!(
            idle.try_recv().expect("idle receiver is connected"),
            Some(())
        );
    }
}
