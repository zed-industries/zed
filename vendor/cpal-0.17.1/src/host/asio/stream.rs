extern crate asio_sys as sys;
extern crate num_traits;

use crate::I24;

use self::num_traits::PrimInt;
use super::Device;
use crate::{
    BackendSpecificError, BufferSize, BuildStreamError, Data, InputCallbackInfo,
    OutputCallbackInfo, PauseStreamError, PlayStreamError, SampleFormat, StreamConfig, StreamError,
};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;

pub struct Stream {
    playing: Arc<AtomicBool>,
    // Ensure the `Driver` does not terminate until the last stream is dropped.
    driver: Arc<sys::Driver>,
    #[allow(dead_code)]
    asio_streams: Arc<Mutex<sys::AsioStreams>>,
    callback_id: sys::CallbackId,
    message_callback_id: sys::MessageCallbackId,
}

// Compile-time assertion that Stream is Send and Sync
crate::assert_stream_send!(Stream);
crate::assert_stream_sync!(Stream);

impl Stream {
    pub fn play(&self) -> Result<(), PlayStreamError> {
        self.playing.store(true, Ordering::SeqCst);
        Ok(())
    }

    pub fn pause(&self) -> Result<(), PauseStreamError> {
        self.playing.store(false, Ordering::SeqCst);
        Ok(())
    }
}

