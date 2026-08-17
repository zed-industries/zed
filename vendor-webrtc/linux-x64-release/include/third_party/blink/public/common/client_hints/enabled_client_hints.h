// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_CLIENT_HINTS_ENABLED_CLIENT_HINTS_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_CLIENT_HINTS_ENABLED_CLIENT_HINTS_H_

#include "services/network/public/mojom/web_client_hints_types.mojom-shared.h"
#include "third_party/blink/public/common/common_export.h"

namespace blink {

// EnabledClientHints stores all the client hints along with whether the hint
// is enabled or not.
class BLINK_COMMON_EXPORT EnabledClientHints {
 public:
  EnabledClientHints() = default;

  // Returns true if the client hint should be sent on the HTTP request header,
  // and returns false otherwise. Even if SetIsEnabled() is called on a client
  // hint, it is still possible that IsEnabled() returns false, if some other
  // conditions (such as a feature being toggled) indicate the client hint
  // should not be sent.
  //
  // Returns false if `type` is not a valid WebClientHintsType value.
  bool IsEnabled(network::mojom::WebClientHintsType type) const;

  // Sets the client hint as enabled for sending in an HTTP request header. Even
  // if the client hint header is set to enabled, it is still possible that
  // other factors (such as feature toggles) should cause the client hint to not
  // be sent, and in that case, IsEnabled() would return false.
  //
  // If `type` is not a valid WebClientHintsType value, nothing is changed (no
  // client hints get enabled).
  void SetIsEnabled(network::mojom::WebClientHintsType type, bool should_send);

  // Returns a list of the enabled client hints.
  std::vector<network::mojom::WebClientHintsType> GetEnabledHints() const;

 private:
  // Deprecated/removed preferences will stick around in this array
  // unused. Consider refactoring into a map down the road.
  std::array<bool,
             static_cast<int>(network::mojom::WebClientHintsType::kMaxValue) +
                 1>
      enabled_types_{};
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_CLIENT_HINTS_ENABLED_CLIENT_HINTS_H_
