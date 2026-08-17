// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_CUSTOM_CUSTOM_INTRINSIC_SIZES_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_CUSTOM_CUSTOM_INTRINSIC_SIZES_H_

#include "third_party/blink/renderer/core/layout/custom/custom_layout_scope.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"

namespace blink {

class CustomLayoutChild;
class LayoutInputNode;

// This represents the result of intrinsicSizes (on a LayoutChild).
//
// This should mirror the information in a MinMaxSize, and it has the
// additional capability that it is exposed to web developers.
class CustomIntrinsicSizes : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  CustomIntrinsicSizes(CustomLayoutChild*,
                       CustomLayoutToken*,
                       double min_content_size,
                       double max_content_size);
  ~CustomIntrinsicSizes() override = default;

  CustomIntrinsicSizes(const CustomIntrinsicSizes&) = delete;
  CustomIntrinsicSizes& operator=(const CustomIntrinsicSizes&) = delete;

  double minContentSize() const { return min_content_size_; }
  double maxContentSize() const { return max_content_size_; }

  const LayoutInputNode& GetLayoutNode() const;

  bool IsValid() const { return token_->IsValid(); }

  void Trace(Visitor*) const override;

 private:
  Member<CustomLayoutChild> child_;
  Member<CustomLayoutToken> token_;

  // The min and max content sizes on this object should never change.
  const double min_content_size_;
  const double max_content_size_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_CUSTOM_CUSTOM_INTRINSIC_SIZES_H_
