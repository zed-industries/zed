// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_ENCRYPTEDMEDIA_HTML_MEDIA_ELEMENT_ENCRYPTED_MEDIA_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_ENCRYPTEDMEDIA_HTML_MEDIA_ELEMENT_ENCRYPTED_MEDIA_H_

#include "third_party/blink/public/platform/web_media_player_encrypted_media_client.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/core/dom/events/event_target.h"
#include "third_party/blink/renderer/core/event_type_names.h"
#include "third_party/blink/renderer/core/html/media/html_media_element.h"
#include "third_party/blink/renderer/core/typed_arrays/dom_typed_array.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/supplementable.h"

namespace media {
enum class EmeInitDataType;
}

namespace blink {

class ExceptionState;
class HTMLMediaElement;
class MediaKeys;
class ScriptState;
class WebContentDecryptionModule;

class MODULES_EXPORT HTMLMediaElementEncryptedMedia final
    : public GarbageCollected<HTMLMediaElementEncryptedMedia>,
      public Supplement<HTMLMediaElement>,
      public WebMediaPlayerEncryptedMediaClient {
 public:
  static const char kSupplementName[];

  static MediaKeys* mediaKeys(HTMLMediaElement&);
  static ScriptPromise<IDLUndefined> setMediaKeys(ScriptState*,
                                                  HTMLMediaElement&,
                                                  MediaKeys*,
                                                  ExceptionState&);
  DEFINE_STATIC_ATTRIBUTE_EVENT_LISTENER(encrypted, kEncrypted)
  DEFINE_STATIC_ATTRIBUTE_EVENT_LISTENER(waitingforkey, kWaitingforkey)

  // WebMediaPlayerEncryptedMediaClient methods
  void Encrypted(media::EmeInitDataType init_data_type,
                 const unsigned char* init_data,
                 unsigned init_data_length) final;
  void DidBlockPlaybackWaitingForKey() final;
  void DidResumePlaybackBlockedForKey() final;
  WebContentDecryptionModule* ContentDecryptionModule();

  static HTMLMediaElementEncryptedMedia& From(HTMLMediaElement&);

  HTMLMediaElementEncryptedMedia(HTMLMediaElement&);
  ~HTMLMediaElementEncryptedMedia();

  void Trace(Visitor*) const override;

 private:
  friend class SetMediaKeysHandler;

  // EventTarget
  bool SetAttributeEventListener(const AtomicString& event_type,
                                 EventListener*);
  EventListener* GetAttributeEventListener(const AtomicString& event_type);

  // Internal values specified by the EME spec:
  // http://w3c.github.io/encrypted-media/#idl-def-HTMLMediaElement
  // The following internal values are added to the HTMLMediaElement:
  // - waiting for key, which shall have a boolean value
  // - attaching media keys, which shall have a boolean value
  bool is_waiting_for_key_;
  bool is_attaching_media_keys_;

  Member<MediaKeys> media_keys_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_ENCRYPTEDMEDIA_HTML_MEDIA_ELEMENT_ENCRYPTED_MEDIA_H_
