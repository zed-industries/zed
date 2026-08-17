// Symphonia
// Copyright (c) 2019-2022 The Project Symphonia Developers.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! The `audio` module provides primitives for working with multi-channel audio buffers of varying
//! sample formats.

use std::borrow::Cow;
use std::fmt;
use std::marker::PhantomData;
use std::mem;
use std::vec::Vec;

use arrayvec::ArrayVec;
use bitflags::bitflags;

use crate::conv::{ConvertibleSample, FromSample, IntoSample};
use crate::errors::Result;
use crate::sample::{i24, u24, Sample};
use crate::units::Duration;

/// The maximum number of audio plane slices `AudioPlanes` or `AudioPlanesMut` will store on the
/// stack before storing the slices on the heap.
const AUDIO_PLANES_STORAGE_STACK_LIMIT: usize = 8;

bitflags! {
    /// A bitmask representing the audio channels in an audio buffer or signal.
    ///
    /// The first 18 defined channels are guaranteed to be identical to those specified by
    /// Microsoft's WAVEFORMATEXTENSIBLE structure. Channels after 18 are defined by Symphonia and
    /// no order is guaranteed.
    #[derive(Default)]
    pub struct Channels: u32 {
        /// Front-left (left) or the Mono channel.
        const FRONT_LEFT         = 0x0000_0001;
        /// Front-right (right) channel.
        const FRONT_RIGHT        = 0x0000_0002;
        /// Front-centre (centre) channel.
        const FRONT_CENTRE       = 0x0000_0004;
        /// Low frequency channel 1.
        const LFE1               = 0x0000_0008;
        /// Rear-left (surround rear left) channel.
        const REAR_LEFT          = 0x0000_0010;
        /// Rear-right (surround rear right) channel.
        const REAR_RIGHT         = 0x0000_0020;
        /// Front left-of-centre (left center) channel.
        const FRONT_LEFT_CENTRE  = 0x0000_0040;
        /// Front right-of-centre (right center) channel.
        const FRONT_RIGHT_CENTRE = 0x0000_0080;
        /// Rear-centre (surround rear centre) channel.
        const REAR_CENTRE        = 0x0000_0100;
        /// Side left (surround left) channel.
        const SIDE_LEFT          = 0x0000_0200;
        /// Side right (surround right) channel.
        const SIDE_RIGHT         = 0x0000_0400;
        /// Top centre channel.
        const TOP_CENTRE         = 0x0000_0800;
        /// Top front-left channel.
        const TOP_FRONT_LEFT     = 0x0000_1000;
        /// Top centre channel.
        const TOP_FRONT_CENTRE   = 0x0000_2000;
        /// Top front-right channel.
        const TOP_FRONT_RIGHT    = 0x0000_4000;
        /// Top rear-left channel.
        const TOP_REAR_LEFT      = 0x0000_8000;
        /// Top rear-centre channel.
        const TOP_REAR_CENTRE    = 0x0001_0000;
        /// Top rear-right channel.
        const TOP_REAR_RIGHT     = 0x0002_0000;
        /// Rear left-of-centre channel.
        const REAR_LEFT_CENTRE   = 0x0004_0000;
        /// Rear right-of-centre channel.
        const REAR_RIGHT_CENTRE  = 0x0008_0000;
        /// Front left-wide channel.
        const FRONT_LEFT_WIDE    = 0x0010_0000;
        /// Front right-wide channel.
        const FRONT_RIGHT_WIDE   = 0x0020_0000;
        /// Front left-high channel.
        const FRONT_LEFT_HIGH    = 0x0040_0000;
        /// Front centre-high channel.
        const FRONT_CENTRE_HIGH  = 0x0080_0000;
        /// Front right-high channel.
        const FRONT_RIGHT_HIGH   = 0x0100_0000;
        /// Low frequency channel 2.
        const LFE2               = 0x0200_0000;
    }
}

/// An iterator over individual channels within a `Channels` bitmask.
pub struct ChannelsIter {
    channels: Channels,
}

impl Iterator for ChannelsIter {
    type Item = Channels;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.channels.is_empty() {
            let channel = Channels::from_bits_truncate(1 << self.channels.bits.trailing_zeros());
            self.channels ^= channel;
            Some(channel)
        }
        else {
            None
        }
    }
}

impl Channels {
    /// Gets the number of channels.
    pub fn count(self) -> usize {
        self.bits.count_ones() as usize
    }

    /// Gets an iterator over individual channels.
    pub fn iter(&self) -> ChannelsIter {
        ChannelsIter { channels: *self }
    }
}

impl fmt::Display for Channels {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:#032b}", self.bits)
    }
}

/// `Layout` describes common audio channel configurations.
#[derive(Copy, Clone, Debug)]
pub enum Layout {
    /// Single centre channel.
    Mono,
    /// Left and Right channels.
    Stereo,
    /// Left and Right channels with a single low-frequency channel.
    TwoPointOne,
    /// Front Left and Right, Rear Left and Right, and a single low-frequency channel.
    FivePointOne,
}

impl Layout {
    /// Converts a channel `Layout` into a `Channels` bit mask.
    pub fn into_channels(self) -> Channels {
        match self {
            Layout::Mono => Channels::FRONT_LEFT,
            Layout::Stereo => Channels::FRONT_LEFT | Channels::FRONT_RIGHT,
            Layout::TwoPointOne => Channels::FRONT_LEFT | Channels::FRONT_RIGHT | Channels::LFE1,
            Layout::FivePointOne => {
                Channels::FRONT_LEFT
                    | Channels::FRONT_RIGHT
                    | Channels::FRONT_CENTRE
                    | Channels::REAR_LEFT
                    | Channels::REAR_RIGHT
                    | Channels::LFE1
            }
        }
    }
}

