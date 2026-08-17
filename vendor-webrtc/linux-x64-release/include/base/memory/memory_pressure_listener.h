// Copyright 2013 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// MemoryPressure provides static APIs for handling memory pressure on
// platforms that have such signals, such as Android and ChromeOS.
// The app will try to discard buffers that aren't deemed essential (individual
// modules will implement their own policy).

#ifndef BASE_MEMORY_MEMORY_PRESSURE_LISTENER_H_
#define BASE_MEMORY_MEMORY_PRESSURE_LISTENER_H_

#include "base/base_export.h"
#include "base/functional/callback.h"
#include "base/location.h"
#include "base/tracing_buildflags.h"

namespace base {

// To start listening, create a new instance, passing a callback to a
// function that takes a MemoryPressureLevel parameter. To stop listening,
// simply delete the listener object. The implementation guarantees
// that the callback will always be called on the thread that created
// the listener.
// Note that even on the same thread, the callback is not guaranteed to be
// called synchronously within the system memory pressure broadcast.
// Please see notes in MemoryPressureLevel enum below: some levels are
// absolutely critical, and if not enough memory is returned to the system,
// it'll potentially kill the app, and then later the app will have to be
// cold-started.
//
// Example:
//
//    void OnMemoryPressure(MemoryPressureLevel memory_pressure_level) {
//       ...
//    }
//
//    // Start listening.
//    auto listener = std::make_unique<MemoryPressureListener>(
//        base::BindRepeating(&OnMemoryPressure));
//
//    ...
//
//    // Stop listening.
//    listener.reset();
//
class BASE_EXPORT MemoryPressureListener {
 public:
  // A Java counterpart will be generated for this enum.
  // The values needs to be kept in sync with the MemoryPressureLevel entry in
  // enums.xml.
  // GENERATED_JAVA_ENUM_PACKAGE: org.chromium.base
  // GENERATED_JAVA_PREFIX_TO_STRIP: MEMORY_PRESSURE_LEVEL_
  enum MemoryPressureLevel {
    // No problems, there is enough memory to use. This event is not sent via
    // callback, but the enum is used in other places to find out the current
    // state of the system.
    MEMORY_PRESSURE_LEVEL_NONE = 0,

    // Modules are advised to free buffers that are cheap to re-allocate and not
    // immediately needed.
    MEMORY_PRESSURE_LEVEL_MODERATE = 1,

    // At this level, modules are advised to free all possible memory.  The
    // alternative is to be killed by the system, which means all memory will
    // have to be re-created, plus the cost of a cold start.
    MEMORY_PRESSURE_LEVEL_CRITICAL = 2,

    // This must be the last value in the enum. The casing is different from the
    // other values to make this enum work well with the
    // UMA_HISTOGRAM_ENUMERATION macro.
    kMaxValue = MEMORY_PRESSURE_LEVEL_CRITICAL,
  };

  using MemoryPressureCallback = RepeatingCallback<void(MemoryPressureLevel)>;
  using SyncMemoryPressureCallback =
      RepeatingCallback<void(MemoryPressureLevel)>;

  MemoryPressureListener(
      const base::Location& creation_location,
      const MemoryPressureCallback& memory_pressure_callback);
  MemoryPressureListener(
      const base::Location& creation_location,
      const MemoryPressureCallback& memory_pressure_callback,
      const SyncMemoryPressureCallback& sync_memory_pressure_callback);

  MemoryPressureListener(const MemoryPressureListener&) = delete;
  MemoryPressureListener& operator=(const MemoryPressureListener&) = delete;

  ~MemoryPressureListener();

  // Intended for use by the platform specific implementation.
  static void NotifyMemoryPressure(MemoryPressureLevel memory_pressure_level);

  // These methods should not be used anywhere else but in memory measurement
  // code, where they are intended to maintain stable conditions across
  // measurements.
  static bool AreNotificationsSuppressed();
  static void SetNotificationsSuppressed(bool suppressed);
  static void SimulatePressureNotification(
      MemoryPressureLevel memory_pressure_level);

  void Notify(MemoryPressureLevel memory_pressure_level);
  void SyncNotify(MemoryPressureLevel memory_pressure_level);

 private:
  static void DoNotifyMemoryPressure(MemoryPressureLevel memory_pressure_level);

  MemoryPressureCallback callback_;
  SyncMemoryPressureCallback sync_memory_pressure_callback_;

  const base::Location creation_location_;
};

}  // namespace base

#endif  // BASE_MEMORY_MEMORY_PRESSURE_LISTENER_H_
