// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_CROSS_THREAD_RESOURCE_TIMING_INFO_COPIER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_CROSS_THREAD_RESOURCE_TIMING_INFO_COPIER_H_

#include "third_party/blink/public/mojom/timing/resource_timing.mojom-blink-forward.h"
#include "third_party/blink/renderer/platform/wtf/cross_thread_copier.h"

namespace WTF {

template <>
struct CrossThreadCopier<blink::mojom::blink::ServerTimingInfoPtr> {
  STATIC_ONLY(CrossThreadCopier);
  using Type = blink::mojom::blink::ServerTimingInfoPtr;
  static Type Copy(const Type&);
};

template <>
struct CrossThreadCopier<blink::mojom::blink::ResourceTimingInfoPtr> {
  STATIC_ONLY(CrossThreadCopier);
  using Type = blink::mojom::blink::ResourceTimingInfoPtr;
  static Type Copy(const Type&);
};

}  // namespace WTF

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_CROSS_THREAD_RESOURCE_TIMING_INFO_COPIER_H_
