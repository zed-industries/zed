// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_SERVICE_WORKER_SERVICE_WORKER_ROUTER_RULE_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_SERVICE_WORKER_SERVICE_WORKER_ROUTER_RULE_H_

#include <stdint.h>

#include <memory>
#include <optional>
#include <tuple>
#include <vector>

#include "third_party/blink/public/common/common_export.h"
#include "third_party/blink/public/common/safe_url_pattern.h"

namespace network::mojom {

enum class RequestMode : int32_t;
enum class RequestDestination : int32_t;
enum class ServiceWorkerRouterSourceType : int32_t;

}  // namespace network::mojom

namespace blink {

class ServiceWorkerRouterCondition;

// TODO(crbug.com/1490445): set this value by discussing in spec proposal.
static constexpr int kServiceWorkerRouterConditionMaxRecursionDepth = 10;
// TODO(crbug.com/1503017): set this value by discussing in spec proposal.
static constexpr size_t kServiceWorkerMaxRouterSize = 256;

struct BLINK_COMMON_EXPORT ServiceWorkerRouterRequestCondition {
  // https://fetch.spec.whatwg.org/#concept-request-method
  // Technically, it can be an arbitrary string, but Chromium would set
  // k*Method in net/http/http_request_headers.h
  std::optional<std::string> method;
  // RequestMode in services/network/public/mojom/fetch_api.mojom
  std::optional<network::mojom::RequestMode> mode;
  // RequestDestination in services/network/public/mojom/fetch_api.mojom
  std::optional<network::mojom::RequestDestination> destination;

  bool operator==(const ServiceWorkerRouterRequestCondition& other) const;
};

struct BLINK_COMMON_EXPORT ServiceWorkerRouterRunningStatusCondition {
  enum class RunningStatusEnum {
    kRunning = 0,
    // This includes kStarting and kStopping in addition to kStopped.
    // These states are consolidated to kNotRunning because they need to
    // wait for ServiceWorker set up to run the fetch handler.
    kNotRunning = 1,
  };

  RunningStatusEnum status;

  bool operator==(
      const ServiceWorkerRouterRunningStatusCondition& other) const {
    return status == other.status;
  }
};

struct BLINK_COMMON_EXPORT ServiceWorkerRouterOrCondition {
  std::vector<ServiceWorkerRouterCondition> conditions;

  bool operator==(const ServiceWorkerRouterOrCondition& other) const;
};

struct BLINK_COMMON_EXPORT ServiceWorkerRouterNotCondition {
  std::unique_ptr<ServiceWorkerRouterCondition> condition;

  ServiceWorkerRouterNotCondition();
  ~ServiceWorkerRouterNotCondition();
  ServiceWorkerRouterNotCondition(const ServiceWorkerRouterNotCondition&);
  ServiceWorkerRouterNotCondition& operator=(
      const ServiceWorkerRouterNotCondition&);
  ServiceWorkerRouterNotCondition(ServiceWorkerRouterNotCondition&&);
  ServiceWorkerRouterNotCondition& operator=(ServiceWorkerRouterNotCondition&&);

  bool operator==(const ServiceWorkerRouterNotCondition& other) const;
};

// TODO(crbug.com/1371756): implement other conditions in the proposal.
class BLINK_COMMON_EXPORT ServiceWorkerRouterCondition {
  // We use aggregated getter/setter in this class in order to emit errors on
  // the callers when other conditions are added in the future
 public:
  using MemberRef =
      std::tuple<std::optional<SafeUrlPattern>&,
                 std::optional<ServiceWorkerRouterRequestCondition>&,
                 std::optional<ServiceWorkerRouterRunningStatusCondition>&,
                 std::optional<ServiceWorkerRouterOrCondition>&,
                 std::optional<ServiceWorkerRouterNotCondition>&>;
  using MemberConstRef = std::tuple<
      const std::optional<SafeUrlPattern>&,
      const std::optional<ServiceWorkerRouterRequestCondition>&,
      const std::optional<ServiceWorkerRouterRunningStatusCondition>&,
      const std::optional<ServiceWorkerRouterOrCondition>&,
      const std::optional<ServiceWorkerRouterNotCondition>&>;

  ServiceWorkerRouterCondition() = default;
  ServiceWorkerRouterCondition(const ServiceWorkerRouterCondition&) = default;
  ServiceWorkerRouterCondition& operator=(const ServiceWorkerRouterCondition&) =
      default;
  ServiceWorkerRouterCondition(ServiceWorkerRouterCondition&&) = default;
  ServiceWorkerRouterCondition& operator=(ServiceWorkerRouterCondition&&) =
      default;

  ServiceWorkerRouterCondition(
      const std::optional<SafeUrlPattern>& url_pattern,
      const std::optional<ServiceWorkerRouterRequestCondition>& request,
      const std::optional<ServiceWorkerRouterRunningStatusCondition>&
          running_status,
      const std::optional<ServiceWorkerRouterOrCondition>& or_condition,
      const std::optional<ServiceWorkerRouterNotCondition>& not_condition)
      : url_pattern_(url_pattern),
        request_(request),
        running_status_(running_status),
        or_condition_(or_condition),
        not_condition_(not_condition) {}

  // Returns tuple: suggest using structured bindings on the caller side.
  MemberRef get() {
    return {url_pattern_, request_, running_status_, or_condition_,
            not_condition_};
  }
  MemberConstRef get() const {
    return {url_pattern_, request_, running_status_, or_condition_,
            not_condition_};
  }

  bool IsEmpty() const {
    return !(url_pattern_ || request_ || running_status_ || or_condition_ ||
             not_condition_);
  }

