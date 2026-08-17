// Copyright 2025 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_CANVAS_UNIQUE_FONT_SELECTOR_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_CANVAS_UNIQUE_FONT_SELECTOR_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/fonts/font_description.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/instrumentation/memory_pressure_listener.h"
#include "third_party/blink/renderer/platform/wtf/vector_backed_linked_list.h"

namespace blink {

class Font;
class FontSelector;
class FontSelectorClient;

// A wrapper of blink::FontSelector.
// This class maintains a cache that returns unique blink::Font instances from
// equivalent blink::FontDescription instances.
class CORE_EXPORT UniqueFontSelector
    : public GarbageCollected<UniqueFontSelector>,
      public MemoryPressureListener {
 public:
  UniqueFontSelector(FontSelector* base_selector, bool enable_cache);
  void Trace(Visitor* visitor) const override;

  const Font* FindOrCreateFont(const FontDescription& description);
  void DidSwitchFrame();

  // The return value is nullptr if this UniqueFontSelector was created with
  // a detached Document.
  FontSelector* BaseFontSelector() const { return base_selector_; }
  void RegisterForInvalidationCallbacks(FontSelectorClient* client);

 private:
  // MemoryPressureListener override:
  void OnPurgeMemory() override;

  Member<FontSelector> base_selector_;

  struct CacheValue {
    DISALLOW_NEW();

   public:
    Member<const Font> font;
    wtf_size_t list_index;
    void Trace(Visitor* visitor) const;
  };
  // The main storage of the cache.
  HeapHashMap<FontDescription, CacheValue> font_cache_;

  struct LruListKey {
    FontDescription description;
    uint32_t generation = -1;
  };
  // The LRU part of the cache.  front() points to the most recently used item,
  // and back() points to the least recently used item.
  VectorBackedLinkedList<LruListKey> lru_list_;
  uint32_t frame_generation_ = 0;

  const bool enable_cache_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_CANVAS_UNIQUE_FONT_SELECTOR_H_
