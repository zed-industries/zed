use std::{
    convert::TryInto,
    sync::{Arc, Mutex},
};

use crate::{Client, SpanLocation};

#[repr(u8)]
/// The API label associated with the given gpu context. The list here only includes
/// APIs that are currently supported by Tracy's own gpu implementations.
//
// Copied from `tracy-client-sys/tracy/common/TracyQueue.hpp:391`. Comment on enum states
// that the values are stable, due to potential serialization issues, so copying this enum
// shouldn't be a problem.
pub enum GpuContextType {
    /// Stand in for other types of contexts.
    Invalid = 0,
    /// An OpenGL context
    OpenGL = 1,
    /// A Vulkan context
    Vulkan = 2,
    /// An OpenCL context
    OpenCL = 3,
    /// A D3D12 context.
    Direct3D12 = 4,
    /// A D3D11 context.
    Direct3D11 = 5,
}

/// Context for creating gpu spans.
///
/// Generally corresponds to a single hardware queue.
///
/// The flow of creating and using gpu context generally looks like this:
///
/// ```rust,no_run
/// # let client = tracy_client::Client::start();
/// // The period of the gpu clock in nanoseconds, as provided by your GPU api.
/// // This value corresponds to 1GHz.
/// let period: f32 = 1_000_000_000.0;
///
/// // GPU API: Record writing a timestamp and resolve that to a mappable buffer.
/// // GPU API: Submit the command buffer writing the timestamp.
/// // GPU API: Immediately block until the submission is finished.
/// // GPU API: Map buffer, get timestamp value.
/// let starting_timestamp: i64 = /* whatever you get from this timestamp */ 0;
///
/// // Create the gpu context
/// let gpu_context = client.new_gpu_context(
///     Some("MyContext"),
///     tracy_client::GpuContextType::Vulkan,
///     starting_timestamp,
///     period
/// ).unwrap();
///
/// // Now you have some work that you want to time on the gpu.
///
/// // GPU API: Record writing a timestamp before the work.
/// let mut span = gpu_context.span_alloc("MyGpuSpan1", "My::Work", "myfile.rs", 12).unwrap();
///
/// // GPU API: Record work.
///
/// // GPU API: Record writing a timestamp after the work.
/// span.end_zone();
///
/// // Some time later, once the written timestamp values are available on the cpu.
/// # let (starting_timestamp, ending_timestamp) = (0, 0);
///
/// span.upload_timestamp_start(starting_timestamp);
/// span.upload_timestamp_end(ending_timestamp);
/// ```
#[derive(Clone)]
pub struct GpuContext {
    #[cfg(feature = "enable")]
    _client: Client,
    #[cfg(feature = "enable")]
    value: u8,
    #[cfg(feature = "enable")]
    span_freelist: Arc<Mutex<Vec<u16>>>,
    _private: (),
}
#[cfg(feature = "enable")]
static GPU_CONTEXT_INDEX: Mutex<u8> = Mutex::new(0);

/// Errors that can occur when creating a gpu context.
#[derive(Debug)]
pub enum GpuContextCreationError {
    /// More than `u8::MAX` contexts have been created at any point in the program.
    TooManyContextsCreated,
}

impl std::fmt::Display for GpuContextCreationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "More than 255 contexts have been created at any point in the execution of this program."
        )
    }
}

impl std::error::Error for GpuContextCreationError {}

#[derive(Debug, PartialEq)]
enum GpuSpanState {
    /// The span has been started. All gpu spans start in this state.
    Started,
    /// The span has been ended, either waiting for timestamp upload or with
    /// timestamp upload completed.
    Ended,
}

/// Span for timing gpu work.
///
/// See the [context level documentation](GpuContext) for more information on use.
///
/// If the span is dropped early, the following happens:
/// - If the span has not been ended, the span is ended. AND
/// - If the span has not had values uploaded, the span is uploaded with
///   the timestamps marking the start of the current gpu context. This
///   will put the span out of the way of other spans.
#[must_use]
pub struct GpuSpan {
    #[cfg(feature = "enable")]
    context: GpuContext,
    #[cfg(feature = "enable")]
    start_query_id: u16,
    #[cfg(feature = "enable")]
    end_query_id: u16,
    #[cfg(feature = "enable")]
    state: GpuSpanState,
    _private: (),
}

