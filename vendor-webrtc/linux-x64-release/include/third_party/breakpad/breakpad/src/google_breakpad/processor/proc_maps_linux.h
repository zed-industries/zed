// Copyright 2013 Google LLC
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_DEBUG_PROC_MAPS_LINUX_H_
#define BASE_DEBUG_PROC_MAPS_LINUX_H_

#include <string>
#include <vector>

#include "common/using_std_string.h"
#include "google_breakpad/common/breakpad_types.h"

namespace google_breakpad {

// Describes a region of mapped memory and the path of the file mapped.
struct MappedMemoryRegion {
  enum Permission {
    READ = 1 << 0,
    WRITE = 1 << 1,
    EXECUTE = 1 << 2,
    PRIVATE = 1 << 3,  // If set, region is private, otherwise it is shared.
  };

  // The address range [start,end) of mapped memory.
  uint64_t start;
  uint64_t end;

  // Byte offset into |path| of the range mapped into memory.
  uint64_t offset;

  // Bitmask of read/write/execute/private/shared permissions.
  uint8_t permissions;

  // Major and minor devices.
  uint8_t major_device;
  uint8_t minor_device;

  // Value of the inode.
  uint64_t inode;

  // Name of the file mapped into memory.
  //
  // NOTE: path names aren't guaranteed to point at valid files. For example,
  // "[heap]" and "[stack]" are used to represent the location of the process'
  // heap and stack, respectively.
  string path;

  // The line from /proc/<pid>/maps that this struct represents.
  string line;
};

// Parses /proc/<pid>/maps input data and stores in |regions|. Returns true
// and updates |regions| if and only if all of |input| was successfully parsed.
bool ParseProcMaps(const string& input,
                   std::vector<MappedMemoryRegion>* regions);

}  // namespace google_breakpad

#endif  // BASE_DEBUG_PROC_MAPS_LINUX_H_
