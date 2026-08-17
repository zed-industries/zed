// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HIGHLIGHT_HIGHLIGHT_REGISTRY_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HIGHLIGHT_HIGHLIGHT_REGISTRY_H_

#include "third_party/blink/renderer/bindings/core/v8/maplike.h"
#include "third_party/blink/renderer/bindings/core/v8/v8_sync_iterator_highlight_registry.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/highlight/highlight.h"
#include "third_party/blink/renderer/core/highlight/highlight_registry_map_entry.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/supplementable.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string_hash.h"

namespace blink {

// Using LinkedHashSet<HighlightRegistryMapEntry> to store the map entries
// because order of insertion needs to be preserved (for iteration and breaking
// priority ties during painting) and there's no generic LinkedHashMap. Note
// that the hash functions for HighlightRegistryMapEntry don't allow storing
// more than one entry with the same key (highlight name).
using HighlightRegistryMap =
    HeapLinkedHashSet<Member<HighlightRegistryMapEntry>>;
using HighlightRegistryMapIterable = Maplike<HighlightRegistry>;
class HighlightsFromPointOptions;
class LocalFrame;
class Text;

class CORE_EXPORT HighlightRegistry : public ScriptWrappable,
                                      public Supplement<LocalDOMWindow>,
                                      public HighlightRegistryMapIterable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static const char kSupplementName[];
  static HighlightRegistry* From(LocalDOMWindow&);

  explicit HighlightRegistry(LocalDOMWindow&);
  ~HighlightRegistry() override;

  void Trace(blink::Visitor*) const override;

  static HighlightRegistry* GetHighlightRegistry(const Node* node);

  void SetForTesting(AtomicString, Highlight*);
  void RemoveForTesting(AtomicString, Highlight*);
  HighlightRegistry* setForBinding(ScriptState*,
                                   AtomicString,
                                   Member<Highlight>,
                                   ExceptionState&);
  void clearForBinding(ScriptState*, ExceptionState&);
  bool deleteForBinding(ScriptState*, const AtomicString&, ExceptionState&);
  wtf_size_t size() const { return highlights_.size(); }

  const HighlightRegistryMap& GetHighlights() const { return highlights_; }
  const HashSet<AtomicString>& GetActiveHighlights(const Text& node) const;
  void ValidateHighlightMarkers();
  void ScheduleRepaint();

  enum OverlayStackingPosition {
    kOverlayStackingPositionBelow = -1,
    kOverlayStackingPositionEquivalent = 0,
    kOverlayStackingPositionAbove = 1,
  };

  // Compares Highlights by priority and breaks ties by order of insertion to
  // the registry: a higher priority takes precedence, and in the case
  // priorities are the same, the most recently registered Highlight takes
  // precedence.
  int8_t CompareOverlayStackingPosition(
      const AtomicString& highlight_name1,
      const AtomicString& highlight_name2) const;

  class IterationSource final
      : public HighlightRegistryMapIterable::IterationSource {
   public:
    explicit IterationSource(const HighlightRegistry& highlight_registry);

    bool FetchNextItem(ScriptState* script_state,
                       String& key,
                       Highlight*& value,
                       ExceptionState& exception_state) override;

    void Trace(blink::Visitor*) const override;

   private:
    wtf_size_t index_;
    HeapVector<Member<HighlightRegistryMapEntry>> highlights_snapshot_;
  };

  HeapVector<Member<Highlight>> highlightsFromPoint(
      float x,
      float y,
      const HighlightsFromPointOptions* options);

 private:
  bool IsAbstractRangePaintable(AbstractRange*, Document*) const;

  HighlightRegistryMap highlights_;
  Member<LocalFrame> frame_;
  // Only valid after ValidateHighlightMarkers(), used to optimize painting.
  HeapHashMap<WeakMember<const Text>, HashSet<AtomicString>>
      active_highlights_in_node_;
  uint64_t dom_tree_version_for_validate_highlight_markers_ = 0;
  uint64_t style_version_for_validate_highlight_markers_ = 0;
  bool force_markers_validation_ = true;
  // Number of Highlights registered so far during the lifetime of this
  // HighlightRegistry. Used to store this information for every Highlight
  // registered in order to break ties when determining Highlight precedence.
  uint64_t highlights_registered_ = 0;

  HighlightRegistryMap::iterator GetMapIterator(const AtomicString& key) const {
    return highlights_.Find<HighlightRegistryMapEntryNameTranslator>(key);
  }

  bool GetMapEntry(ScriptState*,
                   const String& key,
                   Highlight*& value,
                   ExceptionState&) override {
    auto iterator = GetMapIterator(AtomicString(key));
    if (iterator == highlights_.end())
      return false;

    value = iterator->Get()->highlight;
    return true;
  }

  HighlightRegistryMapIterable::IterationSource* CreateIterationSource(
      ScriptState*,
      ExceptionState&) override;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HIGHLIGHT_HIGHLIGHT_REGISTRY_H_
