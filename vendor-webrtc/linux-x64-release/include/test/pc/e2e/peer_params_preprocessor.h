/*
 *  Copyright (c) 2022 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef TEST_PC_E2E_PEER_PARAMS_PREPROCESSOR_H_
#define TEST_PC_E2E_PEER_PARAMS_PREPROCESSOR_H_

#include <memory>
#include <set>
#include <string>

#include "api/test/pclf/peer_configurer.h"

namespace webrtc {
namespace webrtc_pc_e2e {

class PeerParamsPreprocessor {
 public:
  PeerParamsPreprocessor();
  ~PeerParamsPreprocessor();

  // Set missing params to default values if it is required:
  //  * Generate video stream labels if some of them are missing
  //  * Generate audio stream labels if some of them are missing
  //  * Set video source generation mode if it is not specified
  //  * Video codecs under test
  void SetDefaultValuesForMissingParams(PeerConfigurer& peer);

  // Validate peer's parameters, also ensure uniqueness of all video stream
  // labels.
  void ValidateParams(const PeerConfigurer& peer);

 private:
  class DefaultNamesProvider;
  std::unique_ptr<DefaultNamesProvider> peer_names_provider_;

  std::set<std::string> peer_names_;
  std::set<std::string> video_labels_;
  std::set<std::string> audio_labels_;
  std::set<std::string> video_sync_groups_;
  std::set<std::string> audio_sync_groups_;
};

}  // namespace webrtc_pc_e2e
}  // namespace webrtc

#endif  // TEST_PC_E2E_PEER_PARAMS_PREPROCESSOR_H_
