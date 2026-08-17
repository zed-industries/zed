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

#ifndef SRC_TRACE_PROCESSOR_TYPES_SOFTIRQ_ACTION_H_
#define SRC_TRACE_PROCESSOR_TYPES_SOFTIRQ_ACTION_H_

namespace perfetto {
namespace trace_processor {

static constexpr const char* const kActionNames[] = {
    "HI",           "TIMER",   "NET_TX", "NET_RX",  "BLOCK",
    "BLOCK_IOPOLL", "TASKLET", "SCHED",  "HRTIMER", "RCU"};

}  // namespace trace_processor
}  // namespace perfetto

#endif  // SRC_TRACE_PROCESSOR_TYPES_SOFTIRQ_ACTION_H_
