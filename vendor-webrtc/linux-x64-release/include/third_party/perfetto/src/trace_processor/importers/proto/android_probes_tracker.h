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

#ifndef SRC_TRACE_PROCESSOR_IMPORTERS_PROTO_ANDROID_PROBES_TRACKER_H_
#define SRC_TRACE_PROCESSOR_IMPORTERS_PROTO_ANDROID_PROBES_TRACKER_H_

#include <optional>
#include <set>

#include "perfetto/ext/base/string_view.h"

#include "src/trace_processor/storage/trace_storage.h"
#include "src/trace_processor/types/trace_processor_context.h"

namespace perfetto {
namespace trace_processor {

class TraceProcessorContext;

class AndroidProbesTracker : public Destructible {
 public:
  explicit AndroidProbesTracker(TraceStorage*);
  ~AndroidProbesTracker() override;

  // For EnergyBreakdown Descriptor specifications
  struct EnergyConsumerSpecs {
    StringId name;
    StringId type;
    int32_t ordinal;
  };

  struct EntityStateDescriptor {
    StringId entity_name;
    StringId state_name;
    StringId overall_name;
  };

  static AndroidProbesTracker* GetOrCreate(TraceProcessorContext* context) {
    if (!context->android_probes_tracker) {
      context->android_probes_tracker.reset(
          new AndroidProbesTracker(context->storage.get()));
    }
    return static_cast<AndroidProbesTracker*>(
        context->android_probes_tracker.get());
  }

  bool ShouldInsertPackage(const std::string& package_name) const {
    auto it = seen_packages_.find(package_name);
    return it == seen_packages_.end();
  }

  void InsertedPackage(std::string package_name) {
    seen_packages_.emplace(std::move(package_name));
  }

  std::optional<TrackId> GetPowerRailTrack(uint32_t index) {
    if (index >= power_rail_tracks_.size())
      return std::nullopt;
    TrackId track_id = power_rail_tracks_[index];
    return track_id == kInvalidTrackId ? std::nullopt
                                       : std::make_optional(track_id);
  }

  void SetPowerRailTrack(uint32_t index, TrackId track_id) {
    if (power_rail_tracks_.size() <= index)
      power_rail_tracks_.resize(index + 1, kInvalidTrackId);
    power_rail_tracks_[index] = track_id;
  }

  std::optional<EnergyConsumerSpecs> GetEnergyBreakdownDescriptor(
      int32_t consumer_id) {
    auto it = energy_consumer_descriptors_.find(consumer_id);
    // Didn't receive the descriptor
    if (it == energy_consumer_descriptors_.end()) {
      return std::nullopt;
    }
    return it->second;
  }

  void SetEnergyBreakdownDescriptor(int32_t consumer_id,
                                    StringId name,
                                    StringId type,
                                    int32_t ordinal) {
    auto it_consumer_descriptor =
        energy_consumer_descriptors_.find(consumer_id);

    // Either descriptor was repeated or it came after per uid data.
    if (it_consumer_descriptor != energy_consumer_descriptors_.end())
      return;

    energy_consumer_descriptors_[consumer_id] =
        EnergyConsumerSpecs{name, type, ordinal};
  }

  std::optional<EntityStateDescriptor> GetEntityStateDescriptor(
      int32_t entity_id,
      int32_t state_id) {
    uint64_t id = EntityStateKey(entity_id, state_id);
    auto it = entity_state_descriptors_.find(id);
    // Didn't receive the descriptor
    if (it == entity_state_descriptors_.end()) {
      return std::nullopt;
    }
    return it->second;
  }

  void SetEntityStateDescriptor(int32_t entity_id,
                                int32_t state_id,
                                StringId entity_name,
                                StringId state_name) {
    uint64_t id = EntityStateKey(entity_id, state_id);
    auto it_descriptor = entity_state_descriptors_.find(id);

    // Ignore repeated descriptors.
    if (it_descriptor != entity_state_descriptors_.end())
      return;

    std::string overall_str =
        "Entity residency: " + storage_->GetString(entity_name).ToStdString() +
        " is " + storage_->GetString(state_name).ToStdString();

    StringId overall = storage_->InternString(base::StringView(overall_str));

    entity_state_descriptors_[id] =
        EntityStateDescriptor{entity_name, state_name, overall};
  }

 private:
  TraceStorage* storage_;
  std::set<std::string> seen_packages_;
  std::vector<TrackId> power_rail_tracks_;
  std::unordered_map<int32_t, EnergyConsumerSpecs> energy_consumer_descriptors_;
  std::unordered_map<uint64_t, EntityStateDescriptor> entity_state_descriptors_;

  uint64_t EntityStateKey(int32_t entity_id, int32_t state_id) {
    return (static_cast<uint64_t>(entity_id) << 32) |
           static_cast<uint32_t>(state_id);
  }
};

}  // namespace trace_processor
}  // namespace perfetto

#endif  // SRC_TRACE_PROCESSOR_IMPORTERS_PROTO_ANDROID_PROBES_TRACKER_H_
