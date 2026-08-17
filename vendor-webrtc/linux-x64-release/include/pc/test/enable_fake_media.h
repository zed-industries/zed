/*
 *  Copyright 2023 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

// Enables fake media support for PeerConnnectionFactory created from `deps` for
// testing purposes. Such fake media support ignores media dependencies in the
// `PeerConnectionFactoryDependencies`. Allows to test PeerConnection and
// PeerConnectionFactory in the presence of the media, but doesn't test media
// support itself.

#ifndef PC_TEST_ENABLE_FAKE_MEDIA_H_
#define PC_TEST_ENABLE_FAKE_MEDIA_H_

#include <memory>

#include "absl/base/nullability.h"
#include "api/peer_connection_interface.h"
#include "media/base/fake_media_engine.h"

namespace webrtc {

// Enables media support backed by the 'fake_media_engine'.
void EnableFakeMedia(
    PeerConnectionFactoryDependencies& deps,
    absl_nonnull std::unique_ptr<FakeMediaEngine> fake_media_engine);

// Enables media support backed by unspecified lightweight fake implementation.
void EnableFakeMedia(PeerConnectionFactoryDependencies& deps);

}  // namespace webrtc

#endif  //  PC_TEST_ENABLE_FAKE_MEDIA_H_