impl Device {
    pub fn build_input_stream_raw<D, E>(
        &self,
        config: &StreamConfig,
        sample_format: SampleFormat,
        mut data_callback: D,
        error_callback: E,
        _timeout: Option<Duration>,
    ) -> Result<Stream, BuildStreamError>
    where
        D: FnMut(&Data, &InputCallbackInfo) + Send + 'static,
        E: FnMut(StreamError) + Send + 'static,
    {
        let stream_type = self.driver.input_data_type().map_err(build_stream_err)?;

        // Ensure that the desired sample type is supported.
        let expected_sample_format = super::device::convert_data_type(&stream_type)
            .ok_or(BuildStreamError::StreamConfigNotSupported)?;
        if sample_format != expected_sample_format {
            return Err(BuildStreamError::StreamConfigNotSupported);
        }

        // Register the message callback with the driver
        let message_callback_id = self.add_message_callback(error_callback);

        let num_channels = config.channels;
        let buffer_size = self.get_or_create_input_stream(config, sample_format)?;
        let cpal_num_samples = buffer_size * num_channels as usize;

        // Create the buffer depending on the size of the data type.
        let len_bytes = cpal_num_samples * sample_format.sample_size();
        let mut interleaved = vec![0u8; len_bytes];

        let stream_playing = Arc::new(AtomicBool::new(false));
        let playing = Arc::clone(&stream_playing);
        let asio_streams = self.asio_streams.clone();

        // Set the input callback.
        // This is most performance critical part of the ASIO bindings.
        let config = config.clone();
        let callback_id = self.driver.add_callback(move |callback_info| unsafe {
            // If not playing return early.
            if !playing.load(Ordering::SeqCst) {
                return;
            }

            // There is 0% chance of lock contention the host only locks when recreating streams.
            let stream_lock = asio_streams.lock().unwrap();
            let asio_stream = match stream_lock.input {
                Some(ref asio_stream) => asio_stream,
                None => return,
            };

            /// 1. Write from the ASIO buffer to the interleaved CPAL buffer.
            /// 2. Deliver the CPAL buffer to the user callback.
            unsafe fn process_input_callback<A, D, F>(
                data_callback: &mut D,
                interleaved: &mut [u8],
                asio_stream: &sys::AsioStream,
                asio_info: &sys::CallbackInfo,
                sample_rate: crate::SampleRate,
                format: SampleFormat,
                from_endianness: F,
            ) where
                A: Copy,
                D: FnMut(&Data, &InputCallbackInfo) + Send + 'static,
                F: Fn(A) -> A,
            {
                // 1. Write the ASIO channels to the CPAL buffer.
                let interleaved: &mut [A] = cast_slice_mut(interleaved);
                let n_frames = asio_stream.buffer_size as usize;
                let n_channels = interleaved.len() / n_frames;
                let buffer_index = asio_info.buffer_index as usize;
                for ch_ix in 0..n_channels {
                    let asio_channel =
                        asio_channel_slice::<A>(asio_stream, buffer_index, ch_ix, None);
                    for (frame, s_asio) in interleaved.chunks_mut(n_channels).zip(asio_channel) {
                        frame[ch_ix] = from_endianness(*s_asio);
                    }
                }

                // 2. Deliver the interleaved buffer to the callback.
                apply_input_callback_to_data::<A, _>(
                    data_callback,
                    interleaved,
                    asio_stream,
                    asio_info,
                    sample_rate,
                    format,
                );
            }

            match (&stream_type, sample_format) {
                (&sys::AsioSampleType::ASIOSTInt16LSB, SampleFormat::I16) => {
                    process_input_callback::<i16, _, _>(
                        &mut data_callback,
                        &mut interleaved,
                        asio_stream,
                        callback_info,
                        config.sample_rate,
                        SampleFormat::I16,
                        from_le,
                    );
                }
                (&sys::AsioSampleType::ASIOSTInt16MSB, SampleFormat::I16) => {
                    process_input_callback::<i16, _, _>(
                        &mut data_callback,
                        &mut interleaved,
                        asio_stream,
                        callback_info,
                        config.sample_rate,
                        SampleFormat::I16,
                        from_be,
                    );
                }

                (&sys::AsioSampleType::ASIOSTFloat32LSB, SampleFormat::F32) => {
                    process_input_callback::<u32, _, _>(
                        &mut data_callback,
                        &mut interleaved,
                        asio_stream,
                        callback_info,
                        config.sample_rate,
                        SampleFormat::F32,
                        from_le,
                    );
                }
                (&sys::AsioSampleType::ASIOSTFloat32MSB, SampleFormat::F32) => {
                    process_input_callback::<u32, _, _>(
                        &mut data_callback,
                        &mut interleaved,
                        asio_stream,
                        callback_info,
                        config.sample_rate,
                        SampleFormat::F32,
                        from_be,
                    );
                }

                (&sys::AsioSampleType::ASIOSTInt32LSB, SampleFormat::I32) => {
                    process_input_callback::<i32, _, _>(
                        &mut data_callback,
                        &mut interleaved,
                        asio_stream,
                        callback_info,
                        config.sample_rate,
                        SampleFormat::I32,
                        from_le,
                    );
                }
                (&sys::AsioSampleType::ASIOSTInt32MSB, SampleFormat::I32) => {
                    process_input_callback::<i32, _, _>(
                        &mut data_callback,
                        &mut interleaved,
                        asio_stream,
                        callback_info,
                        config.sample_rate,
                        SampleFormat::I32,
                        from_be,
                    );
                }

                (&sys::AsioSampleType::ASIOSTFloat64LSB, SampleFormat::F64) => {
                    process_input_callback::<u64, _, _>(
                        &mut data_callback,
                        &mut interleaved,
                        asio_stream,
                        callback_info,
                        config.sample_rate,
                        SampleFormat::F64,
                        from_le,
                    );
                }
                (&sys::AsioSampleType::ASIOSTFloat64MSB, SampleFormat::F64) => {
                    process_input_callback::<u64, _, _>(
                        &mut data_callback,
                        &mut interleaved,
                        asio_stream,
                        callback_info,
                        config.sample_rate,
                        SampleFormat::F64,
                        from_be,
                    );
                }

                (&sys::AsioSampleType::ASIOSTInt24LSB, SampleFormat::I24) => {
                    process_input_callback_i24(
                        &mut data_callback,
                        &mut interleaved,
                        asio_stream,
                        callback_info,
                        config.sample_rate,
                        true,
                    );
                }
                (&sys::AsioSampleType::ASIOSTInt24MSB, SampleFormat::I24) => {
                    process_input_callback_i24(
                        &mut data_callback,
                        &mut interleaved,
                        asio_stream,
                        callback_info,
                        config.sample_rate,
                        false,
                    );
                }

                unsupported_format_pair => unreachable!(
                    "`build_input_stream_raw` should have returned with unsupported \
                     format {:?}",
                    unsupported_format_pair
                ),
            }
        });

        let driver = self.driver.clone();
        let asio_streams = self.asio_streams.clone();

        // Immediately start the device?
        self.driver.start().map_err(build_stream_err)?;

        Ok(Stream {
            playing: stream_playing,
            driver,
            asio_streams,
            callback_id,
            message_callback_id,
        })
    }

