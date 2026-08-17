// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_MEMBER_COPY_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_MEMBER_COPY_H_

#include <memory>
#include "base/memory/scoped_refptr.h"
#include "third_party/blink/renderer/core/style/content_data.h"
#include "third_party/blink/renderer/core/style/paint_images.h"

namespace blink {

template <typename T>
scoped_refptr<T> MemberCopy(const scoped_refptr<T>& v) {
  return v;
}

template <typename T>
Member<T> MemberCopy(const Member<T>& v) {
  return v;
}

template <typename T>
std::unique_ptr<T> MemberCopy(const std::unique_ptr<T>& v) {
  return v ? v->Clone() : nullptr;
}

inline Member<ContentData> MemberCopy(const Member<ContentData>& v) {
  return v ? v->Clone() : nullptr;
}

inline Member<PaintImages> MemberCopy(const Member<PaintImages>& v) {
  return v ? v->Clone() : nullptr;
}

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_MEMBER_COPY_H_
