// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_ACCESSIBILITY_AX_SELECTION_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_ACCESSIBILITY_AX_SELECTION_H_

#include <stdint.h>

#include <optional>
#include <ostream>

#include "base/dcheck_is_on.h"
#include "third_party/blink/renderer/core/dom/document.h"
#include "third_party/blink/renderer/core/editing/forward.h"
#include "third_party/blink/renderer/core/html/forms/text_control_element.h"
#include "third_party/blink/renderer/modules/accessibility/ax_position.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

// If the |AXSelection| is defined by endpoints that are present in the
// accessibility tree but not in the DOM tree, determines whether setting the
// selection will shrink or extend the |AXSelection| to encompass endpoints that
// are in the DOM.
// Conversely, if a DOM selection is converted to an |AXSelection| via the
// |FromSelection| method, but the endpoints of the DOM selection are not
// present in the accessibility tree, e.g. they are aria-hidden, determines
// whether the conversion will shrink or extend the DOM selection to encompass
// endpoints that are in the accessibility tree.
enum class AXSelectionBehavior { kShrinkToValidRange, kExtendToValidRange };

class MODULES_EXPORT AXSelection final {
  DISALLOW_NEW();

 public:
  class Builder;

  static void ClearCurrentSelection(Document&);

  static AXSelection FromCurrentSelection(
      const Document&,
      const AXSelectionBehavior = AXSelectionBehavior::kExtendToValidRange);

  static AXSelection FromCurrentSelection(const TextControlElement&);

  static AXSelection FromSelection(
      const SelectionInDOMTree&,
      const AXSelectionBehavior = AXSelectionBehavior::kExtendToValidRange);

  AXSelection(const AXSelection&) = default;
  AXSelection& operator=(const AXSelection&) = default;
  ~AXSelection() = default;

  const AXPosition Anchor() const { return anchor_; }
  const AXPosition Focus() const { return focus_; }

  // The selection is invalid if either the anchor or the focus position is
  // invalid, or if the positions are in two separate documents.
  bool IsValid() const;

  operator bool() const { return IsValid(); }

  const SelectionInDOMTree AsSelection(
      const AXSelectionBehavior =
          AXSelectionBehavior::kExtendToValidRange) const;

  // Tries to set the DOM selection to this. Returns |false| if the selection
  // has been cancelled via the "selectionstart" event or if the selection could
  // not be set for any other reason.
  bool Select(
      const AXSelectionBehavior = AXSelectionBehavior::kExtendToValidRange);

  // Returns a string representation of this object.
  String ToString() const;

 private:
  // Holds the endpoints of a selection that affects the value of a text
  // control, i.e. that is inside its shadow tree.
  struct TextControlSelection final {
    TextControlSelection(int start,
                         int end,
                         TextFieldSelectionDirection direction)
        : start(start), end(end), direction(direction) {}
    TextControlSelection()
        : start(-1), end(-1), direction(kSelectionHasNoDirection) {}

    int start;
    int end;
    TextFieldSelectionDirection direction;
  };

  AXSelection();

  // If a layout is pending, update the style and layout along with the DOM
  // tree and style versions of the AXSelection and associated AXPositions.
  void UpdateSelectionIfNecessary();

  // Determines whether this selection is targeted to the contents of a text
  // field, and returns the start and end text offsets, as well as its
  // direction. |start| should always be less than equal to |end|.
  std::optional<TextControlSelection> AsTextControlSelection() const;

  // The |AXPosition| where the selection starts.
  AXPosition anchor_;

  // The |AXPosition| where the selection ends.
  AXPosition focus_;

#if DCHECK_IS_ON()
  // TODO(accessibility): Use layout tree version in place of DOM and style
  // versions.
  uint64_t dom_tree_version_;
  uint64_t style_version_;
#endif

  friend class Builder;
};

class MODULES_EXPORT AXSelection::Builder final {
  STACK_ALLOCATED();

 public:
  Builder() = default;
  ~Builder() = default;
  Builder& SetAnchor(const AXPosition&);
  Builder& SetAnchor(const Position&);
  Builder& SetFocus(const AXPosition&);
  Builder& SetFocus(const Position&);
  Builder& SetSelection(const SelectionInDOMTree&);
  const AXSelection Build();

 private:
  AXSelection selection_;
};

MODULES_EXPORT bool operator==(const AXSelection&, const AXSelection&);
MODULES_EXPORT bool operator!=(const AXSelection&, const AXSelection&);
MODULES_EXPORT std::ostream& operator<<(std::ostream&, const AXSelection&);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_ACCESSIBILITY_AX_SELECTION_H_