    pub fn build_output_stream_raw<D, E>(
        &self,
        config: &StreamConfig,
        sample_format: SampleFormat,
        mut data_callback: D,
        error_callback: E,
        _timeout: Option<Duration>,
    ) -> Result<Stream, BuildStreamError>
    where
        D: FnMut(&mut Data, &OutputCallbackInfo) + Send + 'static,
        E: FnMut(StreamError) + Send + 'static,
    {
        let stream_type = self.driver.output_data_type().map_err(build_stream_err)?;

        // Ensure that the desired sample type is supported.
        let expected_sample_format = super::device::convert_data_type(&stream_type)
            .ok_or(BuildStreamError::StreamConfigNotSupported)?;
        if sample_format != expected_sample_format {
            return Err(BuildStreamError::StreamConfigNotSupported);
        }

        // Register the message callback with the driver
        let message_callback_id = self.add_message_callback(error_callback);

        let num_channels = config.channels;
        let buffer_size = self.get_or_create_output_stream(config, sample_format)?;
        let cpal_num_samples = buffer_size * num_channels as usize;

        // Create buffers depending on data type.
        let len_bytes = cpal_num_samples * sample_format.sample_size();
        let mut interleaved = vec![0u8; len_bytes];
        let current_callback_flag = self.current_callback_flag.clone();

        let stream_playing = Arc::new(AtomicBool::new(false));
        let playing = Arc::clone(&stream_playing);
        let asio_streams = self.asio_streams.clone();

        let config = config.clone();
        let callback_id = self.driver.add_callback(move |callback_info| unsafe {
            // If not playing, return early.
            if !playing.load(Ordering::SeqCst) {
                return;
            }

            // There is 0% chance of lock contention the host only locks when recreating streams.
            let mut stream_lock = asio_streams.lock().unwrap();
            let asio_stream = match stream_lock.output {
                Some(ref mut asio_stream) => asio_stream,
                None => return,
            };

            // Silence the ASIO buffer that is about to be used.
            //
            // Check if any other callbacks have already silenced the buffer associated with
            // the current callback. The flag is updated once per buffer switch.
            let silence =
                current_callback_flag.load(Ordering::Acquire) != callback_info.callback_flag;

            if silence {
                current_callback_flag.store(callback_info.callback_flag, Ordering::Release);
            }

            /// 1. Render the given callback to the given buffer of interleaved samples.
            /// 2. If required, silence the ASIO buffer.
            /// 3. Finally, write the interleaved data to the non-interleaved ASIO buffer,
            ///    performing endianness conversions as necessary.
            #[allow(clippy::too_many_arguments)]
            unsafe fn process_output_callback<A, D, F>(
                data_callback: &mut D,
                interleaved: &mut [u8],
                silence_asio_buffer: bool,
                asio_stream: &mut sys::AsioStream,
                asio_info: &sys::CallbackInfo,
                sample_rate: crate::SampleRate,
                format: SampleFormat,
                mix_samples: F,
            ) where
                A: Copy,
                D: FnMut(&mut Data, &OutputCallbackInfo) + Send + 'static,
                F: Fn(A, A) -> A,
            {
                let interleaved: &mut [A] = cast_slice_mut(interleaved);
                apply_output_callback_to_data::<A, _>(
                    data_callback,
                    interleaved,
                    asio_stream,
                    asio_info,
                    sample_rate,
                    format,
                );
                let n_channels = interleaved.len() / asio_stream.buffer_size as usize;
                let buffer_index = asio_info.buffer_index as usize;

                // Write interleaved samples to ASIO channels, one channel at a time.
                for ch_ix in 0..n_channels {
                    let asio_channel =
                        asio_channel_slice_mut::<A>(asio_stream, buffer_index, ch_ix, None);
                    if silence_asio_buffer {
                        asio_channel.align_to_mut::<u8>().1.fill(0);
                    }
                    for (frame, s_asio) in interleaved.chunks(n_channels).zip(asio_channel) {
                        *s_asio = mix_samples(*s_asio, frame[ch_ix]);
                    }
                }
            }

            match (sample_format, &stream_type) {
                (SampleFormat::I16, &sys::AsioSampleType::ASIOSTInt16LSB) => {
                    process_output_callback::<i16, _, _>(
                        &mut data_callback,
                        &mut interleaved,
                        silence,
                        asio_stream,
                        callback_info,
                        config.sample_rate,
                        SampleFormat::I16,
                        |old_sample, new_sample| {
                            from_le(old_sample).saturating_add(new_sample).to_le()
                        },
                    );
                }
                (SampleFormat::I16, &sys::AsioSampleType::ASIOSTInt16MSB) => {
                    process_output_callback::<i16, _, _>(
                        &mut data_callback,
                        &mut interleaved,
                        silence,
                        asio_stream,
                        callback_info,
                        config.sample_rate,
                        SampleFormat::I16,
                        |old_sample, new_sample| {
                            from_be(old_sample).saturating_add(new_sample).to_be()
                        },
                    );
                }
                (SampleFormat::F32, &sys::AsioSampleType::ASIOSTFloat32LSB) => {
                    process_output_callback::<u32, _, _>(
                        &mut data_callback,
                        &mut interleaved,
                        silence,
                        asio_stream,
                        callback_info,
                        config.sample_rate,
                        SampleFormat::F32,
                        |old_sample, new_sample| {
                            (f32::from_bits(from_le(old_sample)) + f32::from_bits(new_sample))
                                .to_bits()
                                .to_le()
                        },
                    );
                }

                (SampleFormat::F32, &sys::AsioSampleType::ASIOSTFloat32MSB) => {
                    process_output_callback::<u32, _, _>(
                        &mut data_callback,
                        &mut interleaved,
                        silence,
                        asio_stream,
                        callback_info,
                        config.sample_rate,
                        SampleFormat::F32,
                        |old_sample, new_sample| {
                            (f32::from_bits(from_be(old_sample)) + f32::from_bits(new_sample))
                                .to_bits()
                                .to_be()
                        },
                    );
                }

                (SampleFormat::I32, &sys::AsioSampleType::ASIOSTInt32LSB) => {
                    process_output_callback::<i32, _, _>(
                        &mut data_callback,
                        &mut interleaved,
                        silence,
                        asio_stream,
                        callback_info,
                        config.sample_rate,
                        SampleFormat::I32,
                        |old_sample, new_sample| {
                            from_le(old_sample).saturating_add(new_sample).to_le()
                        },
                    );
                }
                (SampleFormat::I32, &sys::AsioSampleType::ASIOSTInt32MSB) => {
                    process_output_callback::<i32, _, _>(
                        &mut data_callback,
                        &mut interleaved,
                        silence,
                        asio_stream,
                        callback_info,
                        config.sample_rate,
                        SampleFormat::I32,
                        |old_sample, new_sample| {
                            from_be(old_sample).saturating_add(new_sample).to_be()
                        },
                    );
                }

                (SampleFormat::F64, &sys::AsioSampleType::ASIOSTFloat64LSB) => {
                    process_output_callback::<u64, _, _>(
                        &mut data_callback,
                        &mut interleaved,
                        silence,
                        asio_stream,
                        callback_info,
                        config.sample_rate,
                        SampleFormat::F64,
                        |old_sample, new_sample| {
                            (f64::from_bits(from_le(old_sample)) + f64::from_bits(new_sample))
                                .to_bits()
                                .to_le()
                        },
                    );
                }

                (SampleFormat::F64, &sys::AsioSampleType::ASIOSTFloat64MSB) => {
                    process_output_callback::<u64, _, _>(
                        &mut data_callback,
                        &mut interleaved,
                        silence,
                        asio_stream,
                        callback_info,
                        config.sample_rate,
                        SampleFormat::F64,
                        |old_sample, new_sample| {
                            (f64::from_bits(from_be(old_sample)) + f64::from_bits(new_sample))
                                .to_bits()
                                .to_be()
                        },
                    );
                }

                (SampleFormat::I24, &sys::AsioSampleType::ASIOSTInt24LSB) => {
                    process_output_callback_i24::<_>(
                        &mut data_callback,
                        &mut interleaved,
                        silence,
                        true,
                        asio_stream,
                        callback_info,
                        config.sample_rate,
                    );
                }

                (SampleFormat::I24, &sys::AsioSampleType::ASIOSTInt24MSB) => {
                    process_output_callback_i24::<_>(
                        &mut data_callback,
                        &mut interleaved,
                        silence,
                        false,
                        asio_stream,
                        callback_info,
                        config.sample_rate,
                    );
                }

                unsupported_format_pair => unreachable!(
                    "`build_output_stream_raw` should have returned with unsupported \
                     format {:?}",
                    unsupported_format_pair
                ),
            }
        });

        let driver = self.driver.clone();
        let asio_streams = self.asio_streams.clone();

        // Immediately start the device?
        self.driver.start().map_err(build_stream_err)?;

        Ok(Stream {
            playing: stream_playing,
            driver,
            asio_streams,
            callback_id,
            message_callback_id,
        })
    }

