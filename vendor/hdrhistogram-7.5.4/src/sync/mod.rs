//! Synchronized types that allow access to a `Histogram` from multiple threads.

use crate::errors::*;
use crate::{Counter, Histogram};
use std::borrow::Borrow;
use std::borrow::BorrowMut;
use std::marker::PhantomData;
use std::ops::{AddAssign, Deref, DerefMut};
use std::sync::{atomic, Arc, Mutex};
use std::time;

/// A write-only handle to a [`SyncHistogram`].
///
/// This handle allows you to record samples from multiple threads, each with its own `Recorder`,
/// concurrently. Writes to a `Recorder` are wait-free and scalable except for when the
/// [`SyncHistogram`] initiates a _phase shift_. During a phase shift, the next write on each
/// associated `Recorder` merges its results into a shared [`Histogram`] that is then made
/// available to the [`SyncHistogram`] once the phase shift completes. Phase shifts should also be
/// relatively cheap for writers, as they mainly need to perform a channel send on an unbounded,
/// lock-free channel.
///
/// An idle `Recorder` will hold up a phase shift indefinitely, or until it times out (is using
/// [`SyncHistogram::refresh_timeout`]. If a `Recorder` will remain idle for extended periods of
/// time, it should call [`Recorder::idle`], which will tell the reader not to wait for this
/// particular writer.
///
/// When a `Recorder` is dropped, all samples are made visible to the next
/// [`SyncHistogram::refresh`].
#[derive(Debug)]
pub struct Recorder<C: Counter> {
    local: Histogram<C>,
    shared: Arc<Shared<C>>,
    last_phase: usize,
}

// make it more ergonomic to record samples
impl<C: Counter> AddAssign<u64> for Recorder<C> {
    fn add_assign(&mut self, value: u64) {
        self.record(value).unwrap();
    }
}

impl<C: Counter> Clone for Recorder<C> {
    fn clone(&self) -> Self {
        // reader will have to wait for one more recorder
        {
            let mut truth = self.shared.truth.lock().unwrap();
            truth.recorders += 1;
        }

        // new recorder starts at the same phase as we do with an empty histogram
        Recorder {
            local: Histogram::new_from(&self.local),
            shared: self.shared.clone(),
            last_phase: self.last_phase,
        }
    }
}

impl<C: Counter> Drop for Recorder<C> {
    fn drop(&mut self) {
        // we'll need to decrement the # of recorders
        let mut truth = self.shared.truth.lock().unwrap();
        truth.recorders -= 1;

        // we also want to communicate the remainder of our samples to the reader
        // we do this under the lock so that if the reader reads .recorders after we update it
        // above, it is guaranteed to see the samples from this recorder.
        // we also _have_ to do it at some point during drop as the reader _may_ have read
        // .recorders _before_ we decremented it above, in which case it's blocking on us!
        // note that we cannot call self.update() here as it would drop the mutex guard
        let h = Histogram::new_from(&self.local);
        let h = std::mem::replace(&mut self.local, h);
        let _ = self.shared.sender.send(h).is_ok(); // if this is err, the reader went away

        // explicitly drop guard to ensure we don't accidentally drop it above
        drop(truth);
    }
}

#[derive(Debug)]
struct Critical {
    recorders: usize,
}

#[derive(Debug)]
struct Shared<C: Counter> {
    truth: Mutex<Critical>,
    sender: crossbeam_channel::Sender<Histogram<C>>,
    phase: atomic::AtomicUsize,
}

/// See [`IdleRecorder`]. This guard borrows the idle [`Recorder`].
pub type IdleRecorderGuard<'a, C> = IdleRecorder<&'a mut Recorder<C>, C>;

/// This guard denotes that a [`Recorder`] is currently idle, and should not be waited on by a
/// [`SyncHistogram`] phase-shift.
#[derive(Debug)]
pub struct IdleRecorder<T, C: Counter>
where
    T: BorrowMut<Recorder<C>>,
{
    recorder: Option<T>,
    c: PhantomData<C>,
}

impl<T, C: Counter> IdleRecorder<T, C>
where
    T: BorrowMut<Recorder<C>>,
{
    fn reactivate(&mut self) {
        let recorder = if let Some(ref mut r) = self.recorder {
            r
        } else {
            // already reactivated
            return;
        };

        let recorder = recorder.borrow_mut();

        // the Recorder is no longer idle, so the reader has to wait for us again
        // this basically means re-incrementing .recorders
        let mut crit = recorder.shared.truth.lock().unwrap();
        crit.recorders += 1;

        // we need to figure out what phase we're joining
        // the easiest way to do that is to adopt the current phase
        //
        // note that we have to load the phase while holding the lock.
        // if we did not, the reader could come along, read our ++'d .recorders (and so wait for us
        // to send), and bump the phase, all before we read it, which would lead us to believe that
        // we were already synchronized when in reality we were not, which would stall the reader
        // even if we issued more writes.
        recorder.last_phase = recorder.shared.phase.load(atomic::Ordering::Acquire);

        // explicitly drop guard to ensure we don't accidentally drop it above
        drop(crit);
    }
}

