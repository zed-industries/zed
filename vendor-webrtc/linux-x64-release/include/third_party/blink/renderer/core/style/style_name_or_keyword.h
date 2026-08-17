// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_STYLE_NAME_OR_KEYWORD_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_STYLE_NAME_OR_KEYWORD_H_

#include "base/check_op.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css_value_keywords.h"
#include "third_party/blink/renderer/core/style/style_name.h"

namespace blink {

class CORE_EXPORT StyleNameOrKeyword {
 public:
  explicit StyleNameOrKeyword(StyleName name)
      : keyword_(CSSValueID::kInvalid), name_(name) {}
  explicit StyleNameOrKeyword(CSSValueID keyword) : keyword_(keyword) {
    DCHECK_NE(keyword, CSSValueID::kInvalid);
  }

  bool IsKeyword() const { return keyword_ != CSSValueID::kInvalid; }

  CSSValueID GetKeyword() const {
    DCHECK(IsKeyword());
    return keyword_;
  }

  const StyleName& GetName() const {
    DCHECK(!IsKeyword());
    return name_;
  }

  bool operator==(const StyleNameOrKeyword& other) const {
    return keyword_ == other.keyword_ && name_ == other.name_;
  }
  bool operator!=(const StyleNameOrKeyword& other) const {
    return !(*this == other);
  }

 private:
  CSSValueID keyword_;
  StyleName name_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_STYLE_NAME_OR_KEYWORD_H_
