// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TOOLS_BINARY_SIZE_LIBSUPERSIZE_VIEWER_CASPIAN_FILE_FORMAT_H_
#define TOOLS_BINARY_SIZE_LIBSUPERSIZE_VIEWER_CASPIAN_FILE_FORMAT_H_

#include <string>
#include <vector>

namespace caspian {

struct SizeInfo;

bool IsDiffSizeInfo(const char* file, unsigned long len);

// Parses a .sizediff, and writes out the two sparse SizeInfos it contains.
// Diffing still needs to be done on the result to obtain a DeltaSizeInfo.
// |file| is not const char* owing to strsep() usage.
void ParseDiffSizeInfo(char* file,
                       unsigned long len,
                       SizeInfo* before,
                       SizeInfo* after,
                       std::vector<std::string>* removed_sources,
                       std::vector<std::string>* added_sources);

// Parses a .size file.
void ParseSizeInfo(const char* gzipped, unsigned long len, SizeInfo* info);

}  // namespace caspian

#endif  // TOOLS_BINARY_SIZE_LIBSUPERSIZE_VIEWER_CASPIAN_FILE_FORMAT_H_
