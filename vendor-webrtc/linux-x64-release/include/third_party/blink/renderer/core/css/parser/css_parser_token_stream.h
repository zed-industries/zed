// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_PARSER_CSS_PARSER_TOKEN_STREAM_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_PARSER_CSS_PARSER_TOKEN_STREAM_H_

// CSSParserTokenStream is the main interface to everything related to CSS
// streaming. It provides an interface to the token with a one-token lookahead,
// i.e., you can not only call Consume() and get a token back, but also Peek()
// to see what the next Consume() would be without actually consuming it. Nearly
// all of our parsers are standard recursive-descent parsers, where you first
// Peek() to see what type of situation you are dealing with, choose your path
// and then Consume().
//
// Most of CSS is written so that one-token lookahead is sufficient to parse;
// however, there are many exceptions. If one-token lookahead is not enough for
// you, you will need to set a savepoint (using Save()), so that you can rewind
// if you have consumed multiple tokens and then figured afterwards that you are
// in the wrong sub-grammar. Restarting will cause duplicated tokenization work
// and thus reduced performance, so it should generally be avoided when
// possible.
//
// Blocks (parens, brackets, braces and functions) are dealt with specially.
// Generally the pattern is to first establish that you are about to enter a
// block (using Peek()), and then set up a BlockGuard (see below). At this
// point, the stream essentially becomes a sub-parser of itself just with the
// same name, and descends into the block. (Calling Consume() on a block-start
// or block-end token is disallowed and will CHECK-fail, which is why you should
// never call Consume() without knowing what kind of token you are about to
// consume.) Once the BlockGuard goes out of scope, the stream fast-forwards to
// the end of the block and past the block-end token. Abstractly, the stream
// ends at either EOF or the beginning/end of a block. Internally, the stream
// keeps a “block stack” to know which end-of-block tokens actually correspond
// to blocks we have descended into.

#include <memory>

#include "base/auto_reset.h"
#include "base/check_op.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/parser/css_tokenizer.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"
#include "third_party/blink/renderer/platform/wtf/wtf_size_t.h"

namespace blink {

namespace detail {

template <typename...>
bool IsTokenTypeOneOf(CSSParserTokenType t) {
  return false;
}

template <CSSParserTokenType Head, CSSParserTokenType... Tail>
bool IsTokenTypeOneOf(CSSParserTokenType t) {
  return t == Head || IsTokenTypeOneOf<Tail...>(t);
}

}  // namespace detail

// Methods prefixed with "Unchecked" can only be called after calls to Peek(),
// EnsureLookAhead(), or AtEnd() with no subsequent modifications to the stream
// such as a consume.
class CORE_EXPORT CSSParserTokenStream {
  STACK_ALLOCATED();

 public:
  // Instantiate this to start reading from a block. When the guard is out of
  // scope, the rest of the block is consumed.
  class BlockGuard {
    STACK_ALLOCATED();

   public:
    explicit BlockGuard(CSSParserTokenStream& stream)
        : stream_(stream), boundaries_(stream.boundaries_) {
      const CSSParserToken next = stream.ConsumeInternal();
      DCHECK_EQ(next.GetBlockType(), CSSParserToken::kBlockStart);
      // Boundaries do not apply within blocks.
      stream.boundaries_ = FlagForTokenType(kEOFToken);
    }

    void SkipToEndOfBlock() {
      DCHECK(!skipped_to_end_of_block_);
      stream_.EnsureLookAhead();
      stream_.UncheckedSkipToEndOfBlock();
      skipped_to_end_of_block_ = true;
    }

    ~BlockGuard() {
      if (!skipped_to_end_of_block_) {
        SkipToEndOfBlock();
      }
      stream_.boundaries_ = boundaries_;
    }

   private:
    CSSParserTokenStream& stream_;
    bool skipped_to_end_of_block_ = false;
    uint64_t boundaries_;
  };

  static constexpr uint64_t FlagForTokenType(CSSParserTokenType token_type) {
    return 1ull << static_cast<uint64_t>(token_type);
  }