/// `SignalSpec` describes the characteristics of a Signal.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct SignalSpec {
    /// The signal sampling rate in hertz (Hz).
    pub rate: u32,

    /// The channel assignments of the signal. The order of the channels in the vector is the order
    /// in which each channel sample is stored in a frame.
    pub channels: Channels,
}

impl SignalSpec {
    pub fn new(rate: u32, channels: Channels) -> Self {
        SignalSpec { rate, channels }
    }

    pub fn new_with_layout(rate: u32, layout: Layout) -> Self {
        SignalSpec { rate, channels: layout.into_channels() }
    }
}

/// Small-storage optimization capable storage of immutable slices of `AudioBuffer` audio planes.
enum AudioPlaneStorage<'a, S, const N: usize> {
    Stack(ArrayVec<&'a [S], N>),
    Heap(Vec<&'a [S]>),
}

/// `AudioPlanes` provides immutable slices to each audio channel (plane) contained in a signal.
pub struct AudioPlanes<'a, S: 'a + Sample> {
    planes: AudioPlaneStorage<'a, S, AUDIO_PLANES_STORAGE_STACK_LIMIT>,
}

impl<'a, S: Sample> AudioPlanes<'a, S> {
    /// Instantiate `AudioPlanes` for the given channel configuration.
    fn new(channels: Channels) -> Self {
        let n_planes = channels.count();

        if n_planes <= AUDIO_PLANES_STORAGE_STACK_LIMIT {
            AudioPlanes { planes: AudioPlaneStorage::Stack(ArrayVec::new()) }
        }
        else {
            AudioPlanes { planes: AudioPlaneStorage::Heap(Vec::with_capacity(n_planes)) }
        }
    }

    /// Push an immutable reference to an audio plane. This function may panic if the number of
    /// pushed planes exceeds the number specified at instantiation.
    fn push(&mut self, plane: &'a [S]) {
        match &mut self.planes {
            AudioPlaneStorage::Stack(planes) => {
                debug_assert!(!planes.is_full());
                planes.push(plane);
            }
            AudioPlaneStorage::Heap(planes) => {
                planes.push(plane);
            }
        }
    }

    /// Gets immutable slices of all the audio planes.
    pub fn planes(&self) -> &[&'a [S]] {
        match &self.planes {
            AudioPlaneStorage::Stack(planes) => planes,
            AudioPlaneStorage::Heap(planes) => planes,
        }
    }
}

/// Small-storage optimization capable storage of mutable slices of `AudioBuffer` audio planes.
enum AudioPlaneStorageMut<'a, S, const N: usize> {
    Stack(ArrayVec<&'a mut [S], N>),
    Heap(Vec<&'a mut [S]>),
}

/// `AudioPlanesMut` provides mutable slices to each audio channel (plane) contained in a signal.
pub struct AudioPlanesMut<'a, S: 'a + Sample> {
    planes: AudioPlaneStorageMut<'a, S, AUDIO_PLANES_STORAGE_STACK_LIMIT>,
}

impl<'a, S: Sample> AudioPlanesMut<'a, S> {
    /// Instantiate `AudioPlanesMut` for the given channel configuration.
    fn new(channels: Channels) -> Self {
        let n_planes = channels.count();

        if n_planes <= AUDIO_PLANES_STORAGE_STACK_LIMIT {
            AudioPlanesMut { planes: AudioPlaneStorageMut::Stack(ArrayVec::new()) }
        }
        else {
            AudioPlanesMut { planes: AudioPlaneStorageMut::Heap(Vec::with_capacity(n_planes)) }
        }
    }

    /// Push a mutable reference to an audio plane. This function may panic if the number of
    /// pushed planes exceeds the number specified at instantiation.
    fn push(&mut self, plane: &'a mut [S]) {
        match &mut self.planes {
            AudioPlaneStorageMut::Stack(planes) => {
                debug_assert!(!planes.is_full());
                planes.push(plane);
            }
            AudioPlaneStorageMut::Heap(storage) => {
                storage.push(plane);
            }
        }
    }

    /// Gets mutable slices of all the audio planes.
    pub fn planes(&mut self) -> &mut [&'a mut [S]] {
        match &mut self.planes {
            AudioPlaneStorageMut::Stack(planes) => planes,
            AudioPlaneStorageMut::Heap(planes) => planes,
        }
    }
}

/// `AudioBuffer` is a container for multi-channel planar audio sample data. An `AudioBuffer` is
/// characterized by the duration (capacity), and audio specification (channels and sample rate).
/// The capacity of an `AudioBuffer` is the maximum number of samples the buffer may store per
/// channel. Manipulation of samples is accomplished through the Signal trait or direct buffer
/// manipulation.
#[derive(Clone)]
pub struct AudioBuffer<S: Sample> {
    buf: Vec<S>,
    spec: SignalSpec,
    n_frames: usize,
    n_capacity: usize,
}

impl<S: Sample> AudioBuffer<S> {
    /// Instantiate a new `AudioBuffer` using the specified signal specification and of the given
    /// duration.
    pub fn new(duration: Duration, spec: SignalSpec) -> Self {
        // The number of channels * duration cannot exceed u64::MAX.
        assert!(duration <= u64::MAX / spec.channels.count() as u64, "duration too large");

        // The total number of samples the buffer will store.
        let n_samples = duration * spec.channels.count() as u64;

        // Practically speaking, it is not possible to allocate more than usize::MAX bytes of
        // samples. This assertion ensures the potential downcast of n_samples to usize below is
        // safe.
        assert!(n_samples <= (usize::MAX / mem::size_of::<S>()) as u64, "duration too large");

        // Allocate sample buffer and default initialize all samples to silence.
        let buf = vec![S::MID; n_samples as usize];

        AudioBuffer { buf, spec, n_frames: 0, n_capacity: duration as usize }
    }

    /// Instantiates an unused `AudioBuffer`. An unused `AudioBuffer` will not allocate any memory,
    /// has a sample rate of 0, and no audio channels.
    pub fn unused() -> Self {
        AudioBuffer {
            buf: Vec::with_capacity(0),
            spec: SignalSpec::new(0, Channels::empty()),
            n_frames: 0,
            n_capacity: 0,
        }
    }

    /// Returns `true` if the `AudioBuffer` is unused.
    pub fn is_unused(&self) -> bool {
        self.n_capacity == 0
    }

    /// Gets the signal specification for the buffer.
    pub fn spec(&self) -> &SignalSpec {
        &self.spec
    }

    /// Gets the total capacity of the buffer. The capacity is the maximum number of audio frames
    /// a buffer can store.
    pub fn capacity(&self) -> usize {
        self.n_capacity
    }

    /// Gets immutable references to all audio planes (channels) within the audio buffer.
    ///
    /// Note: This is not a cheap operation for audio buffers with > 8 channels. It is advisable
    /// that this call is only used when operating on large batches of frames. Generally speaking,
    /// it is almost always better to use `chan()` to selectively choose the plane to read instead.
    pub fn planes(&self) -> AudioPlanes<'_, S> {
        // Fill the audio planes structure with references to the written portion of each audio
        // plane.
        let mut planes = AudioPlanes::new(self.spec.channels);

        for channel in self.buf.chunks_exact(self.n_capacity) {
            planes.push(&channel[..self.n_frames]);
        }

        planes
    }

    /// Gets mutable references to all audio planes (channels) within the buffer.
    ///
    /// Note: This is not a cheap operation for audio buffers with > 8 channels. It is advisable
    /// that this call is only used when modifying large batches of frames. Generally speaking,
    /// it is almost always better to use `render()`, `fill()`, `chan_mut()`, and `chan_pair_mut()`
    /// to modify the buffer instead.
    pub fn planes_mut(&mut self) -> AudioPlanesMut<'_, S> {
        // Fill the audio planes structure with references to the written portion of each audio
        // plane.
        let mut planes = AudioPlanesMut::new(self.spec.channels);

        for channel in self.buf.chunks_exact_mut(self.n_capacity) {
            planes.push(&mut channel[..self.n_frames]);
        }

        planes
    }

    /// Converts the contents of an AudioBuffer into an equivalent destination AudioBuffer of a
    /// different type. If the types are the same then this is a copy operation.
    pub fn convert<T: Sample>(&self, dest: &mut AudioBuffer<T>)
    where
        S: IntoSample<T>,
    {
        assert!(dest.n_capacity >= self.n_capacity);
        assert!(dest.spec == self.spec);

        for c in 0..self.spec.channels.count() {
            let begin = c * self.n_capacity;
            let end = begin + self.n_frames;

            for (d, s) in dest.buf[begin..end].iter_mut().zip(&self.buf[begin..end]) {
                *d = (*s).into_sample();
            }
        }

        dest.n_frames = self.n_frames;
    }

    /// Makes an equivalent AudioBuffer of a different type.
    pub fn make_equivalent<E: Sample>(&self) -> AudioBuffer<E> {
        AudioBuffer::<E>::new(self.n_capacity as Duration, self.spec)
    }
}

