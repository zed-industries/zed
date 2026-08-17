// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_METRICS_PERSISTENT_HISTOGRAM_STORAGE_H_
#define BASE_METRICS_PERSISTENT_HISTOGRAM_STORAGE_H_

#include <string_view>

#include "base/base_export.h"
#include "base/files/file_path.h"

namespace base {

// This class creates a fixed sized persistent memory to allow histograms to be
// stored in it. When a PersistentHistogramStorage is destructed, histograms
// recorded during its lifetime are persisted in the directory
// |storage_base_dir_|/|allocator_name| (see the ctor for allocator_name).
// Histograms are not persisted if the storage directory does not exist on
// destruction. PersistentHistogramStorage should be instantiated as early as
// possible in the process lifetime and should never be instantiated again.
// Persisted histograms will eventually be reported by Chrome.
class BASE_EXPORT PersistentHistogramStorage {
 public:
  enum class StorageDirManagement { kCreate, kUseExisting };

  // Creates a process-wide storage location for histograms that will be written
  // to a file within a directory provided by |set_storage_base_dir()| on
  // destruction.
  // The |allocator_name| is used both as an internal name for the allocator,
  // well as the leaf directory name for the file to which the histograms are
  // persisted. The string must be ASCII.
  // |storage_dir_management| specifies if this instance reuses an existing
  // storage directory, or is responsible for creating one.
  PersistentHistogramStorage(std::string_view allocator_name,
                             StorageDirManagement storage_dir_management);

  PersistentHistogramStorage(const PersistentHistogramStorage&) = delete;
  PersistentHistogramStorage& operator=(const PersistentHistogramStorage&) =
      delete;

  ~PersistentHistogramStorage();

  // The storage directory isn't always known during initial construction so
  // it's set separately. The last one wins if there are multiple calls to this
  // method.
  void set_storage_base_dir(const FilePath& storage_base_dir) {
    storage_base_dir_ = storage_base_dir;
  }

  // Disables histogram storage.
  void Disable() { disabled_ = true; }

 private:
  // Metrics files are written into directory
  // |storage_base_dir_|/|allocator_name| (see the ctor for allocator_name).
  FilePath storage_base_dir_;

  // The setting of the storage directory management.
  const StorageDirManagement storage_dir_management_;

  // A flag indicating if histogram storage is disabled. It starts with false,
  // but can be set to true by the caller who decides to throw away its
  // histogram data.
  bool disabled_ = false;
};

}  // namespace base

#endif  // BASE_METRICS_PERSISTENT_HISTOGRAM_STORAGE_H_
