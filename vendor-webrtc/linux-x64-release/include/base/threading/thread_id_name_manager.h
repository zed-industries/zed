// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_THREADING_THREAD_ID_NAME_MANAGER_H_
#define BASE_THREADING_THREAD_ID_NAME_MANAGER_H_

#include <map>
#include <string>
#include <vector>

#include "base/base_export.h"
#include "base/functional/callback.h"
#include "base/memory/raw_ptr.h"
#include "base/synchronization/lock.h"
#include "base/threading/platform_thread.h"

namespace base {

template <typename T>
struct DefaultSingletonTraits;

class BASE_EXPORT ThreadIdNameManager {
 public:
  static ThreadIdNameManager* GetInstance();

  ThreadIdNameManager(const ThreadIdNameManager&) = delete;
  ThreadIdNameManager& operator=(const ThreadIdNameManager&) = delete;

  static const char* GetDefaultInternedString();

  class BASE_EXPORT Observer {
   public:
    virtual ~Observer();

    // Called on the thread whose name is changing, immediately after the name
    // is set. |name| is a pointer to a C string that is guaranteed to remain
    // valid for the duration of the process.
    //
    // NOTE: Will be called while ThreadIdNameManager's lock is held, so don't
    // call back into it.
    virtual void OnThreadNameChanged(const char* name) = 0;
  };

  // Register the mapping between a thread |id| and |handle|.
  void RegisterThread(PlatformThreadHandle::Handle handle, PlatformThreadId id);

  void AddObserver(Observer*);
  void RemoveObserver(Observer*);

  // Set the name for the current thread.
  void SetName(const std::string& name);

  // Get the name for the given id.
  const char* GetName(PlatformThreadId id);

  // Unlike |GetName|, this method using TLS and avoids touching |lock_|.
  const char* GetNameForCurrentThread();

  // Remove the name for the given id.
  void RemoveName(PlatformThreadHandle::Handle handle, PlatformThreadId id);

  // Return all registered thread ids (note that this doesn't include the main
  // thread id).
  std::vector<PlatformThreadId> GetIds();

 private:
  friend struct DefaultSingletonTraits<ThreadIdNameManager>;

  typedef std::map<PlatformThreadId, PlatformThreadHandle::Handle>
      ThreadIdToHandleMap;
  typedef std::map<PlatformThreadHandle::Handle, std::string*>
      ThreadHandleToInternedNameMap;
  typedef std::map<std::string, raw_ptr<std::string, CtnExperimental>>
      NameToInternedNameMap;

  ThreadIdNameManager();
  ~ThreadIdNameManager();

  // lock_ protects the name_to_interned_name_, thread_id_to_handle_ and
  // thread_handle_to_interned_name_ maps.
  Lock lock_;

  NameToInternedNameMap name_to_interned_name_;
  ThreadIdToHandleMap thread_id_to_handle_;
  ThreadHandleToInternedNameMap thread_handle_to_interned_name_;

  // Treat the main process specially as there is no PlatformThreadHandle.
  raw_ptr<std::string> main_process_name_;
  PlatformThreadId main_process_id_;

  // There's no point using a base::ObserverList behind a lock, so we just use
  // an std::vector instead.
  std::vector<raw_ptr<Observer, VectorExperimental>> observers_;
};

}  // namespace base

#endif  // BASE_THREADING_THREAD_ID_NAME_MANAGER_H_
