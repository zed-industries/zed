// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_LOADING_ATTRIBUTE_VALUE_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_LOADING_ATTRIBUTE_VALUE_H_

namespace blink {

enum class LoadingAttributeValue {
  kAuto,
  kLazy,
  kEager,
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_LOADER_FETCH_LOADING_ATTRIBUTE_VALUE_H_