    /// Create a new CPAL Input Stream.
    ///
    /// If there is no existing ASIO Input Stream it will be created.
    ///
    /// On success, the buffer size of the stream is returned.
    fn get_or_create_input_stream(
        &self,
        config: &StreamConfig,
        sample_format: SampleFormat,
    ) -> Result<usize, BuildStreamError> {
        match self.default_input_config() {
            Ok(f) => {
                let num_asio_channels = f.channels;
                check_config(&self.driver, config, sample_format, num_asio_channels)
            }
            Err(_) => Err(BuildStreamError::StreamConfigNotSupported),
        }?;
        let num_channels = config.channels as usize;
        let mut streams = self.asio_streams.lock().unwrap();

        let buffer_size = match config.buffer_size {
            BufferSize::Fixed(v) => Some(v as i32),
            BufferSize::Default => None,
        };

        // Either create a stream if thers none or had back the
        // size of the current one.
        match streams.input {
            Some(ref input) => Ok(input.buffer_size as usize),
            None => {
                let output = streams.output.take();
                self.driver
                    .prepare_input_stream(output, num_channels, buffer_size)
                    .map(|new_streams| {
                        let bs = match new_streams.input {
                            Some(ref inp) => inp.buffer_size as usize,
                            None => unreachable!(),
                        };
                        *streams = new_streams;
                        bs
                    })
                    .map_err(|ref e| {
                        println!("Error preparing stream: {}", e);
                        BuildStreamError::DeviceNotAvailable
                    })
            }
        }
    }

