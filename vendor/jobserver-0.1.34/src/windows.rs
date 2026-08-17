use crate::FromEnvErrorInner;
use std::ffi::CString;
use std::io;
use std::process::Command;
use std::ptr;
use std::sync::Arc;
use std::thread::{Builder, JoinHandle};

#[derive(Debug)]
pub struct Client {
    sem: Handle,
    name: String,
}

#[derive(Debug)]
pub struct Acquired;

#[allow(clippy::upper_case_acronyms)]
type BOOL = i32;
#[allow(clippy::upper_case_acronyms)]
type DWORD = u32;
#[allow(clippy::upper_case_acronyms)]
type HANDLE = *mut u8;
#[allow(clippy::upper_case_acronyms)]
type LONG = i32;

const ERROR_ALREADY_EXISTS: DWORD = 183;
const FALSE: BOOL = 0;
const INFINITE: DWORD = 0xffffffff;
const SEMAPHORE_MODIFY_STATE: DWORD = 0x2;
const SYNCHRONIZE: DWORD = 0x00100000;
const TRUE: BOOL = 1;

const WAIT_ABANDONED: DWORD = 128u32;
const WAIT_FAILED: DWORD = 4294967295u32;
const WAIT_OBJECT_0: DWORD = 0u32;
const WAIT_TIMEOUT: DWORD = 258u32;

#[link(name = "kernel32")]
extern "system" {
    fn CloseHandle(handle: HANDLE) -> BOOL;
    fn SetEvent(hEvent: HANDLE) -> BOOL;
    fn WaitForMultipleObjects(
        ncount: DWORD,
        lpHandles: *const HANDLE,
        bWaitAll: BOOL,
        dwMilliseconds: DWORD,
    ) -> DWORD;
    fn CreateEventA(
        lpEventAttributes: *mut u8,
        bManualReset: BOOL,
        bInitialState: BOOL,
        lpName: *const i8,
    ) -> HANDLE;
    fn ReleaseSemaphore(
        hSemaphore: HANDLE,
        lReleaseCount: LONG,
        lpPreviousCount: *mut LONG,
    ) -> BOOL;
    fn CreateSemaphoreA(
        lpEventAttributes: *mut u8,
        lInitialCount: LONG,
        lMaximumCount: LONG,
        lpName: *const i8,
    ) -> HANDLE;
    fn OpenSemaphoreA(dwDesiredAccess: DWORD, bInheritHandle: BOOL, lpName: *const i8) -> HANDLE;
    fn WaitForSingleObject(hHandle: HANDLE, dwMilliseconds: DWORD) -> DWORD;
}

impl Client {
    pub fn new(limit: usize) -> io::Result<Client> {
        // Try a bunch of random semaphore names until we get a unique one,
        // but don't try for too long.
        //
        // Note that `limit == 0` is a valid argument above but Windows
        // won't let us create a semaphore with 0 slots available to it. Get
        // `limit == 0` working by creating a semaphore instead with one
        // slot and then immediately acquire it (without ever releaseing it
        // back).
        for _ in 0..100 {
            let bytes = getrandom::u32()?;
            let mut name = format!("__rust_jobserver_semaphore_{}\0", bytes);
            unsafe {
                let create_limit = if limit == 0 { 1 } else { limit };
                let r = CreateSemaphoreA(
                    ptr::null_mut(),
                    create_limit as LONG,
                    create_limit as LONG,
                    name.as_ptr() as *const _,
                );
                if r.is_null() {
                    return Err(io::Error::last_os_error());
                }
                let handle = Handle(r);

                let err = io::Error::last_os_error();
                if err.raw_os_error() == Some(ERROR_ALREADY_EXISTS as i32) {
                    continue;
                }
                name.pop(); // chop off the trailing nul
                let client = Client { sem: handle, name };
                if create_limit != limit {
                    client.acquire()?;
                }
                return Ok(client);
            }
        }

        Err(io::Error::new(
            io::ErrorKind::Other,
            "failed to find a unique name for a semaphore",
        ))
    }

    pub(crate) unsafe fn open(s: &str, _check_pipe: bool) -> Result<Client, FromEnvErrorInner> {
        let name = match CString::new(s) {
            Ok(s) => s,
            Err(e) => return Err(FromEnvErrorInner::CannotParse(e.to_string())),
        };

        let sem = OpenSemaphoreA(SYNCHRONIZE | SEMAPHORE_MODIFY_STATE, FALSE, name.as_ptr());
        if sem.is_null() {
            Err(FromEnvErrorInner::CannotOpenPath(
                s.to_string(),
                io::Error::last_os_error(),
            ))
        } else {
            Ok(Client {
                sem: Handle(sem),
                name: s.to_string(),
            })
        }
    }

