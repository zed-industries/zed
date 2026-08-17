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

#ifndef SRC_TRACE_REDACTION_TRACE_REDACTION_INTEGRATION_FIXTURE_H_
#define SRC_TRACE_REDACTION_TRACE_REDACTION_INTEGRATION_FIXTURE_H_

#include <string>

#include "perfetto/ext/base/status_or.h"
#include "src/base/test/tmp_dir_tree.h"
#include "src/trace_redaction/trace_redaction_framework.h"
#include "src/trace_redaction/trace_redactor.h"

namespace perfetto::trace_redaction {

class TraceRedactionIntegrationFixure {
 protected:
  TraceRedactionIntegrationFixure();

  void SetSourceTrace(std::string_view source_file);

  // Redact the source file and write it to the destination file. The contents
  // of each file can be read using LoadOriginal() and LoadRedacted().
  base::Status Redact(const TraceRedactor& redactor, Context* context);

  base::StatusOr<std::string> LoadOriginal() const;

  base::StatusOr<std::string> LoadRedacted() const;

 private:
  base::StatusOr<std::string> ReadRawTrace(const std::string& path) const;

  base::TmpDirTree tmp_dir_;

  std::string src_trace_;

  // Path to the redacted trace. This will only be valid after Redact()
  // completely. If redaction was successful, this file will be tracked by
  // tmp_dir_.
  std::string dest_trace_;
};

}  // namespace perfetto::trace_redaction

#endif  // SRC_TRACE_REDACTION_TRACE_REDACTION_INTEGRATION_FIXTURE_H_
