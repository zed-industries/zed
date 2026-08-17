// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASOURCE_TRACK_DEFAULT_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASOURCE_TRACK_DEFAULT_H_

#include "third_party/blink/renderer/bindings/core/v8/script_value.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_track_default_type.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class ExceptionState;
class ScriptState;

class TrackDefault final : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static AtomicString AudioKeyword();
  static AtomicString VideoKeyword();
  static AtomicString TextKeyword();

  static TrackDefault* Create(const V8TrackDefaultType& type,
                              const String& language,
                              const String& label,
                              const Vector<String>& kinds,
                              const String& byte_stream_track_id,
                              ExceptionState&);

  TrackDefault(const V8TrackDefaultType& type,
               const String& language,
               const String& label,
               const Vector<String>& kinds,
               const String& byte_stream_track_id);
  ~TrackDefault() override;

  // Implement the IDL
  V8TrackDefaultType type() const { return type_; }
  String byteStreamTrackID() const { return byte_stream_track_id_; }
  String language() const { return language_; }
  String label() const { return label_; }
  ScriptObject kinds(ScriptState*) const;

 private:
  const V8TrackDefaultType type_;
  const String byte_stream_track_id_;
  const String language_;
  const String label_;
  const Vector<String> kinds_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_MEDIASOURCE_TRACK_DEFAULT_H_
