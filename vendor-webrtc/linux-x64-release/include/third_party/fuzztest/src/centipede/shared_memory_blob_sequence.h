// Copyright 2022 The Centipede Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef THIRD_PARTY_CENTIPEDE_SHARED_MEMORY_BLOB_SEQUENCE_H_
#define THIRD_PARTY_CENTIPEDE_SHARED_MEMORY_BLOB_SEQUENCE_H_

#include <climits>
#include <cstddef>
#include <cstdint>
#include <type_traits>

#include "absl/base/nullability.h"

// This library must not depend on anything other than libc,
// so that it does not introduce any dependencies to its users.
// Any such dependencies may get coverage-instrumented, introducing noise
// into coverage reporting.
// Small exceptions for header-only parts of STL may be possible.

namespace fuzztest::internal {

// Simple TLV (tag-length-value) data structure.
// Blob does not own the memory in `data`, just references it.
// `size` is the number of bytes in `data`.
// A blob with zero tag is considered invalid.
// A blob with zero size and non-zero tag is valid but this contradicts
// the current use.
// TODO(kcc): [impl] replace uses of (blob.size == 0) with (!blob.IsValid()).
// TODO(kcc): [impl] consider making it a class.
struct Blob {
  using SizeAndTagT = size_t;
  Blob(SizeAndTagT tag, SizeAndTagT size, const uint8_t *absl_nullable data)
      : tag(tag), size(size), data(data) {}
  Blob() = default;  // Construct an invalid Blob.
  bool IsValid() const { return tag != 0; }

  const SizeAndTagT tag = 0;
  const SizeAndTagT size = 0;
  const uint8_t *data = nullptr;
};

// The BlobSequence is a consecutive sequence of Blobs.
class BlobSequence {
 public:
  // Creates a new blob sequence of arbitrary `data` with given `size` in bytes,
  // must be >= 8. Aborts on any failure. The amount of actual data that can be
  // written is slightly less.
  explicit BlobSequence(uint8_t *data, size_t size);

  // Writes the contents of `blob` to the blob sequence.
  // Returns true on success.
  // Returns false when the blob sequence is full.
  // A failed Write does not change the internal state.
  // Must not be called after Read() w/o first calling Reset().
  bool Write(Blob blob);

  // Writes `tag`/`value` as a blob. `T` should be a POD.
  // Returns true on success.
  template <typename T>
  bool Write(Blob::SizeAndTagT tag, T value) {
    static_assert(std::is_pod_v<T>, "T must be a POD");
    return Write(
        {tag, sizeof(value), reinterpret_cast<const uint8_t *>(&value)});
  }

  // Reads the next blob from the sequence.
  // If a failure happens or no more blobs are left, returns a invalid blob
  // returning false on `IsValid()`. Unless a concurrent writer updates
  // the underlying buffer, all subsequent calls to `Read()` will return invalid
  // blobs. Must not be called after Write() w/o first calling Reset().
  Blob Read();

  // Resets the internal state, allowing to read from or write to
  // starting from the beginning of the blob sequence.
  void Reset();

  // The position after last Write (or last Read).
  size_t offset() const { return offset_; }

 protected:
  // Default constructor to create an empty blob sequence. It is used by the
  // constructors of child classes.
  explicit BlobSequence() = default;

  // data_ contains a sequence of {size, payload} pairs,
  // where size is 8 bytes and payload is size bytes.
  // After writing a blob, we also write 0 in place of the next blob's size,
  // if there is space left so that to overwrite any stale data left there.
  uint8_t *data_ = nullptr;
  size_t size_ = 0;

 private:
  // offset_ points to the position in data_ after last Write (or last Read).
  size_t offset_ = 0;
  bool had_reads_after_reset_ = false;
  bool had_writes_after_reset_ = false;
};

// SharedMemoryBlobSequence:
// enables inter-process communication via shared memory.
//
// It allows one process to write some data, then another process to read it.
// SharedMemoryBlobSequence is thread-compatible.
// It does not perform any inter-process synchronization itself, but relies on
// external synchronization e.g. via process fork/join or semaphores.
//
// Typical usage is to create a SharedMemoryBlobSequence in one process and then
// open SharedMemoryBlobSequence with the same name in another process.
// But it can be done in the same process too.
//
// Usage example:
//  void ParentProcess() {
//    // Create a new blob sequence.
//    SharedMemoryBlobSequence parent("/foo", 1000);
//
//    // Parent process writes some data to the shared blob:
//    parent.Write({some_data, some_data_size});
//    parent.Write({some_other_data, some_other_data_size});
//
//    // Run the child process.
//    ExecuteChildProcessAndWaitUntilItIsDone();
//  }
//
//  void Child() {
//    // Open an existing blob sequence.
//    SharedMemoryBlobSequence child("/foo");
//
//    // Read the data written by parent.
//    while (true) {
//      auto blob = parent.Read();
//      if (!blob.size) break;
//      Use({blob.data, blob.size});
//    }
//  }
//
class SharedMemoryBlobSequence : public BlobSequence {
 public:
  // Creates a new shared blob sequence with `name` (for debugging only, not an
  // actual path). Aborts on any failure. `size` is the size of the shared
  // memory region in bytes, must be >= 8. The amount of actual data that can be
  // written is slightly less.
  // The `use_posix_shmem` argument specifies which API to use to allocate the
  // shared memory. When true, shm_open(2) will be used, otherwise
  // memfd_create(2).
  SharedMemoryBlobSequence(const char *name, size_t size, bool use_posix_shmem);

  // Opens an existing shared blob sequence with the file `path`.
  // Aborts on any failure.
  explicit SharedMemoryBlobSequence(const char *path);

  // Releases all resources.
  ~SharedMemoryBlobSequence();

  // Releases shared memory used by `this`.
  void ReleaseSharedMemory();

  // Returns the number of bytes used by the shared mapping.
  // It will be zero just after creation and after the call to
  // ReleaseSharedMemory().
  size_t NumBytesUsed() const;

  // Gets the file path that can be used to create new instances.
  // TODO(ussuri): Refactor `char *` into a `string_view`.
  const char *absl_nonnull path() const { return path_; }

 private:
  // mmaps `size_` bytes from `fd_`, assigns to `data_`. Crashes if mmap failed.
  void MmapData();

  // Will be initialized as a generated internal path or a copy of `path`
  // passed in.
  char path_[PATH_MAX] = {0};
  int fd_ = -1;  // file descriptor used to mmap the shared memory region.
  // Whether the file pointed to by path_ is owned by this and needs to be
  // deallocated on destruction.
  bool path_is_owned_ = false;
};

}  // namespace fuzztest::internal

#endif  // THIRD_PARTY_CENTIPEDE_SHARED_MEMORY_BLOB_SEQUENCE_H_
