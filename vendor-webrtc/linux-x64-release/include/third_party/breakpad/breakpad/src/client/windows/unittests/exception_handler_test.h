// Copyright 2012 Google LLC
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are
// met:
//
//     * Redistributions of source code must retain the above copyright
// notice, this list of conditions and the following disclaimer.
//     * Redistributions in binary form must reproduce the above
// copyright notice, this list of conditions and the following disclaimer
// in the documentation and/or other materials provided with the
// distribution.
//     * Neither the name of Google LLC nor the names of its
// contributors may be used to endorse or promote products derived from
// this software without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
// LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
// OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
// LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
// DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
// THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
// (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
// OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

#ifndef CLIENT_WINDOWS_UNITTESTS_EXCEPTION_HANDLER_TEST_H_
#define CLIENT_WINDOWS_UNITTESTS_EXCEPTION_HANDLER_TEST_H_

namespace testing {

// By default, GTest (on Windows) installs a SEH filter (and a handler) before
// starting to run all the tests in order to avoid test interruptions is some
// of the tests are crashing.  Unfortunately, this functionality prevents the
// execution to reach the UnhandledExceptionFilter installed by Google-Breakpad
// ExceptionHandler so in order to test the Google-Breakpad exception handling
// code the exception handling done by GTest must be disabled.
// Usage:
//
//  google_breakpad::ExceptionHandler exc(...);
//
//  // Disable GTest SEH handler
//  testing::DisableExceptionHandlerInScope disable_exception_handler;
//  ...
//  ASSERT_DEATH( ... some crash ...);
//
class DisableExceptionHandlerInScope {
 public:
  DisableExceptionHandlerInScope();
  ~DisableExceptionHandlerInScope();

 private:
  bool catch_exceptions_;
};

}  // namespace testing

#endif  // CLIENT_WINDOWS_UNITTESTS_EXCEPTION_HANDLER_TEST_H_
