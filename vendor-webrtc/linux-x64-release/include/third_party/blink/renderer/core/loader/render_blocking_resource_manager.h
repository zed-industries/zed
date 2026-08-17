// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_RENDER_BLOCKING_RESOURCE_MANAGER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_RENDER_BLOCKING_RESOURCE_MANAGER_H_

#include "base/time/time.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/element.h"
#include "third_party/blink/renderer/core/loader/render_blocking_element_link_map.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_set.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/timer.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string_hash.h"

namespace blink {

class Document;
class FontFace;
class PendingLinkPreload;
class Node;
class ScriptElementBase;
class HTMLLinkElement;

// https://html.spec.whatwg.org/#render-blocking-mechanism with some extensions.
class CORE_EXPORT RenderBlockingResourceManager final
    : public GarbageCollected<RenderBlockingResourceManager> {
 public:
  explicit RenderBlockingResourceManager(Document&);
  ~RenderBlockingResourceManager() = default;

  RenderBlockingResourceManager(const RenderBlockingResourceManager&) = delete;
  RenderBlockingResourceManager& operator=(
      const RenderBlockingResourceManager&) = delete;

  bool HasRenderBlockingResources() const {
    return HasNonFontRenderBlockingResources() || HasRenderBlockingFonts();
  }
  bool HasNonFontRenderBlockingResources() const {
    return pending_stylesheet_owner_nodes_.size() || pending_scripts_.size() ||
           element_render_blocking_links_->HasElement(
               RenderBlockingLevel::kBlock);
  }
  bool HasRenderBlockingFonts() const {
    return pending_font_preloads_.size() || imperative_font_loading_count_;
  }
  bool HasPendingStylesheets() const {
    return pending_stylesheet_owner_nodes_.size();
  }

  void WillInsertDocumentBody();

  // Returns true if the sheet is successfully added as a render-blocking
  // resource.
  bool AddPendingStylesheet(const Node& owner_node);
  // If the sheet is a render-blocking resource, removes it and returns true;
  // otherwise, returns false with no operation.
  bool RemovePendingStylesheet(const Node& owner_node);

  void AddPendingScript(const ScriptElementBase& script);
  void RemovePendingScript(const ScriptElementBase& script);

  // We additionally allow font preloading (via <link rel="preload"> or Font
  // Loading API) to block rendering for a short period, so that preloaded fonts
  // have a higher chance to be used by the first paint.
  // Design doc: https://bit.ly/36E8UKB
  void AddPendingFontPreload(const PendingLinkPreload& link);
  void RemovePendingFontPreload(const PendingLinkPreload& link);

  void AddImperativeFontLoading(FontFace*);
  void RemoveImperativeFontLoading();
  void EnsureStartFontPreloadMaxBlockingTimer();
  void EnsureStartFontPreloadMaxFCPDelayTimer();
  void FontPreloadingTimerFired(TimerBase*);

  void AddPendingParsingElementLink(const AtomicString& id,
                                    const HTMLLinkElement* element,
                                    RenderBlockingLevel blocking_level);
  void RemovePendingParsingElement(const AtomicString& id, Element* element);
  void RemovePendingParsingElementLink(const AtomicString& id,
                                       const HTMLLinkElement* element);
  void ClearPendingParsingElements();

  void Trace(Visitor* visitor) const;

 private:
  friend class RenderBlockingResourceManagerTest;

  void OnRenderBlockingElementLinkEmpty(RenderBlockingLevel level);
  void RenderBlockingResourceUnblocked();

  // Exposed to unit tests only.
  void SetFontPreloadTimeoutForTest(base::TimeDelta timeout);
  void DisableFontPreloadTimeoutForTest();
  bool FontPreloadTimerIsActiveForTest() const;

  // Tracks the currently loading top-level stylesheets which block
  // rendering from starting. Sheets loaded using the @import directive are not
  // directly included in this set. See:
  // https://html.spec.whatwg.org/multipage/links.html#link-type-stylesheet
  // https://html.spec.whatwg.org/multipage/semantics.html#update-a-style-block
  HeapHashSet<WeakMember<const Node>> pending_stylesheet_owner_nodes_;

  // Tracks the currently pending render-blocking script elements.
  HeapHashSet<WeakMember<const ScriptElementBase>> pending_scripts_;

  // Tracks the currently pending render-blocking font preloads.
  HeapHashSet<WeakMember<const PendingLinkPreload>> pending_font_preloads_;

  // Tracks the currently pending render-blocking element ids and the links that
  // caused them to be blocking.
  Member<RenderBlockingElementLinkMap> element_render_blocking_links_;

  Member<Document> document_;

  unsigned imperative_font_loading_count_ = 0;

  HeapTaskRunnerTimer<RenderBlockingResourceManager>
      font_preload_max_blocking_timer_;
  HeapTaskRunnerTimer<RenderBlockingResourceManager>
      font_preload_max_fcp_delay_timer_;
  base::TimeDelta font_preload_timeout_;
  bool font_preload_timer_has_fired_ = false;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_RENDER_BLOCKING_RESOURCE_MANAGER_H_
