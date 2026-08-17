/*
 * Copyright 2020 Google LLC.
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     https://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#ifndef RLWE_CONTEXT_H_
#define RLWE_CONTEXT_H_

#include <memory>

#include "absl/memory/memory.h"
#include "error_params.h"
#include "ntt_parameters.h"
#include "status_macros.h"
#include "statusor.h"

namespace rlwe {

// Defines and holds the context of the RLWE encryption scheme.
// Thread safe..
template <typename ModularInt>
class RlweContext {
  using Int = typename ModularInt::Int;
  using ModulusParams = typename ModularInt::Params;

 public:
  // Structure to hold parameters for the RLWE encryption scheme. The parameters
  // include:
  // - modulus, an Int which needs to be congruent to 1 modulo 2 * (1 << log_n);
  // - log_n: the logarithm of the number of coefficients of the polynomials;
  // - log_t: the number of bits of the plaintext space, which will be equal to
  //          (1 << log_t) + 1;
  // - variance, the error variance to use when sampling noises or secrets.
  struct Parameters {
    Int modulus;
    size_t log_n;
    size_t log_t;
    size_t variance;
  };

  // Factory function to create a context from a context_params.
  static rlwe::StatusOr<std::unique_ptr<const RlweContext>> Create(
      Parameters context_params) {
    // Create the modulus parameters.
    RLWE_ASSIGN_OR_RETURN(
        std::unique_ptr<const ModulusParams> modulus_parameters,
        ModulusParams::Create(context_params.modulus));
    // Create the NTT parameters.
    RLWE_ASSIGN_OR_RETURN(NttParameters<ModularInt> ntt_params_temp,
                          InitializeNttParameters<ModularInt>(
                              context_params.log_n, modulus_parameters.get()));
    std::unique_ptr<const NttParameters<ModularInt>> ntt_params =
        std::make_unique<const NttParameters<ModularInt>>(
            std::move(ntt_params_temp));
    // Create the error parameters.
    RLWE_ASSIGN_OR_RETURN(ErrorParams<ModularInt> error_params_temp,
                          ErrorParams<ModularInt>::Create(
                              context_params.log_t, context_params.variance,
                              modulus_parameters.get(), ntt_params.get()));
    std::unique_ptr<const ErrorParams<ModularInt>> error_params =
        std::make_unique<const ErrorParams<ModularInt>>(
            std::move(error_params_temp));
    return absl::WrapUnique<const RlweContext>(
        new RlweContext(std::move(modulus_parameters), std::move(ntt_params),
                        std::move(error_params), std::move(context_params)));
  }

  // Disallow copy and copy-assign, allow move and move-assign.
  RlweContext(const RlweContext&) = delete;
  RlweContext& operator=(const RlweContext&) = delete;
  RlweContext(RlweContext&&) = default;
  RlweContext& operator=(RlweContext&&) = default;
  ~RlweContext() = default;

  // Getters.
  const ModulusParams* GetModulusParams() const {
    return modulus_parameters_.get();
  }
  const NttParameters<ModularInt>* GetNttParams() const {
    return ntt_parameters_.get();
  }
  const ErrorParams<ModularInt>* GetErrorParams() const {
    return error_parameters_.get();
  }
  const Int GetModulus() const { return context_params_.modulus; }
  const size_t GetLogN() const { return context_params_.log_n; }
  const size_t GetN() const { return 1 << context_params_.log_n; }
  const size_t GetLogT() const { return context_params_.log_t; }
  const Int GetT() const {
    return (static_cast<Int>(1) << context_params_.log_t) + static_cast<Int>(1);
  }
  const size_t GetVariance() const { return context_params_.variance; }

 private:
  RlweContext(std::unique_ptr<const ModulusParams> modulus_parameters,
              std::unique_ptr<const NttParameters<ModularInt>> ntt_parameters,
              std::unique_ptr<const ErrorParams<ModularInt>> error_parameters,
              Parameters context_params)
      : modulus_parameters_(std::move(modulus_parameters)),
        ntt_parameters_(std::move(ntt_parameters)),
        error_parameters_(std::move(error_parameters)),
        context_params_(std::move(context_params)) {}

  const std::unique_ptr<const ModulusParams> modulus_parameters_;
  const std::unique_ptr<const NttParameters<ModularInt>> ntt_parameters_;
  const std::unique_ptr<const ErrorParams<ModularInt>> error_parameters_;
  const Parameters context_params_;
};

}  // namespace rlwe

#endif  // RLWE_CONTEXT_H_
