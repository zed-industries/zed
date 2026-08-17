// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_AUDIO_PARAM_MAP_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_AUDIO_PARAM_MAP_H_

#include "third_party/blink/renderer/bindings/core/v8/maplike.h"
#include "third_party/blink/renderer/bindings/core/v8/v8_binding_for_core.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_sync_iterator_audio_param_map.h"
#include "third_party/blink/renderer/modules/webaudio/audio_param.h"
#include "third_party/blink/renderer/platform/bindings/exception_state.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class AudioParam;

class AudioParamMap final : public ScriptWrappable,
                            public Maplike<AudioParamMap> {
  DEFINE_WRAPPERTYPEINFO();

 public:
  using MapType = HeapHashMap<String, Member<AudioParam>>;

  explicit AudioParamMap(const MapType& parameter_map);

  // IDL attributes / methods
  uint32_t size() const { return parameter_map_.size(); }

  const MapType& GetHashMap() const { return parameter_map_; }

  void Trace(Visitor* visitor) const override {
    visitor->Trace(parameter_map_);
    ScriptWrappable::Trace(visitor);
  }

 private:
  PairSyncIterable<AudioParamMap>::IterationSource* CreateIterationSource(
      ScriptState*,
      ExceptionState&) override;
  bool GetMapEntry(ScriptState*,
                   const String& key,
                   AudioParam*& value,
                   ExceptionState&) override;

  const MapType parameter_map_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_AUDIO_PARAM_MAP_H_
