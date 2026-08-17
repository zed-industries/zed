// Copyright 2015 The Bazel Authors. All rights reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//    http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// zip.h -- .zip (.jar) file reading/writing routines.
//
// This file specifies the interface to use the ZIP implementation of ijar.
//

#ifndef INCLUDED_THIRD_PARTY_IJAR_ZIP_H
#define INCLUDED_THIRD_PARTY_IJAR_ZIP_H

#include <sys/stat.h>

#include "third_party/ijar/common.h"

namespace devtools_ijar {

// Tells if this is a directory entry from the mode. This method
// is safer than zipattr_to_mode(attr) & S_IFDIR because the unix
// mode might not be set in DOS zip files.
inline bool zipattr_is_dir(u4 attr) { return (attr & 0x10) != 0; }

// Convert a ZIP file attribute to a Unix file permission mask.
inline mode_t zipattr_to_perm(u4 attr) {
  return ((mode_t)((attr >> 16) & 0777));
}

//
// Class interface for building ZIP files
//
class ZipBuilder {
 public:
  virtual ~ZipBuilder() {}

  // Returns the text for the last error, or null on no last error.
  virtual const char* GetError() = 0;

  // Add a new file to the ZIP, the file will have path "filename"
  // and external attributes "attr". This function returns a pointer
  // to a memory buffer to write the data of the file into. This buffer
  // is owned by ZipBuilder and should not be free'd by the caller. The
  // file length is then specified when the files is finished written
  // using the FinishFile(size_t) function.
  // On failure, returns NULL and GetError() will return an non-empty message.
  virtual u1* NewFile(const char* filename, const u4 attr) = 0;

  // Finish writing a file and specify its length. After calling this method
  // one should not reuse the pointer given by NewFile. The file can be
  // compressed using the deflate algorithm by setting `compress` to true.
  // By default, CRC32 are not computed as java tooling doesn't care, but
  // computing it can be activated by setting `compute_crc` to true.
  // On failure, returns -1 and GetError() will return an non-empty message.
  virtual int FinishFile(size_t filelength,
                         bool compress = false,
                         bool compute_crc = false) = 0;

  // Write an empty file, it is equivalent to:
  //   NewFile(filename, 0);
  //   FinishFile(0);
  // On failure, returns -1 and GetError() will return an non-empty message.
  virtual int WriteEmptyFile(const char* filename) = 0;

  // Finish writing the ZIP file. This method can be called only once
  // (subsequent calls will do nothing) and none of
  // NewFile/FinishFile/WriteEmptyFile should be called after calling Finish. If
  // this method was not called when the object is destroyed, it will be called.
  // It is here as a convenience to get information on the final generated ZIP
  // file.
  // On failure, returns -1 and GetError() will return an non-empty message.
  virtual int Finish() = 0;

  // Get the current size of the ZIP file. This size will not be matching the
  // final ZIP file until Finish() has been called because Finish() is actually
  // writing the central directory of the ZIP File.
  virtual size_t GetSize() = 0;

  // Returns the current number of files stored in the ZIP.
  virtual int GetNumberFiles() = 0;

  // Create a new ZipBuilder writing the file zip_file and the size of the
  // output will be at most estimated_size. Use ZipBuilder::EstimateSize() or
  // ZipExtractor::CalculateOuputLength() to have an estimated_size depending on
  // a list of file to store.
  // On failure, returns NULL. Refer to errno for error code.
  static ZipBuilder* Create(const char* zip_file, size_t estimated_size);

  // Estimate the maximum size of the ZIP files containing files in the "files"
  // null-terminated array.
  // Returns 0 on error.
  static u8 EstimateSize(char const* const* files, char const* const* zip_paths,
                         int nb_entries);
};

//
// An abstract class to process data from a ZipExtractor.
// Derive from this class if you wish to process data from a ZipExtractor.
//
class ZipExtractorProcessor {
 public:
  virtual ~ZipExtractorProcessor() {}

  // Tells whether to skip or process the file "filename". "attr" is the
  // external file attributes and can be converted to unix mode using the
  // zipattr_to_mode() function. This method is suppoed to returns true
  // if the file should be processed and false if it should be skipped.
  virtual bool Accept(const char* filename, const u4 attr) = 0;

  // Process a file accepted by Accept. The file "filename" has external
  // attributes "attr" and length "size". The file content is accessible
  // in the buffer pointed by "data".
  virtual void Process(const char* filename, const u4 attr,
                       const u1* data, const size_t size) = 0;
};

//
// Class interface for reading ZIP files
//
class ZipExtractor {
 public:
  virtual ~ZipExtractor() {}

  // Returns the text for the last error, or null on no last error.
  virtual const char* GetError() = 0;

  // Process the next files, returns false if the end of ZIP file has been
  // reached. The processor provided by the Create method will be called
  // if a file is encountered. If false is returned, check the return value
  // of GetError() for potential errors.
  virtual bool ProcessNext() = 0;

  // Process the all files, returns -1 on error (GetError() will be populated
  // on error).
  virtual int ProcessAll();

  // Reset the file pointer to the beginning.
  virtual void Reset() = 0;

  // Return the size of the ZIP file.
  virtual size_t GetSize() = 0;

  // Return the size of the resulting zip file by keeping only file
  // accepted by the processor and storing them uncompressed. This
  // method can be used to create a ZipBuilder for storing a subset
  // of the input files.
  // On error, 0 is returned and GetError() returns a non-empty message.
  virtual u8 CalculateOutputLength() = 0;

  // Create a ZipExtractor that extract the zip file "filename" and process
  // it with "processor".
  // On error, a null pointer is returned and the value of errno should be
  // checked.
  static ZipExtractor* Create(const char* filename,
                              ZipExtractorProcessor *processor);
};

}  // namespace devtools_ijar

#endif  // INCLUDED_THIRD_PARTY_IJAR_ZIP_H
