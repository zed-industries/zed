// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_ACCESSIBILITY_TESTING_ACCESSIBILITY_SELECTION_TEST_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_ACCESSIBILITY_TESTING_ACCESSIBILITY_SELECTION_TEST_H_

#include <string>

#include "third_party/blink/renderer/modules/accessibility/testing/accessibility_test.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class AXObject;
class AXSelection;
class LocalFrameClient;

// Makes writing and debugging selection tests easier.
class AccessibilitySelectionTest : public AccessibilityTest {
  USING_FAST_MALLOC(AccessibilitySelectionTest);

 public:
  AccessibilitySelectionTest(LocalFrameClient* local_frame_client = nullptr);

 protected:
  void SetUp() override;

  // Gets a text representation of the accessibility tree that is currently
  // selected and annotates it with markers indicating the anchor and focus of
  // |selection|.
  std::string GetCurrentSelectionText() const;

  // Gets a text representation of the accessibility tree encompassing
  // |selection| and annotates it with markers indicating the anchor and focus
  // of |selection|.
  std::string GetSelectionText(const AXSelection& selection) const;

  // Gets a text representation of the accessibility subtree rooted at |subtree|
  // and encompassing |selection|, and annotates it with markers indicating the
  // anchor and focus of |selection|.
  std::string GetSelectionText(const AXSelection& selection,
                               const AXObject& subtree) const;

  // Sets |selection_text| as inner HTML of the document body and returns the
  // resulting |AXSelection|. If there are multiple selection markers, returns
  // only the first selection.
  AXSelection SetSelectionText(const std::string& selection_text) const;

  // Sets |selection_text| as inner HTML of |element| and returns the resulting
  // |AXSelection|. If there are multiple selection markers, returns only the
  // first selection.
  AXSelection SetSelectionText(const std::string& selection_text,
                               HTMLElement& element) const;

  // Compares two HTML files containing a DOM selection and the equivalent
  // accessibility selection.
  void RunSelectionTest(const std::string& test_name,
                        const std::string& suffix = std::string()) const;

 private:
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_ACCESSIBILITY_TESTING_ACCESSIBILITY_SELECTION_TEST_H_
