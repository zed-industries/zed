// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef PARTITION_ALLOC_PARTITION_ALLOC_BASE_WIN_WIN_HANDLE_TYPES_H_
#define PARTITION_ALLOC_PARTITION_ALLOC_BASE_WIN_WIN_HANDLE_TYPES_H_

// Forward declare Windows compatible handles.

#define PA_WINDOWS_HANDLE_TYPE(name) \
  struct name##__;                   \
  typedef struct name##__* name;
#include "partition_alloc/partition_alloc_base/win/win_handle_types_list.inc"
#undef PA_WINDOWS_HANDLE_TYPE

#endif  // PARTITION_ALLOC_PARTITION_ALLOC_BASE_WIN_WIN_HANDLE_TYPES_H_
