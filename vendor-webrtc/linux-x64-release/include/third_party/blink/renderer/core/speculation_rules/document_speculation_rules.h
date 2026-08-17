// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_SPECULATION_RULES_DOCUMENT_SPECULATION_RULES_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_SPECULATION_RULES_DOCUMENT_SPECULATION_RULES_H_

#include "third_party/blink/public/mojom/speculation_rules/speculation_rules.mojom-blink.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/document.h"
#include "third_party/blink/renderer/core/loader/document_loader.h"
#include "third_party/blink/renderer/core/speculation_rules/speculation_rule_set.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_remote.h"
#include "third_party/blink/renderer/platform/supplementable.h"

namespace blink {

class HTMLAnchorElementBase;
class SpeculationCandidate;
class SpeculationRuleLoader;

// This corresponds to the document's list of speculation rule sets.
//
// Updates are pushed asynchronously.
class CORE_EXPORT DocumentSpeculationRules
    : public GarbageCollected<DocumentSpeculationRules>,
      public Supplement<Document> {
 public:
  static const char kSupplementName[];

  static DocumentSpeculationRules& From(Document&);
  static DocumentSpeculationRules* FromIfExists(Document&);

  explicit DocumentSpeculationRules(Document&);

  const HeapVector<Member<SpeculationRuleSet>>& rule_sets() const {
    return rule_sets_;
  }

  // Appends a newly added rule set.
  void AddRuleSet(SpeculationRuleSet*);

  // Removes a rule set from consideration.
  void RemoveRuleSet(SpeculationRuleSet*);

  void AddSpeculationRuleLoader(SpeculationRuleLoader*);
  void RemoveSpeculationRuleLoader(SpeculationRuleLoader*);

  void LinkInserted(HTMLAnchorElementBase* link);
  void LinkRemoved(HTMLAnchorElementBase* link);
  void HrefAttributeChanged(HTMLAnchorElementBase* link,
                            const AtomicString& old_value,
                            const AtomicString& new_value);
  void ReferrerPolicyAttributeChanged(HTMLAnchorElementBase* link);
  void RelAttributeChanged(HTMLAnchorElementBase* link);
  void TargetAttributeChanged(HTMLAnchorElementBase* link);
  void DocumentReferrerPolicyChanged();
  void DocumentBaseURLChanged();
  void DocumentBaseTargetChanged();
  void LinkMatchedSelectorsUpdated(HTMLAnchorElementBase* link);
  void LinkGainedOrLostComputedStyle(HTMLAnchorElementBase* link);
  void DocumentStyleUpdated();
  void ChildStyleRecalcBlocked(Element* root);
  void DidStyleChildren(Element* root);
  void DisplayLockedElementDisconnected(Element* root);

  void DocumentRestoredFromBFCache();
  void InitiatePreview(const KURL& url);

  const HeapVector<Member<StyleRule>>& selectors() { return selectors_; }

  // Requests a future call to UpdateSpeculationCandidates, if none is yet
  // scheduled.
  void QueueUpdateSpeculationCandidates(bool force_style_update = false);

  void Trace(Visitor*) const override;

 private:
  // Retrieves a valid proxy to the speculation host in the browser.
  // May be null if the execution context does not exist.
  mojom::blink::SpeculationHost* GetHost();

  // Executes in a microtask after QueueUpdateSpeculationCandidates.
  void UpdateSpeculationCandidatesMicrotask();

  // Pushes the current speculation candidates to the browser, immediately.
  // Can be entered either through `UpdateSpeculationCandidatesMicrotask` or
  // `DocumentStyleUpdated`.
  void UpdateSpeculationCandidates();

  // Appends all candidates populated from links in the document (based on
  // document rules in all the rule sets).
  void AddLinkBasedSpeculationCandidates(
      HeapVector<Member<SpeculationCandidate>>& candidates);

  // Initializes |link_map_| with all links in the document by traversing
  // through the document in shadow-including tree order.
  void InitializeIfNecessary();

  // Helper methods that are used to deal with link/document attribute changes
  // that could invalidate the list of speculation candidates.
  void LinkAttributeChanged(HTMLAnchorElementBase* link);
  void DocumentPropertyChanged();

  // Helper methods to modify |link_map_|.
  void AddLink(HTMLAnchorElementBase* link);
  void RemoveLink(HTMLAnchorElementBase* link);
  void InvalidateLink(HTMLAnchorElementBase* link);
  void InvalidateAllLinks();

  // Populates |selectors_| and notifies the StyleEngine.
  void UpdateSelectors();

  // Called when LCP is predicted.
  void OnLCPPredicted(const Element* lcp_candidate);

  // Tracks when the next update to speculation candidates is scheduled to
  // occur. See `SetPendingUpdateState` for details.
  enum class PendingUpdateState : uint8_t {
    kNoUpdate = 0,

    // A microtask to run `UpdateSpeculationRulesMicrotask` is queued.
    // It does not need a forced style update.
    kMicrotaskQueued,

    // Candidates should be updated the next time the style engine updates
    // style.
    kOnNextStyleUpdate,

    // A microtask to run `UpdateSpeculationRulesMicrotask` is queued.
    // It must update style when it does so.
    kMicrotaskQueuedWithForcedStyleUpdate,
  };
  friend std::ostream& operator<<(std::ostream&, const PendingUpdateState&);
  void SetPendingUpdateState(PendingUpdateState state);
  bool IsMicrotaskQueued() const {
    return pending_update_state_ == PendingUpdateState::kMicrotaskQueued ||
           pending_update_state_ ==
               PendingUpdateState::kMicrotaskQueuedWithForcedStyleUpdate;
  }

  HeapVector<Member<SpeculationRuleSet>> rule_sets_;
  HeapMojoRemote<mojom::blink::SpeculationHost> host_;
  HeapHashSet<Member<SpeculationRuleLoader>> speculation_rule_loaders_;

  // The following data structures together keep track of all the links in
  // the document. |matched_links_| contains links that match at least one
  // document rule, and also caches a list of speculation candidates created for
  // that link. |unmatched_links_| are links that are known to not match any
  // document rules. |pending_links_| are links that haven't been matched
  // against all the document rules yet.
  // TODO(crbug.com/1371522): Consider removing |unmatched_links_| and
  // re-traverse the document to find all links when a new ruleset is
  // added/removed.
  HeapHashMap<Member<HTMLAnchorElementBase>,
              Member<GCedHeapVector<Member<SpeculationCandidate>>>>
      matched_links_;
  HeapHashSet<Member<HTMLAnchorElementBase>> unmatched_links_;
  HeapHashSet<Member<HTMLAnchorElementBase>> pending_links_;

  // Links with ComputedStyle that wasn't updated after the most recent style
  // update (due to having a display-locked ancestor).
  HeapHashSet<Member<HTMLAnchorElementBase>> stale_links_;
  HeapHashSet<Member<Element>> elements_blocking_child_style_recalc_;

  // Collects every CSS selector from every CSS selector document rule predicate
  // in this document's speculation rules.
  HeapVector<Member<StyleRule>> selectors_;

  bool initialized_ = false;
  PendingUpdateState pending_update_state_ = PendingUpdateState::kNoUpdate;

  // Set to true if the EventHandlerRegistry has recorded this object's need to
  // observe pointer events.
  // TODO(crbug.com/1425870): This can be deleted when/if these discrete events
  // are no longer filtered by default.
  bool wants_pointer_events_ = false;

  bool first_update_after_restored_from_bfcache_ = false;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_SPECULATION_RULES_DOCUMENT_SPECULATION_RULES_H_
