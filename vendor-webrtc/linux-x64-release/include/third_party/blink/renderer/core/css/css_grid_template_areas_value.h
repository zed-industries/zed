/*
 * Copyright (C) 2013 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 *     * Redistributions of source code must retain the above copyright
 * notice, this list of conditions and the following disclaimer.
 *     * Redistributions in binary form must reproduce the above
 * copyright notice, this list of conditions and the following disclaimer
 * in the documentation and/or other materials provided with the
 * distribution.
 *     * Neither the name of Google Inc. nor the names of its
 * contributors may be used to endorse or promote products derived from
 * this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_GRID_TEMPLATE_AREAS_VALUE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_GRID_TEMPLATE_AREAS_VALUE_H_

#include "third_party/blink/renderer/core/css/css_value.h"
#include "third_party/blink/renderer/core/style/grid_area.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"
#include "third_party/blink/renderer/platform/wtf/text/string_hash.h"

namespace blink {
namespace cssvalue {

class CSSGridTemplateAreasValue : public CSSValue {
 public:
  CSSGridTemplateAreasValue(const NamedGridAreaMap&,
                            wtf_size_t row_count,
                            wtf_size_t column_count);
  ~CSSGridTemplateAreasValue() = default;

  String CustomCSSText() const;

  const NamedGridAreaMap& GridAreaMap() const { return grid_area_map_; }
  wtf_size_t RowCount() const { return row_count_; }
  wtf_size_t ColumnCount() const { return column_count_; }

  bool Equals(const CSSGridTemplateAreasValue&) const;

  void TraceAfterDispatch(blink::Visitor* visitor) const {
    CSSValue::TraceAfterDispatch(visitor);
  }

 private:
  NamedGridAreaMap grid_area_map_;
  wtf_size_t row_count_;
  wtf_size_t column_count_;
};

}  // namespace cssvalue

template <>
struct DowncastTraits<cssvalue::CSSGridTemplateAreasValue> {
  static bool AllowFrom(const CSSValue& value) {
    return value.IsGridTemplateAreasValue();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_GRID_TEMPLATE_AREAS_VALUE_H_
