// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_CONTROL_KEY_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_CONTROL_KEY_H_

#include <utility>

#include "third_party/blink/renderer/platform/wtf/hash_traits.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string_hash.h"

namespace blink {

// ControlKey class represents a pair of a name attribute value and a control
// type of a form control element.  It's used in blink::SavedFormState.
class ControlKey {
 public:
  ControlKey(const AtomicString& name, const AtomicString& type)
      : name_(name), type_(type) {}
  ControlKey(const ControlKey& other) = default;

  ControlKey& operator=(const ControlKey& other) {
    name_ = other.GetName();
    type_ = other.GetType();
    return *this;
  }

  const AtomicString& GetName() const { return name_; }
  const AtomicString& GetType() const { return type_; }

  // Hash table deleted values, which are only constructed and never copied or
  // destroyed.
  ControlKey(WTF::HashTableDeletedValueType) : type_(g_star_atom) {}
  bool IsHashTableDeletedValue() const { return type_ == g_star_atom; }

  friend bool operator==(const ControlKey&, const ControlKey&) = default;

 private:
  AtomicString name_;
  AtomicString type_;

  friend struct ControlKeyHashTraits;
};

struct ControlKeyHashTraits
    : TwoFieldsHashTraits<ControlKey, &ControlKey::name_, &ControlKey::type_> {
};

using ControlKeyData = std::pair<const AtomicString&, const AtomicString&>;

// ControlKeyTranslator reduces refcount churn of AtomicStrings on
// HashMap lookups.
struct ControlKeyTranslator {
  static unsigned GetHash(const ControlKeyData& data) {
    // The following hash computation is equivalent to TwoFieldsHashTraits.
    // We can use neither PairHashTraits nor TwoFieldsHashTraits because of
    // reference data members of ControlKeyData.
    return WTF::HashInts(WTF::GetHash(data.first), WTF::GetHash(data.second));
  }
  static bool Equal(const ControlKey& key, const ControlKeyData& data) {
    return ControlKeyData{key.GetName(), key.GetType()} == data;
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_CONTROL_KEY_H_
