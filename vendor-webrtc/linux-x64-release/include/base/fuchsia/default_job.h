// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_FUCHSIA_DEFAULT_JOB_H_
#define BASE_FUCHSIA_DEFAULT_JOB_H_

#include <lib/zx/job.h>

#include "base/base_export.h"

namespace base {

// Gets and sets the job object used for creating new child processes,
// and looking them up by their process IDs.
// zx::job::default_job() will be returned if no job is explicitly set here.
// Only valid handles may be passed to SetDefaultJob().
BASE_EXPORT zx::unowned_job GetDefaultJob();
BASE_EXPORT void SetDefaultJob(zx::job job);

// Replaces the current default job (if any) with the specified zx::job, and
// restores the original default job when going out-of-scope.
// Note that replacing the default job is not thread-safe!
class BASE_EXPORT ScopedDefaultJobForTest {
 public:
  ScopedDefaultJobForTest(zx::job new_default_job);

  ScopedDefaultJobForTest(const ScopedDefaultJobForTest&) = delete;
  ScopedDefaultJobForTest& operator=(const ScopedDefaultJobForTest&) = delete;

  ~ScopedDefaultJobForTest();

 private:
  zx::job old_default_job_;
};

}  // namespace base

#endif  // BASE_FUCHSIA_DEFAULT_JOB_H_
