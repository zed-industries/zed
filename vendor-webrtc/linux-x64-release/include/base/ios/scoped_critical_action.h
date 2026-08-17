// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_IOS_SCOPED_CRITICAL_ACTION_H_
#define BASE_IOS_SCOPED_CRITICAL_ACTION_H_

#include <map>
#include <string>
#include <string_view>
#include <utility>

#include "base/feature_list.h"
#include "base/memory/ref_counted.h"
#include "base/synchronization/lock.h"
#include "base/time/time.h"

namespace base {
namespace ios {

// Skip starting background tasks if the application is terminating.
BASE_DECLARE_FEATURE(kScopedCriticalActionSkipOnShutdown);

// This class attempts to allow the application to continue to run for a period
// of time after it transitions to the background. The construction of an
// instance of this class marks the beginning of a task that needs background
// running time when the application is moved to the background and the
// destruction marks the end of such a task.
//
// Note there is no guarantee that the task will continue to finish when the
// application is moved to the background.
//
// This class should be used at times where leaving a task unfinished might be
// detrimental to user experience. For example, it should be used to ensure that
// the application has enough time to save important data or at least attempt to
// save such data.
class ScopedCriticalAction {
 public:
  ScopedCriticalAction(std::string_view task_name);

  ScopedCriticalAction(const ScopedCriticalAction&) = delete;
  ScopedCriticalAction& operator=(const ScopedCriticalAction&) = delete;

  ~ScopedCriticalAction();

  // Skip starting new background tasks if the application is terminating.
  // This must be triggered by the application and cannot be triggered by
  // a UIApplicationWillTerminateNotification, as that notification fires
  // after -[UIApplicationDelegate applicationWillTerminate:].
  static void ApplicationWillTerminate();

  // Exposed for unit-testing.
  static void ClearNumActiveBackgroundTasksForTest();
  static int GetNumActiveBackgroundTasksForTest();
  static void ResetApplicationWillTerminateForTest();

 private:
  // Core logic; ScopedCriticalAction should not be reference counted so
  // that it follows the normal pattern of stack-allocating ScopedFoo objects,
  // but the expiration handler needs to have a reference counted object to
  // refer to. All functions are thread safe.
  class Core : public base::RefCountedThreadSafe<Core> {
   public:
    Core();

    Core(const Core&) = delete;
    Core& operator=(const Core&) = delete;

    // Informs the OS that the background task has started. This is a
    // static method to ensure that the instance has a non-zero refcount.
    // |task_name| is used by the OS to log any leaked background tasks.
    // Invoking this function more than once is allowed: all except the
    // first successful call will be a no-op.
    static void StartBackgroundTask(scoped_refptr<Core> core,
                                    std::string_view task_name);
    // Informs the OS that the background task has completed. This is a
    // static method to ensure that the instance has a non-zero refcount.
    // Invoking this function more than once is allowed: all except the
    // first call will be a no-op.
    static void EndBackgroundTask(scoped_refptr<Core> core);

   private:
    friend base::RefCountedThreadSafe<Core>;
    ~Core();

    // |UIBackgroundTaskIdentifier| returned by
    // |beginBackgroundTaskWithName:expirationHandler:| when marking the
    // beginning of a long-running background task. It is defined as a uint64_t
    // instead of a |UIBackgroundTaskIdentifier| so this class can be used in
    // .cc files.
    uint64_t background_task_id_ GUARDED_BY(background_task_id_lock_);
    Lock background_task_id_lock_;
  };

  // This class is thread safe.
  class ActiveBackgroundTaskCache {
   public:
    // This struct should be considered internal to this class and opaque to
    // callers.
    struct InternalEntry {
      InternalEntry();
      InternalEntry(const InternalEntry&) = delete;
      InternalEntry(InternalEntry&&);
      ~InternalEntry();

      InternalEntry& operator=(const InternalEntry&) = delete;
      InternalEntry& operator=(InternalEntry&&);

      // The instance of the core that drives the background task.
      scoped_refptr<Core> core;
      // Refcounting for the number of ScopedCriticalAction instances that
      // require the existence of this background task.
      int num_active_handles = 0;
    };

    using NameAndTime = std::pair<std::string, base::TimeTicks>;
    using InternalEntriesMap = std::map<NameAndTime, InternalEntry>;
    // A handle should be treated as an opaque token by the caller.
    using Handle = InternalEntriesMap::iterator;

    // Returns a leaky singleton instance.
    static ActiveBackgroundTaskCache* GetInstance();

    ActiveBackgroundTaskCache();
    ~ActiveBackgroundTaskCache();

    // Starts a new background task if none existed with the same name. If a
    // task already exists with the same name, its lifetime is effectively
    // extended. Callers must invoke ReleaseHandle() once they no longer need to
    // prevent background suspension.
    Handle EnsureBackgroundTaskExistsWithName(std::string_view task_name);

    // Indicates that a previous caller to EnsureBackgroundTaskExistsWithName()
    // no longer needs to prevent background suspension.
    void ReleaseHandle(Handle handle);

    // Skip starting new background tasks if the application is terminating.
    void ApplicationWillTerminate();

    // Exposed for unit-testing.
    void ResetApplicationWillTerminateForTest();

   private:
    std::atomic_bool application_is_terminating_{false};
    InternalEntriesMap entries_map_ GUARDED_BY(entries_map_lock_);
    Lock entries_map_lock_;
  };

  const ActiveBackgroundTaskCache::Handle task_handle_;
};

}  // namespace ios
}  // namespace base

#endif  // BASE_IOS_SCOPED_CRITICAL_ACTION_H_
