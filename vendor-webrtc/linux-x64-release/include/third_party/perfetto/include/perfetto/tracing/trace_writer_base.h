/*
 * Copyright (C) 2019 The Android Open Source Project
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#ifndef INCLUDE_PERFETTO_TRACING_TRACE_WRITER_BASE_H_
#define INCLUDE_PERFETTO_TRACING_TRACE_WRITER_BASE_H_

#include <cstdint>
#include "perfetto/base/export.h"
#include "perfetto/protozero/message_handle.h"

namespace perfetto {

namespace protos {
namespace pbzero {
class TracePacket;
}  // namespace pbzero
}  // namespace protos

// This is a single-thread write interface that allows to write protobufs
// directly into the tracing shared buffer without making any copies.
// The idea is that each data source creates one (or more) TraceWriter for each
// thread it wants to write from. Each TraceWriter will get its own dedicated
// chunk and will write into the shared buffer without any locking most of the
// time.

class PERFETTO_EXPORT_COMPONENT TraceWriterBase {
 public:
  virtual ~TraceWriterBase();

  // Creates a new trace packet and returns a handle to a protozero Message that
  // will write to it. The message will be finalized either by calling directly
  // handle.Finalize() or by letting the handle go out of scope (the message
  // should be finalized before a new call to NewTracePacket is made). The
  // returned handle can be std::move()'d but cannot be used after either: (i)
  // the TraceWriter instance is destroyed, (ii) a subsequence NewTracePacket()
  // call is made on the same TraceWriter instance.
  //
  // The caller can use protozero::MessageHandle::TakeStreamWriter() to write.
  //
  // The caller must call ->Finalize() on the returned trace packet (the handle
  // destructor will take care of that) or explicitly call FinishTracePacket (if
  // using TakeStreamWriter) before calling any method on the same TraceWriter
  // instance.
  //
  // The returned packet handle is always valid, but note that, when using
  // BufferExhaustedPolicy::kDrop and the SMB is exhausted, it may be assigned
  // a garbage chunk and any trace data written into it will be lost. For more
  // details on buffer size choices: https://perfetto.dev/docs/concepts/buffers.
  virtual protozero::MessageHandle<protos::pbzero::TracePacket>
  NewTracePacket() = 0;

  // Tells the TraceWriterBase that the previous packet started with
  // NewTracePacket() is finished.
  //
  // Calling this is optional: the TraceWriterBase can realize that the previous
  // packet is finished when the next NewTracePacket() is called. It is still
  // useful, because the next NewTracePacket may not happen for a while.
  virtual void FinishTracePacket() = 0;

  // Commits the data pending for the current chunk. This can be called
  // only if the handle returned by NewTracePacket() has been destroyed (i.e. we
  // cannot Flush() while writing a TracePacket).
  //
  // Note: Flush() also happens implicitly when destroying the TraceWriter.
  //
  // |callback| is an optional callback. When non-null it will request the
  // service to ACK the flush and will be invoked after the service has
  // acknowledged it. The callback might be NEVER INVOKED if the service crashes
  // or the IPC connection is dropped. The callback should be used only by tests
  // and best-effort features (logging).
  virtual void Flush(std::function<void()> callback = {}) = 0;

  // Bytes written since creation. Not reset when new chunks are acquired.
  virtual uint64_t written() const = 0;

  // Number of times the trace writer entered a mode in which it started
  // dropping data.
  //
  // This does not necessarily correspond to the number of packets/chunks
  // dropped, as multiple such packets/chunks can be dropped on entry into a
  // drop data mode.
  virtual uint64_t drop_count() const = 0;
};

}  // namespace perfetto

#endif  // INCLUDE_PERFETTO_TRACING_TRACE_WRITER_BASE_H_
