// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_QUOTA_QUOTA_UTILS_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_QUOTA_QUOTA_UTILS_H_

#include "mojo/public/cpp/bindings/pending_receiver.h"
#include "third_party/blink/public/mojom/quota/quota_manager_host.mojom-blink.h"

namespace blink {

class ExecutionContext;

void ConnectToQuotaManagerHost(
    ExecutionContext*,
    mojo::PendingReceiver<mojom::blink::QuotaManagerHost>);

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_QUOTA_QUOTA_UTILS_H_
