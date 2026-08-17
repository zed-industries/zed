/*
 * Copyright (C) 2011 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 *     * Redistributions of source code must retain the above copyright
 * notice, this list of conditions and the following disclaimer.
 *     * Neither the name of Google Inc. nor the names of its
 * contributors may be used to endorse or promote products derived from
 * this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_DOM_SHADOW_ROOT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_DOM_SHADOW_ROOT_H_

#include "base/check_op.h"
#include "base/notreached.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/style_sheet_list.h"
#include "third_party/blink/renderer/core/dom/container_node.h"
#include "third_party/blink/renderer/core/dom/document_fragment.h"
#include "third_party/blink/renderer/core/dom/element.h"
#include "third_party/blink/renderer/core/dom/element_rare_data_field.h"
#include "third_party/blink/renderer/core/dom/tree_scope.h"
#include "third_party/blink/renderer/platform/bindings/exception_state.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"

namespace blink {

class Document;
class ExceptionState;
class SetHTMLOptions;
class SetHTMLUnsafeOptions;
class SlotAssignment;
class ReferenceTargetIdObserver;
class V8ShadowRootMode;
class V8SlotAssignmentMode;
class WhitespaceAttacher;

enum class ShadowRootMode { kOpen, kClosed, kUserAgent };

class CORE_EXPORT ShadowRoot final : public DocumentFragment,
                                     public TreeScope,
                                     public ElementRareDataField {
  DEFINE_WRAPPERTYPEINFO();

 public:
  ShadowRoot(Document&, ShadowRootMode, SlotAssignmentMode);
  ~ShadowRoot() override;
  ShadowRoot(const ShadowRoot&) = delete;
  ShadowRoot& operator=(const ShadowRoot&) = delete;

  // Disambiguate between Node and TreeScope hierarchies; TreeScope's
  // implementation is simpler.
  using TreeScope::GetDocument;
  using TreeScope::getElementById;

  // Make protected methods from base class public here.
  using TreeScope::SetDocument;
  using TreeScope::SetParentTreeScope;

  DEFINE_ATTRIBUTE_EVENT_LISTENER(slotchange, kSlotchange)

  Element& host() const {
    DCHECK(ParentOrShadowHostNode());
    return *To<Element>(ParentOrShadowHostNode());
  }
  ShadowRootMode GetMode() const { return static_cast<ShadowRootMode>(mode_); }
  V8ShadowRootMode mode() const;

  bool IsOpen() const { return GetMode() == ShadowRootMode::kOpen; }
  bool IsUserAgent() const { return GetMode() == ShadowRootMode::kUserAgent; }

  bool serializable() const { return serializable_; }
  void setSerializable(bool serializable) { serializable_ = serializable; }

  bool clonable() const { return clonable_; }
  void setClonable(bool clonable) { clonable_ = clonable; }

  InsertionNotificationRequest InsertedInto(ContainerNode&) override;
  void RemovedFrom(ContainerNode&) override;

  void SetNeedsAssignmentRecalc();
  bool NeedsSlotAssignmentRecalc() const;

  // For Internals, don't use this.
  unsigned ChildShadowRootCount() const { return child_shadow_root_count_; }

  void RebuildLayoutTree(WhitespaceAttacher&);
  void DetachLayoutTree(bool performing_reattach) override;

  void RegisterScopedHTMLStyleChild();
  void UnregisterScopedHTMLStyleChild();

  SlotAssignment& GetSlotAssignment() {
    DCHECK(slot_assignment_);
    return *slot_assignment_;
  }

  bool HasSlotAssignment() { return slot_assignment_ != nullptr; }

  HTMLSlotElement* AssignedSlotFor(const Node&);
  void DidAddSlot(HTMLSlotElement&);
  void DidChangeHostChildSlotName(const AtomicString& old_value,
                                  const AtomicString& new_value);

  void DistributeIfNeeded();

  String innerHTML() const;
  void setInnerHTML(const String&, ExceptionState& = ASSERT_NO_EXCEPTION);
  void setHTMLUnsafe(const String& html, ExceptionState&);
  void setHTMLUnsafe(const String& html,
                     SetHTMLUnsafeOptions*,
                     ExceptionState&);
  void setHTML(const String& html, SetHTMLOptions*, ExceptionState&);

  Node* Clone(Document& factory,
              NodeCloningData& data,
              ContainerNode* append_to,
              ExceptionState& append_exception_state) const override;

  void SetDelegatesFocus(bool flag) { delegates_focus_ = flag; }
  bool delegatesFocus() const { return delegates_focus_; }

  void setReferenceTarget(const AtomicString& reference_target);
  const AtomicString& referenceTarget() const;
  Element* referenceTargetElement() const;

  bool IsManualSlotting() const {
    return slot_assignment_mode_ ==
           static_cast<unsigned>(SlotAssignmentMode::kManual);
  }
  bool IsNamedSlotting() const {
    return slot_assignment_mode_ ==
           static_cast<unsigned>(SlotAssignmentMode::kNamed);
  }
  SlotAssignmentMode GetSlotAssignmentMode() const {
    return static_cast<SlotAssignmentMode>(slot_assignment_mode_);
  }
  V8SlotAssignmentMode slotAssignment() const;

  void SetIsDeclarativeShadowRoot(bool flag) {
    DCHECK(!flag || GetMode() == ShadowRootMode::kOpen ||
           GetMode() == ShadowRootMode::kClosed);
    is_declarative_shadow_root_ = flag;
  }
  bool IsDeclarativeShadowRoot() const { return is_declarative_shadow_root_; }

  void SetAvailableToElementInternals(bool flag) {
    DCHECK(!flag || GetMode() == ShadowRootMode::kOpen ||
           GetMode() == ShadowRootMode::kClosed);
    available_to_element_internals_ = flag;
  }
  bool IsAvailableToElementInternals() const {
    return available_to_element_internals_;
  }

  void SetHasFocusgroupAttributeOnDescendant(bool flag) {
    has_focusgroup_attribute_on_descendant_ = flag;
  }
  bool HasFocusgroupAttributeOnDescendant() const {
    return has_focusgroup_attribute_on_descendant_;
  }

  void SetRegistry(CustomElementRegistry*);
  CustomElementRegistry* registry() const { return registry_.Get(); }

  // Revamped scoped custom element registry renames
  // `.registry` to `.customElementRegistry`, and it is read only.
  CustomElementRegistry* customElementRegistry() const override {
    DCHECK(RuntimeEnabledFeatures::ScopedCustomElementRegistryEnabled());
    return registry_.Get();
  }

  bool ContainsShadowRoots() const { return child_shadow_root_count_; }

  void Trace(Visitor*) const override;

 private:
  friend class ReferenceTargetIdObserver;

  void ChildrenChanged(const ChildrenChange&) override;

  SlotAssignment& EnsureSlotAssignment();

  void AddChildShadowRoot() { ++child_shadow_root_count_; }
  void RemoveChildShadowRoot() {
    DCHECK_GT(child_shadow_root_count_, 0u);
    --child_shadow_root_count_;
  }

  void ReferenceTargetChanged();

  Member<SlotAssignment> slot_assignment_;
  Member<CustomElementRegistry> registry_;
  Member<ReferenceTargetIdObserver> reference_target_id_observer_;
  unsigned child_shadow_root_count_ : 16;
  unsigned mode_ : 2;
  unsigned registered_with_parent_shadow_root_ : 1;
  unsigned delegates_focus_ : 1;
  unsigned slot_assignment_mode_ : 1;
  unsigned is_declarative_shadow_root_ : 1;
  unsigned available_to_element_internals_ : 1;
  unsigned has_focusgroup_attribute_on_descendant_ : 1;
  unsigned serializable_ : 1;
  unsigned clonable_ : 1;
};

inline bool Node::IsInUserAgentShadowRoot() const {
  return ContainingShadowRoot() && ContainingShadowRoot()->IsUserAgent();
}

inline ShadowRoot* Node::GetShadowRoot() const {
  auto* this_element = DynamicTo<Element>(this);
  if (!this_element)
    return nullptr;
  return this_element->GetShadowRoot();
}

inline bool IsShadowHost(const Node* node) {
  return node && node->GetShadowRoot();
}

inline bool IsShadowHost(const Node& node) {
  return node.GetShadowRoot();
}

inline bool IsShadowHost(const Element* element) {
  return element && element->GetShadowRoot();
}

inline bool IsShadowHost(const Element& element) {
  return element.GetShadowRoot();
}

inline bool IsAtShadowBoundary(const Element* element) {
  if (!element) {
    return false;
  }
  ContainerNode* parent_node = element->parentNode();
  return parent_node && parent_node->IsShadowRoot();
}

template <>
struct DowncastTraits<ShadowRoot> {
  static bool AllowFrom(const Node& node) { return node.IsShadowRoot(); }

  static bool AllowFrom(const TreeScope& tree_scope) {
    return tree_scope.RootNode().IsShadowRoot();
  }
};

CORE_EXPORT std::ostream& operator<<(std::ostream&, const ShadowRootMode&);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_DOM_SHADOW_ROOT_H_
