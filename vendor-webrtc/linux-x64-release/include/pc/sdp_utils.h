/*
 *  Copyright 2017 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef PC_SDP_UTILS_H_
#define PC_SDP_UTILS_H_

#include <functional>
#include <memory>
#include <string>

#include "api/jsep.h"
#include "p2p/base/transport_info.h"
#include "pc/session_description.h"
#include "rtc_base/system/rtc_export.h"

namespace webrtc {

// Returns a copy of the given session description.
RTC_EXPORT std::unique_ptr<SessionDescriptionInterface> CloneSessionDescription(
    const SessionDescriptionInterface* sdesc);

// Returns a copy of the given session description with the type changed.
RTC_EXPORT std::unique_ptr<SessionDescriptionInterface>
CloneSessionDescriptionAsType(const SessionDescriptionInterface* sdesc,
                              SdpType type);

// Function that takes a single session description content with its
// corresponding transport and produces a boolean.
typedef std::function<bool(const webrtc::ContentInfo*,
                           const webrtc::TransportInfo*)>
    SdpContentPredicate;

// Returns true if the predicate returns true for all contents in the given
// session description.
bool SdpContentsAll(SdpContentPredicate pred, const SessionDescription* desc);

// Returns true if the predicate returns true for none of the contents in the
// given session description.
bool SdpContentsNone(SdpContentPredicate pred, const SessionDescription* desc);

// Function that takes a single session description content with its
// corresponding transport and can mutate the content and/or the transport.
typedef std::function<void(webrtc::ContentInfo*, webrtc::TransportInfo*)>
    SdpContentMutator;

// Applies the mutator function over all contents in the given session
// description.
void SdpContentsForEach(SdpContentMutator fn, SessionDescription* desc);

}  // namespace webrtc

#endif  // PC_SDP_UTILS_H_
