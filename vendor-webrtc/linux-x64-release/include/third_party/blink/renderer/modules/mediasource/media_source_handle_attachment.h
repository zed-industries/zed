// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASOURCE_MEDIA_SOURCE_HANDLE_ATTACHMENT_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASOURCE_MEDIA_SOURCE_HANDLE_ATTACHMENT_H_

#include "base/memory/scoped_refptr.h"
#include "third_party/blink/renderer/bindings/core/v8/serialization/serialized_script_value.h"
#include "third_party/blink/renderer/modules/mediasource/handle_attachment_provider.h"
#include "third_party/blink/renderer/modules/modules_export.h"

namespace blink {

// Used to serialize MediaSourceHandle
class MODULES_EXPORT MediaSourceHandleAttachment
    : public SerializedScriptValue::Attachment {
 public:
  // Internals of a MediaSourceHandle that are included in serialization.
  struct HandleInternals {
    scoped_refptr<HandleAttachmentProvider> attachment_provider;
    String internal_blob_url;
  };

  using MediaSourceHandleVector = Vector<HandleInternals>;

  static const void* const kAttachmentKey;
  MediaSourceHandleAttachment();
  ~MediaSourceHandleAttachment() override;

  bool IsLockedToAgentCluster() const override { return !attachments_.empty(); }

  size_t size() const { return attachments_.size(); }

  MediaSourceHandleVector& Attachments() { return attachments_; }

  const MediaSourceHandleVector& Attachments() const { return attachments_; }

 private:
  MediaSourceHandleVector attachments_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASOURCE_MEDIA_SOURCE_HANDLE_ATTACHMENT_H_
