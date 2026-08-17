// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LCP_CRITICAL_PATH_PREDICTOR_ELEMENT_LOCATOR_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LCP_CRITICAL_PATH_PREDICTOR_ELEMENT_LOCATOR_H_

#include <optional>

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/lcp_critical_path_predictor/element_locator.pb.h"
#include "third_party/blink/renderer/platform/wtf/hash_map.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string_hash.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class Element;
class HTMLToken;

namespace element_locator {

// Attempt to generate an `ElementLocator` that specifies the relative position
// of the `element` within its document.
CORE_EXPORT ElementLocator OfElement(const Element& element);

// Generate a string representation of the given `ElementLocator`.
// Intended for testing and debugging purposes.
// Note: Since we are using the MessageLite runtime, TextFormat is not
//       available, so we need something on our own.
CORE_EXPORT String ToStringForTesting(const ElementLocator&);

// An item of `stack of open elements`
// https://html.spec.whatwg.org/multipage/parsing.html#stack-of-open-elements
struct HTMLStackItem {
  // The container element's tag name.
  // Note that we only track element's local name, which means no support for
  // the XML namespaces.
  const StringImpl* tag_name;

  // The container element's id attribute value.
  AtomicString id_attr;

  // Number of children elements by its `tag_name`.
  HashMap<const StringImpl*, int> children_counts;

  void IncrementChildrenCount(const StringImpl* tag_name);
};

class CORE_EXPORT TokenStreamMatcher {
 public:
  explicit TokenStreamMatcher(Vector<ElementLocator>);
  ~TokenStreamMatcher();

  static void InitSets();

  // Observe a start tag token and returns `true` iff any of the `locators_`
  // match.
  bool ObserveStartTagAndReportMatch(const StringImpl* tag_name,
                                     const HTMLToken& token);

  // Observe a end tag token.
  void ObserveEndTag(const StringImpl* tag_name);

 private:
#ifndef NDEBUG
  // Dump `html_stack_` for debugging purposes.
  void DumpHTMLStack();
#endif

  const Vector<ElementLocator> locators_;

  // https://html.spec.whatwg.org/multipage/parsing.html#stack-of-open-elements
  Vector<HTMLStackItem> html_stack_{
      HTMLStackItem()};  // populate root node stack item.
};

}  // namespace element_locator

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LCP_CRITICAL_PATH_PREDICTOR_ELEMENT_LOCATOR_H_
