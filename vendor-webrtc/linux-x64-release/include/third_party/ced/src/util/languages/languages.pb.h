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

#ifndef UTIL_LANGUAGES_LANGUAGES_PB_H_
#define UTIL_LANGUAGES_LANGUAGES_PB_H_

enum Language {
  ENGLISH       = 0,
  DANISH        = 1,
  DUTCH         = 2,
  FINNISH       = 3,
  FRENCH        = 4,
  GERMAN        = 5,
  HEBREW        = 6,
  ITALIAN       = 7,
  JAPANESE      = 8,
  KOREAN        = 9,
  NORWEGIAN     = 10,
  POLISH        = 11,
  PORTUGUESE    = 12,
  RUSSIAN       = 13,
  SPANISH       = 14,
  SWEDISH       = 15,
  CHINESE       = 16,
  CZECH         = 17,
  GREEK         = 18,
  ICELANDIC     = 19,
  LATVIAN       = 20,
  LITHUANIAN    = 21,
  ROMANIAN      = 22,
  HUNGARIAN     = 23,
  ESTONIAN      = 24,
  TG_UNKNOWN_LANGUAGE   = 25,
  UNKNOWN_LANGUAGE      = 26,
  BULGARIAN     = 27,
  CROATIAN      = 28,
  SERBIAN       = 29,
  IRISH         = 30,      // UI only.
  GALICIAN      = 31,
  TAGALOG       = 32,      // Tagalog (tl) + Filipino (fil),
  TURKISH       = 33,
  UKRAINIAN     = 34,
  HINDI         = 35,
  MACEDONIAN    = 36,
  BENGALI       = 37,
  INDONESIAN    = 38,
  LATIN         = 39,      // UI only.
  MALAY         = 40,
  MALAYALAM     = 41,
  WELSH         = 42,      // UI only.
  NEPALI        = 43,
  TELUGU        = 44,
  ALBANIAN      = 45,
  TAMIL         = 46,
  BELARUSIAN    = 47,
  JAVANESE      = 48,      // UI only.
  OCCITAN       = 49,      // UI only.
  URDU          = 50,
  BIHARI        = 51,
  GUJARATI      = 52,
  THAI          = 53,
  ARABIC        = 54,
  CATALAN       = 55,
  ESPERANTO     = 56,
  BASQUE        = 57,
  INTERLINGUA   = 58,      // UI only.
  KANNADA       = 59,
  PUNJABI       = 60,
  SCOTS_GAELIC  = 61,      // UI only.
  SWAHILI       = 62,
  SLOVENIAN     = 63,
  MARATHI       = 64,
  MALTESE       = 65,
  VIETNAMESE    = 66,
  FRISIAN       = 67,      // UI only.
  SLOVAK        = 68,
  CHINESE_T     = 69,      // This is added to solve the problem of
                           // distinguishing Traditional and Simplified
                           // Chinese when the encoding is UTF8.
  FAROESE       = 70,      // UI only.
  SUNDANESE     = 71,      // UI only.
  UZBEK         = 72,
  AMHARIC       = 73,
  AZERBAIJANI   = 74,
  GEORGIAN      = 75,
  TIGRINYA      = 76,      // UI only.
  PERSIAN       = 77,
  BOSNIAN       = 78,      // UI only. LangId language: CROATIAN (28)
  SINHALESE     = 79,
  NORWEGIAN_N   = 80,      // UI only. LangId language: NORWEGIAN (10)
  PORTUGUESE_P  = 81,      // UI only. LangId language: PORTUGUESE (12)
  PORTUGUESE_B  = 82,      // UI only. LangId language: PORTUGUESE (12)
  XHOSA         = 83,      // UI only.
  ZULU          = 84,      // UI only.
  GUARANI       = 85,
  SESOTHO       = 86,      // UI only.
  TURKMEN       = 87,      // UI only.
  KYRGYZ        = 88,
  BRETON        = 89,      // UI only.
  TWI           = 90,      // UI only.
  YIDDISH       = 91,      // UI only.
  SERBO_CROATIAN= 92,      // UI only. LangId language: SERBIAN (29)
  SOMALI        = 93,      // UI only.
  UIGHUR        = 94,
  KURDISH       = 95,
  MONGOLIAN     = 96,
  ARMENIAN      = 97,
  LAOTHIAN      = 98,
  SINDHI        = 99,
  RHAETO_ROMANCE= 100,     // UI only.
  AFRIKAANS     = 101,
  LUXEMBOURGISH = 102,     // UI only.
  BURMESE       = 103,
  KHMER         = 104,
  TIBETAN       = 105,
  DHIVEHI       = 106,     // sometimes spelled Divehi, lang of Maldives
  CHEROKEE      = 107,
  SYRIAC        = 108,     // UI only.
  LIMBU         = 109,     // UI only.
  ORIYA         = 110,
  ASSAMESE      = 111,     // UI only.
  CORSICAN      = 112,     // UI only.
  INTERLINGUE   = 113,     // UI only.
  KAZAKH        = 114,
  LINGALA       = 115,     // UI only.
  MOLDAVIAN     = 116,     // UI only. LangId language: ROMANIAN (22)
  PASHTO        = 117,
  QUECHUA       = 118,     // UI only.
  SHONA         = 119,     // UI only.
  TAJIK         = 120,
  TATAR         = 121,     // UI only.
  TONGA         = 122,     // UI only.
  YORUBA        = 123,     // UI only.
  CREOLES_AND_PIDGINS_ENGLISH_BASED       = 124,   // UI only.
  CREOLES_AND_PIDGINS_FRENCH_BASED        = 125,   // UI only.
  CREOLES_AND_PIDGINS_PORTUGUESE_BASED    = 126,   // UI only.
  CREOLES_AND_PIDGINS_OTHER               = 127,   // UI only.
  MAORI         = 128,     // UI only.
  WOLOF         = 129,     // UI only.
  ABKHAZIAN     = 130,     // UI only.
  AFAR          = 131,     // UI only.
  AYMARA        = 132,     // UI only.
  BASHKIR       = 133,     // UI only.
  BISLAMA       = 134,     // UI only.
  DZONGKHA      = 135,     // UI only.
  FIJIAN        = 136,     // UI only.
  GREENLANDIC   = 137,     // UI only.
  HAUSA         = 138,     // UI only.
  HAITIAN_CREOLE= 139,     // UI only.
  INUPIAK       = 140,     // UI only.
  INUKTITUT     = 141,
  KASHMIRI      = 142,     // UI only.
  KINYARWANDA   = 143,     // UI only.
  MALAGASY      = 144,     // UI only.
  NAURU         = 145,     // UI only.
  OROMO         = 146,     // UI only.
  RUNDI         = 147,     // UI only.
  SAMOAN        = 148,     // UI only.
  SANGO         = 149,     // UI only.
  SANSKRIT      = 150,
  SISWANT       = 151,     // UI only.
  TSONGA        = 152,     // UI only.
  TSWANA        = 153,     // UI only.
  VOLAPUK       = 154,     // UI only.
  ZHUANG        = 155,     // UI only.
  KHASI         = 156,     // UI only.
  SCOTS         = 157,     // UI only.
  GANDA         = 158,     // UI only.
  MANX          = 159,     // UI only.
  MONTENEGRIN   = 160,     // UI only. LangId language: SERBIAN (29)
  NUM_LANGUAGES = 161,        // Always keep this at the end. It is not a
                              // valid Language enum. It is only used to
                              // indicate the total number of Languages.
  // NOTE: If you add a language, you will break a unittest. See the note
  // at the top of this enum.
};

#endif  // UTIL_LANGUAGES_LANGUAGES_PB_H_
