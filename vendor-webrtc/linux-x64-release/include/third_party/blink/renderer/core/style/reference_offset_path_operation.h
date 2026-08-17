// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_REFERENCE_OFFSET_PATH_OPERATION_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_REFERENCE_OFFSET_PATH_OPERATION_H_

#include "third_party/blink/renderer/core/style/offset_path_operation.h"
#include "third_party/blink/renderer/core/svg/svg_resource.h"
#include "third_party/blink/renderer/platform/heap/persistent.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"

namespace blink {

class SVGResourceClient;

class ReferenceOffsetPathOperation final : public OffsetPathOperation {
 public:
  ReferenceOffsetPathOperation(const String& url,
                               SVGResource* resource,
                               CoordBox coord_box)
      : OffsetPathOperation(coord_box), url_(url), resource_(resource) {}

  void Trace(Visitor* visitor) const override {
    visitor->Trace(resource_);
    OffsetPathOperation::Trace(visitor);
  }

  bool IsEqualAssumingSameType(const OffsetPathOperation& o) const override {
    const auto& other = To<ReferenceOffsetPathOperation>(o);
    return resource_ == other.resource_ && url_ == other.url_;
  }

  OperationType GetType() const override { return kReference; }

  void AddClient(SVGResourceClient& client) {
    if (resource_) {
      resource_->AddClient(client);
    }
  }
  void RemoveClient(SVGResourceClient& client) {
    if (resource_) {
      resource_->RemoveClient(client);
    }
  }

  SVGResource* Resource() const { return resource_.Get(); }
  const AtomicString& Url() const { return url_; }

 private:
  AtomicString url_;
  Member<SVGResource> resource_;
};

template <>
struct DowncastTraits<ReferenceOffsetPathOperation> {
  static bool AllowFrom(const OffsetPathOperation& op) {
    return op.GetType() == OffsetPathOperation::kReference;
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_REFERENCE_OFFSET_PATH_OPERATION_H_
