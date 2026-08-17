// Copyright 2025 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#ifndef FUZZTEST_FUZZTEST_INTERNAL_DOMAINS_BIT_GEN_REF_H_
#define FUZZTEST_FUZZTEST_INTERNAL_DOMAINS_BIT_GEN_REF_H_

#include <cstdint>
#include <optional>
#include <utility>
#include <vector>

#include "absl/log/absl_check.h"
#include "absl/log/absl_log.h"
#include "absl/random/bit_gen_ref.h"
#include "absl/status/status.h"
#include "absl/strings/str_format.h"
#include "./fuzztest/internal/domains/domain_base.h"
#include "./fuzztest/internal/fuzzing_bit_gen.h"
#include "./fuzztest/internal/printer.h"
#include "./fuzztest/internal/serialization.h"

namespace fuzztest::internal {

// The fuzztest "corpus value" holds the fuzzed data stream and a
// maybe-initialized instance of a fuzz-test specific URBG which is bound to
// an absl::BitGenRef in BitGenRefDomain::GetValue.
//
// The URBG instance is lazily initialized when GetBitGen is called and
// destroyed when CleanupBitGen is called.
class BitGenCorpusValue {
 public:
  using InitializerData = std::vector<uint8_t>;
  using URBG = FuzzingBitGen;

  explicit BitGenCorpusValue(InitializerData data)
      : initializer_data_(std::move(data)) {}
  ~BitGenCorpusValue() { CleanupBitGen(); }

  // Copy and move do not initialize the internal URBG instance.
  BitGenCorpusValue(const BitGenCorpusValue& o)
      : initializer_data_(o.initializer_data_), bitgen_(std::nullopt) {}
  BitGenCorpusValue& operator=(const BitGenCorpusValue& o) {
    // The internal URBG should be unused.
    ABSL_CHECK(!bitgen_.has_value());
    initializer_data_ = o.initializer_data_;
    return *this;
  }
  BitGenCorpusValue(BitGenCorpusValue&& o)
      : initializer_data_(std::move(o.initializer_data_)),
        bitgen_(std::nullopt) {}
  BitGenCorpusValue& operator=(BitGenCorpusValue&& o) {
    // The internal URBG should be unused.
    ABSL_CHECK(!o.bitgen_.has_value());
    ABSL_CHECK(!bitgen_.has_value());
    initializer_data_ = std::move(o.initializer_data_);
    return *this;
  }

  InitializerData& initializer_data() { return initializer_data_; }
  const InitializerData& initializer_data() const { return initializer_data_; }

  // Cleanup the internal URBG instance.
  void CleanupBitGen() { bitgen_.reset(); }

  // Returns a reference to the URBG instance.
  // If it has not been initialized, it will be initialized.
  // NOTE: The returned reference is valid until the next call to CleanupBitGen.
  URBG& GetBitGen() const {
    if (!bitgen_.has_value()) {
      bitgen_.emplace(initializer_data_);
    }
    return *bitgen_;
  }

 private:
  // Underlying fuzzed data stream; the input to the URBG constructor.
  // When using util_random::FuzzingBitGen, this is a vector of uint8_t which
  // defines the sequence of random variates.
  std::vector<uint8_t> initializer_data_;
  mutable std::optional<URBG> bitgen_;
};

// A FuzzTest domain for an absl::BitGenRef, which is an arbitrary
// uniform random bit generator and can be used for functions accepting an
// absl::BitGenRef, such as absl::Uniform and other Abseil distribution
// functions. The generated sequences will be stable across executions, though
// it may occasionally be broken when there are changes to the underlying
// implementation such as adding support for new distributions, etc.
//
// The domain accepts an input "data stream" corpus which is used to initialize
// a FuzzingBitGen instance. This internal FuzzingBitGen instance is bound to an
// absl::BitGenRef when GetValue is called.
//
// BitGenRefDomain does not support seeded domains.
// BitGenRefDomain does not support GetRandomValue.
template <typename Inner>
class BitGenRefDomain
    : public domain_implementor::DomainBase<BitGenRefDomain<Inner>,
                                            /*value_type=*/absl::BitGenRef,
                                            /*corpus_type=*/BitGenCorpusValue> {
 public:
  using typename BitGenRefDomain::DomainBase::corpus_type;
  using typename BitGenRefDomain::DomainBase::value_type;

  explicit BitGenRefDomain(const Inner& inner) : inner_(inner) {}
  explicit BitGenRefDomain(Inner&& inner) : inner_(std::move(inner)) {}

  BitGenRefDomain(const BitGenRefDomain&) = default;
  BitGenRefDomain(BitGenRefDomain&&) = default;
  BitGenRefDomain& operator=(const BitGenRefDomain&) = default;
  BitGenRefDomain& operator=(BitGenRefDomain&&) = default;

  corpus_type Init(absl::BitGenRef prng) {
    return corpus_type{inner_.Init(prng)};
  }
  void Mutate(corpus_type& corpus_value, absl::BitGenRef prng,
              const domain_implementor::MutationMetadata& metadata,
              bool only_shrink) {
    corpus_value.CleanupBitGen();
    inner_.Mutate(corpus_value.initializer_data(), prng, metadata, only_shrink);
  }

  absl::BitGenRef GetValue(const corpus_type& corpus_value) const {
    return absl::BitGenRef(corpus_value.GetBitGen());
  }

  value_type GetRandomValue(absl::BitGenRef prng) {
    // See b/404828355
    ABSL_LOG(FATAL) << "The domain doesn't support GetRandomValue().";
  }

  std::optional<corpus_type> FromValue(const value_type&) const {
    // No conversion from absl::BitGenRef back to corpus.
    return std::nullopt;
  }
  absl::Status ValidateCorpusValue(const corpus_type& corpus_value) const {
    return inner_.ValidateCorpusValue(corpus_value.initializer_data());
  }
  void UpdateMemoryDictionary(
      const corpus_type& corpus_value,
      domain_implementor::ConstCmpTablesPtr cmp_tables) {
    return inner_.UpdateMemoryDictionary(corpus_value.initializer_data(),
                                         cmp_tables);
  }
  std::optional<corpus_type> ParseCorpus(const internal::IRObject& obj) const {
    auto x = inner_.ParseCorpus(obj);
    if (x.has_value()) {
      return corpus_type(*std::move(x));
    }
    return std::nullopt;
  }
  internal::IRObject SerializeCorpus(const corpus_type& corpus_value) const {
    return inner_.SerializeCorpus(corpus_value.initializer_data());
  }

  auto GetPrinter() const { return Printer{}; }

 private:
  struct Printer {
    void PrintCorpusValue(const corpus_type& val,
                          domain_implementor::RawSink out,
                          domain_implementor::PrintMode mode) const {
      absl::Format(out, "absl::BitGenRef{}");
    }
  };

  Inner inner_;
};

}  // namespace fuzztest::internal

#endif  // FUZZTEST_FUZZTEST_INTERNAL_DOMAINS_BIT_GEN_REF_H_
