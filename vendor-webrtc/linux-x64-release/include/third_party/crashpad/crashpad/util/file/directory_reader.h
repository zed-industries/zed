// Copyright 2017 The Crashpad Authors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef CRASHPAD_UTIL_FILE_DIRECTORY_READER_H_
#define CRASHPAD_UTIL_FILE_DIRECTORY_READER_H_

#include "base/files/file_path.h"
#include "build/build_config.h"

#if BUILDFLAG(IS_POSIX)
#include "util/posix/scoped_dir.h"
#elif BUILDFLAG(IS_WIN)
#include <windows.h>

#include "util/win/scoped_handle.h"
#endif  // BUILDFLAG(IS_POSIX)

namespace crashpad {

//! \brief Iterates over the file and directory names in a directory.
//!
//! The names enumerated are relative to the specified directory and do not
//! include ".", "..", or files and directories in subdirectories.
class DirectoryReader {
 public:
  //! \brief The result of a call to NextFile().
  enum class Result {
    //! \brief An error occurred and a message was logged.
    kError = -1,

    //! \brief A file was found.
    kSuccess,

    //! \brief No more files were found.
    kNoMoreFiles,
  };

  DirectoryReader();

  DirectoryReader(const DirectoryReader&) = delete;
  DirectoryReader& operator=(const DirectoryReader&) = delete;

  ~DirectoryReader();

  //! \brief Opens the directory specified by \a path for reading.
  //!
  //! \param[in] path The path to the directory to read.
  //! \return `true` on success. `false` on failure with a message logged.
  bool Open(const base::FilePath& path);

  //! \brief Advances the reader to the next file in the directory.
  //!
  //! \param[out] filename The filename of the next file.
  //! \return a #Result value. \a filename is only valid when Result::kSuccess
  //!     is returned. If Result::kError is returned, a message will be
  //!     logged.
  Result NextFile(base::FilePath* filename);

#if BUILDFLAG(IS_POSIX) || DOXYGEN
  //! \brief Returns the file descriptor associated with this reader, logging a
  //!     message and returning -1 on error.
  int DirectoryFD();
#endif

 private:
#if BUILDFLAG(IS_POSIX)
  ScopedDIR dir_;
#elif BUILDFLAG(IS_WIN)
  WIN32_FIND_DATA find_data_;
  ScopedSearchHANDLE handle_;
  bool first_entry_;
#endif  // BUILDFLAG(IS_POSIX)
};

}  // namespace crashpad

#endif  // CRASHPAD_UTIL_FILE_DIRECTORY_READER_H_
