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

#ifndef MEDIAPIPE_GPU_METAL_SHADER_BASE_H_
#define MEDIAPIPE_GPU_METAL_SHADER_BASE_H_

#include <simd/simd.h>

typedef struct {
  // Vertex position in 2D clip space.
  vector_float2 position;
  // Corresponding texture coordinate.
  vector_float2 texture_coordinate;
} MediaPipeTexturedVertex;

// Common buffer indices used in our Metal shaders.
typedef enum {
  MediaPipeBufferIndexInputVertices = 0,
  MediaPipeBufferIndexRgbWeights = 1,
  MediaPipeBufferIndexPixelSize = 2,
  MediaPipeBufferIndexOutputColor = 3,
} MediaPipeBufferIndex;

// Common texture indices used in our Metal shaders.
typedef enum {
  MediaPipeTextureIndexInputColor = 0,
  MediaPipeTextureIndexOutputColor = 1,
} MediaPipeTextureIndex;

typedef vector_float3 MetalRgbWeightPacketType;

#endif  // MEDIAPIPE_GPU_METAL_SHADER_BASE_H_
