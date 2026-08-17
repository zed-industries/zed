// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TOOLS_CLANG_RAW_PTR_PLUGIN_TESTS_THIRD_PARTY_BLINK_RENDERER_DISCOURAGED_TYPE_H_
#define TOOLS_CLANG_RAW_PTR_PLUGIN_TESTS_THIRD_PARTY_BLINK_RENDERER_DISCOURAGED_TYPE_H_

#include <vector>

#include "third_party/blink/public/public.h"

namespace cc {
// Allowed, since type aliases are defined outside of blink:: namespace.
using CcVector = std::vector<double>;
typedef std::vector<double> CcVector2;
}  // namespace cc

namespace blink {

namespace nested {
// Not allowed. Will report error when this type is used for a data member.
using IntVector = std::vector<int>;
// Allowed, since this is a type alias of an allowed type.
using CcVector = cc::CcVector;
}  // namespace nested

struct Foo {
  Foo(std::vector<int> v);

  // Not allowed.
  std::vector<int> v1;

  using FloatVector = std::vector<float>;
  typedef std::vector<float> FloatVector2;
  // Not allowed.
  nested::IntVector v2a;
  FloatVector v2b;
  FloatVector2 v2c;

  // Not allowed.
  std::vector<char> v_array[4][4];

  // cc::CcVector is not under the blink:: namespace, the checker should
  // ignore it and allow the use. In real world, this will be OK as long as
  // audit_non_blink_usages.py allows cc::CcVector.
  cc::CcVector v3a;
  cc::CcVector2 v3b;

  // This is a type alias that ultimately refers to cc::CcVector. Since the
  // underlying type is not under the blink:: namespace, the checker should
  // ignore it and allow the use.
  nested::CcVector v3c;

  // This is a type alias defined in third_party/blink/public/public.h which
  // should not be checked.
  BlinkPublicType v4;

  std::vector<int> v5 __attribute__((annotate("other1")))
  __attribute__((annotate("other2"), annotate("allow_discouraged_type")));

  using VectorAllowed __attribute__((annotate("allow_discouraged_type"))) =
      std::vector<int>;
  VectorAllowed v6;
};

// Function params of discouraged types are allowed.
inline Foo::Foo(std::vector<int> v) {
  // This is OK.
  std::vector<int> vv(v);

  struct {
    // Not allowed.
    std::vector<int> v;
  } sv = {vv};

  v1 = sv.v;
}

template <typename T>
class Template {
  // Not allowed.
  std::vector<T> v1;

  std::vector<T> v2 __attribute__((annotate("allow_discouraged_type")));
};

}  // namespace blink

#endif  // TOOLS_CLANG_RAW_PTR_PLUGIN_TESTS_THIRD_PARTY_BLINK_RENDERER_DISCOURAGED_TYPE_H_
