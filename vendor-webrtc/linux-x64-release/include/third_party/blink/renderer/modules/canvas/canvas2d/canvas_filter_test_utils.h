// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_CANVAS_CANVAS2D_CANVAS_FILTER_TEST_UTILS_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_CANVAS_CANVAS2D_CANVAS_FILTER_TEST_UTILS_H_

#include <string>

#include "base/functional/bind.h"
#include "testing/gmock/include/gmock/gmock.h"
#include "testing/gtest/include/gtest/gtest.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"  // IWYU pragma: keep

namespace blink {
class V8TestingScope;
class V8UnionObjectOrObjectArrayOrString;
}  // namespace blink

namespace blink_testing {

// Matcher testing equality between garbage collected objects.
//
// To be compatible with parameterized tests, `GarbageCollectedIs` lazy creates
// the expected garbage collected object. That's because GC objects can't be
// created in the global scope and so, we can't call `MakeGarbageCollected`
// inside of `INSTANTIATE_TEST_SUITE_P`. To circumvent this,
// `GarbageCollectedIs` delays the creation of the garbage collected objects to
// when the comparison is performed.
//
// Example use:
//  Foo* gc_object = MakeGarbageCollected<Foo>(1, 2);
//  EXPECT_THAT(gc_object, GarbageCollectedIs<Foo>(1, 2));
MATCHER_P(GarbageCollectedIsMatcher, matcher, "") {
  return ExplainMatchResult(testing::Eq(testing::ByRef(*matcher.Run())), *arg,
                            result_listener);
}

template <typename T, typename... Args>
auto GarbageCollectedIs(const Args&... args) {
  return GarbageCollectedIsMatcher(base::BindRepeating(
      [](const Args&... args) { return MakeGarbageCollected<T>(args...); },
      args...));
}

blink::V8UnionObjectOrObjectArrayOrString* ParseFilter(
    blink::V8TestingScope& scope,
    const std::string& value);

}  // namespace blink_testing

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_CANVAS_CANVAS2D_CANVAS_FILTER_TEST_UTILS_H_
