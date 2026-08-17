// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_SYNCHRONIZATION_WAITABLE_EVENT_WATCHER_H_
#define BASE_SYNCHRONIZATION_WAITABLE_EVENT_WATCHER_H_

#include "base/base_export.h"
#include "base/memory/raw_ptr.h"
#include "base/task/sequenced_task_runner.h"
#include "build/blink_buildflags.h"
#include "build/build_config.h"

#if BUILDFLAG(IS_WIN)
#include "base/win/object_watcher.h"
#include "base/win/scoped_handle.h"
#elif BUILDFLAG(IS_APPLE)
#include <dispatch/dispatch.h>

#include <memory>

#include "base/memory/weak_ptr.h"
#include "base/synchronization/waitable_event.h"
#else
#include "base/sequence_checker.h"
#include "base/synchronization/waitable_event.h"
#endif

#if !BUILDFLAG(IS_WIN)
#include "base/functional/callback.h"
#endif

namespace base {

class Flag;
class AsyncWaiter;
class WaitableEvent;

// This class provides a way to wait on a WaitableEvent asynchronously.
//
// Each instance of this object can be waiting on a single WaitableEvent. When
// the waitable event is signaled, a callback is invoked on the sequence that
// called StartWatching(). This callback can be deleted by deleting the waiter.
//
// Typical usage:
//
//   class MyClass {
//    public:
//     void DoStuffWhenSignaled(WaitableEvent *waitable_event) {
//       watcher_.StartWatching(waitable_event,
//           base::BindOnce(&MyClass::OnWaitableEventSignaled, this);
//     }
//    private:
//     void OnWaitableEventSignaled(WaitableEvent* waitable_event) {
//       // OK, time to do stuff!
//     }
//     base::WaitableEventWatcher watcher_;
//   };
//
// In the above example, MyClass wants to "do stuff" when waitable_event
// becomes signaled. WaitableEventWatcher makes this task easy. When MyClass
// goes out of scope, the watcher_ will be destroyed, and there is no need to
// worry about OnWaitableEventSignaled being called on a deleted MyClass
// pointer.
//
// BEWARE: With automatically reset WaitableEvents, a signal may be lost if it
// occurs just before a WaitableEventWatcher is deleted. There is currently no
// safe way to stop watching an automatic reset WaitableEvent without possibly
// missing a signal.
//
// NOTE: you /are/ allowed to delete the WaitableEvent while still waiting on
// it with a Watcher. But pay attention: if the event was signaled and deleted
// right after, the callback may be called with deleted WaitableEvent pointer.

class BASE_EXPORT WaitableEventWatcher
#if BUILDFLAG(IS_WIN)
    : public win::ObjectWatcher::Delegate
#endif
{
 public:
  using EventCallback = OnceCallback<void(WaitableEvent*)>;

  WaitableEventWatcher();

  WaitableEventWatcher(const WaitableEventWatcher&) = delete;
  WaitableEventWatcher& operator=(const WaitableEventWatcher&) = delete;

#if BUILDFLAG(IS_WIN)
  ~WaitableEventWatcher() override;
#else
  ~WaitableEventWatcher();
#endif

  // When |event| is signaled, |callback| is called on the sequence that called
  // StartWatching().
  // |task_runner| is used for asynchronous executions of calling |callback|.
  bool StartWatching(WaitableEvent* event,
                     EventCallback callback,
                     scoped_refptr<SequencedTaskRunner> task_runner);

  // Cancel the current watch. Must be called from the same sequence which
  // started the watch.
  //
  // Does nothing if no event is being watched, nor if the watch has completed.
  // The callback will *not* be called for the current watch after this
  // function returns. Since the callback runs on the same sequence as this
  // function, it cannot be called during this function either.
  void StopWatching();

 private:
#if BUILDFLAG(IS_WIN)
  void OnObjectSignaled(HANDLE h) override;

  // Duplicated handle of the event passed to StartWatching().
  win::ScopedHandle duplicated_event_handle_;

  // A watcher for |duplicated_event_handle_|. The handle MUST outlive
  // |watcher_|.
  win::ObjectWatcher watcher_;

  EventCallback callback_;
  raw_ptr<WaitableEvent, AcrossTasksDanglingUntriaged> event_ = nullptr;
#elif BUILDFLAG(IS_APPLE) && (!BUILDFLAG(IS_IOS) || !BUILDFLAG(USE_BLINK))
  // Invokes the callback and resets the source. Must be called on the task
  // runner on which StartWatching() was called.
  void InvokeCallback();

  // Closure bound to the event being watched. This will be is_null() if
  // nothing is being watched.
  OnceClosure callback_;

  // A reference to the receive right that is kept alive while a watcher
  // is waiting. Null if no event is being watched.
  scoped_refptr<WaitableEvent::ReceiveRight> receive_right_;

  struct Storage;
  std::unique_ptr<Storage> storage_;

  // Used to vend a weak pointer for calling InvokeCallback() from the
  // |source_| event handler.
  WeakPtrFactory<WaitableEventWatcher> weak_ptr_factory_;
#else
  // Instantiated in StartWatching(). Set before the callback runs. Reset in
  // StopWatching() or StartWatching().
  scoped_refptr<Flag> cancel_flag_;

  // Enqueued in the wait list of the watched WaitableEvent.
  raw_ptr<AsyncWaiter, AcrossTasksDanglingUntriaged> waiter_ = nullptr;

  // Kernel of the watched WaitableEvent.
  scoped_refptr<WaitableEvent::WaitableEventKernel> kernel_;

  // Ensures that StartWatching() and StopWatching() are called on the same
  // sequence.
  SEQUENCE_CHECKER(sequence_checker_);
#endif
};

}  // namespace base

#endif  // BASE_SYNCHRONIZATION_WAITABLE_EVENT_WATCHER_H_
