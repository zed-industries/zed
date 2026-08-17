//
//
// Copyright 2018 gRPC authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
//

#ifndef GRPC_SRC_CORE_TSI_ALTS_FRAME_PROTECTOR_FRAME_HANDLER_H
#define GRPC_SRC_CORE_TSI_ALTS_FRAME_PROTECTOR_FRAME_HANDLER_H

#include <grpc/support/port_platform.h>
#include <stdbool.h>
#include <stdlib.h>

const size_t kFrameMessageType = 0x06;
const size_t kFrameLengthFieldSize = 4;
const size_t kFrameMessageTypeFieldSize = 4;
const size_t kFrameMaxSize = 1024 * 1024;
const size_t kFrameHeaderSize =
    kFrameLengthFieldSize + kFrameMessageTypeFieldSize;

///
/// Implementation of frame reader and frame writer. All APIs in the
/// header are thread-compatible.
///

///
/// Main struct for a frame writer. It reads frames from an input buffer, and
/// writes the contents as raw bytes. It does not own the input buffer.
///
typedef struct alts_frame_writer {
  const unsigned char* input_buffer;
  unsigned char header_buffer[kFrameHeaderSize];
  size_t input_bytes_written;
  size_t header_bytes_written;
  size_t input_size;
} alts_frame_writer;

///
/// Main struct for a frame reader. It reads raw bytes and puts the framed
/// result into an output buffer. It does not own the output buffer.
///
typedef struct alts_frame_reader {
  unsigned char* output_buffer;
  unsigned char header_buffer[kFrameHeaderSize];
  size_t header_bytes_read;
  size_t output_bytes_read;
  size_t bytes_remaining;
} alts_frame_reader;

///
/// This method creates a frame writer instance and initializes its internal
/// states.
///
alts_frame_writer* alts_create_frame_writer();

///
/// This method resets internal states of a frame writer and prepares to write
/// a single frame. It does not take ownership of payload_buffer.
/// The payload_buffer must outlive the writer.
///
///- writer: a frame writer instance.
///- buffer: a buffer storing full payload data to be framed.
///- length: size of payload data.
///
/// The method returns true on success and false otherwise.
///
bool alts_reset_frame_writer(alts_frame_writer* writer,
                             const unsigned char* buffer, size_t length);

///
/// This method writes up to bytes_size bytes of a frame to output.
///
///- writer: a frame writer instance.
///- output: an output buffer used to store the frame.
///- bytes_size: an in/out parameter that stores the size of output buffer
///  before the call, and gets written the number of frame bytes written to the
///  buffer.
///
/// The method returns true on success and false otherwise.
///
bool alts_write_frame_bytes(alts_frame_writer* writer, unsigned char* output,
                            size_t* bytes_size);

///
/// This method checks if a reset can be called to write a new frame. It returns
/// true if it's the first time to frame a payload, or the current frame has
/// been finished processing. It returns false if it's not ready yet to start a
/// new frame (e.g., more payload data needs to be accumulated to process the
/// current frame).
///
/// if (alts_is_frame_writer_done(writer)) {
///  // a new frame can be written, call reset.
///  alts_reset_frame_writer(writer, payload_buffer, payload_size);
///} else {
///  // accumulate more payload data until a full frame can be written.
///}
///
///- writer: a frame writer instance.
///
bool alts_is_frame_writer_done(alts_frame_writer* writer);

///
/// This method returns the number of bytes left to write before a complete
/// frame is formed.
///
///- writer: a frame writer instance.
///
size_t alts_get_num_writer_bytes_remaining(alts_frame_writer* writer);

///
/// This method destroys a frame writer instance.
///
///- writer: a frame writer instance.
///
void alts_destroy_frame_writer(alts_frame_writer* writer);

///
/// This method creates a frame reader instance and initializes its internal
/// states.
///
alts_frame_reader* alts_create_frame_reader();

///
/// This method resets internal states of a frame reader (including setting its
/// output_buffer with buffer), and prepares to write processed bytes to
/// an output_buffer. It does not take ownership of buffer. The buffer must
/// outlive reader.
///
///- reader: a frame reader instance.
///- buffer: an output buffer used to store deframed results.
///
/// The method returns true on success and false otherwise.
///
bool alts_reset_frame_reader(alts_frame_reader* reader, unsigned char* buffer);

///
/// This method processes up to the number of bytes given in bytes_size. It may
/// choose not to process all the bytes, if, for instance, more bytes are
/// given to the method than required to complete the current frame.
///
///- reader: a frame reader instance.
///- bytes: a buffer that stores data to be processed.
///- bytes_size: an in/out parameter that stores the size of bytes before the
///  call and gets written the number of bytes processed.
///
/// The method returns true on success and false otherwise.
///
bool alts_read_frame_bytes(alts_frame_reader* reader,
                           const unsigned char* bytes, size_t* bytes_size);

///
/// This method checks if a frame length has been read.
///
///- reader: a frame reader instance.
///
/// The method returns true if a frame length has been read and false otherwise.
///
bool alts_has_read_frame_length(alts_frame_reader* reader);

///
/// This method returns the number of bytes the frame reader intends to write.
/// It may only be called if alts_has_read_frame_length() returns true.
///
///- reader: a frame reader instance.
///
size_t alts_get_reader_bytes_remaining(alts_frame_reader* reader);

///
/// This method resets output_buffer but does not otherwise modify other
/// internal states of a frame reader instance. After being set, the new
/// output_buffer will hold the deframed payload held by the original
/// output_buffer. It does not take ownership of buffer. The buffer must outlive
/// the reader. To distinguish between two reset methods on a frame reader,
///
/// if (alts_fh_is_frame_reader_done(reader)) {
///  // if buffer contains a full payload to be deframed, call reset.
///  alts_reset_frame_reader(reader, buffer);
///}
///
///// if remaining buffer space is not enough to hold a full payload
/// if (buffer_space_remaining < alts_get_reader_bytes_remaining(reader)) {
///  // allocate enough space for a new buffer, copy back data processed so far,
///  // and call reset.
///  alts_reset_reader_output_buffer(reader, new_buffer).
///}
///
///- reader: a frame reader instance.
///- buffer: a buffer used to set reader's output_buffer.
///
void alts_reset_reader_output_buffer(alts_frame_reader* reader,
                                     unsigned char* buffer);

///
/// This method checks if reset can be called to start processing a new frame.
/// If true and reset was previously called, a full frame has been processed and
/// the content of the frame is available in output_buffer.

///- reader: a frame reader instance.
///
bool alts_is_frame_reader_done(alts_frame_reader* reader);

///
/// This method returns output_bytes_read of a frame reader instance.
///
///- reader: a frame reader instance.
///
size_t alts_get_output_bytes_read(alts_frame_reader* reader);

///
/// This method returns output_buffer of a frame reader instance.
///
///- reader: a frame reader instance.
///
unsigned char* alts_get_output_buffer(alts_frame_reader* reader);

///
/// This method destroys a frame reader instance.
///
///- reader: a frame reader instance.
///
void alts_destroy_frame_reader(alts_frame_reader* reader);

#endif  // GRPC_SRC_CORE_TSI_ALTS_FRAME_PROTECTOR_FRAME_HANDLER_H
