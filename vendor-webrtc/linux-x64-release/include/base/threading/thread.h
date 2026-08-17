// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_THREADING_THREAD_H_
#define BASE_THREADING_THREAD_H_

#include <stddef.h>

#include <memory>
#include <string>

#include "base/base_export.h"
#include "base/check.h"
#include "base/compiler_specific.h"
#include "base/functional/callback.h"
#include "base/memory/raw_ptr.h"
#include "base/message_loop/message_pump_type.h"
#include "base/sequence_checker.h"
#include "base/synchronization/atomic_flag.h"
#include "base/synchronization/lock.h"
#include "base/synchronization/waitable_event.h"
#include "base/task/single_thread_task_runner.h"
#include "base/threading/platform_thread.h"
#include "build/build_config.h"

namespace base {

class MessagePump;
class RunLoop;

// IMPORTANT: Instead of creating a base::Thread, consider using
// base::ThreadPool::Create(Sequenced|SingleThread)TaskRunner().
//
// A simple thread abstraction that establishes a MessageLoop on a new thread.
// The consumer uses the MessageLoop of the thread to cause code to execute on
// the thread.  When this object is destroyed the thread is terminated.  All
// pending tasks queued on the thread's message loop will run to completion
// before the thread is terminated.
//
// WARNING! SUBCLASSES MUST CALL Stop() IN THEIR DESTRUCTORS!  See ~Thread().
//
// After the thread is stopped, the destruction sequence is:
//
//  (1) Thread::CleanUp()
//  (2) MessageLoop::~MessageLoop
//  (3.b) CurrentThread::DestructionObserver::WillDestroyCurrentMessageLoop
//
// This API is not thread-safe: unless indicated otherwise its methods are only
// valid from the owning sequence (which is the one from which Start() is
// invoked -- should it differ from the one on which it was constructed).
//
// Sometimes it's useful to kick things off on the initial sequence (e.g.
// construction, Start(), task_runner()), but to then hand the Thread over to a
// pool of users for the last one of them to destroy it when done. For that use
// case, Thread::DetachFromSequence() allows the owning sequence to give up
// ownership. The caller is then responsible to ensure a happens-after
// relationship between the DetachFromSequence() call and the next use of that
// Thread object (including ~Thread()).
class BASE_EXPORT Thread : PlatformThread::Delegate {
 public:
  class BASE_EXPORT Delegate {
   public:
    virtual ~Delegate() = default;

    virtual scoped_refptr<SingleThreadTaskRunner> GetDefaultTaskRunner() = 0;

    // Binds a RunLoop::Delegate and task runner CurrentDefaultHandle to the
    // thread.
    virtual void BindToCurrentThread() = 0;
  };

  struct BASE_EXPORT Options {
    using MessagePumpFactory =
        RepeatingCallback<std::unique_ptr<MessagePump>()>;

    Options();
    Options(MessagePumpType type, size_t size);
    explicit Options(ThreadType thread_type);
    Options(Options&& other);
    Options& operator=(Options&& other);
    ~Options();

    // Specifies the type of message pump that will be allocated on the thread.
    // This is ignored if message_pump_factory.is_null() is false.
    MessagePumpType message_pump_type = MessagePumpType::DEFAULT;

    // An unbound Delegate that will be bound to the thread. Ownership
    // of |delegate| will be transferred to the thread.
    std::unique_ptr<Delegate> delegate = nullptr;

    // Used to create the MessagePump for the MessageLoop. The callback is Run()
    // on the thread. If message_pump_factory.is_null(), then a MessagePump
    // appropriate for |message_pump_type| is created. Setting this forces the
    // MessagePumpType to TYPE_CUSTOM. This is not compatible with a non-null
    // |delegate|.
    MessagePumpFactory message_pump_factory;

    // Specifies the maximum stack size that the thread is allowed to use.
    // This does not necessarily correspond to the thread's initial stack size.
    // A value of 0 indicates that the default maximum should be used.
    size_t stack_size = 0;

    // Specifies the initial thread type.
    ThreadType thread_type = ThreadType::kDefault;

    // If false, the thread will not be joined on destruction. This is intended
    // for threads that want TaskShutdownBehavior::CONTINUE_ON_SHUTDOWN
    // semantics. Non-joinable threads can't be joined (must be leaked and
    // can't be destroyed or Stop()'ed).
    // TODO(gab): allow non-joinable instances to be deleted without causing
    // user-after-frees (proposal @ https://crbug.com/629139#c14)
    bool joinable = true;