    /// Create a new CPAL Output Stream.
    ///
    /// If there is no existing ASIO Output Stream it will be created.
    fn get_or_create_output_stream(
        &self,
        config: &StreamConfig,
        sample_format: SampleFormat,
    ) -> Result<usize, BuildStreamError> {
        match self.default_output_config() {
            Ok(f) => {
                let num_asio_channels = f.channels;
                check_config(&self.driver, config, sample_format, num_asio_channels)
            }
            Err(_) => Err(BuildStreamError::StreamConfigNotSupported),
        }?;
        let num_channels = config.channels as usize;
        let mut streams = self.asio_streams.lock().unwrap();

        let buffer_size = match config.buffer_size {
            BufferSize::Fixed(v) => Some(v as i32),
            BufferSize::Default => None,
        };

        // Either create a stream if thers none or had back the
        // size of the current one.
        match streams.output {
            Some(ref output) => Ok(output.buffer_size as usize),
            None => {
                let input = streams.input.take();
                self.driver
                    .prepare_output_stream(input, num_channels, buffer_size)
                    .map(|new_streams| {
                        let bs = match new_streams.output {
                            Some(ref out) => out.buffer_size as usize,
                            None => unreachable!(),
                        };
                        *streams = new_streams;
                        bs
                    })
                    .map_err(|ref e| {
                        println!("Error preparing stream: {}", e);
                        BuildStreamError::DeviceNotAvailable
                    })
            }
        }
    }

    fn add_message_callback<E>(&self, error_callback: E) -> sys::MessageCallbackId
    where
        E: FnMut(StreamError) + Send + 'static,
    {
        let error_callback_shared = Arc::new(Mutex::new(error_callback));

        self.driver.add_message_callback(move |msg| {
            // Check specifically for ResetRequest
            if let sys::AsioMessageSelectors::kAsioResetRequest = msg {
                if let Ok(mut cb) = error_callback_shared.lock() {
                    cb(StreamError::StreamInvalidated);
                }
            }
        })
    }
}

