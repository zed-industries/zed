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

#ifndef SRC_TRACE_PROCESSOR_IMPORTERS_ETM_TYPES_H_
#define SRC_TRACE_PROCESSOR_IMPORTERS_ETM_TYPES_H_

#include <cstdint>
#include <memory>
#include <variant>

#include "perfetto/ext/base/flat_hash_map.h"
#include "src/trace_processor/importers/etm/opencsd.h"
#include "src/trace_processor/types/destructible.h"

namespace perfetto::trace_processor::etm {

// Wrapper around open_csd library `EtmV4Config` or `ETEConfig` classes so
// instances can be stored in a `TraceStorage`.
class Configuration : public Destructible {
 public:
  explicit Configuration(const ocsd_ete_cfg& cfg) : config_(ETEConfig(&cfg)) {}
  explicit Configuration(const ocsd_etmv4_cfg& cfg)
      : config_(EtmV4Config(&cfg)) {}

  ~Configuration() override;

  const EtmV4Config& etm_v4_config() const {
    return std::visit([](const auto& arg) -> const EtmV4Config& { return arg; },
                      config_);
  }

 private:
  std::variant<EtmV4Config, ETEConfig> config_;
};

}  // namespace perfetto::trace_processor::etm

#endif  // SRC_TRACE_PROCESSOR_IMPORTERS_ETM_TYPES_H_
