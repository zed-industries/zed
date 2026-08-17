mod dispatcher;

use self::dispatcher::Dispatcher;
pub use self::dispatcher::OutputSink;
pub(crate) use self::dispatcher::{AuxStartTagInfo, DispatcherError};
pub use self::dispatcher::{StartTagHandlingResult, TransformController};
use crate::base::SharedEncoding;
use crate::memory::{Arena, SharedMemoryLimiter};
use crate::parser::{Parser, ParserDirective};
use crate::rewriter::RewritingError;

// Pub only for integration tests
pub struct TransformStreamSettings<C, O>
where
    C: TransformController,
    O: OutputSink,
{
    pub transform_controller: C,
    pub output_sink: O,
    pub preallocated_parsing_buffer_size: usize,
    pub memory_limiter: SharedMemoryLimiter,
    pub encoding: SharedEncoding,
    pub strict: bool,
}

// Pub only for integration tests
pub struct TransformStream<C, O>
where
    C: TransformController,
    O: OutputSink,
{
    parser: Parser<Dispatcher<C, O>>,
    buffer: Arena,
    has_buffered_data: bool,
}

impl<C, O> TransformStream<C, O>
where
    C: TransformController,
    O: OutputSink,
{
    pub fn new(settings: TransformStreamSettings<C, O>) -> Self {
        let initial_parser_directive = if settings
            .transform_controller
            .initial_capture_flags()
            .is_empty()
        {
            ParserDirective::WherePossibleScanForTagsOnly
        } else {
            ParserDirective::Lex
        };

        let dispatcher = Dispatcher::new(
            settings.transform_controller,
            settings.output_sink,
            settings.encoding,
        );

        let buffer = Arena::new(
            settings.memory_limiter,
            settings.preallocated_parsing_buffer_size,
        );

        let parser = Parser::new(dispatcher, initial_parser_directive, settings.strict);

        Self {
            parser,
            buffer,
            has_buffered_data: false,
        }
    }

    pub fn write(&mut self, data: &[u8]) -> Result<(), RewritingError> {
        trace!(@write data);

        let chunk = if self.has_buffered_data {
            self.buffer
                .append(data)
                .map_err(RewritingError::MemoryLimitExceeded)?;

            self.buffer.bytes()
        } else {
            data
        };

        trace!(@chunk chunk);

        let consumed_byte_count = self.parser.parse(chunk, false)?;

        self.parser
            .get_dispatcher()
            .flush_remaining_input(chunk, consumed_byte_count);

        if consumed_byte_count < chunk.len() {
            if self.has_buffered_data {
                self.buffer.shift(consumed_byte_count);
            } else if let Some(unconsumed) = data.get(consumed_byte_count..) {
                self.buffer
                    .init_with(unconsumed)
                    .map_err(RewritingError::MemoryLimitExceeded)?;

                self.has_buffered_data = true;
            } else {
                debug_assert!(false);
            }
        } else {
            self.has_buffered_data = false;
        }

        Ok(())
    }

    pub fn end(&mut self) -> Result<(), RewritingError> {
        trace!(@end);

        let chunk = if self.has_buffered_data {
            self.buffer.bytes()
        } else {
            &[]
        };

        trace!(@chunk chunk);

        self.parser.parse(chunk, true)?;
        self.parser.get_dispatcher().finish(chunk)
    }

    #[cfg(feature = "integration_test")]
    #[allow(private_interfaces)]
    pub fn parser(&mut self) -> &mut Parser<Dispatcher<C, O>> {
        &mut self.parser
    }
}
