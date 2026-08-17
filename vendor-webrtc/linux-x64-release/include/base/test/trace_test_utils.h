// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TEST_TRACE_TEST_UTILS_H_
#define BASE_TEST_TRACE_TEST_UTILS_H_

namespace base {

namespace test {

// A scoped class that sets up and tears down tracing support for unit tests.
// Note that only in-process tracing is supported by this harness. See
// //services/tracing for recording traces in multiprocess configurations.
class TracingEnvironment {
 public:
  // Construct a tracing environment using the default Perfetto tracing
  // platform.
  TracingEnvironment();
  ~TracingEnvironment();
};

}  // namespace test
}  // namespace base

#endif  // BASE_TEST_TRACE_TEST_UTILS_H_
