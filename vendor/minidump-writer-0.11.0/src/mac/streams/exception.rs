use {super::*, mach2::exception_types as et};

impl MinidumpWriter {
    /// Writes the [`minidump_common::format::MINIDUMP_EXCEPTION_STREAM`] stream.
    ///
    /// This stream is optional on MacOS as a user requested minidump could
    /// choose not to specify the exception information.
    pub(crate) fn write_exception(
        &mut self,
        buffer: &mut DumpBuf,
        dumper: &TaskDumper,
    ) -> Result<MDRawDirectory, WriterError> {
        // This shouldn't fail since we won't be writing this stream if the crash context is
        // not present
        let crash_context = self
            .crash_context
            .as_ref()
            .ok_or(WriterError::NoCrashContext)?;

        let thread_state = dumper.read_thread_state(crash_context.thread).ok();

        let thread_context = if let Some(ts) = &thread_state {
            let mut cpu = Default::default();
            Self::fill_cpu_context(ts, &mut cpu);
            MemoryWriter::alloc_with_val(buffer, cpu)
                .map(|mw| mw.location())
                .ok()
        } else {
            None
        };

        let exception_record = crash_context
            .exception
            .as_ref()
            .map(|exc| {
                let code = exc.code as u64;

                // `EXC_CRASH` exceptions wrap other exceptions, so we want to
                // retrieve the _actual_ exception
                let wrapped_exc = if exc.kind as u32 == et::EXC_CRASH {
                    recover_exc_crash_wrapped_exception(code)
                } else {
                    None
                };

                // For EXC_RESOURCE and EXC_GUARD crashes Crashpad records the
                // uppermost 32 bits of the exception code in the exception flags,
                // as they are the most interesting for those exceptions. Neither
                // of these exceptions can be wrapped by an `EXC_CRASH`
                //
                // EXC_GUARD
                // code:
                // +-------------------+----------------+--------------+
                // |[63:61] guard type | [60:32] flavor | [31:0] target|
                // +-------------------+----------------+--------------+
                //
                // EXC_RESOURCE
                // code:
                // +--------------------------------------------------------+
                // |[63:61] resource type | [60:58] flavor | [57:32] unused |
                // +--------------------------------------------------------+
                let exception_code =
                    if exc.kind as u32 == et::EXC_RESOURCE || exc.kind as u32 == et::EXC_GUARD {
                        (code >> 32) as u32
                    } else if let Some(wrapped) = wrapped_exc {
                        wrapped.code
                    } else {
                        // For all other exceptions types, the value in the code
                        // _should_ never exceed 32 bits, crashpad does an actual
                        // range check here.
                        if code > u32::MAX.into() {
                            // TODO: do something more than logging?
                            log::warn!("exception code {code:#018x} exceeds the expected 32 bits");
                        }
                        code as u32
                    };

                let exception_kind = if let Some(wrapped) = wrapped_exc {
                    wrapped.kind
                } else {
                    exc.kind
                };

                let exception_address =
                    if exception_kind == et::EXC_BAD_ACCESS && exc.subcode.is_some() {
                        exc.subcode.unwrap_or_default()
                    } else if let Some(ts) = thread_state {
                        ts.pc()
                    } else {
                        0
                    };

                // The naming is confusing here, but it is how it is
                let mut md_exc = MDException {
                    exception_code: exception_kind,
                    exception_flags: exception_code,
                    exception_address,
                    ..Default::default()
                };

                // Now append the (mostly) original information to the "ancillary"
                // exception_information at the end. This allows a minidump parser
                // to recover the full exception information for the crash, rather
                // than only using the (potentially) truncated information we
                // just set in `exception_code` and `exception_flags`
                md_exc.exception_information[0] = exception_kind as u64;
                md_exc.exception_information[1] = code;

                md_exc.number_parameters = if let Some(subcode) = exc.subcode {
                    md_exc.exception_information[2] = subcode;
                    3
                } else {
                    2
                };

                md_exc
            })
            .unwrap_or_default();

        let stream = MDRawExceptionStream {
            thread_id: crash_context.thread,
            exception_record,
            thread_context: thread_context.unwrap_or_default(),
            __align: 0,
        };

        let exc_section = MemoryWriter::<MDRawExceptionStream>::alloc_with_val(buffer, stream)?;

        Ok(MDRawDirectory {
            stream_type: MDStreamType::ExceptionStream as u32,
            location: exc_section.location(),
        })
    }
}

/// [`et::EXC_CRASH`] is a wrapper exception around another exception, but not
/// all exceptions can be wrapped by it, so this function validates that the
/// `EXC_CRASH` is actually valid
#[inline]
fn is_valid_exc_crash(exc_code: u64) -> bool {
    let wrapped = ((exc_code >> 20) & 0xf) as u32;

    !(
        wrapped == et::EXC_CRASH // EXC_CRASH can't wrap another one
        || wrapped == et::EXC_RESOURCE // EXC_RESOURCE would lose information
        || wrapped == et::EXC_GUARD // EXC_GUARD would lose information
        || wrapped == et::EXC_CORPSE_NOTIFY
        // cannot be wrapped
    )
}

/// The details for an exception wrapped by an `EXC_CRASH`
#[derive(Copy, Clone)]
struct WrappedException {
    /// The `EXC_*` that was wrapped
    kind: u32,
    /// The code of the wrapped exception, for all exceptions other than
    /// `EXC_RESOURCE` and `EXC_GUARD` this _should_ never exceed 32 bits, and
    /// is one of the reasons that `EXC_CRASH` cannot wrap those 2 exceptions
    code: u32,
    /// The Unix signal number that the original exception was converted into
    _signal: u8,
}

/// Unwraps an `EXC_CRASH` exception code to the inner exception it wraps.
///
/// Will return `None` if the specified code is wrapping an exception that
/// should not be possible to be wrapped in an `EXC_CRASH`
#[inline]
fn recover_exc_crash_wrapped_exception(code: u64) -> Option<WrappedException> {
    is_valid_exc_crash(code).then(|| WrappedException {
        kind: ((code >> 20) & 0xf) as u32,
        code: (code & 0xfffff) as u32,
        _signal: ((code >> 24) & 0xff) as u8,
    })
}
