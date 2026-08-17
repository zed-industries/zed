// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_DOM_LIVE_NODE_LIST_REGISTRY_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_DOM_LIVE_NODE_LIST_REGISTRY_H_

#include <algorithm>

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/node_list_invalidation_type.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class LiveNodeListBase;

// Weakly holds (node list, invalidation type) pairs, and allows efficient
// queries of whether nodes matching particular invalidation types are present.
// Entries are automatically removed when a node list is collected by the GC.
//
// Adding elements and querying are both efficient, and the data structure is
// reasonably compact (and attempts to remain so). Removal (especially manual
// removal) is somewhat expensive, but expected to be infrequent.
//
// It is invalid to add a (list, type) pair that is already present, or to
// remove one which is not.
class CORE_EXPORT LiveNodeListRegistry {
  DISALLOW_NEW();

 public:
  LiveNodeListRegistry() = default;
  LiveNodeListRegistry(const LiveNodeListRegistry&) = delete;
  LiveNodeListRegistry& operator=(const LiveNodeListRegistry&) = delete;
  void Add(const LiveNodeListBase*, NodeListInvalidationType);
  void Remove(const LiveNodeListBase*, NodeListInvalidationType);

  bool IsEmpty() const { return mask_ == 0; }

  bool ContainsInvalidationType(NodeListInvalidationType type) const {
    return mask_ & MaskForInvalidationType(type);
  }

  bool NeedsInvalidateOnAttributeChange() const {
    return mask_ != 0 &&
           mask_ != MaskForInvalidationType(kDoNotInvalidateOnAttributeChanges);
  }

  void Trace(Visitor*) const;

 private:
  static inline unsigned MaskForInvalidationType(
      NodeListInvalidationType type) {
    return 1u << type;
  }

  void RecomputeMask();

  // Removes any entries corresponding to node lists which have been collected
  // by the GC, and updates the mask accordingly.
  void ProcessCustomWeakness(const LivenessBroker&);

  // We use UntracedMember<> here to recalculate the mask in
  // |ProcessCustomWeakness()|.
  using Entry = std::pair<UntracedMember<const LiveNodeListBase>, unsigned>;
  Vector<Entry> data_;

  unsigned mask_ = 0;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_DOM_LIVE_NODE_LIST_REGISTRY_H_
