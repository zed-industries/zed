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

#ifndef SRC_TRACE_PROCESSOR_DB_COLUMN_STORAGE_LAYER_H_
#define SRC_TRACE_PROCESSOR_DB_COLUMN_STORAGE_LAYER_H_

#include <cstdint>
#include <variant>

#include "src/trace_processor/containers/string_pool.h"
#include "src/trace_processor/db/column/data_layer.h"

namespace perfetto::trace_processor::column {

class StorageLayer : public DataLayer {
 public:
  struct Id {};
  using StoragePtr = std::variant<Id,
                                  const int64_t*,
                                  const int32_t*,
                                  const uint32_t*,
                                  const double*,
                                  const StringPool::Id*>;

  ~StorageLayer() override;
  virtual StoragePtr GetStoragePtr() = 0;

 protected:
  explicit StorageLayer(Impl impl);
};

}  // namespace perfetto::trace_processor::column

#endif  // SRC_TRACE_PROCESSOR_DB_COLUMN_STORAGE_LAYER_H_