/// Errors that can occur when creating a gpu span.
#[derive(Debug)]
pub enum GpuSpanCreationError {
    /// More than `32767` spans are still waiting for gpu data.
    TooManyPendingSpans,
}

impl std::fmt::Display for GpuSpanCreationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Too many spans still waiting for gpu data. There may not be more than 32767 spans that are pending gpu data at once."
        )
    }
}

impl std::error::Error for GpuSpanCreationError {}

impl Client {
    /// Creates a new GPU context.
    ///
    /// - `name` is the name of the context.
    /// - `ty` is the type (backend) of the context.
    /// - `gpu_timestamp` is the gpu side timestamp the corresponds (as close as possible) to this call.
    /// - `period` is the period of the gpu clock in nanoseconds (setting 1.0 means the clock is 1GHz, 1000.0 means 1MHz, etc).
    ///
    /// See the [type level documentation](GpuContext) for more information.
    ///
    /// # Errors
    ///
    /// - If more than 255 contexts were made during the lifetime of the application.
    pub fn new_gpu_context(
        self,
        name: Option<&str>,
        ty: GpuContextType,
        gpu_timestamp: i64,
        period: f32,
    ) -> Result<GpuContext, GpuContextCreationError> {
        #[cfg(feature = "enable")]
        {
            // We use a mutex to lock the context index to prevent races when using fetch_add.
            //
            // This prevents multiple contexts getting the same context id.
            let mut context_index_guard = GPU_CONTEXT_INDEX.lock().unwrap();
            if *context_index_guard == 255 {
                return Err(GpuContextCreationError::TooManyContextsCreated);
            }
            let context = *context_index_guard;
            *context_index_guard += 1;
            drop(context_index_guard);

            // SAFETY:
            // - We know we aren't re-using the context id because of the above logic.
            unsafe {
                sys::___tracy_emit_gpu_new_context_serial(sys::___tracy_gpu_new_context_data {
                    gpuTime: gpu_timestamp,
                    period,
                    context,
                    flags: 0,
                    type_: ty as u8,
                });
            };

            if let Some(name) = name {
                // SAFETY:
                // - We've allocated a context.
                // - The names will copied into the command stream, so the pointers do not need to last.
                unsafe {
                    sys::___tracy_emit_gpu_context_name_serial(
                        sys::___tracy_gpu_context_name_data {
                            context,
                            name: name.as_ptr().cast(),
                            len: name.len().try_into().unwrap_or(u16::MAX),
                        },
                    );
                }
            }

            Ok(GpuContext {
                _client: self,
                value: context,
                span_freelist: Arc::new(Mutex::new((0..=u16::MAX).collect())),
                _private: (),
            })
        }
        #[cfg(not(feature = "enable"))]
        Ok(GpuContext { _private: () })
    }
}

impl GpuContext {
    #[cfg(feature = "enable")]
    fn alloc_span_ids(&self) -> Result<(u16, u16), GpuSpanCreationError> {
        let mut freelist = self.span_freelist.lock().unwrap();
        if freelist.len() < 2 {
            return Err(GpuSpanCreationError::TooManyPendingSpans);
        }
        // These unwraps are unreachable.
        let start = freelist.pop().unwrap();
        let end = freelist.pop().unwrap();
        Ok((start, end))
    }

    /// Creates a new gpu span with the given source location.
    ///
    /// This should be called right next to where you record the corresponding gpu timestamp. This
    /// allows tracy to correctly associate the cpu time with the gpu timestamp.
    ///
    /// # Errors
    ///
    /// - If there are more than 32767 spans waiting for gpu data at once.
    pub fn span(
        &self,
        span_location: &'static SpanLocation,
    ) -> Result<GpuSpan, GpuSpanCreationError> {
        #[cfg(feature = "enable")]
        {
            let (start_query_id, end_query_id) = self.alloc_span_ids()?;

            // SAFETY: We know that the span location is valid forever as it is 'static. `usize` will
            // always be smaller than u64, so no data will be lost.
            unsafe {
                sys::___tracy_emit_gpu_zone_begin_serial(sys::___tracy_gpu_zone_begin_data {
                    srcloc: std::ptr::addr_of!(span_location.data) as usize as u64,
                    queryId: start_query_id,
                    context: self.value,
                });
            };

            Ok(GpuSpan {
                context: self.clone(),
                start_query_id,
                end_query_id,
                state: GpuSpanState::Started,
                _private: (),
            })
        }
        #[cfg(not(feature = "enable"))]
        Ok(GpuSpan { _private: () })
    }

