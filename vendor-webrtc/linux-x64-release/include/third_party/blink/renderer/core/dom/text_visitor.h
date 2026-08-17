// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_DOM_TEXT_VISITOR_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_DOM_TEXT_VISITOR_H_

#include "third_party/blink/renderer/core/core_export.h"

namespace blink {

class Node;

// TextVisitor is called as each node is considered when generating text.
// Note that TextVisitor is not called for all nodes. For example, all
// descendants of hidden nodes are not considered.
class CORE_EXPORT TextVisitor {
 public:
  // Called when `element` is being considered for adding text to the resulting
  // inner-text. `offset` is the current location in the generated text.
  virtual void WillVisit(const Node& element, unsigned offset) = 0;

 protected:
  ~TextVisitor() = default;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_DOM_TEXT_VISITOR_H_