  bool IsOrConditionExclusive() const {
    return or_condition_.has_value() !=
           (url_pattern_ || request_ || running_status_ || not_condition_);
  }
  bool IsNotConditionExclusive() const {
    return not_condition_.has_value() !=
           (url_pattern_ || request_ || running_status_ || or_condition_);
  }

  bool IsValid() const {
    return !IsEmpty() && IsOrConditionExclusive() && IsNotConditionExclusive();
  }

  bool operator==(const ServiceWorkerRouterCondition& other) const;

  static ServiceWorkerRouterCondition WithUrlPattern(
      const SafeUrlPattern& url_pattern) {
    return {url_pattern, std::nullopt, std::nullopt, std::nullopt,
            std::nullopt};
  }
  static ServiceWorkerRouterCondition WithRequest(
      const ServiceWorkerRouterRequestCondition& request) {
    return {std::nullopt, request, std::nullopt, std::nullopt, std::nullopt};
  }
  static ServiceWorkerRouterCondition WithRunningStatus(
      const ServiceWorkerRouterRunningStatusCondition& running_status) {
    return {std::nullopt, std::nullopt, running_status, std::nullopt,
            std::nullopt};
  }
  static ServiceWorkerRouterCondition WithOrCondition(
      const ServiceWorkerRouterOrCondition& or_condition) {
    return {std::nullopt, std::nullopt, std::nullopt, or_condition,
            std::nullopt};
  }
  static ServiceWorkerRouterCondition WithNotCondition(
      const ServiceWorkerRouterNotCondition& not_condition) {
    return {std::nullopt, std::nullopt, std::nullopt, std::nullopt,
            not_condition};
  }

 private:
  // URLPattern to be used for matching.
  std::optional<SafeUrlPattern> url_pattern_;

  // Request to be used for matching.
  std::optional<ServiceWorkerRouterRequestCondition> request_;

  // Running status to be used for matching.
  std::optional<ServiceWorkerRouterRunningStatusCondition> running_status_;

  // `Or` condition to be used for matching
  // We need `_condition` suffix to avoid conflict with reserved keywords in C++
  std::optional<ServiceWorkerRouterOrCondition> or_condition_;

  // `Not` condition to be used for matching
  // We need `_condition` suffix to avoid conflict with reserved keywords in C++
  std::optional<ServiceWorkerRouterNotCondition> not_condition_;
};

// Network source structure.
// TODO(crbug.com/1371756): implement fields in the proposal.
struct BLINK_COMMON_EXPORT ServiceWorkerRouterNetworkSource {
  bool operator==(const ServiceWorkerRouterNetworkSource& other) const =
      default;
};

// Race network and fetch event sources.
struct BLINK_COMMON_EXPORT ServiceWorkerRouterRaceNetworkAndFetchEventSource {
  bool operator==(const ServiceWorkerRouterRaceNetworkAndFetchEventSource&
                      other) const = default;
};

// Fetch handler source structure.
struct BLINK_COMMON_EXPORT ServiceWorkerRouterFetchEventSource {
  bool operator==(const ServiceWorkerRouterFetchEventSource& other) const =
      default;
};

// Cache source structure.
struct BLINK_COMMON_EXPORT ServiceWorkerRouterCacheSource {
  // A name of the Cache object.
  // If the field is not set, any of the Cache objects that the CacheStorage
  // tracks are used for matching as if CacheStorage.match().
  std::optional<std::string> cache_name;

  bool operator==(const ServiceWorkerRouterCacheSource& other) const = default;
};

// Race network and cache sources.
struct BLINK_COMMON_EXPORT ServiceWorkerRouterRaceNetworkAndCacheSource {
  ServiceWorkerRouterCacheSource cache_source;

  bool operator==(const ServiceWorkerRouterRaceNetworkAndCacheSource& other)
      const = default;
};

// This represents a source of the router rule.
// TODO(crbug.com/1371756): implement other sources in the proposal.
struct BLINK_COMMON_EXPORT ServiceWorkerRouterSource {
  network::mojom::ServiceWorkerRouterSourceType type;

  std::optional<ServiceWorkerRouterNetworkSource> network_source;
  std::optional<ServiceWorkerRouterRaceNetworkAndFetchEventSource>
      race_network_and_fetch_event_source;
  std::optional<ServiceWorkerRouterFetchEventSource> fetch_event_source;
  std::optional<ServiceWorkerRouterCacheSource> cache_source;
  std::optional<ServiceWorkerRouterRaceNetworkAndCacheSource>
      race_network_and_cache_source;

  bool operator==(const ServiceWorkerRouterSource& other) const;
};

// This represents a ServiceWorker static routing API's router rule.
// It represents each route.
struct BLINK_COMMON_EXPORT ServiceWorkerRouterRule {
  // A rule can have one condition object. A condition object should not be
  // empty.
  ServiceWorkerRouterCondition condition;
  // There can be a list of sources, and expected to be routed from
  // front to back.
  std::vector<ServiceWorkerRouterSource> sources;

  bool operator==(const ServiceWorkerRouterRule& other) const {
    return condition == other.condition && sources == other.sources;
  }
};

// This represents a condition of the router rule.
// This represents a list of ServiceWorker static routing API's router rules.
struct BLINK_COMMON_EXPORT ServiceWorkerRouterRules {
  std::vector<ServiceWorkerRouterRule> rules;

  bool operator==(const ServiceWorkerRouterRules& other) const {
    return rules == other.rules;
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_SERVICE_WORKER_SERVICE_WORKER_ROUTER_RULE_H_
