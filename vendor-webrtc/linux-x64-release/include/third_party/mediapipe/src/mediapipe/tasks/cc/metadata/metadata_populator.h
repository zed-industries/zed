/* Copyright 2022 The MediaPipe Authors.

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
==============================================================================*/

#ifndef MEDIAPIPE_TASKS_CC_METADATA_METADATA_POPULATOR_H_
#define MEDIAPIPE_TASKS_CC_METADATA_METADATA_POPULATOR_H_

#include "absl/container/flat_hash_map.h"
#include "absl/status/status.h"
#include "absl/status/statusor.h"
#include "flatbuffers/flatbuffers.h"
#include "mediapipe/tasks/metadata/metadata_schema_generated.h"
#include "tensorflow/lite/schema/schema_generated.h"

namespace mediapipe {
namespace tasks {
namespace metadata {

// TODO: bring to feature parity with Python library.

// Provides an interface to pack TFLite ModelMetadata [1] and corresponding
// associated files into a TFLite FlatBuffer.
//
// This class is NOT thread-safe.
//
// [1]: https://www.tensorflow.org/lite/convert/metadata
class ModelMetadataPopulator {
 public:
  // Creates a ModelMetadataPopulator from the provided TFLite Model FlatBuffer
  // and returns a pointer to the new object. Ownership is transferred to the
  // caller. Returns an error if the creation failed, which may happen e.g. if
  // the provided buffer is not a valid TFLite FlatBuffer.
  //
  // It is recommended to obtain and manage the buffer through an
  // ExternalFileHandler[1], which is optimized through mmap(2) to avoid having
  // to load the entire buffer in memory when provided by path or file
  // descriptor.
  //
  // [1]:
  // mediapipe/tasks/cc/core/external_file_handler.h
  static absl::StatusOr<std::unique_ptr<ModelMetadataPopulator>>
  CreateFromModelBuffer(const char* buffer_data, size_t buffer_size);

  // Writes the TFLite ModelMetadata provided as a buffer into the TFLite
  // FlatBuffer model.
  //
  // Warning: this method overwrites any already existing TFLite Model Metadata.
  // Calling this method multiple times overwrites the metadata from previous
  // calls, so this method should usually be called only once.
  void LoadMetadata(const char* metadata_buffer_data,
                    size_t metadata_buffer_size);

  // Loads associated files into the TFLite FlatBuffer model. The input is a map
  // of {filename, file contents}.
  //
  // Warning: this method removes any previously present associated files.
  // Calling this method multiple time removes any associated files from
  // previous calls, so this method should usually be called only once.
  void LoadAssociatedFiles(
      const absl::flat_hash_map<std::string, std::string>& associated_files);

  // Finalizes metadata population. Returns the TFLite FlatBuffer model with
  // metadata and associated files as a string buffer.
  absl::StatusOr<std::string> Populate();

 private:
  // Private constructor.
  explicit ModelMetadataPopulator(const tflite::Model& model);
  // Zips and appends associated files to the provided model buffer. Called
  // internally by `Populate()`.
  absl::StatusOr<std::string> AppendAssociatedFiles(
      const char* model_buffer_data, size_t model_buffer_size);

  // The unpacked model FlatBuffer.
  tflite::ModelT model_t_;
  // The associated files.
  absl::flat_hash_map<std::string, std::string> associated_files_;
};

}  // namespace metadata
}  // namespace tasks
}  // namespace mediapipe

#endif  // MEDIAPIPE_TASKS_CC_METADATA_METADATA_POPULATOR_H_
