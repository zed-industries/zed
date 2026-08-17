// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_P2P_NETWORK_MANAGER_UMA_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_P2P_NETWORK_MANAGER_UMA_H_

namespace blink {

// TODO(crbug.com/787254): Move the enum and helper methods here
// out of the Blink exposed API when all users of it have been Onion souped.

enum IPPermissionStatus {
  PERMISSION_UNKNOWN,  // Requested but have never fired SignalNetworksChanged.
  PERMISSION_NOT_REQUESTED,             // Multiple routes is not requested.
  PERMISSION_DENIED,                    // Requested but denied.
  PERMISSION_GRANTED_WITH_CHECKING,     // Requested and granted after checking
                                        // mic/camera permission.
  PERMISSION_GRANTED_WITHOUT_CHECKING,  // Requested and granted without
                                        // checking mic/camera permission.
  PERMISSION_MAX,
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_P2P_NETWORK_MANAGER_UMA_H_