  // The specified token type will be treated as kEOF while the Boundary is on
  // the stack. However, this does not apply within blocks. For example, if the
  // current boundary is kSemicolonToken, the parsing following will only treat
  // the ';' between 'b' and 'c' as kEOF, because the other ';'-tokens are
  // within a block:
  //
  //  a[1;2;3]b;c
  //
  class Boundary {
    STACK_ALLOCATED();

   public:
    Boundary(CSSParserTokenStream& stream, CSSParserTokenType boundary_type)
        : auto_reset_(&stream.boundaries_,
                      stream.boundaries_ | FlagForTokenType(boundary_type)) {}
    ~Boundary() = default;

   private:
    base::AutoReset<uint64_t> auto_reset_;
  };

  // While EnableUnicodeRanges is true, we invoke a special tokenizer to solve
  // a design mistake in CSS.
  //
  // https://drafts.csswg.org/css-syntax/#consume-unicode-range-value
  class EnableUnicodeRanges {
    STACK_ALLOCATED();

   public:
    explicit EnableUnicodeRanges(CSSParserTokenStream& stream,
                                 bool unicode_ranges_allowed)
        : stream_(stream),
          old_unicode_ranges_allowed_(
              stream.tokenizer_.unicode_ranges_allowed_) {
      stream.tokenizer_.unicode_ranges_allowed_ = unicode_ranges_allowed;
      stream.RetokenizeLookAhead();
    }
    ~EnableUnicodeRanges() {
      stream_.tokenizer_.unicode_ranges_allowed_ = old_unicode_ranges_allowed_;
      stream_.RetokenizeLookAhead();
    }

   private:
    CSSParserTokenStream& stream_;
    const bool old_unicode_ranges_allowed_;
  };

  explicit CSSParserTokenStream(StringView text, wtf_size_t offset = 0)
      : tokenizer_(text, offset), next_(kEOFToken) {}

  CSSParserTokenStream(
      StringView text,
      const Vector<std::pair<wtf_size_t, wtf_size_t>>* attr_taint_ranges)
      : tokenizer_(text, 0),
        next_(kEOFToken),
        attr_taint_ranges_(attr_taint_ranges) {}
  CSSParserTokenStream(CSSParserTokenStream&&) = delete;
  CSSParserTokenStream(const CSSParserTokenStream&) = delete;
  CSSParserTokenStream& operator=(const CSSParserTokenStream&) = delete;

  bool IsAttrTainted(wtf_size_t start_offset, wtf_size_t end_offset) {
    if (!attr_taint_ranges_) {
      return false;
    }
    for (auto [tainted_start, tainted_end] : *attr_taint_ranges_) {
      int64_t end = std::min(end_offset, tainted_end);
      int64_t start = std::max(start_offset, tainted_start);
      if (end - start > 0) {
        return true;
      }
    }
    return false;
  }

  inline void EnsureLookAhead() {
    if (!HasLookAhead()) {
      LookAhead();
    }
  }

  // Forcibly read a lookahead token.
  inline void LookAhead() {
    DCHECK(!HasLookAhead());
    next_ = tokenizer_.TokenizeSingle();
#if DCHECK_IS_ON()
    peeked_at_next_ = false;
#endif
    has_look_ahead_ = true;
  }

  inline bool HasLookAhead() const { return has_look_ahead_; }

  inline const CSSParserToken& Peek() {
    EnsureLookAhead();
    return UncheckedPeek();
  }

  // Skips to the given offset, which _must_ be exactly the end of
  // the current block. Leaves the stream explicitly without lookahead
  // (because the only caller in question wants it so).
  //
  // See FindLengthOfDeclarationList() for how to get a value for
  // “bytes” quickly.
  inline void SkipToEndOfBlock(wtf_size_t bytes) {
    DCHECK(HasLookAhead());
    DCHECK_EQ(next_.GetBlockType(), CSSParserToken::BlockType::kBlockStart);

    tokenizer_.SkipToEndOfBlock(LookAheadOffset() + bytes);
    offset_ = tokenizer_.Offset();
    has_look_ahead_ = false;
  }

  inline const CSSParserToken& UncheckedPeek() const {
    DCHECK(HasLookAhead());
#if DCHECK_IS_ON()
    peeked_at_next_ = true;
#endif
    return next_;
  }

  inline const CSSParserToken& Consume() {
    EnsureLookAhead();
    return UncheckedConsume();
  }

