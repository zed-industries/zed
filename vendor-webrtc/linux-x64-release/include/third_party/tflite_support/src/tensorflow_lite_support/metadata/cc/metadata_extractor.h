/* Copyright 2020 The TensorFlow Authors. All Rights Reserved.

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
#ifndef TENSORFLOW_LITE_SUPPORT_METADATA_CC_METADATA_EXTRACTOR_H_
#define TENSORFLOW_LITE_SUPPORT_METADATA_CC_METADATA_EXTRACTOR_H_

#include "absl/container/flat_hash_map.h"  // from @com_google_absl
#include "absl/status/status.h"  // from @com_google_absl
#include "absl/strings/string_view.h"  // from @com_google_absl
#include "tensorflow/lite/schema/schema_generated.h"
#include "tensorflow_lite_support/cc/port/statusor.h"
#include "tensorflow_lite_support/metadata/metadata_schema_generated.h"

namespace tflite {
namespace metadata {

// Extracts and provides easy access to the TFLite ModelMetadata [1] and
// corresponding associated files packed into a TFLite FlatBuffer, if any.
//
// [1]: https://www.tensorflow.org/lite/convert/metadata
class ModelMetadataExtractor {
 public:
  // Creates a ModelMetadataExtractor from the provided TFLite Model FlatBuffer
  // and returns a pointer to the new object. Ownership is transferred to the
  // caller. Returns an error if the creation failed, which may happen e.g. if
  // the provided buffer is not a valid TFLite FlatBuffer.
  //
  // Warning: Does not take ownership of the provided buffer, which must outlive
  // this object.
  //
  // It is recommended to obtain and manage the buffer through an
  // ExternalFileHandler[1], which is optimized through mmap(2) to avoid having
  // to load the entire buffer in memory when provided by path or file
  // descriptor.
  //
  // [1]:
  // tensorflow_lite_support/c/task/core/external_file_handler.h
  static tflite::support::StatusOr<std::unique_ptr<ModelMetadataExtractor>>
  CreateFromModelBuffer(const char* buffer_data, size_t buffer_size);

  // Returns the pointer to the *first* ProcessUnit with the provided type, or
  // nullptr if none can be found. An error is returned if multiple
  // ProcessUnit-s with the provided type are found.
  static tflite::support::StatusOr<const tflite::ProcessUnit*>
  FindFirstProcessUnit(const tflite::TensorMetadata& tensor_metadata,
                       tflite::ProcessUnitOptions type);

  // Returns the name of the *first* associated file with the provided type and
  // (optional) locale in the provided TensorMetadata, or an empty string if no
  // such associated file can be found (which is not necessarily an error: some
  // models have no associated files at all) or its `name` field is unspecified.
  // Note: see `GetAssociatedFile` to read the actual file contents.
  static std::string FindFirstAssociatedFileName(
      const tflite::TensorMetadata& tensor_metadata,
      tflite::AssociatedFileType type,
      absl::string_view locale = absl::string_view());

  // Returns a pointer to the extracted TFLite Model Metadata, or nullptr if no
  // metadata was present in the Model FlatBuffer provided at creation time.
  const tflite::ModelMetadata* GetModelMetadata() const {
    return model_metadata_;
  }

  // Gets the contents of the associated file with the provided name packed into
  // the model metadata. An error is returned if there is no such associated
  // file.
  tflite::support::StatusOr<absl::string_view> GetAssociatedFile(
      const std::string& filename) const;

  // Gets the model version from the model metadata.  An error is returned if
  // either the metadata does not exist or no model version is present in it.
  tflite::support::StatusOr<std::string> GetModelVersion() const;

  // Note: all methods below retrieves metadata of the *first* subgraph as
  // default.

  // Gets the metadata for input tensors.
  const flatbuffers::Vector<flatbuffers::Offset<tflite::TensorMetadata>>*
  GetInputTensorMetadata() const;

  // Gets the metadata for the input tensor specified by the given index, or
  // nullptr in case there is no metadata or the index is out of range.
  const tflite::TensorMetadata* GetInputTensorMetadata(int index) const;

  // Gets the count of input tensors with metadata in the metadata FlatBuffer.
  // In particular, 0 is returned when there is no metadata.
  int GetInputTensorCount() const;

  // Gets the metadata for output tensors.
  const flatbuffers::Vector<flatbuffers::Offset<tflite::TensorMetadata>>*
  GetOutputTensorMetadata() const;

  // Gets the metadata for the output tensor specified by the given index, or
  // nullptr in case there is no metadata or the index is out of range.
  const tflite::TensorMetadata* GetOutputTensorMetadata(int index) const;

  // Gets the count of output tensors with metadata in the metadata FlatBuffer.
  // In particular, 0 is returned when there is no metadata.
  int GetOutputTensorCount() const;

  // Gets the input process units from SubgraphMetadata.input_process_units,
  // could be nullptr.
  const flatbuffers::Vector<flatbuffers::Offset<tflite::ProcessUnit>>*
  GetInputProcessUnits() const;

  // Gets the input process unit specified by the given index, or nullptr in
  // case there is no input process unit or the index is out of range.
  const tflite::ProcessUnit* GetInputProcessUnit(int index) const;

  // Gets the count of input process units. In particular, 0 is returned when
  // there is no input process units.
  int GetInputProcessUnitsCount() const;

  // Gets the output process units from SubgraphMetadata.output_process_units,
  // could be nullptr.
  const flatbuffers::Vector<flatbuffers::Offset<tflite::ProcessUnit>>*
  GetOutputProcessUnits() const;

  // Gets the output process unit specified by the given index, or nullptr in
  // case there is no output process unit or the index is out of range.
  const tflite::ProcessUnit* GetOutputProcessUnit(int index) const;

  // Gets the count of output process units. In particular, 0 is returned when
  // there is no output process units.
  int GetOutputProcessUnitsCount() const;

 private:
  static constexpr int kDefaultSubgraphIndex = 0;
  // Private default constructor, called from CreateFromModel().
  ModelMetadataExtractor() = default;
  // Initializes the ModelMetadataExtractor from the provided Model FlatBuffer.
  absl::Status InitFromModelBuffer(const char* buffer_data, size_t buffer_size);
  // Extracts and stores in associated_files_ the associated files (if present)
  // packed into the model FlatBuffer data.
  absl::Status ExtractAssociatedFiles(const char* buffer_data,
                                      size_t buffer_size);
  // Pointer to the TFLite Model object from which to read the ModelMetadata.
  const tflite::Model* model_{nullptr};
  // Pointer to the extracted ModelMetadata, if any.
  const tflite::ModelMetadata* model_metadata_{nullptr};
  // The files associated with the ModelMetadata, as a map with the filename
  // (corresponding to a basename, e.g. "labels.txt") as key and a pointer to
  // the file contents as value.
  absl::flat_hash_map<std::string, absl::string_view> associated_files_;
};

}  // namespace metadata
}  // namespace tflite

#endif  // TENSORFLOW_LITE_SUPPORT_METADATA_CC_METADATA_EXTRACTOR_H_
