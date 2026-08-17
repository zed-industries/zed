#![allow(dead_code)]

use pin_project::pin_project;
use std::marker::PhantomData;

use crate::stream::{PollRecv, Stream};

pub fn ready<T>(value: T) -> impl Stream<Item = T>
where
    T: Clone,
{
    crate::stream::repeat(value)
}

pub fn pending<T>() -> impl Stream<Item = T> {
    PendingStream::new()
}

pub fn closed<T>() -> impl Stream<Item = T> {
    ClosedStream::new()
}

pub fn from_iter<I>(iter: I) -> impl Stream<Item = I::Item>
where
    I: IntoIterator,
{
    IterStream::new(iter.into_iter())
}

pub fn from_poll_iter<I, T>(iter: I) -> impl Stream<Item = T>
where
    I: IntoIterator<Item = PollRecv<T>>,
{
    PollIterStream::new(iter.into_iter())
}

struct PendingStream<T> {
    _t: PhantomData<T>,
}

impl<T> PendingStream<T> {
    pub fn new() -> Self {
        Self { _t: PhantomData }
    }
}

impl<T> Stream for PendingStream<T> {
    type Item = T;

    fn poll_recv(
        self: std::pin::Pin<&mut Self>,
        _cx: &mut crate::Context<'_>,
    ) -> crate::stream::PollRecv<Self::Item> {
        PollRecv::Pending
    }
}
struct ClosedStream<T> {
    _t: PhantomData<T>,
}

impl<T> ClosedStream<T> {
    pub fn new() -> Self {
        Self { _t: PhantomData }
    }
}

impl<T> Stream for ClosedStream<T> {
    type Item = T;

    fn poll_recv(
        self: std::pin::Pin<&mut Self>,
        _cx: &mut crate::Context<'_>,
    ) -> crate::stream::PollRecv<Self::Item> {
        PollRecv::Closed
    }
}
#[pin_project]
pub struct IterStream<I: Iterator> {
    iter: I,
}

impl<I> IterStream<I>
where
    I: Iterator,
{
    pub fn new(iter: I) -> Self {
        Self { iter }
    }
}

impl<I> Stream for IterStream<I>
where
    I: Iterator,
{
    type Item = I::Item;

    fn poll_recv(
        self: std::pin::Pin<&mut Self>,
        _cx: &mut crate::Context<'_>,
    ) -> PollRecv<Self::Item> {
        let this = self.project();
        match this.iter.next() {
            Some(value) => PollRecv::Ready(value),
            None => PollRecv::Closed,
        }
    }
}

#[pin_project]
pub struct PollIterStream<I: Iterator, T> {
    iter: I,
    _t: PhantomData<T>,
}

impl<I, T> PollIterStream<I, T>
where
    I: Iterator,
{
    pub fn new(iter: I) -> Self {
        Self {
            iter,
            _t: PhantomData,
        }
    }
}

impl<I, T> Stream for PollIterStream<I, T>
where
    I: Iterator<Item = PollRecv<T>>,
{
    type Item = T;

    fn poll_recv(
        self: std::pin::Pin<&mut Self>,
        _cx: &mut crate::Context<'_>,
    ) -> PollRecv<Self::Item> {
        let this = self.project();
        match this.iter.next() {
            Some(poll) => poll,
            None => PollRecv::Closed,
        }
    }
}
