// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_MEDIA_AUDIO_OUTPUT_DEVICE_CONTROLLER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_MEDIA_AUDIO_OUTPUT_DEVICE_CONTROLLER_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/html/media/html_media_element.h"
#include "third_party/blink/renderer/platform/supplementable.h"

namespace blink {

class CORE_EXPORT AudioOutputDeviceController
    : public Supplement<HTMLMediaElement> {
 public:
  static const char kSupplementName[];

  static AudioOutputDeviceController* From(HTMLMediaElement&);

  virtual void SetSinkId(const String&) = 0;

  void Trace(Visitor*) const override;

 protected:
  explicit AudioOutputDeviceController(HTMLMediaElement&);

  // To be called by the implementation to register itself.
  static void ProvideTo(HTMLMediaElement&, AudioOutputDeviceController*);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_MEDIA_AUDIO_OUTPUT_DEVICE_CONTROLLER_H_
