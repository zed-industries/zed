// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_FILE_DESCRIPTOR_STORE_H_
#define BASE_FILE_DESCRIPTOR_STORE_H_

#include <map>
#include <string>

#include "base/base_export.h"
#include "base/files/memory_mapped_file.h"
#include "base/files/scoped_file.h"

namespace base {

// The file descriptor store is used to associate file descriptors with keys
// that must be unique.
// It is used to share file descriptors from a process to its child.
class BASE_EXPORT FileDescriptorStore {
 public:
  FileDescriptorStore(const FileDescriptorStore&) = delete;
  FileDescriptorStore& operator=(const FileDescriptorStore&) = delete;
  struct Descriptor {
    Descriptor(const std::string& key, base::ScopedFD fd);
    Descriptor(const std::string& key,
               base::ScopedFD fd,
               base::MemoryMappedFile::Region region);
    Descriptor(Descriptor&& other);
    ~Descriptor();

    Descriptor& operator=(Descriptor&& other) = default;

    // Globally unique key.
    std::string key;
    // Actual FD.
    base::ScopedFD fd;
    // Optional region, defaults to kWholeFile.
    base::MemoryMappedFile::Region region;
  };
  using Mapping = std::map<std::string, Descriptor>;

  // Returns the singleton instance of FileDescriptorStore.
  static FileDescriptorStore& GetInstance();

  // Gets a descriptor given a key and also populates |region|.
  // It is a fatal error if the key is not known.
  base::ScopedFD TakeFD(const std::string& key,
                        base::MemoryMappedFile::Region* region);

  // Gets a descriptor given a key. Returns an empty ScopedFD on error.
  base::ScopedFD MaybeTakeFD(const std::string& key,
                             base::MemoryMappedFile::Region* region);

  // Sets the descriptor for the given |key|. This sets the region associated
  // with |key| to kWholeFile.
  void Set(const std::string& key, base::ScopedFD fd);

  // Sets the descriptor and |region| for the given |key|.
  void Set(const std::string& key,
           base::ScopedFD fd,
           base::MemoryMappedFile::Region region);

 private:
  FileDescriptorStore();
  ~FileDescriptorStore();

  Mapping descriptors_;
};

}  // namespace base

#endif  // BASE_FILE_DESCRIPTOR_STORE_H_
