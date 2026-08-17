// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef PARTITION_ALLOC_PARTITION_ALLOC_BASE_FUCHSIA_FUCHSIA_LOGGING_H_
#define PARTITION_ALLOC_PARTITION_ALLOC_BASE_FUCHSIA_FUCHSIA_LOGGING_H_

#include <lib/fit/function.h>
#include <zircon/types.h>

#include "partition_alloc/build_config.h"
#include "partition_alloc/buildflags.h"
#include "partition_alloc/partition_alloc_base/component_export.h"
#include "partition_alloc/partition_alloc_base/logging.h"

// Use the PA_ZX_LOG family of macros along with a zx_status_t containing a
// Zircon error. The error value will be decoded so that logged messages explain
// the error.

namespace partition_alloc::internal::logging {

class PA_COMPONENT_EXPORT(PARTITION_ALLOC_BASE) ZxLogMessage
    : public logging::LogMessage {
 public:
  ZxLogMessage(const char* file_path,
               int line,
               LogSeverity severity,
               zx_status_t zx_err);

  ZxLogMessage(const ZxLogMessage&) = delete;
  ZxLogMessage& operator=(const ZxLogMessage&) = delete;

  ~ZxLogMessage() override;

 private:
  zx_status_t zx_err_;
};

}  // namespace partition_alloc::internal::logging

#define PA_ZX_LOG_STREAM(severity, zx_err) \
  PA_COMPACT_GOOGLE_PLOG_EX_##severity(ZxLogMessage, zx_err).stream()

#define PA_ZX_LOG(severity, zx_err) \
  PA_LAZY_STREAM(PA_ZX_LOG_STREAM(severity, zx_err), PA_LOG_IS_ON(severity))
#define PA_ZX_LOG_IF(severity, condition, zx_err)    \
  PA_LAZY_STREAM(PA_ZX_LOG_STREAM(severity, zx_err), \
                 PA_LOG_IS_ON(severity) && (condition))

#define PA_ZX_CHECK(condition, zx_err)                          \
  PA_LAZY_STREAM(PA_ZX_LOG_STREAM(FATAL, zx_err), !(condition)) \
      << "Check failed: " #condition << ". "

#define PA_ZX_DLOG(severity, zx_err) \
  PA_LAZY_STREAM(PA_ZX_LOG_STREAM(severity, zx_err), PA_DLOG_IS_ON(severity))

#if PA_BUILDFLAG(DCHECKS_ARE_ON)
#define PA_ZX_DLOG_IF(severity, condition, zx_err)   \
  PA_LAZY_STREAM(PA_ZX_LOG_STREAM(severity, zx_err), \
                 PA_DLOG_IS_ON(severity) && (condition))
#else  // PA_BUILDFLAG(DCHECKS_ARE_ON)
#define PA_ZX_DLOG_IF(severity, condition, zx_err) PA_EAT_STREAM_PARAMETERS
#endif  // PA_BUILDFLAG(DCHECKS_ARE_ON)

#define PA_ZX_DCHECK(condition, zx_err)                        \
  PA_LAZY_STREAM(PA_ZX_LOG_STREAM(DCHECK, zx_err),             \
                 PA_BUILDFLAG(DCHECKS_ARE_ON) && !(condition)) \
      << "Check failed: " #condition << ". "

#endif  // PARTITION_ALLOC_PARTITION_ALLOC_BASE_FUCHSIA_FUCHSIA_LOGGING_H_