macro_rules! impl_audio_buffer_ref_func {
    ($var:expr, $buf:ident,$expr:expr) => {
        match $var {
            AudioBufferRef::U8($buf) => $expr,
            AudioBufferRef::U16($buf) => $expr,
            AudioBufferRef::U24($buf) => $expr,
            AudioBufferRef::U32($buf) => $expr,
            AudioBufferRef::S8($buf) => $expr,
            AudioBufferRef::S16($buf) => $expr,
            AudioBufferRef::S24($buf) => $expr,
            AudioBufferRef::S32($buf) => $expr,
            AudioBufferRef::F32($buf) => $expr,
            AudioBufferRef::F64($buf) => $expr,
        }
    };
}

/// `AudioBufferRef` is a copy-on-write reference to an `AudioBuffer` of any type.
#[derive(Clone)]
pub enum AudioBufferRef<'a> {
    U8(Cow<'a, AudioBuffer<u8>>),
    U16(Cow<'a, AudioBuffer<u16>>),
    U24(Cow<'a, AudioBuffer<u24>>),
    U32(Cow<'a, AudioBuffer<u32>>),
    S8(Cow<'a, AudioBuffer<i8>>),
    S16(Cow<'a, AudioBuffer<i16>>),
    S24(Cow<'a, AudioBuffer<i24>>),
    S32(Cow<'a, AudioBuffer<i32>>),
    F32(Cow<'a, AudioBuffer<f32>>),
    F64(Cow<'a, AudioBuffer<f64>>),
}

impl AudioBufferRef<'_> {
    /// Gets the signal specification for the buffer.
    pub fn spec(&self) -> &SignalSpec {
        impl_audio_buffer_ref_func!(self, buf, buf.spec())
    }

    /// Gets the total capacity of the buffer. The capacity is the maximum number of audio frames
    /// a buffer can store.
    pub fn capacity(&self) -> usize {
        impl_audio_buffer_ref_func!(self, buf, buf.capacity())
    }

    /// Gets the number of frames in the buffer.
    pub fn frames(&self) -> usize {
        impl_audio_buffer_ref_func!(self, buf, buf.frames())
    }

    pub fn convert<T>(&self, dest: &mut AudioBuffer<T>)
    where
        T: Sample
            + FromSample<u8>
            + FromSample<u16>
            + FromSample<u24>
            + FromSample<u32>
            + FromSample<i8>
            + FromSample<i16>
            + FromSample<i24>
            + FromSample<i32>
            + FromSample<f32>
            + FromSample<f64>,
    {
        impl_audio_buffer_ref_func!(self, buf, buf.convert(dest))
    }

    pub fn make_equivalent<E: Sample>(&self) -> AudioBuffer<E> {
        impl_audio_buffer_ref_func!(self, buf, buf.make_equivalent::<E>())
    }
}

/// `AsAudioBufferRef` is a trait implemented for `AudioBuffer`s that may be referenced in an
/// `AudioBufferRef`.
pub trait AsAudioBufferRef {
    /// Get an `AudioBufferRef` reference.
    fn as_audio_buffer_ref(&self) -> AudioBufferRef<'_>;
}

macro_rules! impl_as_audio_buffer_ref {
    ($fmt:ty, $ref:path) => {
        impl AsAudioBufferRef for AudioBuffer<$fmt> {
            fn as_audio_buffer_ref(&self) -> AudioBufferRef<'_> {
                $ref(Cow::Borrowed(self))
            }
        }
    };
}

impl_as_audio_buffer_ref!(u8, AudioBufferRef::U8);
impl_as_audio_buffer_ref!(u16, AudioBufferRef::U16);
impl_as_audio_buffer_ref!(u24, AudioBufferRef::U24);
impl_as_audio_buffer_ref!(u32, AudioBufferRef::U32);
impl_as_audio_buffer_ref!(i8, AudioBufferRef::S8);
impl_as_audio_buffer_ref!(i16, AudioBufferRef::S16);
impl_as_audio_buffer_ref!(i24, AudioBufferRef::S24);
impl_as_audio_buffer_ref!(i32, AudioBufferRef::S32);
impl_as_audio_buffer_ref!(f32, AudioBufferRef::F32);
impl_as_audio_buffer_ref!(f64, AudioBufferRef::F64);

/// The `Signal` trait provides methods for rendering and transforming contiguous buffers of audio
/// data.
pub trait Signal<S: Sample> {
    /// Gets the number of actual frames written to the buffer. Conversely, this also is the number
    /// of written samples in any one channel.
    fn frames(&self) -> usize;

    /// Clears all written frames from the buffer. This is a cheap operation and does not zero the
    /// underlying audio data.
    fn clear(&mut self);

    /// Gets an immutable reference to all the written samples in the specified channel.
    fn chan(&self, channel: usize) -> &[S];

    /// Gets a mutable reference to all the written samples in the specified channel.
    fn chan_mut(&mut self, channel: usize) -> &mut [S];

    /// Gets two mutable references to two different channels.
    fn chan_pair_mut(&mut self, first: usize, second: usize) -> (&mut [S], &mut [S]);

    /// Renders a number of silent frames.
    ///
    /// If `n_frames` is `None`, the remaining number of frames will be used.
    fn render_silence(&mut self, n_frames: Option<usize>);

    /// Renders a reserved number of frames. This is a cheap operation and simply advances the frame
    /// counter. The underlying audio data is not modified and should be overwritten through other
    /// means.
    ///
    /// If `n_frames` is `None`, the remaining number of frames will be used. If `n_frames` is too
    /// large, this function will assert.
    fn render_reserved(&mut self, n_frames: Option<usize>);

    /// Renders a number of frames using the provided render function. The number of frames to
    /// render is specified by `n_frames`. If `n_frames` is `None`, the remaining number of frames
    /// in the buffer will be rendered. If the render function returns an error, the render
    /// operation is terminated prematurely.
    fn render<'a, F>(&'a mut self, n_frames: Option<usize>, render: F) -> Result<()>
    where
        F: FnMut(&mut AudioPlanesMut<'a, S>, usize) -> Result<()>;

    /// Clears, and then renders the entire buffer using the fill function. This is a convenience
    /// wrapper around `render` and exhibits the same behaviour as `render` in regards to the fill
    /// function.
    #[inline]
    fn fill<'a, F>(&'a mut self, fill: F) -> Result<()>
    where
        F: FnMut(&mut AudioPlanesMut<'a, S>, usize) -> Result<()>,
    {
        self.clear();
        self.render(None, fill)
    }

    /// Transforms every written sample in the signal using the transformation function provided.
    /// This function does not guarantee an order in which the samples are transformed.
    fn transform<F>(&mut self, f: F)
    where
        F: Fn(S) -> S;

    /// Truncates the buffer to the number of frames specified. If the number of frames in the
    /// buffer is less-than the number of frames specified, then this function does nothing.
    fn truncate(&mut self, n_frames: usize);

    /// Shifts the contents of the buffer back by the number of frames specified. The leading frames
    /// are dropped from the buffer.
    fn shift(&mut self, shift: usize);

    /// Trims samples from the start and end of the buffer.
    fn trim(&mut self, start: usize, end: usize) {
        // First, trim the end to reduce the number of frames have to be shifted when the front is
        // trimmed.
        self.truncate(self.frames().saturating_sub(end));

        // Second, trim the start.
        self.shift(start);
    }
}

impl<S: Sample> Signal<S> for AudioBuffer<S> {
    fn clear(&mut self) {
        self.n_frames = 0;
    }

    fn frames(&self) -> usize {
        self.n_frames
    }

    fn chan(&self, channel: usize) -> &[S] {
        let start = channel * self.n_capacity;

        // If the channel index is invalid the slice will be out-of-bounds.
        assert!(start + self.n_capacity <= self.buf.len(), "invalid channel index");

        &self.buf[start..start + self.n_frames]
    }

    fn chan_mut(&mut self, channel: usize) -> &mut [S] {
        let start = channel * self.n_capacity;

        // If the channel index is invalid the slice will be out-of-bounds.
        assert!(start + self.n_capacity <= self.buf.len(), "invalid channel index");

        &mut self.buf[start..start + self.n_frames]
    }

    fn chan_pair_mut(&mut self, first: usize, second: usize) -> (&mut [S], &mut [S]) {
        // Both channels in the pair must be unique.
        assert!(first != second, "channel indicies cannot be the same");

        let first_idx = self.n_capacity * first;
        let second_idx = self.n_capacity * second;

        // If a channel index is invalid the slice will be out-of-bounds.
        assert!(first_idx + self.n_capacity <= self.buf.len(), "invalid channel index");
        assert!(second_idx + self.n_capacity <= self.buf.len(), "invalid channel index");

        if first_idx < second_idx {
            let (a, b) = self.buf.split_at_mut(second_idx);

            (&mut a[first_idx..first_idx + self.n_frames], &mut b[..self.n_frames])
        }
        else {
            let (a, b) = self.buf.split_at_mut(first_idx);

            (&mut b[..self.n_frames], &mut a[second_idx..second_idx + self.n_frames])
        }
    }

    fn render_silence(&mut self, n_frames: Option<usize>) {
        let n_silent_frames = n_frames.unwrap_or(self.n_capacity - self.n_frames);

        // Do not render past the end of the audio buffer.
        assert!(self.n_frames + n_silent_frames <= self.capacity(), "capacity will be exceeded");

        for channel in self.buf.chunks_exact_mut(self.n_capacity) {
            for sample in &mut channel[self.n_frames..self.n_frames + n_silent_frames] {
                *sample = S::MID;
            }
        }

        self.n_frames += n_silent_frames;
    }

    fn render_reserved(&mut self, n_frames: Option<usize>) {
        let n_reserved_frames = n_frames.unwrap_or(self.n_capacity - self.n_frames);
        // Do not render past the end of the audio buffer.
        assert!(self.n_frames + n_reserved_frames <= self.n_capacity, "capacity will be exceeded");
        self.n_frames += n_reserved_frames;
    }

    fn render<'a, F>(&'a mut self, n_frames: Option<usize>, mut render: F) -> Result<()>
    where
        F: FnMut(&mut AudioPlanesMut<'a, S>, usize) -> Result<()>,
    {
        // The number of frames to be rendered is the amount requested, if specified, or the
        // remainder of the audio buffer.
        let n_render_frames = n_frames.unwrap_or(self.n_capacity - self.n_frames);

        // Do not render past the end of the audio buffer.
        let end = self.n_frames + n_render_frames;
        assert!(end <= self.n_capacity, "capacity will be exceeded");

        // At this point, n_render_frames can be considered "reserved". Create an audio plane
        // structure and fill each plane entry with a reference to the "reserved" samples in each
        // channel respectively.
        let mut planes = AudioPlanesMut::new(self.spec.channels);

        for channel in self.buf.chunks_exact_mut(self.n_capacity) {
            planes.push(&mut channel[self.n_frames..end]);
        }

        // Attempt to render the into the reserved frames, one-by-one, exiting only if there is an
        // error in the render function.
        while self.n_frames < end {
            render(&mut planes, self.n_frames)?;
            self.n_frames += 1;
        }

        Ok(())
    }

    fn transform<F>(&mut self, f: F)
    where
        F: Fn(S) -> S,
    {
        debug_assert!(self.n_frames <= self.n_capacity);

        // Apply the transformation function over each sample in each plane.
        for plane in self.buf.chunks_mut(self.n_capacity) {
            for sample in &mut plane[0..self.n_frames] {
                *sample = f(*sample);
            }
        }
    }

    fn truncate(&mut self, n_frames: usize) {
        if n_frames < self.n_frames {
            self.n_frames = n_frames;
        }
    }

    fn shift(&mut self, shift: usize) {
        if shift >= self.n_frames {
            self.clear();
        }
        else if shift > 0 {
            // Shift the samples down in each plane.
            for plane in self.buf.chunks_mut(self.n_capacity) {
                plane.copy_within(shift..self.n_frames, 0);
            }
            self.n_frames -= shift;
        }
    }
}

/// A `SampleBuffer`, is a sample oriented buffer. It is agnostic to the ordering/layout of samples
/// within the buffer. `SampleBuffer` is mean't for safely importing and exporting sample data to
/// and from Symphonia using the sample's in-memory data-type.
pub struct SampleBuffer<S: Sample> {
    buf: Box<[S]>,
    n_written: usize,
}

impl<S: Sample> SampleBuffer<S> {
    /// Instantiate a new `SampleBuffer` using the specified signal specification and of the given
    /// duration.
    pub fn new(duration: Duration, spec: SignalSpec) -> SampleBuffer<S> {
        // The number of channels * duration cannot exceed u64::MAX.
        assert!(duration <= u64::MAX / spec.channels.count() as u64, "duration too large");

        // The total number of samples the buffer will store.
        let n_samples = duration * spec.channels.count() as u64;

        // Practically speaking, it is not possible to allocate more than usize::MAX bytes of
        // samples. This assertion ensures the potential downcast of n_samples to usize below is
        // safe.
        assert!(n_samples <= (usize::MAX / mem::size_of::<S>()) as u64, "duration too large");

        // Allocate enough memory for all the samples and fill the buffer with silence.
        let buf = vec![S::MID; n_samples as usize].into_boxed_slice();

        SampleBuffer { buf, n_written: 0 }
    }

    /// Gets the number of written samples.
    pub fn len(&self) -> usize {
        self.n_written
    }

    /// Returns `true` if the buffer contains no written samples.
    pub fn is_empty(&self) -> bool {
        self.n_written == 0
    }

    /// Gets an immutable slice of all written samples.
    pub fn samples(&self) -> &[S] {
        &self.buf[..self.n_written]
    }

    /// Gets a mutable slice of all written samples.
    pub fn samples_mut(&mut self) -> &mut [S] {
        &mut self.buf[..self.n_written]
    }

    /// Gets the maximum number of samples the `SampleBuffer` may store.
    pub fn capacity(&self) -> usize {
        self.buf.len()
    }

    /// Clears all written samples.
    pub fn clear(&mut self) {
        self.n_written = 0;
    }

    /// Copies all audio data from the source `AudioBufferRef` in planar channel order into the
    /// `SampleBuffer`. The two buffers must be equivalent.
    pub fn copy_planar_ref(&mut self, src: AudioBufferRef)
    where
        S: ConvertibleSample,
    {
        match src {
            AudioBufferRef::U8(buf) => self.copy_planar_typed(&buf),
            AudioBufferRef::U16(buf) => self.copy_planar_typed(&buf),
            AudioBufferRef::U24(buf) => self.copy_planar_typed(&buf),
            AudioBufferRef::U32(buf) => self.copy_planar_typed(&buf),
            AudioBufferRef::S8(buf) => self.copy_planar_typed(&buf),
            AudioBufferRef::S16(buf) => self.copy_planar_typed(&buf),
            AudioBufferRef::S24(buf) => self.copy_planar_typed(&buf),
            AudioBufferRef::S32(buf) => self.copy_planar_typed(&buf),
            AudioBufferRef::F32(buf) => self.copy_planar_typed(&buf),
            AudioBufferRef::F64(buf) => self.copy_planar_typed(&buf),
        }
    }

    /// Copies all audio data from a source `AudioBuffer` into the `SampleBuffer` in planar
    /// channel order. The two buffers must be equivalent.
    pub fn copy_planar_typed<F>(&mut self, src: &AudioBuffer<F>)
    where
        F: Sample + IntoSample<S>,
    {
        let n_frames = src.frames();
        let n_channels = src.spec.channels.count();
        let n_samples = n_frames * n_channels;

        // Ensure that the capacity of the sample buffer is greater than or equal to the number
        // of samples that will be copied from the source buffer.
        assert!(self.capacity() >= n_samples);

        for ch in 0..n_channels {
            let ch_slice = src.chan(ch);

            for (dst, src) in self.buf[ch * n_frames..].iter_mut().zip(ch_slice) {
                *dst = (*src).into_sample();
            }
        }

        // Commit the written samples.
        self.n_written = n_samples;
    }

    /// Copies all audio data from the source `AudioBufferRef` in interleaved channel order into the
    /// `SampleBuffer`. The two buffers must be equivalent.
    pub fn copy_interleaved_ref(&mut self, src: AudioBufferRef)
    where
        S: ConvertibleSample,
    {
        match src {
            AudioBufferRef::U8(buf) => self.copy_interleaved_typed(&buf),
            AudioBufferRef::U16(buf) => self.copy_interleaved_typed(&buf),
            AudioBufferRef::U24(buf) => self.copy_interleaved_typed(&buf),
            AudioBufferRef::U32(buf) => self.copy_interleaved_typed(&buf),
            AudioBufferRef::S8(buf) => self.copy_interleaved_typed(&buf),
            AudioBufferRef::S16(buf) => self.copy_interleaved_typed(&buf),
            AudioBufferRef::S24(buf) => self.copy_interleaved_typed(&buf),
            AudioBufferRef::S32(buf) => self.copy_interleaved_typed(&buf),
            AudioBufferRef::F32(buf) => self.copy_interleaved_typed(&buf),
            AudioBufferRef::F64(buf) => self.copy_interleaved_typed(&buf),
        }
    }

    /// Copies all audio samples from a source `AudioBuffer` into the `SampleBuffer` in interleaved
    /// channel order. The two buffers must be equivalent.
    pub fn copy_interleaved_typed<F>(&mut self, src: &AudioBuffer<F>)
    where
        F: Sample + IntoSample<S>,
    {
        let n_channels = src.spec.channels.count();
        let n_samples = src.frames() * n_channels;

        // Ensure that the capacity of the sample buffer is greater than or equal to the number
        // of samples that will be copied from the source buffer.
        assert!(self.capacity() >= n_samples);

        // Interleave the source buffer channels into the sample buffer.
        for ch in 0..n_channels {
            let ch_slice = src.chan(ch);

            for (dst, src) in self.buf[ch..].iter_mut().step_by(n_channels).zip(ch_slice) {
                *dst = (*src).into_sample();
            }
        }

        // Commit the written samples.
        self.n_written = n_samples;
    }
}

/// This non-public module contains the trait `Sealed` which is used to constrain
/// `RawSample::RawType` with `bytemuck::Pod`. This is a trade-off to hide `bytemuck` from the public
/// interface. The downside is that `RawSample::RawType` is locked to the types we implement
/// `Sealed` on. To compensate, we implement `Sealed` on all primitive numeric data types, and byte
/// arrays up to 8 bytes long.
mod sealed {
    pub trait Sealed: bytemuck::Pod {}
}

impl sealed::Sealed for u8 {}
impl sealed::Sealed for i8 {}
impl sealed::Sealed for u16 {}
impl sealed::Sealed for i16 {}
impl sealed::Sealed for u32 {}
impl sealed::Sealed for i32 {}
impl sealed::Sealed for u64 {}
impl sealed::Sealed for i64 {}
impl sealed::Sealed for f32 {}
impl sealed::Sealed for f64 {}
impl sealed::Sealed for [u8; 1] {}
impl sealed::Sealed for [u8; 2] {}
impl sealed::Sealed for [u8; 3] {}
impl sealed::Sealed for [u8; 4] {}
impl sealed::Sealed for [u8; 5] {}
impl sealed::Sealed for [u8; 6] {}
impl sealed::Sealed for [u8; 7] {}
impl sealed::Sealed for [u8; 8] {}

/// `RawSample` provides a typed interface for converting a `Sample` from it's in-memory data type
/// to actual binary type.
pub trait RawSample: Sample {
    /// The `RawType` is a primitive data type, or fixed-size byte array, that is the final binary
    /// representation of the sample when written out to a byte-buffer.
    type RawType: Copy + Default + sealed::Sealed;

    fn into_raw_sample(self) -> Self::RawType;
}

impl RawSample for u8 {
    type RawType = u8;

    #[inline(always)]
    fn into_raw_sample(self) -> Self::RawType {
        self
    }
}

impl RawSample for i8 {
    type RawType = i8;

    #[inline(always)]
    fn into_raw_sample(self) -> Self::RawType {
        self
    }
}

impl RawSample for u16 {
    type RawType = u16;

    #[inline(always)]
    fn into_raw_sample(self) -> Self::RawType {
        self
    }
}

impl RawSample for i16 {
    type RawType = i16;

    #[inline(always)]
    fn into_raw_sample(self) -> Self::RawType {
        self
    }
}

impl RawSample for u24 {
    type RawType = [u8; 3];

    #[inline(always)]
    fn into_raw_sample(self) -> Self::RawType {
        self.to_ne_bytes()
    }
}

impl RawSample for i24 {
    type RawType = [u8; 3];

    #[inline(always)]
    fn into_raw_sample(self) -> Self::RawType {
        self.to_ne_bytes()
    }
}

impl RawSample for u32 {
    type RawType = u32;

    #[inline(always)]
    fn into_raw_sample(self) -> Self::RawType {
        self
    }
}

impl RawSample for i32 {
    type RawType = i32;

    #[inline(always)]
    fn into_raw_sample(self) -> Self::RawType {
        self
    }
}

impl RawSample for f32 {
    type RawType = f32;

    #[inline(always)]
    fn into_raw_sample(self) -> Self::RawType {
        self
    }
}

impl RawSample for f64 {
    type RawType = f64;

    #[inline(always)]
    fn into_raw_sample(self) -> Self::RawType {
        self
    }
}

/// A `RawSampleBuffer`, is a byte-oriented sample buffer. All samples copied to this buffer are
/// converted into their packed data-type and stored as a stream of bytes. `RawSampleBuffer` is
/// mean't for safely importing and exporting sample data to and from Symphonia as raw bytes.
pub struct RawSampleBuffer<S: Sample + RawSample> {
    buf: Box<[S::RawType]>,
    n_written: usize,
    // Might take your heart.
    sample_format: PhantomData<S>,
}

impl<S: Sample + RawSample> RawSampleBuffer<S> {
    /// Instantiate a new `RawSampleBuffer` using the specified signal specification and of the given
    /// duration.
    pub fn new(duration: Duration, spec: SignalSpec) -> RawSampleBuffer<S> {
        // The number of channels * duration cannot exceed u64::MAX.
        assert!(duration <= u64::MAX / spec.channels.count() as u64, "duration too large");

        // The total number of samples the buffer will store.
        let n_samples = duration * spec.channels.count() as u64;

        // Practically speaking, it is not possible to allocate more than usize::MAX bytes of raw
        // samples. This assertion ensures the potential downcast of n_samples to usize below is
        // safe.
        assert!(
            n_samples <= (usize::MAX / mem::size_of::<S::RawType>()) as u64,
            "duration too large"
        );

        // Allocate enough memory for all the samples and fill the buffer with silence.
        let buf = vec![S::MID.into_raw_sample(); n_samples as usize].into_boxed_slice();

        RawSampleBuffer { buf, n_written: 0, sample_format: PhantomData }
    }

    /// Gets the number of written samples.
    pub fn len(&self) -> usize {
        self.n_written
    }

    /// Returns `true` if the buffer contains no written samples.
    pub fn is_empty(&self) -> bool {
        self.n_written == 0
    }

    /// Gets the maximum number of samples the `RawSampleBuffer` may store.
    pub fn capacity(&self) -> usize {
        self.buf.len()
    }

    /// Clears all written samples.
    pub fn clear(&mut self) {
        self.n_written = 0;
    }

    /// Gets an immutable slice to the bytes of the sample's written in the `RawSampleBuffer`.
    pub fn as_bytes(&self) -> &[u8] {
        // Get a slice to the written raw samples in the buffer, and convert from &[RawType] to
        // &[u8]. Since &[u8] has the least strict alignment requirements, this should always be
        // safe and therefore cast_slice should never panic.
        bytemuck::cast_slice(&self.buf[..self.n_written])
    }

    /// Copies all audio data from the source `AudioBufferRef` in planar channel order into the
    /// `RawSampleBuffer`. The two buffers must be equivalent.
    pub fn copy_planar_ref(&mut self, src: AudioBufferRef)
    where
        S: ConvertibleSample,
    {
        match src {
            AudioBufferRef::U8(buf) => self.copy_planar_typed(&buf),
            AudioBufferRef::U16(buf) => self.copy_planar_typed(&buf),
            AudioBufferRef::U24(buf) => self.copy_planar_typed(&buf),
            AudioBufferRef::U32(buf) => self.copy_planar_typed(&buf),
            AudioBufferRef::S8(buf) => self.copy_planar_typed(&buf),
            AudioBufferRef::S16(buf) => self.copy_planar_typed(&buf),
            AudioBufferRef::S24(buf) => self.copy_planar_typed(&buf),
            AudioBufferRef::S32(buf) => self.copy_planar_typed(&buf),
            AudioBufferRef::F32(buf) => self.copy_planar_typed(&buf),
            AudioBufferRef::F64(buf) => self.copy_planar_typed(&buf),
        }
    }

    /// Copies all audio data from a source `AudioBuffer` that is of a different sample format type
    /// than that of the `RawSampleBuffer` in planar channel order. The two buffers must be
    /// equivalent.
    pub fn copy_planar_typed<F>(&mut self, src: &AudioBuffer<F>)
    where
        F: Sample + IntoSample<S>,
    {
        let n_channels = src.spec.channels.count();
        let n_samples = n_channels * src.n_frames;

        // Ensure that the capacity of the sample buffer is greater than or equal to the number
        // of samples that will be copied from the source buffer.
        assert!(self.capacity() >= n_samples);

        let dst_buf = &mut self.buf[..n_samples];

        for (ch, dst_ch) in dst_buf.chunks_exact_mut(src.n_frames).enumerate() {
            let src_ch = src.chan(ch);

            for (&s, d) in src_ch.iter().zip(dst_ch) {
                *d = s.into_sample().into_raw_sample();
            }
        }

        self.n_written = n_samples;
    }

    /// Copies all audio data from the source `AudioBuffer` to the `RawSampleBuffer` in planar order.
    /// The two buffers must be equivalent.
    pub fn copy_planar(&mut self, src: &AudioBuffer<S>) {
        let n_channels = src.spec.channels.count();
        let n_samples = src.n_frames * n_channels;

        // Ensure that the capacity of the sample buffer is greater than or equal to the number
        // of samples that will be copied from the source buffer.
        assert!(self.capacity() >= n_samples);

        let dst_buf = &mut self.buf[..n_samples];

        for (ch, dst_ch) in dst_buf.chunks_exact_mut(src.n_frames).enumerate() {
            let src_ch = src.chan(ch);

            for (&s, d) in src_ch.iter().zip(dst_ch) {
                *d = s.into_raw_sample();
            }
        }

        self.n_written = n_samples;
    }

    /// Copies all audio data from the source `AudioBufferRef` in interleaved channel order into the
    /// `RawSampleBuffer`. The two buffers must be equivalent.
    pub fn copy_interleaved_ref(&mut self, src: AudioBufferRef)
    where
        S: ConvertibleSample,
    {
        match src {
            AudioBufferRef::U8(buf) => self.copy_interleaved_typed(&buf),
            AudioBufferRef::U16(buf) => self.copy_interleaved_typed(&buf),
            AudioBufferRef::U24(buf) => self.copy_interleaved_typed(&buf),
            AudioBufferRef::U32(buf) => self.copy_interleaved_typed(&buf),
            AudioBufferRef::S8(buf) => self.copy_interleaved_typed(&buf),
            AudioBufferRef::S16(buf) => self.copy_interleaved_typed(&buf),
            AudioBufferRef::S24(buf) => self.copy_interleaved_typed(&buf),
            AudioBufferRef::S32(buf) => self.copy_interleaved_typed(&buf),
            AudioBufferRef::F32(buf) => self.copy_interleaved_typed(&buf),
            AudioBufferRef::F64(buf) => self.copy_interleaved_typed(&buf),
        }
    }

    /// Copies all audio data from a source `AudioBuffer` that is of a different sample format type
    /// than that of the `RawSampleBuffer` in interleaved channel order. The two buffers must be
    /// equivalent.
    pub fn copy_interleaved_typed<F>(&mut self, src: &AudioBuffer<F>)
    where
        F: Sample + IntoSample<S>,
    {
        let n_frames = src.n_frames;
        let n_channels = src.spec.channels.count();
        let n_samples = n_frames * n_channels;

        // Ensure that the capacity of the sample buffer is greater than or equal to the number
        // of samples that will be copied from the source buffer.
        assert!(self.capacity() >= n_samples);

        // The destination buffer slice.
        let dst_buf = &mut self.buf[..n_samples];

        // Provide slightly optimized interleave algorithms for Mono and Stereo buffers.
        match n_channels {
            // No channels, do nothing.
            0 => (),
            // Mono
            1 => {
                for (&s, d) in src.chan(0).iter().zip(dst_buf) {
                    *d = s.into_sample().into_raw_sample();
                }
            }
            // Stereo
            2 => {
                let l_buf = src.chan(0);
                let r_buf = src.chan(1);

                for ((&l, &r), d) in l_buf.iter().zip(r_buf).zip(dst_buf.chunks_exact_mut(2)) {
                    d[0] = l.into_sample().into_raw_sample();
                    d[1] = r.into_sample().into_raw_sample();
                }
            }
            // 3+ channels
            _ => {
                for ch in 0..n_channels {
                    let src_ch = src.chan(ch);
                    let dst_ch_iter = dst_buf[ch..].iter_mut().step_by(n_channels);

                    for (&s, d) in src_ch.iter().zip(dst_ch_iter) {
                        *d = s.into_sample().into_raw_sample();
                    }
                }
            }
        }

        self.n_written = n_samples;
    }

    /// Copies all audio data from the source `AudioBuffer` to the `RawSampleBuffer` in interleaved
    /// channel order. The two buffers must be equivalent.
    pub fn copy_interleaved(&mut self, src: &AudioBuffer<S>) {
        let n_frames = src.n_frames;
        let n_channels = src.spec.channels.count();
        let n_samples = n_frames * n_channels;

        // Ensure that the capacity of the sample buffer is greater than or equal to the number
        // of samples that will be copied from the source buffer.
        assert!(self.capacity() >= n_samples);

        // The destination buffer slice.
        let dst_buf = &mut self.buf[..n_samples];

        // Provide slightly optimized interleave algorithms for Mono and Stereo buffers.
        match n_channels {
            // No channels, do nothing.
            0 => (),
            // Mono
            1 => {
                for (&s, d) in src.chan(0).iter().zip(dst_buf) {
                    *d = s.into_raw_sample();
                }
            }
            // Stereo
            2 => {
                let l_buf = src.chan(0);
                let r_buf = src.chan(1);

                for ((&l, &r), d) in l_buf.iter().zip(r_buf).zip(dst_buf.chunks_exact_mut(2)) {
                    d[0] = l.into_raw_sample();
                    d[1] = r.into_raw_sample();
                }
            }
            // 3+ channels
            _ => {
                for ch in 0..n_channels {
                    let src_ch = src.chan(ch);
                    let dst_ch_iter = dst_buf[ch..].iter_mut().step_by(n_channels);

                    for (&s, d) in src_ch.iter().zip(dst_ch_iter) {
                        *d = s.into_raw_sample();
                    }
                }
            }
        }

        self.n_written = n_samples;
    }
}
