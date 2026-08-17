// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_PAGE_FOCUSGROUP_CONTROLLER_UTILS_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_PAGE_FOCUSGROUP_CONTROLLER_UTILS_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/focusgroup_flags.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class Element;
class GridFocusgroupStructureInfo;
class KeyboardEvent;

enum class FocusgroupDirection {
  kNone,
  kBackwardInline,
  kBackwardBlock,
  kForwardInline,
  kForwardBlock,
};

enum class FocusgroupType {
  kGrid,
  kLinear,
};

class CORE_EXPORT FocusgroupControllerUtils {
  STATIC_ONLY(FocusgroupControllerUtils);

 public:
  static FocusgroupDirection FocusgroupDirectionForEvent(KeyboardEvent* event);
  static bool IsDirectionBackward(FocusgroupDirection direction);
  static bool IsDirectionForward(FocusgroupDirection direction);
  static bool IsDirectionInline(FocusgroupDirection direction);
  static bool IsDirectionBlock(FocusgroupDirection direction);
  static bool IsAxisSupported(FocusgroupFlags flags,
                              FocusgroupDirection direction);
  static bool WrapsInDirection(FocusgroupFlags flags,
                               FocusgroupDirection direction);
  static bool FocusgroupExtendsInAxis(FocusgroupFlags extending_focusgroup,
                                      FocusgroupFlags focusgroup,
                                      FocusgroupDirection direction);

  static Element* FindNearestFocusgroupAncestor(const Element* element,
                                                FocusgroupType type);
  static Element* NextElement(const Element* current, bool skip_subtree);
  static Element* PreviousElement(const Element* current);
  static Element* LastElementWithin(const Element* current);

  static Element* AdjustElementOutOfUnrelatedFocusgroup(
      Element* element,
      Element* stop_ancestor,
      FocusgroupDirection direction);

  static bool IsFocusgroupItem(const Element* element);
  static bool IsGridFocusgroupItem(const Element* element);

  static GridFocusgroupStructureInfo*
  CreateGridFocusgroupStructureInfoForGridRoot(Element* root);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_PAGE_FOCUSGROUP_CONTROLLER_UTILS_H_
