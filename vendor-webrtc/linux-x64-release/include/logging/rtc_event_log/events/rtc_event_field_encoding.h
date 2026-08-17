/*
 *  Copyright (c) 2021 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef LOGGING_RTC_EVENT_LOG_EVENTS_RTC_EVENT_FIELD_ENCODING_H_
#define LOGGING_RTC_EVENT_LOG_EVENTS_RTC_EVENT_FIELD_ENCODING_H_

#include <cstddef>
#include <cstdint>
#include <optional>
#include <string>
#include <type_traits>
#include <vector>

#include "absl/strings/string_view.h"
#include "api/array_view.h"
#include "api/rtc_event_log/rtc_event.h"
#include "logging/rtc_event_log/events/fixed_length_encoding_parameters_v3.h"
#include "logging/rtc_event_log/events/rtc_event_field_extraction.h"
#include "rtc_base/checks.h"

namespace webrtc {

// To maintain backwards compatibility with past (or future) logs,
// the constants in this enum must not be changed.
// New field types with numerical IDs 5-7 can be added, but old
// parsers will fail to parse events containing the new fields.
enum class FieldType : uint8_t {
  kFixed8 = 0,
  kFixed32 = 1,
  kFixed64 = 2,
  kVarInt = 3,
  kString = 4,
};

// EventParameters map an event name to a numerical ID.
struct EventParameters {
  // The name is primarily used for debugging purposes.
  const char* const name;
  //
  const RtcEvent::Type id;
};

// FieldParameters define the encoding for a field.
struct FieldParameters {
  // The name is primarily used for debugging purposes.
  const char* const name;
  // Numerical ID for the field. Must be strictly greater than 0,
  // and unique within each event type.
  const uint64_t field_id;
  // Encoding type for the base (i.e. non-delta) field in a batch.
  const FieldType field_type;
  // Number of bits after which wrap-around occurs. In most cases,
  // this should be the number of bits in the field data type, i.e.
  // 8 for an uint8_t, 32 for a int32_t and so on. However, `value_width`
  // can be used to achieve a more efficient encoding if it is known
  // that the field uses a smaller number of bits. For example, a
  // 15-bit counter could set `value_width` to 15 even if the data is
  // actually stored in a uint32_t.
  const uint64_t value_width;
  // Field ID 0 is reserved for timestamps.
  static constexpr uint64_t kTimestampField = 0;
};

// The EventEncoder is used to encode a batch of events.
class EventEncoder {
 public:
  EventEncoder(EventParameters params, ArrayView<const RtcEvent*> batch);

  void EncodeField(const FieldParameters& params,
                   const std::vector<uint64_t>& values,
                   const std::vector<bool>* positions = nullptr);

  void EncodeField(const FieldParameters& params,
                   const ValuesWithPositions& values);

  void EncodeField(const FieldParameters& params,
                   const std::vector<absl::string_view>& values);

  std::string AsString();

 private:
  size_t batch_size_;
  uint32_t event_tag_;
  std::vector<std::string> encoded_fields_;
};

std::string EncodeSingleValue(uint64_t value, FieldType field_type);
std::string EncodeDeltasV3(FixedLengthEncodingParametersV3 params,
                           uint64_t base,
                           ArrayView<const uint64_t> values);

// Given a batch of RtcEvents and a member pointer, extract that
// member from each event in the batch. Signed integer members are
// encoded as unsigned, and the bitsize increased so the result can
// represented as a std::vector<uint64_t>.
// This is intended to be used in conjuction with
// EventEncoder::EncodeField to encode a batch of events as follows:
// auto values = ExtractRtcEventMember(batch, RtcEventFoo::timestamp_ms);
// encoder.EncodeField(timestamp_params, values)
template <typename T,
          typename E,
          std::enable_if_t<std::is_integral<T>::value, bool> = true>
std::vector<uint64_t> ExtractRtcEventMember(ArrayView<const RtcEvent*> batch,
                                            const T E::* member) {
  std::vector<uint64_t> values;
  values.reserve(batch.size());
  for (const RtcEvent* event : batch) {
    RTC_CHECK_EQ(event->GetType(), E::kType);
    T value = static_cast<const E*>(event)->*member;
    values.push_back(EncodeAsUnsigned(value));
  }
  return values;
}

// Extract an optional field from a batch of RtcEvents.
// The function returns a vector of positions in addition to the vector of
// values. The vector `positions` has the same length as the batch where
// `positions[i] == true` iff the batch[i]->member has a value.
// The values vector only contains the values that exists, so it
// may be shorter than the batch.
template <typename T,
          typename E,
          std::enable_if_t<std::is_integral<T>::value, bool> = true>
ValuesWithPositions ExtractRtcEventMember(ArrayView<const RtcEvent*> batch,
                                          const std::optional<T> E::* member) {
  ValuesWithPositions result;
  result.position_mask.reserve(batch.size());
  result.values.reserve(batch.size());
  for (const RtcEvent* event : batch) {
    RTC_CHECK_EQ(event->GetType(), E::kType);
    std::optional<T> field = static_cast<const E*>(event)->*member;
    result.position_mask.push_back(field.has_value());
    if (field.has_value()) {
      result.values.push_back(EncodeAsUnsigned(field.value()));
    }
  }
  return result;
}

// Extract an enum field from a batch of RtcEvents.
// Requires specializing RtcEventLogEnum<T> for the enum type T.
template <typename T,
          typename E,
          std::enable_if_t<std::is_enum<T>::value, bool> = true>
std::vector<uint64_t> ExtractRtcEventMember(ArrayView<const RtcEvent*> batch,
                                            const T E::* member) {
  std::vector<uint64_t> values;
  values.reserve(batch.size());
  for (const RtcEvent* event : batch) {
    RTC_CHECK_EQ(event->GetType(), E::kType);
    T value = static_cast<const E*>(event)->*member;
    values.push_back(RtcEventLogEnum<T>::Encode(value));
  }
  return values;
}

// Extract a string field from a batch of RtcEvents.
template <typename E>
std::vector<absl::string_view> ExtractRtcEventMember(
    ArrayView<const RtcEvent*> batch,
    const std::string E::* member) {
  std::vector<absl::string_view> values;
  values.reserve(batch.size());
  for (const RtcEvent* event : batch) {
    RTC_CHECK_EQ(event->GetType(), E::kType);
    absl::string_view str = static_cast<const E*>(event)->*member;
    values.push_back(str);
  }
  return values;
}

}  // namespace webrtc
#endif  // LOGGING_RTC_EVENT_LOG_EVENTS_RTC_EVENT_FIELD_ENCODING_H_
