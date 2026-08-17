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

#ifndef UTIL_LANGUAGES_LANGUAGES_H_
#define UTIL_LANGUAGES_LANGUAGES_H_

// This interface defines the Language enum and functions that depend
// only on Language values.

// A hash-function for Language, hash<Language>, is defined in
// i18n/languages/public/languages-hash.h

#ifndef SWIG
// Language enum defined in languages.proto
// Also description on how to add languages.
#include "util/languages/languages.pb.h"

#else

// TODO: Include a header containing swig-compatible enum.

#endif

const int kNumLanguages = NUM_LANGUAGES;

// Return the default language (ENGLISH).
Language default_language();


// *******************************************
// Language predicates
//   IsValidLanguage()
//   IS_LANGUAGE_UNKNOWN()
//   IsCJKLanguage()
//   IsChineseLanguage()
//   IsNorwegianLanguage()
//   IsPortugueseLanguage()
//   IsRightToLeftLanguage()
//   IsMaybeRightToLeftLanguage()
//   IsSameLanguage()
//   IsScriptRequiringLongerSnippets()
// *******************************************

// IsValidLanguage
// ===============
//
// Function to check if the input is within range of the Language enum. If
// IsValidLanguage(lang) returns true, it is safe to call
// static_cast<Language>(lang).
//
inline bool IsValidLanguage(int lang) {
  return ((lang >= 0) && (lang < kNumLanguages));
}

// Return true if the language is "unknown". (This function was
// previously a macro, hence the spelling in all caps.)
//
inline bool IS_LANGUAGE_UNKNOWN(Language lang) {
  return lang == TG_UNKNOWN_LANGUAGE || lang == UNKNOWN_LANGUAGE;
}

// IsCJKLanguage
// -------------
//
// This function returns true if the language is either Chinese
// (simplified or traditional), Japanese, or Korean.
bool IsCJKLanguage(Language lang);

// IsChineseLanguage
// -----------------
//
// This function returns true if the language is either Chinese
// (simplified or traditional)
bool IsChineseLanguage(Language lang);

// IsNorwegianLanguage
// --------------------
//
// This function returns true if the language is any of the Norwegian
// (regular or Nynorsk).
bool IsNorwegianLanguage(Language lang);

// IsPortugueseLanguage
// --------------------
//
// This function returns true if the language is any of the Portuguese
// languages (regular, Portugal or Brazil)
bool IsPortugueseLanguage(Language lang);

// IsSameLanguage
// --------------
//
// WARNING: This function provides only a simple test on the values of
// the two Language arguments. It returns false if either language is
// invalid. It returns true if the language arguments are equal, or
// if they are both Chinese languages, both Norwegian languages, or
// both Portuguese languages, as defined by IsChineseLanguage,
// IsNorwegianLanguage, and IsPortugueseLanguage. Otherwise it returns
// false.
bool IsSameLanguage(Language lang1, Language lang2);


// IsRightToLeftLanguage
// ---------------------
//
// This function returns true if the language is only written right-to-left
// (E.g., Hebrew, Arabic, Persian etc.)
//
// IMPORTANT NOTE: Technically we're talking about scripts, not languages.
// There are languages that can be written in more than one script.
// Examples:
//   - Kurdish and Azeri ('AZERBAIJANI') can be written left-to-right in
//     Latin or Cyrillic script, and right-to-left in Arabic script.
//   - Sindhi and Punjabi are written in different scripts, depending on
//     region and dialect.
//   - Turkmen used an Arabic script historically, but not any more.
//   - Pashto and Uyghur can use Arabic script, but use a Roman script
//     on the Internet.
//   - Kashmiri and Urdu are written either with Arabic or Devanagari script.
//
// This function only returns true for languages that are always, unequivocally
// written in right-to-left script.
//
// TODO: If we want to do anything special with multi-script languages
// we should create new 'languages' for each language+script, as we do for
// traditional vs. simplified Chinese. However most such languages are rare in
// use and even rarer on the web, so this is unlikely to be something we'll
// be concerned with for a while.
bool IsRightToLeftLanguage(Language lang);

