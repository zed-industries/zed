// Copyright 2013 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_DEBUG_PROC_MAPS_LINUX_H_
#define BASE_DEBUG_PROC_MAPS_LINUX_H_

#include <stdint.h>

#include <optional>
#include <string>
#include <vector>

#include "base/base_export.h"

namespace base::debug {

// Describes a region of mapped memory and the path of the file mapped.
struct BASE_EXPORT MappedMemoryRegion {
  enum Permission {
    READ = 1 << 0,
    WRITE = 1 << 1,
    EXECUTE = 1 << 2,
    PRIVATE = 1 << 3,  // If set, region is private, otherwise it is shared.
  };

  MappedMemoryRegion();
  MappedMemoryRegion(const MappedMemoryRegion&);
  MappedMemoryRegion(MappedMemoryRegion&&) noexcept;
  MappedMemoryRegion& operator=(MappedMemoryRegion&);
  MappedMemoryRegion& operator=(MappedMemoryRegion&&) noexcept;

  // The address range [start,end) of mapped memory.
  uintptr_t start;
  uintptr_t end;

  // Byte offset into |path| of the range mapped into memory.
  unsigned long long offset;

  // Image base, if this mapping corresponds to an ELF image.
  uintptr_t base;

  // Bitmask of read/write/execute/private/shared permissions.
  uint8_t permissions;

  // Major and minor device numbers for the region.
  uint8_t dev_major;
  uint8_t dev_minor;

  // Inode for the region.
  long inode;

  // Name of the file mapped into memory.
  //
  // NOTE: path names aren't guaranteed to point at valid files. For example,
  // "[heap]" and "[stack]" are used to represent the location of the process'
  // heap and stack, respectively.
  std::string path;
};

// Reads the data from /proc/self/maps and stores the result in |proc_maps|.
// Returns true if successful, false otherwise.
//
// There is *NO* guarantee that the resulting contents will be free of
// duplicates or even contain valid entries by time the method returns.
//
//
// THE GORY DETAILS
//
// Did you know it's next-to-impossible to atomically read the whole contents
// of /proc/<pid>/maps? You would think that if we passed in a large-enough
// buffer to read() that It Should Just Work(tm), but sadly that's not the case.
//
// Linux's procfs uses seq_file [1] for handling iteration, text formatting,
// and dealing with resulting data that is larger than the size of a page. That
// last bit is especially important because it means that seq_file will never
// return more than the size of a page in a single call to read().
//
// Unfortunately for a program like Chrome the size of /proc/self/maps is
// larger than the size of page so we're forced to call read() multiple times.
// If the virtual memory table changed in any way between calls to read() (e.g.,
// a different thread calling mprotect()), it can make seq_file generate
// duplicate entries or skip entries.
//
// Even if seq_file was changed to keep flushing the contents of its page-sized
// buffer to the usermode buffer inside a single call to read(), it has to
// release its lock on the virtual memory table to handle page faults while
// copying data to usermode. This puts us in the same situation where the table
// can change while we're copying data.
//
// Alternatives such as fork()-and-suspend-the-parent-while-child-reads were
// attempted, but they present more subtle problems than it's worth. Depending
// on your use case your best bet may be to read /proc/<pid>/maps prior to
// starting other threads.
//
// [1] http://kernelnewbies.org/Documents/SeqFileHowTo
BASE_EXPORT bool ReadProcMaps(std::string* proc_maps);

// Parses /proc/<pid>/maps input data and stores in |regions|. Returns true
// and updates |regions| if and only if all of |input| was successfully parsed.
BASE_EXPORT bool ParseProcMaps(const std::string& input,
                               std::vector<MappedMemoryRegion>* regions);

struct SmapsRollup {
  size_t rss = 0;
  size_t pss = 0;
  size_t pss_anon = 0;
  size_t pss_file = 0;
  size_t pss_shmem = 0;
  size_t private_dirty = 0;
  size_t swap = 0;
  size_t swap_pss = 0;
};

// Attempts to read /proc/self/smaps_rollup. Returns nullopt on error.
BASE_EXPORT std::optional<SmapsRollup> ReadAndParseSmapsRollup();

// |smaps_rollup| should be the result of reading /proc/*/smaps_rollup.
BASE_EXPORT std::optional<SmapsRollup> ParseSmapsRollupForTesting(
    const std::string& smaps_rollup);

}  // namespace base::debug

#endif  // BASE_DEBUG_PROC_MAPS_LINUX_H_
