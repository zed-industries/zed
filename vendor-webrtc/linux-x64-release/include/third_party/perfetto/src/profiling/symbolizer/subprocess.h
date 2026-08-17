
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

#ifndef SRC_PROFILING_SYMBOLIZER_SUBPROCESS_H_
#define SRC_PROFILING_SYMBOLIZER_SUBPROCESS_H_

#include <string>
#include <vector>

#include "perfetto/base/build_config.h"

#if !PERFETTO_BUILDFLAG(PERFETTO_OS_WIN)
#include <sys/types.h>
#include <unistd.h>
#include "perfetto/ext/base/pipe.h"
#endif

namespace perfetto {
namespace profiling {

class Subprocess {
 public:
  Subprocess(const std::string& file, std::vector<std::string> args);
  ~Subprocess();

  int64_t Write(const char* buffer, size_t size);
  int64_t Read(char* buffer, size_t size);

 private:
#if PERFETTO_BUILDFLAG(PERFETTO_OS_WIN)
  void* child_pipe_in_read_ = nullptr;
  void* child_pipe_in_write_ = nullptr;
  void* child_pipe_out_read_ = nullptr;
  void* child_pipe_out_write_ = nullptr;
#else
  base::Pipe input_pipe_;
  base::Pipe output_pipe_;

  pid_t pid_ = -1;
#endif
};

}  // namespace profiling
}  // namespace perfetto

#endif  // SRC_PROFILING_SYMBOLIZER_SUBPROCESS_H_
