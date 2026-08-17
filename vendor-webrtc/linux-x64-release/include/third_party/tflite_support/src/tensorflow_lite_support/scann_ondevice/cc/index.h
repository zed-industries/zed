/* Copyright 2022 The TensorFlow Authors. All Rights Reserved.

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

#ifndef TENSORFLOW_LITE_SUPPORT_SCANN_ONDEVICE_CC_INDEX_H_
#define TENSORFLOW_LITE_SUPPORT_SCANN_ONDEVICE_CC_INDEX_H_

#include <memory>

#include "absl/status/status.h"  // from @com_google_absl
#include "absl/status/statusor.h"  // from @com_google_absl
#include "absl/strings/string_view.h"  // from @com_google_absl
#include "leveldb/cache.h"  // from @com_google_leveldb
#include "leveldb/iterator.h"  // from @com_google_leveldb
#include "leveldb/table.h"  // from @com_google_leveldb
#include "tensorflow_lite_support/scann_ondevice/cc/mem_random_access_file.h"
#include "tensorflow_lite_support/scann_ondevice/proto/index_config.pb.h"

namespace tflite {
namespace scann_ondevice {

// Helper class for getting access to the data contained in the LevelDB index
// file.
//
// This class is NOT thread-safe.
class Index {
 public:
  // Creates an Index from the provided buffer. Ownership is transferred to the
  // caller. Returns an error if the creation failed, which may happen e.g. if
  // the provided buffer is not a valid LevelDB index file.
  //
  // Warning: Does not take ownership of the provided buffer, which must outlive
  // this object.
  static absl::StatusOr<std::unique_ptr<Index>> CreateFromIndexBuffer(
      const char* buffer_data, size_t buffer_size);

  // Parses and returns the `IndexConfig` stored in the index file.
  absl::StatusOr<IndexConfig> GetIndexConfig() const;

  // Provides access to the opaque user info stored in the index file (if any),
  // in raw binary form. Returns an empty string if the index doesn't contain
  // user info.
  absl::StatusOr<absl::string_view> GetUserInfo() const;

  // Provides access to the partition data corresponding to the i-th leaf in the
  // order specified in the `IndexConfig`, in raw binary form.
  //
  // Warning: In order to avoid unnecessary copies, the underlying pointer for
  // the returned string view is only valid until next call to this method.
  absl::StatusOr<absl::string_view> GetPartitionAtIndex(uint32_t i) const;

  // Provides access to the metadata associated with the i-th embedding in the
  // index, in raw binary form.
  //
  // Warning: In order to avoid unnecessary copies, the underlying pointer for
  // the returned string view is only valid until next call to this method.
  absl::StatusOr<absl::string_view> GetMetadataAtIndex(uint32_t i) const;

 private:
  // Private default constructor, called from CreateFromBuffer().
  Index() = default;
  // Initializes the Index from the provided buffer.
  absl::Status InitFromBuffer(const char* buffer_data, size_t buffer_size);

  std::unique_ptr<leveldb::Table> table_;
  std::unique_ptr<MemRandomAccessFile> file_;
  std::unique_ptr<leveldb::Cache> cache_;
  // One iterator per getter, so that calls from one getter don't invalidate
  // results from another one.
  std::unique_ptr<leveldb::Iterator> config_iterator_;
  std::unique_ptr<leveldb::Iterator> info_iterator_;
  std::unique_ptr<leveldb::Iterator> embedding_iterator_;
  std::unique_ptr<leveldb::Iterator> metadata_iterator_;
};

}  // namespace scann_ondevice
}  // namespace tflite

#endif  // TENSORFLOW_LITE_SUPPORT_SCANN_ONDEVICE_CC_INDEX_H_
