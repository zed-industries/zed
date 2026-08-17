// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_DOM_ELEMENT_RARE_DATA_VECTOR_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_DOM_ELEMENT_RARE_DATA_VECTOR_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/element_rare_data_field.h"
#include "third_party/blink/renderer/core/dom/explicitly_set_attr_elements_map.h"
#include "third_party/blink/renderer/core/dom/focusgroup_flags.h"
#include "third_party/blink/renderer/core/dom/has_invalidation_flags.h"
#include "third_party/blink/renderer/core/dom/node_rare_data.h"
#include "third_party/blink/renderer/core/dom/pseudo_element.h"
#include "third_party/blink/renderer/core/dom/pseudo_element_data.h"
#include "third_party/blink/renderer/platform/heap/trace_traits.h"
#include "third_party/blink/renderer/platform/region_capture_crop_id.h"
#include "third_party/blink/renderer/platform/restriction_target_id.h"
#include "third_party/blink/renderer/platform/sparse_vector.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string.h"
#include "third_party/blink/renderer/platform/wtf/type_traits.h"

namespace blink {

class CSSStyleDeclaration;
class ColumnPseudoElement;
class ShadowRoot;
class NamedNodeMap;
class DOMTokenList;
class DatasetDOMStringMap;
class ElementAnimations;
class Attr;
typedef HeapVector<Member<Attr>> AttrNodeList;
class ElementIntersectionObserverData;
class ContainerQueryEvaluator;
class EditContext;
class AnchorElementObserver;
class InlineStylePropertyMap;
class ElementInternals;
class DisplayLockContext;
class ContainerQueryData;
class ResizeObserver;
class ResizeObservation;
class StyleScopeData;
class CustomElementDefinition;
class PopoverData;
class InvokerData;
class InterestInvokerTargetData;
class OutOfFlowData;
class HTMLElement;

enum class ElementFlags;

class CORE_EXPORT ElementRareDataVector final : public NodeRareData {
 private:
  friend class ElementRareDataVectorTest;
  enum class FieldId : unsigned {
    kDataset = 0,
    kShadowRoot = 1,
    kClassList = 2,
    kAttributeMap = 3,
    kAttrNodeList = 4,
    kCssomWrapper = 5,
    kElementAnimations = 6,
    kIntersectionObserverData = 7,
    kPseudoElementData = 8,
    kEditContext = 9,
    kPart = 10,
    kCssomMapWrapper = 11,
    kElementInternals = 12,
    kDisplayLockContext = 13,
    kContainerQueryData = 14,
    kRegionCaptureCropId = 15,
    kResizeObserverData = 16,
    kCustomElementDefinition = 17,
    kPopoverData = 18,
    kPartNamesMap = 19,
    kNonce = 20,
    kIsValue = 21,
    kSavedLayerScrollOffset = 22,
    kAnchorPositionScrollData = 23,
    kAnchorElementObserver = 24,
    kImplicitlyAnchoredElementCount = 25,
    kLastRememberedBlockSize = 26,
    kLastRememberedInlineSize = 27,
    kRestrictionTargetId = 28,
    kStyleScopeData = 29,
    kOutOfFlowData = 30,
    kInvokerData = 31,
    kInterestInvokerTargetData = 32,
    kScrollMarkerGroupData = 33,
    kScrollMarkerGroupContainerData = 34,
    kExplicitlySetElementsForAttr = 35,

    kNumFields = 36,
  };

  ElementRareDataField* GetField(FieldId field_id) const;
  void SetField(FieldId field_id, ElementRareDataField* field);

  template <typename T>
  class DataFieldWrapper final : public GarbageCollected<DataFieldWrapper<T>>,
                                 public ElementRareDataField {
   public:
    T& Get() { return data_; }
    void Trace(Visitor* visitor) const override {
      ElementRareDataField::Trace(visitor);
      TraceIfNeeded<T>::Trace(visitor, data_);
    }

   private:
    T data_;
  };

  template <typename T, typename... Args>
  T& EnsureField(FieldId field_id, Args&&... args) {
    T* field = static_cast<T*>(GetField(field_id));
    if (!field) {
      field = MakeGarbageCollected<T>(std::forward<Args>(args)...);
      SetField(field_id, field);
    }
    return *field;
  }

  template <typename T>
  T& EnsureWrappedField(FieldId field_id) {
    return EnsureField<DataFieldWrapper<T>>(field_id).Get();
  }

  template <typename T, typename U>
  void SetWrappedField(FieldId field_id, U data) {
    EnsureWrappedField<T>(field_id) = std::move(data);
  }

  template <typename T>
  T* GetWrappedField(FieldId field_id) const {
    auto* wrapper = static_cast<DataFieldWrapper<T>*>(GetField(field_id));
    return wrapper ? &wrapper->Get() : nullptr;
  }

