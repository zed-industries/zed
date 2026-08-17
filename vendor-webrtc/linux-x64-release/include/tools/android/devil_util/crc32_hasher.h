// Copyright 2025 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TOOLS_ANDROID_DEVIL_UTIL_CRC32_HASHER_H_
#define TOOLS_ANDROID_DEVIL_UTIL_CRC32_HASHER_H_

#include <optional>
#include <string>
#include <string_view>
#include <vector>

inline constexpr std::string_view kFilePathDelimiter = ":";

class Crc32Hasher {
 public:
  Crc32Hasher();
  ~Crc32Hasher();

  // Given a list of kFilePathDelimiter-separated file paths, return the file
  // paths as a vector.
  std::vector<std::string> ParseFileList(const std::string& combined_paths);
  // If there is no file at the given path, return std::nullopt.
  // Otherwise, return the checksum obtained by hashing the file at that path.
  std::optional<uint32_t> HashFile(const std::string& path);
};

#endif  // TOOLS_ANDROID_DEVIL_UTIL_CRC32_HASHER_H_
