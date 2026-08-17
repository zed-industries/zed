/*
 * Copyright (C) 2023 The Android Open Source Project
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

#ifndef SRC_TRACING_SERVICE_ZLIB_COMPRESSOR_H_
#define SRC_TRACING_SERVICE_ZLIB_COMPRESSOR_H_

#include <vector>

#include "perfetto/ext/tracing/core/trace_packet.h"

namespace perfetto {

// Matches TracingServiceImpl::kMaxTracePacketSliceSize. Exposed for testing.
static constexpr size_t kZlibCompressSliceSize = 128 * 1024 - 512;

void ZlibCompressFn(std::vector<TracePacket>*);

}  // namespace perfetto

#endif  // SRC_TRACING_SERVICE_ZLIB_COMPRESSOR_H_
