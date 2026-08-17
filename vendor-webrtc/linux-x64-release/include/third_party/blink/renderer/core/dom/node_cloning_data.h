// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_DOM_NODE_CLONING_DATA_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_DOM_NODE_CLONING_DATA_H_

#include "base/containers/enum_set.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/child_node_part.h"
#include "third_party/blink/renderer/core/dom/node.h"
#include "third_party/blink/renderer/core/dom/part_root.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_linked_hash_set.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/runtime_enabled_features.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string.h"
#include "third_party/blink/renderer/platform/wtf/wtf_size_t.h"

namespace blink {

enum class CloneOption {
  kIncludeDescendants,
  kPreserveDOMParts,
  kPreserveDOMPartsMinimalAPI,

  // For `CloneOptionSet`.
  kMinValue = kIncludeDescendants,
  kMaxValue = kPreserveDOMPartsMinimalAPI,
};

using CloneOptionSet =
    base::EnumSet<CloneOption, CloneOption::kMinValue, CloneOption::kMaxValue>;

class CORE_EXPORT NodeCloningData final {
  STACK_ALLOCATED();

 public:
  NodeCloningData() : NodeCloningData({}) {}
  NodeCloningData(std::initializer_list<CloneOption> values)
      : clone_options_(values) {}
  NodeCloningData(const NodeCloningData&) = delete;
  NodeCloningData& operator=(const NodeCloningData&) = delete;
  ~NodeCloningData() {}

  bool Has(CloneOption option) const { return clone_options_.Has(option); }
  void Put(CloneOption option) { clone_options_.Put(option); }
  void PushPartRoot(PartRoot& clone) {
    DCHECK(!RuntimeEnabledFeatures::DOMPartsAPIMinimalEnabled());
    cloned_part_root_stack_.push_back(&clone);
  }
  void PopPartRoot(ChildNodePart& expected_top_of_stack) {
    DCHECK(!RuntimeEnabledFeatures::DOMPartsAPIMinimalEnabled());
    if (PartRootStackInvalid()) {
      return;
    }
    PartRoot& top_of_stack = CurrentPartRoot();
    if (&top_of_stack != &expected_top_of_stack) {
      // Mis-nested ChildNodeParts invalidate the clone entirely.
      cloned_part_root_stack_.clear();
      return;
    }

    cloned_part_root_stack_.pop_back();
  }
  bool PartRootStackInvalid() const {
    DCHECK(!RuntimeEnabledFeatures::DOMPartsAPIMinimalEnabled());
    return cloned_part_root_stack_.empty();
  }
  bool PartRootStackHasOnlyDocumentRoot() const {
    DCHECK(!RuntimeEnabledFeatures::DOMPartsAPIMinimalEnabled());
    return cloned_part_root_stack_.size() <= 1;
  }

  PartRoot& CurrentPartRoot() const {
    DCHECK(!RuntimeEnabledFeatures::DOMPartsAPIMinimalEnabled());
    DCHECK(!PartRootStackInvalid());
    return *cloned_part_root_stack_.back();
  }

 private:
  CloneOptionSet clone_options_;
  HeapVector<Member<PartRoot>> cloned_part_root_stack_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_DOM_NODE_CLONING_DATA_H_
