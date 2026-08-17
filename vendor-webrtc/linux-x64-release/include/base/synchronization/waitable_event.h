// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_SYNCHRONIZATION_WAITABLE_EVENT_H_
#define BASE_SYNCHRONIZATION_WAITABLE_EVENT_H_

#include <stddef.h>

#include "base/base_export.h"
#include "base/compiler_specific.h"
#include "base/containers/circular_deque.h"
#include "base/memory/raw_ptr.h"
#include "build/blink_buildflags.h"
#include "build/build_config.h"

#if BUILDFLAG(IS_WIN)
#include "base/win/scoped_handle.h"
#elif BUILDFLAG(IS_APPLE)
#include <mach/mach.h>

#include <memory>

#include "base/apple/scoped_mach_port.h"
#include "base/functional/callback_forward.h"
#include "base/memory/ref_counted.h"
#elif BUILDFLAG(IS_POSIX) || BUILDFLAG(IS_FUCHSIA)
#include <utility>

#include "base/memory/ref_counted.h"
#include "base/synchronization/lock.h"
#endif

namespace base {

class TimeDelta;

// A WaitableEvent can be a useful thread synchronization tool when you want to
// allow one thread to wait for another thread to finish some work. For
// non-Windows systems, this can only be used from within a single address
// space.
//
// Use a WaitableEvent when you would otherwise use a Lock+ConditionVariable to
// protect a simple boolean value.  However, if you find yourself using a
// WaitableEvent in conjunction with a Lock to wait for a more complex state
// change (e.g., for an item to be added to a queue), then you should probably
// be using a ConditionVariable instead of a WaitableEvent.
//
// NOTE: On Windows, this class provides a subset of the functionality afforded
// by a Windows event object.  This is intentional.  If you are writing Windows
// specific code and you need other features of a Windows event, then you might
// be better off just using an Windows event directly.
class BASE_EXPORT WaitableEvent {
 public:
  // Indicates whether a WaitableEvent should automatically reset the event
  // state after a single waiting thread has been released or remain signaled
  // until Reset() is manually invoked.
  enum class ResetPolicy { MANUAL, AUTOMATIC };

  // Indicates whether a new WaitableEvent should start in a signaled state or
  // not.
  enum class InitialState { SIGNALED, NOT_SIGNALED };

  // Constructs a WaitableEvent with policy and initial state as detailed in
  // the above enums.
  WaitableEvent(ResetPolicy reset_policy = ResetPolicy::MANUAL,
                InitialState initial_state = InitialState::NOT_SIGNALED);

#if BUILDFLAG(IS_WIN)
  // Create a WaitableEvent from an Event HANDLE which has already been
  // created. This objects takes ownership of the HANDLE and will close it when
  // deleted.
  explicit WaitableEvent(win::ScopedHandle event_handle);
#endif

  WaitableEvent(const WaitableEvent&) = delete;
  WaitableEvent& operator=(const WaitableEvent&) = delete;

  ~WaitableEvent();

  // Put the event in the un-signaled state.
  void Reset();

  // Put the event in the signaled state.  Causing any thread blocked on Wait
  // to be woken up.
  void Signal();

  // Returns true if the event is in the signaled state, else false.  If this
  // is not a manual reset event, then this test will cause a reset.
  bool IsSignaled() const;

  // Wait indefinitely for the event to be signaled. Wait's return "happens
  // after" |Signal| has completed. This means that it's safe for a
  // WaitableEvent to synchronise its own destruction, like this:
  //
  //   WaitableEvent *e = new WaitableEvent;
  //   SendToOtherThread(e);
  //   e->Wait();
  //   delete e;
  NOT_TAIL_CALLED void Wait();

  // Wait up until wait_delta has passed for the event to be signaled
  // (real-time; ignores time overrides).  Returns true if the event was
  // signaled. Handles spurious wakeups and guarantees that |wait_delta| will
  // have elapsed if this returns false.
  //
  // TimedWait can synchronise its own destruction like |Wait|.
  NOT_TAIL_CALLED bool TimedWait(TimeDelta wait_delta);

#if BUILDFLAG(IS_WIN)
  HANDLE handle() const { return handle_.get(); }
#endif

  // Declares that this WaitableEvent will only ever be used by a thread that is
  // idle at the bottom of its stack and waiting for work (in particular, it is
  // not synchronously waiting on this event before resuming ongoing work). This
  // is useful to avoid telling base-internals that this thread is "blocked"
  // when it's merely idle and ready to do work. As such, this is only expected
  // to be used by thread and thread pool impls. In such cases wakeup.flow
  // events aren't emitted on |Signal|/|Wait|, because threading implementations
  // are responsible for emitting the cause of their wakeup from idle.
  void declare_only_used_while_idle() { only_used_while_idle_ = true; }

  // Wait, synchronously, on multiple events.
  //   waitables: an array of WaitableEvent pointers
  //   count: the number of elements in @waitables
  //
  // returns: the index of a WaitableEvent which has been signaled.
  //
  // You MUST NOT delete any of the WaitableEvent objects while this wait is
  // happening, however WaitMany's return "happens after" the |Signal| call
  // that caused it has completed, like |Wait|.
  //
  // If more than one WaitableEvent is signaled to unblock WaitMany, the lowest
  // index among them is returned.
  NOT_TAIL_CALLED static size_t WaitMany(WaitableEvent** waitables,
                                         size_t count);

  // For asynchronous waiting, see WaitableEventWatcher

