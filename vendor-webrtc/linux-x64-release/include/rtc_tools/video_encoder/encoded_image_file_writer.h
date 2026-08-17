/*
 *  Copyright (c) 2023 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef RTC_TOOLS_VIDEO_ENCODER_ENCODED_IMAGE_FILE_WRITER_H_
#define RTC_TOOLS_VIDEO_ENCODER_ENCODED_IMAGE_FILE_WRITER_H_

#include <memory>
#include <string>
#include <utility>
#include <vector>

#include "modules/video_coding/include/video_codec_interface.h"
#include "modules/video_coding/utility/ivf_file_writer.h"

namespace webrtc {
namespace test {

// The `EncodedImageFileWriter` writes the `EncodedImage` into ivf output. It
// supports SVC to output ivf for all decode targets.
class EncodedImageFileWriter final {
  // The pair of writer and output file name.
  using IvfWriterPair = std::pair<std::unique_ptr<IvfFileWriter>, std::string>;

 public:
  explicit EncodedImageFileWriter(const VideoCodec& video_codec_setting);

  ~EncodedImageFileWriter();

  int Write(const EncodedImage& encoded_image);

 private:
  VideoCodec video_codec_setting_;

  int spatial_layers_ = 0;
  int temporal_layers_ = 0;
  InterLayerPredMode inter_layer_pred_mode_ = InterLayerPredMode::kOff;

  bool is_base_layer_key_frame = false;
  std::vector<IvfWriterPair> decode_target_writers_;
};

}  // namespace test
}  // namespace webrtc

#endif  // RTC_TOOLS_VIDEO_ENCODER_ENCODED_IMAGE_FILE_WRITER_H_
