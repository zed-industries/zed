// Copyright 2016 Google Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
////////////////////////////////////////////////////////////////////////////////

#ifndef UTIL_ENCODINGS_ENCODINGS_H_
#define UTIL_ENCODINGS_ENCODINGS_H_

// This interface defines the Encoding enum and various functions that
// depend only on Encoding values.

// A hash-function for Encoding, hash<Encoding>, is defined in
// i18n/encodings/public/encodings-hash.h

// On some Windows projects, UNICODE may be defined, which would prevent the
// Encoding enum below from compiling. Note that this is a quick fix that does
// not break any existing projects. The UNICODE enum may someday be changed
// to something more specific and non-colliding, but this involves careful
// testing of changes in many other projects.
#undef UNICODE

// NOTE: The Encoding enum must always start at 0. This assumption has
// been made and used.

#ifndef SWIG

#include "util/encodings/encodings.pb.h"

#else

// TODO: Include a SWIG workaround header file.

#endif

const int kNumEncodings = NUM_ENCODINGS;

// some of the popular encoding aliases
// TODO: Make these static const Encoding values instead of macros.
#define LATIN1           ISO_8859_1
#define LATIN2           ISO_8859_2
#define LATIN3           ISO_8859_3
#define LATIN4           ISO_8859_4
#define CYRILLIC         ISO_8859_5
#define ARABIC_ENCODING  ISO_8859_6     // avoiding the same name as language
#define GREEK_ENCODING   ISO_8859_7     // avoiding the same name as language
#define HEBREW_ENCODING  ISO_8859_8     // avoiding the same name as language
#define LATIN5           ISO_8859_9
#define LATIN6           ISO_8859_10
#define KOREAN_HANGUL    KOREAN_EUC_KR

// The default Encoding (LATIN1).
Encoding default_encoding();



// *************************************************************
// Encoding predicates
//   IsValidEncoding()
//   IsEncEncCompatible
//   IsSupersetOfAscii7Bit
//   Is8BitEncoding
//   IsCJKEncoding
//   IsHebrewEncoding
//   IsRightToLeftEncoding
//   IsLogicalRightToLeftEncoding
//   IsVisualRightToLeftEncoding
//   IsIso2022Encoding
//   IsIso2022JpOrVariant
//   IsShiftJisOrVariant
//   IsJapaneseCellPhoneCarrierSpecificEncoding
// *************************************************************

// IsValidEncoding
// ===================================
//
// Function to check if the input language enum is within range.
//

bool IsValidEncoding(Encoding enc);

//
// IsEncEncCompatible
// ------------------
//
// This function is to determine whether or not converting from the
// first encoding to the second requires any changes to the underlying
// text (e.g.  ASCII_7BIT is a subset of UTF8).
//
// TODO: the current implementation is likely incomplete.  It would be
// good to consider the full matrix of all pairs of encodings and to fish out
// all compatible pairs.
//
bool IsEncEncCompatible(const Encoding from, const Encoding to);

// To be a superset of 7-bit Ascii means that bytes 0...127 in the given
// encoding represent the same characters as they do in ISO_8859_1.

// WARNING: This function does not currently return true for all encodings that
// are supersets of Ascii 7-bit.
bool IsSupersetOfAscii7Bit(Encoding e);

// To be an 8-bit encoding means that there are fewer than 256 symbols.
// Each byte determines a new character; there are no multi-byte sequences.

// WARNING: This function does not currently return true for all encodings that
// are 8-bit encodings.
bool Is8BitEncoding(Encoding e);

// IsCJKEncoding
// -------------
//
// This function returns true if the encoding is either Chinese
// (simplified or traditional), Japanese, or Korean. Note: UTF8 is not
// considered a CJK encoding.
bool IsCJKEncoding(Encoding e);

// IsHebrewEncoding
// -------------
//
// This function returns true if the encoding is a Hebrew specific
// encoding (not UTF8, etc).
bool IsHebrewEncoding(Encoding e);

// IsRightToLeftEncoding
// ---------------------
//
// Returns true if the encoding is a right-to-left encoding.
//
// Note that the name of this function is somewhat misleading. There is nothing
// "right to left" about these encodings. They merely contain code points for
// characters in RTL languages such as Hebrew and Arabic. But this is also
// true for UTF-8.
//
// TODO: Get rid of this function. The only special-case we
// should need to worry about are visual encodings. Anything we
// need to do for all 'RTL' encodings we need to do for UTF-8 as well.
bool IsRightToLeftEncoding(Encoding enc);

// IsLogicalRightToLeftEncoding
// ----------------------------
//
// Returns true if the encoding is a logical right-to-left encoding.
// Logical right-to-left encodings are those that the browser renders
// right-to-left and applies the BiDi algorithm to. Therefore the characters
// appear in reading order in the file, and indexing, snippet generation etc.
// should all just work with no special processing.
//
// TODO: Get rid of this function. The only special-case we
// should need to worry about are visual encodings.
bool IsLogicalRightToLeftEncoding(Encoding enc);

