/*
 * Copyright (C) 2020 The Android Open Source Project
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

#ifndef SRC_TRACE_PROCESSOR_IMPORTERS_COMMON_TRACK_COMPRESSOR_H_
#define SRC_TRACE_PROCESSOR_IMPORTERS_COMMON_TRACK_COMPRESSOR_H_

#include <cstddef>
#include <cstdint>
#include <string_view>
#include <tuple>
#include <type_traits>
#include <utility>
#include <vector>

#include "perfetto/ext/base/flat_hash_map.h"
#include "perfetto/ext/base/hash.h"
#include "src/trace_processor/importers/common/track_tracker.h"
#include "src/trace_processor/importers/common/tracks.h"
#include "src/trace_processor/importers/common/tracks_internal.h"
#include "src/trace_processor/storage/trace_storage.h"
#include "src/trace_processor/types/trace_processor_context.h"

namespace perfetto::trace_processor {

class TrackCompressorUnittest;

namespace internal {

template <typename Ds, size_t r, size_t... Is>
constexpr auto UncompressedDimensions(Ds,
                                      std::integral_constant<size_t, r>,
                                      std::index_sequence<Is...>) {
  static_assert(r > 0,
                "Wrong blueprint passed to TrackCompressor Intern* function. "
                "Make sure Blueprint was created using "
                "TrackCompressor::SliceBlueprint *not* tracks::SliceBlueprint");
  return tracks::Dimensions(std::tuple_element_t<Is, Ds>()...);
}

template <typename BlueprintT>
using uncompressed_dimensions_t = decltype(UncompressedDimensions(
    typename BlueprintT::dimensions_t(),
    std::integral_constant<
        size_t,
        std::tuple_size_v<typename BlueprintT::dimensions_t>>(),
    std::make_index_sequence<
        std::tuple_size_v<typename BlueprintT::dimensions_t> == 0
            ? 0
            : std::tuple_size_v<typename BlueprintT::dimensions_t> - 1>()));

}  // namespace internal

// "Compresses" and interns trace processor tracks for a given track type.
//
// When writing traces, sometimes it's not possible to reuse tracks meaning
// people create one track per event. Creating a new track for every event,
// however, leads to an explosion of tracks which is undesirable. This class
// exists to multiplex slices so that multiple events correspond to a single
// track in a way which minimises the number of tracks.
//
// WARNING: the usage of this class SHOULD BE VERY RARE. These days, this class
// mainly exists for legacy usage due to how the Perfetto UI used to work rather
// than an active choice. Prefer making tracks peers and adding a UI plugin if
// you want custom visualization instead of using this class.
class TrackCompressor {
 public:
  explicit TrackCompressor(TraceProcessorContext* context);
  ~TrackCompressor() = default;

  // Starts a new slice which has the given cookie.
  template <typename BlueprintT>
  TrackId InternBegin(
      const BlueprintT& bp,
      const internal::uncompressed_dimensions_t<BlueprintT>& dims,
      int64_t cookie,
      const typename BlueprintT::name_t& name = tracks::BlueprintName()) {
    uint64_t hash = tracks::HashFromBlueprintAndDimensions(bp, dims);
    auto final_dims = std::tuple_cat(
        dims, std::make_tuple(BeginInternal(TypeToNestingBehaviour(bp.type),
                                            hash, cookie)));
    return context_->track_tracker->InternTrack(bp, final_dims, name);
  }

  // Ends a new slice which has the given cookie.
  template <typename BlueprintT>
  TrackId InternEnd(
      BlueprintT bp,
      const internal::uncompressed_dimensions_t<BlueprintT>& dims,
      int64_t cookie,
      const typename BlueprintT::name_t& name = tracks::BlueprintName()) {
    uint64_t hash = tracks::HashFromBlueprintAndDimensions(bp, dims);
    auto final_dims =
        std::tuple_cat(dims, std::make_tuple(EndInternal(hash, cookie)));
    return context_->track_tracker->InternTrack(bp, final_dims, name);
  }

  // Creates a scoped slice.
  // This method makes sure that any other slice in this track set does
  // not happen simultaneously on the returned track.
  template <typename BlueprintT>
  TrackId InternScoped(
      BlueprintT bp,
      const internal::uncompressed_dimensions_t<BlueprintT>& dims,
      int64_t ts,
      int64_t dur,
      const typename BlueprintT::name_t& name = tracks::BlueprintName()) {
    uint64_t hash = tracks::HashFromBlueprintAndDimensions(bp, dims);
    auto final_dims =
        std::tuple_cat(dims, std::make_tuple(ScopedInternal(hash, ts, dur)));
    return context_->track_tracker->InternTrack(bp, final_dims, name);
  }

  // Wrapper around tracks::SliceBlueprint which makes the blueprint eligible
  // for compression with TrackCompressor. Please see documentation of
  // tracks::SliceBlueprint for usage.
  template <typename NB = tracks::NameBlueprintT::Auto, typename... D>
  static constexpr auto SliceBlueprint(
      const char type[],
      tracks::DimensionBlueprintsT<D...> dimensions = {},
      NB name = NB{}) {
    auto blueprint = tracks::SliceBlueprint(type, dimensions, name);
    using BT = decltype(blueprint);
    constexpr auto kCompressorIdxDimensionIndex =
        std::tuple_size_v<typename BT::dimension_blueprints_t>;
    return std::apply(
        [&](auto... x) {
          auto blueprints = blueprint.dimension_blueprints;
          blueprints[kCompressorIdxDimensionIndex] =
              tracks::UintDimensionBlueprint("track_compressor_idx");

          if constexpr (std::is_base_of_v<tracks::NameBlueprintT::FnBase,
                                          typename BT::name_blueprint_t>) {
            using F = decltype(blueprint.name_blueprint.fn);
            auto fn =
                MakeNameFn<F, decltype(x)...>(blueprint.name_blueprint.fn);
            return tracks::BlueprintT<
                decltype(fn), typename BT::unit_blueprint_t, decltype(x)...,
                tracks::DimensionBlueprintT<uint32_t>>{
                {
                    blueprint.event_type,
                    blueprint.type,
                    blueprint.hasher,
                    blueprints,
                },
                fn,
                blueprint.unit_blueprint,
            };
          } else {
            return tracks::BlueprintT<
                typename BT::name_blueprint_t, typename BT::unit_blueprint_t,
                decltype(x)..., tracks::DimensionBlueprintT<uint32_t>>{
                {
                    blueprint.event_type,
                    blueprint.type,
                    blueprint.hasher,
                    blueprints,
                },
                blueprint.name_blueprint,
                blueprint.unit_blueprint,
            };
          }
        },
        typename BT::dimension_blueprints_t());
  }

 private:
  friend class TrackCompressorUnittest;

  // Indicates the nesting behaviour of slices associated to a single slice
  // stack.
  enum class NestingBehaviour {
    // Indicates that slices are nestable; that is, a stack of slices with
    // the same cookie should stack properly, not merely overlap.
    //
    // This pattern should be the default behaviour that most async slices
    // should use.
    kNestable,

    // Indicates that slices are unnestable but also saturating; that is
    // calling Begin -> Begin only causes a single Begin to be recorded.
    // This is only really useful for Android async slices which have this
    // behaviour for legacy reasons. See the comment in
    // SystraceParser::ParseSystracePoint for information on why
    // this behaviour exists.
    kLegacySaturatingUnnestable,
  };

  struct TrackState {
    enum class SliceType { kCookie, kTimestamp };
    SliceType slice_type;

    union {
      // Only valid for |slice_type| == |SliceType::kCookie|.
      int64_t cookie;

      // Only valid for |slice_type| == |SliceType::kTimestamp|.
      int64_t ts_end;
    };

    // Only used for |slice_type| == |SliceType::kCookie|.
    uint32_t nest_count;
  };

  struct TrackSetNew {
    std::vector<TrackState> tracks;
  };

  uint32_t BeginInternal(NestingBehaviour, uint64_t hash, int64_t cookie);

  uint32_t EndInternal(uint64_t hash, int64_t cookie);

  uint32_t ScopedInternal(uint64_t hash, int64_t ts, int64_t dur);

  static constexpr NestingBehaviour TypeToNestingBehaviour(
      std::string_view type) {
    if (type == "atrace_async_slice") {
      return NestingBehaviour::kLegacySaturatingUnnestable;
    }
    return NestingBehaviour::kNestable;
  }

  template <typename F, typename... T>
  static constexpr auto MakeNameFn(F fn) {
    auto f = [fn](typename T::type... y, uint32_t) { return fn(y...); };
    return tracks::NameBlueprintT::Fn<decltype(f)>{{}, f};
  }

  // Returns the state for a track using the following algorithm:
  // 1. If a track exists with the given cookie in the vector, returns
  //    that track.
  // 2. Otherwise, looks for any track in the set which is "open" (i.e.
  //    does not have another slice currently scheduled).
  // 3. Otherwise, creates a new track and adds it to the vector.
  static TrackState& GetOrCreateTrackForCookie(std::vector<TrackState>& tracks,
                                               int64_t cookie);

  base::FlatHashMap<uint64_t, TrackSetNew, base::AlreadyHashed<uint64_t>> sets_;

  TraceProcessorContext* const context_;
};

}  // namespace perfetto::trace_processor

#endif  // SRC_TRACE_PROCESSOR_IMPORTERS_COMMON_TRACK_COMPRESSOR_H_
