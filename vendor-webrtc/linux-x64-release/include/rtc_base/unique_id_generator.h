/*
 *  Copyright 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef RTC_BASE_UNIQUE_ID_GENERATOR_H_
#define RTC_BASE_UNIQUE_ID_GENERATOR_H_

#include <limits>
#include <set>
#include <string>

#include "absl/strings/string_view.h"
#include "api/array_view.h"
#include "api/sequence_checker.h"
#include "rtc_base/synchronization/mutex.h"
#include "rtc_base/system/no_unique_address.h"

namespace webrtc {

// This class will generate numbers. A common use case is for identifiers.
// The generated numbers will be unique, in the local scope of the generator.
// This means that a generator will never generate the same number twice.
// The generator can also be initialized with a sequence of known ids.
// In such a case, it will never generate an id from that list.
// Recommendedations:
//  * Prefer unsigned types.
//  * Prefer larger types (uint8_t will run out quickly).
template <typename TIntegral>
class UniqueNumberGenerator {
 public:
  typedef TIntegral value_type;
  UniqueNumberGenerator();
  // Creates a generator that will never return any value from the given list.
  explicit UniqueNumberGenerator(ArrayView<TIntegral> known_ids);
  ~UniqueNumberGenerator();

  // Generates a number that this generator has never produced before.
  // If there are no available numbers to generate, this method will fail
  // with an `RTC_CHECK`.
  TIntegral GenerateNumber();

  // Alias for GenerateId, used for allowing typed testing
  TIntegral Generate() { return GenerateNumber(); }

  // Adds an id that this generator should no longer generate.
  // Return value indicates whether the ID was hitherto unknown.
  bool AddKnownId(TIntegral value);

 private:
  RTC_NO_UNIQUE_ADDRESS SequenceChecker sequence_checker_{
      webrtc::SequenceChecker::kDetached};
  static_assert(std::is_integral<TIntegral>::value, "Must be integral type.");
  TIntegral counter_ RTC_GUARDED_BY(sequence_checker_);
  std::set<TIntegral> known_ids_ RTC_GUARDED_BY(sequence_checker_);
};

// This class will generate unique ids. Ids are 32 bit unsigned integers.
// The generated ids will be unique, in the local scope of the generator.
// This means that a generator will never generate the same id twice.
// The generator can also be initialized with a sequence of known ids.
// In such a case, it will never generate an id from that list.
class UniqueRandomIdGenerator {
 public:
  typedef uint32_t value_type;
  UniqueRandomIdGenerator();
  // Create a generator that will never return any value from the given list.
  explicit UniqueRandomIdGenerator(ArrayView<uint32_t> known_ids);
  ~UniqueRandomIdGenerator();

  // Generates a random id that this generator has never produced before.
  // This method becomes more expensive with each use, as the probability of
  // collision for the randomly generated numbers increases.
  uint32_t GenerateId();

  // Alias for GenerateId, used for allowing typed testing
  uint32_t Generate() { return GenerateId(); }

  // Adds an id that this generator should no longer generate.
  // Return value indicates whether the ID was hitherto unknown.
  bool AddKnownId(uint32_t value);

 private:
  // TODO(bugs.webrtc.org/12666): This lock is needed due to an instance in
  // SdpOfferAnswerHandler being shared between threads.
  Mutex mutex_;
  std::set<uint32_t> known_ids_ RTC_GUARDED_BY(&mutex_);
};

// This class will generate strings. A common use case is for identifiers.
// The generated strings will be unique, in the local scope of the generator.
// This means that a generator will never generate the same string twice.
// The generator can also be initialized with a sequence of known ids.
// In such a case, it will never generate an id from that list.
class UniqueStringGenerator {
 public:
  typedef std::string value_type;
  UniqueStringGenerator();
  explicit UniqueStringGenerator(ArrayView<std::string> known_ids);
  ~UniqueStringGenerator();

  std::string GenerateString();
  // Alias for GenerateString, used for allowing typed testing
  std::string Generate() { return GenerateString(); }

  // Adds an id that this generator should no longer generate.
  // Return value indicates whether the ID was hitherto unknown.
  bool AddKnownId(absl::string_view value);

 private:
  // This implementation will be simple and will generate "0", "1", ...
  UniqueNumberGenerator<uint32_t> unique_number_generator_;
};

template <typename TIntegral>
UniqueNumberGenerator<TIntegral>::UniqueNumberGenerator() : counter_(0) {}

template <typename TIntegral>
UniqueNumberGenerator<TIntegral>::UniqueNumberGenerator(
    ArrayView<TIntegral> known_ids)
    : counter_(0), known_ids_(known_ids.begin(), known_ids.end()) {}

template <typename TIntegral>
UniqueNumberGenerator<TIntegral>::~UniqueNumberGenerator() {}

template <typename TIntegral>
TIntegral UniqueNumberGenerator<TIntegral>::GenerateNumber() {
  RTC_DCHECK_RUN_ON(&sequence_checker_);
  while (true) {
    RTC_CHECK_LT(counter_, std::numeric_limits<TIntegral>::max());
    auto pair = known_ids_.insert(counter_++);
    if (pair.second) {
      return *pair.first;
    }
  }
}

template <typename TIntegral>
bool UniqueNumberGenerator<TIntegral>::AddKnownId(TIntegral value) {
  RTC_DCHECK_RUN_ON(&sequence_checker_);
  return known_ids_.insert(value).second;
}
}  //  namespace webrtc

// Re-export symbols from the webrtc namespace for backwards compatibility.
// TODO(bugs.webrtc.org/4222596): Remove once all references are updated.
#ifdef WEBRTC_ALLOW_DEPRECATED_NAMESPACES
namespace rtc {
using ::webrtc::UniqueNumberGenerator;
using ::webrtc::UniqueRandomIdGenerator;
using ::webrtc::UniqueStringGenerator;
}  // namespace rtc
#endif  // WEBRTC_ALLOW_DEPRECATED_NAMESPACES

#endif  // RTC_BASE_UNIQUE_ID_GENERATOR_H_
