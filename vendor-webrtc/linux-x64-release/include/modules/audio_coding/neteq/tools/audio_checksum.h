/*
 *  Copyright (c) 2014 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_CODING_NETEQ_TOOLS_AUDIO_CHECKSUM_H_
#define MODULES_AUDIO_CODING_NETEQ_TOOLS_AUDIO_CHECKSUM_H_

#include <cstddef>
#include <cstdint>
#include <memory>
#include <string>

#include "modules/audio_coding/neteq/tools/audio_sink.h"
#include "rtc_base/buffer.h"
#include "rtc_base/message_digest.h"
#include "rtc_base/string_encode.h"
#include "rtc_base/system/arch.h"

namespace webrtc {
namespace test {

class AudioChecksum : public AudioSink {
 public:
  AudioChecksum()
      : checksum_(MessageDigestFactory::Create(DIGEST_MD5)),
        checksum_result_(checksum_->Size()),
        finished_(false) {}

  AudioChecksum(const AudioChecksum&) = delete;
  AudioChecksum& operator=(const AudioChecksum&) = delete;

  bool WriteArray(const int16_t* audio, size_t num_samples) override {
    if (finished_)
      return false;

#ifndef WEBRTC_ARCH_LITTLE_ENDIAN
#error "Big-endian gives a different checksum"
#endif
    checksum_->Update(audio, num_samples * sizeof(*audio));
    return true;
  }

  // Finalizes the computations, and returns the checksum.
  std::string Finish() {
    if (!finished_) {
      finished_ = true;
      checksum_->Finish(checksum_result_.data(), checksum_result_.size());
    }
    return hex_encode(checksum_result_);
  }

 private:
  std::unique_ptr<MessageDigest> checksum_;
  Buffer checksum_result_;
  bool finished_;
};

}  // namespace test
}  // namespace webrtc
#endif  // MODULES_AUDIO_CODING_NETEQ_TOOLS_AUDIO_CHECKSUM_H_
