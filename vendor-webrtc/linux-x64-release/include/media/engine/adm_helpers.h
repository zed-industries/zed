/*
 *  Copyright (c) 2017 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MEDIA_ENGINE_ADM_HELPERS_H_
#define MEDIA_ENGINE_ADM_HELPERS_H_

namespace webrtc {

class AudioDeviceModule;

namespace adm_helpers {

void Init(AudioDeviceModule* adm);

}  // namespace adm_helpers
}  // namespace webrtc

#endif  // MEDIA_ENGINE_ADM_HELPERS_H_
