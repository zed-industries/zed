// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// This file contains utility functions for dealing with the local
// filesystem.

#ifndef BASE_FILES_FILE_UTIL_H_
#define BASE_FILES_FILE_UTIL_H_

#include <stddef.h>
#include <stdint.h>
#include <stdio.h>

#include <limits>
#include <optional>
#include <set>
#include <string>
#include <string_view>
#include <vector>

#include "base/base_export.h"
#include "base/containers/span.h"
#include "base/files/file.h"
#include "base/files/file_path.h"
#include "base/files/scoped_file.h"
#include "base/functional/callback.h"
#include "base/strings/cstring_view.h"
#include "base/types/pass_key.h"
#include "build/build_config.h"

#if BUILDFLAG(IS_WIN)
#include "base/win/windows_types.h"
#elif BUILDFLAG(IS_POSIX) || BUILDFLAG(IS_FUCHSIA)
#include <sys/stat.h>
#include <unistd.h>

#include "base/posix/eintr_wrapper.h"
#endif

namespace content::internal {
class ChildProcessLauncherHelper;
}  // namespace content::internal

namespace base {

class Environment;
class Time;

#if BUILDFLAG(IS_WIN)
class PreventExecuteMappingClasses {
 public:
  using PassKey = base::PassKey<PreventExecuteMappingClasses>;

 private:
  static PassKey GetPassKey() { return PassKey(); }

