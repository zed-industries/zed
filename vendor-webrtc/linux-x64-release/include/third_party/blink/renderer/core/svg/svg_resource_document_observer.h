// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_RESOURCE_DOCUMENT_OBSERVER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_RESOURCE_DOCUMENT_OBSERVER_H_

#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

class SVGResourceDocumentContent;

// Observer for changes in a SVGResourceDocumentContent.
class SVGResourceDocumentObserver : public GarbageCollectedMixin {
 public:
  virtual ~SVGResourceDocumentObserver() = default;

  // The resource document and any resources referenced by it has finished
  // loading ('load' has fired).
  virtual void ResourceNotifyFinished(SVGResourceDocumentContent*) = 0;

  // Content changed in the external document.
  virtual void ResourceContentChanged(SVGResourceDocumentContent*) = 0;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_RESOURCE_DOCUMENT_OBSERVER_H_