  const CSSParserToken& UncheckedConsume() {
    DCHECK(HasLookAhead());
    DCHECK_NE(next_.GetBlockType(), CSSParserToken::kBlockStart);
    DCHECK_NE(next_.GetBlockType(), CSSParserToken::kBlockEnd);

#if DCHECK_IS_ON()
    // This isn't a fool-proof check, but will catch most abuses.
    DCHECK(peeked_at_next_)
        << "You blindly called Consume() without checking the token first, "
        << "thus risking that it's a block-start or block-end token "
        << "if the input data happened to contain one";
#endif

    has_look_ahead_ = false;
    offset_ = tokenizer_.Offset();
    return next_;
  }

  // Allows you to read past the end of blocks without recursing,
  // which is normally not what you want to do when parsing
  // (it is really only useful during variable substitution).
  inline const CSSParserToken& ConsumeRaw() {
    EnsureLookAhead();
    return UncheckedConsumeRaw();
  }

  // See ConsumeRaw().
  const CSSParserToken& UncheckedConsumeRaw() {
    DCHECK(HasLookAhead());
    has_look_ahead_ = false;
    offset_ = tokenizer_.Offset();
    return next_;
  }

  inline bool AtEnd() {
    EnsureLookAhead();
    return UncheckedAtEnd();
  }

  inline bool UncheckedAtEnd() const {
    DCHECK(HasLookAhead());
    return (boundaries_ & FlagForTokenType(next_.GetType())) ||
           next_.GetBlockType() == CSSParserToken::kBlockEnd;
  }

  // Get the index of the character in the original string to be consumed next.
  wtf_size_t Offset() const { return offset_; }

  // Get the index of the starting character of the look-ahead token.
  wtf_size_t LookAheadOffset() const {
    DCHECK(HasLookAhead());
    return tokenizer_.PreviousOffset();
  }

  // Returns a view on a range of characters in the original string.
  StringView StringRangeAt(wtf_size_t start, wtf_size_t length) const;

  // Returns a view on the string that has not been yet consumed.
  // (The lookahead token, if any, does not count as consumed.)
  StringView RemainingText() const;

  void ConsumeWhitespace();
  CSSParserToken ConsumeIncludingWhitespace();
  CSSParserToken ConsumeIncludingWhitespaceRaw();  // See ConsumeRaw().

  // Either consumes a comment token and returns true, or peeks at the next
  // token and return false.
  bool ConsumeCommentOrNothing();

  // Skip tokens until one of these is true:
  //
  //  - EOF is reached.
  //  - The next token would signal a premature end of the current block
  //    (an unbalanced } or similar).
  //  - The next token is of any of the given types, except if it occurs
  //    within a block.
  //
  // The tokens that we consume are discarded. So e.g., if we ask
  // to stop at semicolons, and the rest of the input looks like
  // “.foo { color; } bar ; baz”, we would skip “.foo { color; } bar ”
  // and stop there (the semicolon would remain in the lookahead slot).
  template <CSSParserTokenType... Types>
  void SkipUntilPeekedTypeIs() {
    EnsureLookAhead();

    // Check if the existing lookahead token already marks the end;
    // if so, try to exit as soon as possible. (This is a fairly common
    // case, because some places call SkipUntilPeekedTypeIs() just to
    // ignore garbage after a declaration, and there usually is no such
    // garbage.)
    if (next_.IsEOF() || TokenMarksEnd<Types...>(next_)) {
#if DCHECK_IS_ON()
      // We know what type this is now.
      peeked_at_next_ = true;
#endif
      return;
    }

    // Process the lookahead token.
    unsigned nesting_level = 0;
    if (next_.GetBlockType() == CSSParserToken::kBlockStart) {
      nesting_level++;
    }

    // Add tokens to our return vector until we see either EOF or we meet the
    // return condition. (The termination condition is within the loop.)
    while (true) {
      CSSParserToken token = tokenizer_.TokenizeSingle();
      if (token.IsEOF() ||
          (nesting_level == 0 && TokenMarksEnd<Types...>(token))) {
        next_ = token;
#if DCHECK_IS_ON()
        // We know what type this is now.
        peeked_at_next_ = true;
#endif
        offset_ = tokenizer_.PreviousOffset();
        return;
      } else if (token.GetBlockType() == CSSParserToken::kBlockStart) {
        nesting_level++;
      } else if (token.GetBlockType() == CSSParserToken::kBlockEnd) {
        nesting_level--;
      }
    }
  }

