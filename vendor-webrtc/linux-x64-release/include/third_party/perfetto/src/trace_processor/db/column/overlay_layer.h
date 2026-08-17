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

#ifndef SRC_TRACE_PROCESSOR_DB_COLUMN_OVERLAY_LAYER_H_
#define SRC_TRACE_PROCESSOR_DB_COLUMN_OVERLAY_LAYER_H_

#include <cstdint>

#include "src/trace_processor/db/column/data_layer.h"

namespace perfetto::trace_processor::column {

class OverlayLayer : public DataLayer {
 public:
  ~OverlayLayer() override;

  // Translates the indices separated by |stride| between |start| and |end|
  // through this overlay.
  //
  // Implementations should do something like this:
  //   for (auto* it = start; it < end; it += stride) {
  //     *it = (...translate the index at *it);
  //   }
  virtual void Flatten(uint32_t* start,
                       const uint32_t* end,
                       uint32_t stride) = 0;

 protected:
  explicit OverlayLayer(Impl impl);
};

}  // namespace perfetto::trace_processor::column

#endif  // SRC_TRACE_PROCESSOR_DB_COLUMN_OVERLAY_LAYER_H_