    bool IsValid() const { return !moved_from; }

   private:
    // Set to true when the object is moved into another. Use to prevent reuse
    // of a moved-from object.
    bool moved_from = false;
  };

  // Constructor.
  // name is a display string to identify the thread.
  explicit Thread(const std::string& name);

  Thread(const Thread&) = delete;
  Thread& operator=(const Thread&) = delete;

  // Destroys the thread, stopping it if necessary.
  //
  // NOTE: ALL SUBCLASSES OF Thread MUST CALL Stop() IN THEIR DESTRUCTORS (or
  // guarantee Stop() is explicitly called before the subclass is destroyed).
  // This is required to avoid a data race between the destructor modifying the
  // vtable, and the thread's ThreadMain calling the virtual method Run().  It
  // also ensures that the CleanUp() virtual method is called on the subclass
  // before it is destructed.
  ~Thread() override;

#if BUILDFLAG(IS_WIN)
  // Causes the thread to initialize COM.  This must be called before calling
  // Start() or StartWithOptions().  If |use_mta| is false, the thread is also
  // started with a TYPE_UI message loop.  It is an error to call
  // init_com_with_mta(false) and then StartWithOptions() with any message loop
  // type other than TYPE_UI.
  void init_com_with_mta(bool use_mta) {
    DCHECK(!delegate_);
    com_status_ = use_mta ? MTA : STA;
  }
#endif

  // Starts the thread.  Returns true if the thread was successfully started;
  // otherwise, returns false.  Upon successful return, the message_loop()
  // getter will return non-null.
  //
  // Note: This function can't be called on Windows with the loader lock held;
  // i.e. during a DllMain, global object construction or destruction, atexit()
  // callback.
  bool Start();

  // Starts the thread. Behaves exactly like Start in addition to allow to
  // override the default options.
  //
  // Note: This function can't be called on Windows with the loader lock held;
  // i.e. during a DllMain, global object construction or destruction, atexit()
  // callback.
  bool StartWithOptions(Options options);

  // Starts the thread and wait for the thread to start and run initialization
  // before returning. It's same as calling Start() and then
  // WaitUntilThreadStarted().
  // Note that using this (instead of Start() or StartWithOptions() causes
  // jank on the calling thread, should be used only in testing code.
  bool StartAndWaitForTesting();

  // Blocks until the thread starts running. Called within StartAndWait().
  // Note that calling this causes jank on the calling thread, must be used
  // carefully for production code.
  bool WaitUntilThreadStarted() const;

  // Blocks until all tasks previously posted to this thread have been executed.
  void FlushForTesting();

  // Signals the thread to exit and returns once the thread has exited. The
  // Thread object is completely reset and may be used as if it were newly
  // constructed (i.e., Start may be called again). Can only be called if
  // |joinable_|.
  //
  // Stop may be called multiple times and is simply ignored if the thread is
  // already stopped or currently stopping.
  //
  // Start/Stop are not thread-safe and callers that desire to invoke them from
  // different threads must ensure mutual exclusion.
  //
  // NOTE: If you are a consumer of Thread, it is not necessary to call this
  // before deleting your Thread objects, as the destructor will do it.
  // IF YOU ARE A SUBCLASS OF Thread, YOU MUST CALL THIS IN YOUR DESTRUCTOR.
  void Stop();

  // Signals the thread to exit in the near future.
  //
  // WARNING: This function is not meant to be commonly used. Use at your own
  // risk. Calling this function will cause message_loop() to become invalid in
  // the near future. This function was created to workaround a specific
  // deadlock on Windows with printer worker thread. In any other case, Stop()
  // should be used.
  //
  // Call Stop() to reset the thread object once it is known that the thread has
  // quit.
  void StopSoon();

  // Detaches the owning sequence, indicating that the next call to this API
  // (including ~Thread()) can happen from a different sequence (to which it
  // will be rebound). This call itself must happen on the current owning
  // sequence and the caller must ensure the next API call has a happens-after
  // relationship with this one.
  void DetachFromSequence();

