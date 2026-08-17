// Copyright 2025 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// This is a header which wraps optional to avoid missing
// libcpp_verbose_abort().

#ifndef PARTITION_ALLOC_PARTITION_ALLOC_BASE_CXX_WRAPPER_OPTIONAL_H_
#define PARTITION_ALLOC_PARTITION_ALLOC_BASE_CXX_WRAPPER_OPTIONAL_H_

#include "partition_alloc/build_config.h"

#if PA_BUILDFLAG(IS_WIN) && defined(COMPONENT_BUILD)
#include "partition_alloc/partition_alloc_base/check.h"

// To enable allocator_shim for the windows component build chrome,
// we updated `common_deps` to make all shared libraries and executables
// depend on PartitionAlloc. But this caused PartitionAlloc depend on
// PartitionAlloc. So we need `no_default_deps = true` (this also means
// removing libc++ dependency from PartitionAlloc. If PartitionAlloc
// depends on libc++, we will see deps cycle: PA => libc++ => PA.)
// c.f. base/allocator/partition_alloc/src/partition_alloc/BUILD.gn and
// build/config/BUILD.gn.
// However if we forbidden all libc++ inside PartitionAlloc, lots of
// useful classes, methods, and functions will be unavailable...
// So we will use only inlined ones, because they don't cause libc++.dll
// dependency (e.g. std::min, std::optional).
// Unfortunately some of classes, methods, ... look inlined but depend
// on LIBCPP_VERBOSE_ABORT, i.e. std::__libcpp_verbose_abort(...).
// c.f. buildtools/third_party/libc++/__config_site
// So we will replace LIBCPP_VERBOSE_ABORT() with PartitionAlloc's one
// and will remove the libc++.dll dependency from PartitionAlloc.

#pragma push_macro("_LIBCPP_VERBOSE_ABORT")
#undef _LIBCPP_VERBOSE_ABORT
#define _LIBCPP_VERBOSE_ABORT(...) \
  ::partition_alloc::internal::logging::RawCheckFailureFormat(__VA_ARGS__)
#include <optional>
#pragma pop_macro("_LIBCPP_VERBOSE_ABORT")
#else
#include <optional>
#endif  // PA_BUILDFLAG(IS_WIN) && defined(COMPONENT_BUILD)

#endif  // PARTITION_ALLOC_PARTITION_ALLOC_BASE_CXX_WRAPPER_OPTIONAL_H_
