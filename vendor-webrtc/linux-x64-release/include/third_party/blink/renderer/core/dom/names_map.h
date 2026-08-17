// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_DOM_NAMES_MAP_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_DOM_NAMES_MAP_H_

#include <optional>

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/element_rare_data_field.h"
#include "third_party/blink/renderer/core/dom/space_split_string.h"
#include "third_party/blink/renderer/core/dom/space_split_string_wrapper.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string_hash.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

// Parses and stores mappings from part name to ordered set of part names as in
// http://drafts.csswg.org/css-shadow-parts/.
// TODO(crbug/805271): Deduplicate identical maps as SpaceSplitString does so
// that elements with identical exportparts attributes share instances.
class CORE_EXPORT NamesMap : public GarbageCollected<NamesMap>,
                             public ElementRareDataField {
 public:
  NamesMap() = default;
  NamesMap(const NamesMap&) = delete;
  NamesMap& operator=(const NamesMap&) = delete;
  explicit NamesMap(const AtomicString& string);

  // Clears any existing mapping, parses the string and sets the mapping from
  // that.
  void Set(const AtomicString&);
  void Clear() { data_.clear(); }
  // Inserts value into the ordered set under key.
  void Add(const AtomicString& key, const AtomicString& value);
  SpaceSplitString* Get(const AtomicString& key) const;

  size_t size() const { return data_.size(); }

  void Trace(Visitor* visitor) const override {
    visitor->Trace(data_);
    ElementRareDataField::Trace(visitor);
  }

 private:
  template <typename CharacterType>
  void Set(base::span<const CharacterType>);

  HeapHashMap<AtomicString, Member<SpaceSplitStringWrapper>> data_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_DOM_NAMES_MAP_H_
