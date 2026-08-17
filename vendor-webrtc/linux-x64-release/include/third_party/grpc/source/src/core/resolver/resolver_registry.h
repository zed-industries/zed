//
// Copyright 2015 gRPC authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

#ifndef GRPC_SRC_CORE_RESOLVER_RESOLVER_REGISTRY_H
#define GRPC_SRC_CORE_RESOLVER_RESOLVER_REGISTRY_H

#include <grpc/support/port_platform.h>

#include <map>
#include <memory>
#include <string>
#include <utility>

#include "absl/strings/string_view.h"
#include "src/core/lib/channel/channel_args.h"
#include "src/core/lib/iomgr/iomgr_fwd.h"
#include "src/core/resolver/resolver.h"
#include "src/core/resolver/resolver_factory.h"
#include "src/core/util/orphanable.h"
#include "src/core/util/uri.h"

namespace grpc_core {

class ResolverRegistry final {
 private:
  // Forward declaration needed to use this in Builder.
  struct State {
    std::map<absl::string_view, std::unique_ptr<ResolverFactory>> factories;
    std::string default_prefix;
  };

 public:
  /// Methods used to create and populate the ResolverRegistry.
  /// NOT THREAD SAFE -- to be used only during global gRPC
  /// initialization and shutdown.
  class Builder final {
   public:
    Builder();

    /// Sets the default URI prefix to \a default_prefix.
    void SetDefaultPrefix(std::string default_prefix);

    /// Registers a resolver factory.  The factory will be used to create a
    /// resolver for any URI whose scheme matches that of the factory.
    void RegisterResolverFactory(std::unique_ptr<ResolverFactory> factory);

    /// Returns true iff scheme already has a registered factory.
    bool HasResolverFactory(absl::string_view scheme) const;

    /// Wipe everything in the registry and reset to empty.
    void Reset();

    ResolverRegistry Build();

   private:
    ResolverRegistry::State state_;
  };

  ResolverRegistry(const ResolverRegistry&) = delete;
  ResolverRegistry& operator=(const ResolverRegistry&) = delete;
  ResolverRegistry(ResolverRegistry&&) noexcept;
  ResolverRegistry& operator=(ResolverRegistry&&) noexcept;

  /// Checks whether the user input \a target is valid to create a resolver.
  bool IsValidTarget(absl::string_view target) const;

  /// Creates a resolver given \a target.
  /// First tries to parse \a target as a URI. If this succeeds, tries
  /// to locate a registered resolver factory based on the URI scheme.
  /// If parsing fails or there is no factory for the URI's scheme,
  /// prepends default_prefix to target and tries again.
  /// If a resolver factory is found, uses it to instantiate a resolver and
  /// returns it; otherwise, returns nullptr.
  /// \a args, \a pollset_set, and \a work_serializer are passed to the
  /// factory's \a CreateResolver() method. \a args are the channel args to be
  /// included in resolver results. \a pollset_set is used to drive I/O in the
  /// name resolution process. \a work_serializer is the work_serializer under
  /// which all resolver calls will be run. \a result_handler is used to return
  /// results from the resolver.
  OrphanablePtr<Resolver> CreateResolver(
      absl::string_view target, const ChannelArgs& args,
      grpc_pollset_set* pollset_set,
      std::shared_ptr<WorkSerializer> work_serializer,
      std::unique_ptr<Resolver::ResultHandler> result_handler) const;

  /// Returns the default authority to pass from a client for \a target.
  std::string GetDefaultAuthority(absl::string_view target) const;

  /// Returns \a target with the default prefix prepended, if needed.
  std::string AddDefaultPrefixIfNeeded(absl::string_view target) const;

  /// Returns the resolver factory for \a scheme.
  /// Caller does NOT own the return value.
  ResolverFactory* LookupResolverFactory(absl::string_view scheme) const;

 private:
  explicit ResolverRegistry(State state) : state_(std::move(state)) {}

  // TODO(ctiller): fix callers such that the canonical_target argument can be
  // removed, and replaced with uri.ToString().
  ResolverFactory* FindResolverFactory(absl::string_view target, URI* uri,
                                       std::string* canonical_target) const;

  State state_;
};

}  // namespace grpc_core

#endif  // GRPC_SRC_CORE_RESOLVER_RESOLVER_REGISTRY_H