    pub fn acquire(&self) -> io::Result<Acquired> {
        unsafe {
            let r = WaitForSingleObject(self.sem.0, INFINITE);
            if r == WAIT_OBJECT_0 {
                Ok(Acquired)
            } else {
                Err(io::Error::last_os_error())
            }
        }
    }

    pub fn try_acquire(&self) -> io::Result<Option<Acquired>> {
        match unsafe { WaitForSingleObject(self.sem.0, 0) } {
            WAIT_OBJECT_0 => Ok(Some(Acquired)),
            WAIT_TIMEOUT => Ok(None),
            WAIT_FAILED => Err(io::Error::last_os_error()),
            // We believe this should be impossible for a semaphore, but still
            // check the error code just in case it happens.
            WAIT_ABANDONED => Err(io::Error::new(
                io::ErrorKind::Other,
                "Wait on jobserver semaphore returned WAIT_ABANDONED",
            )),
            _ => unreachable!("Unexpected return value from WaitForSingleObject"),
        }
    }

    pub fn release(&self, _data: Option<&Acquired>) -> io::Result<()> {
        unsafe {
            let r = ReleaseSemaphore(self.sem.0, 1, ptr::null_mut());
            if r != 0 {
                Ok(())
            } else {
                Err(io::Error::last_os_error())
            }
        }
    }

    pub fn string_arg(&self) -> String {
        self.name.clone()
    }

    pub fn available(&self) -> io::Result<usize> {
        // Can't read value of a semaphore on Windows, so
        // try to acquire without sleeping, since we can find out the
        // old value on release. If acquisiton fails, then available is 0.
        unsafe {
            let r = WaitForSingleObject(self.sem.0, 0);
            if r != WAIT_OBJECT_0 {
                Ok(0)
            } else {
                let mut prev: LONG = 0;
                let r = ReleaseSemaphore(self.sem.0, 1, &mut prev);
                if r != 0 {
                    Ok(prev as usize + 1)
                } else {
                    Err(io::Error::last_os_error())
                }
            }
        }
    }

    pub fn configure(&self, _cmd: &mut Command) {
        // nothing to do here, we gave the name of our semaphore to the
        // child above
    }
}

#[derive(Debug)]
struct Handle(HANDLE);
// HANDLE is a raw ptr, but we're send/sync
unsafe impl Sync for Handle {}
unsafe impl Send for Handle {}

impl Drop for Handle {
    fn drop(&mut self) {
        unsafe {
            CloseHandle(self.0);
        }
    }
}

#[derive(Debug)]
pub struct Helper {
    event: Arc<Handle>,
    thread: JoinHandle<()>,
}

pub(crate) fn spawn_helper(
    client: crate::Client,
    state: Arc<super::HelperState>,
    mut f: Box<dyn FnMut(io::Result<crate::Acquired>) + Send>,
) -> io::Result<Helper> {
    let event = unsafe {
        let r = CreateEventA(ptr::null_mut(), TRUE, FALSE, ptr::null());
        if r.is_null() {
            return Err(io::Error::last_os_error());
        } else {
            Handle(r)
        }
    };
    let event = Arc::new(event);
    let event2 = event.clone();
    let thread = Builder::new().spawn(move || {
        let objects = [event2.0, client.inner.sem.0];
        state.for_each_request(|_| {
            const WAIT_OBJECT_1: u32 = WAIT_OBJECT_0 + 1;
            match unsafe { WaitForMultipleObjects(2, objects.as_ptr(), FALSE, INFINITE) } {
                WAIT_OBJECT_0 => {}
                WAIT_OBJECT_1 => f(Ok(crate::Acquired {
                    client: client.inner.clone(),
                    data: Acquired,
                    disabled: false,
                })),
                _ => f(Err(io::Error::last_os_error())),
            }
        });
    })?;
    Ok(Helper { thread, event })
}

impl Helper {
    pub fn join(self) {
        // Unlike unix this logic is much easier. If our thread was blocked
        // in waiting for requests it should already be woken up and
        // exiting. Otherwise it's waiting for a token, so we wake it up
        // with a different event that it's also waiting on here. After
        // these two we should be guaranteed the thread is on its way out,
        // so we can safely `join`.
        let r = unsafe { SetEvent(self.event.0) };
        if r == 0 {
            panic!("failed to set event: {}", io::Error::last_os_error());
        }
        drop(self.thread.join());
    }
}
