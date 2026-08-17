// Copyright 2011 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// WARNING: You should probably be using Thread (thread.h) instead.  Thread is
//          Chrome's message-loop based Thread abstraction, and if you are a
//          thread running in the browser, there will likely be assumptions
//          that your thread will have an associated message loop.
//
// This is a simple thread interface that backs to a native operating system
// thread.  You should use this only when you want a thread that does not have
// an associated MessageLoop.  Unittesting is the best example of this.
//
// The simplest interface to use is DelegateSimpleThread, which will create
// a new thread, and execute the Delegate's virtual Run() in this new thread
// until it has completed, exiting the thread.
//
// NOTE: You *MUST* call Join on the thread to clean up the underlying thread
// resources.  You are also responsible for destructing the SimpleThread object.
// It is invalid to destroy a SimpleThread while it is running, or without
// Start() having been called (and a thread never created).  The Delegate
// object should live as long as a DelegateSimpleThread.
//
// Thread Safety: A SimpleThread is not completely thread safe.  It is safe to
// access it from the creating thread or from the newly created thread.  This
// implies that the creator thread should be the thread that calls Join.
//
// Example:
//   class MyThreadRunner : public DelegateSimpleThread::Delegate { ... };
//   MyThreadRunner runner;
//   DelegateSimpleThread thread(&runner, "good_name_here");
//   thread.Start();
//   // Start will return after the Thread has been successfully started and
//   // initialized.  The newly created thread will invoke runner->Run(), and
//   // run until it returns.
//   thread.Join();  // Wait until the thread has exited.  You *MUST* Join!
//   // The SimpleThread object is still valid, however you may not call Join
//   // or Start again.

#ifndef BASE_THREADING_SIMPLE_THREAD_H_
#define BASE_THREADING_SIMPLE_THREAD_H_

#include <stddef.h>

#include <memory>
#include <string>
#include <vector>

#include "base/base_export.h"
#include "base/compiler_specific.h"
#include "base/containers/queue.h"
#include "base/memory/raw_ptr.h"
#include "base/synchronization/lock.h"
#include "base/synchronization/waitable_event.h"
#include "base/threading/platform_thread.h"

namespace base {

// This is the base SimpleThread.  You can derive from it and implement the
// virtual Run method, or you can use the DelegateSimpleThread interface.
// SimpleThread should not be used to run a MessagePump, `base::Thread` must be
// used for that.
class BASE_EXPORT SimpleThread : public PlatformThread::Delegate {
 public:
  struct BASE_EXPORT Options {
   public:
    Options() = default;
    explicit Options(ThreadType thread_type) : thread_type(thread_type) {}
    ~Options() = default;

    // Allow copies.
    Options(const Options& other) = default;
    Options& operator=(const Options& other) = default;

    // A custom stack size, or 0 for the system default.
    size_t stack_size = 0;

    ThreadType thread_type = ThreadType::kDefault;

    // If false, the underlying thread's PlatformThreadHandle will not be kept
    // around and as such the SimpleThread instance will not be Join()able and
    // must not be deleted before Run() is invoked. After that, it's up to
    // the subclass to determine when it is safe to delete itself.
    bool joinable = true;
  };

  // Creates a SimpleThread. |options| should be used to manage any specific
  // configuration involving the thread creation and management.
  // Every thread has a name, which is a display string to identify the thread.
  // The thread will not be created until Start() is called.
  explicit SimpleThread(const std::string& name);
  SimpleThread(const std::string& name, const Options& options);

  SimpleThread(const SimpleThread&) = delete;
  SimpleThread& operator=(const SimpleThread&) = delete;

  ~SimpleThread() override;

  // Starts the thread and returns only after the thread has started and
  // initialized (i.e. ThreadMain() has been called).
  void Start();

  // Joins the thread. If StartAsync() was used to start the thread, then this
  // first waits for the thread to start cleanly, then it joins.
  void Join();

  // Starts the thread, but returns immediately, without waiting for the thread
  // to have initialized first (i.e. this does not wait for ThreadMain() to have
  // been run first).
  void StartAsync();

  // Subclasses should override the Run method.
  virtual void Run() = 0;

  // Returns the thread id, only valid after the thread has started. If the
  // thread was started using Start(), then this will be valid after the call to
  // Start(). If StartAsync() was used to start the thread, then this must not
  // be called before HasBeenStarted() returns True.
  PlatformThreadId tid();

