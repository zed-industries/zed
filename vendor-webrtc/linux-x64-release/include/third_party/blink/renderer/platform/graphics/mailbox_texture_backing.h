// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#include "third_party/blink/renderer/platform/graphics/paint/paint_image.h"

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_MAILBOX_TEXTURE_BACKING_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_MAILBOX_TEXTURE_BACKING_H_

#include "base/memory/weak_ptr.h"
#include "base/threading/thread_checker.h"
#include "gpu/command_buffer/client/raster_interface.h"
#include "gpu/command_buffer/common/mailbox.h"

namespace blink {
class WebGraphicsContext3DProviderWrapper;
class MailboxRef;

class MailboxTextureBacking : public TextureBacking {
 public:
  explicit MailboxTextureBacking(
      sk_sp<SkImage> sk_image,
      scoped_refptr<MailboxRef> mailbox_ref,
      const gfx::Size& size,
      SkColorType sk_color_type,
      SkAlphaType alpha_type,
      sk_sp<SkColorSpace> sk_color_space,
      base::WeakPtr<WebGraphicsContext3DProviderWrapper>
          context_provider_wrapper);
  explicit MailboxTextureBacking(
      const gpu::Mailbox& mailbox,
      scoped_refptr<MailboxRef> mailbox_ref,
      const gfx::Size& size,
      SkColorType sk_color_type,
      SkAlphaType alpha_type,
      sk_sp<SkColorSpace> sk_color_space,
      base::WeakPtr<WebGraphicsContext3DProviderWrapper>
          context_provider_wrapper);
  ~MailboxTextureBacking() override;
  const SkImageInfo& GetSkImageInfo() override;
  gpu::Mailbox GetMailbox() const override;
  sk_sp<SkImage> GetAcceleratedSkImage() override;
  sk_sp<SkImage> GetSkImageViaReadback() override;
  bool readPixels(const SkImageInfo& dst_info,
                  void* dst_pixels,
                  size_t dst_row_bytes,
                  int src_x,
                  int src_y) override;
  void FlushPendingSkiaOps() override;

 private:
  const sk_sp<SkImage> sk_image_;
  const gpu::Mailbox mailbox_;
  scoped_refptr<MailboxRef> mailbox_ref_;
  const SkImageInfo sk_image_info_;
  base::WeakPtr<WebGraphicsContext3DProviderWrapper> context_provider_wrapper_;
  THREAD_CHECKER(thread_checker_);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_MAILBOX_TEXTURE_BACKING_H_
