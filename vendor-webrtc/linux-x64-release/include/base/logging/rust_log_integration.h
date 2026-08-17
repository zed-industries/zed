// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_LOGGING_RUST_LOG_INTEGRATION_H_
#define BASE_LOGGING_RUST_LOG_INTEGRATION_H_

#include <stddef.h>
#include <stdint.h>

#include "base/base_export.h"
#include "base/logging.h"
#include "base/logging/log_severity.h"
#include "third_party/rust/cxx/v1/cxx.h"

namespace logging {
namespace internal {

// Opaquely wraps a Rust std::fmt::Arguments object, which can be turned into a
// string but must be done so from a Rust stack frame with the help of
// LogMessageRustWrapper below.
struct RustFmtArguments;

// Receives a log line from Rust and forwards it to base logging, because
// logging::LogMessage is not accessible from Rust yet with our current interop
// tools.
//
// TODO(danakj): Should this helper function be replaced with C-like apis next
// to logging::LogMessage that Rust uses more directly?
void print_rust_log(const RustFmtArguments& msg,
                    const char* file,
                    int32_t line,
                    int32_t severity,
                    bool verbose);

// Wraps a LogMessage object so that Rust code can write to its ostream.
class LogMessageRustWrapper {
 public:
  LogMessageRustWrapper(const char* file,
                        int line,
                        ::logging::LogSeverity severity);

  void write_to_stream(rust::Str str);

 private:
  ::logging::LogMessage log_message;
};

}  // namespace internal
}  // namespace logging

#endif  // BASE_LOGGING_RUST_LOG_INTEGRATION_H_
