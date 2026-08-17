// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_FILES_FILE_PATH_WATCHER_INOTIFY_H_
#define BASE_FILES_FILE_PATH_WATCHER_INOTIFY_H_

#include <stddef.h>

#include "base/base_export.h"

namespace base {

// Get the maximum number of inotify watches can be used by a FilePathWatcher
// instance. This is based on /proc/sys/fs/inotify/max_user_watches entry.
BASE_EXPORT size_t GetMaxNumberOfInotifyWatches();

// Overrides max inotify watcher counter for test.
class BASE_EXPORT ScopedMaxNumberOfInotifyWatchesOverrideForTest {
 public:
  explicit ScopedMaxNumberOfInotifyWatchesOverrideForTest(size_t override_max);
  ~ScopedMaxNumberOfInotifyWatchesOverrideForTest();
};

}  // namespace base

#endif  // BASE_FILES_FILE_PATH_WATCHER_INOTIFY_H_
