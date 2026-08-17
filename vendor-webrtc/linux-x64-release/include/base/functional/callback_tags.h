// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// This defines helpful tags for dealing with Callbacks. Those tags can be used
// to construct special callbacks. This lives in its own file to avoid circular
// dependencies.

#ifndef BASE_FUNCTIONAL_CALLBACK_TAGS_H_
#define BASE_FUNCTIONAL_CALLBACK_TAGS_H_

namespace base::internal {

struct NullCallbackTag {
  template <typename Signature>
  struct WithSignature {};
};

struct DoNothingCallbackTag {
  template <typename Signature>
  struct WithSignature {};

  template <typename... BoundArgs>
  struct WithBoundArguments {
    std::tuple<BoundArgs...> bound_args;

    constexpr explicit WithBoundArguments(BoundArgs... args)
        : bound_args(std::forward<BoundArgs>(args)...) {}
  };
};

}  // namespace base::internal

#endif  // BASE_FUNCTIONAL_CALLBACK_TAGS_H_
