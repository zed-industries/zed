// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_NETWORK_IP_ADDRESS_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_NETWORK_IP_ADDRESS_H_

#include "base/hash/hash.h"
#include "net/base/ip_address.h"
#include "third_party/blink/renderer/platform/wtf/hash_traits.h"

namespace WTF {

// Uses a default-constructed zero-sized IPAddress as the empty value and an
// invalid one-byte IPAddress as the deleted value.
template <>
struct HashTraits<net::IPAddress> : GenericHashTraits<net::IPAddress> {
  static const bool kEmptyValueIsZero = true;

  static unsigned GetHash(const net::IPAddress& ip_address) {
    return static_cast<unsigned>(
        base::FastHash(base::span(ip_address.bytes())));
  }

  static bool IsDeletedValue(const net::IPAddress& value) {
    return value.size() == 1;
  }

  static void ConstructDeletedValue(net::IPAddress& slot) {
    uint8_t deleted_val[1] = {0};
    new (&slot) net::IPAddress(deleted_val);
  }
};

}  // namespace WTF

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_NETWORK_IP_ADDRESS_H_