  // This is a private helper class. It's here because it's used by friends of
  // this class (such as WaitableEventWatcher) to be able to enqueue elements
  // of the wait-list
  class Waiter {
   public:
    // Signal the waiter to wake up.
    //
    // Consider the case of a Waiter which is in multiple WaitableEvent's
    // wait-lists. Each WaitableEvent is automatic-reset and two of them are
    // signaled at the same time. Now, each will wake only the first waiter in
    // the wake-list before resetting. However, if those two waiters happen to
    // be the same object (as can happen if another thread didn't have a chance
    // to dequeue the waiter from the other wait-list in time), two auto-resets
    // will have happened, but only one waiter has been signaled!
    //
    // Because of this, a Waiter may "reject" a wake by returning false. In
    // this case, the auto-reset WaitableEvent shouldn't act as if anything has
    // been notified.
    virtual bool Fire(WaitableEvent* signaling_event) = 0;

    // Waiters may implement this in order to provide an extra condition for
    // two Waiters to be considered equal. In WaitableEvent::Dequeue, if the
    // pointers match then this function is called as a final check. See the
    // comments in ~Handle for why.
    virtual bool Compare(void* tag) = 0;

   protected:
    virtual ~Waiter() = default;
  };

 private:
  friend class WaitableEventWatcher;

  // The platform specific portions of Signal, TimedWait, and WaitMany (which do
  // the actual signaling and waiting).
  void SignalImpl();
  bool TimedWaitImpl(TimeDelta wait_delta);
  static size_t WaitManyImpl(WaitableEvent** waitables, size_t count);

#if BUILDFLAG(IS_WIN)
  win::ScopedHandle handle_;
#elif BUILDFLAG(IS_APPLE) && (!BUILDFLAG(IS_IOS) || !BUILDFLAG(USE_BLINK))
  // iOS which supports blink must use the posix variant since opening
  // mach_ports is prevented inside sandbox profiles.
  //
  // Peeks the message queue named by |port| and returns true if a message
  // is present and false if not. If |dequeue| is true, the messsage will be
  // drained from the queue. If |dequeue| is false, the queue will only be
  // peeked. |port| must be a receive right.
  static bool PeekPort(mach_port_t port, bool dequeue);

  // The Mach receive right is waited on by both WaitableEvent and
  // WaitableEventWatcher. It is valid to signal and then delete an event, and
  // a watcher should still be notified. If the right were to be destroyed
  // immediately, the watcher would not receive the signal. Because Mach
  // receive rights cannot have a user refcount greater than one, the right
  // must be reference-counted manually.
  class ReceiveRight : public RefCountedThreadSafe<ReceiveRight> {
   public:
    explicit ReceiveRight(mach_port_t name);

    ReceiveRight(const ReceiveRight&) = delete;
    ReceiveRight& operator=(const ReceiveRight&) = delete;

    mach_port_t Name() const { return right_.get(); }

   private:
    friend class RefCountedThreadSafe<ReceiveRight>;
    ~ReceiveRight();

    apple::ScopedMachReceiveRight right_;
  };

  const ResetPolicy policy_;

  // The receive right for the event.
  scoped_refptr<ReceiveRight> receive_right_;

  // The send right used to signal the event. This can be disposed of with
  // the event, unlike the receive right, since a deleted event cannot be
  // signaled.
  apple::ScopedMachSendRight send_right_;
#elif BUILDFLAG(IS_POSIX) || BUILDFLAG(IS_FUCHSIA)
  // On Windows, you must not close a HANDLE which is currently being waited on.
  // The MSDN documentation says that the resulting behaviour is 'undefined'.
  // To solve that issue each WaitableEventWatcher duplicates the given event
  // handle.

  // However, if we were to include the following members
  // directly then, on POSIX, one couldn't use WaitableEventWatcher to watch an
  // event which gets deleted. This mismatch has bitten us several times now,
  // so we have a kernel of the WaitableEvent, which is reference counted.
  // WaitableEventWatchers may then take a reference and thus match the Windows
  // behaviour.
  struct WaitableEventKernel
      : public RefCountedThreadSafe<WaitableEventKernel> {
   public:
    WaitableEventKernel(ResetPolicy reset_policy, InitialState initial_state);

    bool Dequeue(Waiter* waiter, void* tag);

    base::Lock lock_;
    const bool manual_reset_;
    bool signaled_;
    base::circular_deque<raw_ptr<Waiter, CtnExperimental>> waiters_;

   private:
    friend class RefCountedThreadSafe<WaitableEventKernel>;
    ~WaitableEventKernel();
  };

  typedef std::pair<WaitableEvent*, size_t> WaiterAndIndex;

  // When dealing with arrays of WaitableEvent*, we want to sort by the address
  // of the WaitableEvent in order to have a globally consistent locking order.
  // In that case we keep them, in sorted order, in an array of pairs where the
  // second element is the index of the WaitableEvent in the original,
  // unsorted, array.
  static size_t EnqueueMany(WaiterAndIndex* waitables,
                            size_t count,
                            Waiter* waiter);

  bool SignalAll();
  bool SignalOne();
  void Enqueue(Waiter* waiter);

  scoped_refptr<WaitableEventKernel> kernel_;
#endif

  // Whether a thread invoking Wait() on this WaitableEvent should be considered
  // blocked as opposed to idle (and potentially replaced if part of a pool),
  // and whether WaitableEvent should emit a wakeup.flow event on Signal =>
  // TimedWait.
  bool only_used_while_idle_ = false;
};

}  // namespace base

#endif  // BASE_SYNCHRONIZATION_WAITABLE_EVENT_H_
