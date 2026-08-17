// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBMIDI_MIDI_PORT_MAP_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBMIDI_MIDI_PORT_MAP_H_

#include "third_party/blink/renderer/bindings/core/v8/maplike.h"
#include "third_party/blink/renderer/bindings/core/v8/v8_binding_for_core.h"
#include "third_party/blink/renderer/platform/bindings/exception_state.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

template <typename InterfaceType, typename ValueType>
class MIDIPortMap : public ScriptWrappable, public Maplike<InterfaceType> {
 public:
  explicit MIDIPortMap(const HeapVector<Member<ValueType>>& entries)
      : entries_(entries) {}

  // IDL attributes / methods
  uint32_t size() const { return entries_.size(); }

  void Trace(Visitor* visitor) const override {
    visitor->Trace(entries_);
    ScriptWrappable::Trace(visitor);
  }

 private:
  // We use HeapVector here to keep the entry order.
  using Entries = HeapVector<Member<ValueType>>;
  using IteratorType = typename base::CheckedContiguousIterator<
      const typename Entries::ValueType>;

  typename PairSyncIterable<InterfaceType>::IterationSource*
  CreateIterationSource(ScriptState*, ExceptionState&) override {
    return MakeGarbageCollected<MapIterationSource>(
        this, entries_.CheckedBegin(), entries_.CheckedEnd());
  }

  bool GetMapEntry(ScriptState*,
                   const String& key,
                   ValueType*& value,
                   ExceptionState&) override {
    // FIXME: This function is not O(1). Perhaps it's OK because in typical
    // cases not so many ports are connected.
    for (const auto& p : entries_) {
      if (key == p->id()) {
        value = p;
        return true;
      }
    }
    return false;
  }

  // Note: This template class relies on the fact that m_map.m_entries will
  // never be modified once it is created.
  class MapIterationSource final
      : public PairSyncIterable<InterfaceType>::IterationSource {
   public:
    MapIterationSource(MIDIPortMap<InterfaceType, ValueType>* map,
                       IteratorType iterator,
                       IteratorType end)
        : map_(map), iterator_(iterator), end_(end) {}

    bool FetchNextItem(ScriptState* script_state,
                       String& key,
                       ValueType*& value,
                       ExceptionState&) override {
      if (iterator_ == end_)
        return false;
      key = (*iterator_)->id();
      value = *iterator_;
      ++iterator_;
      return true;
    }

    void Trace(Visitor* visitor) const override {
      visitor->Trace(map_);
      PairSyncIterable<InterfaceType>::IterationSource::Trace(visitor);
    }

   private:
    // map_ is stored just for keeping it alive. It needs to be kept
    // alive while JavaScript holds the iterator to it.
    const Member<const MIDIPortMap<InterfaceType, ValueType>> map_;
    IteratorType iterator_;
    const IteratorType end_;
  };

  const Entries entries_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBMIDI_MIDI_PORT_MAP_H_
