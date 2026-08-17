// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_PARSER_CSS_TOKENIZER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_PARSER_CSS_TOKENIZER_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/parser/css_parser_token.h"
#include "third_party/blink/renderer/core/css/parser/css_tokenizer_input_stream.h"
#include "third_party/blink/renderer/platform/wtf/text/string_view.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class CSSTokenizerInputStream;

class CORE_EXPORT CSSTokenizer {
  STACK_ALLOCATED();

 public:
  // The StringView must live for at least as long as the CSSTokenizer does
  // (i.e., don't send it a temporary String or similar).
  explicit CSSTokenizer(StringView, wtf_size_t offset = 0);
  CSSTokenizer(const CSSTokenizer&) = delete;
  CSSTokenizer& operator=(const CSSTokenizer&) = delete;

  wtf_size_t TokenCount() const;

  wtf_size_t Offset() const { return input_.Offset(); }
  wtf_size_t PreviousOffset() const { return prev_offset_; }
  StringView StringRangeFrom(wtf_size_t start) const;
  StringView StringRangeAt(wtf_size_t start, wtf_size_t length) const;
  const Vector<String>& StringPool() const { return string_pool_; }
  CSSParserToken TokenizeSingle();
  CSSParserToken TokenizeSingleWithComments();

  // Skips to the given offset, which _must_ be exactly the end of
  // the current block. Does _not_ return a new token for lookahead
  // (because the only caller in question does not want that).
  //
  // Leaves PreviousOffset() in an undefined state.
  void SkipToEndOfBlock(wtf_size_t offset) {
    DCHECK_GT(offset, input_.Offset());
#if DCHECK_IS_ON()
    // Verify that the offset is indeed going to be at the
    // end of the current block.
    wtf_size_t base_nesting_level = block_stack_.size();
    DCHECK_GE(base_nesting_level, 1u);
    while (input_.Offset() < offset - 1) {
      TokenizeSingle();
      DCHECK_GE(block_stack_.size(), base_nesting_level);
    }

    // The last token should be block-closing, and take us exactly
    // to the desired offset and nesting level.
    DCHECK_EQ(input_.Offset(), offset - 1);
    DCHECK_EQ(block_stack_.size(), base_nesting_level);
    TokenizeSingle();
    DCHECK_EQ(input_.Offset(), offset);
    DCHECK_EQ(block_stack_.size(), base_nesting_level - 1);
#else
    // Undo block stack mutation.
    block_stack_.pop_back();
#endif
    input_.Restore(offset);
  }

  // See documentation near CSSParserTokenStream.
  CSSParserToken Restore(const CSSParserToken& next, wtf_size_t offset) {
    // Undo block stack mutation.
    if (next.GetBlockType() == CSSParserToken::BlockType::kBlockStart) {
      block_stack_.pop_back();
    } else if (next.GetBlockType() == CSSParserToken::BlockType::kBlockEnd) {
      static_assert(kLeftParenthesisToken == (kRightParenthesisToken - 1));
      static_assert(kLeftBracketToken == (kRightBracketToken - 1));
      static_assert(kLeftBraceToken == (kRightBraceToken - 1));
      block_stack_.push_back(
          static_cast<CSSParserTokenType>(next.GetType() - 1));
    }
    input_.Restore(offset);
    // Produce the post-restore lookahead token.
    return TokenizeSingle();
  }

 private:
  template <bool SkipComments>
  ALWAYS_INLINE CSSParserToken NextToken();

  UChar Consume();
  void Reconsume(UChar);

  CSSParserToken ConsumeNumericToken();
  CSSParserToken ConsumeIdentLikeToken();
  CSSParserToken ConsumeNumber();
  CSSParserToken ConsumeStringTokenUntil(UChar);
  CSSParserToken ConsumeUnicodeRange();
  CSSParserToken ConsumeUrlToken();

  void ConsumeBadUrlRemnants();
  void ConsumeSingleWhitespaceIfNext();
  void ConsumeUntilCommentEndFound();

  bool ConsumeIfNext(UChar);
  StringView ConsumeName();
  UChar32 ConsumeEscape();

  bool NextTwoCharsAreValidEscape();
  bool NextCharsAreNumber(UChar);
  bool NextCharsAreNumber();
  bool NextCharsAreIdentifier(UChar);
  bool NextCharsAreIdentifier();

  CSSParserToken BlockStart(CSSParserTokenType);
  CSSParserToken BlockStart(CSSParserTokenType block_type,
                            CSSParserTokenType,
                            StringView,
                            CSSValueID id);
  CSSParserToken BlockEnd(CSSParserTokenType, CSSParserTokenType start_type);

  CSSParserToken HyphenMinus(UChar);
  CSSParserToken Hash(UChar);
  CSSParserToken LetterU(UChar);

  StringView RegisterString(const String&);

  CSSTokenizerInputStream input_;
  Vector<CSSParserTokenType, 8> block_stack_;

  // We only allocate strings when escapes are used.
  Vector<String> string_pool_;

  friend class CSSParserTokenStream;

  wtf_size_t prev_offset_ = 0;
  wtf_size_t token_count_ = 0;

  // The unicode-range descriptor (unicode_ranges_allowed=true) invokes
  // a special tokenizer to solve a design mistake in CSS.
  //
  // https://drafts.csswg.org/css-syntax/#consume-unicode-range-value
  bool unicode_ranges_allowed_ = false;
};
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_PARSER_CSS_TOKENIZER_H_
