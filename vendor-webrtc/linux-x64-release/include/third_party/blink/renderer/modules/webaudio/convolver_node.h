/*
 * Copyright (C) 2010, Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1.  Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2.  Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE INC. AND ITS CONTRIBUTORS ``AS IS'' AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED. IN NO EVENT SHALL APPLE INC. OR ITS CONTRIBUTORS BE LIABLE
 * FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
 * SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
 * CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
 * LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
 * OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH
 * DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_CONVOLVER_NODE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_CONVOLVER_NODE_H_

#include "base/gtest_prod_util.h"
#include "base/memory/scoped_refptr.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/modules/webaudio/audio_node.h"
#include "third_party/blink/renderer/modules/webaudio/convolver_handler.h"
#include "third_party/blink/renderer/platform/heap/persistent.h"

namespace blink {

class AudioBuffer;
class ConvolverOptions;
class ExceptionState;
class Reverb;
class SharedAudioBuffer;

class MODULES_EXPORT ConvolverNode final : public AudioNode {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static ConvolverNode* Create(BaseAudioContext&, ExceptionState&);
  static ConvolverNode* Create(BaseAudioContext*,
                               const ConvolverOptions*,
                               ExceptionState&);

  explicit ConvolverNode(BaseAudioContext&);

  AudioBuffer* buffer() const;
  void setBuffer(AudioBuffer*, ExceptionState&);
  bool normalize() const;
  void setNormalize(bool);

  void Trace(Visitor*) const override;

  // InspectorHelperMixin
  void ReportDidCreate() final;
  void ReportWillBeDestroyed() final;

 private:
  ConvolverHandler& GetConvolverHandler() const;

  Member<AudioBuffer> buffer_;

  FRIEND_TEST_ALL_PREFIXES(ConvolverNodeTest, ReverbLifetime);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBAUDIO_CONVOLVER_NODE_H_
