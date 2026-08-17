// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_SET_SINK_ID_CALLBACKS_H_
#define THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_SET_SINK_ID_CALLBACKS_H_

#include <optional>

#include "base/functional/callback.h"
#include "third_party/blink/public/platform/web_string.h"

namespace blink {

enum class WebSetSinkIdError {
  kNotFound = 1,
  kNotAuthorized,
  kAborted,
  kNotSupported,
};

using WebSetSinkIdCompleteCallback =
    base::OnceCallback<void(std::optional<WebSetSinkIdError> error)>;

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_PLATFORM_WEB_SET_SINK_ID_CALLBACKS_H_
