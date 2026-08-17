// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_I18N_FILE_UTIL_ICU_H_
#define BASE_I18N_FILE_UTIL_ICU_H_

// File utilities that use the ICU library go in this file.

#include <string>

#include "base/files/file_path.h"
#include "base/i18n/base_i18n_export.h"

namespace base {
namespace i18n {

// Returns true if file_name does not have any illegal character. The input
// param has the same restriction as that for ReplaceIllegalCharacters.
BASE_I18N_EXPORT bool IsFilenameLegal(const std::u16string& file_name);

// Replaces characters in |file_name| that are illegal for file names with
// |replace_char|. |file_name| must not be a full or relative path, but just the
// file name component (since slashes are considered illegal). Any leading or
// trailing whitespace or periods in |file_name| is also replaced with the
// |replace_char|, unless |replace_char| itself is a whitespace or period, in
// which case they are trimmed.
//
// Example:
//   "bad:file*name?.txt" will be turned into "bad_file_name_.txt" when
//   |replace_char| is '_'.
//
// If |replace_char| is a whitespace or period and |file_name| contains no legal
// characters, the result will be enclosed by '_'s.
// If |replace_char| is a whitespace or period and |file_name| contains no legal
// characters except a file extension, the result will have '_' prepended.
//
// Warning: Do not use this function as the sole means of sanitizing a filename.
//   While the resulting filename itself would be legal, it doesn't necessarily
//   mean that the file will behave safely. On Windows, certain reserved names
//   refer to devices rather than files (E.g. LPT1), and some filenames could be
//   interpreted as shell namespace extensions (E.g. Foo.{<GUID>}).
//
// On Windows, Chrome OS and Mac, the file system encoding is already known and
// parsed as UTF-8 and UTF-16 accordingly.
// On Linux, the file name will be parsed as UTF8.
// TODO(asanka): Move full filename sanitization logic here.
BASE_I18N_EXPORT void ReplaceIllegalCharactersInPath(
    FilePath::StringType* file_name,
    char replace_char);

// Compares two filenames using the current locale information. This can be
// used to sort directory listings. It behaves like "operator<" for use in
// std::sort.
BASE_I18N_EXPORT bool LocaleAwareCompareFilenames(const FilePath& a,
                                                  const FilePath& b);

// Calculates the canonical file-system representation of |file_name| base name.
// Modifies |file_name| in place. No-op if not on ChromeOS.
BASE_I18N_EXPORT void NormalizeFileNameEncoding(FilePath* file_name);

}  // namespace i18n
}  // namespace base

#endif  // BASE_I18N_FILE_UTIL_ICU_H_
