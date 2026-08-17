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

#ifndef SRC_TRACE_REDACTION_TRACE_REDACTOR_H_
#define SRC_TRACE_REDACTION_TRACE_REDACTOR_H_

#include <memory>
#include <string>
#include <string_view>
#include <vector>

#include "perfetto/base/status.h"
#include "perfetto/trace_processor/trace_blob_view.h"
#include "src/trace_redaction/trace_redaction_framework.h"

namespace perfetto::trace_redaction {

// Removes sensitive information from Perfetto traces by executing collect,
// build, and transforms primitives in the correct order.
//
// The caller is responsible for adding all necessary primitives. Primitives
// are not directly dependent on each other, but rather dependent on the
// information inside of the context.
class TraceRedactor {
 public:
  TraceRedactor();
  virtual ~TraceRedactor();

  // Entry point for redacting a trace. Regardless of success/failure, `context`
  // will contain the current state.
  base::Status Redact(std::string_view source_filename,
                      std::string_view dest_filename,
                      Context* context) const;

  // T must be derived from trace_redaction::CollectPrimitive.
  template <typename T>
  T* emplace_collect() {
    auto uptr = std::make_unique<T>();
    auto* ptr = uptr.get();
    collectors_.push_back(std::move(uptr));
    return ptr;
  }

  // T must be derived from trace_redaction::BuildPrimitive.
  template <typename T>
  T* emplace_build() {
    auto uptr = std::make_unique<T>();
    auto* ptr = uptr.get();
    builders_.push_back(std::move(uptr));
    return ptr;
  }

  // T must be derived from trace_redaction::TransformPrimitive.
  template <typename T>
  T* emplace_transform() {
    auto uptr = std::make_unique<T>();
    auto* ptr = uptr.get();
    transformers_.push_back(std::move(uptr));
    return ptr;
  }

  struct Config {
    // Controls whether or not the verify primitive is added to the pipeline.
    // This should always be enabled unless you know that your test content
    // fails verification.
    bool verify = true;
  };

  static std::unique_ptr<TraceRedactor> CreateInstance(const Config& config);

 private:
  // Run all collectors on a packet because moving to the next package.
  //
  // ```
  //  with context:
  //   for packet in packets:
  //     for collector in collectors:
  //       collector(context, packet)
  // ```
  base::Status Collect(Context* context,
                       const trace_processor::TraceBlobView& view) const;

  // Runs builders once.
  //
  // ```
  //  with context:
  //   for builder in builders:
  //      builder(context)
  // ```
  base::Status Build(Context* context) const;

  // Runs all transformers on a packet before moving to the next package.
  //
  // ```
  //  with context:
  //   for packet in packets:
  //     for transform in transformers:
  //       transform(context, packet)
  // ```
  base::Status Transform(const Context& context,
                         const trace_processor::TraceBlobView& view,
                         const std::string& dest_file) const;

  std::vector<std::unique_ptr<CollectPrimitive>> collectors_;
  std::vector<std::unique_ptr<BuildPrimitive>> builders_;
  std::vector<std::unique_ptr<TransformPrimitive>> transformers_;
};

}  // namespace perfetto::trace_redaction

#endif  // SRC_TRACE_REDACTION_TRACE_REDACTOR_H_
