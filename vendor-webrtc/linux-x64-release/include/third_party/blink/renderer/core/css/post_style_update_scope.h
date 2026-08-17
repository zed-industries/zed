// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_POST_STYLE_UPDATE_SCOPE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_POST_STYLE_UPDATE_SCOPE_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_set.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class Element;
class Document;
class ComputedStyle;
class CSSAnimationUpdate;

// PostStyleUpdateScope applies pending animation, and initiates clearing of the
// focused element, on destruction, if it is the *current* scope. A
// PostStyleUpdateScope becomes the current scope upon construction if there
// isn't one already.
class CORE_EXPORT PostStyleUpdateScope {
  STACK_ALLOCATED();

 public:
  explicit PostStyleUpdateScope(Document&);
  ~PostStyleUpdateScope();

  class AnimationData {
    STACK_ALLOCATED();

   public:
    // Set a pending CSSAnimationUpdate for a given Element.
    //
    // The update will be automatically applied when the owning
    // PostStyleUpdateScope object goes out of scope.
    void SetPendingUpdate(Element&, const CSSAnimationUpdate&);

    // When calculating transition updates, we need the old style of the element
    // to set up the transition correctly. Container queries can cause the style
    // to be calculated (and replaced on Element) multiple times before we have
    // the final after-change ComputedStyle, hence we need to store the
    // "original" old style for affected elements in order to avoid triggering
    // transitions based on some abandoned and intermediate ComputedStyle.
    //
    // This function takes the current ComputedStyle of the element, and stores
    // it as the old style. If an old style was already stored for this Element,
    // this function does nothing.
    //
    // The old styles remain until the PostStyleUpdateScope object goes out of
    // scope.
    void StoreOldStyleIfNeeded(Element&);

    // If an old-style was previously stored using StoreOldStyleIfNeeded,
    // this function returns that ComputedStyle. Otherwise returns the current
    // ComputedStyle on the Element.
    const ComputedStyle* GetOldStyle(const Element&) const;

    bool HasOldStyles() const { return !old_styles_.empty(); }

   private:
    friend class PostStyleUpdateScope;
    friend class ContainerQueryTest;
    friend class StyleResolverTest;

    HeapHashSet<Member<Element>> elements_with_pending_updates_;
    HeapHashMap<Member<const Element>, Member<const ComputedStyle>> old_styles_;
  };

  class PseudoData {
    STACK_ALLOCATED();

   public:
    // Add a pending ::backdrop update for a given originating element.
    //
    // This is required when a ::backdrop exists on a container query container:
    // Since ::backdrop comes *before* the originating element in the layout
    // tree, it is not possible to correctly update ::backdrop pseudo-elements
    // in a single pass if the originating element is the container. Therefore
    // "conditional" ::backdrop pseudo-elements handled in a follow-up
    // style/layout pass.
    void AddPendingBackdrop(Element& originating_element);

   private:
    friend class PostStyleUpdateScope;

    HeapVector<Member<Element>> pending_backdrops_;
  };

  static AnimationData* CurrentAnimationData();
  static PseudoData* CurrentPseudoData();

  static bool InPendingPseudoUpdate() {
    return current_ && !current_->GetPseudoData();
  }

  // If there is a CurrentAnimationData() and old-style was previously stored
  // using StoreOldStyleIfNeeded, this function returns that ComputedStyle.
  // Otherwise returns the current ComputedStyle on the Element.
  static const ComputedStyle* GetOldStyle(const Element&);

  // Apply side-effects from the style update, e.g. starting and stopping
  // animations.
  //
  // A return value of true means that style needs to be updated again.
  // This can happen for e.g. ::backdrop pseudo-elements in container queries
  // (see PseudoData::AddPendingBackdrop).
  bool Apply();

 private:
  Document& document_;
  // Note that |animation_data_| is only used if the PostStyleUpdateScope is the
  // current scope. Otherwise it will remain empty.
  AnimationData animation_data_;
  PseudoData pseudo_data_;

  // Set to true by ApplyPseudo to prevent subsequent style recalc passes from
  // adding things to PseudoData (which could cause infinite loops).
  bool nullify_pseudo_data_ = false;

  bool ApplyPseudo();
  void ApplyAnimations();

  PseudoData* GetPseudoData() {
    return nullify_pseudo_data_ ? nullptr : &pseudo_data_;
  }

  static PostStyleUpdateScope* current_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_POST_STYLE_UPDATE_SCOPE_H_
