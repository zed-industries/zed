// Copyright 2021 The Crashpad Authors
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

#ifndef CRASHPAD_UTIL_IOS_IOS_INTERMEDIATE_DUMP_FORMAT_H_
#define CRASHPAD_UTIL_IOS_IOS_INTERMEDIATE_DUMP_FORMAT_H_

#include <stdint.h>

namespace crashpad {
namespace internal {

// Define values for intermediate dump enum class IntermediateDumpKey. Use
// |INTERMEDIATE_DUMP_KEYS| so it is easier to print human readable keys in
// logs.
// clang-format off
#define INTERMEDIATE_DUMP_KEYS(TD) \
  TD(kInvalid, 0) \
  TD(kVersion, 1) \
  TD(kMachException, 1000) \
    TD(kCodes, 1001) \
    TD(kException, 1002) \
    TD(kFlavor, 1003) \
    TD(kState, 1004) \
  TD(kSignalException, 2000) \
    TD(kSignalNumber, 2001) \
    TD(kSignalCode, 2002) \
    TD(kSignalAddress, 2003) \
  TD(kNSException, 2500) \
  TD(kModules, 3000) \
    TD(kAddress, 3001) \
    TD(kFileType, 3002) \
    TD(kName, 3003) \
    TD(kSize, 3004) \
    TD(kDylibCurrentVersion, 3005) \
    TD(kSourceVersion, 3006) \
    TD(kTimestamp, 3007) \
    TD(kUUID, 3008) \
    TD(kAnnotationObjects, 3009) \
    TD(kAnnotationsSimpleMap, 3010) \
    TD(kAnnotationsVector, 3011) \
    TD(kAnnotationType, 3012) \
    TD(kAnnotationName, 3013) \
    TD(kAnnotationValue, 3014) \
    TD(kAnnotationsCrashInfo, 3015) \
    TD(kAnnotationsCrashInfoMessage1, 3016) \
    TD(kAnnotationsCrashInfoMessage2, 3017) \
    TD(kAnnotationsDyldErrorString, 3018) \
    TD(kModuleExtraMemoryRegions, 3019) \
    TD(kModuleExtraMemoryRegionAddress, 3020) \
    TD(kModuleExtraMemoryRegionData, 3021) \
    TD(kModuleIntermediateDumpExtraMemoryRegions, 3022) \
    TD(kModuleIntermediateDumpExtraMemoryRegionAddress, 3023) \
    TD(kModuleIntermediateDumpExtraMemoryRegionData, 3024) \
  TD(kProcessInfo, 4000) \
    TD(kParentPID, 4001) \
    TD(kPID, 4002) \
    TD(kStartTime, 4003) \
    TD(kSnapshotTime, 4004) \
    TD(kTaskBasicInfo, 4005) \
    TD(kTaskThreadTimes, 4006) \
    TD(kSystemTime, 4007) \
    TD(kUserTime, 4008) \
  TD(kSystemInfo, 5000) \
    TD(kCpuCount, 5001) \
    TD(kCpuVendor, 5002) \
    TD(kDaylightName, 5003) \
    TD(kDaylightOffsetSeconds, 5004) \
    TD(kHasDaylightSavingTime, 5005) \
    TD(kIsDaylightSavingTime, 5006) \
    TD(kMachineDescription, 5007) \
    TD(kOSVersionBugfix, 5008) \
    TD(kOSVersionBuild, 5009) \
    TD(kOSVersionMajor, 5010) \
    TD(kOSVersionMinor, 5011) \
    TD(kPageSize, 5012) \
    TD(kStandardName, 5013) \
    TD(kStandardOffsetSeconds, 5014) \
    TD(kVMStat, 5015) \
    TD(kActive, 5016) \
    TD(kFree, 5017) \
    TD(kInactive, 5018) \
    TD(kWired, 5019) \
    TD(kAddressMask, 5020) \
    TD(kCrashpadUptime, 5021) \
  TD(kThreads, 6000) \
    TD(kDebugState, 6001) \
    TD(kFloatState, 6002) \
    TD(kThreadState, 6003) \
    TD(kPriority, 6004) \
    TD(kStackRegionAddress, 6005) \
    TD(kStackRegionData, 6006) \
    TD(kSuspendCount, 6007) \
    TD(kThreadID, 6008) \
    TD(kThreadDataAddress, 6009) \
    TD(kThreadUncaughtNSExceptionFrames, 6010) \
    TD(kThreadContextMemoryRegions, 6011) \
    TD(kThreadContextMemoryRegionAddress, 6012) \
    TD(kThreadContextMemoryRegionData, 6013) \
    TD(kThreadName, 6014) \
  TD(kMaxValue, 65535) \
// clang-format on

//! \brief They key for items in the intermediate dump file.
//!
//! These values are persisted to the intermediate crash dump file. Entries
//! should not be renumbered and numeric values should never be reused.
enum class IntermediateDumpKey : uint16_t {
#define X(NAME, VALUE) NAME = VALUE,
  INTERMEDIATE_DUMP_KEYS(X)
#undef X
};


}  // namespace internal
}  // namespace crashpad

#endif  // CRASHPAD_UTIL_IOS_IOS_INTERMEDIATE_DUMP_FORMAT_H_
