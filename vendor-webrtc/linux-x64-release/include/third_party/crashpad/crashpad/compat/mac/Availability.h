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

#ifndef CRASHPAD_COMPAT_MAC_AVAILABILITY_H_
#define CRASHPAD_COMPAT_MAC_AVAILABILITY_H_

// Until the 10.15 SDK, the contents of <AvailabilityVersions.h> was in-line in
// <Availability.h>, but since then, it was broken out into its own header.
// This compat version of <Availability.h> allows these macros to always appear
// to be provided by the new header, <AvailabilityVersions.h>, even when an
// older SDK is in use.

#include_next <Availability.h>

#include <AvailabilityVersions.h>

#endif  // CRASHPAD_COMPAT_MAC_AVAILABILITY_H_
