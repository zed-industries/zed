// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_ALLOCATOR_EARLY_ZONE_REGISTRATION_APPLE_H_
#define BASE_ALLOCATOR_EARLY_ZONE_REGISTRATION_APPLE_H_

// This is an Apple-only file, used to register PartitionAlloc's zone *before*
// the process becomes multi-threaded.

namespace partition_alloc {

// Must be called *once*, *before* the process becomes multi-threaded.
void EarlyMallocZoneRegistration();

// Tricks the registration code to believe that PartitionAlloc was not already
// registered. This allows a future library load to register PartitionAlloc's
// zone as well, rather than bailing out.
//
// This is mutually exclusive with EarlyMallocZoneRegistration(), and should
// ideally be removed. Indeed, by allowing two zones to be registered, we still
// end up with a split heap, and more memory usage.
//
// This is a hack for https://crbug.com/1274236.
void AllowDoublePartitionAllocZoneRegistration();

}  // namespace partition_alloc

#endif  // BASE_ALLOCATOR_EARLY_ZONE_REGISTRATION_APPLE_H_
