/*
 *  Copyright 2016 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef API_STATS_RTC_STATS_H_
#define API_STATS_RTC_STATS_H_

#include <stddef.h>
#include <stdint.h>

#include <memory>
#include <optional>
#include <string>
#include <vector>

#include "api/stats/attribute.h"
#include "api/units/timestamp.h"
#include "rtc_base/checks.h"
#include "rtc_base/system/rtc_export.h"

namespace webrtc {

// Abstract base class for RTCStats-derived dictionaries, see
// https://w3c.github.io/webrtc-stats/.
//
// All derived classes must have the following static variable defined:
//   static const char kType[];
// It is used as a unique class identifier and a string representation of the
// class type, see https://w3c.github.io/webrtc-stats/#rtcstatstype-str*.
// Use the `WEBRTC_RTCSTATS_IMPL` macro when implementing subclasses, see macro
// for details.
//
// Derived classes list their dictionary attributes, std::optional<T>, as
// public fields, allowing the following:
//
// RTCFooStats foo("fooId", Timestamp::Micros(GetCurrentTime()));
// foo.bar = 42;
// foo.baz = std::vector<std::string>();
// foo.baz->push_back("hello world");
// uint32_t x = *foo.bar;
//
// Pointers to all the attributes are available with `Attributes()`, allowing
// iteration:
//
// for (const auto& attribute : foo.Attributes()) {
//   printf("%s = %s\n", attribute.name(), attribute.ToString().c_str());
// }
class RTC_EXPORT RTCStats {
 public:
  RTCStats(const std::string& id, Timestamp timestamp)
      : id_(id), timestamp_(timestamp) {}
  RTCStats(const RTCStats& other);
  virtual ~RTCStats();

  virtual std::unique_ptr<RTCStats> copy() const = 0;

  const std::string& id() const { return id_; }
  // Time relative to the UNIX epoch (Jan 1, 1970, UTC), in microseconds.
  Timestamp timestamp() const { return timestamp_; }
  void set_timestamp(Timestamp timestamp) { timestamp_ = timestamp; }

  // Returns the static member variable `kType` of the implementing class.
  virtual const char* type() const = 0;
  // Returns all attributes of this stats object, i.e. a list of its individual
  // metrics as viewed via the Attribute wrapper.
  std::vector<Attribute> Attributes() const;
  template <typename T>
  Attribute GetAttribute(const std::optional<T>& stat) const {
    for (const auto& attribute : Attributes()) {
      if (!attribute.holds_alternative<T>()) {
        continue;
      }
      if (std::get<const std::optional<T>*>(attribute.as_variant()) == &stat) {
        return attribute;
      }
    }
    RTC_CHECK_NOTREACHED();
  }
  // Checks if the two stats objects are of the same type and have the same
  // attribute values. Timestamps are not compared. These operators are exposed
  // for testing.
  bool operator==(const RTCStats& other) const;
  bool operator!=(const RTCStats& other) const;

  // Creates a JSON readable string representation of the stats
  // object, listing all of its attributes (names and values).
  std::string ToJson() const;

  // Downcasts the stats object to an `RTCStats` subclass `T`. DCHECKs that the
  // object is of type `T`.
  template <typename T>
  const T& cast_to() const {
    RTC_DCHECK_EQ(type(), T::kType);
    return static_cast<const T&>(*this);
  }

 protected:
  virtual std::vector<Attribute> AttributesImpl(
      size_t additional_capacity) const;

  std::string id_;
  Timestamp timestamp_;
};

// All `RTCStats` classes should use these macros.
// `WEBRTC_RTCSTATS_DECL` is placed in a public section of the class definition.
// `WEBRTC_RTCSTATS_IMPL` is placed outside the class definition (in a .cc).
//
// These macros declare (in _DECL) and define (in _IMPL) the static `kType` and
// overrides methods as required by subclasses of `RTCStats`: `copy`, `type` and
// `AttributesImpl`. The |...| argument is a list of addresses to each attribute
// defined in the implementing class. The list must have at least one attribute.
//
// (Since class names need to be known to implement these methods this cannot be
// part of the base `RTCStats`. While these methods could be implemented using
// templates, that would only work for immediate subclasses. Subclasses of
// subclasses also have to override these methods, resulting in boilerplate
// code. Using a macro avoids this and works for any `RTCStats` class, including
// grandchildren.)
//
// Sample usage:
//
// rtcfoostats.h:
//   class RTCFooStats : public RTCStats {
//    public:
//     WEBRTC_RTCSTATS_DECL();
//
//     RTCFooStats(const std::string& id, Timestamp timestamp);
//
//     std::optional<int32_t> foo;
//     std::optional<int32_t> bar;
//   };
//
// rtcfoostats.cc:
//   WEBRTC_RTCSTATS_IMPL(RTCFooStats, RTCStats, "foo-stats"
//       &foo,
//       &bar);
//
//   RTCFooStats::RTCFooStats(const std::string& id, Timestamp timestamp)
//       : RTCStats(id, timestamp),
//         foo("foo"),
//         bar("bar") {
//   }
//
#define WEBRTC_RTCSTATS_DECL(SelfT)                                         \
 protected:                                                                 \
  std::vector<webrtc::Attribute> AttributesImpl(size_t additional_capacity) \
      const override;                                                       \
                                                                            \
 public:                                                                    \
  static const char kType[];                                                \
                                                                            \
  template <typename Sink>                                                  \
  friend void AbslStringify(Sink& sink, const SelfT& stats) {               \
    sink.Append(stats.ToJson());                                            \
  }                                                                         \
                                                                            \
  std::unique_ptr<webrtc::RTCStats> copy() const override;                  \
  const char* type() const override

#define WEBRTC_RTCSTATS_IMPL(this_class, parent_class, type_str, ...)         \
  const char this_class::kType[] = type_str;                                  \
                                                                              \
  std::unique_ptr<webrtc::RTCStats> this_class::copy() const {                \
    return std::make_unique<this_class>(*this);                               \
  }                                                                           \
                                                                              \
  const char* this_class::type() const {                                      \
    return this_class::kType;                                                 \
  }                                                                           \
                                                                              \
  std::vector<webrtc::Attribute> this_class::AttributesImpl(                  \
      size_t additional_capacity) const {                                     \
    webrtc::AttributeInit attribute_inits[] = {__VA_ARGS__};                  \
    size_t attribute_inits_size =                                             \
        sizeof(attribute_inits) / sizeof(attribute_inits[0]);                 \
    std::vector<webrtc::Attribute> attributes = parent_class::AttributesImpl( \
        attribute_inits_size + additional_capacity);                          \
    for (size_t i = 0; i < attribute_inits_size; ++i) {                       \
      attributes.push_back(std::visit(                                        \
          [&](const auto* field) {                                            \
            return Attribute(attribute_inits[i].name, field);                 \
          },                                                                  \
          attribute_inits[i].variant));                                       \
    }                                                                         \
    return attributes;                                                        \
  }

}  // namespace webrtc

#endif  // API_STATS_RTC_STATS_H_
