// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_P2P_MDNS_RESPONDER_ADAPTER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_P2P_MDNS_RESPONDER_ADAPTER_H_

#include "mojo/public/cpp/bindings/shared_remote.h"
#include "services/network/public/mojom/mdns_responder.mojom-blink-forward.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/webrtc/rtc_base/ip_address.h"
#include "third_party/webrtc/rtc_base/mdns_responder_interface.h"

namespace blink {

class MojoBindingContext;

// This class is created on the main thread but is used only on the WebRTC
// worker threads. The MdnsResponderAdapter implements the WebRTC mDNS responder
// interface via the MdnsResponder service in Chromium, and is used to register
// and resolve mDNS hostnames to conceal local IP addresses.
class PLATFORM_EXPORT MdnsResponderAdapter
    : public webrtc::MdnsResponderInterface {
 public:
  // The adapter should be created on the main thread to have access to the
  // connector to the service manager.
  explicit MdnsResponderAdapter(MojoBindingContext& context);
  MdnsResponderAdapter(const MdnsResponderAdapter&) = delete;
  MdnsResponderAdapter& operator=(const MdnsResponderAdapter&) = delete;
  ~MdnsResponderAdapter() override;

  // webrtc::MdnsResponderInterface implementation.
  void CreateNameForAddress(const webrtc::IPAddress& addr,
                            NameCreatedCallback callback) override;
  void RemoveNameForAddress(const webrtc::IPAddress& addr,
                            NameRemovedCallback callback) override;

 private:
  mojo::SharedRemote<network::mojom::blink::MdnsResponder>
      shared_remote_client_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_P2P_MDNS_RESPONDER_ADAPTER_H_
