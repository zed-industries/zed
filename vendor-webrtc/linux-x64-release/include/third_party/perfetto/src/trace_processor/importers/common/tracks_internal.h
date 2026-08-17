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

#ifndef SRC_TRACE_PROCESSOR_IMPORTERS_COMMON_TRACKS_INTERNAL_H_
#define SRC_TRACE_PROCESSOR_IMPORTERS_COMMON_TRACKS_INTERNAL_H_

#include <array>
#include <cstddef>
#include <cstdint>
#include <string_view>
#include <tuple>

#include "perfetto/ext/base/hash.h"
#include "src/trace_processor/containers/string_pool.h"

namespace perfetto::trace_processor::tracks {

template <typename... T>
using DimensionsT = std::tuple<T...>;

struct DimensionBlueprintBase {
  std::string_view name;
};

template <typename T>
struct DimensionBlueprintT : DimensionBlueprintBase {
  using type = T;
};

struct NameBlueprintT {
  struct Auto {
    using name_t = std::nullptr_t;
  };
  struct Static {
    using name_t = std::nullptr_t;
    const char* name;
  };
  struct Dynamic {
    using name_t = StringPool::Id;
  };
  struct FnBase {
    using name_t = std::nullptr_t;
  };
  template <typename F>
  struct Fn : FnBase {
    F fn;
  };
};

struct BlueprintBase {
  std::string_view event_type;
  std::string_view type;
  base::Hasher hasher;
  std::array<DimensionBlueprintBase, 8> dimension_blueprints;
};

template <typename NB, typename UB, typename... DB>
struct BlueprintT : BlueprintBase {
  using name_blueprint_t = NB;
  using unit_blueprint_t = UB;
  using name_t = typename NB::name_t;
  using unit_t = typename UB::unit_t;
  using dimension_blueprints_t = std::tuple<DB...>;
  using dimensions_t = DimensionsT<typename DB::type...>;
  name_blueprint_t name_blueprint;
  unit_blueprint_t unit_blueprint;
};

template <typename... T>
using DimensionBlueprintsT = std::tuple<T...>;

struct UnitBlueprintT {
  struct Unknown {
    using unit_t = std::nullptr_t;
  };
  struct Static {
    using unit_t = const char*;
    const char* name;
  };
  struct Dynamic {
    using unit_t = StringPool::Id;
  };
};

template <typename BlueprintT, typename Dims>
constexpr uint64_t HashFromBlueprintAndDimensions(const BlueprintT& bp,
                                                  const Dims& dims) {
  base::Hasher hasher(bp.hasher);
  std::apply([&](auto&&... args) { ((hasher.Update(args)), ...); }, dims);
  return hasher.digest();
}

}  // namespace perfetto::trace_processor::tracks

#endif  // SRC_TRACE_PROCESSOR_IMPORTERS_COMMON_TRACKS_INTERNAL_H_
