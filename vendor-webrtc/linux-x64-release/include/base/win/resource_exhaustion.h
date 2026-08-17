// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_WIN_RESOURCE_EXHAUSTION_H_
#define BASE_WIN_RESOURCE_EXHAUSTION_H_

#include "base/base_export.h"

namespace base::win {

using OnResourceExhaustedFunction = void (*)();

// Sets a callback to be run in the event that a system resource is exhausted
// such that a system restart is the only recovery. Typically, there is no
// point in letting the process continue execution when this happens.
BASE_EXPORT void SetOnResourceExhaustedFunction(
    OnResourceExhaustedFunction on_resource_exhausted);

// Reports that some system resource has been exhausted. A callback, if provided
// will be run to allow for application-specific handling.
BASE_EXPORT void OnResourceExhausted();

}  // namespace base::win

#endif  // BASE_WIN_RESOURCE_EXHAUSTION_H_