  template <typename T>
  void SetOptionalField(FieldId field_id, std::optional<T> data) {
    if (data) {
      SetWrappedField<T>(field_id, *data);
    } else {
      SetField(field_id, nullptr);
    }
  }

  template <typename T>
  std::optional<T> GetOptionalField(FieldId field_id) const {
    if (auto* value = GetWrappedField<T>(field_id)) {
      return *value;
    }
    return std::nullopt;
  }

 public:
  ElementRareDataVector();
  ~ElementRareDataVector() override;

  void SetPseudoElement(
      PseudoId,
      PseudoElement*,
      const AtomicString& document_transition_tag = g_null_atom);
  PseudoElement* GetPseudoElement(
      PseudoId,
      const AtomicString& document_transition_tag = g_null_atom) const;
  bool HasViewTransitionGroupPseudoElement() const;
  bool HasScrollButtonOrMarkerGroupPseudos() const;
  PseudoElementData::PseudoElementVector GetPseudoElements() const;
  void AddColumnPseudoElement(ColumnPseudoElement&);
  const ColumnPseudoElementsVector* GetColumnPseudoElements() const;
  ColumnPseudoElement* GetColumnPseudoElement(wtf_size_t idx) const;
  void ClearColumnPseudoElements(wtf_size_t to_keep);

  CSSStyleDeclaration& EnsureInlineCSSStyleDeclaration(Element* owner_element);

  ShadowRoot* GetShadowRoot() const;
  void SetShadowRoot(ShadowRoot& shadow_root);

  NamedNodeMap* AttributeMap() const;
  void SetAttributeMap(NamedNodeMap* attribute_map);

  DOMTokenList* GetClassList() const;
  void SetClassList(DOMTokenList* class_list);

  DatasetDOMStringMap* Dataset() const;
  void SetDataset(DatasetDOMStringMap* dataset);

  ScrollOffset SavedLayerScrollOffset() const;
  void SetSavedLayerScrollOffset(ScrollOffset offset);

  ElementAnimations* GetElementAnimations();
  void SetElementAnimations(ElementAnimations* element_animations);

  bool HasPseudoElements() const;
  void ClearPseudoElements();

  AttrNodeList& EnsureAttrNodeList();
  AttrNodeList* GetAttrNodeList();
  void RemoveAttrNodeList();
  void AddAttr(Attr* attr);

  ElementIntersectionObserverData* IntersectionObserverData() const;
  ElementIntersectionObserverData& EnsureIntersectionObserverData();

  ContainerQueryEvaluator* GetContainerQueryEvaluator() const;
  void SetContainerQueryEvaluator(ContainerQueryEvaluator* evaluator);

  const AtomicString& GetNonce() const;
  void SetNonce(const AtomicString& nonce);

  const AtomicString& IsValue() const;
  void SetIsValue(const AtomicString& is_value);

  EditContext* GetEditContext() const;
  void SetEditContext(EditContext* edit_context);

  void SetPart(DOMTokenList* part);
  DOMTokenList* GetPart() const;

  void SetPartNamesMap(const AtomicString part_names);
  const NamesMap* PartNamesMap() const;

  InlineStylePropertyMap& EnsureInlineStylePropertyMap(Element* owner_element);
  InlineStylePropertyMap* GetInlineStylePropertyMap();

  const ElementInternals* GetElementInternals() const;
  ElementInternals& EnsureElementInternals(HTMLElement& target);

  DisplayLockContext* EnsureDisplayLockContext(Element* element);
  DisplayLockContext* GetDisplayLockContext() const;

  ContainerQueryData& EnsureContainerQueryData();
  ContainerQueryData* GetContainerQueryData() const;
  void ClearContainerQueryData();

  StyleScopeData& EnsureStyleScopeData();
  StyleScopeData* GetStyleScopeData() const;

  OutOfFlowData& EnsureOutOfFlowData();
  OutOfFlowData* GetOutOfFlowData() const;
  void ClearOutOfFlowData();

  // Returns the crop-ID if one was set, or nullptr otherwise.
  const RegionCaptureCropId* GetRegionCaptureCropId() const;
  // Sets a crop-ID on the item. Must be called at most once. Cannot be used
  // to unset a previously set crop-ID.
  void SetRegionCaptureCropId(std::unique_ptr<RegionCaptureCropId> crop_id);

  // Returns the ID backing a RestrictionTarget if one was set on the Element,
  // or nullptr otherwise.
  const RestrictionTargetId* GetRestrictionTargetId() const;
  // Returns the ID backing a RestrictionTarget if one was set on the Element,
  // or nullptr otherwise.
  // Sets an ID backing a RestrictionTarget associated with the Element.
  // Must be called at most once. Cannot be used to unset a previously set IDs.
  void SetRestrictionTargetId(std::unique_ptr<RestrictionTargetId> id);

