// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_PROPERTIES_CSS_PROPERTY_REF_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_PROPERTIES_CSS_PROPERTY_REF_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/properties/longhands/custom_property.h"

namespace blink {

class CSSPropertyName;
class Document;

// Use this class to acquire a reference to a CSSProperty instance. The
// reference returned by GetProperty() may point to the embedded CustomProperty
// object, hence this reference is only valid for the lifetime of the
// CSSPropertyRef object.
//
// Usage:
//
//   CSSPropertyRef ref(some_string, document);
//
//   if (ref.IsValid()) {
//     LOG(INFO) << ref.GetProperty().GetName();
//   }
//
// Note that any CSSPropertyRef constructor may produce an invalid
// CSSPropertyRef (e.g. if a non-existent property name is provided), so be
// sure to always check IsValid() before calling GetProperty().
class CORE_EXPORT CSSPropertyRef {
  STACK_ALLOCATED();

 public:
  // Look up (or create) a CSSProperty.
  //
  // If the incoming 'name' is not a CSS property, the CSSPropertyRef is
  // invalid.
  CSSPropertyRef(const String& name, const Document&);

  // Like above, but will never produce an invalid CSSPropertyRef.
  CSSPropertyRef(const CSSPropertyName&, const Document&);

  // If you already have a CSSProperty& object, you may use it to get
  // a CSSPropertyRef again.
  //
  // Note that the CSSProperty& returned by GetProperty() may be different
  // than the incoming CSSProperty&.
  //
  // Note also that this CSSPropertyRef is invalid if the incoming CSSProperty&
  // is the static Variable instance.
  CSSPropertyRef(const CSSProperty&);

  bool IsValid() const { return property_id_ != CSSPropertyID::kInvalid; }

  const CSSProperty& GetProperty() const {
    DCHECK(IsValid());
    if (property_id_ == CSSPropertyID::kVariable) {
      return custom_property_;
    }
    return CSSProperty::Get(ResolveCSSPropertyID(property_id_));
  }

  const CSSUnresolvedProperty& GetUnresolvedProperty() const {
    if (IsPropertyAlias(property_id_)) {
      return *GetPropertyInternal(property_id_);
    }
    return GetProperty();
  }

 private:
  CSSPropertyID property_id_;
  CustomProperty custom_property_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_PROPERTIES_CSS_PROPERTY_REF_H_
