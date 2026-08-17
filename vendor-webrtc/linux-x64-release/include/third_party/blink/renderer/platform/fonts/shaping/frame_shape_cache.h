// Copyright 2025 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_SHAPING_FRAME_SHAPE_CACHE_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_SHAPING_FRAME_SHAPE_CACHE_H_

#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/text/text_direction.h"
#include "third_party/blink/renderer/platform/wtf/text/string_hash.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/blink/renderer/platform/wtf/vector_backed_linked_list.h"
#include "ui/gfx/geometry/rect_f.h"

namespace blink {

class PlainTextItem;
class PlainTextNode;
class ShapeResult;

// FrameShapeCache manages a cache for PlainTextNode, and a cache for
// ShapeResult and ink bounds. A PlainTextPainter instance owns multiple
// FrameShapeCache instances, and each FrameShapeCache instance is associated
// to a specific `Font`.
//
// The caches are aware of frames distinguished by DidSwitchFrame() calls. This
// allows the caches to purge old entries when the frame is switched.
class PLATFORM_EXPORT FrameShapeCache
    : public GarbageCollected<FrameShapeCache> {
 public:
  FrameShapeCache();
  void Trace(Visitor* visitor) const;

  FrameShapeCache(const FrameShapeCache&) = delete;
  FrameShapeCache& operator=(const FrameShapeCache&) = delete;

  // This function should be called between the end of an animation frame and
  // the beginning of the next animation frame.
  void DidSwitchFrame();

  // Cache entry for the PlainTextNode cache.
  // Clients must not access `list_index`.
  struct NodeEntry {
    DISALLOW_NEW();

   public:
    // Cached data.
    Member<PlainTextNode> node;
    // A field for LRU management. It's kNotFound for entries created
    // in the initial frame.
    wtf_size_t list_index;

    void Trace(Visitor* visitor) const;
  };

  // Find a PlainTextNode cache entry for the specified `text` and `direction`.
  // If it's not found, new entry is created.
  NodeEntry* FindOrCreateNodeEntry(const String& text, TextDirection direction);

  // This should be called if FindOrCreateNodeEntry() didn't find an
  // existing entry.
  void RegisterNodeEntry(const String& text,
                         TextDirection direction,
                         PlainTextNode* node,
                         NodeEntry* entry);

  // Cache entry for the ShapeResult cache.
  // Clients must not access `list_index`.
  struct ShapeEntry {
    DISALLOW_NEW();

   public:
    // Cached data.
    Member<const ShapeResult> shape_result;
    gfx::RectF ink_bounds;
    // A field for LRU management. It's kNotFound for entries created
    // in the initial frame.
    wtf_size_t list_index;

    void Trace(Visitor* visitor) const;
  };

  // Find a ShapeResult cache entry for the specified `text` and `direction`.
  // If it's not found, new entry is created.
  ShapeEntry* FindOrCreateShapeEntry(const String& word,
                                     TextDirection direction);

  // This should be called if FindOrCreateShapeEntry() didn't find an
  // existing entry.
  void RegisterShapeEntry(const PlainTextItem& item, ShapeEntry* entry);

 private:
  using KeyType = std::pair<String, TextDirection>;
  struct ListKey {
    KeyType key;
    uint32_t generation;
  };
  using LruList = VectorBackedLinkedList<ListKey>;

  template <typename E>
  E* FindOrCreateEntry(const String& text,
                       TextDirection direction,
                       HeapHashMap<KeyType, E>& map,
                       LruList& lru_list);
  wtf_size_t ListIndexForNewEntry(const String& text,
                                  TextDirection direction,
                                  LruList& lru_list);
  template <typename E>
  void RemoveOldEntries(HeapHashMap<KeyType, E>& map, LruList& lru_list);

  HeapHashMap<KeyType, NodeEntry> node_map_;
  LruList node_lru_list_;

  HeapHashMap<KeyType, ShapeEntry> shape_map_;
  LruList shape_lru_list_;

  static constexpr uint32_t kInitialFrame = 0;
  uint32_t frame_generation_ = kInitialFrame;

  // True if a cache has a new entry in the current frame.
  bool added_new_entries_ = false;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_FONTS_SHAPING_FRAME_SHAPE_CACHE_H_
