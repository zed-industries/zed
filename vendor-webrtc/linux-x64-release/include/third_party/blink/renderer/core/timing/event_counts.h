// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_TIMING_EVENT_COUNTS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_TIMING_EVENT_COUNTS_H_

#include "third_party/blink/renderer/bindings/core/v8/maplike.h"
#include "third_party/blink/renderer/bindings/core/v8/v8_binding_for_core.h"
#include "third_party/blink/renderer/bindings/core/v8/v8_sync_iterator_event_counts.h"
#include "third_party/blink/renderer/platform/bindings/exception_state.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string_hash.h"

namespace blink {

class EventCounts final : public ScriptWrappable, public Maplike<EventCounts> {
  DEFINE_WRAPPERTYPEINFO();

 public:
  EventCounts();

  const HashMap<AtomicString, uint64_t>& Map() const {
    return event_count_map_;
  }

  // IDL attributes / methods
  wtf_size_t size() const { return event_count_map_.size(); }

  void Add(const AtomicString& event_type);

  // Add multiple events with the same event type.
  void AddMultipleEvents(const AtomicString& event_type, uint64_t count);

  void Trace(Visitor* visitor) const override {
    ScriptWrappable::Trace(visitor);
  }

 private:
  // Maplike implementation.
  PairSyncIterable<EventCounts>::IterationSource* CreateIterationSource(
      ScriptState*,
      ExceptionState&) override;
  bool GetMapEntry(ScriptState*,
                   const String& key,
                   uint64_t& value,
                   ExceptionState&) override;

  HashMap<AtomicString, uint64_t> event_count_map_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_TIMING_EVENT_COUNTS_H_
