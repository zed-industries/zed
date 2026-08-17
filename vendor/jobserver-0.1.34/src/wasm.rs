use crate::FromEnvErrorInner;
use std::io;
use std::process::Command;
use std::sync::{Arc, Condvar, Mutex};
use std::thread::{Builder, JoinHandle};

#[derive(Debug)]
pub struct Client {
    inner: Arc<Inner>,
}

#[derive(Debug)]
struct Inner {
    count: Mutex<usize>,
    cvar: Condvar,
}

#[derive(Debug)]
pub struct Acquired(());

impl Client {
    pub fn new(limit: usize) -> io::Result<Client> {
        Ok(Client {
            inner: Arc::new(Inner {
                count: Mutex::new(limit),
                cvar: Condvar::new(),
            }),
        })
    }

    pub(crate) unsafe fn open(_s: &str, _check_pipe: bool) -> Result<Client, FromEnvErrorInner> {
        Err(FromEnvErrorInner::Unsupported)
    }

    pub fn acquire(&self) -> io::Result<Acquired> {
        let mut lock = self.inner.count.lock().unwrap_or_else(|e| e.into_inner());
        while *lock == 0 {
            lock = self
                .inner
                .cvar
                .wait(lock)
                .unwrap_or_else(|e| e.into_inner());
        }
        *lock -= 1;
        Ok(Acquired(()))
    }

    pub fn try_acquire(&self) -> io::Result<Option<Acquired>> {
        let mut lock = self.inner.count.lock().unwrap_or_else(|e| e.into_inner());
        if *lock == 0 {
            Ok(None)
        } else {
            *lock -= 1;
            Ok(Some(Acquired(())))
        }
    }

    pub fn release(&self, _data: Option<&Acquired>) -> io::Result<()> {
        let mut lock = self.inner.count.lock().unwrap_or_else(|e| e.into_inner());
        *lock += 1;
        drop(lock);
        self.inner.cvar.notify_one();
        Ok(())
    }

    pub fn string_arg(&self) -> String {
        panic!(
            "On this platform there is no cross process jobserver support,
             so Client::configure is not supported."
        );
    }

    pub fn available(&self) -> io::Result<usize> {
        let lock = self.inner.count.lock().unwrap_or_else(|e| e.into_inner());
        Ok(*lock)
    }

    pub fn configure(&self, _cmd: &mut Command) {
        unreachable!();
    }
}

#[derive(Debug)]
pub struct Helper {
    thread: JoinHandle<()>,
}

pub(crate) fn spawn_helper(
    client: crate::Client,
    state: Arc<super::HelperState>,
    mut f: Box<dyn FnMut(io::Result<crate::Acquired>) + Send>,
) -> io::Result<Helper> {
    let thread = Builder::new().spawn(move || {
        state.for_each_request(|_| f(client.acquire()));
    })?;

    Ok(Helper { thread })
}

impl Helper {
    pub fn join(self) {
        // TODO: this is not correct if the thread is blocked in
        // `client.acquire()`.
        drop(self.thread.join());
    }
}
