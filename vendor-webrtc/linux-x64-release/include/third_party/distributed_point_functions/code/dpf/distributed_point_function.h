/*
 * Copyright 2021 Google LLC
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

#ifndef DISTRIBUTED_POINT_FUNCTIONS_DPF_DISTRIBUTED_POINT_FUNCTION_H_
#define DISTRIBUTED_POINT_FUNCTIONS_DPF_DISTRIBUTED_POINT_FUNCTION_H_

#include <algorithm>
#include <array>
#include <cstddef>
#include <cstdint>
#include <limits>
#include <memory>
#include <string>
#include <tuple>
#include <type_traits>
#include <utility>
#include <vector>

#include "absl/container/btree_map.h"
#include "absl/container/flat_hash_map.h"
#include "absl/container/inlined_vector.h"
#include "absl/log/absl_check.h"
#include "absl/meta/type_traits.h"
#include "absl/numeric/int128.h"
#include "absl/status/status.h"
#include "absl/status/statusor.h"
#include "absl/strings/str_cat.h"
#include "absl/strings/str_format.h"
#include "absl/strings/string_view.h"
#include "absl/types/span.h"
#include "dpf/aes_128_fixed_key_hash.h"
#include "dpf/distributed_point_function.pb.h"
#include "dpf/internal/evaluate_prg_hwy.h"
#include "dpf/internal/maybe_deref_span.h"
#include "dpf/internal/proto_validator.h"
#include "dpf/internal/value_type_helpers.h"
#include "google/protobuf/repeated_ptr_field.h"
#include "hwy/aligned_allocator.h"

namespace distributed_point_functions {

// Type trait for all supported types. Used to provide meaningful error messages
// in std::enable_if template guards.
template <typename T>
using is_supported_type = dpf_internal::is_supported_type<T>;
template <typename T>
constexpr bool is_supported_type_v = is_supported_type<T>::value;

// Converts a given Value to the template parameter T.
//
// Returns INVALID_ARGUMENT if the conversion fails.
template <typename T, typename = absl::enable_if_t<is_supported_type_v<T>>>
absl::StatusOr<T> FromValue(const Value& value) {
  return dpf_internal::ValueTypeHelper<T>::FromValue(value);
}

// ToValue Converts the argument to a Value.
template <typename T, typename = absl::enable_if_t<is_supported_type_v<T>>>
Value ToValue(const T& input) {
  return dpf_internal::ValueTypeHelper<T>::ToValue(input);
}

// ToValueType<T> Returns a `ValueType` message describing T.
template <typename T, typename = absl::enable_if_t<is_supported_type_v<T>>>
ValueType ToValueType() {
  return dpf_internal::ValueTypeHelper<T>::ToValueType();
}

// Implements key generation and evaluation of distributed point functions.
// A distributed point function (DPF) is parameterized by an index `alpha` and a
// value `beta`. The key generation procedure produces two keys `k_a`, `k_b`.
// Evaluating each key on any point `x` in the DPF domain results in an additive
// secret share of `beta`, if `x == alpha`, and a share of 0 otherwise. This
// class also supports *incremental* DPFs that can additionally be evaluated on
// prefixes of points, resulting in different values `beta_i`for each prefix of
// `alpha`.
class DistributedPointFunction {
 public:
  // Creates a new instance of a distributed point function that can be
  // evaluated only at the output layer.
  //
  // Returns INVALID_ARGUMENT if the parameters are invalid.
  static absl::StatusOr<std::unique_ptr<DistributedPointFunction>> Create(
      const DpfParameters& parameters);

  // Creates a new instance of an *incremental* DPF that can be evaluated at
  // multiple layers. Each parameter set in `parameters` should specify the
  // domain size and element size at one of the layers to be evaluated, in
  // increasing domain size order. Element sizes must be non-decreasing.
  //
  // Returns INVALID_ARGUMENT if the parameters are invalid.
  static absl::StatusOr<std::unique_ptr<DistributedPointFunction>>
  CreateIncremental(absl::Span<const DpfParameters> parameters);

  // DistributedPointFunction is neither copyable nor movable.
  DistributedPointFunction(const DistributedPointFunction&) = delete;
  DistributedPointFunction& operator=(const DistributedPointFunction&) = delete;

  // Converts the argument to a `Value` proto. Also registers the corresponding
  // value type with the DPF by calling `RegisterValueType<T>()`.
  template <typename T>
  absl::StatusOr<Value> ToValue(const T& in) {
    absl::Status status = RegisterValueType<T>();
    if (!status.ok()) {
      return status;
    }
    return distributed_point_functions::ToValue(in);
  }

  // Registers the template parameter type with this DPF. Note that it is rarely
  // necessary to call this function by hand: It is called by `Create` and
  // `CreateIncremental` for all unsigned integer types, including
  // absl::uint128, and on every call to ToValue<T>. Only call this function
  // when passing `Value`s created by other means than ToValue<T>.
  //
  // Returns OK on success and otherwise an INTERNAL status describing the
  // failure.
  template <typename T>
  absl::Status RegisterValueType() {
    return RegisterValueTypeImpl<T>(value_correction_functions_);
  }

  // Generates a pair of keys for a DPF that evaluates to `beta` when evaluated
  // `alpha`. The type of `beta` must match the ValueType passed in `parameters`
  // at construction.
  //
  // This function provides three overloads: One with `absl::uint128` for
  // `beta`, which implies the output type is a simple integer; One with a
  // `Value` proto for `beta`, which can be used for all supported value types;
  // And a templated version that computes the Value by calling ToValue<T> on
  // the argument.
  //
  // Example Usages (assuming a std::unique_ptr<DistributedPointFunction> dpf):
  //
  //   // Simple integer:
  //   dpf->GenerateKeys(23, 42);
  //
  //   // Explicit `Value` proto:
  //   Value value;
  //   value[1]->mutable_tuple->add_elements()
  //     ->mutable_integer->set_value_uint64(12);
  //   value[1]->mutable_tuple->add_elements()
  //     ->mutable_integer->set_value_uint64(34);
  //   // Must be called once before calling GenerateKeys for any type that is
  //   // not a simple integer. The type should match the one in the
  //   // DpfParameters passed at construction.
  //   dpf->RegisterValueType<Tuple<uint32_t, uint64_t>>();
  //   dpf->GenerateKeys(23, value);
  //
  //   // Templated version (no call to RegisterValueType needed):
  //   dpf->GenerateKeys(23, Tuple<uint32_t, uint64_t>{12, 34});
  //
  // Returns INVALID_ARGUMENT if used on an incremental DPF with more
  // than one set of parameters, if `alpha` is outside of the domain specified
  // at construction, or if `beta` does not match the value type passed at
  // construction.
  // Returns FAILED_PRECONDITION if `RegisterValueType<T>` has not been called
  // for the type in the `DpfParameters` passed at construction.

  // Overload for simple integers.
  absl::StatusOr<std::pair<DpfKey, DpfKey>> GenerateKeys(absl::uint128 alpha,
                                                         absl::uint128 beta) {
    return GenerateKeysIncremental(alpha, absl::MakeConstSpan(&beta, 1));
  }

  // Overload for explicit Value proto.
  absl::StatusOr<std::pair<DpfKey, DpfKey>> GenerateKeys(absl::uint128 alpha,
                                                         Value beta) {
    return GenerateKeysIncremental(alpha, absl::MakeConstSpan(&beta, 1));
  }

  // Template for automatic conversion to Value proto. Disabled if the argument
  // is convertible to `absl::uint128` or `Value` to make overloading
  // unambiguous.
  template <typename T, typename = absl::enable_if_t<
                            !std::is_convertible<T, absl::uint128>::value &&
                            !std::is_convertible<T, Value>::value &&
                            is_supported_type_v<T>>>
  absl::StatusOr<std::pair<DpfKey, DpfKey>> GenerateKeys(absl::uint128 alpha,
                                                         const T& beta) {
    absl::StatusOr<Value> value = ToValue<T>(beta);
    if (!value.ok()) {
      return value.status();
    }
    return GenerateKeysIncremental(alpha, absl::MakeConstSpan(&(*value), 1));
  }

  // Generates a pair of keys for an incremental DPF. For each parameter i
  // passed at construction, the DPF evaluates to `beta[i]` at the lowest
  // `parameters_[i].log_domain_size()` bits of `alpha`.
  //
  // Similar to `GenerateKeys`, supports three overloads: One for simple
  // integers, passed as an `absl::Span<const absl::uint128>`; One for a span of
  // `Value` protos; And a variadic function template that automatically
  // converts the passed arguments to a vector of `Value`s.
  //
  // Example Usages (assuming a std::unique_ptr<DistributedPointFunction> dpf):
  //
  //   // Simple integers:
  //   std::vector<absl::uint128> beta{123, 456};
  //   dpf->GenerateKeysIncremental(23, beta);
  //
  //   // Explicit Value protos:
  //   std::vector<Value> beta(2);
  //   value[0]->mutable_integer()->set_value_uint128(42);
  //   value[1]->mutable_tuple->add_elements()
  //   ->mutable_integer->set_value_uint64(12);
  //   value[1]->mutable_tuple->add_elements()
  //   ->mutable_integer->set_value_uint64(34);
  //   // Must be called once before calling GenerateKeys for any type that is
  //   // not a simple integer. The type should match the one in the
  //   // DpfParameters passed at construction.
  //   dpf->RegisterValueType<Tuple<uint32_t, uint64_t>>();
  //   dpf->GenerateKeysIncremental(23, beta);
  //
  //   // Templated version (equivalent to the one above):
  //   dpf->GenerateKeysIncremental(23, 42, Tuple<uint32_t, uint64_t>{12, 34}));
  //
  // Returns INVALID_ARGUMENT if `beta.size() != parameters_.size()`, if `alpha`
  // is outside of the domain specified at construction, or if `beta` does not
  // match the element type passed at construction.
  // Returns FAILED_PRECONDITION if `RegisterValueType<T>` has not been called
  // for all types in the `DpfParameters` passed at construction.

  // Legacy interface for absl::uint128, which doesn't require explicitly
  // converting to absl::Span<const absl::uint128>.
  absl::StatusOr<std::pair<DpfKey, DpfKey>> GenerateKeysIncremental(
      absl::uint128 alpha, const std::vector<absl::uint128>& beta) {
    return GenerateKeysIncremental(alpha, absl::MakeConstSpan(beta));
  }

  // Templated version when all value types are equal.
  template <typename T>
  absl::StatusOr<std::pair<DpfKey, DpfKey>> GenerateKeysIncremental(
      absl::uint128 alpha, absl::Span<const T> beta) {
    std::vector<Value> values(beta.size());
    for (int i = 0; i < static_cast<int>(beta.size()); ++i) {
      absl::StatusOr<Value> value = ToValue(beta[i]);
      if (!value.ok()) {
        return value.status();
      }
      values[i] = std::move(*value);
    }
    return GenerateKeysIncremental(alpha, values);
  }

  // Overload for Value protos.
  absl::StatusOr<std::pair<DpfKey, DpfKey>> GenerateKeysIncremental(
      absl::uint128 alpha, absl::Span<const Value> beta);

  // Variadic template version. Disabled if the first argument is convertible to
  // a span of `absl::uint128`s or `Value`s to make overloading unambiguous.
  template <
      typename T0, typename... Tn,
      typename = absl::enable_if_t<
          !std::is_convertible<T0, absl::Span<const Value>>::value &&
          !std::is_convertible<T0, absl::Span<const absl::uint128>>::value &&
          absl::conjunction<is_supported_type<T0>,
                            is_supported_type<Tn>...>::value>>
  absl::StatusOr<std::pair<DpfKey, DpfKey>> GenerateKeysIncremental(
      absl::uint128 alpha, T0&& beta_0, Tn&&... beta_n);

  // Returns an `EvaluationContext` for incrementally evaluating the given
  // DpfKey.
  //
  // Returns INVALID_ARGUMENT if `key` doesn't match the parameters given at
  // construction.
  absl::StatusOr<EvaluationContext> CreateEvaluationContext(DpfKey key) const;

  // Evaluates the given `hierarchy_level` of the DPF under all `prefixes`
  // passed to this function. If `prefixes` is empty, evaluation starts from the
  // seed of `ctx.key`. Otherwise, each element of `prefixes` must fit in the
  // domain size of `ctx.previous_hierarchy_level`. Further, `prefixes` may only
  // contain extensions of the prefixes passed in the previous call. For
  // example, in the following sequence of calls, for each element p2 of
  // `prefixes2`, there must be an element p1 of `prefixes1` such that p1 is a
  // prefix of p2:
  //
  //   DPF_ASSIGN_OR_RETURN(std::unique_ptr<EvaluationContext> ctx,
  //                        dpf->CreateEvaluationContext(key));
  //   using T0 = ...;
  //   DPF_ASSIGN_OR_RETURN(std::vector<T0> evaluations0,
  //                        dpf->EvaluateUntil(0, {}, *ctx));
  //
  //   std::vector<absl::uint128> prefixes1 = ...;
  //   using T1 = ...;
  //   DPF_ASSIGN_OR_RETURN(std::vector<T1> evaluations1,
  //                        dpf->EvaluateUntil(1, prefixes1, *ctx));
  //   ...
  //   std::vector<absl::uint128> prefixes2 = ...;
  //   using T2 = ...;
  //   DPF_ASSIGN_OR_RETURN(std::vector<T2> evaluations2,
  //                        dpf->EvaluateUntil(3, prefixes2, *ctx));
  //
  // The prefixes are read from the lowest-order bits of the corresponding
  // absl::uint128. The number of bits used for each prefix depends on the
  // output domain size of the previously evaluated hierarchy level. For
  // example, if `ctx` was last evaluated on a hierarchy level with output
  // domain size 2**20, then the 20 lowest-order bits of each element in
  // `prefixes` are used.
  //
  // Returns `INVALID_ARGUMENT` if
  //   - any element of `prefixes` is larger than the next hierarchy level's
  //     log_domain_size,
  //   - `prefixes` contains elements that are not extensions of previous
  //     prefixes, or
  //   - the bit-size of T doesn't match the next hierarchy level's
  //     element_bitsize.
  template <typename T>
  absl::StatusOr<std::vector<T>> EvaluateUntil(
      int hierarchy_level, absl::Span<const absl::uint128> prefixes,
      EvaluationContext& ctx) const;

  template <typename T>
  absl::StatusOr<std::vector<T>> EvaluateNext(
      absl::Span<const absl::uint128> prefixes, EvaluationContext& ctx) const {
    if (prefixes.empty()) {
      return EvaluateUntil<T>(0, prefixes, ctx);
    } else {
      return EvaluateUntil<T>(ctx.previous_hierarchy_level() + 1, prefixes,
                              ctx);
    }
  }

  // Evaluates a single key at one or multiple points, up to the given
  // `hierarchy_level`. Each element of `evaluation_points` must be within the
  // domain of this DPF at `hierarchy_level`.
  //
  // Example:
  //
  //   DpfKey key = ...;
  //   std::vector<absl::uint128> evaluation_points = {1, 23, 42};
  //   // Evaluate `key` on {1, 23, 42}.
  //   DPF_ASSIGN_OR_RETURN(std::vector<T> result,
  //                        dpf->EvaluateAt(key, 0, evaluation_points);
  //
  // Returns INVALID_ARGUMENT if `key` is malformed, or if `hierarchy_level` or
  // any element of `evaluation_points` is out of range.
  template <typename T>
  absl::StatusOr<std::vector<T>> EvaluateAt(
      const DpfKey& key, int hierarchy_level,
      absl::Span<const absl::uint128> evaluation_points) const {
    return EvaluateAtImpl<T>(key, hierarchy_level, evaluation_points, nullptr);
  }

  // Evaluates a single key at one or multiple points, up to the given
  // `hierarchy_level`. Each element of `evaluation_points` must be within the
  // domain of this DPF at `hierarchy_level`.
  //
  // If `ctx.partial_evaluations_size() != 0`, uses the given partial
  // evaluations as starting point of the DPF evaluation. Otherwise, the result
  // is equivalent to calling `EvaluateAt(ctx.key(), hierarchy_level,
  // evaluation_points)`.
  //
  // When successful, `ctx` is updated to include partial evaluations at
  // `hierarchy_level`. The contents of `ctx` are undefined in case of an error.
  //
  // Returns INVALID_ARGUMENT if `ctx` is malformed, if `hierarchy_level` or
  // any element of `evaluation_points` is out of range, or
  // `ctx.partial_evaluations()` does not contain the prefixes of all
  // `evaluation_points` at `ctx.partial_evaluations_level()`.
  template <typename T>
  absl::StatusOr<std::vector<T>> EvaluateAt(
      int hierarchy_level, absl::Span<const absl::uint128> evaluation_points,
      EvaluationContext& ctx) const {
    return EvaluateAtImpl<T>(ctx.key(), hierarchy_level, evaluation_points,
                             &ctx);
  }

  // Evaluates a span of DPF keys. The i-th key is evaluated at
  // evaluation_points[i]. After each hierarchy level, calls `op` on the output
  // at that hierarchy level. `op` must be callable with the following
  // signature:
  //
  //  op(int hierarchy_level, absl::Span<T> values)
  //
  // It should return a value that is implicitly convertible to `bool`.
  //
  // This method is intended for use cases similar to
  //
  // absl::StatusOr<std::vector<T>> EvaluateAt(
  //   int hierarchy_level, absl::Span<const absl::uint128> evaluation_points,
  //   EvaluationContext& ctx)
  //
  // but without the overhead of EvaluationContext. Instead, all operations on
  // intermediate values, and obtaining the final result, should be done via
  // `op`.
  //
  // Return absl::OkStatus() after successfully evaluating `op` on the last
  // hierarchy level, or as soon as `op` returns `false`. Returns
  // INVALID_ARGUMENT in case any `key` is malformed, or if any of the
  // `evaluation_points` are out of range.
  template <typename T, typename Fn>
  absl::Status EvaluateAndApply(
      dpf_internal::MaybeDerefSpan<const DpfKey>,
      absl::Span<const absl::uint128> evaluation_points, Fn op,
      int evaluation_points_rightshift = 0) const;

  // Returns the DpfParameters of this DPF.
  inline absl::Span<const DpfParameters> parameters() const {
    return parameters_;
  }

 private:
  // BitVector is a vector of bools. Allows for faster access times than
  // std::vector<bool>, as well as inlining if the size is small.
  using BitVector =
      absl::InlinedVector<bool,
                          std::max<size_t>(1, sizeof(bool*) / sizeof(bool))>;

  // Seeds and control bits resulting from a DPF expansion. This type is
  // returned by `ExpandSeeds` and `ExpandAndUpdateContext`.
  struct DpfExpansion {
    // Ensures that seeds are aligned correctly for SIMD operations.
    hwy::AlignedFreeUniquePtr<absl::uint128[]> seeds;
    BitVector control_bits;
  };

  // A function for computing value corrections. Used as return type in
  // `GetValueCorrectionFunction`.
  using ValueCorrectionFunction = absl::StatusOr<std::vector<Value>> (*)(
      absl::string_view, absl::string_view, int block_index, const Value&,
      bool);

  // Private constructor, called by `CreateIncremental`.
  DistributedPointFunction(
      std::unique_ptr<dpf_internal::ProtoValidator> proto_validator,
      std::vector<int> blocks_needed, Aes128FixedKeyHash prg_left,
      Aes128FixedKeyHash prg_right, Aes128FixedKeyHash prg_value,
      absl::flat_hash_map<std::string, ValueCorrectionFunction>
          value_correction_functions);

  // Computes the value correction for the given `hierarchy_level`, `seeds`,
  // index `alpha` and value `beta`. If `invert` is true, the individual values
  // in the returned block are multiplied element-wise by -1. Expands `seeds`
  // using `prg_ctx_value_`, then calls the function returned by
  // `GetValueCorrectionFunction(parameters_[hierarchy_level])` to obtain the
  // value correction words.
  //
  // Returns multiple values in the case of packing, and a single Value
  // otherwise.
  //
  // Returns INTERNAL in case the PRG expansion fails, and UNIMPLEMENTED if
  // `element_bitsize` is not supported.
  absl::StatusOr<std::vector<Value>> ComputeValueCorrection(
      int hierarchy_level, absl::Span<const absl::uint128> seeds,
      absl::uint128 alpha, const Value& beta, bool invert) const;

  // Expands the PRG seeds at the next `tree_level` for an incremental DPF with
  // index `alpha` and values `beta`, updates `seeds` and `control_bits`, and
  // writes the next correction word to `keys`. Called from
  // `GenerateKeysIncremental`.
  absl::Status GenerateNext(int tree_level, absl::uint128 alpha,
                            absl::Span<const Value> beta,
                            absl::Span<absl::uint128> seeds,
                            absl::Span<bool> control_bits,
                            absl::Span<DpfKey> keys) const;

  // Computes the tree index (representing a path in the FSS tree) from the
  // given `domain_index` and `hierarchy_level`. Does NOT check whether the
  // given domain index fits in the domain at `hierarchy_level`.
  absl::uint128 DomainToTreeIndex(absl::uint128 domain_index,
                                  int hierarchy_level) const;

  // Computes the block index (pointing to an element in a batched 128-bit
  // block) from the given `domain_index` and `hierarchy_level`. Does NOT check
  // whether the given domain index fits in the domain at `hierarchy_level`.
  int DomainToBlockIndex(absl::uint128 domain_index, int hierarchy_level) const;

  // Performs DPF evaluation of the given `seeds` using prg_ctx_left_ or
  // prg_ctx_right_, and the given `control_bits` and `correction_words`. At
  // each level `l < correction_words.size()`, the evaluation for the i-th seed
  // in `partial_evaluations` continues along the left or right path depending
  // on the l-th most significant bit among the lowest `correction_words.size()`
  // bits of `paths[i]`.
  //
  // The output is written to `seeds_out` and `control_bits_out`. These may
  // overlap with `seeds` and `control_bits`. We use output spans instead of a
  // return value to allow the caller to pre-allocate aligned output arrays,
  // which is necessary for the vectorized implementation. The output is
  // undefined if `correction_words.size() == 0`.
  //
  // Returns INVALID_ARGUMENT if the input sizes don't match.
  // Returns INTERNAL in case of OpenSSL errors.
  absl::Status EvaluateSeeds(
      absl::Span<const absl::uint128> seeds,
      absl::Span<const bool> control_bits,
      absl::Span<const absl::uint128> paths,
      absl::Span<const CorrectionWord* const> correction_words,
      absl::Span<absl::uint128> seeds_out,
      absl::Span<bool> control_bits_out) const;

  // Performs DPF expansion of the given `partial_evaluations` using
  // prg_ctx_left_ and prg_ctx_right_, and the given `correction_words`. In
  // more detail, each of the partial evaluations is subjected to a full
  // subtree expansion of `correction_words.size()` levels, and the
  // concatenated result is provided in the response. The result contains
  // `(partial_evaluations.size() * (2^correction_words.size())` evaluations
  // in a single `DpfExpansion`.
  //
  // Returns INTERNAL in case of OpenSSL errors.
  absl::StatusOr<DpfExpansion> ExpandSeeds(
      const DpfExpansion& partial_evaluations,
      absl::Span<const CorrectionWord* const> correction_words) const;

  // Computes partial evaluations of the paths to `prefixes` up to
  // `hierarchy_level`, to be used as the starting point of the expansion of
  // `ctx`. If `update_ctx
  // == true`, saves the partial evaluations of `ctx.previous_hierarchy_level`
  // to `ctx` and sets `ctx.partial_evaluations_level` to
  // `ctx.previous_hierarchy_level`. Called by `ExpandAndUpdateContext`.
  //
  // Returns INVALID_ARGUMENT if any element of `prefixes` is not found in
  // `ctx.partial_evaluations()`, or `ctx.partial_evaluations()` contains
  // duplicate prefixes with inconsistent seeds or control bits.
  absl::StatusOr<DpfExpansion> ComputePartialEvaluations(
      absl::Span<const absl::uint128> prefixes, int hierarchy_level,
      bool update_ctx, EvaluationContext& ctx) const;

  // Extracts the seeds for the given `prefixes` from `ctx` and expands them as
  // far as needed for the next hierarchy level. Returns the result as a
  // `DpfExpansion`. Called by `EvaluateUntil`, where the expanded seeds are
  // corrected to obtain output values.
  // After expansion, `ctx.hierarchy_level()` is increased. If this isn't the
  // last expansion, the expanded seeds are also saved in `ctx` for the next
  // expansion.
  //
  // Returns INVALID_ARGUMENT if any element of `prefixes` is not found in
  // `ctx.partial_evaluations()`, or `ctx.partial_evaluations()` contains
  // duplicate prefixes with inconsistent seeds or control bits. Returns
  // INTERNAL in case of OpenSSL errors.
  absl::StatusOr<DpfExpansion> ExpandAndUpdateContext(
      int hierarchy_level, absl::Span<const absl::uint128> prefixes,
      EvaluationContext& ctx) const;

  // Compute output PRG value of expanded seeds using prg_ctx_value_.
  // Returns blocks_needed_[hierarchy_level] * expansion.seeds.size() blocks,
  // where every blocks_needed_[hierarchy_level] correspond to the hash of an
  // input seed.
  //
  // Returns INTERNAL in case of OpenSSL errors.
  absl::StatusOr<hwy::AlignedFreeUniquePtr<absl::uint128[]>> HashExpandedSeeds(
      int hierarchy_level, absl::Span<const absl::uint128> expansion) const;

  // Deterministically serializes the given value_type.
  //
  // Returns OK on success and INTERNAL in case serialization fails.
  static absl::StatusOr<std::string> SerializeValueTypeDeterministically(
      const ValueType& value_type);

  // Returns the value correction function for the given parameters.
  // For all value types except unsigned integers, these functions have to be
  // first registered using RegisterValueType<T>.
  //
  // Returns UNIMPLEMENTED if no matching function was registered.
  absl::StatusOr<ValueCorrectionFunction> GetValueCorrectionFunction(
      const DpfParameters& parameters) const;

  // Static implementation of RegisterValueType<T>, so we can call it from
  // `Create`.
  template <typename T>
  static absl::Status RegisterValueTypeImpl(
      absl::flat_hash_map<std::string, ValueCorrectionFunction>&
          value_correction_functions);

  // For the given `key` and `hierarchy_level`, returns the value correction
  // words as an array of integers, where the size of the array matches the
  // number of batched elements per block.
  template <typename T>
  absl::StatusOr<std::array<T, dpf_internal::ElementsPerBlock<T>()>>
  GetValueCorrectionAsArray(const DpfKey& key, int hierarchy_level) const;

  // Joint implementation of the two variants of `EvaluateAt<T>`. If `ctx !=
  // NULL`, `key` must point to `ctx->key()`, and `*ctx` will be updated with
  // the partial evaluations at this `hierarchy_level`.
  //
  template <typename T>
  absl::StatusOr<std::vector<T>> EvaluateAtImpl(
      const DpfKey& key, int hierarchy_level,
      absl::Span<const absl::uint128> evaluation_points,
      EvaluationContext* ctx) const;

  // Used to validate DpfParameters, DpfKey and EvaluationContext protos.
  const std::unique_ptr<dpf_internal::ProtoValidator> proto_validator_;

  // DP parameters passed to the factory function. Contains the domain size and
  // element size for hierarchy level of the incremental DPF. Owned by
  // proto_validator_.
  const absl::Span<const DpfParameters> parameters_;

  // Number of levels in the evaluation tree. This is always less than or equal
  // to the largest log_domain_size in parameters_.
  const int tree_levels_needed_;

  // Maps levels of the FSS evaluation tree to hierarchy levels (i.e., elements
  // of parameters_).
  const absl::flat_hash_map<int, int>& tree_to_hierarchy_;

  // The inverse of tree_to_hierarchy_.
  const std::vector<int>& hierarchy_to_tree_;

  // Cached numbers of AES blocks needed for value correction at each hierarchy
  // level.
  const std::vector<int> blocks_needed_;

  // Pseudorandom generator used for seed expansion (left and right), and value
  // correction. The PRG G(x) for hierarchy level i is defined as the
  // concatenation of
  //
  //   H_left(x), H_right(x), H_value(x + 0), ..., H_value(x + k-1)
  //
  // where k is equal to blocks_needed_[i], and H_*(x) is the evaluation of
  // prg_*_ on input x.
  const Aes128FixedKeyHash prg_left_;
  const Aes128FixedKeyHash prg_right_;
  const Aes128FixedKeyHash prg_value_;

  // Maps serialized `ValueType` messages to the correct value correction
  // functions. Map values are instantiations of
  // `dpf_internal::ComputeValueCorrectionFor`. Relies on protobuf's
  // deterministic serialization feature. This has the caveat that messages with
  // unknown fields are not supported. However, as long as `ValueType` consists
  // of a single `oneof` field, this is fine, since we either know the value
  // type and have deterministic serialization because the `ValueType` can only
  // contain one field, or we don't know the type and wouldn't be able to
  // correct values for it anyway.
  absl::flat_hash_map<std::string, ValueCorrectionFunction>
      value_correction_functions_;
};

//========================//
// Implementation Details //
//========================//

template <typename T>
absl::Status DistributedPointFunction::RegisterValueTypeImpl(
    absl::flat_hash_map<std::string, ValueCorrectionFunction>&
        value_correction_functions) {
  ValueType value_type = ToValueType<T>();
  absl::StatusOr<std::string> serialized_value_type =
      SerializeValueTypeDeterministically(value_type);
  if (!serialized_value_type.ok()) {
    return serialized_value_type.status();
  }
  value_correction_functions[*serialized_value_type] =
      dpf_internal::ComputeValueCorrectionFor<T>;
  return absl::OkStatus();
}

template <typename T0, typename... Tn, typename /*= absl::enable_if_t<...>*/>
absl::StatusOr<std::pair<DpfKey, DpfKey>>
DistributedPointFunction::GenerateKeysIncremental(absl::uint128 alpha,
                                                  T0&& beta_0, Tn&&... beta_n) {
  // Convert the first element of beta. We need to treat it separately to be
  // able to check its type in the enable_if above.
  absl::StatusOr<Value> value = ToValue(beta_0);
  if (!value.ok()) {
    return value.status();
  }
  std::vector<Value> values = {std::move(*value)};
  values.reserve(1 + sizeof...(beta_n));
  // Convert all values in the parameter pack, stopping at the first error.
  absl::Status status = absl::OkStatus();
  // We create an unused std::tuple<Tn...> here, because its braced-initializer
  // list constructor allows us to operate on beta_n in a well-defined order. In
  // C++17, this could be replaced by a fold expression instead.
  std::tuple<Tn...>{[this, &status, &values, &value](auto&& beta_i) -> Tn {
    if (status.ok()) {
      value = this->ToValue(beta_i);
      if (value.ok()) {
        values.push_back(std::move(*value));
      } else {
        status = value.status();
      }
    }
    return Tn{};
  }(beta_n)...};
  // Return if there was an error during conversion, otherwise generate keys.
  if (!status.ok()) {
    return status;
  }
  return GenerateKeysIncremental(alpha, values);
}

