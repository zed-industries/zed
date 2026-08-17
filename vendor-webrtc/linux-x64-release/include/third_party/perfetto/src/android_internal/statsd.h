/*
 * Copyright (C) 2023 The Android Open Source Project
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

#ifndef SRC_ANDROID_INTERNAL_STATSD_H_
#define SRC_ANDROID_INTERNAL_STATSD_H_

#include <stddef.h>
#include <stdint.h>

// This header declares proxy functions defined in
// libperfetto_android_internal.so that access internal android functions (e.g.
// hwbinder).
// Do not add any include to either perfetto headers or android headers. See
// README.md for more.

namespace perfetto {
namespace android_internal {

extern "C" {

// These functions are not thread safe unless specified otherwise.

const uint32_t kAtomCallbackReasonStatsdInitiated = 1;
const uint32_t kAtomCallbackReasonFlushRequested = 2;
const uint32_t kAtomCallbackReasonSubscriptionEnded = 3;

typedef void (*AtomCallback)(int32_t subscription_id,
                             uint32_t reason,
                             uint8_t* payload,
                             size_t num_bytes,
                             void* cookie);

int32_t __attribute__((visibility("default"))) AddAtomSubscription(
    const uint8_t* subscription_config,
    size_t num_bytes,
    AtomCallback callback,
    void* cookie);

void __attribute__((visibility("default"))) RemoveAtomSubscription(
    int32_t subscription_id);

void __attribute__((visibility("default"))) FlushAtomSubscription(
    int32_t subscription_id);

}  // extern "C"

}  // namespace android_internal
}  // namespace perfetto

#endif  // SRC_ANDROID_INTERNAL_STATSD_H_