  // Allowed to open log files in arbitrary locations.
  friend class content::internal::ChildProcessLauncherHelper;
};
#endif

//-----------------------------------------------------------------------------
// Functions that involve filesystem access or modification:

// Returns an absolute version of a relative path. Returns an empty path on
// error. This function can result in I/O so it can be slow.
//
// On POSIX, this function calls realpath(), so:
// 1) it fails if the path does not exist.
// 2) it expands all symlink components of the path.
// 3) it removes "." and ".." directory components.
BASE_EXPORT FilePath MakeAbsoluteFilePath(const FilePath& input);

#if BUILDFLAG(IS_POSIX)
// Prepends the current working directory if `input` is not already absolute,
// and removes "/./" and "/../" This is similar to MakeAbsoluteFilePath(), but
// MakeAbsoluteFilePath() expands all symlinks in the path and this does not.
//
// This may block if `input` is a relative path, when calling
// GetCurrentDirectory().
//
// This doesn't return std::nullopt unless (1) `input` is empty, or (2)
// `input` is a relative path and GetCurrentDirectory() fails.
[[nodiscard]] BASE_EXPORT std::optional<FilePath>
MakeAbsoluteFilePathNoResolveSymbolicLinks(const FilePath& input);
#endif

// Returns the total number of bytes used by all the files under |root_path|.
// If the path does not exist the function returns 0.
//
// This function is implemented using the FileEnumerator class so it is not
// particularly speedy on any platform.
BASE_EXPORT int64_t ComputeDirectorySize(const FilePath& root_path);

// Deletes the given path, whether it's a file or a directory.
// If it's a directory, it's perfectly happy to delete all of the directory's
// contents, but it will not recursively delete subdirectories and their
// contents.
// Returns true if successful, false otherwise. It is considered successful to
// attempt to delete a file that does not exist.
//
// In POSIX environment and if |path| is a symbolic link, this deletes only
// the symlink. (even if the symlink points to a non-existent file)
BASE_EXPORT bool DeleteFile(const FilePath& path);

// Deletes the given path, whether it's a file or a directory.
// If it's a directory, it's perfectly happy to delete all of the
// directory's contents, including subdirectories and their contents.
// Returns true if successful, false otherwise. It is considered successful
// to attempt to delete a file that does not exist.
//
// In POSIX environment and if |path| is a symbolic link, this deletes only
// the symlink. (even if the symlink points to a non-existent file)
//
// WARNING: USING THIS EQUIVALENT TO "rm -rf", SO USE WITH CAUTION.
BASE_EXPORT bool DeletePathRecursively(const FilePath& path);

// Returns a closure that, when run on any sequence that allows blocking calls,
// will kick off a potentially asynchronous operation to delete `path`, whose
// behavior is similar to `DeleteFile()` and `DeletePathRecursively()`
// respectively.
//
// In contrast to `DeleteFile()` and `DeletePathRecursively()`, the thread pool
// may be used in case retries are needed. On Windows, in particular, retries
// will be attempted for some time to allow other programs (e.g., anti-virus
// scanners or malware) to close any open handles to `path` or its contents. If
// `reply_callback` is not null, it will be posted to the caller's sequence with
// true if `path` was fully deleted or false otherwise.
//
// WARNING: It is NOT safe to use `path` until `reply_callback` is run, as the
// retry task may still be actively trying to delete it.
BASE_EXPORT OnceClosure
GetDeleteFileCallback(const FilePath& path,
                      OnceCallback<void(bool)> reply_callback = {});
BASE_EXPORT OnceClosure
GetDeletePathRecursivelyCallback(const FilePath& path,
                                 OnceCallback<void(bool)> reply_callback = {});

#if BUILDFLAG(IS_WIN)
// Schedules to delete the given path, whether it's a file or a directory, until
// the operating system is restarted.
// Note:
// 1) The file/directory to be deleted should exist in a temp folder.
// 2) The directory to be deleted must be empty.
BASE_EXPORT bool DeleteFileAfterReboot(const FilePath& path);

// Prevents opening the file at `path` with EXECUTE access by adding a deny ACE
// on the filesystem. This allows the file handle to be safely passed to an
// untrusted process. See also `File::FLAG_WIN_NO_EXECUTE`.
BASE_EXPORT bool PreventExecuteMapping(const FilePath& path);

// Same as PreventExecuteMapping but DCHECK for known allowed paths is omitted.
// Only call this if you know the path you are providing is safe to mark as
// non-executable, such as log files.
BASE_EXPORT bool PreventExecuteMappingUnchecked(
    const FilePath& path,
    base::PassKey<PreventExecuteMappingClasses> passkey);

// Set `path_key` to the second of two valid paths that support safely marking a
// file as non-execute. The first allowed path is always PATH_TEMP. This is
// needed to avoid layering violations, as the user data dir is an embedder
// concept and only known later at runtime.
BASE_EXPORT void SetExtraNoExecuteAllowedPath(int path_key);
#endif  // BUILDFLAG(IS_WIN)

// Moves the given path, whether it's a file or a directory.
// If a simple rename is not possible, such as in the case where the paths are
// on different volumes, this will attempt to copy and delete. Returns
// true for success.
// This function fails if either path contains traversal components ('..').
BASE_EXPORT bool Move(const FilePath& from_path, const FilePath& to_path);

// Renames file |from_path| to |to_path|. Both paths must be on the same
// volume, or the function will fail. Destination file will be created
// if it doesn't exist. Prefer this function over Move when dealing with
// temporary files. On Windows it preserves attributes of the target file.
// Returns true on success, leaving *error unchanged.
// Returns false on failure and sets *error appropriately, if it is non-NULL.
BASE_EXPORT bool ReplaceFile(const FilePath& from_path,
                             const FilePath& to_path,
                             File::Error* error);

// Copies a single file. Use CopyDirectory() to copy directories.
// This function fails if either path contains traversal components ('..').
// This function also fails if |to_path| is a directory.
//
// On POSIX, if |to_path| is a symlink, CopyFile() will follow the symlink. This
// may have security implications. Use with care.
//
// If |to_path| already exists and is a regular file, it will be overwritten,
// though its permissions will stay the same.
//
// If |to_path| does not exist, it will be created. The new file's permissions
// varies per platform:
//
// - This function keeps the metadata on Windows. The read only bit is not kept.
// - On Mac and iOS, |to_path| retains |from_path|'s permissions, except user
//   read/write permissions are always set.
// - On Linux and Android, |to_path| has user read/write permissions only. i.e.
//   Always 0600.
// - On ChromeOS, |to_path| has user read/write permissions and group/others
//   read permissions. i.e. Always 0644.
BASE_EXPORT bool CopyFile(const FilePath& from_path, const FilePath& to_path);

// Copies the contents of one file into another.
// The files are taken as is: the copy is done starting from the current offset
// of |infile| until the end of |infile| is reached, into the current offset of
// |outfile|.
BASE_EXPORT bool CopyFileContents(File& infile, File& outfile);

// Copies the given path, and optionally all subdirectories and their contents
// as well.
//
// If there are files existing under to_path, always overwrite. Returns true
// if successful, false otherwise. Wildcards on the names are not supported.
//
// This function has the same metadata behavior as CopyFile().
//
// If you only need to copy a file use CopyFile, it's faster.
BASE_EXPORT bool CopyDirectory(const FilePath& from_path,
                               const FilePath& to_path,
                               bool recursive);

// Like CopyDirectory() except trying to overwrite an existing file will not
// work and will return false.
BASE_EXPORT bool CopyDirectoryExcl(const FilePath& from_path,
                                   const FilePath& to_path,
                                   bool recursive);

// Returns true if the given path exists on the local filesystem,
// false otherwise.
BASE_EXPORT bool PathExists(const FilePath& path);

// Returns true if the given path is readable by the user, false otherwise.
BASE_EXPORT bool PathIsReadable(const FilePath& path);

// Returns true if the given path is writable by the user, false otherwise.
BASE_EXPORT bool PathIsWritable(const FilePath& path);

// Returns true if the given path exists and is a directory, false otherwise.
BASE_EXPORT bool DirectoryExists(const FilePath& path);

// Returns true if the contents of the two files given are equal, false
// otherwise.  If either file can't be read, returns false.
BASE_EXPORT bool ContentsEqual(const FilePath& filename1,
                               const FilePath& filename2);

// Returns true if the contents of the two text files given are equal, false
// otherwise.  This routine treats "\r\n" and "\n" as equivalent.
BASE_EXPORT bool TextContentsEqual(const FilePath& filename1,
                                   const FilePath& filename2);

// Reads the file at |path| and returns a vector of bytes on success, and
// nullopt on error. For security reasons, a |path| containing path traversal
// components ('..') is treated as a read error, returning nullopt.
BASE_EXPORT std::optional<std::vector<uint8_t>> ReadFileToBytes(
    const FilePath& path);

// Reads the file at |path| into |contents| and returns true on success and
// false on error.  For security reasons, a |path| containing path traversal
// components ('..') is treated as a read error and |contents| is set to empty.
// In case of I/O error, |contents| holds the data that could be read from the
// file before the error occurred.
// |contents| may be NULL, in which case this function is useful for its side
// effect of priming the disk cache (could be used for unit tests).
BASE_EXPORT bool ReadFileToString(const FilePath& path, std::string* contents);

// Reads the file at |path| into |contents| and returns true on success and
// false on error.  For security reasons, a |path| containing path traversal
// components ('..') is treated as a read error and |contents| is set to empty.
// In case of I/O error, |contents| holds the data that could be read from the
// file before the error occurred.  When the file size exceeds |max_size|, the
// function returns false with |contents| holding the file truncated to
// |max_size|.
// |contents| may be NULL, in which case this function is useful for its side
// effect of priming the disk cache (could be used for unit tests).
BASE_EXPORT bool ReadFileToStringWithMaxSize(const FilePath& path,
                                             std::string* contents,
                                             size_t max_size);

// As ReadFileToString, but reading from an open stream after seeking to its
// start (if supported by the stream). This can also be used to read the whole
// file from a file descriptor by converting the file descriptor into a stream
// by using base::FileToFILE() before calling this function.
BASE_EXPORT bool ReadStreamToString(FILE* stream, std::string* contents);

// As ReadFileToStringWithMaxSize, but reading from an open stream after seeking
// to its start (if supported by the stream).
BASE_EXPORT bool ReadStreamToStringWithMaxSize(FILE* stream,
                                               size_t max_size,
                                               std::string* contents);

#if BUILDFLAG(IS_POSIX) || BUILDFLAG(IS_FUCHSIA)

// Reads exactly as many bytes as `buffer` can hold from file descriptor `fd`
// into `buffer`. This function is protected against EINTR and partial reads.
// Returns true iff `buffer` was successfully filled with bytes read from `fd`.
BASE_EXPORT bool ReadFromFD(int fd, span<char> buffer);

// Performs the same function as CreateAndOpenTemporaryStreamInDir(), but
// returns the file-descriptor wrapped in a ScopedFD, rather than the stream
// wrapped in a ScopedFILE.
// The caller is responsible for deleting the file `path` points to, if
// appropriate.
BASE_EXPORT ScopedFD CreateAndOpenFdForTemporaryFileInDir(const FilePath& dir,
                                                          FilePath* path);

#endif  // BUILDFLAG(IS_POSIX) || BUILDFLAG(IS_FUCHSIA)

#if BUILDFLAG(IS_POSIX)

// ReadFileToStringNonBlocking is identical to ReadFileToString except it
// guarantees that it will not block. This guarantee is provided on POSIX by
// opening the file as O_NONBLOCK. This variant should only be used on files
// which are guaranteed not to block (such as kernel files). Or in situations
// where a partial read would be acceptable because the backing store returned
// EWOULDBLOCK.
BASE_EXPORT bool ReadFileToStringNonBlocking(const base::FilePath& file,
                                             std::string* ret);

// Creates a symbolic link at |symlink| pointing to |target|.  Returns
// false on failure.
BASE_EXPORT bool CreateSymbolicLink(const FilePath& target,
                                    const FilePath& symlink);

// Reads the given |symlink| and returns the raw string in |target|.
// Returns false upon failure.
// IMPORTANT NOTE: if the string stored in the symlink is a relative file path,
// it should be interpreted relative to the symlink's directory, NOT the current
// working directory. ReadSymbolicLinkAbsolute() may be the better choice.
BASE_EXPORT bool ReadSymbolicLink(const FilePath& symlink, FilePath* target);

// Same as ReadSymbolicLink(), but properly converts it into an absolute path if
// the link is relative.
// Can fail if readlink() fails, or if
// MakeAbsoluteFilePathNoResolveSymbolicLinks() fails on the resulting absolute
// path.
BASE_EXPORT std::optional<FilePath> ReadSymbolicLinkAbsolute(
    const FilePath& symlink);

// Bits and masks of the file permission.
enum FilePermissionBits {
  // clang-format off
  FILE_PERMISSION_MASK              = S_IRWXU | S_IRWXG | S_IRWXO,
  FILE_PERMISSION_USER_MASK         = S_IRWXU,
  FILE_PERMISSION_GROUP_MASK        = S_IRWXG,
  FILE_PERMISSION_OTHERS_MASK       = S_IRWXO,

