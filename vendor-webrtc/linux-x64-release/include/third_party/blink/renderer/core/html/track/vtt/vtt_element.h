/*
 * Copyright (C) 2013 Apple Inc.  All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_TRACK_VTT_VTT_ELEMENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_TRACK_VTT_VTT_ELEMENT_H_

#include "third_party/blink/renderer/core/html/html_element.h"
#include "third_party/blink/renderer/core/html/track/text_track.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"

namespace blink {

enum class VttNodeType {
  kNone = 0,
  kClass,
  kItalic,
  kLanguage,
  kBold,
  kUnderline,
  kRuby,
  kRubyText,
  kVoice
};

class VTTElement final : public Element {
 public:
  HTMLElement* CreateEquivalentHTMLElement(Document&);

  VTTElement(VttNodeType, Document*);

  Element& CloneWithoutAttributesAndChildren(Document&) const override;

  VttNodeType GetVttNodeType() const {
    return static_cast<VttNodeType>(vtt_node_type_);
  }

  bool IsPastNode() const { return is_past_node_; }
  void SetIsPastNode(bool);

  bool IsVTTElement() const override { return true; }
  AtomicString Language() const { return language_; }
  void SetLanguage(AtomicString value) { language_ = value; }

  static const QualifiedName& VoiceAttributeName() {
    DEFINE_STATIC_LOCAL(QualifiedName, voice_attr, (AtomicString("voice")));
    return voice_attr;
  }

  static const QualifiedName& LangAttributeName() {
    DEFINE_STATIC_LOCAL(QualifiedName, attr, (AtomicString("lang")));
    return attr;
  }

  const TextTrack* GetTrack() const { return track_.Get(); }

  void SetTrack(TextTrack*);
  void Trace(Visitor*) const override;

 private:
  Member<TextTrack> track_;
  unsigned is_past_node_ : 1;
  const unsigned vtt_node_type_ : 4;

  AtomicString language_;
};

template <>
struct DowncastTraits<VTTElement> {
  static bool AllowFrom(const Node& node) { return node.IsVTTElement(); }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_TRACK_VTT_VTT_ELEMENT_H_