template <typename T>
absl::StatusOr<std::vector<T>> DistributedPointFunction::EvaluateUntil(
    int hierarchy_level, absl::Span<const absl::uint128> prefixes,
    EvaluationContext& ctx) const {
  absl::Status status = proto_validator_->ValidateEvaluationContext(ctx);
  if (!status.ok()) {
    return status;
  }
  if (hierarchy_level < 0 ||
      hierarchy_level >= static_cast<int>(parameters_.size())) {
    return absl::InvalidArgumentError(
        "`hierarchy_level` must be non-negative and less than "
        "parameters_.size()");
  }
  absl::StatusOr<bool> types_are_equal = dpf_internal::ValueTypesAreEqual(
      ToValueType<T>(), parameters_[hierarchy_level].value_type());
  if (!types_are_equal.ok()) {
    return types_are_equal.status();
  } else if (!*types_are_equal) {
    return absl::InvalidArgumentError(
        "Value type T doesn't match parameters at `hierarchy_level`");
  }
  if (hierarchy_level <= ctx.previous_hierarchy_level()) {
    return absl::InvalidArgumentError(
        "`hierarchy_level` must be greater than "
        "`ctx.previous_hierarchy_level`");
  }
  if ((ctx.previous_hierarchy_level() < 0) != (prefixes.empty())) {
    return absl::InvalidArgumentError(
        "`prefixes` must be empty if and only if this is the first call with "
        "`ctx`.");
  }

  int previous_log_domain_size = 0;
  int previous_hierarchy_level = ctx.previous_hierarchy_level();
  if (!prefixes.empty()) {
    ABSL_DCHECK_GE(ctx.previous_hierarchy_level(), 0);
    previous_log_domain_size =
        parameters_[previous_hierarchy_level].log_domain_size();
    for (absl::uint128 prefix : prefixes) {
      if (previous_log_domain_size < 128 &&
          prefix >= (absl::uint128{1} << previous_log_domain_size)) {
        return absl::InvalidArgumentError(
            absl::StrFormat("Index %d out of range for hierarchy level %d",
                            prefix, previous_hierarchy_level));
      }
    }
  }
  int64_t prefixes_size = static_cast<int64_t>(prefixes.size());

  // Check that the output size is not too large. We first check that the
  // domain size blowup fits in an int64_t, and then check that the total size
  // of all elements doesn't over flow a size_t.
  int log_domain_size = parameters_[hierarchy_level].log_domain_size();
  if (log_domain_size - previous_log_domain_size >= 63) {
    return absl::InvalidArgumentError(
        "Domain size gap too large. Please evaluate fewer hierarchy "
        "levels at once, or insert intermediate hierarchy levels.");
  }
  int64_t outputs_per_prefix = int64_t{1}
                               << (log_domain_size - previous_log_domain_size);
  if (absl::uint128{prefixes_size} * outputs_per_prefix >
      std::numeric_limits<size_t>::max() / 2) {
    return absl::InvalidArgumentError(
        "Output size would be too large. Please evaluate fewer hierarchy "
        "levels at once, insert intermediate hierarchy levels, or evaluate on "
        "fewer prefixes at once.");
  }

  // The `prefixes` passed in by the caller refer to the domain of the previous
  // hierarchy level. However, because we batch multiple elements of type T in a
  // single uint128 block, multiple prefixes can actually refer to the same
  // block in the FSS evaluation tree. On a high level, our approach is as
  // follows:
  //
  // 1. Split up each element of `prefixes` into a tree index, pointing to a
  //    block in the FSS tree, and a block index, pointing to an element of type
  //    T in that block.
  //
  // 2. Compute a list of unique `tree_indices`, and for each original prefix,
  //    remember the position of the corresponding tree index in `tree_indices`.
  //
  // 3. After expanding the unique `tree_indices`, use the positions saved in
  //    Step (2) together with the corresponding block index to retrieve the
  //    expanded values for each prefix, and return them in the same order as
  //    `prefixes`.
  //
  // `tree_indices` holds the unique tree indices from `prefixes`, to be passed
  // to `ExpandAndUpdateContext`.
  std::vector<absl::uint128> tree_indices;
  tree_indices.reserve(prefixes_size);
  // `tree_indices_inverse` is the inverse of `tree_indices`, used for
  // deduplicating and constructing `prefix_map`. Use a btree_map because we
  // expect `prefixes` (and thus `tree_indices`) to be sorted.
  absl::btree_map<absl::uint128, int64_t> tree_indices_inverse;
  // `prefix_map` maps each i < prefixes.size() to an element of `tree_indices`
  // and a block index. Used to select which elements to return after the
  // expansion, to ensure the result is ordered the same way as `prefixes`.
  std::vector<std::pair<int64_t, int>> prefix_map;
  prefix_map.reserve(prefixes_size);
  for (int64_t i = 0; i < prefixes_size; ++i) {
    absl::uint128 tree_index =
        DomainToTreeIndex(prefixes[i], previous_hierarchy_level);
    int block_index = DomainToBlockIndex(prefixes[i], previous_hierarchy_level);

    // Check if `tree_index` already exists in `tree_indices`.
    size_t previous_size = tree_indices_inverse.size();
    auto it = tree_indices_inverse.try_emplace(tree_indices_inverse.end(),
                                               tree_index, tree_indices.size());
    if (tree_indices_inverse.size() > previous_size) {
      tree_indices.push_back(tree_index);
    }
    prefix_map.push_back(std::make_pair(it->second, block_index));
  }

  // Perform expansion of unique `tree_indices`.
  absl::StatusOr<DpfExpansion> expansion =
      ExpandAndUpdateContext(hierarchy_level, tree_indices, ctx);
  if (!expansion.ok()) {
    return expansion.status();
  }
  const auto expansion_size =
      static_cast<int64_t>(expansion->control_bits.size());
  auto seeds = absl::MakeConstSpan(expansion->seeds.get(), expansion_size);

  // Hash the expanded seeds.
  absl::StatusOr<hwy::AlignedFreeUniquePtr<absl::uint128[]>> hashed_expansion =
      HashExpandedSeeds(hierarchy_level, seeds);
  if (!hashed_expansion.ok()) {
    return hashed_expansion.status();
  }

  // Get output correction word from `ctx`.
  constexpr int elements_per_block = dpf_internal::ElementsPerBlock<T>();
  const ::google::protobuf::RepeatedPtrField<Value>* value_correction = nullptr;
  if (hierarchy_level < static_cast<int>(parameters_.size()) - 1) {
    value_correction =
        &(ctx.key()
              .correction_words(hierarchy_to_tree_[hierarchy_level])
              .value_correction());
  } else {
    // Last level value correction is stored in an extra proto field, since we
    // have one less correction word than tree levels.
    value_correction = &(ctx.key().last_level_value_correction());
  }

  // Split output correction into elements of type T.
  absl::StatusOr<std::array<T, elements_per_block>> correction_ints =
      dpf_internal::ValuesToArray<T>(*value_correction);
  if (!correction_ints.ok()) {
    return correction_ints.status();
  }

  // Compute value corrections for each block in `expanded_seeds`. We have to
  // account for the fact that blocks might not be full (i.e., have less than
  // elements_per_block elements).
  const int corrected_elements_per_block =
      1 << (parameters_[hierarchy_level].log_domain_size() -
            hierarchy_to_tree_[hierarchy_level]);
  const int blocks_needed = blocks_needed_[hierarchy_level];
  ABSL_DCHECK(corrected_elements_per_block <= elements_per_block);
  std::vector<T> corrected_expansion(expansion_size *
                                     corrected_elements_per_block);
  for (int64_t i = 0; i < expansion_size; ++i) {
    std::array<T, elements_per_block> current_elements =
        dpf_internal::ConvertBytesToArrayOf<T>(absl::string_view(
            reinterpret_cast<const char*>(hashed_expansion->get() +
                                          i * blocks_needed),
            blocks_needed * sizeof(absl::uint128)));
    for (int j = 0; j < corrected_elements_per_block; ++j) {
      if (expansion->control_bits[i]) {
        current_elements[j] += (*correction_ints)[j];
      }
      if (ctx.key().party() == 1) {
        current_elements[j] = -current_elements[j];
      }
      corrected_expansion[i * corrected_elements_per_block + j] =
          current_elements[j];
    }
  }

  if (prefixes.empty()) {
    // If prefixes is empty (i.e., this is the first evaluation of `ctx`), just
    // return the expansion.
    ABSL_DCHECK(static_cast<int>(corrected_expansion.size()) ==
                outputs_per_prefix);
    return corrected_expansion;
  } else {
    // Otherwise, only return elements under `prefixes`.
    int blocks_per_tree_prefix =
        expansion->control_bits.size() / tree_indices.size();
    std::vector<T> result(prefixes_size * outputs_per_prefix);
    for (int64_t i = 0; i < prefixes_size; ++i) {
      int64_t prefix_expansion_start =
          prefix_map[i].first * blocks_per_tree_prefix *
              corrected_elements_per_block +
          prefix_map[i].second * outputs_per_prefix;
      std::copy_n(&corrected_expansion[prefix_expansion_start],
                  outputs_per_prefix, &result[i * outputs_per_prefix]);
    }
    return result;
  }
}

