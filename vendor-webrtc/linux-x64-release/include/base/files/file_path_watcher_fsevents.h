// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_FILES_FILE_PATH_WATCHER_FSEVENTS_H_
#define BASE_FILES_FILE_PATH_WATCHER_FSEVENTS_H_

#include <CoreServices/CoreServices.h>
#include <stddef.h>

#include <vector>

#include "base/apple/scoped_dispatch_object.h"
#include "base/files/file_path.h"
#include "base/files/file_path_watcher.h"
#include "base/memory/weak_ptr.h"

namespace base {

// Mac-specific file watcher implementation based on FSEvents.
// There are trade-offs between the FSEvents implementation and a kqueue
// implementation. The biggest issues are that FSEvents on 10.6 sometimes drops
// events and kqueue does not trigger for modifications to a file in a watched
// directory. See file_path_watcher_mac.cc for the code that decides when to
// use which one.
class FilePathWatcherFSEvents : public FilePathWatcher::PlatformDelegate {
 public:
  FilePathWatcherFSEvents();
  FilePathWatcherFSEvents(const FilePathWatcherFSEvents&) = delete;
  FilePathWatcherFSEvents& operator=(const FilePathWatcherFSEvents&) = delete;
  ~FilePathWatcherFSEvents() override;

  // FilePathWatcher::PlatformDelegate overrides.
  bool Watch(const FilePath& path,
             Type type,
             const FilePathWatcher::Callback& callback) override;
  void Cancel() override;

 private:
  static void FSEventsCallback(ConstFSEventStreamRef stream,
                               void* event_watcher,
                               size_t num_events,
                               void* event_paths,
                               const FSEventStreamEventFlags flags[],
                               const FSEventStreamEventId event_ids[]);

  // Called from FSEventsCallback whenever there is a change to the paths.
  void OnFilePathsChanged(const std::vector<FilePath>& paths);

  // Called on the task_runner() thread to dispatch path events. Can't access
  // target_ and resolved_target_ directly as those are modified on the
  // libdispatch thread.
  void DispatchEvents(const std::vector<FilePath>& paths,
                      const FilePath& target,
                      const FilePath& resolved_target);

  // (Re-)Initialize the event stream to start reporting events from
  // |start_event|.
  void UpdateEventStream(FSEventStreamEventId start_event);

  // Returns true if resolving the target path got a different result than
  // last time it was done.
  bool ResolveTargetPath();

  // Report an error watching the given target.
  void ReportError(const FilePath& target);

  // Destroy the event stream.
  void DestroyEventStream();

  // Start watching the FSEventStream.
  void StartEventStream(FSEventStreamEventId start_event, const FilePath& path);

  // Callback to notify upon changes.
  // (Only accessed from the task_runner() thread.)
  FilePathWatcher::Callback callback_;

  // The dispatch queue on which the event stream is scheduled.
  apple::ScopedDispatchObject<dispatch_queue_t> queue_;

  // Target path to watch (passed to callback).
  // (Only accessed from the libdispatch queue.)
  FilePath target_;

  // Target path with all symbolic links resolved.
  // (Only accessed from the libdispatch queue.)
  FilePath resolved_target_;

  // Backend stream we receive event callbacks from (strong reference).
  // (Only accessed from the libdispatch queue.)
  FSEventStreamRef fsevent_stream_ = nullptr;

  WeakPtrFactory<FilePathWatcherFSEvents> weak_factory_{this};
};

}  // namespace base

#endif  // BASE_FILES_FILE_PATH_WATCHER_FSEVENTS_H_
