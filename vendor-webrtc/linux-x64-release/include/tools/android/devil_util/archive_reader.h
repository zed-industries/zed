// Copyright 2025 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TOOLS_ANDROID_DEVIL_UTIL_ARCHIVE_READER_H_
#define TOOLS_ANDROID_DEVIL_UTIL_ARCHIVE_READER_H_

#include <fstream>
#include <string>

#include "archive_helper.h"

class ArchiveReader {
 public:
  ArchiveReader();
  ~ArchiveReader();

  // Extract the next portion of the input archive. Repeatedly call this
  // function to extract the entirety of the input archive. Return true if the
  // entirety of the input archive has been processed, and false otherwise.
  bool ExtractArchiveStreaming(char* input_buffer, size_t input_buffer_size);

 private:
  // A buffer containing the magic bytes that we have read from the archive.
  char magic_bytes_buffer_[kMagicBytesLength];
  // The position of the magic bytes buffer that we are currently at.
  // Anything in the buffer that is before this position have been populated.
  uint64_t magic_bytes_pos_ = 0;
  // The actual magic bytes that we have parsed from the magic bytes buffer.
  std::string magic_bytes_;
  // A buffer containing the path length that we have read from the archive.
  char cur_member_path_length_buffer_[kPathLengthSize];
  // The position of the path length buffer that we are currently at.
  uint64_t cur_member_path_length_pos_ = 0;
  // The actual path length that we have parsed from the path length buffer.
  uint64_t cur_member_path_length_ = 0;
  // A buffer containing the file path that we have read from the archive.
  char cur_member_path_buffer_[kMaxPathLength];
  // The position of the file path buffer that we are currently at.
  uint64_t cur_member_path_pos_ = 0;
  // The actual file path that we have parsed from the file path buffer.
  std::string cur_member_path_;
  // A buffer containing the content length that we have read from the archive.
  char cur_member_content_length_buffer_[kContentLengthSize];
  // The position of the content length buffer that we are currently at.
  uint64_t cur_member_content_length_pos_ = 0;
  // The actual content length that we have parsed from the buffer.
  uint64_t cur_member_content_length_ = 0;
  // The output stream of the current member. We write the file contents of the
  // current member to this output stream.
  std::ofstream cur_member_ofstream_;
  // The number of bytes of file contents that we have written to output stream.
  uint64_t cur_member_content_pos_ = 0;
  // If true, then we have finished processing the current member.
  bool start_new_member_ = true;

  // Read the magic bytes at the beginning of the archive.
  void ReadMagicBytes(char** input_buffer, size_t* input_buffer_size);
  // Read the path length of the current member if needed.
  void ReadCurMemberPathLength(char** input_buffer, size_t* input_buffer_size);
  // Read the file path of the current member if needed.
  void ReadCurMemberPath(char** input_buffer, size_t* input_buffer_size);
  // Read the content length of the current member if needed.
  void ReadCurMemberContentLength(char** input_buffer,
                                  size_t* input_buffer_size);
  // Read the file content of the current member if needed.
  void ReadCurMemberContent(char** input_buffer, size_t* input_buffer_size);
};

#endif  // TOOLS_ANDROID_DEVIL_UTIL_ARCHIVE_READER_H_
