// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_HTML_PICTURE_ELEMENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_HTML_PICTURE_ELEMENT_H_

#include "third_party/blink/renderer/core/html/html_element.h"

namespace blink {

// Description of a change to a <source> element.
enum class ImageSourceChangeType {
  // A <source> element was added.
  kAdded,
  // A <source> element was removed.
  kRemoved,
  // An attribute of a <source> element changed.
  kAttribute,
  // The 'media' condition of a <source> element changed.
  kMedia,
};

class HTMLPictureElement final : public HTMLElement {
  DEFINE_WRAPPERTYPEINFO();

 public:
  explicit HTMLPictureElement(Document&);

  void SourceChanged(ImageSourceChangeType);
  void SourceDimensionChanged();
  void RemoveListenerFromSourceChildren();
  void AddListenerToSourceChildren();

 private:
  InsertionNotificationRequest InsertedInto(ContainerNode&) override;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_HTML_PICTURE_ELEMENT_H_
