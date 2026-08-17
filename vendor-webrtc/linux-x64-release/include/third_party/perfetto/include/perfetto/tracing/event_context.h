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

#ifndef INCLUDE_PERFETTO_TRACING_EVENT_CONTEXT_H_
#define INCLUDE_PERFETTO_TRACING_EVENT_CONTEXT_H_

#include "perfetto/protozero/message_handle.h"
#include "perfetto/tracing/internal/track_event_internal.h"
#include "perfetto/tracing/traced_proto.h"
#include "protos/perfetto/trace/trace_packet.pbzero.h"

namespace perfetto {
namespace protos {
namespace pbzero {
class DebugAnnotation;
}  // namespace pbzero
}  // namespace protos

namespace internal {
class TrackEventInternal;
}

// Allows adding custom arguments into track events. Example:
//
//   TRACE_EVENT_BEGIN("category", "Title",
//                     [](perfetto::EventContext ctx) {
//                       auto* log = ctx.event()->set_log_message();
//                       log->set_body_iid(1234);
//
//                       ctx.AddDebugAnnotation("name", 1234);
//                     });
//
class PERFETTO_EXPORT_COMPONENT EventContext {
 public:
  EventContext(EventContext&&) = default;

  // For Chromium during the transition phase to the client library.
  // TODO(eseckler): Remove once Chromium has switched to client lib entirely.
  explicit EventContext(
      protos::pbzero::TrackEvent* event,
      internal::TrackEventIncrementalState* incremental_state = nullptr,
      bool filter_debug_annotations = false)
      : event_(event),
        incremental_state_(incremental_state),
        filter_debug_annotations_(filter_debug_annotations) {}

  ~EventContext();

  internal::TrackEventIncrementalState* GetIncrementalState() const {
    return incremental_state_;
  }

  // Disclaimer: Experimental method, subject to change.
  // Exposed publicly to emit some TrackEvent fields in Chromium only in local
  // tracing. Long-term, we really shouldn't be (ab)using the
  // filter_debug_annotation setting for this.
  //
  // TODO(kraskevich): Come up with a more precise name once we have more than
  // one usecase.
  bool ShouldFilterDebugAnnotations() const {
    if (tls_state_) {
      return tls_state_->filter_debug_annotations;
    }
    // In Chromium tls_state_ is nullptr, so we need to get this information
    // from a separate field.
    return filter_debug_annotations_;
  }

  // Get a TrackEvent message to write typed arguments to.
  //
  // event() is a template method to allow callers to specify a subclass of
  // TrackEvent instead. Those subclasses correspond to TrackEvent message with
  // application-specific extensions. More information in
  // design-docs/extensions.md.
  template <typename EventType = protos::pbzero::TrackEvent>
  EventType* event() const {
    // As the method does downcasting, we check that a target subclass does
    // not add new fields.
    static_assert(
        sizeof(EventType) == sizeof(protos::pbzero::TrackEvent),
        "Event type must be binary-compatible with protos::pbzero::TrackEvent");
    return static_cast<EventType*>(event_);
  }

  // Convert a raw pointer to protozero message to TracedProto which captures
  // the reference to this EventContext.
  template <typename MessageType>
  TracedProto<MessageType> Wrap(MessageType* message) {
    static_assert(std::is_base_of<protozero::Message, MessageType>::value,
                  "TracedProto can be used only with protozero messages");

    return TracedProto<MessageType>(message, this);
  }

  // Add a new `debug_annotation` proto message and populate it from |value|
  // using perfetto::TracedValue API. Users should generally prefer passing
  // values directly to TRACE_EVENT (i.e. TRACE_EVENT(..., "arg", value, ...);)
  // but in rare cases (e.g. when an argument should be written conditionally)
  // EventContext::AddDebugAnnotation provides an explicit equivalent.
  template <typename EventNameType, typename T>
  void AddDebugAnnotation(EventNameType&& name, T&& value) {
    if (tls_state_ && tls_state_->filter_debug_annotations)
      return;
    auto annotation = AddDebugAnnotation(std::forward<EventNameType>(name));
    WriteIntoTracedValue(internal::CreateTracedValueFromProto(annotation, this),
                         std::forward<T>(value));
  }

  // Read arbitrary user data that is associated with the thread-local per
  // instance state of the track event. `key` must be non-null and unique
  // per TrackEventTlsStateUserData subclass.
  TrackEventTlsStateUserData* GetTlsUserData(const void* key);

  // Set arbitrary user data that is associated with the thread-local per
  // instance state of the track event. `key` must be non-null and unique
  // per TrackEventTlsStateUserData subclass.
  void SetTlsUserData(const void* key,
                      std::unique_ptr<TrackEventTlsStateUserData> data);

 private:
  template <typename, size_t, typename, typename>
  friend class TrackEventInternedDataIndex;
  friend class internal::TrackEventInternal;

  using TracePacketHandle =
      ::protozero::MessageHandle<protos::pbzero::TracePacket>;

  EventContext(TraceWriterBase* trace_writer,
               TracePacketHandle,
               internal::TrackEventIncrementalState*,
               internal::TrackEventTlsState*);
  EventContext(const EventContext&) = delete;

  protos::pbzero::DebugAnnotation* AddDebugAnnotation(const char* name);
  protos::pbzero::DebugAnnotation* AddDebugAnnotation(
      ::perfetto::DynamicString name);

  TraceWriterBase* trace_writer_ = nullptr;
  TracePacketHandle trace_packet_;
  protos::pbzero::TrackEvent* event_;
  internal::TrackEventIncrementalState* incremental_state_;
  // TODO(mohitms): Make it const-reference instead of pointer, once we
  // are certain that it cannot be nullptr. Once we switch to client library in
  // chrome, we can make that happen.
  internal::TrackEventTlsState* tls_state_ = nullptr;
  // TODO(kraskevich): Come up with a more precise name once we have more than
  // one usecase.
  // TODO(kraskevich): Remove once Chromium has fully switched to client lib.
  const bool filter_debug_annotations_ = false;
};

}  // namespace perfetto

#endif  // INCLUDE_PERFETTO_TRACING_EVENT_CONTEXT_H_
