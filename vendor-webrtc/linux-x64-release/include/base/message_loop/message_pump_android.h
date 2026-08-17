// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_MESSAGE_LOOP_MESSAGE_PUMP_ANDROID_H_
#define BASE_MESSAGE_LOOP_MESSAGE_PUMP_ANDROID_H_

#include <jni.h>

#include <memory>
#include <optional>

#include "base/base_export.h"
#include "base/functional/callback.h"
#include "base/memory/raw_ptr.h"
#include "base/message_loop/message_pump.h"
#include "base/time/time.h"

struct ALooper;

namespace base {

class IOWatcher;
class RunLoop;

// This class implements a MessagePump needed for MessagePumpType::UI and
// MessagePumpType::JAVA MessageLoops on OS_ANDROID platform.
//
// It works by registering two file descriptors for the Looper to additionally
// poll: one for delayed work and one for non-delayed work. For queueing
// immediate work within the Looper it writes to the eventfd(2). For delayed
// work it performs timerfd_settime(2).
//
// See: https://developer.android.com/ndk/reference/group/looper.
class BASE_EXPORT MessagePumpAndroid : public MessagePump {
 public:
  MessagePumpAndroid();

  MessagePumpAndroid(const MessagePumpAndroid&) = delete;
  MessagePumpAndroid& operator=(const MessagePumpAndroid&) = delete;

  ~MessagePumpAndroid() override;

  void Run(Delegate* delegate) override;
  void Quit() override;
  void ScheduleWork() override;
  void ScheduleDelayedWork(
      const Delegate::NextWorkInfo& next_work_info) override;
  IOWatcher* GetIOWatcher() override;

  static void InitializeFeatures();

  // Attaches |delegate| to this native MessagePump. |delegate| will from then
  // on be invoked by the native loop to process application tasks.
  virtual void Attach(Delegate* delegate);

  // We call Abort when there is a pending JNI exception, meaning that the
  // current thread will crash when we return to Java.
  // We can't call any JNI-methods before returning to Java as we would then
  // cause a native crash (instead of the original Java crash).
  void Abort() { should_abort_ = true; }
  bool IsAborted() { return should_abort_; }
  bool ShouldQuit() const { return should_abort_ || quit_; }

  // Tells the RunLoop to quit when idle, calling the callback when it's safe
  // for the Thread to stop.
  void QuitWhenIdle(base::OnceClosure callback);

  // These functions are only public so that the looper callbacks can call them,
  // and should not be called from outside this class.
  void OnDelayedLooperCallback();
  virtual void OnNonDelayedLooperCallback();  // Overridden for testing.

  void set_is_type_ui(bool is_type_ui) { is_type_ui_ = is_type_ui; }

 protected:
  Delegate* SetDelegate(Delegate* delegate);
  bool SetQuit(bool quit);
  virtual void DoDelayedLooperWork();
  virtual void DoNonDelayedLooperWork(bool do_idle_work);

 private:
  void ScheduleWorkInternal(bool do_idle_work);

  void OnReturnFromLooper();

  // Unlike other platforms, we don't control the message loop as it's
  // controlled by the Android Looper, so we can't run a RunLoop to keep the
  // Thread this pump belongs to alive. However, threads are expected to have an
  // active run loop, so we manage a RunLoop internally here, starting/stopping
  // it as necessary.
  std::unique_ptr<RunLoop> run_loop_;

  // See Abort().
  bool should_abort_ = false;

  // Whether this message pump is quitting, or has quit.
  bool quit_ = false;

  // The MessageLoop::Delegate for this pump.
  raw_ptr<Delegate> delegate_ = nullptr;

  // The time at which we are currently scheduled to wake up and perform a
  // delayed task. This avoids redundantly scheduling |delayed_fd_| with the
  // same timeout when subsequent work phases all go idle on the same pending
  // delayed task; nullopt if no wakeup is currently scheduled.
  std::optional<TimeTicks> delayed_scheduled_time_;

  // If set, a callback to fire when the message pump is quit.
  base::OnceClosure on_quit_callback_;

  // The file descriptor used to signal that non-delayed work is available.
  int non_delayed_fd_;

  // The file descriptor used to signal that delayed work is available.
  int delayed_fd_;

  // The Android Looper for this thread.
  raw_ptr<ALooper> looper_ = nullptr;

  // The JNIEnv* for this thread, used to check for pending exceptions.
  raw_ptr<JNIEnv> env_;

  // Whether this message serves a MessagePumpType::UI, and therefore can
  // consult with the input hint living on the UI thread.
  bool is_type_ui_ = false;

  // The IOWatcher for this thread, lazily initialized as needed.
  std::unique_ptr<IOWatcher> io_watcher_;
};

}  // namespace base

#endif  // BASE_MESSAGE_LOOP_MESSAGE_PUMP_ANDROID_H_
