// Copyright 2011 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_I18N_RTL_H_
#define BASE_I18N_RTL_H_

#include <string>
#include <string_view>

#include "base/i18n/base_i18n_export.h"
#include "build/build_config.h"

namespace base {

class FilePath;

namespace i18n {

const char16_t kRightToLeftMark = 0x200F;
const char16_t kLeftToRightMark = 0x200E;
const char16_t kLeftToRightEmbeddingMark = 0x202A;
const char16_t kRightToLeftEmbeddingMark = 0x202B;
const char16_t kPopDirectionalFormatting = 0x202C;
const char16_t kLeftToRightOverride = 0x202D;
const char16_t kRightToLeftOverride = 0x202E;

// Locale.java mirrored this enum TextDirection. Please keep in sync.
enum TextDirection {
  UNKNOWN_DIRECTION = 0,
  RIGHT_TO_LEFT = 1,
  LEFT_TO_RIGHT = 2,
  TEXT_DIRECTION_MAX = LEFT_TO_RIGHT,
};

// Get the locale that the currently running process has been configured to use.
// The return value is of the form language[-country] (e.g., en-US) where the
// language is the 2 or 3 letter code from ISO-639.
BASE_I18N_EXPORT std::string GetConfiguredLocale();

// Canonicalize a string (eg. a POSIX locale string) to a Chrome locale name.
BASE_I18N_EXPORT std::string GetCanonicalLocale(const std::string& locale);

// Sets the default locale of ICU.
// Once the application locale of Chrome in GetApplicationLocale is determined,
// the default locale of ICU need to be changed to match the application locale
// so that ICU functions work correctly in a locale-dependent manner.
// This is handy in that we don't have to call GetApplicationLocale()
// everytime we call locale-dependent ICU APIs as long as we make sure
// that this is called before any locale-dependent API is called.
BASE_I18N_EXPORT void SetICUDefaultLocale(const std::string& locale_string);

// Returns true if the application text direction is right-to-left.
BASE_I18N_EXPORT bool IsRTL();

// A test utility function to set the application default text direction.
BASE_I18N_EXPORT void SetRTLForTesting(bool rtl);

// Returns whether the text direction for the default ICU locale is RTL.  This
// assumes that SetICUDefaultLocale has been called to set the default locale to
// the UI locale of Chrome.
// NOTE: Generally, you should call IsRTL() instead of this.
BASE_I18N_EXPORT bool ICUIsRTL();

// Gets the explicitly forced text direction for debugging. If no forcing is
// applied, returns UNKNOWN_DIRECTION.
BASE_I18N_EXPORT TextDirection GetForcedTextDirection();

// Returns the text direction for |locale_name|.
// As a startup optimization, this method checks the locale against a list of
// Chrome-supported RTL locales.
BASE_I18N_EXPORT TextDirection
GetTextDirectionForLocaleInStartUp(const char* locale_name);

// Returns the text direction for |locale_name|.
BASE_I18N_EXPORT TextDirection
GetTextDirectionForLocale(const char* locale_name);

// Given the string in |text|, returns the directionality of the first or last
// character with strong directionality in the string. If no character in the
// text has strong directionality, LEFT_TO_RIGHT is returned. The Bidi
// character types L, LRE, LRO, R, AL, RLE, and RLO are considered as strong
// directionality characters. Please refer to http://unicode.org/reports/tr9/
// for more information.
BASE_I18N_EXPORT TextDirection
GetFirstStrongCharacterDirection(std::u16string_view text);
BASE_I18N_EXPORT TextDirection
GetLastStrongCharacterDirection(std::u16string_view text);

// Given the string in |text|, returns LEFT_TO_RIGHT or RIGHT_TO_LEFT if all the
// strong directionality characters in the string are of the same
// directionality. It returns UNKNOWN_DIRECTION if the string contains a mix of
// LTR and RTL strong directionality characters. Defaults to LEFT_TO_RIGHT if
// the string does not contain directionality characters. Please refer to
// http://unicode.org/reports/tr9/ for more information.
BASE_I18N_EXPORT TextDirection GetStringDirection(std::u16string_view text);

// Given the string in |text|, this function modifies the string in place with
// the appropriate Unicode formatting marks that mark the string direction
// (either left-to-right or right-to-left). The function checks both the current
// locale and the contents of the string in order to determine the direction of
// the returned string. The function returns true if the string in |text| was
// properly adjusted.
//
// Certain LTR strings are not rendered correctly when the context is RTL. For
// example, the string "Foo!" will appear as "!Foo" if it is rendered as is in
// an RTL context. Calling this function will make sure the returned localized
// string is always treated as a right-to-left string. This is done by
// inserting certain Unicode formatting marks into the returned string.
//
// ** Notes about the Windows version of this function:
// TODO(idana) bug 6806: this function adjusts the string in question only
// if the current locale is right-to-left. The function does not take care of
// the opposite case (an RTL string displayed in an LTR context) since
// adjusting the string involves inserting Unicode formatting characters that
// Windows does not handle well unless right-to-left language support is
// installed. Since the English version of Windows doesn't have right-to-left
// language support installed by default, inserting the direction Unicode mark
// results in Windows displaying squares.
BASE_I18N_EXPORT bool AdjustStringForLocaleDirection(std::u16string* text);

// Undoes the actions of the above function (AdjustStringForLocaleDirection).
BASE_I18N_EXPORT bool UnadjustStringForLocaleDirection(std::u16string* text);

// Ensures |text| contains no unterminated directional formatting characters, by
// appending the appropriate pop-directional-formatting characters to the end of
// |text|.
BASE_I18N_EXPORT void EnsureTerminatedDirectionalFormatting(
    std::u16string* text);

// Sanitizes the |text| by terminating any directional override/embedding
// characters and then adjusting the string for locale direction.
BASE_I18N_EXPORT void SanitizeUserSuppliedString(std::u16string* text);

// Returns true if the string contains at least one character with strong right
// to left directionality; that is, a character with either R or AL Unicode
// BiDi character type.
BASE_I18N_EXPORT bool StringContainsStrongRTLChars(std::u16string_view text);

// Wraps a string with an LRE-PDF pair which essentialy marks the string as a
// Left-To-Right string. Doing this is useful in order to make sure LTR
// strings are rendered properly in an RTL context.
BASE_I18N_EXPORT void WrapStringWithLTRFormatting(std::u16string* text);

// Wraps a string with an RLE-PDF pair which essentialy marks the string as a
// Right-To-Left string. Doing this is useful in order to make sure RTL
// strings are rendered properly in an LTR context.
BASE_I18N_EXPORT void WrapStringWithRTLFormatting(std::u16string* text);

// Wraps file path to get it to display correctly in RTL UI. All filepaths
// should be passed through this function before display in UI for RTL locales.
BASE_I18N_EXPORT void WrapPathWithLTRFormatting(const FilePath& path,
                                                std::u16string* rtl_safe_path);

// Return the string in |text| wrapped with LRE (Left-To-Right Embedding) and
// PDF (Pop Directional Formatting) marks, if needed for UI display purposes.
[[nodiscard]] BASE_I18N_EXPORT std::u16string
GetDisplayStringInLTRDirectionality(std::u16string_view text);

// Strip the beginning (U+202A..U+202B, U+202D..U+202E) and/or ending (U+202C)
// explicit bidi control characters from |text|, if there are any. Otherwise,
// return the text itself. Explicit bidi control characters display and have
// semantic effect. They can be deleted so they might not always appear in a
// pair.
[[nodiscard]] BASE_I18N_EXPORT std::u16string
StripWrappingBidiControlCharacters(std::u16string_view text);

}  // namespace i18n
}  // namespace base

#endif  // BASE_I18N_RTL_H_
