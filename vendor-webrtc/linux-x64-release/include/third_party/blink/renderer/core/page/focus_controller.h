/*
 * Copyright (C) 2006, 2007 Apple Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE COMPUTER, INC. ``AS IS'' AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL APPLE COMPUTER, INC. OR
 * CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY
 * OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_PAGE_FOCUS_CONTROLLER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_PAGE_FOCUS_CONTROLLER_H_

#include "base/memory/scoped_refptr.h"
#include "third_party/blink/public/mojom/input/focus_type.mojom-blink-forward.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_set.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"

namespace blink {

struct FocusParams;
class ContainerNode;
class Document;
class Element;
class FocusChangedObserver;
class Frame;
class HTMLElement;
class HTMLFrameOwnerElement;
class InputDeviceCapabilities;
class LocalFrame;
class Page;
class RemoteFrame;

class CORE_EXPORT FocusController final
    : public GarbageCollected<FocusController> {
 public:
  using OwnerMap = HeapHashMap<Member<ContainerNode>, Member<Element>>;

  explicit FocusController(Page*);
  FocusController(const FocusController&) = delete;
  FocusController& operator=(const FocusController&) = delete;

  void SetFocusedFrame(Frame*, bool notify_embedder = true);
  void FocusDocumentView(Frame*, bool notify_embedder = true);
  LocalFrame* FocusedFrame() const;
  Frame* FocusedOrMainFrame() const;

  // Clears |focused_frame_| if it's been detached.
  void FrameDetached(Frame* detached_frame);

  // Finds the focused HTMLFrameOwnerElement, if any, in the provided frame.
  // An HTMLFrameOwnerElement is considered focused if the frame it owns, or
  // one of its descendant frames, is currently focused.
  HTMLFrameOwnerElement* FocusedFrameOwnerElement(
      LocalFrame& current_frame) const;

  // Determines whether the provided Document has focus according to
  // http://www.w3.org/TR/html5/editing.html#dom-document-hasfocus
  bool IsDocumentFocused(const Document&) const;

  bool SetInitialFocus(mojom::blink::FocusType);
  bool AdvanceFocus(mojom::blink::FocusType type,
                    InputDeviceCapabilities* source_capabilities = nullptr) {
    return AdvanceFocus(type, false, source_capabilities);
  }
  bool AdvanceFocusAcrossFrames(
      mojom::blink::FocusType,
      RemoteFrame* from,
      LocalFrame* to,
      InputDeviceCapabilities* source_capabilities = nullptr);
  static Element* FindFocusableElementInShadowHost(const Element& shadow_host);
  static HTMLElement* FindScopeOwnerSlotOrReadingFlowContainer(const Element&);

  // Returns the next focusable element (likely an <input> field) after the
  // given element in focus traversal and within the enclosing <form> that
  // requires user input before submitting the form (all <form>less <input>s are
  // considered as one virtual form). Used by an Android virtual keyboard and
  // Autofill to infer whether the enclosing <form> is ready for auto-submission
  // after filling the given element or focus should be firstly moved to the
  // next focusable element.
  Element* NextFocusableElementForImeAndAutofill(Element*,
                                                 mojom::blink::FocusType);
  Element* FindFocusableElementForImeAutofillAndTesting(mojom::blink::FocusType,
                                                        Element&,
                                                        OwnerMap&);

  bool SetFocusedElement(Element*, Frame*, const FocusParams&);
  // |setFocusedElement| variant with SelectionBehaviorOnFocus::None,
  // |kFocusTypeNone, and null InputDeviceCapabilities.
  bool SetFocusedElement(Element*, Frame*);

  void SetActive(bool);
  bool IsActive() const { return is_active_ || is_emulating_focus_; }

  void SetFocused(bool);
  bool IsFocused() const { return is_focused_ || is_emulating_focus_; }

  void SetFocusEmulationEnabled(bool);

  void RegisterFocusChangedObserver(FocusChangedObserver*);

  static int AdjustedTabIndex(const Element&);

  void Trace(Visitor*) const;

 private:
  bool AdvanceFocus(mojom::blink::FocusType,
                    bool initial_focus,
                    InputDeviceCapabilities* source_capabilities = nullptr);
  bool AdvanceFocusInDocumentOrder(
      LocalFrame*,
      Element* start,
      mojom::blink::FocusType,
      bool initial_focus,
      InputDeviceCapabilities* source_capabilities);

  void NotifyFocusChangedObservers() const;

  void ActiveHasChanged();
  void FocusHasChanged();

  Member<Page> page_;
  Member<Frame> focused_frame_;
  bool is_active_;
  bool is_focused_;
  bool is_changing_focused_frame_;
  bool is_emulating_focus_;
  HeapHashSet<WeakMember<FocusChangedObserver>> focus_changed_observers_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_PAGE_FOCUS_CONTROLLER_H_
