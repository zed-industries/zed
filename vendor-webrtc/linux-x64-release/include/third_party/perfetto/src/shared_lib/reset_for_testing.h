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

#ifndef SRC_SHARED_LIB_RESET_FOR_TESTING_H_
#define SRC_SHARED_LIB_RESET_FOR_TESTING_H_

#include "perfetto/public/abi/data_source_abi.h"

// This headers declares a few functions that are exposed only to tests

namespace perfetto {
namespace shlib {

// Resets the shared library thread local state for data sources on the current
// thread.
void ResetDataSourceTls();

// Resets the shared library thread local state for track event on the current
// thread.
void ResetTrackEventTls();

// Destroys a registered data source. This only works after ResetForTesting().
void DsImplDestroy(PerfettoDsImpl*);

// Uninitializes the shared library as best as it can. Only exposed for testing
// scenarios where it can be guaranteed that no tracing sessions or other
// operations are happening when this call is made.
void ResetForTesting();

}  // namespace shlib
}  // namespace perfetto

#endif  // SRC_SHARED_LIB_RESET_FOR_TESTING_H_
