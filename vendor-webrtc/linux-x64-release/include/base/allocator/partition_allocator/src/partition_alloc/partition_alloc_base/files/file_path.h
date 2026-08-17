// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// FilePath is a container for pathnames stored in a platform's native string
// type, providing containers for manipulation in according with the
// platform's conventions for pathnames.  It supports the following path
// types:
//
//                   POSIX            Windows
//                   ---------------  ----------------------------------
// Fundamental type  char[]           wchar_t[]
// Encoding          unspecified*     UTF-16
// Separator         /                \, tolerant of /
// Drive letters     no               case-insensitive A-Z followed by :
// Alternate root    // (surprise!)   \\ (2 Separators), for UNC paths
//
// * The encoding need not be specified on POSIX systems, although some
//   POSIX-compliant systems do specify an encoding.  Mac OS X uses UTF-8.
//   Chrome OS also uses UTF-8.
//   Linux does not specify an encoding, but in practice, the locale's
//   character set may be used.
//
// For more arcane bits of path trivia, see below.
//
// FilePath objects are intended to be used anywhere paths are.  An
// application may pass FilePath objects around internally, masking the
// underlying differences between systems, only differing in implementation
// where interfacing directly with the system.  For example, a single
// OpenFile(const FilePath &) function may be made available, allowing all
// callers to operate without regard to the underlying implementation.  On
// POSIX-like platforms, OpenFile might wrap fopen, and on Windows, it might
// wrap _wfopen_s, perhaps both by calling file_path.value().c_str().  This
// allows each platform to pass pathnames around without requiring conversions
// between encodings, which has an impact on performance, but more imporantly,
// has an impact on correctness on platforms that do not have well-defined
// encodings for pathnames.
//
// Several methods are available to perform common operations on a FilePath
// object, such as determining the parent directory (DirName), isolating the
// final path component (BaseName), and appending a relative pathname string
// to an existing FilePath object (Append).  These methods are highly
// recommended over attempting to split and concatenate strings directly.
// These methods are based purely on string manipulation and knowledge of
// platform-specific pathname conventions, and do not consult the filesystem
// at all, making them safe to use without fear of blocking on I/O operations.
// These methods do not function as mutators but instead return distinct
// instances of FilePath objects, and are therefore safe to use on const
// objects.  The objects themselves are safe to share between threads.
//
// To aid in initialization of FilePath objects from string literals, a
// FILE_PATH_LITERAL macro is provided, which accounts for the difference
// between char[]-based pathnames on POSIX systems and wchar_t[]-based
// pathnames on Windows.
//
// As a precaution against premature truncation, paths can't contain NULs.
//
// Because a FilePath object should not be instantiated at the global scope,
// instead, use a FilePath::CharType[] and initialize it with
// FILE_PATH_LITERAL.  At runtime, a FilePath object can be created from the
// character array.  Example:
//
// | const FilePath::CharType kLogFileName[] = FILE_PATH_LITERAL("log.txt");
// |
// | void Function() {
// |   FilePath log_file_path(kLogFileName);
// |   [...]
// | }
//
// WARNING: FilePaths should ALWAYS be displayed with LTR directionality, even
// when the UI language is RTL. This means you always need to pass filepaths
// through base::i18n::WrapPathWithLTRFormatting() before displaying it in the
// RTL UI.
//
// This is a very common source of bugs, please try to keep this in mind.
//
// ARCANE BITS OF PATH TRIVIA
//
//  - A double leading slash is actually part of the POSIX standard.  Systems
//    are allowed to treat // as an alternate root, as Windows does for UNC
//    (network share) paths.  Most POSIX systems don't do anything special
//    with two leading slashes, but FilePath handles this case properly
//    in case it ever comes across such a system.  FilePath needs this support
//    for Windows UNC paths, anyway.
//    References:
//    The Open Group Base Specifications Issue 7, sections 3.267 ("Pathname")
//    and 4.12 ("Pathname Resolution"), available at:
//    http://www.opengroup.org/onlinepubs/9699919799/basedefs/V1_chap03.html#tag_03_267
//    http://www.opengroup.org/onlinepubs/9699919799/basedefs/V1_chap04.html#tag_04_12
//
//  - Windows treats c:\\ the same way it treats \\.  This was intended to
//    allow older applications that require drive letters to support UNC paths
//    like \\server\share\path, by permitting c:\\server\share\path as an
//    equivalent.  Since the OS treats these paths specially, FilePath needs
//    to do the same.  Since Windows can use either / or \ as the separator,
//    FilePath treats c://, c:\\, //, and \\ all equivalently.
//    Reference:
//    The Old New Thing, "Why is a drive letter permitted in front of UNC
//    paths (sometimes)?", available at:
//    http://blogs.msdn.com/oldnewthing/archive/2005/11/22/495740.aspx

#ifndef PARTITION_ALLOC_PARTITION_ALLOC_BASE_FILES_FILE_PATH_H_
#define PARTITION_ALLOC_PARTITION_ALLOC_BASE_FILES_FILE_PATH_H_

#include <cstddef>
#include <iosfwd>
#include <string>