  // Returns a TaskRunner for this thread. Use the TaskRunner's PostTask
  // methods to execute code on the thread. Returns nullptr if the thread is not
  // running (e.g. before Start or after Stop have been called). Callers can
  // hold on to this even after the thread is gone; in this situation, attempts
  // to PostTask() will fail.
  //
  // In addition to this Thread's owning sequence, this can also safely be
  // called from the underlying thread itself.
  scoped_refptr<SingleThreadTaskRunner> task_runner() const {
    // This class doesn't provide synchronization around |message_loop_base_|
    // and as such only the owner should access it (and the underlying thread
    // which never sees it before it's set). In practice, many callers are
    // coming from unrelated threads but provide their own implicit (e.g. memory
    // barriers from task posting) or explicit (e.g. locks) synchronization
    // making the access of |message_loop_base_| safe... Changing all of those
    // callers is unfeasible; instead verify that they can reliably see
    // |message_loop_base_ != nullptr| without synchronization as a proof that
    // their external synchronization catches the unsynchronized effects of
    // Start().
    DCHECK(owning_sequence_checker_.CalledOnValidSequence() ||
           (id_event_.IsSignaled() && id_ == PlatformThread::CurrentId()) ||
           delegate_);
    return delegate_ ? delegate_->GetDefaultTaskRunner() : nullptr;
  }

  // Returns the name of this thread (for display in debugger too).
  const std::string& thread_name() const LIFETIME_BOUND { return name_; }

  // Returns the thread ID.  Should not be called before the first Start*()
  // call.  Keeps on returning the same ID even after a Stop() call. The next
  // Start*() call renews the ID.
  //
  // WARNING: This function will block if the thread hasn't started yet.
  //
  // This method is thread-safe.
  PlatformThreadId GetThreadId() const;

  // Returns true if the thread has been started, and not yet stopped.
  bool IsRunning() const;

 protected:
  // Called just prior to starting the message loop
  virtual void Init() {}

  // Called to start the run loop. Inhibit tail calls to this function so that
  // the caller will be on the stack for profiling and crash analysis.
  NOT_TAIL_CALLED virtual void Run(RunLoop* run_loop);

  // Called just after the message loop ends
  virtual void CleanUp() {}

  static void SetThreadWasQuitProperly(bool flag);
  static bool GetThreadWasQuitProperly();

 private:
  // Friends for message_loop() access:
  friend class MessageLoopTaskRunnerTest;
  friend class ScheduleWorkTest;

#if BUILDFLAG(IS_WIN)
  enum ComStatus {
    NONE,
    STA,
    MTA,
  };
#endif

  // PlatformThread::Delegate methods:
  void ThreadMain() override;

  void ThreadQuitHelper();

#if BUILDFLAG(IS_WIN)
  // Whether this thread needs to initialize COM, and if so, in what mode.
  ComStatus com_status_ = NONE;
#endif

  // Mirrors the Options::joinable field used to start this thread. Verified
  // on Stop() -- non-joinable threads can't be joined (must be leaked).
  bool joinable_ = true;

  // If true, we're in the middle of stopping, and shouldn't access
  // |message_loop_|. It may non-nullptr and invalid.
  // Should be written on the thread that created this thread. Also read data
  // could be wrong on other threads.
  bool stopping_ = false;

  // True while inside of Run().
  bool running_ = false;
  mutable base::Lock running_lock_;  // Protects |running_|.

  // The thread's handle.
  PlatformThreadHandle thread_;
  mutable base::Lock thread_lock_;  // Protects |thread_|.

  // The thread's id once it has started.
  PlatformThreadId id_ = kInvalidThreadId;
  // Protects |id_| which must only be read while it's signaled.
  mutable WaitableEvent id_event_;

  // The thread's Delegate and RunLoop are valid only while the thread is
  // alive. Set by the created thread.
  std::unique_ptr<Delegate> delegate_;

  raw_ptr<RunLoop> run_loop_ = nullptr;

  // The name of the thread.  Used for debugging purposes.
  const std::string name_;

  // Signaled when the created thread gets ready to use the message loop.
  mutable WaitableEvent start_event_;

  // This class is not thread-safe, use this to verify access from the owning
  // sequence of the Thread.
  SequenceChecker owning_sequence_checker_;
};

}  // namespace base

#endif  // BASE_THREADING_THREAD_H_
