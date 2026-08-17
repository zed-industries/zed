// Copyright 2025 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_PEER_CONNECTION_UTIL_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_PEER_CONNECTION_UTIL_H_

#include "third_party/blink/renderer/core/dom/dom_high_res_time_stamp.h"
#include "third_party/blink/renderer/core/execution_context/execution_context.h"

namespace blink {

// Returns a DOMHighResTimeStamp relative to Performance.timeOrigin.
DOMHighResTimeStamp CalculateRTCEncodedFrameTimestamp(ExecutionContext*,
                                                      base::TimeTicks);

// Returns a DOMHighResTimeStamp equivalent to the given delta
DOMHighResTimeStamp CalculateRTCEncodedFrameTimeDelta(ExecutionContext*,
                                                      base::TimeDelta);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_PEER_CONNECTION_UTIL_H_