// IsVisualRightToLeftEncoding
// ---------------------------
//
// Returns true if the encoding is a visual right-to-left encoding.
// Visual right-to-left encodings are those that the browser renders
// left-to-right and does not apply the BiDi algorithm to. Therefore each
// line appears in reverse order in the file, lines are manually wrapped
// by abusing <br> or <p> tags, etc. Visual RTL encoding is a relic of
// the prehistoric days when browsers couldn't render right-to-left, but
// unfortunately some visual pages persist to this day. These documents require
// special processing so that we don't index or snippet them with each line
// reversed.
bool IsVisualRightToLeftEncoding(Encoding enc);

// IsIso2022Encoding
// -----------------
//
// Returns true if the encoding is a kind of ISO 2022 such as
// ISO-2022-JP.
bool IsIso2022Encoding(Encoding enc);

// IsIso2022JpOrVariant
// --------------------
//
// Returns true if the encoding is ISO-2022-JP or a variant such as
// KDDI's ISO-2022-JP.
bool IsIso2022JpOrVariant(Encoding enc);

// IsShiftJisOrVariant
// --------------------
//
// Returns true if the encoding is Shift_JIS or a variant such as
// KDDI's Shift_JIS.
bool IsShiftJisOrVariant(Encoding enc);

// IsJapanesCellPhoneCarrierSpecificEncoding
// -----------------------------------------
//
// Returns true if it's Japanese cell phone carrier specific encoding
// such as KDDI_SHIFT_JIS.
bool IsJapaneseCellPhoneCarrierSpecificEncoding(Encoding enc);



// *************************************************************
// ENCODING NAMES
//
// This interface defines a standard name for each valid encoding, and
// a standard name for invalid encodings. (Some names use all upper
// case, but others use mixed case.)
//
//   EncodingName() [Encoding to name]
//   MimeEncodingName() [Encoding to name]
//   EncodingFromName() [name to Encoding]
//   EncodingNameAliasToEncoding() [name to Encoding]
//   default_encoding_name()
//   invalid_encoding_name()
// *************************************************************

// EncodingName
// ------------
//
// Given the encoding, returns its standard name.
// Return invalid_encoding_name() if the encoding is invalid.
//
const char* EncodingName(Encoding enc);

//
// MimeEncodingName
// ----------------
//
// Return the "preferred MIME name" of an encoding.
//
// This name is suitable for using in HTTP headers, HTML tags,
// and as the "charset" parameter of a MIME Content-Type.
const char* MimeEncodingName(Encoding enc);


// The maximum length of an encoding name
const int kMaxEncodingNameSize = 50;

// The standard name of the default encoding.
const char* default_encoding_name();

// The name used for an invalid encoding.
const char* invalid_encoding_name();

// EncodingFromName
// ----------------
//
// If enc_name matches the standard name of an Encoding, using a
// case-insensitive comparison, set *encoding to that Encoding and
// return true.  Otherwise set *encoding to UNKNOWN_ENCODING and
// return false.
//
// REQUIRES: encoding must not be NULL.
//
bool EncodingFromName(const char* enc_name, Encoding *encoding);

//
// EncodingNameAliasToEncoding
// ---------------------------
//
// If enc_name matches the standard name or an alias of an Encoding,
// using a case-insensitive comparison, return that
// Encoding. Otherwise, return UNKNOWN_ENCODING.
//
// Aliases include most mime-encoding names (e.g., "ISO-8859-7" for
// GREEK), alternate names (e.g., "cyrillic" for ISO_8859_5) and
// common variations with hyphens and underscores (e.g., "koi8-u" and
// "koi8u" for RUSSIAN_KOI8_R).

Encoding EncodingNameAliasToEncoding(const char *enc_name);

// *************************************************************
// Miscellany
// *************************************************************

// PreferredWebOutputEncoding
// --------------------------
//
// Some multi-byte encodings use byte values that coincide with the
// ASCII codes for HTML syntax characters <>"&' and browsers like MSIE
// can misinterpret these, as indicated in an external XSS report from
// 2007-02-15. Here, we map these dangerous encodings to safer ones. We
// also use UTF8 instead of encodings that we don't support in our
// output, and we generally try to be conservative in what we send out.
// Where the client asks for single- or double-byte encodings that are
// not as common, we substitute a more common single- or double-byte
// encoding, if there is one, thereby preserving the client's intent
// to use less space than UTF-8. This also means that characters
// outside the destination set will be converted to HTML NCRs (&#NNN;)
// if requested.
Encoding PreferredWebOutputEncoding(Encoding enc);


#endif  // UTIL_ENCODINGS_ENCODINGS_H_
