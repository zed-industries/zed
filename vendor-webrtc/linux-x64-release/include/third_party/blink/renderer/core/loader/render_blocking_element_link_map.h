// Copyright 2025 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_RENDER_BLOCKING_ELEMENT_LINK_MAP_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_RENDER_BLOCKING_ELEMENT_LINK_MAP_H_

#include "third_party/blink/renderer/core/html/html_link_element.h"
#include "third_party/blink/renderer/core/loader/render_blocking_level.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_set.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/functional.h"

namespace blink {

// Tracks the currently pending render-blocking element ids and the links that
// caused them to be blocking.
// For instance,
//   <link id="link1" rel="expect" href="#target-id1" blocking="render"/>
//   <link id="link2" rel="expect" href="#target-id1" blocking="render"/>
//   <link id="link3" rel="expect" href="#target-id2" blocking="render"/>
//   <link id="link4" rel="expect" href="#target-id3"
//   blocking="full-framerate"/>
// will be stored as the map:
// {
//  render: {
//    "target-id1":{link1, link2},
//    "target-id2":{link3}
//  },
//  full-framerate: {
//    "target-id3":{link4}
//  }
// }.
class CORE_EXPORT RenderBlockingElementLinkMap
    : public GarbageCollected<RenderBlockingElementLinkMap> {
 public:
  using RenderBlockingElementSetEmtpyCallback =
      base::RepeatingCallback<void(RenderBlockingLevel)>;
  explicit RenderBlockingElementLinkMap(
      RenderBlockingElementSetEmtpyCallback
          blocking_elements_set_empty_handler);

  // Read-only iteration through all the links with an active render-blocking
  // element.
  void ForEach(base::RepeatingCallback<void(RenderBlockingLevel level,
                                            const HTMLLinkElement&)> func);

  // Add a <link rel="expect" href="#target_element_id" blocking="level"/>
  // element. The link will not be added if target_element_id is empty.
  void AddLinkWithTargetElement(const AtomicString& target_element_id,
                                const HTMLLinkElement* link,
                                RenderBlockingLevel level);
  // Remove the #target_element_id element, all the related links will be
  // removed.
  void RemoveTargetElement(const AtomicString& target_element_id);
  // Remove the <link rel="expect" href="#target_element_id" blocking="level"/>,
  // the #target_element_id will be removed only when there are no other
  // blocking links referring to the same id.
  void RemoveLinkWithTargetElement(const AtomicString& target_element_id,
                                   const HTMLLinkElement* link);

  // Clear all the links in the map. The render blocking element set empty
  // handler will be called for all the blocking levels.
  void Clear();

  bool HasElement(RenderBlockingLevel level) const;

  void Trace(Visitor* visitor) const;

  ~RenderBlockingElementLinkMap();

 private:
  using ElementLinkMap = GCedHeapHashMap<
      AtomicString,
      Member<GCedHeapHashSet<WeakMember<const HTMLLinkElement>>>>;

  void AddToElementLinkMap(ElementLinkMap* element_link_map,
                           const AtomicString& target_element_id,
                           const HTMLLinkElement* link);
  void RemoveTargetElementFromElementLinkMap(
      ElementLinkMap* element_link_map,
      RenderBlockingLevel level,
      const AtomicString& target_element_id);
  void RemoveLinkFromElementLinkMap(ElementLinkMap* element_link_map,
                                    RenderBlockingLevel level,
                                    const AtomicString& target_element_id,
                                    const HTMLLinkElement* link);
  HeapHashMap<RenderBlockingLevel, Member<ElementLinkMap>>
      element_render_blocking_links_;
  RenderBlockingElementSetEmtpyCallback on_blocking_elements_empty_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_RENDER_BLOCKING_ELEMENT_LINK_MAP_H_
