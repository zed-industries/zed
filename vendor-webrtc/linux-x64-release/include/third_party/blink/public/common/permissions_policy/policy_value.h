// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_PERMISSIONS_POLICY_POLICY_VALUE_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_PERMISSIONS_POLICY_POLICY_VALUE_H_

#include "third_party/blink/public/common/common_export.h"
#include "third_party/blink/public/mojom/permissions_policy/policy_value.mojom-shared.h"

namespace blink {

// ListValue (PolicyValue)
// ----------------------
// PolicyValue is a union of types (int / double / set<int> / bool ) that can be
// used to specify the parameter of a policy.

// TODO(crbug.com/1119481): Add the following types: enum, inc/dec int, inc
// double, set.
class BLINK_COMMON_EXPORT PolicyValue {
 public:
  PolicyValue();
  PolicyValue(const PolicyValue&);
  PolicyValue& operator=(const PolicyValue&);

  static PolicyValue CreateBool(bool);
  static PolicyValue CreateDecDouble(double);
  static PolicyValue CreateEnum(int32_t);

  // A 'max' PolicyValue is the most permissive value for the policy.
  static PolicyValue CreateMaxPolicyValue(mojom::PolicyValueType type);
  // A 'min' PolicyValue is the most restrictive value for the policy.
  static PolicyValue CreateMinPolicyValue(mojom::PolicyValueType type);

  mojom::PolicyValueType Type() const { return type_; }
  void SetType(mojom::PolicyValueType type) { type_ = type; }

  // PolicyValue getters.
  // Note the getters also DCHECKs that the type is correct.
  bool BoolValue() const;
  double DoubleValue() const;
  int32_t IntValue() const;

  // PolicyValue setters.
  // Note the getters also DCHECKs that the type is correct.
  void SetBoolValue(bool bool_value);
  void SetDoubleValue(double double_value);
  void SetIntValue(int32_t int_value);

  void SetToMax();
  void SetToMin();

  // Test whether this policy value is compatible with required policy value.
  // Note: a.IsCompatibleWith(b) == true does not necessary indicate
  // b.IsCompatibleWith(a) == false, because not all policy value types support
  // strictness comparison, e.g. enum.
  bool IsCompatibleWith(const PolicyValue& required) const;

 private:
  explicit PolicyValue(bool bool_value);
  PolicyValue(int32_t int_value, mojom::PolicyValueType type);
  PolicyValue(double double_value, mojom::PolicyValueType type);

  mojom::PolicyValueType type_;
  bool bool_value_ = false;
  double double_value_ = 0.0;
  int32_t int_value_ = -1;
};

bool BLINK_COMMON_EXPORT operator==(const PolicyValue& lhs,
                                    const PolicyValue& rhs);
bool BLINK_COMMON_EXPORT operator!=(const PolicyValue& lhs,
                                    const PolicyValue& rhs);
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_PERMISSIONS_POLICY_POLICY_VALUE_H_
