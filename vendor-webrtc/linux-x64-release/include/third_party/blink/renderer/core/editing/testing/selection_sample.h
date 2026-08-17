// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_TESTING_SELECTION_SAMPLE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_TESTING_SELECTION_SAMPLE_H_

#include <string>

#include "third_party/blink/renderer/core/editing/forward.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"

namespace blink {

class ContainerNode;
class HTMLElement;

// |SelectionSample| provides parsing HTML text with selection markers and
// serializes DOM tree with selection markers.
// Selection markers are represents by "^" for selection base and "|" for
// selection extent like "assert_selection.js" in web test.
//
// To set selection at before children or after children instead of start or
// end of |Text| node, we should use selection marker only |Comment| node like:
//  <span><!--^-->foo<!--|--></span>
// This notation yields selection of base=SPAN@0 and extent=SPAN@1.
class SelectionSample final {
  STATIC_ONLY(SelectionSample);

 public:
  // TDOO(editng-dev): We will have flat tree version of |SetSelectionText()|
  // and |GetSelectionText()| when we need.
  // Set |selection_text|, which is HTML markup with selection markers as inner
  // HTML to |HTMLElement| and returns |SelectionInDOMTree|.
  static SelectionInDOMTree SetSelectionText(HTMLElement*,
                                             const std::string& selection_text);

  // Note: We don't add namespace declaration if |ContainerNode| doesn't
  // have it.
  // Note: We don't escape "--" in comment.
  static std::string GetSelectionText(const ContainerNode&,
                                      const SelectionInDOMTree&);
  static std::string GetSelectionTextInFlatTree(const ContainerNode&,
                                                const SelectionInFlatTree&);
  static void ConvertTemplatesToShadowRootsForTesring(HTMLElement&);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_EDITING_TESTING_SELECTION_SAMPLE_H_
