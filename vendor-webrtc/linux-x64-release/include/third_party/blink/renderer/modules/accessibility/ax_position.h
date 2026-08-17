// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_ACCESSIBILITY_AX_POSITION_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_ACCESSIBILITY_AX_POSITION_H_

#include <stdint.h>

#include <ostream>

#include "base/dcheck_is_on.h"
#include "third_party/blink/renderer/core/editing/forward.h"
#include "third_party/blink/renderer/core/editing/text_affinity.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/heap/persistent.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class AXObject;
class ContainerNode;
class Document;
class Node;
class OffsetMapping;

// When converting to a DOM position from an |AXPosition| or vice versa, and the
// corresponding position is invalid, doesn't exist, or is inside an ignored
// object or a range of ignored objects, determines how to adjust the new
// position in order to make it valid.
enum class AXPositionAdjustmentBehavior { kMoveLeft, kMoveRight };

// Describes a position in the Blink accessibility tree.
// A position is either anchored to before or after a child object inside a
// container object, or is anchored to a character inside a text object.
// The former are called tree positions, and the latter text positions.
// Tree positions are never located on a specific |AXObject|. Rather, they are
// always between two objects, or an object and the start / end of their
// container's children, known as "before children" and "after children"
// positions respectively. They should be thought of like a caret that is always
// between two characters. Another way of calling these types of positions is
// object anchored and text anchored.
class MODULES_EXPORT AXPosition final {
  DISALLOW_NEW();

 public:
  //
  // Convert between DOM and AX positions and vice versa.
  // |Create...| and |FromPosition| methods will by default skip over any
  // ignored object and return the next unignored position to the right of that
  // object.
  //

  static const AXPosition CreatePositionBeforeObject(
      const AXObject& child,
      const AXPositionAdjustmentBehavior =
          AXPositionAdjustmentBehavior::kMoveRight);
  static const AXPosition CreatePositionAfterObject(
      const AXObject& child,
      const AXPositionAdjustmentBehavior =
          AXPositionAdjustmentBehavior::kMoveRight);
  static const AXPosition CreateFirstPositionInObject(
      const AXObject& container,
      const AXPositionAdjustmentBehavior =
          AXPositionAdjustmentBehavior::kMoveRight);
  static const AXPosition CreateLastPositionInObject(
      const AXObject& container,
      const AXPositionAdjustmentBehavior =
          AXPositionAdjustmentBehavior::kMoveRight);
  static const AXPosition CreatePositionInTextObject(
      const AXObject& container,
      const int offset,
      const TextAffinity = TextAffinity::kDownstream,
      const AXPositionAdjustmentBehavior =
          AXPositionAdjustmentBehavior::kMoveRight);
  static const AXPosition FromPosition(
      const Position&,
      const TextAffinity = TextAffinity::kDownstream,
      const AXPositionAdjustmentBehavior =
          AXPositionAdjustmentBehavior::kMoveRight);
  static const AXPosition FromPosition(
      const PositionWithAffinity&,
      const AXPositionAdjustmentBehavior =
          AXPositionAdjustmentBehavior::kMoveRight);

  // Creates an empty position. |IsValid| will return false.
  AXPosition();

  AXPosition(const AXPosition&) = default;
  AXPosition& operator=(const AXPosition&) = default;
  ~AXPosition() = default;

  // The |AXObject| in which the tree position is located, or in whose text the
  // text position is found.
  const AXObject* ContainerObject() const { return container_object_; }

  // Returns |nullptr| for text, and "after children" or equivalent positions.
  const AXObject* ChildAfterTreePosition() const;

  // Only valid for tree positions.
  int ChildIndex() const;

  // Only valid for text positions.
  int TextOffset() const;

  // If this is a text position, the length of the text in its container object.
  int MaxTextOffset() const;

  // When the same character offset could correspond to two possible caret
  // positions, upstream means it's on the previous line rather than the next
  // line.
  // Only valid for text positions.
  TextAffinity Affinity() const;

  // Verifies if the anchor is present and if it's set to a live object with a
  // connected node.
  bool IsValid(String* failure_reason = nullptr) const;

  operator bool() const { return IsValid(); }

  // Returns whether this is a position anchored to a character inside a text
  // object.
  bool IsTextPosition() const;

  const AXPosition CreateNextPosition() const;
  const AXPosition CreatePreviousPosition() const;

  // Returns an adjusted position by skipping over any ignored objects in the
  // case of a "before object" or "after object" position, or skipping over any
  // ignored children in the case of a "before children" or "after children"
  // position. If a text object is ignored, returns a position anchored at the
  // nearest object, which might not be a text object. If the container object
  // is ignored, tries to find if an equivalent position exists in its unignored
  // parent, since all the children of an ignored object in the accessibility
  // tree appear as children of its immediate unignored parent.
  const AXPosition AsUnignoredPosition(
      const AXPositionAdjustmentBehavior =
          AXPositionAdjustmentBehavior::kMoveRight) const;

