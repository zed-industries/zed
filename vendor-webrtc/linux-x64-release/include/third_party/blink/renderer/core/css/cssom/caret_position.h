// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_CARET_POSITION_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_CARET_POSITION_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/member.h"

namespace blink {

class DOMRect;
class Node;

class CORE_EXPORT CaretPosition : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  CaretPosition(Node* node, unsigned offset);
  Node* offsetNode() const;
  unsigned offset() const;
  DOMRect* getClientRect() const;
  void Trace(Visitor*) const override;

 private:
  Member<Node> node_;
  unsigned offset_;
};
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_CARET_POSITION_H_
