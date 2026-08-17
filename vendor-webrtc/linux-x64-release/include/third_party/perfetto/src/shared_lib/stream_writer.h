/*
 * Copyright (C) 2022 The Android Open Source Project
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

#ifndef SRC_SHARED_LIB_STREAM_WRITER_H_
#define SRC_SHARED_LIB_STREAM_WRITER_H_

#include "perfetto/public/abi/stream_writer_abi.h"

#include "perfetto/protozero/scattered_stream_writer.h"

namespace perfetto {

// Copies the visible state from `sw` to `*w`.
inline void UpdateStreamWriter(const protozero::ScatteredStreamWriter& sw,
                               struct PerfettoStreamWriter* w) {
  w->begin = sw.cur_range().begin;
  w->end = sw.cur_range().end;
  w->write_ptr = sw.write_ptr();
  w->written_previously = static_cast<size_t>(sw.written_previously());
}

}  // namespace perfetto

#endif  // SRC_SHARED_LIB_STREAM_WRITER_H_
