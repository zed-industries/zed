// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_SPEECH_SPEECH_SYNTHESIS_BASE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_SPEECH_SPEECH_SYNTHESIS_BASE_H_

#include "base/functional/callback.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace WTF {
class String;
}  // namespace WTF

namespace blink {

class LocalDOMWindow;

// SpeechSynthesisBase is the parent class of SpeechSynthesis in /modules. The
// primary function of this class is to make SpeechSynthesis methods accessible
// in /core. /core is where all audio description handling occurs.
class CORE_EXPORT SpeechSynthesisBase : public GarbageCollectedMixin {
 public:
  using OnSpeakingCompletedCallback = base::RepeatingCallback<void()>;

  SpeechSynthesisBase(const SpeechSynthesisBase&) = delete;
  SpeechSynthesisBase& operator=(const SpeechSynthesisBase&) = delete;
  ~SpeechSynthesisBase() = default;

  typedef SpeechSynthesisBase* (*SpeechSynthesisBaseCreateFunction)(
      LocalDOMWindow&);

  // GarbageCollectedMixin
  void Trace(Visitor* visitor) const override {}

  // Sets create_function_.
  // Init is intended to be called in modules_initializer.cc, where
  // SpeechSynthesisBase has access to a create function defined in
  // /modules/../speech_synthesis.cc.
  static void Init(SpeechSynthesisBaseCreateFunction function);

  // Uses create_function_ set in Init to instantiate a new SpeechSynthesisBase
  // object. create_function_ must take in a LocalDOMWindow, so Create also
  // takes in LocalDOMWindow.
  // Create is intended to be called in /core.
  static SpeechSynthesisBase* Create(LocalDOMWindow& window);

  // Overridden in speech_synthesis.cc.
  virtual void Speak(const WTF::String& text, const WTF::String& lang) = 0;
  virtual void Cancel() = 0;
  virtual void Pause() = 0;
  virtual void Resume() = 0;
  virtual bool Speaking() const = 0;

  void SetOnSpeakingCompletedCallback(OnSpeakingCompletedCallback callback);

  // Calls on_speaking_completed_callback_ when audio description is finished
  // speaking. Run in SpeechSynthesis::HandleSpeakingCompleted.
  void HandleSpeakingCompleted();

 protected:
  SpeechSynthesisBase() = default;

 private:
  // Creates a SpeechSynthesis object returned as type SpeechSynthesisBase for
  // use in /core.
  static SpeechSynthesisBaseCreateFunction create_function_;

  // Restarts the video once if it was paused by VTTCue::OnExit.
  OnSpeakingCompletedCallback on_speaking_completed_callback_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_SPEECH_SPEECH_SYNTHESIS_BASE_H_