    /// Creates a new gpu span with the given name, function, file, and line.
    ///
    /// This should be called right next to where you record the corresponding gpu timestamp. This
    /// allows tracy to correctly associate the cpu time with the gpu timestamp.
    ///
    /// # Errors
    ///
    /// - If there are more than 32767 spans waiting for gpu data at once.
    pub fn span_alloc(
        &self,
        name: &str,
        function: &str,
        file: &str,
        line: u32,
    ) -> Result<GpuSpan, GpuSpanCreationError> {
        #[cfg(feature = "enable")]
        {
            let srcloc = unsafe {
                sys::___tracy_alloc_srcloc_name(
                    line,
                    file.as_ptr().cast(),
                    file.len(),
                    function.as_ptr().cast(),
                    function.len(),
                    name.as_ptr().cast(),
                    name.len(),
                    0,
                )
            };

            let (start_query_id, end_query_id) = self.alloc_span_ids()?;

            unsafe {
                sys::___tracy_emit_gpu_zone_begin_alloc_serial(sys::___tracy_gpu_zone_begin_data {
                    srcloc,
                    queryId: start_query_id,
                    context: self.value,
                });
            };

            Ok(GpuSpan {
                context: self.clone(),
                start_query_id,
                end_query_id,
                state: GpuSpanState::Started,
                _private: (),
            })
        }
        #[cfg(not(feature = "enable"))]
        Ok(GpuSpan { _private: () })
    }
}

impl GpuSpan {
    /// Marks the end of the given gpu span. This should be called right next to where you record
    /// the corresponding gpu timestamp for the end of the span. This allows tracy to correctly
    /// associate the cpu time with the gpu timestamp.
    ///
    /// Only the first time you call this function will it actually emit a gpu zone end event. Any
    /// subsequent calls will be ignored.
    pub fn end_zone(&mut self) {
        #[cfg(feature = "enable")]
        {
            if self.state != GpuSpanState::Started {
                return;
            }
            unsafe {
                sys::___tracy_emit_gpu_zone_end_serial(sys::___tracy_gpu_zone_end_data {
                    queryId: self.end_query_id,
                    context: self.context.value,
                });
            };
            self.state = GpuSpanState::Ended;
        }
    }

    /// Supplies the GPU timestamp for the start of this span.
    ///
    /// In order to avoid confusing Tracy, you must call
    /// [`Self::upload_timestamp_start`] and [`Self::upload_timestamp_end`] in
    /// monotonically increasing timestamp order. For example, if you have two
    /// nested spans *outer* and *inner*, you must supply the timestamps in
    /// this order: (1) *outer* start; (2) *inner* start; (3) *inner* end; (4)
    /// *outer* end.
    pub fn upload_timestamp_start(&self, start_timestamp: i64) {
        #[cfg(feature = "enable")]
        unsafe {
            sys::___tracy_emit_gpu_time_serial(sys::___tracy_gpu_time_data {
                gpuTime: start_timestamp,
                queryId: self.start_query_id,
                context: self.context.value,
            });
        };
    }

    /// Supplies the GPU timestamp for the end of this span.
    ///
    /// In order to avoid confusing Tracy, you must call
    /// [`Self::upload_timestamp_start`] and [`Self::upload_timestamp_end`] in
    /// monotonically increasing timestamp order. For example, if you have two
    /// nested spans *outer* and *inner*, you must supply the timestamps in this
    /// order: (1) *outer* start; (2) *inner* start; (3) *inner* end; (4)
    /// *outer* end.
    pub fn upload_timestamp_end(&self, end_timestamp: i64) {
        #[cfg(feature = "enable")]
        unsafe {
            sys::___tracy_emit_gpu_time_serial(sys::___tracy_gpu_time_data {
                gpuTime: end_timestamp,
                queryId: self.end_query_id,
                context: self.context.value,
            });
        };
    }
}

impl Drop for GpuSpan {
    fn drop(&mut self) {
        #[cfg(feature = "enable")]
        {
            match self.state {
                GpuSpanState::Started => {
                    self.end_zone();
                }
                GpuSpanState::Ended => {}
            }

            // Put the ids back into the freelist.
            let mut freelist = self.context.span_freelist.lock().unwrap();
            freelist.push(self.start_query_id);
            freelist.push(self.end_query_id);
            drop(freelist);
        }
    }
}
