// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_CSS_RESOURCE_VALUE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_CSS_RESOURCE_VALUE_H_

#include "base/notreached.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/cssom/css_style_value.h"
#include "third_party/blink/renderer/platform/loader/fetch/resource.h"

namespace blink {

class CORE_EXPORT CSSResourceValue : public CSSStyleValue {
 public:
  CSSResourceValue(const CSSResourceValue&) = delete;
  CSSResourceValue& operator=(const CSSResourceValue&) = delete;
  ~CSSResourceValue() override = default;

  const String state() const {
    switch (Status()) {
      case ResourceStatus::kNotStarted:
        return "unloaded";
      case ResourceStatus::kPending:
        return "loading";
      case ResourceStatus::kCached:
        return "loaded";
      case ResourceStatus::kLoadError:
      case ResourceStatus::kDecodeError:
        return "error";
      default:
        NOTREACHED();
    }
  }

  void Trace(Visitor* visitor) const override { CSSStyleValue::Trace(visitor); }

 protected:
  CSSResourceValue() = default;

  virtual ResourceStatus Status() const = 0;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSSOM_CSS_RESOURCE_VALUE_H_