impl<C: Counter> IdleRecorder<Recorder<C>, C> {
    /// Mark the wrapped [`Recorder`] as active again and return it.
    pub fn activate(mut self) -> Recorder<C> {
        self.reactivate();
        self.recorder.take().unwrap()
    }

    /// Clone the wrapped [`Recorder`].
    pub fn recorder(&self) -> Recorder<C> {
        self.recorder.as_ref().unwrap().clone()
    }
}

impl<T, C: Counter> Drop for IdleRecorder<T, C>
where
    T: BorrowMut<Recorder<C>>,
{
    fn drop(&mut self) {
        self.reactivate()
    }
}

impl<C: Counter> Recorder<C> {
    fn with_hist<F, R>(&mut self, f: F) -> R
    where
        F: FnOnce(&mut Histogram<C>) -> R,
    {
        let r = f(&mut self.local);
        let phase = self.shared.phase.load(atomic::Ordering::Acquire);
        if phase != self.last_phase {
            self.update();
            self.last_phase = phase;
        }
        r
    }

    // return our current histogram and leave a cleared one in its place
    fn shed(&mut self) -> Histogram<C> {
        let h = Histogram::new_from(&self.local);
        std::mem::replace(&mut self.local, h)
    }

    fn update(&mut self) {
        let h = self.shed();
        let _ = self.shared.sender.send(h).is_ok(); // if this is err, the reader went away
    }

    fn deactivate(&mut self) {
        let phase;
        {
            // we're leaving rotation, so we need to decrement .recorders
            let mut crit = self.shared.truth.lock().unwrap();
            crit.recorders -= 1;

            // make sure we don't hold up the current phase shift (if any)
            phase = self.shared.phase.load(atomic::Ordering::Acquire);
            if phase != self.last_phase {
                // can't call self.update() due to borrow of self.shared above
                let h = Histogram::new_from(&self.local);
                let h = std::mem::replace(&mut self.local, h);
                let _ = self.shared.sender.send(h).is_ok(); // if this is err, the reader went away
            }
        }
        self.last_phase = phase;
    }

    /// Call this method if the Recorder will be idle for a while.
    ///
    /// Until the returned guard is dropped, the associated [`SyncHistogram`] will not wait for
    /// this recorder on a phase shift.
    pub fn idle(&mut self) -> IdleRecorderGuard<C> {
        self.deactivate();
        IdleRecorder {
            recorder: Some(self),
            c: PhantomData,
        }
    }

    /// Mark this `Recorder` as inactive.
    ///
    /// Until the returned guard is consumed, either by calling [`IdleRecorder::activate`] or by
    /// dropping it, the associated [`SyncHistogram`] will not wait for this recorder on a phase
    /// shift.
    pub fn into_idle(mut self) -> IdleRecorder<Self, C> {
        self.deactivate();
        IdleRecorder {
            recorder: Some(self),
            c: PhantomData,
        }
    }

    /// See [`Histogram::add`].
    pub fn add<B: Borrow<Histogram<C>>>(&mut self, source: B) -> Result<(), AdditionError> {
        self.with_hist(move |h| h.add(source))
    }

    /// See [`Histogram::add_correct`].
    pub fn add_correct<B: Borrow<Histogram<C>>>(
        &mut self,
        source: B,
        interval: u64,
    ) -> Result<(), RecordError> {
        self.with_hist(move |h| h.add_correct(source, interval))
    }

    /// See [`Histogram::subtract`].
    pub fn subtract<B: Borrow<Histogram<C>>>(
        &mut self,
        subtrahend: B,
    ) -> Result<(), SubtractionError> {
        self.with_hist(move |h| h.subtract(subtrahend))
    }

    /// See [`Histogram::record`].
    pub fn record(&mut self, value: u64) -> Result<(), RecordError> {
        self.with_hist(move |h| h.record(value))
    }

    /// See [`Histogram::saturating_record`].
    pub fn saturating_record(&mut self, value: u64) {
        self.with_hist(move |h| h.saturating_record(value))
    }

    /// See [`Histogram::record_n`].
    pub fn record_n(&mut self, value: u64, count: C) -> Result<(), RecordError> {
        self.with_hist(move |h| h.record_n(value, count))
    }

