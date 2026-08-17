/*
 * Copyright (C) 2012, Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 *
 * 1.  Redistributions of source code must retain the above copyright
 *     notice, this list of conditions and the following disclaimer.
 * 2.  Redistributions in binary form must reproduce the above copyright
 *     notice, this list of conditions and the following disclaimer in the
 *     documentation and/or other materials provided with the distribution.
 * 3.  Neither the name of Apple Computer, Inc. ("Apple") nor the names of
 *     its contributors may be used to endorse or promote products derived
 *     from this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE AND ITS CONTRIBUTORS "AS IS" AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
 * WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 * DISCLAIMED. IN NO EVENT SHALL APPLE OR ITS CONTRIBUTORS BE LIABLE FOR ANY
 * DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
 * (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
 * LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND
 * ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF
 * THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_ACCESSIBILITY_AX_NODE_OBJECT_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_ACCESSIBILITY_AX_NODE_OBJECT_H_

#include "base/dcheck_is_on.h"
#include "third_party/blink/renderer/core/html/forms/html_form_control_element.h"
#include "third_party/blink/renderer/modules/accessibility/ax_object.h"
#include "third_party/blink/renderer/modules/modules_export.h"

namespace blink {

class AXObjectCacheImpl;
class Element;
class HTMLElement;
class HTMLLabelElement;
class Node;

class MODULES_EXPORT AXNodeObject : public AXObject {
 public:
  // Note: when constructed with a Node, the LayoutObject will be ignored
  // in property computations.
  AXNodeObject(Node*, AXObjectCacheImpl&);
  AXNodeObject(LayoutObject*, AXObjectCacheImpl&);

  AXNodeObject(const AXNodeObject&) = delete;
  AXNodeObject& operator=(const AXNodeObject&) = delete;

  ~AXNodeObject() override;

  static std::optional<String> GetCSSAltText(const Element*);

  void Trace(Visitor*) const override;

  // Call to force-load inline text boxes for the current subtree.
  void LoadInlineTextBoxes() override;
  // Should inline text boxes be considered when adding chldren to this node.
  bool ShouldLoadInlineTextBoxes() const override;

  ScrollableArea* GetScrollableAreaIfScrollable() const final;

 protected:
#if DCHECK_IS_ON()
  bool initialized_ = false;
  mutable bool getting_bounds_ = false;
#endif

  // The accessibility role, not taking the ARIA role into account.
  ax::mojom::blink::Role native_role_ = ax::mojom::blink::Role::kUnknown;

  // The ARIA role, not taking the native role into account.
  ax::mojom::blink::Role aria_role_ = ax::mojom::blink::Role::kUnknown;

  bool HasCustomElementTreeProcessing() const;
  bool ShouldIncludeCustomElement() const;
  AXObjectInclusion ShouldIncludeBasedOnSemantics(
      IgnoredReasons* = nullptr) const;
  bool ComputeIsIgnored(IgnoredReasons*) const override;
  ax::mojom::blink::Role DetermineRoleValue() override;
  ax::mojom::blink::Role NativeRoleIgnoringAria() const override;
  void AlterSliderOrSpinButtonValue(bool increase);
  AXObject* ActiveDescendant() const override;
  String AriaAccessibilityDescription() const;
  String AutoComplete() const override;

  // For table objects.
  bool IsDataTable() const override;
  unsigned ColumnCount() const override;
  unsigned RowCount() const override;
  void ColumnHeaders(AXObjectVector&) const override;
  void RowHeaders(AXObjectVector&) const override;
  AXObject* CellForColumnAndRow(unsigned column, unsigned row) const override;
  // For table cells.
  unsigned ColumnIndex() const override;
  unsigned RowIndex() const override;  // Also for a table row.
  unsigned ColumnSpan() const override;
  unsigned RowSpan() const override;
  // For a table row or column.
  AXObject* HeaderObject() const override;
  // For a table row or column header.
  ax::mojom::blink::SortDirection GetSortDirection() const override;
  // Role determination within a table.
  ax::mojom::blink::Role DetermineTableSectionRole() const;
  ax::mojom::blink::Role DetermineTableCellRole() const;
  ax::mojom::blink::Role DetermineTableRowRole() const;

  Element* MenuItemElementForMenu() const;
  HTMLElement* CorrespondingControlForLabelElement() const;

  //
  // Overridden from AXObject.
  //

  void Init(AXObject* parent) override;
  void Detach() override;
  bool IsAXNodeObject() const final;

  // Check object role or purpose.
  bool IsAutofillAvailable() const override;
  bool IsDefault() const final;
  bool IsFieldset() const final;
  bool IsHovered() const final;
  bool IsImageButton() const;
  bool IsInputImage() const final;
  bool IsLineBreakingObject() const override;
  bool IsLoaded() const override;
  bool IsMultiSelectable() const override;
  bool IsNativeImage() const final;
  bool IsProgressIndicator() const override;
  bool IsSlider() const override;
  bool IsSpinButton() const override;
  bool IsNativeSlider() const override;
  bool IsNativeSpinButton() const override;
  bool IsEmbeddingElement() const override;
  bool IsLinked() const override;
  bool IsVisible() const override;
  bool IsVisited() const override;

  // Check object state.
  bool IsClickable() const final;
  bool IsFocused() const override;
  AccessibilityExpanded IsExpanded() const override;
  AccessibilitySelectedState IsSelected() const override;
  bool IsSelectedFromFocusSupported() const override;
  bool IsSelectedFromFocus() const override;
  bool IsNotUserSelectable() const override;
  bool IsRequired() const final;
  bool IsControl() const override;
  AXRestriction Restriction() const override;

  // Properties of static elements.
  const AtomicString& AccessKey() const override;
  RGBA32 ColorValue() const final;
  RGBA32 GetColor() const final;
  RGBA32 BackgroundColor() const override;
  const AtomicString& ComputedFontFamily() const final;
  String FontFamilyForSerialization() const final;
  // Font size is in pixels.
  float FontSize() const final;
  float FontWeight() const final;
  bool CanvasHasFallbackContent() const final;
  int HeadingLevel() const final;
  unsigned HierarchicalLevel() const final;
  void SerializeMarkerAttributes(ui::AXNodeData* node_data) const override;
  ax::mojom::blink::ListStyle GetListStyle() const final;
  AXObject* InPageLinkTarget() const override;
  const AtomicString& EffectiveTarget() const override;
  AccessibilityOrientation Orientation() const override;

  AXObject* GetChildFigcaption() const override;
  bool IsDescendantOfLandmarkDisallowedElement() const override;

  // Is a redundant label of a radio button or checkbox.
  static bool IsRedundantLabel(HTMLLabelElement* label);

  // Used to compute kRadioGroupIds, which is only used on Mac.
  // TODO(accessibility) Consider computing on browser side and removing here.
  AXObjectVector RadioButtonsInGroup() const override;
  static HeapVector<Member<HTMLInputElement>> FindAllRadioButtonsWithSameName(
      HTMLInputElement* radio_button);

  ax::mojom::blink::WritingDirection GetTextDirection() const final;
  ax::mojom::blink::TextPosition GetTextPosition() const final;
  void GetTextStyleAndTextDecorationStyle(
      int32_t* text_style,
      ax::mojom::blink::TextDecorationStyle* text_overline_style,
      ax::mojom::blink::TextDecorationStyle* text_strikethrough_style,
      ax::mojom::blink::TextDecorationStyle* text_underline_style) const final;

  String ImageDataUrl(const gfx::Size& max_size) const final;
  int TextOffsetInFormattingContext(int offset) const override;

  // Object attributes.
  ax::mojom::blink::TextAlign GetTextAlign() const final;
  float GetTextIndent() const final;

  // Properties of interactive elements.
  ax::mojom::blink::AriaCurrentState GetAriaCurrentState() const final;
  ax::mojom::blink::InvalidState GetInvalidState() const final;
  bool IsValidFormControl(ListedElement* form_control) const;
  bool ValueForRange(float* out_value) const override;
  bool MaxValueForRange(float* out_value) const override;
  bool MinValueForRange(float* out_value) const override;
  bool StepValueForRange(float* out_value) const override;
  KURL Url() const override;
  AXObject* ChooserPopup() const override;
  String GetValueForControl() const override;
  String GetValueForControl(AXObjectSet& visited) const override;
  String SlowGetValueForControlIncludingContentEditable() const override;
  String SlowGetValueForControlIncludingContentEditable(
      AXObjectSet& visited) const override;
  String TextFromDescendants(AXObjectSet& visited,
                             const AXObject* aria_label_or_description_root,
                             bool recursive) const override;

  // ARIA attributes.
  ax::mojom::blink::Role RawAriaRole() const final;
  ax::mojom::blink::HasPopup HasPopup() const override;
  ax::mojom::blink::IsPopup IsPopup() const override;
  bool IsEditableRoot() const override;
  bool HasContentEditableAttributeSet() const override;

  // Modify or take an action on an object.
  bool OnNativeSetSelectedAction(bool selected) override;
  bool OnNativeSetValueAction(const String&) override;

  // AX name calculation.
  String GetName(ax::mojom::blink::NameFrom&,
                 AXObjectVector* name_objects) const override;
  String TextAlternative(bool recursive,
                         const AXObject* aria_label_or_description_root,
                         AXObjectSet& visited,
                         ax::mojom::blink::NameFrom&,
                         AXRelatedObjectVector*,
                         NameSources*) const override;
  // If name_sources are being collected in a call to TextAlternative(), the
  // algorithm will continue even after finding a valid text alternative in
  // order to collect all viable name_sources. This can cause the original text
  // alternative to be overwritten. So, at the end of TextAlternative() it's
  // necessary to call GetSavedTextAlternativeFromNameSource() to recover the
  // original text alternative from name_sources.
  static String GetSavedTextAlternativeFromNameSource(
      bool found_text_alternative,
      ax::mojom::NameFrom& name_from,
      AXRelatedObjectVector* related_objects,
      NameSources* name_sources);
  String Description(ax::mojom::blink::NameFrom,
                     ax::mojom::blink::DescriptionFrom&,
                     AXObjectVector* description_objects) const override;
  String Description(ax::mojom::blink::NameFrom,
                     ax::mojom::blink::DescriptionFrom&,
                     DescriptionSources*,
                     AXRelatedObjectVector*) const override;
  String SVGDescription(ax::mojom::blink::NameFrom,
                        ax::mojom::blink::DescriptionFrom&,
                        DescriptionSources*,
                        AXRelatedObjectVector*) const;
  String Placeholder(ax::mojom::blink::NameFrom) const override;
  String Title(ax::mojom::blink::NameFrom) const override;

  // Location
  void GetRelativeBounds(AXObject** out_container,
                         gfx::RectF& out_bounds_in_container,
                         gfx::Transform& out_container_transform,
                         bool* clips_children = nullptr) const override;

  void AddChildren() override;
  bool CanHaveChildren() const override;
  // Set is_from_aria_owns to true if the child is being added because it was
  // pointed to from aria-owns.
  void AddChild(AXObject*, bool is_from_aria_owns = false);
  // Add a child that must be included in tree, enforced via DCHECK.
  void AddChildAndCheckIncluded(AXObject*, bool is_from_aria_owns = false);
  // If node is non-null, GetOrCreate an AXObject for it and add as a child.
  // This includes expanding the given node if the structure needs to be
  // unpacked. For example, scrollers and their nested scroll-marker-groups
  // become siblings.
  void AddNodeChild(Node*);
  // Set is_from_aria_owns to true if the child is being insert because it was
  // pointed to from aria-owns.
  void InsertChild(AXObject*, unsigned index, bool is_from_aria_owns = false);
  void SelectedOptions(AXObjectVector&) const override;

  // Properties of the object's owning document or page.
  double EstimatedLoadingProgress() const override;

  // DOM and Render tree access.
  Element* ActionElement() const override;
  Element* AnchorElement() const override;
  Document* GetDocument() const override;
  Node* GetNode() const final;
  LayoutObject* GetLayoutObject() const final;

  // Modify or take an action on an object.
  bool OnNativeBlurAction() final;
  bool OnNativeFocusAction() final;
  bool OnNativeIncrementAction() final;
  bool OnNativeDecrementAction() final;
  bool OnNativeSetSequentialFocusNavigationStartingPointAction() final;

  // Notifications that this object may have changed.
  void HandleAriaExpandedChanged() override;
  void HandleActiveDescendantChanged() override;

  // Gets a list of nodes that form an error message for this node, if it
  // exists. Error messages from ARIA will always override native error
  // messages.
  AXObjectVector ErrorMessage() const override;
  // Gets a list of nodes created from HTML validation that form an error
  // message for this node, if any exist.
  AXObjectVector ErrorMessageFromHTML() const override;
  // Gets a list of nodes specified by `aria-errormessage`, `aria-controls`,
  // etc. that form an error message for this node, if any exist.
  AXObjectVector RelationVectorFromAria(
      const QualifiedName& attr_name) const override;

  // Position in set and Size of set
  int PosInSet() const override;
  int SetSize() const override;

  // Aria-owns.
  void ComputeAriaOwnsChildren(
      HeapVector<Member<AXObject>>& owned_children) const;

  // Helper method for LoadInlineTextBoxes().
  void LoadInlineTextBoxesHelper() override;

  //
  // Layout object specific methods.
  //

  AXObject* AccessibilityHitTest(const gfx::Point&) const override;

  // If we can't determine a useful role from the DOM node, attempt to determine
  // a role from the layout object.
  ax::mojom::blink::Role RoleFromLayoutObjectOrNode() const;

  // Called when autofill/autocomplete suggestion availability changes on a form
  // control.
  void HandleAutofillSuggestionAvailabilityChanged(
      WebAXAutofillSuggestionAvailability suggestion_availability) override;

  // Word boundaries are only exposed for inline text boxes and list markers.
  // This override implements word boundaries for list markers.
  void GetWordBoundaries(Vector<int>& word_starts,
                         Vector<int>& word_ends) const override;

 private:
  // Store values that could change over the lifetime of the AXObject, but
  // are repeatedly looked up during serialization. While the tree is frozen,
  // the value remains constant. The generation ID is incremented each time
  // the tree is frozen. Anytime a value is recomputed that is stored in this
  // cache, it compares the current vs cached generation, updating the cached
  // value and generation if needed.
  struct GenerationalCache : public GarbageCollected<GenerationalCache> {
    virtual void Trace(Visitor*) const;
    uint64_t generation = 0;
    Member<AXObject> next_on_line;
    Member<AXObject> previous_on_line;
  };
  mutable Member<GenerationalCache> generational_cache_;
  void MaybeResetCache() const;
  AXObject* SetNextOnLine(AXObject* next_on_line) const;
  AXObject* SetPreviousOnLine(AXObject* previous_on_line) const;

  // This function returns the text of a tooltip associated with the element.
  // Although there are two ways of doing this, it is unlikely that an author
  // would provide 2 overlapping types of tooltips. Order of precedence:
  // 1. The title attribute is currently preferred if present.
  // 2. The contents of a plain hint, which has no interesting semantic or
  // interactive content, is used next.
  // TODO(accessibility): Follow-up with standards discussion to determine
  // whether a different order of precedence makes sense.
  String TextAlternativeFromTooltip(
      ax::mojom::blink::NameFrom& name_from,
      NameSources* name_sources,
      bool* found_text_alternative,
      String* text_alternative,
      AXRelatedObjectVector* related_objects) const;

  String TextAlternativeFromTitleAttribute(
      const AtomicString& title,
      ax::mojom::blink::NameFrom& name_from,
      NameSources* name_sources,
      bool* found_text_alternative) const;
  String NativeTextAlternative(AXObjectSet& visited,
                               ax::mojom::blink::NameFrom&,
                               AXRelatedObjectVector*,
                               NameSources*,
                               bool* found_text_alternative) const;
  String MaybeAppendFileDescriptionToName(const String& name) const;
  bool ShouldIncludeContentInTextAlternative(
      bool recursive,
      const AXObject* aria_label_or_description_root,
      AXObjectSet& visited) const;
  String PlaceholderFromNativeAttribute() const;
  String GetValueContributionToName(AXObjectSet& visited) const;
  bool UseNameFromSelectedOption() const;
  virtual bool IsTabItemSelected() const;

  void AddNodeChildImpl(Node*);
  void AddChildrenImpl();
  void AddNodeChildren();
  void AddPseudoElementChildrenFromLayoutTree();
  bool CanAddLayoutChild(LayoutObject& child);
  void AddInlineTextBoxChildren();
  void AddInlineTextBoxChildrenWithBlockFlowIterator();
  void AddImageMapChildren();
  void AddPopupChildren();
  bool HasValidHTMLTableStructureAndLayout() const;
  void AddTableChildren();
  bool FindAllTableCellsWithRole(ax::mojom::blink::Role, AXObjectVector&) const;
  void AddValidationMessageChild();
  void AddAccessibleNodeChildren();
  void AddOwnedChildren();
  void AddScrollMarkerGroupChildren();
#if DCHECK_IS_ON()
  void CheckValidChild(AXObject* child);
#endif

  ax::mojom::blink::TextPosition GetTextPositionFromRole() const;

  static bool IsNameFromLabelElement(HTMLElement* control);

  // Hit testing.
  AXObject* AccessibilityImageMapHitTest(HTMLAreaElement*,
                                         const gfx::Point&) const;

  // Inline text boxes.
  //
  // Get either the first inline block descendant or deepest descendant that
  // is included in the tree. |start_object| does not have to be included in the
  // tree. If |first| is true, returns the deepest first descendant. Otherwise,
  // returns the deepest last descendant.
  AXObject* GetFirstInlineBlockOrDeepestInlineAXChildInLayoutTree(
      AXObject* start_object,
      bool first) const;
  AXObject* NextOnLine() const override;
  AXObject* PreviousOnLine() const override;
#if defined(REDUCE_AX_INLINE_TEXTBOXES)
  bool always_load_inline_text_boxes_ = false;
#endif

  Member<Node> node_;
  Member<LayoutObject> layout_object_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_ACCESSIBILITY_AX_NODE_OBJECT_H_
