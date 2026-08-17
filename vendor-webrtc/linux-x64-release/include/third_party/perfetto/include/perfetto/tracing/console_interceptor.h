/*
 * Copyright (C) 2020 The Android Open Source Project
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

#ifndef INCLUDE_PERFETTO_TRACING_CONSOLE_INTERCEPTOR_H_
#define INCLUDE_PERFETTO_TRACING_CONSOLE_INTERCEPTOR_H_

#include "perfetto/base/compiler.h"
#include "perfetto/base/logging.h"
#include "perfetto/tracing/interceptor.h"
#include "perfetto/tracing/track_event_state_tracker.h"

#include <stdarg.h>

#include <functional>
#include <map>
#include <vector>

#if PERFETTO_BUILDFLAG(PERFETTO_OS_WIN)
#include <io.h>
#else
#include <unistd.h>
#endif

#if defined(__GNUC__) || defined(__clang__)
#define PERFETTO_PRINTF_ATTR \
  __attribute__((format(printf, /*format_index=*/2, /*first_to_check=*/3)))
#else
#define PERFETTO_PRINTF_ATTR
#endif

#if PERFETTO_BUILDFLAG(PERFETTO_OS_WIN) && !defined(STDOUT_FILENO)
#define STDOUT_FILENO 1
#define STDERR_FILENO 2
#endif

namespace perfetto {
namespace protos {
namespace pbzero {
class DebugAnnotation_Decoder;
class TracePacket_Decoder;
class TrackEvent_Decoder;
}  // namespace pbzero
}  // namespace protos

struct ConsoleColor;

class PERFETTO_EXPORT_COMPONENT ConsoleInterceptor
    : public Interceptor<ConsoleInterceptor> {
 public:
  ~ConsoleInterceptor() override;

  static void Register();
  static void OnTracePacket(InterceptorContext context);

  static void SetOutputFdForTesting(int fd);

  void OnSetup(const SetupArgs&) override;
  void OnStart(const StartArgs&) override;
  void OnStop(const StopArgs&) override;

  struct ThreadLocalState : public InterceptorBase::ThreadLocalState {
    ThreadLocalState(ThreadLocalStateArgs&);
    ~ThreadLocalState() override;

    // Destination file. Assumed to stay valid until the program ends (i.e., is
    // stderr or stdout).
    int fd{};
    bool use_colors{};

    // Messages up to this length are buffered and written atomically. If a
    // message is longer, it will be printed with multiple writes.
    std::array<char, 1024> message_buffer{};
    size_t buffer_pos{};

    // We only support a single trace writer sequence per thread, so the
    // sequence state is stored in TLS.
    TrackEventStateTracker::SequenceState sequence_state;
    uint64_t start_time_ns{};
  };

 private:
  class Delegate;

  // Appends a formatted message to |message_buffer_| or directly to the output
  // file if the buffer is full.
  static void Printf(InterceptorContext& context,
                     const char* format,
                     ...) PERFETTO_PRINTF_ATTR;
  static void Flush(InterceptorContext& context);
  static void SetColor(InterceptorContext& context, const ConsoleColor&);
  static void SetColor(InterceptorContext& context, const char*);

  static void PrintDebugAnnotations(InterceptorContext&,
                                    const protos::pbzero::TrackEvent_Decoder&,
                                    const ConsoleColor& slice_color,
                                    const ConsoleColor& highlight_color);
  static void PrintDebugAnnotationName(
      InterceptorContext&,
      const perfetto::protos::pbzero::DebugAnnotation_Decoder& annotation);
  static void PrintDebugAnnotationValue(
      InterceptorContext&,
      const perfetto::protos::pbzero::DebugAnnotation_Decoder& annotation);

  int fd_ = STDOUT_FILENO;
  bool use_colors_ = true;

  TrackEventStateTracker::SessionState session_state_;
  uint64_t start_time_ns_{};
};

}  // namespace perfetto

#endif  // INCLUDE_PERFETTO_TRACING_CONSOLE_INTERCEPTOR_H_
