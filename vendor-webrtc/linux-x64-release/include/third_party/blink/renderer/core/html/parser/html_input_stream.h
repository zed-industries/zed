/*
 * Copyright (C) 2010 Google, Inc. All Rights Reserved.
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
 * THIS SOFTWARE IS PROVIDED BY APPLE INC. ``AS IS'' AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL APPLE INC. OR
 * CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY
 * OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_PARSER_HTML_INPUT_STREAM_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_PARSER_HTML_INPUT_STREAM_H_

#include "third_party/blink/renderer/core/html/parser/input_stream_preprocessor.h"
#include "third_party/blink/renderer/platform/text/segmented_string.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

// The InputStream is made up of a sequence of SegmentedStrings:
//
// [--current--][--next--][--next--] ... [--next--]
//            /\                         (also called last_)
//            L_ current insertion point
//
// The current segmented string is stored in InputStream.  Each of the
// afterInsertionPoint buffers are stored in InsertionPointRecords on the stack.
//
// We remove characters from the "current" string in the InputStream.
// document.write() will add characters at the current insertion point, which
// appends them to the "current" string.
//
// last_ is a pointer to the last of the afterInsertionPoint strings. The
// network adds data at the end of the InputStream, which appends them to the
// "last" string.
class HTMLInputStream {
  DISALLOW_NEW();

 public:
  HTMLInputStream() : last_(&first_) {}
  HTMLInputStream(const HTMLInputStream&) = delete;
  HTMLInputStream& operator=(const HTMLInputStream&) = delete;

  void AppendToEnd(const SegmentedString& string) { last_->Append(string); }

  void InsertAtCurrentInsertionPoint(const SegmentedString& string) {
    first_.Append(string);
  }

  bool HasInsertionPoint() const { return &first_ != last_; }

  void MarkEndOfFile() {
    last_->Append(
        SegmentedString(String(base::span_from_ref(kEndOfFileMarker))));
    last_->Close();
  }

  void CloseWithoutMarkingEndOfFile() { last_->Close(); }

  bool HaveSeenEndOfFile() const { return last_->IsClosed(); }

  SegmentedString& Current() { return first_; }
  const SegmentedString& Current() const { return first_; }

  void SplitInto(SegmentedString& next) {
    next = first_;
    first_ = SegmentedString();
    first_.SetNextSegmentedString(&next);
    if (last_ == &first_) {
      // We used to only have one SegmentedString in the InputStream but now we
      // have two.  That means first_ is no longer also the last_ string,
      // |next| is now the last one.
      last_ = &next;
    }
  }

  void MergeFrom(SegmentedString& next) {
    first_.Append(next);
    first_.SetNextSegmentedString(next.NextSegmentedString());
    if (last_ == &next) {
      // The string |next| used to be the last SegmentedString in
      // the InputStream.  Now that it's been merged into first_,
      // that makes first_ the last one.
      last_ = &first_;
    }
    if (next.IsClosed()) {
      // We also need to merge the "closed" state from next to first_.
      // Arguably, this work could be done in Append().
      first_.Close();
    }
  }

  unsigned length() const {
    unsigned total_length = 0;
    for (const auto* current = &first_; current;
         current = current->NextSegmentedString()) {
      total_length += current->length();
    };
    return total_length;
  }

 private:
  SegmentedString first_;
  SegmentedString* last_;
};

class InsertionPointRecord {
  STACK_ALLOCATED();

 public:
  explicit InsertionPointRecord(HTMLInputStream& input_stream)
      : input_stream_(&input_stream),
        line_(input_stream_->Current().CurrentLine()),
        column_(input_stream_->Current().CurrentColumn()) {
    input_stream_->SplitInto(next_);
    // We 'fork' current position and use it for the generated script part. This
    // is a bit weird, because generated part does not have positions within an
    // HTML document.
    input_stream_->Current().SetCurrentPosition(line_, column_, 0);
  }

  InsertionPointRecord(const InsertionPointRecord&) = delete;
  InsertionPointRecord& operator=(const InsertionPointRecord&) = delete;

  ~InsertionPointRecord() {
    // Some inserted text may have remained in input stream. E.g. if script has
    // written "&amp" or "<table", it stays in buffer because it cannot be
    // properly tokenized before we see next part.
    int unparsed_remainder_length = input_stream_->Current().length();
    input_stream_->MergeFrom(next_);
    // We restore position for the character that goes right after unparsed
    // remainder.
    input_stream_->Current().SetCurrentPosition(line_, column_,
                                                unparsed_remainder_length);
  }

 private:
  HTMLInputStream* input_stream_;
  SegmentedString next_;
  OrdinalNumber line_;
  OrdinalNumber column_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_PARSER_HTML_INPUT_STREAM_H_
