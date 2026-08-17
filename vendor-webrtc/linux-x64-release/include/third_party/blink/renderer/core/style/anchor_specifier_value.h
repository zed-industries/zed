// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_ANCHOR_SPECIFIER_VALUE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_ANCHOR_SPECIFIER_VALUE_H_

#include "base/types/pass_key.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"

namespace blink {

class ScopedCSSName;

// Represents an anchor specifier: default | auto | <dashed-ident>
// https://drafts4.csswg.org/css-anchor-1/#target-anchor-element
class CORE_EXPORT AnchorSpecifierValue
    : public GarbageCollected<AnchorSpecifierValue> {
 public:
  enum class Type {
    kDefault,
    kNamed,
  };

  // Creates a named value.
  explicit AnchorSpecifierValue(const ScopedCSSName&);

  // Gets or creates the default keyword value.
  static AnchorSpecifierValue* Default();

  // For creating the keyword values only.
  explicit AnchorSpecifierValue(base::PassKey<AnchorSpecifierValue>, Type type);

  bool IsDefault() const { return type_ == Type::kDefault; }
  bool IsNamed() const { return type_ == Type::kNamed; }
  const ScopedCSSName& GetName() const {
    DCHECK(IsNamed());
    DCHECK(name_);
    return *name_;
  }

  bool operator==(const AnchorSpecifierValue&) const;
  bool operator!=(const AnchorSpecifierValue& other) const {
    return !operator==(other);
  }

  unsigned GetHash() const;

  void Trace(Visitor*) const;

 private:
  Type type_;
  Member<const ScopedCSSName> name_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_ANCHOR_SPECIFIER_VALUE_H_
