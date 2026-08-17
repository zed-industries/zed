// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_DOM_DOCUMENT_PARSER_TIMING_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_DOM_DOCUMENT_PARSER_TIMING_H_

#include "base/time/time.h"
#include "third_party/blink/renderer/core/dom/document.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/supplementable.h"

namespace blink {

// DocumentParserTiming is responsible for tracking parser-related timings for a
// given document.
class DocumentParserTiming final
    : public GarbageCollected<DocumentParserTiming>,
      public Supplement<Document> {
 public:
  static const char kSupplementName[];

  explicit DocumentParserTiming(Document&);
  DocumentParserTiming(const DocumentParserTiming&) = delete;
  DocumentParserTiming& operator=(const DocumentParserTiming&) = delete;
  ~DocumentParserTiming() = default;

  static DocumentParserTiming& From(Document&);

  // markParserStart and markParserStop methods record the time that the
  // parser was first started/stopped, and notify that the document parser
  // timing has changed. These methods do nothing (early return) if a time has
  // already been recorded for the given parser event, or if a parser has
  // already been detached.
  void MarkParserStart();
  void MarkParserStop();

  // markParserDetached records that the parser is detached from the
  // document. A single document may have multiple parsers, if e.g. the
  // document is re-opened using document.write. DocumentParserTiming only
  // wants to record parser start and stop time for the first parser. To avoid
  // recording parser start and stop times for re-opened documents, we keep
  // track of whether a parser has been detached, and avoid recording
  // start/stop times for subsequent parsers, after the first parser has been
  // detached.
  void MarkParserDetached();

  // Record a duration of time that the parser yielded due to loading a
  // script. scriptInsertedViaDocumentWrite indicates whether the
  // script causing blocking was inserted via document.write. This may be
  // called multiple times, once for each time the parser yields on a script
  // load.
  void RecordParserBlockedOnScriptLoadDuration(
      base::TimeDelta duration,
      bool script_inserted_via_document_write);

  // Record a duration of time that the parser spent executing a script.
  // scriptInsertedViaDocumentWrite indicates whether the script being executed
  // was inserted via document.write. This may be called multiple times, once
  // for each time the parser executes a script.
  void RecordParserBlockedOnScriptExecutionDuration(
      base::TimeDelta duration,
      bool script_inserted_via_document_write);

  // The getters below return monotonically-increasing time, or zero if the
  // given parser event has not yet occurred.

  base::TimeTicks ParserStart() const { return parser_start_; }
  base::TimeTicks ParserStop() const { return parser_stop_; }

  // Returns the sum of all blocking script load durations reported via
  // recordParseBlockedOnScriptLoadDuration.
  base::TimeDelta ParserBlockedOnScriptLoadDuration() const {
    return parser_blocked_on_script_load_duration_;
  }

  // Returns the sum of all blocking script load durations due to
  // document.write reported via recordParseBlockedOnScriptLoadDuration. Note
  // that some uncommon cases are not currently covered by this method. See
  // crbug/600711 for details.
  base::TimeDelta ParserBlockedOnScriptLoadFromDocumentWriteDuration() const {
    return parser_blocked_on_script_load_from_document_write_duration_;
  }

  // Returns the sum of all script execution durations reported via
  // recordParseBlockedOnScriptExecutionDuration.
  base::TimeDelta ParserBlockedOnScriptExecutionDuration() const {
    return parser_blocked_on_script_execution_duration_;
  }

  // Returns the sum of all script execution durations due to
  // document.write reported via recordParseBlockedOnScriptExecutionDuration.
  // Note that some uncommon cases are not currently covered by this method. See
  // crbug/600711 for details.
  base::TimeDelta ParserBlockedOnScriptExecutionFromDocumentWriteDuration()
      const {
    return parser_blocked_on_script_execution_from_document_write_duration_;
  }

  void Trace(Visitor*) const override;

 private:
  void NotifyDocumentParserTimingChanged();

  base::TimeTicks parser_start_;
  base::TimeTicks parser_stop_;
  base::TimeDelta parser_blocked_on_script_load_duration_;
  base::TimeDelta parser_blocked_on_script_load_from_document_write_duration_;
  base::TimeDelta parser_blocked_on_script_execution_duration_;
  base::TimeDelta
      parser_blocked_on_script_execution_from_document_write_duration_;
  bool parser_detached_ = false;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_DOM_DOCUMENT_PARSER_TIMING_H_
