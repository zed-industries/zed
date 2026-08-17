// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_ALLOW_DISCOURAGED_TYPE_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_ALLOW_DISCOURAGED_TYPE_H_

#if defined(__clang__)
#define ALLOW_DISCOURAGED_TYPE(reason) \
  __attribute__((annotate("allow_discouraged_type")))
#else  // !defined(__clang__)
#define ALLOW_DISCOURAGED_TYPE(reason)
#endif  // !defined(__clang__)

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_ALLOW_DISCOURAGED_TYPE_H_
