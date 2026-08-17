// Copyright 2013 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_FILES_MEMORY_MAPPED_FILE_H_
#define BASE_FILES_MEMORY_MAPPED_FILE_H_

#include <stddef.h>
#include <stdint.h>

#include <utility>

#include "base/base_export.h"
#include "base/containers/span.h"
#include "base/files/file.h"
#include "base/memory/raw_ptr_exclusion.h"
#include "build/build_config.h"

#if BUILDFLAG(IS_WIN)
#include "base/win/scoped_handle.h"
#endif

namespace base {

class FilePath;

class BASE_EXPORT MemoryMappedFile {
 public:
  enum Access {
    // Mapping a file into memory effectively allows for file I/O on any thread.
    // The accessing thread could be paused while data from the file is paged
    // into memory. Worse, a corrupted filesystem could cause a SEGV within the
    // program instead of just an I/O error.
    READ_ONLY,

    // This provides read/write access to a file and must be used with care of
    // the additional subtleties involved in doing so. Though the OS will do
    // the writing of data on its own time, too many dirty pages can cause
    // the OS to pause the thread while it writes them out. The pause can
    // be as much as 1s on some systems.
    READ_WRITE,

    // This provides read/write access to the mapped file contents as above, but
    // applies a copy-on-write policy such that no writes are carried through to
    // the underlying file.
    READ_WRITE_COPY,

    // This provides read/write access but with the ability to write beyond
    // the end of the existing file up to a maximum size specified as the
    // "region". Depending on the OS, the file may or may not be immediately
    // extended to the maximum size though it won't be loaded in RAM until
    // needed. Note, however, that the maximum size will still be reserved
    // in the process address space.
    READ_WRITE_EXTEND,

#if BUILDFLAG(IS_WIN)
    // This provides read access, but as executable code used for prefetching
    // DLLs into RAM to avoid inefficient hard fault patterns such as during
    // process startup. The accessing thread could be paused while data from
    // the file is read into memory (if needed).
    READ_CODE_IMAGE,
#endif
  };

  // The default constructor sets all members to invalid/null values.
  MemoryMappedFile();
  MemoryMappedFile(const MemoryMappedFile&) = delete;
  MemoryMappedFile& operator=(const MemoryMappedFile&) = delete;
  ~MemoryMappedFile();

  // Used to hold information about a region [offset + size] of a file.
  struct BASE_EXPORT Region {
    static const Region kWholeFile;

    friend bool operator==(const Region&, const Region&) = default;

    // Start of the region (measured in bytes from the beginning of the file).
    int64_t offset;

    // Length of the region in bytes.
    size_t size;
  };

  // Opens an existing file and maps it into memory. |access| can be read-only
  // or read/write but not read/write+extend. If this object already points
  // to a valid memory mapped file then this method will fail and return
  // false. If it cannot open the file, the file does not exist, or the
  // memory mapping fails, it will return false.
  [[nodiscard]] bool Initialize(const FilePath& file_name, Access access);
  [[nodiscard]] bool Initialize(const FilePath& file_name) {
    return Initialize(file_name, READ_ONLY);
  }

  // As above, but works with an already-opened file. |access| can be read-only
  // or read/write but not read/write+extend. MemoryMappedFile takes ownership
  // of |file| and closes it when done. |file| must have been opened with
  // permissions suitable for |access|. If the memory mapping fails, it will
  // return false.
  [[nodiscard]] bool Initialize(File file, Access access);
  [[nodiscard]] bool Initialize(File file) {
    return Initialize(std::move(file), READ_ONLY);
  }

  // As above, but works with a region of an already-opened file. |access|
  // must not be READ_CODE_IMAGE. If READ_WRITE_EXTEND is specified then
  // |region| provides the maximum size of the file. If the memory mapping
  // fails, it return false.
  [[nodiscard]] bool Initialize(File file, const Region& region, Access access);
  [[nodiscard]] bool Initialize(File file, const Region& region) {
    return Initialize(std::move(file), region, READ_ONLY);
  }

  const uint8_t* data() const { return bytes_.data(); }
  uint8_t* data() { return bytes_.data(); }
  size_t length() const { return bytes_.size(); }

  span<const uint8_t> bytes() const { return bytes_; }
  span<uint8_t> mutable_bytes() { return bytes_; }

  // Is file_ a valid file handle that points to an open, memory mapped file?
  bool IsValid() const;

 private:
  // Given the arbitrarily aligned memory region [start, size], returns the
  // boundaries of the region aligned to the granularity specified by the OS,
  // (a page on Linux, ~32k on Windows) as follows:
  // - |aligned_start| is page aligned and <= |start|.
  // - |aligned_size| is a multiple of the VM granularity and >= |size|.
  // - |offset| is the displacement of |start| w.r.t |aligned_start|.
  static void CalculateVMAlignedBoundaries(int64_t start,
                                           size_t size,
                                           int64_t* aligned_start,
                                           size_t* aligned_size,
                                           int32_t* offset);

#if BUILDFLAG(IS_WIN)
  // Maps the executable file to memory, point `bytes_` to the memory range.
  // Return true on success.
  bool MapImageToMemory(Access access);
#endif

  // Map the file to memory, point `bytes_` to that memory address. Return true
  // on success, false on any kind of failure. This is a helper for
  // Initialize().
  bool MapFileRegionToMemory(const Region& region, Access access);

  // Closes all open handles.
  void CloseHandles();

  File file_;

  // RAW_PTR_EXCLUSION: Never allocated by PartitionAlloc (always mmap'ed), so
  // there is no benefit to using a raw_span, only cost.
  RAW_PTR_EXCLUSION span<uint8_t> bytes_;

#if BUILDFLAG(IS_WIN)
  win::ScopedHandle file_mapping_;
#endif
};

}  // namespace base

#endif  // BASE_FILES_MEMORY_MAPPED_FILE_H_