  FILE_PERMISSION_READ_BY_USER      = S_IRUSR,
  FILE_PERMISSION_WRITE_BY_USER     = S_IWUSR,
  FILE_PERMISSION_EXECUTE_BY_USER   = S_IXUSR,
  FILE_PERMISSION_READ_BY_GROUP     = S_IRGRP,
  FILE_PERMISSION_WRITE_BY_GROUP    = S_IWGRP,
  FILE_PERMISSION_EXECUTE_BY_GROUP  = S_IXGRP,
  FILE_PERMISSION_READ_BY_OTHERS    = S_IROTH,
  FILE_PERMISSION_WRITE_BY_OTHERS   = S_IWOTH,
  FILE_PERMISSION_EXECUTE_BY_OTHERS = S_IXOTH,
  // clang-format on
};

// Reads the permission of the given |path|, storing the file permission
// bits in |mode|. If |path| is symbolic link, |mode| is the permission of
// a file which the symlink points to.
BASE_EXPORT bool GetPosixFilePermissions(const FilePath& path, int* mode);
// Sets the permission of the given |path|. If |path| is symbolic link, sets
// the permission of a file which the symlink points to.
BASE_EXPORT bool SetPosixFilePermissions(const FilePath& path, int mode);

// Returns true iff |executable| can be found in any directory specified by the
// environment variable in |env|.
BASE_EXPORT bool ExecutableExistsInPath(Environment* env,
                                        const FilePath::StringType& executable);

#if BUILDFLAG(IS_LINUX) || BUILDFLAG(IS_CHROMEOS) || BUILDFLAG(IS_AIX)
// Determine if files under a given |path| can be mapped and then mprotect'd
// PROT_EXEC. This depends on the mount options used for |path|, which vary
// among different Linux distributions and possibly local configuration. It also
// depends on details of kernel--ChromeOS uses the noexec option for /dev/shm
// but its kernel allows mprotect with PROT_EXEC anyway.
BASE_EXPORT bool IsPathExecutable(const FilePath& path);
#endif  // BUILDFLAG(IS_LINUX) || BUILDFLAG(IS_CHROMEOS) || BUILDFLAG(IS_AIX)

#endif  // BUILDFLAG(IS_POSIX)

// Returns true if the given directory is empty
BASE_EXPORT bool IsDirectoryEmpty(const FilePath& dir_path);

// Get the temporary directory provided by the system.
//
// WARNING: In general, you should use CreateTemporaryFile variants below
// instead of this function. Those variants will ensure that the proper
// permissions are set so that other users on the system can't edit them while
// they're open (which can lead to security issues).
BASE_EXPORT bool GetTempDir(FilePath* path);

// Get the home directory. This is more complicated than just getenv("HOME")
// as it knows to fall back on getpwent() etc.
//
// You should not generally call this directly. Instead use DIR_HOME with the
// path service which will use this function but cache the value.
// Path service may also override DIR_HOME.
BASE_EXPORT FilePath GetHomeDir();

// Returns a new temporary file in |dir| with a unique name. The file is opened
// for exclusive read, write, and delete access.
// On success, |temp_file| is populated with the full path to the created file.
//
// NOTE: Exclusivity is unique to Windows. On Windows, the returned file
// supports File::DeleteOnClose. On other platforms, the caller is responsible
// for deleting the file `temp_file` points to, if appropriate.
BASE_EXPORT File CreateAndOpenTemporaryFileInDir(const FilePath& dir,
                                                 FilePath* temp_file);

// Creates a temporary file. The full path is placed in `path`, and the
// function returns true if was successful in creating the file. The file will
// be empty and all handles closed after this function returns.
// The caller is responsible for deleting the file `path` points to, if
// appropriate.
BASE_EXPORT bool CreateTemporaryFile(FilePath* path);

// Same as CreateTemporaryFile() but the file is created in `dir`.
// The caller is responsible for deleting the file `temp_file` points to, if
// appropriate.
BASE_EXPORT bool CreateTemporaryFileInDir(const FilePath& dir,
                                          FilePath* temp_file);

// Returns the file name for a temporary file by using a platform-specific
// naming scheme that incorporates |identifier|.
BASE_EXPORT FilePath
FormatTemporaryFileName(FilePath::StringViewType identifier);

// Create and open a temporary file stream for exclusive read, write, and delete
// access. The full path is placed in `path`. Returns the opened file stream, or
// null in case of error.
// NOTE: Exclusivity is unique to Windows. On Windows, the returned file
// supports File::DeleteOnClose. On other platforms, the caller is responsible
// for deleting the file `path` points to, if appropriate.
BASE_EXPORT ScopedFILE CreateAndOpenTemporaryStream(FilePath* path);

// Similar to CreateAndOpenTemporaryStream(), but the file is created in `dir`.
BASE_EXPORT ScopedFILE CreateAndOpenTemporaryStreamInDir(const FilePath& dir,
                                                         FilePath* path);

// Do NOT USE in new code. Use ScopedTempDir instead.
// TODO(crbug.com/40446440) Remove existing usage and make this an
// implementation detail inside ScopedTempDir.
//
// Create a new directory. If prefix is provided, the new directory name is in
// the format of prefixyyyy.
// NOTE: prefix is ignored in the POSIX implementation.
// If success, return true and output the full path of the directory created.
//
// For Windows, this directory is usually created in a secure location if the
// caller is admin. This is because the default %TEMP% folder for Windows is
// insecure, since low privilege users can get the path of folders under %TEMP%
// after creation and are able to create subfolders and files within these
// folders which can lead to privilege escalation.
BASE_EXPORT bool CreateNewTempDirectory(const FilePath::StringType& prefix,
                                        FilePath* new_temp_path);

// Create a directory within another directory.
// Extra characters will be appended to |prefix| to ensure that the
// new directory does not have the same name as an existing directory.
BASE_EXPORT bool CreateTemporaryDirInDir(const FilePath& base_dir,
                                         FilePath::StringViewType prefix,
                                         FilePath* new_dir);

// Ensures a directory exists, if necessary, creating it and parent directories.
// Returns true if the directory already existed or was created, leaving *error
// unchanged.
// Returns false on failure and sets *error appropriately if it is non-NULL.
//
// The created directories can only be accessed by the current user, except on
// ChromeOS, where the directories created under `~/MyFiles` or `/media` can
// also be accessed by ChromeOS services.
BASE_EXPORT bool CreateDirectoryAndGetError(const FilePath& full_path,
                                            File::Error* error);

// Backward-compatible convenience method for the above.
BASE_EXPORT bool CreateDirectory(const FilePath& full_path);

// Returns the file size, or std::nullopt on failure.
BASE_EXPORT std::optional<int64_t> GetFileSize(const FilePath& file_path);

// Same as above, but as an OnceCallback.
BASE_EXPORT OnceCallback<std::optional<int64_t>()> GetFileSizeCallback(
    const FilePath& path);

// Sets |real_path| to |path| with symbolic links and junctions expanded.
// On Windows, the function ensures that the resulting |real_path| starts with a
// drive letter.
//
// The |path| parameter can reference either a file or a directory. The function
// will fail if |path| points to a nonexistent path or to a volume that isn't
// mapped to a drive letter on Windows.
//
// In addition, on Windows this function will fail if the resulting |real_path|
// would exceed 'MAX_PATH' characters in length.
BASE_EXPORT bool NormalizeFilePath(const FilePath& path, FilePath* real_path);

#if BUILDFLAG(IS_WIN)

// Given a path in NT native form ("\Device\HarddiskVolumeXX\..."),
// return in |drive_letter_path| the equivalent path that starts with
// a drive letter ("C:\...").  Return false if no such path exists.
BASE_EXPORT bool DevicePathToDriveLetterPath(const FilePath& device_path,
                                             FilePath* drive_letter_path);

// Method that wraps the win32 GetLongPathName API, normalizing the specified
// path to its long form. An example where this is needed is when comparing
// temp file paths. If a username isn't a valid 8.3 short file name (even just a
// lengthy name like "user with long name"), Windows will set the TMP and TEMP
// environment variables to be 8.3 paths. ::GetTempPath (called in
// base::GetTempDir) just uses the value specified by TMP or TEMP, and so can
// return a short path. Returns an empty path on error.
BASE_EXPORT FilePath MakeLongFilePath(const FilePath& input);

// Creates a hard link named |to_file| to the file |from_file|. Both paths
// must be on the same volume, and |from_file| may not name a directory.
// Returns true if the hard link is created, false if it fails.
BASE_EXPORT bool CreateWinHardLink(const FilePath& to_file,
                                   const FilePath& from_file);
#endif

// This function will return if the given file is a symlink or not.
BASE_EXPORT bool IsLink(const FilePath& file_path);

// Returns information about the given file path. Also see |File::GetInfo|.
BASE_EXPORT bool GetFileInfo(const FilePath& file_path, File::Info* info);

// Sets the time of the last access and the time of the last modification.
BASE_EXPORT bool TouchFile(const FilePath& path,
                           const Time& last_accessed,
                           const Time& last_modified);

// Wrapper for fopen-like calls. Returns non-NULL FILE* on success. The
// underlying file descriptor (POSIX) or handle (Windows) is unconditionally
// configured to not be propagated to child processes.
BASE_EXPORT FILE* OpenFile(const FilePath& filename, const char* mode);

// Closes file opened by OpenFile. Returns true on success.
BASE_EXPORT bool CloseFile(FILE* file);

// Associates a standard FILE stream with an existing File. Note that this
// functions take ownership of the existing File.
BASE_EXPORT FILE* FileToFILE(File file, const char* mode);

// Returns a new handle to the file underlying |file_stream|.
BASE_EXPORT File FILEToFile(FILE* file_stream);

// Truncates an open file to end at the location of the current file pointer.
// This is a cross-platform analog to Windows' SetEndOfFile() function.
BASE_EXPORT bool TruncateFile(FILE* file);

// Reads from the file into `buffer`. This will read at most as many bytes as
// `buffer` can hold, but may not always fill `buffer` entirely.
// Returns the number of bytes read, or nullopt on error.
// TODO(crbug.com/40227936): Despite the 64-bit return value, this only supports
// reading at most INT_MAX bytes. The program will crash if a buffer is passed
// whose length exceeds INT_MAX.
BASE_EXPORT std::optional<uint64_t> ReadFile(const FilePath& filename,
                                             span<char> buffer);
BASE_EXPORT std::optional<uint64_t> ReadFile(const FilePath& filename,
                                             span<uint8_t> buffer);

// Same as above, but returns -1 on error.
// TODO(crbug.com/40284755): Migrate callers to the span variant.
BASE_EXPORT int ReadFile(const FilePath& filename, char* data, int max_size);

// Writes |data| into the file, overwriting any data that was previously there.
// Returns true if and only if all of |data| was written. If the file does not
// exist, it gets created with read/write permissions for all.
BASE_EXPORT bool WriteFile(const FilePath& filename, span<const uint8_t> data);

// Another WriteFile() variant that takes a std::string_view so callers don't
// have to do manual conversions from a char span to a uint8_t span.
BASE_EXPORT bool WriteFile(const FilePath& filename, std::string_view data);

#if BUILDFLAG(IS_POSIX) || BUILDFLAG(IS_FUCHSIA)
// Appends |data| to |fd|. Does not close |fd| when done.  Returns true iff all
// of |data| were written to |fd|.
BASE_EXPORT bool WriteFileDescriptor(int fd, span<const uint8_t> data);

// WriteFileDescriptor() variant that takes a std::string_view so callers don't
// have to do manual conversions from a char span to a uint8_t span.
BASE_EXPORT bool WriteFileDescriptor(int fd, std::string_view data);

// Allocates disk space for the file referred to by |fd| for the byte range
// starting at |offset| and continuing for |size| bytes. The file size will be
// changed if |offset|+|len| is greater than the file size. Zeros will fill the
// new space.
// After a successful call, subsequent writes into the specified range are
// guaranteed not to fail because of lack of disk space.
BASE_EXPORT bool AllocateFileRegion(File* file, int64_t offset, size_t size);
#endif

// Appends |data| to |filename|.  Returns true iff |data| were written to
// |filename|.
BASE_EXPORT bool AppendToFile(const FilePath& filename,
                              span<const uint8_t> data);

// AppendToFile() variant that takes a std::string_view so callers don't have to
// do manual conversions from a char span to a uint8_t span.
BASE_EXPORT bool AppendToFile(const FilePath& filename, std::string_view data);

// Gets the current working directory for the process.
BASE_EXPORT bool GetCurrentDirectory(FilePath* path);

// Sets the current working directory for the process.
BASE_EXPORT bool SetCurrentDirectory(const FilePath& path);

// The largest value attempted by GetUniquePath.
enum { kMaxUniqueFiles = 100 };

// Returns |path| if it does not exist. Otherwise, returns |path| with the
// suffix " (N)" appended to its basename before any file extension, where N is
// a number between 1 and 100 (inclusive). Returns an empty path if no such
// number can be found.
BASE_EXPORT FilePath GetUniquePath(const FilePath& path);

// Same as `GetUniquePath()`, except this method allows specifying a custom
// suffix printf format string in cases where the default format doesn't work
// (for example because you need a filename without spaces in it). Passing
// " (%d)" as `suffix_format` makes this behave identical to `GetUniquePath()`.
BASE_EXPORT FilePath
GetUniquePathWithSuffixFormat(const FilePath& path,
                              base::cstring_view suffix_format);

// Sets the given |fd| to non-blocking mode.
// Returns true if it was able to set it in the non-blocking mode, otherwise
// false.
BASE_EXPORT bool SetNonBlocking(int fd);

// Hints the OS to prefetch the first |max_bytes| of |file_path| into its cache.
//
// If called at the appropriate time, this can reduce the latency incurred by
// feature code that needs to read the file.
//
// |max_bytes| specifies how many bytes should be pre-fetched. It may exceed the
// file's size. Passing in std::numeric_limits<int64_t>::max() is a convenient
// way to get the entire file pre-fetched.
//
// |is_executable| specifies whether the file is to be prefetched as
// executable code or as data. Windows treats the file backed pages in RAM
// differently, and specifying the wrong value results in two copies in RAM.
//
// |sequential| hints that the file will be read sequentially in the future.
// This has the affect of using POSIX_FADV_SEQUENTIAL on supported POSIX
// systems.
//
// Returns true if at least part of the requested range was successfully
// prefetched.
//
// Calling this before using ::LoadLibrary() on Windows is more efficient memory
// wise, but we must be sure no other threads try to LoadLibrary() the file
// while we are doing the mapping and prefetching, or the process will get a
// private copy of the DLL via COW.
BASE_EXPORT bool PreReadFile(
    const FilePath& file_path,
    bool is_executable,
    bool sequential,
    int64_t max_bytes = std::numeric_limits<int64_t>::max());

#if BUILDFLAG(IS_POSIX) || BUILDFLAG(IS_FUCHSIA)

// Creates a pipe. Returns true on success, otherwise false.
// On success, |read_fd| will be set to the fd of the read side, and
// |write_fd| will be set to the one of write side. If |non_blocking|
// is set the pipe will be created with O_NONBLOCK|O_CLOEXEC flags set
// otherwise flag is set to zero (default).
BASE_EXPORT bool CreatePipe(ScopedFD* read_fd,
                            ScopedFD* write_fd,
                            bool non_blocking = false);

// Creates a non-blocking, close-on-exec pipe.
// This creates a non-blocking pipe that is not intended to be shared with any
// child process. This will be done atomically if the operating system supports
// it. Returns true if it was able to create the pipe, otherwise false.
BASE_EXPORT bool CreateLocalNonBlockingPipe(span<int, 2u> fds);

// Sets the given |fd| to close-on-exec mode.
// Returns true if it was able to set it in the close-on-exec mode, otherwise
// false.
BASE_EXPORT bool SetCloseOnExec(int fd);

// Removes close-on-exec flag from the given |fd|.
// Returns true if it was able to remove the close-on-exec flag, otherwise
// false.
BASE_EXPORT bool RemoveCloseOnExec(int fd);
#endif  // BUILDFLAG(IS_POSIX) || BUILDFLAG(IS_FUCHSIA)

#if BUILDFLAG(IS_MAC)
// Test that |path| can only be changed by a given user and members of
// a given set of groups.
// Specifically, test that all parts of |path| under (and including) |base|:
// * Exist.
// * Are owned by a specific user.
// * Are not writable by all users.
// * Are owned by a member of a given set of groups, or are not writable by
//   their group.
// * Are not symbolic links.
// This is useful for checking that a config file is administrator-controlled.
// |base| must contain |path|.
BASE_EXPORT bool VerifyPathControlledByUser(const base::FilePath& base,
                                            const base::FilePath& path,
                                            uid_t owner_uid,
                                            const std::set<gid_t>& group_gids);

// Is |path| writable only by a user with administrator privileges?
// This function uses Mac OS conventions.  The super user is assumed to have
// uid 0, and the administrator group is assumed to be named "admin".
// Testing that |path|, and every parent directory including the root of
// the filesystem, are owned by the superuser, controlled by the group
// "admin", are not writable by all users, and contain no symbolic links.
// Will return false if |path| does not exist.
BASE_EXPORT bool VerifyPathControlledByAdmin(const base::FilePath& path);
#endif  // BUILDFLAG(IS_MAC)

// Returns the maximum length of path component on the volume containing
// the directory |path|, in the number of FilePath::CharType, or -1 on failure.
BASE_EXPORT int GetMaximumPathComponentLength(const base::FilePath& path);

#if BUILDFLAG(IS_POSIX) || BUILDFLAG(IS_FUCHSIA)
// Get a temporary directory for shared memory files. The directory may depend
// on whether the destination is intended for executable files, which in turn
// depends on how /dev/shmem was mounted. As a result, you must supply whether
// you intend to create executable shmem segments so this function can find
// an appropriate location.
BASE_EXPORT bool GetShmemTempDir(bool executable, FilePath* path);
#endif

// Internal --------------------------------------------------------------------

namespace internal {

// Same as Move but allows paths with traversal components.
// Use only with extreme care.
BASE_EXPORT bool MoveUnsafe(const FilePath& from_path, const FilePath& to_path);

#if BUILDFLAG(IS_WIN)
// Copy from_path to to_path recursively and then delete from_path recursively.
// Returns true if all operations succeed.
// This function simulates Move(), but unlike Move() it works across volumes.
// This function is not transactional.
BASE_EXPORT bool CopyAndDeleteDirectory(const FilePath& from_path,
                                        const FilePath& to_path);
#endif  // BUILDFLAG(IS_WIN)

#if BUILDFLAG(IS_LINUX) || BUILDFLAG(IS_CHROMEOS) || BUILDFLAG(IS_ANDROID)
// CopyFileContentsWithSendfile will use the sendfile(2) syscall to perform a
// file copy without moving the data between kernel and userspace. This is much
// more efficient than sequences of read(2)/write(2) calls. The |retry_slow|
// parameter instructs the caller that it should try to fall back to a normal
// sequences of read(2)/write(2) syscalls.
//
// The input file |infile| must be opened for reading and the output file
// |outfile| must be opened for writing.
BASE_EXPORT bool CopyFileContentsWithSendfile(File& infile,
                                              File& outfile,
                                              bool& retry_slow);
#endif  // BUILDFLAG(IS_LINUX) || BUILDFLAG(IS_CHROMEOS) ||
        // BUILDFLAG(IS_ANDROID)

}  // namespace internal
}  // namespace base

#endif  // BASE_FILES_FILE_UTIL_H_
