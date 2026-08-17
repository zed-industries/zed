// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_CONTAINER_ANNOTATIONS_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_CONTAINER_ANNOTATIONS_H_

#include "build/build_config.h"
#include "third_party/blink/renderer/platform/wtf/sanitizers.h"

// TODO(ochang): Remove the ARCH_CPU_X86_64 condition to enable this for X86
// once the crashes there have been fixed: http://crbug.com/461406
#if defined(ADDRESS_SANITIZER) &&                      \
    (BUILDFLAG(IS_LINUX) || BUILDFLAG(IS_CHROMEOS)) && \
    defined(ARCH_CPU_X86_64)

// Annotations require buffers to begin on an 8-byte boundary. See
// documentation:
//   https://github.com/llvm-mirror/compiler-rt/blob/master/include/sanitizer/common_interface_defs.h#L154

#define ANNOTATE_CONTIGUOUS_CONTAINER

#define ANNOTATE_NEW_BUFFER(buffer, capacity, newSize)                       \
  if (buffer) {                                                              \
    __sanitizer_annotate_contiguous_container(buffer, (buffer) + (capacity), \
                                              (buffer) + (capacity),         \
                                              (buffer) + (newSize));         \
  }

#define ANNOTATE_DELETE_BUFFER(buffer, capacity, oldSize)                    \
  if (buffer) {                                                              \
    __sanitizer_annotate_contiguous_container(buffer, (buffer) + (capacity), \
                                              (buffer) + (oldSize),          \
                                              (buffer) + (capacity));        \
  }

#define ANNOTATE_CHANGE_SIZE(buffer, capacity, oldSize, newSize)             \
  if (buffer) {                                                              \
    __sanitizer_annotate_contiguous_container(buffer, (buffer) + (capacity), \
                                              (buffer) + (oldSize),          \
                                              (buffer) + (newSize));         \
  }

#else

#define ANNOTATE_NEW_BUFFER(buffer, capacity, newSize)
#define ANNOTATE_DELETE_BUFFER(buffer, capacity, oldSize)
#define ANNOTATE_CHANGE_SIZE(buffer, capacity, oldSize, newSize)

#endif

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_WTF_CONTAINER_ANNOTATIONS_H_
