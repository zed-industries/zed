/*
 *  Copyright (c) 2019 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef MODULES_RTP_RTCP_SOURCE_RTP_DEPENDENCY_DESCRIPTOR_READER_H_
#define MODULES_RTP_RTCP_SOURCE_RTP_DEPENDENCY_DESCRIPTOR_READER_H_

#include <cstdint>

#include "api/array_view.h"
#include "api/transport/rtp/dependency_descriptor.h"
#include "rtc_base/bitstream_reader.h"

namespace webrtc {
// Deserializes DependencyDescriptor rtp header extension.
class RtpDependencyDescriptorReader {
 public:
  // Parses the dependency descriptor.
  RtpDependencyDescriptorReader(ArrayView<const uint8_t> raw_data,
                                const FrameDependencyStructure* structure,
                                DependencyDescriptor* descriptor);
  RtpDependencyDescriptorReader(const RtpDependencyDescriptorReader&) = delete;
  RtpDependencyDescriptorReader& operator=(
      const RtpDependencyDescriptorReader&) = delete;

  // Returns true if parse was successful.
  bool ParseSuccessful() { return buffer_.Ok(); }

 private:
  // Functions to read template dependency structure.
  void ReadTemplateDependencyStructure();
  void ReadTemplateLayers();
  void ReadTemplateDtis();
  void ReadTemplateFdiffs();
  void ReadTemplateChains();
  void ReadResolutions();

  // Function to read details for the current frame.
  void ReadMandatoryFields();
  void ReadExtendedFields();
  void ReadFrameDependencyDefinition();

  void ReadFrameDtis();
  void ReadFrameFdiffs();
  void ReadFrameChains();

  // Output.
  DependencyDescriptor* const descriptor_;
  // Values that are needed while reading the descriptor, but can be discarded
  // when reading is complete.
  BitstreamReader buffer_;
  int frame_dependency_template_id_ = 0;
  bool active_decode_targets_present_flag_ = false;
  bool custom_dtis_flag_ = false;
  bool custom_fdiffs_flag_ = false;
  bool custom_chains_flag_ = false;
  const FrameDependencyStructure* structure_ = nullptr;
};

}  // namespace webrtc

#endif  // MODULES_RTP_RTCP_SOURCE_RTP_DEPENDENCY_DESCRIPTOR_READER_H_
