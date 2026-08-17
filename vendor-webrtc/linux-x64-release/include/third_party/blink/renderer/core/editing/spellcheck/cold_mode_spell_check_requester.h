// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_SPELLCHECK_COLD_MODE_SPELL_CHECK_REQUESTER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_SPELLCHECK_COLD_MODE_SPELL_CHECK_REQUESTER_H_

#include "third_party/blink/renderer/core/editing/forward.h"
#include "third_party/blink/renderer/core/editing/position.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

class Element;
class LocalDOMWindow;
class IdleDeadline;
class SpellCheckRequester;

// This class is only supposed to be used by IdleSpellCheckController in cold
// mode invocation. Not to be confused with SpellCheckRequester. The class
// iteratively checks the editing host currently focused when the document is
// idle.
// See design doc for details: https://goo.gl/zONC3v
class ColdModeSpellCheckRequester
    : public GarbageCollected<ColdModeSpellCheckRequester> {
 public:
  explicit ColdModeSpellCheckRequester(LocalDOMWindow&);
  ColdModeSpellCheckRequester(const ColdModeSpellCheckRequester&) = delete;
  ColdModeSpellCheckRequester& operator=(const ColdModeSpellCheckRequester&) =
      delete;

  void SetNeedsMoreInvocationForTesting() {
    needs_more_invocation_for_testing_ = true;
  }

  void Invoke(IdleDeadline*);

  // Called when code mode checking is currently not needed (due to, e.g., user
  // has resumed active).
  void ClearProgress();

  // Called when document is detached or spellchecking is globally disabled.
  void Deactivate();

  bool FullyCheckedCurrentRootEditable() const;

  bool HasFullyChecked(const Element& element) const {
    return fully_checked_root_editables_.Contains(&element);
  }
  void RemoveFromFullyChecked(const Element& element) {
    fully_checked_root_editables_.erase(&element);
  }

  void ElementRemoved(Element* element);

  void Trace(Visitor*) const;

 private:
  SpellCheckRequester& GetSpellCheckRequester() const;

  const Element* CurrentFocusedEditable() const;

  enum class CheckingType { kNone, kLocal, kFull };
  CheckingType AccumulateTextDeltaAndComputeCheckingType(
      const Element& element_to_check);

  void RequestLocalChecking(const Element& element_to_check);

  void RequestFullChecking(const Element& element_to_check, IdleDeadline*);

  // Returns true if there's anything remaining to check, false otherwise
  bool RequestCheckingForNextChunk();
  void SetHasFullyCheckedCurrentRootEditable();

  // The window this cold mode checker belongs to.
  const Member<LocalDOMWindow> window_;

  // The root editable element checked in the last invocation for full checking.
  // |nullptr| if not invoked yet or didn't find any root editable element for
  // full checking.
  Member<const Element> root_editable_;

  // If |root_editable_| is non-null and hasn't been fully checked, the id of
  // the last checked chunk and the remaining range to check;
  // Otherwise, |kInvalidChunkIndex| and null.
  int last_chunk_index_;
  Member<Range> remaining_check_range_;

  // After fully checking an element, we don't want to repeatedly check its
  // content in full unless a significant amount of change has taken place. We
  // heuristically measure this as the accumulated length change in the element
  // since the last time it was fully checked.
  struct FullyCheckedEditableEntry {
    int previous_checked_length = 0;
    int accumulated_delta = 0;
    uint64_t previous_checked_dom_tree_version = 0u;
  };
  HeapHashMap<WeakMember<const Element>, FullyCheckedEditableEntry>
      fully_checked_root_editables_;

  // A test-only flag for forcing lifecycle advancing.
  mutable bool needs_more_invocation_for_testing_;
};
}

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_SPELLCHECK_COLD_MODE_SPELL_CHECK_REQUESTER_H_
