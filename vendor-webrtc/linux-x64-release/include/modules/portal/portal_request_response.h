/*
 *  Copyright (c) 2022 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_PORTAL_PORTAL_REQUEST_RESPONSE_H_
#define MODULES_PORTAL_PORTAL_REQUEST_RESPONSE_H_

namespace webrtc {
namespace xdg_portal {

// Contains type of responses that can be observed when making a request to
// a desktop portal interface.
enum class RequestResponse {
  // Unknown, the initialized status.
  kUnknown,
  // Success, the request is carried out.
  kSuccess,
  // The user cancelled the interaction.
  kUserCancelled,
  // The user interaction was ended in some other way.
  kError,

  kMaxValue = kError,
};

}  // namespace xdg_portal
}  // namespace webrtc
#endif  // MODULES_PORTAL_PORTAL_REQUEST_RESPONSE_H_
