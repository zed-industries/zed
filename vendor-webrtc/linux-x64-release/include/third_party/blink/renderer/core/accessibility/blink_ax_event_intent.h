// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_ACCESSIBILITY_BLINK_AX_EVENT_INTENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_ACCESSIBILITY_BLINK_AX_EVENT_INTENT_H_

#include <string>

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/editing/commands/edit_command.h"
#include "third_party/blink/renderer/core/editing/selection_modifier.h"
#include "third_party/blink/renderer/core/editing/set_selection_options.h"
#include "third_party/blink/renderer/core/editing/text_granularity.h"
#include "third_party/blink/renderer/core/editing/visible_units.h"
#include "third_party/blink/renderer/platform/text/text_direction.h"
#include "third_party/blink/renderer/platform/wtf/hash_table_deleted_value_type.h"
#include "third_party/blink/renderer/platform/wtf/hash_traits.h"
#include "ui/accessibility/ax_enums.mojom-blink-forward.h"
#include "ui/accessibility/ax_event.h"
#include "ui/accessibility/ax_event_intent.h"

namespace blink {

// Adapts a ui::AXEventIntent for use in a Blink HashCountedSet.
//
// An event intent describes what caused an accessibility event to be raised.
// For example, a character could have been typed, a word replaced, or a line
// deleted. Or, the selection could have been extended to the beginning of the
// previous word, or it could have been moved to the end of the next line.
class CORE_EXPORT BlinkAXEventIntent final {
 public:
  static BlinkAXEventIntent FromEditCommand(const EditCommand& edit_command);
  static BlinkAXEventIntent FromClearedSelection(
      const SetSelectionBy set_selection_by);
  static BlinkAXEventIntent FromModifiedSelection(
      const SelectionModifyAlteration alter,
      const SelectionModifyDirection direction,
      const TextGranularity granularity,
      const SetSelectionBy set_selection_by,
      const TextDirection direction_of_selection,
      const PlatformWordBehavior platform_word_behavior);
  static BlinkAXEventIntent FromNewSelection(
      const TextGranularity granularity,
      bool is_base_first,
      const SetSelectionBy set_selection_by);

  // Creates an empty (uninitialized) instance.
  BlinkAXEventIntent();

  // Constructs an event intent which contains only a command without any other
  // arguments. This is used e.g. by the selection changed event when the
  // current selection is cleared.
  explicit BlinkAXEventIntent(ax::mojom::blink::Command command);

  // Constructs an editing event intent; which is primarily attached to a text
  // changed or a text attributes changed event.
  BlinkAXEventIntent(ax::mojom::blink::Command command,
                     ax::mojom::blink::InputEventType input_event_type);

  // Constructs a selection event intent; which is attached to a selection
  // changed event.
  BlinkAXEventIntent(ax::mojom::blink::Command command,
                     ax::mojom::blink::TextBoundary text_boundary,
                     ax::mojom::blink::MoveDirection move_direction);

  // Used by HashCountedSet to create a deleted BlinkAXEventIntent instance.
  explicit BlinkAXEventIntent(WTF::HashTableDeletedValueType type);

  ~BlinkAXEventIntent();

  BlinkAXEventIntent(const BlinkAXEventIntent& intent);
  BlinkAXEventIntent& operator=(const BlinkAXEventIntent& intent);

  CORE_EXPORT friend bool operator==(const BlinkAXEventIntent& a,
                                     const BlinkAXEventIntent& b);
  CORE_EXPORT friend bool operator!=(const BlinkAXEventIntent& a,
                                     const BlinkAXEventIntent& b);

  const ui::AXEventIntent& intent() const { return intent_; }
  ui::AXEventIntent& intent() { return intent_; }

  // Returns "true" if this represents an initialized BlinkAXEventIntent
  // instance.
  bool is_initialized() const { return is_initialized_; }

  // Returns "true" if this represents a deleted BlinkAXEventIntent instance.
  bool IsHashTableDeletedValue() const;

  // Returns a string representation of this instance.
  std::string ToString() const;

 private:
  ui::AXEventIntent intent_;

  // Set to "true" if this represents an initialized BlinkAXEventIntent
  // instance. An empty (uninitialized) instance is created either by calling
  // the default constructor or by simply zeroing out a block of memory
  // equivalent to the size of this class. The latter may be done by the HashSet
  // for performance reasons.
  //
  // This member is needed so that our hash function will never return the same
  // value for an uninitialized and an initialized instance. Otherwise an
  // uninitialized instance's memory may have random values.
  bool is_initialized_ = false;

  // Set to "true" if this represents a deleted BlinkAXEventIntent instance.
  bool is_deleted_ = false;
};

struct CORE_EXPORT BlinkAXEventIntentHashTraits
    : WTF::SimpleClassHashTraits<BlinkAXEventIntent> {
  // Computes the hash of a BlinkAXEventIntent instance.
  static unsigned GetHash(const BlinkAXEventIntent& key);
  // Zeroed memory cannot be used for BlinkAXEventIntent.
  static constexpr bool kEmptyValueIsZero = false;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_ACCESSIBILITY_BLINK_AX_EVENT_INTENT_H_
