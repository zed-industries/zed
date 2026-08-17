/*
 *  Copyright (c) 2015 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef SDK_OBJC_NATIVE_SRC_AUDIO_HELPERS_H_
#define SDK_OBJC_NATIVE_SRC_AUDIO_HELPERS_H_

#import <Foundation/Foundation.h>
#include <objc/objc.h>

#include <string>

namespace webrtc {
namespace ios {

bool CheckAndLogError(BOOL success, NSError* error);

NSString* NSStringFromStdString(const std::string& stdString);
std::string StdStringFromNSString(NSString* nsString);

// Return thread ID as a string.
std::string GetThreadId();

// Return thread ID as string suitable for debug logging.
std::string GetThreadInfo();

// Returns [NSThread currentThread] description as string.
// Example: <NSThread: 0x170066d80>{number = 1, name = main}
std::string GetCurrentThreadDescription();

#if defined(WEBRTC_IOS)
// Returns the current name of the operating system.
std::string GetSystemName();

// Returns the current version of the operating system as a string.
std::string GetSystemVersionAsString();

// Returns the version of the operating system in double representation.
// Uses a cached value of the system version.
double GetSystemVersion();

// Returns the device type.
// Examples: ”iPhone” and ”iPod touch”.
std::string GetDeviceType();
#endif  // defined(WEBRTC_IOS)

// Returns a more detailed device name.
// Examples: "iPhone 5s (GSM)" and "iPhone 6 Plus".
std::string GetDeviceName();

// Returns the name of the process. Does not uniquely identify the process.
std::string GetProcessName();

// Returns the identifier of the process (often called process ID).
int GetProcessID();

// Returns a string containing the version of the operating system on which the
// process is executing. The string is string is human readable, localized, and
// is appropriate for displaying to the user.
std::string GetOSVersionString();

// Returns the number of processing cores available on the device.
int GetProcessorCount();

#if defined(WEBRTC_IOS)
// Indicates whether Low Power Mode is enabled on the iOS device.
bool GetLowPowerModeEnabled();
#endif

}  // namespace ios
}  // namespace webrtc

#endif  // SDK_OBJC_NATIVE_SRC_AUDIO_HELPERS_H_
