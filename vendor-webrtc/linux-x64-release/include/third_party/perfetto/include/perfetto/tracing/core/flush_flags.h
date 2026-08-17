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

#ifndef INCLUDE_PERFETTO_TRACING_CORE_FLUSH_FLAGS_H_
#define INCLUDE_PERFETTO_TRACING_CORE_FLUSH_FLAGS_H_

#include <stddef.h>
#include <stdint.h>

namespace perfetto {

// This class is a wrapper around the uint64_t flags that are sent across the
// tracing protocol whenenver a flush occurs. It helps determining the reason
// and initiator of the flush.
// NOTE: the values here are part of the tracing protocol ABI. Do not renumber.
class FlushFlags {
 public:
  enum class Initiator : uint64_t {
    // DO NOT RENUMBER, ABI.
    kUnknown = 0,
    kTraced = 1,
    kPerfettoCmd = 2,
    kConsumerSdk = 3,
    kMax,
  };

  enum class Reason : uint64_t {
    // DO NOT RENUMBER, ABI.
    kUnknown = 0,
    kPeriodic = 1,
    kTraceStop = 2,
    kTraceClone = 3,
    kExplicit = 4,
    kMax,
  };

  enum class CloneTarget : uint64_t {
    // DO NOT RENUMBER, ABI.
    kUnknown = 0,
    kBugreport = 1,
    kMax,
  };

  explicit FlushFlags(uint64_t flags = 0) : flags_(flags) {}
  FlushFlags(Initiator i, Reason r, CloneTarget c = CloneTarget::kUnknown)
      : flags_((static_cast<uint64_t>(i) << kInitiatorShift) |
               (static_cast<uint64_t>(r) << kReasonShift) |
               (static_cast<uint64_t>(c) << kCloneTargetShift)) {}

  bool operator==(const FlushFlags& o) const { return flags_ == o.flags_; }
  bool operator!=(const FlushFlags& o) const { return !(*this == o); }

  Initiator initiator() const {
    // Due to version mismatch we might see a value from the future that we
    // didn't know yet. If that happens, short ciruit to kUnknown.
    static_assert(
        uint64_t(Initiator::kMax) - 1 <= (kInitiatorMask >> kInitiatorShift),
        "enum out of range");
    const uint64_t value = (flags_ & kInitiatorMask) >> kInitiatorShift;
    return value < uint64_t(Initiator::kMax) ? Initiator(value)
                                             : Initiator::kUnknown;
  }

  Reason reason() const {
    static_assert(uint64_t(Reason::kMax) - 1 <= (kReasonMask >> kReasonShift),
                  "enum out of range");
    const uint64_t value = (flags_ & kReasonMask) >> kReasonShift;
    return value < uint64_t(Reason::kMax) ? Reason(value) : Reason::kUnknown;
  }

  CloneTarget clone_target() const {
    static_assert(uint64_t(CloneTarget::kMax) - 1 <=
                      (kCloneTargetMask >> kCloneTargetShift),
                  "enum out of range");
    const uint64_t value = (flags_ & kCloneTargetMask) >> kCloneTargetShift;
    return value < uint64_t(CloneTarget::kMax) ? CloneTarget(value)
                                               : CloneTarget::kUnknown;
  }

  uint64_t flags() const { return flags_; }

 private:
  // DO NOT CHANGE, ABI.
  static constexpr uint64_t kReasonMask = 0xF;
  static constexpr uint64_t kReasonShift = 0;
  static constexpr uint64_t kInitiatorMask = 0xF0;
  static constexpr uint64_t kInitiatorShift = 4;
  static constexpr uint64_t kCloneTargetMask = 0xF00;
  static constexpr uint64_t kCloneTargetShift = 8;

  uint64_t flags_ = 0;
};

}  // namespace perfetto

#endif  // INCLUDE_PERFETTO_TRACING_CORE_FLUSH_FLAGS_H_
