// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_PARSER_CSS_NESTING_TYPE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_PARSER_CSS_NESTING_TYPE_H_

#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

// Note that order matters: kNesting effectively also means kScope,
// and therefore it's convenient to compute the max CSSNestingType
// in some cases.
enum class CSSNestingType {
  // We are not in a nesting context, and '&' resolves like :scope instead.
  kNone,
  // We are in a nesting context as defined by @scope.
  //
  // https://drafts.csswg.org/css-cascade-6/#scope-atrule
  // https://drafts.csswg.org/selectors-4/#scope-pseudo
  kScope,
  // We are in a css-nesting nesting context, and '&' resolves according to:
  // https://drafts.csswg.org/css-nesting-1/#nest-selector
  kNesting,
  // We are inside @function. The parsing behavior is generally the same as
  // kNesting, except we don't allow qualified rules, and we emit
  // CSSFunctionDeclarations instead of CSSNestedDeclarations.
  kFunction,
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_PARSER_CSS_NESTING_TYPE_H_
