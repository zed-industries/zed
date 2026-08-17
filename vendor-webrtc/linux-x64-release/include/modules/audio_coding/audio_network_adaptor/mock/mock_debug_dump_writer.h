/*
 *  Copyright (c) 2016 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_CODING_AUDIO_NETWORK_ADAPTOR_MOCK_MOCK_DEBUG_DUMP_WRITER_H_
#define MODULES_AUDIO_CODING_AUDIO_NETWORK_ADAPTOR_MOCK_MOCK_DEBUG_DUMP_WRITER_H_

#include "modules/audio_coding/audio_network_adaptor/debug_dump_writer.h"
#include "test/gmock.h"

namespace webrtc {

class MockDebugDumpWriter : public DebugDumpWriter {
 public:
  ~MockDebugDumpWriter() override { Die(); }
  MOCK_METHOD(void, Die, ());

  MOCK_METHOD(void,
              DumpEncoderRuntimeConfig,
              (const AudioEncoderRuntimeConfig& config, int64_t timestamp),
              (override));
  MOCK_METHOD(void,
              DumpNetworkMetrics,
              (const Controller::NetworkMetrics& metrics, int64_t timestamp),
              (override));
#if WEBRTC_ENABLE_PROTOBUF
  MOCK_METHOD(void,
              DumpControllerManagerConfig,
              (const audio_network_adaptor::config::ControllerManager&
                   controller_manager_config,
               int64_t timestamp),
              (override));
#endif
};

}  // namespace webrtc

#endif  // MODULES_AUDIO_CODING_AUDIO_NETWORK_ADAPTOR_MOCK_MOCK_DEBUG_DUMP_WRITER_H_