template <typename T>
absl::StatusOr<std::array<T, dpf_internal::ElementsPerBlock<T>()>>
DistributedPointFunction::GetValueCorrectionAsArray(const DpfKey& key,
                                                    int hierarchy_level) const {
  // Get output correction word from `key`.
  const ::google::protobuf::RepeatedPtrField<Value>* value_correction = nullptr;
  if (hierarchy_level < static_cast<int>(parameters_.size()) - 1) {
    value_correction =
        &(key.correction_words(hierarchy_to_tree_[hierarchy_level])
              .value_correction());
  } else {
    // Last level value correction is stored in an extra proto field, since we
    // have one less correction word than tree levels.
    value_correction = &(key.last_level_value_correction());
  }

  // Split output correction into elements of type T, and return it.
  return dpf_internal::ValuesToArray<T>(*value_correction);
}

template <typename T>
absl::StatusOr<std::vector<T>> DistributedPointFunction::EvaluateAtImpl(
    const DpfKey& key, int hierarchy_level,
    absl::Span<const absl::uint128> evaluation_points,
    EvaluationContext* ctx) const {
  if (ctx != nullptr) {
    if (&key != &ctx->key()) {
      return absl::InvalidArgumentError(
          "`key` and `ctx->key()` must refer to the same object");
    }
  }
  if (hierarchy_level < 0) {
    return absl::InvalidArgumentError("`hierarchy_level` must be non-negative");
  }
  if (hierarchy_level >= static_cast<int>(parameters_.size())) {
    return absl::InvalidArgumentError(
        "`hierarchy_level` must be less than the number of parameters passed "
        "at construction");
  }
  const auto num_evaluation_points =
      static_cast<int64_t>(evaluation_points.size());
  const int log_domain_size = parameters_[hierarchy_level].log_domain_size();
  absl::uint128 max_evaluation_point = absl::Uint128Max();
  if (log_domain_size < 128) {
    max_evaluation_point = (absl::uint128{1} << log_domain_size) - 1;
  }
  // Check if `evaluation_points` are inside the domain. This has minimal (~ 1%)
  // performance impact.
  for (int64_t i = 0; i < num_evaluation_points; ++i) {
    if (evaluation_points[i] > max_evaluation_point) {
      return absl::InvalidArgumentError(
          absl::StrCat("`evaluation_points[", i,
                       "]` larger than the domain size at hierarchy level ",
                       hierarchy_level));
    }
  }
  absl::Status status = proto_validator_->ValidateDpfKey(key);
  if (!status.ok()) {
    return status;
  }
  if (num_evaluation_points == 0) {
    return std::vector<T>{};  // Nothing to do.
  }

  // Split up evaluation_points into tree indices and block indices, if we're
  // operating on a packed type. Otherwise set `tree_indices` to
  // `evaluation_points`.
  hwy::AlignedFreeUniquePtr<absl::uint128[]> maybe_recomputed_tree_indices;
  constexpr int elements_per_block = dpf_internal::ElementsPerBlock<T>();
  absl::Span<const absl::uint128> tree_indices;
  if (elements_per_block > 1) {
    maybe_recomputed_tree_indices =
        hwy::AllocateAligned<absl::uint128>(num_evaluation_points);
    if (maybe_recomputed_tree_indices == nullptr) {
      return absl::ResourceExhaustedError("Memory allocation error");
    }
    for (int64_t i = 0; i < num_evaluation_points; ++i) {
      maybe_recomputed_tree_indices[i] =
          DomainToTreeIndex(evaluation_points[i], hierarchy_level);
    }
    tree_indices = absl::MakeConstSpan(maybe_recomputed_tree_indices.get(),
                                       num_evaluation_points);
    // Copy evaluation_points to new array if not aligned.
  } else {
    // This avoids copying the evaluation points when elements_per_block == 1.
    tree_indices = evaluation_points;
  }

  // Set up partial evaluations for the selected tree_indices. If we have a
  // context `ctx`, Compute them from `ctx.partial_evaluations`, otherwise start
  // from the beginning.
  absl::StatusOr<DpfExpansion> selected_partial_evaluations = DpfExpansion();
  int start_level = 0;
  if (!ctx) {
    // No context or context was never evaluated -> start from the beginning.
    absl::uint128 seed = absl::MakeUint128(key.seed().high(), key.seed().low());
    bool party = key.party();
    selected_partial_evaluations->seeds =
        hwy::AllocateAligned<absl::uint128>(num_evaluation_points);
    if (selected_partial_evaluations->seeds == nullptr) {
      return absl::ResourceExhaustedError("Memory allocation error");
    }
    auto seeds = absl::MakeSpan(selected_partial_evaluations->seeds.get(),
                                num_evaluation_points);
    std::fill(seeds.begin(), seeds.end(), seed);
    selected_partial_evaluations->control_bits.resize(num_evaluation_points,
                                                      party);
  } else {
    // We have a context -> Use it to compute partial evaluations. Always update
    // `ctx`, since unlike for full expansion the amount of proto data written
    // will always be `tree_indices.size()` and should therefore be negligible.
    selected_partial_evaluations =
        ComputePartialEvaluations(tree_indices, hierarchy_level,
                                  /*update_ctx=*/true, *ctx);
    if (!selected_partial_evaluations.ok()) {
      return selected_partial_evaluations.status();
    }
    start_level = hierarchy_to_tree_[hierarchy_level];
  }

  // Evaluate DPFs.
  const int stop_level = hierarchy_to_tree_[hierarchy_level];
  absl::Span<absl::uint128> seeds(
      selected_partial_evaluations->seeds.get(),
      selected_partial_evaluations->control_bits.size());
  auto correction_words = absl::MakeConstSpan(key.correction_words())
                              .subspan(start_level, stop_level - start_level);
  status =
      EvaluateSeeds(seeds, selected_partial_evaluations->control_bits,
                    tree_indices, correction_words, seeds,
                    absl::MakeSpan(selected_partial_evaluations->control_bits));
  if (!status.ok()) {
    return status;
  }
  ABSL_DCHECK(static_cast<int64_t>(seeds.size()) == num_evaluation_points);

  // Hash `seeds`.
  absl::StatusOr<hwy::AlignedFreeUniquePtr<absl::uint128[]>> hashed_expansion =
      HashExpandedSeeds(hierarchy_level, seeds);
  if (!hashed_expansion.ok()) {
    return hashed_expansion.status();
  }

  // Get value correction words.
  absl::StatusOr<std::array<T, elements_per_block>> correction_ints =
      GetValueCorrectionAsArray<T>(key, hierarchy_level);
  if (!correction_ints.ok()) {
    return correction_ints.status();
  }

  // Perform value correction.
  std::vector<T> result(num_evaluation_points);
  const int blocks_needed = blocks_needed_[hierarchy_level];
  for (int64_t i = 0; i < num_evaluation_points; ++i) {
    std::array<T, elements_per_block> current_elements =
        dpf_internal::ConvertBytesToArrayOf<T>(absl::string_view(
            reinterpret_cast<const char*>(hashed_expansion->get() +
                                          i * blocks_needed),
            blocks_needed * sizeof(absl::uint128)));
    int block_index = 0;
    if (elements_per_block > 1) {
      block_index = DomainToBlockIndex(evaluation_points[i], hierarchy_level);
    }
    result[i] = current_elements[block_index];
    if (selected_partial_evaluations->control_bits[i]) {
      result[i] += (*correction_ints)[block_index];
    }
    if (key.party() == 1) {
      result[i] = -result[i];
    }
  }

  if (ctx) {
    ctx->set_previous_hierarchy_level(hierarchy_level);
  }

  return result;
}

