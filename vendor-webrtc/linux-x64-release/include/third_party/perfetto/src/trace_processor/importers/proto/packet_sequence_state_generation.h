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

#ifndef SRC_TRACE_PROCESSOR_IMPORTERS_PROTO_PACKET_SEQUENCE_STATE_GENERATION_H_
#define SRC_TRACE_PROCESSOR_IMPORTERS_PROTO_PACKET_SEQUENCE_STATE_GENERATION_H_

#include <array>
#include <cstddef>
#include <cstdint>
#include <optional>
#include <tuple>
#include <type_traits>
#include <unordered_map>
#include <utility>

#include "perfetto/public/compiler.h"
#include "perfetto/trace_processor/ref_counted.h"
#include "perfetto/trace_processor/trace_blob_view.h"
#include "src/trace_processor/importers/proto/track_event_sequence_state.h"
#include "src/trace_processor/util/interned_message_view.h"

#include "protos/perfetto/trace/trace_packet_defaults.pbzero.h"
#include "protos/perfetto/trace/track_event/thread_descriptor.pbzero.h"
#include "protos/perfetto/trace/track_event/track_event.pbzero.h"

namespace perfetto::trace_processor {

using InternedMessageMap =
    std::unordered_map<uint64_t /*iid*/, InternedMessageView>;
using InternedFieldMap =
    std::unordered_map<uint32_t /*field_id*/, InternedMessageMap>;

class TraceProcessorContext;

class StackProfileSequenceState;
class ProfilePacketSequenceState;
class V8SequenceState;
struct AndroidKernelWakelockState;

using CustomStateClasses = std::tuple<StackProfileSequenceState,
                                      ProfilePacketSequenceState,
                                      V8SequenceState,
                                      AndroidKernelWakelockState>;

// This is the public API exposed to packet tokenizers and parsers to access
// state attached to a packet sequence. This state evolves as packets are
// processed in sequence order. A packet that requires sequence state to be
// properly parsed should snapshot this state by taking a copy of the RefPtr to
// the currently active generation and passing it along with parsing specific
// data to the sorting stage.
class PacketSequenceStateGeneration : public RefCounted {
 public:
  // Base class to attach custom state to the sequence state. This state is keep
  // per sequence and per incremental state interval, that is, each time
  // incremental state is reset a new instance is created but not each time
  // `TracePacketDefaults` are updated. Note that this means that different
  // `PacketSequenceStateGeneration` instances might point to the same
  // `CustomState` (because they only differ in their `TracePacketDefaults`).
  //
  // ATTENTION: You should not create instances of these classes yourself but
  // use the `PacketSequenceStateGeneration::GetCustomState<>' method instead.
  class CustomState : public RefCounted {
   public:
    virtual ~CustomState();

   protected:
    template <uint32_t FieldId, typename MessageType>
    typename MessageType::Decoder* LookupInternedMessage(uint64_t iid) {
      return generation_->LookupInternedMessage<FieldId, MessageType>(iid);
    }

    InternedMessageView* GetInternedMessageView(uint32_t field_id,
                                                uint64_t iid) {
      return generation_->GetInternedMessageView(field_id, iid);
    }

    template <typename T>
    std::remove_cv_t<T>* GetCustomState() {
      return generation_->GetCustomState<T>();
    }

    bool pid_and_tid_valid() const { return generation_->pid_and_tid_valid(); }
    int32_t pid() const { return generation_->pid(); }
    int32_t tid() const { return generation_->tid(); }

   private:
    friend PacketSequenceStateGeneration;
    // Called when the a new generation is created as a result of
    // `TracePacketDefaults` being updated.
    void set_generation(PacketSequenceStateGeneration* generation) {
      generation_ = generation;
    }

