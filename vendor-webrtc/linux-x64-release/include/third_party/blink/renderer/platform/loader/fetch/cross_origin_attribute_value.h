// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_CROSS_ORIGIN_ATTRIBUTE_VALUE_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_CROSS_ORIGIN_ATTRIBUTE_VALUE_H_

namespace blink {

// This corresponds to the CORS settings attributes defined in the HTML spec:
// https://html.spec.whatwg.org/C/#cors-settings-attributes
enum CrossOriginAttributeValue {
  kCrossOriginAttributeNotSet,
  kCrossOriginAttributeAnonymous,
  kCrossOriginAttributeUseCredentials,
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_CROSS_ORIGIN_ATTRIBUTE_VALUE_H_