// IsMaybeRightToLeftLanguage
// --------------------------
//
// This function returns true if the language may appear on the web in a
// right-to-left script (E.g., Hebrew, Arabic, Persian, Urdu, Kurdish, etc.)
//
// NOTE: See important notes under IsRightToLeftLanguage(...).
//
// This function returns true for languages that *may* appear on the web in a
// right-to-left script, even if they may also appear in a left-to-right
// script.
//
// This function should typically be used in cases where doing some work on
// left-to-right text would be OK (usually a no-op), and this function is used
// just to cut down on unnecessary work on regular, LTR text.
bool IsMaybeRightToLeftLanguage(Language lang);

// IsScriptRequiringLongerSnippets
// --------------------
//
// This function returns true if the script chracteristics require longer
// snippet length (Devanagari, Bengali, Gurmukhi,
// Gujarati, Oriya, Tamil, Telugu, Kannada, Malayalam).
// COMMENTED OUT TO REDUCE DEPENDENCIES ON GOOGLE3 CODE
// bool IsScriptRequiringLongerSnippets(UnicodeScript script);


// *******************************************
// LANGUAGE NAMES
//
// This interface defines a standard name for each valid Language,
// and a standard name for invalid languages. Some language names use all
// uppercase letters, but others use mixed case.
//   LanguageName() [Language to name]
//   LanguageEnumName() [language to enum name]
//   LanguageFromName() [name to Language]
//   default_language_name()
//   invalid_language_name()
// *******************************************

// Given a Language, returns its standard name.
// Return invalid_language_name() if the language is invalid.
const char* LanguageName(Language lang);

// Given a Language, return the name of the enum constant for that
// language. In all but a few cases, this is the same as its standard
// name. For example, LanguageName(CHINESE) returns "Chinese", but
// LanguageEnumName(CHINESE) returns "CHINESE". This is intended for
// code that is generating C++ code, where the enum constant is more
// useful than its integer value.  Return "NUM_LANGUAGES" if
// the language is invalid.
const char* LanguageEnumName(Language lang);

// The maximum length of a standard language name.
const int kMaxLanguageNameSize = 50;

// The standard name for the default language.
const char* default_language_name();

// The standard name for all invalid languages.
const char* invalid_language_name();

// If lang_name matches the standard name of a Language, using a
// case-insensitive comparison, set *language to that Language and
// return true.
// Otherwise, set *language to UNKNOWN_LANGUAGE and return false.
//
// For backwards compatibility, "HATIAN_CREOLE" is allowed as a name
// for HAITIAN_CREOLE, and "QUECHAU" is allowed as a name for QUECHUA.
// For compatibility with LanguageEnumName, "UNKNOWN_LANGUAGE" is allowed
// as a name for UNKNOWN_LANGUAGE (the return value is true in this case,
// as it is for "Unknown"), and "CHINESE_T" is allowed as a name for
// CHINESE_T (i.e., a synonym for "ChineseT").
//
// REQUIRES: language must not be NULL.
//
bool LanguageFromName(const char* lang_name, Language *language);



// *******************************************
// LANGUAGE CODES
//
// This interface defines a standard code for each valid language, and
// a standard code for invalid languages. These are derived from ISO codes,
// with some Google additions.
//   LanguageCode()
//   default_language_code()
//   invalid_language_code()
//   LanguageCodeWithDialects()
//   LanguageCodeISO639_1()
//   LanguageCodeISO639_2()
// *******************************************

// Given a Language, return its standard code. There are Google-specific codes:
//     For CHINESE_T, return "zh-TW".
//     For TG_UNKNOWN_LANGUAGE, return "ut".
//     For UNKNOWN_LANGUAGE, return "un".
//     For PORTUGUESE_P, return "pt-PT".
//     For PORTUGUESE_B, return "pt-BR".
//     For LIMBU, return "sit-NP".
//     For CHEROKEE, return "chr".
//     For SYRIAC, return "syr".
// Otherwise return the ISO 639-1 two-letter language code for lang.
// If lang is invalid, return invalid_language_code().
//
// NOTE: See the note below about the codes for Chinese languages.
//
const char* LanguageCode(Language lang);

// The maximum length of a language code.
const int kMaxLanguageCodeSize = 50;

// The standard code for the default language.
const char* default_language_code();

// The standard code for all invalid languages.
const char* invalid_language_code();


