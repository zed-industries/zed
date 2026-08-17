// Copyright 2015 The Crashpad Authors
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

#ifndef CRASHPAD_SNAPSHOT_MINIDUMP_MINIDUMP_STRING_READER_H_
#define CRASHPAD_SNAPSHOT_MINIDUMP_MINIDUMP_STRING_READER_H_

#include <windows.h>
#include <dbghelp.h>

#include <string>

#include "base/strings/utf_string_conversions.h"
#include "minidump/minidump_extensions.h"
#include "util/file/file_reader.h"

namespace crashpad {
namespace internal {

//! \brief Reads a MinidumpUTF8String from a minidump file at offset \a rva in
//!     \a file_reader, and returns it in \a string.
//!
//! \return `true` on success, with \a string set. `false` on failure, with a
//!     message logged.
bool ReadMinidumpUTF8String(FileReaderInterface* file_reader,
                            RVA rva,
                            std::string* string);

//! \brief 64-bit specialization of ReadMinidumpUTF8String.
bool ReadMinidumpUTF8String(FileReaderInterface* file_reader,
                            RVA64 rva,
                            std::string* string);

//! \brief Reads a MinidumpUTF16String from a minidump file at offset \a rva in
//!     \a file_reader, and returns it in \a string.
//!
//! \return `true` on success, with \a string set. `false` on failure, with a
//!     message logged.
bool ReadMinidumpUTF16String(FileReaderInterface* file_reader,
                             RVA rva,
                             std::u16string* string);

//! \brief 64-bit specialization of ReadMinidumpUTF16String.
bool ReadMinidumpUTF16String(FileReaderInterface* file_reader,
                             RVA64 rva,
                             std::u16string* string);

//! \brief Reads a MinidumpUTF16String from a minidump file at offset \a rva in
//!     \a file_reader, and returns it in \a string.
//!
//! \return `true` on success, with \a string set. `false` on failure, with a
//!     message logged.
bool ReadMinidumpUTF16String(FileReaderInterface* file_reader,
                             RVA rva,
                             std::string* string);

//! \brief 64-bit specialization of ReadMinidumpUTF16String.
bool ReadMinidumpUTF16String(FileReaderInterface* file_reader,
                             RVA64 rva,
                             std::string* string);

}  // namespace internal
}  // namespace crashpad

#endif  // CRASHPAD_SNAPSHOT_MINIDUMP_MINIDUMP_STRING_READER_H_
