// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_PART_NAMES_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_PART_NAMES_H_

#include "base/memory/stack_allocated.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/wtf/hash_set.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string_hash.h"
#include "third_party/blink/renderer/platform/wtf/text/string_hash.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class NamesMap;
class SpaceSplitString;

// Represents a set of part names as we ascend through scopes and apply
// partmaps. A single partmap is applied by looking up each name in the map and
// taking the union of all of the values found (which are sets of names). This
// becomes the new set of names. Multiple partmaps are applied in succession.
class PartNames {
  STACK_ALLOCATED();

 public:
  PartNames();
  explicit PartNames(const SpaceSplitString& names);
  ~PartNames() { pending_maps_.clear(); }
  PartNames(const PartNames&) = delete;
  PartNames& operator=(const PartNames&) = delete;
  // Adds a new map to be applied. It does that apply the map and update the set
  // of names immediately. That will only be done if actually needed.
  //
  // This captures a reference to names_map.
  void PushMap(const NamesMap& names_map);
  // Returns true if name is included in the set. Applies any pending maps
  // before checking.
  bool Contains(const AtomicString& name);
  // Returns the number of part names in the set.
  size_t size();

 private:
  // Really updates the set as described in ApplyMap.
  void ApplyMap(const NamesMap& names_map);

  HashSet<AtomicString> names_;
  // A queue of maps that have been passed to ApplyMap but not yet
  // applied. These will be applied only if Contains is eventually called.
  HeapVector<Member<const NamesMap>> pending_maps_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_PART_NAMES_H_
