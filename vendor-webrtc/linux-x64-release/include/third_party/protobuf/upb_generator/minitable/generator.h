// Protocol Buffers - Google's data interchange format
// Copyright 2023 Google LLC.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#include "google/protobuf/compiler/code_generator.h"
#include "upb/reflection/def.hpp"
#include "upb_generator/common.h"
#include "upb_generator/file_layout.h"

namespace upb {
namespace generator {

struct MiniTableOptions {
  bool bootstrap = false;
  bool one_output_per_message = false;
  bool strip_nonfunctional_codegen = false;
};

void WriteMiniTableSource(const DefPoolPair& pools, upb::FileDefPtr file,
                          const MiniTableOptions& options, Output& output);
void WriteMiniTableMultipleSources(const DefPoolPair& pools,
                                   upb::FileDefPtr file,
                                   const MiniTableOptions& options,
                                   google::protobuf::compiler::GeneratorContext* context);
void WriteMiniTableHeader(const DefPoolPair& pools, upb::FileDefPtr file,
                          const MiniTableOptions& options, Output& output);

}  // namespace generator
}  // namespace upb
