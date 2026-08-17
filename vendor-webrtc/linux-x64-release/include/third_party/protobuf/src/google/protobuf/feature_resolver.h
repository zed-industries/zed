// Protocol Buffers - Google's data interchange format
// Copyright 2023 Google Inc.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

#ifndef GOOGLE_PROTOBUF_FEATURE_RESOLVER_H__
#define GOOGLE_PROTOBUF_FEATURE_RESOLVER_H__

#include <memory>
#include <string>
#include <utility>
#include <vector>

#include "absl/container/flat_hash_set.h"
#include "absl/status/status.h"
#include "absl/status/statusor.h"
#include "absl/strings/string_view.h"
#include "absl/types/span.h"
#include "google/protobuf/descriptor.h"
#include "google/protobuf/descriptor.pb.h"
#include "google/protobuf/dynamic_message.h"

// Must be included last.
#include "google/protobuf/port_def.inc"

namespace google {
namespace protobuf {

// These helpers implement the unique behaviors of edition features.  For more
// details, see go/protobuf-editions-features.
class PROTOBUF_EXPORT FeatureResolver {
 public:
  FeatureResolver(FeatureResolver&&) = default;
  FeatureResolver& operator=(FeatureResolver&&) = delete;

  // Compiles a set of FeatureSet extensions into a mapping of edition to unique
  // defaults.  This is the most complicated part of feature resolution, and by
  // abstracting this out into an intermediate message, we can make feature
  // resolution significantly more portable.
  static absl::StatusOr<FeatureSetDefaults> CompileDefaults(
      const Descriptor* feature_set,
      absl::Span<const FieldDescriptor* const> extensions,
      Edition minimum_edition, Edition maximum_edition);

  // Creates a new FeatureResolver at a specific edition.  This calculates the
  // default feature set for that edition, using the output of CompileDefaults.
  static absl::StatusOr<FeatureResolver> Create(
      Edition edition, const FeatureSetDefaults& defaults);

  // Creates a new feature set using inheritance and default behavior. This is
  // designed to be called recursively, and the parent feature set is expected
  // to be a fully merged one.  The returned FeatureSet will be fully resolved
  // for any extensions that were used to construct the defaults.
  absl::StatusOr<FeatureSet> MergeFeatures(
      const FeatureSet& merged_parent, const FeatureSet& unmerged_child) const;

  // Validates an unresolved FeatureSet object to make sure they obey the
  // lifetime requirements.  This needs to run *within* the pool being built, so
  // that the descriptors of any feature extensions are known and can be
  // validated.  `pool_descriptor` should point to the FeatureSet descriptor
  // inside the pool, or nullptr if one doesn't exist,
  //
  // This will return error messages for any explicitly set features used before
  // their introduction or after their removal.  Warnings will be included for
  // any explicitly set features that have been deprecated.
  struct ValidationResults {
    std::vector<std::string> errors;
    std::vector<std::string> warnings;
  };
  static ValidationResults ValidateFeatureLifetimes(
      Edition edition, const FeatureSet& features,
      const Descriptor* pool_descriptor);

 private:
  explicit FeatureResolver(FeatureSet defaults)
      : defaults_(std::move(defaults)) {}

  FeatureSet defaults_;
};

}  // namespace protobuf
}  // namespace google

#endif  // GOOGLE_PROTOBUF_FEATURE_RESOLVER_H__

#include "google/protobuf/port_undef.inc"