  // Restarts
  // ========
  //
  // CSSParserTokenStream has limited restart capabilities through the
  // Save and Restore functions.
  //
  // Saving the stream is allowed under the condition that the lookahead token
  // is present. (See HasLookAhead). This avoids having to store whether or not
  // we have a lookahead token.
  //
  // Restoring the stream is allowed under the following conditions:
  //
  //  1. The lookahead token is present (at the time of Restore). This is
  //     important for undoing mutations to the tokenizer's block stack (see
  //     CSSTokenizer::Restore).
  //  2. The Save/Restore pair does not cross a BlockGuard.
  //  3. The Save/Restore pair does not cross a Boundary. (See section below).
  //     This limitation avoids having to store the boundary.
  //
  //
  // Restoring
  // =========
  //
  // Suppose that we had a short string to tokenize.
  //
  //  - The '^' indicates the position of the tokenizer (CSSTokenizer).
  //  - The 'offset' indicates the value of CSSParserTokenStream::offset_.
  //
  // These values temporarily go out of sync when producing lookahead values,
  // because doing so moves the position of the tokenizer only. The stream
  // offset does not catch up until the lookahead is Consumed.
  //
  // The initial state looks like this:
  //
  //   span:hover { X }  [offset=0]
  //   ^
  // Ensuring lookahead moves the tokenizer position (but not the stream
  // offset):
  //
  //   span:hover { X }  [offset=0, lookahead=span]
  //       ^
  // Consuming that lookahead token makes the offset catch up:
  //
  //   span:hover { X }  [offset=4]
  //       ^
  // Ensure lookahead again:
  //
  //   span:hover { X }  [offset=4, lookahead=:]
  //        ^
  // Consuming again:
  //
  //   span:hover { X }  [offset=5]
  //        ^
  // Now suppose that we had saved the stream state earlier,
  // at [offset=0, lookahead=span] (keeping in mind that having lookahead is
  // a prerequisite for saving the stream). We can restore to that position,
  // provided that we first ensure lookahead:
  //
  //   span:hover { X }  [offset=5, lookahead=hover]
  //             ^
  // The restore process will then do two things. First, rewind the tokenizer's
  // position to that of the saved stream offset (0):
  //
  //   span:hover { X }  [offset=5, lookahead=hover]
  //   ^
  // Then, set the stream offset to that rewound tokenizer position (0),
  // and recreate the lookahead from that point:
  //
  //   span:hover { X }  [offset=0, lookahead=span]
  //       ^
  // Now that the restore is finished, we have exactly the same state as when
  // it was saved: [offset=0, lookahead=span].
  //
  //
  // Blocks
  // ======
  //
  // Suppose instead that we want to restore to offset=0 in this state:
  //
  //   span:hover { X }  [offset=11, lookahead={]
  //               ^
  // Now we have a problem, because producing the lookahead token for '{'
  // modified the block stack of the CSSTokenizer. This is why the restore
  // process requires a lookahead token: we inspect the block type of that
  // lookahead token to *undo* the mutation before the rest of the restore
  // process.
  //
  //  - If the lookahead token has BlockType::kBlockStart,
  //    then we simply pop the recently pushed token type from the stack.
  //  - If the lookahead token has BlockType::kBlockEnd,
  //    then we push the matching token type to the stack to restore the
  //    recently popped token type.
  //
  // Note that it's not possible to Consume past a block-start or block-end:
  // a BlockGuard is required to enter blocks, which also ensures that we always
  // consume the entire block. Note also that block-end tokens are treated as
  // EOF (see UncheckedAtEnd): it is therefore not possible to escape the
  // current block during a BlockGuard. For these reasons, we only ever need to
  // undo at most one mutation to the block stack: the block stack mutation
  // caused by the "final" lookahead before the restore process.
  //
  // Boundaries
  // ==========
  //
  // The state may be saved and restored during a Boundary, but the boundary
  // conditions must be the same during the call to Restore as they were
  // during the call to Save. For example, can you use a boundary that's
  // created and destroyed between the Save/Restore calls:
  //
  //  State s = stream.Save();
  //  {
  //    CSSParserTokenStream::Boundary boundary(...);
  //    ConsumeSomething(stream);
  //  }
  //  stream.Restore(s);
  //
  // Or you can use a boundary that exists during both Save and Restore calls:
  //
  //  CSSParserTokenStream::Boundary boundary(...);
  //  State s = stream.Save();
  //  ConsumeSomething(stream);
  //  stream.Restore(s);
  //
  // However, a Save/Restore pair must not cross the boundary. The following
  // will trigger a DCHECK:
  //
  //  State s = stream.Save();
  //  ConsumeSomething(stream);
  //  {
  //    CSSParserTokenStream::Boundary boundary(...);
  //    stream.Restore(s);
  //  }

#if DCHECK_IS_ON()
  struct State {
    STACK_ALLOCATED();

