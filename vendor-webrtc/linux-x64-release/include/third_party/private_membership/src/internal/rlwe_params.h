// Copyright 2020 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//    https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef THIRD_PARTY_PRIVATE_MEMBERSHIP_SRC_INTERNAL_RLWE_PARAMS_H_
#define THIRD_PARTY_PRIVATE_MEMBERSHIP_SRC_INTERNAL_RLWE_PARAMS_H_

#include "third_party/private_membership/src/private_membership_rlwe.pb.h"
#include "third_party/shell-encryption/src/context.h"
#include "third_party/shell-encryption/src/error_params.h"
#include "third_party/shell-encryption/src/montgomery.h"
#include "third_party/shell-encryption/src/ntt_parameters.h"
#include "third_party/shell-encryption/src/statusor.h"

namespace private_membership {
namespace rlwe {

template <typename ModularInt>
::rlwe::StatusOr<
    std::vector<std::unique_ptr<const ::rlwe::RlweContext<ModularInt>>>>
CreateContexts(const RlweParameters& rlwe_params);

template <typename ModularInt>
::rlwe::StatusOr<std::unique_ptr<const typename ModularInt::Params>>
CreateModulusParams(const Uint128& modulus);

template <typename ModularInt>
::rlwe::StatusOr<std::unique_ptr<const ::rlwe::NttParameters<ModularInt>>>
CreateNttParams(const RlweParameters& rlwe_params,
                const typename ModularInt::Params* modulus_params);

template <typename ModularInt>
::rlwe::StatusOr<std::unique_ptr<const ::rlwe::ErrorParams<ModularInt>>>
CreateErrorParams(const RlweParameters& rlwe_params,
                  const typename ModularInt::Params* modulus_params,
                  const ::rlwe::NttParameters<ModularInt>* ntt_params);

}  // namespace rlwe
}  // namespace private_membership

#endif  // THIRD_PARTY_PRIVATE_MEMBERSHIP_SRC_INTERNAL_RLWE_PARAMS_H_
