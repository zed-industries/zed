// Copyright 2025 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_ANNOTATION_NODE_ANNOTATION_SELECTOR_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_ANNOTATION_NODE_ANNOTATION_SELECTOR_H_

#include "third_party/blink/renderer/core/annotation/annotation_selector.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/graphics/dom_node_id.h"
#include "third_party/blink/renderer/platform/heap/visitor.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

// NodeAnnotationSelector allows attaching to DOM based on a provided node id.
class CORE_EXPORT NodeAnnotationSelector : public AnnotationSelector {
 public:
  explicit NodeAnnotationSelector(const DOMNodeId node_id);
  ~NodeAnnotationSelector() override = default;

  void Trace(Visitor* visitor) const override;
  // AnnotationSelector Interface
  String Serialize() const override;
  void FindRange(Range& search_range,
                 SearchType type,
                 FinishedCallback finished_cb) override;
  bool IsTextSelector() const override { return false; }

 private:
  DOMNodeId node_id_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_ANNOTATION_NODE_ANNOTATION_SELECTOR_H_
