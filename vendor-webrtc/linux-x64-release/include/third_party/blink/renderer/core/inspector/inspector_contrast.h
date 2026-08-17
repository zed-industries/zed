// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_INSPECTOR_CONTRAST_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_INSPECTOR_CONTRAST_H_

#include "cc/base/rtree.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/document.h"

namespace blink {

struct CORE_EXPORT ContrastInfo {
  STACK_ALLOCATED();

 public:
  Element* element;
  bool able_to_compute_contrast = false;
  float threshold_aa;
  float threshold_aaa;
  float contrast_ratio;
  String font_size;
  String font_weight;
};

struct CORE_EXPORT TextInfo {
  STACK_ALLOCATED();

 public:
  String font_size;
  String font_weight;
};

// Calculates the contrast of elements in a document.
class CORE_EXPORT InspectorContrast {
  STACK_ALLOCATED();

 public:
  explicit InspectorContrast(Document*);
  ContrastInfo GetContrast(Element*);
  std::vector<ContrastInfo> GetElementsWithContrastIssues(bool report_aaa,
                                                          size_t max_elements);
  Vector<Color> GetBackgroundColors(Element*, float* text_opacity);
  TextInfo GetTextInfo(Element*);

 private:
  void SortElementsByPaintOrder(HeapVector<Member<Node>>&, Document*);
  HeapVector<Member<Node>> ElementsFromRect(const PhysicalRect& rect,
                                            Document& document);
  bool GetColorsFromRect(PhysicalRect rect,
                         Document& document,
                         Element* top_element,
                         Vector<Color>& colors,
                         float* text_opacity);
  void CollectNodesAndBuildRTreeIfNeeded();

  // It is safe to keep raw pointers to Node because rtree_
  // only operates on nodes retained in the elements_ HeapVector below.
  // See InspectorContrast::CollectNodesAndBuildRTreeIfNeeded and
  // crbug.com/1222445.
  cc::RTree<Node*> rtree_;
  HeapVector<Member<Node>> elements_;
  Document* document_;
  bool rtree_built_ = false;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_INSPECTOR_INSPECTOR_CONTRAST_H_
