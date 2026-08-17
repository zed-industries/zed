// Copyright 2024 The MediaPipe Authors.
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

#ifndef MEDIAPIPE_TASKS_GENAI_INFERENCE_UTILS_LLM_UTILS_MODEL_DATA_H_
#define MEDIAPIPE_TASKS_GENAI_INFERENCE_UTILS_LLM_UTILS_MODEL_DATA_H_

#include <cstdint>
#include <functional>
#include <memory>
#include <optional>
#include <string>
#include <utility>

#include "absl/status/statusor.h"
#include "absl/strings/string_view.h"
#include "absl/types/span.h"
#include "mediapipe/framework/port/status_macros.h"
#include "mediapipe/tasks/cc/genai/inference/proto/llm_params.pb.h"
#include "mediapipe/tasks/cc/genai/inference/utils/llm_utils/memory_mapped_file.h"
#include "mediapipe/tasks/cc/genai/inference/utils/llm_utils/scoped_file.h"
#include "tensorflow/lite/model_builder.h"

namespace mediapipe::tasks::genai::llm_utils {

// Provides access to data tied to an underlying resource. The resource may be
// released when this object is destroyed and spans previously returned from
// GetData() will no longer be valid.
template <typename T>
class DataHolder {
 public:
  virtual ~DataHolder() = default;

  virtual absl::Span<T> GetData() const = 0;

  // The underlying data may be at an offset into a buffer. This method gets the
  // underlying data with no offsets.
  virtual absl::Span<T> GetRawData() const { return GetData(); }
};

struct OffsetAndSize {
  uint64_t offset = 0;
  uint64_t size = 0;
};
// Gets an offset and size which will be valid to pass to MemoryMappedFile.
OffsetAndSize GetAlignedOffsetAndSize(uint64_t base_offset, uint64_t base_size);

// Creates a DataHolder by memory mapping `file`. `key` can be passed as an
// optimization when the same file is being mapped multiple times. It should be
// unique to `file`.
template <typename T>
absl::StatusOr<std::unique_ptr<DataHolder<T>>> CreateMemoryMappedDataHolder(
    ScopedFile::PlatformFile file, uint64_t offset = 0, uint64_t size = 0,
    absl::string_view key = "") {
  class MemoryMappedDataHolder : public DataHolder<T> {
   public:
    explicit MemoryMappedDataHolder(std::unique_ptr<MemoryMappedFile> region,
                                    uint64_t offset, uint64_t size)
        : region_(std::move(region)), offset_(offset), size_(size) {}
    ~MemoryMappedDataHolder() override = default;

    absl::Span<T> GetData() const override {
      return absl::MakeSpan(static_cast<T*>(region_->data()) + offset_, size_);
    }

    absl::Span<T> GetRawData() const override {
      return absl::MakeSpan(static_cast<T*>(region_->data()),
                            region_->length());
    }

   private:
    std::unique_ptr<MemoryMappedFile> region_;
    uint64_t offset_;
    uint64_t size_;
  };

  OffsetAndSize offset_and_size;
  if (offset != 0 || size != 0) {
    offset_and_size = GetAlignedOffsetAndSize(offset, size);
  }
  MP_ASSIGN_OR_RETURN(auto region,
                      MemoryMappedFile::Create(file, offset_and_size.offset,
                                               offset_and_size.size, key));
  if (size == 0) {
    size = region->length();
  }
  return std::make_unique<MemoryMappedDataHolder>(
      std::move(region), offset - offset_and_size.offset, size);
}

// This class is responsible for accessing the underlying model data and
// abstracting out any differences in file formats.
class ModelData {
 public:
  // Loads from a single tflite flatbuffer. The allocation should contain the
  // whole model including buffers.
  static absl::StatusOr<std::shared_ptr<ModelData>> Create(
      std::shared_ptr<tflite::FlatBufferModel> model);

  // Loads a tflite model from a file. This is more efficient than the above
  // method since the data can be read into memory as needed.
  static absl::StatusOr<std::shared_ptr<ModelData>> Create(ScopedFile file);

  // Similar to the above, but accept shared_ptr. The smart pointer is pointing
  // to a constant object, indicating there's only read access.
  static absl::StatusOr<std::shared_ptr<ModelData>> Create(
      std::shared_ptr<const ScopedFile> file);

  // Loads `ModelData` from the provided `weight_path`, which contains a tflite
  // file.
  static absl::StatusOr<std::shared_ptr<ModelData>> Create(
      absl::string_view weight_path);

  enum ReadMode {
    KEEP = 0,
    DISCARD = 1,
    DISCARD_ALL = 2,
  };
  using ReadDataFn =
      std::function<void*(uint64_t offset, uint64_t size, int mode)>;
  // Loads a tflite model using the passed `fn`, and reads buffers as needed.
  static absl::StatusOr<std::shared_ptr<ModelData>> Create(ReadDataFn fn);

  virtual ~ModelData() = default;

  // Get the type for the model. If a type is not specified by the model files,
  // std::nullopt will be returned.
  virtual std::optional<odml::infra::proto::LlmModelType> GetModelType() = 0;

  // Get the LoRA rank of the model, or std::nullopt if this is not a set of
  // LoRA weights.
  virtual std::optional<int> LoRARank() = 0;

  // Get the parameters to define the model.
  virtual const odml::infra::proto::LlmParameters& GetLlmParameters() = 0;

  // Read a metadata string about the model.
  virtual absl::StatusOr<std::string> ReadMetadata(absl::string_view name) = 0;

  // Returns the maximum tensor size for this model.
  virtual uint64_t GetMaxTensorSize() const = 0;

  // Gets the size of the tensor with `name` or 0 if it does not exist.
  virtual uint64_t GetTensorSize(absl::string_view name) const = 0;

  // Returns the tensor data of the tensor with `name`.
  virtual absl::StatusOr<std::unique_ptr<DataHolder<uint8_t>>> ReadTensor(
      absl::string_view name) = 0;

  // Frees the underlying data.
  virtual void Clear() = 0;

  // Holds the tflite model as well as the backing data.
  struct ModelWithData {
    std::unique_ptr<tflite::FlatBufferModel> model;
    std::unique_ptr<DataHolder<uint8_t>> data;
  };
  // Reads a tflite model from the main model.
  virtual absl::StatusOr<ModelWithData> ReadModel(absl::string_view name) = 0;
};

// Holds data referring to a set of LoRA weights.
struct LoRAData {
  // The ID used to refer to this LoRA.
  uint32_t id;

  // The weight data for this LoRA.
  std::shared_ptr<ModelData> model_data;
};

}  // namespace mediapipe::tasks::genai::llm_utils

#endif  // MEDIAPIPE_TASKS_GENAI_INFERENCE_UTILS_LLM_UTILS_MODEL_DATA_H_
