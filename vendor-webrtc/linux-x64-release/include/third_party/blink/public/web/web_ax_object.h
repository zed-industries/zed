/*
 * Copyright (C) 2009 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 *     * Redistributions of source code must retain the above copyright
 * notice, this list of conditions and the following disclaimer.
 *     * Redistributions in binary form must reproduce the above
 * copyright notice, this list of conditions and the following disclaimer
 * in the documentation and/or other materials provided with the
 * distribution.
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

#ifndef THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_AX_OBJECT_H_
#define THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_AX_OBJECT_H_

#include <vector>

#include "third_party/blink/public/platform/web_common.h"
#include "third_party/blink/public/platform/web_private_ptr.h"
#include "third_party/blink/public/web/web_ax_enums.h"
#include "ui/accessibility/ax_enums.mojom-shared.h"
#include "ui/accessibility/ax_event_intent.h"
#include "ui/accessibility/ax_mode.h"
#include "ui/accessibility/ax_node_id_forward.h"
#include "ui/accessibility/ax_tree_data.h"
#include "ui/accessibility/ax_tree_id.h"
#include "ui/accessibility/ax_tree_source.h"

namespace gfx {
class Point;
class RectF;
class Rect;
class Size;
class Transform;
}

namespace ui {
struct AXActionData;
class AXNode;
struct AXNodeData;
}

namespace blink {

class AXObject;
class WebAXObject;
class WebNode;
class WebDocument;
class WebString;
class WebURL;

// A container for passing around a reference to AXObject.
class BLINK_EXPORT WebAXObject {
 public:
  ~WebAXObject() { Reset(); }

  WebAXObject() = default;
  WebAXObject(const WebAXObject& o) { Assign(o); }
  WebAXObject& operator=(const WebAXObject& o) {
    Assign(o);
    return *this;
  }

  bool operator==(const WebAXObject& other) const;
  bool operator!=(const WebAXObject& other) const;
  bool operator<(const WebAXObject& other) const;
  bool operator<=(const WebAXObject& other) const;
  bool operator>(const WebAXObject& other) const;
  bool operator>=(const WebAXObject& other) const;
  static WebAXObject FromWebNode(const WebNode&);
  static WebAXObject FromWebDocument(const WebDocument&);
  static WebAXObject FromWebDocumentByID(const WebDocument&, int);
  static WebAXObject FromWebDocumentFirstWithRole(const WebDocument&,
                                                  ax::mojom::Role role);
  static WebAXObject FromWebDocumentFocused(const WebDocument&);
  static bool IsDirty(const WebDocument&);

  void Reset();
  void Assign(const WebAXObject&);
  bool Equals(const WebAXObject&) const;

  bool IsNull() const { return private_.IsNull(); }
  // isDetached also checks for null, so it's safe to just call isDetached.
  bool IsDetached() const;

  int AxID() const;

  unsigned ChildCount() const;

  WebAXObject ChildAt(unsigned) const;
  WebAXObject ParentObject() const;

  // Serialize the properties of this node into |node_data|.
  void Serialize(ui::AXNodeData* node_data,
                 ui::AXMode accessibility_mode) const;

  void OnLoadInlineTextBoxes() const;
  void SetImageAsDataNodeId(const gfx::Size& max_size) const;
  int ImageDataNodeId() const;

  ax::mojom::CheckedState CheckedState() const;
  bool IsClickable() const;
  bool IsFocused() const;
  bool IsModal() const;

  bool IsVisited() const;

  bool CanSetValueAttribute() const;
  unsigned ColorValue() const;
  WebAXObject AriaActiveDescendant() const;
  WebString AutoComplete() const;
  ax::mojom::AriaCurrentState AriaCurrentState() const;
  bool IsEditable() const;
  bool AriaOwns(std::vector<WebAXObject>& owns_elements) const;
  bool CanvasHasFallbackContent() const;
  ax::mojom::InvalidState InvalidState() const;
  int HeadingLevel() const;
  int HierarchicalLevel() const;
  WebAXObject HitTest(const gfx::Point&) const;
  // Get the WebAXObject's bounds in frame-relative coordinates as a gfx::Rect.
  gfx::Rect GetBoundsInFrameCoordinates() const;
  WebString Language() const;
  WebAXObject InPageLinkTarget() const;
  ax::mojom::Role Role() const;
  WebString GetValueForControl() const;
  ax::mojom::WritingDirection GetTextDirection() const;
  WebURL Url() const;

  // Retrieves the accessible name of the object, an enum indicating where the
  // name was derived from, and a list of related objects that were used to
  // derive the name, if any.
  WebString GetName(ax::mojom::NameFrom&,
                    std::vector<WebAXObject>& name_objects) const;
  // Simplified version of |name| when nameFrom and nameObjects aren't needed.
  WebString GetName() const;
  // Takes the result of nameFrom from calling |name|, above, and retrieves the
  // accessible description of the object, which is secondary to |name|, an enum
  // indicating where the description was derived from, and a list of objects
  // that were used to derive the description, if any.
  WebString Description(ax::mojom::NameFrom,
                        ax::mojom::DescriptionFrom&,
                        std::vector<WebAXObject>& description_objects) const;
  // Takes the result of nameFrom and descriptionFrom from calling |name| and
  // |description|, above, and retrieves the placeholder of the object, if
  // present and if it wasn't already exposed by one of the two functions above.
  WebString Placeholder(ax::mojom::NameFrom) const;

  //
  // Document-level interfaces.
  //
  // These are intended to be called on the root WebAXObject.
  //

  bool IsLoaded() const;

  // The following selection functions get or set the global document
  // selection and can be called on any object in the tree.

  void Selection(bool& is_selection_backward,
                 WebAXObject& anchor_object,
                 int& anchor_offset,
                 ax::mojom::TextAffinity& anchor_affinity,
                 WebAXObject& focus_object,
                 int& focus_offset,
                 ax::mojom::TextAffinity& focus_affinity) const;

  // Live regions.
  bool LiveRegionAtomic() const;
  WebString LiveRegionRelevant() const;
  WebString LiveRegionStatus() const;

  bool SupportsRangeValue() const;
  bool ValueForRange(float* out_value) const;
  bool MaxValueForRange(float* out_value) const;
  bool MinValueForRange(float* out_value) const;
  bool StepValueForRange(float* out_value) const;

  WebNode GetNode() const;
  WebDocument GetDocument() const;
  bool IsIgnored() const;
  bool IsIncludedInTree() const;

  // Get the verb associated with performing the default action
  // on this object.
  ax::mojom::DefaultActionVerb Action() const;

  // Perform an action, return true if handled.
  //
  // NEW: we're migrating to have all actions handled via this interface.
  bool PerformAction(const ui::AXActionData&) const;

  // Actions. Return true if handled.
  //
  // OLD: the od way is that we had separate APIs for every individual
  // action. We're migrating to use PerformAction() for everything.
  // To clear the selection, use an anchor_offset of
  // kNoSelectionOffset.
  bool SetSelection(const WebAXObject& anchor_object,
                    int anchor_offset,
                    const WebAXObject& focus_object,
                    int focus_offset) const;
  // Make this object visible by scrolling as many nested scrollable views as
  // needed.
  bool ScrollToMakeVisible() const;
  // Same, but if the whole object can't be made visible, try for this subrect,
  // in local coordinates. We also allow passing horizontal and vertical scroll
  // alignments. These specify where in the content area to scroll the object.
  bool ScrollToMakeVisibleWithSubFocus(
      const gfx::Rect&,
      ax::mojom::ScrollAlignment horizontal_scroll_alignment =
          ax::mojom::ScrollAlignment::kScrollAlignmentCenter,
      ax::mojom::ScrollAlignment vertical_scroll_alignment =
          ax::mojom::ScrollAlignment::kScrollAlignmentCenter,
      ax::mojom::ScrollBehavior scroll_behavior =
          ax::mojom::ScrollBehavior::kDoNotScrollIfVisible) const;

  // For a table
  unsigned ColumnCount() const;
  unsigned RowCount() const;
  WebAXObject CellForColumnAndRow(unsigned column, unsigned row) const;
  void RowHeaders(std::vector<WebAXObject>&) const;
  void ColumnHeaders(std::vector<WebAXObject>&) const;

  // For a table cell
  unsigned CellColumnIndex() const;
  unsigned CellColumnSpan() const;
  unsigned CellRowIndex() const;
  unsigned CellRowSpan() const;
  ax::mojom::SortDirection SortDirection() const;

  // Walk the WebAXObjects on the same line. This is supported on any
  // object type but primarily intended to be used for inline text boxes.
  WebAXObject NextOnLine() const;
  WebAXObject PreviousOnLine() const;

  // For an inline text box.
  void CharacterOffsets(std::vector<int>&) const;
  void GetWordBoundaries(std::vector<int>& starts,
                         std::vector<int>& ends) const;

  // Scrollable containers.
  // Also scrollable by user.
  gfx::Point GetScrollOffset() const;
  gfx::Point MinimumScrollOffset() const;
  gfx::Point MaximumScrollOffset() const;
  void SetScrollOffset(const gfx::Point&) const;

  // Every object's bounding box is returned relative to a
  // container object (which is guaranteed to be an ancestor) and
  // optionally a transformation matrix that needs to be applied too.
  // To compute the absolute bounding box of an element, start with its
  // boundsInContainer and apply the transform. Then as long as its container is
  // not null, walk up to its container and offset by the container's offset
  // from origin, the container's scroll position if any, and apply the
  // container's transform.  Do this until you reach the root of the tree.
  // If the container clips its children, for example with overflow:hidden
  // or similar, set |clips_children| to true.
  void GetRelativeBounds(WebAXObject& offset_container,
                         gfx::RectF& bounds_in_container,
                         gfx::Transform& container_transform,
                         bool* clips_children = nullptr) const;

  // Marks this object as dirty (needing serialization).
  void AddDirtyObjectToSerializationQueue(
      ax::mojom::EventFrom event_from,
      ax::mojom::Action event_from_action,
      std::vector<ui::AXEventIntent> event_intents) const;

  // Returns a brief description of the object, suitable for debugging. E.g. its
  // role and name.
  WebString ToString(bool verbose = true) const;

  void HandleAutofillSuggestionAvailabilityChanged(
      WebAXAutofillSuggestionAvailability suggestion_availability) const;

  // Methods for plugins to stitch a tree into this node.

  // Get a new AXID that's not used by any accessibility node in this process,
  // for when the client needs to insert additional nodes into the accessibility
  // tree.
  int GenerateAXID();
  void SetPluginTreeSource(
      ui::AXTreeSource<const ui::AXNode*, ui::AXTreeData*, ui::AXNodeData>*
          source);
  void MarkPluginDescendantDirty(ui::AXNodeID node_id);

#if INSIDE_BLINK
  WebAXObject(AXObject*);
  WebAXObject& operator=(AXObject*);
  operator AXObject*() const;
#endif

 private:
  WebPrivatePtrForGC<AXObject> private_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_AX_OBJECT_H_
