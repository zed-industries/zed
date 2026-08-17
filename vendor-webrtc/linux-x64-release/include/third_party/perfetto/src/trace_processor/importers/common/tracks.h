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

#ifndef SRC_TRACE_PROCESSOR_IMPORTERS_COMMON_TRACKS_H_
#define SRC_TRACE_PROCESSOR_IMPORTERS_COMMON_TRACKS_H_

#include <array>
#include <cstddef>
#include <cstdint>
#include <tuple>

#include "perfetto/ext/base/hash.h"
#include "perfetto/ext/base/string_view.h"
#include "src/trace_processor/containers/string_pool.h"
#include "src/trace_processor/importers/common/tracks_internal.h"

namespace perfetto::trace_processor::tracks {

// This file contains the "public API" for creating track blueprints.
// See TrackTracker::InternTrack for usages of the functions in this file.

// Start of blueprint functions.

// Creates a blueprint for a slice track.
// See TrackTracker::InternTrack for usage of this function.
template <typename NB = NameBlueprintT::Auto, typename... D>
constexpr auto SliceBlueprint(const char type[],
                              DimensionBlueprintsT<D...> dimensions = {},
                              NB name = NB{}) {
  static_assert(sizeof...(D) < 8, "At most 8 dimensions are supported");
  auto dims_array = std::apply(
      [](auto&&... x) { return std::array<DimensionBlueprintBase, 8>{x...}; },
      dimensions);
  return BlueprintT<NB, UnitBlueprintT::Unknown, D...>{
      {
          "slice",
          type,
          base::Hasher::CreatePartial(type),
          dims_array,
      },
      name,
      UnitBlueprintT::Unknown{},
  };
}

// Creates a blueprint for a counter track.
// See TrackTracker::InternTrack for usage on this function.
template <typename NB = NameBlueprintT::Auto,
          typename UB = UnitBlueprintT::Unknown,
          typename... D>
constexpr auto CounterBlueprint(const char type[],
                                UB unit,
                                DimensionBlueprintsT<D...> dimensions = {},
                                NB name = NB{}) {
  static_assert(sizeof...(D) < 8, "At most 8 dimensions are supported");
  auto dims_array = std::apply(
      [](auto&&... x) { return std::array<DimensionBlueprintBase, 8>{x...}; },
      dimensions);
  return BlueprintT<NB, UB, D...>{
      {
          "counter",
          type,
          base::Hasher::CreatePartial(type),
          dims_array,
      },
      name,
      unit,
  };
}

// Wraps all the dimension blueprints before passing them to SliceBlueprint()
// or CounterBlueprint().
template <typename... DimensionBlueprint>
constexpr auto DimensionBlueprints(DimensionBlueprint... dimensions) {
  return DimensionBlueprintsT<DimensionBlueprint...>{dimensions...};
}

// Adds a unit32_t dimension with the given name.
constexpr auto UintDimensionBlueprint(const char name[]) {
  return DimensionBlueprintT<uint32_t>{{name}};
}

// Adds a string dimension with the given name.
constexpr auto StringDimensionBlueprint(const char name[]) {
  return DimensionBlueprintT<base::StringView>{{name}};
}

// Adds a int64_t dimension with the given name.
constexpr auto LongDimensionBlueprint(const char name[]) {
  return DimensionBlueprintT<int64_t>{{name}};
}

// Indicates the name should be automatically determined by trace processor.
constexpr auto AutoNameBlueprint() {
  return NameBlueprintT::Auto{};
}

// Indicates the name of the track should be given by a static string. This
// should really only be used when the track has no dimensions as it's quite
// confusing in queries otherwise.
constexpr auto StaticNameBlueprint(const char name[]) {
  return NameBlueprintT::Static{name};
}

// Indicates the name of the track is dynamic and will be provided at runtime to
// InternTrack.
constexpr auto DynamicNameBlueprint() {
  return NameBlueprintT::Dynamic{};
}

// Indicates the name of the track is a function which accepts as input the
// dimensions of the track and returns a base::StackString containing the
// results of transforming the dimensions.
template <typename F>
constexpr auto FnNameBlueprint(F fn) {
  return NameBlueprintT::Fn<F>{{}, fn};
}

// Indicates that the unit of this track is given by a static string.
constexpr auto StaticUnitBlueprint(const char unit[]) {
  return UnitBlueprintT::Static{unit};
}

// Indicates the unit of this track is dynamic and will be provided at
// InternTrack time.
constexpr auto DynamicUnitBlueprint() {
  return UnitBlueprintT::Dynamic{};
}

// Indicates that the units of the counter are unknown. Should not be used, is
// only intended for counter tracks which predate the introduction of track
// blueprints.
constexpr auto UnknownUnitBlueprint() {
  return UnitBlueprintT::Unknown{};
}

// End of blueprint functions.

// Start of InternTrack helper functions.

// Wraps all the dimensions for a track before passing them to InternTrack.
template <typename... D>
constexpr auto Dimensions(D... dimensions) {
  return DimensionsT<D...>{dimensions...};
}

// Indicates that the name of the track was provided in the blueprint.
constexpr std::nullptr_t BlueprintName() {
  return nullptr;
}

// Indicates that the name of the track should be `id`. Only valid if
// `DynamicNameBlueprint()` was passed when creating the blueprint.
constexpr StringPool::Id DynamicName(StringPool::Id id) {
  return id;
}

// Indicates that the unit of the track was provided in the blueprint.
constexpr std::nullptr_t BlueprintUnit() {
  return nullptr;
}

// Indicates that the unit of the track should be `id`. Only valid if
// `DynamicUnitBlueprint()` was passed when creating the blueprint.
constexpr StringPool::Id DynamicUnit(StringPool::Id id) {
  return id;
}

// End of InternTrack helper functions.

}  // namespace perfetto::trace_processor::tracks

#endif  // SRC_TRACE_PROCESSOR_IMPORTERS_COMMON_TRACKS_H_
