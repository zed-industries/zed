// Copyright 2022 The Centipede Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef THIRD_PARTY_CENTIPEDE_KNOBS_H_
#define THIRD_PARTY_CENTIPEDE_KNOBS_H_

#include <algorithm>
#include <cstddef>
#include <cstdint>
#include <functional>
#include <string_view>

#include "absl/types/span.h"
#include "./common/defs.h"

namespace fuzztest::internal {

// Opaque ID object to be used by Knobs.
// Supported usage:
//   * Create a new KnobId global object via Knobs::New().
//   * Compare two KnobIds for equality.
//   * Pass to Knobs' member functions.
class KnobId {
 public:
  bool operator==(const KnobId& other) const { return id_ == other.id_; }

 private:
  friend class Knobs;
  FRIEND_TEST(Knobs, Choose);
  KnobId(size_t id) : id_(id) {}
  KnobId() = default;
  size_t id() const { return id_; }

  size_t id_ = {};
};

// Knobs (will) control all randomized choices made by the fuzzing engine.
//
// The intent is to find optimal values for knobs using machine learning.
//
// Examples of the choices that the engine can make using knobs:
// * Choosing whether to add a given element to the corpus based on what
//   features it has, its size, its resource consumption, etc.
// * Choosing a corpus element to mutate, or an element pair to cross-over.
//   E.g. make the choice depending on the features associated with elements,
//   their sizes, etc.
// * Choosing how to mutate.
//   E.g. whether to insert, overwrite, swap, etc., or whether to cross-over.
//
// `Knobs` is effectively a fixed-size array of bytes with named elements.
// The engine loads this array at startup or uses a default value zero.
// The engine may also pass Knobs to a custom mutator that supports it.
//
// Each knob has its own interpretation.
// Some knobs are probability weights, with `0` meaning "never" or "rare"
//  and 255 meaning "frequently".
// Some knobs have a meaning in combination with other knobs, e.g.
//  when choosing one of N strategies, N knobs will be used as weights.
// Some knobs may mean the number of repetitions of a certain process.
//
// A knob value is accessed via a KnobId.
// KnobIds are created by Knobs::New() as file-scope globals.
// The allocation of KnobIds is stable between the executions of the engine,
// but will change when the engine changes in some significant way
// (e.g. new knobs are added/removed or linking order changes).
// I.e. the optimal knob values will need to be re-learned after major changes
// in the engine.
// This way knobs can be created locally in every source file, w/o having a
// centralized knob repository.
//
// A KnobId can be used to access a knob value: Knobs::Value().
// A set of KnobIds can be used to choose from several choices: Knobs::Choose().
// One KnobID can be used to choose from two choices: Knobs::GenerateBool().
//
// TODO(kcc): figure out how to share knobs with other processes/binaries,
// such as custom mutators.
class Knobs {
 public:
  // Total number of knobs. Keep it small-ish for now.
  static constexpr size_t kNumKnobs = 32;
  using value_type = uint8_t;
  using signed_value_type = int8_t;

  // Creates and returns a new KnobId and associates a `knob_name` with it.
  // Must be called at the process startup (assign the result to a global):
  //   static const KnobId knob_weight_of_foo = Knobs::NewId("weight_of_foo");
  // Will trap if runs out of IDs.
  static KnobId NewId(std::string_view knob_name);

  // Returns the name associated with `knob_id`.
  static std::string_view Name(KnobId knob_id) {
    return knob_names_[knob_id.id()];
  }

  // Sets all knobs to the same value `value`.
  void Set(value_type value) {
    for (auto& knob : knobs_) {
      knob = value;
    }
  }

  // Sets the knobs to values from `values`. If `values.size() < kNumKnobs`,
  // only the first `values.size()` values will be set.
  void Set(absl::Span<const value_type> values) {
    size_t n = std::min(kNumKnobs, values.size());
    for (size_t i = 0; i < n; ++i) {
      knobs_[i] = values[i];
    }
  }

  // Returns the value associated with `knob_id`.
  value_type Value(KnobId knob_id) const {
    if (knob_id.id() >= kNumKnobs) __builtin_trap();
    return knobs_[knob_id.id()];
  }

  // Returns the signed value associated with `knob_id`.
  signed_value_type SignedValue(KnobId knob_id) const { return Value(knob_id); }

  // Calls `callback(Name, Value)` for every KnobId created by NewId().
  void ForEachKnob(
      const std::function<void(std::string_view, Knobs::value_type)>& callback)
      const {
    for (size_t i = 0; i < next_id_; ++i) {
      callback(Name(i), Value(i));
    }
  }

  // Returns one of the `choices`.
  // `knob_ids` and `choices` must have the same size and be non-empty.
  // Uses knob values associated with knob_ids as probability weights for
  // respective choices.
  // E.g. if knobs.Value(knobA) == 100 and knobs.Value(knobB) == 10, then
  // Choose<...>({knobA, knobB}, {A, B}, rng()) is approximately 10x more likely
  // to return A than B.
  //
  // If all knob values are zero, behaves as if they were all 1.
  //
  // `random` is a random number derived from an RNG.
  // TODO(kcc): consider making this more similar to GenerateBool() and
  // requiring 1 knob fewer than choices.size().
  template <typename T>
  T Choose(absl::Span<const KnobId> knob_ids, absl::Span<const T> choices,
           uint64_t random) const {
    if (choices.empty()) __builtin_trap();
    if (knob_ids.size() != choices.size()) __builtin_trap();
    size_t sum = 0;
    for (auto knob_id : knob_ids) {
      sum += Value(knob_id);
    }
    if (sum == 0) return choices[random % choices.size()];
    random %= sum;
    size_t partial_sum = 0;
    size_t idx = 0;
    for (auto knob_id : knob_ids) {
      partial_sum += Value(knob_id);
      if (partial_sum > random) return choices[idx];
      ++idx;
    }
    __builtin_unreachable();
  }

  // Chooses between two strategies, i.e. returns true or false.
  // Treats the value of the knob associated with `knob_id` as signed integer.
  // If knob == -128, returns false. If knob == 127 returns true.
  // For other values, returns randomly true of false, with higher probability
  // of true for higher values of knob.
  // If knob == 0, returns true with a ~ 50% chance.
  // `random` is a random number used to produce random choice.
  bool GenerateBool(KnobId knob_id, uint64_t random) const {
    signed_value_type signed_value = SignedValue(knob_id);  // in [-128,127]
    signed_value_type rand = random % 255 - 127;            // in [-127,127]
    // signed_value == 127 => always true.
    // signed_value == -128 => always false.
    // signed_value == 0 => true ~ half the time.
    return signed_value >= rand;
  }

  // Variant of Choose() where the choices are KnobIds themselves.
  // Returns one of the `choices` based on the respective knobs.
  KnobId Choose(absl::Span<const KnobId> choices, uint64_t random) const {
    return Choose<KnobId>(choices, choices, random);
  }

 private:
  static size_t next_id_;
  static std::string_view knob_names_[kNumKnobs];
  value_type knobs_[kNumKnobs] = {};
};
}  // namespace fuzztest::internal

#endif  // THIRD_PARTY_CENTIPEDE_KNOBS_H_
