/*
 * Copyright (C) 2010 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 *     * Redistributions of source code must retain the above copyright
 * notice, this list of conditions and the following disclaimer.
 *     * Redistributions in binary form must reproduce the above
 * copyright notice, this list of conditions and the following disclaimer
 * in the documentation and/or other materials provided with the
 * distribution.
 *     * Neither the name of Google Inc. nor the names of its
 * contributors may be used to endorse or promote products derived from
 * this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_WINDOW_FEATURES_H_
#define THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_WINDOW_FEATURES_H_

#include <optional>
#include <vector>

#include "third_party/blink/public/platform/web_string.h"

namespace blink {

struct WebWindowFeatures {
  int x = 0;
  bool x_set = false;
  int y = 0;
  bool y_set = false;
  int width = 0;
  bool width_set = false;
  int height = 0;
  bool height_set = false;

  bool is_popup = false;

  // Whether the created window is a partitioned popin. If true, `is_popup` must
  // be true. See: https://explainers-by-googlers.github.io/partitioned-popins/
  bool is_partitioned_popin = false;

  // The members above this line are transferred through mojo
  // in the form of |struct WindowFeatures| defined in window_features.mojom,
  // to be used across process boundaries.
  // Below members are the ones not transferred through mojo.
  bool resizable = true;

  bool noopener = false;
  bool explicit_opener = false;
  bool noreferrer = false;
  bool background = false;
  bool persistent = false;

  // If `std::nullopt`, no impression should be set on the navigation.
  // If `std::vector::empty()`, an impression should be set but no background
  // request should be made. Otherwise, an impression should be set and a
  // background request should be made to the contained relative URL.
  //
  // TODO(apaseltiner): Investigate moving this field to a non-public struct
  // since it is only needed within //third_party/blink.
  std::optional<std::vector<WebString>> attribution_srcs;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_WEB_WEB_WINDOW_FEATURES_H_