  // Adjusts the position by skipping over any objects that don't have a
  // corresponding |node| in the DOM tree, e.g. list bullets.
  const AXPosition AsValidDOMPosition(
      const AXPositionAdjustmentBehavior =
          AXPositionAdjustmentBehavior::kMoveRight) const;

  // Converts to a DOM position.
  const PositionWithAffinity ToPositionWithAffinity(
      const AXPositionAdjustmentBehavior =
          AXPositionAdjustmentBehavior::kMoveLeft) const;
  const Position ToPosition(const AXPositionAdjustmentBehavior =
                                AXPositionAdjustmentBehavior::kMoveLeft) const;

  // Returns a string representation of this object.
  String ToString() const;

 private:
  // Only used by static Create... methods.
  explicit AXPosition(const AXObject& container);

  // Searches the DOM tree starting from a particular child node within a
  // particular container node, and in the direction indicated by the adjustment
  // behavior, until it finds a node whose corresponding AX object is not
  // ignored. Returns nullptr if an unignored object is not found within the
  // provided container node. The container node could be nullptr if the whole
  // DOM tree needs to be searched.
  static const AXObject* FindNeighboringUnignoredObject(
      const Document& document,
      const Node& child_node,
      const ContainerNode* container_node,
      const AXPositionAdjustmentBehavior adjustment_behavior);

  // Returns true if `character` is not included in the accessible text.
  // Ignored characters include zero-width space and isolate characters.
  static bool IsIgnoredCharacter(UChar character);

  // Returns the number of characters before `content_offset` that are ignored.
  // OffsetMappingUnits have offsets based on characters that we may exclude
  // from the text we expose to assistive technologies, such as:
  // * break opportunities inserted after preliminary whitespace in elements
  //   with `style= "whitespace: pre-wrap;"`
  // * isolate characters inserted in the content of SVG `text` and tspan`
  //   elements when `x` coordinates are specified.
  // Examples:
  // <div contenteditable="true" style="white-space: pre-wrap;">   Bar</div>
  // * Number of characters in the accessible text: 6 ("   Bar")
  // * Number of characters in the content: 7
  //
  // <text x="0 10 20 30 40 50 60 70 80 90 100 110"
  //       y="20">Hel<tspan>lo </tspan><tspan>world</tspan>!</text>
  // * Number of characters in the accessible text: 12 ("Hello world!")
  // * Number of characters in the content: 36
  //
  // The location of these ignored characters can be identified by checking
  // the OffsetMapping for non-contiguous units. For instance, in the case of
  // the SVG text, the "H" has a content range of 1-2, the "e" next to it a
  // content range of 4-5.
  //
  // Note that `<wbr>`, whose zero-width-space character is also ignored, does
  // have a mapping unit and corresponding node. As a result, its character
  // would not be included in the count returned here. Because it has a node,
  // we are already associating its offsets with the ignored accessible object.
  int GetLeadingIgnoredCharacterCount(const OffsetMapping* mapping,
                                      const Node* node,
                                      int container_offset,
                                      int content_offset) const;

  // The |AXObject| in which the position is present.
  // Only valid during a single document lifecycle hence no need to maintain a
  // strong reference to it.
  WeakPersistent<const AXObject> container_object_;

  // If the position is anchored to before or after an object, the number of
  // child objects in |container_object_| that come before the position.
  // If this is a text position, the number of characters in the canonical text
  // of |container_object_| before the position. The canonical text is the DOM
  // node's text but with, e.g., whitespace collapsed and any transformations
  // applied.
  int text_offset_or_child_index_;

  // When the same character offset could correspond to two possible caret
  // positions.
  TextAffinity affinity_;

#if DCHECK_IS_ON()
  // TODO(nektar): Use layout tree version in place of DOM and style versions.
  uint64_t dom_tree_version_;
  uint64_t style_version_;
#endif

  friend class AXSelection;
};

MODULES_EXPORT bool operator==(const AXPosition&, const AXPosition&);
MODULES_EXPORT bool operator!=(const AXPosition&, const AXPosition&);
MODULES_EXPORT bool operator<(const AXPosition&, const AXPosition&);
MODULES_EXPORT bool operator<=(const AXPosition&, const AXPosition&);
MODULES_EXPORT bool operator>(const AXPosition&, const AXPosition&);
MODULES_EXPORT bool operator>=(const AXPosition&, const AXPosition&);
MODULES_EXPORT std::ostream& operator<<(std::ostream&, const AXPosition&);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_ACCESSIBILITY_AX_POSITION_H_
