// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_FILE_SYSTEM_ACCESS_FILE_SYSTEM_ACCESS_CAPACITY_TRACKER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_FILE_SYSTEM_ACCESS_FILE_SYSTEM_ACCESS_CAPACITY_TRACKER_H_

#include "base/functional/callback.h"
#include "base/types/pass_key.h"
#include "mojo/public/cpp/bindings/pending_remote.h"
#include "third_party/blink/public/mojom/file_system_access/file_system_access_file_modification_host.mojom-blink.h"
#include "third_party/blink/renderer/core/execution_context/execution_context.h"
#include "third_party/blink/renderer/platform/mojo/heap_mojo_remote.h"

namespace blink {

class ExecutionContext;
class FileSystemAccessRegularFileDelegate;

// Tracks the capacity available to a (single)
// FileSystemAccessRegularFileDelegate.
//
// This is performed by keeping track of the file's size and the capacity for
// growth received by the browser process. If needed, additional capacity for
// growth is requested from the browser process.
class FileSystemAccessCapacityTracker final
    : public GarbageCollected<FileSystemAccessCapacityTracker> {
 public:
  explicit FileSystemAccessCapacityTracker(
      ExecutionContext* context,
      mojo::PendingRemote<mojom::blink::FileSystemAccessFileModificationHost>
          file_modification_host_remote,
      int64_t file_size,
      base::PassKey<FileSystemAccessRegularFileDelegate>);

  FileSystemAccessCapacityTracker(const FileSystemAccessCapacityTracker&) =
      delete;
  FileSystemAccessCapacityTracker& operator=(
      const FileSystemAccessCapacityTracker&) = delete;

  // Requests a change of the file's capacity to at least `required_capacity`.
  // If `file_capacity_` is smaller than `required_capacity`, additional
  // capacity is requested from the browser process. Calls `callback` with true
  // if and only if `file_capacity_` is at least `required_capacity`.
  //
  // TODO(https://crbug.com/1240056): Consider unifying the synchronous and the
  // asynchronous mojo interfaces.
  void RequestFileCapacityChange(int64_t required_capacity,
                                 base::OnceCallback<void(bool)> callback);

  // Requests a change of the file's capacity to at least `required_capacity`.
  // If `file_capacity_` is smaller than `required_capacity`, additional
  // capacity is requested from the browser process. Returns true if and only if
  // `file_capacity_` is at least `required_capacity`.
  //
  // This function may trigger a synchronous mojo call and should only be called
  // through the synchronous API of Access Handles.
  bool RequestFileCapacityChangeSync(int64_t required_capacity);

  // This method should be called for each modification to the file, even if the
  // file's size does not change. The caller must make sure that
  // `file_capacity_` is at least `new_size`.
  void OnFileContentsModified(int64_t new_size);

  // GarbageCollected
  void Trace(Visitor* visitor) const {
    visitor->Trace(file_modification_host_);
  }

 private:
  // Called after the mojo call to RequestCapacityChange is done.
  void DidRequestCapacityChange(int64_t new_size,
                                base::OnceCallback<void(bool)> callback,
                                int64_t granted_capacity);

  // Computes the size of the next capacity request.
  // The minimum capacity request is 1MB. Requests then double in size up to
  // 128MB. Beyond 128MB, requests are the multiples of 128MB.
  static int64_t GetNextCapacityRequestSize(int64_t required_capacity);

  SEQUENCE_CHECKER(sequence_checker_);

  // Used to route capacity allocation requests to the browser.
  HeapMojoRemote<mojom::blink::FileSystemAccessFileModificationHost>
      file_modification_host_;

  // Size of the file represented by the FileSystemAccessRegularFileDelegate
  // owning this.
  int64_t file_size_ GUARDED_BY_CONTEXT(sequence_checker_);

  // Total capacity available for the FileSystemAccessRegularFileDelegate owning
  // this. At all times, 0 <= `file_size` <= `file_capacity_` must hold.
  //
  // A file's initial capacity is its size, hence `file_capacity_` is
  // initialized with `file_size_`.
  int64_t file_capacity_ GUARDED_BY_CONTEXT(sequence_checker_);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_FILE_SYSTEM_ACCESS_FILE_SYSTEM_ACCESS_CAPACITY_TRACKER_H_
