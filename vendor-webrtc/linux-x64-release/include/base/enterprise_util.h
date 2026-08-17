// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_ENTERPRISE_UTIL_H_
#define BASE_ENTERPRISE_UTIL_H_

#include "base/base_export.h"
#include "build/build_config.h"

namespace base {

// Returns true if an outside entity manages the current machine. To be
// "managed" means that an entity such as a company or school is applying
// policies to this device. This is primarily checking the device for MDM
// management.
// Not all managed devices are enterprise devices, as BYOD (bring your own
// device) is becoming more common in connection with workplace joining of
// personal computers.
BASE_EXPORT bool IsManagedDevice();

// Returns true if the device should be considered an enterprise device. To be
// an enterprise device means that the enterprise actually owns or has complete
// control over a device. This is primarily checking if the device is joined to
// a domain.
// Not all enterprise devices are managed devices because not all enterprises
// actually apply policies to all devices.
BASE_EXPORT bool IsEnterpriseDevice();

// Returns true if the device is either managed or enterprise. In general, it is
// recommended to use the PlatformManagementService to obtain this information,
// if possible.
BASE_EXPORT bool IsManagedOrEnterpriseDevice();

#if BUILDFLAG(IS_APPLE)

// Returns the state of the management of the device. For more details:
// https://blog.fleetsmith.com/what-is-user-approved-mdm-uamdm/ .

// These values are persisted to logs. Entries must not be renumbered and
// numeric values must never be reused.
enum class MacDeviceManagementState {
  kFailureAPIUnavailable = 0,
  kFailureUnableToParseResult = 1,
  kNoEnrollment = 2,
  kLimitedMDMEnrollment = 3,
  kFullMDMEnrollment = 4,
  kDEPMDMEnrollment = 5,

  kMaxValue = kDEPMDMEnrollment
};
BASE_EXPORT MacDeviceManagementState IsDeviceRegisteredWithManagement();

// Returns whether the device and/or the current user is enrolled to a domain.
struct DeviceUserDomainJoinState {
  bool device_joined;
  bool user_joined;
};
BASE_EXPORT DeviceUserDomainJoinState AreDeviceAndUserJoinedToDomain();

#endif  // BUILDFLAG(IS_APPLE)

}  // namespace base

#endif  // BASE_ENTERPRISE_UTIL_H_
