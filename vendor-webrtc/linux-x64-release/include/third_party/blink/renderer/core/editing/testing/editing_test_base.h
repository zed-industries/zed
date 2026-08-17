// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_TESTING_EDITING_TEST_BASE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_TESTING_EDITING_TEST_BASE_H_

#include <gtest/gtest.h>
#include <memory>
#include <string>
#include "third_party/blink/renderer/core/editing/forward.h"
#include "third_party/blink/renderer/core/html/html_element.h"
#include "third_party/blink/renderer/core/testing/page_test_base.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"

namespace blink {

class FrameSelection;
class HTMLElement;

class EditingTestBase : public PageTestBase {
  USING_FAST_MALLOC(EditingTestBase);

 public:
  static ShadowRoot* CreateShadowRootForElementWithIDAndSetInnerHTML(
      TreeScope&,
      const char* host_element_id,
      const char* shadow_root_content);

 protected:
  EditingTestBase();
  ~EditingTestBase() override;

  // Returns |Position| for specified |caret_text|, which is HTML markup with
  // caret marker "|".
  Position SetCaretTextToBody(const std::string& caret_text);

  // Returns |SelectionInDOMTree| for specified |selection_text| by using
  // |SetSelectionText()| on BODY.
  SelectionInDOMTree SetSelectionTextToBody(const std::string& selection_text);

  // Sets |HTMLElement#innerHTML| with |selection_text|, which is HTML markup
  // with selection markers "^" and "|" and returns |SelectionInDOMTree| of
  // specified selection markers.
  // See also |GetSelectionText()| which returns selection text from specified
  // |ContainerNode| and |SelectionInDOMTree|.
  // Note: Unlike |assert_selection()|, this function doesn't change
  // |FrameSelection|.
  SelectionInDOMTree SetSelectionText(HTMLElement*,
                                      const std::string& selection_text);

  // Returns selection text for child nodes of BODY with specific |Position|.
  std::string GetCaretTextFromBody(const Position&) const;

  // Returns selection text for child nodes of BODY with specified
  // |SelectionInDOMTree|.
  std::string GetSelectionTextFromBody(const SelectionInDOMTree&) const;

  std::string GetSelectionTextFromBody() const;

  // Returns selection text for child nodes of BODY with specified
  // |SelectionInFlatTree|.
  std::string GetSelectionTextInFlatTreeFromBody(
      const SelectionInFlatTree&) const;

  ShadowRoot* SetShadowContent(const char* shadow_content,
                               const char* shadow_host_id);

};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_TESTING_EDITING_TEST_BASE_H_
