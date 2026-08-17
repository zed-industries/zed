/*
 * Copyright (C) 2008, 2009, 2011 Apple Inc. All rights reserved.
 * Copyright (C) 2008 Nuanti Ltd.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_ACCESSIBILITY_AX_OBJECT_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_ACCESSIBILITY_AX_OBJECT_H_

#include <optional>
#include <ostream>
#include <utility>

#include "base/dcheck_is_on.h"
#include "base/memory/raw_ref.h"
#include "base/memory/stack_allocated.h"
#include "third_party/blink/public/common/input/web_input_event.h"
#include "third_party/blink/public/web/web_ax_enums.h"
#include "third_party/blink/renderer/core/accessibility/axid.h"
#include "third_party/blink/renderer/core/dom/element.h"
#include "third_party/blink/renderer/core/editing/markers/document_marker.h"
#include "third_party/blink/renderer/core/inspector/protocol/accessibility.h"
#include "third_party/blink/renderer/core/layout/geometry/physical_rect.h"
#include "third_party/blink/renderer/core/scroll/scroll_alignment.h"
#include "third_party/blink/renderer/modules/accessibility/ax_block_flow_iterator.h"
#include "third_party/blink/renderer/modules/accessibility/ax_enums.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/graphics/color.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_set.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/weborigin/kurl.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"
#include "ui/accessibility/ax_common.h"
#include "ui/accessibility/ax_enums.mojom-blink.h"
#include "ui/accessibility/ax_mode.h"
#include "ui/accessibility/ax_node_id_forward.h"
#include "ui/accessibility/ax_tree_id.h"

namespace gfx {
class Transform;
}

namespace ui {
struct AXActionData;
struct AXNodeData;
struct AXRelativeBounds;
}

namespace blink {

class AbstractInlineTextBox;
class AXObject;
class AXObjectCacheImpl;
class LayoutObject;
class LocalFrameView;
class Node;
class ScrollableArea;
class V8HighlightType;

class IgnoredReason {
  DISALLOW_NEW();

 public:
  AXIgnoredReason reason;
  Member<const AXObject> related_object;

  explicit IgnoredReason(AXIgnoredReason reason)
      : reason(reason), related_object(nullptr) {}

  IgnoredReason(AXIgnoredReason r, const AXObject* obj)
      : reason(r), related_object(obj) {}

  void Trace(Visitor* visitor) const { visitor->Trace(related_object); }
};

class NameSourceRelatedObject final
    : public GarbageCollected<NameSourceRelatedObject> {
 public:
  WeakMember<AXObject> object;
  String text;

  NameSourceRelatedObject(AXObject* object, String text)
      : object(object), text(text) {}

  NameSourceRelatedObject(const NameSourceRelatedObject&) = delete;
  NameSourceRelatedObject& operator=(const NameSourceRelatedObject&) = delete;

  void Trace(Visitor* visitor) const { visitor->Trace(object); }
};

typedef HeapVector<Member<NameSourceRelatedObject>> AXRelatedObjectVector;
class NameSource {
  DISALLOW_NEW();

 public:
  String text;
  bool superseded = false;
  bool invalid = false;
  ax::mojom::blink::NameFrom type = ax::mojom::blink::NameFrom::kNone;
  const raw_ref<const QualifiedName> attribute;
  AtomicString attribute_value;
  AXTextSource native_source = kAXTextFromNativeSourceUninitialized;
  AXRelatedObjectVector related_objects;

  NameSource(bool superseded, const QualifiedName& attr)
      : superseded(superseded), attribute(attr) {}

  explicit NameSource(bool superseded)
      : superseded(superseded), attribute(QualifiedName::Null()) {}

  void Trace(Visitor* visitor) const { visitor->Trace(related_objects); }
};

class DescriptionSource {
  DISALLOW_NEW();

 public:
  String text;
  bool superseded = false;
  bool invalid = false;
  ax::mojom::blink::DescriptionFrom type =
      ax::mojom::blink::DescriptionFrom::kNone;
  const raw_ref<const QualifiedName> attribute;
  AtomicString attribute_value;
  AXTextSource native_source = kAXTextFromNativeSourceUninitialized;
  AXRelatedObjectVector related_objects;

  DescriptionSource(bool superseded, const QualifiedName& attr)
      : superseded(superseded), attribute(attr) {}

  explicit DescriptionSource(bool superseded)
      : superseded(superseded), attribute(QualifiedName::Null()) {}

  void Trace(Visitor* visitor) const { visitor->Trace(related_objects); }
};

// Returns a ax::mojom::blink::MarkerType cast to an int, suitable
// for serializing into AXNodeData.
int32_t ToAXMarkerType(DocumentMarker::MarkerType marker_type);

// Returns a ax::mojom::blink::HighlightType cast to an int, suitable
// for serializing into AXNodeData.
int32_t ToAXHighlightType(const V8HighlightType& highlight_type);

}  // namespace blink

WTF_ALLOW_INIT_WITH_MEM_FUNCTIONS(blink::IgnoredReason)
WTF_ALLOW_INIT_WITH_MEM_FUNCTIONS(blink::NameSource)
WTF_ALLOW_INIT_WITH_MEM_FUNCTIONS(blink::DescriptionSource)

namespace blink {

class MODULES_EXPORT AXObject : public GarbageCollected<AXObject> {
 public:
  typedef HeapVector<Member<AXObject>> AXObjectVector;

  // Iterator for the ancestors of an |AXObject|.
  // Walks through all the unignored parents of the object up to the root.
  // Does not include the object itself in the list of ancestors.
  class MODULES_EXPORT AncestorsIterator final {
    STACK_ALLOCATED();

   public:
    using iterator_category = std::forward_iterator_tag;
    using value_type = AXObject;
    using difference_type = ptrdiff_t;
    using pointer = value_type*;
    using reference = value_type&;

    AncestorsIterator() = default;
    ~AncestorsIterator() = default;

    AncestorsIterator(const AncestorsIterator& other)
        : current_(other.current_) {}

    AncestorsIterator& operator=(const AncestorsIterator& other) {
      current_ = other.current_;
      return *this;
    }

    AncestorsIterator& operator++() {
      current_ = (current_ && !current_->IsDetached())
                     ? current_->ParentObjectUnignored()
                     : nullptr;
      return *this;
    }

    AncestorsIterator operator++(int) {
      AncestorsIterator ret = *this;
      ++*this;
      return ret;
    }

    AXObject& operator*() const {
      DCHECK(current_);
      return *current_;
    }

    AXObject* operator->() const {
      DCHECK(current_);
      return static_cast<AXObject*>(current_);
    }

    MODULES_EXPORT friend void swap(AncestorsIterator& left,
                                    AncestorsIterator& right) {
      std::swap(left.current_, right.current_);
    }

    MODULES_EXPORT friend bool operator==(const AncestorsIterator& left,
                                          const AncestorsIterator& right) {
      return left.current_ == right.current_;
    }

    MODULES_EXPORT friend bool operator!=(const AncestorsIterator& left,
                                          const AncestorsIterator& right) {
      return !(left == right);
    }

   private:
    explicit AncestorsIterator(AXObject& current) : current_(&current) {}

    friend class AXObject;
    friend class AXObjectCacheImpl;

    AXObject* current_;
  };

 protected:
  explicit AXObject(AXObjectCacheImpl&);

#if DCHECK_IS_ON()
  bool is_initializing_ = false;
  bool is_computing_role_ = false;
  bool is_updating_cached_values_ = false;
  bool is_initialized_ = false;
#endif
#if !defined(NDEBUG)
  // Keep track of what the object used to be, to make it easier to debug
  // situations involving detached objects.
  String detached_object_debug_info_;
#endif

#if AX_FAIL_FAST_BUILD()
  bool is_adding_children_ = false;
  mutable bool is_computing_text_from_descendants_ = false;
#endif

 public:
  AXObject(const AXObject&) = delete;
  AXObject& operator=(const AXObject&) = delete;

  virtual ~AXObject();
  virtual void Trace(Visitor*) const;

  static unsigned NumberOfLiveAXObjects() { return number_of_live_ax_objects_; }

  // After constructing an AXObject, it must be given a
  // unique ID, then added to AXObjectCacheImpl, and finally Init() must
  // be called last.
  void SetAXObjectID(AXID ax_object_id) { id_ = ax_object_id; }
  // Initialize the object and set the |parent|, which can only be null for the
  // root of the tree.
  virtual void Init(AXObject* parent);

  // TODO(chrishtr): these methods are not really const, but there are various
  // other related methods that need const. Review/fix.

  // Sets, clears or queries the has_dirty_descendants_ bit. This dirty
  // bit controls whether the object, or a descendant, needs to be visited
  // a tree walk to update the AX tree via
  // AXObjectCacheImpl::FinalizeTree(). It does not directly indicate
  // whether children, parent or other pointers are actually out of date; there
  // are other dirty bits such as children_dirty_ for that.
  void SetAncestorsHaveDirtyDescendants();
  void SetHasDirtyDescendants(bool dirty);
  bool HasDirtyDescendants() const { return has_dirty_descendants_; }

  // When the corresponding WebCore object that this AXObject
  // wraps is deleted, it must be detached.
  virtual void Detach();
  bool IsDetached() const;

  // Updates the cached attribute values. This may be recursive, so to prevent
  // deadlocks, functions called here may only search up the tree (ancestors),
  // not down.
  // Fires children change on the parent if the node's ignored or included in
  // tree status changes. Use |notify_parent_of_ignored_changes = false| to
  // prevent this.
  void UpdateCachedAttributeValuesIfNeeded(
      bool notify_parent_of_ignored_changes = true);

  // Invalidates cached_* members on this object only by resetting the
  // modification count.
  // To instead invalidate on all objects in a subtree, call
  // AXObjectCacheImpl::InvalidateCachedValuesOnSubtree().
  void InvalidateCachedValues(TreeUpdateReason reason);
  bool NeedsToUpdateCachedValues() const { return cached_values_need_update_; }
  bool ChildrenNeedToUpdateCachedValues() const {
    return child_cached_values_need_update_;
  }
  void CheckCanAccessCachedValues() const;

  // The AXObjectCacheImpl that owns this object, and its unique ID within this
  // cache.
  AXObjectCacheImpl& AXObjectCache() const {
    DCHECK(ax_object_cache_);
    return *ax_object_cache_;
  }

  AXID AXObjectID() const { return id_; }

  // Wrappers that retrieve either an Accessibility Object Model property,
  // or the equivalent ARIA attribute, in that order.
  virtual AbstractInlineTextBox* GetInlineTextBox() const { return nullptr; }
  virtual std::optional<AXBlockFlowIterator::FragmentIndex> GetFragmentIndex()
      const {
    return std::nullopt;
  }

  static const GCedHeapVector<Member<Element>>*
  ElementsFromAttributeOrInternals(const Element* from,
                                   const QualifiedName& attribute);
  static Element* ElementFromAttributeOrInternals(
      const Element* from,
      const QualifiedName& attribute);

  // Serialize the properties of this node into |node_data|.
  void Serialize(ui::AXNodeData* node_data,
                 ui::AXMode accessibility_mode,
                 bool is_snapshot = false) const;

  // Determine subclass type.
  virtual bool IsImageMapLink() const;
  virtual bool IsAXNodeObject() const;
  virtual bool IsAXInlineTextBox() const;
  virtual bool IsList() const;
  virtual bool IsProgressIndicator() const;
  virtual bool IsAXRadioInput() const;
  virtual bool IsSlider() const;
  virtual bool IsValidationMessage() const;

  // Returns true if this object is an ARIA text field, i.e. it is neither an
  // <input> nor a <textarea>, but it has an ARIA role of textbox, searchbox or
  // (on certain platforms) combobox.
  bool IsARIATextField() const;

  // Returns true if the AXObject is a button based on its AXRole.
  bool IsButton() const;
  bool IsCanvas() const;
  bool IsColorWell() const;
  virtual bool IsControl() const;
  virtual bool IsDefault() const;
  virtual bool IsFieldset() const;
  bool IsHeading() const;
  bool IsImage() const;
  virtual bool IsInputImage() const;
  bool IsLink() const;
  bool IsMenu() const;
  bool IsMenuRelated() const;
  bool IsMeter() const;
  virtual bool IsNativeImage() const;
  virtual bool IsNativeSpinButton() const;

  // Returns true if this object is an input element of a text field type, such
  // as type="text" or type="tel", or a textarea.
  bool IsAtomicTextField() const;

  // Returns true if this object is not an <input> or a <textarea>, and is
  // either a contenteditable, or has the CSS user-modify style set to something
  // editable.
  bool IsNonAtomicTextField() const;

  // Returns the lowest text field ancestor, including itself.
  AXObject* GetTextFieldAncestor();

  // Returns true if this object is a text field that is used for entering
  // passwords, i.e. <input type=password>.
  bool IsPasswordField() const;

  // Returns true if this object is a text field that is used for entering
  // passwords, but its input should be masked from the user.
  bool IsPasswordFieldAndShouldHideValue() const;

  bool IsPresentational() const;
  bool IsRangeValueSupported() const;
  bool IsScrollbar() const;
  virtual bool IsNativeSlider() const;
  virtual bool IsSpinButton() const;
  bool IsTabItem() const;
  bool IsTabList() const;

  // This object is a text field. This is any widget in which the user should be
  // able to enter and edit text.
  //
  // Examples include <input type="text">, <input type="password">, <textarea>,
  // <div contenteditable="true">, <div role="textbox">, <div role="searchbox">
  // and <div role="combobox">. Note that when an ARIA role that indicates that
  // the widget is editable is used, such as "role=textbox", the element doesn't
  // need to be contenteditable for this method to return true, as in theory
  // JavaScript could be used to implement editing functionality. In practice,
  // this situation should be rare.
  bool IsTextField() const;

  bool IsTextObject() const;
  bool IsTree() const { return RoleValue() == ax::mojom::blink::Role::kTree; }
  bool IsWebArea() const {
    return RoleValue() == ax::mojom::blink::Role::kRootWebArea;
  }

  // Check object state.
  virtual bool IsAutofillAvailable() const;
  virtual bool IsClickable() const;
  // Is |this| disabled for any of the follow reasons:
  // * aria-disabled
  // * disabled form control
  // * a focusable descendant of a disabled container
  // * a descendant of a disabled complex control such as a date picker
  bool IsDisabled() const;
  virtual AccessibilityExpanded IsExpanded() const;
  virtual bool IsFocused() const;
  virtual bool IsHovered() const;

  // Returns true if this object starts a new paragraph in the accessibility
  // tree's text representation.
  virtual bool IsLineBreakingObject() const;

  virtual bool IsLinked() const;
  virtual bool IsLoaded() const;
  virtual bool IsModal() const;
  virtual bool IsMultiSelectable() const;
  virtual bool IsRequired() const;
  virtual AccessibilitySelectedState IsSelected() const;
  virtual bool IsSelectedFromFocusSupported() const;
  // Is the object selected because selection is following focus?
  virtual bool IsSelectedFromFocus() const;
  virtual bool IsNotUserSelectable() const;
  virtual bool IsVisible() const;
  virtual bool IsVisited() const;

  // Returns true if the current element is text or an image, and all
  // the children are also plain content by this definition.
  // TODO(https://crbug.com/1426889): return false if there are "interesting"
  // elements in the subtree, even within a deep descendant, possibly reusing
  // CSS :has() in order to make a performant and flexible implementation.
  bool IsPlainContent() const;

  // Check whether value can be modified.
  bool CanSetValueAttribute() const;

  // Is the element focusable?
  bool CanSetFocusAttribute() const { return cached_can_set_focus_attribute_; }
  bool CanSetFocusAttribute();

  // Whether objects are included in the tree. Nodes that are included in the
  // tree are serialized, even if they are ignored. This allows browser-side
  // accessibility code to have a more accurate representation of the tree. e.g.
  // inspect hidden nodes referenced by labeled-by, know where line breaking
  // elements are, etc.
  bool IsIncludedInTree() const;
  bool IsIncludedInTree();
  bool CachedIsIncludedInTree() const;

  // Whether objects are ignored, i.e. hidden from the AT.
  bool IsIgnored() const;
  bool IsIgnored();

  // Whether an ignored object should still be included in the serialized tree.
  // Reasons for doing this:
  // - An object is in a hidden subtree that will be recursive name computation,
  // which traverses the AXObject hierarchy.
  // - Internal bookkeeping reasons, e.g. keeping children that cannot be
  // reached via NodeTraversal, as LayoutTreeBuilderTraversal is not always
  // safe.
  // - Line breaking objects and objects with an associated language, which
  // although not exposed via a11y APIs, are useful in browser-side property
  // computations for nodes that are exposed.
  bool IsIgnoredButIncludedInTree() const;
  bool IsIgnoredButIncludedInTree();

  // Is visibility:hidden or display:none being used to hide this element.
  bool IsHiddenViaStyle() const { return cached_is_hidden_via_style_; }
  bool IsHiddenViaStyle();
  // Whether this is part of the label or description for another element.
  // This is used to ensure hidden objects are included in the tree, and the
  // implementation currently only ensures that an element's ancestor was part
  // of a label or description at some point during the lifetime of the page, as
  // the relation cache does not bother clear old aria-labelledby/describedby
  // ids. However, for purposes of preventing too many hidden objects from being
  // serialized, it works well.
  bool IsUsedForLabelOrDescription() const {
    return cached_is_used_for_label_or_description_;
  }
  bool IsUsedForLabelOrDescription();

  typedef HeapVector<IgnoredReason> IgnoredReasons;
  virtual bool ComputeIsIgnored(IgnoredReasons* = nullptr) const;
  bool ShouldIgnoreForHiddenOrInert(IgnoredReasons* = nullptr) const;
  bool IsInert() const { return cached_is_inert_; }
  bool IsInert();
  bool IsAriaHiddenRoot() const;
  bool IsAriaHidden() const { return cached_is_aria_hidden_; }
  bool IsAriaHidden();
  const AXObject* AriaHiddenRoot() const;
  bool ComputeIsInert(IgnoredReasons* = nullptr) const;
  bool ComputeIsAriaHidden(IgnoredReasons* = nullptr) const;
  bool IsBlockedByAriaModalDialog(IgnoredReasons* = nullptr) const;
  bool IsDescendantOfDisabledNode() const {
    return cached_is_descendant_of_disabled_node_;
  }
  bool IsDescendantOfDisabledNode();
  bool ComputeIsIgnoredButIncludedInTree();
  const AXObject* GetAtomicTextFieldAncestor(int max_levels_to_check = 3) const;
  const AXObject* DatetimeAncestor() const;
  bool ComputeIsDescendantOfDisabledNode();
  // Some objects, such as table header containers, could be the children of
  // more than one object but have only one primary parent.
  bool HasIndirectChildren() const;
  bool IsExcludedByFormControlsFilter() const;

  void SetIsOnScreen(bool visibility) { cached_is_on_screen_ = visibility; }
  bool WasEverOnScreen() const {
    return cached_is_on_screen_ ? cached_is_on_screen_.value() : false;
  }

  // A node can oly flip from off-screen to on-screen if it was explicitly
  // marked as off-screen at some point. Since we keep track if a node was ever
  // on-screen, it can't also flip from on-screen to off-screen because of this
  // reason.
  bool CanFlipFromOffScreenToOnScreen() const {
    return cached_is_on_screen_ && !cached_is_on_screen_.value();
  }

  //
  // Accessible name calculation
  //

  // Retrieves the accessible name of the object, an enum indicating where the
  // name was derived from, and a list of objects that were used to derive the
  // name, if any.
  virtual String GetName(ax::mojom::blink::NameFrom&,
                         AXObjectVector* name_objects) const;

  typedef HeapVector<NameSource> NameSources;
  // Retrieves the accessible name of the object and a list of all potential
  // sources for the name, indicating which were used.
  String GetName(NameSources*) const;

  typedef HeapVector<DescriptionSource> DescriptionSources;
  // Takes the result of nameFrom from calling |name|, above, and retrieves the
  // accessible description of the object, which is secondary to |name|, an enum
  // indicating where the description was derived from, and a list of objects
  // that were used to derive the description, if any.
  virtual String Description(ax::mojom::blink::NameFrom,
                             ax::mojom::blink::DescriptionFrom&,
                             AXObjectVector* description_objects) const {
    return String();
  }

  // Same as above, but returns a list of all potential sources for the
  // description, indicating which were used.
  virtual String Description(ax::mojom::blink::NameFrom,
                             ax::mojom::blink::DescriptionFrom&,
                             DescriptionSources*,
                             AXRelatedObjectVector*) const {
    return String();
  }

  // Takes the result of nameFrom and descriptionFrom from calling |name| and
  // |description|, above, and retrieves the placeholder of the object, if
  // present and if it wasn't already exposed by one of the two functions above.
  virtual String Placeholder(ax::mojom::blink::NameFrom) const {
    return String();
  }

  // Takes the result of nameFrom and retrieves the HTML Title of the object,
  // if present and if it wasn't already exposed by |GetName| above.
  // HTML Title is typically used as a tooltip.
  virtual String Title(ax::mojom::blink::NameFrom) const { return String(); }

  // Internal functions used by name and description, above.
  typedef HeapHashSet<Member<const AXObject>> AXObjectSet;
  virtual String TextAlternative(bool recursive,
                                 const AXObject* aria_label_or_description_root,
                                 AXObjectSet& visited,
                                 ax::mojom::blink::NameFrom& name_from,
                                 AXRelatedObjectVector* related_objects,
                                 NameSources* name_sources) const {
    return String();
  }
  virtual String TextFromDescendants(
      AXObjectSet& visited,
      const AXObject* aria_label_or_description_root,
      bool recursive) const {
    return String();
  }

  // Returns result of Accessible Name Calculation algorithm.
  // This is a simpler high-level interface to |name| used by Inspector.
  // If name_from_out is non-null, it will contain the source of the name.
  String ComputedName(
      ax::mojom::blink::NameFrom* name_from_out = nullptr) const;

  // Internal function used to determine whether the element supports deriving
  // its accessible name from its descendants. The result of calling |GetName|
  // may be derived by other means even when this returns true.
  // This is intended to be faster than calling |GetName| or
  // |TextAlternative|, and without side effects (it won't call
  // AXObjectCache->GetOrCreate).
  bool SupportsNameFromContents(bool recursive,
                                bool consider_focus = false) const;

  //
  // Properties of static elements.
  //

  virtual const AtomicString& AccessKey() const { return g_null_atom; }
  virtual RGBA32 BackgroundColor() const { return Color::kTransparent.Rgb(); }
  virtual RGBA32 GetColor() const { return Color::kBlack.Rgb(); }
  // Used by objects of role ColorWellRole.
  virtual RGBA32 ColorValue() const { return Color::kTransparent.Rgb(); }
  virtual bool CanvasHasFallbackContent() const { return false; }
  // Returns the font family that was cascaded onto ComputedStyle. This may
  // contain non-user-friendly internal names.
  virtual const AtomicString& ComputedFontFamily() const { return g_null_atom; }
  // Returns the font family used on this platform. This is a user friendly name
  // that is appropriate for serialization.
  virtual String FontFamilyForSerialization() const { return String(); }
  // Font size is in pixels.
  virtual float FontSize() const { return 0.0f; }
  virtual float FontWeight() const { return 0.0f; }
  // Value should be 1-based. 0 means not supported.
  virtual int HeadingLevel() const { return 0; }
  // Value should be 1-based. 0 means not supported.
  virtual unsigned HierarchicalLevel() const { return 0; }
  // Return the content of an image or canvas as an image data url in
  // PNG format. If |maxSize| is not empty and if the image is larger than
  // those dimensions, the image will be resized proportionally first to fit.
  virtual String ImageDataUrl(const gfx::Size& max_size) const {
    return g_null_atom;
  }
  // If this element points to another element in the same page, e.g.
  // <a href="#foo">, this will return the AXObject for the target.
  // The object returned should be unignored. If necessary, it will return
  // a descendant of the actual target.
  virtual AXObject* InPageLinkTarget() const;
  // Returns the value of the "target" attribute, e.g. <a href="example.com"
  // target="blank">.
  virtual const AtomicString& EffectiveTarget() const;
  virtual AccessibilityOrientation Orientation() const;
  virtual ax::mojom::blink::ListStyle GetListStyle() const {
    return ax::mojom::blink::ListStyle::kNone;
  }
  virtual ax::mojom::blink::TextAlign GetTextAlign() const {
    return ax::mojom::blink::TextAlign::kNone;
  }
  virtual ax::mojom::blink::WritingDirection GetTextDirection() const {
    return ax::mojom::blink::WritingDirection::kLtr;
  }
  virtual float GetTextIndent() const { return 0.0f; }
  virtual ax::mojom::blink::TextPosition GetTextPosition() const {
    return ax::mojom::blink::TextPosition::kNone;
  }

  virtual void GetTextStyleAndTextDecorationStyle(
      int32_t* text_style,
      ax::mojom::blink::TextDecorationStyle* text_overline_style,
      ax::mojom::blink::TextDecorationStyle* text_strikethrough_style,
      ax::mojom::blink::TextDecorationStyle* text_underline_style) const {
    *text_style = 0;
    *text_overline_style = ax::mojom::blink::TextDecorationStyle::kNone;
    *text_strikethrough_style = ax::mojom::blink::TextDecorationStyle::kNone;
    *text_underline_style = ax::mojom::blink::TextDecorationStyle::kNone;
  }

  virtual AXObject* GetChildFigcaption() const;
  virtual bool IsDescendantOfLandmarkDisallowedElement() const;

  virtual AXObjectVector RadioButtonsInGroup() const {
    return AXObjectVector();
  }
  virtual KURL Url() const { return KURL(); }
  virtual AXObject* ChooserPopup() const { return nullptr; }

  // Load inline text boxes for just this node, even if
  // AXObjectCache().GetAXMode().has_mode(ui::AXMode::kInlineTextBoxes) is
  // false. Must be called with clean layout.
  virtual void LoadInlineTextBoxes();
  virtual void LoadInlineTextBoxesHelper();
  // When adding children to this node, consider inline textboxes.
  virtual bool ShouldLoadInlineTextBoxes() const { return false; }

  // Walk the AXObjects on the same line.
  virtual AXObject* NextOnLine() const;
  virtual AXObject* PreviousOnLine() const;

  // Searches the object's ancestors for an aria-invalid attribute of type
  // spelling or grammar, and returns a document marker representing the value
  // of this attribute. As an optimization, goes up until the deepest line
  // breaking object which, in most cases, is the paragraph containing this
  // object.
  std::optional<const DocumentMarker::MarkerType>
  GetAriaSpellingOrGrammarMarker() const;

  // For all inline text objects: Returns the horizontal pixel offset of each
  // character in the object's text, rounded to the nearest integer. Negative
  // values are returned for RTL text.
  virtual void TextCharacterOffsets(Vector<int>&) const;

  // For all inline text boxes: Returns the start and end character offset of
  // each word in the object's text.
  virtual void GetWordBoundaries(Vector<int>& word_starts,
                                 Vector<int>& word_ends) const;

  // For all inline text boxes and atomic text fields: Returns the length of the
  // inline's text or the field's value respectively.
  virtual int TextLength() const;

  // Supported on layout inline, layout text, layout replaced, and layout block
  // flow, provided that they are at inline-level, i.e. "display=inline" or
  // "display=inline-block". Also supported on atomic text fields. For all other
  // object types, returns |offset|.
  //
  // For layout inline, text, replaced, and block flow: Translates the given
  // character offset to the equivalent offset in the object's formatting
  // context. The formatting context is the deepest block flow ancestor,
  // (excluding the current object), e.g. the containing paragraph. If this
  // object is somehow not a descendant of a block flow in the layout tree,
  // returns |offset|.
  //
  // For example, if this object is a span, and |offset| is 0, this method would
  // return the number of characters, excluding any collapsed white space found
  // in the DOM, from the start of the layout inline's deepest block flow
  // ancestor, e.g. the beginning of the paragraph in which the span is found.
  //
  // For atomic text fields: Simply returns |offset|, because atomic text fields
  // have no collapsed white space and so no translation from a DOM to an
  // accessible text offset is necessary. An atomic text field does not expose
  // its internal implementation to assistive software, appearing as a single
  // leaf node in the accessibility tree. It includes <input> and <textarea>.
  virtual int TextOffsetInFormattingContext(int offset) const;

  // For all inline text boxes and atomic text fields. For all other object
  // types, returns |offset|.
  //
  // For inline text boxes: Translates the given character offset to the
  // equivalent offset in the object's static text or line break parent. If this
  // object is somehow not a descendant of a block flow in the layout tree,
  // returns the given offset.
  //
  // For example, if the given offset is 0, this would return the number of
  // characters, excluding any collapsed white space found in the DOM, from the
  // start of the inline text box's static text parent.
  //
  // For atomic text fields: Simply returns |offset|, because atomic text fields
  // have no collapsed white space and so no translation from a DOM to an
  // accessible text offset is necessary.
  virtual int TextOffsetInContainer(int offset) const;

  // Properties of interactive elements.
  ax::mojom::blink::DefaultActionVerb Action() const;
  ax::mojom::blink::CheckedState CheckedState() const;
  virtual ax::mojom::blink::AriaCurrentState GetAriaCurrentState() const {
    return ax::mojom::blink::AriaCurrentState::kNone;
  }
  virtual ax::mojom::blink::InvalidState GetInvalidState() const {
    return ax::mojom::blink::InvalidState::kNone;
  }
  virtual bool ValueForRange(float* out_value) const { return false; }
  virtual bool MaxValueForRange(float* out_value) const { return false; }
  virtual bool MinValueForRange(float* out_value) const { return false; }
  virtual bool StepValueForRange(float* out_value) const { return false; }

  // Returns the value of a control such as a plain text field, a content
  // editable, a submit button, a slider, a progress bar, a scroll bar, a meter,
  // a spinner, a <select> element, a date picker or an ARIA combo box. In order
  // to improve performance during our cross-process communication with the
  // browser, we avoid computing the value of a content editable from its inner
  // text. (See `AXObject::SlowGetValueForControlIncludingContentEditable()`.)
  // For range controls, such as sliders and scroll bars, the value of
  // aria-valuetext takes priority over the value of aria-valuenow.
  virtual String GetValueForControl() const;
  virtual String GetValueForControl(AXObjectSet& visited) const;

  // Similar to `AXObject::GetValueForControl()` above, but also computes the
  // value of a content editable from its inner text. Sending this value to the
  // browser process might be slow if the content editable has a lot of content.
  // So, we should prefer computing the value of a content editable on the
  // browser side.
  virtual String SlowGetValueForControlIncludingContentEditable() const;
  virtual String SlowGetValueForControlIncludingContentEditable(
      AXObjectSet& visited) const;

  virtual AXRestriction Restriction() const;

  //
  // ARIA role attribute.
  //
  // How role calculation works:
  //
  // Note that “ARIA role” does not refer to the same thing as “role” here.
  //
  // (1) Extract the raw ARIA role from the role=”role type” in the object.
  // (2) Process the raw ARIA role by applying a set of rules. This new value is
  //     considered the ARIA role.
  // (3) Determine the native role.
  // (4) Using the ARIA role and native role, determine the role.
  // (5) If possible, apply contextual rules on the role to get the final role.
  //
  // Because the final role calculation in (5) involves ancestor values, a
  // change in an ancestor can affect the final role of the object. In cases
  // where it is difficult to check for this change, the role from (4) is used
  // instead of the final role from (5).

  // (1) Determine the ARIA role purely based on the role attribute, when no
  // additional rules or limitations on role usage are applied. Use
  // RawAriaRole() instead if the raw role does not need to be recomputed.
  ax::mojom::blink::Role DetermineRawAriaRole() const;

  // (2) Determine the ARIA role after applying rules based on other properties.
  ax::mojom::blink::Role DetermineAriaRole() const;

  // (3) Determine the native role using other ARIA properties (without using
  // the ARIA role).
  virtual ax::mojom::blink::Role NativeRoleIgnoringAria() const = 0;

  // (4) Determine the role using the ARIA role and native role. Use
  // RoleValue() instead if the role does not need to be recomputed.
  virtual ax::mojom::blink::Role DetermineRoleValue();

  // (5) Return the role after all possible rules from HTML-AAM, WAI-ARIA, etc.
  // have been applied.
  //
  // This method is useful in cases where the final role exposed to ATs needs
  // to change based on contextual information. For instance, an svgRoot should
  // be exposed as an image if it lacks accessible children. Whether or not it
  // has accessible children is not known at the time the role is assigned and
  // may depend on whether or not a given platform includes children that other
  // platforms ignore.
  ax::mojom::blink::Role ComputeFinalRoleForSerialization() const;

  // Returns the cached raw ARIA role from DetermineRawAriaRole().
  virtual ax::mojom::blink::Role RawAriaRole() const;

  // Returns the cached role from DetermineRoleValue().
  ax::mojom::blink::Role RoleValue() const;

  // The role attribute is a string consisting of an ordered list of one or more
  // roles (aka tokens) separated by spaces. This method finds the first token
  // in the string that matches an internal role enum and returns that enum.
  //
  // The roles listed after that first role are considered fallback roles. A
  // fallback role can be used when the first role cannot be used (due to an
  // authoring error). If a fallback role for a nameless form or region is
  // needed, set ignore_form_and_region to true.
  //
  // https://w3c.github.io/aria/#document-handling_author-errors_roles
  static ax::mojom::blink::Role FirstValidRoleInRoleString(
      const String&,
      bool ignore_form_and_region = false);

  // Return the equivalent ARIA name for an enumerated role, or g_null_atom.
  static const AtomicString& AriaRoleName(ax::mojom::blink::Role);

  // Return the equivalent internal role name as a string. Used in DOM Inspector
  // and for debugging.
  static const String InternalRoleName(ax::mojom::blink::Role);

  // Return a role name, preferring the ARIA over the internal name.
  // Optional boolean out param |*is_internal| will be false if the role matches
  // an ARIA role, and true if an internal role name is used (no ARIA mapping).
  static const String RoleName(ax::mojom::blink::Role,
                               bool* is_internal = nullptr);

  // Get the role to be used in StringAttribute::kRole, which is used in the
  // xml-roles object attribute.
  const AtomicString& GetRoleStringForSerialization(ui::AXNodeData* node_data) const;

  // ARIA attributes.
  bool ElementHasAnyAriaAttribute(
      bool does_undo_role_presentation = false) const;
  virtual AXObject* ActiveDescendant() const { return nullptr; }
  virtual String AutoComplete() const { return String(); }
  virtual AXObjectVector ErrorMessage() const { return AXObjectVector(); }
  virtual AXObjectVector ErrorMessageFromHTML() const {
    return AXObjectVector();
  }
  virtual AXObjectVector RelationVectorFromAria(
      const QualifiedName& attr_name) const {
    return AXObjectVector();
  }

  // Determines whether this object has an associated popup menu, list, or grid,
  // such as in the case of an ARIA combobox or when the browser offers an
  // autocomplete suggestion.
  virtual ax::mojom::blink::HasPopup HasPopup() const;

  // Determines whether this object is a popup, and what type.
  virtual ax::mojom::blink::IsPopup IsPopup() const;

  // Heuristic to get the target popover for an invoking element.
  AXObject* GetPopoverTargetForInvoker() const;

  // Heuristic to get the target element defined by the `commandfor` attribute
  // on an invoking element.
  AXObject* GetCommandForElement() const;

  // Heuristic to get the interest target for an invoking element.
  // Returns null if the interest target points to plain content and can be
  // expose as a description instead.
  AXObject* GetInterestTargetForInvoker() const;

  // Elements can be positioned relative to other elements with CSS anchor
  // positioning. This function returns the positioned element that should be
  // added to the aria-details list.
  AXObject* GetPositionedObjectForAnchor(ui::AXNodeData* node_data) const;

  // Heuristic to get the listbox for an <input role="combobox">.
  AXObject* GetControlsListboxForTextfieldCombobox() const;

  // Returns true if this object is within or at the root of an editable region,
  // such as a contenteditable. Also, returns true if this object is an atomic
  // text field, i.e. an input or a textarea. Note that individual subtrees
  // within an editable region could be made non-editable via e.g.
  // contenteditable="false".
  bool IsEditable() const;

  // Returns true if this object is at the root of an editable region, such as a
  // contenteditable. Does not return true if this object is an atomic text
  // field, i.e. an input or a textarea.
  //
  // https://w3c.github.io/editing/execCommand.html#editing-host
  virtual bool IsEditableRoot() const;

  // Returns true if this object has contenteditable="true" or
  // contenteditable="plaintext-only".
  virtual bool HasContentEditableAttributeSet() const;

  // Returns true if the user can enter multiple lines of text inside this
  // editable region. By default, textareas and content editables can accept
  // multiple lines of text.
  bool IsMultiline() const;

  // Same as `IsEditable()` but returns whether the region accepts rich text
  // as well.
  bool IsRichlyEditable() const;

  bool SupportsARIAExpanded() const;
  bool SupportsARIAReadOnly() const;

  // Returns 0-based index.
  int IndexInParent() const;

  // Returns true if the object is not orphaned and has no siblings.
  bool IsOnlyChild() const;

  // Value should be 1-based. 0 means not supported.
  virtual int PosInSet() const { return 0; }
  virtual int SetSize() const { return 0; }
  bool SupportsARIASetSizeAndPosInSet() const;

  // Helpers for menulist, aka <select size=1>.
  bool IsMenuList() const;
  bool ComputeIsInMenuListSubtree();
  bool IsInMenuListSubtree() const { return cached_is_in_menu_list_subtree_; }
  bool IsInMenuListSubtree();
  // Find first ancestor or |this| that matches.
  const AXObject* AncestorMenuListOption() const;
  const AXObject* AncestorMenuList() const;

  // ARIA live-region features.
  bool IsLiveRegionRoot() const;  // Any live region, including polite="off".
  bool IsActiveLiveRegionRoot() const;  // Live region that is not polite="off".
  // Containing element that controls aria-live properties.
  AXObject* LiveRegionRoot() const { return cached_live_region_root_; }
  AXObject* LiveRegionRoot();
  virtual const AtomicString& LiveRegionStatus() const;
  virtual const AtomicString& LiveRegionRelevant() const;
  bool LiveRegionAtomic() const;

  const AtomicString& ContainerLiveRegionStatus() const;
  const AtomicString& ContainerLiveRegionRelevant() const;
  bool ContainerLiveRegionAtomic() const;
  bool ContainerLiveRegionBusy() const;

  // Every object's bounding box is returned relative to a
  // container object (which is guaranteed to be an ancestor) and
  // optionally a transformation matrix that needs to be applied too.
  // To compute the absolute bounding box of an element, start with its
  // boundsInContainer and apply the transform. Then as long as its container is
  // not null, walk up to its container and offset by the container's offset
  // from origin, the container's scroll position if any, and apply the
  // container's transform.  Do this until you reach the root of the tree.
  // If the object clips its children, for example by having overflow:hidden,
  // set |clips_children| to true.
  virtual void GetRelativeBounds(AXObject** out_container,
                                 gfx::RectF& out_bounds_in_container,
                                 gfx::Transform& out_container_transform,
                                 bool* clips_children = nullptr) const;

  gfx::RectF LocalBoundingBoxRectForAccessibility();

  // Get the bounds in frame-relative coordinates as a PhysicalRect.
  PhysicalRect GetBoundsInFrameCoordinates() const;

  // Hit testing.
  // Called on the root AX object to return the deepest available element.
  virtual AXObject* AccessibilityHitTest(const gfx::Point&) const {
    return nullptr;
  }
  // Called on the AX object after the layout tree determines which is the right
  // AXObject.
  AXObject* ElementAccessibilityHitTest(const gfx::Point&) const;

  //
  // High-level accessibility tree access. Other modules should only use these
  // methods.
  //
  // The following methods may support one or more kinds of objects. There are
  // three kinds: Objects that are excluded from the accessibility tree by
  // default, such as white space found in HTML, objects that are included in
  // the tree but that are ignored, such as an empty div, and unignored objects.

  // Iterates through the node's unignored ancestors up to the root, starting
  // from the node's unignored parent, i.e. does not include the node itself in
  // the list of ancestors.
  //
  // Initially, it can be called on all nodes, including those that are
  // accessibility ignored, but only traverses through the list of ancestors
  // that are unignored and included in the accessibility tree.
  AncestorsIterator UnignoredAncestorsBegin() const;
  AncestorsIterator UnignoredAncestorsEnd() const;

  // Returns the number of children, including children that are included in the
  // accessibility tree but are accessibility ignored.
  //
  // Can be called on all nodes, even on nodes that are excluded from the
  // accessibility tree.
  int ChildCountIncludingIgnored() const;

  // Returns the child with the given index in the list of all children,
  // including those that are accessibility ignored.
  //
  // Can be called on all nodes, even on nodes that are excluded from the
  // accessibility tree.
  AXObject* ChildAtIncludingIgnored(int index) const;

  // Returns the node's children, including any children that are included in
  // the accessibility tree but are accessibility ignored.
  //
  // Can be called on all nodes, including nodes that are excluded from the
  // accessibility tree.
  const AXObjectVector& ChildrenIncludingIgnored() const;
  const AXObjectVector& ChildrenIncludingIgnored();

  // Returns the node's unignored descendants that are one level deeper than
  // this node, after removing all accessibility ignored nodes from the tree.
  //
  // Flattens accessibility ignored nodes, so each unignored child will have the
  // same unignored parent, but may have a different parent in tree.
  //
  // Can be called on all nodes that are included in the accessibility tree,
  // including those that are accessibility ignored.
  // TODO(accessibility) This actually returns ignored children when they are
  // included in the tree. A better name would be ChildrenIncludedInTree().
  const AXObjectVector UnignoredChildren() const;
  const AXObjectVector UnignoredChildren();

  // Returns the first child for this object.
  // Works for all nodes that are included in the accessibility tree, and may
  // return nodes that are accessibility ignored.
  AXObject* FirstChildIncludingIgnored() const;

  // Returns the last child for this object.
  // Works for all nodes that are included in the accessibility tree, and may
  // return nodes that are accessibility ignored.
  AXObject* LastChildIncludingIgnored() const;

  // Returns the deepest first child for this object.
  // Works for all nodes that are included in the accessibility tree, and may
  // return nodes that are accessibility ignored.
  AXObject* DeepestFirstChildIncludingIgnored() const;

  // Returns the deepest last child for this object.
  // Works for all nodes that are included in the accessibility tree, and may
  // return nodes that are accessibility ignored.
  AXObject* DeepestLastChildIncludingIgnored() const;

  // Next sibling for this object, where the sibling may be
  // an accessibility ignored object.
  // Works for all nodes that are included in the accessibility tree,
  // and may return nodes that are accessibility ignored.
  AXObject* NextSiblingIncludingIgnored() const;

  // Previous sibling for this object, where the sibling may be
  // an accessibility ignored object.
  // Works for all nodes that are included in the accessibility tree,
  // and may return nodes that are accessibility ignored.
  AXObject* PreviousSiblingIncludingIgnored() const;
  // This version is safe to call in methods used to build the parents children.
  AXObject* CachedPreviousSiblingIncludingIgnored() const;

  // Returns the next object in tree using depth-first pre-order traversal,
  // optionally staying within a specified AXObject.
  // Works for all nodes that are included in the accessibility tree,
  // and may return nodes that are accessibility ignored.
  AXObject* NextInPreOrderIncludingIgnored(
      const AXObject* within = nullptr) const;

  // Returns the previous object in tree using depth-first pre-order traversal,
  // optionally staying within a specified AXObject.
  // Works for all nodes that are included in the accessibility tree,
  // and may return nodes that are accessibility ignored.
  AXObject* PreviousInPreOrderIncludingIgnored(
      const AXObject* within = nullptr) const;

  // Returns the previous object in tree using depth-first post-order traversal,
  // optionally staying within a specified AXObject.
  // Works for all nodes that are included in the accessibility tree,
  // and may return nodes that are accessibility ignored.
  AXObject* PreviousInPostOrderIncludingIgnored(
      const AXObject* within = nullptr) const;

  // Returns the first object (using pre-order search) that has the given role
  // in the subtree rooted at this object.
  AXObject* FirstObjectWithRole(ax::mojom::blink::Role role) const;

  // Returns the number of children that are not accessibility ignored.
  //
  // Unignored children are the objects that are one level deeper than the
  // current object after all accessibility ignored descendants are removed.
  //
  // Can be called on all nodes that are included in the accessibility tree,
  // including those that are accessibility ignored.
  int UnignoredChildCount() const;

  // Returns the unignored child with the given index.
  //
  // Unignored children are the objects that are one level deeper than the
  // current object after all accessibility ignored descendants are removed.
  //
  // Can be called on all nodes that are included in the accessibility tree,
  // including those that are accessibility ignored.
  AXObject* UnignoredChildAt(int index) const;

  // Next sibling for this object that's not accessibility ignored.
  //
  // Flattens accessibility ignored nodes, so the sibling will have the
  // same unignored parent, but may have a different parent in tree.
  //
  // Doesn't work with nodes that are accessibility ignored.
  AXObject* UnignoredNextSibling() const;

  // Previous sibling for this object that's not accessibility ignored.
  //
  // Flattens accessibility ignored nodes, so the sibling will have the
  // same unignored parent, but may have a different parent in tree.
  //
  // Doesn't work with nodes that are accessibility ignored.
  AXObject* UnignoredPreviousSibling() const;

  // Next object in tree using depth-first pre-order traversal that's
  // not accessibility ignored.
  // Doesn't work with nodes that are accessibility ignored.
  AXObject* UnignoredNextInPreOrder() const;

  // Previous object in tree using depth-first pre-order traversal that's
  // not accessibility ignored.
  // Doesn't work with nodes that are accessibility ignored.
  AXObject* UnignoredPreviousInPreOrder() const;

  // Get the parent of this object.
  //
  // Works for all nodes, and may return nodes that are ignored,
  // including nodes that might not be in the tree.
  // - ParentObject() (const) asserts the parent is present.
  // - ParentObject() (non-const) returns the parent if there is one, otherwise
  //   it prunes the subtree.
  // - ParentObjectIfPresent() returns null if the parent is missing.
  // Both methods return null for the root.
  // Most callers should use ParentObject(), but ParentObjectIfPresent() can be
  // helpful when parent-child relations are being constructed or torn down.
  AXObject* ParentObject() const;
  AXObject* ParentObject();
  AXObject* ParentObjectIfPresent() const { return parent_; }

  // Get the current unignored children without refreshing them, even if
  // children_dirty_ aka NeedsToUpdateChildren() is true.
  const AXObjectVector& CachedChildrenIncludingIgnored() const {
    return children_;
  }

  // Sets the parent AXObject directly. If the parent of this object is known,
  // this can be faster than using ComputeParent().
  void SetParent(AXObject* new_parent);

  // If parent was not initialized during AddChildren() it can be computed by
  // walking the DOM (or layout for nodeless aka anonymous layout object).
  // ComputeParent() adds DCHECKs to ensure that it is not being called when
  // an attached parent_ is already cached, and that it is possible to compute
  // the parent. It calls ComputeParentImpl() for the actual work.
  AXObject* ComputeParent() const;

  // Same as ComputeParent, but does not assert if there is no parent to compute
  // (i.e. because the parent does not belong to the tree anymore).
  AXObject* ComputeParentOrNull() const;

  // Can this node be used to compute the natural parent of an object?
  // A natural parent is one where the LayoutTreeBuilderTraversal::Parent()
  // matches the DOM node for the AXObject parent.
  // Counter examples:
  // * An image cannot be a natural parent, because while it can be the parent
  // of <area> elements, there isn't a matching DOM parent-child relationship,
  // as the areas are associated via a <map> element.
  static bool CanComputeAsNaturalParent(Node*);

  // For a given image, return a <map> that's actually used for it.
  static HTMLMapElement* GetMapForImage(Node* image);

  // Can AXObjects backed by this element have AXObject children?
  static bool CanHaveChildren(Element& element);

  // Given the candidate parent node, return a node that can be used for the
  // parent, or null if no parent is possible. For example, passing in a <map>
  // will return the associated <img>, because the image would parent any of the
  // map's descendants.
  static Node* GetParentNodeForComputeParent(AXObjectCacheImpl&, Node*);

  // Compute the AXObject parent for the given node.
  // Does not take aria-owns into account.
  static AXObject* ComputeNonARIAParent(AXObjectCacheImpl& cache, Node* node);

  // Returns true if |parent_| is null and not at the root.
  bool IsMissingParent() const;

  // Compute a missing parent, and ask it to update children.
  // Must only be called if IsMissingParent() is true.
  void RepairMissingParent() const;

  // Is this the root of this object hierarchy.
  bool IsRoot() const;

#if AX_FAIL_FAST_BUILD()
  // Get/Prints the entire AX subtree to the screen for debugging, with |this|
  // highlighted via a "*" notation.
  std::string GetAXTreeForThis() const;
  void ShowAXTreeForThis() const;
#endif

#if EXPENSIVE_DCHECKS_ARE_ON()
  // Check that all objects in the subtree, even unincluded ones, are flagged as
  // being part of a name or description, so that the algorithm for determining
  // whether ignored objects should be included can return true for hidden nodes
  // needed for label or description computations.
  void CheckSubtreeIsForLabelOrDescription(const AXObject*) const;
#endif

  // Get or create the first ancestor that's not accessibility ignored.
  // Works for all nodes.
  AXObject* ParentObjectUnignored() const;

  // Get or create the first ancestor that's included in the accessibility tree.
  // Works for all nodes, and may return nodes that are accessibility ignored.
  AXObject* ParentObjectIncludedInTree() const;

  AXObject* ContainerWidget() const;
  bool IsContainerWidget() const;

  AXObject* ContainerListMarkerIncludingIgnored() const;

  // There are two types of traversal for obtaining children:
  // 1. LayoutTreeBuilderTraversal. Despite the name, this traverses a flattened
  // DOM tree that includes pseudo element children such as ::before, and where
  // shadow DOM slotting has been run.
  // 2. LayoutObject traversal. This is necessary if there is no parent node,
  // or in a pseudo element subtree.
  bool ShouldUseLayoutObjectTraversalForChildren() const;
  // Is this a safe time to use FlatTreeTraversal in this document? Also covers
  // use of LayoutTreeBuilderTraversal, which is used often in the accessibility
  // module, and built on top of FlatTreeTraversal.
  static bool CanSafelyUseFlatTreeTraversalNow(Document& document);
  virtual bool CanHaveChildren() const { return true; }
  void UpdateChildrenIfNecessary();
  bool NeedsToUpdateChildren() const;
  void SetNeedsToUpdateChildren(bool update = true);
  virtual void ClearChildren();
  void DetachFromParent();
  virtual void SelectedOptions(AXObjectVector&) const {}

  // Properties of the object's owning document or page.
  virtual double EstimatedLoadingProgress() const { return 0; }
  virtual AXObject* RootScroller() const;

  //
  // DOM and layout tree access.
  //

  // Returns the associated DOM node or, if an associated layout object is
  // present, the node of the associated layout object.
  //
  // If this object is associated with generated content, or a list marker,
  // returns a pseudoelement. It does not return the node that generated the
  // content or the list marker.
  virtual Node* GetNode() const;
  // Looks for the first ancestor AXObject (inclusive) that has a node, and
  // returns that node.
  Node* GetClosestNode() const {
    return GetNode() ? GetNode() : ParentObject()->GetClosestNode();
  }
  // Looks for the first ancestor AXObject (inclusive) that has an element, and
  // returns that element.
  Element* GetClosestElement() const;

  // Returns the associated layout object if any.
  virtual LayoutObject* GetLayoutObject() const;

  // Returns the same as `AXObject::GetNode()` if the node is an Element,
  // otherwise returns nullptr.
  Element* GetElement() const;

  virtual Document* GetDocument() const = 0;
  LocalFrameView* DocumentFrameView() const;
  virtual Element* AnchorElement() const { return nullptr; }
  virtual Element* ActionElement() const { return nullptr; }

  // For non-root nodes, this returns the language attribute value. For the
  // root node (kRootWebArea), this returns the first non-empty value from the
  // following list: the language attribute in the <html> element, the language
  // specified in the <meta> tag, the Accept-Language HTTP header, the default
  // language of the browser's UI.
  AtomicString Language() const;

  // ARIA attribute access: use these methods in order to ensure that values
  // are also retrieved from elementInternals on custom elements.
  // For non-ARIA attributes, it's ok to just use Element methods.
  bool HasAriaAttribute(const QualifiedName&) const;
  static bool HasAriaAttribute(const Element& element, const QualifiedName&);
  const AtomicString& AriaAttribute(const QualifiedName&) const;
  static const AtomicString& AriaAttribute(const Element& element,
                                           const QualifiedName&);

  // The following HasAriaFooAttribute() methods return true if the attribute
  // is present. `out_value` is filled with the value of the attribute or a
  // default value if the attribute is not present.
  bool AriaBooleanAttribute(const QualifiedName& attribute,
                            bool* out_value = nullptr) const;
  bool AriaFloatAttribute(const QualifiedName& attribute,
                          float* out_value = nullptr) const;
  bool AriaIntAttribute(const QualifiedName& attribute,
                        int32_t* out_value = nullptr) const;
  const AtomicString& AriaTokenAttribute(const QualifiedName& attribute) const;
  // AriaStringAttribute() is a synonym for GetAttribute(), because it does
  // not need to do any additional processing on the value.
  const AtomicString& AriaStringAttribute(
      const QualifiedName& attribute) const {
    return AriaAttribute(attribute);
  }

  // Additional boolean ARIA convenience methods.
  bool IsAriaAttributeTrue(const QualifiedName&) const;
  static bool IsAriaAttributeTrue(const Element& element, const QualifiedName&);

  // Scrollable containers.
  bool IsScrollableContainer() const;
  bool IsUserScrollable() const;  // Only true if actual scrollbars are present.
  gfx::Point GetScrollOffset() const;
  gfx::Point MinimumScrollOffset() const;
  gfx::Point MaximumScrollOffset() const;
  void Scroll(ax::mojom::blink::Action scroll_action) const;
  void SetScrollOffset(const gfx::Point&) const;

  // Tables and grids.
  bool IsTableLikeRole() const;
  bool IsTableRowLikeRole() const;
  bool IsTableCellLikeRole() const;
  virtual bool IsDataTable() const { return false; }

  // For a table.
  virtual unsigned ColumnCount() const;
  virtual unsigned RowCount() const;
  virtual void ColumnHeaders(AXObjectVector&) const;
  virtual void RowHeaders(AXObjectVector&) const;
  virtual AXObject* CellForColumnAndRow(unsigned column, unsigned row) const;

  // For a cell.
  virtual unsigned ColumnIndex() const;
  virtual unsigned RowIndex() const;
  virtual unsigned ColumnSpan() const;
  virtual unsigned RowSpan() const;
  virtual ax::mojom::blink::SortDirection GetSortDirection() const {
    return ax::mojom::blink::SortDirection::kNone;
  }

  // For a row or column.
  virtual AXObject* HeaderObject() const { return nullptr; }

  // If this object itself scrolls, return its ScrollableArea.
  virtual ScrollableArea* GetScrollableAreaIfScrollable() const {
    return nullptr;
  }

  // Modify or take an action on an object. Returns true if handled.
  bool PerformAction(const ui::AXActionData&);
  // TODO(accessibility) Do this through PerformAction() and move to private.
  bool RequestScrollToMakeVisibleWithSubFocusAction(
      const gfx::Rect&,
      blink::mojom::blink::ScrollAlignment horizontal_scroll_alignment,
      blink::mojom::blink::ScrollAlignment vertical_scroll_alignment);

  // These are actions, just like the actions above, and they allow us
  // to keep track of nodes that gain or lose accessibility focus, but
  // this isn't exposed to the open web so they're explicitly marked as
  // internal so it's clear that these should not dispatch DOM events.
  virtual bool InternalSetAccessibilityFocusAction();
  virtual bool InternalClearAccessibilityFocusAction();

  // Native implementations of actions. These all return true if handled.
  virtual bool OnNativeDecrementAction();
  virtual bool OnNativeClickAction();
  virtual bool OnNativeBlurAction();
  virtual bool OnNativeFocusAction();
  virtual bool OnNativeIncrementAction();
  bool OnNativeScrollToGlobalPointAction(const gfx::Point&) const;
  bool OnNativeScrollToMakeVisibleAction() const;
  bool OnNativeScrollToMakeVisibleWithSubFocusAction(
      const gfx::Rect&,
      blink::mojom::blink::ScrollAlignment horizontal_scroll_alignment,
      blink::mojom::blink::ScrollAlignment vertical_scroll_alignment) const;
  virtual bool OnNativeSetSelectedAction(bool);
  virtual bool OnNativeSetSequentialFocusNavigationStartingPointAction();
  virtual bool OnNativeSetValueAction(const String&);
  bool OnNativeShowContextMenuAction();
  bool OnNativeKeyboardAction(const ax::mojom::Action action);

  // Notifications that this object may have changed.
  // TODO(accessibility): Remove virtual -- the only override is in a unit test.
  virtual void ChildrenChangedWithCleanLayout();
  virtual void HandleActiveDescendantChanged() {}
  virtual void HandleAutofillSuggestionAvailabilityChanged(
      WebAXAutofillSuggestionAvailability) {}
  virtual void HandleAriaExpandedChanged() {}

  // Static helper functions.
  // TODO(accessibility) Move these to a static helper util class.
  static bool IsFrame(const Node*);
  static bool HasARIAOwns(Element* element);
  // Should this own a child tree (e.g. an iframe).
  virtual bool IsEmbeddingElement() const { return false; }
  // Is this a widget that requires container widget.
  bool IsSubWidget() const;

  // Given two AX objects, returns the lowest common ancestor and the child
  // indices in that ancestor corresponding to the branch under which each
  // object is to be found. If the lowest common ancestor is the same as either
  // of the objects, the corresponding index is set to -1 to indicate this.
  static const AXObject* LowestCommonAncestor(const AXObject& first,
                                              const AXObject& second,
                                              int* index_in_ancestor1,
                                              int* index_in_ancestor2);

  bool IsHiddenForTextAlternativeCalculation(
      const AXObject* aria_label_or_description_root) const;

  // Extra checks that occur right before a node is evaluated for serialization.
  void PreSerializationConsistencyCheck() const;

  // Returns a string representation of this object.
  // Must only be used after `init()`has been called.
  virtual String ToString(bool verbose = true) const;
  static String GetNodeString(Node* node);

  void PopulateAXRelativeBounds(ui::AXRelativeBounds& bounds,
                                bool* clips_children) const;

 protected:
  AXID id_;
  // Any parent, regardless of whether it's ignored or not included in the tree.
  Member<AXObject> parent_;
  // Only children that are included in tree, maybe rename to children_in_tree_.
  AXObjectVector children_;
  bool has_dirty_descendants_ = false;

  // The final role, taking into account the ARIA role and native role.
  ax::mojom::blink::Role role_;

  virtual void AddChildren() = 0;

  // Collapses multiple whitespace characters into one. Used by GetName().
  String SimplifyName(const String&,
                      ax::mojom::blink::NameFrom& name_from) const;
  // Returns true if the object's role prohibits it from being named, even by
  // the author. See https://w3c.github.io/aria/#namefromprohibited
  bool IsNameProhibited() const;
  std::string GetProhibitedNameError(
      const String& prohibited_name,
      ax::mojom::blink::NameFrom& prohibited_name_from) const;

  static String RecursiveTextAlternative(
      const AXObject&,
      const AXObject* aria_label_or_description_root,
      AXObjectSet& visited);
  static String RecursiveTextAlternative(
      const AXObject&,
      const AXObject* aria_label_or_description_root,
      AXObjectSet& visited,
      ax::mojom::blink::NameFrom& name_from);
  String AriaTextAlternative(bool recursive,
                             const AXObject* aria_label_or_description_root,
                             AXObjectSet& visited,
                             ax::mojom::blink::NameFrom&,
                             AXRelatedObjectVector*,
                             NameSources*,
                             bool* found_text_alternative) const;
  String TextFromElements(bool in_aria_labelledby_traversal,
                          AXObjectSet& visited,
                          base::span<const Member<Element>> elements,
                          AXRelatedObjectVector* related_objects) const;
  static bool HasAriaLabelledbyElements(Element* from);

  // Return true if the ame is from @aria-label / @aria-labelledby.
  static bool IsNameFromAriaAttribute(Element* element);
  // Return true if the name is from @aria-label / @aria-labelledby / @title.
  bool IsNameFromAuthorAttribute() const;

  ax::mojom::blink::Role ButtonRoleType() const;

  bool CanSetSelectedAttribute() const;
  const AXObject* InertRoot() const;

  // Finds table, table row, and table cell parents and children
  // skipping over generic containers.
  AXObjectVector TableRowChildren() const;
  AXObjectVector TableCellChildren() const;

  // Helpers for serialization.
  void SerializeBoundingBoxAttributes(ui::AXNodeData& dst) const;
  void SerializeActionAttributes(ui::AXNodeData* node_data) const;
  void SerializeChildTreeID(ui::AXNodeData* node_data) const;
  void SerializeChooserPopupAttributes(ui::AXNodeData* node_data) const;
  void SerializeColorAttributes(ui::AXNodeData* node_data) const;
  void SerializeImplicitActions(ui::AXNodeData* node_data) const;
  void SerializeElementAttributes(ui::AXNodeData* node_data) const;
  void SerializeHTMLNonStandardAttributesForJAWS(
      ui::AXNodeData* node_data) const;
  void SerializeHTMLAttributesForSnapshot(ui::AXNodeData* node_data) const;
  void SerializeHTMLTagAndClass(ui::AXNodeData* node_data) const;
  void SerializeHTMLId(ui::AXNodeData* node_data) const;
  void SerializeInlineTextBox(ui::AXNodeData* node_data) const;
#if BUILDFLAG(IS_WIN)
  void SerializeJAWSNonStandardHTMLAttributes(ui::AXNodeData* node_data) const;
#endif
  void SerializeLangAttribute(ui::AXNodeData* node_data) const;
  void SerializeLineAttributes(ui::AXNodeData* node_data) const;
  void SerializeListAttributes(ui::AXNodeData* node_data) const;
  void SerializeListMarkerAttributes(ui::AXNodeData* dst) const;
  void SerializeLiveRegionAttributes(ui::AXNodeData* node_data) const;
  void SerializeNameAndDescriptionAttributes(ui::AXMode accessibility_mode,
                                             ui::AXNodeData* node_data) const;
  void SerializeScreenReaderAttributes(ui::AXNodeData* node_data) const;
  void SerializeOtherScreenReaderAttributes(ui::AXNodeData* node_data) const;
  void SerializeMathContent(ui::AXNodeData* node_data) const;
  void SerializeRelationAttributes(ui::AXNodeData* node_data) const;
  void SerializeScrollAttributes(ui::AXNodeData* node_data) const;
  void SerializeStyleAttributes(ui::AXNodeData* node_data) const;
  void SerializeTableAttributes(ui::AXNodeData* node_data) const;
  void SerializeUnignoredAttributes(ui::AXNodeData* node_data,
                                    ui::AXMode accessibility_mode,
                                    bool is_snapshot) const;
  void SerializeComputedDetailsRelation(ui::AXNodeData* node_data) const;

  // Serialization implemented in specific subclasses.
  virtual void SerializeMarkerAttributes(ui::AXNodeData* node_data) const;

  void SerializeImageDataAttributes(ui::AXNodeData* node_data) const;
  void SerializeTextInsertionDeletionOffsetAttributes(
      ui::AXNodeData* node_data) const;

  void SetCachedValuesNeedUpdate(
      bool cached_values_need_update,
      std::optional<TreeUpdateReason> reason = std::nullopt);
  void SetAXObjectCacheForTest(AXObjectCacheImpl& ax_object_cache) {
    ax_object_cache_ = &ax_object_cache;
  }

 private:
  bool ComputeCanSetFocusAttribute();
  String KeyboardShortcut() const;
  void UpdateStyleAndLayoutTreeForNode(Node& node);
  void OnInheritedCachedValuesChanged();
  static const AtomicString& GetInternalsAttribute(const Element&,
                                                   const QualifiedName&);

  // Returns true if this node should use the aria role combobox menu button.
  bool ShouldUseComboboxMenuButtonRole() const;

  bool children_dirty_ : 1 = false;

  // Do the rest of the cached_* member variables need to be recomputed?
  bool cached_values_need_update_ : 1 = true;
  // Do children need to recompute their cached values?
  bool child_cached_values_need_update_ : 1 = false;

  // The following cached attribute values (the ones starting with cached_**)
  // are only valid if cached_values_need_update_ is false.
  // Objects are marked ignored at construction time (and thus by default they
  // not included in the tree), so that if object becomes included in Init()
  // or in a future page update, the included node count will be incremented via
  // AXObjectCacheImpl::UpdateIncludedNodeCount().
  bool cached_is_ignored_ : 1 = true;
  bool cached_is_ignored_but_included_in_tree_ : 1 = false;
  bool cached_is_inert_ : 1 = false;
  bool cached_is_aria_hidden_ : 1 = false;
  bool cached_is_hidden_via_style_ : 1 = false;
  bool cached_is_used_for_label_or_description_ : 1;
  bool cached_is_descendant_of_disabled_node_ : 1 = false;
  bool cached_can_set_focus_attribute_ : 1 = false;
  bool cached_is_in_menu_list_subtree_ : 1 = false;
  std::optional<bool> cached_is_on_screen_;

  Member<AXObject> cached_live_region_root_;
  gfx::RectF cached_local_bounding_box_;

  Member<AXObjectCacheImpl> ax_object_cache_;

  bool IsCheckable() const;
  static bool IsNativeCheckboxInMixedState(const Node*);
  static bool IncludesARIAWidgetRole(const String&);
  static bool HasInteractiveARIAAttribute(const Element&);
  ax::mojom::blink::Role RemapAriaRoleDueToParent(ax::mojom::blink::Role) const;
  unsigned ComputeAriaColumnIndex() const;
  unsigned ComputeAriaRowIndex() const;
  const ComputedStyle* GetComputedStyle() const;
  bool ComputeIsHiddenViaStyle(const ComputedStyle*);
  bool ComputeIsUsedForLabelOrDescription();
  bool ComputeIsInertViaStyle(const ComputedStyle*,
                              IgnoredReasons* = nullptr) const;

  // Private action interfaces. Return bool if action is performed.
  bool RequestDecrementAction();
  bool RequestClickAction();
  bool RequestFocusAction();
  bool RequestIncrementAction();
  bool RequestScrollToGlobalPointAction(const gfx::Point&);
  bool RequestScrollToMakeVisibleAction();
  bool RequestSetSelectedAction(bool);
  bool RequestSetSequentialFocusNavigationStartingPointAction();
  bool RequestSetValueAction(const String&);
  bool RequestShowContextMenuAction();
  bool RequestExpandAction();
  bool RequestCollapseAction();

  // Returns an updated layout object to be used in a native scroll action. Note
  // that this updates style for `GetNode()` as well as layout for any layout
  // objects generated. Returns nullptr if a native scroll action to the node is
  // not possible.
  LayoutObject* GetLayoutObjectForNativeScrollAction() const;

  void DispatchKeyboardEvent(LocalDOMWindow* local_dom_window,
                             WebInputEvent::Type type,
                             ax::mojom::blink::Action action) const;

  // Return true if it's necessary to destroy a subtrees when detaching
  // from the parent.
  bool ShouldDestroyWhenDetachingFromParent() const;

  // Attaches the tree with the given ID to this object as a child tree and
  // updates the cache.
  void SetChildTree(const ui::AXTreeID& child_tree_id);

  static unsigned number_of_live_ax_objects_;

  FRIEND_TEST_ALL_PREFIXES(AccessibilityTest, GetParentNodeForComputeParent);
  FRIEND_TEST_ALL_PREFIXES(AccessibilityTest, NodesRequiringCacheUpdate);
};

MODULES_EXPORT bool operator==(const AXObject& first, const AXObject& second);
MODULES_EXPORT bool operator!=(const AXObject& first, const AXObject& second);
MODULES_EXPORT bool operator<(const AXObject& first, const AXObject& second);
MODULES_EXPORT bool operator<=(const AXObject& first, const AXObject& second);
MODULES_EXPORT bool operator>(const AXObject& first, const AXObject& second);
MODULES_EXPORT bool operator>=(const AXObject& first, const AXObject& second);
MODULES_EXPORT std::ostream& operator<<(std::ostream&, const AXObject&);
MODULES_EXPORT std::ostream& operator<<(std::ostream&, const AXObject*);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_ACCESSIBILITY_AX_OBJECT_H_
