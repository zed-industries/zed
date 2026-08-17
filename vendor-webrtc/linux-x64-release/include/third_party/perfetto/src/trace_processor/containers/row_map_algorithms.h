/*
 * Copyright (C) 2022 The Android Open Source Project
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

#ifndef SRC_TRACE_PROCESSOR_CONTAINERS_ROW_MAP_ALGORITHMS_H_
#define SRC_TRACE_PROCESSOR_CONTAINERS_ROW_MAP_ALGORITHMS_H_

#include <cstdint>
#include <vector>

#include "perfetto/base/logging.h"
#include "src/trace_processor/containers/bit_vector.h"

// This file contains fundamental algorithms used by RowMap.
//
// This file is structured in a way to make benchmarking easy. The intention is
// to use this to decide which heurustics to use and the value of magic
// constants in RowMap algorithms.

namespace perfetto {
namespace trace_processor {
namespace row_map_algorithms {

// Returns a vector containing elements from |iv| selected by indices from
// |selector|.
inline std::vector<uint32_t> SelectIvWithIv(
    const std::vector<uint32_t>& iv,
    const std::vector<uint32_t>& selector) {
  std::vector<uint32_t> ret(selector.size());
  for (uint32_t i = 0; i < selector.size(); ++i) {
    PERFETTO_DCHECK(selector[i] < iv.size());
    ret[i] = iv[selector[i]];
  }
  return ret;
}

// Returns a vector containing elements from |bv| by first converting to an
// index vector and then selecting indices from |selector|.
inline std::vector<uint32_t> SelectBvWithIvByConvertToIv(
    const BitVector& bv,
    const std::vector<uint32_t>& selector) {
  return SelectIvWithIv(bv.GetSetBitIndices(), selector);
}

// Returns a vector containing elements from |bv| by selecting indices from
// |selector| using IndexOfNthSet calls.
inline std::vector<uint32_t> SelectBvWithIvByIndexOfNthSet(
    const BitVector& bv,
    const std::vector<uint32_t>& selector) {
  std::vector<uint32_t> iv(selector.size());
  for (uint32_t i = 0; i < selector.size(); ++i) {
    iv[i] = bv.IndexOfNthSet(selector[i]);
  }
  return iv;
}

}  // namespace row_map_algorithms
}  // namespace trace_processor
}  // namespace perfetto

#endif  // SRC_TRACE_PROCESSOR_CONTAINERS_ROW_MAP_ALGORITHMS_H_
