// Copyright 2025 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TOOLS_ANDROID_DEVIL_UTIL_ARCHIVE_HELPER_H_
#define TOOLS_ANDROID_DEVIL_UTIL_ARCHIVE_HELPER_H_

#include <cstdint>
#include <string_view>

// Every custom archive file created by the devil_util program starts with a
// fixed-length "magic bytes" to identify the file format being used.
inline constexpr uint64_t kMagicBytesLength = 50;
inline constexpr std::string_view kDevilUtilArchiveV1MagicBytes =
    "DEVIL_UTIL_ARCHIVE_V1";

// Using 4 bytes to store the length of the file path allows us to have file
// paths that contain up to 9999 characters.
inline constexpr uint64_t kPathLengthSize = 4;

// Using 12 bytes to store the size of the file content allows us to have files
// that are up to one terabyte in size.
inline constexpr uint64_t kContentLengthSize = 12;

// Given that file paths can contain up to 9999 characters, the max file path
// length is 10000 if we include the null terminator.
inline constexpr uint64_t kMaxPathLength = 10000;

#endif  // TOOLS_ANDROID_DEVIL_UTIL_ARCHIVE_HELPER_H_
