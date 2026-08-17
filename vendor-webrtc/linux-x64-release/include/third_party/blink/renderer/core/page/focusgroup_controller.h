// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_PAGE_FOCUSGROUP_CONTROLLER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_PAGE_FOCUSGROUP_CONTROLLER_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class Element;
class GridFocusgroupStructureInfo;
class KeyboardEvent;
class LocalFrame;

enum class FocusgroupDirection;

class CORE_EXPORT FocusgroupController {
  STATIC_ONLY(FocusgroupController);

 public:
  static bool HandleArrowKeyboardEvent(KeyboardEvent* event,
                                       const LocalFrame* frame);

 private:
  // Entry point into Focusgroup advancement. Returns true if the key press
  // moved the focus.
  static bool Advance(Element* initial_element, FocusgroupDirection direction);

  static bool AdvanceForward(Element* initial_element,
                             FocusgroupDirection direction);
  static bool CanExitFocusgroupForward(const Element* exiting_focusgroup,
                                       const Element* entering_focusgroup,
                                       FocusgroupDirection direction);
  static bool CanExitFocusgroupForwardRecursive(
      const Element* exiting_focusgroup,
      const Element* next_element,
      FocusgroupDirection direction,
      bool check_wrap);
  static Element* WrapForward(Element* nearest_focusgroup,
                              FocusgroupDirection direction);

  static bool AdvanceBackward(Element* initial_element,
                              FocusgroupDirection direction);
  static Element* WrapBackward(Element* nearest_focusgroup,
                               FocusgroupDirection direction);

  static bool AdvanceInGrid(Element* initial_element,
                            Element* grid_root,
                            FocusgroupDirection direction);
  static Element* WrapOrFlowInGrid(Element* element,
                                   FocusgroupDirection direction,
                                   GridFocusgroupStructureInfo* helper);

  static void Focus(Element* element, FocusgroupDirection direction);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_PAGE_FOCUSGROUP_CONTROLLER_H_