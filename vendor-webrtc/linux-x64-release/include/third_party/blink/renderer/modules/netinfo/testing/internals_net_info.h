// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_NETINFO_TESTING_INTERNALS_NET_INFO_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_NETINFO_TESTING_INTERNALS_NET_INFO_H_

#include "third_party/blink/renderer/platform/bindings/exception_state.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class Internals;
class V8EffectiveConnectionType;

class InternalsNetInfo {
  STATIC_ONLY(InternalsNetInfo);

 public:
  static void setNetworkConnectionInfoOverride(
      Internals& internals,
      bool on_line,
      const String& type,
      const V8EffectiveConnectionType& effective_type,
      uint32_t http_rtt_msec,
      double downlink_max_mbps,
      ExceptionState& exception_state);
  static void setSaveDataEnabled(Internals&, bool enabled);

  static void clearNetworkConnectionInfoOverride(Internals&);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_NETINFO_TESTING_INTERNALS_NET_INFO_H_