   private:
    friend class CSSParserTokenStream;
    State(wtf_size_t offset, uint64_t boundaries)
        : offset_(offset), boundaries_(boundaries) {}
    wtf_size_t offset_;
    uint64_t boundaries_;
  };
#else   // !DCHECK_IS_ON()
  using State = wtf_size_t;
#endif  // DCHECK_IS_ON()

  State Save() const {
    DCHECK(has_look_ahead_);
#if DCHECK_IS_ON()
    return State(offset_, boundaries_);
#else   // !DCHECK_IS_ON()
    return offset_;
#endif  // DCHECK_IS_ON()
  }

  void Restore(State state) {
    DCHECK(has_look_ahead_);
#if DCHECK_IS_ON()
    if (offset_ == state.offset_) {
      // See comment below.
      return;
    }
    offset_ = state.offset_;
    DCHECK_EQ(state.boundaries_, boundaries_) << "Boundary-crossing restore";
#else   // !DCHECK_IS_ON()
    if (offset_ == state) {
      // No rewind needed, so we don't need to re-tokenize.
      // This happens especially often in MathFunctionParser
      // due to its design; it would perhaps be better to fix that
      // and other callers (it's cheaper never to rewind than to
      // test that rewind isn't needed), but this saves
      // quite a bit of time in total, so the test is generally
      // worth it.
      return;
    }
    offset_ = state;
#endif  // DCHECK_IS_ON()
    next_ = tokenizer_.Restore(next_, offset_);
#if DCHECK_IS_ON()
    peeked_at_next_ = true;  // It's possible.
#endif
  }

  // A RestoringBlockGuard is an object that allows you to enter a block,
  // and guarantees that (once destroyed) the stream is not left in the middle
  // of that block. This guarantee is met one of two ways:
  //
  //  1. The guard is *released*, and no action is taken when it goes
  //     out of scope, except consuming the block-end. Releasing a guard
  //     is only possible at the block-end (or EOF).
  //  2. The guard is not released, and the stream is restored to the
  //     specified state when the guard goes out of scope. This is the default
  //     behavior.
  //
  // This is useful in situations where you need to speculatively enter a block,
  // but then expect to "abort" parsing depending on the first few tokens within
  // that block.
  //
  // Note that the provided state must not cross another [Restoring]BlockGuard.
  class RestoringBlockGuard {
    STACK_ALLOCATED();

    // Outer boundaries do not "inherit" into the block. They will be restored
    // in the destructor.
    static uint64_t ResetStreamBoundaries(CSSParserTokenStream& stream) {
      uint64_t original = stream.boundaries_;
      stream.boundaries_ = FlagForTokenType(kEOFToken);
      return original;
    }

   public:
    explicit RestoringBlockGuard(CSSParserTokenStream& stream)
        : stream_(stream),
          boundaries_(ResetStreamBoundaries(stream)),
          state_(stream.Save()) {
      const CSSParserToken next = stream.ConsumeInternal();
      DCHECK_EQ(next.GetBlockType(), CSSParserToken::kBlockStart);
    }