    /// See [`Histogram::saturating_record_n`].
    pub fn saturating_record_n(&mut self, value: u64, count: C) {
        self.with_hist(move |h| h.saturating_record_n(value, count))
    }

    /// See [`Histogram::record_correct`].
    pub fn record_correct(&mut self, value: u64, interval: u64) -> Result<(), RecordError> {
        self.with_hist(move |h| h.record_correct(value, interval))
    }

    /// See [`Histogram::record_n_correct`].
    pub fn record_n_correct(
        &mut self,
        value: u64,
        count: C,
        interval: u64,
    ) -> Result<(), RecordError> {
        self.with_hist(move |h| h.record_n_correct(value, count, interval))
    }
}

/// A `Histogram` that can be written to by multiple threads concurrently.
///
/// Each writer thread should have a [`Recorder`], which allows it to record new samples without
/// synchronization. New recorded samples are made available through this histogram by calling
/// [`SyncHistogram::refresh`], which blocks until it has synchronized with every recorder.
#[derive(Debug)]
pub struct SyncHistogram<C: Counter> {
    merged: Histogram<C>,
    shared: Arc<Shared<C>>,
    receiver: crossbeam_channel::Receiver<Histogram<C>>,
}

impl<C: Counter> SyncHistogram<C> {
    fn refresh_inner(&mut self, timeout: Option<time::Duration>) {
        let end = timeout.map(|dur| time::Instant::now() + dur);

        // time to start a phase change
        // we first want to drain any histograms left over by dropped recorders
        // note that we do this _before_ incrementing the phase, so we know they're "old"
        while let Ok(h) = self.receiver.try_recv() {
            self.merged
                .add(&h)
                .expect("TODO: failed to merge histogram");
        }

        // make sure no recorders can join or leave in the middle of this
        let recorders = self.shared.truth.lock().unwrap().recorders;

        // then, we tell writers to phase
        let _ = self.shared.phase.fetch_add(1, atomic::Ordering::AcqRel);

        // we want to wait for writers to all have phased
        let mut phased = 0;

        // at this point, we expect to get at least truth.recorders histograms
        while phased < recorders {
            let h = if let Some(end) = end {
                let now = time::Instant::now();
                if now > end {
                    break;
                }

                match self.receiver.recv_timeout(end - now) {
                    Ok(h) => h,
                    Err(crossbeam_channel::RecvTimeoutError::Timeout) => break,
                    Err(crossbeam_channel::RecvTimeoutError::Disconnected) => unreachable!(),
                }
            } else {
                self.receiver
                    .recv()
                    .expect("SyncHistogram has an Arc<Shared> with a Receiver")
            };

            self.merged
                .add(&h)
                .expect("TODO: failed to merge histogram");
            phased += 1;
        }

        // we also gobble up extra histograms we may have been sent from more dropped writers
        while let Ok(h) = self.receiver.try_recv() {
            self.merged
                .add(&h)
                .expect("TODO: failed to merge histogram");
        }
    }

    /// Block until writes from all [`Recorder`] instances for this histogram have been
    /// incorporated.
    pub fn refresh(&mut self) {
        self.refresh_inner(None)
    }

    /// Block until writes from all [`Recorder`] instances for this histogram have been
    /// incorporated, or until the given amount of time has passed.
    pub fn refresh_timeout(&mut self, timeout: time::Duration) {
        self.refresh_inner(Some(timeout))
    }

    /// Obtain another multi-threaded writer for this histogram.
    ///
    /// Note that writes made to the `Recorder` will not be visible until the next call to
    /// [`SyncHistogram::refresh`].
    pub fn recorder(&self) -> Recorder<C> {
        // we will have to wait for one more recorder
        {
            let mut truth = self.shared.truth.lock().unwrap();
            truth.recorders += 1;
        }

        // new recorder starts at the current phase with an empty histogram
        Recorder {
            local: Histogram::new_from(&self.merged),
            shared: self.shared.clone(),
            last_phase: self.shared.phase.load(atomic::Ordering::Acquire),
        }
    }
}

impl<C: Counter> From<Histogram<C>> for SyncHistogram<C> {
    fn from(h: Histogram<C>) -> Self {
        let (tx, rx) = crossbeam_channel::unbounded();
        SyncHistogram {
            merged: h,
            receiver: rx,
            shared: Arc::new(Shared {
                truth: Mutex::new(Critical { recorders: 0 }),
                sender: tx,
                phase: atomic::AtomicUsize::new(0),
            }),
        }
    }
}

impl<C: Counter> Deref for SyncHistogram<C> {
    type Target = Histogram<C>;
    fn deref(&self) -> &Self::Target {
        &self.merged
    }
}

impl<C: Counter> DerefMut for SyncHistogram<C> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.merged
    }
}
