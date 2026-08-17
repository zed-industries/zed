// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_FETCH_MULTIPART_PARSER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_FETCH_MULTIPART_PARSER_H_

#include "base/check_op.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/network/http_header_map.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

// This class parses a multipart message which is supplied (possible in chunks)
// to MultipartParser::appendData which parses the data and passes resulting
// part header fields and data to Client.
//
// - MultipartParser::appendData does not accept base64, quoted-printable nor
//   otherwise transfer encoded multipart message parts (no-op transfer
//   encodings "binary", "7bit" and "8bit" are OK).
// - If MultipartParser::cancel() is called, Client's methods will not be
//   called anymore.
class CORE_EXPORT MultipartParser final
    : public GarbageCollected<MultipartParser> {
 public:
  // Client recieves parsed part header fields and data.
  class CORE_EXPORT Client : public GarbageCollectedMixin {
   public:
    virtual ~Client() = default;
    // The method is called whenever header fields of a part are parsed.
    virtual void PartHeaderFieldsInMultipartReceived(
        const HTTPHeaderMap& header_fields) = 0;
    // The method is called whenever some data of a part is parsed which
    // can happen zero or more times per each part. It always holds that
    // |size| > 0.
    virtual void PartDataInMultipartReceived(base::span<const char> bytes) = 0;
    // The method is called whenever all data of a complete part is parsed.
    virtual void PartDataInMultipartFullyReceived() = 0;
    void Trace(Visitor* visitor) const override {}
  };

  MultipartParser(Vector<char> boundary, Client*);
  MultipartParser(const MultipartParser&) = delete;
  MultipartParser& operator=(const MultipartParser&) = delete;
  bool AppendData(base::span<const char> bytes);
  void Cancel();
  bool Finish();

  bool IsCancelled() const { return state_ == State::kCancelled; }

  void Trace(Visitor*) const;

 private:
  class Matcher {
    DISALLOW_NEW();

   public:
    Matcher();
    Matcher(base::span<const char> match_data, size_t num_matched_bytes);

    bool Match(char value) {
      if (value != match_data_[num_matched_bytes_]) {
        return false;
      }
      ++num_matched_bytes_;
      return true;
    }
    bool Match(base::span<const char> data);
    bool IsMatchComplete() const {
      return num_matched_bytes_ == match_data_.size();
    }
    size_t NumMatchedBytes() const { return num_matched_bytes_; }
    void SetNumMatchedBytes(size_t);

    base::span<const char> MatchedData() const {
      return match_data_.first(num_matched_bytes_);
    }

   private:
    base::span<const char> match_data_;
    size_t num_matched_bytes_ = 0u;
  };

  Matcher CloseDelimiterSuffixMatcher() const;
  Matcher DelimiterMatcher(size_t num_already_matched_bytes = 0u) const;
  Matcher DelimiterSuffixMatcher() const;

  void ParseDataAndDelimiter(base::span<const char>& bytes);
  void ParseDelimiter(base::span<const char>& bytes);
  bool ParseHeaderFields(base::span<const char>& bytes,
                         HTTPHeaderMap* header_fields);
  void ParseTransportPadding(base::span<const char>& bytes) const;

  Matcher matcher_;
  Vector<char> buffered_header_bytes_;
  Member<Client> client_;
  Vector<char> delimiter_;

  enum class State {
    kParsingPreamble,
    kParsingDelimiterSuffix,
    kParsingPartHeaderFields,
    kParsingPartOctets,
    kParsingDelimiterOrCloseDelimiterSuffix,
    kParsingCloseDelimiterSuffix,
    kParsingEpilogue,
    kCancelled,
    kFinished
  } state_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_FETCH_MULTIPART_PARSER_H_
