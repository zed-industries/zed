/*
 * Copyright (C) 2011 Apple Inc.  All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE COMPUTER, INC. ``AS IS'' AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL APPLE COMPUTER, INC. OR
 * CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY
 * OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_TRACK_TRACK_BASE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_TRACK_TRACK_BASE_H_

#include "third_party/blink/public/platform/web_media_player.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/supplementable.h"
#include "third_party/blink/renderer/platform/wtf/text/atomic_string.h"

namespace blink {

class HTMLMediaElement;

class CORE_EXPORT TrackBase : public Supplementable<TrackBase> {
 public:
  virtual ~TrackBase();

  String id() const { return id_; }

  WebMediaPlayer::TrackType GetType() const { return type_; }

  AtomicString label() const { return label_; }
  AtomicString language() const { return language_; }

  void SetMediaElement(HTMLMediaElement* media_element) {
    media_element_ = media_element;
  }
  HTMLMediaElement* MediaElement() const { return media_element_.Get(); }

  void Trace(Visitor*) const override;

 protected:
  TrackBase(WebMediaPlayer::TrackType,
            const AtomicString& label,
            const AtomicString& language,
            const String& id);

  WebMediaPlayer::TrackType type_;
  AtomicString label_;
  AtomicString language_;
  String id_;
  Member<HTMLMediaElement> media_element_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_TRACK_TRACK_BASE_H_
