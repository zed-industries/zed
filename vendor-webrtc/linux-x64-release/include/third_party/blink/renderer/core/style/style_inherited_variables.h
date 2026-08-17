// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_STYLE_INHERITED_VARIABLES_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_STYLE_INHERITED_VARIABLES_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/css_value.h"
#include "third_party/blink/renderer/core/css/css_variable_data.h"
#include "third_party/blink/renderer/core/style/style_variables.h"
#include "third_party/blink/renderer/platform/heap/persistent.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"
#include "third_party/blink/renderer/platform/wtf/ref_counted.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string_hash.h"

#include <iosfwd>

namespace blink {

class CORE_EXPORT StyleInheritedVariables
    : public GarbageCollected<StyleInheritedVariables> {
 public:
  StyleInheritedVariables() = default;
  StyleInheritedVariables(StyleInheritedVariables& other) {
    variables_ = other.variables_;
  }

  void Trace(Visitor* visitor) const { visitor->Trace(variables_); }

  bool operator==(const StyleInheritedVariables& other) const {
    return variables_ == other.variables_;
  }
  bool operator!=(const StyleInheritedVariables& other) const {
    return !(*this == other);
  }

  void SetData(const AtomicString& name, CSSVariableData* value) {
    DCHECK(!value || !value->NeedsVariableResolution());
    variables_.SetData(name, value);
  }
  std::optional<CSSVariableData*> GetData(const AtomicString& name) const {
    return variables_.GetData(name);
  }

  void SetValue(const AtomicString& name, const CSSValue* value) {
    variables_.SetValue(name, value);
  }
  std::optional<const CSSValue*> GetValue(const AtomicString& name) const {
    return variables_.GetValue(name);
  }

  // Note that not all custom property names returned here necessarily have
  // valid values, due to cycles or references to invalid variables without
  // using a fallback.
  void CollectNames(HashSet<AtomicString>& names) const {
    variables_.CollectNames(names);
  }

  // For debugging/logging.
  friend std::ostream& operator<<(std::ostream& stream,
                                  const StyleInheritedVariables& variables) {
    return stream << variables.variables_;
  }

 private:
  StyleVariables variables_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_STYLE_INHERITED_VARIABLES_H_
