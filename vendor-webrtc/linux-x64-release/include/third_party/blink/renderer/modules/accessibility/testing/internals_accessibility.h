// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_ACCESSIBILITY_TESTING_INTERNALS_ACCESSIBILITY_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_ACCESSIBILITY_TESTING_INTERNALS_ACCESSIBILITY_H_

#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

// InternalsAccessibility implements several accessibility-related methods that
// are exposed on the Internals object. These methods are used by various WPTs
// to inspect aspects of the accessibility tree that aren't directly obtainable
// through standard JavaScript APIs.
//
// Two of the methods, getComputedLabel and getComputedRole, are exposed on
// WPT's TestDriver as get_computed_label and get_computed_role [1]. The
// TestDriver methods are hooked up to the implementations on the internals
// object in testdriver-vendor.js [2].
//
// See third_party/blink/web_tests/external/wpt/accname/basic.html [3] for an
// example usage of get_computed_label and get_computed_role.
//
// [1]
// https://source.chromium.org/chromium/chromium/src/+/main:third_party/blink/web_tests/external/wpt/resources/testdriver.js;l=235
// [2]
// https://source.chromium.org/chromium/chromium/src/+/main:third_party/blink/web_tests/resources/testdriver-vendor.js
// [3]
// https://source.chromium.org/chromium/chromium/src/+/main:third_party/blink/web_tests/external/wpt/accname/basic.html

namespace WTF {
class String;
}  // namespace WTF

namespace blink {

class Element;
class Internals;

class InternalsAccessibility {
  STATIC_ONLY(InternalsAccessibility);

 public:
  static unsigned numberOfLiveAXObjects(Internals&);

  static WTF::String getComputedLabel(Internals&, const Element* element);
  static WTF::String getComputedRole(Internals&, const Element* element);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_ACCESSIBILITY_TESTING_INTERNALS_ACCESSIBILITY_H_
