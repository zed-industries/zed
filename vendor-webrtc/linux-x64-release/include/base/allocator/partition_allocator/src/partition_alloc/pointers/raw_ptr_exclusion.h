// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// IWYU pragma: private, include "base/memory/raw_ptr_exclusion.h"

#ifndef PARTITION_ALLOC_POINTERS_RAW_PTR_EXCLUSION_H_
#define PARTITION_ALLOC_POINTERS_RAW_PTR_EXCLUSION_H_

// This header will be leakily included even when
// `!use_partition_alloc`, which is okay because it's a leaf header.
#include "partition_alloc/build_config.h"
#include "partition_alloc/buildflags.h"
#include "partition_alloc/partition_alloc_base/compiler_specific.h"  // nogncheck

#if PA_HAS_ATTRIBUTE(annotate)
#if defined(OFFICIAL_BUILD) && !PA_BUILDFLAG(FORCE_ENABLE_RAW_PTR_EXCLUSION)
// The annotation changed compiler output and increased binary size so disable
// for official builds.
// TODO(crbug.com/40836910): Remove when issue is resolved.
#define RAW_PTR_EXCLUSION
#else
// Marks a field as excluded from the `raw_ptr<T>` usage enforcement via
// Chromium Clang plugin.
//
// Example:
//     RAW_PTR_EXCLUSION Foo* foo_;
//
// `RAW_PTR_EXCLUSION` should be avoided, as exclusions makes it significantly
// easier for any bug involving the pointer to become a security vulnerability.
// For additional guidance please see the "When to use raw_ptr<T>" section of
// `//base/memory/raw_ptr.md`.
#define RAW_PTR_EXCLUSION __attribute__((annotate("raw_ptr_exclusion")))
#endif
#else
#define RAW_PTR_EXCLUSION
#endif

#endif  // PARTITION_ALLOC_POINTERS_RAW_PTR_EXCLUSION_H_
