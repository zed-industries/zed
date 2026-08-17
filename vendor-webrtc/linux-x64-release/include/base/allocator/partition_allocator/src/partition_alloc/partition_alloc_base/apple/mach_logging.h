// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef PARTITION_ALLOC_PARTITION_ALLOC_BASE_APPLE_MACH_LOGGING_H_
#define PARTITION_ALLOC_PARTITION_ALLOC_BASE_APPLE_MACH_LOGGING_H_

#include <mach/mach.h>

#include "partition_alloc/build_config.h"
#include "partition_alloc/buildflags.h"
#include "partition_alloc/partition_alloc_base/component_export.h"
#include "partition_alloc/partition_alloc_base/logging.h"

// Use the PA_MACH_LOG family of macros along with a mach_error_t
// (kern_return_t) containing a Mach error. The error value will be decoded so
// that logged messages explain the error.
//
// Examples:
//
//   kern_return_t kr = mach_timebase_info(&info);
//   if (kr != KERN_SUCCESS) {
//     PA_MACH_LOG(ERROR, kr) << "mach_timebase_info";
//   }
//
//   kr = vm_deallocate(task, address, size);
//   PA_MACH_DCHECK(kr == KERN_SUCCESS, kr) << "vm_deallocate";

namespace partition_alloc::internal::logging {

class PA_COMPONENT_EXPORT(PARTITION_ALLOC_BASE) MachLogMessage
    : public partition_alloc::internal::logging::LogMessage {
 public:
  MachLogMessage(const char* file_path,
                 int line,
                 LogSeverity severity,
                 mach_error_t mach_err);

  MachLogMessage(const MachLogMessage&) = delete;
  MachLogMessage& operator=(const MachLogMessage&) = delete;

  ~MachLogMessage() override;

 private:
  mach_error_t mach_err_;
};

}  // namespace partition_alloc::internal::logging

#if PA_BUILDFLAG(DCHECKS_ARE_ON)
#define PA_MACH_DVLOG_IS_ON(verbose_level) PA_VLOG_IS_ON(verbose_level)
#else
#define PA_MACH_DVLOG_IS_ON(verbose_level) 0
#endif

#define PA_MACH_LOG_STREAM(severity, mach_err) \
  PA_COMPACT_GOOGLE_PLOG_EX_##severity(MachLogMessage, mach_err).stream()
#define PA_MACH_VLOG_STREAM(verbose_level, mach_err)    \
  ::partition_alloc::internal::logging::MachLogMessage( \
      __FILE__, __LINE__, -verbose_level, mach_err)     \
      .stream()

#define PA_MACH_LOG(severity, mach_err) \
  PA_LAZY_STREAM(PA_MACH_LOG_STREAM(severity, mach_err), PA_LOG_IS_ON(severity))
#define PA_MACH_LOG_IF(severity, condition, mach_err)    \
  PA_LAZY_STREAM(PA_MACH_LOG_STREAM(severity, mach_err), \
                 PA_LOG_IS_ON(severity) && (condition))

#define PA_MACH_VLOG(verbose_level, mach_err)                  \
  PA_LAZY_STREAM(PA_MACH_VLOG_STREAM(verbose_level, mach_err), \
                 PA_VLOG_IS_ON(verbose_level))
#define PA_MACH_VLOG_IF(verbose_level, condition, mach_err)    \
  PA_LAZY_STREAM(PA_MACH_VLOG_STREAM(verbose_level, mach_err), \
                 PA_VLOG_IS_ON(verbose_level) && (condition))

#define PA_MACH_CHECK(condition, mach_err)                          \
  PA_LAZY_STREAM(PA_MACH_LOG_STREAM(FATAL, mach_err), !(condition)) \
      << "Check failed: " #condition << ". "

#define PA_MACH_DLOG(severity, mach_err)                 \
  PA_LAZY_STREAM(PA_MACH_LOG_STREAM(severity, mach_err), \
                 PA_DLOG_IS_ON(severity))
#define PA_MACH_DLOG_IF(severity, condition, mach_err)   \
  PA_LAZY_STREAM(PA_MACH_LOG_STREAM(severity, mach_err), \
                 PA_DLOG_IS_ON(severity) && (condition))

#define PA_MACH_DVLOG(verbose_level, mach_err)                 \
  PA_LAZY_STREAM(PA_MACH_VLOG_STREAM(verbose_level, mach_err), \
                 PA_MACH_DVLOG_IS_ON(verbose_level))
#define PA_MACH_DVLOG_IF(verbose_level, condition, mach_err)   \
  PA_LAZY_STREAM(PA_MACH_VLOG_STREAM(verbose_level, mach_err), \
                 PA_MACH_DVLOG_IS_ON(verbose_level) && (condition))

#define PA_MACH_DCHECK(condition, mach_err)                    \
  PA_LAZY_STREAM(PA_MACH_LOG_STREAM(FATAL, mach_err),          \
                 PA_BUILDFLAG(DCHECKS_ARE_ON) && !(condition)) \
      << "Check failed: " #condition << ". "

#endif  // PARTITION_ALLOC_PARTITION_ALLOC_BASE_APPLE_MACH_LOGGING_H_
