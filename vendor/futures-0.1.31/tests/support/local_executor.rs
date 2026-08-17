//! Execution of futures on a single thread
//!
//! This module has no special handling of any blocking operations other than
//! futures-aware inter-thread communications, and is not intended to be used to
//! manage I/O. For futures that do I/O you'll likely want to use `tokio-core`.

#![allow(bare_trait_objects, unknown_lints)]

use std::cell::{Cell, RefCell};
use std::sync::{Arc, Mutex, mpsc};

use futures::executor::{self, Spawn, Notify};
use futures::future::{Executor, ExecuteError};
use futures::{Future, Async};

/// Main loop object
pub struct Core {
    tx: mpsc::Sender<usize>,
    rx: mpsc::Receiver<usize>,
    notify: Arc<MyNotify>,

    // Slab of running futures used to track what's running and what slots are
    // empty. Slot indexes are then sent along tx/rx above to indicate which
    // future is ready to get polled.
    tasks: RefCell<Vec<Slot>>,
    next_vacant: Cell<usize>,
}

enum Slot {
    Vacant { next_vacant: usize },
    Running(Option<Spawn<Box<Future<Item = (), Error = ()>>>>),
}

impl Core {
    /// Create a new `Core`.
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel();
        Core {
            notify: Arc::new(MyNotify {
                tx: Mutex::new(tx.clone()),
            }),
            tx: tx,
            rx: rx,
            next_vacant: Cell::new(0),
            tasks: RefCell::new(Vec::new()),
        }
    }

    /// Spawn a future to be executed by a future call to `run`.
    ///
    /// The future `f` provided will not be executed until `run` is called
    /// below. While futures passed to `run` are executing, the future provided
    /// here will be executed concurrently as well.
    pub fn spawn<F>(&self, f: F)
        where F: Future<Item=(), Error=()> + 'static
    {
        let idx = self.next_vacant.get();
        let mut tasks = self.tasks.borrow_mut();
        match tasks.get_mut(idx) {
            Some(&mut Slot::Vacant { next_vacant }) => {
                self.next_vacant.set(next_vacant);
            }
            Some(&mut Slot::Running (_)) => {
                panic!("vacant points to running future")
            }
            None => {
                assert_eq!(idx, tasks.len());
                tasks.push(Slot::Vacant { next_vacant: 0 });
                self.next_vacant.set(idx + 1);
            }
        }
        tasks[idx] = Slot::Running(Some(executor::spawn(Box::new(f))));
        self.tx.send(idx).unwrap();
    }

    /// Run the loop until the future `f` completes.
    ///
    /// This method will block the current thread until the future `f` has
    /// resolved. While waiting on `f` to finish it will also execute any
    /// futures spawned via `spawn` above.
    pub fn run<F>(&self, f: F) -> Result<F::Item, F::Error>
        where F: Future,
    {
        let id = usize::max_value();
        self.tx.send(id).unwrap();
        let mut f = executor::spawn(f);
        loop {
            if self.turn() {
                match f.poll_future_notify(&self.notify, id)? {
                    Async::Ready(e) => return Ok(e),
                    Async::NotReady => {}
                }
            }
        }
    }

    /// "Turns" this event loop one tick.
    ///
    /// This'll block the current thread until something happens, and once an
    /// event happens this will act on that event.
    ///
    /// # Return value
    ///
    /// Returns `true` if the future passed to `run` should be polled or `false`
    /// otherwise.
    fn turn(&self) -> bool {
        let task_id = self.rx.recv().unwrap();
        if task_id == usize::max_value() {
            return true
        }

        // This may be a spurious wakeup so we're not guaranteed to have a
        // future associated with `task_id`, so do a fallible lookup.
        //
        // Note that we don't want to borrow `self.tasks` for too long so we
        // try to extract the future here and leave behind a tombstone future
        // which'll get replaced or removed later. This is how we support
        // spawn-in-run.
        let mut future = match self.tasks.borrow_mut().get_mut(task_id) {
            Some(&mut Slot::Running(ref mut future)) => future.take().unwrap(),
            Some(&mut Slot::Vacant { .. }) => return false,
            None => return false,
        };

        // Drive this future forward. If it's done we remove it and if it's not
        // done then we put it back in the tasks array.
        let done = match future.poll_future_notify(&self.notify, task_id) {
            Ok(Async::Ready(())) | Err(()) => true,
            Ok(Async::NotReady) => false,
        };
        let mut tasks = self.tasks.borrow_mut();
        if done {
            tasks[task_id] = Slot::Vacant { next_vacant: self.next_vacant.get() };
            self.next_vacant.set(task_id);
        } else {
            tasks[task_id] = Slot::Running(Some(future));
        }

        return false
    }
}

impl<F> Executor<F> for Core
    where F: Future<Item = (), Error = ()> + 'static,
{
    fn execute(&self, future: F) -> Result<(), ExecuteError<F>> {
        self.spawn(future);
        Ok(())
    }
}

struct MyNotify {
    // TODO: it's pretty unfortunate to use a `Mutex` here where the `Sender`
    //       itself is basically `Sync` as-is. Ideally this'd use something like
    //       an off-the-shelf mpsc queue as well as `thread::park` and
    //       `Thread::unpark`.
    tx: Mutex<mpsc::Sender<usize>>,
}

impl Notify for MyNotify {
    fn notify(&self, id: usize) {
        drop(self.tx.lock().unwrap().send(id));
    }
}
