/*
 *  Copyright (c) 2019 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef TEST_PC_E2E_ANALYZER_VIDEO_SINGLE_PROCESS_ENCODED_IMAGE_DATA_INJECTOR_H_
#define TEST_PC_E2E_ANALYZER_VIDEO_SINGLE_PROCESS_ENCODED_IMAGE_DATA_INJECTOR_H_

#include <cstdint>
#include <map>
#include <memory>
#include <utility>
#include <vector>

#include "api/video/encoded_image.h"
#include "rtc_base/synchronization/mutex.h"
#include "test/pc/e2e/analyzer/video/encoded_image_data_injector.h"

namespace webrtc {
namespace webrtc_pc_e2e {

// Based on assumption that all call participants are in the same OS process
// and uses same QualityAnalyzingVideoContext to obtain
// EncodedImageDataInjector.
//
// To inject frame id and discard flag into EncodedImage injector uses last 3rd
// and 2nd bytes of EncodedImage payload. Then it uses last byte for frame
// sub id, that is required to distinguish different spatial layers. The origin
// data from these 3 bytes will be stored inside injector's internal storage and
// then will be restored during extraction phase.
//
// This injector won't add any extra overhead into EncodedImage payload and
// support frames with any size of payload. Also assumes that every EncodedImage
// payload size is greater or equals to 3 bytes
//
// This injector doesn't support video frames/encoded images without frame ID.
class SingleProcessEncodedImageDataInjector
    : public EncodedImageDataPropagator {
 public:
  SingleProcessEncodedImageDataInjector();
  ~SingleProcessEncodedImageDataInjector() override;

  // Id and discard flag will be injected into EncodedImage buffer directly.
  // This buffer won't be fully copied, so `source` image buffer will be also
  // changed.
  EncodedImage InjectData(uint16_t id,
                          bool discard,
                          const EncodedImage& source) override;

  void Start(int expected_receivers_count) override {
    MutexLock crit(&lock_);
    expected_receivers_count_ = expected_receivers_count;
  }
  void AddParticipantInCall() override;
  void RemoveParticipantInCall() override;
  EncodedImageExtractionResult ExtractData(const EncodedImage& source) override;

 private:
  // Contains data required to extract frame id from EncodedImage and restore
  // original buffer.
  struct ExtractionInfo {
    // Number of bytes from the beginning of the EncodedImage buffer that will
    // be used to store frame id and sub id.
    const static size_t kUsedBufferSize = 3;
    // Frame sub id to distinguish encoded images for different spatial layers.
    uint8_t sub_id;
    // Flag to show is this encoded images should be discarded by analyzing
    // decoder because of not required spatial layer/simulcast stream.
    bool discard;
    // Data from first 3 bytes of origin encoded image's payload.
    uint8_t origin_data[ExtractionInfo::kUsedBufferSize];
    // Count of how many times this frame was received.
    int received_count = 0;
  };

  struct ExtractionInfoVector {
    ExtractionInfoVector();
    ~ExtractionInfoVector();

    // Next sub id, that have to be used for this frame id.
    uint8_t next_sub_id = 0;
    std::map<uint8_t, ExtractionInfo> infos;
  };

  Mutex lock_;
  int expected_receivers_count_ RTC_GUARDED_BY(lock_);
  // Stores a mapping from frame id to extraction info for spatial layers
  // for this frame id. There can be a lot of them, because if frame was
  // dropped we can't clean it up, because we won't receive a signal on
  // decoder side about that frame. In such case it will be replaced
  // when sub id will overlap.
  std::map<uint16_t, ExtractionInfoVector> extraction_cache_
      RTC_GUARDED_BY(lock_);
};

}  // namespace webrtc_pc_e2e
}  // namespace webrtc

#endif  // TEST_PC_E2E_ANALYZER_VIDEO_SINGLE_PROCESS_ENCODED_IMAGE_DATA_INJECTOR_H_
