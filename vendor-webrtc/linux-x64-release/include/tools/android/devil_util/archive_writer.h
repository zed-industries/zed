// Copyright 2025 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef TOOLS_ANDROID_DEVIL_UTIL_ARCHIVE_WRITER_H_
#define TOOLS_ANDROID_DEVIL_UTIL_ARCHIVE_WRITER_H_

#include <fstream>
#include <string>
#include <vector>

#include "archive_helper.h"

// This archive writer can be used to create an archive in a custom file format.
// This custom archive format starts with a fixed-length magic bytes which
// identify the file format being used. The archive then stores each member
// sequentially. Each member begins with a fixed-length field indicating the
// length of the file path (where numbers are in their base-10 representation
// and they are converted to ASCII to avoid issues related to endianness),
// followed by the variable-length file path itself. Next, a fixed-length field
// specifies the length of the file content, followed by the variable-length
// file content. The archive terminates with a file path length of 0; the
// archive reader can use this to tell whether it has reached the end of file.
class ArchiveWriter {
 public:
  struct ArchiveMember {
    std::string file_path_in_host;
    std::string file_path_in_archive;
  };

  explicit ArchiveWriter(std::vector<ArchiveMember> members);
  ~ArchiveWriter();

  // Create the next portion of the archive and write it to the output_buffer.
  // Repeatedly call this function to create the entire archive. Return the
  // number of bytes that have been actually written to the output_buffer.
  // The return value will be equal to output_buffer_size unless we have
  // finished creating the entire archive.
  size_t CreateArchiveStreaming(char* output_buffer, size_t output_buffer_size);

 private:
  // The number of bytes of the magic bytes that we have written to the archive.
  uint64_t magic_bytes_pos_ = 0;
  // Contain one entry for each member of the archive.
  // Each entry contains the file path of the member in the host machine, and
  // the file path of the member in the archive.
  std::vector<ArchiveMember> members_;
  // The index of the member that we are currently processing.
  size_t cur_member_index_ = 0;
  // The file path of the current member that should be stored in the archive.
  std::string cur_member_path_;
  // The content length of the current member that should be stored in archive.
  uint64_t cur_member_content_length_ = 0;
  // The input stream of the current member. We read the file contents of the
  // current member from this input stream.
  std::ifstream cur_member_ifstream_;
  // The number of bytes of path length that we have written to the archive.
  uint64_t cur_member_path_length_pos_ = 0;
  // The number of bytes of file path that we have written to the archive.
  uint64_t cur_member_path_pos_ = 0;
  // The number of bytes of content length that we have written to the archive.
  uint64_t cur_member_content_length_pos_ = 0;
  // The number of bytes of file content that we have written to the archive.
  uint64_t cur_member_content_pos_ = 0;
  // If true, then we have finished processing the current member.
  bool start_next_member_ = true;

  // Write the magic bytes that identify the file format being used.
  void WriteMagicBytes(char** output_buffer, size_t* output_buffer_size);
  // Write the path length of the current member to output_buffer if needed.
  void WriteCurMemberPathLength(char** output_buffer,
                                size_t* output_buffer_size);
  // Write the file path of the current member to output_buffer if needed.
  void WriteCurMemberPath(char** output_buffer, size_t* output_buffer_size);
  // Write the content length of the current member to output_buffer if needed.
  void WriteCurMemberContentLength(char** output_buffer,
                                   size_t* output_buffer_size);
  // Write the file content of the current member to output_buffer if needed.
  void WriteCurMemberContent(char** output_buffer, size_t* output_buffer_size);
};

#endif  // TOOLS_ANDROID_DEVIL_UTIL_ARCHIVE_WRITER_H_
