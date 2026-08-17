/*
 * Copyright (C) 2025 The Android Open Source Project
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#ifndef SRC_KERNEL_UTILS_KERNEL_WAKELOCK_ERRORS_H_
#define SRC_KERNEL_UTILS_KERNEL_WAKELOCK_ERRORS_H_

#include <cinttypes>

constexpr uint64_t kKernelWakelockErrorZeroValue = 1;
constexpr uint64_t kKernelWakelockErrorNonMonotonicValue = 2;

#endif  // SRC_KERNEL_UTILS_KERNEL_WAKELOCK_ERRORS_H_
