// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_STYLE_INITIAL_DATA_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_STYLE_INITIAL_DATA_H_

#include "base/memory/ptr_util.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/css_value.h"
#include "third_party/blink/renderer/core/style/style_variables.h"
#include "third_party/blink/renderer/platform/heap/persistent.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string_hash.h"

namespace blink {

class Document;
class PropertyRegistry;

// Holds data stored on the initial ComputedStyle only.
//
// An instance of this class is created once, and then shared between all the
// ComputedStyles that inherit (directly or indirectly) from the initial style.
class CORE_EXPORT StyleInitialData : public GarbageCollected<StyleInitialData> {
 public:
  StyleInitialData(Document&, const PropertyRegistry&);

  void Trace(Visitor* visitor) const { visitor->Trace(variables_); }

  bool operator==(const StyleInitialData& other) const;
  bool operator!=(const StyleInitialData& other) const {
    return !(*this == other);
  }

  bool HasInitialVariables() const { return !variables_.IsEmpty(); }

  void CollectVariableNames(HashSet<AtomicString>& names) const {
    return variables_.CollectNames(names);
  }

  CSSVariableData* GetVariableData(const AtomicString& name) const {
    return variables_.GetData(name).value_or(nullptr);
  }

  const CSSValue* GetVariableValue(const AtomicString& name) const {
    return variables_.GetValue(name).value_or(nullptr);
  }

  unsigned GetViewportUnitFlags() const { return viewport_unit_flags_; }

 private:

  // Initial values for all registered properties. This is set on
  // the initial style, and then shared with all other styles that directly or
  // indirectly inherit from that.
  StyleVariables variables_;
  // This is equal to `PropertyRegistry::GetViewportUnitFlags()` at the time the
  // `StyleInitialData` was created.
  //
  // Since StyleInitialData is only (re)created during style resolution, this
  // tells us whether ComputedStyles from that process depend on viewport units
  // or not, which in turns tells us if we need to recalculate any styles when
  // we resize.
  //
  // PropertyRegistry::GetViewportUnitFlags on the other hand, can change
  // immediately via JavaScript, and is also affected by active style updates.
  // Hence this is not useful for understanding whether or not any current
  // ComputedStyles need to be invalidated by a resize.
  unsigned viewport_unit_flags_ = 0;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_STYLE_INITIAL_DATA_H_
