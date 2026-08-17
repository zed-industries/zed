// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_STYLE_NAME_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_STYLE_NAME_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string.h"

namespace blink {

// Represents any named entity, provided by e.g. <custom-ident> | <string>.
//
// The StyleName will remember whether it came from a <custom-ident> or a
// a <string>, such that it can be serialized accordingly.
class CORE_EXPORT StyleName {
 public:
  enum class Type { kCustomIdent, kString };

  StyleName() = default;
  explicit StyleName(const AtomicString& value, Type type)
      : type_(type), value_(value) {}

  Type GetType() const { return type_; }

  bool IsCustomIdent() const { return type_ == Type::kCustomIdent; }

  const AtomicString& GetValue() const { return value_; }

  bool operator==(const StyleName& other) const {
    return type_ == other.type_ && value_ == other.value_;
  }
  bool operator!=(const StyleName& other) const { return !(*this == other); }

 private:
  Type type_ = Type::kString;
  AtomicString value_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_STYLE_NAME_H_