    // Note: A `InternedDataTracker` instance can be linked to multiple
    // `PacketSequenceStateGeneration` instances (when there are multiple
    // `TracePacketDefaults` in the same interning context). `generation_` will
    // point to the latest one. We keep this member private to prevent misuse /
    // confusion around this fact. Instead subclasses should access the public
    // methods of this class to get any interned data.
    // TODO(carlscab): Given that CustomState is ref counted this pointer might
    // become invalid. CustomState should not be ref pointed and instead be
    // owned by the `PacketSequenceStateGeneration` instance pointed at by
    // `generation_`.
    PacketSequenceStateGeneration* generation_ = nullptr;
  };

  static RefPtr<PacketSequenceStateGeneration> CreateFirst(
      TraceProcessorContext* context);

  RefPtr<PacketSequenceStateGeneration> OnPacketLoss();

  RefPtr<PacketSequenceStateGeneration> OnIncrementalStateCleared();

  RefPtr<PacketSequenceStateGeneration> OnNewTracePacketDefaults(
      TraceBlobView trace_packet_defaults);

  bool pid_and_tid_valid() const {
    return track_event_sequence_state_.pid_and_tid_valid();
  }
  int32_t pid() const { return track_event_sequence_state_.pid(); }
  int32_t tid() const { return track_event_sequence_state_.tid(); }

  // Returns |nullptr| if the message with the given |iid| was not found (also
  // records a stat in this case).
  template <uint32_t FieldId, typename MessageType>
  typename MessageType::Decoder* LookupInternedMessage(uint64_t iid) {
    auto* interned_message_view = GetInternedMessageView(FieldId, iid);
    if (!interned_message_view)
      return nullptr;

    return interned_message_view->template GetOrCreateDecoder<MessageType>();
  }

  InternedMessageView* GetInternedMessageView(uint32_t field_id, uint64_t iid);
  // Returns |nullptr| if no defaults were set.
  InternedMessageView* GetTracePacketDefaultsView() {
    if (!trace_packet_defaults_.has_value()) {
      return nullptr;
    }

    return &*trace_packet_defaults_;
  }

  // Returns |nullptr| if no defaults were set.
  protos::pbzero::TracePacketDefaults::Decoder* GetTracePacketDefaults() {
    if (!trace_packet_defaults_.has_value()) {
      return nullptr;
    }
    return trace_packet_defaults_
        ->GetOrCreateDecoder<protos::pbzero::TracePacketDefaults>();
  }

  // Returns |nullptr| if no TrackEventDefaults were set.
  protos::pbzero::TrackEventDefaults::Decoder* GetTrackEventDefaults() {
    auto* packet_defaults_view = GetTracePacketDefaultsView();
    if (packet_defaults_view) {
      auto* track_event_defaults_view =
          packet_defaults_view
              ->GetOrCreateSubmessageView<protos::pbzero::TracePacketDefaults,
                                          protos::pbzero::TracePacketDefaults::
                                              kTrackEventDefaultsFieldNumber>();
      if (track_event_defaults_view) {
        return track_event_defaults_view
            ->GetOrCreateDecoder<protos::pbzero::TrackEventDefaults>();
      }
    }
    return nullptr;
  }

  // Extension point for custom incremental state. Custom state classes need to
  // inherit from `CustomState`.
  //
  // A common use case for this custom state is to store cache mappings between
  // interning ids (iid) and TraceProcessor objects (e.g. table row). When we
  // see an iid we need to access the InternedMessageView for that iid, and
  // possibly do some computations, the result of all of this could then be
  // cached so that next time we encounter the same iid we could reuse this
  // cached value. This caching is only valid until incremental state is
  // cleared, from then on subsequent iid values on the sequence will no longer
  // refer to the same entities as the iid values before the clear. Custom state
  // classes no not need to explicitly handle this: they are attached to an
  // `IncrementalState` instance, and a new one is created when the state is
  // cleared, so iid values after the clear will be processed by a new (empty)
  // custom state instance.
  template <typename T>
  std::remove_cv_t<T>* GetCustomState();

  int64_t IncrementAndGetTrackEventTimeNs(int64_t delta_ns) {
    return track_event_sequence_state_.IncrementAndGetTrackEventTimeNs(
        delta_ns);
  }

