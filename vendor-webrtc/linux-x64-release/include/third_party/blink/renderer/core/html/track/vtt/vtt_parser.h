/*
 * Copyright (C) 2011 Google Inc.  All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 *     * Redistributions of source code must retain the above copyright
 * notice, this list of conditions and the following disclaimer.
 *     * Redistributions in binary form must reproduce the above
 * copyright notice, this list of conditions and the following disclaimer
 * in the documentation and/or other materials provided with the
 * distribution.
 *     * Neither the name of Google Inc. nor the names of its
 * contributors may be used to endorse or promote products derived from
 * this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_TRACK_VTT_VTT_PARSER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_TRACK_VTT_VTT_PARSER_H_

#include <memory>
#include "third_party/blink/renderer/core/dom/document_fragment.h"
#include "third_party/blink/renderer/core/html/parser/text_resource_decoder.h"
#include "third_party/blink/renderer/core/html/track/vtt/buffered_line_reader.h"
#include "third_party/blink/renderer/core/html/track/vtt/vtt_cue.h"
#include "third_party/blink/renderer/core/html/track/vtt/vtt_tokenizer.h"
#include "third_party/blink/renderer/core/html_names.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/wtf/text/string_builder.h"

namespace blink {

class Document;
class VTTScanner;

class VTTParserClient : public GarbageCollectedMixin {
 public:
  virtual ~VTTParserClient() = default;

  virtual void NewCuesParsed() = 0;
  virtual void FileFailedToParse() = 0;

  void Trace(Visitor* visitor) const override {}
};

// Implementation of the WebVTT parser algorithm.
// https://w3c.github.io/webvtt/#webvtt-parser-algorithm
class VTTParser final : public GarbageCollected<VTTParser> {
 public:
  enum ParseState {
    kInitial,
    kHeader,
    kId,
    kTimingsAndSettings,
    kCueText,
    kRegion,
    kBadCue,
    kStyle
  };

  VTTParser(VTTParserClient*, Document&);
  ~VTTParser() = default;

  static inline bool IsRecognizedTag(const AtomicString& tag_name) {
    return tag_name == html_names::kITag || tag_name == html_names::kBTag ||
           tag_name == html_names::kUTag || tag_name == html_names::kRubyTag ||
           tag_name == html_names::kRtTag;
  }
  static inline bool IsASpace(UChar c) {
    // WebVTT space characters are U+0020 SPACE, U+0009 CHARACTER
    // TABULATION (tab), U+000A LINE FEED (LF), U+000C FORM FEED (FF), and
    // U+000D CARRIAGE RETURN (CR).
    return c == ' ' || c == '\t' || c == '\n' || c == '\f' || c == '\r';
  }
  static inline bool IsValidSettingDelimiter(UChar c) {
    // ... a WebVTT cue consists of zero or more of the following components, in
    // any order, separated from each other by one or more U+0020 SPACE
    // characters or U+0009 CHARACTER TABULATION (tab) characters.
    return c == ' ' || c == '\t';
  }
  static bool CollectTimeStamp(const String&, double& time_stamp);

  // Useful functions for parsing percentage settings.
  static bool ParsePercentageValue(VTTScanner& value_scanner,
                                   double& percentage);
  static bool ParsePercentageValuePair(VTTScanner&, char, gfx::PointF&);

  // Create the DocumentFragment representation of the WebVTT cue text.
  static DocumentFragment* CreateDocumentFragmentFromCueText(Document&,
                                                             const String&,
                                                             TextTrack*);

  // Input data to the parser to parse.
  void ParseBytes(base::span<const char> data);
  void Flush();

  // Transfers ownership of last parsed cues to caller.
  void GetNewCues(HeapVector<Member<TextTrackCue>>&);

  // Transfers ownership of last parsed style sheets to caller.
  void GetNewStyleSheets(HeapVector<Member<CSSStyleSheet>>&);

  void Trace(Visitor*) const;

 private:
  Member<Document> document_;
  ParseState state_;

  void Parse();
  void FlushPendingCue();
  bool HasRequiredFileIdentifier(const String& line);
  ParseState CollectCueId(const String&);
  ParseState CollectTimingsAndSettings(const String&);
  ParseState CollectCueText(const String&);
  ParseState RecoverCue(const String&);
  ParseState IgnoreBadCue(const String&);
  ParseState CollectRegionSettings(const String&);
  ParseState CollectWebVTTBlock(const String&);
  ParseState CheckAndRecoverCue(const String& line);
  ParseState CollectStyleSheet(const String& line);
  bool CheckAndCreateRegion(const String& line);
  bool CheckAndStoreRegion(const String& line);
  void CreateNewCue();
  void ResetCueValues();

  static bool CollectTimeStamp(VTTScanner& input, double& time_stamp);

  BufferedLineReader line_reader_;
  std::unique_ptr<TextResourceDecoder> decoder_;
  AtomicString current_id_;
  double current_start_time_;
  double current_end_time_;
  StringBuilder current_content_;
  String previous_line_;
  String current_settings_;
  Member<VTTRegion> current_region_;
  Member<VTTParserClient> client_;
  HeapVector<Member<CSSStyleSheet>> style_sheets_;
  HeapVector<Member<TextTrackCue>> cue_list_;

  // Used for histogram metric logging only.
  bool contains_style_block_;

  VTTRegionMap region_map_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_TRACK_VTT_VTT_PARSER_H_
