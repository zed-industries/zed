/* Copyright 2023 The MediaPipe Authors.

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
==============================================================================*/

// Fork of several UTF utils originally written by Rob Pike and Ken Thompson.
#ifndef MEDIAPIPE_TASKS_CC_TEXT_LANGUAGE_DETECTOR_CUSTOM_OPS_UTILS_UTF_UTF_H_
#define MEDIAPIPE_TASKS_CC_TEXT_LANGUAGE_DETECTOR_CUSTOM_OPS_UTILS_UTF_UTF_H_ 1

#include <stdint.h>

// Code-point values in Unicode 4.0 are 21 bits wide.
typedef signed int Rune;

#define uchar _utfuchar

typedef unsigned char uchar;

#define nelem(x) (sizeof(x) / sizeof((x)[0]))

enum {
  UTFmax = 4,          // maximum bytes per rune
  Runeerror = 0xFFFD,  // decoding error in UTF
  Runemax = 0x10FFFF,  // maximum rune value
};

#ifdef __cplusplus
extern "C" {
#endif

/*
 * rune routines
 */

/*
 * These routines were written by Rob Pike and Ken Thompson
 * and first appeared in Plan 9.
 * SEE ALSO
 * utf (7)
 * tcs (1)
 */

// utf_runetochar copies (encodes) one rune, pointed to by r, to at most
// UTFmax bytes starting at s and returns the number of bytes generated.

int utf_runetochar(char* s, const Rune* r);

// utf_charntorune copies (decodes) at most UTFmax bytes starting at `str` to
// one rune, pointed to by `rune`, access at most `length` bytes of `str`, and
// returns the number of bytes consumed.
// If the UTF sequence is incomplete within n bytes,
// utf_charntorune will set *r to Runeerror and return 0. If it is complete
// but not in UTF format, it will set *r to Runeerror and return 1.
//
// Added 2004-09-24 by Wei-Hwa Huang

int utf_charntorune(Rune* rune, const char* str, int length);

// Unicode defines some characters as letters and
// specifies three cases: upper, lower, and title.  Mappings among the
// cases are also defined, although they are not exhaustive: some
// upper case letters have no lower case mapping, and so on.  Unicode
// also defines several character properties, a subset of which are
// checked by these routines.  These routines are based on Unicode
// version 3.0.0.
//
// NOTE: The routines are implemented in C, so isalpharrune returns 0 for false
// and 1 for true.
//
// utf_tolowerrune is the Unicode case mapping. It returns the character
// unchanged if it has no defined mapping.

Rune utf_tolowerrune(Rune r);

// utf_isalpharune tests for Unicode letters; this includes ideographs in
// addition to alphabetic characters.

int utf_isalpharune(Rune r);

// (The comments in this file were copied from the manpage files rune.3,
// isalpharune.3, and runestrcat.3. Some formatting changes were also made
// to conform to Google style. /JRM 11/11/05)

#ifdef __cplusplus
}
#endif

#endif  // MEDIAPIPE_TASKS_CC_TEXT_LANGUAGE_DETECTOR_CUSTOM_OPS_UTILS_UTF_UTF_H_
