// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef PARTITION_ALLOC_PARTITION_ALLOC_FORWARD_H_
#define PARTITION_ALLOC_PARTITION_ALLOC_FORWARD_H_

#include <cstddef>
#include <cstdint>
#include <type_traits>

#include "partition_alloc/buildflags.h"
#include "partition_alloc/partition_alloc_base/compiler_specific.h"
#include "partition_alloc/partition_alloc_base/component_export.h"
#include "partition_alloc/partition_alloc_base/cxx_wrapper/algorithm.h"
#include "partition_alloc/partition_alloc_base/thread_annotations.h"
#include "partition_alloc/partition_alloc_config.h"

namespace partition_alloc {

namespace internal {

// Alignment has two constraints:
// - Alignment requirement for scalar types: alignof(std::max_align_t)
// - Alignment requirement for operator new().
//
// The two are separate on Windows 64 bits, where the first one is 8 bytes, and
// the second one 16. We could technically return something different for
// malloc() and operator new(), but this would complicate things, and most of
// our allocations are presumably coming from operator new() anyway.
constexpr size_t kAlignment =
    std::max(alignof(max_align_t),
             static_cast<size_t>(__STDCPP_DEFAULT_NEW_ALIGNMENT__));
static_assert(kAlignment <= 16,
              "PartitionAlloc doesn't support a fundamental alignment larger "
              "than 16 bytes.");

class PA_LOCKABLE Lock;

// This type trait verifies a type can be used as a pointer offset.
//
// We support pointer offsets in signed (ptrdiff_t) or unsigned (size_t) values.
// Smaller types are also allowed.
template <typename Z>
static constexpr bool is_offset_type =
    std::is_integral_v<Z> && sizeof(Z) <= sizeof(ptrdiff_t);

enum class MetadataKind { kWritable, kReadOnly };

template <const MetadataKind kind, typename T>
struct MaybeConst {
  using Type = std::conditional_t<kind == MetadataKind::kReadOnly, T const, T>;
};

template <const MetadataKind kind, typename T>
using MaybeConstT = typename MaybeConst<kind, T>::Type;

template <MetadataKind>
struct SlotSpanMetadata;

}  // namespace internal

class PartitionStatsDumper;

struct PartitionRoot;

namespace internal {
// Declare PartitionRootLock() for thread analysis. Its implementation
// is defined in partition_root.h.
Lock& PartitionRootLock(PartitionRoot*);
}  // namespace internal

}  // namespace partition_alloc

// From https://clang.llvm.org/docs/AttributeReference.html#malloc:
//
// The malloc attribute indicates that the function acts like a system memory
// allocation function, returning a pointer to allocated storage disjoint from
// the storage for any other object accessible to the caller.
//
// Note that it doesn't apply to realloc()-type functions, as they can return
// the same pointer as the one passed as a parameter, as noted in e.g. stdlib.h
// on Linux systems.
#if PA_HAS_ATTRIBUTE(malloc)
#define PA_MALLOC_FN __attribute__((malloc))
#endif

#if !defined(PA_MALLOC_FN)
#define PA_MALLOC_FN
#endif

#if !defined(PA_MALLOC_ALIGNED)
#define PA_MALLOC_ALIGNED
#endif

#endif  // PARTITION_ALLOC_PARTITION_ALLOC_FORWARD_H_
