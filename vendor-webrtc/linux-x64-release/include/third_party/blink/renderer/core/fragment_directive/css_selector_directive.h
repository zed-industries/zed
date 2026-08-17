// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FRAGMENT_DIRECTIVE_CSS_SELECTOR_DIRECTIVE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FRAGMENT_DIRECTIVE_CSS_SELECTOR_DIRECTIVE_H_

#include "third_party/blink/renderer/core/frame/directive.h"

#include "third_party/blink/renderer/platform/weborigin/kurl.h"

namespace blink {

// Parses directive string and extracts value part of CssSelectorDirective
// to be used with QuerySelector() for finding the element
// https://github.com/WICG/scroll-to-text-fragment/blob/main/EXTENSIONS.md#proposed-solution
// TODO(crbug/1265423): Rename to SelectorDirective
class CssSelectorDirective : public Directive {
 public:
  static CssSelectorDirective* TryParse(const String& directive_string);
  static Type ClassType() { return kSelector; }
  explicit CssSelectorDirective(const String& value);

  const AtomicString value() const { return value_; }

 protected:
  String ToStringImpl() const override;

 private:
  // an accepted CSS selector string specified in the directive's value field,
  AtomicString value_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FRAGMENT_DIRECTIVE_CSS_SELECTOR_DIRECTIVE_H_
