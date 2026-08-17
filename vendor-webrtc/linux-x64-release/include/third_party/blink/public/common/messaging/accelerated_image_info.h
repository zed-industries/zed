// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_MESSAGING_ACCELERATED_IMAGE_INFO_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_MESSAGING_ACCELERATED_IMAGE_INFO_H_

#include "base/functional/callback.h"
#include "gpu/command_buffer/client/client_shared_image.h"
#include "gpu/command_buffer/common/mailbox_holder.h"
#include "gpu/command_buffer/common/shared_image_usage.h"
#include "mojo/public/cpp/bindings/pending_remote.h"
#include "third_party/blink/public/common/common_export.h"
#include "third_party/skia/include/core/SkImage.h"

namespace blink {

// This struct represents all the information needed to create an
// AcceleratedStaticImageBitmap in the receiving process.
// See third_party/blink/public/mojom/messaging/static_bitmap_image.mojom
// for details.
struct BLINK_COMMON_EXPORT AcceleratedImageInfo {
  gpu::ExportedSharedImage shared_image;
  gpu::SyncToken sync_token;
  gfx::Size size;
  viz::SharedImageFormat format;
  SkAlphaType alpha_type;
  gfx::ColorSpace color_space;
  base::OnceCallback<void(const gpu::SyncToken& sync_token)> release_callback;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_MESSAGING_ACCELERATED_IMAGE_INFO_H_
