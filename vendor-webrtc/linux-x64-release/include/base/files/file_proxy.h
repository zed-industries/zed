// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_FILES_FILE_PROXY_H_
#define BASE_FILES_FILE_PROXY_H_

#include <stdint.h>

#include "base/base_export.h"
#include "base/containers/span.h"
#include "base/files/file.h"
#include "base/files/file_path.h"
#include "base/functional/callback_forward.h"
#include "base/memory/weak_ptr.h"

namespace base {

class TaskRunner;
class Time;

// This class provides asynchronous access to a File. All methods follow the
// same rules of the equivalent File method, as they are implemented by bouncing
// the operation to File using a TaskRunner.
//
// This class performs automatic proxying to close the underlying file at
// destruction.
//
// The TaskRunner is in charge of any sequencing of the operations, but a single
// operation can be proxied at a time, regardless of the use of a callback.
// In other words, having a sequence like
//
//   proxy.Write(...);
//   proxy.Write(...);
//
// means the second Write will always fail.
class BASE_EXPORT FileProxy final {
 public:
  // This callback is used by methods that report only an error code. It is
  // valid to pass a null callback to some functions that takes a
  // StatusCallback, in which case the operation will complete silently.
  using StatusCallback = OnceCallback<void(File::Error)>;
  using CreateTemporaryCallback =
      OnceCallback<void(File::Error, const FilePath&)>;
  using GetFileInfoCallback =
      OnceCallback<void(File::Error, const File::Info&)>;
  using ReadCallback =
      OnceCallback<void(File::Error, base::span<const char> data)>;
  using WriteCallback = OnceCallback<void(File::Error, int bytes_written)>;

  explicit FileProxy(TaskRunner* task_runner);
  FileProxy(const FileProxy&) = delete;
  FileProxy& operator=(const FileProxy&) = delete;
  ~FileProxy();

  // Creates or opens a file with the given flags. It is invalid to pass a null
  // callback. If File::FLAG_CREATE is set in |file_flags| it always tries to
  // create a new file at the given |file_path| and fails if the file already
  // exists.
  //
  // This returns false if task posting to |task_runner| has failed.
  bool CreateOrOpen(const FilePath& file_path,
                    uint32_t file_flags,
                    StatusCallback callback);

  // Creates a temporary file for writing. The path and an open file are
  // returned. It is invalid to pass a null callback. These additional file
  // flags will be added on top of the default file flags:
  //   File::FLAG_CREATE_ALWAYS
  //   File::FLAG_WRITE
  //   File::FLAG_WIN_TEMPORARY.
  //
  // This returns false if task posting to |task_runner| has failed.
  bool CreateTemporary(uint32_t additional_file_flags,
                       CreateTemporaryCallback callback);

  // Returns true if the underlying |file_| is valid.
  bool IsValid() const;

  // Returns true if a new file was created (or an old one truncated to zero
  // length to simulate a new file), and false otherwise.
  bool created() const { return file_.created(); }

  // Claims ownership of |file|. It is an error to call this method when
  // IsValid() returns true.
  void SetFile(File file);

  File TakeFile();

  // Returns a new File object that is a duplicate of the underlying |file_|.
  // See the comment at File::Duplicate for caveats.
  File DuplicateFile();

  PlatformFile GetPlatformFile() const;

  // Proxies File::Close. The callback can be null.
  // This returns false if task posting to |task_runner| has failed.
  bool Close(StatusCallback callback);

  // Proxies File::GetInfo. The callback can't be null.
  // This returns false if task posting to |task_runner| has failed.
  bool GetInfo(GetFileInfoCallback callback);

  // Proxies File::Read. The callback can't be null.
  // This returns false if |bytes_to_read| is less than zero, or
  // if task posting to |task_runner| has failed.
  bool Read(int64_t offset, int bytes_to_read, ReadCallback callback);

  // Proxies File::Write. The callback can be null.
  // This returns false if |bytes_to_write| is less than or equal to zero,
  // if |buffer| is NULL, or if task posting to |task_runner| has failed.
  bool Write(int64_t offset,
             base::span<const uint8_t> data,
             WriteCallback callback);

  // Proxies File::SetTimes. The callback can be null.
  // This returns false if task posting to |task_runner| has failed.
  bool SetTimes(Time last_access_time,
                Time last_modified_time,
                StatusCallback callback);

  // Proxies File::SetLength. The callback can be null.
  // This returns false if task posting to |task_runner| has failed.
  bool SetLength(int64_t length, StatusCallback callback);

  // Proxies File::Flush. The callback can be null.
  // This returns false if task posting to |task_runner| has failed.
  bool Flush(StatusCallback callback);

 private:
  friend class FileHelper;
  TaskRunner* task_runner() { return task_runner_.get(); }

  scoped_refptr<TaskRunner> task_runner_;
  File file_;

  base::WeakPtrFactory<FileProxy> weak_ptr_factory_{this};
};

}  // namespace base

#endif  // BASE_FILES_FILE_PROXY_H_