impl Drop for Stream {
    fn drop(&mut self) {
        self.driver.remove_callback(self.callback_id);
        self.driver
            .remove_message_callback(self.message_callback_id);
    }
}

fn asio_ns_to_double(val: sys::bindings::asio_import::ASIOTimeStamp) -> f64 {
    let two_raised_to_32 = 4294967296.0;
    val.lo as f64 + val.hi as f64 * two_raised_to_32
}

/// Asio retrieves system time via `timeGetTime` which returns the time in milliseconds.
fn system_time_to_stream_instant(
    system_time: sys::bindings::asio_import::ASIOTimeStamp,
) -> crate::StreamInstant {
    let systime_ns = asio_ns_to_double(system_time);
    let secs = systime_ns as i64 / 1_000_000_000;
    let nanos = (systime_ns as i64 - secs * 1_000_000_000) as u32;
    crate::StreamInstant::new(secs, nanos)
}

// Convert the given duration in frames at the given sample rate to a `std::time::Duration`.
#[inline]
fn frames_to_duration(frames: usize, rate: crate::SampleRate) -> std::time::Duration {
    let secsf = frames as f64 / rate as f64;
    let secs = secsf as u64;
    let nanos = ((secsf - secs as f64) * 1_000_000_000.0) as u32;
    std::time::Duration::new(secs, nanos)
}

/// Check whether or not the desired config is supported by the stream.
///
/// Checks sample rate, data type, number of channels, and buffer size.
fn check_config(
    driver: &sys::Driver,
    config: &StreamConfig,
    sample_format: SampleFormat,
    num_asio_channels: u16,
) -> Result<(), BuildStreamError> {
    let StreamConfig {
        channels,
        sample_rate,
        buffer_size,
    } = config;

    // Validate buffer size if `Fixed` is specified. This is necessary because ASIO's
    // `create_buffers` only validates the upper bound (returns `InvalidBufferSize` if > max) but
    // does NOT validate the lower bound. Passing a buffer size below min would be accepted but
    // behavior is unspecified.
    if let BufferSize::Fixed(requested_size) = buffer_size {
        let (min, max) = driver.buffersize_range().map_err(build_stream_err)?;
        let requested_size_i32 = *requested_size as i32;
        if !(min..=max).contains(&requested_size_i32) {
            return Err(BuildStreamError::StreamConfigNotSupported);
        }
    }

    // Try and set the sample rate to what the user selected.
    let sample_rate = (*sample_rate).into();
    if sample_rate != driver.sample_rate().map_err(build_stream_err)? {
        if driver
            .can_sample_rate(sample_rate)
            .map_err(build_stream_err)?
        {
            driver
                .set_sample_rate(sample_rate)
                .map_err(build_stream_err)?;
        } else {
            return Err(BuildStreamError::StreamConfigNotSupported);
        }
    }
    // unsigned formats are not supported by asio
    match sample_format {
        SampleFormat::I16 | SampleFormat::I24 | SampleFormat::I32 | SampleFormat::F32 => (),
        _ => return Err(BuildStreamError::StreamConfigNotSupported),
    }
    if *channels > num_asio_channels {
        return Err(BuildStreamError::StreamConfigNotSupported);
    }
    Ok(())
}

/// Cast a byte slice into a mutable slice of desired type.
///
/// Safety: it's up to the caller to ensure that the input slice has valid bit representations.
unsafe fn cast_slice_mut<T>(v: &mut [u8]) -> &mut [T] {
    debug_assert!(v.len() % std::mem::size_of::<T>() == 0);
    std::slice::from_raw_parts_mut(v.as_mut_ptr() as *mut T, v.len() / std::mem::size_of::<T>())
}

/// Helper function to convert from little endianness.
fn from_le<T: PrimInt>(t: T) -> T {
    T::from_le(t)
}

/// Helper function to convert from little endianness.
fn from_be<T: PrimInt>(t: T) -> T {
    T::from_be(t)
}

