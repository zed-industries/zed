// Copyright 2020 The MediaPipe Authors.
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

#ifndef MEDIAPIPE_FRAMEWORK_FORMATS_TENSOR_MTL_BUFFER_VIEW_H_
#define MEDIAPIPE_FRAMEWORK_FORMATS_TENSOR_MTL_BUFFER_VIEW_H_

#import <Metal/Metal.h>

#include <algorithm>
#include <functional>
#include <initializer_list>
#include <numeric>
#include <tuple>
#include <type_traits>
#include <utility>
#include <vector>

#include "absl/container/flat_hash_map.h"
#include "absl/synchronization/mutex.h"
#include "mediapipe/framework/formats/tensor.h"
#include "mediapipe/framework/port.h"

namespace mediapipe {
class MtlBufferView : public Tensor::View {
 public:
  // The command buffer status is checked for completeness if GPU-to-CPU
  // synchronization is required.
  static MtlBufferView GetReadView(const Tensor& tensor,
                                   id<MTLCommandBuffer> command_buffer);
  static MtlBufferView GetWriteView(const Tensor& tensor,
                                    id<MTLCommandBuffer> command_buffer);
  static MtlBufferView GetWriteView(const Tensor& tensor, id<MTLDevice> device);

  id<MTLBuffer> buffer() const { return buffer_; }
  MtlBufferView(MtlBufferView&& src) : Tensor::View(std::move(src.lock_)) {
    buffer_ = std::exchange(src.buffer_, nil);
  }

 protected:
  friend class Tensor;
  static void AllocateMtlBuffer(const Tensor& tensor, id<MTLDevice> device);
  MtlBufferView(id<MTLBuffer> buffer, std::unique_ptr<absl::MutexLock>&& lock)
      : Tensor::View(std::move(lock)), buffer_(buffer) {}
  id<MTLBuffer> buffer_;
};

}  // namespace mediapipe

#endif  // MEDIAPIPE_FRAMEWORK_FORMATS_TENSOR_MTL_BUFFER_VIEW_H_
