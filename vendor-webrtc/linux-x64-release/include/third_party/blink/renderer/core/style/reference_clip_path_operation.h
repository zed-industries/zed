/*
 * Copyright (C) 2012 Adobe Systems Incorporated. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 *
 * 1. Redistributions of source code must retain the above
 *    copyright notice, this list of conditions and the following
 *    disclaimer.
 * 2. Redistributions in binary form must reproduce the above
 *    copyright notice, this list of conditions and the following
 *    disclaimer in the documentation and/or other materials
 *    provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDER "AS IS" AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER BE
 * LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY,
 * OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR
 * TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF
 * THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
 * SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_REFERENCE_CLIP_PATH_OPERATION_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_REFERENCE_CLIP_PATH_OPERATION_H_

#include "third_party/blink/renderer/core/style/clip_path_operation.h"
#include "third_party/blink/renderer/core/svg/svg_resource.h"
#include "third_party/blink/renderer/platform/heap/persistent.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"
namespace blink {

class SVGResourceClient;

class ReferenceClipPathOperation final : public ClipPathOperation {
 public:
  ReferenceClipPathOperation(const String& url, SVGResource* resource)
      : url_(url), resource_(resource) {}

  void Trace(Visitor* visitor) const override {
    visitor->Trace(resource_);
    ClipPathOperation::Trace(visitor);
  }

  void AddClient(SVGResourceClient&);
  void RemoveClient(SVGResourceClient&);

  bool IsLoading() const;

  SVGResource* Resource() const { return resource_.Get(); }
  const AtomicString& Url() const { return url_; }

 private:
  bool operator==(const ClipPathOperation&) const override;
  OperationType GetType() const override { return kReference; }

  AtomicString url_;
  Member<SVGResource> resource_;
};

template <>
struct DowncastTraits<ReferenceClipPathOperation> {
  static bool AllowFrom(const ClipPathOperation& op) {
    return op.GetType() == ClipPathOperation::kReference;
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_REFERENCE_CLIP_PATH_OPERATION_H_
