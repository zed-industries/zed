use core::future::Future;
use core::mem;
use core::pin::Pin;
use core::task::{ready, Context, Poll};

/// A future that may have completed.
#[derive(Debug)]
pub(crate) enum MaybeDone<Fut: Future> {
    /// A not-yet-completed future
    Future(Fut),

    /// The output of the completed future
    Done(Fut::Output),

    /// The empty variant after the result of a [`MaybeDone`] has been
    /// taken using the [`take`](MaybeDone::take) method.
    Gone,
}

impl<Fut: Future> MaybeDone<Fut> {
    /// Create a new instance of `MaybeDone`.
    pub(crate) fn new(future: Fut) -> MaybeDone<Fut> {
        Self::Future(future)
    }
}

impl<T, E, Fut> MaybeDone<Fut>
where
    Fut: Future<Output = Result<T, E>>,
{
    /// Attempt to take the `Ok(output)` of a `MaybeDone` without driving it towards completion.
    /// If the future is done but is an `Err(_)`, this will return `None`.
    #[inline]
    pub(crate) fn take_ok(self: Pin<&mut Self>) -> Option<T> {
        let this = unsafe { self.get_unchecked_mut() };
        match this {
            MaybeDone::Done(Ok(_)) => {}
            MaybeDone::Done(Err(_)) | MaybeDone::Future(_) | MaybeDone::Gone => return None,
        }
        match mem::replace(this, MaybeDone::Gone) {
            MaybeDone::Done(Ok(output)) => Some(output),
            _ => unreachable!(),
        }
    }

    /// Attempt to take the `Err(output)` of a `MaybeDone` without driving it towards completion.
    /// If the future is done but is an `Ok(_)`, this will return `None`.
    #[inline]
    pub(crate) fn take_err(self: Pin<&mut Self>) -> Option<E> {
        let this = unsafe { self.get_unchecked_mut() };
        match this {
            MaybeDone::Done(Err(_)) => {}
            MaybeDone::Done(Ok(_)) | MaybeDone::Future(_) | MaybeDone::Gone => return None,
        }
        match mem::replace(this, MaybeDone::Gone) {
            MaybeDone::Done(Err(output)) => Some(output),
            _ => unreachable!(),
        }
    }
}

impl<Fut: Future> Future for MaybeDone<Fut> {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let res = unsafe {
            match Pin::as_mut(&mut self).get_unchecked_mut() {
                MaybeDone::Future(a) => ready!(Pin::new_unchecked(a).poll(cx)),
                MaybeDone::Done(_) => return Poll::Ready(()),
                MaybeDone::Gone => panic!("MaybeDone polled after value taken"),
            }
        };
        self.set(MaybeDone::Done(res));
        Poll::Ready(())
    }
}
