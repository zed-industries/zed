// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TRACE_EVENT_TYPED_MACROS_EMBEDDER_SUPPORT_H_
#define BASE_TRACE_EVENT_TYPED_MACROS_EMBEDDER_SUPPORT_H_

#include "base/base_export.h"
#include "base/memory/raw_ptr.h"
#include "base/memory/raw_ptr_exclusion.h"
#include "base/trace_event/trace_event.h"
#include "third_party/perfetto/include/perfetto/tracing/internal/track_event_internal.h"
#include "third_party/perfetto/protos/perfetto/trace/track_event/track_event.pbzero.h"

namespace base {
namespace trace_event {

// This header's declarations are implemented in typed_macros_internal.cc.

// Handle to a TrackEvent which notifies a listener upon its destruction (after
// the event lambda has emitted any typed event arguments).
class BASE_EXPORT TrackEventHandle {
 public:
  using TrackEvent = perfetto::protos::pbzero::TrackEvent;
  using IncrementalState = perfetto::internal::TrackEventIncrementalState;

  class BASE_EXPORT CompletionListener {
   public:
    virtual ~CompletionListener();
    virtual void OnTrackEventCompleted() = 0;
  };

  // Creates a handle to |event| which notifies |listener| on the handle's
  // destruction, i.e. after the event lambda has emitted any typed arguments
  // into the event. Note that |listener| must outlive the TRACE_EVENT call,
  // i.e. cannot be destroyed until OnTrackEventCompleted() is called. Ownership
  // of both TrackEvent and the listener remains with the caller.
  TrackEventHandle(TrackEvent*,
                   IncrementalState*,
                   CompletionListener*,
                   bool filter_debug_annotations);

  // Creates an invalid handle.
  TrackEventHandle();

  ~TrackEventHandle();

  explicit operator bool() const { return event_; }
  TrackEvent& operator*() const { return *event_; }
  TrackEvent* operator->() const { return event_; }
  TrackEvent* get() const { return event_; }

  IncrementalState* incremental_state() const { return incremental_state_; }

  bool ShouldFilterDebugAnnotations() const {
    return filter_debug_annotations_;
  }

 private:
  // RAW_PTR_EXCLUSION: Performance reasons: based on this sampling profiler
  // result on ChromeOS. go/brp-cros-prof-diff-20230403
  RAW_PTR_EXCLUSION TrackEvent* event_;
  RAW_PTR_EXCLUSION IncrementalState* incremental_state_;
  RAW_PTR_EXCLUSION CompletionListener* listener_;
  const bool filter_debug_annotations_ = false;
};

// Handle to a TracePacket which notifies a listener upon its destruction (after
// base has emitted all data into the packet).
class BASE_EXPORT TracePacketHandle {
 public:
  using TracePacket = perfetto::protos::pbzero::TracePacket;
  using PerfettoPacketHandle = protozero::MessageHandle<TracePacket>;

  class BASE_EXPORT CompletionListener {
   public:
    virtual ~CompletionListener();
    virtual void OnTracePacketCompleted() = 0;
  };

  // Creates a handle to |packet| which notifies |listener| on the handle's
  // destruction, i.e. after base has emitted all data into the packet. Note
  // that |listener| must outlive the TRACE_EVENT call, i.e. cannot be destroyed
  // until OnTracePacketCompleted() is called. Ownership of both TrackEvent and
  // the listener remains with the caller.
  TracePacketHandle(PerfettoPacketHandle, CompletionListener*);

  // Creates an invalid handle.
  TracePacketHandle();

  ~TracePacketHandle();

  // Move only.
  TracePacketHandle(TracePacketHandle&&) noexcept;
  TracePacketHandle& operator=(TracePacketHandle&&);

  explicit operator bool() const { return static_cast<bool>(packet_); }
  TracePacket& operator*() const { return *packet_; }
  TracePacket* operator->() const { return packet_.get(); }
  TracePacket* get() const { return packet_.get(); }

  PerfettoPacketHandle TakePerfettoHandle() { return std::move(packet_); }

 private:
  PerfettoPacketHandle packet_;
  raw_ptr<CompletionListener> listener_;
};

using PrepareTrackEventFunction = TrackEventHandle (*)(TraceEvent*);
using PrepareTracePacketFunction = TracePacketHandle (*)();
using EmitEmptyTracePacketFunction = void (*)();

// Embedder should call this (only once) to set the callback invoked when a
// typed event should be emitted. The callback functions may be executed on any
// thread.
BASE_EXPORT void EnableTypedTraceEvents(
    PrepareTrackEventFunction typed_event_callback,
    PrepareTracePacketFunction trace_packet_callback,
    EmitEmptyTracePacketFunction empty_packet_callback);

BASE_EXPORT void ResetTypedTraceEventsForTesting();

}  // namespace trace_event
}  // namespace base

#endif  // BASE_TRACE_EVENT_TYPED_MACROS_EMBEDDER_SUPPORT_H_