  using ResizeObserverDataMap =
      HeapHashMap<Member<ResizeObserver>, Member<ResizeObservation>>;
  ResizeObserverDataMap* ResizeObserverData() const;
  ResizeObserverDataMap& EnsureResizeObserverData();

  void SetCustomElementDefinition(CustomElementDefinition* definition);
  CustomElementDefinition* GetCustomElementDefinition() const;

  void SetLastRememberedBlockSize(std::optional<LayoutUnit> size);
  void SetLastRememberedInlineSize(std::optional<LayoutUnit> size);
  std::optional<LayoutUnit> LastRememberedBlockSize() const;
  std::optional<LayoutUnit> LastRememberedInlineSize() const;

  PopoverData* GetPopoverData() const;
  PopoverData& EnsurePopoverData();
  void RemovePopoverData();

  InvokerData* GetInvokerData() const;
  InvokerData& EnsureInvokerData();

  InterestInvokerTargetData* GetInterestInvokerTargetData() const;
  InterestInvokerTargetData& EnsureInterestInvokerTargetData();
  void RemoveInterestInvokerTargetData();

  bool HasElementFlag(ElementFlags mask) const {
    return element_flags_ & static_cast<uint16_t>(mask);
  }
  void SetElementFlag(ElementFlags mask, bool value) {
    element_flags_ =
        (element_flags_ & ~static_cast<uint16_t>(mask)) |
        (-static_cast<uint16_t>(value) & static_cast<uint16_t>(mask));
  }
  void ClearElementFlag(ElementFlags mask) {
    element_flags_ &= ~static_cast<uint16_t>(mask);
  }

  void SetTabIndexExplicitly() {
    SetElementFlag(ElementFlags::kTabIndexWasSetExplicitly, true);
  }
  void ClearTabIndexExplicitly() {
    ClearElementFlag(ElementFlags::kTabIndexWasSetExplicitly);
  }

  ScrollMarkerGroupData* GetScrollMarkerGroupData() const;
  void RemoveScrollMarkerGroupData();
  ScrollMarkerGroupData& EnsureScrollMarkerGroupData(Element*);

  void SetScrollMarkerGroupContainerData(ScrollMarkerGroupData*);
  ScrollMarkerGroupData* GetScrollMarkerGroupContainerData() const;

  ExplicitlySetAttrElementsMap* GetExplicitlySetElementsForAttr() const;
  ExplicitlySetAttrElementsMap& EnsureExplicitlySetElementsForAttr();

  AnchorPositionScrollData* GetAnchorPositionScrollData() const;
  void RemoveAnchorPositionScrollData();
  AnchorPositionScrollData& EnsureAnchorPositionScrollData(Element*);

  AnchorElementObserver& EnsureAnchorElementObserver(Element*);
  AnchorElementObserver* GetAnchorElementObserver() const;

  void IncrementImplicitlyAnchoredElementCount();
  void DecrementImplicitlyAnchoredElementCount();
  bool HasImplicitlyAnchoredElement() const;

  void SetDidAttachInternals() { fields_.did_attach_internals = true; }
  bool DidAttachInternals() const { return fields_.did_attach_internals; }
  bool HasUndoStack() const { return fields_.has_undo_stack; }
  void SetHasUndoStack(bool value) { fields_.has_undo_stack = value; }
  void SetPseudoElementStylesChangeCounters(bool value) {
    fields_.has_counters_styles = value;
  }
  bool PseudoElementStylesAffectCounters() const {
    return fields_.has_counters_styles;
  }
  bool ScrollbarPseudoElementStylesDependOnFontMetrics() const {
    return fields_.scrollbar_pseudo_element_styles_depend_on_font_metrics;
  }
  void SetScrollbarPseudoElementStylesDependOnFontMetrics(bool value) {
    fields_.scrollbar_pseudo_element_styles_depend_on_font_metrics = value;
  }
  void SetHasBeenExplicitlyScrolled() {
    fields_.has_been_explicitly_scrolled = true;
  }
  bool HasBeenExplicitlyScrolled() const {
    return fields_.has_been_explicitly_scrolled;
  }

  FocusgroupFlags GetFocusgroupFlags() const {
    return fields_.focusgroup_flags;
  }
  void SetFocusgroupFlags(FocusgroupFlags flags) {
    fields_.focusgroup_flags = flags;
  }
  void ClearFocusgroupFlags() {
    fields_.focusgroup_flags = FocusgroupFlags::kNone;
  }

