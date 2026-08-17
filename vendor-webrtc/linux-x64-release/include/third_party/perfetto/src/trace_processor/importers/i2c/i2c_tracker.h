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

#ifndef SRC_TRACE_PROCESSOR_IMPORTERS_I2C_I2C_TRACKER_H_
#define SRC_TRACE_PROCESSOR_IMPORTERS_I2C_I2C_TRACKER_H_

#include <limits>
#include <tuple>

#include "perfetto/ext/base/flat_hash_map.h"
#include "perfetto/ext/base/string_view.h"
#include "src/trace_processor/importers/common/slice_tracker.h"
#include "src/trace_processor/importers/common/track_tracker.h"
#include "src/trace_processor/storage/trace_storage.h"
#include "src/trace_processor/types/destructible.h"
#include "src/trace_processor/types/trace_processor_context.h"

namespace perfetto {
namespace trace_processor {

static constexpr size_t kMaxI2cAdapters = 256;

using I2cAdapterId = uint32_t;

class I2cTracker : public Destructible {
 public:
  I2cTracker(const I2cTracker&) = delete;
  I2cTracker& operator=(const I2cTracker&) = delete;
  ~I2cTracker() override;
  static I2cTracker* GetOrCreate(TraceProcessorContext* context) {
    if (!context->i2c_tracker) {
      context->i2c_tracker.reset(new I2cTracker(context));
    }
    return static_cast<I2cTracker*>(context->i2c_tracker.get());
  }

  struct I2cAdapterMessageCount {
    I2cAdapterId adapter_nr{};
    uint32_t nr_msgs{};
  };

  // Enter processes the start of an I2C transaction, which consists of a
  // series i2c_write and i2c_read's. Multiple messages may be included in a
  // transaction. The position of a write or read message in the transaction
  // is indicated by the msg_nr field.
  void Enter(int64_t ts, UniqueTid utid, uint32_t adapter_nr, uint32_t msg_nr);

  // Exit processes the end of an I2C transaction, which is indicated by an
  // i2c_result. The nr_msgs field indicates how many write or read requests
  // are considered to be matching the current i2c_result.
  void Exit(int64_t ts, UniqueTid utid, uint32_t adapter_nr, uint32_t nr_msgs);

 private:
  explicit I2cTracker(TraceProcessorContext*);
  TraceProcessorContext* const context_;
  std::array<StringId, kMaxI2cAdapters> i2c_adapter_to_string_id_{};

  // In-flight I2C operation counts per I2C adapter per Unique TID. This is
  // used to match an i2c_result message against the i2c_read and i2c_write
  // messages that precede it in the transaction.
  base::FlatHashMap<UniqueTid, std::vector<I2cAdapterMessageCount>>
      inflight_i2c_ops_;
};

}  // namespace trace_processor
}  // namespace perfetto

#endif  // SRC_TRACE_PROCESSOR_IMPORTERS_I2C_I2C_TRACKER_H_
