// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HIGHLIGHT_HIGHLIGHT_REGISTRY_MAP_ENTRY_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HIGHLIGHT_HIGHLIGHT_REGISTRY_MAP_ENTRY_H_

#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string_hash.h"

namespace blink {

class Highlight;

struct HighlightRegistryMapEntry final
    : public GarbageCollected<HighlightRegistryMapEntry> {
  HighlightRegistryMapEntry(const AtomicString& highlight_name,
                            Member<Highlight> highlight,
                            uint64_t registration_position)
      : highlight(highlight),
        highlight_name(highlight_name),
        registration_position(registration_position) {}
  explicit HighlightRegistryMapEntry(const HighlightRegistryMapEntry* entry)
      : HighlightRegistryMapEntry(entry->highlight_name,
                                  entry->highlight,
                                  entry->registration_position) {}

  void Trace(blink::Visitor* visitor) const { visitor->Trace(highlight); }

  Member<Highlight> highlight = nullptr;
  AtomicString highlight_name = g_null_atom;

  // Used to break ties when comparing two Highlights for precedence. This
  // number means |highlight| was the |registration_position|-th Highlight
  // registered in the HighlightRegistry.
  uint64_t registration_position = 0;
};

// Translator used for looking up a HighlightRegistryMapEntry using only the
// name. Use with the special Find<Translator>() on the highlights set.
struct HighlightRegistryMapEntryNameTranslator {
  STATIC_ONLY(HighlightRegistryMapEntryNameTranslator);

  static unsigned GetHash(const AtomicString& name) {
    return WTF::GetHash(name);
  }
  static bool Equal(const HighlightRegistryMapEntry* entry,
                    const AtomicString& name) {
    DCHECK(entry);
    return HashTraits<AtomicString>::Equal(entry->highlight_name, name);
  }
};

}  // namespace blink

namespace WTF {

template <>
struct HashTraits<blink::Member<blink::HighlightRegistryMapEntry>>
    : MemberHashTraits<blink::HighlightRegistryMapEntry> {
  // Note that GetHash and Equal only take into account the |highlight_name|
  // because |HighlightRegistryMapEntry| is used for storing map entries
  // inside a set (i.e. there can only be one map entry in the set with the
  // same key which is |highlight_name|).
  static inline unsigned GetHash(
      const blink::Member<blink::HighlightRegistryMapEntry>& key) {
    DCHECK(key);
    return WTF::GetHash(key->highlight_name);
  }
  static inline bool Equal(
      const blink::Member<blink::HighlightRegistryMapEntry>& a,
      const blink::Member<blink::HighlightRegistryMapEntry>& b) {
    DCHECK(a && b);
    return HashTraits<AtomicString>::Equal(a->highlight_name,
                                           b->highlight_name);
  }

  static constexpr bool kSafeToCompareToEmptyOrDeleted = false;
};

}  // namespace WTF

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HIGHLIGHT_HIGHLIGHT_REGISTRY_MAP_ENTRY_H_
