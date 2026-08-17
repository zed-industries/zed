// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_KEYBOARD_KEYBOARD_LAYOUT_MAP_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_KEYBOARD_KEYBOARD_LAYOUT_MAP_H_

#include "third_party/blink/renderer/bindings/core/v8/maplike.h"
#include "third_party/blink/renderer/bindings/core/v8/v8_binding_for_core.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_sync_iterator_keyboard_layout_map.h"
#include "third_party/blink/renderer/platform/bindings/exception_state.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class KeyboardLayoutMap final : public ScriptWrappable,
                                public Maplike<KeyboardLayoutMap> {
  DEFINE_WRAPPERTYPEINFO();

 public:
  explicit KeyboardLayoutMap(const HashMap<String, String>& map);

  const HashMap<String, String>& Map() const { return layout_map_; }

  // IDL attributes / methods
  uint32_t size() const { return layout_map_.size(); }

  void Trace(Visitor* visitor) const override {
    ScriptWrappable::Trace(visitor);
  }

 private:
  // Maplike implementation.
  PairSyncIterable<KeyboardLayoutMap>::IterationSource* CreateIterationSource(
      ScriptState*,
      ExceptionState&) override;
  bool GetMapEntry(ScriptState*,
                   const String& key,
                   String& value,
                   ExceptionState&) override;

  const HashMap<String, String> layout_map_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_KEYBOARD_KEYBOARD_LAYOUT_MAP_H_