// --------------------------------------------
// NOTE: CHINESE LANGUAGE CODES
//
// There are three functions that return codes for Chinese languages.
// LanguageCode(lang) and LanguageCodeWithDialects(lang) are defined here.
// LanguageCode(lang, encoding) is defined in i18n/encodings.lang_enc.h.
// The following list shows the different results.
//
// LanguageCode(CHINESE) returns "zh"
// LanguageCode(CHINESE_T) returns "zh-TW".
//
// LanguageCodeWithDialects(CHINESE) returns "zh-CN".
// LanguageCodeWithDialects(CHINESE_T) returns "zh-TW".
//
// LanguageCode(CHINESE_T, <any encoding>) returns "zh-TW".
// LanguageCode(CHINESE, CHINESE_BIG5) returns "zh-TW".
// LanguageCode(CHINESE, <any other encoding>) returns "zh-CN".
//
// --------------------------------------------

// LanguageCodeWithDialects
// ------------------------
//
// If lang is CHINESE, return "zh-CN". Otherwise return LanguageCode(lang).
const char* LanguageCodeWithDialects(Language lang);

// LanguageCodeISO639_1
// --------------------
//
// Return the ISO 639-1 two-letter language code for lang.
// Return invalid_language_code() if lang is invalid or does not have
// an ISO 639-1 two-letter language code.
const char* LanguageCodeISO639_1(Language lang);

// LanguageCodeISO639_2
// --------------------
//
// Return the ISO 639-2 three-letter language for lang.
// Return invalid_language_code() if lang is invalid or does not have
// an ISO 639-2 three-letter language code.
const char* LanguageCodeISO639_2(Language lang);

// LanguageFromCode
// ----------------
//
// If lang_code matches the code for a Language, using a case-insensitive
// comparison, set *lang to that Language and return true.
// Otherwise, set *lang to UNKNOWN_LANGUAGE and return false.
//
// lang_code can be an ISO 639-1 (two-letter) code, an ISO 639-2
// (three-letter) code, or a Google-specific code (see LanguageCode).
//
// Certain language-code aliases are also allowed:
//   For "zh-cn" and "zh_cn", set *lang to CHINESE.
//   For "zh-tw" and "zh_tw", set *lang to CHINESE_T.
//   For "he", set *lang to HEBREW.
//   For "in", set *lang to INDONESIAN.
//   For "ji", set *lang to YIDDISH.
//   For "fil", set *lang to TAGALOG.
//
// REQUIRES: 'lang' must not be NULL.
bool LanguageFromCode(const char* lang_code, Language *language);


// LanguageFromCodeOrName
// ----------------------
//
// If lang_code_or_name is a language code or a language name.
// set *language to the corresponding Language and return true.
// Otherwise set *language to UNKNOWN_LANGUAGE and return false.
//
bool LanguageFromCodeOrName(const char* lang_code_or_name,
                            Language* language);

// LanguageNameFromCode
// --------------------
//
// If language_code is the code for a Language (see LanguageFromCode),
// return the standard name of that language (see LanguageName).
// Otherwise return invalid_language_name().
//
const char* LanguageNameFromCode(const char* language_code);


// Miscellany

// LanguageCodeToUnderscoreForm
// ----------------------------
//
// Given a language code, convert the dash "-" to underscore "_".
//
// Specifically, if result_length <= strlen(lang_code), set result[0]
// to '\0' and return false. Otherwise, copy lang_code to result,
// converting every dash to an underscore, converting every character
// before the first dash or underscore to lower case, and converting
// every character after the first dash or underscore to upper
// case. If there is no dash or underscore, convert the entire string
// to lower case.
//
// REQUIRES: 'lang_code' must not be NULL. 'result' must not be NULL.

bool LanguageCodeToUnderscoreForm(const char* lang_code,
                                  char* result,
                                  int result_length);

//
// AlwaysPutInExpectedRestrict
// ---------------------------
//
// For Web pages in certain top-level domains, Web Search always
// applies a "country restrict". If 'tld' matches one of those, using
// a case-SENSITIVE comparison, set *expected_language to the Language
// most commonly found in that top-level domain and return true.
// Otherwise, set *expected_language to UNKNOWN_LANGUAGE and return false.
bool AlwaysPutInExpectedRestrict(const char *tld, Language *expected_language);


#endif  // UTIL_LANGUAGES_LANGUAGES_H_
