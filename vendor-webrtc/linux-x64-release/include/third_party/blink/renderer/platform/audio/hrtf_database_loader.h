/*
 * Copyright (C) 2010 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 *
 * 1.  Redistributions of source code must retain the above copyright
 *     notice, this list of conditions and the following disclaimer.
 * 2.  Redistributions in binary form must reproduce the above copyright
 *     notice, this list of conditions and the following disclaimer in the
 *     documentation and/or other materials provided with the distribution.
 * 3.  Neither the name of Apple Computer, Inc. ("Apple") nor the names of
 *     its contributors may be used to endorse or promote products derived
 *     from this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE AND ITS CONTRIBUTORS "AS IS" AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
 * WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 * DISCLAIMED. IN NO EVENT SHALL APPLE OR ITS CONTRIBUTORS BE LIABLE FOR ANY
 * DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
 * (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
 * LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND
 * ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF
 * THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_AUDIO_HRTF_DATABASE_LOADER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_AUDIO_HRTF_DATABASE_LOADER_H_

#include <memory>

#include "base/synchronization/lock.h"
#include "base/synchronization/waitable_event.h"
#include "third_party/blink/renderer/platform/audio/hrtf_database.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/scheduler/public/non_main_thread.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/ref_counted.h"

namespace base {
class WaitableEvent;
}  // namespace base

namespace blink {

// HRTFDatabaseLoader will asynchronously load the default HRTFDatabase in a new
// thread.
class PLATFORM_EXPORT HRTFDatabaseLoader final
    : public RefCounted<HRTFDatabaseLoader> {
  USING_FAST_MALLOC(HRTFDatabaseLoader);

 public:
  // Lazily creates a HRTFDatabaseLoader (if not already created) for the given
  // sample-rate and starts loading asynchronously (when created the first
  // time).
  // Returns the HRTFDatabaseLoader.
  // Must be called from the main thread.
  static scoped_refptr<HRTFDatabaseLoader>
  CreateAndLoadAsynchronouslyIfNecessary(float sample_rate);

  // Both constructor and destructor must be called from the main thread.
  ~HRTFDatabaseLoader();

  // Returns true once the default database has been completely loaded.  This
  // must be called from the audio thread.
  bool IsLoaded() { return Database(); }

  // May be called from both main and audio thread, and also can be called more
  // than once.
  void WaitForLoaderThreadCompletion();

  // Returns the database or nullptr if the database doesn't yet exist.  Must
  // be called from the audio thread.
  HRTFDatabase* Database();

 private:
  // Both constructor and destructor must be called from the main thread.
  explicit HRTFDatabaseLoader(float sample_rate);

  // If it hasn't already been loaded, creates a new thread and initiates
  // asynchronous loading of the default database.
  // This must be called from the main thread.
  void LoadAsynchronously();

  // Called in asynchronous loading thread.
  void LoadTask();
  void CleanupTask(base::WaitableEvent*);

  // `lock_` MUST be held when accessing `hrtf_database_` or `thread_` because
  // it can be accessed by multiple threads (e.g multiple AudioContexts).
  base::Lock lock_;
  std::unique_ptr<HRTFDatabase> hrtf_database_ GUARDED_BY(lock_);
  std::unique_ptr<NonMainThread> thread_ GUARDED_BY(lock_);

  const float database_sample_rate_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_AUDIO_HRTF_DATABASE_LOADER_H_
