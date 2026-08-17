// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_MEMORY_SHARED_MEMORY_SWITCH_H_
#define BASE_MEMORY_SHARED_MEMORY_SWITCH_H_

#include <string_view>

#include "base/base_export.h"
#include "base/memory/read_only_shared_memory_region.h"
#include "base/memory/unsafe_shared_memory_region.h"
#include "base/types/expected.h"
#include "build/blink_buildflags.h"
#include "build/build_config.h"

#if !BUILDFLAG(USE_BLINK)
#error "This is only intended for platforms that use blink."
#endif

#if BUILDFLAG(IS_APPLE)
#include "base/apple/mach_port_rendezvous.h"
#endif

#if BUILDFLAG(IS_POSIX) && !BUILDFLAG(IS_APPLE)
#include "base/files/platform_file.h"
#include "base/posix/global_descriptors.h"
#endif

namespace base {

class CommandLine;
struct LaunchOptions;

namespace shared_memory {

// Indicates failure modes of deserializing a shared memory switch value.
enum class SharedMemoryError {
  kNoError,
  kUnexpectedTokensCount,
  kParseInt0Failed,
  kParseInt4Failed,
  kUnexpectedHandleType,
  kInvalidHandle,
  kGetFDFailed,
  kDeserializeGUIDFailed,
  kDeserializeFailed,
  kCreateTrialsFailed,
  kUnexpectedSize,
};

#if BUILDFLAG(IS_APPLE)
#if !BUILDFLAG(IS_IOS_TVOS)
using SharedMemoryMachPortRendezvousKey = MachPortsForRendezvous::key_type;
#else
// Mach port rendezvous is unused on tvOS as it cannot launch new processes.
// This type is provided so the function interface remains compatible with
// other Apple platforms. Functions with this type in their interface are
// not called on tvOS.
using SharedMemoryMachPortRendezvousKey = uint32_t;
#endif
#endif

// Updates `command_line` and `launch_options` to use `switch_name` to pass
// `read_only_memory_region` to child process that is about to be launched.
// This should be called in the parent process as a part of setting up the
// launch conditions of the child. This call will update the `command_line`
// and `launch_options`. On posix, where we prefer to use a zygote instead of
// using the launch_options to launch a new process, the platform
// `descriptor_to_share` is returned. The caller is expected to transmit the
// descriptor to the launch flow for the zygote.
BASE_EXPORT void AddToLaunchParameters(
    std::string_view switch_name,
    const ReadOnlySharedMemoryRegion& read_only_memory_region,
#if BUILDFLAG(IS_APPLE)
    SharedMemoryMachPortRendezvousKey rendezvous_key,
#elif BUILDFLAG(IS_POSIX)
    GlobalDescriptors::Key descriptor_key,
    ScopedFD& out_descriptor_to_share,
#endif
    CommandLine* command_line,
    LaunchOptions* launch_options);

// Updates `command_line` and `launch_options` to use `switch_name` to pass
// `unsafe_memory_region` to a child process that is about to be launched.
// This should be called in the parent process as a part of setting up the
// launch conditions of the child. This call will update the `command_line`
// and `launch_options`. On posix, where we prefer to use a zygote instead of
// using the launch_options to launch a new process, the platform
// `descriptor_to_share` is returned. The caller is expected to transmit the
// descriptor to the launch flow for the zygote.
BASE_EXPORT void AddToLaunchParameters(
    std::string_view switch_name,
    const UnsafeSharedMemoryRegion& unsafe_memory_region,
#if BUILDFLAG(IS_APPLE)
    SharedMemoryMachPortRendezvousKey rendezvous_key,
#elif BUILDFLAG(IS_POSIX)
    GlobalDescriptors::Key descriptor_key,
    ScopedFD& out_descriptor_to_share,
#endif
    CommandLine* command_line,
    LaunchOptions* launch_options);

// Returns an UnsafeSharedMemoryRegion deserialized from `switch_value`.
BASE_EXPORT expected<UnsafeSharedMemoryRegion, SharedMemoryError>
UnsafeSharedMemoryRegionFrom(std::string_view switch_value);

// Returns an ReadOnlySharedMemoryRegion deserialized from `switch_value`.
BASE_EXPORT expected<ReadOnlySharedMemoryRegion, SharedMemoryError>
ReadOnlySharedMemoryRegionFrom(std::string_view switch_value);

}  // namespace shared_memory
}  // namespace base

#endif  // BASE_MEMORY_SHARED_MEMORY_SWITCH_H_
