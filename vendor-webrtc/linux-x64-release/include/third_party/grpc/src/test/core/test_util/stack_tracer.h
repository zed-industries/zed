//
//
// Copyright 2020 the gRPC authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
//

#ifndef GRPC_TEST_CORE_TEST_UTIL_STACK_TRACER_H
#define GRPC_TEST_CORE_TEST_UTIL_STACK_TRACER_H

#include <grpc/support/port_platform.h>

#include <string>

namespace grpc_core {
namespace testing {

// Returns the current stack trace as a string. To have symbolized stack-traces,
// InitializeStackTracer needs to be called beforehand.
//
// Example of stack-trace is
// Stack trace:
//    @           0x405b0f        192  StackTracerTest_Basic_Test::TestBody()
//    @     0x7fbace6baf75        288  testing::internal::RunAllTests()
//    @     0x7fbace6baa93        144  testing::UnitTest::Run()
//    @           0x405d4d         64  main
//
std::string GetCurrentStackTrace();

// Initializes a stack tracer so that GetCurrentStackTrace can work.
// This inits debug symbols and sets this as a gRPC stack-trace provider.
void InitializeStackTracer(const char* argv0);

}  // namespace testing
}  // namespace grpc_core

#endif  // GRPC_TEST_CORE_TEST_UTIL_STACK_TRACER_H