  // Returns True if the thread has been started and initialized (i.e. if
  // ThreadMain() has run). If the thread was started with StartAsync(), but it
  // hasn't been initialized yet (i.e. ThreadMain() has not run), then this will
  // return False.
  bool HasBeenStarted();

  // Returns True if Join() has ever been called.
  bool HasBeenJoined() const { return joined_; }

  // Returns true if Start() or StartAsync() has been called.
  bool HasStartBeenAttempted() { return start_called_; }

  // Overridden from PlatformThread::Delegate:
  void ThreadMain() override;

 private:
  // This is called just before the thread is started. This is called regardless
  // of whether Start() or StartAsync() is used to start the thread.
  virtual void BeforeStart() {}

  // This is called just after the thread has been initialized and just before
  // Run() is called. This is called on the newly started thread.
  virtual void BeforeRun() {}

  // This is called just before the thread is joined. The thread is started and
  // has been initialized before this is called.
  virtual void BeforeJoin() {}

  const std::string name_;
  const Options options_;
  PlatformThreadHandle thread_;  // PlatformThread handle, reset after Join.
  WaitableEvent event_;          // Signaled if Start() was ever called.
  PlatformThreadId tid_ = kInvalidThreadId;  // The backing thread's id.
  bool joined_ = false;                      // True if Join has been called.
  // Set to true when the platform-thread creation has started.
  bool start_called_ = false;
};

// A SimpleThread which delegates Run() to its Delegate. Non-joinable
// DelegateSimpleThread are safe to delete after Run() was invoked, their
// Delegates are also safe to delete after that point from this class' point of
// view (although implementations must of course make sure that Run() will not
// use their Delegate's member state after its deletion).
class BASE_EXPORT DelegateSimpleThread : public SimpleThread {
 public:
  class BASE_EXPORT Delegate {
   public:
    virtual ~Delegate() = default;
    virtual void Run() = 0;
  };

  DelegateSimpleThread(Delegate* delegate, const std::string& name_prefix);
  DelegateSimpleThread(Delegate* delegate,
                       const std::string& name_prefix,
                       const Options& options);

  DelegateSimpleThread(const DelegateSimpleThread&) = delete;
  DelegateSimpleThread& operator=(const DelegateSimpleThread&) = delete;

  ~DelegateSimpleThread() override;
  void Run() override;

 private:
  raw_ptr<Delegate> delegate_;
};

// DelegateSimpleThreadPool allows you to start up a fixed number of threads,
// and then add jobs which will be dispatched to the threads.  This is
// convenient when you have a lot of small work that you want done
// multi-threaded, but don't want to spawn a thread for each small bit of work.
//
// You just call AddWork() to add a delegate to the list of work to be done.
// JoinAll() will make sure that all outstanding work is processed, and wait
// for everything to finish.  You can reuse a pool, so you can call Start()
// again after you've called JoinAll().
class BASE_EXPORT DelegateSimpleThreadPool
    : public DelegateSimpleThread::Delegate {
 public:
  typedef DelegateSimpleThread::Delegate Delegate;

  DelegateSimpleThreadPool(const std::string& name_prefix, size_t num_threads);

  DelegateSimpleThreadPool(const DelegateSimpleThreadPool&) = delete;
  DelegateSimpleThreadPool& operator=(const DelegateSimpleThreadPool&) = delete;

  ~DelegateSimpleThreadPool() override;

  // Start up all of the underlying threads, and start processing work if we
  // have any.
  void Start();

  // Make sure all outstanding work is finished, and wait for and destroy all
  // of the underlying threads in the pool.
  void JoinAll();

  // It is safe to AddWork() any time, before or after Start().
  // Delegate* should always be a valid pointer, NULL is reserved internally.
  void AddWork(Delegate* work, size_t repeat_count = 1);

  // We implement the Delegate interface, for running our internal threads.
  void Run() override;

 private:
  const std::string name_prefix_;
  size_t num_threads_;
  std::vector<std::unique_ptr<DelegateSimpleThread>> threads_;
  base::queue<raw_ptr<Delegate, CtnExperimental>> delegates_;
  base::Lock lock_;    // Locks delegates_
  WaitableEvent dry_;  // Not signaled when there is no work to do.
};

}  // namespace base

#endif  // BASE_THREADING_SIMPLE_THREAD_H_
