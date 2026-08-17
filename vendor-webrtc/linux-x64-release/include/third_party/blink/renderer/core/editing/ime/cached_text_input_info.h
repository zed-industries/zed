// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_IME_CACHED_TEXT_INPUT_INFO_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_IME_CACHED_TEXT_INPUT_INFO_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/editing/iterators/text_iterator_behavior.h"
#include "third_party/blink/renderer/core/editing/plain_text_range.h"
#include "third_party/blink/renderer/core/editing/position.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class LayoutObject;

// |CachedTextInputInfo| holds plain text, and plain text range for
// composition and selection in |element_| until layout of |element_|
// changed. This class also provides faster version of |PlaintTextRange|.
class CORE_EXPORT CachedTextInputInfo final {
  DISALLOW_NEW();

 public:
  CachedTextInputInfo(const CachedTextInputInfo&) = delete;
  CachedTextInputInfo() = default;

  CachedTextInputInfo& operator=(const CachedTextInputInfo&) = delete;

  void EnsureCached(const ContainerNode& element) const;
  PlainTextRange GetComposition(const EphemeralRange& range) const;
  PlainTextRange GetPlainTextRange(const EphemeralRange& range) const;
  PlainTextRange GetSelection(const EphemeralRange& range) const;
  String GetText() const;
  bool IsValidFor(const ContainerNode& element) const;

  // For cache invalidation
  void DidChangeVisibility(const LayoutObject& layout_object);
  void DidLayoutSubtree(const LayoutObject& layout_object);
  void DidUpdateLayout(const LayoutObject& layout_object);
  void LayoutObjectWillBeDestroyed(const LayoutObject& layout_object);

  void Trace(Visitor*) const;

 private:
  class CachedPlainTextRange final {
    DISALLOW_NEW();

   public:
    CachedPlainTextRange(const CachedPlainTextRange&) = delete;
    CachedPlainTextRange() = default;

    CachedPlainTextRange& operator=(const CachedPlainTextRange&) = delete;

    void Clear();
    PlainTextRange Get() const;
    bool IsValidFor(const EphemeralRange& range) const;
    void Set(const EphemeralRange& range, const PlainTextRange& text_range);

    void Trace(Visitor*) const;

   private:
    // |start_| and |end_| can be disconnected from document.
    mutable Position start_;
    mutable Position end_;
    mutable unsigned start_offset_ = kNotFound;
    mutable unsigned end_offset_ = kNotFound;
  };

  static TextIteratorBehavior Behavior();
  void Clear() const;
  void ClearIfNeeded(const LayoutObject& layout_object);
  PlainTextRange GetPlainTextRangeWithCache(
      const EphemeralRange& range,
      CachedPlainTextRange* text_range) const;
  unsigned RangeLength(const EphemeralRange& range) const;

  mutable Member<const ContainerNode> container_;
  mutable WeakMember<const LayoutObject> layout_object_;
  // Records offset of text node from start of |container_|.
  mutable HeapHashMap<Member<const Text>, unsigned> offset_map_;
  mutable String text_;
  mutable CachedPlainTextRange composition_;
  mutable CachedPlainTextRange selection_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_IME_CACHED_TEXT_INPUT_INFO_H_
