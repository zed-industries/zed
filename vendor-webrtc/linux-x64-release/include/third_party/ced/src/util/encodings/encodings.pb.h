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

#ifndef UTIL_ENCODINGS_ENCODINGS_PB_H_
#define UTIL_ENCODINGS_ENCODINGS_PB_H_

enum Encoding {
  ISO_8859_1           =  0,  // Teragram ASCII
  ISO_8859_2           =  1,  // Teragram Latin2
  ISO_8859_3           =  2,  // in BasisTech but not in Teragram
  ISO_8859_4           =  3,  // Teragram Latin4
  ISO_8859_5           =  4,  // Teragram ISO-8859-5
  ISO_8859_6           =  5,  // Teragram Arabic
  ISO_8859_7           =  6,  // Teragram Greek
  ISO_8859_8           =  7,  // Teragram Hebrew
  ISO_8859_9           =  8,  // in BasisTech but not in Teragram
  ISO_8859_10          =  9,  // in BasisTech but not in Teragram
  JAPANESE_EUC_JP      = 10,  // Teragram EUC_JP
  JAPANESE_SHIFT_JIS   = 11,  // Teragram SJS
  JAPANESE_JIS         = 12,  // Teragram JIS
  CHINESE_BIG5         = 13,  // Teragram BIG5
  CHINESE_GB           = 14,  // Teragram GB
  CHINESE_EUC_CN       = 15,  // Misnamed. Should be EUC_TW. Was Basis Tech
                              // CNS11643EUC, before that Teragram EUC-CN(!)
                              // See //i18n/basistech/basistech_encodings.h
  KOREAN_EUC_KR        = 16,  // Teragram KSC
  UNICODE              = 17,  // Teragram Unicode
  CHINESE_EUC_DEC      = 18,  // Misnamed. Should be EUC_TW. Was Basis Tech
                              // CNS11643EUC, before that Teragram EUC.
  CHINESE_CNS          = 19,  // Misnamed. Should be EUC_TW. Was Basis Tech
                              // CNS11643EUC, before that Teragram CNS.
  CHINESE_BIG5_CP950   = 20,  // Teragram BIG5_CP950
  JAPANESE_CP932       = 21,  // Teragram CP932
  UTF8                 = 22,
  UNKNOWN_ENCODING     = 23,
  ASCII_7BIT           = 24,  // ISO_8859_1 with all characters <= 127.
                              // Should be present only in the crawler
                              // and in the repository,
                              // *never* as a result of Document::encoding().
  RUSSIAN_KOI8_R       = 25,  // Teragram KOI8R
  RUSSIAN_CP1251       = 26,  // Teragram CP1251

  //----------------------------------------------------------
  // These are _not_ output from teragram. Instead, they are as
  // detected in the headers of usenet articles.
  MSFT_CP1252          = 27,  // 27: CP1252 aka MSFT euro ascii
  RUSSIAN_KOI8_RU      = 28,  // CP21866 aka KOI8-U, used for Ukrainian.
                              // Misnamed, this is _not_ KOI8-RU but KOI8-U.
                              // KOI8-U is used much more often than KOI8-RU.
  MSFT_CP1250          = 29,  // CP1250 aka MSFT eastern european
  ISO_8859_15          = 30,  // aka ISO_8859_0 aka ISO_8859_1 euroized
  //----------------------------------------------------------

  //----------------------------------------------------------
  // These are in BasisTech but not in Teragram. They are
  // needed for new interface languages. Now detected by
  // research langid
  MSFT_CP1254          = 31,  // used for Turkish
  MSFT_CP1257          = 32,  // used in Baltic countries
  //----------------------------------------------------------

  //----------------------------------------------------------
  //----------------------------------------------------------
  // New encodings detected by Teragram
  ISO_8859_11          = 33,  // aka TIS-620, used for Thai
  MSFT_CP874           = 34,  // used for Thai
  MSFT_CP1256          = 35,  // used for Arabic

  //----------------------------------------------------------
  // Detected as ISO_8859_8 by Teragram, but can be found in META tags
  MSFT_CP1255          = 36,  // Logical Hebrew Microsoft
  ISO_8859_8_I         = 37,  // Iso Hebrew Logical
  HEBREW_VISUAL        = 38,  // Iso Hebrew Visual
  //----------------------------------------------------------

