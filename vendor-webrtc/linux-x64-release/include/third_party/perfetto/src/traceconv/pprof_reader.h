/*
 * Copyright (C) 2024 The Android Open Source Project
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

#ifndef SRC_TRACECONV_PPROF_READER_H_
#define SRC_TRACECONV_PPROF_READER_H_

#include <cstdint>

#include "protos/third_party/pprof/profile.gen.h"

namespace perfetto {
namespace pprof {

class PprofProfileReader {
 public:
  explicit PprofProfileReader(const std::string& path);

  uint64_t get_sample_count() const;

  std::string get_string_by_index(uint64_t string_index) const;

  int64_t get_string_index(const std::string& str) const;

  uint64_t find_location_id(const std::string& function_name) const;

  std::vector<std::string> get_sample_function_names(
      const third_party::perftools::profiles::gen::Sample& sample) const;

  // Finds all samples from the profile where its location equals to the passed
  // function name and returns them. It looks for the last (the most specific)
  // function name to be equal to last_function_name
  std::vector<third_party::perftools::profiles::gen::Sample> get_samples(
      const std::string& last_function_name) const;

  third_party::perftools::profiles::gen::Location find_location(
      const uint64_t location_id) const;

  third_party::perftools::profiles::gen::Function find_function(
      const uint64_t function_id) const;

  uint64_t get_sample_value_index(const std::string& value_name) const;

  int64_t get_samples_value_sum(const std::string& last_function_name,
                                const std::string& value_name) const;

 private:
  third_party::perftools::profiles::gen::Profile profile_;
};

}  // namespace pprof
}  // namespace perfetto

#endif  // SRC_TRACECONV_PPROF_READER_H_