template <typename T, typename Fn>
absl::Status DistributedPointFunction::EvaluateAndApply(
    dpf_internal::MaybeDerefSpan<const DpfKey> keys,
    absl::Span<const absl::uint128> evaluation_points, Fn op,
    int evaluation_points_rightshift) const {
  if (evaluation_points.size() != keys.size()) {
    return absl::InvalidArgumentError(
        "`keys.size()` != `evaluation_points.size()`");
  }
  for (size_t i = 0; i < keys.size(); ++i) {
    absl::Status status = proto_validator_->ValidateDpfKey(keys[i]);
    if (!status.ok()) return status;
  }

  const int64_t num_keys = keys.size();
  const int num_hierarchy_levels = parameters_.size();
  DpfExpansion eval;
  eval.control_bits.resize(num_keys);
  eval.seeds = hwy::AllocateAligned<absl::uint128>(num_keys);
  if (eval.seeds == nullptr) {
    return absl::ResourceExhaustedError("Memory allocation error");
  }
  absl::Span<absl::uint128> seeds(eval.seeds.get(), num_keys);
  absl::Span<bool> control_bits(eval.control_bits);
  hwy::AlignedFreeUniquePtr<absl::uint128[]> correction_seeds;
  BitVector correction_control_bits_left, correction_control_bits_right;
  std::vector<T> values(num_keys);

  // Initialize seeds and control bits.
  for (int64_t i = 0; i < num_keys; ++i) {
    seeds[i] = absl::MakeUint128(keys[i].seed().high(), keys[i].seed().low());
    control_bits[i] = keys[i].party();
  }

  int start_level = 0;
  int stop_level = hierarchy_to_tree_[0];
  for (int hierarchy_level = 0; hierarchy_level < num_hierarchy_levels;
       ++hierarchy_level) {
    if (hierarchy_level > 0) {
      start_level = stop_level;
      stop_level = hierarchy_to_tree_[hierarchy_level];
    }

    // Compute index shifts for the current level.
    const int domain_index_rightshift =
        evaluation_points_rightshift + parameters_.back().log_domain_size() -
        parameters_[hierarchy_level].log_domain_size();
    const int tree_index_rightshift = evaluation_points_rightshift +
                                      parameters_.back().log_domain_size() -
                                      hierarchy_to_tree_[hierarchy_level];

    int num_tree_levels = stop_level - start_level;
    if (num_tree_levels > 0) {
      correction_seeds =
          hwy::AllocateAligned<absl::uint128>(num_tree_levels * num_keys);
      if (correction_seeds == nullptr) {
        return absl::ResourceExhaustedError("Memory allocation error");
      }
      correction_control_bits_left.resize(num_tree_levels * num_keys);
      correction_control_bits_right.resize(num_tree_levels * num_keys);
      for (int i = 0; i < num_tree_levels; ++i) {
        for (int64_t j = 0; j < num_keys; ++j) {
          const int64_t index = i * num_keys + j;
          const CorrectionWord& cw = keys[j].correction_words(start_level + i);
          correction_seeds[index] =
              absl::MakeUint128(cw.seed().high(), cw.seed().low());
          correction_control_bits_left[index] = cw.control_left();
          correction_control_bits_right[index] = cw.control_right();
        }
      }

      // Evaluate the current hierarchy level for all keys.
      absl::Status status = dpf_internal::EvaluateSeeds(
          seeds.size(), num_tree_levels, num_tree_levels * num_keys,
          seeds.data(), control_bits.data(), evaluation_points.data(),
          tree_index_rightshift, correction_seeds.get(),
          correction_control_bits_left.data(),
          correction_control_bits_right.data(), prg_left_, prg_right_,
          seeds.data(), control_bits.data());
      if (!status.ok()) {
        return status;
      }
    }

    // Hash `seeds`.
    absl::StatusOr<hwy::AlignedFreeUniquePtr<absl::uint128[]>>
        hashed_expansion = HashExpandedSeeds(hierarchy_level, seeds);
    if (!hashed_expansion.ok()) {
      return hashed_expansion.status();
    }

    // Compute value correction for the current level.
    constexpr int elements_per_block = dpf_internal::ElementsPerBlock<T>();
    const int blocks_needed = blocks_needed_[hierarchy_level];
    for (int64_t i = 0; i < num_keys; ++i) {
      std::array<T, elements_per_block> current_elements =
          dpf_internal::ConvertBytesToArrayOf<T>(absl::string_view(
              reinterpret_cast<const char*>(hashed_expansion->get() +
                                            i * blocks_needed),
              blocks_needed * sizeof(absl::uint128)));
      absl::StatusOr<std::array<T, elements_per_block>> correction_ints =
          GetValueCorrectionAsArray<T>(keys[i], hierarchy_level);
      if (!correction_ints.ok()) {
        return correction_ints.status();
      }
      int block_index = 0;
      if (elements_per_block > 1 && domain_index_rightshift < 128) {
        block_index = DomainToBlockIndex(
            evaluation_points[i] >> domain_index_rightshift, hierarchy_level);
      }
      values[i] = current_elements[block_index];
      if (control_bits[i]) {
        values[i] += (*correction_ints)[block_index];
      }
      if (keys[i].party() == 1) {
        values[i] = -values[i];
      }
    }

    // Call the callback with the values at the current level, and return if the
    // result is `false`.
    if (!op(values)) {
      break;
    }
  }
  return absl::OkStatus();
}

}  // namespace distributed_point_functions

#endif  // DISTRIBUTED_POINT_FUNCTIONS_DPF_DISTRIBUTED_POINT_FUNCTION_H_
