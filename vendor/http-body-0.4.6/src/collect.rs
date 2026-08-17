use std::{
    collections::VecDeque,
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

use super::Body;

use bytes::{Buf, Bytes};
use http::HeaderMap;
use pin_project_lite::pin_project;

pin_project! {
    /// Future that resolves into a [`Collected`].
    pub struct Collect<T>
    where
        T: Body,
    {
        #[pin]
        body: T,
        collected: Option<Collected<T::Data>>,
        is_data_done: bool,
    }
}

impl<T: Body> Collect<T> {
    pub(crate) fn new(body: T) -> Self {
        Self {
            body,
            collected: Some(Collected::default()),
            is_data_done: false,
        }
    }
}

impl<T: Body> Future for Collect<T> {
    type Output = Result<Collected<T::Data>, T::Error>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut me = self.project();

        loop {
            if !*me.is_data_done {
                match me.body.as_mut().poll_data(cx) {
                    Poll::Ready(Some(Ok(data))) => {
                        me.collected.as_mut().unwrap().push_data(data);
                    }
                    Poll::Ready(Some(Err(err))) => {
                        return Poll::Ready(Err(err));
                    }
                    Poll::Ready(None) => {
                        *me.is_data_done = true;
                    }
                    Poll::Pending => return Poll::Pending,
                }
            } else {
                match me.body.as_mut().poll_trailers(cx) {
                    Poll::Ready(Ok(Some(trailers))) => {
                        me.collected.as_mut().unwrap().push_trailers(trailers);
                        break;
                    }
                    Poll::Ready(Err(err)) => {
                        return Poll::Ready(Err(err));
                    }
                    Poll::Ready(Ok(None)) => break,
                    Poll::Pending => return Poll::Pending,
                }
            }
        }

        Poll::Ready(Ok(me.collected.take().expect("polled after complete")))
    }
}

/// A collected body produced by [`Body::collect`] which collects all the DATA frames
/// and trailers.
#[derive(Debug)]
pub struct Collected<B> {
    bufs: BufList<B>,
    trailers: Option<HeaderMap>,
}

impl<B: Buf> Collected<B> {
    /// If there is a trailers frame buffered, returns a reference to it.
    ///
    /// Returns `None` if the body contained no trailers.
    pub fn trailers(&self) -> Option<&HeaderMap> {
        self.trailers.as_ref()
    }

    /// Aggregate this buffered into a [`Buf`].
    pub fn aggregate(self) -> impl Buf {
        self.bufs
    }

    /// Convert this body into a [`Bytes`].
    pub fn to_bytes(mut self) -> Bytes {
        self.bufs.copy_to_bytes(self.bufs.remaining())
    }

    fn push_data(&mut self, data: B) {
        // Only push this frame if it has some data in it, to avoid crashing on
        // `BufList::push`.
        if data.has_remaining() {
            self.bufs.push(data);
        }
    }

    fn push_trailers(&mut self, trailers: HeaderMap) {
        if let Some(current) = &mut self.trailers {
            current.extend(trailers);
        } else {
            self.trailers = Some(trailers);
        }
    }
}

impl<B> Default for Collected<B> {
    fn default() -> Self {
        Self {
            bufs: BufList::default(),
            trailers: None,
        }
    }
}

impl<B> Unpin for Collected<B> {}

#[derive(Debug)]
struct BufList<T> {
    bufs: VecDeque<T>,
}

impl<T: Buf> BufList<T> {
    #[inline]
    pub(crate) fn push(&mut self, buf: T) {
        debug_assert!(buf.has_remaining());
        self.bufs.push_back(buf);
    }

    /*
    #[inline]
    pub(crate) fn pop(&mut self) -> Option<T> {
        self.bufs.pop_front()
    }
    */
}

impl<T: Buf> Buf for BufList<T> {
    #[inline]
    fn remaining(&self) -> usize {
        self.bufs.iter().map(|buf| buf.remaining()).sum()
    }

    #[inline]
    fn chunk(&self) -> &[u8] {
        self.bufs.front().map(Buf::chunk).unwrap_or_default()
    }

    #[inline]
    fn advance(&mut self, mut cnt: usize) {
        while cnt > 0 {
            {
                let front = &mut self.bufs[0];
                let rem = front.remaining();
                if rem > cnt {
                    front.advance(cnt);
                    return;
                } else {
                    front.advance(rem);
                    cnt -= rem;
                }
            }
            self.bufs.pop_front();
        }
    }

    #[inline]
    fn chunks_vectored<'t>(&'t self, dst: &mut [std::io::IoSlice<'t>]) -> usize {
        if dst.is_empty() {
            return 0;
        }
        let mut vecs = 0;
        for buf in &self.bufs {
            vecs += buf.chunks_vectored(&mut dst[vecs..]);
            if vecs == dst.len() {
                break;
            }
        }
        vecs
    }

    #[inline]
    fn copy_to_bytes(&mut self, len: usize) -> Bytes {
        use bytes::{BufMut, BytesMut};
        // Our inner buffer may have an optimized version of copy_to_bytes, and if the whole
        // request can be fulfilled by the front buffer, we can take advantage.
        match self.bufs.front_mut() {
            Some(front) if front.remaining() == len => {
                let b = front.copy_to_bytes(len);
                self.bufs.pop_front();
                b
            }
            Some(front) if front.remaining() > len => front.copy_to_bytes(len),
            _ => {
                assert!(len <= self.remaining(), "`len` greater than remaining");
                let mut bm = BytesMut::with_capacity(len);
                bm.put(self.take(len));
                bm.freeze()
            }
        }
    }
}

impl<T> Default for BufList<T> {
    fn default() -> Self {
        BufList {
            bufs: VecDeque::new(),
        }
    }
}
