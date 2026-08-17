// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_IOS_DEVICE_UTIL_H_
#define BASE_IOS_DEVICE_UTIL_H_

#include <base/types/expected.h>
#include <mach/mach.h>
#include <stdint.h>

#include <string>

namespace ios {
namespace device_util {

// Returns true if the application is running on a device with 512MB or more
// RAM.
bool RamIsAtLeast512Mb();

// Returns true if the application is running on a device with 1024MB or more
// RAM.
bool RamIsAtLeast1024Mb();

// Returns true if the application is running on a device with |ram_in_mb| MB or
// more RAM.
// Use with caution! Actual RAM reported by devices is less than the commonly
// used powers-of-two values. For example, a 512MB device may report only 502MB
// RAM. The convenience methods above should be used in most cases because they
// correctly handle this issue.
bool RamIsAtLeast(uint64_t ram_in_mb);

// Returns true if the device has only one core.
bool IsSingleCoreDevice();

// Returns the MAC address of the interface with name |interface_name|.
std::string GetMacAddress(const std::string& interface_name);

// Returns a random UUID.
std::string GetRandomId();

// Returns an identifier for the device, using the given |salt|. A global
// identifier is generated the first time this method is called, and the salt
// is used to be able to generate distinct identifiers for the same device. If
// |salt| is NULL, a default value is used. Unless you are using this value for
// something that should be anonymous, you should probably pass NULL.
std::string GetDeviceIdentifier(const char* salt);

// Returns the iOS Vendor ID for this device. Using this value can have privacy
// implications.
std::string GetVendorId();

// Returns a hashed version of |in_string| using |salt| (which must not be
// zero-length). Different salt values should result in differently hashed
// strings.
std::string GetSaltedString(const std::string& in_string,
                            const std::string& salt);

// Returns a task_vm_info runtime memory data on Apple platforms in
// case of KERN_SUCCESS. Otherwise returns an error of kern_return_t type.
[[nodiscard]] base::expected<task_vm_info, kern_return_t> GetTaskVMInfo();
}  // namespace device_util
}  // namespace ios

#endif  // BASE_IOS_DEVICE_UTIL_H_
