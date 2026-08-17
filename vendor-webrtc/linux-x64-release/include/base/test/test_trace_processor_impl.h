// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// TestTraceProcessorImpl encapsulates Perfetto's TraceProcessor. This is needed
// to prevent symbol conflicts between libtrace_processor and libperfetto.

#ifndef BASE_TEST_TEST_TRACE_PROCESSOR_IMPL_H_
#define BASE_TEST_TEST_TRACE_PROCESSOR_IMPL_H_

#include <memory>

#include "test_trace_processor_export.h"
#include "third_party/abseil-cpp/absl/status/status.h"

namespace perfetto::trace_processor {
struct Config;
class TraceProcessor;
}  // namespace perfetto::trace_processor

namespace base::test {

// Chrome uses a custom memory allocator in non-component builds that manages
// all allocations on the heap. Since test_trace_processor is a separate library
// even in a non-component build, Chrome is not able to free memory allocated by
// the code in this library. This leads to crashes when e.g. vectors created
// here are passed to the code in Chrome tests.
// So we define a custom class to hold the result of the query execution.
// Since its destructor is defined in this library, it will be freed using
// the same allocator that was used to create it.
// See crbug.com/1453617 for more info.
class TEST_TRACE_PROCESSOR_EXPORT QueryResultOrError {
 public:
  using QueryResult = std::vector<std::vector<std::string>>;

  QueryResultOrError(const QueryResult& result);
  QueryResultOrError(const std::string& error);

  bool ok() const { return error_.empty(); }

  const QueryResult& result() const { return result_; }

  const std::string& error() const { return error_; }

  ~QueryResultOrError();

 private:
  QueryResult result_;
  std::string error_;
};

class TEST_TRACE_PROCESSOR_EXPORT TestTraceProcessorImpl {
 public:
  // Note: All arguments must be received as refs/ptrs as receiving them
  // as moved copies, on Windows, causes them to be destroyed in
  // TEST_TRACE_PROCESSOR_IMPL's DLL after having been allocated in the
  // caller's DLL which is not allowed.

  TestTraceProcessorImpl();
  ~TestTraceProcessorImpl();

  TestTraceProcessorImpl(TestTraceProcessorImpl&& other);
  TestTraceProcessorImpl& operator=(TestTraceProcessorImpl&& other);

  absl::Status ParseTrace(const std::vector<char>& raw_trace);

  // Runs the sql query on the parsed trace and returns the result as a
  // vector of strings.
  QueryResultOrError ExecuteQuery(const std::string& sql) const;

  using PerfettoSQLModule = std::vector<std::pair<std::string, std::string>>;
  // Overrides PerfettoSQL module with |name| and |files| containing pairs of
  // strings {include_key, sql_file_contents}.
  absl::Status OverrideSqlModule(const std::string& name,
                                 const PerfettoSQLModule& module);

 private:
  std::unique_ptr<perfetto::trace_processor::Config> config_;
  std::unique_ptr<perfetto::trace_processor::TraceProcessor> trace_processor_;
};

}  // namespace base::test

#endif  // BASE_TEST_TEST_TRACE_PROCESSOR_IMPL_H_