/// Shorthand for retrieving the asio buffer slice associated with a channel.
///
/// The channel length is automatically inferred from the buffer size or some
/// value can be passed to enforce a certain length (for odd sized sample formats)
unsafe fn asio_channel_slice<T>(
    asio_stream: &sys::AsioStream,
    buffer_index: usize,
    channel_index: usize,
    requested_channel_length: Option<usize>,
) -> &[T] {
    let channel_length = requested_channel_length.unwrap_or(asio_stream.buffer_size as usize);
    let buff_ptr: *const T =
        asio_stream.buffer_infos[channel_index].buffers[buffer_index] as *const _;
    std::slice::from_raw_parts(buff_ptr, channel_length)
}

/// Shorthand for retrieving the asio buffer slice associated with a channel.
///
/// The channel length is automatically inferred from the buffer size or some
/// value can be passed to enforce a certain length (for odd sized sample formats)
unsafe fn asio_channel_slice_mut<T>(
    asio_stream: &mut sys::AsioStream,
    buffer_index: usize,
    channel_index: usize,
    requested_channel_length: Option<usize>,
) -> &mut [T] {
    let channel_length = requested_channel_length.unwrap_or(asio_stream.buffer_size as usize);
    let buff_ptr: *mut T = asio_stream.buffer_infos[channel_index].buffers[buffer_index] as *mut _;
    std::slice::from_raw_parts_mut(buff_ptr, channel_length)
}

fn build_stream_err(e: sys::AsioError) -> BuildStreamError {
    match e {
        sys::AsioError::NoDrivers | sys::AsioError::HardwareMalfunction => {
            BuildStreamError::DeviceNotAvailable
        }
        sys::AsioError::InvalidInput | sys::AsioError::BadMode => BuildStreamError::InvalidArgument,
        err => {
            let description = format!("{}", err);
            BackendSpecificError { description }.into()
        }
    }
}

/// Convert i24 bytes to i32
fn i24_bytes_to_i32(i24_bytes: &[u8; 3], little_endian: bool) -> i32 {
    let sample = if little_endian {
        i32::from_le_bytes([i24_bytes[0], i24_bytes[1], i24_bytes[2], 0u8])
    } else {
        i32::from_le_bytes([i24_bytes[2], i24_bytes[1], i24_bytes[0], 0u8])
    };
    if sample & 0x800000 != 0 {
        sample | -0x1000000
    } else {
        sample
    }
}

unsafe fn process_output_callback_i24<D>(
    data_callback: &mut D,
    interleaved: &mut [u8],
    silence_asio_buffer: bool,
    little_endian: bool,
    asio_stream: &mut sys::AsioStream,
    asio_info: &sys::CallbackInfo,
    sample_rate: crate::SampleRate,
) where
    D: FnMut(&mut Data, &OutputCallbackInfo) + Send + 'static,
{
    let format = SampleFormat::I24;
    let interleaved: &mut [I24] = cast_slice_mut(interleaved);
    apply_output_callback_to_data::<I24, _>(
        data_callback,
        interleaved,
        asio_stream,
        asio_info,
        sample_rate,
        format,
    );

    // Size of samples in the ASIO buffer (has to be 3 in this case)
    let asio_sample_size_bytes = 3;
    let n_channels = interleaved.len() / asio_stream.buffer_size as usize;
    let buffer_index = asio_info.buffer_index as usize;

    // Write interleaved samples to ASIO channels, one channel at a time.
    for ch_ix in 0..n_channels {
        // Take channel as u8 array ([u8; 3] packets to represent i24)
        let asio_channel = asio_channel_slice_mut(
            asio_stream,
            buffer_index,
            ch_ix,
            Some(asio_stream.buffer_size as usize * asio_sample_size_bytes),
        );

        if silence_asio_buffer {
            asio_channel.align_to_mut::<u8>().1.fill(0);
        }

        // Fill in every channel from the interleaved vector
        for (channel_sample, sample_in_buffer) in asio_channel
            .chunks_mut(asio_sample_size_bytes)
            .zip(interleaved.iter().skip(ch_ix).step_by(n_channels))
        {
            // Add samples from buffer if no silence was applied, otherwise just overwrite
            let result = if silence_asio_buffer {
                sample_in_buffer.inner()
            } else {
                let sample = i24_bytes_to_i32(
                    &[channel_sample[0], channel_sample[1], channel_sample[2]],
                    little_endian,
                );
                (sample_in_buffer.inner() + sample).clamp(-8388608, 8388607)
            };
            let bytes = result.to_le_bytes();
            if little_endian {
                channel_sample[0] = bytes[0];
                channel_sample[1] = bytes[1];
                channel_sample[2] = bytes[2];
            } else {
                channel_sample[2] = bytes[0];
                channel_sample[1] = bytes[1];
                channel_sample[0] = bytes[2];
            }
        }
    }
}