#include "partition_alloc/build_config.h"
#include "partition_alloc/partition_alloc_base/component_export.h"

// Windows-style drive letter support and pathname separator characters can be
// enabled and disabled independently, to aid testing.  These #defines are
// here so that the same setting can be used in both the implementation and
// in the unit test.
#if PA_BUILDFLAG(IS_WIN)
#define PA_FILE_PATH_USES_DRIVE_LETTERS
#define PA_FILE_PATH_USES_WIN_SEPARATORS
#endif  // PA_BUILDFLAG(IS_WIN)

// Macros for string literal initialization of FilePath::CharType[].
#if PA_BUILDFLAG(IS_WIN)
#define PA_FILE_PATH_LITERAL(x) L##x
#elif PA_BUILDFLAG(IS_POSIX) || PA_BUILDFLAG(IS_FUCHSIA)
#define PA_FILE_PATH_LITERAL(x) x
#endif  // PA_BUILDFLAG(IS_WIN)

namespace partition_alloc::internal::base {

// An abstraction to isolate users from the differences between native
// pathnames on different platforms.
class PA_COMPONENT_EXPORT(PARTITION_ALLOC_BASE) FilePath {
 public:
#if PA_BUILDFLAG(IS_WIN)
  // On Windows, for Unicode-aware applications, native pathnames are wchar_t
  // arrays encoded in UTF-16.
  typedef std::wstring StringType;
#elif PA_BUILDFLAG(IS_POSIX) || PA_BUILDFLAG(IS_FUCHSIA)
  // On most platforms, native pathnames are char arrays, and the encoding
  // may or may not be specified.  On Mac OS X, native pathnames are encoded
  // in UTF-8.
  typedef std::string StringType;
#endif  // PA_BUILDFLAG(IS_WIN)

  typedef StringType::value_type CharType;

  // Null-terminated array of separators used to separate components in paths.
  // Each character in this array is a valid separator, but kSeparators[0] is
  // treated as the canonical separator and is used when composing pathnames.
  static constexpr CharType kSeparators[] =
#if defined(PA_FILE_PATH_USES_WIN_SEPARATORS)
      PA_FILE_PATH_LITERAL("\\/");
#else   // PA_FILE_PATH_USES_WIN_SEPARATORS
      PA_FILE_PATH_LITERAL("/");
#endif  // PA_FILE_PATH_USES_WIN_SEPARATORS

  // std::size(kSeparators), i.e., the number of separators in kSeparators plus
  // one (the null terminator at the end of kSeparators).
  static constexpr size_t kSeparatorsLength = std::size(kSeparators);

  // The special path component meaning "this directory."
  static constexpr CharType kCurrentDirectory[] = PA_FILE_PATH_LITERAL(".");

  // The special path component meaning "the parent directory."
  static constexpr CharType kParentDirectory[] = PA_FILE_PATH_LITERAL("..");

  // The character used to identify a file extension.
  static constexpr CharType kExtensionSeparator = PA_FILE_PATH_LITERAL('.');

  FilePath();
  FilePath(const FilePath& that);
  explicit FilePath(const StringType& that);
  ~FilePath();
  FilePath& operator=(const FilePath& that);

  // Constructs FilePath with the contents of |that|, which is left in valid but
  // unspecified state.
  FilePath(FilePath&& that) noexcept;
  // Replaces the contents with those of |that|, which is left in valid but
  // unspecified state.
  FilePath& operator=(FilePath&& that) noexcept;

  // Required for some STL containers and operations
  bool operator<(const FilePath& that) const { return path_ < that.path_; }

  const StringType& value() const { return path_; }

  [[nodiscard]] bool empty() const { return path_.empty(); }

  void clear() { path_.clear(); }

  // Returns true if |character| is in kSeparators.
  static bool IsSeparator(CharType character);

  // Returns a FilePath by appending a separator and the supplied path
  // component to this object's path.  Append takes care to avoid adding
  // excessive separators if this object's path already ends with a separator.
  // If this object's path is kCurrentDirectory, a new FilePath corresponding
  // only to |component| is returned.  |component| must be a relative path;
  // it is an error to pass an absolute path.
  [[nodiscard]] FilePath Append(const FilePath& component) const;
  [[nodiscard]] FilePath Append(const StringType& component) const;

 private:
  // Remove trailing separators from this object.  If the path is absolute, it
  // will never be stripped any more than to refer to the absolute root
  // directory, so "////" will become "/", not "".  A leading pair of
  // separators is never stripped, to support alternate roots.  This is used to
  // support UNC paths on Windows.
  void StripTrailingSeparatorsInternal();

  StringType path_;
};

}  // namespace partition_alloc::internal::base

namespace std {

template <>
struct hash<::partition_alloc::internal::base::FilePath> {
  typedef ::partition_alloc::internal::base::FilePath argument_type;
  typedef std::size_t result_type;
  result_type operator()(argument_type const& f) const {
    return hash<::partition_alloc::internal::base::FilePath::StringType>()(
        f.value());
  }
};

}  // namespace std

#endif  // PARTITION_ALLOC_PARTITION_ALLOC_BASE_FILES_FILE_PATH_H_
