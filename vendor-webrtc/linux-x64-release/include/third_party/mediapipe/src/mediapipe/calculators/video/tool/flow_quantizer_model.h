// Copyright 2019 The MediaPipe Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// Quantization model to convert a real value float number (flow field) to a
// 8-bit discrete number.
#ifndef MEDIAPIPE_CALCULATORS_VIDEO_TOOL_FLOW_QUANTIZER_MODEL_H_
#define MEDIAPIPE_CALCULATORS_VIDEO_TOOL_FLOW_QUANTIZER_MODEL_H_

#include <cstdint>

#include "mediapipe/calculators/video/tool/flow_quantizer_model.pb.h"
#include "mediapipe/framework/formats/motion/optical_flow_field.h"
#include "mediapipe/framework/tool/status_util.h"

namespace mediapipe {

class FlowQuantizerModel {
 public:
  // Initializes the model proto.
  void Init();
  // Quantizes flow field with the model.
  uint8_t Apply(const float val, const int channel) const;
  // Loads model from proto.
  void LoadFromProto(const QuantizerModelData& data);
  // Gets proto from model.
  const QuantizerModelData& GetModelData() const;
  // Used in training. Updates the model proto by reading the flow fields.
  // TODO: This model is currently manually set. Need to find a way to
  // learn from flow fields directly.
  void AddSampleFlowField(const OpticalFlowField& flow);

 private:
  QuantizerModelData model_;
};
}  // namespace mediapipe

#endif  // MEDIAPIPE_CALCULATORS_VIDEO_TOOL_FLOW_QUANTIZER_MODEL_H_
