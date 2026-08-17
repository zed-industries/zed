// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_ITERATORS_TEXT_ITERATOR_BEHAVIOR_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_ITERATORS_TEXT_ITERATOR_BEHAVIOR_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class CORE_EXPORT TextIteratorBehavior final {
  DISALLOW_NEW();

 public:
  class CORE_EXPORT Builder;

  TextIteratorBehavior(const TextIteratorBehavior& other);
  TextIteratorBehavior();

  bool operator==(const TextIteratorBehavior& other) const;
  bool operator!=(const TextIteratorBehavior& other) const;

  bool DoesNotBreakAtReplacedElement() const {
    return values_.bits.does_not_break_at_replaced_element;
  }
  bool EmitsCharactersBetweenAllVisiblePositions() const {
    return values_.bits.emits_characters_between_all_visible_positions;
  }
  bool EmitsImageAltText() const { return values_.bits.emits_image_alt_text; }
  bool EmitsSpaceForNbsp() const { return values_.bits.emits_space_for_nbsp; }
  bool EmitsObjectReplacementCharacter() const {
    return values_.bits.emits_object_replacement_character;
  }
  bool EmitsOriginalText() const { return values_.bits.emits_original_text; }
  bool EntersOpenShadowRoots() const {
    return values_.bits.enters_open_shadow_roots;
  }
  bool EmitsSmallXForTextSecurity() const {
    return values_.bits.emits_small_x_for_text_security;
  }
  bool EntersTextControls() const { return values_.bits.enters_text_controls; }
  bool ExcludeAutofilledValue() const {
    return values_.bits.exclude_autofilled_value;
  }
  bool ForSelectionToString() const {
    return values_.bits.for_selection_to_string;
  }
  bool ForWindowFind() const { return values_.bits.for_window_find; }
  bool IgnoresStyleVisibility() const {
    return values_.bits.ignores_style_visibility;
  }
  bool StopsOnFormControls() const {
    return values_.bits.stops_on_form_controls;
  }
  bool DoesNotEmitSpaceBeyondRangeEnd() const {
    return values_.bits.does_not_emit_space_beyond_range_end;
  }

  bool SkipsUnselectableContent() const {
    return values_.bits.skips_unselectable_content;
  }

  bool SuppressesExtraNewlineEmission() const {
    return values_.bits.suppresses_newline_emission;
  }

  bool IgnoresDisplayLock() const { return values_.bits.ignores_display_lock; }

  bool EmitsPunctuationForReplacedElements() const {
    return values_.bits.emits_punctuation_for_replaced_elements;
  }

  bool IgnoresCSSTextTransforms() const {
    return values_.bits.ignores_css_text_transforms;
  }

  static TextIteratorBehavior EmitsObjectReplacementCharacterBehavior();
  static TextIteratorBehavior IgnoresStyleVisibilityBehavior();
  static TextIteratorBehavior DefaultRangeLengthBehavior();
  static TextIteratorBehavior AllVisiblePositionsRangeLengthBehavior();
  static TextIteratorBehavior
  AllVisiblePositionsIncludingShadowRootRangeLengthBehavior();
  static TextIteratorBehavior NoTrailingSpaceRangeLengthBehavior();

 private:
  union {
    unsigned all;
    struct {
      bool does_not_break_at_replaced_element : 1;
      bool emits_characters_between_all_visible_positions : 1;
      bool emits_image_alt_text : 1;
      bool emits_space_for_nbsp : 1;
      bool emits_object_replacement_character : 1;
      bool emits_original_text : 1;
      bool emits_small_x_for_text_security : 1;
      bool enters_open_shadow_roots : 1;
      bool enters_text_controls : 1;
      bool exclude_autofilled_value : 1;
      bool for_selection_to_string : 1;
      bool for_window_find : 1;
      bool ignores_style_visibility : 1;
      bool stops_on_form_controls : 1;
      bool does_not_emit_space_beyond_range_end : 1;
      bool skips_unselectable_content : 1;
      bool suppresses_newline_emission : 1;
      bool ignores_display_lock : 1;
      bool emits_punctuation_for_replaced_elements : 1;
      bool ignores_css_text_transforms : 1;
    } bits;
  } values_;
};

class CORE_EXPORT TextIteratorBehavior::Builder final {
  STACK_ALLOCATED();

 public:
  explicit Builder(const TextIteratorBehavior&);
  Builder();
  Builder(const Builder&) = delete;
  Builder& operator=(const Builder&) = delete;
  ~Builder();

  TextIteratorBehavior Build();

  Builder& SetDoesNotBreakAtReplacedElement(bool);
  Builder& SetEmitsCharactersBetweenAllVisiblePositions(bool);
  Builder& SetEmitsImageAltText(bool);
  Builder& SetEmitsSpaceForNbsp(bool);
  Builder& SetEmitsObjectReplacementCharacter(bool);
  Builder& SetEmitsOriginalText(bool);
  Builder& SetEmitsSmallXForTextSecurity(bool);
  Builder& SetEntersOpenShadowRoots(bool);
  Builder& SetEntersTextControls(bool);
  Builder& SetExcludeAutofilledValue(bool);
  Builder& SetForSelectionToString(bool);
  Builder& SetForWindowFind(bool);
  Builder& SetIgnoresStyleVisibility(bool);
  Builder& SetStopsOnFormControls(bool);
  Builder& SetDoesNotEmitSpaceBeyondRangeEnd(bool);
  Builder& SetSkipsUnselectableContent(bool);
  Builder& SetSuppressesExtraNewlineEmission(bool);
  Builder& SetIgnoresDisplayLock(bool);
  Builder& SetEmitsPunctuationForReplacedElements(bool);
  Builder& SetIgnoresCSSTextTransforms(bool);

 private:
  TextIteratorBehavior behavior_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_ITERATORS_TEXT_ITERATOR_BEHAVIOR_H_