  bool AffectedBySubjectHas() const {
    return fields_.has_invalidation_flags.affected_by_subject_has;
  }
  void SetAffectedBySubjectHas() {
    fields_.has_invalidation_flags.affected_by_subject_has = true;
  }
  bool AffectedByNonSubjectHas() const {
    return fields_.has_invalidation_flags.affected_by_non_subject_has;
  }
  void SetAffectedByNonSubjectHas() {
    fields_.has_invalidation_flags.affected_by_non_subject_has = true;
  }
  bool AncestorsOrAncestorSiblingsAffectedByHas() const {
    return fields_.has_invalidation_flags
        .ancestors_or_ancestor_siblings_affected_by_has;
  }
  void SetAncestorsOrAncestorSiblingsAffectedByHas() {
    fields_.has_invalidation_flags
        .ancestors_or_ancestor_siblings_affected_by_has = true;
  }
  unsigned GetSiblingsAffectedByHasFlags() const {
    return fields_.has_invalidation_flags.siblings_affected_by_has;
  }
  bool HasSiblingsAffectedByHasFlags(unsigned flags) const {
    return fields_.has_invalidation_flags.siblings_affected_by_has & flags;
  }
  void SetSiblingsAffectedByHasFlags(unsigned flags) {
    fields_.has_invalidation_flags.siblings_affected_by_has |= flags;
  }
  bool AffectedByPseudoInHas() const {
    return fields_.has_invalidation_flags.affected_by_pseudos_in_has;
  }
  void SetAffectedByPseudoInHas() {
    fields_.has_invalidation_flags.affected_by_pseudos_in_has = true;
  }
  bool AncestorsOrSiblingsAffectedByHoverInHas() const {
    return fields_.has_invalidation_flags
        .ancestors_or_siblings_affected_by_hover_in_has;
  }
  void SetAncestorsOrSiblingsAffectedByHoverInHas() {
    fields_.has_invalidation_flags
        .ancestors_or_siblings_affected_by_hover_in_has = true;
  }
  bool AncestorsOrSiblingsAffectedByActiveInHas() const {
    return fields_.has_invalidation_flags
        .ancestors_or_siblings_affected_by_active_in_has;
  }
  void SetAncestorsOrSiblingsAffectedByActiveInHas() {
    fields_.has_invalidation_flags
        .ancestors_or_siblings_affected_by_active_in_has = true;
  }
  bool AncestorsOrSiblingsAffectedByFocusInHas() const {
    return fields_.has_invalidation_flags
        .ancestors_or_siblings_affected_by_focus_in_has;
  }
  void SetAncestorsOrSiblingsAffectedByFocusInHas() {
    fields_.has_invalidation_flags
        .ancestors_or_siblings_affected_by_focus_in_has = true;
  }
  bool AncestorsOrSiblingsAffectedByFocusVisibleInHas() const {
    return fields_.has_invalidation_flags
        .ancestors_or_siblings_affected_by_focus_visible_in_has;
  }
  void SetAncestorsOrSiblingsAffectedByFocusVisibleInHas() {
    fields_.has_invalidation_flags
        .ancestors_or_siblings_affected_by_focus_visible_in_has = true;
  }
  bool AffectedByLogicalCombinationsInHas() const {
    return fields_.has_invalidation_flags
        .affected_by_logical_combinations_in_has;
  }
  void SetAffectedByLogicalCombinationsInHas() {
    fields_.has_invalidation_flags.affected_by_logical_combinations_in_has =
        true;
  }
  bool AffectedByMultipleHas() const {
    return fields_.has_invalidation_flags.affected_by_multiple_has;
  }
  void SetAffectedByMultipleHas() {
    fields_.has_invalidation_flags.affected_by_multiple_has = true;
  }

  void Trace(blink::Visitor*) const override;

 private:
  // Using inheritance instead of composition to pack bytes better.
  struct Fields : public SparseVector<FieldId, Member<ElementRareDataField>> {
    unsigned did_attach_internals : 1 = false;
    unsigned has_undo_stack : 1 = false;
    unsigned scrollbar_pseudo_element_styles_depend_on_font_metrics : 1 = false;
    // This never gets reset, since we would have to keep track for
    // every pseudo element whether it has counter style or not.
    // But since situations when counter style if removed from
    // pseudo element are rare, we are fine with it, since
    // it doesn't hurt performance much.
    unsigned has_counters_styles : 1 = false;
    unsigned has_been_explicitly_scrolled : 1 = false;
    HasInvalidationFlags has_invalidation_flags;
    FocusgroupFlags focusgroup_flags = FocusgroupFlags::kNone;
  };
  Fields fields_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_DOM_ELEMENT_RARE_DATA_VECTOR_H_
