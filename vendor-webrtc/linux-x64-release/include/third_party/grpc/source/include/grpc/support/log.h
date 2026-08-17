/*
 *
 * Copyright 2015 gRPC authors.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 *
 */

#ifndef GRPC_SUPPORT_LOG_H
#define GRPC_SUPPORT_LOG_H

#include <grpc/support/port_platform.h>
#include <stdarg.h>
#include <stdlib.h> /* for abort() */

#ifdef __cplusplus
extern "C" {
#endif

/** The severity of a log message - use the #defines below when calling into
   grpc_absl_log to additionally supply file and line data */
typedef enum gpr_log_severity {
  GPR_LOG_SEVERITY_DEBUG,
  GPR_LOG_SEVERITY_INFO,
  GPR_LOG_SEVERITY_ERROR
} gpr_log_severity;

/** Macros to build log contexts at various severity levels */
#define GPR_DEBUG __FILE__, __LINE__, GPR_LOG_SEVERITY_DEBUG
#define GPR_INFO __FILE__, __LINE__, GPR_LOG_SEVERITY_INFO
#define GPR_ERROR __FILE__, __LINE__, GPR_LOG_SEVERITY_ERROR

/**
 * EXPERIMENTAL. API stability not guaranteed.
 * Should only be used from gRPC PHP and RUBY.
 * This will be removed once Ruby and PHP can start using C++ APIs.
 * We would replace this with calls to absl LOG functions.
 * grpc_absl_log is equivalent to
 * ABSL_LOG(severity) << message_str;
 * **/
GPRAPI void grpc_absl_log(const char* file, int line, gpr_log_severity severity,
                          const char* message_str);

/**
 * EXPERIMENTAL. API stability not guaranteed.
 * Should only be used from gRPC PHP and RUBY.
 * This will be removed once Ruby and PHP can start using C++ APIs.
 * We would replace this with calls to absl LOG functions.
 * grpc_absl_log_int is equivalent to
 * ABSL_LOG(severity) << message_str << num;
 * **/
GPRAPI void grpc_absl_log_int(const char* file, int line,
                              gpr_log_severity severity,
                              const char* message_str, intptr_t num);

/**
 * EXPERIMENTAL. API stability not guaranteed.
 * Should only be used from gRPC PHP and RUBY.
 * This will be removed once Ruby and PHP can start using C++ APIs.
 * We would replace this with calls to absl LOG functions.
 * grpc_absl_log_str is equivalent to
 * ABSL_LOG(severity) << message_str1 << message_str2;
 * **/
GPRAPI void grpc_absl_log_str(const char* file, int line,
                              gpr_log_severity severity,
                              const char* message_str1,
                              const char* message_str2);

GPRAPI void gpr_log_verbosity_init(void);

#ifdef __cplusplus
}
#endif

#endif /* GRPC_SUPPORT_LOG_H */