    // Attempts to release the guard. If the guard could not be released
    // (i.e. we are not at the end of the block or EOF), then this call
    // has no effect, and ~RestoringBlockGuard will restore the stream to
    // the pre-guard state. If the guard could be released, then this
    // function consumes the block-end token.
    //
    // The return value of this function is useful for checking whether or
    // not we are at the end of the block. If we expect to be at the end
    // of a block, we can try to Release the guard. If that succeeded
    // we were indeed at the end, otherwise we had unexpected trailing
    // tokens (a parse failure of whatever we're trying to parse).
    //
    // Note that the following examples ignore whitespace tokens.
    //
    //  Example 1: rgb(0, 128, 64)
    //                           ^
    //  After consuming '64' from this stream, we try to Release the guard.
    //  Since we're at the end of the block, the guard is released.
    //
    //  Example 2: rgb(0, 128, 64 nonsense)
    //                            ^
    //  After consuming '64' from this stream, we try to Release the guard.
    //  Since we're not at the end of the block, the guard isn't released,
    //  which means that we have unknown trailing tokens.
    bool Release() {
      DCHECK(!released_);
      stream_.EnsureLookAhead();
      if (stream_.next_.IsEOF() ||
          stream_.next_.GetBlockType() == CSSParserToken::kBlockEnd) {
        stream_.UncheckedConsumeInternal();
        released_ = true;
        return true;
      }
      return false;
    }

    ~RestoringBlockGuard() {
      stream_.EnsureLookAhead();
      if (!released_) {
        // The guard has not been released, and we need to restore to the
        // pre-guard state.
        stream_.Restore(state_);
        // Pops the item pushed by the call to UncheckedConsumeInternal.
        // Note that if we happen to be at the end of the block, then we already
        // popped the block stack, but Restore would have pushed to the stack
        // again.
        stream_.PopBlockStack();
      }
      stream_.boundaries_ = boundaries_;
    }

   private:
    CSSParserTokenStream& stream_;
    uint64_t boundaries_;
    State state_;
    bool released_ = false;
  };

  wtf_size_t TokenCount() const { return tokenizer_.TokenCount(); }

 private:
  template <CSSParserTokenType... EndTypes>
  ALWAYS_INLINE bool TokenMarksEnd(const CSSParserToken& token) {
    return (boundaries_ & FlagForTokenType(token.GetType())) ||
           token.GetBlockType() == CSSParserToken::kBlockEnd ||
           detail::IsTokenTypeOneOf<EndTypes...>(token.GetType());
  }

  const CSSParserToken& PeekInternal() {
    EnsureLookAhead();
    return UncheckedPeekInternal();
  }

  const CSSParserToken& UncheckedPeekInternal() const {
    DCHECK(HasLookAhead());
    return next_;
  }

  const CSSParserToken& ConsumeInternal() {
    EnsureLookAhead();
    return UncheckedConsumeInternal();
  }

  const CSSParserToken& UncheckedConsumeInternal() {
    DCHECK(HasLookAhead());
    has_look_ahead_ = false;
    offset_ = tokenizer_.Offset();
    return next_;
  }

  // Used after switching tokenizer_.unicode_ranges_allowed_, which may change
  // interpretation of the tokens (and thus, what the lookahead token should
  // be).
  void RetokenizeLookAhead() {
    if (has_look_ahead_) {
      next_ = tokenizer_.Restore(next_, tokenizer_.PreviousOffset());
#if DCHECK_IS_ON()
      peeked_at_next_ = false;
#endif
    }
  }

  // Assuming the last token was a BlockStart token, ignores tokens
  // until the matching BlockEnd token or EOF. Requires but does _not_
  // leave a lookahead token active (for unknown reasons).
  void UncheckedSkipToEndOfBlock();

  void PopBlockStack() { tokenizer_.block_stack_.pop_back(); }

  CSSTokenizer tokenizer_;
  CSSParserToken next_;
#if DCHECK_IS_ON()
  mutable bool peeked_at_next_ = false;
#endif
  wtf_size_t offset_ = 0;
  bool has_look_ahead_ = false;
  uint64_t boundaries_ = FlagForTokenType(kEOFToken);
  // Attr tainted intervals [start, end).
  const Vector<std::pair<wtf_size_t, wtf_size_t>>* attr_taint_ranges_ = nullptr;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_PARSER_CSS_PARSER_TOKEN_STREAM_H_
