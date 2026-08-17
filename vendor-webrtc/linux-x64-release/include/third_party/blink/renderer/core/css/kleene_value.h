// Copyright 2025 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_KLEENE_VALUE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_KLEENE_VALUE_H_

namespace blink {

// See Kleene 3-valued logic
//
// https://drafts.csswg.org/mediaqueries-4/#evaluating
enum class KleeneValue {
  kTrue,
  kFalse,
  kUnknown,
};

inline KleeneValue KleeneNot(KleeneValue a) {
  switch (a) {
    case KleeneValue::kTrue:
      return KleeneValue::kFalse;
    case KleeneValue::kFalse:
      return KleeneValue::kTrue;
    case KleeneValue::kUnknown:
      return KleeneValue::kUnknown;
  }
}

inline KleeneValue KleeneOr(KleeneValue a, KleeneValue b) {
  switch (a) {
    case KleeneValue::kTrue:
      return KleeneValue::kTrue;
    case KleeneValue::kFalse:
      return b;
    case KleeneValue::kUnknown:
      return (b == KleeneValue::kTrue) ? KleeneValue::kTrue
                                       : KleeneValue::kUnknown;
  }
}

inline KleeneValue KleeneAnd(KleeneValue a, KleeneValue b) {
  switch (a) {
    case KleeneValue::kTrue:
      return b;
    case KleeneValue::kFalse:
      return KleeneValue::kFalse;
    case KleeneValue::kUnknown:
      return (b == KleeneValue::kFalse) ? KleeneValue::kFalse
                                        : KleeneValue::kUnknown;
  }
}

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_KLEENE_VALUE_H_
