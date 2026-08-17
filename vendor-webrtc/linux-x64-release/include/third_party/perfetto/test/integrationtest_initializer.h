/*
 * Copyright (C) 2024 The Android Open Source Project
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

#ifndef TEST_INTEGRATIONTEST_INITIALIZER_H_
#define TEST_INTEGRATIONTEST_INITIALIZER_H_

namespace perfetto::integration_tests {

// Simple mechanism to execute code at the beginning of the integrationtest
// main() before the gtest tests are run.
//
// Usage
// ```
// int PERFETTO_UNUSED initializer =
//     integration_tests::Register...Initializer(
//         &InitializerFunction);
// ```
//
// This is probably more verbose than required to keep the implementation
// straightforward and avoid as much as possible all the pitfalls of static
// initialization order.

// Implemented in integrationtest_main.cc

int RegisterHeapprofdEndToEndTestInitializer(void (*fn)(void));
int RegisterApiIntegrationTestInitializer(void (*fn)(void));

}  // namespace perfetto::integration_tests

#endif  // TEST_INTEGRATIONTEST_INITIALIZER_H_
