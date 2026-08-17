/*
 * Copyright (C) 2011 Apple Inc. All rights reserved.
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
 * THIS SOFTWARE IS PROVIDED BY APPLE INC. AND ITS CONTRIBUTORS ``AS IS''
 * AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO,
 * THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL APPLE INC. OR ITS CONTRIBUTORS
 * BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
 * CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
 * SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
 * INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
 * CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
 * ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF
 * THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_TEXT_TRACK_LOADER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_TEXT_TRACK_LOADER_H_

#include "third_party/blink/renderer/core/html/track/vtt/vtt_parser.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/loader/fetch/cross_origin_attribute_value.h"
#include "third_party/blink/renderer/platform/loader/fetch/raw_resource.h"

namespace blink {

class Document;
class TextTrackLoader;

class TextTrackLoaderClient : public GarbageCollectedMixin {
 public:
  virtual ~TextTrackLoaderClient() = default;

  virtual void NewCuesAvailable(TextTrackLoader*) = 0;
  virtual void CueLoadingCompleted(TextTrackLoader*, bool loading_failed) = 0;
};

class TextTrackLoader final : public GarbageCollected<TextTrackLoader>,
                              public RawResourceClient,
                              private VTTParserClient {
 public:
  TextTrackLoader(TextTrackLoaderClient&, Document&);
  ~TextTrackLoader() override;

  bool Load(const KURL&, CrossOriginAttributeValue);
  void Detach();

  enum State { kLoading, kFinished, kFailed };
  State LoadState() { return state_; }

  void GetNewCues(HeapVector<Member<TextTrackCue>>& output_cues);
  void GetNewStyleSheets(HeapVector<Member<CSSStyleSheet>>& output_sheets);

  void Trace(Visitor*) const override;

 private:
  // RawResourceClient
  void DataReceived(Resource*, base::span<const char> data) override;
  void NotifyFinished(Resource*) override;
  String DebugName() const override { return "TextTrackLoader"; }

  // VTTParserClient
  void NewCuesParsed() override;
  void FileFailedToParse() override;

  void CancelLoad();
  void CueLoadTimerFired(TimerBase*);
  void CorsPolicyPreventedLoad(const SecurityOrigin*, const KURL&);

  Document& GetDocument() const { return *document_; }

  Member<TextTrackLoaderClient> client_;
  Member<VTTParser> cue_parser_;
  // FIXME: Remove this pointer and get the Document from m_client.
  Member<Document> document_;
  HeapTaskRunnerTimer<TextTrackLoader> cue_load_timer_;
  State state_;
  bool new_cues_available_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LOADER_TEXT_TRACK_LOADER_H_
