// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_LIFECYCLE_UPDATE_H_
#define THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_LIFECYCLE_UPDATE_H_

#include <stdint.h>

namespace blink {

// The type of lifecycle update to request.
enum class WebLifecycleUpdate { kLayout, kPrePaint, kAll };

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_LIFECYCLE_UPDATE_H_