unsafe fn process_input_callback_i24<D>(
    data_callback: &mut D,
    interleaved: &mut [u8],
    asio_stream: &sys::AsioStream,
    asio_info: &sys::CallbackInfo,
    sample_rate: crate::SampleRate,
    little_endian: bool,
) where
    D: FnMut(&Data, &InputCallbackInfo) + Send + 'static,
{
    let format = SampleFormat::I24;

    // 1. Write the ASIO channels to the CPAL buffer.
    let interleaved: &mut [I24] = cast_slice_mut(interleaved);
    let n_frames = asio_stream.buffer_size as usize;
    let n_channels = interleaved.len() / n_frames;
    let buffer_index = asio_info.buffer_index as usize;
    let asio_sample_size_bytes = 3;

    for ch_ix in 0..n_channels {
        let asio_channel = asio_channel_slice::<u8>(
            asio_stream,
            buffer_index,
            ch_ix,
            Some(n_frames * asio_sample_size_bytes),
        );
        for (channel_sample, sample_in_buffer) in asio_channel
            .chunks(asio_sample_size_bytes)
            .zip(interleaved.iter_mut().skip(ch_ix).step_by(n_channels))
        {
            let sample = i24_bytes_to_i32(
                &[channel_sample[0], channel_sample[1], channel_sample[2]],
                little_endian,
            );
            *sample_in_buffer = I24::new(sample).unwrap();
        }
    }

    // 2. Deliver the interleaved buffer to the callback.
    apply_input_callback_to_data::<I24, _>(
        data_callback,
        interleaved,
        asio_stream,
        asio_info,
        sample_rate,
        format,
    );
}

/// Apply the output callback to the interleaved buffer.
unsafe fn apply_output_callback_to_data<A, D>(
    data_callback: &mut D,
    interleaved: &mut [A],
    asio_stream: &mut sys::AsioStream,
    asio_info: &sys::CallbackInfo,
    sample_rate: crate::SampleRate,
    sample_format: SampleFormat,
) where
    A: Copy,
    D: FnMut(&mut Data, &OutputCallbackInfo) + Send + 'static,
{
    let mut data = Data::from_parts(
        interleaved.as_mut_ptr() as *mut (),
        interleaved.len(),
        sample_format,
    );
    let callback = system_time_to_stream_instant(asio_info.system_time);
    let delay = frames_to_duration(asio_stream.buffer_size as usize, sample_rate);
    let playback = callback
        .add(delay)
        .expect("`playback` occurs beyond representation supported by `StreamInstant`");
    let timestamp = crate::OutputStreamTimestamp { callback, playback };
    let info = OutputCallbackInfo { timestamp };
    data_callback(&mut data, &info);
}

/// Apply the input callback to the interleaved buffer.
unsafe fn apply_input_callback_to_data<A, D>(
    data_callback: &mut D,
    interleaved: &mut [A],
    asio_stream: &sys::AsioStream,
    asio_info: &sys::CallbackInfo,
    sample_rate: crate::SampleRate,
    format: SampleFormat,
) where
    A: Copy,
    D: FnMut(&Data, &InputCallbackInfo) + Send + 'static,
{
    let data = Data::from_parts(
        interleaved.as_mut_ptr() as *mut (),
        interleaved.len(),
        format,
    );
    let callback = system_time_to_stream_instant(asio_info.system_time);
    let delay = frames_to_duration(asio_stream.buffer_size as usize, sample_rate);
    let capture = callback
        .sub(delay)
        .expect("`capture` occurs before origin of alsa `StreamInstant`");
    let timestamp = crate::InputStreamTimestamp { callback, capture };
    let info = InputCallbackInfo { timestamp };
    data_callback(&data, &info);
}
