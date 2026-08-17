/*
 *  Copyright (c) 2021 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef LOGGING_RTC_EVENT_LOG_EVENTS_RTC_EVENT_DEFINITION_H_
#define LOGGING_RTC_EVENT_LOG_EVENTS_RTC_EVENT_DEFINITION_H_

#include <cstdint>
#include <string>
#include <vector>

#include "absl/strings/string_view.h"
#include "api/array_view.h"
#include "api/rtc_event_log/rtc_event.h"
#include "logging/rtc_event_log/events/rtc_event_field_encoding.h"
#include "logging/rtc_event_log/events/rtc_event_field_encoding_parser.h"
#include "logging/rtc_event_log/events/rtc_event_log_parse_status.h"

namespace webrtc {

template <typename EventType, typename LoggedType, typename T>
struct RtcEventFieldDefinition {
  const T EventType::*event_member;
  T LoggedType::*logged_member;
  FieldParameters params;
};

// Base case
template <typename EventType, typename LoggedType, typename... Ts>
class RtcEventDefinitionImpl {
 public:
  void EncodeImpl(EventEncoder&, ArrayView<const RtcEvent*>) const {}
  RtcEventLogParseStatus ParseImpl(EventParser&, ArrayView<LoggedType>) const {
    return RtcEventLogParseStatus::Success();
  }
};

// Recursive case
template <typename EventType, typename LoggedType, typename T, typename... Ts>
class RtcEventDefinitionImpl<EventType, LoggedType, T, Ts...> {
 public:
  constexpr RtcEventDefinitionImpl(
      RtcEventFieldDefinition<EventType, LoggedType, T> field,
      RtcEventFieldDefinition<EventType, LoggedType, Ts>... rest)
      : field_(field), rest_(rest...) {}

  void EncodeImpl(EventEncoder& encoder,
                  ArrayView<const RtcEvent*> batch) const {
    auto values = ExtractRtcEventMember(batch, field_.event_member);
    encoder.EncodeField(field_.params, values);
    rest_.EncodeImpl(encoder, batch);
  }

  RtcEventLogParseStatus ParseImpl(EventParser& parser,
                                   ArrayView<LoggedType> output_batch) const {
    RtcEventLogParseStatusOr<ArrayView<uint64_t>> result =
        parser.ParseNumericField(field_.params);
    if (!result.ok())
      return result.status();
    auto status = PopulateRtcEventMember(result.value(), field_.logged_member,
                                         output_batch);
    if (!status.ok())
      return status;

    return rest_.ParseImpl(parser, output_batch);
  }

 private:
  RtcEventFieldDefinition<EventType, LoggedType, T> field_;
  RtcEventDefinitionImpl<EventType, LoggedType, Ts...> rest_;
};

// The RtcEventDefinition sets up a mapping between the fields
// in an RtcEvent and the corresponding fields in the parsed struct.
// For example, an RtcFoo class containing two fields; `uint32_t bar`
// and `bool baz` (a log timestamp is always implicitly added)
// might have a definition
// RtcEventDefinition<RtcFoo, LoggedFoo, uint32_t, bool>(
//   {"foo", RtcFoo::Type},
//   {&RtcFoo::bar_, &LoggedFoo::bar, {"bar", 1, FieldType::kVarInt, 32}},
//   {&RtcFoo::baz_, &LoggedFoo::baz, {"baz", 2, FieldType::kFixed8, 1}},
// );
// In addition to defining string names to aid debugging,
// this specifies that
// * RtcFoo::Type uniquely identifies an RtcFoo in the encoded stream
// * The `bar` field has ID 1, is encoded as a VarInt
//   (when not delta compressed), and wraps around after 32 bits.
// * The `baz` field has ID 2, is encoded as an 8-bit field
//   (when not delta compressed), and wraps around after 1 bit.
// Note that the numerical field and event IDs can't be changed since
// that would break compatibility with old logs.
// In most cases (including all cases where wrap around isn't
// expected), the wrap around should be equal to the bitwidth of
// the field.
template <typename EventType, typename LoggedType, typename... Ts>
class RtcEventDefinition {
 public:
  constexpr RtcEventDefinition(
      EventParameters params,
      RtcEventFieldDefinition<EventType, LoggedType, Ts>... fields)
      : params_(params), fields_(fields...) {}

  std::string EncodeBatch(ArrayView<const RtcEvent*> batch) const {
    EventEncoder encoder(params_, batch);
    fields_.EncodeImpl(encoder, batch);
    return encoder.AsString();
  }

  RtcEventLogParseStatus ParseBatch(absl::string_view s,
                                    bool batched,
                                    std::vector<LoggedType>& output) const {
    EventParser parser;
    auto status = parser.Initialize(s, batched);
    if (!status.ok())
      return status;

    ArrayView<LoggedType> output_batch =
        ExtendLoggedBatch(output, parser.NumEventsInBatch());

    constexpr FieldParameters timestamp_params{"timestamp_ms",
                                               FieldParameters::kTimestampField,
                                               FieldType::kVarInt, 64};
    RtcEventLogParseStatusOr<ArrayView<uint64_t>> result =
        parser.ParseNumericField(timestamp_params);
    if (!result.ok())
      return result.status();
    status = PopulateRtcEventTimestamp(result.value(), &LoggedType::timestamp,
                                       output_batch);
    if (!status.ok())
      return status;

    return fields_.ParseImpl(parser, output_batch);
  }

 private:
  EventParameters params_;
  RtcEventDefinitionImpl<EventType, LoggedType, Ts...> fields_;
};

}  // namespace webrtc

#endif  // LOGGING_RTC_EVENT_LOG_EVENTS_RTC_EVENT_DEFINITION_H_
