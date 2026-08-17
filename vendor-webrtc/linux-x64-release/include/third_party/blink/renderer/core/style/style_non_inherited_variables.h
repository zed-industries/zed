// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_STYLE_NON_INHERITED_VARIABLES_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_STYLE_NON_INHERITED_VARIABLES_H_

#include <memory>
#include <utility>

#include "base/memory/ptr_util.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/css_value.h"
#include "third_party/blink/renderer/core/css/css_variable_data.h"
#include "third_party/blink/renderer/core/style/style_variables.h"
#include "third_party/blink/renderer/platform/heap/persistent.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string_hash.h"

namespace blink {

class CORE_EXPORT StyleNonInheritedVariables
    : public GarbageCollected<StyleNonInheritedVariables> {
 public:
  void Trace(Visitor* visitor) const { visitor->Trace(variables_); }

  bool operator==(const StyleNonInheritedVariables& other) const {
    return variables_ == other.variables_;
  }

  bool operator!=(const StyleNonInheritedVariables& other) const {
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

  void CollectNames(HashSet<AtomicString>& names) const {
    variables_.CollectNames(names);
  }

  friend CORE_EXPORT std::ostream& operator<<(
      std::ostream& stream,
      const StyleNonInheritedVariables& variables);

 private:
  StyleVariables variables_;
};

inline CORE_EXPORT std::ostream& operator<<(
    std::ostream& stream,
    const StyleNonInheritedVariables& variables) {
  return stream << variables.variables_;
}

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_STYLE_NON_INHERITED_VARIABLES_H_