  //----------------------------------------------------------
  // Detected by research langid
  CZECH_CP852          = 39,
  CZECH_CSN_369103     = 40,  // aka ISO_IR_139 aka KOI8_CS
  MSFT_CP1253          = 41,  // used for Greek
  RUSSIAN_CP866        = 42,
  //----------------------------------------------------------

  //----------------------------------------------------------
  // Handled by iconv in glibc
  ISO_8859_13          = 43,
  ISO_2022_KR          = 44,
  GBK                  = 45,
  GB18030              = 46,
  BIG5_HKSCS           = 47,
  ISO_2022_CN          = 48,

  //-----------------------------------------------------------
  // Detected by xin liu's detector
  // Handled by transcoder
  // (Indic encodings)

  TSCII                = 49,
  TAMIL_MONO           = 50,
  TAMIL_BI             = 51,
  JAGRAN               = 52,


  MACINTOSH_ROMAN      = 53,
  UTF7                 = 54,
  BHASKAR              = 55,  // Indic encoding - Devanagari
  HTCHANAKYA           = 56,  // 56 Indic encoding - Devanagari

  //-----------------------------------------------------------
  // These allow a single place (inputconverter and outputconverter)
  // to do UTF-16 <==> UTF-8 bulk conversions and UTF-32 <==> UTF-8
  // bulk conversions, with interchange-valid checking on input and
  // fallback if needed on ouput.
  UTF16BE              = 57,  // big-endian UTF-16
  UTF16LE              = 58,  // little-endian UTF-16
  UTF32BE              = 59,  // big-endian UTF-32
  UTF32LE              = 60,  // little-endian UTF-32
  //-----------------------------------------------------------

  //-----------------------------------------------------------
  // An encoding that means "This is not text, but it may have some
  // simple ASCII text embedded". Intended input conversion (not yet
  // implemented) is to keep strings of >=4 seven-bit ASCII characters
  // (follow each kept string with an ASCII space), delete the rest of
  // the bytes. This will pick up and allow indexing of e.g. captions
  // in JPEGs. No output conversion needed.
  BINARYENC            = 61,
  //-----------------------------------------------------------

  //-----------------------------------------------------------
  // Some Web pages allow a mixture of HZ-GB and GB-2312 by using
  // ~{ ... ~} for 2-byte pairs, and the browsers support this.
  HZ_GB_2312           = 62,
  //-----------------------------------------------------------

  //-----------------------------------------------------------
  // Some external vendors make the common input error of
  // converting MSFT_CP1252 to UTF8 *twice*. No output conversion needed.
  UTF8UTF8             = 63,
  //-----------------------------------------------------------

  //-----------------------------------------------------------
  // Handled by transcoder for tamil language specific font
  // encodings without the support for detection at present.
  TAM_ELANGO           = 64,  // Elango - Tamil
  TAM_LTTMBARANI       = 65,  // Barani - Tamil
  TAM_SHREE            = 66,  // Shree - Tamil
  TAM_TBOOMIS          = 67,  // TBoomis - Tamil
  TAM_TMNEWS           = 68,  // TMNews - Tamil
  TAM_WEBTAMIL         = 69,  // Webtamil - Tamil
  //-----------------------------------------------------------

  //-----------------------------------------------------------
  // Shift_JIS variants used by Japanese cell phone carriers.
  KDDI_SHIFT_JIS       = 70,
  DOCOMO_SHIFT_JIS     = 71,
  SOFTBANK_SHIFT_JIS   = 72,
  // ISO-2022-JP variants used by KDDI and SoftBank.
  KDDI_ISO_2022_JP     = 73,
  SOFTBANK_ISO_2022_JP = 74,
  //-----------------------------------------------------------

  NUM_ENCODINGS        = 75,  // Always keep this at the end. It is not a
                              // valid Encoding enum, it is only used to
                              // indicate the total number of Encodings.
};

#endif  // UTIL_ENCODINGS_ENCODINGS_PB_H_
