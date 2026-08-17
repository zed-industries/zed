// Copyright 2013 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_ANDROID_JAVA_HANDLER_THREAD_H_
#define BASE_ANDROID_JAVA_HANDLER_THREAD_H_

#include <jni.h>

#include <memory>

#include "base/android/scoped_java_ref.h"
#include "base/dcheck_is_on.h"
#include "base/memory/raw_ptr.h"
#include "base/task/sequence_manager/sequence_manager.h"
#include "base/task/sequence_manager/task_queue.h"
#include "base/task/single_thread_task_runner.h"

namespace base {

class MessagePumpAndroid;

namespace android {

// A Java Thread with a native message loop. To run tasks, post them
// to the message loop and they will be scheduled along with Java tasks
// on the thread.
// This is useful for callbacks where the receiver expects a thread
// with a prepared Looper.
class BASE_EXPORT JavaHandlerThread {
 public:
  // Create new thread.
  explicit JavaHandlerThread(
      const char* name,
      base::ThreadType thread_type = base::ThreadType::kDefault);
  // Wrap and connect to an existing JavaHandlerThread.
  // |obj| is an instance of JavaHandlerThread.
  explicit JavaHandlerThread(
      const char* name,
      const base::android::ScopedJavaLocalRef<jobject>& obj);
  virtual ~JavaHandlerThread();

  // Gets the TaskRunner associated with the message loop.
  // Called from any thread.
  scoped_refptr<SingleThreadTaskRunner> task_runner() const {
    return state_ ? state_->default_task_queue->task_runner() : nullptr;
  }

  // Called from the parent thread.
  void Start();
  void Stop();

  // Called from java on the newly created thread.
  // Start() will not return before this methods has finished.
  void InitializeThread(JNIEnv* env, jlong event);
  // Called from java on this thread.
  void OnLooperStopped(JNIEnv* env);

  // Called from this thread.
  void StopSequenceManagerForTesting();
  // Called from this thread.
  void JoinForTesting();

  // Called from this thread.
  // See comment in JavaHandlerThread.java regarding use of this function.
  void ListenForUncaughtExceptionsForTesting();
  // Called from this thread.
  ScopedJavaLocalRef<jthrowable> GetUncaughtExceptionIfAny();

  // Returns the thread ID.  Should not be called before the first Start*()
  // call. This method is thread-safe.
  PlatformThreadId GetThreadId() const;

 protected:
  // Struct exists so JavaHandlerThread destructor can intentionally leak in an
  // abort scenario.
  struct State {
    State();
    ~State();

    std::unique_ptr<sequence_manager::SequenceManager> sequence_manager;
    sequence_manager::TaskQueue::Handle default_task_queue;
    raw_ptr<MessagePumpAndroid> pump = nullptr;
  };

  State* state() const { return state_.get(); }

  // Semantically the same as base::Thread#Init(), but unlike base::Thread the
  // Android Looper will already be running. This Init() call will still run
  // before other tasks are posted to the thread.
  virtual void Init() {}

  // Semantically the same as base::Thread#CleanUp(), called after the message
  // loop ends. The Android Looper will also have been quit by this point.
  virtual void CleanUp() {}

  std::unique_ptr<State> state_;

 private:
  void StartMessageLoop();

  void StopOnThread();
  void QuitThreadSafely();

  const char* name_;
  base::PlatformThreadId thread_id_{};
  ScopedJavaGlobalRef<jobject> java_thread_;
#if DCHECK_IS_ON()
  bool initialized_ = false;
#endif
};

}  // namespace android
}  // namespace base

#endif  // BASE_ANDROID_JAVA_HANDLER_THREAD_H_
