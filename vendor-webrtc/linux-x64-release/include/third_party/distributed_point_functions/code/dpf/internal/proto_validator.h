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

#ifndef DISTRIBUTED_POINT_FUNCTIONS_DPF_INTERNAL_PROTO_VALIDATOR_H_
#define DISTRIBUTED_POINT_FUNCTIONS_DPF_INTERNAL_PROTO_VALIDATOR_H_

#include <memory>
#include <vector>

#include "absl/container/flat_hash_map.h"
#include "absl/status/status.h"
#include "absl/status/statusor.h"
#include "absl/types/span.h"
#include "dpf/distributed_point_function.pb.h"

namespace distributed_point_functions {
namespace dpf_internal {
// ProtoValidator is used to validate protos for DPF parameters, keys, and
// evaluation contexts. Also holds information computed from the DPF parameters,
// such as the mappings between hierarchy and tree levels.
class ProtoValidator {
 public:
  // The negative logarithm of the total variation distance from uniform that a
  // *full* evaluation of a hierarchy level is allowed to have. Used as the
  // default value for DpfParameters that don't have an explicit per-element
  // security parameter set.
  static constexpr double kDefaultSecurityParameter = 40;

  // Security parameters that differ by less than this are considered equal.
  static constexpr double kSecurityParameterEpsilon = 0.0001;

  // Checks the validity of `parameters` and returns a ProtoValidator, which
  // will be used to validate DPF keys and evaluation contexts afterwards.
  //
  // Returns INVALID_ARGUMENT if `parameters` are invalid.
  static absl::StatusOr<std::unique_ptr<ProtoValidator>> Create(
      absl::Span<const DpfParameters> parameters);

  // Checks the validity of `parameters`.
  // Returns OK on success, and INVALID_ARGUMENT otherwise.
  static absl::Status ValidateParameters(
      absl::Span<const DpfParameters> parameters);

  // Checks that `key` is valid for the `parameters` passed at construction.
  // Returns OK on success, and INVALID_ARGUMENT otherwise.
  absl::Status ValidateDpfKey(const DpfKey& key) const;

  // Checks that `ctx` is valid for the `parameters` passed at construction.
  // Returns OK on success, and INVALID_ARGUMENT otherwise.
  absl::Status ValidateEvaluationContext(const EvaluationContext& ctx) const;

  // Checks that the given ValueType is valid.
  // Returns OK on success and INVALID_ARGUMENT otherwise.
  static absl::Status ValidateValueType(const ValueType& value_type);

  // Checks that `value` is valid for `type`.
  // Returns OK on success and INVALID_ARGUMENT otherwise.
  static absl::Status ValidateValue(const Value& value, const ValueType& type);

  // Checks that `value` is valid for `parameters[i]` passed at construction.
  // Returns OK on success and INVALID_ARGUMENT otherwise.
  inline absl::Status ValidateValue(const Value& value, int i) const {
    return ValidateValue(value, parameters_[i].value_type());
  }

  // ProtoValidator is not copyable.
  ProtoValidator(const ProtoValidator&) = delete;
  ProtoValidator& operator=(const ProtoValidator&) = delete;

  // ProtoValidator is movable.
  ProtoValidator(ProtoValidator&&) = default;
  ProtoValidator& operator=(ProtoValidator&&) = default;

  // Getters.
  absl::Span<const DpfParameters> parameters() const { return parameters_; }
  int tree_levels_needed() const { return tree_levels_needed_; }
  const absl::flat_hash_map<int, int>& tree_to_hierarchy() const {
    return tree_to_hierarchy_;
  }
  const std::vector<int>& hierarchy_to_tree() const {
    return hierarchy_to_tree_;
  }

 private:
  ProtoValidator(std::vector<DpfParameters> parameters, int tree_levels_needed,
                 absl::flat_hash_map<int, int> tree_to_hierarchy,
                 std::vector<int> hierarchy_to_tree);

  // The DpfParameters passed at construction.
  std::vector<DpfParameters> parameters_;

  // Number of levels in the evaluation tree. This is always less than or equal
  // to the largest log_domain_size in parameters_.
  int tree_levels_needed_;

  // Maps levels of the FSS evaluation tree to hierarchy levels (i.e., elements
  // of parameters_).
  absl::flat_hash_map<int, int> tree_to_hierarchy_;

  // The inverse of tree_to_hierarchy_.
  std::vector<int> hierarchy_to_tree_;
};

}  // namespace dpf_internal
}  // namespace distributed_point_functions

#endif  // DISTRIBUTED_POINT_FUNCTIONS_DPF_INTERNAL_PROTO_VALIDATOR_H_
