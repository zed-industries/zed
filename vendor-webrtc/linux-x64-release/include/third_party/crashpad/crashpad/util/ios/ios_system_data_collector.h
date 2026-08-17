// Copyright 2020 The Crashpad Authors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef CRASHPAD_UTIL_IOS_IOS_SYSTEM_DATA_COLLECTOR_H_
#define CRASHPAD_UTIL_IOS_IOS_SYSTEM_DATA_COLLECTOR_H_

#import <CoreFoundation/CoreFoundation.h>

#include <functional>
#include <string>

namespace crashpad {
namespace internal {

//! \brief Used to collect system level data before a crash occurs.
class IOSSystemDataCollector {
 public:
  IOSSystemDataCollector();
  ~IOSSystemDataCollector();

  void OSVersion(int* major, int* minor, int* bugfix) const;
  const std::string& MachineDescription() const { return machine_description_; }
  int ProcessorCount() const { return processor_count_; }
  const std::string& Build() const { return build_; }
  const std::string& BundleIdentifier() const { return bundle_identifier_; }
  bool IsExtension() const { return is_extension_; }
  const std::string& CPUVendor() const { return cpu_vendor_; }
  bool HasDaylightSavingTime() const { return has_next_daylight_saving_time_; }
  bool IsDaylightSavingTime() const { return is_daylight_saving_time_; }
  int StandardOffsetSeconds() const { return standard_offset_seconds_; }
  int DaylightOffsetSeconds() const { return daylight_offset_seconds_; }
  const std::string& StandardName() const { return standard_name_; }
  const std::string& DaylightName() const { return daylight_name_; }
  bool IsApplicationActive() const { return active_; }
  uint64_t AddressMask() const { return address_mask_; }
  uint64_t InitializationTime() const { return initialization_time_ns_; }

  // Currently unused by minidump.
  int Orientation() const { return orientation_; }

  // A completion callback that takes a bool indicating that the application has
  // become active or inactive.
  using ActiveApplicationCallback = std::function<void(bool)>;

  void SetActiveApplicationCallback(ActiveApplicationCallback callback) {
    active_application_callback_ = callback;
  }

 private:
  // Notification handlers for time zone, orientation and active state.
  void InstallHandlers();
  void SystemTimeZoneDidChangeNotification();
  void OrientationDidChangeNotification();
  void ApplicationDidChangeActiveNotification();

  int major_version_;
  int minor_version_;
  int patch_version_;
  std::string build_;
  std::string bundle_identifier_;
  bool is_extension_;
  std::string machine_description_;
  int orientation_;
  bool active_;
  int processor_count_;
  std::string cpu_vendor_;
  bool has_next_daylight_saving_time_;
  bool is_daylight_saving_time_;
  int standard_offset_seconds_;
  int daylight_offset_seconds_;
  std::string standard_name_;
  std::string daylight_name_;
  ActiveApplicationCallback active_application_callback_;
  uint64_t address_mask_;

  // Time in nanoseconds as returned by ClockMonotonicNanoseconds() to store the
  // crashpad start time. This clock increments monotonically but pauses while
  // the system is asleep. It should not be compared to other system time
  // sources.
  uint64_t initialization_time_ns_;
};

}  // namespace internal
}  // namespace crashpad

#endif  // CRASHPAD_UTIL_IOS_IOS_SYSTEM_DATA_COLLECTOR_H_