  int64_t IncrementAndGetTrackEventThreadTimeNs(int64_t delta_ns) {
    return track_event_sequence_state_.IncrementAndGetTrackEventThreadTimeNs(
        delta_ns);
  }

  int64_t IncrementAndGetTrackEventThreadInstructionCount(int64_t delta) {
    return track_event_sequence_state_
        .IncrementAndGetTrackEventThreadInstructionCount(delta);
  }

  bool track_event_timestamps_valid() const {
    return track_event_sequence_state_.timestamps_valid();
  }

  void SetThreadDescriptor(
      const protos::pbzero::ThreadDescriptor::Decoder& descriptor) {
    track_event_sequence_state_.SetThreadDescriptor(descriptor);
  }

  // TODO(carlscab): Nobody other than `ProtoTraceReader` should care about
  // this. Remove.
  bool IsIncrementalStateValid() const { return is_incremental_state_valid_; }

 private:
  friend class PacketSequenceStateBuilder;

  using CustomStateArray =
      std::array<RefPtr<CustomState>, std::tuple_size_v<CustomStateClasses>>;

  // Helper to find the index in a tuple of a given type. Lookups are done
  // ignoring cv qualifiers. If no index is found size of the tuple is returned.
  //
  // ATTENTION: Duplicate types in the tuple will trigger a compiler error.
  template <typename Tuple, typename Type, size_t index = 0>
  static constexpr size_t FindUniqueType() {
    constexpr size_t kSize = std::tuple_size_v<Tuple>;
    if constexpr (index < kSize) {
      using TypeAtIndex = typename std::tuple_element<index, Tuple>::type;
      if constexpr (std::is_same_v<std::remove_cv_t<Type>,
                                   std::remove_cv_t<TypeAtIndex>>) {
        static_assert(FindUniqueType<Tuple, Type, index + 1>() == kSize,
                      "Duplicate types.");
        return index;
      } else {
        return FindUniqueType<Tuple, Type, index + 1>();
      }
    } else {
      return kSize;
    }
  }

  PacketSequenceStateGeneration(TraceProcessorContext* context,
                                TrackEventSequenceState track_state,
                                bool is_incremental_state_valid)
      : context_(context),
        track_event_sequence_state_(std::move(track_state)),
        is_incremental_state_valid_(is_incremental_state_valid) {}

  PacketSequenceStateGeneration(
      TraceProcessorContext* context,
      InternedFieldMap interned_data,
      TrackEventSequenceState track_event_sequence_state,
      CustomStateArray custom_state,
      TraceBlobView trace_packet_defaults,
      bool is_incremental_state_valid);

  // Add an interned message to this incremental state view. This can only be
  // called by `PacketSequenceStateBuilder' (which is a friend) as packet
  // tokenizers and parsers should never deal directly with reading interned
  // data out of trace packets.
  void InternMessage(uint32_t field_id, TraceBlobView message);

  TraceProcessorContext* const context_;
  InternedFieldMap interned_data_;
  TrackEventSequenceState track_event_sequence_state_;
  CustomStateArray custom_state_;
  std::optional<InternedMessageView> trace_packet_defaults_;
  // TODO(carlscab): Should not be needed as clients of this class should not
  // care about validity.
  bool is_incremental_state_valid_ = true;
};

template <typename T>
std::remove_cv_t<T>* PacketSequenceStateGeneration::GetCustomState() {
  constexpr size_t index = FindUniqueType<CustomStateClasses, T>();
  static_assert(index < std::tuple_size_v<CustomStateClasses>, "Not found");
  auto& ptr = custom_state_[index];
  if (PERFETTO_UNLIKELY(ptr.get() == nullptr)) {
    ptr.reset(new T(context_));
    ptr->set_generation(this);
  }

  return static_cast<std::remove_cv_t<T>*>(ptr.get());
}

}  // namespace perfetto::trace_processor

#endif  // SRC_TRACE_PROCESSOR_IMPORTERS_PROTO_PACKET_SEQUENCE_STATE_GENERATION_H_
