// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_UNRAR_GOOGLE_UNRAR_WRAPPER_H_
#define THIRD_PARTY_UNRAR_GOOGLE_UNRAR_WRAPPER_H_

#include "base/files/file.h"
#include "base/files/file_path.h"
#include "base/files/platform_file.h"
#include "base/memory/scoped_refptr.h"

// Forward declare the unrar symbols needed for extraction, so users of
// RarReader don't need all the symbols from unrar.
class Archive;
class CmdExtract;
class CommandData;

namespace third_party_unrar {

// This class is used for extracting RAR files, one entry at a time.
class RarReader {
 public:
  struct EntryInfo {
    // The relative path of this entry, within the archive.
    base::FilePath file_path;

    // Whether the entry is a directory or a file.
    bool is_directory;

    // Whether the entry has encrypted contents.
    bool is_encrypted;

    // The actual size of the entry.
    size_t file_size;

    // Whether the contents are valid
    bool contents_valid;
  };

  RarReader();
  ~RarReader();

  // Opens the RAR archive in |rar_file|, and uses |temp_file| for extracting
  // each entry.
  bool Open(base::File rar_file, base::File temp_file);

  // Extracts the next entry in the RAR archive. Returns true on success and
  // updates the information in |current_entry()|.
  bool ExtractNextEntry();

  // Returns the EntryInfo for the most recently extracted entry in the RAR
  // archive.
  const EntryInfo& current_entry() { return current_entry_; }

  void SetPassword(const std::string& password);

  bool HeadersEncrypted() const;
  bool HeaderDecryptionFailed() const;

 private:
  // The temporary file used for extracting each entry. This allows RAR
  // extraction to safely occur within a sandbox.
  base::File temp_file_;

  // The RAR archive being extracted.
  base::File rar_file_;

  // Information for the current entry in the RAR archive.
  EntryInfo current_entry_;

  // Unrar data structures needed for extraction.
  std::unique_ptr<Archive> archive_;
  std::unique_ptr<CmdExtract> extractor_;
  std::unique_ptr<CommandData> command_;

  // Password used for encrypted entries.
  std::string password_;
};

}  // namespace third_party_unrar

#endif  // THIRD_PARTY_UNRAR_GOOGLE_UNRAR_WRAPPER_H_
