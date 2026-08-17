// Protocol Buffers - Google's data interchange format
// Copyright 2023 Google LLC.  All rights reserved.
//
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file or at
// https://developers.google.com/open-source/licenses/bsd

// Class for parsing tokenized text from a ZeroCopyInputStream.

#ifndef UPB_IO_TOKENIZER_H_
#define UPB_IO_TOKENIZER_H_

#include "upb/base/status.h"
#include "upb/base/string_view.h"
#include "upb/io/zero_copy_input_stream.h"
#include "upb/mem/arena.h"

// Must be included last.
#include "upb/port/def.inc"

#ifdef __cplusplus
extern "C" {
#endif

typedef enum {
  kUpb_TokenType_Start,  // Next() has not yet been called.
  kUpb_TokenType_End,    // End of input reached. "text" is empty.

  // A sequence of letters, digits, and underscores, not starting with a digit.
  // It is an error for a number to be followed by an identifier with no space
  // in between.
  kUpb_TokenType_Identifier,

  // A sequence of digits representing an integer. Normally the digits are
  // decimal, but a prefix of "0x" indicates a hex number and a leading zero
  // indicates octal, just like with C numeric literals. A leading negative
  // sign is NOT included in the token; it's up to the parser to interpret the
  // unary minus operator on its own.
  kUpb_TokenType_Integer,

  // A floating point literal, with a fractional part and/or an exponent.
  // Always in decimal. Again, never negative.
  kUpb_TokenType_Float,

  // A quoted sequence of escaped characters.
  // Either single or double quotes can be used, but they must match.
  // A string literal cannot cross a line break.
  kUpb_TokenType_String,

  // Any other printable character, like '!' or '+'.
  // Symbols are always a single character, so "!+$%" is four tokens.
  kUpb_TokenType_Symbol,

  // A sequence of whitespace.
  // This token type is only produced if report_whitespace() is true.
  // It is not reported for whitespace within comments or strings.
  kUpb_TokenType_Whitespace,

  // A newline ('\n'). This token type is only produced if report_whitespace()
  // is true and report_newlines() is also true.
  // It is not reported for newlines in comments or strings.
  kUpb_TokenType_Newline,
} upb_TokenType;

typedef enum {
  // Set to allow floats to be suffixed with the letter 'f'. Tokens which would
  // otherwise be integers but which have the 'f' suffix will be forced to be
  // interpreted as floats. For all other purposes, the 'f' is ignored.
  kUpb_TokenizerOption_AllowFAfterFloat = 1 << 0,

  // If set, whitespace tokens are reported by Next().
  kUpb_TokenizerOption_ReportWhitespace = 1 << 1,

  // If set, newline tokens are reported by Next().
  // This is a superset of ReportWhitespace.
  kUpb_TokenizerOption_ReportNewlines = 1 << 2,

  // By default the tokenizer expects C-style (/* */) comments.
  // If set, it expects shell-style (#) comments instead.
  kUpb_TokenizerOption_CommentStyleShell = 1 << 3,
} upb_Tokenizer_Option;

typedef struct upb_Tokenizer upb_Tokenizer;

// Can be passed a flat array and/or a ZCIS as input.
// The array will be read first (if non-NULL), then the stream (if non-NULL).
upb_Tokenizer* upb_Tokenizer_New(const void* data, size_t size,
                                 upb_ZeroCopyInputStream* input, int options,
                                 upb_Arena* arena);

void upb_Tokenizer_Fini(upb_Tokenizer* t);

// Advance the tokenizer to the next input token. Returns True on success.
// Returns False and (clears *status on EOF, sets *status on error).
bool upb_Tokenizer_Next(upb_Tokenizer* t, upb_Status* status);

// Accessors for inspecting current/previous parse tokens,
// which are opaque to the tokenizer (to reduce copying).

upb_TokenType upb_Tokenizer_Type(const upb_Tokenizer* t);
int upb_Tokenizer_Column(const upb_Tokenizer* t);
int upb_Tokenizer_EndColumn(const upb_Tokenizer* t);
int upb_Tokenizer_Line(const upb_Tokenizer* t);
int upb_Tokenizer_TextSize(const upb_Tokenizer* t);
const char* upb_Tokenizer_TextData(const upb_Tokenizer* t);

// External helper: validate an identifier.
bool upb_Tokenizer_IsIdentifier(const char* data, int size);

// Parses a TYPE_INTEGER token. Returns false if the result would be
// greater than max_value. Otherwise, returns true and sets *output to the
// result. If the text is not from a Token of type TYPE_INTEGER originally
// parsed by a Tokenizer, the result is undefined (possibly an assert
// failure).
bool upb_Parse_Integer(const char* text, uint64_t max_value, uint64_t* output);

// Parses a TYPE_FLOAT token. This never fails, so long as the text actually
// comes from a TYPE_FLOAT token parsed by Tokenizer. If it doesn't, the
// result is undefined (possibly an assert failure).
double upb_Parse_Float(const char* text);

// Parses a TYPE_STRING token. This never fails, so long as the text actually
// comes from a TYPE_STRING token parsed by Tokenizer. If it doesn't, the
// result is undefined (possibly an assert failure).
upb_StringView upb_Parse_String(const char* text, upb_Arena* arena);

#ifdef __cplusplus
} /* extern "C" */
#endif

#include "upb/port/undef.inc"

#endif  // UPB_IO_TOKENIZER_H_
