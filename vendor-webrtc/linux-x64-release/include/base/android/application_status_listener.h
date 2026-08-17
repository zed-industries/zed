// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_ANDROID_APPLICATION_STATUS_LISTENER_H_
#define BASE_ANDROID_APPLICATION_STATUS_LISTENER_H_

#include <jni.h>

#include <memory>

#include "base/android/jni_android.h"
#include "base/base_export.h"
#include "base/functional/callback_forward.h"

namespace base {
namespace android {

// Define application state values like APPLICATION_STATE_VISIBLE in a
// way that ensures they're always the same than their Java counterpart.
//
// Note that these states represent the most visible Activity state.
// If there are activities with states paused and stopped, only
// HAS_PAUSED_ACTIVITIES should be returned.
//
// A Java counterpart will be generated for this enum.
// GENERATED_JAVA_ENUM_PACKAGE: org.chromium.base
enum ApplicationState {
  APPLICATION_STATE_UNKNOWN = 0,
  APPLICATION_STATE_HAS_RUNNING_ACTIVITIES = 1,
  APPLICATION_STATE_HAS_PAUSED_ACTIVITIES = 2,
  APPLICATION_STATE_HAS_STOPPED_ACTIVITIES = 3,
  APPLICATION_STATE_HAS_DESTROYED_ACTIVITIES = 4
};

// A native helper class to listen to state changes of the Android
// Application. This mirrors org.chromium.base.ApplicationStatus.
// any thread.
//
// To start listening, create a new instance, passing a callback to a
// function that takes an ApplicationState parameter. To stop listening,
// simply delete the listener object. The implementation guarantees
// that the callback will always be called on the thread that created
// the listener.
//
// Example:
//
//    void OnApplicationStateChange(ApplicationState state) {
//       ...
//    }
//
//    // Start listening.
//    auto my_listener = ApplicationStatusListener::New(
//        base::BindRepeating(&OnApplicationStateChange));
//
//    ...
//
//    // Stop listening.
//    my_listener.reset();
//
class BASE_EXPORT ApplicationStatusListener {
 public:
  using ApplicationStateChangeCallback =
      base::RepeatingCallback<void(ApplicationState)>;

  ApplicationStatusListener(const ApplicationStatusListener&) = delete;
  ApplicationStatusListener& operator=(const ApplicationStatusListener&) =
      delete;

  virtual ~ApplicationStatusListener();

  // Sets the callback to call when application state changes.
  virtual void SetCallback(const ApplicationStateChangeCallback& callback) = 0;

  // Notify observers that application state has changed.
  virtual void Notify(ApplicationState state) = 0;

  // Create a new listener. This object should only be used on a single thread.
  static std::unique_ptr<ApplicationStatusListener> New(
      const ApplicationStateChangeCallback& callback);

  // Internal use only: must be public to be called from JNI and unit tests.
  static void NotifyApplicationStateChange(ApplicationState state);

  // Expose jni call for ApplicationStatus.getStateForApplication.
  static ApplicationState GetState();

  // Returns true if the app is currently foregrounded.
  static bool HasVisibleActivities();

 protected:
  ApplicationStatusListener();
};

}  // namespace android
}  // namespace base

#endif  // BASE_ANDROID_APPLICATION_STATUS_LISTENER_H_
