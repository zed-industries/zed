// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_TIMING_LAYOUT_SHIFT_ATTRIBUTION_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_TIMING_LAYOUT_SHIFT_ATTRIBUTION_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/member.h"

namespace blink {

class DOMRectReadOnly;
class Node;
class ScriptObject;
class ScriptState;

class CORE_EXPORT LayoutShiftAttribution : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static LayoutShiftAttribution* Create(Node*,
                                        DOMRectReadOnly* previous,
                                        DOMRectReadOnly* current);
  LayoutShiftAttribution(Node*,
                         DOMRectReadOnly* previous,
                         DOMRectReadOnly* current);
  ~LayoutShiftAttribution() override;

  Node* node() const;
  // Return node_ unconditionally, skipping the checks that apply
  // to exposing it through bindings.
  Node* rawNodeForInspector() const;
  DOMRectReadOnly* previousRect() const;
  DOMRectReadOnly* currentRect() const;

  ScriptObject toJSONForBinding(ScriptState*) const;
  void Trace(Visitor*) const override;

  WeakMember<Node> node_;
  Member<DOMRectReadOnly> previous_rect_;
  Member<DOMRectReadOnly> current_rect_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_TIMING_LAYOUT_SHIFT_ATTRIBUTION_H_
