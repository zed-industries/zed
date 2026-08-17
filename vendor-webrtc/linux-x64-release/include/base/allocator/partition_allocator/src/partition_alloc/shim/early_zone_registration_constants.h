// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef PARTITION_ALLOC_SHIM_EARLY_ZONE_REGISTRATION_CONSTANTS_H_
#define PARTITION_ALLOC_SHIM_EARLY_ZONE_REGISTRATION_CONSTANTS_H_

// This is an Apple-only file, used to register PartitionAlloc's zone *before*
// the process becomes multi-threaded. These constants are shared between the
// allocator shim which installs the PartitionAlloc's malloc zone and the
// application which installs the "early malloc zone" to reserve the zone slot.

namespace allocator_shim {

static constexpr char kDelegatingZoneName[] =
    "DelegatingDefaultZoneForPartitionAlloc";
static constexpr char kPartitionAllocZoneName[] = "PartitionAlloc";

// Zone version. Determines which callbacks are set in the various malloc_zone_t
// structs.
#if (__MAC_OS_X_VERSION_MAX_ALLOWED >= 130000) || \
    (__IPHONE_OS_VERSION_MAX_ALLOWED >= 160100)
#define PA_TRY_FREE_DEFAULT_IS_AVAILABLE 1
#endif
#if PA_TRY_FREE_DEFAULT_IS_AVAILABLE
constexpr int kZoneVersion = 13;
#else
constexpr int kZoneVersion = 9;
#endif

}  // namespace allocator_shim

#endif  // PARTITION_ALLOC_SHIM_EARLY_ZONE_REGISTRATION_CONSTANTS_H_
