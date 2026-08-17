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

#ifndef COMPACT_ENC_DET_COMPACT_ENC_DET_GENERATED_TABLES_H_
#define COMPACT_ENC_DET_COMPACT_ENC_DET_GENERATED_TABLES_H_

#include "compact_enc_det/compact_enc_det.h"
#include "compact_enc_det/compact_enc_det_generated_tables2.h"
#include "util/basictypes.h"
#include "util/encodings/encodings.pb.h"

enum RankedEncoding {
  F_ASCII_7_bit,		// [0] encoding 24
  F_Latin1,		// [1] encoding 0
  F_UTF8,		// [2] encoding 22
  F_GB,		// [3] encoding 14
  F_CP1252,		// [4] encoding 27
  F_KSC,		// [5] encoding 16
  F_SJS,		// [6] encoding 11
  F_EUC_JP,		// [7] encoding 10
  F_BIG5,		// [8] encoding 13
  F_Latin2,		// [9] encoding 1
  F_CP1251,		// [10] encoding 26
  F_CP1256,		// [11] encoding 35
  F_CP1250,		// [12] encoding 29
  F_Latin5,		// [13] encoding 8
  F_ISO_8859_11,		// [14] encoding 33
  F_ISO_8859_15,		// [15] encoding 30
  F_CP1257,		// [16] encoding 32
  F_CP1255,		// [17] encoding 36
  F_KOI8R,		// [18] encoding 25
  F_GBK,		// [19] encoding 45
  F_Greek,		// [20] encoding 6
  F_JIS,		// [21] encoding 12
  F_CP1254,		// [22] encoding 31
  F_CP1253,		// [23] encoding 41
  F_CP932,		// [24] encoding 21
  F_Hebrew,		// [25] encoding 7
  F_KOI8U,		// [26] encoding 28
  F_ISO_8859_5,		// [27] encoding 4
  F_CP874,		// [28] encoding 34
  F_ISO_8859_13,		// [29] encoding 43
  F_Latin4,		// [30] encoding 3
  F_MACINTOSH,		// [31] encoding 53
  F_GB18030,		// [32] encoding 46
  F_CP852,		// [33] encoding 39
  F_Arabic,		// [34] encoding 5
  F_BIG5_HKSCS,		// [35] encoding 47
  F_CP866,		// [36] encoding 42
  F_UTF_16BE,		// [37] encoding 57
  F_Latin3,		// [38] encoding 2
  F_UTF_16LE,		// [39] encoding 58
  F_HZ_GB_2312,		// [40] encoding 62
  F_CSN_369103,		// [41] encoding 40
  F_ISO_2022_KR,		// [42] encoding 44
  F_Latin6,		// [43] encoding 9
  F_UTF7,		// [44] encoding 54
  F_ISO_2022_CN,		// [45] encoding 48
  F_BIG5_CP950,		// [46] encoding 20
  F_JAGRAN,		// [47] encoding 52
  F_BHASKAR,		// [48] encoding 55
  F_HTCHANAKYA,		// [49] encoding 56
  F_TSCII,		// [50] encoding 49
  F_TAM,		// [51] encoding 50
  F_TAB,		// [52] encoding 51
  F_EUC_CN,		// [53] encoding 15
  F_EUC,		// [54] encoding 18
  F_CNS,		// [55] encoding 19
  F_UTF_32BE,		// [56] encoding 59
  F_UTF_32LE,		// [57] encoding 60
  F_X_BINARYENC,		// [58] encoding 61
  F_X_UTF8UTF8,		// [59] encoding 63
  F_X_TAM_ELANGO,		// [60] encoding 64
  F_X_TAM_LTTMBARANI,		// [61] encoding 65
  F_X_TAM_SHREE,		// [62] encoding 66
  F_X_TAM_TBOOMIS,		// [63] encoding 67
  F_X_TAM_TMNEWS,		// [64] encoding 68
  F_X_TAM_WEBTAMIL,		// [65] encoding 69
  F_UTF8CP1252,		// [66] encoding 63
  NUM_RANKEDENCODING
};

static const Encoding kMapToEncoding[NUM_RANKEDENCODING] = {
  ASCII_7BIT,		// encoding 24
  ISO_8859_1,		// encoding 0
  UTF8,		// encoding 22
  CHINESE_GB,		// encoding 14
  MSFT_CP1252,		// encoding 27
  KOREAN_EUC_KR,		// encoding 16
  JAPANESE_SHIFT_JIS,		// encoding 11
  JAPANESE_EUC_JP,		// encoding 10
  CHINESE_BIG5,		// encoding 13
  ISO_8859_2,		// encoding 1
  RUSSIAN_CP1251,		// encoding 26
  MSFT_CP1256,		// encoding 35
  MSFT_CP1250,		// encoding 29
  ISO_8859_9,		// encoding 8
  ISO_8859_11,		// encoding 33
  ISO_8859_15,		// encoding 30
  MSFT_CP1257,		// encoding 32
  MSFT_CP1255,		// encoding 36
  RUSSIAN_KOI8_R,		// encoding 25
  GBK,		// encoding 45
  ISO_8859_7,		// encoding 6
  JAPANESE_JIS,		// encoding 12
  MSFT_CP1254,		// encoding 31
  MSFT_CP1253,		// encoding 41
  JAPANESE_CP932,		// encoding 21
  ISO_8859_8,		// encoding 7
  RUSSIAN_KOI8_RU,		// encoding 28
  ISO_8859_5,		// encoding 4
  MSFT_CP874,		// encoding 34
  ISO_8859_13,		// encoding 43
  ISO_8859_4,		// encoding 3
  MACINTOSH_ROMAN,		// encoding 53
  GB18030,		// encoding 46
  CZECH_CP852,		// encoding 39
  ISO_8859_6,		// encoding 5
  BIG5_HKSCS,		// encoding 47
  RUSSIAN_CP866,		// encoding 42
  UTF16BE,		// encoding 57
  ISO_8859_3,		// encoding 2
  UTF16LE,		// encoding 58
  HZ_GB_2312,		// encoding 62
  CZECH_CSN_369103,		// encoding 40
  ISO_2022_KR,		// encoding 44
  ISO_8859_10,		// encoding 9
  UTF7,		// encoding 54
  ISO_2022_CN,		// encoding 48
  CHINESE_BIG5_CP950,		// encoding 20
  JAGRAN,		// encoding 52
  BHASKAR,		// encoding 55
  HTCHANAKYA,		// encoding 56
  TSCII,		// encoding 49
  TAMIL_MONO,		// encoding 50
  TAMIL_BI,		// encoding 51
  CHINESE_EUC_CN,		// encoding 15
  CHINESE_EUC_DEC,		// encoding 18
  CHINESE_CNS,		// encoding 19
  UTF32BE,		// encoding 59
  UTF32LE,		// encoding 60
  BINARYENC,		// encoding 61
  UTF8UTF8,		// encoding 63
  TAM_ELANGO,		// encoding 64
  TAM_LTTMBARANI,		// encoding 65
  TAM_SHREE,		// encoding 66
  TAM_TBOOMIS,		// encoding 67
  TAM_TMNEWS,		// encoding 68
  TAM_WEBTAMIL,		// encoding 69
  UTF8UTF8,		// encoding 63
};

// Massaged TLD or charset, followed by packed encoding probs
typedef struct {
  char key_prob[20];
} HintEntry;

static const HintEntry kLangHintProbs[] = {	// MaxRange 192
  {{0x61,0x62,0x6b,0x68,0x61,0x7a,0x69,0x61, 0x21,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "abkhazia"
      // UTF8=191  [top UTF8]
  {{0x61,0x66,0x61,0x72,0x5f,0x5f,0x5f,0x5f, 0x21,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "afar____"
      // UTF8=191  [top UTF8]
  {{0x61,0x66,0x72,0x69,0x6b,0x61,0x61,0x6e, 0x11,0xb6,0x21,0xac,0xa1,0xa2,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "afrikaan"
      // Latin1=182  CP1252=172  ISO-8859-15=162  [top Latin1]
  {{0x61,0x6c,0x62,0x61,0x6e,0x69,0x61,0x6e, 0x11,0xb6,0x21,0xac,0xa1,0xa2,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "albanian"
      // Latin1=182  CP1252=172  ISO-8859-15=162  [top Latin1]
  {{0x61,0x6d,0x68,0x61,0x72,0x69,0x63,0x5f, 0x21,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "amharic_"
      // UTF8=191  [top UTF8]
  {{0x61,0x72,0x61,0x62,0x69,0x63,0x5f,0x5f, 0x03,0x84,0x53,0xa2,0x11,0x3b,0x62,0xbc,0x34,0x10,0x51,0x83,}}, // "arabic__"
      // ASCII-7-bit=132  Latin1=83  UTF8=162  CP1252=59  CP1256=188  CP1250=52  Arabic=131  [top CP1256]
  {{0x61,0x72,0x6d,0x65,0x6e,0x69,0x61,0x6e, 0x01,0x5f,0x11,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "armenian"
      // ASCII-7-bit=95  UTF8=190  [top UTF8]
  {{0x61,0x73,0x73,0x61,0x6d,0x65,0x73,0x65, 0x21,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "assamese"
      // UTF8=191  [top UTF8]
  {{0x61,0x79,0x6d,0x61,0x72,0x61,0x5f,0x5f, 0x21,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "aymara__"
      // UTF8=191  [top UTF8]
  {{0x61,0x7a,0x65,0x72,0x62,0x61,0x69,0x6a, 0x21,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "azerbaij"
      // UTF8=191  [top UTF8]
  {{0x62,0x61,0x73,0x68,0x6b,0x69,0x72,0x5f, 0x21,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "bashkir_"
      // UTF8=191  [top UTF8]
  {{0x62,0x61,0x73,0x71,0x75,0x65,0x5f,0x5f, 0x11,0xb6,0x21,0xac,0xa1,0xa2,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "basque__"
      // Latin1=182  CP1252=172  ISO-8859-15=162  [top Latin1]
  {{0x62,0x65,0x6c,0x61,0x72,0x75,0x73,0x69, 0xa1,0xb5,0x71,0xa1,0x72,0x97,0xab,0x81,0x8d,0x00,0x00,0x00,}}, // "belarusi"
      // CP1251=181  KOI8R=161  KOI8U=151  ISO-8859-5=171  CP866=141  [top CP1251]
  {{0x62,0x65,0x6e,0x67,0x61,0x6c,0x69,0x5f, 0x11,0xb6,0x21,0xac,0xa1,0xa2,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "bengali_"
      // Latin1=182  CP1252=172  ISO-8859-15=162  [top Latin1]
  {{0x62,0x69,0x68,0x61,0x72,0x69,0x5f,0x5f, 0x11,0xb6,0x21,0xac,0xa1,0xa2,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "bihari__"
      // Latin1=182  CP1252=172  ISO-8859-15=162  [top Latin1]
  {{0x62,0x69,0x73,0x6c,0x61,0x6d,0x61,0x5f, 0x21,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "bislama_"
      // UTF8=191  [top UTF8]
  {{0x62,0x6f,0x73,0x6e,0x69,0x61,0x6e,0x5f, 0x91,0xaf,0x21,0xb9,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "bosnian_"
      // Latin2=175  CP1250=185  [top CP1250]
  {{0x62,0x72,0x65,0x74,0x6f,0x6e,0x5f,0x5f, 0x11,0xb5,0x21,0x97,0x81,0xab,0x11,0xa1,0x00,0x00,0x00,0x00,}}, // "breton__"
      // Latin1=181  CP1252=151  Latin5=171  ISO-8859-15=161  [top Latin1]
  {{0x62,0x75,0x6c,0x67,0x61,0x72,0x69,0x61, 0x03,0x70,0x47,0xad,0x11,0x45,0x51,0xb5,0x71,0x95,0x81,0x9f,}}, // "bulgaria"
      // ASCII-7-bit=112  Latin1=71  UTF8=173  CP1252=69  CP1251=181  KOI8R=149  ISO-8859-5=159  [top CP1251]
  {{0x62,0x75,0x72,0x6d,0x65,0x73,0x65,0x5f, 0x21,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "burmese_"
      // UTF8=191  [top UTF8]
  {{0x63,0x61,0x74,0x61,0x6c,0x61,0x6e,0x5f, 0x03,0x8b,0xb8,0xa0,0x11,0xa4,0xa1,0x96,0x10,0x61,0x31,0x00,}}, // "catalan_"
      // ASCII-7-bit=139  Latin1=184  UTF8=160  CP1252=164  ISO-8859-15=150  Latin3=49  [top Latin1]
  {{0x63,0x68,0x65,0x72,0x6f,0x6b,0x65,0x65, 0x21,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "cherokee"
      // UTF8=191  [top UTF8]
  {{0x63,0x68,0x69,0x6e,0x65,0x73,0x65,0x5f, 0x01,0x5c,0x12,0xa8,0xbb,0x11,0x74,0x21,0x6d,0xa1,0x7d,0x00,}}, // "chinese_"
      // ASCII-7-bit=92  UTF8=168  GB=187  KSC=116  BIG5=109  GBK=125  [top GB]
  {{0x63,0x68,0x69,0x6e,0x65,0x73,0x65,0x74, 0x06,0x73,0x5f,0xad,0x59,0x43,0x36,0x21,0xb9,0x10,0xa1,0x38,}}, // "chineset"
      // ASCII-7-bit=115  Latin1=95  UTF8=173  GB=89  CP1252=67  KSC=54  BIG5=185  BIG5_HKSCS=56  [top BIG5]
  {{0x63,0x6f,0x72,0x73,0x69,0x63,0x61,0x6e, 0x12,0xaf,0xb9,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "corsican"
      // Latin1=175  UTF8=185  [top UTF8]
  {{0x63,0x72,0x65,0x6f,0x6c,0x65,0x73,0x61, 0x21,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "creolesa"
      // UTF8=191  [top UTF8]
  {{0x63,0x72,0x6f,0x61,0x74,0x69,0x61,0x6e, 0x03,0x91,0x7b,0xa6,0x11,0x86,0x41,0xac,0x21,0xb4,0x31,0x4d,}}, // "croatian"
      // ASCII-7-bit=145  Latin1=123  UTF8=166  CP1252=134  Latin2=172  CP1250=180  CP1257=77  [top CP1250]
  {{0x63,0x7a,0x65,0x63,0x68,0x5f,0x5f,0x5f, 0x01,0x89,0x11,0xb1,0x61,0x98,0x21,0xb5,0x10,0x41,0x7d,0x00,}}, // "czech___"
      // ASCII-7-bit=137  UTF8=177  Latin2=152  CP1250=181  CP852=125  [top CP1250]
  {{0x64,0x61,0x6e,0x69,0x73,0x68,0x5f,0x5f, 0x03,0x99,0xb8,0xa6,0x11,0x9a,0x41,0x38,0x21,0x32,0x21,0x84,}}, // "danish__"
      // ASCII-7-bit=153  Latin1=184  UTF8=166  CP1252=154  Latin2=56  CP1250=50  ISO-8859-15=132  [top Latin1]
  {{0x64,0x68,0x69,0x76,0x65,0x68,0x69,0x5f, 0x21,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "dhivehi_"
      // UTF8=191  [top UTF8]
  {{0x64,0x75,0x74,0x63,0x68,0x5f,0x5f,0x5f, 0x03,0xb1,0xae,0xa3,0x11,0xa1,0x41,0x41,0x21,0x44,0x21,0x7f,}}, // "dutch___"
      // ASCII-7-bit=177  Latin1=174  UTF8=163  CP1252=161  Latin2=65  CP1250=68  ISO-8859-15=127  [top ASCII-7-bit]
  {{0x64,0x7a,0x6f,0x6e,0x67,0x6b,0x68,0x61, 0x21,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "dzongkha"
      // UTF8=191  [top UTF8]
  {{0x65,0x6e,0x67,0x6c,0x69,0x73,0x68,0x5f, 0x06,0xb9,0xa0,0xa2,0x5d,0x94,0x55,0x21,0x56,0x61,0x69,0x00,}}, // "english_"
      // ASCII-7-bit=185  Latin1=160  UTF8=162  GB=93  CP1252=148  KSC=85  BIG5=86  ISO-8859-15=105  [top ASCII-7-bit]
  {{0x65,0x73,0x70,0x65,0x72,0x61,0x6e,0x74, 0x03,0x89,0xb4,0xa2,0x12,0xaa,0x45,0x61,0x4c,0x21,0xa0,0x00,}}, // "esperant"
      // ASCII-7-bit=137  Latin1=180  UTF8=162  CP1252=170  KSC=69  CP1250=76  ISO-8859-15=160  [top Latin1]
  {{0x65,0x73,0x74,0x6f,0x6e,0x69,0x61,0x6e, 0x03,0x90,0xab,0xb1,0x11,0x91,0xa2,0x7e,0xa3,0xc2,0x8e,0x98,}}, // "estonian"
      // ASCII-7-bit=144  Latin1=171  UTF8=177  CP1252=145  ISO-8859-15=126  CP1257=163  ISO-8859-13=142  Latin4=152  [top UTF8]
  {{0x66,0x61,0x72,0x6f,0x65,0x73,0x65,0x5f, 0x11,0xb6,0x21,0xac,0xa1,0xa2,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "faroese_"
      // Latin1=182  CP1252=172  ISO-8859-15=162  [top Latin1]
  {{0x66,0x69,0x6a,0x69,0x61,0x6e,0x5f,0x5f, 0x21,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "fijian__"
      // UTF8=191  [top UTF8]
  {{0x66,0x69,0x6e,0x6e,0x69,0x73,0x68,0x5f, 0x03,0x96,0xb7,0xa9,0x11,0x9c,0x71,0x42,0x22,0x8b,0x39,0x00,}}, // "finnish_"
      // ASCII-7-bit=150  Latin1=183  UTF8=169  CP1252=156  CP1250=66  ISO-8859-15=139  CP1257=57  [top Latin1]
  {{0x66,0x72,0x65,0x6e,0x63,0x68,0x5f,0x5f, 0x03,0x99,0xb6,0xaa,0x11,0xa0,0x62,0x4f,0x46,0x21,0x86,0x00,}}, // "french__"
      // ASCII-7-bit=153  Latin1=182  UTF8=170  CP1252=160  CP1256=79  CP1250=70  ISO-8859-15=134  [top Latin1]
  {{0x66,0x72,0x69,0x73,0x69,0x61,0x6e,0x5f, 0x11,0xb6,0x21,0xac,0xa1,0xa2,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "frisian_"
      // Latin1=182  CP1252=172  ISO-8859-15=162  [top Latin1]
  {{0x67,0x61,0x6c,0x69,0x63,0x69,0x61,0x6e, 0x11,0xb6,0x21,0xac,0xa1,0xa2,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "galician"
      // Latin1=182  CP1252=172  ISO-8859-15=162  [top Latin1]
  {{0x67,0x61,0x6e,0x64,0x61,0x5f,0x5f,0x5f, 0x21,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "ganda___"
      // UTF8=191  [top UTF8]
  {{0x67,0x65,0x6f,0x72,0x67,0x69,0x61,0x6e, 0x01,0x6c,0x11,0xbe,0x11,0x1c,0x10,0x21,0x1c,0x00,0x00,0x00,}}, // "georgian"
      // ASCII-7-bit=108  UTF8=190  CP1252=28  CP1253=28  [top UTF8]
  {{0x67,0x65,0x72,0x6d,0x61,0x6e,0x5f,0x5f, 0x03,0xa2,0xb7,0xa6,0x11,0x9b,0x41,0x56,0x21,0x5d,0x21,0x7c,}}, // "german__"
      // ASCII-7-bit=162  Latin1=183  UTF8=166  CP1252=155  Latin2=86  CP1250=93  ISO-8859-15=124  [top Latin1]
  {{0x67,0x72,0x65,0x65,0x6b,0x5f,0x5f,0x5f, 0x03,0x81,0x54,0xad,0x11,0x52,0xd1,0x31,0x11,0xb4,0x21,0xa6,}}, // "greek___"
      // ASCII-7-bit=129  Latin1=84  UTF8=173  CP1252=82  KOI8R=49  Greek=180  CP1253=166  [top Greek]
  {{0x67,0x72,0x65,0x65,0x6e,0x6c,0x61,0x6e, 0x21,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "greenlan"
      // UTF8=191  [top UTF8]
  {{0x67,0x75,0x61,0x72,0x61,0x6e,0x69,0x5f, 0x11,0xb9,0x20,0x91,0xaf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "guarani_"
      // Latin1=185  Latin6=175  [top Latin1]
  {{0x67,0x75,0x6a,0x61,0x72,0x61,0x74,0x69, 0x03,0x79,0xb6,0x76,0x11,0xac,0xa1,0xa2,0x00,0x00,0x00,0x00,}}, // "gujarati"
      // ASCII-7-bit=121  Latin1=182  UTF8=118  CP1252=172  ISO-8859-15=162  [top Latin1]
  {{0x68,0x61,0x69,0x74,0x69,0x61,0x6e,0x63, 0x21,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "haitianc"
      // UTF8=191  [top UTF8]
  {{0x68,0x61,0x75,0x73,0x61,0x5f,0x5f,0x5f, 0x21,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "hausa___"
      // UTF8=191  [top UTF8]
  {{0x68,0x65,0x62,0x72,0x65,0x77,0x5f,0x5f, 0x03,0x76,0x46,0xab,0x11,0x3b,0x51,0x32,0x61,0xb8,0x71,0x9f,}}, // "hebrew__"
      // ASCII-7-bit=118  Latin1=70  UTF8=171  CP1252=59  CP1251=50  CP1255=184  Hebrew=159  [top CP1255]
  {{0x68,0x69,0x6e,0x64,0x69,0x5f,0x5f,0x5f, 0x11,0xb5,0x21,0xab,0xa1,0xa1,0x10,0xf3,0x97,0x8d,0x83,0x00,}}, // "hindi___"
      // Latin1=181  CP1252=171  ISO-8859-15=161  JAGRAN=151  BHASKAR=141  HTCHANAKYA=131  [top Latin1]
  {{0x68,0x75,0x6e,0x67,0x61,0x72,0x69,0x61, 0x03,0x93,0x9f,0xad,0x11,0x6f,0x41,0xae,0x21,0xa9,0x21,0x40,}}, // "hungaria"
      // ASCII-7-bit=147  Latin1=159  UTF8=173  CP1252=111  Latin2=174  CP1250=169  ISO-8859-15=64  [top Latin2]
  {{0x69,0x63,0x65,0x6c,0x61,0x6e,0x64,0x69, 0x03,0x7f,0xb8,0x9c,0x11,0xa4,0x11,0x1d,0x51,0x2f,0x21,0x99,}}, // "icelandi"
      // ASCII-7-bit=127  Latin1=184  UTF8=156  CP1252=164  SJS=29  CP1250=47  ISO-8859-15=153  [top Latin1]
  {{0x69,0x6e,0x64,0x6f,0x6e,0x65,0x73,0x69, 0x03,0xb2,0xae,0x99,0x11,0xa2,0x11,0x5b,0x41,0x70,0x31,0x91,}}, // "indonesi"
      // ASCII-7-bit=178  Latin1=174  UTF8=153  CP1252=162  SJS=91  CP1256=112  ISO-8859-15=145  [top ASCII-7-bit]
  {{0x69,0x6e,0x74,0x65,0x72,0x6c,0x69,0x6e, 0x12,0xb0,0xb0,0x11,0xa6,0xa1,0x9c,0x00,0x00,0x00,0x00,0x00,}}, // "interlin"
      // Latin1=176  UTF8=176  CP1252=166  ISO-8859-15=156  [top Latin1]
  {{0x69,0x6e,0x75,0x6b,0x74,0x69,0x74,0x75, 0x21,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "inuktitu"
      // UTF8=191  [top UTF8]
  {{0x69,0x6e,0x75,0x70,0x69,0x61,0x6b,0x5f, 0x21,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "inupiak_"
      // UTF8=191  [top UTF8]
  {{0x69,0x72,0x69,0x73,0x68,0x5f,0x5f,0x5f, 0x11,0xb6,0x21,0xac,0xa1,0xa2,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "irish___"
      // Latin1=182  CP1252=172  ISO-8859-15=162  [top Latin1]
  {{0x69,0x74,0x61,0x6c,0x69,0x61,0x6e,0x5f, 0x03,0xa7,0xb4,0xa4,0x11,0xa4,0x41,0x4d,0x21,0x55,0x21,0x78,}}, // "italian_"
      // ASCII-7-bit=167  Latin1=180  UTF8=164  CP1252=164  Latin2=77  CP1250=85  ISO-8859-15=120  [top Latin1]
  {{0x6a,0x61,0x70,0x61,0x6e,0x65,0x73,0x65, 0x01,0x68,0x11,0xa7,0x32,0xb4,0xad,0xd1,0x78,0x21,0x62,0x00,}}, // "japanese"
      // ASCII-7-bit=104  UTF8=167  SJS=180  EUC-JP=173  JIS=120  CP932=98  [top SJS]
  {{0x6a,0x61,0x76,0x61,0x6e,0x65,0x73,0x65, 0x11,0xb6,0x21,0xac,0xa1,0xa2,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "javanese"
      // Latin1=182  CP1252=172  ISO-8859-15=162  [top Latin1]
  {{0x6b,0x61,0x6e,0x6e,0x61,0x64,0x61,0x5f, 0x03,0x65,0xb6,0x81,0x11,0xac,0xa1,0xa2,0x00,0x00,0x00,0x00,}}, // "kannada_"
      // ASCII-7-bit=101  Latin1=182  UTF8=129  CP1252=172  ISO-8859-15=162  [top Latin1]
  {{0x6b,0x61,0x73,0x68,0x6d,0x69,0x72,0x69, 0x21,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "kashmiri"
      // UTF8=191  [top UTF8]
  {{0x6b,0x61,0x7a,0x61,0x6b,0x68,0x5f,0x5f, 0x21,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "kazakh__"
      // UTF8=191  [top UTF8]
  {{0x6b,0x68,0x61,0x73,0x69,0x5f,0x5f,0x5f, 0x21,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "khasi___"
      // UTF8=191  [top UTF8]
  {{0x6b,0x68,0x6d,0x65,0x72,0x5f,0x5f,0x5f, 0x21,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "khmer___"
      // UTF8=191  [top UTF8]
  {{0x6b,0x69,0x6e,0x79,0x61,0x72,0x77,0x61, 0x21,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "kinyarwa"
      // UTF8=191  [top UTF8]
  {{0x6b,0x6f,0x72,0x65,0x61,0x6e,0x5f,0x5f, 0x06,0x5d,0x34,0x9d,0x20,0x1a,0xbd,0x11,0x0c,0x20,0x21,0x76,}}, // "korean__"
      // ASCII-7-bit=93  Latin1=52  UTF8=157  GB=32  CP1252=26  KSC=189  EUC-JP=12  ISO-2022-KR=118  [top KSC]
  {{0x6b,0x75,0x72,0x64,0x69,0x73,0x68,0x5f, 0xb1,0xb9,0x10,0x61,0xaf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "kurdish_"
      // CP1256=185  Arabic=175  [top CP1256]
  {{0x6b,0x79,0x72,0x67,0x79,0x7a,0x5f,0x5f, 0x10,0x61,0xaf,0x41,0xb9,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "kyrgyz__"
      // CP1254=175  ISO-8859-5=185  [top ISO-8859-5]
  {{0x6c,0x61,0x6f,0x74,0x68,0x69,0x61,0x6e, 0x01,0x40,0x11,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "laothian"
      // ASCII-7-bit=64  UTF8=190  [top UTF8]
  {{0x6c,0x61,0x74,0x69,0x6e,0x5f,0x5f,0x5f, 0x11,0xb6,0x21,0xac,0xa1,0xa2,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "latin___"
      // Latin1=182  CP1252=172  ISO-8859-15=162  [top Latin1]
  {{0x6c,0x61,0x74,0x76,0x69,0x61,0x6e,0x5f, 0x03,0x80,0x55,0xac,0x11,0x64,0xb1,0xb4,0xc2,0x99,0xa3,0x00,}}, // "latvian_"
      // ASCII-7-bit=128  Latin1=85  UTF8=172  CP1252=100  CP1257=180  ISO-8859-13=153  Latin4=163  [top CP1257]
  {{0x6c,0x69,0x6d,0x62,0x75,0x5f,0x5f,0x5f, 0x21,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "limbu___"
      // UTF8=191  [top UTF8]
  {{0x6c,0x69,0x6e,0x67,0x61,0x6c,0x61,0x5f, 0x21,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "lingala_"
      // UTF8=191  [top UTF8]
  {{0x6c,0x69,0x74,0x68,0x75,0x61,0x6e,0x69, 0x03,0x7c,0x5d,0xaa,0x11,0x73,0xb1,0xb7,0xc2,0x94,0x9d,0x00,}}, // "lithuani"
      // ASCII-7-bit=124  Latin1=93  UTF8=170  CP1252=115  CP1257=183  ISO-8859-13=148  Latin4=157  [top CP1257]
  {{0x6c,0x75,0x78,0x65,0x6d,0x62,0x6f,0x75, 0x11,0xb6,0x21,0xac,0xa1,0xa2,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "luxembou"
      // Latin1=182  CP1252=172  ISO-8859-15=162  [top Latin1]
  {{0x6d,0x61,0x63,0x65,0x64,0x6f,0x6e,0x69, 0x03,0x7a,0x54,0xa9,0x11,0x4b,0x51,0xb3,0x71,0x9e,0x81,0xa8,}}, // "macedoni"
      // ASCII-7-bit=122  Latin1=84  UTF8=169  CP1252=75  CP1251=179  KOI8R=158  ISO-8859-5=168  [top CP1251]
  {{0x6d,0x61,0x6c,0x61,0x67,0x61,0x73,0x79, 0x21,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "malagasy"
      // UTF8=191  [top UTF8]
  {{0x6d,0x61,0x6c,0x61,0x79,0x5f,0x5f,0x5f, 0x11,0xb6,0x21,0xac,0xa1,0xa2,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "malay___"
      // Latin1=182  CP1252=172  ISO-8859-15=162  [top Latin1]
  {{0x6d,0x61,0x6c,0x61,0x79,0x61,0x6c,0x61, 0x03,0x48,0xb6,0x81,0x11,0xac,0xa1,0xa2,0x00,0x00,0x00,0x00,}}, // "malayala"
      // ASCII-7-bit=72  Latin1=182  UTF8=129  CP1252=172  ISO-8859-15=162  [top Latin1]
  {{0x6d,0x61,0x6c,0x74,0x65,0x73,0x65,0x5f, 0x11,0xb6,0x21,0xac,0xa1,0xa2,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "maltese_"
      // Latin1=182  CP1252=172  ISO-8859-15=162  [top Latin1]
  {{0x6d,0x61,0x6e,0x78,0x5f,0x5f,0x5f,0x5f, 0x21,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "manx____"
      // UTF8=191  [top UTF8]
  {{0x6d,0x61,0x6f,0x72,0x69,0x5f,0x5f,0x5f, 0x21,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "maori___"
      // UTF8=191  [top UTF8]
  {{0x6d,0x61,0x72,0x61,0x74,0x68,0x69,0x5f, 0x21,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "marathi_"
      // UTF8=191  [top UTF8]
  {{0x6d,0x6f,0x6c,0x64,0x61,0x76,0x69,0x61, 0x21,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "moldavia"
      // UTF8=191  [top UTF8]
  {{0x6d,0x6f,0x6e,0x67,0x6f,0x6c,0x69,0x61, 0xa1,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "mongolia"
      // CP1251=191  [top CP1251]
  {{0x6e,0x61,0x75,0x72,0x75,0x5f,0x5f,0x5f, 0x21,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "nauru___"
      // UTF8=191  [top UTF8]
  {{0x6e,0x65,0x70,0x61,0x6c,0x69,0x5f,0x5f, 0x21,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "nepali__"
      // UTF8=191  [top UTF8]
  {{0x6e,0x6f,0x72,0x77,0x65,0x67,0x69,0x61, 0x03,0x92,0xb8,0xa8,0x11,0x9c,0x41,0x30,0x31,0x24,0x11,0x8e,}}, // "norwegia"
      // ASCII-7-bit=146  Latin1=184  UTF8=168  CP1252=156  Latin2=48  Latin5=36  ISO-8859-15=142  [top Latin1]
  {{0x6f,0x63,0x63,0x69,0x74,0x61,0x6e,0x5f, 0x11,0xb6,0x21,0xac,0xa1,0xa2,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "occitan_"
      // Latin1=182  CP1252=172  ISO-8859-15=162  [top Latin1]
  {{0x6f,0x72,0x69,0x79,0x61,0x5f,0x5f,0x5f, 0x11,0xb6,0x21,0xac,0xa1,0xa2,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "oriya___"
      // Latin1=182  CP1252=172  ISO-8859-15=162  [top Latin1]
  {{0x6f,0x72,0x6f,0x6d,0x6f,0x5f,0x5f,0x5f, 0x21,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "oromo___"
      // UTF8=191  [top UTF8]
  {{0x70,0x61,0x73,0x68,0x74,0x6f,0x5f,0x5f, 0xb1,0xb9,0x10,0x61,0xaf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "pashto__"
      // CP1256=185  Arabic=175  [top CP1256]
  {{0x70,0x65,0x72,0x73,0x69,0x61,0x6e,0x5f, 0x12,0x44,0xb6,0x11,0x33,0x62,0xae,0x19,0x10,0x51,0x9f,0x00,}}, // "persian_"
      // Latin1=68  UTF8=182  CP1252=51  CP1256=174  CP1250=25  Arabic=159  [top UTF8]
  {{0x70,0x6f,0x6c,0x69,0x73,0x68,0x5f,0x5f, 0x05,0x85,0x6c,0xa8,0x26,0x57,0x41,0xb9,0x21,0x99,0x31,0x23,}}, // "polish__"
      // ASCII-7-bit=133  Latin1=108  UTF8=168  GB=38  CP1252=87  Latin2=185  CP1250=153  CP1257=35  [top Latin2]
  {{0x70,0x6f,0x72,0x74,0x75,0x67,0x75,0x65, 0x03,0x96,0xb9,0xa6,0x11,0x9a,0x11,0x30,0x51,0x36,0x21,0x86,}}, // "portugue"
      // ASCII-7-bit=150  Latin1=185  UTF8=166  CP1252=154  SJS=48  CP1250=54  ISO-8859-15=134  [top Latin1]
  {{0x70,0x75,0x6e,0x6a,0x61,0x62,0x69,0x5f, 0x03,0x42,0xb6,0x7b,0x11,0xac,0xa1,0xa2,0x00,0x00,0x00,0x00,}}, // "punjabi_"
      // ASCII-7-bit=66  Latin1=182  UTF8=123  CP1252=172  ISO-8859-15=162  [top Latin1]
  {{0x71,0x75,0x65,0x63,0x68,0x75,0x61,0x5f, 0x21,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "quechua_"
      // UTF8=191  [top UTF8]
  {{0x72,0x68,0x61,0x65,0x74,0x6f,0x72,0x6f, 0x11,0xb6,0x21,0xac,0xa1,0xa2,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "rhaetoro"
      // Latin1=182  CP1252=172  ISO-8859-15=162  [top Latin1]
  {{0x72,0x6f,0x6d,0x61,0x6e,0x69,0x61,0x6e, 0x03,0xb2,0x9d,0xa5,0x11,0x92,0x42,0xa7,0x51,0x11,0x99,0x00,}}, // "romanian"
      // ASCII-7-bit=178  Latin1=157  UTF8=165  CP1252=146  Latin2=167  CP1251=81  CP1250=153  [top ASCII-7-bit]
  {{0x72,0x75,0x6e,0x64,0x69,0x5f,0x5f,0x5f, 0x21,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "rundi___"
      // UTF8=191  [top UTF8]
  {{0x72,0x75,0x73,0x73,0x69,0x61,0x6e,0x5f, 0x01,0x74,0x11,0xa9,0x71,0xb9,0x71,0x99,0x81,0x82,0x81,0x6d,}}, // "russian_"
      // ASCII-7-bit=116  UTF8=169  CP1251=185  KOI8R=153  ISO-8859-5=130  CP866=109  [top CP1251]
  {{0x73,0x61,0x6d,0x6f,0x61,0x6e,0x5f,0x5f, 0x21,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "samoan__"
      // UTF8=191  [top UTF8]
  {{0x73,0x61,0x6e,0x67,0x6f,0x5f,0x5f,0x5f, 0x21,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "sango___"
      // UTF8=191  [top UTF8]
  {{0x73,0x61,0x6e,0x73,0x6b,0x72,0x69,0x74, 0x21,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "sanskrit"
      // UTF8=191  [top UTF8]
  {{0x73,0x63,0x6f,0x74,0x73,0x5f,0x5f,0x5f, 0x21,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "scots___"
      // UTF8=191  [top UTF8]
  {{0x73,0x63,0x6f,0x74,0x73,0x67,0x61,0x65, 0x11,0xb6,0x21,0xac,0xa1,0xa2,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "scotsgae"
      // Latin1=182  CP1252=172  ISO-8859-15=162  [top Latin1]
  {{0x73,0x65,0x72,0x62,0x69,0x61,0x6e,0x5f, 0x03,0x93,0x77,0xad,0x11,0x85,0x42,0xad,0x52,0x12,0xae,0x4a,}}, // "serbian_"
      // ASCII-7-bit=147  Latin1=119  UTF8=173  CP1252=133  Latin2=173  CP1251=82  CP1250=174  Latin5=74  [top CP1250]
  {{0x73,0x65,0x72,0x62,0x6f,0x63,0x72,0x6f, 0x91,0xaf,0x21,0xb9,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "serbocro"
      // Latin2=175  CP1250=185  [top CP1250]
  {{0x73,0x65,0x73,0x6f,0x74,0x68,0x6f,0x5f, 0x11,0xb9,0x21,0xaf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "sesotho_"
      // Latin1=185  CP1252=175  [top Latin1]
  {{0x73,0x68,0x6f,0x6e,0x61,0x5f,0x5f,0x5f, 0x21,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "shona___"
      // UTF8=191  [top UTF8]
  {{0x73,0x69,0x6e,0x64,0x68,0x69,0x5f,0x5f, 0xb1,0xb9,0x10,0x61,0xaf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "sindhi__"
      // CP1256=185  Arabic=175  [top CP1256]
  {{0x73,0x69,0x6e,0x68,0x61,0x6c,0x65,0x73, 0x11,0xb6,0x21,0xac,0xa1,0xa2,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "sinhales"
      // Latin1=182  CP1252=172  ISO-8859-15=162  [top Latin1]
  {{0x73,0x69,0x73,0x77,0x61,0x6e,0x74,0x5f, 0x21,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "siswant_"
      // UTF8=191  [top UTF8]
  {{0x73,0x6c,0x6f,0x76,0x61,0x6b,0x5f,0x5f, 0x03,0x88,0x6e,0xaf,0x11,0x67,0x41,0xa5,0x21,0xb3,0x00,0x00,}}, // "slovak__"
      // ASCII-7-bit=136  Latin1=110  UTF8=175  CP1252=103  Latin2=165  CP1250=179  [top CP1250]
  {{0x73,0x6c,0x6f,0x76,0x65,0x6e,0x69,0x61, 0x03,0x8e,0x71,0xb2,0x11,0x80,0x42,0xaa,0x39,0x11,0xad,0x00,}}, // "slovenia"
      // ASCII-7-bit=142  Latin1=113  UTF8=178  CP1252=128  Latin2=170  CP1251=57  CP1250=173  [top UTF8]
  {{0x73,0x6f,0x6d,0x61,0x6c,0x69,0x5f,0x5f, 0x11,0xb9,0x21,0xaf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "somali__"
      // Latin1=185  CP1252=175  [top Latin1]
  {{0x73,0x70,0x61,0x6e,0x69,0x73,0x68,0x5f, 0x03,0x9b,0xb8,0xa7,0x11,0x98,0x41,0x45,0x21,0x45,0x21,0x77,}}, // "spanish_"
      // ASCII-7-bit=155  Latin1=184  UTF8=167  CP1252=152  Latin2=69  CP1250=69  ISO-8859-15=119  [top Latin1]
  {{0x73,0x75,0x6e,0x64,0x61,0x6e,0x65,0x73, 0x11,0xb6,0x21,0xac,0xa1,0xa2,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "sundanes"
      // Latin1=182  CP1252=172  ISO-8859-15=162  [top Latin1]
  {{0x73,0x77,0x61,0x68,0x69,0x6c,0x69,0x5f, 0x11,0xb6,0x21,0xac,0xa1,0xa2,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "swahili_"
      // Latin1=182  CP1252=172  ISO-8859-15=162  [top Latin1]
  {{0x73,0x77,0x65,0x64,0x69,0x73,0x68,0x5f, 0x03,0x90,0xba,0xa4,0x11,0x8d,0x41,0x2c,0x21,0x2c,0x21,0x7a,}}, // "swedish_"
      // ASCII-7-bit=144  Latin1=186  UTF8=164  CP1252=141  Latin2=44  CP1250=44  ISO-8859-15=122  [top Latin1]
  {{0x73,0x79,0x72,0x69,0x61,0x63,0x5f,0x5f, 0x01,0x6a,0x11,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "syriac__"
      // ASCII-7-bit=106  UTF8=190  [top UTF8]
  {{0x74,0x61,0x67,0x61,0x6c,0x6f,0x67,0x5f, 0x11,0xb6,0x21,0xac,0xa1,0xa2,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "tagalog_"
      // Latin1=182  CP1252=172  ISO-8859-15=162  [top Latin1]
  {{0x74,0x61,0x6a,0x69,0x6b,0x5f,0x5f,0x5f, 0x21,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "tajik___"
      // UTF8=191  [top UTF8]
  {{0x74,0x61,0x6d,0x69,0x6c,0x5f,0x5f,0x5f, 0x12,0xb4,0x8e,0x11,0xaa,0xa1,0xa0,0x20,0x23,0x96,0x8c,0x82,}}, // "tamil___"
      // Latin1=180  UTF8=142  CP1252=170  ISO-8859-15=160  TSCII=150  TAM=140  TAB=130  [top Latin1]
  {{0x74,0x61,0x74,0x61,0x72,0x5f,0x5f,0x5f, 0x21,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "tatar___"
      // UTF8=191  [top UTF8]
  {{0x74,0x65,0x6c,0x75,0x67,0x75,0x5f,0x5f, 0x03,0x66,0xb6,0x90,0x11,0xac,0xa1,0xa2,0x00,0x00,0x00,0x00,}}, // "telugu__"
      // ASCII-7-bit=102  Latin1=182  UTF8=144  CP1252=172  ISO-8859-15=162  [top Latin1]
  {{0x74,0x68,0x61,0x69,0x5f,0x5f,0x5f,0x5f, 0x05,0x7a,0x53,0xa2,0x24,0x46,0x91,0xba,0xd1,0x9e,0x21,0x29,}}, // "thai____"
      // ASCII-7-bit=122  Latin1=83  UTF8=162  GB=36  CP1252=70  ISO-8859-11=186  CP874=158  MACINTOSH=41  [top ISO-8859-11]
  {{0x74,0x69,0x62,0x65,0x74,0x61,0x6e,0x5f, 0x01,0x42,0x11,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "tibetan_"
      // ASCII-7-bit=66  UTF8=190  [top UTF8]
  {{0x74,0x69,0x67,0x72,0x69,0x6e,0x79,0x61, 0x21,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "tigrinya"
      // UTF8=191  [top UTF8]
  {{0x74,0x6f,0x6e,0x67,0x61,0x5f,0x5f,0x5f, 0x21,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "tonga___"
      // UTF8=191  [top UTF8]
  {{0x74,0x73,0x6f,0x6e,0x67,0x61,0x5f,0x5f, 0x21,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "tsonga__"
      // UTF8=191  [top UTF8]
  {{0x74,0x73,0x77,0x61,0x6e,0x61,0x5f,0x5f, 0x21,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "tswana__"
      // UTF8=191  [top UTF8]
  {{0x74,0x75,0x72,0x6b,0x69,0x73,0x68,0x5f, 0x03,0x81,0x7f,0xa5,0x11,0x6e,0x81,0xba,0x11,0x3d,0x61,0x95,}}, // "turkish_"
      // ASCII-7-bit=129  Latin1=127  UTF8=165  CP1252=110  Latin5=186  ISO-8859-15=61  CP1254=149  [top Latin5]
  {{0x74,0x75,0x72,0x6b,0x6d,0x65,0x6e,0x5f, 0x91,0xb9,0x21,0xaf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "turkmen_"
      // Latin2=185  CP1250=175  [top Latin2]
  {{0x74,0x77,0x69,0x5f,0x5f,0x5f,0x5f,0x5f, 0x11,0xac,0x21,0xb6,0xa1,0xa2,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "twi_____"
      // Latin1=172  CP1252=182  ISO-8859-15=162  [top CP1252]
  {{0x75,0x69,0x67,0x68,0x75,0x72,0x5f,0x5f, 0x21,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "uighur__"
      // UTF8=191  [top UTF8]
  {{0x75,0x6b,0x72,0x61,0x69,0x6e,0x69,0x61, 0x21,0xa0,0x71,0xb7,0x71,0x91,0x72,0x98,0xa2,0x81,0x84,0x00,}}, // "ukrainia"
      // UTF8=160  CP1251=183  KOI8R=145  KOI8U=152  ISO-8859-5=162  CP866=132  [top CP1251]
  {{0x75,0x72,0x64,0x75,0x5f,0x5f,0x5f,0x5f, 0xb1,0xb9,0x10,0x61,0xaf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "urdu____"
      // CP1256=185  Arabic=175  [top CP1256]
  {{0x75,0x7a,0x62,0x65,0x6b,0x5f,0x5f,0x5f, 0x11,0xb6,0x21,0xac,0xa1,0xa2,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "uzbek___"
      // Latin1=182  CP1252=172  ISO-8859-15=162  [top Latin1]
  {{0x76,0x69,0x65,0x74,0x6e,0x61,0x6d,0x65, 0x03,0x81,0xa8,0xb7,0x11,0x9e,0xa1,0x94,0x00,0x00,0x00,0x00,}}, // "vietname"
      // ASCII-7-bit=129  Latin1=168  UTF8=183  CP1252=158  ISO-8859-15=148  [top UTF8]
  {{0x76,0x6f,0x6c,0x61,0x70,0x75,0x6b,0x5f, 0x21,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "volapuk_"
      // UTF8=191  [top UTF8]
  {{0x77,0x65,0x6c,0x73,0x68,0x5f,0x5f,0x5f, 0x11,0xb6,0x21,0xac,0xa1,0xa2,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "welsh___"
      // Latin1=182  CP1252=172  ISO-8859-15=162  [top Latin1]
  {{0x77,0x6f,0x6c,0x6f,0x66,0x5f,0x5f,0x5f, 0x21,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "wolof___"
      // UTF8=191  [top UTF8]
  {{0x78,0x68,0x6f,0x73,0x61,0x5f,0x5f,0x5f, 0x11,0xb6,0x21,0xac,0xa1,0xa2,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "xhosa___"
      // Latin1=182  CP1252=172  ISO-8859-15=162  [top Latin1]
  {{0x79,0x69,0x64,0x64,0x69,0x73,0x68,0x5f, 0x10,0x11,0xb9,0x71,0xaf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "yiddish_"
      // CP1255=185  Hebrew=175  [top CP1255]
  {{0x79,0x6f,0x72,0x75,0x62,0x61,0x5f,0x5f, 0x21,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "yoruba__"
      // UTF8=191  [top UTF8]
  {{0x7a,0x68,0x75,0x61,0x6e,0x67,0x5f,0x5f, 0x21,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "zhuang__"
      // UTF8=191  [top UTF8]
  {{0x7a,0x75,0x6c,0x75,0x5f,0x5f,0x5f,0x5f, 0x11,0xb6,0x21,0xac,0xa1,0xa2,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "zulu____"
      // Latin1=182  CP1252=172  ISO-8859-15=162  [top Latin1]
};

static const int kLangHintProbsSize = 151;

static const HintEntry kTLDHintProbs[] = {	// MaxRange 192
  {{0x5f,0x5f,0x5f,0x5f, 0x0f,0xa8,0xa1,0xa3,0xa0,0x8e,0x8e,0x8a,0x7e,0xa8,0x77,0x7b,0x8b,0x75,0x79,0x7e,}}, // "____"
        // ASCII-7-bit=168  Latin1=161  UTF8=163  GB=160  CP1252=142  KSC=142  SJS=138  EUC-JP=126  BIG5=168  Latin2=119  CP1251=123  CP1256=139  CP1250=117  Latin5=121  ISO-8859-11=126  [top ASCII-7-bit]
  {{0x61,0x63,0x5f,0x5f, 0x08,0xa0,0x9a,0xa1,0x65,0x92,0x8f,0xb1,0xa2,0x22,0x56,0x8a,0x21,0x56,0x61,0x87,}}, // "ac__"
        // ASCII-7-bit=160  Latin1=154  UTF8=161  GB=101  CP1252=146  KSC=143  SJS=177  EUC-JP=162  CP1251=86  CP1256=138  ISO-8859-11=86  JIS=135  [top SJS]
  {{0x61,0x64,0x5f,0x5f, 0x03,0xa6,0xb6,0x93,0x11,0xa8,0x11,0x74,0x81,0x5d,0x81,0x5d,0x00,0x00,0x00,0x00,}}, // "ad__"
        // ASCII-7-bit=166  Latin1=182  UTF8=147  CP1252=168  SJS=116  ISO-8859-15=93  CP932=93  [top Latin1]
  {{0x61,0x65,0x5f,0x5f, 0x05,0xa4,0x81,0xac,0x42,0x86,0x11,0x5b,0x25,0x4f,0x4a,0xb5,0x3b,0x52,0x00,0x00,}}, // "ae__"
        // ASCII-7-bit=164  Latin1=129  UTF8=172  GB=66  CP1252=134  SJS=91  Latin2=79  CP1251=74  CP1256=181  CP1250=59  Latin5=82  [top CP1256]
  {{0x61,0x65,0x72,0x6f, 0x03,0xaf,0xab,0xab,0x12,0x98,0x6a,0x11,0x6a,0x21,0x96,0x21,0x6a,0x00,0x00,0x00,}}, // "aero"
        // ASCII-7-bit=175  Latin1=171  UTF8=171  CP1252=152  KSC=106  EUC-JP=106  CP1251=150  Latin5=106  [top ASCII-7-bit]
  {{0x61,0x66,0x5f,0x5f, 0x03,0xb6,0x95,0xaf,0x11,0x8c,0x61,0x80,0x11,0x62,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "af__"
        // ASCII-7-bit=182  Latin1=149  UTF8=175  CP1252=140  CP1256=128  Latin5=98  [top ASCII-7-bit]
  {{0x61,0x67,0x5f,0x5f, 0x03,0xa8,0xb4,0xa2,0x11,0x9a,0x12,0x95,0x86,0x21,0x60,0x41,0x7a,0x00,0x00,0x00,}}, // "ag__"
        // ASCII-7-bit=168  Latin1=180  UTF8=162  CP1252=154  SJS=149  EUC-JP=134  CP1251=96  ISO-8859-15=122  [top Latin1]
  {{0x61,0x69,0x5f,0x5f, 0x03,0xb8,0x8f,0x9b,0x11,0x9d,0x12,0x8c,0x97,0x11,0x90,0xb1,0x67,0x00,0x00,0x00,}}, // "ai__"
        // ASCII-7-bit=184  Latin1=143  UTF8=155  CP1252=157  SJS=140  EUC-JP=151  Latin2=144  JIS=103  [top ASCII-7-bit]
  {{0x61,0x6c,0x5f,0x5f, 0x03,0xac,0x99,0xae,0x11,0xa1,0x31,0x57,0x41,0x57,0x21,0xa7,0x31,0x57,0x00,0x00,}}, // "al__"
        // ASCII-7-bit=172  Latin1=153  UTF8=174  CP1252=161  BIG5=87  Latin5=87  CP1257=167  Greek=87  [top UTF8]
  {{0x61,0x6d,0x5f,0x5f, 0x08,0xac,0x9a,0xab,0x68,0x9d,0x58,0x82,0x56,0x22,0xac,0x5a,0x00,0x00,0x00,0x00,}}, // "am__"
        // ASCII-7-bit=172  Latin1=154  UTF8=171  GB=104  CP1252=157  KSC=88  SJS=130  EUC-JP=86  CP1251=172  CP1256=90  [top ASCII-7-bit]
  {{0x61,0x6e,0x5f,0x5f, 0x03,0xb6,0xad,0x94,0x11,0x99,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "an__"
        // ASCII-7-bit=182  Latin1=173  UTF8=148  CP1252=153  [top ASCII-7-bit]
  {{0x61,0x6f,0x5f,0x5f, 0x03,0x9f,0xb5,0xab,0x11,0x9f,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "ao__"
        // ASCII-7-bit=159  Latin1=181  UTF8=171  CP1252=159  [top Latin1]
  {{0x61,0x71,0x5f,0x5f, 0x03,0xb7,0xa9,0x9c,0x11,0x8a,0x51,0x97,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "aq__"
        // ASCII-7-bit=183  Latin1=169  UTF8=156  CP1252=138  CP1251=151  [top ASCII-7-bit]
  {{0x61,0x72,0x5f,0x5f, 0x03,0xa0,0xb9,0x9e,0x13,0x98,0x55,0x2c,0x13,0x28,0x26,0x27,0x11,0x2e,0x21,0x42,}}, // "ar__"
        // ASCII-7-bit=160  Latin1=185  UTF8=158  CP1252=152  KSC=85  SJS=44  BIG5=40  Latin2=38  CP1251=39  CP1250=46  ISO-8859-15=66  [top Latin1]
  {{0x61,0x73,0x5f,0x5f, 0x03,0xa9,0xb7,0x9f,0x11,0x94,0x11,0x52,0x22,0x64,0x52,0x12,0x7d,0x74,0x21,0x52,}}, // "as__"
        // ASCII-7-bit=169  Latin1=183  UTF8=159  CP1252=148  SJS=82  Latin2=100  CP1251=82  CP1250=125  Latin5=116  CP1257=82  [top Latin1]
  {{0x61,0x74,0x5f,0x5f, 0x03,0xa1,0xb8,0xa5,0x11,0x9a,0x11,0x48,0x21,0x51,0x13,0x45,0x53,0x4a,0x11,0x62,}}, // "at__"
        // ASCII-7-bit=161  Latin1=184  UTF8=165  CP1252=154  SJS=72  Latin2=81  CP1256=69  CP1250=83  Latin5=74  ISO-8859-15=98  [top Latin1]
  {{0x61,0x75,0x5f,0x5f, 0x09,0xb8,0xa3,0x9f,0x4e,0x9a,0x55,0x54,0x3e,0x5e,0x22,0x30,0x3d,0x21,0x36,0x00,}}, // "au__"
        // ASCII-7-bit=184  Latin1=163  UTF8=159  GB=78  CP1252=154  KSC=85  SJS=84  EUC-JP=62  BIG5=94  CP1256=48  CP1250=61  ISO-8859-15=54  [top ASCII-7-bit]
  {{0x61,0x77,0x5f,0x5f, 0x03,0xb6,0xa2,0xaa,0x11,0x99,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "aw__"
        // ASCII-7-bit=182  Latin1=162  UTF8=170  CP1252=153  [top ASCII-7-bit]
  {{0x61,0x78,0x5f,0x5f, 0x03,0x9d,0xba,0xa2,0x11,0x90,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "ax__"
        // ASCII-7-bit=157  Latin1=186  UTF8=162  CP1252=144  [top Latin1]
  {{0x61,0x7a,0x5f,0x5f, 0x03,0x9a,0x7d,0xb8,0x11,0x86,0x54,0xa8,0x54,0x54,0x91,0x41,0x4c,0x31,0x6c,0x00,}}, // "az__"
        // ASCII-7-bit=154  Latin1=125  UTF8=184  CP1252=134  CP1251=168  CP1256=84  CP1250=84  Latin5=145  KOI8R=76  CP1254=108  [top UTF8]
  {{0x62,0x61,0x5f,0x5f, 0x03,0xa0,0x7e,0xb2,0x11,0x78,0x44,0x89,0x66,0x49,0xb1,0x00,0x00,0x00,0x00,0x00,}}, // "ba__"
        // ASCII-7-bit=160  Latin1=126  UTF8=178  CP1252=120  Latin2=137  CP1251=102  CP1256=73  CP1250=177  [top UTF8]
  {{0x62,0x62,0x5f,0x5f, 0x03,0xba,0xa0,0x7f,0x11,0xa0,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "bb__"
        // ASCII-7-bit=186  Latin1=160  UTF8=127  CP1252=160  [top ASCII-7-bit]
  {{0x62,0x64,0x5f,0x5f, 0x03,0xbd,0x94,0x8c,0x11,0x8a,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "bd__"
        // ASCII-7-bit=189  Latin1=148  UTF8=140  CP1252=138  [top ASCII-7-bit]
  {{0x62,0x65,0x5f,0x5f, 0x03,0xb1,0xb0,0xa1,0x11,0x9d,0x11,0x5f,0x22,0x4e,0x50,0x12,0x4d,0x59,0x11,0x5f,}}, // "be__"
        // ASCII-7-bit=177  Latin1=176  UTF8=161  CP1252=157  SJS=95  Latin2=78  CP1251=80  CP1250=77  Latin5=89  ISO-8859-15=95  [top ASCII-7-bit]
  {{0x62,0x66,0x5f,0x5f, 0x03,0x9f,0xb9,0x63,0x11,0xa6,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "bf__"
        // ASCII-7-bit=159  Latin1=185  UTF8=99  CP1252=166  [top Latin1]
  {{0x62,0x67,0x5f,0x5f, 0x05,0x96,0x70,0xab,0x4a,0x74,0x51,0xb9,0x11,0x4f,0x51,0x44,0x31,0x45,0x41,0x54,}}, // "bg__"
        // ASCII-7-bit=150  Latin1=112  UTF8=171  GB=74  CP1252=116  CP1251=185  CP1250=79  KOI8R=68  CP1254=69  ISO-8859-5=84  [top CP1251]
  {{0x62,0x68,0x5f,0x5f, 0x03,0x9f,0x94,0xa5,0x11,0x84,0x11,0x53,0x41,0xb8,0x10,0x61,0x70,0x00,0x00,0x00,}}, // "bh__"
        // ASCII-7-bit=159  Latin1=148  UTF8=165  CP1252=132  SJS=83  CP1256=184  Arabic=112  [top CP1256]
  {{0x62,0x69,0x5f,0x5f, 0x03,0xa4,0xa5,0xb8,0x12,0x82,0x65,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "bi__"
        // ASCII-7-bit=164  Latin1=165  UTF8=184  CP1252=130  KSC=101  [top UTF8]
  {{0x62,0x69,0x7a,0x5f, 0x0e,0xae,0xa5,0xa1,0x77,0x96,0x7f,0x95,0x9c,0x7a,0x8e,0x8b,0x80,0x80,0x92,0x00,}}, // "biz_"
        // ASCII-7-bit=174  Latin1=165  UTF8=161  GB=119  CP1252=150  KSC=127  SJS=149  EUC-JP=156  BIG5=122  Latin2=142  CP1251=139  CP1256=128  CP1250=128  Latin5=146  [top ASCII-7-bit]
  {{0x62,0x6a,0x5f,0x5f, 0x03,0x9b,0xb6,0x8a,0x11,0xaf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "bj__"
        // ASCII-7-bit=155  Latin1=182  UTF8=138  CP1252=175  [top Latin1]
  {{0x62,0x6d,0x5f,0x5f, 0x05,0xbb,0x95,0xa0,0x5a,0x95,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "bm__"
        // ASCII-7-bit=187  Latin1=149  UTF8=160  GB=90  CP1252=149  [top ASCII-7-bit]
  {{0x62,0x6e,0x5f,0x5f, 0x05,0xb8,0x98,0xa6,0x6d,0xa0,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "bn__"
        // ASCII-7-bit=184  Latin1=152  UTF8=166  GB=109  CP1252=160  [top ASCII-7-bit]
  {{0x62,0x6f,0x5f,0x5f, 0x03,0x9a,0xba,0x9f,0x11,0x9c,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "bo__"
        // ASCII-7-bit=154  Latin1=186  UTF8=159  CP1252=156  [top Latin1]
  {{0x62,0x72,0x5f,0x5f, 0x07,0x9c,0xba,0x9c,0x1f,0x95,0x21,0x43,0x15,0x1c,0x20,0x17,0x0e,0x2b,0x21,0x5a,}}, // "br__"
        // ASCII-7-bit=156  Latin1=186  UTF8=156  GB=31  CP1252=149  KSC=33  SJS=67  BIG5=28  Latin2=32  CP1251=23  CP1256=14  CP1250=43  ISO-8859-15=90  [top Latin1]
  {{0x62,0x73,0x5f,0x5f, 0x03,0xb2,0xb4,0x9c,0x11,0x76,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "bs__"
        // ASCII-7-bit=178  Latin1=180  UTF8=156  CP1252=118  [top Latin1]
  {{0x62,0x74,0x5f,0x5f, 0x03,0xb9,0x96,0xa7,0x11,0x94,0x11,0x6f,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "bt__"
        // ASCII-7-bit=185  Latin1=150  UTF8=167  CP1252=148  SJS=111  [top ASCII-7-bit]
  {{0x62,0x77,0x5f,0x5f, 0x03,0xbb,0x9b,0x88,0x11,0x9d,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "bw__"
        // ASCII-7-bit=187  Latin1=155  UTF8=136  CP1252=157  [top ASCII-7-bit]
  {{0x62,0x79,0x5f,0x5f, 0x03,0x8a,0x7b,0xa4,0x11,0x74,0x42,0x5d,0xb6,0x11,0x4b,0x21,0x5f,0x21,0xa9,0x00,}}, // "by__"
        // ASCII-7-bit=138  Latin1=123  UTF8=164  CP1252=116  Latin2=93  CP1251=182  CP1250=75  ISO-8859-15=95  KOI8R=169  [top CP1251]
  {{0x62,0x7a,0x5f,0x5f, 0x03,0xaf,0x9f,0xa1,0x19,0x90,0x89,0xa4,0x9e,0x8c,0x65,0x8d,0x64,0x70,0x51,0x7e,}}, // "bz__"
        // ASCII-7-bit=175  Latin1=159  UTF8=161  CP1252=144  KSC=137  SJS=164  EUC-JP=158  BIG5=140  Latin2=101  CP1251=141  CP1256=100  CP1250=112  KOI8R=126  [top ASCII-7-bit]
  {{0x63,0x61,0x5f,0x5f, 0x07,0xb3,0xac,0xa0,0x5b,0x9b,0x5f,0x49,0x15,0x56,0x3c,0x5d,0x48,0x42,0x21,0x94,}}, // "ca__"
        // ASCII-7-bit=179  Latin1=172  UTF8=160  GB=91  CP1252=155  KSC=95  SJS=73  BIG5=86  Latin2=60  CP1251=93  CP1256=72  CP1250=66  ISO-8859-15=148  [top ASCII-7-bit]
  {{0x63,0x61,0x74,0x5f, 0x03,0x9a,0xb4,0xad,0x11,0x9f,0x11,0x30,0x31,0x30,0x32,0x30,0x6e,0x00,0x00,0x00,}}, // "cat_"
        // ASCII-7-bit=154  Latin1=180  UTF8=173  CP1252=159  SJS=48  CP1251=48  ISO-8859-11=48  ISO-8859-15=110  [top Latin1]
  {{0x63,0x63,0x5f,0x5f, 0x09,0x9d,0xab,0xad,0x9b,0x86,0x80,0x90,0x9e,0x92,0x21,0x8a,0x11,0x7a,0x51,0x75,}}, // "cc__"
        // ASCII-7-bit=157  Latin1=171  UTF8=173  GB=155  CP1252=134  KSC=128  SJS=144  EUC-JP=158  BIG5=146  CP1256=138  Latin5=122  GBK=117  [top UTF8]
  {{0x63,0x64,0x5f,0x5f, 0x09,0xae,0xa2,0xb2,0x5a,0x95,0x5a,0x8f,0x64,0x5a,0x11,0x7d,0x11,0x74,0x11,0x5a,}}, // "cd__"
        // ASCII-7-bit=174  Latin1=162  UTF8=178  GB=90  CP1252=149  KSC=90  SJS=143  EUC-JP=100  BIG5=90  CP1251=125  CP1250=116  ISO-8859-11=90  [top UTF8]
  {{0x63,0x67,0x5f,0x5f, 0x03,0x83,0x8d,0xbe,0x11,0x83,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "cg__"
        // ASCII-7-bit=131  Latin1=141  UTF8=190  CP1252=131  [top UTF8]
  {{0x63,0x68,0x5f,0x5f, 0x05,0xaa,0xb6,0xa1,0x4c,0x9a,0x11,0x46,0x25,0x49,0x3e,0x41,0x44,0x43,0x11,0x66,}}, // "ch__"
        // ASCII-7-bit=170  Latin1=182  UTF8=161  GB=76  CP1252=154  SJS=70  Latin2=73  CP1251=62  CP1256=65  CP1250=68  Latin5=67  ISO-8859-15=102  [top Latin1]
  {{0x63,0x69,0x5f,0x5f, 0x03,0x9c,0xae,0xb3,0x11,0xa1,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "ci__"
        // ASCII-7-bit=156  Latin1=174  UTF8=179  CP1252=161  [top UTF8]
  {{0x63,0x6b,0x5f,0x5f, 0x03,0xba,0x9c,0x9e,0x11,0x9a,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "ck__"
        // ASCII-7-bit=186  Latin1=156  UTF8=158  CP1252=154  [top ASCII-7-bit]
  {{0x63,0x6c,0x5f,0x5f, 0x03,0xa4,0xb9,0x9c,0x11,0x97,0x11,0x3e,0x27,0x1b,0x1b,0x2d,0x34,0x2b,0x21,0x3b,}}, // "cl__"
        // ASCII-7-bit=164  Latin1=185  UTF8=156  CP1252=151  SJS=62  Latin2=27  CP1251=27  CP1256=45  CP1250=52  Latin5=43  ISO-8859-11=33  ISO-8859-15=59  [top Latin1]
  {{0x63,0x6d,0x5f,0x5f, 0x03,0x93,0xbd,0x64,0x11,0x97,0xa1,0x6c,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "cm__"
        // ASCII-7-bit=147  Latin1=189  UTF8=100  CP1252=151  ISO-8859-15=108  [top Latin1]
  {{0x63,0x6e,0x5f,0x5f, 0x09,0x8c,0x5c,0xa7,0xba,0x4f,0x48,0x57,0x3c,0x8d,0x12,0x4e,0x4f,0x71,0x64,0x00,}}, // "cn__"
        // ASCII-7-bit=140  Latin1=92  UTF8=167  GB=186  CP1252=79  KSC=72  SJS=87  EUC-JP=60  BIG5=141  CP1251=78  CP1256=79  GBK=100  [top GB]
  {{0x63,0x6f,0x5f,0x5f, 0x03,0xa8,0xb7,0xa3,0x12,0x91,0x27,0x31,0x3f,0x13,0x2f,0x2a,0x2a,0x61,0x27,0x00,}}, // "co__"
        // ASCII-7-bit=168  Latin1=183  UTF8=163  CP1252=145  KSC=39  Latin2=63  CP1256=47  CP1250=42  Latin5=42  Greek=39  [top Latin1]
  {{0x63,0x6f,0x6d,0x5f, 0x09,0xb2,0xa5,0xa7,0x94,0x94,0x87,0x87,0x7d,0x82,0x12,0x6e,0x89,0x12,0x7f,0x70,}}, // "com_"
        // ASCII-7-bit=178  Latin1=165  UTF8=167  GB=148  CP1252=148  KSC=135  SJS=135  EUC-JP=125  BIG5=130  CP1251=110  CP1256=137  Latin5=127  ISO-8859-11=112  [top ASCII-7-bit]
  {{0x63,0x6f,0x6f,0x70, 0x03,0xaf,0xa8,0xa0,0x14,0x9c,0x75,0xa7,0x86,0x71,0x78,0x00,0x00,0x00,0x00,0x00,}}, // "coop"
        // ASCII-7-bit=175  Latin1=168  UTF8=160  CP1252=156  KSC=117  SJS=167  EUC-JP=134  ISO-8859-15=120  [top ASCII-7-bit]
  {{0x63,0x72,0x5f,0x5f, 0x03,0x99,0xb7,0xad,0x11,0x84,0x81,0x28,0x11,0x28,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "cr__"
        // ASCII-7-bit=153  Latin1=183  UTF8=173  CP1252=132  Latin5=40  ISO-8859-15=40  [top Latin1]
  {{0x63,0x75,0x5f,0x5f, 0x03,0xa0,0xb7,0x9f,0x11,0xa6,0x53,0x31,0x45,0x45,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "cu__"
        // ASCII-7-bit=160  Latin1=183  UTF8=159  CP1252=166  CP1251=49  CP1256=69  CP1250=69  [top Latin1]
  {{0x63,0x76,0x5f,0x5f, 0x03,0x90,0xbc,0x8f,0x11,0x98,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "cv__"
        // ASCII-7-bit=144  Latin1=188  UTF8=143  CP1252=152  [top Latin1]
  {{0x63,0x78,0x5f,0x5f, 0x03,0xae,0xa2,0xa5,0x15,0x87,0x76,0x9f,0x9a,0x83,0x41,0x85,0x11,0x7e,0x51,0x99,}}, // "cx__"
        // ASCII-7-bit=174  Latin1=162  UTF8=165  CP1252=135  KSC=118  SJS=159  EUC-JP=154  BIG5=131  Latin5=133  ISO-8859-15=126  JIS=153  [top ASCII-7-bit]
  {{0x63,0x79,0x5f,0x5f, 0x03,0xaa,0x88,0xac,0x11,0x86,0x51,0x63,0x21,0x5c,0x61,0x9e,0x12,0x52,0xae,0x00,}}, // "cy__"
        // ASCII-7-bit=170  Latin1=136  UTF8=172  CP1252=134  CP1251=99  Latin5=92  Greek=158  CP1254=82  CP1253=174  [top CP1253]
  {{0x63,0x7a,0x5f,0x5f, 0x03,0x8f,0x74,0xb2,0x11,0x56,0x42,0x8c,0x4a,0x11,0xb5,0x10,0x21,0x3f,0x11,0x41,}}, // "cz__"
        // ASCII-7-bit=143  Latin1=116  UTF8=178  CP1252=86  Latin2=140  CP1251=74  CP1250=181  MACINTOSH=63  CP852=65  [top CP1250]
  {{0x64,0x65,0x5f,0x5f, 0x06,0xa4,0xb7,0xa4,0x40,0x9a,0x36,0x35,0x4b,0x4d,0x43,0x4d,0x4f,0x11,0x79,0x00,}}, // "de__"
        // ASCII-7-bit=164  Latin1=183  UTF8=164  GB=64  CP1252=154  KSC=54  Latin2=75  CP1251=77  CP1256=67  CP1250=77  Latin5=79  ISO-8859-15=121  [top Latin1]
  {{0x64,0x6a,0x5f,0x5f, 0x08,0xa3,0xad,0xa9,0x90,0xa2,0x7d,0x7a,0x68,0x21,0xa0,0x11,0x5e,0xb1,0x5e,0x00,}}, // "dj__"
        // ASCII-7-bit=163  Latin1=173  UTF8=169  GB=144  CP1252=162  KSC=125  SJS=122  EUC-JP=104  CP1251=160  CP1250=94  CP932=94  [top Latin1]
  {{0x64,0x6b,0x5f,0x5f, 0x03,0x9d,0xb8,0xa7,0x11,0x93,0x11,0x39,0x25,0x38,0x34,0x57,0x43,0x3d,0x11,0x54,}}, // "dk__"
        // ASCII-7-bit=157  Latin1=184  UTF8=167  CP1252=147  SJS=57  Latin2=56  CP1251=52  CP1256=87  CP1250=67  Latin5=61  ISO-8859-15=84  [top Latin1]
  {{0x64,0x6d,0x5f,0x5f, 0x03,0xbc,0x76,0xa3,0x11,0x83,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "dm__"
        // ASCII-7-bit=188  Latin1=118  UTF8=163  CP1252=131  [top ASCII-7-bit]
  {{0x64,0x6f,0x5f,0x5f, 0x05,0xa4,0xb6,0xa9,0x6b,0x93,0x31,0x43,0x61,0x57,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "do__"
        // ASCII-7-bit=164  Latin1=182  UTF8=169  GB=107  CP1252=147  BIG5=67  ISO-8859-15=87  [top Latin1]
  {{0x64,0x7a,0x5f,0x5f, 0x03,0x9e,0xb6,0x8d,0x11,0xa1,0x62,0xa6,0x4e,0x10,0x51,0x58,0x00,0x00,0x00,0x00,}}, // "dz__"
        // ASCII-7-bit=158  Latin1=182  UTF8=141  CP1252=161  CP1256=166  CP1250=78  Arabic=88  [top Latin1]
  {{0x65,0x63,0x5f,0x5f, 0x03,0xa2,0xba,0x9c,0x11,0x96,0x35,0x3c,0x32,0x5a,0x32,0x3c,0x00,0x00,0x00,0x00,}}, // "ec__"
        // ASCII-7-bit=162  Latin1=186  UTF8=156  CP1252=150  BIG5=60  Latin2=50  CP1251=90  CP1256=50  CP1250=60  [top Latin1]
  {{0x65,0x64,0x75,0x5f, 0x07,0xbb,0x97,0x99,0x51,0x94,0x6b,0x49,0x11,0x4e,0x21,0x4f,0x13,0x4c,0x41,0x44,}}, // "edu_"
        // ASCII-7-bit=187  Latin1=151  UTF8=153  GB=81  CP1252=148  KSC=107  SJS=73  BIG5=78  CP1256=79  Latin5=76  ISO-8859-11=65  ISO-8859-15=68  [top ASCII-7-bit]
  {{0x65,0x65,0x5f,0x5f, 0x03,0x97,0xaf,0xb4,0x11,0x95,0x42,0x6f,0x78,0x42,0x82,0x87,0xc2,0x5e,0x65,0x00,}}, // "ee__"
        // ASCII-7-bit=151  Latin1=175  UTF8=180  CP1252=149  Latin2=111  CP1251=120  ISO-8859-15=130  CP1257=135  ISO-8859-13=94  Latin4=101  [top UTF8]
  {{0x65,0x67,0x5f,0x5f, 0x05,0x9f,0x7a,0xa7,0x55,0x7d,0x61,0xb9,0x32,0x28,0x28,0x61,0x28,0x00,0x00,0x00,}}, // "eg__"
        // ASCII-7-bit=159  Latin1=122  UTF8=167  GB=85  CP1252=125  CP1256=185  ISO-8859-15=40  CP1257=40  CP1253=40  [top CP1256]
  {{0x65,0x73,0x5f,0x5f, 0x05,0x9f,0xb8,0xa8,0x22,0x91,0x11,0x2b,0x15,0x18,0x33,0x4d,0x31,0x23,0x21,0x6c,}}, // "es__"
        // ASCII-7-bit=159  Latin1=184  UTF8=168  GB=34  CP1252=145  SJS=43  BIG5=24  Latin2=51  CP1251=77  CP1256=49  CP1250=35  ISO-8859-15=108  [top Latin1]
  {{0x65,0x74,0x5f,0x5f, 0x03,0xb5,0x9a,0xa8,0x11,0xa6,0x11,0x7e,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "et__"
        // ASCII-7-bit=181  Latin1=154  UTF8=168  CP1252=166  SJS=126  [top ASCII-7-bit]
  {{0x65,0x75,0x5f,0x5f, 0x03,0xa8,0xa4,0xb5,0x11,0x91,0x42,0x8d,0x69,0x12,0x8b,0x6a,0x11,0x78,0x41,0x78,}}, // "eu__"
        // ASCII-7-bit=168  Latin1=164  UTF8=181  CP1252=145  Latin2=141  CP1251=105  CP1250=139  Latin5=106  ISO-8859-15=120  Greek=120  [top UTF8]
  {{0x66,0x69,0x5f,0x5f, 0x03,0x9e,0xb7,0xaa,0x11,0x96,0x51,0x46,0x11,0x31,0x22,0x69,0x31,0x00,0x00,0x00,}}, // "fi__"
        // ASCII-7-bit=158  Latin1=183  UTF8=170  CP1252=150  CP1251=70  CP1250=49  ISO-8859-15=105  CP1257=49  [top Latin1]
  {{0x66,0x6a,0x5f,0x5f, 0x05,0xba,0x8b,0x9f,0x59,0xa0,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "fj__"
        // ASCII-7-bit=186  Latin1=139  UTF8=159  GB=89  CP1252=160  [top ASCII-7-bit]
  {{0x66,0x6b,0x5f,0x5f, 0x02,0xba,0xa6,0x21,0x96,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "fk__"
        // ASCII-7-bit=186  Latin1=166  CP1252=150  [top ASCII-7-bit]
  {{0x66,0x6d,0x5f,0x5f, 0x03,0x91,0x92,0xbc,0x11,0x82,0x12,0x7d,0x6d,0x12,0x86,0x6a,0x51,0x67,0x00,0x00,}}, // "fm__"
        // ASCII-7-bit=145  Latin1=146  UTF8=188  CP1252=130  SJS=125  EUC-JP=109  Latin2=134  CP1251=106  CP1257=103  [top UTF8]
  {{0x66,0x6f,0x5f,0x5f, 0x03,0x93,0xbc,0x9c,0x11,0x8c,0x71,0x57,0x21,0x51,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "fo__"
        // ASCII-7-bit=147  Latin1=188  UTF8=156  CP1252=140  CP1250=87  ISO-8859-15=81  [top Latin1]
  {{0x66,0x72,0x5f,0x5f, 0x03,0x9f,0xb3,0xaf,0x11,0x99,0x17,0x37,0x32,0x32,0x35,0x41,0x4f,0x35,0x21,0x77,}}, // "fr__"
        // ASCII-7-bit=159  Latin1=179  UTF8=175  CP1252=153  SJS=55  EUC-JP=50  BIG5=50  Latin2=53  CP1251=65  CP1256=79  CP1250=53  ISO-8859-15=119  [top Latin1]
  {{0x67,0x61,0x5f,0x5f, 0x03,0xa8,0xb6,0xa5,0x11,0x95,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "ga__"
        // ASCII-7-bit=168  Latin1=182  UTF8=165  CP1252=149  [top Latin1]
  {{0x67,0x64,0x5f,0x5f, 0x05,0xb5,0xac,0x9a,0x80,0x8a,0x81,0x97,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "gd__"
        // ASCII-7-bit=181  Latin1=172  UTF8=154  GB=128  CP1252=138  Latin5=151  [top ASCII-7-bit]
  {{0x67,0x65,0x5f,0x5f, 0x03,0xa5,0x7d,0xba,0x11,0x8d,0x21,0x58,0x21,0x8b,0x11,0x71,0x21,0x4f,0xb1,0x4c,}}, // "ge__"
        // ASCII-7-bit=165  Latin1=125  UTF8=186  CP1252=141  EUC-JP=88  CP1251=139  CP1250=113  ISO-8859-15=79  ISO-8859-5=76  [top UTF8]
  {{0x67,0x67,0x5f,0x5f, 0x03,0xad,0xa9,0xb0,0x11,0x95,0x11,0x60,0x81,0x93,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "gg__"
        // ASCII-7-bit=173  Latin1=169  UTF8=176  CP1252=149  SJS=96  ISO-8859-15=147  [top UTF8]
  {{0x67,0x68,0x5f,0x5f, 0x03,0xac,0xb3,0x99,0x11,0xa6,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "gh__"
        // ASCII-7-bit=172  Latin1=179  UTF8=153  CP1252=166  [top Latin1]
  {{0x67,0x69,0x5f,0x5f, 0x03,0xb3,0xa1,0xa1,0x11,0x9c,0xa1,0xa8,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "gi__"
        // ASCII-7-bit=179  Latin1=161  UTF8=161  CP1252=156  ISO-8859-15=168  [top ASCII-7-bit]
  {{0x67,0x6c,0x5f,0x5f, 0x03,0xa7,0xb2,0xaa,0x11,0xa1,0x11,0x43,0x11,0x4d,0x61,0x70,0x00,0x00,0x00,0x00,}}, // "gl__"
        // ASCII-7-bit=167  Latin1=178  UTF8=170  CP1252=161  SJS=67  BIG5=77  ISO-8859-15=112  [top Latin1]
  {{0x67,0x6d,0x5f,0x5f, 0x03,0xb8,0x89,0xa0,0x11,0xa7,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "gm__"
        // ASCII-7-bit=184  Latin1=137  UTF8=160  CP1252=167  [top ASCII-7-bit]
  {{0x67,0x6e,0x5f,0x5f, 0x03,0xaa,0xb8,0x89,0x11,0x9d,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "gn__"
        // ASCII-7-bit=170  Latin1=184  UTF8=137  CP1252=157  [top Latin1]
  {{0x67,0x6f,0x76,0x5f, 0x05,0xbd,0x92,0x94,0x22,0x87,0x11,0x2e,0x33,0x36,0x15,0x2e,0x13,0x14,0x1f,0x0e,}}, // "gov_"
        // ASCII-7-bit=189  Latin1=146  UTF8=148  GB=34  CP1252=135  SJS=46  CP1251=54  CP1256=21  CP1250=46  ISO-8859-11=20  ISO-8859-15=31  CP1257=14  [top ASCII-7-bit]
  {{0x67,0x70,0x5f,0x5f, 0x03,0x98,0x9f,0xbb,0x11,0x83,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "gp__"
        // ASCII-7-bit=152  Latin1=159  UTF8=187  CP1252=131  [top UTF8]
  {{0x67,0x72,0x5f,0x5f, 0x05,0xa1,0x7f,0xa8,0x45,0x80,0x42,0x49,0x54,0x12,0x44,0x45,0x61,0xb5,0x21,0x9e,}}, // "gr__"
        // ASCII-7-bit=161  Latin1=127  UTF8=168  GB=69  CP1252=128  Latin2=73  CP1251=84  CP1250=68  Latin5=69  Greek=181  CP1253=158  [top Greek]
  {{0x67,0x73,0x5f,0x5f, 0x05,0xb0,0x93,0x98,0x68,0x88,0x13,0xa2,0xa7,0x75,0x42,0x99,0x98,0x91,0x66,0x00,}}, // "gs__"
        // ASCII-7-bit=176  Latin1=147  UTF8=152  GB=104  CP1252=136  SJS=162  EUC-JP=167  BIG5=117  Latin5=153  ISO-8859-11=152  CP932=102  [top ASCII-7-bit]
  {{0x67,0x74,0x5f,0x5f, 0x03,0xa3,0xb6,0xa9,0x12,0x95,0x3d,0x91,0x3d,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "gt__"
        // ASCII-7-bit=163  Latin1=182  UTF8=169  CP1252=149  KSC=61  ISO-8859-15=61  [top Latin1]
  {{0x67,0x75,0x5f,0x5f, 0x03,0xb9,0xa0,0x7d,0x11,0xa5,0x11,0x7d,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "gu__"
        // ASCII-7-bit=185  Latin1=160  UTF8=125  CP1252=165  SJS=125  [top ASCII-7-bit]
  {{0x67,0x79,0x5f,0x5f, 0x03,0xbc,0x82,0xa2,0x11,0x87,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "gy__"
        // ASCII-7-bit=188  Latin1=130  UTF8=162  CP1252=135  [top ASCII-7-bit]
  {{0x68,0x6b,0x5f,0x5f, 0x07,0xa6,0x6c,0xa3,0xa0,0x6b,0x38,0x53,0x11,0xb5,0xa1,0x73,0xc1,0x3d,0x21,0x49,}}, // "hk__"
        // ASCII-7-bit=166  Latin1=108  UTF8=163  GB=160  CP1252=107  KSC=56  SJS=83  BIG5=181  GBK=115  GB18030=61  BIG5_HKSCS=73  [top BIG5]
  {{0x68,0x6d,0x5f,0x5f, 0x05,0xa8,0x9e,0xaa,0x67,0x67,0x13,0xac,0x9f,0x95,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "hm__"
        // ASCII-7-bit=168  Latin1=158  UTF8=170  GB=103  CP1252=103  SJS=172  EUC-JP=159  BIG5=149  [top SJS]
  {{0x68,0x6e,0x5f,0x5f, 0x03,0xa1,0xb7,0xa6,0x11,0x9c,0x51,0x4d,0x31,0x4d,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "hn__"
        // ASCII-7-bit=161  Latin1=183  UTF8=166  CP1252=156  CP1251=77  ISO-8859-11=77  [top Latin1]
  {{0x68,0x72,0x5f,0x5f, 0x03,0x99,0x67,0xa7,0x11,0x67,0x42,0x9d,0x20,0x11,0xb8,0x10,0x21,0x26,0x11,0x32,}}, // "hr__"
        // ASCII-7-bit=153  Latin1=103  UTF8=167  CP1252=103  Latin2=157  CP1251=32  CP1250=184  MACINTOSH=38  CP852=50  [top CP1250]
  {{0x68,0x74,0x5f,0x5f, 0x03,0x95,0x97,0xbc,0x11,0x92,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "ht__"
        // ASCII-7-bit=149  Latin1=151  UTF8=188  CP1252=146  [top UTF8]
  {{0x68,0x75,0x5f,0x5f, 0x05,0xa2,0xa4,0xad,0x33,0x76,0x11,0x3d,0x25,0xa7,0x4a,0x40,0xa7,0x35,0x11,0x3a,}}, // "hu__"
        // ASCII-7-bit=162  Latin1=164  UTF8=173  GB=51  CP1252=118  SJS=61  Latin2=167  CP1251=74  CP1256=64  CP1250=167  Latin5=53  ISO-8859-15=58  [top UTF8]
  {{0x69,0x64,0x5f,0x5f, 0x07,0xb8,0xab,0x8c,0x24,0x97,0x33,0x46,0x14,0x2c,0x1f,0x1f,0x7b,0x22,0x29,0x4b,}}, // "id__"
        // ASCII-7-bit=184  Latin1=171  UTF8=140  GB=36  CP1252=151  KSC=51  SJS=70  BIG5=44  Latin2=31  CP1251=31  CP1256=123  ISO-8859-11=41  ISO-8859-15=75  [top ASCII-7-bit]
  {{0x69,0x65,0x5f,0x5f, 0x05,0xb3,0xa6,0xa0,0x45,0x9d,0x11,0x46,0x21,0x67,0x12,0x32,0x4a,0x22,0xa3,0x36,}}, // "ie__"
        // ASCII-7-bit=179  Latin1=166  UTF8=160  GB=69  CP1252=157  SJS=70  Latin2=103  CP1256=50  CP1250=74  ISO-8859-15=163  CP1257=54  [top ASCII-7-bit]
  {{0x69,0x6c,0x5f,0x5f, 0x03,0x94,0x74,0xa9,0x11,0x6e,0x21,0x3b,0x23,0x79,0x83,0x3d,0x41,0xb8,0x71,0x90,}}, // "il__"
        // ASCII-7-bit=148  Latin1=116  UTF8=169  CP1252=110  EUC-JP=59  CP1251=121  CP1256=131  CP1250=61  CP1255=184  Hebrew=144  [top CP1255]
  {{0x69,0x6d,0x5f,0x5f, 0x05,0xb7,0x8e,0xaf,0x89,0x7e,0x51,0x48,0x21,0x71,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "im__"
        // ASCII-7-bit=183  Latin1=142  UTF8=175  GB=137  CP1252=126  CP1251=72  Latin5=113  [top ASCII-7-bit]
  {{0x69,0x6e,0x5f,0x5f, 0x08,0xbb,0x95,0x9c,0x5e,0x99,0x61,0x6f,0x67,0x21,0x60,0x22,0x69,0x4d,0x11,0x4c,}}, // "in__"
        // ASCII-7-bit=187  Latin1=149  UTF8=156  GB=94  CP1252=153  KSC=97  SJS=111  EUC-JP=103  CP1251=96  Latin5=105  ISO-8859-11=77  CP1257=76  [top ASCII-7-bit]
  {{0x69,0x6e,0x66,0x6f, 0x05,0xac,0xa7,0xa7,0x76,0x94,0x18,0x93,0x82,0x70,0x90,0x91,0x82,0x9c,0x78,0x00,}}, // "info"
        // ASCII-7-bit=172  Latin1=167  UTF8=167  GB=118  CP1252=148  SJS=147  EUC-JP=130  BIG5=112  Latin2=144  CP1251=145  CP1256=130  CP1250=156  Latin5=120  [top ASCII-7-bit]
  {{0x69,0x6e,0x74,0x5f, 0x05,0xb2,0xa7,0xb0,0x3a,0x84,0x53,0x62,0x55,0x54,0x71,0x45,0x21,0x39,0x31,0x50,}}, // "int_"
        // ASCII-7-bit=178  Latin1=167  UTF8=176  GB=58  CP1252=132  CP1251=98  CP1256=85  CP1250=84  Greek=69  CP1253=57  ISO-8859-5=80  [top ASCII-7-bit]
  {{0x69,0x6f,0x5f,0x5f, 0x09,0xaf,0x98,0xac,0xa8,0x85,0x77,0x83,0x7b,0x67,0x12,0x88,0x71,0x22,0x77,0x67,}}, // "io__"
        // ASCII-7-bit=175  Latin1=152  UTF8=172  GB=168  CP1252=133  KSC=119  SJS=131  EUC-JP=123  BIG5=103  CP1251=136  CP1256=113  ISO-8859-11=119  ISO-8859-15=103  [top ASCII-7-bit]
  {{0x69,0x72,0x5f,0x5f, 0x07,0x9e,0x8c,0xb5,0x55,0x7e,0x22,0x3c,0x25,0x32,0x32,0xae,0x36,0x3e,0x81,0x52,}}, // "ir__"
        // ASCII-7-bit=158  Latin1=140  UTF8=181  GB=85  CP1252=126  KSC=34  SJS=60  Latin2=50  CP1251=50  CP1256=174  CP1250=54  Latin5=62  CP1254=82  [top UTF8]
  {{0x69,0x73,0x5f,0x5f, 0x05,0x8f,0xbc,0xa0,0x1b,0x84,0x15,0x3b,0x1b,0x2f,0x1b,0x25,0x11,0x32,0x21,0x1b,}}, // "is__"
        // ASCII-7-bit=143  Latin1=188  UTF8=160  GB=27  CP1252=132  SJS=59  EUC-JP=27  BIG5=47  Latin2=27  CP1251=37  CP1250=50  ISO-8859-15=27  [top Latin1]
  {{0x69,0x74,0x5f,0x5f, 0x03,0xac,0xb4,0x9e,0x11,0xa2,0x11,0x35,0x21,0x44,0x21,0x3e,0x21,0x60,0x51,0x55,}}, // "it__"
        // ASCII-7-bit=172  Latin1=180  UTF8=158  CP1252=162  SJS=53  Latin2=68  CP1250=62  ISO-8859-15=96  JIS=85  [top Latin1]
  {{0x6a,0x65,0x5f,0x5f, 0x03,0xb7,0x8e,0xab,0x11,0x9e,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "je__"
        // ASCII-7-bit=183  Latin1=142  UTF8=171  CP1252=158  [top ASCII-7-bit]
  {{0x6a,0x6d,0x5f,0x5f, 0x03,0xb8,0xa6,0x93,0x11,0xa0,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "jm__"
        // ASCII-7-bit=184  Latin1=166  UTF8=147  CP1252=160  [top ASCII-7-bit]
  {{0x6a,0x6f,0x5f,0x5f, 0x03,0xa5,0x73,0xb0,0x11,0x82,0x52,0x3c,0xb2,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "jo__"
        // ASCII-7-bit=165  Latin1=115  UTF8=176  CP1252=130  CP1251=60  CP1256=178  [top CP1256]
  {{0x6a,0x6f,0x62,0x73, 0x03,0xb7,0xa1,0xa5,0x11,0xa0,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "jobs"
        // ASCII-7-bit=183  Latin1=161  UTF8=165  CP1252=160  [top ASCII-7-bit]
  {{0x6a,0x70,0x5f,0x5f, 0x09,0x9b,0x48,0xa4,0x45,0x42,0x4b,0xb3,0xad,0x48,0x51,0x1e,0x61,0x73,0x21,0x63,}}, // "jp__"
        // ASCII-7-bit=155  Latin1=72  UTF8=164  GB=69  CP1252=66  KSC=75  SJS=179  EUC-JP=173  BIG5=72  ISO-8859-11=30  JIS=115  CP932=99  [top SJS]
  {{0x6b,0x65,0x5f,0x5f, 0x03,0xb9,0x9a,0xa1,0x13,0x9e,0x4b,0x5b,0x11,0x4b,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "ke__"
        // ASCII-7-bit=185  Latin1=154  UTF8=161  CP1252=158  KSC=75  SJS=91  BIG5=75  [top ASCII-7-bit]
  {{0x6b,0x67,0x5f,0x5f, 0x05,0x94,0x71,0x9f,0x67,0x83,0x11,0xa8,0x31,0xb7,0x12,0x57,0x57,0x41,0x6f,0x00,}}, // "kg__"
        // ASCII-7-bit=148  Latin1=113  UTF8=159  GB=103  CP1252=131  SJS=168  CP1251=183  CP1250=87  Latin5=87  KOI8R=111  [top CP1251]
  {{0x6b,0x68,0x5f,0x5f, 0x05,0xb6,0xa1,0xa1,0x53,0xa5,0x12,0x74,0x73,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "kh__"
        // ASCII-7-bit=182  Latin1=161  UTF8=161  GB=83  CP1252=165  SJS=116  EUC-JP=115  [top ASCII-7-bit]
  {{0x6b,0x69,0x5f,0x5f, 0x03,0xb8,0xac,0x98,0x11,0x82,0x81,0x61,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "ki__"
        // ASCII-7-bit=184  Latin1=172  UTF8=152  CP1252=130  Latin5=97  [top ASCII-7-bit]
  {{0x6b,0x6e,0x5f,0x5f, 0x01,0xba,0x11,0x89,0x11,0xaa,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "kn__"
        // ASCII-7-bit=186  UTF8=137  CP1252=170  [top ASCII-7-bit]
  {{0x6b,0x72,0x5f,0x5f, 0x09,0x80,0x39,0x92,0x43,0x2e,0xbe,0x5f,0x3a,0x3d,0x31,0x0c,0x00,0x00,0x00,0x00,}}, // "kr__"
        // ASCII-7-bit=128  Latin1=57  UTF8=146  GB=67  CP1252=46  KSC=190  SJS=95  EUC-JP=58  BIG5=61  CP1250=12  [top KSC]
  {{0x6b,0x77,0x5f,0x5f, 0x03,0x91,0x69,0xb2,0x11,0x71,0x61,0xb5,0x11,0x40,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "kw__"
        // ASCII-7-bit=145  Latin1=105  UTF8=178  CP1252=113  CP1256=181  Latin5=64  [top CP1256]
  {{0x6b,0x79,0x5f,0x5f, 0x03,0xab,0x9d,0xb6,0x11,0x9a,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "ky__"
        // ASCII-7-bit=171  Latin1=157  UTF8=182  CP1252=154  [top UTF8]
  {{0x6b,0x7a,0x5f,0x5f, 0x03,0x8d,0x7b,0xb1,0x12,0x74,0x3d,0x41,0xb5,0x21,0x4a,0x41,0x8b,0x81,0x4a,0x00,}}, // "kz__"
        // ASCII-7-bit=141  Latin1=123  UTF8=177  CP1252=116  KSC=61  CP1251=181  Latin5=74  KOI8R=139  ISO-8859-5=74  [top CP1251]
  {{0x6c,0x61,0x5f,0x5f, 0x05,0xa8,0x8e,0xab,0xab,0x71,0x13,0x9f,0x6a,0x99,0x41,0x77,0x51,0x64,0x41,0x7e,}}, // "la__"
        // ASCII-7-bit=168  Latin1=142  UTF8=171  GB=171  CP1252=113  SJS=159  EUC-JP=106  BIG5=153  Latin5=119  GBK=100  CP932=126  [top UTF8]
  {{0x6c,0x62,0x5f,0x5f, 0x03,0xb3,0x95,0xaf,0x11,0x90,0x11,0x35,0x41,0x9f,0x10,0x61,0x3f,0x00,0x00,0x00,}}, // "lb__"
        // ASCII-7-bit=179  Latin1=149  UTF8=175  CP1252=144  SJS=53  CP1256=159  Arabic=63  [top ASCII-7-bit]
  {{0x6c,0x63,0x5f,0x5f, 0x02,0xab,0xa8,0x21,0xb5,0x11,0x7f,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "lc__"
        // ASCII-7-bit=171  Latin1=168  CP1252=181  SJS=127  [top CP1252]
  {{0x6c,0x69,0x5f,0x5f, 0x05,0xad,0xb5,0x9d,0x72,0x93,0x12,0x5f,0x53,0x22,0x89,0x64,0x11,0x5f,0x51,0x53,}}, // "li__"
        // ASCII-7-bit=173  Latin1=181  UTF8=157  GB=114  CP1252=147  SJS=95  EUC-JP=83  CP1251=137  CP1256=100  Latin5=95  GBK=83  [top Latin1]
  {{0x6c,0x6b,0x5f,0x5f, 0x03,0xb9,0x9e,0x9d,0x11,0xa1,0x12,0x47,0x47,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "lk__"
        // ASCII-7-bit=185  Latin1=158  UTF8=157  CP1252=161  SJS=71  EUC-JP=71  [top ASCII-7-bit]
  {{0x6c,0x72,0x5f,0x5f, 0x03,0xad,0xac,0x8a,0x11,0xb1,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "lr__"
        // ASCII-7-bit=173  Latin1=172  UTF8=138  CP1252=177  [top CP1252]
  {{0x6c,0x73,0x5f,0x5f, 0x03,0xb7,0xa3,0x97,0x11,0xa8,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "ls__"
        // ASCII-7-bit=183  Latin1=163  UTF8=151  CP1252=168  [top ASCII-7-bit]
  {{0x6c,0x74,0x5f,0x5f, 0x06,0x8c,0x64,0xae,0x3b,0x6d,0x5d,0x32,0x48,0x71,0x11,0x55,0x31,0xb8,0xc1,0x6e,}}, // "lt__"
        // ASCII-7-bit=140  Latin1=100  UTF8=174  GB=59  CP1252=109  KSC=93  Latin2=72  CP1251=113  CP1250=85  CP1257=184  ISO-8859-13=110  [top CP1257]
  {{0x6c,0x75,0x5f,0x5f, 0x05,0xa6,0xb2,0xad,0x47,0x9f,0x41,0x42,0x22,0x54,0x3f,0x11,0x53,0x10,0x51,0x3b,}}, // "lu__"
        // ASCII-7-bit=166  Latin1=178  UTF8=173  GB=71  CP1252=159  Latin2=66  CP1250=84  Latin5=63  ISO-8859-15=83  UTF-16BE=59  [top Latin1]
  {{0x6c,0x76,0x5f,0x5f, 0x03,0x97,0x6d,0xb5,0x11,0x6b,0x51,0x92,0x51,0xb0,0x11,0x4e,0xa2,0x5f,0x52,0x00,}}, // "lv__"
        // ASCII-7-bit=151  Latin1=109  UTF8=181  CP1252=107  CP1251=146  CP1257=176  KOI8R=78  ISO-8859-13=95  Latin4=82  [top UTF8]
  {{0x6c,0x79,0x5f,0x5f, 0x07,0xa4,0x77,0xb0,0x59,0x8c,0x92,0x59,0x32,0x59,0xb1,0x00,0x00,0x00,0x00,0x00,}}, // "ly__"
        // ASCII-7-bit=164  Latin1=119  UTF8=176  GB=89  CP1252=140  KSC=146  SJS=89  CP1251=89  CP1256=177  [top CP1256]
  {{0x6d,0x61,0x5f,0x5f, 0x03,0xa2,0xb3,0xa6,0x11,0xa0,0x62,0x9e,0x5d,0x22,0x7d,0x4b,0x10,0x11,0x47,0x00,}}, // "ma__"
        // ASCII-7-bit=162  Latin1=179  UTF8=166  CP1252=160  CP1256=158  CP1250=93  ISO-8859-15=125  CP1257=75  Arabic=71  [top Latin1]
  {{0x6d,0x63,0x5f,0x5f, 0x03,0x87,0xbe,0x73,0x11,0x89,0xa1,0x47,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "mc__"
        // ASCII-7-bit=135  Latin1=190  UTF8=115  CP1252=137  ISO-8859-15=71  [top Latin1]
  {{0x6d,0x64,0x5f,0x5f, 0x03,0xa4,0x82,0xad,0x11,0x86,0x11,0x61,0x15,0x61,0x74,0xae,0x6e,0x99,0x51,0x9d,}}, // "md__"
        // ASCII-7-bit=164  Latin1=130  UTF8=173  CP1252=134  SJS=97  BIG5=97  Latin2=116  CP1251=174  CP1256=110  CP1250=153  KOI8R=157  [top CP1251]
  {{0x6d,0x67,0x5f,0x5f, 0x03,0x99,0xba,0x85,0x11,0xa6,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "mg__"
        // ASCII-7-bit=153  Latin1=186  UTF8=133  CP1252=166  [top Latin1]
  {{0x6d,0x69,0x6c,0x5f, 0x03,0xba,0x9a,0x9a,0x13,0x9a,0x44,0x50,0x11,0x24,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "mil_"
        // ASCII-7-bit=186  Latin1=154  UTF8=154  CP1252=154  KSC=68  SJS=80  BIG5=36  [top ASCII-7-bit]
  {{0x6d,0x6b,0x5f,0x5f, 0x03,0x95,0x72,0xab,0x11,0x73,0x42,0x3f,0xa0,0x11,0x66,0x51,0xb6,0x81,0x56,0x00,}}, // "mk__"
        // ASCII-7-bit=149  Latin1=114  UTF8=171  CP1252=115  Latin2=63  CP1251=160  CP1250=102  KOI8R=182  ISO-8859-5=86  [top KOI8R]
  {{0x6d,0x6c,0x5f,0x5f, 0x03,0x85,0xbd,0x85,0x11,0x96,0x81,0x59,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "ml__"
        // ASCII-7-bit=133  Latin1=189  UTF8=133  CP1252=150  Latin5=89  [top Latin1]
  {{0x6d,0x6d,0x5f,0x5f, 0x03,0xb5,0xa1,0x7d,0x11,0xaf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "mm__"
        // ASCII-7-bit=181  Latin1=161  UTF8=125  CP1252=175  [top ASCII-7-bit]
  {{0x6d,0x6e,0x5f,0x5f, 0x07,0x9f,0x7f,0xb8,0x79,0x7d,0x4f,0x5f,0x11,0x56,0x11,0xa7,0x00,0x00,0x00,0x00,}}, // "mn__"
        // ASCII-7-bit=159  Latin1=127  UTF8=184  GB=121  CP1252=125  KSC=79  SJS=95  BIG5=86  CP1251=167  [top UTF8]
  {{0x6d,0x6f,0x5f,0x5f, 0x05,0xa0,0x9e,0xaa,0x9e,0x86,0x11,0x4a,0x11,0xb2,0xa1,0x59,0xc1,0x36,0x00,0x00,}}, // "mo__"
        // ASCII-7-bit=160  Latin1=158  UTF8=170  GB=158  CP1252=134  SJS=74  BIG5=178  GBK=89  GB18030=54  [top BIG5]
  {{0x6d,0x6f,0x62,0x69, 0x08,0xb6,0x95,0xaa,0x7f,0x7d,0x53,0xa0,0x71,0x13,0x65,0x6d,0x78,0xc1,0x73,0x00,}}, // "mobi"
        // ASCII-7-bit=182  Latin1=149  UTF8=170  GB=127  CP1252=125  KSC=83  SJS=160  EUC-JP=113  Latin2=101  CP1251=109  CP1256=120  CP932=115  [top ASCII-7-bit]
  {{0x6d,0x70,0x5f,0x5f, 0x03,0xbe,0x5a,0x54,0x11,0x5c,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "mp__"
        // ASCII-7-bit=190  Latin1=90  UTF8=84  CP1252=92  [top ASCII-7-bit]
  {{0x6d,0x71,0x5f,0x5f, 0x03,0xa3,0xb8,0x95,0x11,0xa5,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "mq__"
        // ASCII-7-bit=163  Latin1=184  UTF8=149  CP1252=165  [top Latin1]
  {{0x6d,0x72,0x5f,0x5f, 0x03,0x9b,0xab,0x8d,0x11,0x96,0x61,0xb6,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "mr__"
        // ASCII-7-bit=155  Latin1=171  UTF8=141  CP1252=150  CP1256=182  [top CP1256]
  {{0x6d,0x73,0x5f,0x5f, 0x03,0xb4,0xae,0x9c,0x18,0x8a,0x73,0x86,0x6f,0x53,0x56,0x6c,0x75,0x11,0x85,0x00,}}, // "ms__"
        // ASCII-7-bit=180  Latin1=174  UTF8=156  CP1252=138  KSC=115  SJS=134  EUC-JP=111  BIG5=83  Latin2=86  CP1251=108  CP1256=117  Latin5=133  [top ASCII-7-bit]
  {{0x6d,0x74,0x5f,0x5f, 0x05,0xbc,0x87,0x9b,0x5c,0x8b,0x42,0x2c,0x40,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "mt__"
        // ASCII-7-bit=188  Latin1=135  UTF8=155  GB=92  CP1252=139  Latin2=44  CP1251=64  [top ASCII-7-bit]
  {{0x6d,0x75,0x5f,0x5f, 0x09,0xb4,0xa4,0xa9,0x57,0x9f,0x4d,0x7e,0x70,0x4d,0x11,0x4d,0x21,0x57,0x11,0x4d,}}, // "mu__"
        // ASCII-7-bit=180  Latin1=164  UTF8=169  GB=87  CP1252=159  KSC=77  SJS=126  EUC-JP=112  BIG5=77  CP1251=77  Latin5=87  ISO-8859-15=77  [top ASCII-7-bit]
  {{0x6d,0x75,0x73,0x65, 0x07,0xb3,0xa9,0xa6,0x76,0x90,0x56,0x88,0x13,0x8e,0x6a,0x7a,0x00,0x00,0x00,0x00,}}, // "muse"
        // ASCII-7-bit=179  Latin1=169  UTF8=166  GB=118  CP1252=144  KSC=86  SJS=136  BIG5=142  Latin2=106  CP1251=122  [top ASCII-7-bit]
  {{0x6d,0x76,0x5f,0x5f, 0x03,0xb6,0x98,0xab,0x11,0x9f,0x61,0x53,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "mv__"
        // ASCII-7-bit=182  Latin1=152  UTF8=171  CP1252=159  CP1256=83  [top ASCII-7-bit]
  {{0x6d,0x77,0x5f,0x5f, 0x05,0xb7,0xa3,0xa8,0x7f,0x8d,0x11,0x7f,0xc1,0x6b,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "mw__"
        // ASCII-7-bit=183  Latin1=163  UTF8=168  GB=127  CP1252=141  SJS=127  GBK=107  [top ASCII-7-bit]
  {{0x6d,0x78,0x5f,0x5f, 0x05,0xa7,0xba,0x94,0x45,0x93,0x11,0x1d,0x12,0x25,0x29,0x21,0x27,0x21,0x3c,0x00,}}, // "mx__"
        // ASCII-7-bit=167  Latin1=186  UTF8=148  GB=69  CP1252=147  SJS=29  BIG5=37  Latin2=41  CP1250=39  ISO-8859-15=60  [top Latin1]
  {{0x6d,0x79,0x5f,0x5f, 0x05,0xb3,0xb3,0x8d,0x6b,0x8f,0x13,0x4d,0x50,0x7f,0x12,0x3f,0x6b,0x12,0x4b,0x3e,}}, // "my__"
        // ASCII-7-bit=179  Latin1=179  UTF8=141  GB=107  CP1252=143  SJS=77  EUC-JP=80  BIG5=127  CP1251=63  CP1256=107  Latin5=75  ISO-8859-11=62  [top ASCII-7-bit]
  {{0x6d,0x7a,0x5f,0x5f, 0x03,0x8e,0xaf,0xb7,0x11,0x8f,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "mz__"
        // ASCII-7-bit=142  Latin1=175  UTF8=183  CP1252=143  [top UTF8]
  {{0x6e,0x61,0x5f,0x5f, 0x03,0xba,0xa1,0x97,0x11,0x9c,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "na__"
        // ASCII-7-bit=186  Latin1=161  UTF8=151  CP1252=156  [top ASCII-7-bit]
  {{0x6e,0x61,0x6d,0x65, 0x08,0xb2,0xa4,0xa9,0x8a,0x90,0x7d,0x7c,0x6b,0x15,0x80,0x94,0x6b,0x8b,0x77,0x00,}}, // "name"
        // ASCII-7-bit=178  Latin1=164  UTF8=169  GB=138  CP1252=144  KSC=125  SJS=124  EUC-JP=107  Latin2=128  CP1251=148  CP1256=107  CP1250=139  Latin5=119  [top ASCII-7-bit]
  {{0x6e,0x63,0x5f,0x5f, 0x03,0xa7,0xb9,0x8c,0x11,0x9e,0x11,0x72,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "nc__"
        // ASCII-7-bit=167  Latin1=185  UTF8=140  CP1252=158  SJS=114  [top Latin1]
  {{0x6e,0x65,0x5f,0x5f, 0x03,0xad,0xb9,0x80,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "ne__"
        // ASCII-7-bit=173  Latin1=185  UTF8=128  [top Latin1]
  {{0x6e,0x65,0x74,0x5f, 0x0f,0xac,0xa6,0xa4,0x93,0x93,0x94,0x97,0x83,0x86,0x7e,0x80,0x96,0x7a,0x89,0x73,}}, // "net_"
        // ASCII-7-bit=172  Latin1=166  UTF8=164  GB=147  CP1252=147  KSC=148  SJS=151  EUC-JP=131  BIG5=134  Latin2=126  CP1251=128  CP1256=150  CP1250=122  Latin5=137  ISO-8859-11=115  [top ASCII-7-bit]
  {{0x6e,0x66,0x5f,0x5f, 0x03,0xb5,0xa9,0xa8,0x11,0x87,0xa1,0x77,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "nf__"
        // ASCII-7-bit=181  Latin1=169  UTF8=168  CP1252=135  ISO-8859-15=119  [top ASCII-7-bit]
  {{0x6e,0x67,0x5f,0x5f, 0x03,0xbd,0x95,0x89,0x11,0x92,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "ng__"
        // ASCII-7-bit=189  Latin1=149  UTF8=137  CP1252=146  [top ASCII-7-bit]
  {{0x6e,0x69,0x5f,0x5f, 0x03,0x9d,0xb3,0xa9,0x11,0xa8,0x81,0x40,0x11,0x36,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "ni__"
        // ASCII-7-bit=157  Latin1=179  UTF8=169  CP1252=168  Latin5=64  ISO-8859-15=54  [top Latin1]
  {{0x6e,0x6c,0x5f,0x5f, 0x05,0xb2,0xae,0xa0,0x32,0xa1,0x36,0x31,0x42,0x39,0x3b,0x33,0x45,0x11,0x6f,0x00,}}, // "nl__"
        // ASCII-7-bit=178  Latin1=174  UTF8=160  GB=50  CP1252=161  BIG5=49  Latin2=66  CP1251=57  CP1256=59  CP1250=51  Latin5=69  ISO-8859-15=111  [top ASCII-7-bit]
  {{0x6e,0x6f,0x5f,0x5f, 0x05,0x99,0xb8,0xaa,0x47,0x8d,0x11,0x31,0x22,0x34,0x3e,0x42,0x70,0x33,0x00,0x00,}}, // "no__"
        // ASCII-7-bit=153  Latin1=184  UTF8=170  GB=71  CP1252=141  SJS=49  Latin2=52  CP1251=62  ISO-8859-15=112  CP1257=51  [top Latin1]
  {{0x6e,0x70,0x5f,0x5f, 0x03,0xb2,0x9f,0xa5,0x11,0xac,0x11,0x61,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "np__"
        // ASCII-7-bit=178  Latin1=159  UTF8=165  CP1252=172  SJS=97  [top ASCII-7-bit]
  {{0x6e,0x72,0x5f,0x5f, 0x03,0xbe,0x77,0x7a,0x11,0x62,0x71,0x44,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "nr__"
        // ASCII-7-bit=190  Latin1=119  UTF8=122  CP1252=98  CP1250=68  [top ASCII-7-bit]
  {{0x6e,0x75,0x5f,0x5f, 0x03,0xae,0xb4,0x9e,0x11,0x96,0x12,0x7c,0x76,0x22,0x75,0x62,0xa1,0x65,0x00,0x00,}}, // "nu__"
        // ASCII-7-bit=174  Latin1=180  UTF8=158  CP1252=150  SJS=124  EUC-JP=118  CP1251=117  CP1256=98  CP1254=101  [top Latin1]
  {{0x6e,0x7a,0x5f,0x5f, 0x0d,0xba,0x97,0xa2,0x5f,0x97,0x5c,0x5a,0x4b,0x4a,0x30,0x47,0x36,0x34,0x21,0x2b,}}, // "nz__"
        // ASCII-7-bit=186  Latin1=151  UTF8=162  GB=95  CP1252=151  KSC=92  SJS=90  EUC-JP=75  BIG5=74  Latin2=48  CP1251=71  CP1256=54  CP1250=52  ISO-8859-15=43  [top ASCII-7-bit]
  {{0x6f,0x6d,0x5f,0x5f, 0x03,0x9a,0x8a,0x89,0x11,0x87,0x61,0xbc,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "om__"
        // ASCII-7-bit=154  Latin1=138  UTF8=137  CP1252=135  CP1256=188  [top CP1256]
  {{0x6f,0x72,0x67,0x5f, 0x0e,0xb5,0x9f,0xac,0x76,0x90,0x7e,0x6e,0x69,0x71,0x6d,0x6a,0x78,0x60,0x73,0x00,}}, // "org_"
        // ASCII-7-bit=181  Latin1=159  UTF8=172  GB=118  CP1252=144  KSC=126  SJS=110  EUC-JP=105  BIG5=113  Latin2=109  CP1251=106  CP1256=120  CP1250=96  Latin5=115  [top ASCII-7-bit]
  {{0x70,0x61,0x5f,0x5f, 0x03,0xa3,0xb8,0xa7,0x11,0x87,0x71,0x31,0x21,0x41,0x21,0x31,0x00,0x00,0x00,0x00,}}, // "pa__"
        // ASCII-7-bit=163  Latin1=184  UTF8=167  CP1252=135  CP1250=49  ISO-8859-15=65  KOI8R=49  [top Latin1]
  {{0x70,0x65,0x5f,0x5f, 0x05,0xa8,0xb8,0x9d,0x3a,0x99,0x11,0x2e,0x13,0x2e,0x1e,0x1e,0x11,0x32,0x00,0x00,}}, // "pe__"
        // ASCII-7-bit=168  Latin1=184  UTF8=157  GB=58  CP1252=153  SJS=46  BIG5=46  Latin2=30  CP1251=30  CP1250=50  [top Latin1]
  {{0x70,0x66,0x5f,0x5f, 0x03,0xa2,0xb5,0xa3,0x11,0xa7,0x11,0x63,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "pf__"
        // ASCII-7-bit=162  Latin1=181  UTF8=163  CP1252=167  SJS=99  [top Latin1]
  {{0x70,0x67,0x5f,0x5f, 0x05,0xb8,0xa6,0x81,0x6a,0xa5,0x11,0x70,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "pg__"
        // ASCII-7-bit=184  Latin1=166  UTF8=129  GB=106  CP1252=165  SJS=112  [top ASCII-7-bit]
  {{0x70,0x68,0x5f,0x5f, 0x03,0xb5,0x9e,0x99,0x15,0xac,0x57,0x65,0x4f,0x51,0x31,0x31,0x11,0x4b,0xd1,0x29,}}, // "ph__"
        // ASCII-7-bit=181  Latin1=158  UTF8=153  CP1252=172  KSC=87  SJS=101  EUC-JP=79  BIG5=81  CP1250=49  ISO-8859-11=75  CP874=41  [top ASCII-7-bit]
  {{0x70,0x6b,0x5f,0x5f, 0x03,0xb9,0x9b,0x8c,0x11,0xa3,0x11,0x33,0x11,0x99,0x21,0x29,0x00,0x00,0x00,0x00,}}, // "pk__"
        // ASCII-7-bit=185  Latin1=155  UTF8=140  CP1252=163  SJS=51  BIG5=153  CP1256=41  [top ASCII-7-bit]
  {{0x70,0x6c,0x5f,0x5f, 0x03,0x89,0x62,0xa8,0x11,0x4b,0x42,0xba,0x40,0x11,0x92,0xe1,0x3e,0x00,0x00,0x00,}}, // "pl__"
        // ASCII-7-bit=137  Latin1=98  UTF8=168  CP1252=75  Latin2=186  CP1251=64  CP1250=146  ISO-8859-5=62  [top Latin2]
  {{0x70,0x6e,0x5f,0x5f, 0x06,0xbb,0xa1,0x99,0x66,0x66,0x66,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "pn__"
        // ASCII-7-bit=187  Latin1=161  UTF8=153  GB=102  CP1252=102  KSC=102  [top ASCII-7-bit]
  {{0x70,0x72,0x5f,0x5f, 0x03,0x9b,0xb5,0xaf,0x11,0x94,0x41,0x65,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "pr__"
        // ASCII-7-bit=155  Latin1=181  UTF8=175  CP1252=148  Latin2=101  [top Latin1]
  {{0x70,0x72,0x6f,0x5f, 0x03,0xb3,0x9e,0xad,0x11,0x87,0x13,0x65,0x6f,0x65,0x11,0x9f,0x21,0x79,0x00,0x00,}}, // "pro_"
        // ASCII-7-bit=179  Latin1=158  UTF8=173  CP1252=135  SJS=101  EUC-JP=111  BIG5=101  CP1251=159  Latin5=121  [top ASCII-7-bit]
  {{0x70,0x73,0x5f,0x5f, 0x03,0x99,0x8f,0x9d,0x11,0x9f,0x61,0xb9,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "ps__"
        // ASCII-7-bit=153  Latin1=143  UTF8=157  CP1252=159  CP1256=185  [top CP1256]
  {{0x70,0x74,0x5f,0x5f, 0x05,0x99,0xb5,0xad,0x1d,0x95,0x11,0x21,0x52,0x36,0x40,0x12,0x90,0x23,0x51,0x1d,}}, // "pt__"
        // ASCII-7-bit=153  Latin1=181  UTF8=173  GB=29  CP1252=149  SJS=33  CP1250=54  Latin5=64  ISO-8859-15=144  CP1257=35  CP1254=29  [top Latin1]
  {{0x70,0x79,0x5f,0x5f, 0x03,0x9f,0xbb,0x90,0x13,0x97,0x49,0x35,0x21,0x35,0x51,0x5a,0x00,0x00,0x00,0x00,}}, // "py__"
        // ASCII-7-bit=159  Latin1=187  UTF8=144  CP1252=151  KSC=73  SJS=53  Latin2=53  ISO-8859-15=90  [top Latin1]
  {{0x71,0x61,0x5f,0x5f, 0x03,0x9a,0x89,0xb0,0x11,0x82,0x61,0xb5,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "qa__"
        // ASCII-7-bit=154  Latin1=137  UTF8=176  CP1252=130  CP1256=181  [top CP1256]
  {{0x72,0x65,0x5f,0x5f, 0x03,0x8a,0xb4,0xb1,0x11,0x97,0xa1,0x8a,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "re__"
        // ASCII-7-bit=138  Latin1=180  UTF8=177  CP1252=151  ISO-8859-15=138  [top Latin1]
  {{0x72,0x6f,0x5f,0x5f, 0x08,0xb4,0xa0,0xa4,0x3c,0x94,0x57,0x2e,0x29,0x14,0xa2,0x3a,0x33,0x8b,0x21,0x37,}}, // "ro__"
        // ASCII-7-bit=180  Latin1=160  UTF8=164  GB=60  CP1252=148  KSC=87  SJS=46  EUC-JP=41  Latin2=162  CP1251=58  CP1256=51  CP1250=139  ISO-8859-15=55  [top ASCII-7-bit]
  {{0x72,0x75,0x5f,0x5f, 0x05,0x8d,0x6d,0xa1,0x56,0x67,0x31,0x43,0x12,0xba,0x46,0x61,0x9b,0x72,0x46,0x48,}}, // "ru__"
        // ASCII-7-bit=141  Latin1=109  UTF8=161  GB=86  CP1252=103  BIG5=67  CP1251=186  CP1256=70  KOI8R=155  KOI8U=70  ISO-8859-5=72  [top CP1251]
  {{0x72,0x77,0x5f,0x5f, 0x03,0xb7,0xa2,0xa4,0x11,0xa0,0x61,0x5e,0x31,0x6e,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "rw__"
        // ASCII-7-bit=183  Latin1=162  UTF8=164  CP1252=160  CP1256=94  ISO-8859-15=110  [top ASCII-7-bit]
  {{0x73,0x61,0x5f,0x5f, 0x05,0x91,0x5b,0xac,0x3f,0x69,0x11,0x1f,0x41,0xb9,0x11,0x1f,0x00,0x00,0x00,0x00,}}, // "sa__"
        // ASCII-7-bit=145  Latin1=91  UTF8=172  GB=63  CP1252=105  SJS=31  CP1256=185  Latin5=31  [top CP1256]
  {{0x73,0x62,0x5f,0x5f, 0x03,0xb8,0x8a,0xad,0x11,0x8b,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "sb__"
        // ASCII-7-bit=184  Latin1=138  UTF8=173  CP1252=139  [top ASCII-7-bit]
  {{0x73,0x63,0x5f,0x5f, 0x03,0xb5,0x9e,0xad,0x11,0x85,0x12,0x8d,0x88,0x23,0x57,0x4d,0x5c,0x00,0x00,0x00,}}, // "sc__"
        // ASCII-7-bit=181  Latin1=158  UTF8=173  CP1252=133  SJS=141  EUC-JP=136  CP1251=87  CP1256=77  CP1250=92  [top ASCII-7-bit]
  {{0x73,0x64,0x5f,0x5f, 0x03,0x9b,0x77,0x8a,0x11,0x6d,0x61,0xbd,0x10,0x61,0x73,0x00,0x00,0x00,0x00,0x00,}}, // "sd__"
        // ASCII-7-bit=155  Latin1=119  UTF8=138  CP1252=109  CP1256=189  Arabic=115  [top CP1256]
  {{0x73,0x65,0x5f,0x5f, 0x05,0x96,0xbb,0xa0,0x23,0x82,0x36,0x23,0x2e,0x25,0x3b,0x43,0x22,0x11,0x4c,0x00,}}, // "se__"
        // ASCII-7-bit=150  Latin1=187  UTF8=160  GB=35  CP1252=130  BIG5=35  Latin2=46  CP1251=37  CP1256=59  CP1250=67  Latin5=34  ISO-8859-15=76  [top Latin1]
  {{0x73,0x67,0x5f,0x5f, 0x09,0xb8,0x9c,0xa6,0x84,0x94,0x4c,0x6e,0x50,0x71,0x31,0x41,0x11,0x48,0xd1,0x3a,}}, // "sg__"
        // ASCII-7-bit=184  Latin1=156  UTF8=166  GB=132  CP1252=148  KSC=76  SJS=110  EUC-JP=80  BIG5=113  CP1250=65  ISO-8859-11=72  CP874=58  [top ASCII-7-bit]
  {{0x73,0x68,0x5f,0x5f, 0x0a,0xaa,0x9b,0xa1,0xa9,0x84,0x77,0xa1,0x98,0x9b,0x5d,0x51,0x6d,0x31,0x6f,0x00,}}, // "sh__"
        // ASCII-7-bit=170  Latin1=155  UTF8=161  GB=169  CP1252=132  KSC=119  SJS=161  EUC-JP=152  BIG5=155  Latin2=93  ISO-8859-15=109  GBK=111  [top ASCII-7-bit]
  {{0x73,0x69,0x5f,0x5f, 0x03,0x95,0x6b,0xb7,0x11,0x6e,0x42,0x9f,0x3d,0x11,0xa9,0x21,0x17,0x10,0x11,0x24,}}, // "si__"
        // ASCII-7-bit=149  Latin1=107  UTF8=183  CP1252=110  Latin2=159  CP1251=61  CP1250=169  ISO-8859-15=23  CP852=36  [top UTF8]
  {{0x73,0x6b,0x5f,0x5f, 0x03,0x95,0x74,0xb0,0x11,0x60,0x36,0x53,0x92,0x55,0x47,0xb5,0x3f,0x10,0x11,0x3a,}}, // "sk__"
        // ASCII-7-bit=149  Latin1=116  UTF8=176  CP1252=96  BIG5=83  Latin2=146  CP1251=85  CP1256=71  CP1250=181  Latin5=63  MACINTOSH=58  [top CP1250]
  {{0x73,0x6c,0x5f,0x5f, 0x03,0xac,0x85,0x8f,0x11,0xb9,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "sl__"
        // ASCII-7-bit=172  Latin1=133  UTF8=143  CP1252=185  [top CP1252]
  {{0x73,0x6d,0x5f,0x5f, 0x03,0xa8,0xa7,0xb1,0x11,0xa8,0x91,0x6f,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "sm__"
        // ASCII-7-bit=168  Latin1=167  UTF8=177  CP1252=168  ISO-8859-11=111  [top UTF8]
  {{0x73,0x6e,0x5f,0x5f, 0x03,0x9d,0xb8,0x9f,0x11,0xa2,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "sn__"
        // ASCII-7-bit=157  Latin1=184  UTF8=159  CP1252=162  [top Latin1]
  {{0x73,0x72,0x5f,0x5f, 0x03,0xa6,0xad,0xb2,0x12,0x9d,0x6a,0x61,0x84,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "sr__"
        // ASCII-7-bit=166  Latin1=173  UTF8=178  CP1252=157  KSC=106  CP1250=132  [top UTF8]
  {{0x73,0x74,0x5f,0x5f, 0x03,0xb4,0xab,0x9d,0x15,0x88,0x79,0x99,0x94,0x7e,0x31,0x61,0x81,0x4e,0x21,0x6d,}}, // "st__"
        // ASCII-7-bit=180  Latin1=171  UTF8=157  CP1252=136  KSC=121  SJS=153  EUC-JP=148  BIG5=126  CP1250=97  JIS=78  CP932=109  [top ASCII-7-bit]
  {{0x73,0x75,0x5f,0x5f, 0x03,0xa4,0x6f,0xa0,0x11,0x70,0x51,0xb9,0x71,0x94,0x71,0x44,0x00,0x00,0x00,0x00,}}, // "su__"
        // ASCII-7-bit=164  Latin1=111  UTF8=160  CP1252=112  CP1251=185  KOI8R=148  KOI8U=68  [top CP1251]
  {{0x73,0x76,0x5f,0x5f, 0x03,0x9d,0xb9,0xa5,0x11,0x8f,0x41,0x3c,0x51,0x3c,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "sv__"
        // ASCII-7-bit=157  Latin1=185  UTF8=165  CP1252=143  Latin2=60  ISO-8859-15=60  [top Latin1]
  {{0x73,0x79,0x5f,0x5f, 0x03,0x82,0x5e,0x90,0x11,0x5d,0x61,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "sy__"
        // ASCII-7-bit=130  Latin1=94  UTF8=144  CP1252=93  CP1256=190  [top CP1256]
  {{0x73,0x7a,0x5f,0x5f, 0x03,0xb7,0xac,0x6c,0x11,0xa1,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "sz__"
        // ASCII-7-bit=183  Latin1=172  UTF8=108  CP1252=161  [top ASCII-7-bit]
  {{0x74,0x63,0x5f,0x5f, 0x08,0xa9,0xaa,0x9b,0x62,0x74,0x8d,0x8f,0x7a,0x21,0x67,0x22,0xab,0x9b,0x41,0x70,}}, // "tc__"
        // ASCII-7-bit=169  Latin1=170  UTF8=155  GB=98  CP1252=116  KSC=141  SJS=143  EUC-JP=122  CP1251=103  Latin5=171  ISO-8859-11=155  GBK=112  [top Latin5]
  {{0x74,0x66,0x5f,0x5f, 0x02,0xa3,0xbc,0x21,0x71,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "tf__"
        // ASCII-7-bit=163  Latin1=188  CP1252=113  [top Latin1]
  {{0x74,0x67,0x5f,0x5f, 0x03,0xb0,0xb3,0x7e,0x11,0xa5,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "tg__"
        // ASCII-7-bit=176  Latin1=179  UTF8=126  CP1252=165  [top Latin1]
  {{0x74,0x68,0x5f,0x5f, 0x09,0x96,0x61,0x93,0x3e,0x67,0x2f,0x52,0x38,0x23,0x11,0x35,0x31,0xbd,0xd1,0x76,}}, // "th__"
        // ASCII-7-bit=150  Latin1=97  UTF8=147  GB=62  CP1252=103  KSC=47  SJS=82  EUC-JP=56  BIG5=35  CP1251=53  ISO-8859-11=189  CP874=118  [top ISO-8859-11]
  {{0x74,0x6a,0x5f,0x5f, 0x03,0xab,0x74,0xb1,0x11,0x67,0x51,0xae,0x71,0x84,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "tj__"
        // ASCII-7-bit=171  Latin1=116  UTF8=177  CP1252=103  CP1251=174  KOI8R=132  [top UTF8]
  {{0x74,0x6b,0x5f,0x5f, 0x03,0xbc,0x94,0x9d,0x11,0x6b,0x12,0x74,0x6b,0x12,0x53,0x46,0x12,0x74,0x6d,0x00,}}, // "tk__"
        // ASCII-7-bit=188  Latin1=148  UTF8=157  CP1252=107  SJS=116  EUC-JP=107  Latin2=83  CP1251=70  CP1250=116  Latin5=109  [top ASCII-7-bit]
  {{0x74,0x6c,0x5f,0x5f, 0x05,0xb1,0xb0,0x88,0x61,0x83,0xa1,0xa8,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "tl__"
        // ASCII-7-bit=177  Latin1=176  UTF8=136  GB=97  CP1252=131  ISO-8859-15=168  [top ASCII-7-bit]
  {{0x74,0x6d,0x5f,0x5f, 0x03,0xb4,0x84,0xad,0x11,0x82,0x42,0x5a,0xa4,0x12,0x5a,0x5a,0x00,0x00,0x00,0x00,}}, // "tm__"
        // ASCII-7-bit=180  Latin1=132  UTF8=173  CP1252=130  Latin2=90  CP1251=164  CP1250=90  Latin5=90  [top ASCII-7-bit]
  {{0x74,0x6e,0x5f,0x5f, 0x03,0x9f,0xa2,0xac,0x11,0x9d,0x11,0x4b,0x21,0x3b,0x11,0xb0,0x10,0x61,0x3b,0x00,}}, // "tn__"
        // ASCII-7-bit=159  Latin1=162  UTF8=172  CP1252=157  SJS=75  Latin2=59  CP1256=176  Arabic=59  [top CP1256]
  {{0x74,0x6f,0x5f,0x5f, 0x03,0xa5,0xa2,0xa5,0x15,0x89,0x8a,0xac,0x99,0x9b,0x31,0x70,0x21,0x87,0x51,0x6c,}}, // "to__"
        // ASCII-7-bit=165  Latin1=162  UTF8=165  CP1252=137  KSC=138  SJS=172  EUC-JP=153  BIG5=155  CP1250=112  ISO-8859-15=135  JIS=108  [top SJS]
  {{0x74,0x70,0x5f,0x5f, 0x03,0x95,0x9e,0xad,0x11,0x67,0x12,0xb3,0x99,0xd1,0x67,0x21,0x67,0x00,0x00,0x00,}}, // "tp__"
        // ASCII-7-bit=149  Latin1=158  UTF8=173  CP1252=103  SJS=179  EUC-JP=153  JIS=103  CP932=103  [top SJS]
  {{0x74,0x72,0x5f,0x5f, 0x03,0x8d,0x6c,0xa6,0x11,0x61,0x11,0x3c,0x32,0x3c,0x4a,0x11,0xba,0x81,0x91,0x00,}}, // "tr__"
        // ASCII-7-bit=141  Latin1=108  UTF8=166  CP1252=97  SJS=60  CP1251=60  CP1256=74  Latin5=186  CP1254=145  [top Latin5]
  {{0x74,0x72,0x61,0x76, 0x05,0xa9,0xa3,0xa7,0x97,0x95,0x11,0xac,0x13,0x74,0x7f,0x6b,0x11,0x63,0x00,0x00,}}, // "trav"
        // ASCII-7-bit=169  Latin1=163  UTF8=167  GB=151  CP1252=149  SJS=172  BIG5=116  Latin2=127  CP1251=107  CP1250=99  [top SJS]
  {{0x74,0x74,0x5f,0x5f, 0x07,0xb5,0xaf,0x9a,0x49,0x92,0x59,0x49,0x61,0x75,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "tt__"
        // ASCII-7-bit=181  Latin1=175  UTF8=154  GB=73  CP1252=146  KSC=89  SJS=73  Latin5=117  [top ASCII-7-bit]
  {{0x74,0x76,0x5f,0x5f, 0x0e,0xa7,0xa6,0xad,0x89,0x94,0x85,0x9e,0x8d,0x7b,0x74,0x7c,0x88,0x7b,0x81,0x00,}}, // "tv__"
        // ASCII-7-bit=167  Latin1=166  UTF8=173  GB=137  CP1252=148  KSC=133  SJS=158  EUC-JP=141  BIG5=123  Latin2=116  CP1251=124  CP1256=136  CP1250=123  Latin5=129  [top UTF8]
  {{0x74,0x77,0x5f,0x5f, 0x05,0x85,0x52,0xab,0x5d,0x57,0x13,0x50,0x2e,0xba,0x31,0x23,0x61,0x2e,0xf1,0x21,}}, // "tw__"
        // ASCII-7-bit=133  Latin1=82  UTF8=171  GB=93  CP1252=87  SJS=80  EUC-JP=46  BIG5=186  CP1250=35  GBK=46  BIG5_HKSCS=33  [top BIG5]
  {{0x74,0x7a,0x5f,0x5f, 0x03,0xae,0xb5,0x8b,0x13,0xa0,0x4c,0x4c,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "tz__"
        // ASCII-7-bit=174  Latin1=181  UTF8=139  CP1252=160  KSC=76  SJS=76  [top Latin1]
  {{0x75,0x61,0x5f,0x5f, 0x03,0x87,0x61,0xa2,0x11,0x57,0x44,0x4d,0xbb,0x35,0x48,0x51,0x80,0x72,0x8c,0x3d,}}, // "ua__"
        // ASCII-7-bit=135  Latin1=97  UTF8=162  CP1252=87  Latin2=77  CP1251=187  CP1256=53  CP1250=72  KOI8R=128  KOI8U=140  ISO-8859-5=61  [top CP1251]
  {{0x75,0x67,0x5f,0x5f, 0x03,0xb8,0x9a,0xa0,0x11,0xa6,0x11,0x68,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "ug__"
        // ASCII-7-bit=184  Latin1=154  UTF8=160  CP1252=166  SJS=104  [top ASCII-7-bit]
  {{0x75,0x6b,0x5f,0x5f, 0x05,0xb7,0xa7,0x9e,0x62,0x97,0x27,0x58,0x62,0x51,0x53,0x62,0x64,0x54,0x11,0x6f,}}, // "uk__"
        // ASCII-7-bit=183  Latin1=167  UTF8=158  GB=98  CP1252=151  EUC-JP=88  BIG5=98  Latin2=81  CP1251=83  CP1256=98  CP1250=100  Latin5=84  ISO-8859-15=111  [top ASCII-7-bit]
  {{0x75,0x73,0x5f,0x5f, 0x06,0xba,0x94,0xa5,0x45,0x92,0x48,0x24,0x54,0x47,0x5a,0x56,0x11,0x66,0x11,0x40,}}, // "us__"
        // ASCII-7-bit=186  Latin1=148  UTF8=165  GB=69  CP1252=146  KSC=72  BIG5=84  Latin2=71  CP1251=90  CP1256=86  Latin5=102  ISO-8859-15=64  [top ASCII-7-bit]
  {{0x75,0x79,0x5f,0x5f, 0x03,0xa2,0xba,0x96,0x11,0x9a,0x72,0x2c,0x2c,0x11,0x52,0x00,0x00,0x00,0x00,0x00,}}, // "uy__"
        // ASCII-7-bit=162  Latin1=186  UTF8=150  CP1252=154  CP1250=44  Latin5=44  ISO-8859-15=82  [top Latin1]
  {{0x75,0x7a,0x5f,0x5f, 0x05,0x91,0x68,0xb1,0x39,0x88,0x54,0xb5,0x43,0x39,0x6e,0x41,0x7c,0x00,0x00,0x00,}}, // "uz__"
        // ASCII-7-bit=145  Latin1=104  UTF8=177  GB=57  CP1252=136  CP1251=181  CP1256=67  CP1250=57  Latin5=110  KOI8R=124  [top CP1251]
  {{0x76,0x61,0x5f,0x5f, 0x03,0xae,0xb3,0x8f,0x11,0xa7,0xb1,0x4a,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "va__"
        // ASCII-7-bit=174  Latin1=179  UTF8=143  CP1252=167  CP1257=74  [top Latin1]
  {{0x76,0x63,0x5f,0x5f, 0x08,0xba,0x7a,0x9a,0x5e,0x79,0x4a,0x9a,0x95,0x12,0x54,0x7f,0x81,0x4a,0x00,0x00,}}, // "vc__"
        // ASCII-7-bit=186  Latin1=122  UTF8=154  GB=94  CP1252=121  KSC=74  SJS=154  EUC-JP=149  Latin2=84  CP1251=127  GBK=74  [top ASCII-7-bit]
  {{0x76,0x65,0x5f,0x5f, 0x05,0x97,0xbc,0x8c,0x14,0x9b,0x11,0x14,0x11,0x1e,0x31,0x42,0x21,0x55,0x31,0x24,}}, // "ve__"
        // ASCII-7-bit=151  Latin1=188  UTF8=140  GB=20  CP1252=155  SJS=20  BIG5=30  CP1250=66  ISO-8859-15=85  GBK=36  [top Latin1]
  {{0x76,0x67,0x5f,0x5f, 0x05,0xac,0xb2,0x9e,0x6d,0x97,0x13,0x95,0x81,0x95,0x11,0x69,0x21,0x63,0x61,0x59,}}, // "vg__"
        // ASCII-7-bit=172  Latin1=178  UTF8=158  GB=109  CP1252=151  SJS=149  EUC-JP=129  BIG5=149  CP1251=105  Latin5=99  Greek=89  [top Latin1]
  {{0x76,0x69,0x5f,0x5f, 0x03,0xb9,0x90,0xaa,0x11,0x93,0x11,0x68,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "vi__"
        // ASCII-7-bit=185  Latin1=144  UTF8=170  CP1252=147  SJS=104  [top ASCII-7-bit]
  {{0x76,0x6e,0x5f,0x5f, 0x03,0x94,0x92,0xbd,0x12,0x83,0x22,0x21,0x2c,0x32,0x26,0x2e,0x00,0x00,0x00,0x00,}}, // "vn__"
        // ASCII-7-bit=148  Latin1=146  UTF8=189  CP1252=131  KSC=34  BIG5=44  CP1250=38  Latin5=46  [top UTF8]
  {{0x76,0x75,0x5f,0x5f, 0x03,0xae,0xb6,0x97,0x11,0x7b,0x11,0x5b,0x31,0x6a,0x41,0x4e,0x71,0x90,0x00,0x00,}}, // "vu__"
        // ASCII-7-bit=174  Latin1=182  UTF8=151  CP1252=123  SJS=91  CP1251=106  ISO-8859-15=78  CP1253=144  [top Latin1]
  {{0x77,0x73,0x5f,0x5f, 0x05,0xae,0xaa,0xa6,0x6c,0x91,0x18,0x76,0x69,0x7c,0x78,0x99,0x9d,0x7d,0x70,0x00,}}, // "ws__"
        // ASCII-7-bit=174  Latin1=170  UTF8=166  GB=108  CP1252=145  SJS=118  EUC-JP=105  BIG5=124  Latin2=120  CP1251=153  CP1256=157  CP1250=125  Latin5=112  [top ASCII-7-bit]
  {{0x79,0x65,0x5f,0x5f, 0x03,0x9e,0x94,0xab,0x11,0x8b,0x61,0xb6,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "ye__"
        // ASCII-7-bit=158  Latin1=148  UTF8=171  CP1252=139  CP1256=182  [top CP1256]
  {{0x79,0x75,0x5f,0x5f, 0x03,0xa4,0x7f,0xaf,0x11,0x78,0x42,0x87,0x80,0x12,0xb3,0x2c,0xd1,0x60,0x00,0x00,}}, // "yu__"
        // ASCII-7-bit=164  Latin1=127  UTF8=175  CP1252=120  Latin2=135  CP1251=128  CP1250=179  Latin5=44  ISO-8859-5=96  [top CP1250]
  {{0x7a,0x61,0x5f,0x5f, 0x05,0xb8,0xa3,0x97,0x42,0xa1,0x11,0x30,0x11,0x4e,0x13,0x3a,0x4f,0x41,0x21,0x42,}}, // "za__"
        // ASCII-7-bit=184  Latin1=163  UTF8=151  GB=66  CP1252=161  SJS=48  BIG5=78  CP1251=58  CP1256=79  CP1250=65  ISO-8859-15=66  [top ASCII-7-bit]
  {{0x7a,0x6d,0x5f,0x5f, 0x03,0xb8,0x8e,0x9c,0x11,0xa9,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "zm__"
        // ASCII-7-bit=184  Latin1=142  UTF8=156  CP1252=169  [top ASCII-7-bit]
  {{0x7a,0x77,0x5f,0x5f, 0x05,0xbb,0x95,0x9b,0x59,0x9a,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "zw__"
        // ASCII-7-bit=187  Latin1=149  UTF8=155  GB=89  CP1252=154  [top ASCII-7-bit]
};

static const int kTLDHintProbsSize = 247;

static const HintEntry kCharsetHintProbs[] = {	// MaxRange 192
  {{0x5f,0x5f,0x5f,0x5f,0x30,0x36,0x34,0x36, 0x02,0xbd,0x7f,0x21,0x95,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "____0646"
        // ASCII-7-bit=189  Latin1=127  CP1252=149  [top ASCII-7-bit]
  {{0x5f,0x5f,0x5f,0x5f,0x31,0x32,0x35,0x30, 0x01,0x96,0xb1,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "____1250"
        // ASCII-7-bit=150  CP1250=190  [top CP1250]
  {{0x5f,0x5f,0x5f,0x5f,0x31,0x32,0x35,0x31, 0x01,0x7a,0x91,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "____1251"
        // ASCII-7-bit=122  CP1251=190  [top CP1251]
  {{0x5f,0x5f,0x5f,0x5f,0x31,0x32,0x35,0x32, 0x02,0x99,0x9d,0x21,0xbc,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "____1252"
        // ASCII-7-bit=153  Latin1=157  CP1252=188  [top CP1252]
  {{0x5f,0x5f,0x5f,0x5f,0x31,0x32,0x35,0x33, 0x01,0x79,0x10,0x61,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "____1253"
        // ASCII-7-bit=121  CP1253=190  [top CP1253]
  {{0x5f,0x5f,0x5f,0x5f,0x31,0x32,0x35,0x34, 0x01,0x71,0xc1,0xaf,0x81,0xb9,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "____1254"
        // ASCII-7-bit=113  Latin5=175  CP1254=185  [top CP1254]
  {{0x5f,0x5f,0x5f,0x5f,0x31,0x32,0x35,0x35, 0x01,0x86,0x10,0x01,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "____1255"
        // ASCII-7-bit=134  CP1255=190  [top CP1255]
  {{0x5f,0x5f,0x5f,0x5f,0x31,0x32,0x35,0x36, 0x01,0x78,0xa1,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "____1256"
        // ASCII-7-bit=120  CP1256=190  [top CP1256]
  {{0x5f,0x5f,0x5f,0x5f,0x31,0x32,0x35,0x37, 0x01,0x79,0xf1,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "____1257"
        // ASCII-7-bit=121  CP1257=190  [top CP1257]
  {{0x5f,0x5f,0x5f,0x5f,0x31,0x38,0x30,0x30, 0x10,0x21,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "____1800"
        // KOI8R=191  [top KOI8R]
  {{0x5f,0x5f,0x5f,0x5f,0x33,0x36,0x30,0x30, 0xc1,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "____3600"
        // CP1250=191  [top CP1250]
  {{0x5f,0x5f,0x5f,0x5f,0x33,0x36,0x39,0x39, 0x01,0xad,0x11,0xb9,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "____3699"
        // ASCII-7-bit=173  UTF8=185  [top UTF8]
  {{0x5f,0x5f,0x5f,0x5f,0x34,0x34,0x30,0x30, 0x02,0xbc,0x87,0x21,0xa4,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "____4400"
        // ASCII-7-bit=188  Latin1=135  CP1252=164  [top ASCII-7-bit]
  {{0x5f,0x5f,0x5f,0x5f,0x35,0x30,0x30,0x31, 0x01,0x9a,0x11,0xbd,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "____5001"
        // ASCII-7-bit=154  UTF8=189  [top UTF8]
  {{0x5f,0x5f,0x5f,0x5f,0x35,0x39,0x31,0x35, 0x02,0xa8,0xa6,0x21,0xa7,0xa1,0xb2,0x00,0x00,0x00,0x00,0x00,}}, // "____5915"
        // ASCII-7-bit=168  Latin1=166  CP1252=167  ISO-8859-15=178  [top ISO-8859-15]
  {{0x5f,0x5f,0x5f,0x5f,0x36,0x34,0x36,0x5f, 0x02,0xbe,0x8e,0x21,0x81,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "____646_"
        // ASCII-7-bit=190  Latin1=142  CP1252=129  [top ASCII-7-bit]
  {{0x5f,0x5f,0x5f,0x5f,0x38,0x35,0x39,0x31, 0x02,0xae,0xb8,0x21,0x94,0xa1,0x2f,0x00,0x00,0x00,0x00,0x00,}}, // "____8591"
        // ASCII-7-bit=174  Latin1=184  CP1252=148  ISO-8859-15=47  [top Latin1]
  {{0x5f,0x5f,0x5f,0x5f,0x38,0x35,0x39,0x32, 0x01,0x8c,0x81,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "____8592"
        // ASCII-7-bit=140  Latin2=190  [top Latin2]
  {{0x5f,0x5f,0x5f,0x5f,0x38,0x35,0x39,0x34, 0x10,0xe1,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "____8594"
        // Latin4=191  [top Latin4]
  {{0x5f,0x5f,0x5f,0x5f,0x38,0x35,0x39,0x35, 0x01,0x6f,0x10,0xa1,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "____8595"
        // ASCII-7-bit=111  ISO-8859-5=190  [top ISO-8859-5]
  {{0x5f,0x5f,0x5f,0x5f,0x38,0x35,0x39,0x37, 0x10,0x41,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "____8597"
        // Greek=191  [top Greek]
  {{0x5f,0x5f,0x5f,0x5f,0x38,0x35,0x39,0x39, 0x01,0x72,0xc1,0xbe,0x81,0x7b,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "____8599"
        // ASCII-7-bit=114  Latin5=190  CP1254=123  [top Latin5]
  {{0x5f,0x5f,0x5f,0x5f,0x38,0x38,0x36,0x31, 0x91,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "____8861"
        // Latin2=191  [top Latin2]
  {{0x5f,0x5f,0x5f,0x5f,0x38,0x5f,0x5f,0x5f, 0x03,0x98,0x5d,0xbd,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "____8___"
        // ASCII-7-bit=152  Latin1=93  UTF8=189  [top UTF8]
  {{0x5f,0x5f,0x5f,0x5f,0x5f,0x5f,0x5f,0x5f, 0x07,0xb1,0xaa,0x9c,0x95,0x96,0x8e,0x8c,0x11,0x82,0x00,0x00,}}, // "________"
        // ASCII-7-bit=177  Latin1=170  UTF8=156  GB=149  CP1252=150  KSC=142  SJS=140  BIG5=130  [top ASCII-7-bit]
  {{0x61,0x6e,0x73,0x69,0x33,0x34,0x5f,0x5f, 0x02,0xbe,0x6e,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "ansi34__"
        // ASCII-7-bit=190  Latin1=110  [top ASCII-7-bit]
  {{0x61,0x6e,0x73,0x69,0x5f,0x5f,0x5f,0x5f, 0x02,0xa2,0xb9,0x21,0xa4,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "ansi____"
        // ASCII-7-bit=162  Latin1=185  CP1252=164  [top Latin1]
  {{0x61,0x72,0x72,0x61,0x5f,0x5f,0x5f,0x5f, 0x01,0x90,0xa1,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "arra____"
        // ASCII-7-bit=144  CP1256=190  [top CP1256]
  {{0x61,0x73,0x63,0x69,0x5f,0x5f,0x5f,0x5f, 0x02,0xbe,0x72,0x21,0x71,0xa1,0x53,0x00,0x00,0x00,0x00,0x00,}}, // "asci____"
        // ASCII-7-bit=190  Latin1=114  CP1252=113  ISO-8859-15=83  [top ASCII-7-bit]
  {{0x61,0x75,0x74,0x6f,0x5f,0x5f,0x5f,0x5f, 0x01,0x9b,0x51,0xbd,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "auto____"
        // ASCII-7-bit=155  SJS=189  [top SJS]
  {{0x62,0x67,0x5f,0x5f,0x32,0x33,0x31,0x32, 0x01,0x93,0x21,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "bg__2312"
        // ASCII-7-bit=147  GB=190  [top GB]
  {{0x62,0x68,0x61,0x73,0x5f,0x5f,0x5f,0x5f, 0x30,0x01,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "bhas____"
        // BHASKAR=191  [top BHASKAR]
  {{0x62,0x69,0x67,0x5f,0x35,0x5f,0x5f,0x5f, 0x01,0x84,0x71,0xbe,0x10,0xa1,0x2f,0x00,0x00,0x00,0x00,0x00,}}, // "big_5___"
        // ASCII-7-bit=132  BIG5=190  BIG5_HKSCS=47  [top BIG5]
  {{0x62,0x69,0x67,0x5f,0x38,0x35,0x39,0x31, 0x11,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "big_8591"
        // Latin1=191  [top Latin1]
  {{0x62,0x69,0x67,0x68,0x35,0x5f,0x5f,0x5f, 0x01,0x88,0x71,0xae,0x10,0xa1,0xb8,0x00,0x00,0x00,0x00,0x00,}}, // "bigh5___"
        // ASCII-7-bit=136  BIG5=174  BIG5_HKSCS=184  [top BIG5_HKSCS]
  {{0x62,0x69,0x6e,0x61,0x5f,0x5f,0x5f,0x5f, 0x30,0xa1,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "bina____"
        // X-BINARYENC=191  [top X-BINARYENC]
  {{0x62,0x6f,0x74,0x5f,0x5f,0x5f,0x5f,0x5f, 0xd1,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "bot_____"
        // Latin5=191  [top Latin5]
  {{0x62,0x73,0x5f,0x5f,0x34,0x37,0x33,0x30, 0x02,0xb8,0xa8,0x21,0xa3,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "bs__4730"
        // ASCII-7-bit=184  Latin1=168  CP1252=163  [top ASCII-7-bit]
  {{0x63,0x68,0x61,0x72,0x5f,0x5f,0x5f,0x5f, 0x02,0xa5,0xbb,0x21,0x91,0xa1,0x28,0x00,0x00,0x00,0x00,0x00,}}, // "char____"
        // ASCII-7-bit=165  Latin1=187  CP1252=145  ISO-8859-15=40  [top Latin1]
  {{0x63,0x6e,0x73,0x5f,0x5f,0x5f,0x5f,0x5f, 0x30,0x71,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "cns_____"
        // CNS=191  [top CNS]
  {{0x63,0x6f,0x6e,0x66,0x5f,0x5f,0x5f,0x5f, 0x01,0x9f,0x11,0xbd,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "conf____"
        // ASCII-7-bit=159  UTF8=189  [top UTF8]
  {{0x63,0x6f,0x6e,0x74,0x5f,0x5f,0x5f,0x5f, 0x01,0xa4,0x11,0xbc,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "cont____"
        // ASCII-7-bit=164  UTF8=188  [top UTF8]
  {{0x63,0x70,0x5f,0x5f,0x31,0x32,0x35,0x30, 0x01,0x97,0xb1,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "cp__1250"
        // ASCII-7-bit=151  CP1250=190  [top CP1250]
  {{0x63,0x70,0x5f,0x5f,0x31,0x32,0x35,0x31, 0x01,0x7c,0x91,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "cp__1251"
        // ASCII-7-bit=124  CP1251=190  [top CP1251]
  {{0x63,0x70,0x5f,0x5f,0x31,0x32,0x35,0x32, 0x02,0xab,0xa9,0x21,0xb5,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "cp__1252"
        // ASCII-7-bit=171  Latin1=169  CP1252=181  [top CP1252]
  {{0x63,0x70,0x5f,0x5f,0x31,0x32,0x35,0x33, 0x01,0x79,0x10,0x31,0x7f,0x21,0xbe,0x00,0x00,0x00,0x00,0x00,}}, // "cp__1253"
        // ASCII-7-bit=121  Greek=127  CP1253=190  [top CP1253]
  {{0x63,0x70,0x5f,0x5f,0x31,0x32,0x35,0x34, 0x01,0x5b,0xc1,0xaf,0x81,0xb9,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "cp__1254"
        // ASCII-7-bit=91  Latin5=175  CP1254=185  [top CP1254]
  {{0x63,0x70,0x5f,0x5f,0x31,0x32,0x35,0x35, 0x01,0x86,0x10,0x01,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "cp__1255"
        // ASCII-7-bit=134  CP1255=190  [top CP1255]
  {{0x63,0x70,0x5f,0x5f,0x31,0x32,0x35,0x36, 0x01,0x5e,0xa1,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "cp__1256"
        // ASCII-7-bit=94  CP1256=190  [top CP1256]
  {{0x63,0x70,0x5f,0x5f,0x31,0x32,0x35,0x37, 0x01,0xa8,0xf1,0xbb,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "cp__1257"
        // ASCII-7-bit=168  CP1257=187  [top CP1257]
  {{0x63,0x70,0x5f,0x5f,0x38,0x35,0x30,0x5f, 0x02,0x97,0x98,0x21,0x8c,0xa1,0xbc,0x00,0x00,0x00,0x00,0x00,}}, // "cp__850_"
        // ASCII-7-bit=151  Latin1=152  CP1252=140  ISO-8859-15=188  [top ISO-8859-15]
  {{0x63,0x70,0x5f,0x5f,0x38,0x35,0x32,0x5f, 0x01,0x8f,0x20,0x01,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "cp__852_"
        // ASCII-7-bit=143  CP852=190  [top CP852]
  {{0x63,0x70,0x5f,0x5f,0x38,0x36,0x36,0x5f, 0x01,0xa2,0x20,0x31,0xbc,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "cp__866_"
        // ASCII-7-bit=162  CP866=188  [top CP866]
  {{0x63,0x70,0x63,0x5f,0x39,0x34,0x33,0x5f, 0x01,0x26,0x51,0xbe,0x10,0x11,0x68,0x00,0x00,0x00,0x00,0x00,}}, // "cpc_943_"
        // ASCII-7-bit=38  SJS=190  CP932=104  [top SJS]
  {{0x63,0x70,0x63,0x7a,0x31,0x32,0x35,0x30, 0xc1,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "cpcz1250"
        // CP1250=191  [top CP1250]
  {{0x63,0x73,0x69,0x73,0x5f,0x5f,0x5f,0x5f, 0x01,0x9c,0x10,0x01,0xbd,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "csis____"
        // ASCII-7-bit=156  CP1255=189  [top CP1255]
  {{0x63,0x73,0x6e,0x5f,0x39,0x31,0x30,0x33, 0x20,0x91,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "csn_9103"
        // CSN_369103=191  [top CSN_369103]
  {{0x63,0x73,0x73,0x68,0x5f,0x5f,0x5f,0x5f, 0x01,0x7f,0x51,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "cssh____"
        // ASCII-7-bit=127  SJS=190  [top SJS]
  {{0x63,0x73,0x77,0x69,0x31,0x32,0x35,0x30, 0xc1,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "cswi1250"
        // CP1250=191  [top CP1250]
  {{0x63,0x73,0x77,0x69,0x33,0x31,0x5f,0x5f, 0x61,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "cswi31__"
        // SJS=191  [top SJS]
  {{0x63,0x7a,0x77,0x69,0x31,0x32,0x35,0x30, 0xc1,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "czwi1250"
        // CP1250=191  [top CP1250]
  {{0x64,0x61,0x64,0x6b,0x38,0x35,0x39,0x31, 0x11,0xbe,0x21,0x7d,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "dadk8591"
        // Latin1=190  CP1252=125  [top Latin1]
  {{0x64,0x61,0x69,0x73,0x38,0x35,0x39,0x31, 0x02,0x6f,0xbe,0x21,0x6f,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "dais8591"
        // ASCII-7-bit=111  Latin1=190  CP1252=111  [top Latin1]
  {{0x64,0x65,0x5f,0x5f,0x5f,0x5f,0x5f,0x5f, 0x02,0x9d,0xbc,0x21,0x95,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "de______"
        // ASCII-7-bit=157  Latin1=188  CP1252=149  [top Latin1]
  {{0x64,0x65,0x61,0x73,0x5f,0x5f,0x5f,0x5f, 0x02,0x8f,0xbd,0x21,0x92,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "deas____"
        // ASCII-7-bit=143  Latin1=189  CP1252=146  [top Latin1]
  {{0x64,0x65,0x64,0x65,0x38,0x35,0x39,0x31, 0x02,0x92,0xbe,0x21,0x87,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "dede8591"
        // ASCII-7-bit=146  Latin1=190  CP1252=135  [top Latin1]
  {{0x64,0x65,0x66,0x61,0x5f,0x5f,0x5f,0x5f, 0x02,0xbc,0x9f,0x21,0x89,0xa1,0x6b,0x00,0x00,0x00,0x00,0x00,}}, // "defa____"
        // ASCII-7-bit=188  Latin1=159  CP1252=137  ISO-8859-15=107  [top ASCII-7-bit]
  {{0x64,0x65,0x69,0x73,0x35,0x39,0x31,0x35, 0x11,0x83,0xd1,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "deis5915"
        // Latin1=131  ISO-8859-15=190  [top ISO-8859-15]
  {{0x64,0x65,0x69,0x73,0x38,0x35,0x39,0x31, 0x02,0x92,0xbd,0x21,0x89,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "deis8591"
        // ASCII-7-bit=146  Latin1=189  CP1252=137  [top Latin1]
  {{0x64,0x65,0x6c,0x65,0x5f,0x5f,0x5f,0x5f, 0x02,0xa9,0xba,0x21,0x92,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "dele____"
        // ASCII-7-bit=169  Latin1=186  CP1252=146  [top Latin1]
  {{0x64,0x65,0x75,0x74,0x5f,0x5f,0x5f,0x5f, 0x02,0x74,0xb8,0x21,0xaf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "deut____"
        // ASCII-7-bit=116  Latin1=184  CP1252=175  [top Latin1]
  {{0x64,0x6f,0x6f,0x72,0x31,0x32,0x35,0x32, 0x11,0x79,0x21,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "door1252"
        // Latin1=121  CP1252=190  [top CP1252]
  {{0x65,0x63,0x75,0x6a,0x5f,0x5f,0x5f,0x5f, 0x71,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "ecuj____"
        // EUC-JP=191  [top EUC-JP]
  {{0x65,0x63,0x75,0x6b,0x5f,0x5f,0x5f,0x5f, 0x01,0x71,0x41,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "ecuk____"
        // ASCII-7-bit=113  KSC=190  [top KSC]
  {{0x65,0x65,0x6d,0x73,0x31,0x32,0x35,0x30, 0xc1,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "eems1250"
        // CP1250=191  [top CP1250]
  {{0x65,0x6e,0x5f,0x5f,0x38,0x35,0x39,0x31, 0x02,0x6f,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "en__8591"
        // ASCII-7-bit=111  Latin1=190  [top Latin1]
  {{0x65,0x6e,0x5f,0x5f,0x5f,0x5f,0x5f,0x5f, 0x02,0xbe,0x92,0x21,0x82,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "en______"
        // ASCII-7-bit=190  Latin1=146  CP1252=130  [top ASCII-7-bit]
  {{0x65,0x6e,0x63,0x6f,0x5f,0x5f,0x5f,0x5f, 0x01,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "enco____"
        // ASCII-7-bit=191  [top ASCII-7-bit]
  {{0x65,0x6e,0x67,0x5f,0x5f,0x5f,0x5f,0x5f, 0x01,0x8b,0x71,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "eng_____"
        // ASCII-7-bit=139  BIG5=190  [top BIG5]
  {{0x65,0x6e,0x67,0x62,0x5f,0x5f,0x5f,0x5f, 0x02,0xbe,0x7d,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "engb____"
        // ASCII-7-bit=190  Latin1=125  [top ASCII-7-bit]
  {{0x65,0x6e,0x69,0x73,0x38,0x35,0x39,0x31, 0x02,0x96,0xbc,0x21,0x9a,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "enis8591"
        // ASCII-7-bit=150  Latin1=188  CP1252=154  [top Latin1]
  {{0x65,0x6e,0x75,0x6b,0x38,0x35,0x39,0x31, 0x11,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "enuk8591"
        // Latin1=191  [top Latin1]
  {{0x65,0x6e,0x75,0x6b,0x5f,0x5f,0x5f,0x5f, 0x51,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "enuk____"
        // KSC=191  [top KSC]
  {{0x65,0x6e,0x75,0x73,0x35,0x39,0x31,0x35, 0x02,0x6f,0x7f,0xd1,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "enus5915"
        // ASCII-7-bit=111  Latin1=127  ISO-8859-15=190  [top ISO-8859-15]
  {{0x65,0x6e,0x75,0x73,0x38,0x35,0x39,0x31, 0x02,0x9c,0xbc,0x21,0x9b,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "enus8591"
        // ASCII-7-bit=156  Latin1=188  CP1252=155  [top Latin1]
  {{0x65,0x6e,0x75,0x73,0x5f,0x5f,0x5f,0x5f, 0x02,0xbb,0xa1,0x21,0x9e,0xa1,0x68,0x00,0x00,0x00,0x00,0x00,}}, // "enus____"
        // ASCII-7-bit=187  Latin1=161  CP1252=158  ISO-8859-15=104  [top ASCII-7-bit]
  {{0x65,0x6e,0x75,0x74,0x38,0x5f,0x5f,0x5f, 0x01,0x81,0xf1,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "enut8___"
        // ASCII-7-bit=129  CP1257=190  [top CP1257]
  {{0x65,0x73,0x5f,0x5f,0x5f,0x5f,0x5f,0x5f, 0x02,0xb4,0xb3,0x21,0x9d,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "es______"
        // ASCII-7-bit=180  Latin1=179  CP1252=157  [top ASCII-7-bit]
  {{0x65,0x73,0x65,0x73,0x38,0x35,0x39,0x31, 0x02,0x82,0xbe,0x21,0x6e,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "eses8591"
        // ASCII-7-bit=130  Latin1=190  CP1252=110  [top Latin1]
  {{0x65,0x73,0x65,0x73,0x5f,0x5f,0x5f,0x5f, 0x02,0xa6,0xba,0x21,0x96,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "eses____"
        // ASCII-7-bit=166  Latin1=186  CP1252=150  [top Latin1]
  {{0x65,0x73,0x69,0x73,0x38,0x35,0x39,0x31, 0x11,0xbe,0x21,0x87,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "esis8591"
        // Latin1=190  CP1252=135  [top Latin1]
  {{0x65,0x74,0x65,0x65,0x38,0x35,0x39,0x31, 0x11,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "etee8591"
        // Latin1=191  [top Latin1]
  {{0x65,0x74,0x69,0x73,0x38,0x35,0x39,0x31, 0x11,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "etis8591"
        // Latin1=191  [top Latin1]
  {{0x65,0x75,0x63,0x5f,0x32,0x5f,0x5f,0x5f, 0x01,0xbe,0x31,0x72,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "euc_2___"
        // ASCII-7-bit=190  CP1252=114  [top ASCII-7-bit]
  {{0x65,0x75,0x63,0x5f,0x5f,0x5f,0x5f,0x5f, 0x01,0x7d,0x61,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "euc_____"
        // ASCII-7-bit=125  EUC-JP=190  [top EUC-JP]
  {{0x65,0x75,0x63,0x63,0x5f,0x5f,0x5f,0x5f, 0x01,0x6f,0x30,0x41,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "eucc____"
        // ASCII-7-bit=111  EUC-CN=190  [top EUC-CN]
  {{0x65,0x75,0x63,0x64,0x5f,0x5f,0x5f,0x5f, 0x30,0x61,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "eucd____"
        // EUC=191  [top EUC]
  {{0x65,0x75,0x63,0x6a,0x5f,0x5f,0x5f,0x5f, 0x01,0x68,0x61,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "eucj____"
        // ASCII-7-bit=104  EUC-JP=190  [top EUC-JP]
  {{0x65,0x75,0x63,0x6b,0x5f,0x5f,0x5f,0x5f, 0x01,0x6d,0x41,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "euck____"
        // ASCII-7-bit=109  KSC=190  [top KSC]
  {{0x65,0x75,0x63,0x75,0x5f,0x5f,0x5f,0x5f, 0x01,0x6d,0x41,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "eucu____"
        // ASCII-7-bit=109  KSC=190  [top KSC]
  {{0x65,0x75,0x6b,0x6b,0x5f,0x5f,0x5f,0x5f, 0x51,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "eukk____"
        // KSC=191  [top KSC]
  {{0x65,0x75,0x72,0x6b,0x5f,0x5f,0x5f,0x5f, 0x01,0x71,0x41,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "eurk____"
        // ASCII-7-bit=113  KSC=190  [top KSC]
  {{0x66,0x65,0x61,0x74,0x5f,0x5f,0x5f,0x5f, 0x41,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "feat____"
        // CP1252=191  [top CP1252]
  {{0x66,0x66,0x5f,0x5f,0x30,0x5f,0x5f,0x5f, 0x02,0x9e,0xba,0x21,0xa5,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "ff__0___"
        // ASCII-7-bit=158  Latin1=186  CP1252=165  [top Latin1]
  {{0x66,0x69,0x66,0x69,0x38,0x35,0x39,0x31, 0x11,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "fifi8591"
        // Latin1=191  [top Latin1]
  {{0x66,0x72,0x66,0x72,0x38,0x35,0x39,0x31, 0x02,0x79,0xbc,0x21,0xa3,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "frfr8591"
        // ASCII-7-bit=121  Latin1=188  CP1252=163  [top Latin1]
  {{0x66,0x72,0x66,0x72,0x38,0x5f,0x5f,0x5f, 0x02,0xa6,0xad,0x21,0xb5,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "frfr8___"
        // ASCII-7-bit=166  Latin1=173  CP1252=181  [top CP1252]
  {{0x66,0x72,0x69,0x73,0x38,0x35,0x39,0x31, 0x02,0x80,0xbd,0x21,0x9e,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "fris8591"
        // ASCII-7-bit=128  Latin1=189  CP1252=158  [top Latin1]
  {{0x66,0x72,0x75,0x74,0x38,0x5f,0x5f,0x5f, 0x02,0x8c,0xb3,0x21,0xb5,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "frut8___"
        // ASCII-7-bit=140  Latin1=179  CP1252=181  [top CP1252]
  {{0x67,0x62,0x5f,0x5f,0x31,0x32,0x35,0x31, 0x01,0x6f,0x91,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "gb__1251"
        // ASCII-7-bit=111  CP1251=190  [top CP1251]
  {{0x67,0x62,0x5f,0x5f,0x32,0x31,0x33,0x32, 0x01,0x91,0x21,0xbe,0xf1,0x70,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "gb__2132"
        // ASCII-7-bit=145  GB=190  GBK=112  [top GB]
  {{0x67,0x62,0x5f,0x5f,0x32,0x33,0x31,0x32, 0x01,0x7a,0x21,0xbe,0xf1,0x5c,0xc1,0x37,0x00,0x00,0x00,0x00,}}, // "gb__2312"
        // ASCII-7-bit=122  GB=190  GBK=92  GB18030=55  [top GB]
  {{0x67,0x62,0x5f,0x5f,0x32,0x33,0x32,0x31, 0x01,0x7d,0x21,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "gb__2321"
        // ASCII-7-bit=125  GB=190  [top GB]
  {{0x67,0x62,0x5f,0x5f,0x33,0x32,0x31,0x32, 0x01,0x92,0x21,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "gb__3212"
        // ASCII-7-bit=146  GB=190  [top GB]
  {{0x67,0x62,0x5f,0x5f,0x38,0x30,0x33,0x30, 0x01,0x73,0x21,0xaf,0xf1,0x59,0xc1,0xb9,0x00,0x00,0x00,0x00,}}, // "gb__8030"
        // ASCII-7-bit=115  GB=175  GBK=89  GB18030=185  [top GB18030]
  {{0x67,0x62,0x5f,0x5f,0x38,0x35,0x39,0x31, 0x02,0x7f,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "gb__8591"
        // ASCII-7-bit=127  Latin1=190  [top Latin1]
  {{0x67,0x62,0x5f,0x5f,0x5f,0x5f,0x5f,0x5f, 0x01,0x71,0x21,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "gb______"
        // ASCII-7-bit=113  GB=190  [top GB]
  {{0x67,0x62,0x6b,0x5f,0x5f,0x5f,0x5f,0x5f, 0x01,0x76,0x21,0xaf,0xf1,0xb9,0xc1,0x13,0x00,0x00,0x00,0x00,}}, // "gbk_____"
        // ASCII-7-bit=118  GB=175  GBK=185  GB18030=19  [top GBK]
  {{0x67,0x64,0x5f,0x5f,0x32,0x33,0x31,0x32, 0x01,0x56,0x21,0xbe,0xf1,0x72,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "gd__2312"
        // ASCII-7-bit=86  GB=190  GBK=114  [top GB]
  {{0x67,0x65,0x69,0x73,0x38,0x35,0x39,0x31, 0x02,0x79,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "geis8591"
        // ASCII-7-bit=121  Latin1=190  [top Latin1]
  {{0x67,0x65,0x6e,0x65,0x31,0x32,0x35,0x31, 0xa1,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "gene1251"
        // CP1251=191  [top CP1251]
  {{0x67,0x69,0x73,0x6f,0x38,0x35,0x39,0x31, 0x11,0xbe,0x21,0x6f,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "giso8591"
        // Latin1=190  CP1252=111  [top Latin1]
  {{0x67,0x72,0x65,0x65,0x5f,0x5f,0x5f,0x5f, 0x01,0x90,0x10,0x31,0xbe,0x21,0x86,0x00,0x00,0x00,0x00,0x00,}}, // "gree____"
        // ASCII-7-bit=144  Greek=190  CP1253=134  [top Greek]
  {{0x68,0x72,0x77,0x69,0x31,0x32,0x35,0x30, 0xc1,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "hrwi1250"
        // CP1250=191  [top CP1250]
  {{0x68,0x74,0x63,0x68,0x5f,0x5f,0x5f,0x5f, 0x30,0x11,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "htch____"
        // HTCHANAKYA=191  [top HTCHANAKYA]
  {{0x68,0x74,0x6d,0x6c,0x5f,0x5f,0x5f,0x5f, 0x01,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "html____"
        // ASCII-7-bit=191  [top ASCII-7-bit]
  {{0x68,0x74,0x74,0x70,0x5f,0x5f,0x5f,0x5f, 0x02,0xbb,0xa4,0x21,0x8d,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "http____"
        // ASCII-7-bit=187  Latin1=164  CP1252=141  [top ASCII-7-bit]
  {{0x68,0x7a,0x67,0x62,0x32,0x33,0x31,0x32, 0x01,0x85,0x20,0x71,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "hzgb2312"
        // ASCII-7-bit=133  HZ-GB-2312=190  [top HZ-GB-2312]
  {{0x69,0x5f,0x5f,0x5f,0x38,0x35,0x39,0x31, 0x02,0x79,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "i___8591"
        // ASCII-7-bit=121  Latin1=190  [top Latin1]
  {{0x69,0x61,0x6e,0x6f,0x35,0x5f,0x5f,0x5f, 0x02,0xbe,0x61,0x21,0x54,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "iano5___"
        // ASCII-7-bit=190  Latin1=97  CP1252=84  [top ASCII-7-bit]
  {{0x69,0x62,0x6d,0x5f,0x38,0x35,0x32,0x5f, 0x01,0xac,0x20,0x01,0xba,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "ibm_852_"
        // ASCII-7-bit=172  CP852=186  [top CP852]
  {{0x69,0x62,0x6d,0x5f,0x38,0x36,0x36,0x5f, 0x01,0x84,0x20,0x31,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "ibm_866_"
        // ASCII-7-bit=132  CP866=190  [top CP866]
  {{0x69,0x62,0x6d,0x5f,0x39,0x34,0x32,0x5f, 0x61,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "ibm_942_"
        // SJS=191  [top SJS]
  {{0x69,0x63,0x6f,0x5f,0x38,0x35,0x39,0x31, 0x02,0x79,0xbb,0x21,0xa9,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "ico_8591"
        // ASCII-7-bit=121  Latin1=187  CP1252=169  [top Latin1]
  {{0x69,0x6e,0x64,0x6f,0x31,0x32,0x35,0x31, 0xa1,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "indo1251"
        // CP1251=191  [top CP1251]
  {{0x69,0x6e,0x73,0x6f,0x38,0x35,0x39,0x31, 0x02,0x6f,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "inso8591"
        // ASCII-7-bit=111  Latin1=190  [top Latin1]
  {{0x69,0x6f,0x73,0x5f,0x38,0x35,0x39,0x31, 0x02,0x97,0xbd,0x21,0x6e,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "ios_8591"
        // ASCII-7-bit=151  Latin1=189  CP1252=110  [top Latin1]
  {{0x69,0x6f,0x73,0x6f,0x38,0x35,0x39,0x31, 0x11,0xbe,0x21,0x79,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "ioso8591"
        // Latin1=190  CP1252=121  [top Latin1]
  {{0x69,0x73,0x5f,0x5f,0x35,0x39,0x31,0x35, 0x11,0x7f,0xd1,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "is__5915"
        // Latin1=127  ISO-8859-15=190  [top ISO-8859-15]
  {{0x69,0x73,0x5f,0x5f,0x38,0x35,0x39,0x31, 0x02,0xad,0xb7,0x21,0x9f,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "is__8591"
        // ASCII-7-bit=173  Latin1=183  CP1252=159  [top Latin1]
  {{0x69,0x73,0x5f,0x5f,0x38,0x35,0x39,0x32, 0x01,0x78,0x81,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "is__8592"
        // ASCII-7-bit=120  Latin2=190  [top Latin2]
  {{0x69,0x73,0x5f,0x5f,0x38,0x35,0x39,0x37, 0x10,0x41,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "is__8597"
        // Greek=191  [top Greek]
  {{0x69,0x73,0x5f,0x5f,0x38,0x35,0x39,0x38, 0x01,0x6f,0x10,0x81,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "is__8598"
        // ASCII-7-bit=111  Hebrew=190  [top Hebrew]
  {{0x69,0x73,0x5f,0x5f,0x38,0x35,0x39,0x39, 0xd1,0xbe,0x81,0x88,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "is__8599"
        // Latin5=190  CP1254=136  [top Latin5]
  {{0x69,0x73,0x61,0x5f,0x35,0x39,0x31,0x35, 0x02,0x86,0x89,0xd1,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "isa_5915"
        // ASCII-7-bit=134  Latin1=137  ISO-8859-15=190  [top ISO-8859-15]
  {{0x69,0x73,0x64,0x5f,0x38,0x35,0x39,0x31, 0x02,0x79,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "isd_8591"
        // ASCII-7-bit=121  Latin1=190  [top Latin1]
  {{0x69,0x73,0x64,0x6f,0x38,0x35,0x39,0x31, 0x02,0x6f,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "isdo8591"
        // ASCII-7-bit=111  Latin1=190  [top Latin1]
  {{0x69,0x73,0x6e,0x5f,0x38,0x35,0x39,0x31, 0x02,0x6f,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "isn_8591"
        // ASCII-7-bit=111  Latin1=190  [top Latin1]
  {{0x69,0x73,0x6f,0x5f,0x30,0x36,0x34,0x36, 0x02,0xb8,0xaa,0x21,0xa3,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "iso_0646"
        // ASCII-7-bit=184  Latin1=170  CP1252=163  [top ASCII-7-bit]
  {{0x69,0x73,0x6f,0x5f,0x31,0x30,0x34,0x30, 0x02,0x98,0xb2,0x21,0xb4,0xa1,0x5e,0x00,0x00,0x00,0x00,0x00,}}, // "iso_1040"
        // ASCII-7-bit=152  Latin1=178  CP1252=180  ISO-8859-15=94  [top CP1252]
  {{0x69,0x73,0x6f,0x5f,0x31,0x32,0x35,0x30, 0x01,0x90,0xb1,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "iso_1250"
        // ASCII-7-bit=144  CP1250=190  [top CP1250]
  {{0x69,0x73,0x6f,0x5f,0x31,0x32,0x35,0x31, 0x01,0x78,0x91,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "iso_1251"
        // ASCII-7-bit=120  CP1251=190  [top CP1251]
  {{0x69,0x73,0x6f,0x5f,0x31,0x32,0x35,0x32, 0x02,0xad,0x9e,0x21,0xb7,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "iso_1252"
        // ASCII-7-bit=173  Latin1=158  CP1252=183  [top CP1252]
  {{0x69,0x73,0x6f,0x5f,0x31,0x32,0x35,0x33, 0x10,0x41,0x83,0x21,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "iso_1253"
        // Greek=131  CP1253=190  [top CP1253]
  {{0x69,0x73,0x6f,0x5f,0x31,0x32,0x35,0x34, 0xd1,0x9b,0x81,0xbd,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "iso_1254"
        // Latin5=155  CP1254=189  [top CP1254]
  {{0x69,0x73,0x6f,0x5f,0x31,0x32,0x35,0x35, 0x01,0x79,0x10,0x01,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "iso_1255"
        // ASCII-7-bit=121  CP1255=190  [top CP1255]
  {{0x69,0x73,0x6f,0x5f,0x31,0x32,0x35,0x36, 0x01,0x6f,0xa1,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "iso_1256"
        // ASCII-7-bit=111  CP1256=190  [top CP1256]
  {{0x69,0x73,0x6f,0x5f,0x31,0x32,0x35,0x37, 0x01,0x7f,0xf1,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "iso_1257"
        // ASCII-7-bit=127  CP1257=190  [top CP1257]
  {{0x69,0x73,0x6f,0x5f,0x31,0x5f,0x5f,0x5f, 0x02,0x85,0xb5,0x21,0xb3,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "iso_1___"
        // ASCII-7-bit=133  Latin1=181  CP1252=179  [top Latin1]
  {{0x69,0x73,0x6f,0x5f,0x32,0x30,0x32,0x32, 0x01,0x97,0x51,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "iso_2022"
        // ASCII-7-bit=151  SJS=190  [top SJS]
  {{0x69,0x73,0x6f,0x5f,0x35,0x35,0x39,0x31, 0x02,0xa9,0xb8,0x21,0xa4,0xa1,0x3c,0x00,0x00,0x00,0x00,0x00,}}, // "iso_5591"
        // ASCII-7-bit=169  Latin1=184  CP1252=164  ISO-8859-15=60  [top Latin1]
  {{0x69,0x73,0x6f,0x5f,0x35,0x35,0x39,0x32, 0x02,0x9a,0xbd,0x21,0x92,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "iso_5592"
        // ASCII-7-bit=154  Latin1=189  CP1252=146  [top Latin1]
  {{0x69,0x73,0x6f,0x5f,0x35,0x38,0x39,0x31, 0x02,0xa1,0xbc,0x21,0x8b,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "iso_5891"
        // ASCII-7-bit=161  Latin1=188  CP1252=139  [top Latin1]
  {{0x69,0x73,0x6f,0x5f,0x35,0x39,0x31,0x30, 0x01,0xaa,0x20,0xa1,0xba,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "iso_5910"
        // ASCII-7-bit=170  Latin6=186  [top Latin6]
  {{0x69,0x73,0x6f,0x5f,0x35,0x39,0x31,0x31, 0x01,0x86,0xd1,0xbe,0xd1,0x66,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "iso_5911"
        // ASCII-7-bit=134  ISO-8859-11=190  CP874=102  [top ISO-8859-11]
  {{0x69,0x73,0x6f,0x5f,0x35,0x39,0x31,0x33, 0x01,0x9c,0xf1,0xa1,0xc1,0xbb,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "iso_5913"
        // ASCII-7-bit=156  CP1257=161  ISO-8859-13=187  [top ISO-8859-13]
  {{0x69,0x73,0x6f,0x5f,0x35,0x39,0x31,0x34, 0x02,0x93,0xbd,0x21,0x95,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "iso_5914"
        // ASCII-7-bit=147  Latin1=189  CP1252=149  [top Latin1]
  {{0x69,0x73,0x6f,0x5f,0x35,0x39,0x31,0x35, 0x02,0x98,0xad,0x21,0x81,0xa1,0xb7,0x00,0x00,0x00,0x00,0x00,}}, // "iso_5915"
        // ASCII-7-bit=152  Latin1=173  CP1252=129  ISO-8859-15=183  [top ISO-8859-15]
  {{0x69,0x73,0x6f,0x5f,0x35,0x39,0x31,0x36, 0x01,0xae,0xb1,0xb9,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "iso_5916"
        // ASCII-7-bit=174  CP1250=185  [top CP1250]
  {{0x69,0x73,0x6f,0x5f,0x35,0x39,0x32,0x32, 0x01,0xa7,0x81,0xbb,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "iso_5922"
        // ASCII-7-bit=167  Latin2=187  [top Latin2]
  {{0x69,0x73,0x6f,0x5f,0x36,0x33,0x39,0x32, 0x02,0x7e,0xbe,0x21,0x82,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "iso_6392"
        // ASCII-7-bit=126  Latin1=190  CP1252=130  [top Latin1]
  {{0x69,0x73,0x6f,0x5f,0x36,0x33,0x39,0x5f, 0x01,0xa6,0xa1,0xbc,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "iso_639_"
        // ASCII-7-bit=166  CP1256=188  [top CP1256]
  {{0x69,0x73,0x6f,0x5f,0x36,0x34,0x36,0x31, 0x02,0x7d,0xbe,0x21,0x68,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "iso_6461"
        // ASCII-7-bit=125  Latin1=190  CP1252=104  [top Latin1]
  {{0x69,0x73,0x6f,0x5f,0x38,0x35,0x31,0x31, 0x02,0xb0,0xb7,0x21,0x92,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "iso_8511"
        // ASCII-7-bit=176  Latin1=183  CP1252=146  [top Latin1]
  {{0x69,0x73,0x6f,0x5f,0x38,0x35,0x36,0x31, 0x02,0x9f,0xba,0x21,0xa5,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "iso_8561"
        // ASCII-7-bit=159  Latin1=186  CP1252=165  [top Latin1]
  {{0x69,0x73,0x6f,0x5f,0x38,0x35,0x38,0x31, 0x01,0x8d,0x51,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "iso_8581"
        // ASCII-7-bit=141  SJS=190  [top SJS]
  {{0x69,0x73,0x6f,0x5f,0x38,0x35,0x39,0x30, 0x02,0x99,0xbc,0x21,0x9c,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "iso_8590"
        // ASCII-7-bit=153  Latin1=188  CP1252=156  [top Latin1]
  {{0x69,0x73,0x6f,0x5f,0x38,0x35,0x39,0x31, 0x02,0xae,0xb8,0x21,0x99,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "iso_8591"
        // ASCII-7-bit=174  Latin1=184  CP1252=153  [top Latin1]
  {{0x69,0x73,0x6f,0x5f,0x38,0x35,0x39,0x32, 0x01,0x95,0x81,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "iso_8592"
        // ASCII-7-bit=149  Latin2=190  [top Latin2]
  {{0x69,0x73,0x6f,0x5f,0x38,0x35,0x39,0x33, 0x01,0x9f,0x20,0x51,0xbd,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "iso_8593"
        // ASCII-7-bit=159  Latin3=189  [top Latin3]
  {{0x69,0x73,0x6f,0x5f,0x38,0x35,0x39,0x34, 0x01,0xac,0x10,0xd1,0xba,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "iso_8594"
        // ASCII-7-bit=172  Latin4=186  [top Latin4]
  {{0x69,0x73,0x6f,0x5f,0x38,0x35,0x39,0x35, 0x01,0xa6,0x10,0xa1,0xbc,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "iso_8595"
        // ASCII-7-bit=166  ISO-8859-5=188  [top ISO-8859-5]
  {{0x69,0x73,0x6f,0x5f,0x38,0x35,0x39,0x36, 0x01,0xae,0x20,0x11,0xb9,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "iso_8596"
        // ASCII-7-bit=174  Arabic=185  [top Arabic]
  {{0x69,0x73,0x6f,0x5f,0x38,0x35,0x39,0x37, 0x01,0x96,0x10,0x31,0xbd,0x21,0x8f,0x00,0x00,0x00,0x00,0x00,}}, // "iso_8597"
        // ASCII-7-bit=150  Greek=189  CP1253=143  [top Greek]
  {{0x69,0x73,0x6f,0x5f,0x38,0x35,0x39,0x38, 0x01,0x9b,0x10,0x81,0xbd,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "iso_8598"
        // ASCII-7-bit=155  Hebrew=189  [top Hebrew]
  {{0x69,0x73,0x6f,0x5f,0x38,0x35,0x39,0x39, 0x01,0x7a,0xc1,0xbe,0x81,0x7e,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "iso_8599"
        // ASCII-7-bit=122  Latin5=190  CP1254=126  [top Latin5]
  {{0x69,0x73,0x6f,0x5f,0x38,0x36,0x30,0x31, 0x02,0xba,0x94,0x21,0xa6,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "iso_8601"
        // ASCII-7-bit=186  Latin1=148  CP1252=166  [top ASCII-7-bit]
  {{0x69,0x73,0x6f,0x5f,0x38,0x36,0x39,0x31, 0x02,0xad,0xb9,0x21,0x83,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "iso_8691"
        // ASCII-7-bit=173  Latin1=185  CP1252=131  [top Latin1]
  {{0x69,0x73,0x6f,0x5f,0x38,0x36,0x39,0x32, 0x91,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "iso_8692"
        // Latin2=191  [top Latin2]
  {{0x69,0x73,0x6f,0x5f,0x38,0x38,0x35,0x31, 0x02,0xac,0xb7,0x21,0x9f,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "iso_8851"
        // ASCII-7-bit=172  Latin1=183  CP1252=159  [top Latin1]
  {{0x69,0x73,0x6f,0x5f,0x38,0x38,0x35,0x39, 0x02,0xaa,0xba,0x21,0x80,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "iso_8859"
        // ASCII-7-bit=170  Latin1=186  CP1252=128  [top Latin1]
  {{0x69,0x73,0x6f,0x5f,0x38,0x38,0x39,0x39, 0xd1,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "iso_8899"
        // Latin5=191  [top Latin5]
  {{0x69,0x73,0x6f,0x5f,0x38,0x39,0x31,0x31, 0x02,0x8c,0xbd,0x21,0x9a,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "iso_8911"
        // ASCII-7-bit=140  Latin1=189  CP1252=154  [top Latin1]
  {{0x69,0x73,0x6f,0x5f,0x38,0x39,0x31,0x5f, 0x01,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "iso_891_"
        // ASCII-7-bit=191  [top ASCII-7-bit]
  {{0x69,0x73,0x6f,0x5f,0x38,0x39,0x35,0x31, 0x02,0xa3,0xbc,0x21,0x91,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "iso_8951"
        // ASCII-7-bit=163  Latin1=188  CP1252=145  [top Latin1]
  {{0x69,0x73,0x6f,0x5f,0x39,0x30,0x30,0x31, 0x02,0x75,0xa7,0x21,0x85,0xa1,0xbb,0x00,0x00,0x00,0x00,0x00,}}, // "iso_9001"
        // ASCII-7-bit=117  Latin1=167  CP1252=133  ISO-8859-15=187  [top ISO-8859-15]
  {{0x69,0x73,0x6f,0x5f,0x39,0x35,0x35,0x31, 0x01,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "iso_9551"
        // ASCII-7-bit=191  [top ASCII-7-bit]
  {{0x69,0x73,0x6f,0x5f,0x39,0x35,0x39,0x31, 0x02,0x72,0xbe,0x21,0x7b,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "iso_9591"
        // ASCII-7-bit=114  Latin1=190  CP1252=123  [top Latin1]
  {{0x69,0x73,0x6f,0x5f,0x39,0x35,0x39,0x32, 0x01,0x7f,0x81,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "iso_9592"
        // ASCII-7-bit=127  Latin2=190  [top Latin2]
  {{0x69,0x73,0x6f,0x5f,0x39,0x35,0x39,0x39, 0x01,0x84,0xc1,0xbb,0x81,0xa6,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "iso_9599"
        // ASCII-7-bit=132  Latin5=187  CP1254=166  [top Latin5]
  {{0x69,0x73,0x6f,0x5f,0x5f,0x5f,0x5f,0x5f, 0x02,0x99,0xbc,0x21,0x96,0xa1,0x2e,0x00,0x00,0x00,0x00,0x00,}}, // "iso_____"
        // ASCII-7-bit=153  Latin1=188  CP1252=150  ISO-8859-15=46  [top Latin1]
  {{0x69,0x73,0x6f,0x61,0x38,0x35,0x39,0x31, 0x02,0x9a,0xbd,0x21,0x8a,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "isoa8591"
        // ASCII-7-bit=154  Latin1=189  CP1252=138  [top Latin1]
  {{0x69,0x73,0x6f,0x62,0x38,0x35,0x39,0x31, 0x02,0x86,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "isob8591"
        // ASCII-7-bit=134  Latin1=190  [top Latin1]
  {{0x69,0x73,0x6f,0x63,0x32,0x30,0x32,0x32, 0x20,0xd1,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "isoc2022"
        // ISO_2022_CN=191  [top ISO_2022_CN]
  {{0x69,0x73,0x6f,0x63,0x38,0x35,0x39,0x31, 0x02,0x79,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "isoc8591"
        // ASCII-7-bit=121  Latin1=190  [top Latin1]
  {{0x69,0x73,0x6f,0x63,0x38,0x35,0x39,0x32, 0x91,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "isoc8592"
        // Latin2=191  [top Latin2]
  {{0x69,0x73,0x6f,0x64,0x38,0x35,0x39,0x31, 0x02,0x6f,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "isod8591"
        // ASCII-7-bit=111  Latin1=190  [top Latin1]
  {{0x69,0x73,0x6f,0x65,0x38,0x35,0x39,0x31, 0x02,0x93,0xbd,0x21,0x8b,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "isoe8591"
        // ASCII-7-bit=147  Latin1=189  CP1252=139  [top Latin1]
  {{0x69,0x73,0x6f,0x66,0x35,0x39,0x31,0x35, 0x11,0x6f,0xd1,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "isof5915"
        // Latin1=111  ISO-8859-15=190  [top ISO-8859-15]
  {{0x69,0x73,0x6f,0x68,0x38,0x35,0x39,0x31, 0x02,0x79,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "isoh8591"
        // ASCII-7-bit=121  Latin1=190  [top Latin1]
  {{0x69,0x73,0x6f,0x69,0x36,0x5f,0x5f,0x5f, 0x02,0xbe,0x92,0x21,0x7f,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "isoi6___"
        // ASCII-7-bit=190  Latin1=146  CP1252=127  [top ASCII-7-bit]
  {{0x69,0x73,0x6f,0x69,0x38,0x35,0x39,0x31, 0x02,0xa2,0xbc,0x21,0x8c,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "isoi8591"
        // ASCII-7-bit=162  Latin1=188  CP1252=140  [top Latin1]
  {{0x69,0x73,0x6f,0x69,0x38,0x35,0x39,0x32, 0x91,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "isoi8592"
        // Latin2=191  [top Latin2]
  {{0x69,0x73,0x6f,0x69,0x38,0x35,0x39,0x35, 0x01,0xa4,0x10,0xa1,0xbc,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "isoi8595"
        // ASCII-7-bit=164  ISO-8859-5=188  [top ISO-8859-5]
  {{0x69,0x73,0x6f,0x69,0x38,0x35,0x39,0x36, 0x01,0x79,0x20,0x11,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "isoi8596"
        // ASCII-7-bit=121  Arabic=190  [top Arabic]
  {{0x69,0x73,0x6f,0x69,0x38,0x35,0x39,0x38, 0x01,0x83,0x10,0x01,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "isoi8598"
        // ASCII-7-bit=131  CP1255=190  [top CP1255]
  {{0x69,0x73,0x6f,0x69,0x38,0x35,0x39,0x39, 0xd1,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "isoi8599"
        // Latin5=191  [top Latin5]
  {{0x69,0x73,0x6f,0x69,0x38,0x38,0x35,0x39, 0x02,0xae,0xb7,0x21,0x9a,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "isoi8859"
        // ASCII-7-bit=174  Latin1=183  CP1252=154  [top Latin1]
  {{0x69,0x73,0x6f,0x69,0x38,0x39,0x5f,0x5f, 0xb1,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "isoi89__"
        // CP1256=191  [top CP1256]
  {{0x69,0x73,0x6f,0x6a,0x32,0x30,0x30,0x32, 0x71,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "isoj2002"
        // EUC-JP=191  [top EUC-JP]
  {{0x69,0x73,0x6f,0x6a,0x32,0x30,0x32,0x32, 0x01,0x44,0x10,0x41,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "isoj2022"
        // ASCII-7-bit=68  JIS=190  [top JIS]
  {{0x69,0x73,0x6f,0x6a,0x38,0x35,0x39,0x31, 0x02,0x79,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "isoj8591"
        // ASCII-7-bit=121  Latin1=190  [top Latin1]
  {{0x69,0x73,0x6f,0x6b,0x32,0x30,0x30,0x32, 0x01,0x7a,0x41,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "isok2002"
        // ASCII-7-bit=122  KSC=190  [top KSC]
  {{0x69,0x73,0x6f,0x6b,0x32,0x30,0x32,0x32, 0x20,0xa1,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "isok2022"
        // ISO-2022-KR=191  [top ISO-2022-KR]
  {{0x69,0x73,0x6f,0x6c,0x31,0x5f,0x5f,0x5f, 0x01,0xa6,0x11,0xbc,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "isol1___"
        // ASCII-7-bit=166  UTF8=188  [top UTF8]
  {{0x69,0x73,0x6f,0x6c,0x35,0x39,0x31,0x31, 0x01,0x83,0xd1,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "isol5911"
        // ASCII-7-bit=131  ISO-8859-11=190  [top ISO-8859-11]
  {{0x69,0x73,0x6f,0x6c,0x37,0x5f,0x5f,0x5f, 0x02,0xa4,0xb8,0x21,0xa7,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "isol7___"
        // ASCII-7-bit=164  Latin1=184  CP1252=167  [top Latin1]
  {{0x69,0x73,0x6f,0x6c,0x38,0x35,0x39,0x31, 0x02,0x79,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "isol8591"
        // ASCII-7-bit=121  Latin1=190  [top Latin1]
  {{0x69,0x73,0x6f,0x6d,0x38,0x35,0x39,0x31, 0x02,0x6f,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "isom8591"
        // ASCII-7-bit=111  Latin1=190  [top Latin1]
  {{0x69,0x73,0x6f,0x6e,0x38,0x35,0x39,0x31, 0x02,0x79,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "ison8591"
        // ASCII-7-bit=121  Latin1=190  [top Latin1]
  {{0x69,0x73,0x6f,0x6f,0x38,0x35,0x39,0x31, 0x02,0x6e,0xbe,0x21,0x6e,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "isoo8591"
        // ASCII-7-bit=110  Latin1=190  CP1252=110  [top Latin1]
  {{0x69,0x73,0x6f,0x70,0x35,0x39,0x31,0x35, 0xf1,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "isop5915"
        // ISO-8859-15=191  [top ISO-8859-15]
  {{0x69,0x73,0x6f,0x70,0x38,0x35,0x39,0x31, 0x02,0x91,0xbe,0x21,0x6f,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "isop8591"
        // ASCII-7-bit=145  Latin1=190  CP1252=111  [top Latin1]
  {{0x69,0x73,0x6f,0x73,0x38,0x35,0x39,0x31, 0x02,0x84,0xbe,0x21,0x8d,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "isos8591"
        // ASCII-7-bit=132  Latin1=190  CP1252=141  [top Latin1]
  {{0x69,0x73,0x6f,0x75,0x36,0x34,0x36,0x31, 0x02,0xa6,0xb9,0x21,0xa1,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "isou6461"
        // ASCII-7-bit=166  Latin1=185  CP1252=161  [top Latin1]
  {{0x69,0x73,0x6f,0x75,0x36,0x34,0x36,0x5f, 0x01,0xbe,0x31,0x8e,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "isou646_"
        // ASCII-7-bit=190  CP1252=142  [top ASCII-7-bit]
  {{0x69,0x73,0x6f,0x75,0x38,0x35,0x39,0x31, 0x02,0x6f,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "isou8591"
        // ASCII-7-bit=111  Latin1=190  [top Latin1]
  {{0x69,0x73,0x6f,0x75,0x38,0x5f,0x5f,0x5f, 0x02,0xa2,0xbc,0x21,0x8c,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "isou8___"
        // ASCII-7-bit=162  Latin1=188  CP1252=140  [top Latin1]
  {{0x69,0x73,0x6f,0x77,0x31,0x32,0x35,0x30, 0x01,0x6e,0xb1,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "isow1250"
        // ASCII-7-bit=110  CP1250=190  [top CP1250]
  {{0x69,0x73,0x6f,0x77,0x31,0x32,0x35,0x31, 0xa1,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "isow1251"
        // CP1251=191  [top CP1251]
  {{0x69,0x73,0x6f,0x77,0x31,0x32,0x35,0x33, 0x01,0x6f,0x10,0x61,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "isow1253"
        // ASCII-7-bit=111  CP1253=190  [top CP1253]
  {{0x69,0x73,0x6f,0x77,0x38,0x35,0x39,0x31, 0x02,0x89,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "isow8591"
        // ASCII-7-bit=137  Latin1=190  [top Latin1]
  {{0x69,0x73,0x6f,0x78,0x38,0x35,0x39,0x31, 0x02,0x6f,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "isox8591"
        // ASCII-7-bit=111  Latin1=190  [top Latin1]
  {{0x69,0x73,0x6f,0x7a,0x38,0x35,0x39,0x31, 0x02,0x8b,0xbe,0x21,0x79,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "isoz8591"
        // ASCII-7-bit=139  Latin1=190  CP1252=121  [top Latin1]
  {{0x69,0x73,0x70,0x5f,0x38,0x35,0x39,0x31, 0x02,0x86,0xbe,0x21,0x6f,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "isp_8591"
        // ASCII-7-bit=134  Latin1=190  CP1252=111  [top Latin1]
  {{0x69,0x73,0x73,0x5f,0x38,0x35,0x39,0x31, 0x02,0x79,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "iss_8591"
        // ASCII-7-bit=121  Latin1=190  [top Latin1]
  {{0x69,0x73,0x73,0x6f,0x38,0x35,0x39,0x31, 0x02,0x6f,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "isso8591"
        // ASCII-7-bit=111  Latin1=190  [top Latin1]
  {{0x69,0x73,0x74,0x5f,0x35,0x39,0x31,0x35, 0x01,0x79,0xe1,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "ist_5915"
        // ASCII-7-bit=121  ISO-8859-15=190  [top ISO-8859-15]
  {{0x69,0x73,0x74,0x6f,0x38,0x35,0x39,0x31, 0x11,0xbe,0x21,0x6f,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "isto8591"
        // Latin1=190  CP1252=111  [top Latin1]
  {{0x69,0x74,0x69,0x73,0x38,0x35,0x39,0x31, 0x02,0x6f,0xbe,0x21,0x86,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "itis8591"
        // ASCII-7-bit=111  Latin1=190  CP1252=134  [top Latin1]
  {{0x69,0x74,0x69,0x74,0x35,0x39,0x31,0x35, 0x41,0x79,0xa1,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "itit5915"
        // CP1252=121  ISO-8859-15=190  [top ISO-8859-15]
  {{0x69,0x74,0x69,0x74,0x38,0x35,0x39,0x31, 0x11,0xbe,0x21,0x8f,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "itit8591"
        // Latin1=190  CP1252=143  [top Latin1]
  {{0x69,0x74,0x69,0x74,0x5f,0x5f,0x5f,0x5f, 0x02,0xb7,0xab,0x21,0xa4,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "itit____"
        // ASCII-7-bit=183  Latin1=171  CP1252=164  [top ASCII-7-bit]
  {{0x69,0x75,0x5f,0x5f,0x38,0x35,0x39,0x31, 0x02,0x6f,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "iu__8591"
        // ASCII-7-bit=111  Latin1=190  [top Latin1]
  {{0x69,0x77,0x69,0x6e,0x31,0x32,0x35,0x30, 0xc1,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "iwin1250"
        // CP1250=191  [top CP1250]
  {{0x69,0x77,0x69,0x6e,0x31,0x32,0x35,0x37, 0x10,0x01,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "iwin1257"
        // CP1257=191  [top CP1257]
  {{0x69,0x79,0x73,0x6f,0x38,0x35,0x39,0x31, 0x02,0x79,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "iyso8591"
        // ASCII-7-bit=121  Latin1=190  [top Latin1]
  {{0x6a,0x61,0x5f,0x5f,0x5f,0x5f,0x5f,0x5f, 0x01,0x78,0x51,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "ja______"
        // ASCII-7-bit=120  SJS=190  [top SJS]
  {{0x6a,0x61,0x67,0x72,0x5f,0x5f,0x5f,0x5f, 0x20,0xf1,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "jagr____"
        // JAGRAN=191  [top JAGRAN]
  {{0x6a,0x69,0x73,0x5f,0x5f,0x5f,0x5f,0x5f, 0x01,0x81,0x10,0x41,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "jis_____"
        // ASCII-7-bit=129  JIS=190  [top JIS]
  {{0x6b,0x61,0x6d,0x63,0x5f,0x5f,0x5f,0x5f, 0x20,0x11,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "kamc____"
        // CP852=191  [top CP852]
  {{0x6b,0x6f,0x5f,0x5f,0x5f,0x5f,0x5f,0x5f, 0x01,0x7c,0x41,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "ko______"
        // ASCII-7-bit=124  KSC=190  [top KSC]
  {{0x6b,0x6f,0x69,0x5f,0x37,0x5f,0x5f,0x5f, 0x01,0xbe,0x31,0x6b,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "koi_7___"
        // ASCII-7-bit=190  CP1252=107  [top ASCII-7-bit]
  {{0x6b,0x6f,0x69,0x72,0x38,0x5f,0x5f,0x5f, 0x01,0x8b,0x10,0x11,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "koir8___"
        // ASCII-7-bit=139  KOI8R=190  [top KOI8R]
  {{0x6b,0x6f,0x69,0x75,0x38,0x5f,0x5f,0x5f, 0x01,0x77,0x10,0x91,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "koiu8___"
        // ASCII-7-bit=119  KOI8U=190  [top KOI8U]
  {{0x6b,0x6f,0x6b,0x72,0x5f,0x5f,0x5f,0x5f, 0x01,0x4b,0x41,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "kokr____"
        // ASCII-7-bit=75  KSC=190  [top KSC]
  {{0x6b,0x6f,0x6b,0x73,0x35,0x36,0x30,0x31, 0x01,0x75,0x41,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "koks5601"
        // ASCII-7-bit=117  KSC=190  [top KSC]
  {{0x6b,0x6f,0x72,0x65,0x5f,0x5f,0x5f,0x5f, 0x01,0x4e,0x41,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "kore____"
        // ASCII-7-bit=78  KSC=190  [top KSC]
  {{0x6b,0x72,0x5f,0x5f,0x5f,0x5f,0x5f,0x5f, 0x01,0x74,0x41,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "kr______"
        // ASCII-7-bit=116  KSC=190  [top KSC]
  {{0x6b,0x72,0x63,0x5f,0x35,0x36,0x30,0x31, 0x01,0x74,0x41,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "krc_5601"
        // ASCII-7-bit=116  KSC=190  [top KSC]
  {{0x6b,0x73,0x63,0x5f,0x35,0x35,0x30,0x31, 0x51,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "ksc_5501"
        // KSC=191  [top KSC]
  {{0x6b,0x73,0x63,0x5f,0x35,0x36,0x30,0x31, 0x01,0x62,0x41,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "ksc_5601"
        // ASCII-7-bit=98  KSC=190  [top KSC]
  {{0x6b,0x73,0x63,0x5f,0x36,0x30,0x30,0x31, 0x51,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "ksc_6001"
        // KSC=191  [top KSC]
  {{0x6c,0x61,0x73,0x74,0x5f,0x5f,0x5f,0x5f, 0x02,0xb7,0xaf,0x21,0x90,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "last____"
        // ASCII-7-bit=183  Latin1=175  CP1252=144  [top ASCII-7-bit]
  {{0x6c,0x61,0x74,0x69,0x31,0x5f,0x5f,0x5f, 0x02,0xa3,0xbb,0x21,0x9b,0xa1,0x73,0x00,0x00,0x00,0x00,0x00,}}, // "lati1___"
        // ASCII-7-bit=163  Latin1=187  CP1252=155  ISO-8859-15=115  [top Latin1]
  {{0x6c,0x61,0x74,0x69,0x32,0x5f,0x5f,0x5f, 0x01,0x94,0x81,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "lati2___"
        // ASCII-7-bit=148  Latin2=190  [top Latin2]
  {{0x6c,0x61,0x74,0x69,0x35,0x5f,0x5f,0x5f, 0x01,0x7c,0xc1,0xbe,0x81,0x87,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "lati5___"
        // ASCII-7-bit=124  Latin5=190  CP1254=135  [top Latin5]
  {{0x6c,0x61,0x74,0x69,0x38,0x35,0x39,0x31, 0x11,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "lati8591"
        // Latin1=191  [top Latin1]
  {{0x6c,0x61,0x74,0x69,0x38,0x38,0x35,0x39, 0x91,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "lati8859"
        // Latin2=191  [top Latin2]
  {{0x6c,0x69,0x6e,0x75,0x31,0x32,0x35,0x32, 0x11,0x79,0x21,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "linu1252"
        // Latin1=121  CP1252=190  [top CP1252]
  {{0x6c,0x6f,0x67,0x69,0x5f,0x5f,0x5f,0x5f, 0x01,0x88,0x11,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "logi____"
        // ASCII-7-bit=136  UTF8=190  [top UTF8]
  {{0x6c,0x73,0x6f,0x5f,0x38,0x35,0x39,0x31, 0x02,0x6f,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "lso_8591"
        // ASCII-7-bit=111  Latin1=190  [top Latin1]
  {{0x6c,0x74,0x6f,0x5f,0x38,0x35,0x39,0x31, 0x02,0x6f,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "lto_8591"
        // ASCII-7-bit=111  Latin1=190  [top Latin1]
  {{0x6c,0x74,0x77,0x69,0x31,0x32,0x35,0x37, 0x10,0x01,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "ltwi1257"
        // CP1257=191  [top CP1257]
  {{0x6d,0x61,0x63,0x5f,0x5f,0x5f,0x5f,0x5f, 0x01,0x82,0x91,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "mac_____"
        // ASCII-7-bit=130  CP1251=190  [top CP1251]
  {{0x6d,0x61,0x63,0x63,0x5f,0x5f,0x5f,0x5f, 0x01,0x94,0x10,0xe1,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "macc____"
        // ASCII-7-bit=148  MACINTOSH=190  [top MACINTOSH]
  {{0x6d,0x61,0x63,0x69,0x5f,0x5f,0x5f,0x5f, 0x02,0xbd,0x93,0x21,0x8b,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "maci____"
        // ASCII-7-bit=189  Latin1=147  CP1252=139  [top ASCII-7-bit]
  {{0x6d,0x61,0x63,0x72,0x5f,0x5f,0x5f,0x5f, 0x01,0xaf,0x10,0xe1,0xb9,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "macr____"
        // ASCII-7-bit=175  MACINTOSH=185  [top MACINTOSH]
  {{0x6d,0x73,0x5f,0x5f,0x38,0x37,0x34,0x5f, 0x01,0x80,0xd1,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "ms__874_"
        // ASCII-7-bit=128  ISO-8859-11=190  [top ISO-8859-11]
  {{0x6d,0x73,0x5f,0x5f,0x39,0x33,0x32,0x5f, 0x01,0x91,0x51,0xbe,0x10,0x11,0x82,0x00,0x00,0x00,0x00,0x00,}}, // "ms__932_"
        // ASCII-7-bit=145  SJS=190  CP932=130  [top SJS]
  {{0x6d,0x73,0x5f,0x5f,0x39,0x34,0x39,0x5f, 0x01,0x49,0x41,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "ms__949_"
        // ASCII-7-bit=73  KSC=190  [top KSC]
  {{0x6d,0x73,0x5f,0x5f,0x39,0x35,0x30,0x5f, 0x01,0x75,0x71,0xbe,0x10,0xa1,0x43,0x00,0x00,0x00,0x00,0x00,}}, // "ms__950_"
        // ASCII-7-bit=117  BIG5=190  BIG5_HKSCS=67  [top BIG5]
  {{0x6d,0x73,0x63,0x70,0x31,0x32,0x35,0x30, 0xc1,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "mscp1250"
        // CP1250=191  [top CP1250]
  {{0x6d,0x73,0x68,0x6b,0x39,0x35,0x30,0x5f, 0x01,0x82,0x71,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "mshk950_"
        // ASCII-7-bit=130  BIG5=190  [top BIG5]
  {{0x6d,0x73,0x77,0x69,0x31,0x32,0x35,0x30, 0xc1,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "mswi1250"
        // CP1250=191  [top CP1250]
  {{0x6d,0x73,0x77,0x69,0x31,0x32,0x35,0x33, 0x10,0x41,0x8f,0x21,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "mswi1253"
        // Greek=143  CP1253=190  [top CP1253]
  {{0x6d,0x78,0x5f,0x5f,0x5f,0x5f,0x5f,0x5f, 0x21,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "mx______"
        // UTF8=191  [top UTF8]
  {{0x6e,0x65,0x77,0x5f,0x5f,0x5f,0x5f,0x5f, 0x02,0xab,0xb2,0x21,0xaf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "new_____"
        // ASCII-7-bit=171  Latin1=178  CP1252=175  [top Latin1]
  {{0x6e,0x66,0x7a,0x5f,0x32,0x30,0x31,0x30, 0x02,0x80,0xbc,0x21,0xa3,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "nfz_2010"
        // ASCII-7-bit=128  Latin1=188  CP1252=163  [top Latin1]
  {{0x6e,0x69,0x73,0x6f,0x38,0x35,0x39,0x31, 0x02,0x6f,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "niso8591"
        // ASCII-7-bit=111  Latin1=190  [top Latin1]
  {{0x6e,0x6c,0x61,0x69,0x38,0x35,0x39,0x31, 0x11,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "nlai8591"
        // Latin1=191  [top Latin1]
  {{0x6e,0x6c,0x6e,0x6c,0x38,0x35,0x39,0x31, 0x11,0xbe,0x21,0x6f,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "nlnl8591"
        // Latin1=190  CP1252=111  [top Latin1]
  {{0x6e,0x6f,0x5f,0x5f,0x5f,0x5f,0x5f,0x5f, 0x01,0xa4,0x71,0xbc,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "no______"
        // ASCII-7-bit=164  BIG5=188  [top BIG5]
  {{0x6e,0x6f,0x69,0x73,0x38,0x35,0x39,0x31, 0x11,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "nois8591"
        // Latin1=191  [top Latin1]
  {{0x6e,0x6f,0x6e,0x65,0x5f,0x5f,0x5f,0x5f, 0x01,0x9b,0x51,0xbd,0x10,0x11,0x70,0x00,0x00,0x00,0x00,0x00,}}, // "none____"
        // ASCII-7-bit=155  SJS=189  CP932=112  [top SJS]
  {{0x6e,0x75,0x6c,0x6c,0x5f,0x5f,0x5f,0x5f, 0x01,0x92,0x71,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "null____"
        // ASCII-7-bit=146  BIG5=190  [top BIG5]
  {{0x6f,0x5f,0x5f,0x5f,0x38,0x35,0x39,0x31, 0x02,0x6f,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "o___8591"
        // ASCII-7-bit=111  Latin1=190  [top Latin1]
  {{0x6f,0x6e,0x5f,0x5f,0x5f,0x5f,0x5f,0x5f, 0x21,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "on______"
        // UTF8=191  [top UTF8]
  {{0x6f,0x73,0x69,0x5f,0x35,0x39,0x31,0x35, 0x01,0x6f,0xe1,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "osi_5915"
        // ASCII-7-bit=111  ISO-8859-15=190  [top ISO-8859-15]
  {{0x6f,0x73,0x6f,0x5f,0x38,0x35,0x39,0x31, 0x02,0x6f,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "oso_8591"
        // ASCII-7-bit=111  Latin1=190  [top Latin1]
  {{0x6f,0x73,0x70,0x5f,0x38,0x35,0x39,0x38, 0x01,0x6f,0x10,0x81,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "osp_8598"
        // ASCII-7-bit=111  Hebrew=190  [top Hebrew]
  {{0x6f,0x77,0x69,0x6e,0x31,0x32,0x35,0x36, 0xb1,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "owin1256"
        // CP1256=191  [top CP1256]
  {{0x70,0x61,0x72,0x5f,0x5f,0x5f,0x5f,0x5f, 0x02,0x6e,0xb8,0x21,0xaf,0xa1,0x64,0x00,0x00,0x00,0x00,0x00,}}, // "par_____"
        // ASCII-7-bit=110  Latin1=184  CP1252=175  ISO-8859-15=100  [top Latin1]
  {{0x70,0x63,0x5f,0x5f,0x5f,0x5f,0x5f,0x5f, 0x21,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "pc______"
        // UTF8=191  [top UTF8]
  {{0x70,0x6c,0x69,0x73,0x38,0x35,0x39,0x32, 0x91,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "plis8592"
        // Latin2=191  [top Latin2]
  {{0x70,0x6c,0x70,0x6c,0x38,0x35,0x39,0x32, 0x91,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "plpl8592"
        // Latin2=191  [top Latin2]
  {{0x70,0x72,0x65,0x64,0x5f,0x5f,0x5f,0x5f, 0x02,0xb4,0xa3,0x21,0xb1,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "pred____"
        // ASCII-7-bit=180  Latin1=163  CP1252=177  [top ASCII-7-bit]
  {{0x70,0x74,0x62,0x72,0x38,0x35,0x39,0x31, 0x02,0x6e,0xbd,0x21,0x9a,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "ptbr8591"
        // ASCII-7-bit=110  Latin1=189  CP1252=154  [top Latin1]
  {{0x70,0x74,0x62,0x72,0x5f,0x5f,0x5f,0x5f, 0x01,0x79,0x11,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "ptbr____"
        // ASCII-7-bit=121  UTF8=190  [top UTF8]
  {{0x70,0x74,0x69,0x73,0x38,0x35,0x39,0x31, 0x11,0xbe,0x21,0x7e,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "ptis8591"
        // Latin1=190  CP1252=126  [top Latin1]
  {{0x70,0x74,0x70,0x74,0x35,0x39,0x31,0x35, 0x11,0x89,0x21,0x6f,0xa1,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "ptpt5915"
        // Latin1=137  CP1252=111  ISO-8859-15=190  [top ISO-8859-15]
  {{0x72,0x66,0x63,0x5f,0x5f,0x5f,0x5f,0x5f, 0x01,0x87,0x11,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "rfc_____"
        // ASCII-7-bit=135  UTF8=190  [top UTF8]
  {{0x72,0x6f,0x69,0x73,0x38,0x35,0x39,0x31, 0x02,0x83,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "rois8591"
        // ASCII-7-bit=131  Latin1=190  [top Latin1]
  {{0x72,0x6f,0x72,0x6f,0x38,0x35,0x39,0x32, 0x01,0x99,0x81,0xbd,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "roro8592"
        // ASCII-7-bit=153  Latin2=189  [top Latin2]
  {{0x72,0x75,0x72,0x75,0x31,0x32,0x35,0x31, 0xa1,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "ruru1251"
        // CP1251=191  [top CP1251]
  {{0x72,0x75,0x77,0x69,0x31,0x32,0x35,0x31, 0x01,0x6f,0x91,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "ruwi1251"
        // ASCII-7-bit=111  CP1251=190  [top CP1251]
  {{0x73,0x65,0x65,0x6d,0x38,0x35,0x39,0x31, 0x02,0x6f,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "seem8591"
        // ASCII-7-bit=111  Latin1=190  [top Latin1]
  {{0x73,0x65,0x74,0x63,0x5f,0x5f,0x5f,0x5f, 0x01,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "setc____"
        // ASCII-7-bit=191  [top ASCII-7-bit]
  {{0x73,0x68,0x69,0x66,0x31,0x32,0x35,0x32, 0x02,0x86,0x6f,0x21,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "shif1252"
        // ASCII-7-bit=134  Latin1=111  CP1252=190  [top CP1252]
  {{0x73,0x68,0x69,0x66,0x5f,0x5f,0x5f,0x5f, 0x01,0x6e,0x51,0xbe,0x10,0x11,0x6b,0x00,0x00,0x00,0x00,0x00,}}, // "shif____"
        // ASCII-7-bit=110  SJS=190  CP932=107  [top SJS]
  {{0x73,0x69,0x66,0x74,0x5f,0x5f,0x5f,0x5f, 0x01,0x72,0x51,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "sift____"
        // ASCII-7-bit=114  SJS=190  [top SJS]
  {{0x73,0x6a,0x69,0x73,0x5f,0x5f,0x5f,0x5f, 0x01,0x79,0x51,0xbe,0x10,0x11,0x5d,0x00,0x00,0x00,0x00,0x00,}}, // "sjis____"
        // ASCII-7-bit=121  SJS=190  CP932=93  [top SJS]
  {{0x73,0x6b,0x77,0x69,0x31,0x32,0x35,0x30, 0xc1,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "skwi1250"
        // CP1250=191  [top CP1250]
  {{0x73,0x6f,0x5f,0x5f,0x35,0x39,0x31,0x35, 0x02,0x86,0x6f,0xd1,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "so__5915"
        // ASCII-7-bit=134  Latin1=111  ISO-8859-15=190  [top ISO-8859-15]
  {{0x73,0x6f,0x5f,0x5f,0x38,0x35,0x39,0x31, 0x02,0x9a,0xbd,0x21,0x8b,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "so__8591"
        // ASCII-7-bit=154  Latin1=189  CP1252=139  [top Latin1]
  {{0x73,0x6f,0x5f,0x5f,0x38,0x35,0x39,0x32, 0x91,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "so__8592"
        // Latin2=191  [top Latin2]
  {{0x73,0x76,0x73,0x65,0x38,0x35,0x39,0x31, 0x11,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "svse8591"
        // Latin1=191  [top Latin1]
  {{0x74,0x61,0x62,0x5f,0x5f,0x5f,0x5f,0x5f, 0x30,0x41,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "tab_____"
        // TAB=191  [top TAB]
  {{0x74,0x61,0x6d,0x5f,0x5f,0x5f,0x5f,0x5f, 0x30,0x31,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "tam_____"
        // TAM=191  [top TAM]
  {{0x74,0x65,0x78,0x74,0x5f,0x5f,0x5f,0x5f, 0x02,0xac,0xb7,0x21,0xa0,0xa1,0x49,0x00,0x00,0x00,0x00,0x00,}}, // "text____"
        // ASCII-7-bit=172  Latin1=183  CP1252=160  ISO-8859-15=73  [top Latin1]
  {{0x74,0x69,0x73,0x5f,0x36,0x31,0x38,0x5f, 0x01,0x75,0xd1,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "tis_618_"
        // ASCII-7-bit=117  ISO-8859-11=190  [top ISO-8859-11]
  {{0x74,0x69,0x73,0x5f,0x36,0x32,0x30,0x5f, 0x01,0x82,0xd1,0xbe,0xd1,0x7b,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "tis_620_"
        // ASCII-7-bit=130  ISO-8859-11=190  CP874=123  [top ISO-8859-11]
  {{0x74,0x72,0x5f,0x5f,0x38,0x35,0x39,0x39, 0xd1,0xbe,0x81,0x6f,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "tr__8599"
        // Latin5=190  CP1254=111  [top Latin5]
  {{0x74,0x72,0x5f,0x5f,0x5f,0x5f,0x5f,0x5f, 0xd1,0xbe,0x81,0x5f,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "tr______"
        // Latin5=190  CP1254=95  [top Latin5]
  {{0x74,0x72,0x69,0x73,0x38,0x35,0x39,0x39, 0xd1,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "tris8599"
        // Latin5=191  [top Latin5]
  {{0x74,0x73,0x63,0x69,0x5f,0x5f,0x5f,0x5f, 0x30,0x21,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "tsci____"
        // TSCII=191  [top TSCII]
  {{0x75,0x63,0x73,0x5f,0x32,0x5f,0x5f,0x5f, 0x02,0xb8,0xa7,0x21,0xa3,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "ucs_2___"
        // ASCII-7-bit=184  Latin1=167  CP1252=163  [top ASCII-7-bit]
  {{0x75,0x66,0x74,0x5f,0x38,0x5f,0x5f,0x5f, 0x01,0xb0,0x11,0xb8,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "uft_8___"
        // ASCII-7-bit=176  UTF8=184  [top UTF8]
  {{0x75,0x69,0x73,0x6f,0x38,0x35,0x39,0x31, 0x02,0x6f,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "uiso8591"
        // ASCII-7-bit=111  Latin1=190  [top Latin1]
  {{0x75,0x6e,0x69,0x63,0x31,0x31,0x5f,0x5f, 0x01,0xa7,0x11,0xbb,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "unic11__"
        // ASCII-7-bit=167  UTF8=187  [top UTF8]
  {{0x75,0x6e,0x69,0x63,0x5f,0x5f,0x5f,0x5f, 0x02,0xbe,0x90,0x21,0x85,0xa1,0x45,0x00,0x00,0x00,0x00,0x00,}}, // "unic____"
        // ASCII-7-bit=190  Latin1=144  CP1252=133  ISO-8859-15=69  [top ASCII-7-bit]
  {{0x75,0x6e,0x6b,0x6e,0x38,0x5f,0x5f,0x5f, 0x02,0xa2,0xbb,0x21,0x95,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "unkn8___"
        // ASCII-7-bit=162  Latin1=187  CP1252=149  [top Latin1]
  {{0x75,0x6e,0x6b,0x6e,0x5f,0x5f,0x5f,0x5f, 0x01,0x9c,0x51,0xbd,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "unkn____"
        // ASCII-7-bit=156  SJS=189  [top SJS]
  {{0x75,0x70,0x66,0x5f,0x38,0x5f,0x5f,0x5f, 0x21,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "upf_8___"
        // UTF8=191  [top UTF8]
  {{0x75,0x73,0x5f,0x5f,0x5f,0x5f,0x5f,0x5f, 0x02,0xbe,0x7d,0x21,0x7d,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "us______"
        // ASCII-7-bit=190  Latin1=125  CP1252=125  [top ASCII-7-bit]
  {{0x75,0x73,0x61,0x73,0x5f,0x5f,0x5f,0x5f, 0x02,0xbe,0x83,0x21,0x6a,0xa1,0x38,0x00,0x00,0x00,0x00,0x00,}}, // "usas____"
        // ASCII-7-bit=190  Latin1=131  CP1252=106  ISO-8859-15=56  [top ASCII-7-bit]
  {{0x75,0x73,0x65,0x6e,0x5f,0x5f,0x5f,0x5f, 0x02,0xb8,0x94,0x21,0xad,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "usen____"
        // ASCII-7-bit=184  Latin1=148  CP1252=173  [top ASCII-7-bit]
  {{0x75,0x73,0x65,0x72,0x5f,0x5f,0x5f,0x5f, 0x02,0xb9,0x9e,0x21,0xa7,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "user____"
        // ASCII-7-bit=185  Latin1=158  CP1252=167  [top ASCII-7-bit]
  {{0x75,0x73,0x69,0x73,0x38,0x35,0x39,0x31, 0x02,0x78,0xbe,0x21,0x78,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "usis8591"
        // ASCII-7-bit=120  Latin1=190  CP1252=120  [top Latin1]
  {{0x75,0x73,0x6f,0x5f,0x38,0x35,0x39,0x31, 0x02,0x79,0xbc,0x21,0xa5,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "uso_8591"
        // ASCII-7-bit=121  Latin1=188  CP1252=165  [top Latin1]
  {{0x75,0x74,0x66,0x5f,0x31,0x36,0x5f,0x5f, 0x01,0xb0,0x11,0xb8,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "utf_16__"
        // ASCII-7-bit=176  UTF8=184  [top UTF8]
  {{0x75,0x74,0x66,0x5f,0x33,0x32,0x5f,0x5f, 0x02,0xb5,0xa9,0x21,0x9f,0xa1,0xa1,0x00,0x00,0x00,0x00,0x00,}}, // "utf_32__"
        // ASCII-7-bit=181  Latin1=169  CP1252=159  ISO-8859-15=161  [top ASCII-7-bit]
  {{0x75,0x74,0x66,0x5f,0x35,0x39,0x31,0x35, 0x11,0x90,0xd1,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "utf_5915"
        // Latin1=144  ISO-8859-15=190  [top ISO-8859-15]
  {{0x75,0x74,0x66,0x5f,0x37,0x5f,0x5f,0x5f, 0x01,0x88,0x20,0xb1,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "utf_7___"
        // ASCII-7-bit=136  UTF7=190  [top UTF7]
  {{0x75,0x74,0x66,0x5f,0x38,0x35,0x39,0x31, 0x02,0x95,0xbd,0x21,0x8c,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "utf_8591"
        // ASCII-7-bit=149  Latin1=189  CP1252=140  [top Latin1]
  {{0x75,0x74,0x66,0x5f,0x38,0x35,0x39,0x39, 0xd1,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "utf_8599"
        // Latin5=191  [top Latin5]
  {{0x75,0x74,0x66,0x5f,0x38,0x5f,0x5f,0x5f, 0x01,0xae,0x11,0xb9,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "utf_8___"
        // ASCII-7-bit=174  UTF8=185  [top UTF8]
  {{0x75,0x74,0x66,0x5f,0x5f,0x5f,0x5f,0x5f, 0x02,0xbe,0x8a,0x21,0x74,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "utf_____"
        // ASCII-7-bit=190  Latin1=138  CP1252=116  [top ASCII-7-bit]
  {{0x75,0x74,0x66,0x62,0x31,0x36,0x5f,0x5f, 0x01,0xa5,0x20,0x41,0xbc,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "utfb16__"
        // ASCII-7-bit=165  UTF-16BE=188  [top UTF-16BE]
  {{0x75,0x74,0x66,0x62,0x33,0x32,0x5f,0x5f, 0x30,0x81,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "utfb32__"
        // UTF-32BE=191  [top UTF-32BE]
  {{0x75,0x74,0x66,0x69,0x38,0x35,0x39,0x31, 0x02,0x99,0xbd,0x21,0x87,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "utfi8591"
        // ASCII-7-bit=153  Latin1=189  CP1252=135  [top Latin1]
  {{0x75,0x74,0x66,0x6c,0x31,0x36,0x5f,0x5f, 0x20,0x71,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "utfl16__"
        // UTF-16LE=191  [top UTF-16LE]
  {{0x75,0x74,0x66,0x6c,0x33,0x32,0x5f,0x5f, 0x30,0x91,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "utfl32__"
        // UTF-32LE=191  [top UTF-32LE]
  {{0x75,0x74,0x66,0x75,0x38,0x38,0x5f,0x5f, 0x30,0xb1,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "utfu88__"
        // X-UTF8UTF8=191  [top X-UTF8UTF8]
  {{0x76,0x61,0x6c,0x75,0x5f,0x5f,0x5f,0x5f, 0x11,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "valu____"
        // Latin1=191  [top Latin1]
  {{0x76,0x69,0x73,0x75,0x5f,0x5f,0x5f,0x5f, 0x01,0x84,0x10,0x81,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "visu____"
        // ASCII-7-bit=132  Hebrew=190  [top Hebrew]
  {{0x77,0x61,0x69,0x6e,0x31,0x32,0x35,0x30, 0xc1,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "wain1250"
        // CP1250=191  [top CP1250]
  {{0x77,0x65,0x69,0x73,0x35,0x39,0x31,0x35, 0x02,0x9f,0x7d,0x21,0x84,0xa1,0xbc,0x00,0x00,0x00,0x00,0x00,}}, // "weis5915"
        // ASCII-7-bit=159  Latin1=125  CP1252=132  ISO-8859-15=188  [top ISO-8859-15]
  {{0x77,0x65,0x69,0x73,0x38,0x35,0x39,0x31, 0x11,0xbe,0x21,0x7e,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "weis8591"
        // Latin1=190  CP1252=126  [top Latin1]
  {{0x77,0x65,0x73,0x74,0x31,0x32,0x35,0x32, 0x01,0x6f,0x31,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "west1252"
        // ASCII-7-bit=111  CP1252=190  [top CP1252]
  {{0x77,0x65,0x73,0x74,0x38,0x35,0x39,0x31, 0x02,0x79,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "west8591"
        // ASCII-7-bit=121  Latin1=190  [top Latin1]
  {{0x77,0x65,0x73,0x74,0x5f,0x5f,0x5f,0x5f, 0x02,0xa9,0x9d,0x21,0xb9,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "west____"
        // ASCII-7-bit=169  Latin1=157  CP1252=185  [top CP1252]
  {{0x77,0x69,0x64,0x6e,0x31,0x32,0x35,0x30, 0xc1,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "widn1250"
        // CP1250=191  [top CP1250]
  {{0x77,0x69,0x64,0x6f,0x31,0x32,0x35,0x30, 0x01,0x7c,0xb1,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "wido1250"
        // ASCII-7-bit=124  CP1250=190  [top CP1250]
  {{0x77,0x69,0x64,0x6f,0x31,0x32,0x35,0x31, 0xa1,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "wido1251"
        // CP1251=191  [top CP1251]
  {{0x77,0x69,0x64,0x6f,0x31,0x32,0x35,0x32, 0x11,0xa9,0x21,0xbb,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "wido1252"
        // Latin1=169  CP1252=187  [top CP1252]
  {{0x77,0x69,0x64,0x6f,0x31,0x32,0x35,0x36, 0xb1,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "wido1256"
        // CP1256=191  [top CP1256]
  {{0x77,0x69,0x6d,0x64,0x31,0x32,0x35,0x31, 0xa1,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "wimd1251"
        // CP1251=191  [top CP1251]
  {{0x77,0x69,0x6e,0x5f,0x31,0x32,0x35,0x30, 0x01,0x8d,0xb1,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "win_1250"
        // ASCII-7-bit=141  CP1250=190  [top CP1250]
  {{0x77,0x69,0x6e,0x5f,0x31,0x32,0x35,0x31, 0x01,0x8f,0x91,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "win_1251"
        // ASCII-7-bit=143  CP1251=190  [top CP1251]
  {{0x77,0x69,0x6e,0x5f,0x31,0x32,0x35,0x32, 0x02,0xac,0xa4,0x21,0xb6,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "win_1252"
        // ASCII-7-bit=172  Latin1=164  CP1252=182  [top CP1252]
  {{0x77,0x69,0x6e,0x5f,0x31,0x32,0x35,0x33, 0x10,0x41,0x85,0x21,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "win_1253"
        // Greek=133  CP1253=190  [top CP1253]
  {{0x77,0x69,0x6e,0x5f,0x31,0x32,0x35,0x34, 0x01,0x6f,0xc1,0xaf,0x81,0xb9,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "win_1254"
        // ASCII-7-bit=111  Latin5=175  CP1254=185  [top CP1254]
  {{0x77,0x69,0x6e,0x5f,0x31,0x32,0x35,0x35, 0x10,0x11,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "win_1255"
        // CP1255=191  [top CP1255]
  {{0x77,0x69,0x6e,0x5f,0x31,0x32,0x35,0x36, 0x01,0x7f,0xa1,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "win_1256"
        // ASCII-7-bit=127  CP1256=190  [top CP1256]
  {{0x77,0x69,0x6e,0x5f,0x31,0x32,0x35,0x37, 0x01,0x8c,0xf1,0xbe,0xc1,0x77,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "win_1257"
        // ASCII-7-bit=140  CP1257=190  ISO-8859-13=119  [top CP1257]
  {{0x77,0x69,0x6e,0x5f,0x38,0x37,0x34,0x5f, 0x01,0x56,0xd1,0xaf,0xd1,0xb9,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "win_874_"
        // ASCII-7-bit=86  ISO-8859-11=175  CP874=185  [top CP874]
  {{0x77,0x69,0x6e,0x5f,0x5f,0x5f,0x5f,0x5f, 0x01,0x9a,0x91,0xbd,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "win_____"
        // ASCII-7-bit=154  CP1251=189  [top CP1251]
  {{0x77,0x69,0x6e,0x63,0x31,0x32,0x35,0x30, 0xc1,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "winc1250"
        // CP1250=191  [top CP1250]
  {{0x77,0x69,0x6e,0x63,0x31,0x32,0x35,0x31, 0xa1,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "winc1251"
        // CP1251=191  [top CP1251]
  {{0x77,0x69,0x6e,0x64,0x31,0x32,0x33,0x34, 0xb1,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "wind1234"
        // CP1256=191  [top CP1256]
  {{0x77,0x69,0x6e,0x64,0x31,0x32,0x35,0x30, 0x01,0x88,0xb1,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "wind1250"
        // ASCII-7-bit=136  CP1250=190  [top CP1250]
  {{0x77,0x69,0x6e,0x64,0x31,0x32,0x35,0x31, 0x01,0x8b,0x91,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "wind1251"
        // ASCII-7-bit=139  CP1251=190  [top CP1251]
  {{0x77,0x69,0x6e,0x64,0x31,0x32,0x35,0x32, 0x02,0xa5,0xac,0x21,0xb6,0xa1,0x4f,0x00,0x00,0x00,0x00,0x00,}}, // "wind1252"
        // ASCII-7-bit=165  Latin1=172  CP1252=182  ISO-8859-15=79  [top CP1252]
  {{0x77,0x69,0x6e,0x64,0x31,0x32,0x35,0x33, 0x01,0x94,0x10,0x31,0xae,0x21,0xb8,0x00,0x00,0x00,0x00,0x00,}}, // "wind1253"
        // ASCII-7-bit=148  Greek=174  CP1253=184  [top CP1253]
  {{0x77,0x69,0x6e,0x64,0x31,0x32,0x35,0x34, 0x01,0x73,0xc1,0xaf,0x81,0xb9,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "wind1254"
        // ASCII-7-bit=115  Latin5=175  CP1254=185  [top CP1254]
  {{0x77,0x69,0x6e,0x64,0x31,0x32,0x35,0x35, 0x01,0x86,0x10,0x01,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "wind1255"
        // ASCII-7-bit=134  CP1255=190  [top CP1255]
  {{0x77,0x69,0x6e,0x64,0x31,0x32,0x35,0x36, 0x01,0x74,0xa1,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "wind1256"
        // ASCII-7-bit=116  CP1256=190  [top CP1256]
  {{0x77,0x69,0x6e,0x64,0x31,0x32,0x35,0x37, 0x01,0x87,0xf1,0xbe,0xc1,0x52,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "wind1257"
        // ASCII-7-bit=135  CP1257=190  ISO-8859-13=82  [top CP1257]
  {{0x77,0x69,0x6e,0x64,0x33,0x31,0x5f,0x5f, 0x01,0x62,0x51,0xbe,0x10,0x11,0x5e,0x00,0x00,0x00,0x00,0x00,}}, // "wind31__"
        // ASCII-7-bit=98  SJS=190  CP932=94  [top SJS]
  {{0x77,0x69,0x6e,0x64,0x38,0x34,0x37,0x5f, 0xe1,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "wind847_"
        // ISO-8859-11=191  [top ISO-8859-11]
  {{0x77,0x69,0x6e,0x64,0x38,0x35,0x32,0x5f, 0x01,0x79,0x20,0x01,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "wind852_"
        // ASCII-7-bit=121  CP852=190  [top CP852]
  {{0x77,0x69,0x6e,0x64,0x38,0x35,0x39,0x31, 0x02,0x9a,0xbd,0x21,0x89,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "wind8591"
        // ASCII-7-bit=154  Latin1=189  CP1252=137  [top Latin1]
  {{0x77,0x69,0x6e,0x64,0x38,0x35,0x39,0x32, 0x01,0x83,0x81,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "wind8592"
        // ASCII-7-bit=131  Latin2=190  [top Latin2]
  {{0x77,0x69,0x6e,0x64,0x38,0x35,0x39,0x36, 0x01,0x6f,0x20,0x11,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "wind8596"
        // ASCII-7-bit=111  Arabic=190  [top Arabic]
  {{0x77,0x69,0x6e,0x64,0x38,0x35,0x39,0x37, 0x01,0x6f,0x10,0x31,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "wind8597"
        // ASCII-7-bit=111  Greek=190  [top Greek]
  {{0x77,0x69,0x6e,0x64,0x38,0x35,0x39,0x39, 0x01,0x6c,0xc1,0xbe,0x81,0x6c,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "wind8599"
        // ASCII-7-bit=108  Latin5=190  CP1254=108  [top Latin5]
  {{0x77,0x69,0x6e,0x64,0x38,0x36,0x36,0x5f, 0x20,0x41,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "wind866_"
        // CP866=191  [top CP866]
  {{0x77,0x69,0x6e,0x64,0x38,0x37,0x34,0x5f, 0x01,0x8a,0xd1,0xbe,0xd1,0x7d,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "wind874_"
        // ASCII-7-bit=138  ISO-8859-11=190  CP874=125  [top ISO-8859-11]
  {{0x77,0x69,0x6e,0x64,0x38,0x38,0x35,0x39, 0x02,0x97,0xb6,0x21,0xb1,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "wind8859"
        // ASCII-7-bit=151  Latin1=182  CP1252=177  [top Latin1]
  {{0x77,0x69,0x6e,0x64,0x38,0x5f,0x5f,0x5f, 0x01,0x93,0x11,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "wind8___"
        // ASCII-7-bit=147  UTF8=190  [top UTF8]
  {{0x77,0x69,0x6e,0x64,0x39,0x33,0x32,0x5f, 0x01,0x7d,0x51,0xa4,0x10,0x11,0xbc,0x00,0x00,0x00,0x00,0x00,}}, // "wind932_"
        // ASCII-7-bit=125  SJS=164  CP932=188  [top CP932]
  {{0x77,0x69,0x6e,0x64,0x39,0x34,0x39,0x5f, 0x01,0x7b,0x41,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "wind949_"
        // ASCII-7-bit=123  KSC=190  [top KSC]
  {{0x77,0x69,0x6e,0x64,0x39,0x35,0x30,0x5f, 0x01,0x6f,0x71,0x7f,0x20,0x51,0xbe,0x00,0x00,0x00,0x00,0x00,}}, // "wind950_"
        // ASCII-7-bit=111  BIG5=127  BIG5-CP950=190  [top BIG5-CP950]
  {{0x77,0x69,0x6e,0x64,0x5f,0x5f,0x5f,0x5f, 0x01,0xb5,0x11,0xb4,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "wind____"
        // ASCII-7-bit=181  UTF8=180  [top ASCII-7-bit]
  {{0x77,0x69,0x6e,0x65,0x31,0x32,0x35,0x32, 0x01,0x6f,0x31,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "wine1252"
        // ASCII-7-bit=111  CP1252=190  [top CP1252]
  {{0x77,0x69,0x6e,0x6f,0x31,0x32,0x35,0x30, 0xc1,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "wino1250"
        // CP1250=191  [top CP1250]
  {{0x77,0x69,0x6e,0x6f,0x31,0x32,0x35,0x31, 0xa1,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "wino1251"
        // CP1251=191  [top CP1251]
  {{0x77,0x69,0x6e,0x73,0x31,0x32,0x35,0x35, 0x10,0x11,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "wins1255"
        // CP1255=191  [top CP1255]
  {{0x77,0x69,0x72,0x64,0x31,0x32,0x35,0x31, 0xa1,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "wird1251"
        // CP1251=191  [top CP1251]
  {{0x77,0x69,0x73,0x6f,0x38,0x35,0x39,0x31, 0x11,0xbe,0x21,0x7f,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "wiso8591"
        // Latin1=190  CP1252=127  [top Latin1]
  {{0x77,0x6e,0x64,0x6f,0x31,0x32,0x35,0x31, 0xa1,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "wndo1251"
        // CP1251=191  [top CP1251]
  {{0x77,0x6e,0x64,0x6f,0x31,0x32,0x35,0x36, 0x01,0x6e,0xa1,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "wndo1256"
        // ASCII-7-bit=110  CP1256=190  [top CP1256]
  {{0x77,0x6f,0x6e,0x64,0x31,0x32,0x35,0x32, 0x41,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "wond1252"
        // CP1252=191  [top CP1252]
  {{0x77,0x6f,0x72,0x67,0x31,0x32,0x35,0x32, 0x01,0x83,0x31,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "worg1252"
        // ASCII-7-bit=131  CP1252=190  [top CP1252]
  {{0x79,0x65,0x73,0x5f,0x5f,0x5f,0x5f,0x5f, 0x02,0xbe,0x81,0x21,0x8b,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "yes_____"
        // ASCII-7-bit=190  Latin1=129  CP1252=139  [top ASCII-7-bit]
  {{0x79,0x6b,0x74,0x63,0x5f,0x5f,0x5f,0x5f, 0x51,0xbf,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "yktc____"
        // KSC=191  [top KSC]
  {{0x7a,0x73,0x6f,0x5f,0x38,0x35,0x39,0x31, 0x02,0x6f,0xbe,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,}}, // "zso_8591"
        // ASCII-7-bit=111  Latin1=190  [top Latin1]
};

static const int kCharsetHintProbsSize = 438;

static const uint8 kDefaultProb[NUM_RANKEDENCODING] = {	// MaxRange 192
177, 170, 156, 149, 150, 142, 140, 124,  130, 127, 124, 118, 127, 118, 109, 104,  98, 93, 96, 82, 84, 81, 80, 64,  61, 57, 53, 47, 42, 28, 24, 22,
  17, 0, 5, 1, 5, 12, 0, 5,  0, 0, 0, 0, 0, 0, 0, 0,  0, 0, 0, 0, 0, 0,  0, 0, 0, 0, 0, 0, 0, 0,  0, 0, 0, 0, 0, };

static const int kMaxTldKey = 4;
static const int kMaxTldVector = 16;
static const int kMaxCharsetKey = 8;
static const int kMaxCharsetVector = 12;
static const int kMaxLangKey = 8;
static const int kMaxLangVector = 12;
// Smoothing percentage across encodings with same UTF-8 result: 100%

static const UnigramEntry unigram_table[NUM_RANKEDENCODING] = {
{ // ASCII-7-bit (788.373M chars) [0]
  {NULL, NULL, NULL, NULL},
  77, 207, 29, 27, 255,
    {0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   189,189,189,189,189,189,189,189, 189,189,189,189,189,189,189,189, 189,189,189,189,189,189,189,189, 189,189,189,189,189,189,189,189,
   189,189,189,189,189,189,189,189, 189,189,189,189,189,189,189,189, 189,189,189,189,189,189,189,189, 189,189,189,189,189,189,189,189,
   189,189,189,189,189,189,189,189, 189,189,189,189,189,189,189,189, 189,189,189,189,189,189,189,189, 189,189,189,189,189,189,189,189,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   },
  {0,0,0,0,0,0,0,0, 0,189,189,0,0,189,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   0,189,189,189,189,189,189,189, 189,189,189,189,189,189,189,189, 189,189,189,189,189,189,189,189, 189,189,189,189,189,189,189,189,
   189,189,189,189,189,189,189,189, 189,189,189,189,189,189,189,189, 189,189,189,189,189,189,189,189, 189,189,189,189,189,189,189,189,
   189,189,189,189,189,189,189,189, 189,189,189,189,189,189,189,189, 189,189,189,189,189,189,189,189, 189,189,189,189,189,189,189,0,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   },
  {0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   2,0,2,2,2,2,2,2, 0,0,0,0,0,0,0,0, 2,0,2,2,2,2,2,2, 0,0,0,0,0,0,0,0,
   2,0,2,2,2,2,2,2, 0,0,0,0,0,0,0,0, 2,0,2,2,2,2,2,2, 0,0,0,0,0,0,0,0,
   2,0,2,2,2,2,2,2, 0,0,0,0,0,0,0,0, 2,0,2,2,2,2,2,2, 0,0,0,0,0,0,0,0,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   },
},

{ // Latin1 (1792.786M chars) [1]
  {NULL, NULL, ced_hires_13, ced_hires_13, },
  87, 217, 37, 20, 128,
    {1,0,1,1,0,0,1,0, 0,9,9,0,1,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   186,137,105,112,140,106,126,145, 132,113,128,124,123,101,126,119, 141,113,107,116,129,113,113,143, 130,105,127,128,103,116,106,128,
   122,138,132,155,161,129,133,190, 124,152,124,119,117,144,120,121, 127,136,122,132,112,139,144,116, 113,101,117,117,145,120,135,114,
   121,138,107,125,163,129,131,190, 120,141,112,110,114,143,115,133, 124,135,126,131,118,141,160,110, 114,104,119,99,148,119,129,117,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   195,159,115,166,155,125,133,156, 130,168,156,159,121,135,150,133, 169,125,135,118,150,113,114,173, 126,107,176,178,97,131,103,170,
   165,209,166,183,211,198,182,194, 193,219,182,153,130,205,161,148, 160,187,142,208,169,167,210,131, 192,122,189,152,212,181,158,185,
   195,214,155,184,212,208,182,181, 183,213,171,154,161,215,150,137, 175,187,162,210,158,168,208,131, 192,156,189,142,199,193,155,120,
   },
  {43,13,19,71,0,0,0,0, 0,145,153,0,0,165,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   226,142,175,92,109,120,168,161, 150,151,135,110,183,159,177,142, 146,163,159,149,146,144,137,136, 133,134,166,128,200,102,117,143,
   102,197,188,202,195,190,180,196, 185,179,172,189,202,198,215,195, 185,164,216,206,204,172,190,182, 158,170,181,119,107,136,114,132,
   134,194,189,198,192,189,178,194, 182,176,171,189,201,194,213,194, 181,151,213,203,201,171,189,182, 155,176,182,106,136,85,122,2,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   187,138,112,113,139,107,128,136, 133,137,118,120,118,129,135,123, 139,125,101,126,128,108,123,140, 131,107,125,133,123,119,104,138,
   133,128,130,187,160,126,123,145, 123,153,122,122,114,136,125,117, 153,141,127,129,107,167,144,114, 113,105,118,118,134,117,122,164,
   130,132,122,187,161,131,122,139, 122,137,119,120,113,138,123,110, 153,141,127,130,133,167,146,104, 117,105,130,109,131,117,133,119,
   },
  {128,0,110,142,142,142,142,140, 0,0,142,142,142,142,142,142, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   22,0,2,2,2,2,2,2, 0,0,2,14,2,2,2,2, 46,0,2,14,2,2,2,2, 0,0,2,18,2,4,2,6,
   16,0,2,2,2,2,2,2, 0,0,2,2,2,2,2,2, 42,0,2,10,2,2,2,2, 0,0,2,2,2,2,2,2,
   18,0,2,2,2,2,2,2, 0,0,2,2,2,2,2,2, 38,0,2,6,2,2,2,2, 0,0,2,2,2,2,2,2,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   164,0,136,164,110,108,108,106, 0,0,190,164,136,134,122,152, 158,0,144,156,112,116,106,106, 0,0,166,212,158,164,146,162,
   90,0,92,96,139,139,94,92, 0,0,148,166,184,176,134,146, 98,0,96,104,139,139,110,110, 0,0,138,172,174,190,154,164,
   130,0,138,126,38,48,135,135, 0,0,130,156,128,148,186,180, 120,0,126,120,48,56,139,141, 0,0,142,156,140,186,166,188,
   },
},

{ // UTF8 (16713.069M chars) [2]
  {NULL, NULL, NULL, NULL},
  169, 203, 42, 24, 131,
    {197,207,201,202,188,181,180,180, 188,183,184,187,188,182,178,183, 181,179,172,178,182,182,183,183, 181,181,183,180,189,181,175,180,
   182,183,175,177,184,183,183,184, 187,176,181,181,178,186,187,184, 187,177,175,183,184,176,174,177, 196,183,188,186,194,184,180,181,
   0,0,183,211,189,187,147,104, 65,123,87,102,121,13,187,176, 216,203,118,108,88,134,124,193, 206,202,155,161,58,0,119,0,
   181,176,180,213,192,206,202,197, 192,187,165,177,183,170,90,182, 71,0,0,13,1,0,0,0, 0,0,0,0,0,0,0,0,
   119,115,120,96,114,98,104,106, 121,111,96,110,113,105,109,100, 109,107,92,112,112,97,91,102, 110,119,116,99,117,110,104,106,
   133,132,135,130,132,139,129,131, 133,127,121,126,132,128,121,138, 136,128,120,126,145,114,120,123, 132,124,128,122,138,120,117,132,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,64,
   },
  {0,0,0,0,0,0,0,0, 0,109,143,0,0,144,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   189,125,169,73,68,107,151,137, 146,147,111,102,152,138,154,129, 142,152,148,137,131,132,127,127, 123,121,139,118,189,98,96,127,
   87,124,119,124,117,115,112,111, 114,127,109,102,114,120,120,114, 120,94,118,122,122,113,106,113, 95,99,88,117,109,125,97,107,
   80,139,113,146,112,126,103,99, 115,147,96,97,118,131,156,123, 132,80,111,140,146,136,107,102, 86,116,85,83,110,84,118,0,
   204,210,207,204,196,192,190,188, 194,187,191,189,194,188,179,187, 184,183,175,181,186,187,183,187, 184,189,185,186,191,184,178,186,
   192,196,179,183,192,185,183,198, 192,196,189,182,182,193,189,192, 199,194,188,195,190,194,185,185, 203,191,196,195,200,196,197,188,
   0,0,121,113,72,69,49,4, 0,0,0,71,0,0,113,107, 113,102,0,0,0,0,19,86, 134,132,91,105,0,0,0,0,
   174,140,150,207,185,199,194,190, 187,181,160,174,177,157,75,176, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   },
  {0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   6,0,2,2,2,6,2,2, 0,0,0,0,50,38,2,128, 4,0,2,2,2,4,2,2, 0,0,0,0,54,40,2,128,
   4,0,2,2,2,6,2,2, 0,0,0,0,128,128,128,128, 2,0,2,2,2,2,2,2, 0,0,0,0,128,128,128,128,
   2,0,2,2,2,2,2,2, 0,0,0,0,128,128,128,128, 128,0,118,118,128,128,128,128, 0,0,0,0,128,128,64,128,
   176,0,168,172,172,172,162,164, 120,126,126,116,126,140,134,128, 168,0,170,166,172,170,174,176, 118,122,118,116,142,116,138,128,
   168,0,170,168,168,172,176,174, 110,120,114,112,136,142,140,128, 168,0,172,170,160,166,160,164, 120,126,118,116,140,134,136,128,
   0,0,0,0,0,0,0,0, 116,122,140,128,0,0,0,0, 0,0,0,0,0,0,0,0, 128,120,126,134,0,0,0,0,
   0,0,0,0,0,0,0,0, 134,132,120,126,0,0,0,0, 0,0,0,0,0,0,0,0, 116,152,138,116,0,0,0,0,
   },
},

{ // GB (9061.562M chars) [3]
  {NULL, ced_hires_3, ced_hires_4, ced_hires_5, },
  204, 189, 27, 16, 128,
    {0,0,0,0,0,0,0,0, 0,0,0,0,0,0,66,73, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   70,204,125,202,219,216,120,114, 118,156,113,85,96,78,81,79, 190,196,199,195,195,208,196,199, 196,203,195,200,201,197,193,195,
   193,195,190,195,195,183,196,191, 201,197,207,197,197,198,201,199, 201,191,203,201,198,196,202,200, 134,116,114,132,122,124,118,115,
   115,120,116,120,141,121,124,123, 121,120,121,113,125,116,117,110, 117,114,112,113,119,112,124,123, 84,74,83,78,69,83,77,0,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,37, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   0,93,66,68,84,3,51,0, 116,102,41,61,83,57,66,12, 70,85,73,66,54,64,44,68, 0,3,3,3,54,73,70,54,
   95,66,54,44,66,78,26,37, 57,41,3,57,13,2,0,0, 54,51,0,41,68,70,70,59, 41,70,41,13,0,64,59,80,
   12,3,51,12,13,26,41,3, 0,79,47,61,2,19,66,64, 37,37,13,93,54,13,54,51, 51,12,59,73,89,54,54,94,
   },
  {101,14,0,0,0,0,0,0, 16,16,0,0,68,49,44,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   81,56,73,0,0,0,64,71, 49,49,0,84,53,69,60,32, 84,60,49,35,15,19,18,1, 2,0,15,0,92,29,0,0,
   83,69,66,85,103,49,69,51, 56,71,53,83,70,73,65,91, 71,59,61,71,80,77,72,79, 61,69,67,64,66,73,77,73,
   70,75,71,70,86,62,64,69, 71,66,39,69,73,64,74,75, 79,69,71,70,83,80,53,92, 69,66,59,69,69,85,90,3,
   94,72,77,73,64,67,62,71, 80,76,76,72,65,74,80,73, 77,93,79,99,84,69,81,94, 51,35,67,74,49,67,84,78,
   126,196,197,192,196,176,185,184, 195,185,196,194,198,187,184,191, 190,190,180,189,189,194,181,191, 182,191,188,196,189,191,187,192,
   186,174,192,193,203,190,193,194, 190,188,189,195,181,187,196,191, 200,178,184,186,189,186,192,180, 193,176,195,182,185,179,182,186,
   185,187,190,185,183,187,185,186, 177,190,189,192,187,187,181,180, 178,185,188,196,172,177,186,192, 192,182,197,186,178,187,185,0,
   },
  {128,0,128,128,128,128,128,128, 128,128,104,128,114,124,128,118, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   46,0,46,50,32,40,40,34, 128,128,26,28,26,30,32,32, 42,0,42,44,26,34,34,28, 128,128,46,50,46,52,54,54,
   44,0,44,46,28,36,36,30, 128,128,48,50,46,52,56,54, 50,0,50,54,34,42,42,38, 128,128,50,52,50,56,58,58,
   128,0,128,128,128,128,128,128, 128,128,58,62,58,64,70,66, 128,0,128,128,128,128,128,128, 128,128,40,42,40,44,48,46,
   0,0,0,0,154,180,164,168, 188,176,128,106,122,124,134,92, 0,0,0,0,128,128,128,136, 128,128,132,130,130,132,130,130,
   0,0,0,0,242,218,218,232, 138,134,137,131,129,117,127,113, 0,0,0,0,202,206,218,216, 110,112,129,127,129,127,127,131,
   0,0,0,0,202,218,218,206, 114,106,122,126,126,128,130,132, 0,0,0,0,210,214,206,206, 120,114,124,128,126,136,122,126,
   0,0,0,0,206,208,200,202, 218,236,136,124,124,122,126,126, 230,194,200,214,190,198,194,216, 192,186,128,122,132,128,130,120,
   },
},

{ // CP1252 (408.280M chars) [4]
  {NULL, NULL, ced_hires_13, ced_hires_13, },
  89, 209, 40, 30, 128,
    {116,114,130,121,123,133,113,118, 108,96,180,101,111,113,172,102, 63,68,105,82,59,104,73,70, 61,79,141,65,74,59,133,92,
   184,136,106,111,139,104,125,143, 132,111,126,123,121,100,125,118, 143,111,106,114,127,111,112,142, 128,104,127,127,102,114,106,126,
   120,172,132,158,159,131,136,188, 123,153,123,119,120,159,118,120, 127,136,121,131,136,137,143,117, 112,109,131,116,143,151,134,114,
   120,172,130,128,161,130,131,188, 120,143,112,109,119,159,114,131, 122,133,125,130,137,139,158,109, 114,108,131,98,146,151,128,116,
   169,126,98,99,143,148,118,105, 94,83,203,88,113,99,199,98, 95,134,176,160,158,170,169,143, 114,141,201,118,111,110,201,130,
   193,157,113,163,153,123,131,154, 127,166,153,157,119,132,148,131, 167,123,133,115,148,111,112,171, 124,105,174,176,96,128,101,168,
   163,207,164,180,208,196,180,192, 191,217,180,151,128,203,159,146, 158,185,140,206,167,165,207,128, 190,119,187,150,210,179,156,183,
   193,212,152,182,209,206,180,179, 181,211,168,151,159,213,148,135, 173,185,160,208,156,166,206,129, 190,153,187,139,197,191,153,118,
   },
  {128,43,4,69,19,0,0,0, 0,146,153,0,0,165,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   226,141,174,92,108,119,170,160, 148,152,134,110,182,158,177,142, 147,162,157,147,145,143,136,135, 132,133,165,129,200,100,121,143,
   118,197,187,200,193,200,178,194, 183,191,170,191,201,196,214,194, 184,162,214,205,205,174,188,180, 157,168,179,122,110,135,115,131,
   133,196,189,196,191,201,176,192, 180,192,169,191,199,192,212,193, 182,149,211,202,203,175,187,180, 153,174,180,105,136,86,122,30,
   113,107,124,132,114,134,112,118, 113,93,171,97,104,105,166,96, 99,108,124,118,117,133,112,109, 102,129,171,117,112,107,166,135,
   183,137,111,110,137,105,126,133, 132,134,125,119,116,126,133,122, 144,123,99,123,127,108,120,139, 128,105,124,131,120,117,103,138,
   130,152,132,184,157,126,125,142, 124,153,120,120,118,183,123,115, 150,138,125,127,107,164,145,111, 111,103,118,117,132,116,120,161,
   130,157,121,184,158,129,123,136, 124,145,121,118,111,183,121,110, 150,139,124,127,130,164,143,101, 116,102,132,106,129,115,130,117,
   },
  {32,0,2,24,2,2,2,2, 22,20,16,36,6,24,6,26, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   24,0,2,2,2,2,2,2, 2,2,2,16,2,2,2,4, 46,0,2,16,2,2,2,2, 6,6,2,20,2,6,2,10,
   14,0,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 38,0,2,6,2,2,2,2, 2,2,2,2,2,2,2,2,
   14,0,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 34,0,2,4,2,2,2,2, 2,2,2,2,2,2,2,2,
   130,0,106,124,140,126,112,112, 160,134,134,140,180,136,142,134, 134,0,132,126,86,84,140,128, 118,162,132,166,130,138,180,136,
   162,0,136,166,110,110,108,106, 118,138,192,162,130,136,116,152, 158,0,144,154,112,118,106,106, 148,164,166,210,152,164,138,164,
   88,0,92,96,139,141,94,92, 180,136,146,162,174,174,124,144, 98,0,96,104,139,139,110,112, 184,150,134,164,162,186,142,158,
   128,0,138,128,38,48,135,137, 130,178,128,152,118,146,176,178, 120,0,126,120,48,56,139,141, 136,180,140,150,130,184,156,184,
   },
},

{ // KSC (5258.976M chars) [5]
  {NULL, ced_hires_6, ced_hires_7, ced_hires_8, },
  203, 186, 27, 9, 128,
    {71,109,117,106,108,109,104,107, 110,108,108,112,103,104,106,105, 102,108,101,103,104,107,99,103, 104,99,107,103,102,103,107,98,
   106,206,164,204,164,121,141,122, 146,119,136,130,120,90,91,81, 216,212,164,201,215,207,187,206, 213,208,207,204,210,209,205,216,
   225,214,178,201,188,198,199,213, 199,93,107,104,116,113,115,113, 108,107,109,113,110,97,99,104, 101,113,115,107,110,108,111,117,
   110,108,103,116,115,142,146,136, 118,134,125,131,144,112,112,126, 112,117,107,103,107,103,102,104, 101,114,109,104,105,93,0,0,
   52,0,0,60,0,1,0,0, 0,12,0,0,28,0,18,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   0,78,41,83,69,52,75,65, 1,50,60,76,0,62,88,58, 79,72,60,68,0,65,54,44, 72,58,78,86,46,82,73,48,
   57,67,58,67,72,57,60,86, 18,31,65,50,69,65,85,79, 52,57,54,34,54,0,41,38, 0,78,82,58,50,78,1,46,
   0,57,68,0,72,67,48,65, 48,41,31,107,18,67,63,0, 12,48,0,52,0,28,85,46, 82,67,44,48,48,1,0,0,
   },
  {0,0,0,0,0,0,0,0, 0,44,71,0,0,61,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   117,80,77,0,0,0,66,63, 78,84,0,97,84,72,82,66, 4,74,65,78,51,54,50,9, 24,6,52,5,117,40,0,44,
   63,9,0,52,36,0,9,0, 0,0,0,24,48,0,51,51, 60,1,55,58,40,0,7,6, 59,8,0,81,54,73,75,73,
   57,2,57,35,48,28,1,2, 0,42,0,6,50,1,20,8, 38,0,0,32,57,0,4,2, 35,32,36,56,57,50,89,38,
   66,99,111,102,98,106,96,97, 95,99,98,96,98,104,96,95, 98,97,92,103,92,97,93,96, 95,96,104,97,94,93,92,91,
   157,208,188,195,195,184,194,180, 174,186,184,181,197,195,200,187, 193,181,186,192,167,192,186,169, 205,187,203,199,192,172,174,185,
   186,179,196,199,181,187,182,201, 179,175,184,177,204,183,204,207, 190,189,185,176,191,179,192,189, 182,200,194,190,178,181,182,187,
   177,176,199,166,189,187,186,198, 184,182,174,196,175,194,193,169, 178,186,174,191,161,176,204,191, 198,180,187,178,193,181,149,9,
   },
  {128,0,128,128,128,128,128,128, 128,128,80,86,78,94,92,92, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   92,0,42,48,90,68,96,74, 128,128,44,44,42,48,48,50, 58,0,10,16,56,36,58,40, 128,128,42,44,40,46,48,48,
   66,0,18,26,66,44,66,50, 128,128,42,44,42,48,48,50, 128,0,128,128,128,128,128,128, 128,128,50,52,48,58,58,58,
   128,0,108,108,128,128,128,128, 128,128,40,40,38,44,44,46, 128,0,128,128,128,128,128,128, 128,128,54,56,50,60,58,60,
   0,0,0,0,186,204,190,182, 244,244,124,124,72,82,142,64, 0,0,0,0,164,206,174,184, 240,238,122,124,78,80,144,52,
   0,0,0,0,186,200,180,188, 142,140,147,131,99,107,101,91, 0,0,0,0,174,196,184,194, 112,114,125,127,127,125,133,129,
   0,0,0,0,196,204,194,204, 120,108,124,126,130,132,116,128, 0,0,0,0,0,0,0,0, 0,0,124,130,122,130,126,126,
   0,0,0,0,0,0,0,0, 0,0,144,130,102,108,110,110, 0,0,0,0,0,0,0,0, 0,0,126,130,124,124,130,126,
   },
},

{ // SJS (6339.756M chars) [6]
  {NULL, NULL, NULL, NULL},
  151, 136, 55, 16, 129,
    {50,188,238,215,160,59,66,108, 192,200,192,194,200,199,202,200, 190,190,186,192,182,192,188,186, 101,96,109,95,94,95,102,113,
   84,100,117,108,117,135,81,77, 99,97,99,87,94,104,96,118, 120,115,111,101,96,105,108,108, 111,98,126,107,129,116,102,97,
   121,104,84,106,118,104,105,80, 97,98,110,102,121,108,114,99, 91,90,109,89,86,81,89,103, 106,103,118,100,101,114,121,115,
   114,112,106,111,106,104,109,112, 108,105,167,179,185,80,87,184, 83,79,20,45,22,55,39,0, 21,0,65,55,62,0,0,0,
   0,223,195,229,105,32,59,131, 61,180,189,185,180,185,191,185, 177,167,169,174,171,174,169,172, 163,80,84,78,90,80,80,85,
   59,108,81,89,100,121,105,17, 50,24,63,0,25,17,1,53, 95,61,66,50,0,31,31,81, 122,2,32,1,95,88,0,56,
   61,50,56,0,94,59,44,0, 38,84,50,0,106,63,0,18, 44,63,28,0,24,0,80,50, 56,88,38,24,1,93,98,91,
   113,91,95,105,97,102,91,93, 93,99,91,0,23,0,0,44, 0,76,0,0,7,42,0,0, 0,0,0,0,0,0,0,0,
   },
  {0,0,0,0,0,0,0,0, 0,15,79,0,0,65,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   113,69,98,19,0,8,69,67, 80,86,51,58,73,87,70,67, 54,73,78,62,54,54,44,46, 38,34,67,70,119,41,36,55,
   200,198,192,189,158,190,180,168, 171,181,178,171,176,161,191,184, 172,169,180,178,178,160,185,181, 191,169,175,200,180,158,184,172,
   177,159,190,176,163,179,174,191, 178,187,183,159,184,168,174,182, 179,167,178,179,178,180,188,176, 172,175,172,178,169,173,171,37,
   175,177,171,172,170,176,176,180, 168,182,188,188,179,179,172,172, 155,178,155,196,173,164,176,173, 162,162,164,156,168,174,177,175,
   179,175,195,159,185,163,180,172, 181,189,191,180,175,184,173,177, 166,185,174,185,165,192,178,192, 167,176,168,178,189,191,180,179,
   167,183,176,162,191,193,192,176, 191,193,177,160,203,189,170,166, 173,161,169,158,174,146,173,170, 162,160,175,166,189,175,165,174,
   186,163,175,177,160,176,185,183, 187,191,188,159,174,173,175,176, 193,187,161,164,151,172,158,144, 152,140,181,168,175,37,20,2,
   },
  {46,0,6,6,2,2,2,2, 2,2,2,2,2,2,2,2, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   128,0,126,120,8,12,14,18, 34,38,30,30,26,42,32,42, 128,0,128,126,6,10,10,14, 38,42,34,34,30,46,36,46,
   128,0,128,128,10,14,16,20, 52,56,48,46,42,60,50,60, 128,0,128,128,12,16,18,22, 52,56,48,46,42,60,50,60,
   94,0,52,54,2,2,2,2, 32,38,28,28,24,40,30,40, 128,0,128,128,46,50,52,58, 82,88,78,78,72,94,80,90,
   0,0,0,0,140,140,138,138, 132,130,132,130,134,132,132,128, 0,0,0,0,134,142,142,142, 130,136,134,134,124,134,132,142,
   214,0,248,242,110,114,102,98, 152,140,134,122,120,120,88,72, 182,0,226,234,144,134,80,86, 136,114,134,134,126,154,96,86,
   178,0,234,228,102,112,100,108, 140,126,126,128,112,160,72,74, 180,0,232,230,100,114,108,110, 138,130,120,146,128,142,50,56,
   0,0,0,0,134,146,130,142, 138,144,128,148,64,80,66,78, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   },
},

{ // EUC-JP (4368.914M chars) [7]
  {NULL, ced_hires_0, ced_hires_1, ced_hires_2, },
  202, 178, 27, 15, 128,
    {0,0,0,0,0,0,0,0, 0,0,0,0,0,0,173,100, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   0,228,186,180,229,225,122,123, 168,7,2,28,62,133,86,74, 195,190,202,195,193,195,191,198, 201,199,199,204,200,197,198,200,
   200,193,197,196,192,199,202,195, 189,196,201,190,197,197,191,183, 120,107,102,109,112,111,101,113, 107,110,96,107,105,115,111,124,
   116,109,107,105,105,141,147,137, 110,140,100,120,146,102,103,102, 105,112,105,133,84,0,0,0, 0,54,7,43,73,0,9,0,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   },
  {146,59,0,0,61,22,13,0, 60,71,0,0,80,26,61,28, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   83,27,41,0,46,51,35,0, 49,37,24,68,35,38,32,13, 131,42,28,35,34,12,0,0, 63,21,33,0,74,25,11,68,
   61,0,0,0,68,11,0,0, 31,1,1,0,39,0,101,78, 61,83,87,94,79,39,85,70, 76,64,19,86,75,87,77,75,
   72,46,73,78,49,96,75,69, 60,49,27,69,85,65,60,49, 74,36,61,72,76,65,80,67, 67,77,74,64,73,63,88,36,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   164,202,199,199,199,177,202,190, 185,187,195,193,192,191,183,196, 192,181,174,190,182,189,177,195, 190,197,177,183,211,189,184,194,
   186,192,177,195,187,176,192,194, 198,186,201,196,186,188,198,191, 190,186,182,176,189,182,193,190, 188,181,187,183,189,177,189,189,
   180,185,188,180,184,187,184,191, 191,188,197,192,194,179,192,186, 182,187,188,200,180,176,184,190, 174,170,179,160,193,182,193,31,
   },
  {0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   2,0,44,8,34,20,28,28, 0,0,0,0,0,0,0,0, 2,0,56,14,40,28,34,34, 0,0,0,0,0,0,0,0,
   2,0,60,16,42,30,36,38, 0,0,0,0,0,0,0,0, 92,0,128,128,128,128,128,128, 0,0,0,0,0,0,0,0,
   70,0,128,88,128,128,128,128, 0,0,0,0,0,0,0,0, 102,0,128,120,128,128,128,128, 0,0,0,0,0,0,0,0,
   0,0,146,186,190,162,182,180, 178,180,134,130,124,136,18,30, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   246,0,174,232,204,218,212,204, 0,0,131,131,129,123,123,117, 250,0,190,230,210,212,216,214, 0,0,125,121,125,131,131,133,
   250,0,192,232,204,216,202,214, 0,0,120,126,124,128,128,132, 244,0,172,236,206,224,218,216, 0,0,126,128,124,130,124,128,
   246,0,202,232,208,214,212,210, 0,0,144,110,104,110,108,108, 238,0,210,224,196,224,210,208, 0,0,118,114,104,116,104,152,
   },
},

{ // BIG5 (2431.102M chars) [8]
  {NULL, NULL, NULL, NULL},
  157, 174, 62, 10, 129,
    {0,0,0,0,0,0,0,0, 0,0,0,0,0,0,70,70, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   0,182,164,130,214,207,204,195, 202,198,207,192,194,201,196,187, 202,189,195,196,188,194,190,181, 194,188,192,185,188,188,191,185,
   184,179,185,186,180,189,110,108, 94,123,111,121,108,113,105,112, 116,103,101,106,110,102,114,99, 88,91,120,98,95,107,108,94,
   113,93,95,93,94,131,139,129, 121,130,93,111,138,87,98,87, 90,89,97,92,97,92,92,96, 102,147,84,82,84,87,88,107,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   0,216,162,127,212,204,205,192, 184,187,189,191,193,185,187,179, 188,190,183,192,177,189,189,198, 181,189,172,184,183,180,173,165,
   177,175,172,174,167,176,167,100, 42,127,129,92,89,80,107,94, 90,84,93,108,107,102,93,84, 105,100,83,90,94,79,92,95,
   84,84,90,92,77,89,103,113, 89,87,81,74,82,87,92,56, 77,96,70,88,72,107,102,81, 90,70,90,80,91,56,42,19,
   },
  {90,0,0,0,0,0,0,0, 0,0,0,0,0,4,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   77,44,72,0,0,0,44,0, 53,54,4,69,21,39,51,1, 78,36,31,5,0,4,0,0, 0,0,5,0,85,0,0,0,
   200,201,188,193,189,168,184,189, 192,178,176,174,183,184,182,191, 186,180,173,170,180,184,162,188, 182,171,166,172,168,185,178,176,
   173,183,178,161,180,180,169,177, 177,186,185,178,177,170,187,186, 184,188,165,193,177,186,185,182, 179,178,178,175,185,176,192,2,
   51,0,0,0,0,0,0,0, 0,0,0,19,0,0,0,14, 52,0,0,0,3,17,19,1, 0,0,0,0,0,0,0,0,
   157,190,169,191,185,175,165,184, 179,175,174,184,175,175,167,166, 181,177,178,189,170,179,183,169, 182,186,200,181,167,183,169,167,
   184,179,171,169,182,173,187,181, 182,184,182,175,183,177,188,186, 186,187,182,194,163,176,175,182, 180,172,184,181,179,176,173,180,
   175,175,178,172,178,181,187,170, 177,191,189,178,186,179,167,176, 165,169,173,182,190,175,181,181, 187,183,185,177,183,178,177,107,
   },
  {128,0,128,128,72,86,86,80, 128,128,128,128,128,128,128,128, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   48,0,42,44,2,2,2,2, 80,76,2,2,2,2,2,2, 54,0,52,52,2,2,2,2, 96,96,2,2,2,2,2,2,
   74,0,76,72,2,2,2,2, 128,128,2,2,2,2,2,2, 128,0,128,128,8,18,18,14, 128,128,28,26,26,26,26,28,
   128,0,128,128,2,4,4,2, 128,128,34,30,30,30,32,32, 128,0,128,128,4,14,12,10, 128,128,38,34,34,34,36,36,
   0,0,0,0,160,160,160,166, 0,0,160,122,152,116,140,114, 0,0,0,0,150,150,150,148, 0,0,148,150,150,150,150,148,
   0,0,0,0,142,138,138,134, 0,0,136,138,134,134,136,134, 0,0,0,0,132,138,142,146, 0,0,136,130,138,138,134,136,
   0,0,0,0,136,146,134,142, 0,0,126,134,134,130,144,140, 0,0,0,0,136,138,138,144, 0,0,142,144,128,136,130,124,
   0,0,0,0,144,140,134,134, 0,0,160,126,108,102,108,106, 224,128,202,216,130,148,132,136, 204,204,126,118,114,144,122,150,
   },
},

{ // Latin2 (315.882M chars) [9]
  {NULL, NULL, ced_hires_14, ced_hires_14, },
  90, 204, 45, 27, 127,
    {0,0,0,0,0,0,0,0, 0,13,13,0,0,13,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   176,178,79,184,121,96,188,129, 119,153,115,86,132,84,143,152, 125,178,73,182,114,95,188,79, 117,152,115,86,133,93,141,152,
   89,170,119,130,151,109,85,137, 171,147,170,100,153,155,102,118, 87,92,116,171,110,99,128,101, 181,128,158,80,133,126,117,98,
   90,170,94,124,153,103,95,127, 169,141,170,94,155,155,99,117, 81,92,116,171,113,99,148,82, 180,129,154,79,137,126,117,81,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   185,202,72,207,144,140,210,145, 119,178,153,122,170,124,172,196, 158,217,79,211,140,141,210,120, 115,176,151,152,187,124,173,200,
   97,198,155,156,200,127,165,183, 202,208,195,143,194,195,151,143, 149,174,157,197,159,157,199,120, 192,171,178,136,201,170,155,174,
   102,203,144,173,201,101,197,171, 203,203,208,143,202,204,139,150, 148,188,166,199,147,166,197,121, 192,191,178,144,189,182,155,121,
   },
  {65,0,0,0,0,0,0,0, 0,129,154,0,0,162,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   228,154,181,85,87,110,164,158, 144,155,125,102,191,153,189,148, 131,145,140,130,127,125,117,115, 113,114,177,135,201,88,123,155,
   81,200,180,210,197,203,168,188, 175,191,170,190,200,190,208,194, 182,141,204,199,200,181,182,186, 146,188,181,102,112,126,100,120,
   110,200,181,209,196,205,167,186, 172,191,169,190,200,189,207,194, 180,135,203,198,199,182,181,191, 143,189,181,91,125,72,102,0,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   176,176,79,167,122,89,163,118, 119,144,100,124,140,114,142,179, 121,178,71,168,109,88,163,67, 117,144,103,124,140,102,142,179,
   91,160,116,126,149,84,190,132, 166,138,143,91,83,185,105,126, 81,118,127,168,91,58,130,99, 171,124,126,82,122,95,120,153,
   93,161,107,127,150,87,190,124, 167,128,145,91,82,185,102,129, 83,119,127,170,121,59,134,85, 172,125,112,80,118,93,120,85,
   },
  {132,0,128,156,156,156,156,154, 0,0,156,156,156,156,156,156, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   10,0,2,2,2,2,2,2, 0,0,2,2,2,2,2,2, 14,0,2,2,2,2,2,2, 0,0,2,2,2,2,2,2,
   24,0,2,2,2,2,2,2, 0,0,2,2,2,2,2,2, 24,0,2,2,2,2,2,2, 0,0,2,2,2,2,2,2,
   24,0,2,2,2,2,2,2, 0,0,2,2,2,2,2,2, 24,0,2,2,2,2,2,2, 0,0,2,2,2,2,2,2,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   132,0,98,130,142,136,102,114, 0,0,172,130,168,162,108,142, 130,0,136,132,52,54,136,130, 0,0,108,174,82,90,170,166,
   98,0,90,98,141,143,100,90, 0,0,164,118,170,178,130,128, 100,0,92,100,139,143,108,110, 0,0,166,92,174,156,128,142,
   134,0,136,134,36,54,135,135, 0,0,114,168,90,144,168,180, 130,0,130,134,40,56,137,141, 0,0,102,170,80,158,172,156,
   },
},

{ // CP1251 (2609.249M chars) [10]
  {NULL, NULL, ced_hires_10, ced_hires_10, },
  190, 219, 69, 19, 128,
    {61,54,71,50,88,0,0,0, 0,0,73,0,84,68,72,51, 56,82,97,103,74,0,0,0, 0,0,69,0,81,63,70,32,
   175,85,84,108,0,54,0,0, 121,94,119,0,0,89,99,124, 0,0,150,148,47,70,0,0, 121,0,117,0,105,2,0,124,
   203,190,201,189,197,200,175,191, 205,172,201,198,196,207,207,203, 205,207,204,187,179,172,176,183, 172,169,147,176,170,168,160,171,
   201,185,197,186,194,200,174,188, 203,172,197,197,192,205,205,197, 203,204,203,185,172,167,174,181, 170,169,147,176,170,161,157,169,
   31,0,88,0,134,0,0,0, 85,0,28,0,0,0,5,0, 0,100,141,125,124,0,0,0, 1,0,16,0,11,11,33,0,
   184,2,95,21,0,6,0,0, 87,157,63,0,0,123,138,72, 0,0,102,143,7,102,0,0, 126,0,110,0,84,0,0,130,
   157,125,161,130,131,135,111,123, 148,122,140,123,137,137,153,133, 131,147,134,135,118,122,118,114, 110,103,83,118,114,116,116,143,
   193,147,189,166,165,195,131,156, 199,187,178,167,181,169,190,141, 170,175,180,173,120,175,145,137, 140,111,88,180,182,106,164,193,
   },
  {0,0,0,0,0,0,0,0, 0,123,152,0,0,158,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   223,147,180,67,83,115,163,151, 135,154,99,111,188,166,187,138, 128,140,138,123,122,123,115,111, 108,109,171,133,198,101,111,143,
   134,133,114,124,118,126,111,112, 113,129,111,108,118,121,114,117, 124,83,116,126,121,108,110,114, 76,103,91,97,102,121,55,113,
   81,143,106,111,119,137,102,102, 116,138,96,102,116,118,119,128, 122,71,118,150,131,126,116,96, 72,98,99,47,123,70,85,0,
   48,44,53,47,0,85,0,0, 0,0,66,0,81,17,64,0, 50,75,98,66,98,71,62,54, 0,60,67,96,81,29,65,16,
   175,84,89,98,74,0,54,89, 125,68,109,58,57,85,81,121, 74,34,155,158,0,74,29,81, 128,29,110,103,98,0,0,121,
   210,181,193,178,187,209,169,174, 206,177,192,194,187,202,211,177, 199,195,203,188,160,171,169,173, 162,156,142,188,185,132,168,185,
   213,184,195,179,188,212,171,176, 208,178,193,197,188,203,215,181, 203,197,205,192,162,172,170,174, 162,157,150,190,185,141,170,186,
   },
  {128,0,52,78,128,128,128,128, 128,128,74,82,12,26,10,22, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   42,0,2,6,48,58,46,48, 106,84,22,28,2,2,2,2, 56,0,2,20,64,76,62,64, 128,128,56,64,2,8,2,6,
   2,0,2,2,2,2,2,2, 128,110,26,34,2,2,2,2, 2,0,2,2,2,6,2,2, 128,128,42,50,2,2,2,2,
   2,0,2,2,2,2,2,2, 66,44,2,2,2,2,2,2, 2,0,2,2,2,8,2,2, 80,56,2,4,2,2,2,2,
   130,0,112,138,218,228,196,188, 184,166,182,184,138,132,110,112, 144,0,134,138,190,188,202,204, 196,226,158,170,124,124,124,126,
   174,0,136,170,182,184,172,174, 168,194,204,146,106,102,84,84, 154,0,152,148,150,166,162,138, 158,178,156,166,124,128,130,130,
   146,0,152,154,166,162,144,134, 134,86,98,126,139,139,112,110, 148,0,150,162,176,170,154,134, 130,104,96,128,139,141,112,108,
   150,0,154,148,132,128,120,110, 122,130,92,130,6,4,139,139, 150,0,154,152,96,132,122,106, 124,124,82,130,6,2,139,141,
   },
},

{ // CP1256 (4291.965M chars) [11]
  {NULL, NULL, NULL, NULL},
  175, 213, 75, 16, 129,
    {89,120,82,85,70,119,81,87, 78,57,0,61,52,107,87,0, 120,88,122,83,73,125,94,89, 114,96,1,85,34,116,73,0,
   173,142,76,85,128,71,110,132, 117,107,30,127,106,73,102,99, 133,84,73,90,108,82,96,130, 115,70,110,116,69,78,76,146,
   37,143,167,196,136,188,168,229, 204,148,208,168,191,193,183,197, 170,203,175,195,189,190,178,101, 181,157,204,175,195,197,193,197,
   97,209,116,216,198,191,209,110, 85,125,90,85,128,210,82,85, 126,99,116,154,99,137,142,85, 146,71,143,75,117,89,124,44,
   157,89,85,86,132,136,106,93, 83,69,0,77,102,72,53,0, 66,103,144,128,127,138,137,111, 67,109,0,87,78,86,35,10,
   182,172,102,152,141,111,120,142, 116,155,25,145,107,121,136,120, 155,111,121,104,137,100,101,159, 112,93,117,165,83,117,88,152,
   26,167,114,129,77,84,119,189, 178,200,192,163,167,160,159,182, 136,190,152,168,143,156,162,117, 154,133,178,121,152,172,165,172,
   181,188,141,190,186,184,180,168, 169,200,157,140,190,187,136,123, 149,117,121,139,145,135,137,118, 122,142,125,128,186,100,134,77,
   },
  {116,3,1,0,0,0,0,0, 0,134,143,0,0,157,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   228,134,177,78,100,111,160,151, 140,149,130,106,178,144,169,141, 137,152,147,136,134,132,126,124, 121,123,169,107,200,102,108,114,
   99,132,117,131,120,127,115,113, 116,121,112,109,121,123,118,119, 123,94,122,129,129,117,115,119, 115,108,101,110,97,133,114,122,
   120,160,152,167,156,164,144,156, 150,149,124,140,162,163,170,153, 151,130,177,173,168,136,148,104, 122,130,139,80,126,96,114,0,
   97,97,88,86,83,119,83,85, 98,64,79,60,61,80,88,47, 115,91,101,80,93,122,92,91, 105,117,46,102,80,115,80,37,
   173,163,81,85,125,72,114,122, 119,118,106,104,102,81,117,107, 133,87,73,88,113,82,96,129, 115,78,124,125,69,76,76,152,
   36,168,139,171,153,157,172,218, 198,200,201,168,187,192,186,204, 174,209,179,200,190,181,185,101, 182,157,199,169,196,190,195,192,
   104,226,105,204,209,198,208,118, 101,130,101,80,174,214,92,83, 163,120,130,171,118,155,158,79, 156,70,154,80,108,87,119,81,
   },
  {90,0,18,44,128,128,54,56, 102,84,40,60,2,2,2,34, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   42,0,2,2,48,48,10,14, 62,48,10,28,2,2,2,2, 62,0,2,22,72,74,30,34, 80,62,22,42,2,2,2,16,
   2,0,2,2,2,2,2,2, 36,24,2,6,2,2,2,2, 2,0,2,2,2,2,2,2, 48,34,2,16,2,2,2,2,
   2,0,2,2,2,2,2,2, 30,18,2,2,2,2,2,2, 48,0,2,8,54,56,16,20, 64,52,12,30,2,2,2,4,
   178,0,138,160,188,202,138,136, 246,218,168,188,116,124,126,114, 160,0,142,144,184,174,160,162, 212,248,162,204,118,126,126,120,
   168,0,140,164,178,172,140,138, 162,176,200,170,106,100,92,136, 172,0,144,162,174,166,138,138, 212,212,170,220,92,90,90,122,
   142,0,150,150,92,128,44,72, 114,114,114,108,128,130,134,130, 142,0,150,148,114,142,74,80, 126,116,112,110,134,132,130,132,
   140,0,148,142,112,124,154,154, 120,132,116,116,134,132,128,132, 140,0,132,132,126,136,172,174, 158,152,148,140,132,130,128,164,
   },
},

{ // CP1250 (456.295M chars) [12]
  {NULL, NULL, ced_hires_15, ced_hires_15, },
  90, 207, 44, 30, 128,
    {106,94,109,3,114,124,102,101, 0,71,177,95,143,118,167,98, 48,59,96,73,40,99,64,61, 0,67,141,59,108,83,130,60,
   178,82,81,175,131,134,115,136, 125,101,117,114,114,90,115,134, 133,104,75,173,117,99,105,135, 120,134,117,119,129,95,129,131,
   94,177,121,140,156,127,86,139, 173,151,161,106,170,163,105,122, 92,104,118,172,129,101,131,105, 183,159,162,83,135,145,120,103,
   95,177,123,127,157,116,103,130, 171,145,162,96,170,163,102,121, 83,93,119,172,131,103,150,86, 182,159,158,82,139,145,119,83,
   162,120,91,35,137,141,111,98, 13,75,196,81,168,151,193,131, 88,128,169,153,152,163,162,136, 10,134,194,112,166,184,194,139,
   187,92,75,209,146,163,125,147, 121,160,155,150,112,126,141,198, 160,116,81,213,142,104,106,164, 117,176,153,170,170,126,171,202,
   99,200,157,158,202,129,167,185, 204,210,197,145,196,197,153,145, 151,176,159,199,161,159,201,122, 194,173,180,138,203,172,157,176,
   104,205,146,175,203,103,199,173, 205,205,211,145,204,206,141,152, 150,191,168,201,150,168,199,123, 194,193,180,146,191,184,157,123,
   },
  {122,0,0,0,0,0,0,0, 0,143,154,0,0,165,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   227,153,180,86,97,112,169,159, 147,157,128,106,190,155,189,149, 138,150,147,134,133,129,124,123, 120,121,178,134,203,92,128,154,
   110,204,183,200,193,208,171,187, 178,196,172,193,197,190,210,197, 183,143,205,201,201,184,184,176, 150,190,178,114,114,132,107,122,
   115,204,184,198,191,209,169,185, 175,197,172,193,196,187,209,197, 181,137,202,200,201,186,184,176, 145,192,178,94,130,77,114,0,
   105,89,105,77,114,127,87,104, 65,77,167,87,121,154,174,103, 85,93,106,115,112,126,100,99, 71,123,167,110,121,155,174,104,
   177,116,81,166,127,131,117,124, 123,122,102,105,107,115,124,164, 136,109,72,166,119,93,103,132, 119,133,104,122,116,103,122,164,
   93,165,121,130,150,102,158,134, 169,143,144,96,85,190,109,129, 88,98,136,169,98,62,135,101, 172,130,135,83,124,100,121,154,
   95,167,111,128,151,90,158,126, 170,139,146,91,84,191,105,135, 89,99,137,172,123,63,135,88, 173,130,131,81,120,97,121,86,
   },
  {28,0,2,18,2,2,2,2, 14,14,14,22,4,16,2,16, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   22,0,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 32,0,2,2,2,2,2,2, 2,2,2,2,2,2,2,2,
   16,0,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 18,0,2,2,2,2,2,2, 2,2,2,2,2,2,2,2,
   16,0,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 18,0,2,2,2,2,2,2, 2,2,2,2,2,2,2,2,
   130,0,104,120,140,130,110,116, 162,132,114,116,180,146,140,112, 134,0,136,128,84,84,138,128, 108,162,124,148,120,118,180,146,
   140,0,112,138,140,134,98,104, 108,128,184,142,144,178,120,158, 136,0,132,138,94,84,138,132, 116,142,142,178,124,144,154,188,
   94,0,90,96,141,143,100,90, 180,114,154,138,170,172,130,124, 96,0,94,98,139,143,108,112, 170,106,170,116,174,152,128,136,
   130,0,138,132,36,52,135,135, 114,180,122,162,92,140,170,174, 128,0,130,132,40,56,137,141, 94,170,112,180,80,154,172,150,
   },
},

{ // Latin5 (322.539M chars) [13]
  {NULL, NULL, ced_hires_18, ced_hires_18, },
  96, 232, 51, 21, 128,
    {20,0,20,20,20,20,20,20, 20,37,37,20,20,16,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   174,125,94,100,128,93,114,133, 120,100,115,113,111,97,114,106, 129,100,95,102,119,100,101,130, 117,91,117,117,87,95,93,116,
   109,114,123,142,149,117,104,183, 111,140,112,106,104,116,108,107, 147,123,109,109,99,127,169,104, 100,88,100,105,175,189,165,102,
   108,113,104,109,151,116,97,192, 108,128,102,97,101,120,103,120, 178,123,114,107,105,128,165,98, 101,92,102,87,175,201,186,105,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   183,147,103,153,143,113,121,144, 117,156,143,147,109,122,138,121, 157,113,123,105,138,101,102,161, 114,95,164,166,85,118,91,158,
   153,197,154,170,198,186,170,182, 181,207,170,141,118,193,149,136, 201,175,130,196,157,155,197,118, 180,109,177,140,200,218,211,173,
   183,202,142,172,199,196,170,169, 171,201,158,141,149,203,138,125, 202,175,150,198,146,156,196,119, 180,143,177,129,187,238,211,108,
   },
  {50,6,11,57,0,0,0,0, 0,144,149,0,0,164,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   226,146,185,81,100,113,168,166, 142,154,126,112,190,161,183,145, 134,151,147,137,134,132,126,124, 121,122,176,137,206,105,128,149,
   92,194,178,190,185,190,170,185, 176,197,160,189,202,193,206,184, 174,152,207,197,198,181,179,170, 146,175,180,111,108,128,102,138,
   127,196,180,195,187,192,172,183, 173,199,158,205,212,204,220,183, 177,140,211,202,199,186,178,170, 143,189,200,94,140,80,111,5,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   175,126,100,100,127,94,116,124, 121,125,106,108,106,120,123,111, 127,113,88,113,118,96,111,128, 119,95,113,121,111,107,92,126,
   118,110,119,175,148,113,108,168, 110,141,110,109,101,124,114,104, 171,129,114,115,94,154,141,101, 100,92,102,106,132,171,177,152,
   114,115,112,174,149,118,107,171, 109,124,108,108,100,125,114,97, 190,129,114,113,120,155,143,91, 105,92,113,100,137,200,199,107,
   },
  {130,0,102,144,144,142,108,138, 0,0,154,154,154,154,154,154, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   36,0,2,2,2,2,2,2, 0,0,14,38,8,2,8,2, 62,0,2,22,2,10,2,4, 0,0,20,44,12,6,12,2,
   26,0,2,2,2,2,2,2, 0,0,2,4,2,2,2,2, 14,0,2,2,2,2,2,2, 0,0,2,2,2,2,2,2,
   20,0,2,2,2,2,2,2, 0,0,2,2,2,2,2,2, 4,0,2,2,2,2,2,2, 0,0,2,2,2,2,2,2,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   164,0,136,160,116,116,104,106, 0,0,202,178,144,126,130,118, 160,0,142,150,118,124,102,106, 0,0,180,246,168,156,154,124,
   90,0,92,92,145,147,90,92, 0,0,156,176,188,168,138,134, 112,0,112,116,141,141,114,116, 0,0,110,138,166,180,166,148,
   132,0,138,122,44,54,131,135, 0,0,130,156,124,128,182,162, 128,0,132,132,50,60,135,133, 0,0,112,120,100,130,142,168,
   },
},

{ // ISO-8859-11 (489.481M chars) [14]
  {NULL, NULL, NULL, NULL},
  184, 198, 48, 20, 127,
    {0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   175,209,195,88,202,83,128,203, 192,152,191,169,88,166,132,138, 156,134,149,172,200,200,176,200, 176,211,199,194,179,150,192,164,
   175,205,201,213,136,199,75,202, 176,173,203,200,140,209,144,123, 197,203,215,179,198,198,169,189, 190,187,93,39,18,43,4,72,
   210,196,186,186,192,106,151,183, 207,206,151,138,179,120,113,126, 122,127,132,111,120,111,102,99, 96,94,33,88,83,68,64,77,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   184,175,143,82,159,80,91,182, 155,114,141,116,87,135,98,96, 126,88,93,153,172,160,139,156, 134,188,176,156,101,92,163,122,
   113,183,179,173,84,159,74,169, 155,135,151,108,92,170,98,140, 166,90,180,144,149,166,80,79, 133,137,84,1,4,52,40,119,
   116,99,97,96,100,92,161,101, 174,168,108,110,179,87,92,109, 117,117,114,109,105,106,99,96, 94,95,20,89,93,21,54,98,
   },
  {70,0,0,46,0,0,0,0, 0,136,148,0,0,160,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   207,129,181,82,82,93,164,157, 152,152,114,111,166,150,188,146, 128,141,136,126,124,123,115,111, 109,109,165,100,202,96,118,126,
   79,118,108,118,109,116,106,106, 104,104,108,101,111,116,105,100, 115,76,111,119,110,92,99,104, 73,98,81,125,114,120,90,106,
   61,105,100,99,109,106,92,82, 92,104,93,95,95,106,105,107, 114,58,104,111,104,93,100,89, 80,71,96,55,132,63,99,0,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   175,206,190,91,199,78,140,206, 190,148,188,166,92,168,152,144, 148,139,143,179,199,196,177,200, 172,213,199,192,172,143,191,162,
   175,208,201,213,153,202,62,203, 176,170,200,196,121,209,145,145, 194,206,218,188,201,202,172,189, 193,191,94,64,59,63,57,74,
   203,186,178,183,187,119,161,181, 208,206,150,139,187,118,121,123, 127,115,118,111,108,125,108,109, 106,112,71,99,90,69,77,91,
   },
  {128,0,148,148,148,148,148,148, 0,0,148,148,148,148,148,148, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   2,0,2,2,8,8,16,14, 0,0,2,2,2,2,2,36, 2,0,2,2,4,4,14,10, 0,0,2,2,2,2,2,34,
   2,0,2,2,2,2,8,4, 0,0,2,2,2,2,2,32, 2,0,2,2,4,4,14,10, 0,0,2,2,2,2,2,44,
   2,0,2,2,6,6,14,10, 0,0,2,2,2,2,2,42, 68,0,20,30,128,126,128,112, 0,0,16,12,6,8,14,128,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   166,0,158,164,182,174,180,178, 0,0,126,118,128,132,130,78, 158,0,162,160,132,152,142,144, 0,0,120,122,126,134,130,92,
   158,0,162,160,128,160,132,146, 0,0,126,126,124,134,128,90, 166,0,162,160,142,146,146,156, 0,0,134,134,128,68,132,84,
   162,0,162,160,138,154,148,150, 0,0,130,134,132,114,112,128, 158,0,164,148,196,208,204,216, 0,0,84,76,64,52,134,254,
   },
},

{ // ISO-8859-15 (27.581M chars) [15]
  {NULL, NULL, ced_hires_21, ced_hires_21, },
  86, 217, 37, 21, 127,
    {0,0,0,0,0,0,0,0, 0,85,85,0,0,85,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   187,137,107,112,127,105,104,142, 106,111,127,124,123,101,126,118, 133,112,107,116,117,113,113,142, 111,105,127,127,92,89,91,126,
   121,138,130,159,161,129,133,190, 123,152,123,118,116,144,119,121, 126,135,122,132,111,139,144,115, 113,103,117,117,145,120,135,114,
   121,137,113,131,163,128,131,190, 121,141,112,111,114,143,115,133, 123,135,125,131,110,140,160,110, 114,105,118,101,148,119,129,118,
   0,0,0,0,0,0,0,0, 0,34,33,0,0,33,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   195,159,116,166,180,125,125,156, 145,168,155,159,121,135,150,134, 169,125,135,118,145,114,115,173, 125,108,176,178,138,141,110,170,
   165,209,166,183,211,198,182,194, 193,219,182,153,130,205,161,148, 160,187,142,208,169,167,209,131, 192,122,189,152,212,181,158,185,
   195,214,155,184,212,208,182,181, 183,213,170,154,161,215,150,137, 175,187,162,210,158,168,208,131, 192,156,189,142,199,193,155,120,
   },
  {23,0,0,70,0,0,0,0, 0,145,155,0,0,165,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   226,142,175,96,115,116,169,162, 150,154,125,112,184,159,177,144, 147,165,159,150,148,149,139,138, 134,136,166,128,201,103,116,143,
   104,197,188,202,195,190,180,196, 185,179,172,189,202,198,215,195, 185,164,216,206,204,173,190,182, 158,170,181,116,107,135,114,133,
   130,194,189,197,192,189,178,194, 182,176,171,189,201,194,213,194, 181,151,213,203,201,172,189,182, 155,176,182,107,136,90,117,0,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   187,138,113,113,149,107,108,134, 123,135,117,120,118,129,131,122, 135,125,101,126,120,109,123,140, 113,107,124,133,118,119,101,138,
   132,128,129,187,160,126,122,145, 123,152,121,121,113,136,125,116, 152,141,127,129,106,167,144,114, 113,104,118,117,134,117,122,164,
   130,132,121,187,161,130,119,138, 122,137,119,120,112,137,123,109, 153,141,126,130,124,167,145,104, 116,104,130,109,131,116,130,120,
   },
  {178,0,128,156,122,126,124,128, 0,0,168,202,168,202,168,202, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   22,0,2,2,2,2,2,2, 0,0,2,14,2,2,2,2, 50,0,2,18,2,2,2,2, 0,0,2,20,2,2,2,6,
   16,0,2,2,2,2,2,2, 0,0,2,2,2,2,2,2, 42,0,2,8,2,2,2,2, 0,0,2,2,2,2,2,2,
   16,0,2,2,2,2,2,2, 0,0,2,2,2,2,2,2, 38,0,2,4,2,2,2,2, 0,0,2,2,2,2,2,2,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   160,0,138,164,108,108,102,104, 0,0,190,160,132,134,120,146, 158,0,144,154,112,118,106,110, 0,0,160,218,160,164,148,160,
   88,0,92,94,139,139,94,92, 0,0,152,166,184,176,134,148, 98,0,96,102,139,139,110,110, 0,0,138,172,174,190,154,164,
   128,0,138,126,40,48,135,135, 0,0,130,160,128,148,186,180, 120,0,126,120,48,54,139,141, 0,0,136,160,142,188,166,188,
   },
},

{ // CP1257 (41.264M chars) [16]
  {NULL, NULL, ced_hires_20, ced_hires_20, },
  84, 222, 39, 20, 128,
    {112,108,100,0,132,119,103,89, 0,78,64,75,0,77,68,67, 11,35,79,71,21,82,53,35, 0,31,45,33,0,0,49,0,
   173,72,85,94,121,69,110,128, 74,94,81,110,109,101,104,75, 124,97,89,88,110,94,97,129, 74,80,81,113,82,88,85,80,
   132,139,163,137,149,121,106,164, 124,141,114,153,127,139,154,153, 181,121,165,109,89,101,125,103, 97,82,78,145,112,85,162,98,
   138,140,163,106,150,118,115,164, 124,135,112,155,125,133,155,153, 179,115,165,109,88,104,145,100, 130,82,79,145,129,84,161,94,
   157,114,85,13,131,136,105,92, 1,81,72,77,10,83,100,70, 81,122,164,148,146,158,157,131, 0,129,85,106,48,81,114,1,
   181,85,101,152,141,118,119,142, 101,154,98,145,107,120,136,92, 155,111,121,103,136,99,100,159, 111,92,101,164,84,116,89,89,
   171,189,211,72,196,184,158,199, 199,205,85,210,168,152,203,176, 211,170,179,194,91,153,195,116, 172,97,91,199,198,89,204,171,
   205,199,217,92,198,194,183,201, 200,199,104,216,166,147,205,177, 211,185,178,196,93,154,194,117, 220,101,91,199,185,90,202,118,
   },
  {116,0,0,0,0,0,0,0, 0,139,155,0,0,164,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   228,153,188,77,92,107,206,155, 138,165,122,102,192,146,189,135, 135,150,145,135,132,131,124,123, 120,121,178,143,202,85,105,154,
   107,193,189,191,196,191,164,188, 169,204,198,197,197,192,203,181, 177,134,209,211,206,182,187,168, 142,176,171,109,99,129,101,112,
   100,195,190,187,196,195,162,186, 165,207,198,197,197,192,202,181, 176,126,208,211,206,184,192,168, 136,178,172,83,124,77,109,0,
   117,90,127,119,115,124,85,99, 57,71,0,89,81,86,68,85, 80,86,101,138,134,121,102,96, 36,92,81,109,21,74,91,19,
   173,101,111,88,123,70,112,120, 89,109,84,101,109,115,119,115, 133,107,98,98,116,107,100,128, 113,93,107,121,96,107,110,117,
   136,143,168,99,146,122,112,156, 159,136,103,157,139,159,155,146, 175,101,144,111,83,103,128,97, 149,82,75,140,115,76,159,150,
   138,144,169,93,147,125,112,157, 159,131,104,158,139,160,159,147, 175,102,144,111,84,103,122,91, 149,83,76,143,108,75,158,101,
   },
  {100,0,60,88,62,66,62,66, 128,122,100,126,86,90,84,92, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   38,0,2,2,2,2,2,2, 52,46,20,44,6,12,6,14, 70,0,6,34,6,12,6,12, 64,56,32,54,18,22,16,24,
   26,0,2,2,2,2,2,2, 4,2,2,2,2,2,2,2, 22,0,2,2,2,2,2,2, 8,4,2,2,2,2,2,2,
   26,0,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 24,0,2,2,2,2,2,2, 4,2,2,2,2,2,2,2,
   168,0,132,156,116,128,102,104, 234,200,166,204,168,184,164,158, 152,0,138,142,110,104,124,126, 210,216,166,230,168,168,172,154,
   164,0,130,166,110,108,106,106, 148,164,204,170,130,130,130,128, 164,0,140,158,102,98,106,104, 204,214,178,244,158,162,146,152,
   102,0,88,106,141,141,94,98, 190,154,154,162,178,188,140,126, 102,0,90,98,141,139,114,106, 152,140,118,176,188,174,148,132,
   132,0,134,134,26,40,135,135, 176,186,144,164,96,138,176,190, 126,0,136,122,28,38,135,133, 140,172,130,130,114,154,186,176,
   },
},

{ // CP1255 (313.575M chars) [17]
  {NULL, NULL, NULL, ced_hires_12, },
  192, 233, 81, 15, 127,
    {98,94,106,111,72,121,83,91, 81,65,26,68,0,71,0,69, 81,85,102,93,99,127,102,95, 75,97,17,87,61,82,71,67,
   175,124,97,90,58,73,104,127, 119,93,86,110,111,82,101,112, 129,87,94,110,112,86,97,132, 116,73,60,114,71,123,75,120,
   119,65,141,119,118,109,112,113, 121,112,61,74,117,67,73,62, 61,98,78,56,68,54,124,148, 38,49,42,32,35,40,36,27,
   211,211,193,200,214,221,187,200, 193,220,109,200,214,125,214,131, 205,198,202,108,201,91,192,199, 210,210,202,32,36,81,108,32,
   159,117,88,89,134,138,108,95, 85,73,57,78,26,89,0,88, 63,104,145,129,128,139,138,112, 83,110,64,88,33,78,35,6,
   184,147,104,154,150,113,122,144, 118,156,77,147,109,123,138,122, 157,113,123,106,139,101,103,161, 114,95,71,167,86,119,91,159,
   81,40,33,27,57,50,53,76, 89,84,40,25,97,25,54,51, 36,79,53,49,25,25,27,56, 26,26,27,24,26,39,25,39,
   181,180,170,178,203,186,155,176, 171,197,173,144,189,203,159,191, 133,169,170,166,142,163,146,172, 192,178,205,35,59,102,136,39,
   },
  {118,0,0,0,0,0,0,0, 0,137,146,0,0,165,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   227,155,187,100,97,112,172,178, 136,158,128,132,188,170,183,153, 137,155,149,140,137,135,129,128, 124,127,173,136,203,112,110,154,
   108,138,122,146,128,136,120,118, 124,127,114,112,126,128,125,124, 133,137,129,135,139,118,124,121, 117,118,103,111,143,128,106,131,
   142,144,112,120,123,140,110,110, 119,135,106,107,129,123,126,129, 124,108,122,153,134,129,118,103, 84,104,101,80,145,91,116,0,
   106,72,108,107,84,120,79,80, 98,59,0,68,0,76,0,66, 101,102,115,98,109,128,101,107, 92,118,68,108,103,102,97,114,
   175,126,108,86,95,75,110,121, 124,112,102,101,106,84,113,116, 133,82,132,131,119,84,97,130, 116,74,64,119,72,123,75,135,
   128,83,129,119,124,114,117,127, 127,114,43,82,134,59,75,65, 44,112,90,59,50,58,80,109, 50,44,39,43,41,44,40,43,
   205,205,192,202,209,223,186,199, 192,223,176,195,211,204,206,190, 204,195,200,171,199,162,190,198, 213,203,213,43,41,82,94,46,
   },
  {86,0,44,70,130,132,130,122, 110,94,68,88,90,132,2,10, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   36,0,2,2,40,40,40,40, 60,46,16,34,34,72,2,2, 62,0,2,26,74,72,72,70, 70,54,24,42,44,82,2,2,
   60,0,2,22,70,70,68,68, 128,128,100,128,128,128,26,36, 62,0,2,26,66,66,66,68, 128,128,128,128,128,128,48,62,
   2,0,2,2,2,2,2,2, 24,10,2,2,2,34,2,2, 2,0,2,2,2,2,2,2, 34,20,2,10,10,46,2,2,
   170,0,136,156,182,190,170,166, 248,224,180,222,232,172,106,66, 154,0,140,142,176,164,188,188, 214,246,176,224,176,218,104,106,
   166,0,132,164,178,170,168,166, 160,170,204,170,154,182,76,76, 162,0,140,154,182,180,172,168, 224,230,176,228,214,218,94,74,
   172,0,148,146,180,182,188,180, 236,188,168,220,190,202,120,120, 158,0,146,158,180,180,180,178, 162,232,182,208,174,184,68,66,
   146,0,150,148,106,132,122,126, 110,90,74,98,128,90,131,133, 142,0,150,144,92,132,120,128, 78,90,74,72,124,138,133,127,
   },
},

{ // KOI8R (315.553M chars) [18]
  {NULL, NULL, ced_hires_9, ced_hires_9, },
  189, 220, 69, 17, 128,
    {17,17,17,17,17,17,17,17, 17,39,39,17,17,39,17,17, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2,
   17,17,17,117,17,17,17,17, 17,17,17,17,17,17,17,17, 17,17,17,118,17,25,17,17, 17,17,17,25,17,17,17,47,
   155,199,183,172,193,200,169,185, 168,203,166,197,197,191,205,205, 196,169,201,203,201,186,173,196, 170,177,187,171,168,166,181,137,
   158,201,188,174,195,200,175,189, 171,204,166,200,198,196,207,207, 201,170,204,206,202,187,175,199, 170,177,189,174,172,166,183,137,
   7,7,7,7,7,7,7,7, 7,39,39,7,7,39,7,7, 7,7,7,7,7,7,7,7, 7,7,7,7,7,7,7,7,
   7,7,7,123,7,7,7,7, 7,7,7,7,7,7,7,7, 7,7,7,74,7,7,7,7, 7,7,7,7,7,7,7,100,
   164,191,135,141,166,194,118,163, 175,198,187,177,169,182,165,191, 138,192,164,174,180,171,125,189, 179,178,152,139,98,108,139,73,
   131,162,130,110,132,145,129,147, 126,158,123,145,139,148,148,152, 142,148,128,154,139,149,108,167, 114,119,123,105,122,90,109,85,
   },
  {0,0,0,0,0,0,0,0, 0,111,171,0,0,149,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   223,146,172,30,25,90,164,138, 116,148,90,87,187,167,194,124, 72,106,101,90,90,85,93,77, 75,75,165,131,188,82,114,147,
   125,92,36,71,49,93,30,0, 64,104,0,30,49,47,61,97, 110,0,62,58,56,11,54,71, 45,83,11,70,85,112,54,106,
   92,94,57,74,51,94,36,30, 66,106,0,34,57,53,67,98, 113,0,67,61,51,49,56,17, 49,84,41,0,111,57,57,0,
   47,49,56,30,0,36,0,0, 0,0,0,25,17,0,0,11, 0,36,17,0,0,0,0,0, 0,0,49,0,0,11,0,0,
   25,0,0,123,0,0,0,0, 0,0,0,0,0,0,0,0, 57,17,45,121,30,57,11,36, 66,11,36,41,36,49,58,53,
   169,212,182,169,188,212,160,181, 172,207,177,191,196,188,203,214, 181,185,201,197,203,191,170,194, 184,189,174,162,139,157,176,118,
   167,209,180,168,186,208,159,179, 171,206,177,190,193,187,201,211, 178,185,198,195,202,188,168,192, 184,187,173,162,133,156,176,117,
   },
  {130,0,102,154,154,154,154,154, 154,154,154,154,112,154,116,154, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   92,0,38,74,128,128,128,128, 128,128,128,128,22,34,24,36, 92,0,38,74,128,128,128,128, 128,128,128,128,44,54,46,56,
   2,0,2,2,18,22,26,26, 110,128,40,40,2,2,2,2, 2,0,2,2,22,28,32,30, 128,128,48,50,2,2,2,2,
   2,0,2,2,16,20,24,24, 128,128,76,80,2,2,2,2, 2,0,2,2,20,24,30,28, 128,128,82,84,2,2,2,2,
   154,0,128,178,178,178,178,178, 178,178,178,178,128,178,140,178, 154,0,132,178,178,178,178,178, 178,178,178,178,138,178,178,178,
   146,0,154,136,178,182,180,178, 178,178,180,178,134,146,82,92, 148,0,152,168,180,184,180,178, 178,180,180,180,90,106,136,148,
   154,0,154,152,154,146,152,150, 134,154,138,86,141,141,6,4, 154,0,154,156,114,150,158,160, 166,172,142,122,139,141,2,12,
   136,0,154,156,180,184,184,158, 124,140,110,136,112,106,141,139, 140,0,152,158,180,186,178,172, 130,144,112,138,114,110,139,141,
   },
},

{ // GBK (106.219M chars) [19]
  {NULL, NULL, NULL, NULL},
  203, 189, 27, 17, 128,
    {80,136,138,128,141,121,130,133, 125,121,114,108,141,130,70,75, 125,127,117,120,119,135,127,109, 133,129,119,130,122,125,128,114,
   74,204,125,202,219,215,119,114, 118,156,113,86,97,79,82,81, 189,196,199,195,195,208,196,199, 196,203,194,200,201,197,193,195,
   192,195,189,194,195,183,196,191, 201,197,207,197,197,197,201,199, 201,191,203,201,198,196,202,200, 134,116,114,132,122,124,118,115,
   115,120,116,120,141,121,124,123, 121,120,121,113,125,116,117,110, 117,114,112,113,119,112,124,123, 85,77,84,80,72,84,79,0,
   135,112,99,113,112,124,117,103, 124,95,92,89,122,95,0,51, 0,0,0,0,0,0,0,0, 25,1,0,0,0,0,0,0,
   1,113,69,80,84,69,59,13, 116,104,61,70,84,69,73,47, 74,88,74,72,72,73,61,73, 51,54,54,59,70,77,75,75,
   96,68,68,66,84,83,47,68, 64,64,44,78,54,61,26,26, 64,64,47,59,70,74,70,66, 64,80,61,44,44,69,66,82,
   51,57,66,57,64,47,64,54, 18,82,68,66,40,66,70,69, 51,70,40,88,37,37,40,59, 59,40,102,70,73,70,68,0,
   },
  {0,0,0,0,0,0,0,0, 0,1,60,0,0,64,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   94,78,82,0,0,0,93,107, 64,60,46,107,76,56,82,44, 29,64,73,39,27,27,18,14, 14,15,18,0,111,60,0,46,
   104,95,102,121,118,73,105,75, 96,131,74,109,89,100,103,111, 93,108,98,107,126,103,101,125, 113,99,98,90,89,109,120,88,
   77,85,85,92,91,104,101,109, 101,83,74,90,111,98,98,117, 96,93,127,97,106,100,91,99, 91,102,105,84,114,93,91,17,
   124,109,99,116,90,100,105,89, 96,95,103,108,93,117,111,110, 98,115,123,116,94,82,112,111, 83,97,106,96,74,105,115,121,
   126,196,197,192,196,176,185,184, 195,185,196,194,198,187,184,191, 190,190,180,189,189,194,181,191, 181,191,188,196,189,191,187,192,
   186,174,192,193,202,189,193,194, 190,188,189,195,181,187,196,191, 200,178,184,186,189,186,192,180, 193,176,195,182,185,179,182,186,
   185,187,190,185,183,187,185,186, 177,190,189,191,187,187,181,180, 178,185,188,196,173,177,186,192, 192,182,197,186,177,187,185,0,
   },
  {144,0,168,168,120,122,152,128, 124,116,2,2,2,2,4,4, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   78,0,26,38,2,2,12,8, 128,128,22,22,20,24,26,26, 72,0,20,32,2,2,6,2, 128,128,40,42,40,44,46,46,
   74,0,22,34,2,2,6,4, 128,128,40,42,38,44,46,46, 80,0,28,40,6,4,14,10, 128,128,44,46,42,50,52,50,
   128,0,128,128,98,102,124,110, 128,128,46,50,46,54,56,54, 128,0,128,128,128,128,128,128, 128,128,38,40,38,44,46,44,
   0,0,0,0,236,252,250,242, 230,226,122,106,114,124,130,128, 0,0,0,0,252,238,254,254, 240,234,112,116,106,116,144,130,
   0,0,0,0,242,220,220,232, 106,108,136,130,128,116,126,114, 0,0,0,0,206,208,218,216, 82,88,128,126,128,126,126,132,
   0,0,0,0,204,218,220,208, 84,82,122,128,128,128,132,132, 0,0,0,0,212,214,210,210, 92,88,124,128,126,136,120,126,
   0,0,0,0,208,210,204,206, 200,208,136,124,124,122,126,126, 0,0,0,0,198,202,200,218, 200,196,128,122,132,128,130,120,
   },
},

{ // Greek (109.816M chars) [20]
  {NULL, NULL, ced_hires_11, ced_hires_11, },
  186, 219, 72, 19, 128,
    {0,0,0,0,0,0,0,0, 0,24,24,0,0,24,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   174,64,123,80,13,49,101,123, 114,89,0,152,107,83,76,53, 122,81,71,65,105,84,180,126, 183,174,186,111,183,56,165,168,
   117,206,178,191,194,206,163,186, 183,198,202,195,198,199,165,203, 201,196,56,205,210,185,180,183, 154,177,136,87,180,183,177,186,
   157,202,171,187,188,204,165,188, 183,199,199,194,197,198,161,202, 194,196,154,201,208,185,175,180, 147,179,136,79,183,166,168,0,
   0,0,0,0,0,0,0,0, 0,34,34,0,0,34,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   183,58,94,153,91,63,120,143, 117,155,71,146,108,122,78,71, 156,112,122,105,105,84,83,160, 96,97,84,166,98,118,76,75,
   70,162,129,130,139,152,103,165, 133,144,142,123,130,144,98,158, 139,113,63,159,134,140,122,126, 98,117,81,46,175,143,180,165,
   64,199,125,122,122,176,111,189, 115,196,146,142,141,197,113,188, 137,142,206,133,145,191,115,124, 78,148,115,74,181,161,152,93,
   },
  {98,0,0,0,0,0,0,0, 2,135,160,0,67,161,42,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   225,142,174,72,92,107,173,153, 133,152,109,98,183,147,185,142, 132,150,145,135,133,131,124,123, 119,121,167,138,195,117,108,120,
   78,122,108,129,112,120,111,107, 109,113,108,105,115,117,111,121, 118,81,119,121,112,95,108,105, 80,100,85,102,97,121,98,109,
   109,116,101,104,112,116,102,95, 104,115,100,100,121,112,116,124, 116,72,112,117,112,104,110,87, 82,90,98,64,123,66,101,0,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   174,42,84,78,172,159,104,116, 116,102,153,94,101,82,171,60, 122,76,68,71,118,78,185,127, 180,182,189,143,179,67,177,172,
   111,207,161,179,175,201,159,197, 167,203,190,188,186,197,160,205, 189,198,0,199,200,190,174,173, 145,183,137,110,188,186,184,191,
   66,211,163,182,177,206,160,199, 172,206,192,193,189,200,165,208, 196,203,194,200,201,194,176,176, 146,185,138,110,182,181,174,0,
   },
  {146,0,130,170,170,170,170,168, 0,0,166,170,136,168,130,154, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   34,0,2,6,52,58,56,62, 0,0,4,2,2,2,2,2, 4,0,2,2,22,28,24,32, 0,0,14,2,2,2,2,2,
   2,0,2,2,2,2,2,4, 0,0,6,2,2,2,2,2, 2,0,2,2,2,6,2,8, 0,0,2,2,2,2,2,2,
   2,0,2,2,2,2,2,6, 0,0,2,2,2,2,2,2, 2,0,2,2,2,10,6,12, 0,0,2,2,2,2,2,2,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   166,0,136,172,188,188,186,188, 0,0,186,116,110,106,106,106, 164,0,146,164,180,178,180,174, 0,0,98,100,140,144,98,104,
   146,0,152,154,164,164,162,156, 0,0,52,142,139,137,118,112, 148,0,152,154,150,144,142,134, 0,0,48,136,139,131,120,126,
   150,0,152,146,92,128,132,126, 0,0,144,72,30,124,139,139, 150,0,152,150,88,124,136,128, 0,0,66,76,4,118,141,139,
   },
},

{ // JIS (138.804M chars) [21]
  {NULL, NULL, NULL, NULL},
  37, 154, 2, 1, 129,
    {0,0,0,0,0,0,0,0, 0,99,99,0,0,99,107,197, 0,0,0,0,0,0,0,0, 0,0,0,254,0,0,0,0,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   },
  {119,0,0,0,0,0,0,0, 0,53,49,0,0,49,0,0, 0,0,0,0,0,0,0,0, 0,0,0,197,0,0,0,0,
   73,56,65,75,246,65,49,49, 243,61,49,49,49,49,49,49, 49,56,61,49,49,49,49,49, 49,49,56,56,49,49,49,49,
   49,49,49,49,49,49,49,49, 49,49,49,49,49,56,143,49, 49,49,97,49,75,49,59,49, 49,49,61,49,49,53,56,53,
   49,98,49,53,49,49,49,94, 49,49,49,49,63,69,74,49, 49,49,53,65,49,49,49,49, 49,49,49,49,49,56,53,0,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   },
  {130,184,30,216,132,172,164,208, 0,0,0,0,0,0,0,0, 128,2,128,38,128,122,124,104, 0,0,0,0,0,0,0,0,
   128,128,128,128,128,128,128,128, 0,0,0,0,0,0,0,0, 128,128,128,128,128,128,128,128, 0,0,0,0,0,0,0,0,
   128,128,128,128,128,128,128,128, 0,0,0,0,0,0,0,0, 128,128,128,128,128,128,128,128, 0,0,0,0,0,0,0,0,
   128,128,128,128,128,128,128,128, 0,0,0,0,0,0,0,0, 128,128,128,128,128,128,128,128, 0,0,0,0,0,0,0,0,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   },
},

{ // CP1254 (20.130M chars) [22]
  {NULL, NULL, ced_hires_18, ced_hires_18, },
  97, 228, 51, 26, 128,
    {105,102,117,110,111,122,102,106, 97,93,168,92,91,102,30,86, 53,83,126,96,72,96,70,66, 56,74,134,61,61,50,82,84,
   173,124,96,100,127,93,113,131, 120,100,114,112,110,97,113,106, 131,100,95,101,117,100,100,131, 116,92,116,116,90,96,94,114,
   108,154,124,147,148,125,114,181, 110,139,111,106,104,133,107,106, 146,124,109,108,100,126,167,105, 99,95,111,104,173,187,163,102,
   108,154,121,114,150,117,103,190, 108,128,103,98,103,136,103,119, 177,122,113,108,105,127,163,98, 101,94,113,91,173,200,184,105,
   157,114,89,88,132,136,106,94, 88,90,191,83,103,92,21,83, 80,123,164,148,146,158,157,131, 101,129,189,106,99,97,60,118,
   182,145,102,152,141,111,120,142, 116,154,142,145,107,121,136,120, 155,111,121,104,137,100,101,159, 112,95,162,165,88,117,91,157,
   152,195,152,169,197,184,168,180, 179,205,169,140,116,192,148,134, 200,173,128,194,156,153,196,117, 179,108,175,138,198,216,209,171,
   181,200,141,171,198,195,168,167, 169,200,157,140,147,201,136,123, 200,173,148,196,144,154,194,118, 178,142,175,128,185,237,209,106,
   },
  {116,4,4,52,4,0,0,0, 0,144,148,0,0,163,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   225,145,184,86,101,112,168,165, 141,153,125,111,189,160,182,143, 136,151,146,136,133,132,125,124, 121,122,175,136,205,105,128,148,
   108,194,176,188,184,192,169,184, 174,197,159,189,200,191,205,183, 174,151,205,195,198,180,178,169, 146,174,178,113,109,127,105,136,
   126,195,178,194,186,194,171,181, 172,199,157,204,211,202,219,182, 177,139,210,201,200,185,177,168, 141,187,198,94,139,85,111,0,
   103,94,114,122,104,131,103,108, 103,87,157,89,94,91,51,79, 84,99,153,107,139,122,104,101, 94,118,157,108,103,94,118,122,
   173,126,101,99,126,94,115,123, 121,124,114,109,106,119,122,111, 133,112,90,112,118,97,110,129, 118,94,113,120,110,107,93,127,
   117,118,123,173,147,114,112,166, 112,142,110,109,107,167,114,104, 170,127,113,114,95,153,141,101, 99,92,103,107,131,170,176,150,
   116,123,113,173,147,118,110,169, 112,133,111,107,99,168,113,98, 188,128,113,112,119,153,142,91, 104,92,117,99,135,199,197,107,
   },
  {84,0,50,70,50,58,42,52, 90,82,80,100,68,64,68,44, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   36,0,2,2,2,2,2,2, 32,24,16,38,6,2,6,2, 62,0,2,22,2,10,2,4, 36,28,22,44,12,8,10,2,
   26,0,2,2,2,2,2,2, 2,2,2,4,2,2,2,2, 16,0,2,2,2,2,2,2, 2,2,2,2,2,2,2,2,
   20,0,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 4,0,2,2,2,2,2,2, 2,2,2,2,2,2,2,2,
   140,0,112,126,144,140,106,118, 180,152,152,160,192,130,148,104, 142,0,134,130,100,98,134,132, 138,168,142,178,148,152,184,152,
   164,0,136,162,116,118,104,106, 140,152,204,176,140,126,126,118, 160,0,144,152,118,124,102,106, 174,182,180,238,162,154,150,126,
   92,0,94,92,145,149,90,92, 190,154,156,172,184,168,132,134, 112,0,114,118,143,143,116,118, 140,146,112,138,166,182,162,148,
   132,0,138,124,44,56,131,135, 142,176,130,156,120,128,176,162, 128,0,132,132,50,60,137,135, 130,160,114,122,98,130,138,168,
   },
},

{ // CP1253 (37.682M chars) [23]
  {NULL, NULL, ced_hires_11, ced_hires_11, },
  186, 218, 73, 21, 128,
    {98,79,82,91,78,122,85,90, 0,82,68,76,0,75,0,65, 79,105,101,115,101,127,107,94, 0,98,0,91,0,78,48,0,
   174,70,182,88,122,76,110,130, 119,95,70,153,108,88,101,71, 130,87,79,77,106,197,107,129, 184,175,187,114,184,75,166,169,
   118,206,179,192,195,207,164,187, 184,199,203,195,198,200,165,203, 202,197,70,205,210,185,180,184, 155,178,137,90,181,184,178,187,
   86,203,172,188,189,203,161,188, 181,199,199,194,198,198,162,203, 195,196,156,202,208,186,176,181, 148,179,137,85,184,167,169,70,
   158,116,88,89,133,138,107,95, 0,84,72,80,9,91,0,87, 58,101,143,127,125,137,136,110, 0,108,60,84,28,75,32,6,
   183,68,95,153,143,112,121,144, 117,156,66,147,109,122,138,78, 157,112,123,105,106,101,102,161, 98,99,87,166,100,118,81,80,
   77,163,130,131,140,152,105,166, 133,145,142,124,131,145,100,159, 140,114,66,160,135,141,123,127, 100,118,86,68,176,144,181,166,
   74,200,125,123,122,177,112,190, 115,197,147,142,142,198,114,189, 137,143,206,134,146,192,116,125, 82,149,116,80,182,162,153,66,
   },
  {118,0,0,0,0,0,0,0, 0,138,161,0,0,163,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   226,143,175,80,96,111,174,154, 134,154,123,101,184,149,186,143, 136,152,147,137,135,133,126,125, 122,123,168,139,197,118,114,121,
   107,133,118,132,121,129,117,114, 119,125,113,112,122,124,119,128, 125,97,124,130,131,115,120,120, 116,111,103,108,101,126,102,112,
   111,143,112,125,126,141,110,108, 120,135,105,107,128,122,124,134, 122,86,123,151,133,128,120,103, 90,103,102,77,126,78,111,0,
   91,79,78,82,79,127,85,84, 0,77,0,78,58,76,0,63, 76,87,132,84,113,123,101,97, 0,101,76,105,55,86,59,12,
   175,74,185,85,123,84,113,121, 119,111,74,101,104,87,118,74, 133,87,79,85,119,144,95,130, 181,183,190,144,180,79,178,172,
   112,208,162,180,176,202,160,198, 168,204,190,189,187,198,160,206, 190,199,70,200,200,190,175,174, 145,183,138,111,190,189,185,193,
   80,212,167,183,177,208,161,200, 173,208,193,194,190,201,165,209, 197,204,194,200,202,195,177,177, 147,186,139,111,184,182,175,72,
   },
  {120,0,80,110,184,184,160,166, 142,128,86,68,44,50,40,48, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   20,0,2,2,32,36,28,32, 64,50,2,2,2,2,2,2, 2,0,2,2,10,14,6,10, 78,62,14,2,2,2,2,2,
   2,0,2,2,2,2,2,2, 70,56,6,2,2,2,2,2, 2,0,2,2,2,2,2,2, 58,46,2,2,2,2,2,2,
   2,0,2,2,2,2,2,2, 32,20,2,2,2,2,2,2, 2,0,2,2,2,2,2,2, 38,26,2,2,2,2,2,2,
   168,0,138,162,188,202,172,168, 248,222,164,132,100,104,104,108, 150,0,142,148,180,174,188,192, 212,242,158,156,116,114,118,120,
   162,0,136,170,180,178,174,172, 148,160,176,108,132,134,106,104, 162,0,146,164,172,168,166,156, 118,142,100,98,134,138,126,114,
   144,0,152,152,156,152,144,136, 116,120,132,142,139,135,116,112, 146,0,150,152,142,134,126,118, 106,104,132,134,139,131,120,126,
   148,0,152,144,90,118,116,110, 122,122,68,86,30,122,137,139, 150,0,152,148,90,116,122,112, 122,132,66,88,12,118,139,137,
   },
},

{ // CP932 (5.390M chars) [24]
  {NULL, NULL, NULL, NULL},
  151, 140, 55, 26, 129,
    {73,188,237,214,159,83,84,109, 191,199,191,194,200,199,202,199, 190,190,186,192,182,191,188,186, 105,102,112,102,102,102,107,115,
   101,107,119,112,119,135,100,99, 106,106,107,101,103,109,105,120, 121,117,114,106,104,110,111,112, 114,105,127,110,129,118,108,105,
   123,109,100,111,120,109,109,99, 105,106,114,108,122,112,116,106, 102,102,113,101,100,99,102,108, 110,108,119,107,108,116,122,117,
   117,116,111,115,112,109,113,115, 113,111,167,178,184,104,106,183, 160,142,115,156,149,130,163,163, 184,191,154,133,75,97,94,93,
   0,223,195,229,107,12,47,131, 44,180,189,184,179,185,191,185, 177,166,168,174,170,173,169,172, 162,86,89,86,93,87,87,91,
   99,113,100,104,106,122,111,98, 99,99,99,99,99,99,99,99, 105,99,100,99,98,99,99,102, 124,99,99,99,106,103,99,99,
   100,99,99,98,105,99,99,99, 99,101,99,99,112,99,98,99, 99,100,99,99,99,98,102,99, 99,103,99,99,99,104,106,104,
   115,99,101,108,103,106,99,99, 99,103,97,0,23,91,96,0, 105,107,95,165,148,101,154,130, 96,175,132,116,100,109,98,101,
   },
  {87,0,0,0,0,0,0,0, 0,84,91,0,0,87,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   114,86,99,84,84,84,86,86, 90,93,85,89,87,94,87,86, 91,88,89,86,85,85,84,84, 84,84,85,87,119,84,84,84,
   199,198,192,189,158,189,180,168, 171,180,178,171,176,161,191,183, 172,169,179,177,178,160,185,181, 190,169,175,200,180,158,185,172,
   177,159,190,175,162,179,173,191, 177,187,182,158,184,167,174,182, 179,167,177,179,178,179,187,176, 174,176,173,178,169,174,176,0,
   175,177,175,172,170,176,176,180, 169,182,188,188,178,179,172,172, 159,180,158,195,174,169,176,176, 162,162,166,158,168,174,176,177,
   179,175,195,161,185,163,180,172, 181,189,190,180,175,184,173,177, 166,185,175,184,166,192,178,192, 166,176,169,178,190,190,180,180,
   168,183,176,163,192,193,192,176, 191,192,177,162,203,189,171,167, 173,162,169,159,174,148,173,170, 163,160,175,166,188,175,165,174,
   186,163,176,177,162,176,185,182, 187,190,188,161,175,174,179,176, 192,187,162,164,156,172,160,145, 155,145,181,167,175,93,93,93,
   },
  {84,0,82,82,2,2,4,6, 2,2,8,8,4,18,2,2, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   128,0,126,120,6,10,10,14, 22,24,18,18,14,28,20,30, 128,0,128,126,2,8,8,12, 22,26,18,18,14,30,20,30,
   128,0,128,128,6,10,12,14, 26,28,22,22,18,32,24,34, 128,0,128,128,8,12,14,16, 26,30,22,22,18,32,24,34,
   68,0,44,44,2,2,2,2, 26,30,24,22,18,34,24,34, 66,0,40,42,2,2,2,2, 2,2,2,2,2,2,2,2,
   0,0,0,0,140,138,140,138, 132,130,132,132,136,132,134,128, 0,0,0,0,134,140,142,142, 130,134,132,134,124,134,132,142,
   222,128,252,248,124,128,128,130, 148,136,134,126,124,132,86,120, 214,128,240,244,136,132,130,132, 136,118,136,132,126,152,84,118,
   212,128,244,242,126,130,132,134, 136,128,130,128,118,156,82,122, 212,128,242,242,126,130,132,134, 136,130,128,144,130,142,66,122,
   0,0,0,0,134,144,136,142, 138,144,128,150,72,88,76,86, 184,180,164,164,126,140,126,154, 134,144,126,132,124,128,136,134,
   },
},

{ // Hebrew (10.753M chars) [25]
  {NULL, NULL, NULL, NULL},
  196, 235, 78, 9, 128,
    {0,0,0,0,0,0,0,0, 0,63,63,0,0,63,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   165,0,80,80,80,80,80,80, 80,82,80,80,82,89,80,80, 85,80,80,80,86,80,80,82, 80,80,83,80,80,80,80,0,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,80,
   205,203,189,201,216,218,185,196, 190,220,185,190,212,212,202,200, 199,194,199,175,193,168,187,196, 214,202,217,0,0,0,0,0,
   0,0,0,0,0,0,0,0, 0,77,77,0,0,77,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   145,0,87,87,105,87,87,87, 88,138,88,88,88,89,105,87, 89,88,88,87,91,87,87,118, 87,87,87,88,88,89,87,0,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,87,
   194,196,170,172,206,189,164,179, 163,185,130,182,196,152,197,145, 174,174,179,130,174,124,162,174, 179,192,183,0,0,0,0,0,
   },
  {0,0,0,0,0,0,0,0, 0,136,147,0,0,157,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   227,138,183,80,80,95,170,157, 104,163,103,95,150,163,151,133, 86,103,112,90,86,90,87,88, 85,86,126,96,204,95,106,111,
   82,84,81,84,82,82,81,81, 82,84,81,81,81,81,82,82, 82,80,85,94,81,81,80,82, 81,80,82,83,104,130,83,93,
   114,87,82,84,83,83,82,81, 82,88,80,81,82,82,82,86, 83,81,84,85,88,82,81,85, 80,82,80,81,92,108,97,0,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   165,0,81,80,80,80,80,81, 80,82,80,80,81,81,80,80, 86,80,81,80,85,80,80,83, 80,80,81,80,80,80,80,0,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,80,
   209,210,192,200,210,223,187,198, 189,223,131,200,211,161,212,142, 207,193,203,135,198,112,191,196, 208,208,204,0,0,0,0,0,
   },
  {180,0,128,152,204,204,204,202, 0,0,202,202,0,164,118,124, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   54,0,2,14,128,84,104,102, 0,0,66,128,0,128,2,2, 128,0,36,58,128,128,128,128, 0,0,98,128,0,128,4,16,
   128,0,128,128,128,128,128,128, 0,0,128,128,0,128,128,128, 128,0,76,98,128,128,128,128, 0,0,128,128,0,128,50,62,
   2,0,2,2,16,2,8,10, 0,0,2,38,0,78,2,2, 2,0,2,2,26,10,18,20, 0,0,6,56,0,98,2,2,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   154,0,146,158,230,204,230,232, 0,0,186,226,0,186,128,130, 186,0,128,164,228,226,226,226, 0,0,226,226,0,186,118,124,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 164,0,128,152,188,188,188,186, 0,0,186,186,0,148,116,124,
   150,0,150,150,136,142,142,136, 0,0,130,118,0,118,130,132, 154,0,150,154,146,158,148,158, 0,0,124,124,0,122,132,128,
   },
},

{ // KOI8U (14.358M chars) [26]
  {NULL, NULL, ced_hires_9, ced_hires_9, },
  189, 220, 69, 18, 129,
    {87,87,87,87,87,87,87,87, 87,95,95,87,87,95,87,87, 84,84,84,84,84,84,84,84, 84,84,84,84,84,84,84,84,
   87,87,87,119,129,87,162,137, 87,87,87,87,87,90,87,87, 87,87,87,119,131,87,163,137, 87,87,87,87,87,92,87,87,
   156,198,183,173,193,200,169,185, 168,203,166,197,197,191,205,205, 196,169,201,203,201,185,173,196, 170,177,187,171,168,165,181,137,
   158,200,188,175,195,200,175,189, 171,204,166,200,198,196,207,207, 201,170,204,206,202,187,175,199, 170,177,189,174,172,166,183,137,
   82,82,82,82,82,82,82,82, 82,95,95,82,82,95,82,82, 56,56,56,56,56,56,56,56, 56,56,56,56,56,56,56,56,
   82,82,82,124,131,82,157,142, 82,82,82,82,82,83,82,82, 82,82,82,87,89,82,115,97, 82,82,82,82,82,82,82,103,
   164,191,135,141,166,194,119,163, 175,198,187,177,169,181,165,191, 138,192,163,174,180,171,125,189, 179,178,152,139,101,110,139,87,
   131,161,130,111,132,145,129,147, 126,157,123,145,139,148,148,152, 142,148,128,154,139,149,110,167, 115,120,124,107,123,95,110,92,
   },
  {0,0,0,0,0,0,0,0, 0,112,170,0,0,149,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   223,146,172,86,86,96,164,138, 117,148,96,95,187,167,194,124, 88,108,104,96,97,94,99,91, 91,91,165,131,188,92,116,147,
   126,98,86,90,87,99,86,86, 87,107,86,86,86,87,87,101, 112,86,87,87,86,86,87,91, 86,93,86,87,94,113,86,107,
   97,99,87,90,86,99,86,86, 87,108,86,86,87,87,88,102, 114,86,88,87,86,86,87,86, 86,94,86,86,113,86,87,0,
   87,87,87,86,86,86,86,86, 86,86,86,86,86,86,86,86, 86,86,86,86,86,86,86,86, 86,86,86,86,86,86,86,86,
   86,86,86,123,128,86,170,136, 86,86,86,86,86,87,86,86, 88,86,87,121,127,88,168,135, 88,86,86,87,87,87,86,87,
   169,212,182,169,188,211,160,181, 172,207,177,191,196,188,203,214, 181,185,201,197,203,191,170,194, 184,189,174,162,139,157,176,119,
   167,209,179,168,186,208,159,179, 171,205,177,190,193,187,202,210, 178,185,198,195,202,188,168,192, 184,187,173,162,133,156,176,118,
   },
  {132,0,98,138,200,200,200,198, 200,200,174,176,92,102,94,104, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   44,0,2,24,84,92,96,96, 128,128,46,48,2,2,2,2, 42,0,2,24,84,90,96,94, 128,128,88,90,14,24,16,26,
   2,0,2,2,10,14,16,16, 36,36,2,2,2,2,2,2, 2,0,2,2,16,20,22,22, 46,46,2,2,2,2,2,2,
   2,0,2,2,8,12,14,14, 128,128,26,28,2,2,2,2, 2,0,2,2,14,16,20,18, 112,128,30,32,2,2,2,2,
   174,0,126,164,224,224,224,222, 224,224,180,182,112,124,116,126, 200,0,128,168,224,224,224,222, 224,224,182,184,114,126,118,128,
   146,0,154,150,184,194,200,194, 224,224,170,142,134,140,78,88, 158,0,142,170,224,224,224,222, 224,224,140,172,104,106,136,142,
   156,0,154,152,148,140,142,142, 118,120,138,74,139,139,16,24, 154,0,154,156,130,146,148,152, 126,124,138,84,139,141,18,30,
   136,0,154,156,180,186,186,174, 116,118,108,138,112,106,141,139, 140,0,152,158,182,184,182,178, 122,122,112,138,114,110,139,141,
   },
},

{ // ISO-8859-5 (4.566M chars) [27]
  {NULL, NULL, NULL, NULL},
  178, 203, 63, 18, 128,
    {0,0,0,0,0,0,0,0, 0,73,73,0,0,73,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   174,139,153,105,121,99,146,124, 153,123,130,127,103,100,117,115, 203,190,202,193,196,200,173,190, 205,165,202,200,198,206,204,202,
   204,206,200,185,176,172,175,186, 177,166,129,174,167,168,175,170, 198,179,195,186,191,199,170,186, 202,165,197,197,190,203,202,194,
   202,202,199,184,167,167,172,183, 170,164,142,174,169,160,162,170, 145,139,152,104,117,99,146,124, 151,121,129,126,103,100,117,115,
   0,0,0,0,0,0,0,0, 0,96,96,0,0,96,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   183,107,100,119,101,102,114,101, 108,107,102,105,110,123,100,105, 155,132,162,147,135,132,126,125, 144,118,136,128,135,134,143,136,
   129,148,131,134,127,123,119,121, 119,113,118,126,116,118,119,138, 196,133,191,156,166,192,126,148, 197,189,175,160,180,171,185,142,
   170,174,180,171,119,174,145,137, 136,107,110,179,181,109,162,193, 112,129,101,118,119,103,147,123, 126,105,102,131,111,99,125,102,
   },
  {0,0,0,0,0,0,0,0, 0,114,146,0,0,157,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   222,138,169,99,103,106,165,146, 132,152,105,105,185,162,182,129, 128,138,133,123,122,122,115,115, 112,113,166,133,204,103,107,138,
   107,127,114,123,114,133,112,117, 112,112,116,112,118,118,113,127, 126,101,118,122,116,107,107,118, 101,108,104,104,106,112,100,115,
   103,127,109,118,116,128,106,107, 105,117,110,111,115,113,118,136, 126,100,117,119,115,110,109,117, 100,113,108,99,124,100,99,0,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   174,147,138,136,115,100,153,120, 139,118,125,128,109,100,117,138, 209,174,192,178,187,205,163,176, 205,178,190,191,185,200,208,172,
   198,193,199,188,157,168,168,173, 166,150,130,185,182,133,166,183, 215,178,196,181,189,211,165,178, 208,178,192,200,187,202,213,176,
   204,195,202,197,156,173,167,173, 161,154,133,189,185,135,171,185, 146,148,138,136,116,99,155,119, 139,120,128,128,109,103,118,137,
   },
  {176,0,130,152,216,216,216,214, 0,0,182,122,134,118,130,216, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   36,0,2,2,42,50,44,50, 0,0,16,2,2,2,2,24, 2,0,2,2,2,2,2,2, 0,0,18,2,2,2,2,26,
   2,0,2,2,2,4,2,4, 0,0,32,2,2,2,2,42, 2,0,2,2,2,2,2,2, 0,0,2,2,2,2,2,2,
   2,0,2,2,2,8,2,8, 0,0,2,2,2,2,2,2, 46,0,2,4,54,64,54,64, 0,0,38,2,2,2,2,48,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   178,0,132,168,188,188,180,186, 0,0,192,126,130,94,98,138, 140,0,150,154,182,178,186,182, 0,0,122,138,138,120,116,102,
   146,0,146,160,198,200,184,190, 0,0,126,138,140,118,114,104, 148,0,154,154,116,130,140,130, 0,0,78,28,40,138,138,134,
   156,0,154,148,128,138,132,134, 0,0,88,38,50,138,140,144, 150,0,152,140,186,198,198,204, 0,0,142,88,100,130,142,182,
   },
},

{ // CP874 (90.889M chars) [28]
  {NULL, NULL, NULL, NULL},
  183, 197, 49, 21, 127,
    {105,102,97,58,82,121,0,91, 87,94,89,61,69,81,0,58, 84,99,96,131,98,112,94,97, 77,90,89,80,56,80,84,0,
   175,209,195,89,202,83,128,203, 192,151,191,169,87,166,132,138, 156,134,149,172,200,200,176,200, 176,211,199,193,179,149,192,164,
   175,205,201,213,136,199,77,202, 175,173,203,200,140,209,144,122, 197,203,215,179,198,198,169,189, 190,187,93,58,56,58,53,74,
   209,196,186,186,191,106,151,183, 207,205,151,138,179,120,113,126, 122,127,132,111,120,111,102,99, 97,95,58,89,84,72,69,79,
   159,116,43,30,27,138,0,65, 1,58,62,0,19,89,0,87, 48,92,134,118,116,128,127,100, 0,39,50,0,17,66,20,0,
   184,175,143,83,159,81,91,181, 155,114,141,116,87,135,98,96, 126,89,93,153,172,160,139,156, 133,188,176,156,101,93,162,122,
   113,182,178,173,85,158,76,169, 155,135,151,108,93,170,98,140, 166,90,179,144,149,166,81,80, 132,137,85,52,53,62,58,119,
   116,99,98,96,101,93,161,101, 173,168,107,110,179,86,93,109, 117,116,114,109,105,106,99,96, 94,95,54,89,94,56,64,98,
   },
  {119,0,0,44,0,0,0,0, 0,138,149,0,0,161,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   209,129,181,84,86,94,166,157, 152,153,115,111,167,151,188,147, 133,142,138,128,126,125,117,113, 111,112,165,104,202,96,121,126,
   106,131,115,123,116,126,111,111, 114,120,111,105,118,121,113,118, 121,94,117,127,130,114,114,118, 116,109,100,126,115,121,92,108,
   78,142,110,112,119,136,104,98, 116,133,97,101,116,118,116,127, 119,79,119,150,131,126,114,102, 84,97,98,64,134,70,101,0,
   99,94,75,63,80,121,0,92, 94,84,92,59,78,81,49,49, 77,82,103,84,123,111,96,101, 80,100,87,78,55,86,81,0,
   175,206,190,92,199,79,139,206, 190,148,188,166,94,168,152,144, 148,139,143,179,199,195,177,200, 172,213,199,192,172,143,191,161,
   174,207,201,213,153,202,68,203, 176,170,200,196,121,209,144,145, 194,206,217,188,201,202,172,189, 193,191,94,68,66,68,64,77,
   203,186,177,183,187,119,161,181, 208,206,149,139,187,118,121,123, 127,115,118,111,108,125,108,109, 106,112,74,99,91,72,79,91,
   },
  {110,0,78,92,172,172,158,156, 114,122,40,36,30,34,38,118, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   2,0,2,2,2,2,2,2, 54,48,2,2,2,2,2,36, 2,0,2,2,2,2,2,2, 54,46,2,2,2,2,2,34,
   2,0,2,2,2,2,2,2, 54,46,2,2,2,2,2,34, 2,0,2,2,2,2,2,2, 64,58,2,2,2,2,2,44,
   2,0,2,2,2,2,2,2, 62,56,2,2,2,2,2,42, 66,0,18,30,108,104,86,80, 128,128,16,12,6,8,14,128,
   174,0,152,160,182,202,168,162, 246,218,114,92,92,80,144,174, 156,0,156,144,186,176,196,194, 204,246,128,124,120,72,138,184,
   164,0,158,164,172,166,158,160, 106,118,126,118,128,132,130,82, 156,0,162,160,124,144,118,126, 140,136,120,122,126,134,130,94,
   156,0,162,160,120,152,110,128, 92,114,126,124,126,134,128,90, 164,0,160,162,132,138,124,138, 88,116,134,134,128,66,132,86,
   160,0,160,160,130,146,126,132, 98,116,132,134,134,114,112,128, 156,0,164,148,202,212,190,188, 168,192,88,80,70,70,134,254,
   },
},

{ // ISO-8859-13 (0.207M chars) [29]
  {NULL, NULL, ced_hires_20, ced_hires_20, },
  87, 221, 44, 20, 128,
    {0,0,0,0,0,0,0,0, 0,140,140,0,0,140,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   172,141,141,141,142,142,141,144, 141,141,141,141,141,141,141,141, 142,141,141,141,141,141,141,144, 141,141,141,142,141,141,141,141,
   145,148,162,142,153,142,141,165, 141,149,141,156,144,147,157,156, 180,141,166,141,141,141,143,141, 141,141,141,151,142,141,162,141,
   145,149,162,141,154,141,141,165, 142,147,141,156,144,146,157,156, 178,141,165,141,141,141,151,141, 142,141,141,151,145,141,162,141,
   0,0,0,0,0,0,0,0, 0,103,103,0,0,103,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   179,147,141,155,148,163,142,148, 141,157,141,149,141,141,147,141, 156,141,143,141,164,141,141,160, 142,141,141,164,141,141,141,141,
   171,188,209,141,195,182,160,197, 197,203,141,208,168,155,202,175, 209,170,178,192,141,156,194,142, 172,141,141,197,196,141,202,170,
   203,197,216,141,196,193,181,199, 198,198,141,215,167,153,203,176, 209,184,177,194,141,156,192,141, 218,141,141,197,184,141,201,147,
   },
  {0,0,0,0,0,0,0,0, 0,147,156,0,0,163,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   226,155,187,141,141,141,205,156, 147,165,143,141,191,150,188,145, 145,153,151,146,146,145,143,143, 142,143,177,149,200,141,141,157,
   141,192,188,189,194,190,165,186, 169,203,196,195,196,191,201,180, 177,147,207,210,204,181,186,168, 149,175,171,141,141,143,141,141,
   141,193,189,186,194,193,163,185, 166,205,196,195,195,191,200,180, 175,144,206,209,204,182,190,168, 147,177,171,141,143,141,141,0,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   172,142,142,141,143,141,142,142, 141,142,141,141,142,141,142,143, 144,141,141,141,144,142,141,144, 142,141,142,143,141,142,142,143,
   146,151,168,141,152,142,142,159, 160,146,141,160,148,161,157,152, 174,141,150,141,141,141,143,141, 154,141,141,149,142,141,160,154,
   147,151,168,141,152,142,142,159, 161,143,141,160,148,161,160,152, 174,141,150,141,141,141,143,141, 154,141,141,150,142,141,160,141,
   },
  {158,0,124,150,126,130,126,128, 0,0,170,174,162,166,162,166, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   24,0,2,2,2,2,2,2, 0,0,2,6,2,2,2,2, 30,0,2,2,2,2,2,2, 0,0,6,12,2,2,2,4,
   18,0,2,2,2,2,2,2, 0,0,2,2,2,2,2,2, 16,0,2,2,2,2,2,2, 0,0,2,2,2,2,2,2,
   18,0,2,2,2,2,2,2, 0,0,2,2,2,2,2,2, 16,0,2,2,2,2,2,2, 0,0,2,2,2,2,2,2,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   162,0,130,160,124,122,116,118, 0,0,184,168,156,160,156,160, 160,0,136,152,118,120,118,120, 0,0,168,176,162,164,162,166,
   112,0,92,110,143,143,98,102, 0,0,160,164,168,178,152,154, 116,0,94,110,143,141,116,110, 0,0,156,162,178,166,152,152,
   130,0,134,134,68,72,135,137, 0,0,158,164,152,156,168,178, 126,0,136,124,74,78,137,135, 0,0,156,162,150,158,178,166,
   },
},

{ // Latin4 (1.274M chars) [30]
  {NULL, NULL, ced_hires_17, ced_hires_17, },
  82, 215, 39, 25, 128,
    {0,0,0,0,0,0,0,0, 0,115,115,0,0,115,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   173,117,116,117,126,117,148,132, 125,177,162,132,116,116,164,120, 128,118,116,117,124,117,148,116, 124,175,162,130,116,116,163,116,
   157,149,123,151,148,122,117,138, 167,143,123,118,151,148,118,153, 121,162,122,139,120,119,129,118, 117,116,148,118,124,121,142,117,
   157,149,118,127,151,122,117,137, 166,138,123,117,150,149,118,153, 120,162,122,131,121,119,146,117, 118,128,146,116,134,116,142,117,
   0,0,0,0,0,0,0,0, 0,70,70,0,0,70,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   181,199,116,118,142,116,174,142, 124,175,196,171,116,126,169,127, 155,213,117,118,137,116,173,125, 122,173,197,168,117,142,170,118,
   207,195,152,169,196,183,168,186, 199,205,192,140,210,191,148,201, 147,178,118,155,156,154,195,123, 178,174,175,140,198,117,198,171,
   213,200,142,170,197,194,168,195, 200,199,205,141,216,201,138,202, 146,177,121,146,145,154,194,125, 178,220,175,131,185,119,198,126,
   },
  {57,0,0,38,0,0,0,0, 0,131,151,0,0,166,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   228,152,180,117,117,121,211,153, 140,167,128,117,189,148,187,138, 132,143,139,132,130,129,124,123, 122,123,176,138,202,117,118,154,
   118,186,190,201,200,189,167,191, 171,184,194,192,197,190,201,180, 181,140,208,211,206,171,184,129, 140,156,180,118,122,126,119,124,
   127,186,190,200,200,192,165,190, 168,185,194,192,197,189,200,179, 179,135,207,211,206,172,186,130, 136,162,180,118,128,117,119,0,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   173,117,117,117,128,123,146,126, 126,173,155,140,117,124,156,121, 126,119,117,117,121,134,147,117, 125,173,155,141,136,134,156,135,
   164,151,124,120,147,123,118,143, 164,136,118,120,153,166,122,151, 121,140,124,152,118,120,129,119, 117,154,119,117,123,117,140,151,
   164,153,120,117,148,124,119,144, 165,128,118,120,153,167,121,156, 125,142,122,153,127,119,128,121, 118,154,121,117,120,117,144,118,
   },
  {162,0,122,152,126,130,126,128, 0,0,168,174,170,184,168,186, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   20,0,2,2,2,2,2,2, 0,0,2,2,2,2,2,2, 24,0,2,2,2,2,2,2, 0,0,2,2,2,2,2,2,
   26,0,2,2,2,2,2,2, 0,0,2,2,2,2,2,2, 36,0,2,2,2,2,2,2, 0,0,2,2,2,2,2,2,
   26,0,2,2,2,2,2,2, 0,0,2,2,2,2,2,2, 36,0,2,2,2,2,2,2, 0,0,2,2,2,2,2,2,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   142,0,104,138,144,136,100,96, 0,0,180,140,178,180,148,160, 132,0,136,126,68,68,138,130, 0,0,140,176,140,152,182,186,
   108,0,90,112,143,143,98,98, 0,0,176,164,182,164,150,154, 118,0,100,112,139,145,112,108, 0,0,180,154,176,174,158,170,
   134,0,132,134,46,52,137,137, 0,0,142,182,140,160,182,166, 120,0,140,126,56,60,127,133, 0,0,152,184,148,180,174,174,
   },
},

{ // MACINTOSH (7.890M chars) [31]
  {NULL, NULL, NULL, NULL},
  93, 176, 57, 40, 128,
    {157,149,165,157,149,138,123,160, 152,165,162,148,156,168,162,155, 132,145,140,141,135,131,120,139, 145,132,114,87,138,116,126,107,
   122,121,109,120,124,153,124,137, 115,116,118,128,124,104,106,113, 104,117,104,104,109,107,139,107, 126,104,109,117,116,104,105,110,
   111,159,167,120,108,142,122,128, 139,154,202,125,151,121,112,145, 122,135,125,128,127,169,111,105, 105,105,106,131,106,115,157,151,
   178,165,177,166,176,175,150,161, 175,138,169,172,167,178,180,169, 181,177,176,161,145,151,149,162, 150,150,134,154,149,146,136,108,
   187,160,176,222,183,180,186,200, 209,171,190,175,171,176,227,198, 178,155,199,157,162,155,181,198, 159,170,176,133,176,169,177,184,
   153,174,115,162,149,204,131,171, 181,164,183,160,143,100,144,155, 100,123,132,132,125,113,123,102, 131,114,100,143,166,100,142,153,
   167,156,133,122,124,144,123,171, 183,183,202,158,173,141,144,148, 197,182,196,197,174,218,115,99, 113,111,102,198,116,150,173,165,
   163,179,162,158,169,174,179,196, 171,195,195,169,176,142,198,171, 137,149,180,180,122,144,118,130, 126,177,118,147,152,115,134,107,
   },
  {42,0,13,0,0,0,0,0, 0,162,166,0,0,180,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   232,145,182,109,125,120,185,162, 148,164,123,123,190,165,187,146, 152,167,161,152,152,149,142,140, 139,140,164,140,212,106,144,148,
   114,197,179,197,187,193,175,187, 174,172,164,164,193,190,204,189, 182,167,202,202,200,165,181,159, 152,152,154,137,122,135,112,138,
   124,201,182,197,188,200,175,187, 178,184,163,166,195,192,204,192, 182,163,203,213,204,179,184,151, 152,150,156,105,146,106,124,0,
   154,118,136,147,125,135,127,149, 150,171,149,158,123,138,165,150, 145,167,173,164,158,153,119,174, 161,141,125,139,166,124,149,128,
   121,135,106,117,127,149,123,154, 119,135,121,131,129,103,106,107, 103,121,103,103,108,106,141,104, 125,103,103,109,119,103,106,107,
   121,146,166,124,112,147,123,119, 142,156,201,137,158,141,109,143, 126,135,114,134,124,136,109,103, 105,103,103,132,103,152,150,164,
   189,156,172,160,169,185,143,151, 179,154,167,169,166,174,187,145, 178,169,174,164,137,148,147,154, 144,138,124,165,164,121,147,108,
   },
  {64,0,40,60,48,54,44,48, 40,32,58,74,24,52,16,30, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   30,0,2,2,2,2,2,2, 2,2,2,14,2,2,2,2, 40,0,2,14,2,8,2,2, 14,8,34,52,2,28,2,6,
   2,0,2,2,2,2,2,2, 2,2,2,14,2,2,2,2, 16,0,2,2,2,2,2,2, 2,2,2,2,2,2,2,2,
   2,0,2,2,2,2,2,2, 2,2,2,4,2,2,2,2, 2,0,2,2,2,2,2,2, 2,2,14,30,2,8,2,2,
   110,0,128,112,130,132,130,130, 172,160,146,144,140,156,152,144, 116,0,126,118,72,74,142,134, 166,176,152,142,114,142,138,132,
   150,0,140,144,110,108,104,102, 150,144,198,192,158,160,124,142, 118,0,138,130,110,122,126,128, 152,146,184,206,166,166,128,142,
   154,0,132,150,124,122,106,110, 110,102,160,166,174,156,114,104, 126,0,130,136,116,114,128,136, 178,160,156,162,138,174,134,142,
   118,0,122,114,142,144,108,96, 120,128,142,140,104,154,162,162, 124,0,132,118,132,136,124,118, 114,100,130,140,112,150,162,164,
   },
},

{ // GB18030 (0.640M chars) [32]
  {NULL, NULL, NULL, NULL},
  202, 189, 30, 18, 127,
    {107,143,144,138,146,135,139,141, 137,141,140,132,146,144,130,130, 137,138,134,135,134,142,138,131, 141,139,135,139,136,137,138,133,
   130,205,136,202,219,216,134,132, 133,158,133,130,130,130,130,130, 190,196,199,196,196,209,197,199, 197,204,195,200,202,198,193,195,
   193,196,190,195,196,183,196,192, 202,197,207,198,197,198,202,200, 202,191,204,201,199,197,203,200, 140,132,132,140,134,135,133,131,
   132,133,132,133,146,134,134,134, 133,133,134,132,135,132,132,131, 133,132,131,131,133,132,135,135, 130,130,130,130,130,130,130,0,
   150,126,124,127,127,133,129,124, 132,124,123,123,132,122,120,121, 35,63,39,52,35,68,64,64, 75,69,63,67,52,52,39,65,
   121,121,121,121,122,120,121,121, 126,122,121,120,121,120,121,121, 121,121,121,120,121,121,120,121, 121,121,120,121,122,121,121,121,
   121,121,121,121,121,120,120,120, 121,120,120,121,121,121,121,120, 120,120,121,120,120,120,120,121, 121,121,121,121,121,121,121,121,
   121,120,121,120,121,120,120,120, 121,121,121,121,120,121,120,120, 120,120,120,122,120,120,120,121, 121,121,120,120,121,121,120,0,
   },
  {0,0,0,0,0,0,0,0, 0,79,79,0,0,79,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   79,85,79,79,79,79,95,79, 83,83,79,93,93,79,87,79, 126,115,111,106,110,106,103,106, 108,108,79,79,140,83,79,110,
   132,131,132,136,135,131,132,131, 131,141,131,133,131,132,132,133, 131,133,131,132,138,132,132,137, 134,131,131,131,131,133,136,131,
   131,131,131,131,131,132,132,133, 131,131,131,131,134,131,131,135, 131,131,139,131,132,131,131,132, 131,132,132,131,134,131,131,0,
   138,132,131,134,130,131,132,130, 130,130,131,132,131,134,133,133, 131,134,137,134,131,130,133,133, 130,131,132,131,130,132,134,136,
   138,196,198,193,197,176,186,185, 196,185,197,195,199,187,185,192, 190,191,180,190,190,195,182,192, 182,191,188,196,190,191,188,193,
   187,175,193,194,203,190,193,195, 191,189,190,196,182,188,196,192, 200,179,185,187,190,187,193,181, 193,177,196,183,186,180,183,186,
   186,188,190,186,184,188,186,187, 178,191,190,192,188,188,182,181, 179,186,189,196,174,178,187,193, 193,183,197,187,178,188,186,0,
   },
  {218,0,240,172,146,146,148,146, 120,44,2,2,2,2,2,2, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   56,0,26,2,2,2,2,2, 74,70,2,2,2,2,2,2, 50,0,20,2,2,2,2,2, 84,76,2,2,2,2,2,2,
   50,0,22,2,2,2,2,2, 86,76,2,2,2,2,2,2, 58,0,28,2,2,2,2,2, 86,76,2,2,2,2,2,2,
   128,0,128,64,36,36,38,38, 86,76,2,2,2,2,2,2, 128,0,128,68,40,40,42,42, 84,76,2,2,2,2,2,2,
   0,0,0,212,188,194,188,186, 180,180,120,114,116,122,126,126, 0,0,0,192,194,188,194,194, 176,176,116,120,116,122,136,128,
   0,0,0,194,206,204,216,214, 118,118,136,130,128,116,126,112, 0,0,0,198,208,208,228,220, 112,112,126,126,128,126,126,130,
   0,0,0,188,210,208,232,222, 112,112,120,126,126,126,130,130, 0,0,0,192,210,208,232,222, 120,120,124,128,126,134,120,124,
   0,0,0,188,210,208,236,222, 176,176,128,122,120,122,124,124, 0,0,0,180,210,208,236,222, 178,178,122,122,122,124,126,124,
   },
},

{ // CP852 (9.112M chars) [33]
  {NULL, NULL, NULL, NULL},
  85, 183, 47, 36, 128,
    {147,99,153,98,113,162,97,141, 139,113,105,98,99,105,112,106, 117,111,78,82,68,70,92,81, 93,63,63,89,87,89,89,145,
   187,171,114,163,117,105,174,173, 163,149,96,97,180,97,98,99, 96,96,96,96,96,186,101,174, 97,96,96,96,96,99,100,96,
   96,96,96,96,96,96,112,102, 96,96,96,96,96,96,96,96, 97,99,112,109,110,120,170,99, 174,96,96,96,96,98,162,96,
   115,98,115,99,99,120,184,183, 99,167,99,97,153,153,98,101, 97,97,96,97,96,158,99,96, 100,97,97,100,191,191,96,158,
   178,142,216,118,134,202,143,174, 177,140,130,157,125,109,134,147, 194,166,135,125,142,148,164,161, 172,142,141,143,163,165,129,208,
   222,229,177,175,143,129,196,201, 175,165,92,102,208,119,112,129, 92,92,92,92,92,217,114,205, 120,92,92,92,92,119,130,92,
   92,92,92,92,92,92,117,134, 95,95,92,92,92,92,92,99, 100,101,140,134,151,163,210,124, 213,92,92,92,92,116,181,94,
   176,147,127,120,122,171,198,197, 98,182,97,109,208,197,117,134, 100,102,93,126,93,173,129,92, 111,100,112,145,203,203,92,168,
   },
  {0,0,0,0,0,0,0,0, 0,124,152,0,0,168,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   234,148,183,99,97,113,164,166, 117,160,105,108,196,160,193,147, 123,151,151,145,144,134,140,137, 123,116,177,135,205,109,140,149,
   99,185,176,199,192,207,123,147, 188,196,177,198,197,199,208,171, 173,98,190,193,202,160,196,129, 101,116,185,101,102,119,97,112,
   97,188,176,199,192,210,124,151, 188,197,178,199,198,199,208,173, 176,98,191,194,203,165,197,129, 100,117,186,97,109,97,97,0,
   127,96,131,106,97,136,102,129, 109,96,96,129,100,96,97,102, 122,147,109,96,96,114,120,101, 101,96,96,149,150,109,105,174,
   176,201,108,134,125,115,179,179, 153,141,96,96,173,96,98,99, 96,96,96,96,96,174,106,99, 96,96,96,96,96,110,110,96,
   96,96,96,96,96,96,98,98, 96,96,96,96,96,96,96,98, 97,97,137,96,141,138,199,98, 99,96,96,96,96,100,136,96,
   108,105,96,97,97,138,174,174, 99,131,100,96,100,100,100,102, 116,100,96,107,96,115,97,96, 100,97,97,99,180,181,96,164,
   },
  {92,0,50,80,56,70,54,68, 82,58,30,64,104,40,54,46, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   8,0,2,2,2,2,2,2, 2,2,2,2,6,2,2,2, 20,0,2,2,2,2,2,2, 4,2,2,2,24,2,2,2,
   78,0,10,38,14,28,12,26, 96,60,28,64,128,38,54,46, 26,0,2,2,2,2,2,2, 2,2,2,2,24,2,2,2,
   14,0,2,2,2,2,2,2, 2,2,2,2,24,2,2,2, 8,0,2,2,2,2,2,2, 10,2,2,2,32,2,2,2,
   138,0,138,136,100,100,126,124, 180,178,162,150,190,144,146,142, 122,0,120,134,128,124,134,128, 182,146,170,148,172,110,140,132,
   132,0,132,130,120,106,130,132, 158,170,152,172,148,158,164,162, 86,0,86,90,142,152,50,60, 150,144,154,132,162,132,174,174,
   148,0,132,140,116,136,116,134, 232,176,140,174,232,152,168,164, 128,0,124,124,134,132,122,132, 156,160,164,142,166,136,162,164,
   124,0,124,120,130,132,128,132, 156,156,160,132,156,166,160,146, 118,0,96,134,136,98,136,108, 136,130,160,158,150,168,118,150,
   },
},

{ // Arabic (0.205M chars) [34]
  {NULL, NULL, NULL, NULL},
  180, 214, 71, 14, 128,
    {0,0,0,0,0,0,0,0, 0,121,121,0,0,121,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   136,0,0,0,136,0,0,0, 0,0,0,0,138,136,0,0, 0,0,0,0,0,0,0,0, 0,0,0,136,0,0,0,152,
   0,151,167,198,147,188,171,230, 205,155,210,171,192,196,187,199, 176,204,173,195,191,195,175,181, 170,203,174,0,0,0,0,0,
   182,201,194,197,213,218,204,199, 211,169,210,171,161,136,141,183, 141,152,136,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   0,0,0,0,0,0,0,0, 0,133,133,0,0,133,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   145,0,0,0,145,0,0,0, 0,0,0,0,184,145,0,0, 0,0,0,0,0,0,0,0, 0,0,0,148,0,0,0,159,
   0,171,146,149,145,145,147,191, 180,203,194,168,171,165,164,185, 152,192,159,171,155,163,167,168, 152,175,156,0,0,0,0,0,
   152,177,173,172,190,189,191,184, 173,192,184,156,145,145,146,145, 145,149,145,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   },
  {0,0,0,0,0,0,0,0, 0,137,171,0,0,150,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   227,142,169,137,137,137,163,144, 137,152,138,137,171,141,178,147, 138,139,137,137,137,138,137,137, 137,137,167,137,194,137,137,137,
   137,137,137,137,137,137,137,137, 137,137,137,137,137,137,137,137, 137,137,137,137,137,137,137,137, 137,137,137,137,138,137,137,142,
   137,137,137,137,137,137,137,137, 137,137,137,137,137,137,137,137, 137,137,137,137,137,137,137,137, 137,137,137,137,137,137,137,0,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   137,164,151,154,137,159,165,165, 160,155,154,156,173,137,172,165, 170,158,160,163,159,161,154,161, 189,173,179,140,185,172,164,157,
   0,171,144,171,160,163,176,218, 199,202,201,175,192,195,189,204, 182,210,177,202,184,184,183,185, 163,200,179,0,0,0,0,0,
   160,196,197,191,226,205,211,195, 209,178,216,158,137,138,147,151, 146,154,138,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   },
  {158,0,126,158,180,178,180,180, 0,0,144,124,130,132,124,178, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   52,0,2,28,50,50,50,52, 0,0,2,2,2,2,2,34, 52,0,2,28,50,50,50,52, 0,0,20,10,2,2,2,56,
   2,0,2,2,2,2,2,2, 0,0,2,2,2,2,2,2, 2,0,2,2,2,2,2,2, 0,0,2,2,2,2,2,16,
   2,0,2,2,2,2,2,2, 0,0,2,2,2,2,2,2, 48,0,2,24,46,46,46,48, 0,0,22,12,2,2,2,58,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   166,0,150,140,158,156,158,158, 0,0,146,124,130,132,122,178, 154,0,138,166,170,170,170,170, 0,0,134,162,120,124,114,168,
   134,0,150,152,144,144,144,144, 0,0,106,68,128,132,132,124, 144,0,150,152,154,154,154,154, 0,0,110,76,134,124,130,130,
   156,0,150,144,146,146,146,146, 0,0,140,142,130,128,126,130, 156,0,134,156,178,178,178,178, 0,0,138,116,132,130,128,174,
   },
},

{ // BIG5_HKSCS (0.271M chars) [35]
  {NULL, NULL, NULL, NULL},
  157, 175, 62, 14, 128,
    {0,0,0,0,0,0,0,0, 108,149,150,140,0,150,152,140, 132,132,131,132,132,132,132,132, 132,132,132,131,131,132,131,130,
   139,181,164,132,214,207,204,195, 203,198,208,192,195,201,196,187, 202,189,195,196,188,194,190,182, 195,188,192,186,188,188,191,185,
   184,180,185,187,180,189,169,181, 135,142,140,142,141,141,140,141, 141,140,140,140,141,140,141,140, 140,140,142,140,140,141,141,140,
   140,140,140,140,140,146,149,145, 143,146,140,141,149,140,140,140, 140,140,140,140,140,140,140,140, 140,151,141,140,141,139,140,0,
   0,0,0,0,0,0,0,0, 134,133,133,134,0,124,134,133, 122,122,122,122,122,122,122,122, 122,122,122,121,121,122,122,121,
   132,216,163,137,212,204,205,192, 184,187,190,192,193,185,187,179, 188,190,183,193,178,189,189,198, 181,189,172,184,183,181,173,166,
   178,176,172,174,168,176,168,176, 163,140,141,134,134,134,135,134, 134,134,135,135,135,135,134,134, 135,135,134,134,134,134,134,134,
   134,134,134,134,134,134,135,136, 134,134,134,134,134,134,135,134, 134,134,134,134,134,136,135,134, 134,134,144,134,134,134,134,0,
   },
  {0,0,0,0,0,0,0,0, 0,81,81,0,0,81,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   0,81,81,81,81,81,81,81, 81,81,81,81,81,81,81,81, 81,81,81,81,81,81,81,81, 81,81,81,81,85,81,81,81,
   200,202,189,193,189,173,185,189, 193,179,177,176,184,185,182,192, 186,181,175,172,181,185,166,189, 183,172,168,173,170,186,179,178,
   174,184,179,166,182,181,171,178, 178,187,186,179,178,172,187,186, 185,189,169,194,178,186,186,183, 180,180,179,177,186,177,193,0,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   157,191,170,191,186,176,168,184, 181,177,175,185,177,176,169,168, 182,178,179,190,172,180,184,171, 183,186,201,182,169,184,171,170,
   185,180,173,171,183,175,187,182, 183,185,183,177,184,178,189,186, 187,188,183,195,166,177,177,182, 181,174,185,182,180,178,175,181,
   176,176,179,175,179,183,187,172, 179,192,190,179,187,180,169,178, 168,171,175,183,191,176,182,182, 188,183,186,178,183,179,177,0,
   },
  {232,0,254,254,110,120,118,114, 128,128,4,2,2,2,2,4, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   40,0,16,16,2,2,2,2, 128,128,2,2,2,2,2,2, 46,0,24,22,2,2,2,2, 128,128,2,2,2,2,2,2,
   66,0,42,40,2,2,2,2, 128,128,2,2,2,2,2,2, 128,0,128,128,2,2,2,2, 128,128,2,2,2,2,2,2,
   128,0,128,128,2,2,2,2, 128,128,2,2,2,2,2,2, 128,0,128,128,2,2,2,2, 128,128,2,2,2,2,2,2,
   0,0,0,0,132,142,144,140, 0,0,140,124,126,126,126,126, 0,0,0,0,134,142,142,138, 0,0,136,134,134,134,134,134,
   0,0,0,0,142,138,138,134, 0,0,136,138,132,132,134,134, 0,0,0,0,132,138,140,144, 0,0,134,128,138,138,132,136,
   0,0,0,0,134,144,136,140, 0,0,126,134,132,132,142,138, 0,0,0,0,134,144,142,138, 0,0,136,134,134,134,136,136,
   0,0,0,0,134,144,142,138, 0,0,144,132,132,132,132,132, 0,0,0,0,136,142,140,138, 0,0,136,132,134,136,134,138,
   },
},

{ // CP866 (75.238M chars) [36]
  {NULL, NULL, NULL, NULL},
  144, 168, 54, 34, 129,
    {202,189,201,189,198,201,176,191, 205,171,201,198,196,207,207,203, 202,204,202,184,177,168,173,180, 170,166,142,175,168,167,156,168,
   200,185,197,186,194,201,174,188, 204,171,197,197,192,205,205,197, 60,60,60,60,60,60,60,60, 60,60,60,60,60,60,60,60,
   60,60,60,60,60,60,60,60, 60,60,60,60,60,60,60,60, 60,60,60,60,60,60,60,60, 60,60,60,60,60,60,60,60,
   203,204,203,185,172,167,174,181, 171,168,145,178,171,163,156,169, 122,121,61,61,64,61,61,61, 82,60,60,60,60,60,60,149,
   155,129,160,132,130,136,112,122, 155,123,142,126,140,137,156,133, 109,124,109,108,94,99,94,88, 84,75,58,94,87,91,94,119,
   193,150,189,167,164,195,129,155, 199,188,180,165,183,167,190,143, 55,55,55,55,55,55,55,55, 55,55,55,55,55,55,55,55,
   55,55,55,55,55,55,55,55, 55,55,55,55,55,55,55,55, 55,55,55,55,55,55,55,55, 55,55,55,55,55,55,55,55,
   172,176,181,172,118,175,146,138, 137,99,105,181,182,104,163,194, 90,129,56,55,55,62,55,57, 106,70,89,55,55,55,55,167,
   },
  {0,0,0,0,0,0,0,0, 0,118,150,0,0,157,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   221,147,180,74,65,105,160,149, 120,156,94,110,186,165,186,136, 108,128,124,115,112,121,107,104, 100,97,169,132,195,113,104,144,
   134,102,74,101,82,91,78,71, 88,102,80,94,92,80,75,94, 112,60,73,125,77,72,89,82, 78,91,81,80,102,111,73,107,
   93,109,104,99,117,99,90,77, 101,105,109,107,97,101,119,121, 134,59,107,112,113,105,117,82, 72,94,116,60,90,75,74,0,
   210,180,193,179,186,209,168,174, 206,179,191,195,187,202,211,177, 198,196,203,188,161,170,168,174, 161,156,124,189,187,127,169,186,
   213,183,195,180,188,212,170,177, 208,179,193,198,188,204,215,181, 60,60,60,60,60,60,60,60, 60,60,60,60,60,60,60,60,
   60,60,60,60,60,60,60,60, 60,60,60,60,60,60,60,60, 60,60,60,60,60,60,60,60, 60,60,60,60,60,60,60,60,
   202,198,205,193,164,172,168,175, 162,157,125,191,187,137,171,187, 127,130,61,61,67,67,60,61, 82,60,61,60,66,61,60,147,
   },
  {2,0,2,2,50,52,44,42, 12,26,10,176,176,176,22,96, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   2,0,2,2,4,6,2,2, 2,2,2,68,68,68,2,8, 128,0,58,84,128,128,128,128, 52,68,48,128,128,128,66,128,
   128,0,58,84,128,128,128,128, 52,68,48,128,128,128,66,128, 128,0,58,84,128,128,128,128, 52,68,48,128,128,128,66,128,
   2,0,2,2,16,18,10,10, 2,2,2,86,86,86,2,18, 64,0,4,30,96,98,92,86, 2,2,2,128,128,128,2,62,
   146,0,152,154,174,180,156,152, 138,138,114,116,116,116,110,112, 148,0,150,162,184,184,176,158, 138,140,112,130,130,130,110,118,
   154,0,152,150,150,138,126,128, 10,12,138,118,118,118,138,116, 176,0,132,158,200,200,200,198, 118,136,116,200,200,200,132,200,
   176,0,132,158,200,200,200,198, 118,136,116,200,200,200,132,200, 176,0,132,158,200,200,200,198, 118,136,116,200,200,200,132,200,
   152,0,152,154,122,140,132,122, 10,14,136,132,132,132,138,120, 166,0,126,164,192,204,210,212, 118,118,110,200,200,200,116,224,
   },
},

{ // UTF-16BE (921.761M chars) [37]
  {NULL, NULL, NULL, NULL},
  58, 128, 46, 5, 128,
    {255,148,128,107,171,104,157,100, 118,108,101,120,124,90,133,117, 110,115,104,101,109,109,114,104, 109,116,118,96,107,107,124,118,
   70,74,71,58,71,66,66,63, 77,54,49,59,103,87,65,64, 97,64,107,90,85,62,70,84, 68,102,96,78,93,84,65,62,
   98,86,102,72,68,106,98,98, 86,95,76,75,78,83,75,75, 86,85,82,77,76,85,76,77, 73,80,63,66,87,93,89,101,
   71,70,54,77,57,100,56,43, 81,50,69,75,59,63,75,80, 86,62,64,76,70,35,84,71, 72,68,83,83,81,83,96,112,
   158,116,117,132,124,138,121,108, 116,113,119,126,121,103,105,108, 120,97,117,112,89,96,106,109, 108,102,107,100,99,84,118,104,
   97,103,109,97,115,85,102,101, 103,94,96,102,108,105,104,111, 112,95,96,109,94,101,100,108, 109,120,113,105,98,89,95,106,
   95,111,93,114,92,107,109,118, 118,105,105,108,86,103,97,102, 101,110,88,98,94,117,109,105, 78,77,73,73,66,78,86,78,
   104,107,89,105,83,121,89,89, 97,107,111,111,101,103,99,100, 88,81,89,122,99,70,98,85, 99,87,106,114,125,103,97,136,
   },
  {236,123,89,99,100,109,89,100, 108,198,193,73,119,181,104,83, 99,103,97,99,99,97,82,94, 101,109,110,107,104,93,93,106,
   216,151,203,161,118,166,169,173, 164,164,144,152,168,182,186,198, 189,182,179,173,169,172,169,168, 170,168,177,180,198,196,198,153,
   139,171,167,169,169,168,165,158, 160,167,150,145,164,164,166,162, 165,146,165,168,170,153,155,156, 147,151,149,145,145,143,117,176,
   111,206,186,196,198,208,187,191, 193,205,165,175,201,192,202,201, 195,149,202,203,208,188,183,182, 175,180,162,150,135,150,118,103,
   112,108,111,92,98,93,88,76, 111,111,111,115,106,106,97,112, 115,111,81,95,101,109,110,98, 111,93,99,76,83,70,93,76,
   118,92,84,86,91,83,65,85, 86,99,71,87,101,87,83,95, 96,99,90,82,97,95,62,97, 88,93,93,96,90,93,79,99,
   100,101,86,92,104,90,86,96, 95,94,77,82,107,97,85,85, 105,105,86,90,102,108,87,87, 97,112,82,93,109,99,100,96,
   112,124,97,99,120,103,92,104, 114,131,110,75,88,121,83,84, 84,109,72,116,98,89,110,72, 99,78,96,85,114,106,69,129,
   },
  {126,120,126,128,126,126,126,126, 206,210,222,240,224,222,220,216, 122,164,100,126,150,150,112,122, 232,224,222,208,220,232,222,228,
   28,128,42,46,76,82,30,42, 128,128,128,128,128,128,128,128, 18,128,28,34,58,64,18,28, 128,128,128,128,128,128,128,128,
   16,128,26,32,54,62,16,26, 128,128,128,128,128,128,128,128, 22,128,32,38,70,74,22,32, 128,128,128,128,128,128,128,128,
   30,128,42,48,82,88,32,42, 128,128,128,128,128,128,128,128, 90,190,74,128,122,156,100,110, 212,182,194,168,162,186,118,178,
   96,180,78,128,126,140,100,104, 98,96,94,100,98,90,80,92, 108,222,102,132,140,156,114,122, 236,234,220,202,226,226,226,230,
   118,206,78,150,134,154,108,118, 228,218,192,196,192,176,210,224, 112,218,96,148,132,158,110,122, 232,232,220,208,226,212,224,228,
   112,224,102,146,142,156,108,124, 234,232,226,222,226,226,216,222, 114,196,84,148,140,164,110,118, 242,232,196,208,200,198,192,206,
   110,160,74,150,140,156,114,118, 230,224,160,160,162,160,160,220, 124,240,102,144,132,152,106,116, 230,226,200,198,162,174,190,230,
   },
},

{ // Latin3 (0.294M chars) [38]
  {NULL, NULL, ced_hires_16, ced_hires_16, },
  99, 202, 43, 23, 128,
    {0,0,0,0,0,0,0,0, 0,142,142,0,0,142,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   180,143,143,143,145,0,143,148, 145,143,143,143,143,143,0,146, 147,143,143,143,144,143,143,148, 145,206,195,143,143,143,0,145,
   143,143,143,0,158,143,143,150, 143,151,143,143,143,143,143,143, 0,145,143,162,143,143,147,143, 143,143,143,143,172,143,143,143,
   154,143,143,0,165,177,154,182, 165,150,143,148,143,144,143,145, 0,145,143,162,143,143,157,143, 143,143,143,143,172,144,143,143,
   0,0,0,0,0,0,0,0, 0,112,112,0,0,112,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   188,148,144,162,154,0,145,154, 146,209,160,192,146,145,0,200, 163,148,147,145,151,144,144,167, 145,236,159,192,146,145,0,204,
   161,202,162,87,203,144,144,187, 186,211,176,153,145,198,159,150, 76,180,149,201,165,145,202,146, 144,144,182,153,204,144,144,178,
   188,206,154,0,204,144,144,175, 177,206,166,153,158,208,152,146, 0,180,159,203,157,145,200,146, 144,155,182,149,192,144,144,147,
   },
  {86,0,0,0,0,0,0,0, 0,149,151,0,0,159,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   223,146,167,143,143,144,158,154, 152,149,146,143,193,154,169,146, 150,160,164,151,150,149,147,146, 146,146,160,144,200,143,144,146,
   143,194,180,194,186,193,173,187, 178,180,164,179,196,190,207,184, 177,157,206,202,196,184,182,176, 156,184,174,143,143,145,144,143,
   144,192,181,189,184,193,172,184, 175,179,163,204,214,197,221,182, 173,152,217,199,214,184,180,176, 154,186,193,144,147,143,144,0,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   180,143,143,143,146,164,143,146, 146,145,143,171,143,144,177,162, 146,143,143,143,145,143,143,148, 146,199,202,192,143,143,169,162,
   145,144,145,0,158,143,143,149, 143,151,144,143,143,146,144,143, 0,144,144,146,143,143,148,144, 143,143,143,143,146,143,143,161,
   144,144,144,0,159,143,143,146, 143,147,144,143,143,146,143,143, 0,144,144,147,146,143,150,143, 143,143,144,143,145,143,143,143,
   },
  {162,0,130,150,128,132,120,124, 0,0,156,140,170,170,172,172, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   24,0,2,2,2,2,2,2, 0,0,2,2,2,2,2,2, 2,0,2,2,2,2,2,2, 0,0,2,2,2,2,2,2,
   30,0,2,2,2,2,2,2, 0,0,2,2,2,2,2,2, 26,0,2,2,2,2,2,2, 0,0,2,2,2,2,2,2,
   16,0,2,2,2,2,2,2, 0,0,2,2,2,2,2,2, 24,0,2,2,2,2,2,2, 0,0,2,2,2,2,2,2,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   144,0,124,142,134,134,124,120, 0,0,174,132,162,162,162,166, 118,0,132,130,74,76,134,134, 0,0,126,164,140,140,140,142,
   118,0,100,110,147,145,94,90, 0,0,152,136,178,170,170,170, 124,0,102,116,143,145,108,108, 0,0,170,132,166,166,164,166,
   134,0,140,130,82,86,131,131, 0,0,168,152,154,156,164,156, 130,0,124,124,92,96,137,137, 0,0,148,152,164,170,166,166,
   },
},

{ // UTF-16LE (912.138M chars) [39]
  {NULL, NULL, NULL, NULL},
  58, 128, 46, 4, 127,
    {255,146,126,120,177,123,159,99, 110,78,94,116,115,99,130,112, 110,112,82,97,100,94,113,102, 100,94,112,92,103,102,120,113,
   60,84,76,72,67,43,63,72, 75,64,74,96,95,88,83,52, 78,81,96,84,76,57,31,82, 83,85,86,82,92,78,88,68,
   84,91,96,52,84,96,99,105, 89,83,72,65,83,88,84,86, 81,94,75,70,97,95,78,57, 109,82,33,91,80,77,77,75,
   25,59,64,81,47,69,60,19, 96,31,92,30,20,68,72,81, 58,66,54,80,88,55,60,66, 55,40,45,78,55,99,113,116,
   161,117,113,130,127,131,118,108, 119,110,115,123,129,107,105,108, 116,99,114,110,93,99,108,111, 109,103,113,103,97,84,101,103,
   91,100,101,94,110,85,98,97, 98,88,86,93,108,101,99,107, 111,91,107,105,91,95,97,104, 105,118,111,98,97,92,87,101,
   101,106,99,111,87,110,107,116, 114,105,99,102,87,99,93,98, 100,103,88,98,89,111,105,101, 111,92,89,98,104,108,110,105,
   102,102,83,99,77,115,78,86, 85,100,104,107,98,100,92,97, 86,77,86,117,94,64,98,84, 97,89,103,109,121,97,98,126,
   },
  {236,123,100,94,96,100,91,97, 108,196,193,90,112,182,101,92, 101,105,95,98,102,96,89,96, 98,112,104,103,108,101,95,106,
   216,149,203,169,119,166,174,176, 165,165,142,152,168,181,187,198, 189,182,180,175,173,177,172,171, 173,176,177,184,198,197,198,156,
   139,172,168,169,171,168,166,159, 159,167,150,145,165,165,166,163, 165,145,164,168,170,153,155,156, 147,150,149,144,147,141,118,176,
   125,206,186,196,197,208,187,190, 192,205,164,175,200,191,201,201, 195,151,202,202,207,188,182,182, 175,179,161,149,134,149,121,100,
   115,103,100,89,101,94,90,87, 101,114,108,116,108,98,92,113, 113,107,90,81,101,110,105,94, 107,95,107,74,84,72,92,81,
   118,95,89,86,95,86,81,84, 92,99,79,88,102,91,89,97, 104,100,92,91,99,99,79,96, 89,102,98,99,90,95,85,101,
   101,103,87,94,104,94,85,94, 97,94,83,90,106,96,82,86, 105,103,85,93,102,108,92,91, 91,103,79,87,101,91,92,87,
   112,124,97,99,119,112,90,105, 108,128,107,81,91,121,89,85, 88,97,79,113,98,90,111,79, 102,83,100,94,112,102,86,130,
   },
  {128,124,128,126,126,126,126,126, 204,202,224,222,224,224,220,218, 120,164,116,98,140,156,124,122, 220,224,168,162,168,220,206,238,
   82,158,96,72,110,162,88,92, 170,160,158,158,158,176,158,164, 54,158,66,68,104,108,56,66, 158,158,158,158,158,148,158,158,
   48,158,58,62,88,96,50,60, 158,158,158,158,158,148,158,158, 50,158,60,64,90,98,52,62, 158,158,158,158,158,148,158,158,
   62,158,74,80,110,116,66,76, 158,158,158,158,158,148,158,158, 112,192,96,138,138,132,104,106, 206,210,184,198,188,196,186,186,
   106,174,86,120,124,134,92,112, 96,94,82,82,82,78,66,76, 122,214,110,132,136,160,112,112, 228,230,210,212,214,210,216,212,
   90,200,118,146,140,162,114,120, 234,222,202,200,206,188,196,196, 96,206,128,142,144,156,116,126, 238,216,210,206,214,206,218,208,
   94,216,130,142,136,160,108,126, 238,236,218,214,222,212,214,212, 94,214,134,138,140,158,114,124, 238,238,212,208,212,204,214,208,
   62,162,54,148,148,164,116,124, 232,226,164,164,162,150,164,174, 94,210,110,146,134,162,112,118, 232,228,192,198,194,188,198,208,
   },
},

{ // HZ-GB-2312 (87.400M chars) [40]
  {NULL, NULL, NULL, NULL},
  92, 0, 37, 0, 255,
    {0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   189,189,189,189,189,189,189,189, 189,189,189,189,189,189,189,189, 189,189,189,189,189,189,189,189, 189,189,189,189,189,189,189,189,
   189,189,189,189,189,189,189,189, 189,189,189,189,189,189,189,189, 189,189,189,189,189,189,189,189, 189,189,189,189,189,189,189,189,
   189,189,189,189,189,189,189,189, 189,189,189,189,189,189,189,189, 189,189,189,189,189,189,189,189, 189,189,189,189,189,189,189,189,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   },
  {126,0,164,0,0,0,126,0, 178,139,148,0,0,132,154,146, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   161,195,176,211,162,158,148,144, 213,209,179,164,154,148,168,174, 182,184,172,182,184,188,187,175, 180,183,176,176,182,177,179,172,
   170,193,196,199,167,165,195,168, 183,178,178,180,168,178,186,182, 183,158,180,182,181,183,186,180, 155,132,136,132,146,136,132,132,
   132,132,136,139,173,183,178,171, 162,161,132,142,144,132,139,139, 132,136,139,144,132,132,149,139, 132,132,136,152,139,132,245,0,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   },
  {0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   2,0,2,2,2,2,2,2, 0,0,0,0,0,0,0,0, 2,0,2,2,2,2,2,2, 0,0,0,0,0,0,0,0,
   2,0,2,2,2,2,2,2, 0,0,0,0,0,0,0,0, 2,0,2,2,2,2,2,2, 0,0,0,0,0,0,0,0,
   2,0,2,2,2,2,2,2, 0,0,0,0,0,0,0,0, 2,0,2,2,2,2,2,2, 0,0,0,0,0,0,0,0,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   },
},

{ // CSN_369103 (8.850M chars) [41]
  {NULL, NULL, NULL, NULL},
  90, 204, 46, 27, 127,
    {0,0,0,0,0,0,0,0, 0,90,90,0,0,90,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   176,178,96,184,92,102,188,127, 120,153,117,98,133,96,143,153, 122,178,93,182,115,102,188,96, 119,152,117,98,133,101,141,152,
   98,169,119,130,151,111,96,137, 171,147,170,103,153,155,105,120, 96,98,118,171,112,104,128,105, 180,129,158,96,133,127,119,102,
   99,169,99,125,153,107,102,128, 169,142,170,99,155,155,103,119, 95,99,118,171,112,105,148,95, 180,130,154,95,137,127,119,96,
   0,0,0,0,0,0,0,0, 0,55,55,0,0,55,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   185,202,93,207,98,140,210,145, 119,178,153,123,170,124,172,196, 158,217,94,211,139,141,210,121, 116,176,151,152,187,124,173,200,
   103,198,155,156,200,128,165,183, 202,208,195,143,194,194,151,144, 149,174,157,197,159,157,199,120, 192,171,178,137,201,170,155,174,
   106,203,144,173,201,105,197,170, 203,203,208,143,202,204,139,150, 148,188,166,199,147,166,197,121, 192,191,178,144,188,182,155,122,
   },
  {62,0,0,0,0,0,0,0, 0,129,153,0,0,162,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   228,154,181,95,91,108,164,158, 144,155,114,105,191,153,189,148, 132,145,139,130,128,126,119,117, 115,116,177,135,201,95,124,155,
   93,200,180,210,197,203,168,188, 175,191,170,190,200,190,208,194, 182,141,204,199,200,181,182,186, 146,188,181,105,114,126,105,120,
   112,201,181,209,196,205,167,186, 172,191,169,190,200,189,207,194, 180,135,203,198,199,182,181,191, 143,189,181,100,125,92,99,0,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   176,176,95,167,93,99,163,117, 120,144,106,125,140,115,142,179, 121,178,93,168,111,97,163,92, 119,144,107,125,141,107,143,179,
   99,160,117,127,149,96,190,132, 166,138,143,97,95,185,108,127, 94,120,128,168,99,91,130,104, 171,126,127,95,123,101,122,153,
   100,161,110,128,150,96,190,125, 167,128,145,96,95,185,105,130, 94,120,128,170,114,91,134,95, 172,126,113,95,119,100,122,97,
   },
  {182,0,124,152,122,132,122,132, 0,0,162,164,156,176,156,176, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   12,0,2,2,2,2,2,2, 0,0,2,2,2,2,2,2, 14,0,2,2,2,2,2,2, 0,0,2,2,2,2,2,2,
   24,0,2,2,2,2,2,2, 0,0,2,2,2,2,2,2, 26,0,2,2,2,2,2,2, 0,0,2,2,2,2,2,2,
   24,0,2,2,2,2,2,2, 0,0,2,2,2,2,2,2, 26,0,2,2,2,2,2,2, 0,0,2,2,2,2,2,2,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   132,0,96,130,142,136,100,114, 0,0,172,130,168,162,110,142, 130,0,136,132,52,54,136,130, 0,0,108,174,92,106,170,166,
   98,0,90,100,140,142,100,90, 0,0,164,122,170,178,132,132, 100,0,92,102,138,142,108,110, 0,0,166,106,174,156,130,144,
   134,0,136,134,38,54,134,134, 0,0,118,168,102,146,168,180, 132,0,130,134,44,58,136,140, 0,0,108,170,98,158,172,156,
   },
},

{ // ISO-2022-KR (85.145M chars) [42]
  {NULL, NULL, NULL, NULL},
  44, 144, 15, 3, 129,
    {0,0,130,0,0,0,0,0, 0,66,66,0,0,66,213,252, 0,0,0,0,0,0,0,0, 0,0,0,224,0,0,91,91,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   },
  {80,0,0,0,0,0,0,0, 0,155,178,0,0,191,94,16, 0,0,0,0,0,0,0,0, 0,0,0,159,0,0,115,80,
   237,178,211,150,215,142,197,197, 215,186,147,141,202,191,204,184, 182,193,189,179,176,177,168,168, 173,174,186,191,233,164,175,184,
   177,172,153,166,162,158,159,169, 164,164,152,159,147,160,152,147, 160,143,152,154,156,148,153,153, 128,113,118,160,143,179,134,168,
   120,133,133,128,134,139,123,121, 142,133,113,107,120,125,168,114, 123,102,122,140,136,109,112,132, 116,115,107,101,157,124,155,0,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   },
  {130,130,126,130,130,130,118,128, 0,0,0,0,0,0,0,0, 42,2,134,2,62,40,70,60, 0,0,0,0,0,0,0,0,
   128,128,128,128,128,128,128,128, 0,0,0,0,0,0,0,0, 128,128,128,128,128,128,128,128, 0,0,0,0,0,0,0,0,
   128,128,128,128,128,128,128,128, 0,0,0,0,0,0,0,0, 128,128,128,128,128,128,128,128, 0,0,0,0,0,0,0,0,
   128,128,128,128,128,128,128,128, 0,0,0,0,0,0,0,0, 128,128,128,128,128,128,128,128, 0,0,0,0,0,0,0,0,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   },
},

{ // Latin6 (0.061M chars) [43]
  {NULL, NULL, ced_hires_19, ced_hires_19, },
  93, 214, 54, 26, 129,
    {0,0,0,0,0,0,0,0, 0,156,156,0,0,156,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   175,158,158,158,158,158,158,159, 158,158,161,158,159,158,158,158, 158,158,158,158,159,158,158,159, 158,158,161,158,159,158,158,158,
   159,164,158,159,162,158,158,158, 171,161,158,158,159,162,158,158, 158,160,158,158,158,158,159,158, 158,158,162,158,158,158,158,158,
   159,164,158,158,163,158,158,158, 171,160,158,158,159,162,158,158, 158,160,158,158,158,158,162,158, 158,159,162,158,159,158,158,158,
   0,0,0,0,0,0,0,0, 0,130,130,0,0,130,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   180,198,158,158,160,158,158,160, 158,163,190,158,168,158,159,158, 163,212,158,158,162,158,158,166, 159,158,180,158,168,158,158,158,
   206,194,163,172,195,183,171,185, 198,203,191,160,209,190,162,159, 161,179,158,193,165,163,194,158, 179,176,175,160,196,171,160,172,
   211,198,160,173,196,193,171,194, 199,198,204,160,214,199,160,158, 167,178,159,195,161,163,193,158, 178,219,176,159,185,179,160,158,
   },
  {0,0,0,0,0,0,0,0, 0,160,164,0,0,168,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   227,163,177,159,159,159,210,162, 160,170,159,159,189,161,185,159, 162,161,160,159,159,159,159,159, 159,159,178,159,200,159,159,164,
   159,184,177,200,197,187,171,187, 174,180,189,189,194,188,202,180, 180,160,204,208,200,171,181,173, 162,166,178,159,159,159,159,159,
   159,184,179,199,197,189,170,185, 172,181,189,189,194,186,201,179, 179,159,203,208,200,171,185,173, 162,169,178,159,159,159,159,0,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   176,159,159,159,159,159,159,159, 159,159,163,159,161,159,159,159, 159,159,159,159,159,160,159,159, 159,159,163,159,161,159,159,159,
   161,164,159,159,163,159,159,159, 168,160,159,159,159,168,159,159, 160,159,159,159,159,159,159,159, 159,159,159,159,159,159,159,164,
   161,164,159,159,163,159,159,159, 169,159,159,159,159,169,159,159, 160,159,159,159,159,159,159,159, 159,159,159,159,159,159,159,159,
   },
  {152,0,122,146,128,132,128,130, 0,0,154,156,154,156,154,158, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   2,0,2,2,2,2,2,2, 0,0,2,2,2,2,2,2, 4,0,2,2,2,2,2,2, 0,0,2,2,2,2,2,2,
   2,0,2,2,2,2,2,2, 0,0,2,2,2,2,2,2, 4,0,2,2,2,2,2,2, 0,0,2,2,2,2,2,2,
   2,0,2,2,2,2,2,2, 0,0,2,2,2,2,2,2, 4,0,2,2,2,2,2,2, 0,0,2,2,2,2,2,2,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   144,0,114,140,138,132,120,128, 0,0,162,154,152,154,152,154, 142,0,138,134,108,112,134,126, 0,0,154,158,154,156,154,156,
   118,0,98,118,143,145,106,106, 0,0,154,154,162,156,154,154, 132,0,108,126,143,143,118,118, 0,0,154,156,158,158,154,156,
   126,0,132,132,90,92,139,139, 0,0,152,156,152,156,162,156, 126,0,140,126,98,102,131,131, 0,0,154,156,154,158,156,158,
   },
},

{ // UTF7 (0.037M chars) [44]
  {NULL, NULL, NULL, NULL},
  77, 207, 29, 27, 255,
    {0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   189,189,189,189,189,189,189,189, 189,189,189,189,189,189,189,189, 189,189,189,189,189,189,189,189, 189,189,189,189,189,189,189,189,
   189,189,189,189,189,189,189,189, 189,189,189,189,189,189,189,189, 189,189,189,189,189,189,189,189, 189,189,189,189,189,189,189,189,
   189,189,189,189,189,189,189,189, 189,189,189,189,189,189,189,189, 189,189,189,189,189,189,189,189, 189,189,189,189,189,189,189,184,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   },
  {0,0,0,0,0,0,0,0, 0,189,189,0,0,189,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   0,189,189,189,189,189,189,189, 189,189,189,0,189,189,189,189, 189,189,189,189,189,189,189,189, 189,189,189,189,189,189,189,189,
   189,189,189,189,189,189,189,189, 189,189,189,189,189,189,189,189, 189,189,189,189,189,189,189,189, 189,189,189,189,189,189,189,189,
   189,189,189,189,189,189,189,189, 189,189,189,189,189,189,189,189, 189,189,189,189,189,189,189,189, 189,189,189,189,189,189,189,0,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   },
  {0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   2,0,2,2,2,2,2,2, 0,0,0,0,0,0,0,0, 2,0,2,2,2,2,2,2, 0,0,0,0,0,0,0,0,
   2,0,2,2,2,2,2,2, 0,0,0,0,0,0,0,0, 2,0,2,2,2,2,2,2, 0,0,0,0,0,0,0,0,
   2,0,2,2,2,2,2,2, 0,0,0,0,0,0,0,0, 2,0,2,2,2,2,2,2, 0,0,0,0,0,0,0,0,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   },
},

{ // ISO_2022_CN (63.392M chars) [45]
  {NULL, NULL, NULL, NULL},
  43, 144, 14, 3, 129,
    {0,0,0,0,0,0,0,0, 0,70,70,0,0,70,213,252, 0,0,0,0,0,0,0,0, 0,0,0,222,0,0,0,0,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   },
  {79,0,0,0,0,0,0,0, 0,155,177,0,0,190,19,19, 0,0,0,0,0,0,0,0, 0,0,0,158,0,0,114,79,
   240,177,210,148,212,141,197,195, 214,185,147,140,202,190,203,183, 182,192,188,179,175,177,167,168, 173,174,186,190,231,164,175,183,
   177,172,152,166,161,157,158,168, 164,163,151,158,146,160,151,146, 159,143,151,153,155,147,152,152, 127,112,117,160,142,178,133,168,
   119,132,130,126,124,138,122,120, 141,131,112,106,119,124,122,112, 122,101,108,135,135,101,111,130, 112,115,106,96,156,123,154,0,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   },
  {130,130,126,128,130,128,128,130, 0,0,0,0,0,0,0,0, 44,2,134,2,62,26,62,10, 0,0,0,0,0,0,0,0,
   128,128,128,128,128,128,128,128, 0,0,0,0,0,0,0,0, 128,128,128,128,128,128,128,128, 0,0,0,0,0,0,0,0,
   128,128,128,128,128,128,128,128, 0,0,0,0,0,0,0,0, 128,128,128,128,128,128,128,128, 0,0,0,0,0,0,0,0,
   128,128,128,128,128,128,128,128, 0,0,0,0,0,0,0,0, 128,128,128,128,128,128,128,128, 0,0,0,0,0,0,0,0,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   },
},

{ // BIG5-CP950 (0.029M chars) [46]
  {NULL, NULL, NULL, NULL},
  158, 182, 61, 26, 128,
    {0,166,166,166,166,166,166,166, 166,177,177,166,166,177,166,166, 158,158,158,158,158,158,158,158, 158,158,158,158,158,158,158,158,
   166,179,169,151,208,201,198,190, 197,193,202,187,190,196,191,184, 197,185,191,191,184,189,185,179, 190,185,188,182,184,184,187,182,
   181,178,182,183,178,186,166,166, 166,166,166,166,166,166,166,166, 166,166,166,166,166,166,166,166, 166,166,166,166,166,166,166,166,
   166,166,166,166,166,167,168,167, 166,167,166,166,168,166,166,166, 166,166,166,166,166,166,166,166, 166,168,166,166,166,166,166,101,
   0,161,161,161,161,161,161,161, 161,161,161,161,161,161,161,161, 149,149,149,149,149,149,149,149, 149,149,149,149,149,149,149,149,
   161,211,166,161,206,199,200,187, 180,182,185,186,188,181,182,176, 183,186,179,188,175,185,185,193, 178,185,170,180,179,177,170,167,
   175,174,171,172,168,174,169,161, 161,161,161,161,161,161,161,161, 161,161,161,161,161,161,161,161, 161,161,161,161,161,161,161,161,
   161,161,161,161,161,161,161,161, 161,161,161,161,161,161,161,161, 161,161,161,161,161,161,161,161, 161,161,161,161,161,161,161,167,
   },
  {0,0,0,0,0,0,0,0, 0,113,113,0,0,113,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   0,113,113,113,113,113,113,113, 113,113,113,113,113,113,113,113, 113,113,113,113,113,113,113,113, 113,113,113,113,113,113,113,113,
   197,198,187,191,188,176,184,188, 190,180,179,179,184,184,183,189, 185,182,178,177,182,184,174,187, 183,177,175,178,176,185,181,180,
   178,183,181,174,182,182,176,180, 180,186,185,181,180,177,186,185, 184,187,175,191,180,185,185,183, 181,180,181,179,185,179,190,0,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   150,188,176,189,185,179,174,184, 181,179,178,184,179,179,175,174, 182,179,180,187,176,180,183,176, 183,185,197,182,175,183,176,175,
   184,181,177,176,183,178,186,182, 182,184,183,179,184,180,187,185, 186,186,182,191,174,179,179,182, 182,177,184,182,180,179,178,181,
   179,179,180,177,180,182,186,176, 180,190,188,180,185,181,175,179, 174,176,178,183,188,178,182,182, 186,183,185,180,183,180,180,121,
   },
  {170,0,170,170,104,110,110,108, 128,128,2,2,2,2,2,78, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   14,0,2,2,2,2,2,2, 128,128,2,2,2,2,2,2, 20,0,2,2,2,2,2,2, 128,128,2,2,2,2,2,2,
   34,0,10,10,2,2,2,2, 128,128,2,2,2,2,2,2, 42,0,18,18,2,2,2,2, 128,128,2,2,2,2,2,2,
   40,0,18,16,2,2,2,2, 128,128,2,2,2,2,2,2, 42,0,18,18,2,2,2,2, 128,128,2,2,2,2,2,2,
   0,0,0,0,134,142,140,138, 0,0,132,132,132,132,132,132, 0,0,0,0,134,142,140,138, 0,0,136,134,134,136,136,136,
   0,0,0,0,142,136,138,134, 0,0,134,138,132,134,134,134, 0,0,0,0,134,136,140,144, 0,0,134,130,138,136,134,136,
   0,0,0,0,136,142,138,140, 0,0,130,134,134,132,140,138, 0,0,0,0,134,142,140,138, 0,0,136,134,134,136,136,136,
   0,0,0,0,134,142,140,138, 0,0,138,134,134,134,134,134, 160,128,160,160,134,140,140,138, 128,128,136,134,134,134,136,136,
   },
},

{ // JAGRAN (0.046M chars) [47]
  {NULL, NULL, NULL, NULL},
  142, 199, 66, 34, 133,
    {174,174,174,174,174,174,174,174, 174,182,182,174,174,182,174,174, 174,174,174,174,174,174,174,174, 174,174,174,0,0,0,174,174,
   0,182,182,182,182,182,182,182, 182,182,182,182,182,0,182,182, 182,182,0,182,182,182,182,182, 182,182,182,182,182,182,182,182,
   182,182,182,182,182,182,182,182, 182,182,182,182,182,182,182,182, 182,182,182,182,182,182,182,182, 182,182,182,182,182,182,0,182,
   182,182,182,182,182,182,182,182, 182,182,182,182,182,182,0,182, 182,182,182,182,182,182,182,182, 182,182,182,182,182,182,182,182,
   166,166,166,166,166,166,166,166, 166,178,178,166,166,178,166,166, 166,166,166,166,166,166,166,166, 166,166,166,0,0,0,166,166,
   0,178,178,178,178,178,178,178, 178,178,178,178,178,0,178,178, 166,166,0,166,166,166,166,166, 166,166,166,166,166,166,166,166,
   166,166,166,166,166,166,166,166, 166,166,166,166,166,166,166,166, 166,166,166,166,166,166,166,166, 166,166,166,166,166,166,0,166,
   166,166,166,166,166,166,166,166, 166,166,166,166,166,166,0,166, 178,178,178,178,178,178,178,178, 178,178,178,178,178,178,178,178,
   },
  {0,0,0,0,0,0,0,0, 0,180,180,0,0,180,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   0,180,127,180,180,180,180,180, 180,180,180,180,180,180,180,180, 180,180,180,180,180,180,180,180, 180,180,180,180,180,180,180,180,
   180,180,180,180,180,180,127,180, 180,180,180,180,180,180,180,180, 180,180,127,180,180,180,180,180, 180,180,180,180,180,180,180,180,
   180,180,180,180,180,180,180,180, 180,180,180,180,180,180,180,180, 180,180,180,180,180,180,180,180, 180,180,180,180,127,180,180,0,
   176,176,176,176,176,176,176,176, 176,176,176,176,176,176,176,176, 176,176,176,176,176,176,176,176, 176,176,176,0,0,0,176,176,
   0,176,176,176,176,176,176,176, 176,176,176,176,176,0,176,176, 176,176,0,176,176,176,176,176, 176,176,176,176,176,176,176,176,
   176,176,176,176,176,176,176,176, 176,176,176,176,176,176,176,176, 176,176,176,176,176,176,176,176, 176,176,176,176,176,176,0,176,
   176,176,176,176,176,176,176,176, 176,176,176,176,176,176,0,176, 176,176,176,176,176,176,176,176, 176,176,176,176,176,176,176,176,
   },
  {0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   2,0,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,0,2,2,2,2,2,2, 0,0,0,0,0,0,0,0,
   2,0,2,2,2,2,2,2, 0,0,0,0,0,0,0,0, 2,0,2,2,2,2,2,2, 0,0,0,0,0,0,0,0,
   2,0,2,2,2,2,2,2, 0,0,0,0,0,0,0,0, 2,0,2,2,2,2,2,2, 2,2,2,2,2,2,2,2,
   132,0,132,132,132,132,132,132, 130,130,130,130,130,130,130,130, 136,0,134,134,134,134,134,134, 130,130,130,130,130,130,130,130,
   136,0,134,134,134,134,134,134, 130,130,130,130,130,130,130,130, 136,0,134,134,134,134,134,134, 130,130,130,130,130,130,130,130,
   136,0,134,136,134,134,136,134, 130,130,130,130,130,130,130,130, 136,0,134,134,134,134,134,134, 130,130,130,130,130,130,130,130,
   136,0,134,134,134,134,134,134, 130,130,130,130,130,130,130,130, 136,0,134,136,134,134,136,134, 130,130,130,130,130,130,130,130,
   },
},

{ // BHASKAR (0.047M chars) [48]
  {NULL, NULL, NULL, NULL},
  141, 199, 66, 34, 132,
    {174,174,174,174,174,174,174,174, 174,182,182,174,174,182,174,174, 174,174,174,174,174,174,174,174, 174,174,174,0,0,0,174,174,
   0,182,182,182,181,182,182,182, 182,182,182,182,182,0,182,182, 182,182,0,182,182,182,182,182, 182,182,182,182,182,182,182,182,
   182,182,182,182,182,182,182,182, 182,182,182,182,182,182,182,182, 182,182,182,182,182,182,182,182, 182,182,182,182,182,182,0,182,
   182,182,182,182,182,182,182,182, 182,182,182,182,182,182,0,181, 182,182,182,182,182,182,182,182, 182,182,182,182,182,182,182,182,
   166,166,166,166,166,166,166,166, 166,178,178,166,166,178,166,166, 166,166,166,166,166,166,166,166, 166,166,166,0,0,0,166,166,
   0,178,178,178,178,178,178,178, 178,178,178,178,178,0,178,178, 166,166,0,166,166,166,166,166, 166,166,166,166,166,166,166,166,
   166,166,166,166,166,166,166,166, 166,166,166,166,166,166,166,166, 166,166,166,166,166,166,166,166, 166,166,166,166,166,166,0,166,
   166,166,166,166,166,166,166,166, 166,166,166,166,166,166,0,166, 178,178,178,178,178,178,178,178, 178,178,178,178,178,178,178,178,
   },
  {0,0,0,0,0,0,0,0, 0,180,180,0,0,180,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   0,180,180,180,180,180,180,180, 180,180,180,180,180,180,180,180, 180,180,180,180,180,180,180,180, 180,180,180,180,180,180,180,180,
   180,180,180,180,180,180,126,180, 180,180,180,180,180,180,180,180, 180,180,126,180,180,180,180,180, 180,180,180,180,180,180,180,180,
   180,180,180,180,180,180,180,180, 180,180,180,180,180,180,180,180, 180,180,180,180,180,180,180,180, 180,180,180,180,180,180,180,0,
   176,176,176,176,176,176,176,176, 176,176,176,176,176,176,176,176, 176,176,176,176,176,176,176,176, 176,176,176,0,0,0,176,176,
   0,176,176,176,176,176,176,176, 176,176,176,176,176,0,176,176, 176,176,0,176,176,176,176,176, 176,176,176,176,176,176,176,176,
   176,176,176,176,176,176,176,176, 176,176,176,176,176,176,176,176, 176,176,176,176,176,176,176,176, 176,176,176,176,176,176,0,176,
   176,176,176,176,176,176,176,176, 176,176,176,176,176,176,0,176, 176,176,176,176,176,176,176,176, 176,176,176,176,176,176,176,176,
   },
  {0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   2,0,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,0,2,2,2,2,2,2, 0,0,0,0,0,0,0,0,
   2,0,2,2,2,2,2,2, 0,0,0,0,0,0,0,0, 2,0,2,2,2,2,2,2, 0,0,0,0,0,0,0,0,
   2,0,2,2,2,2,2,2, 0,0,0,0,0,0,0,0, 2,0,2,2,2,2,2,2, 2,2,2,2,2,2,2,2,
   132,0,132,132,132,132,132,132, 130,130,130,130,130,130,130,130, 134,0,134,134,134,134,134,134, 130,130,130,130,130,130,130,130,
   136,0,134,134,134,134,134,134, 130,130,130,130,130,130,130,130, 136,0,134,134,134,134,134,134, 130,130,130,130,130,130,130,130,
   134,0,134,136,134,134,136,134, 130,130,130,130,130,130,130,130, 136,0,134,134,134,134,134,134, 130,130,130,130,130,130,130,130,
   136,0,134,134,134,134,134,134, 130,130,130,130,130,130,130,130, 134,0,134,136,134,134,136,134, 130,130,130,130,130,130,130,130,
   },
},

{ // HTCHANAKYA (0.041M chars) [49]
  {NULL, NULL, NULL, NULL},
  142, 202, 68, 32, 133,
    {173,0,0,0,173,173,173,173, 173,182,171,173,105,171,0,0, 0,173,173,0,173,173,173,173, 0,0,173,0,0,0,0,0,
   181,182,182,182,181,182,182,182, 182,182,182,182,182,0,182,182, 182,182,182,182,182,182,182,0, 182,182,182,182,182,182,182,182,
   182,182,182,182,182,182,182,182, 182,182,182,182,182,182,182,182, 182,182,182,182,182,182,182,182, 182,182,182,182,182,182,182,182,
   182,182,182,182,182,182,182,182, 182,182,182,182,182,182,0,181, 182,182,182,182,182,182,182,182, 182,182,182,182,182,182,182,182,
   169,0,0,0,169,169,169,169, 169,180,170,169,0,170,0,0, 0,169,169,0,169,169,169,169, 0,0,169,0,0,0,0,0,
   0,180,180,180,179,180,180,180, 180,180,180,180,180,0,180,180, 169,169,169,169,169,169,169,0, 169,169,169,169,169,169,169,169,
   169,169,169,169,169,169,169,169, 169,169,169,169,169,169,169,169, 169,169,169,169,169,169,169,169, 169,169,169,169,169,169,169,169,
   169,169,169,169,169,169,169,169, 169,169,169,169,169,169,0,168, 180,180,180,180,180,180,180,180, 180,180,180,180,180,180,180,180,
   },
  {0,0,0,0,0,0,0,0, 0,181,181,0,0,181,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   0,181,129,181,181,181,181,181, 181,181,181,181,181,181,181,181, 181,181,181,181,181,181,181,181, 181,181,181,181,181,181,181,181,
   181,181,181,181,181,181,181,181, 181,181,181,129,181,181,181,181, 181,181,181,181,181,181,181,181, 181,181,181,181,181,181,181,181,
   181,181,181,181,181,181,181,181, 181,181,181,181,181,181,181,181, 181,181,181,181,181,181,181,181, 181,181,181,181,181,181,181,0,
   176,0,0,0,176,176,176,176, 176,176,0,176,0,0,0,0, 0,176,176,0,176,176,176,176, 0,0,176,0,0,0,0,0,
   175,176,176,176,176,176,176,176, 176,176,176,176,176,0,176,176, 176,176,176,176,176,176,176,0, 176,176,176,176,176,176,176,176,
   176,176,176,176,176,176,176,176, 176,176,176,176,176,176,176,176, 176,176,176,176,176,176,176,176, 176,176,176,176,176,176,176,176,
   176,176,176,176,176,176,176,176, 176,176,176,176,176,176,0,176, 176,176,176,176,176,176,176,176, 176,176,176,176,176,176,176,176,
   },
  {0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   2,0,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,0,2,2,2,2,2,2, 0,0,0,0,0,0,0,0,
   2,0,2,2,2,2,2,2, 0,0,0,0,0,0,0,0, 2,0,2,2,2,2,2,2, 0,0,0,0,0,0,0,0,
   2,0,2,2,2,2,2,2, 0,0,0,0,0,0,0,0, 2,0,2,2,2,2,2,2, 2,2,2,2,2,2,2,2,
   130,0,128,130,128,130,130,128, 128,128,128,128,128,128,128,128, 134,0,134,134,134,134,134,134, 132,132,132,132,132,132,132,132,
   134,0,134,134,134,134,134,134, 132,132,132,132,132,132,132,132, 134,0,134,134,134,134,134,134, 132,132,132,132,132,132,132,132,
   134,0,134,134,134,134,134,134, 132,132,132,132,132,132,132,132, 134,0,134,134,134,134,134,134, 132,132,132,132,132,132,132,132,
   134,0,134,134,134,134,134,134, 132,132,132,132,132,132,132,132, 134,0,134,134,134,134,134,134, 132,132,132,132,132,132,132,132,
   },
},

{ // TSCII (0.047M chars) [50]
  {NULL, NULL, NULL, NULL},
  141, 199, 66, 33, 134,
    {173,173,173,173,173,173,173,173, 173,182,182,173,173,182,173,173, 173,173,173,173,173,173,173,173, 173,173,173,173,173,173,0,173,
   0,181,181,181,181,181,157,157, 157,181,0,181,181,181,181,181, 181,181,181,181,181,181,181,181, 181,181,181,181,181,181,181,181,
   181,181,181,181,181,181,181,181, 181,181,181,181,181,181,181,181, 181,181,181,181,181,181,181,181, 181,181,181,181,181,181,181,181,
   181,181,181,181,181,181,181,181, 181,181,181,181,181,181,181,181, 181,181,181,181,181,181,181,181, 181,181,181,181,181,181,181,0,
   167,167,167,167,167,167,167,167, 167,179,179,167,167,167,167,167, 167,167,175,168,170,167,167,167, 167,167,167,167,167,167,0,167,
   0,178,178,178,178,178,0,0, 0,183,0,178,178,178,178,178, 167,167,167,167,167,167,167,167, 167,167,167,167,167,167,167,167,
   167,167,167,167,167,167,167,167, 167,167,167,167,167,167,167,167, 167,167,167,167,167,167,167,167, 167,167,167,167,167,167,167,167,
   167,167,167,167,167,167,167,167, 167,167,167,167,167,167,167,167, 178,178,178,178,178,178,178,178, 178,178,178,178,178,178,178,0,
   },
  {0,0,0,0,0,0,0,0, 0,179,179,0,0,125,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   170,179,179,179,179,179,179,179, 179,179,179,179,179,179,179,179, 179,179,179,179,179,179,179,179, 179,179,179,179,179,179,179,179,
   179,179,179,179,179,179,179,179, 179,179,179,179,179,179,179,179, 179,179,179,179,179,179,179,179, 179,179,179,179,179,179,179,179,
   179,180,179,179,179,180,179,179, 179,180,179,179,179,179,179,179, 179,179,179,182,179,179,179,179, 179,179,179,179,179,179,179,0,
   175,175,175,175,175,175,175,175, 175,175,175,175,175,175,175,175, 175,175,175,175,175,175,175,175, 175,175,175,175,175,175,0,175,
   0,175,175,175,175,175,0,0, 0,175,0,175,175,175,175,175, 175,175,175,175,175,175,175,175, 175,175,175,175,175,175,175,175,
   175,175,175,175,175,175,175,175, 175,175,175,175,175,175,175,175, 175,175,175,175,175,175,175,175, 175,175,175,175,175,175,175,175,
   175,175,175,175,175,175,175,175, 175,175,175,175,175,175,175,175, 175,175,175,175,175,175,175,175, 175,175,175,175,175,175,175,0,
   },
  {0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   2,0,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,0,2,2,2,2,2,2, 0,0,0,0,0,0,0,0,
   2,0,2,2,2,2,2,2, 0,0,0,0,0,0,0,0, 2,0,2,2,2,2,2,2, 0,0,0,0,0,0,0,0,
   2,0,2,2,2,2,2,2, 0,0,0,0,0,0,0,0, 2,0,2,2,2,2,2,2, 2,2,2,2,2,2,2,2,
   132,0,132,132,132,132,132,132, 128,128,130,128,128,128,128,128, 132,0,134,132,132,132,134,136, 130,130,132,130,130,130,130,130,
   134,0,136,134,134,134,134,134, 132,130,130,132,132,130,130,130, 134,0,134,134,134,134,134,134, 130,130,132,130,130,130,130,130,
   134,0,134,134,134,134,134,134, 130,130,132,130,130,130,130,130, 134,0,134,134,134,134,134,134, 130,130,132,130,130,130,130,130,
   134,0,134,134,134,134,134,134, 130,130,132,130,130,130,130,130, 134,0,134,134,134,134,134,134, 130,130,132,130,130,130,130,130,
   },
},

{ // TAM (0.036M chars) [51]
  {NULL, NULL, NULL, NULL},
  140, 203, 70, 33, 133,
    {0,0,174,174,174,174,174,174, 174,184,184,174,174,174,0,0, 0,0,0,0,0,0,0,0, 174,174,174,174,174,0,0,174,
   0,183,183,183,183,0,183,183, 183,0,161,161,161,0,183,183, 183,183,183,183,183,183,183,0, 183,183,183,183,183,183,183,183,
   183,183,183,183,183,183,183,183, 183,183,0,183,183,183,183,183, 0,0,0,0,0,0,183,183, 183,183,183,183,183,183,183,183,
   183,183,183,183,183,183,183,183, 183,183,183,183,183,183,183,183, 183,183,183,183,183,183,183,183, 183,183,183,183,183,183,183,183,
   0,0,170,170,170,170,170,170, 170,181,181,170,170,0,0,0, 0,0,0,0,0,0,0,0, 170,170,170,170,170,0,0,170,
   0,181,181,181,181,0,181,181, 181,0,0,0,0,0,181,181, 170,170,170,170,170,170,170,0, 170,170,170,170,170,170,170,170,
   170,170,170,170,170,170,170,170, 170,170,0,170,170,170,170,170, 0,0,0,0,0,0,170,170, 170,170,170,170,170,170,170,170,
   170,170,170,170,170,170,170,170, 170,170,170,170,170,170,170,170, 181,181,181,181,181,181,181,181, 181,181,181,181,181,181,181,181,
   },
  {0,0,0,0,0,0,0,0, 0,182,182,0,0,132,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   0,182,182,182,182,182,182,182, 182,182,182,182,182,182,182,182, 182,182,182,182,182,182,182,182, 182,182,182,182,182,182,182,182,
   182,182,182,182,182,182,182,182, 182,182,182,182,182,182,182,182, 182,182,182,182,182,182,182,182, 182,182,182,132,132,132,132,132,
   132,182,182,182,182,182,182,182, 182,182,182,182,182,182,182,182, 182,182,182,182,182,182,182,182, 182,182,182,132,132,132,132,0,
   0,0,177,177,177,177,177,177, 177,177,177,177,177,0,0,0, 0,0,0,0,0,0,0,0, 177,177,177,177,177,0,0,177,
   0,177,177,177,177,0,177,177, 177,0,0,0,0,0,177,177, 177,177,177,177,177,177,177,0, 177,177,177,177,177,177,177,177,
   177,177,177,177,177,177,177,177, 177,177,0,177,177,177,177,177, 0,0,0,0,0,0,177,177, 177,177,177,177,177,177,177,177,
   177,177,177,177,177,177,177,177, 178,178,178,178,178,178,178,178, 178,178,178,178,178,178,178,178, 178,178,178,178,178,178,177,177,
   },
  {0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   2,0,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,0,2,2,2,2,2,2, 0,0,0,0,0,0,0,0,
   2,0,2,2,2,2,2,2, 0,0,0,0,0,0,0,0, 2,0,2,2,2,2,2,2, 0,0,0,0,0,0,0,0,
   2,0,2,2,2,2,2,2, 0,0,0,0,0,0,0,0, 2,0,2,2,2,2,2,2, 2,2,2,2,2,2,2,2,
   132,0,132,132,132,132,132,132, 130,130,130,128,128,128,128,128, 134,0,136,136,136,134,136,134, 132,132,132,134,134,132,132,132,
   134,0,134,134,134,134,134,134, 132,132,132,132,132,132,134,134, 134,0,134,134,134,134,134,134, 132,134,132,132,132,132,132,132,
   134,0,134,134,134,134,134,134, 132,134,132,132,132,132,132,132, 134,0,134,134,134,134,134,134, 132,132,134,132,132,132,132,132,
   134,0,134,134,134,134,134,134, 132,134,132,132,132,132,132,132, 134,0,134,134,134,134,134,134, 132,134,132,132,132,132,132,132,
   },
},

{ // TAB (0.030M chars) [52]
  {NULL, NULL, NULL, NULL},
  137, 210, 72, 28, 132,
    {0,0,0,0,0,0,0,0, 0,174,174,0,0,174,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   0,0,183,183,183,0,183,183, 183,0,165,165,165,0,183,183, 183,183,183,183,183,183,183,0, 183,183,183,183,183,183,183,183,
   183,183,183,183,183,183,183,183, 183,183,0,183,183,183,183,183, 0,0,0,0,0,0,183,0, 0,0,0,183,183,183,183,183,
   183,183,183,183,183,183,183,183, 183,183,183,183,183,183,183,183, 183,183,183,183,183,183,183,183, 183,183,183,183,183,183,183,183,
   0,0,0,0,0,0,0,0, 0,174,174,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   0,0,186,186,186,0,186,186, 186,0,0,0,0,0,186,186, 177,177,177,177,177,177,177,0, 177,177,177,177,177,177,177,177,
   177,177,177,177,177,177,177,177, 177,177,0,177,177,177,177,177, 0,0,0,0,0,0,177,0, 0,0,0,177,177,177,177,177,
   177,177,177,177,177,177,177,177, 177,177,177,177,177,177,177,177, 186,186,186,186,186,186,186,186, 186,186,186,186,186,186,186,186,
   },
  {0,0,0,0,0,0,0,0, 0,183,183,0,0,136,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   0,183,183,183,183,183,183,183, 183,183,183,183,183,183,183,183, 183,183,183,183,183,183,183,183, 183,183,183,183,183,183,183,183,
   183,183,183,183,183,183,183,183, 183,183,183,183,183,183,183,183, 183,183,183,183,183,183,183,183, 183,183,183,183,183,183,183,183,
   183,183,183,183,183,183,183,183, 183,183,183,183,183,183,183,183, 183,183,183,183,183,183,183,183, 183,183,183,183,183,183,183,0,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   0,0,176,176,176,0,176,176, 176,0,0,0,0,0,176,176, 176,176,176,176,176,176,176,0, 176,176,176,176,176,176,176,176,
   176,176,176,176,176,176,176,176, 176,176,0,176,176,176,176,176, 0,0,0,0,0,0,176,0, 0,0,0,176,176,176,176,176,
   176,176,176,176,176,176,176,176, 177,177,177,177,177,177,177,177, 177,177,177,177,177,177,177,177, 177,177,177,177,177,177,176,176,
   },
  {0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   2,0,2,2,2,2,2,2, 128,128,2,2,2,2,2,2, 2,0,2,2,2,2,2,2, 0,0,0,0,0,0,0,0,
   2,0,2,2,2,2,2,2, 0,0,0,0,0,0,0,0, 2,0,2,2,2,2,2,2, 0,0,0,0,0,0,0,0,
   2,0,2,2,2,2,2,2, 0,0,0,0,0,0,0,0, 2,0,2,2,2,2,2,2, 128,128,2,2,2,2,2,2,
   2,0,2,2,2,2,2,2, 128,128,2,2,2,2,2,2, 128,0,128,128,128,128,128,128, 128,128,128,128,128,128,128,128,
   132,0,132,134,134,134,134,132, 128,128,134,134,134,134,138,138, 132,0,132,132,132,132,132,132, 128,128,136,136,136,136,136,136,
   132,0,132,132,132,132,132,132, 128,128,136,136,136,136,136,136, 132,0,134,134,134,134,134,134, 128,128,136,136,136,136,136,136,
   132,0,132,134,134,134,134,132, 128,128,136,136,136,136,136,136, 132,0,132,134,134,134,134,132, 128,128,136,136,136,136,136,136,
   },
},

{ // EUC-CN (0.035M chars) [53]
  {NULL, NULL, NULL, NULL},
  197, 192, 37, 32, 128,
    {0,0,0,0,0,0,0,0, 0,169,169,0,0,169,119,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   0,240,216,176,191,186,168,185, 184,166,0,0,118,0,0,0, 0,103,0,0,0,0,29,0, 0,0,0,0,0,0,0,0,
   0,0,169,0,197,197,190,192, 190,212,188,187,188,186,190,188, 187,186,186,185,184,186,187,187, 185,185,186,185,193,186,186,190,
   185,185,185,187,189,185,186,191, 185,185,185,184,186,185,184,184, 185,184,184,184,184,184,184,184, 186,184,184,184,184,173,0,0,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   0,147,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   },
  {0,0,0,0,0,0,0,0, 0,134,134,0,0,134,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   0,134,134,134,134,134,134,134, 134,134,134,134,134,134,134,134, 134,134,134,134,134,134,134,134, 134,134,134,134,134,134,134,134,
   134,134,134,134,138,134,138,134, 134,138,134,134,138,134,134,134, 134,134,134,134,138,138,134,134, 134,134,134,134,134,134,134,134,
   134,134,134,134,134,134,134,134, 134,134,134,134,138,134,134,134, 134,134,134,134,134,134,134,134, 134,134,134,134,134,134,134,0,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   170,230,193,197,184,184,184,184, 210,184,185,182,188,181,181,185, 183,207,189,182,181,182,181,180, 181,181,182,185,181,179,187,187,
   183,180,180,181,182,180,186,180, 180,185,195,191,183,180,182,180, 179,182,189,183,182,180,192,195, 179,179,181,182,181,179,183,183,
   180,180,178,184,182,198,195,192, 180,183,186,180,180,183,181,181, 183,179,180,189,179,179,178,178, 182,179,212,180,186,180,211,0,
   },
  {182,0,182,182,180,182,182,182, 128,128,128,128,128,128,128,128, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   2,0,2,2,2,2,2,2, 128,128,2,10,14,14,12,8, 76,0,52,52,50,52,52,52, 128,128,128,128,128,128,128,128,
   4,0,2,2,2,2,2,2, 128,128,128,128,128,128,128,128, 8,0,2,2,2,2,2,2, 128,128,128,128,128,128,128,128,
   8,0,2,2,2,2,2,2, 128,128,128,128,128,128,128,128, 12,0,2,2,2,2,2,2, 128,128,128,128,128,128,128,128,
   0,0,0,0,0,0,0,0, 0,0,76,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   0,0,0,0,0,0,0,0, 0,0,134,124,114,118,122,124, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   0,0,0,0,0,0,0,0, 0,0,116,126,130,128,126,136, 0,0,0,0,0,0,0,0, 0,0,116,128,136,132,134,124,
   0,0,0,0,0,0,0,0, 0,0,118,130,132,134,130,124, 0,0,0,0,0,0,0,0, 0,0,118,130,132,134,130,126,
   },
},

{ // EUC (15478 chars) [54]
  {NULL, NULL, NULL, NULL},
  197, 200, 38, 33, 129,
    {0,0,0,0,0,0,0,0, 0,173,173,0,0,173,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   0,234,211,180,190,188,173,189, 189,170,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   0,0,174,0,193,195,190,191, 191,208,190,189,190,189,191,190, 190,189,190,189,189,190,189,190, 189,189,190,189,193,189,190,191,
   189,189,189,190,191,190,190,193, 189,190,189,189,190,189,189,189, 189,189,189,189,189,189,189,189, 190,189,189,189,189,178,0,0,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   },
  {0,0,0,0,0,0,0,0, 0,140,140,0,0,140,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   0,140,140,140,140,140,140,140, 140,140,140,140,140,140,140,140, 140,140,140,140,140,140,140,140, 140,140,140,140,140,140,140,140,
   140,140,140,140,140,140,140,140, 140,140,140,140,140,140,140,140, 140,140,140,140,140,140,140,140, 140,140,140,140,140,140,140,140,
   140,140,140,140,140,140,140,140, 140,140,140,140,140,140,140,140, 140,140,140,140,140,140,140,140, 140,140,140,140,140,140,140,0,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   162,226,192,195,186,186,185,186, 205,186,187,185,188,185,185,186, 184,202,187,184,184,184,183,183, 184,184,184,185,183,183,187,187,
   183,183,183,183,184,183,186,183, 183,185,192,189,184,183,184,183, 182,184,187,184,183,182,190,192, 182,182,182,183,184,182,184,184,
   182,182,182,185,184,194,191,190, 183,184,185,183,183,184,183,183, 184,182,182,188,182,182,182,182, 184,182,206,183,185,182,206,0,
   },
  {176,0,176,176,176,176,176,176, 128,128,128,128,128,128,128,128, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   2,0,2,2,2,2,2,2, 128,128,128,128,128,128,128,128, 128,0,128,128,128,128,128,128, 128,128,128,128,128,128,128,128,
   2,0,2,2,2,2,2,2, 128,128,128,128,128,128,128,128, 2,0,2,2,2,2,2,2, 128,128,128,128,128,128,128,128,
   2,0,2,2,2,2,2,2, 128,128,128,128,128,128,128,128, 2,0,2,2,2,2,2,2, 128,128,128,128,128,128,128,128,
   0,0,0,0,0,0,0,0, 0,0,2,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   0,0,0,0,0,0,0,0, 0,0,136,124,116,118,122,124, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   0,0,0,0,0,0,0,0, 0,0,118,126,128,128,128,136, 0,0,0,0,0,0,0,0, 0,0,118,128,132,130,130,126,
   0,0,0,0,0,0,0,0, 0,0,120,128,130,132,130,126, 0,0,0,0,0,0,0,0, 0,0,120,130,130,130,130,126,
   },
},

{ // CNS (15478 chars) [55]
  {NULL, NULL, NULL, NULL},
  197, 200, 38, 33, 129,
    {0,0,0,0,0,0,0,0, 0,173,173,0,0,173,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   0,234,211,180,190,188,173,189, 189,170,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   0,0,174,0,193,195,190,191, 191,208,190,189,190,189,191,190, 190,189,190,189,189,190,189,190, 189,189,190,189,193,189,190,191,
   189,189,189,190,191,190,190,193, 189,190,189,189,190,189,189,189, 189,189,189,189,189,189,189,189, 190,189,189,189,189,178,0,0,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   },
  {0,0,0,0,0,0,0,0, 0,140,140,0,0,140,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   0,140,140,140,140,140,140,140, 140,140,140,140,140,140,140,140, 140,140,140,140,140,140,140,140, 140,140,140,140,140,140,140,140,
   140,140,140,140,140,140,140,140, 140,140,140,140,140,140,140,140, 140,140,140,140,140,140,140,140, 140,140,140,140,140,140,140,140,
   140,140,140,140,140,140,140,140, 140,140,140,140,140,140,140,140, 140,140,140,140,140,140,140,140, 140,140,140,140,140,140,140,0,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   162,226,192,195,186,186,185,186, 205,186,187,185,188,185,185,186, 184,202,187,184,184,184,183,183, 184,184,184,185,183,183,187,187,
   183,183,183,183,184,183,186,183, 183,185,192,189,184,183,184,183, 182,184,187,184,183,182,190,192, 182,182,182,183,184,182,184,184,
   182,182,182,185,184,194,191,190, 183,184,185,183,183,184,183,183, 184,182,182,188,182,182,182,182, 184,182,206,183,185,182,206,0,
   },
  {176,0,176,176,176,176,176,176, 128,128,128,128,128,128,128,128, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   2,0,2,2,2,2,2,2, 128,128,128,128,128,128,128,128, 128,0,128,128,128,128,128,128, 128,128,128,128,128,128,128,128,
   2,0,2,2,2,2,2,2, 128,128,128,128,128,128,128,128, 2,0,2,2,2,2,2,2, 128,128,128,128,128,128,128,128,
   2,0,2,2,2,2,2,2, 128,128,128,128,128,128,128,128, 2,0,2,2,2,2,2,2, 128,128,128,128,128,128,128,128,
   0,0,0,0,0,0,0,0, 0,0,2,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   0,0,0,0,0,0,0,0, 0,0,136,124,116,118,122,124, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   0,0,0,0,0,0,0,0, 0,0,118,126,128,128,128,136, 0,0,0,0,0,0,0,0, 0,0,118,128,132,130,130,126,
   0,0,0,0,0,0,0,0, 0,0,120,128,130,132,130,126, 0,0,0,0,0,0,0,0, 0,0,120,130,130,130,130,126,
   },
},

{ // UTF-32BE (1032.458M chars) [56]
  {NULL, NULL, NULL, NULL},
  77, 151, 56, 41, 127,
    {250,137,119,116,158,116,151,92, 103,89,82,111,111,75,204,151, 102,106,71,86,94,92,104,91, 88,92,106,215,97,93,99,106,
   79,97,116,106,115,126,78,66, 89,96,94,80,79,92,90,90, 93,101,90,90,72,96,79,91, 82,89,78,88,80,99,84,86,
   86,92,83,59,101,100,101,79, 89,93,71,46,102,89,64,68, 78,74,72,65,71,56,71,64, 76,84,65,68,92,98,94,106,
   182,177,181,215,194,207,203,198, 193,188,166,179,184,171,90,183, 67,2,0,27,20,10,16,20, 7,0,3,46,24,71,43,104,
   160,104,104,120,120,123,106,99, 106,101,107,114,119,100,91,96, 110,88,107,102,79,91,100,101, 99,92,100,94,88,67,104,92,
   68,80,83,73,92,62,75,75, 79,64,69,73,88,82,78,88, 92,73,82,84,71,76,74,82, 84,98,91,76,78,66,71,80,
   78,89,78,91,66,88,86,94, 97,83,76,79,62,78,71,75, 80,86,62,78,68,93,85,80, 83,80,76,75,69,83,90,83,
   82,82,61,78,58,98,56,64, 68,80,85,85,74,85,73,74, 60,46,58,83,68,31,72,64, 71,62,78,78,88,72,62,103,
   },
  {231,96,83,86,82,95,79,87, 91,190,189,71,101,175,87,75, 88,93,82,83,89,84,75,84, 89,98,94,150,95,84,82,97,
   211,164,198,158,206,163,163,167, 204,160,139,149,163,176,181,194, 186,180,173,172,169,172,164,166, 171,171,175,177,193,193,193,164,
   169,173,164,169,169,167,165,166, 164,164,153,153,160,162,163,160, 163,145,162,165,167,153,156,157, 144,147,146,140,138,138,110,171,
   103,201,181,192,192,203,181,185, 187,200,161,171,196,187,196,196, 190,145,197,197,203,183,178,178, 170,174,157,147,130,147,113,85,
   194,202,196,197,161,172,160,164, 174,168,169,165,163,162,161,172, 167,152,151,154,172,173,173,172, 170,167,172,172,181,172,158,158,
   158,165,152,155,174,160,165,166, 161,156,155,157,154,167,173,166, 170,159,161,164,160,162,155,162, 190,174,180,181,186,173,165,168,
   96,98,83,90,99,87,83,90, 91,88,75,80,102,90,78,81, 99,97,81,86,97,102,86,85, 95,107,84,92,106,96,96,92,
   105,120,91,98,114,104,86,102, 104,125,102,72,80,118,81,79, 79,91,70,110,91,83,108,69, 93,74,94,86,107,97,65,124,
   },
  {132,132,126,132,132,132,132,132, 122,128,140,130,230,224,222,224, 60,24,152,14,68,74,40,40, 104,122,122,98,204,218,206,224,
   8,96,12,22,38,48,10,20, 44,64,80,56,128,128,128,128, 20,128,26,34,56,70,22,32, 40,58,72,50,128,128,128,128,
   20,128,24,34,54,66,22,32, 38,56,70,50,128,128,128,128, 26,128,30,40,62,74,28,38, 40,60,76,52,128,128,128,128,
   2,2,2,2,2,2,2,2, 42,62,78,54,128,128,128,128, 134,180,74,128,144,158,114,92, 116,128,122,78,150,192,132,164,
   118,142,64,114,106,120,82,106, 14,12,26,10,92,80,68,72, 138,192,94,128,132,154,112,108, 34,52,40,14,124,118,102,106,
   142,188,72,144,148,176,108,110, 156,152,90,112,202,162,200,174, 142,200,86,138,146,162,110,118, 156,146,132,106,208,200,212,192,
   142,208,88,138,138,162,96,118, 154,152,138,122,208,212,182,178, 138,180,68,144,148,178,114,122, 154,162,112,112,184,192,158,168,
   140,128,102,142,150,172,108,116, 148,148,148,148,20,14,4,12, 136,196,112,138,136,176,110,118, 132,148,136,112,148,160,162,150,
   },
},

{ // UTF-32LE (1032.461M chars) [57]
  {NULL, NULL, NULL, NULL},
  77, 152, 56, 41, 127,
    {250,137,119,116,158,116,151,92, 103,89,82,111,111,75,204,150, 102,106,71,86,94,92,104,91, 88,92,106,215,97,93,99,106,
   79,97,116,106,114,126,78,66, 89,96,94,144,79,92,90,90, 93,101,90,90,72,96,79,91, 80,89,78,88,80,99,84,86,
   86,92,83,58,101,100,101,79, 89,93,71,46,102,89,64,68, 78,74,72,65,71,54,71,64, 76,84,65,68,92,98,94,106,
   182,177,181,215,193,207,203,198, 193,188,166,178,184,171,90,183, 67,3,0,27,20,11,17,20, 8,0,4,46,24,71,162,104,
   160,104,104,120,120,123,106,99, 106,101,107,114,118,100,91,94, 110,88,107,102,79,91,100,101, 99,92,100,94,88,67,104,92,
   68,80,83,73,92,62,75,75, 79,64,71,124,88,82,78,88, 92,73,82,84,71,76,74,82, 84,98,91,76,78,66,71,80,
   78,89,78,91,66,88,86,94, 97,83,76,79,62,78,71,75, 80,86,62,78,68,93,85,80, 83,80,76,75,69,83,90,83,
   82,82,61,78,57,98,57,64, 68,80,85,85,74,85,73,74, 60,46,57,83,68,31,72,64, 71,62,78,78,88,72,122,103,
   },
  {231,96,83,86,82,95,79,87, 91,190,189,71,101,175,87,75, 88,93,82,83,89,84,75,84, 88,98,94,158,95,84,82,97,
   211,164,198,159,206,163,163,167, 204,160,139,149,163,176,181,194, 186,180,174,172,169,172,165,166, 171,171,175,177,193,193,193,164,
   169,173,164,169,169,167,165,166, 164,164,155,154,161,163,165,160, 163,145,162,165,167,154,157,157, 144,147,146,140,138,138,110,171,
   103,201,181,192,192,203,181,185, 187,200,161,171,196,187,196,196, 190,145,197,197,203,183,178,177, 170,174,157,147,130,147,131,85,
   193,202,196,197,161,172,160,164, 174,168,169,165,163,162,161,172, 167,152,151,154,172,173,173,172, 170,167,172,172,181,172,158,158,
   158,165,152,155,174,160,165,166, 161,155,155,157,154,167,173,166, 170,159,161,164,160,162,155,162, 190,174,180,181,186,173,165,168,
   99,99,87,92,100,90,85,93, 93,89,79,83,102,91,81,82, 101,97,82,87,97,102,87,86, 95,107,84,92,106,96,96,92,
   105,120,93,102,115,107,92,103, 105,125,103,77,82,118,81,84, 79,91,70,110,91,83,107,69, 93,74,94,86,107,97,65,124,
   },
  {132,124,126,132,132,132,132,132, 122,128,140,130,228,224,222,224, 60,14,152,16,66,74,40,40, 104,122,122,98,204,218,206,224,
   44,212,2,12,34,104,44,134, 116,112,144,150,220,208,220,130, 20,116,26,34,56,70,22,32, 40,58,72,50,128,128,128,128,
   20,104,24,34,54,66,22,32, 38,56,70,50,128,128,128,128, 26,114,30,40,60,74,28,38, 42,60,76,52,128,128,128,128,
   2,2,2,2,2,2,2,2, 44,62,78,54,128,128,128,128, 84,190,114,148,160,156,70,62, 116,118,160,144,212,200,222,166,
   118,134,64,114,106,120,82,106, 14,12,26,8,90,80,66,72, 138,168,94,128,130,154,112,108, 34,52,40,14,122,116,100,106,
   130,162,60,130,122,146,98,100, 146,140,78,102,202,162,172,162, 142,200,86,138,144,162,110,118, 158,146,132,106,208,200,212,192,
   142,208,88,138,136,162,96,118, 154,152,138,122,208,212,182,176, 138,180,68,144,148,178,114,122, 156,162,112,112,182,192,156,168,
   140,128,102,142,150,172,108,116, 148,148,148,148,18,14,4,12, 124,168,100,124,114,154,98,106, 76,90,78,54,122,120,108,108,
   },
},

{ // X-BINARYENC (0 chars) [58]
  {NULL, NULL, NULL, NULL},
  0, 0, 0, 0, 255,
    {0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   },
  {0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   },
  {128,128,128,128,128,128,128,128, 128,128,128,128,128,128,128,128, 128,128,128,128,128,128,128,128, 128,128,128,128,128,128,128,128,
   128,128,128,128,128,128,128,128, 128,128,128,128,128,128,128,128, 128,128,128,128,128,128,128,128, 128,128,128,128,128,128,128,128,
   128,128,128,128,128,128,128,128, 128,128,128,128,128,128,128,128, 128,128,128,128,128,128,128,128, 128,128,128,128,128,128,128,128,
   128,128,128,128,128,128,128,128, 128,128,128,128,128,128,128,128, 128,128,128,128,128,128,128,128, 128,128,128,128,128,128,128,128,
   128,128,128,128,128,128,128,128, 128,128,128,128,128,128,128,128, 128,128,128,128,128,128,128,128, 128,128,128,128,128,128,128,128,
   128,128,128,128,128,128,128,128, 128,128,128,128,128,128,128,128, 128,128,128,128,128,128,128,128, 128,128,128,128,128,128,128,128,
   128,128,128,128,128,128,128,128, 128,128,128,128,128,128,128,128, 128,128,128,128,128,128,128,128, 128,128,128,128,128,128,128,128,
   128,128,128,128,128,128,128,128, 128,128,128,128,128,128,128,128, 128,128,128,128,128,128,128,128, 128,128,128,128,128,128,128,128,
   },
},

{ // X-UTF8UTF8 (43.271M chars) [59]
  {NULL, NULL, NULL, NULL},
  150, 194, 32, 13, 129,
    {191,104,167,173,203,123,113,0, 0,69,69,80,0,107,0,105, 95,0,125,146,116,0,0,0, 104,129,184,0,125,122,159,0,
   139,144,162,111,112,105,148,117, 115,101,122,104,217,112,130,114, 151,116,107,120,116,116,98,89, 124,119,136,113,115,116,126,105,
   0,0,239,243,62,181,145,62, 62,62,62,147,62,62,62,62, 62,62,62,62,62,62,62,62, 62,62,62,62,62,62,62,62,
   0,0,220,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   0,58,0,0,0,0,5,0, 0,0,36,0,102,23,86,28, 9,0,75,142,74,0,0,0, 89,85,64,0,119,139,99,0,
   121,119,159,71,88,75,139,88, 67,69,78,82,156,71,75,61, 134,70,55,54,61,63,56,60, 80,102,119,63,76,71,134,73,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   },
  {0,0,0,0,0,0,0,0, 0,24,140,0,0,147,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   195,107,138,80,100,83,158,123, 107,139,106,91,157,127,159,122, 111,135,127,120,115,117,108,109, 106,109,120,113,182,78,121,111,
   84,149,138,143,143,145,133,136, 138,145,127,129,143,142,149,143, 141,113,143,148,151,137,137,140, 124,119,115,112,96,117,97,106,
   108,137,132,140,136,131,138,130, 131,133,119,128,140,133,137,128, 135,113,137,152,143,136,130,123, 116,108,106,112,84,77,91,0,
   196,150,226,235,119,165,141,52, 52,52,52,115,52,142,52,124, 121,52,147,165,107,52,52,52, 97,124,126,52,187,166,117,52,
   216,191,222,181,200,177,168,187, 191,218,177,170,169,189,169,153, 169,175,160,194,171,165,189,171, 182,163,176,175,198,160,155,150,
   0,0,205,182,0,189,133,0, 0,0,0,163,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   0,0,209,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   },
  {0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   12,0,2,2,2,2,2,2, 0,0,0,0,2,128,2,128, 66,0,20,36,40,48,50,54, 0,0,0,0,2,128,2,128,
   2,0,2,2,2,2,2,2, 0,0,0,0,128,128,128,128, 128,0,84,128,128,128,128,128, 0,0,0,0,128,128,128,128,
   10,0,2,2,2,2,2,2, 0,0,0,0,128,128,128,128, 128,0,128,128,128,128,128,128, 0,0,0,0,128,128,128,128,
   120,0,130,112,192,198,136,140, 2,168,134,98,134,128,124,128, 160,0,168,162,178,176,178,178, 2,2,2,2,170,128,2,128,
   178,0,174,178,150,154,158,156, 2,2,2,2,158,128,164,128, 156,0,162,152,182,182,182,184, 2,26,2,2,170,128,10,128,
   0,0,128,128,128,128,128,128, 128,108,130,130,2,128,2,128, 0,0,128,128,128,128,128,128, 120,210,124,154,74,128,70,128,
   0,0,128,128,128,128,128,128, 140,2,2,2,2,128,2,128, 0,0,128,128,128,128,128,128, 128,128,128,128,128,128,128,128,
   },
},

{ // X-TAM-ELANGO (0.036M chars) [60]
  {NULL, NULL, NULL, NULL},
  126, 180, 58, 30, 129,
    {0,180,180,180,180,180,180,180, 180,191,191,180,180,191,0,180, 170,170,170,170,170,170,170,170, 170,170,170,170,170,170,0,170,
   0,180,180,180,180,180,180,180, 180,180,180,180,0,0,180,0, 180,180,180,180,180,180,180,180, 180,180,180,180,180,180,180,180,
   180,180,180,180,180,180,0,180, 180,180,180,180,180,180,180,180, 180,180,180,180,180,180,180,180, 180,180,180,180,0,0,180,180,
   180,0,180,180,0,0,0,0, 0,0,0,0,180,0,180,180, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   0,181,181,181,181,181,181,181, 181,191,191,181,181,191,0,181, 171,171,171,171,171,171,171,171, 171,171,171,171,171,171,0,171,
   0,181,181,181,181,181,181,181, 181,181,181,181,0,0,181,0, 181,181,181,181,181,181,181,181, 181,181,181,181,181,181,181,181,
   181,181,181,181,181,181,0,181, 181,181,181,181,181,181,181,181, 181,181,181,181,181,181,181,181, 181,181,181,181,0,0,181,181,
   181,0,181,181,0,0,0,0, 0,0,0,0,181,0,181,181, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   },
  {0,0,0,0,0,0,0,0, 0,180,180,0,0,180,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   0,180,180,180,180,180,180,180, 180,180,180,180,180,180,180,180, 180,180,180,180,180,180,180,180, 180,180,180,180,180,180,180,180,
   180,180,180,180,180,180,180,180, 180,180,180,180,180,180,180,180, 180,180,180,180,180,180,180,180, 180,180,180,180,180,180,180,180,
   180,180,180,180,180,180,180,180, 180,180,180,180,180,180,180,180, 180,180,180,180,180,180,180,180, 180,180,180,180,180,180,180,0,
   0,180,180,180,180,180,180,180, 180,180,180,180,180,180,0,180, 180,180,180,180,180,180,180,180, 180,180,180,180,180,180,0,180,
   0,180,180,180,180,180,180,180, 180,180,180,180,0,0,180,0, 180,180,180,180,180,180,180,180, 180,180,180,180,180,180,180,180,
   180,180,180,180,180,180,0,180, 180,180,180,180,180,180,180,180, 180,180,180,180,180,180,180,180, 180,180,180,180,0,0,180,180,
   180,0,180,180,0,0,0,0, 0,0,0,0,180,0,180,180, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   },
  {110,0,110,110,110,110,110,110, 110,110,110,110,110,110,110,128, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   2,0,2,2,2,2,2,2, 2,2,2,2,2,2,2,128, 2,0,2,2,2,2,2,2, 2,2,2,2,2,2,2,128,
   2,0,2,2,2,2,2,2, 2,2,2,2,2,2,2,128, 2,0,2,2,2,2,2,2, 2,2,2,2,2,2,2,128,
   2,0,2,2,2,2,2,2, 2,2,2,2,2,2,2,128, 128,0,128,128,128,128,128,128, 128,128,128,128,128,128,128,128,
   132,0,132,132,132,132,132,132, 132,132,132,132,132,132,132,128, 136,0,134,134,134,134,134,134, 136,136,136,136,136,136,136,128,
   136,0,136,136,136,136,136,136, 136,136,136,136,136,136,136,128, 136,0,134,134,134,134,134,134, 136,136,136,136,136,136,136,128,
   136,0,134,134,134,134,134,134, 136,136,136,136,136,136,136,128, 136,0,134,134,134,134,134,134, 136,136,136,136,136,136,136,128,
   136,0,136,136,136,136,136,136, 136,136,136,136,136,136,136,128, 128,0,128,128,128,128,128,128, 128,128,128,128,128,128,128,128,
   },
},

{ // X-TAM-LTTMBARANI (0.043M chars) [61]
  {NULL, NULL, NULL, NULL},
  141, 199, 69, 34, 128,
    {0,178,178,0,178,0,178,178, 0,187,187,178,178,176,0,0, 0,0,0,0,0,168,0,0, 0,0,168,168,168,0,0,168,
   178,178,178,178,178,178,178,178, 178,178,178,178,178,178,178,178, 178,178,178,178,178,178,178,178, 178,178,178,178,178,178,178,178,
   178,178,178,178,178,178,178,178, 178,178,178,178,178,178,178,178, 178,178,178,178,178,178,178,178, 178,178,178,178,178,178,178,178,
   178,178,178,178,178,178,178,178, 178,178,178,178,178,178,178,178, 178,178,178,178,178,178,178,178, 178,178,178,178,178,178,178,178,
   0,176,176,0,176,0,176,176, 0,187,187,176,176,178,0,0, 0,0,0,0,0,165,0,0, 0,0,165,165,165,0,0,165,
   0,176,176,176,176,176,177,176, 176,176,176,176,176,176,176,176, 176,176,176,176,176,176,176,182, 176,176,176,176,176,176,176,176,
   176,176,176,176,176,176,176,176, 176,176,176,176,176,176,176,176, 176,176,176,176,176,176,176,176, 176,176,176,176,176,177,176,176,
   176,176,176,176,176,176,176,176, 176,176,176,176,176,176,176,176, 181,176,176,176,176,176,176,176, 176,176,176,176,176,189,176,176,
   },
  {0,0,0,0,0,0,0,0, 0,178,177,0,0,177,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   185,177,177,177,177,177,178,177, 177,177,177,177,178,177,178,177, 177,177,177,177,177,177,177,177, 177,177,177,177,180,177,177,177,
   177,177,177,177,177,177,177,177, 177,177,177,177,177,177,177,177, 177,177,177,177,177,177,177,177, 177,177,177,177,177,177,177,177,
   177,177,177,177,177,177,177,177, 177,177,177,177,177,177,177,177, 177,177,177,177,177,177,177,177, 177,177,177,177,177,177,177,0,
   0,178,178,0,178,0,178,178, 0,178,178,178,178,0,0,0, 0,0,0,0,0,178,0,0, 0,0,178,178,178,0,0,178,
   178,178,178,178,178,178,178,178, 178,178,178,178,178,178,178,178, 178,178,178,178,178,178,178,179, 179,178,178,178,178,178,178,178,
   178,178,178,178,178,178,178,178, 178,178,178,178,178,178,178,178, 178,178,178,178,178,178,178,178, 178,178,178,178,178,178,178,178,
   178,178,178,178,178,178,178,178, 178,178,178,178,178,178,178,178, 178,178,178,178,178,178,178,178, 178,178,178,178,178,178,178,178,
   },
  {116,0,114,116,116,116,116,116, 118,118,116,118,118,118,118,118, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   2,0,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,0,2,2,2,2,2,2, 2,2,2,2,2,2,2,2,
   2,0,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,0,2,2,2,2,2,2, 2,2,2,2,2,2,2,2,
   2,0,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,0,2,2,2,2,2,2, 2,2,2,2,2,2,2,2,
   134,0,132,134,134,134,134,134, 132,132,132,132,132,132,132,132, 138,0,136,138,138,138,138,138, 136,136,136,136,136,136,136,136,
   138,0,138,138,138,138,138,138, 136,136,136,136,136,136,136,136, 138,0,138,138,138,138,138,138, 136,136,136,136,136,136,136,136,
   138,0,136,138,138,138,138,138, 136,136,136,136,136,136,136,136, 138,0,138,138,138,138,138,138, 136,136,136,136,136,136,136,136,
   138,0,136,138,138,138,138,138, 136,136,136,136,136,136,136,136, 136,0,142,138,136,136,136,136, 136,136,136,136,136,136,136,136,
   },
},

{ // X-TAM-SHREE (0.037M chars) [62]
  {NULL, NULL, NULL, NULL},
  140, 204, 70, 30, 129,
    {0,0,0,0,0,0,0,0, 0,188,179,0,0,179,0,0, 0,168,0,0,0,168,0,0, 0,0,0,168,0,0,0,168,
   0,178,178,178,178,178,178,178, 178,178,178,178,0,0,178,178, 178,178,178,178,178,178,0,0, 178,178,178,178,178,178,178,178,
   178,178,178,178,178,178,178,178, 178,178,178,178,178,178,178,178, 178,178,178,178,178,178,178,178, 178,178,178,178,178,178,178,178,
   178,178,178,178,178,178,178,178, 178,178,178,178,178,178,178,178, 178,178,178,178,178,178,178,178, 178,178,178,178,178,178,0,178,
   0,0,0,0,0,0,0,0, 0,188,178,0,0,178,0,0, 0,169,0,0,0,169,0,0, 0,0,0,169,0,0,0,169,
   0,179,179,179,179,179,179,179, 179,179,179,179,0,0,179,179, 179,179,179,179,179,179,0,0, 179,179,179,179,179,179,179,179,
   179,179,179,179,179,179,179,179, 179,179,179,179,179,179,179,179, 179,179,179,179,179,179,179,179, 179,179,179,179,179,179,179,179,
   179,179,179,179,179,179,179,179, 179,179,179,179,179,179,179,179, 179,179,179,179,179,179,179,179, 179,179,179,179,179,179,0,179,
   },
  {0,0,0,0,0,0,0,0, 0,179,179,0,0,179,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   0,179,179,179,179,179,179,179, 179,179,179,179,179,179,179,179, 179,179,179,179,179,179,179,179, 179,179,179,179,179,179,179,179,
   179,179,179,179,179,179,179,179, 179,179,179,179,179,179,179,179, 179,179,179,179,179,179,179,179, 179,179,179,179,179,179,179,179,
   179,179,179,179,179,179,179,179, 179,179,179,179,179,179,179,179, 179,179,179,179,179,179,179,179, 179,179,179,179,179,179,179,0,
   0,0,0,0,0,0,0,0, 0,179,0,0,0,0,0,0, 0,179,0,0,0,179,0,0, 0,0,0,179,0,0,0,179,
   0,179,179,179,179,179,179,179, 179,179,179,179,0,0,179,179, 179,179,179,179,179,179,0,0, 179,179,179,179,179,179,179,179,
   179,179,179,179,179,179,179,179, 179,179,179,179,179,179,179,179, 179,179,179,179,179,179,179,179, 179,179,179,179,179,179,179,179,
   179,179,179,179,179,179,179,179, 179,179,179,179,179,179,179,179, 179,179,179,179,179,179,179,179, 179,179,179,179,179,179,0,179,
   },
  {132,0,132,132,132,132,132,132, 134,134,132,132,132,132,132,132, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   2,0,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,0,2,2,2,2,2,2, 2,2,2,2,2,2,2,2,
   2,0,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,0,2,2,2,2,2,2, 2,2,2,2,2,2,2,2,
   2,0,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,0,2,2,2,2,2,2, 2,2,2,2,2,2,2,2,
   118,0,118,118,118,118,118,118, 118,118,118,118,118,118,118,118, 136,0,136,138,138,138,138,136, 138,138,138,138,138,138,138,138,
   136,0,136,136,136,136,136,136, 138,138,138,138,138,138,138,138, 136,0,136,136,136,136,136,136, 138,138,138,138,138,138,138,138,
   136,0,136,136,136,136,136,136, 138,138,138,138,138,138,138,138, 136,0,136,136,136,136,136,136, 138,138,138,138,138,138,138,138,
   136,0,136,136,136,136,136,136, 138,138,138,138,138,138,138,138, 136,0,136,136,136,136,136,136, 138,138,138,138,138,138,138,138,
   },
},

{ // X-TAM-TBOOMIS (0.038M chars) [63]
  {NULL, NULL, NULL, NULL},
  139, 205, 71, 31, 129,
    {178,0,0,0,0,0,0,0, 0,178,178,0,0,178,0,0, 0,0,0,168,168,0,0,0, 0,0,0,0,0,0,0,0,
   178,178,178,178,178,178,178,178, 178,178,178,178,178,178,178,178, 178,178,178,178,178,178,178,178, 178,178,178,178,178,178,178,178,
   178,178,178,178,178,178,178,178, 178,178,178,178,178,178,178,178, 178,178,178,178,178,178,178,178, 178,178,178,178,178,178,178,178,
   178,178,178,178,178,178,178,178, 178,178,178,178,178,178,178,178, 178,178,178,178,178,178,178,178, 178,178,178,178,178,178,178,178,
   178,0,0,0,0,0,0,0, 0,178,178,0,0,178,0,0, 0,0,0,168,168,0,0,0, 0,0,0,0,0,0,0,0,
   0,178,178,178,178,178,178,178, 178,178,178,178,178,178,178,178, 178,178,178,178,178,178,178,178, 178,178,178,178,178,178,178,178,
   178,178,178,178,178,178,178,178, 178,178,178,178,178,178,178,178, 178,178,178,178,178,178,178,178, 178,178,178,178,178,178,178,178,
   178,178,178,178,178,200,178,178, 178,178,178,178,178,178,178,178, 178,178,178,178,178,178,178,178, 179,178,178,178,181,190,178,188,
   },
  {0,0,0,0,0,0,0,0, 0,178,178,0,0,178,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   200,178,178,178,178,178,178,178, 178,178,178,178,178,178,178,178, 178,178,178,178,178,178,178,178, 178,178,178,178,180,178,178,178,
   178,178,178,178,178,178,178,178, 178,178,178,178,178,178,178,178, 178,178,178,178,178,178,178,178, 178,178,178,178,178,178,178,178,
   178,178,178,178,178,178,178,178, 178,178,178,178,178,178,178,178, 178,178,178,178,178,178,178,178, 178,178,178,178,178,178,178,177,
   178,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,178,178,0,0,0, 0,0,0,0,0,0,0,0,
   177,178,178,178,178,178,178,178, 178,178,178,178,178,178,178,178, 178,178,178,178,178,178,178,178, 178,178,178,178,178,178,178,178,
   178,178,178,178,178,178,178,178, 178,178,178,178,178,178,178,178, 178,178,178,178,178,178,178,178, 178,178,178,178,178,178,178,178,
   178,178,178,178,178,178,178,178, 178,178,178,178,178,178,178,178, 178,178,178,178,178,178,178,178, 178,178,178,178,178,178,178,178,
   },
  {134,0,130,132,132,132,132,132, 132,132,132,132,132,132,132,132, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   2,0,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,0,2,2,2,2,2,2, 2,2,2,2,2,2,2,2,
   2,0,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,0,2,2,2,2,2,2, 2,2,2,2,2,2,2,2,
   2,0,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,0,2,2,2,2,2,2, 94,94,94,94,94,94,94,94,
   118,0,114,116,116,116,116,118, 116,116,118,116,116,116,116,116, 138,0,134,136,136,136,136,138, 136,136,138,136,136,136,136,136,
   138,0,134,136,136,136,136,136, 136,136,138,136,136,136,136,136, 138,0,134,136,136,136,136,138, 136,136,138,136,136,136,136,136,
   138,0,134,136,136,136,136,138, 136,136,138,136,136,136,136,136, 138,0,134,136,136,136,136,138, 136,136,138,136,136,136,136,136,
   134,0,144,134,134,134,134,134, 136,136,138,136,136,136,136,136, 134,0,138,136,134,134,134,134, 136,136,138,136,136,136,136,136,
   },
},

{ // X-TAM-TMNEWS (0.037M chars) [64]
  {NULL, NULL, NULL, NULL},
  141, 205, 71, 29, 128,
    {0,0,0,0,0,0,0,0, 0,179,179,0,0,179,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   179,179,179,179,179,179,179,179, 179,179,179,179,179,179,179,179, 179,179,179,179,179,179,179,179, 179,179,179,179,179,179,179,179,
   179,179,179,179,179,179,179,179, 179,179,179,179,179,179,179,179, 179,179,179,179,179,179,179,179, 179,179,179,179,179,179,179,179,
   179,179,179,179,179,179,179,179, 179,179,179,179,179,179,179,179, 179,179,179,179,179,179,179,179, 179,179,179,179,179,179,179,179,
   0,0,0,0,0,0,0,0, 0,179,179,0,0,179,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   0,179,179,179,179,179,179,179, 179,179,179,179,179,179,179,179, 179,179,179,179,179,179,179,179, 179,179,179,179,179,179,179,179,
   179,179,179,179,179,179,179,179, 179,179,179,179,179,179,179,179, 179,179,179,179,179,179,179,179, 179,179,179,179,179,179,179,179,
   179,179,179,179,179,179,179,179, 179,179,179,179,179,179,179,179, 179,179,179,190,179,180,181,179, 180,179,179,179,179,179,179,179,
   },
  {0,0,0,0,0,0,0,0, 0,179,179,0,0,179,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   180,179,180,179,179,179,179,179, 179,179,179,179,180,179,180,179, 179,179,179,179,179,179,179,179, 179,179,180,179,181,179,179,179,
   179,179,179,179,179,179,179,179, 179,179,179,179,179,179,179,179, 179,179,179,179,179,179,179,179, 179,179,179,179,179,179,179,179,
   179,179,179,179,179,179,179,179, 179,179,179,179,179,179,179,179, 179,179,179,179,179,179,179,179, 179,179,179,179,179,179,179,0,
   0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   179,180,180,180,180,180,180,180, 180,180,180,180,180,180,180,180, 180,180,180,180,180,180,180,180, 180,180,180,180,180,180,180,180,
   180,180,180,180,180,180,180,180, 180,180,180,180,180,180,180,180, 180,180,180,180,180,180,180,180, 180,180,180,180,180,180,180,180,
   180,180,180,180,180,180,180,180, 180,180,180,180,180,180,180,180, 180,180,180,180,180,180,180,180, 180,180,180,180,180,180,180,180,
   },
  {138,0,136,136,138,138,138,138, 128,128,136,136,136,136,136,136, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   2,0,2,2,2,2,2,2, 128,128,2,2,2,2,2,2, 2,0,2,2,2,2,2,2, 128,128,2,2,2,2,2,2,
   2,0,2,2,2,2,2,2, 128,128,2,2,2,2,2,2, 2,0,2,2,2,2,2,2, 128,128,2,2,2,2,2,2,
   2,0,2,2,2,2,2,2, 128,128,2,2,2,2,2,2, 2,0,2,2,2,2,2,2, 128,128,2,2,2,2,2,2,
   2,0,2,2,2,2,2,2, 128,128,2,2,2,2,2,2, 128,0,128,128,128,128,128,128, 128,128,128,128,128,128,128,128,
   138,0,136,136,136,136,136,136, 128,128,136,136,136,136,136,136, 138,0,136,136,138,138,138,136, 128,128,136,136,136,136,136,136,
   138,0,136,136,138,138,138,136, 128,128,136,136,136,136,136,136, 138,0,136,136,138,138,138,136, 128,128,136,136,136,136,136,136,
   138,0,136,136,138,138,138,136, 128,128,136,136,136,136,136,136, 136,0,140,136,136,136,136,136, 128,128,136,136,136,136,136,136,
   },
},

{ // X-TAM-WEBTAMIL (0.050M chars) [65]
  {NULL, NULL, NULL, NULL},
  142, 193, 66, 37, 129,
    {178,178,178,178,178,178,178,178, 178,186,186,178,178,186,178,178, 169,169,169,169,169,169,0,169, 169,169,169,169,169,169,169,169,
   178,178,178,178,178,178,178,178, 178,178,178,178,178,178,178,178, 178,178,178,178,178,178,178,179, 178,178,178,178,178,178,178,178,
   178,178,178,178,178,178,178,178, 178,178,178,178,178,178,178,178, 178,178,178,178,178,178,178,178, 178,178,178,178,178,178,178,178,
   178,178,178,178,178,178,178,178, 178,178,178,178,178,178,178,178, 178,178,178,178,178,178,178,178, 178,178,178,178,178,178,178,178,
   174,174,174,174,174,174,174,174, 174,186,186,174,174,186,174,174, 162,162,162,162,162,162,0,162, 162,162,162,162,162,162,162,162,
   0,174,174,174,174,174,174,174, 174,174,174,174,174,174,174,174, 174,174,174,174,174,174,174,181, 174,174,174,174,174,174,174,174,
   174,174,174,174,174,174,174,174, 174,174,174,174,174,174,174,174, 174,174,174,174,174,174,174,174, 174,174,174,174,174,174,174,174,
   174,174,174,174,174,174,174,174, 174,174,174,174,174,174,174,174, 174,174,174,174,174,174,174,174, 174,174,174,174,174,174,174,174,
   },
  {0,0,0,0,0,0,0,0, 0,178,177,0,0,177,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   158,177,177,177,177,177,178,177, 177,177,177,177,177,177,178,177, 177,177,177,177,177,177,177,177, 177,177,177,177,179,177,177,177,
   177,177,177,177,177,177,177,177, 177,177,177,177,177,177,177,177, 177,177,177,177,177,177,177,177, 177,177,177,177,177,177,177,177,
   177,177,177,177,177,177,177,177, 177,177,177,177,178,177,177,177, 177,177,177,177,177,177,177,177, 177,177,177,177,177,177,177,0,
   177,177,177,177,177,177,177,177, 177,177,177,177,177,177,177,177, 177,177,177,177,177,177,133,177, 177,177,177,177,177,177,177,177,
   177,177,177,177,177,177,177,177, 177,177,177,177,177,177,177,177, 177,177,177,177,177,177,177,178, 177,177,177,177,177,177,177,177,
   177,177,177,177,177,177,177,177, 177,177,177,177,177,177,177,177, 177,177,177,177,177,177,177,177, 177,177,177,177,177,177,177,177,
   177,177,177,177,177,177,177,177, 177,177,177,177,177,177,177,177, 177,177,177,177,177,177,177,177, 177,177,177,177,177,177,177,177,
   },
  {108,0,108,108,108,108,108,108, 110,110,110,110,110,110,110,110, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   2,0,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,0,2,2,2,2,2,2, 2,2,2,2,2,2,2,2,
   2,0,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,0,2,2,2,2,2,2, 2,2,2,2,2,2,2,2,
   2,0,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,0,2,2,2,2,2,2, 2,2,2,2,2,2,2,2,
   134,0,134,134,134,134,134,134, 132,132,132,132,132,132,132,132, 138,0,138,138,138,138,138,138, 134,134,134,134,134,134,134,134,
   138,0,138,138,138,138,138,138, 134,134,134,134,134,134,134,134, 138,0,138,138,138,138,138,138, 134,134,134,134,134,134,134,134,
   138,0,138,138,138,138,138,138, 134,134,134,134,134,134,134,134, 138,0,138,138,138,138,138,138, 134,134,134,134,134,134,134,134,
   138,0,138,138,138,138,138,138, 134,134,134,134,134,134,134,134, 138,0,138,138,138,138,138,138, 134,134,134,134,134,134,134,134,
   },
},

{ // UTF8CP1252 (178.156M chars) [66]
  {NULL, NULL, NULL, NULL},
  127, 200, 59, 31, 133,
    {181,189,183,184,176,167,163,162, 171,165,167,170,170,164,161,165, 163,161,165,161,164,166,165,166, 163,164,168,163,171,163,158,162,
   206,174,166,168,175,173,174,175, 177,166,173,171,187,176,178,175, 178,168,166,174,175,166,165,168, 186,173,179,177,184,174,171,171,
   119,121,206,216,183,183,147,141, 116,157,118,125,127,125,181,171, 210,198,125,120,113,136,145,187, 200,196,150,157,110,121,122,116,
   177,173,191,209,190,201,198,198, 188,185,166,178,183,167,123,183, 120,134,134,122,116,132,155,115, 122,120,125,97,146,127,124,122,
   186,143,146,128,162,163,140,136, 145,136,144,136,144,130,143,126, 134,159,199,183,181,182,183,161, 136,161,149,139,147,147,140,133,
   194,145,152,160,146,142,140,152, 139,166,140,162,150,144,159,142, 165,141,142,133,158,130,129,165, 138,130,156,172,141,139,132,152,
   146,177,150,157,188,159,153,165, 178,205,165,144,125,177,148,136, 125,164,126,181,156,148,180,118, 161,109,162,137,189,137,127,170,
   196,183,151,159,189,172,153,166, 185,210,166,152,154,180,147,138, 133,164,153,186,156,147,178,152, 160,162,164,140,188,144,126,109,
   },
  {148,50,56,78,0,0,0,0, 0,146,153,0,0,166,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   225,137,174,103,110,121,177,155, 149,159,126,115,183,160,179,142, 162,172,165,154,151,151,144,143, 141,143,157,138,205,108,138,142,
   124,187,176,189,179,184,170,182, 175,169,154,158,186,182,198,181, 175,156,199,194,193,169,173,155, 158,150,147,134,124,138,112,134,
   122,193,181,189,179,191,172,183, 177,178,152,159,187,182,199,184, 175,153,200,203,195,176,174,143, 147,148,143,122,135,97,118,45,
   189,193,197,200,179,177,173,171, 177,169,174,172,176,171,162,170, 167,165,159,165,169,171,167,170, 167,172,168,170,176,168,162,169,
   201,180,183,168,179,169,167,182, 176,187,173,166,167,178,173,174, 182,177,171,180,173,177,171,169, 186,175,179,179,186,179,179,171,
   129,129,169,152,124,152,130,133, 107,158,114,130,116,114,119,120, 120,118,126,120,107,124,147,116, 137,132,118,122,118,119,119,156,
   174,145,176,207,186,198,193,190, 186,184,161,173,176,161,128,175, 133,123,125,123,107,155,137,103, 119,110,131,110,136,122,127,123,
   },
  {0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
   2,0,2,2,2,2,2,2, 0,0,0,0,2,4,2,6, 2,0,2,2,2,2,2,2, 0,0,0,0,4,14,2,16,
   2,0,2,2,2,2,2,2, 0,0,0,0,2,2,2,2, 2,0,2,2,2,2,2,2, 0,0,0,0,2,2,2,2,
   2,0,2,2,2,2,2,2, 0,0,0,0,2,2,2,2, 32,0,2,4,2,2,2,2, 0,0,0,0,2,2,2,2,
   156,0,136,152,124,138,112,108, 126,134,130,124,120,126,140,96, 138,0,138,136,122,116,134,138, 124,130,118,122,140,122,146,112,
   150,0,140,156,114,112,112,112, 108,122,140,112,150,128,142,122, 144,0,142,152,118,120,114,114, 124,134,120,126,132,136,144,128,
   0,0,90,80,150,152,98,98, 134,120,142,130,128,128,70,112, 0,0,102,98,150,148,124,110, 134,128,128,144,104,128,66,114,
   0,0,138,118,64,72,142,142, 138,138,122,134,110,120,110,140, 0,0,126,122,70,82,148,144, 88,108,108,110,162,194,128,196,
   },
},

};		// End unigram_table

static const uint8 kMostLikelyEncoding[] = {
// 00xx
  37,39,39,39,39,39,39,39, 39,37,37,39,39,39,39,39, 39,39,39,39,39,39,39,39, 39,39,39,39,39,39,39,39,
  37,37,37,39,39,37,39,39, 39,39,37,39,37,37,39,37, 39,39,39,39,39,39,39,39, 39,39,37,39,37,39,37,39,
  37,39,39,39,39,37,39,39, 37,37,39,37,39,39,39,39, 37,39,37,37,37,37,37,37, 39,39,39,37,39,37,56,39,
  39,37,37,37,37,37,39,37, 37,37,37,37,37,37,37,37, 37,39,37,37,37,37,37,37, 39,37,37,37,39,37,39,56,
  56,39,56,56,39,39,39,39, 56,56,56,56,56,56,39,56, 56,56,39,39,57,56,56,39, 56,56,56,57,39,39,56,39,
  39,39,39,39,39,39,39,39, 39,39,39,39,39,39,39,57, 39,56,39,39,39,56,39,39, 39,39,39,39,39,57,39,39,
  39,39,57,39,39,39,39,39, 39,39,39,39,56,39,39,39, 39,39,39,39,39,57,39,39, 57,57,57,57,56,57,56,57,
  39,37,39,39,37,39,39,39, 39,37,39,39,39,37,39,39, 39,37,39,37,39,39,39,39, 39,39,39,39,37,39,39,57,
  // 01xx
  56,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,39,37,37,37,
  39,39,39,37,37,37,39,39, 39,39,37,37,39,39,39,37, 39,37,39,37,37,37,37,37, 37,37,39,39,39,37,37,39,
  39,37,37,37,37,39,37,39, 37,39,39,39,39,39,37,39, 39,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  37,37,39,39,39,39,37,37, 37,39,39,37,39,39,39,39, 39,37,39,39,39,39,39,39, 39,39,37,37,37,37,37,37,
  39,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 39,37,37,37,37,37,37,37, 39,37,37,37,37,37,37,37,
  39,37,37,39,39,39,39,39, 37,39,37,39,39,39,37,37, 37,39,37,39,39,37,39,39, 37,39,37,37,37,37,37,39,
  37,39,39,39,37,39,39,39, 39,39,39,39,39,37,37,37, 37,37,37,39,37,39,37,39, 37,56,56,56,37,56,39,56,
  39,39,39,39,39,39,39,39, 39,39,39,39,39,39,39,37, 39,39,39,39,39,37,39,37, 39,39,39,37,39,39,37,39,
  // 02xx
  37,37,37,39,37,37,37,39, 39,39,39,37,37,39,37,37, 37,8,37,37,37,37,37,37, 37,37,37,37,39,39,37,39,
  37,37,39,40,37,39,37,37, 39,39,39,39,39,39,39,39, 39,37,37,37,37,37,40,37, 40,40,39,39,39,39,37,37,
  6,39,39,40,39,37,39,39, 39,39,37,37,39,39,37,37, 37,37,37,37,37,37,37,37, 39,37,37,37,37,37,39,37,
  37,37,37,37,39,37,37,37, 37,37,39,37,37,37,37,39, 37,37,39,37,39,37,37,37, 37,37,37,37,37,37,37,37,
  37,37,37,39,37,37,37,39, 37,37,37,37,37,37,37,37, 39,37,37,37,37,37,37,37, 39,37,37,37,37,37,37,37,
  37,37,37,37,37,39,37,37, 37,37,37,37,37,37,37,37, 37,37,37,39,39,37,37,39, 39,37,37,37,37,37,37,37,
  37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  37,39,37,37,37,37,37,39, 37,39,39,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,39,37,37,37,37,
  // 03xx
  56,37,37,37,56,37,37,37, 37,37,39,56,37,39,39,37, 39,37,37,39,37,39,37,37, 37,39,56,39,37,37,39,39,
  39,39,39,37,37,37,39,39, 39,39,39,39,39,39,39,39, 37,39,39,39,39,39,39,39, 39,56,39,39,39,39,39,56,
  39,39,39,56,56,39,39,39, 39,39,39,39,39,39,37,37, 37,37,37,37,37,37,37,37, 37,37,39,37,37,37,39,37,
  37,37,37,39,37,37,37,39, 37,39,37,37,37,39,39,39, 39,37,37,37,39,37,37,37, 37,37,37,37,37,37,40,37,
  39,37,37,39,39,37,37,37, 37,37,37,37,39,37,37,39, 37,37,37,37,37,37,37,37, 39,37,37,37,37,37,37,37,
  37,37,39,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 39,37,37,37,37,37,37,39, 37,37,37,37,37,37,37,37,
  37,37,37,39,37,39,37,37, 37,39,39,37,39,37,37,37, 39,37,39,37,39,37,37,37, 39,39,39,39,39,39,37,37,
  // 04xx
  56,39,37,39,37,39,37,37, 37,39,39,37,39,39,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  39,37,39,37,37,37,39,39, 37,39,39,37,39,39,39,37, 39,39,37,39,39,39,39,37, 39,39,39,39,39,39,39,37,
  39,39,39,39,39,37,39,39, 39,39,39,39,39,37,39,39, 37,39,37,37,39,37,37,37, 37,39,37,37,39,39,37,39,
  37,39,37,39,39,37,37,37, 39,39,37,39,37,37,39,39, 37,37,37,37,37,37,37,37, 37,37,37,37,37,39,39,37,
  37,37,37,39,37,37,37,37, 37,37,37,39,37,37,37,37, 37,37,37,39,37,37,37,39, 37,37,37,37,39,37,37,39,
  39,37,39,39,37,37,39,39, 37,39,37,37,39,39,37,37, 37,39,37,37,39,39,37,39, 37,39,37,39,39,39,39,37,
  37,37,37,39,37,37,39,37, 39,37,39,37,37,39,39,37, 39,39,37,37,37,39,39,39, 37,37,37,37,37,37,37,37,
  39,37,39,39,39,39,39,39, 37,37,39,37,39,37,37,37, 37,37,37,37,39,39,39,37, 39,39,39,39,37,39,37,37,
  // 05xx
  56,37,37,37,37,37,37,37, 37,37,39,37,37,39,37,37, 37,37,37,39,37,37,37,37, 37,37,37,37,37,37,37,37,
  39,39,39,37,37,37,39,39, 39,39,39,39,39,39,39,39, 37,39,39,39,37,37,40,37, 37,39,39,39,39,39,39,39,
  37,39,40,39,37,37,37,39, 37,37,37,37,40,37,37,37, 37,37,37,39,37,37,39,37, 37,37,37,37,37,37,39,39,
  39,37,37,37,39,37,37,37, 37,37,37,37,37,37,39,37, 37,37,37,37,37,37,37,37, 37,37,39,37,37,39,37,37,
  39,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,39,39,37, 39,39,37,37,37,37,37,37,
  39,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,39,37,37,37, 37,37,37,37,37,37,37,37,
  37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,39,37,37, 37,37,37,37,39,37,37,37,
  // 06xx
  56,37,37,37,37,37,37,37, 37,37,39,37,39,39,39,39, 37,37,37,39,37,37,37,37, 39,39,37,37,37,37,37,37,
  39,39,39,37,37,37,39,37, 37,39,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,39,39,39,39,39,39,
  37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,39,37,37,37,37,37, 37,37,37,37,39,37,37,37,
  37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,39, 37,37,39,37,39,37,37,37, 37,37,37,37,37,37,37,39,
  37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 39,37,39,37,37,37,37,37, 37,37,37,39,37,37,37,37,
  39,37,37,39,37,37,37,39, 37,37,37,39,39,39,39,37, 37,37,37,37,37,37,37,39, 37,37,37,39,37,37,37,39,
  37,37,37,37,37,37,37,37, 37,37,39,37,37,37,37,39, 37,37,37,39,37,37,37,39, 37,37,37,37,37,37,37,37,
  39,39,39,39,39,37,39,37, 39,37,39,37,39,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  // 07xx
  56,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,39,37,37,37,
  39,37,39,40,37,37,37,37, 37,39,37,37,39,37,39,37, 37,39,37,40,37,40,37,37, 37,37,40,37,39,37,37,37,
  37,37,39,37,37,37,40,40, 37,37,37,40,37,37,39,37, 37,37,39,37,37,39,37,37, 37,39,39,37,39,39,37,37,
  37,37,37,39,37,37,37,37, 39,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  37,37,39,37,37,37,39,37, 37,39,37,37,37,37,37,37, 37,37,37,37,37,37,37,39, 37,37,37,37,37,37,37,37,
  37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  37,37,37,37,37,37,37,37, 37,39,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  // 08xx
  37,37,37,37,37,37,37,37, 3,39,39,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  37,37,39,40,40,37,37,37, 40,40,37,37,37,37,37,37, 37,39,40,40,40,40,40,40, 40,40,40,40,40,40,40,37,
  40,40,40,40,37,37,37,37, 40,40,37,37,40,37,37,37, 37,37,37,37,39,37,37,37, 37,37,37,37,39,37,39,37,
  37,39,37,37,37,37,37,39, 37,37,37,37,37,37,39,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,40,37,
  37,37,39,37,37,37,37,37, 37,37,39,37,39,37,37,37, 37,37,37,37,37,37,37,37, 39,37,37,37,37,39,37,37,
  37,37,37,37,37,37,37,37, 37,37,37,37,37,37,39,37, 37,39,37,37,37,37,37,37, 39,37,37,37,37,37,37,37,
  37,37,37,39,37,37,39,37, 39,37,37,37,37,39,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,39,37,37,37,37,39,
  // 09xx
  37,37,37,37,37,37,37,37, 37,0,0,37,37,0,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  37,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,37,
  0,10,0,0,0,0,0,0, 0,0,0,0,0,10,0,10, 10,0,0,0,0,0,0,0, 0,0,0,0,0,10,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  // 0Axx
  37,37,37,37,37,37,37,37, 37,0,0,37,37,0,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  37,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,37,
  0,10,0,0,0,0,0,0, 0,0,0,0,0,10,0,10, 10,0,0,0,0,0,0,0, 0,0,0,0,0,10,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  // 0Bxx
  56,37,37,37,37,37,37,37, 37,37,39,37,37,37,37,40, 37,37,37,37,37,37,37,37, 37,39,37,37,37,37,37,37,
  39,40,39,40,37,37,37,37, 37,39,37,37,39,39,39,40, 37,39,39,39,39,39,39,40, 39,39,39,37,39,37,37,37,
  37,37,40,40,37,37,40,37, 39,37,37,37,39,37,39,37, 39,37,37,37,37,37,37,39, 37,37,37,37,39,37,37,37,
  37,37,37,37,37,37,37,39, 37,37,37,37,37,39,37,37, 37,37,37,37,37,37,37,39, 37,37,39,37,37,37,40,37,
  37,37,37,37,37,37,39,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  // 0Cxx
  57,37,39,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  39,37,39,37,37,37,37,37, 37,39,37,37,39,37,37,37, 37,39,37,37,37,37,37,37, 37,37,39,37,39,37,37,39,
  37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,39,37,37,37, 37,37,37,37,37,37,37,37,
  37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,39, 37,37,37,37,37,37,37,37,
  39,37,37,37,39,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,39,37,
  37,37,37,37,37,37,37,37, 37,37,37,37,37,39,37,37, 37,37,37,39,37,37,37,37, 37,37,37,37,37,37,37,37,
  37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,39,
  // 0Dxx
  37,37,37,37,37,37,37,37, 37,0,0,37,37,0,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  37,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,37,
  0,10,0,0,0,0,0,0, 0,0,0,0,0,10,0,10, 10,0,0,0,0,0,0,0, 0,0,0,0,0,10,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  // 0Exx
  56,37,37,37,37,37,37,37, 37,37,37,37,37,39,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  39,57,56,57,56,37,56,37, 56,56,37,56,56,37,39,39, 57,57,57,57,57,57,57,57, 57,57,57,57,57,57,57,57,
  57,57,56,57,57,57,57,57, 57,57,56,57,57,57,56,56, 57,56,57,56,56,57,56,57, 56,56,56,56,57,56,56,56,
  56,56,56,56,56,56,39,56, 56,56,56,56,56,56,56,56, 56,56,56,56,56,37,56,56, 37,56,39,56,56,37,37,37,
  37,37,37,39,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  39,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,39,39,37,37,37,37,
  37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 6,37,37,37,37,37,37,37, 37,37,37,37,37,37,39,37,
  37,37,37,37,37,37,37,37, 37,39,37,39,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,39,
  // 0Fxx
  37,37,40,37,37,37,37,37, 40,42,42,37,37,42,40,37, 37,37,37,37,37,37,37,37, 37,40,37,57,37,37,37,37,
  42,42,42,42,42,42,42,42, 42,42,42,42,42,42,42,42, 42,42,42,42,42,42,42,42, 42,42,42,42,42,42,42,42,
  42,42,42,42,42,42,42,42, 42,42,42,42,42,42,42,42, 42,42,42,42,42,42,42,42, 42,42,42,42,42,42,42,42,
  42,42,42,42,42,42,42,42, 42,42,42,42,42,42,42,42, 42,42,42,42,42,42,42,42, 42,42,42,42,42,42,42,37,
  37,39,42,37,37,37,37,37, 37,37,37,37,37,37,39,37, 39,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  // 10xx
  56,39,37,37,39,37,39,37, 37,37,39,37,37,37,37,37, 37,37,37,39,37,37,37,37, 37,37,37,37,39,39,37,39,
  39,40,39,37,37,37,37,37, 40,39,40,37,39,39,39,39, 37,39,37,39,40,40,37,37, 40,40,39,39,39,37,37,39,
  37,40,40,40,37,40,37,37, 37,40,40,37,40,37,37,37, 37,37,37,37,37,39,37,37, 37,37,37,37,37,37,37,37,
  37,37,39,37,37,37,37,37, 37,37,37,37,37,37,39,37, 37,37,39,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  39,37,37,37,37,37,37,37, 37,37,37,37,39,37,39,37, 39,37,37,39,37,37,37,37, 39,39,37,37,37,37,37,37,
  37,37,37,37,37,37,37,37, 3,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,39,37,37,37,37,37,
  // 11xx
  56,39,37,37,39,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  37,40,39,40,37,37,37,37, 37,37,37,37,37,37,39,37, 37,40,40,40,40,40,40,37, 40,40,40,40,40,40,40,40,
  37,40,40,40,40,37,40,40, 40,40,40,40,37,40,37,39, 39,37,37,37,39,37,37,37, 37,37,37,37,39,37,37,37,
  37,37,39,39,37,37,37,37, 39,37,37,37,39,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,39,37,37,40,37,
  37,39,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  37,37,37,37,37,37,37,37, 37,3,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  37,37,37,37,37,37,37,37, 37,39,37,37,37,37,37,37, 37,37,37,37,37,37,39,37, 37,37,37,37,37,37,37,37,
  39,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,39,37,37,37,37,37,
  // 12xx
  37,37,37,37,39,39,37,37, 37,37,37,39,37,39,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  39,40,39,39,37,37,37,37, 37,39,37,39,37,37,39,39, 37,39,37,40,37,37,37,37, 39,40,37,39,39,37,37,37,
  40,37,37,37,37,39,37,37, 37,37,39,37,37,37,37,37, 39,37,39,37,37,37,37,39, 37,37,37,37,37,37,37,37,
  39,37,39,37,37,37,37,37, 37,39,37,37,37,39,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,40,37,
  37,37,39,37,37,37,37,39, 37,37,37,37,37,37,37,37, 39,37,37,37,37,37,37,37, 37,37,39,37,37,37,37,37,
  37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  39,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,39,
  // 13xx
  37,39,37,37,37,37,37,37, 37,37,39,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,39,37,37,
  39,37,37,40,37,37,37,37, 37,37,39,37,39,39,37,37, 37,39,40,37,40,37,37,37, 37,37,40,37,39,37,37,37,
  37,37,37,40,37,37,37,37, 37,37,37,37,40,37,39,37, 37,37,37,37,37,37,37,39, 37,37,37,37,37,37,37,37,
  37,37,37,37,39,37,39,37, 39,37,39,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,40,39,
  37,37,37,37,37,39,37,37, 37,37,39,37,39,37,37,37, 37,37,37,37,37,37,37,39, 37,39,39,37,39,37,37,39,
  37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,39,37,37,37,6,37, 37,37,37,37,37,37,37,37,
  37,37,37,37,37,37,37,37, 39,37,37,39,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  37,37,37,37,37,37,37,37, 37,37,37,37,37,39,37,37, 37,37,37,39,37,39,37,37, 37,37,37,37,37,37,37,39,
  // 14xx
  37,37,37,37,37,39,37,37, 37,37,37,37,37,37,37,37, 37,39,39,39,39,37,37,37, 37,37,37,37,37,37,37,37,
  39,37,37,40,37,39,37,37, 39,37,37,39,39,39,39,37, 37,37,37,39,40,40,40,40, 40,37,40,40,39,37,40,40,
  40,40,40,40,40,37,39,39, 40,40,40,40,37,40,39,37, 37,37,37,37,37,37,37,37, 37,37,37,37,39,37,37,37,
  37,37,37,37,37,37,37,37, 37,37,37,37,39,37,37,39, 37,37,37,37,37,37,37,37, 39,37,37,39,37,39,40,37,
  37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 39,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,39,37,37, 37,37,37,37,39,37,39,37,
  37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,39,
  // 15xx
  56,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  39,40,39,40,39,37,39,37, 37,37,37,37,39,37,37,37, 37,37,39,37,40,40,37,37, 37,40,37,37,39,37,40,40,
  37,40,40,37,37,37,37,37, 37,37,40,37,40,37,37,37, 37,37,37,37,37,37,39,37, 39,39,37,37,37,37,37,39,
  39,39,37,37,37,37,37,37, 37,37,37,37,37,37,37,39, 37,37,37,37,37,37,37,37, 39,37,37,37,37,37,40,37,
  39,37,37,37,37,37,37,39, 39,37,37,37,37,37,37,39, 37,39,39,37,37,37,37,37, 37,37,37,37,37,39,37,37,
  37,39,37,37,37,39,37,39, 37,37,37,39,39,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,39,37,37,37,
  37,37,39,37,37,37,37,37, 39,37,37,37,37,37,37,37, 39,37,37,37,37,37,39,37, 37,37,37,37,37,37,37,37,
  37,37,37,37,37,37,37,37, 37,37,39,37,37,37,37,37, 37,37,39,37,37,37,37,37, 37,37,37,37,37,37,37,39,
  // 16xx
  56,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  37,39,37,37,37,37,37,37, 37,40,37,37,37,37,37,40, 37,39,37,40,37,37,40,37, 40,40,37,40,37,37,40,37,
  37,40,37,37,40,37,40,37, 40,37,37,40,37,37,39,37, 37,37,37,39,37,37,37,39, 37,39,37,37,37,37,37,37,
  39,37,39,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,39,
  37,37,37,37,37,37,39,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  37,37,37,37,37,37,37,39, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  // 17xx
  56,37,37,37,37,37,37,37, 37,37,39,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  37,37,37,37,37,39,37,37, 40,37,37,37,37,37,40,37, 37,39,40,37,37,37,37,37, 37,37,37,37,37,40,40,37,
  37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,39, 37,37,39,39,39,37,39,37, 37,37,37,37,37,37,37,37,
  37,37,37,37,37,37,37,37, 39,37,37,37,39,37,39,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,40,37,
  39,37,39,37,37,39,37,39, 37,37,39,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  // 18xx
  37,37,37,37,39,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,39,37,40,37,37,37,37, 37,37,37,37,40,37,37,40,
  37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,39, 39,37,37,37,37,37,37,39, 37,37,39,37,39,37,37,37,
  37,37,39,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,39,
  37,37,37,37,37,37,37,39, 37,37,39,37,37,37,37,37, 37,37,37,37,39,37,37,37, 39,39,37,37,37,37,37,37,
  37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,39,37,37,37, 37,37,37,37,37,37,37,37,
  37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,39,
  // 19xx
  37,37,37,37,37,37,37,37, 37,39,37,37,37,39,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  37,37,39,37,40,37,37,37, 37,37,37,37,37,37,40,37, 37,37,37,37,37,37,37,37, 37,37,40,37,37,40,37,37,
  37,37,37,40,37,37,37,37, 37,37,37,37,37,37,37,37, 39,37,39,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  37,37,37,39,37,37,37,37, 37,37,39,37,37,37,37,37, 37,39,37,37,37,37,37,37, 37,39,37,37,37,39,37,37,
  37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 39,37,37,37,37,39,37,37, 37,37,37,37,37,37,37,37,
  37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  39,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  // 1Axx
  37,37,37,37,37,37,37,37, 37,37,4,37,37,37,37,40, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  37,40,39,40,37,37,37,37, 37,37,37,37,37,37,37,37, 37,39,37,37,40,40,37,37, 37,37,37,37,37,37,40,6,
  40,37,37,37,37,37,37,37, 37,37,37,37,37,37,39,39, 37,37,37,37,37,37,37,37, 37,39,37,37,37,37,37,37,
  37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,39,37,37,
  37,37,37,37,37,39,37,37, 37,37,37,37,37,37,37,37, 39,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,39,
  // 1Bxx
  56,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  37,37,37,37,57,37,37,37, 57,37,37,37,37,37,37,37, 37,39,37,37,40,40,37,37, 40,37,37,37,37,40,37,37,
  37,37,37,37,37,40,37,37, 37,37,37,37,37,37,56,37, 37,37,39,37,39,37,37,37, 37,37,39,37,37,39,37,37,
  37,39,37,37,37,37,37,39, 37,37,37,37,39,39,39,37, 37,37,39,39,37,37,37,37, 37,37,37,37,37,37,40,37,
  37,37,37,37,37,37,37,37, 37,37,37,39,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  37,37,37,37,37,37,37,37, 37,39,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,39,
  // 1Cxx
  37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  39,37,37,40,37,39,37,37, 37,37,40,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  37,37,37,40,37,37,37,37, 37,40,40,37,37,37,39,37, 37,37,37,37,37,37,37,37, 37,39,39,37,37,37,37,37,
  37,37,37,37,39,37,37,37, 37,39,37,37,37,37,37,37, 37,37,37,39,37,39,37,37, 37,37,37,37,37,37,37,37,
  37,37,37,37,37,37,37,39, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,39,39, 37,37,37,37,37,37,37,37,
  37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,39,37,37,37, 37,37,37,37,39,37,37,37,
  37,39,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  // 1Dxx
  37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  39,37,40,40,37,39,37,37, 37,37,37,37,37,37,37,37, 37,39,37,37,37,37,37,37, 40,37,37,37,37,40,37,37,
  37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,39, 37,37,39,37,37,37,37,37, 37,37,37,37,37,37,39,37,
  39,37,37,37,37,37,37,39, 37,37,37,37,37,37,39,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  37,37,37,37,39,37,37,37, 37,37,37,39,37,39,39,37, 39,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  37,37,37,37,37,37,37,37, 37,37,37,37,37,39,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  // 1Exx
  57,37,37,37,37,37,37,37, 37,39,39,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,39,37,37,
  39,40,39,40,37,37,39,39, 37,39,37,37,39,39,39,39, 37,40,42,40,40,40,40,37, 40,40,39,39,39,40,37,39,
  37,39,42,37,37,37,42,37, 42,39,40,40,40,39,39,39, 39,37,37,37,39,39,37,37, 37,37,42,37,37,37,37,37,
  37,39,37,39,39,39,37,37, 39,39,42,39,37,37,39,39, 39,37,37,37,39,39,37,39, 37,39,37,37,37,37,40,37,
  37,37,39,37,37,37,37,37, 37,37,37,37,37,39,37,37, 39,37,39,37,37,37,37,39, 37,37,37,37,37,37,39,37,
  37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,39,
  // 1Fxx
  37,37,37,37,39,37,39,37, 37,39,37,37,37,39,40,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  37,40,42,40,37,37,37,37, 42,37,37,42,39,37,37,42, 37,39,40,40,40,40,40,40, 40,40,40,40,40,40,42,40,
  40,40,42,40,40,40,40,42, 40,40,40,40,42,42,37,37, 39,37,37,37,37,37,37,39, 37,37,39,37,37,37,37,37,
  37,39,37,37,37,37,39,39, 37,37,37,37,37,37,37,37, 37,37,37,37,37,39,37,39, 37,37,37,37,37,37,40,37,
  37,37,39,37,37,37,37,37, 37,37,37,37,37,39,37,37, 39,37,37,37,37,37,39,37, 37,37,37,37,37,37,39,37,
  37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,39,37,37,37,37, 37,37,37,39,37,37,37,37,
  37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,39,
  // 20xx
  37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,37,
  37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  // 21xx
  37,37,37,37,37,37,37,37, 37,2,2,37,37,2,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,37,
  3,10,11,11,11,11,11,11, 11,11,0,11,11,10,0,10, 10,11,11,11,11,11,11,11, 0,11,0,11,11,10,0,0,
  11,1,11,11,11,11,11,11, 11,11,1,11,11,11,11,11, 11,11,11,11,11,11,11,11, 11,11,1,11,11,11,11,1,
  1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,11, 1,1,1,1,1,1,1,1,
  11,1,11,1,1,1,1,11, 11,11,11,11,1,1,11,11, 1,1,1,1,11,1,1,11, 1,11,1,11,11,1,1,1,
  // 22xx
  37,37,37,37,37,37,37,37, 37,2,2,37,37,2,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,37,
  3,10,11,11,11,11,11,11, 11,11,0,11,11,10,0,10, 10,11,11,11,11,11,11,11, 0,11,0,11,11,10,0,0,
  11,1,11,11,11,11,11,11, 11,11,1,11,11,11,11,11, 11,11,11,11,11,11,11,11, 11,11,1,11,11,11,11,1,
  1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,11, 1,1,1,1,1,1,1,1,
  11,1,11,1,1,1,1,11, 11,11,11,11,1,1,11,11, 1,1,1,1,11,1,1,11, 1,11,1,11,11,1,1,1,
  // 23xx
  37,37,37,37,37,37,37,37, 37,2,2,37,37,2,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,37,
  3,10,11,11,11,11,11,11, 11,11,0,11,11,10,0,10, 10,11,11,11,11,11,11,11, 0,11,0,11,11,10,0,0,
  11,1,11,11,11,11,11,11, 11,11,1,11,11,11,11,11, 11,11,11,11,11,11,11,11, 11,11,1,11,11,11,11,1,
  1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,11, 1,1,1,1,1,1,1,1,
  11,1,11,1,1,1,1,11, 11,11,11,11,1,1,11,11, 1,1,1,1,11,1,1,11, 1,11,1,11,11,1,1,1,
  // 24xx
  37,37,37,37,37,37,37,37, 37,2,2,37,37,2,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,37,
  3,10,11,11,11,11,11,11, 11,11,0,11,11,10,0,10, 10,11,11,11,11,11,11,11, 0,11,0,11,11,10,0,0,
  11,1,11,11,11,11,11,11, 11,11,1,11,11,11,11,11, 11,11,11,11,11,11,11,11, 11,11,1,11,11,11,11,1,
  1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,11, 1,1,1,1,1,1,1,1,
  11,1,11,1,1,1,1,11, 11,11,11,11,1,1,11,11, 1,1,1,1,11,1,1,11, 1,11,1,11,11,1,1,1,
  // 25xx
  37,37,37,37,37,37,37,37, 37,2,2,37,37,2,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,37,
  3,10,11,11,11,11,11,11, 11,11,0,11,11,10,0,10, 10,11,11,11,11,11,11,11, 0,11,0,11,11,10,0,0,
  11,1,11,11,11,11,11,11, 11,11,1,11,11,11,11,11, 11,11,11,11,11,11,11,11, 11,11,1,11,11,11,11,1,
  1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,11, 1,1,1,1,1,1,1,1,
  11,1,11,1,1,1,1,11, 11,11,11,11,1,1,11,11, 1,1,1,1,11,1,1,11, 1,11,1,11,11,1,1,1,
  // 26xx
  37,37,37,37,37,37,37,37, 37,2,2,37,37,2,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,37,
  3,10,11,11,11,11,11,11, 11,11,0,11,11,10,0,10, 10,11,11,11,11,11,11,11, 0,11,0,11,11,10,0,0,
  11,1,11,11,11,11,11,11, 11,11,1,11,11,11,11,11, 11,11,11,11,11,11,11,11, 11,11,1,11,11,11,11,1,
  1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,11, 1,1,1,1,1,1,1,1,
  11,1,11,1,1,1,1,11, 11,11,11,11,1,1,11,11, 1,1,1,1,11,1,1,11, 1,11,1,11,11,1,1,1,
  // 27xx
  37,37,37,37,37,37,37,37, 37,2,2,37,37,2,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,37,
  3,10,11,11,11,11,11,11, 11,11,0,11,11,10,0,10, 10,11,11,11,11,11,11,11, 0,11,0,11,11,10,0,0,
  11,1,11,11,11,11,11,11, 11,11,1,11,11,11,11,11, 11,11,11,11,11,11,11,11, 11,11,1,11,11,11,11,1,
  1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,11, 1,1,1,1,1,1,1,1,
  11,1,11,1,1,1,1,11, 11,11,11,11,1,1,11,11, 1,1,1,1,11,1,1,11, 1,11,1,11,11,1,1,1,
  // 28xx
  37,37,37,37,37,37,37,37, 37,2,2,37,37,2,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,37,
  3,10,11,11,11,11,11,11, 11,11,0,11,11,10,0,10, 10,11,11,11,11,11,11,11, 0,11,0,11,11,10,0,0,
  11,1,11,11,11,11,11,11, 11,11,1,11,11,11,11,11, 11,11,11,11,11,11,11,11, 11,11,1,11,11,11,11,1,
  1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,11, 1,1,1,1,1,1,1,1,
  11,1,11,1,1,1,1,11, 11,11,11,11,1,1,11,11, 1,1,1,1,11,1,1,11, 1,11,1,11,11,1,1,1,
  // 29xx
  37,37,37,37,37,37,37,37, 37,2,2,37,37,2,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,37,
  3,10,11,11,11,11,11,11, 11,11,0,11,11,10,0,10, 10,11,11,11,11,11,11,11, 0,11,0,11,11,10,0,0,
  11,1,11,11,11,11,11,11, 11,11,1,11,11,11,11,11, 11,11,11,11,11,11,11,11, 11,11,1,11,11,11,11,1,
  1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,11, 1,1,1,1,1,1,1,1,
  11,1,11,1,1,1,1,11, 11,11,11,11,1,1,11,11, 1,1,1,1,11,1,1,11, 1,11,1,11,11,1,1,1,
  // 2Axx
  37,37,37,37,37,37,37,37, 37,2,2,37,37,2,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,37,
  3,10,11,11,11,11,11,11, 11,11,0,11,11,10,0,10, 10,11,11,11,11,11,11,11, 0,11,0,11,11,10,0,0,
  11,1,11,11,11,11,11,11, 11,11,1,11,11,11,11,11, 11,11,11,11,11,11,11,11, 11,11,1,11,11,11,11,1,
  1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,11, 1,1,1,1,1,1,1,1,
  11,1,11,1,1,1,1,11, 11,11,11,11,1,1,11,11, 1,1,1,1,11,1,1,11, 1,11,1,11,11,1,1,1,
  // 2Bxx
  2,39,1,37,37,37,37,5, 3,2,2,37,3,2,42,42, 37,40,37,37,37,40,37,37, 40,37,40,57,37,37,37,21,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,37,
  3,3,3,3,11,11,11,11, 3,3,3,3,3,3,3,3, 3,3,3,3,3,3,3,3, 3,11,12,11,12,36,12,4,
  11,3,3,3,3,3,3,3, 3,3,3,3,3,3,3,3, 3,3,3,3,3,3,3,3, 3,3,3,3,3,3,3,3,
  2,2,2,2,2,2,2,11, 2,1,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,11,2,1,11,13,6,
  14,10,2,2,2,2,2,2, 2,2,2,2,2,10,17,2, 10,10,10,10,10,10,1,17, 17,17,17,2,11,10,13,37,
  // 2Cxx
  37,37,37,37,37,37,37,37, 37,2,2,37,37,2,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,37,
  3,10,11,11,11,11,11,11, 11,11,0,11,11,10,0,10, 10,11,11,11,11,11,11,11, 0,11,0,11,11,10,0,0,
  11,1,11,11,11,11,11,11, 11,11,1,11,11,11,11,11, 11,11,11,11,11,11,11,11, 11,11,1,11,11,11,11,1,
  1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,11, 1,1,1,1,1,1,1,1,
  11,1,11,1,1,1,1,11, 11,11,11,11,1,1,11,11, 1,1,1,1,11,1,1,11, 1,11,1,11,11,1,1,1,
  // 2Dxx
  37,37,37,37,37,37,37,37, 37,2,2,37,37,2,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,37,
  3,10,11,11,11,11,11,11, 11,11,0,11,11,10,0,10, 10,11,11,11,11,11,11,11, 0,11,0,11,11,10,0,0,
  11,1,11,11,11,11,11,11, 11,11,1,11,11,11,11,11, 11,11,11,11,11,11,11,11, 11,11,1,11,11,11,11,1,
  1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,11, 1,1,1,1,1,1,1,1,
  11,1,11,1,1,1,1,11, 11,11,11,11,1,1,11,11, 1,1,1,1,11,1,1,11, 1,11,1,11,11,1,1,1,
  // 2Exx
  37,37,37,37,37,37,37,37, 37,2,2,37,37,2,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,37,
  3,10,11,11,11,11,11,11, 11,11,0,11,11,10,0,10, 10,11,11,11,11,11,11,11, 0,11,0,11,11,10,0,0,
  11,1,11,11,11,11,11,11, 11,11,1,11,11,11,11,11, 11,11,11,11,11,11,11,11, 11,11,1,11,11,11,11,1,
  1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,11, 1,1,1,1,1,1,1,1,
  11,1,11,1,1,1,1,11, 11,11,11,11,1,1,11,11, 1,1,1,1,11,1,1,11, 1,11,1,11,11,1,1,1,
  // 2Fxx
  37,37,37,37,37,37,37,37, 37,2,2,37,37,2,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,37,
  3,10,11,11,11,11,11,11, 11,11,0,11,11,10,0,10, 10,11,11,11,11,11,11,11, 0,11,0,11,11,10,0,0,
  11,1,11,11,11,11,11,11, 11,11,1,11,11,11,11,11, 11,11,11,11,11,11,11,11, 11,11,1,11,11,11,11,1,
  1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,11, 1,1,1,1,1,1,1,1,
  11,1,11,1,1,1,1,11, 11,11,11,11,1,1,11,11, 1,1,1,1,11,1,1,11, 1,11,1,11,11,1,1,1,
  // 30xx
  37,37,37,37,37,37,37,37, 37,2,2,37,37,2,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,37,
  3,10,11,11,11,11,11,11, 11,11,0,11,11,10,0,10, 10,11,11,11,11,11,11,11, 0,11,0,11,11,10,0,0,
  11,1,11,11,11,11,11,11, 11,11,1,11,11,11,11,11, 11,11,11,11,11,11,11,11, 11,11,1,11,11,11,11,1,
  1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,11, 1,1,1,1,1,1,1,1,
  11,1,11,1,1,1,1,11, 11,11,11,11,1,1,11,11, 1,1,1,1,11,1,1,11, 1,11,1,11,11,1,1,1,
  // 31xx
  37,37,37,37,37,37,37,37, 37,2,2,37,37,2,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,37,
  3,10,11,11,11,11,11,11, 11,11,0,11,11,10,0,10, 10,11,11,11,11,11,11,11, 0,11,0,11,11,10,0,0,
  11,1,11,11,11,11,11,11, 11,11,1,11,11,11,11,11, 11,11,11,11,11,11,11,11, 11,11,1,11,11,11,11,1,
  1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,11, 1,1,1,1,1,1,1,1,
  11,1,11,1,1,1,1,11, 11,11,11,11,1,1,11,11, 1,1,1,1,11,1,1,11, 1,11,1,11,11,1,1,1,
  // 32xx
  37,37,37,37,37,37,37,37, 37,2,2,37,37,2,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,37,
  3,10,11,11,11,11,11,11, 11,11,0,11,11,10,0,10, 10,11,11,11,11,11,11,11, 0,11,0,11,11,10,0,0,
  11,1,11,11,11,11,11,11, 11,11,1,11,11,11,11,11, 11,11,11,11,11,11,11,11, 11,11,1,11,11,11,11,1,
  1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,11, 1,1,1,1,1,1,1,1,
  11,1,11,1,1,1,1,11, 11,11,11,11,1,1,11,11, 1,1,1,1,11,1,1,11, 1,11,1,11,11,1,1,1,
  // 33xx
  37,37,37,37,37,37,37,37, 37,2,2,37,37,2,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,37,
  3,10,11,11,11,11,11,11, 11,11,0,11,11,10,0,10, 10,11,11,11,11,11,11,11, 0,11,0,11,11,10,0,0,
  11,1,11,11,11,11,11,11, 11,11,1,11,11,11,11,11, 11,11,11,11,11,11,11,11, 11,11,1,11,11,11,11,1,
  1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,11, 1,1,1,1,1,1,1,1,
  11,1,11,1,1,1,1,11, 11,11,11,11,1,1,11,11, 1,1,1,1,11,1,1,11, 1,11,1,11,11,1,1,1,
  // 34xx
  37,37,37,37,37,37,37,37, 37,2,2,37,37,2,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,37,
  3,10,11,11,11,11,11,11, 11,11,0,11,11,10,0,10, 10,11,11,11,11,11,11,11, 0,11,0,11,11,10,0,0,
  11,1,11,11,11,11,11,11, 11,11,1,11,11,11,11,11, 11,11,11,11,11,11,11,11, 11,11,1,11,11,11,11,1,
  1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,11, 1,1,1,1,1,1,1,1,
  11,1,11,1,1,1,1,11, 11,11,11,11,1,1,11,11, 1,1,1,1,11,1,1,11, 1,11,1,11,11,1,1,1,
  // 35xx
  37,37,37,37,37,37,37,37, 37,2,2,37,37,2,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,37,
  3,10,11,11,11,11,11,11, 11,11,0,11,11,10,0,10, 10,11,11,11,11,11,11,11, 0,11,0,11,11,10,0,0,
  11,1,11,11,11,11,11,11, 11,11,1,11,11,11,11,11, 11,11,11,11,11,11,11,11, 11,11,1,11,11,11,11,1,
  1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,11, 1,1,1,1,1,1,1,1,
  11,1,11,1,1,1,1,11, 11,11,11,11,1,1,11,11, 1,1,1,1,11,1,1,11, 1,11,1,11,11,1,1,1,
  // 36xx
  37,37,37,37,37,37,37,37, 37,2,2,37,37,2,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,37,
  3,10,11,11,11,11,11,11, 11,11,0,11,11,10,0,10, 10,11,11,11,11,11,11,11, 0,11,0,11,11,10,0,0,
  11,1,11,11,11,11,11,11, 11,11,1,11,11,11,11,11, 11,11,11,11,11,11,11,11, 11,11,1,11,11,11,11,1,
  1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,11, 1,1,1,1,1,1,1,1,
  11,1,11,1,1,1,1,11, 11,11,11,11,1,1,11,11, 1,1,1,1,11,1,1,11, 1,11,1,11,11,1,1,1,
  // 37xx
  37,37,37,37,37,37,37,37, 37,2,2,37,37,2,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,37,
  3,10,11,11,11,11,11,11, 11,11,0,11,11,10,0,10, 10,11,11,11,11,11,11,11, 0,11,0,11,11,10,0,0,
  11,1,11,11,11,11,11,11, 11,11,1,11,11,11,11,11, 11,11,11,11,11,11,11,11, 11,11,1,11,11,11,11,1,
  1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,11, 1,1,1,1,1,1,1,1,
  11,1,11,1,1,1,1,11, 11,11,11,11,1,1,11,11, 1,1,1,1,11,1,1,11, 1,11,1,11,11,1,1,1,
  // 38xx
  37,37,37,37,37,37,37,37, 37,2,2,37,37,2,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,37,
  3,10,11,11,11,11,11,11, 11,11,0,11,11,10,0,10, 10,11,11,11,11,11,11,11, 0,11,0,11,11,10,0,0,
  11,1,11,11,11,11,11,11, 11,11,1,11,11,11,11,11, 11,11,11,11,11,11,11,11, 11,11,1,11,11,11,11,1,
  1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,11, 1,1,1,1,1,1,1,1,
  11,1,11,1,1,1,1,11, 11,11,11,11,1,1,11,11, 1,1,1,1,11,1,1,11, 1,11,1,11,11,1,1,1,
  // 39xx
  37,37,37,37,37,37,37,37, 37,2,2,37,37,2,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,37,
  3,10,11,11,11,11,11,11, 11,11,0,11,11,10,0,10, 10,11,11,11,11,11,11,11, 0,11,0,11,11,10,0,0,
  11,1,11,11,11,11,11,11, 11,11,1,11,11,11,11,11, 11,11,11,11,11,11,11,11, 11,11,1,11,11,11,11,1,
  1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,11, 1,1,1,1,1,1,1,1,
  11,1,11,1,1,1,1,11, 11,11,11,11,1,1,11,11, 1,1,1,1,11,1,1,11, 1,11,1,11,11,1,1,1,
  // 3Axx
  37,37,37,37,37,37,37,37, 37,2,2,37,37,2,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,37,
  3,10,11,11,11,11,11,11, 11,11,0,11,11,10,0,10, 10,11,11,11,11,11,11,11, 0,11,0,11,11,10,0,0,
  11,1,11,11,11,11,11,11, 11,11,1,11,11,11,11,11, 11,11,11,11,11,11,11,11, 11,11,1,11,11,11,11,1,
  1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,11, 1,1,1,1,1,1,1,1,
  11,1,11,1,1,1,1,11, 11,11,11,11,1,1,11,11, 1,1,1,1,11,1,1,11, 1,11,1,11,11,1,1,1,
  // 3Bxx
  37,37,37,37,37,37,37,37, 37,2,2,37,37,2,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,37,
  3,10,11,11,11,11,11,11, 11,11,0,11,11,10,0,10, 10,11,11,11,11,11,11,11, 0,11,0,11,11,10,0,0,
  11,1,11,11,11,11,11,11, 11,11,1,11,11,11,11,11, 11,11,11,11,11,11,11,11, 11,11,1,11,11,11,11,1,
  1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,11, 1,1,1,1,1,1,1,1,
  11,1,11,1,1,1,1,11, 11,11,11,11,1,1,11,11, 1,1,1,1,11,1,1,11, 1,11,1,11,11,1,1,1,
  // 3Cxx
  37,37,37,37,37,37,37,37, 37,2,2,37,37,2,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,37,
  3,10,11,11,11,11,11,11, 11,11,0,11,11,10,0,10, 10,11,11,11,11,11,11,11, 0,11,0,11,11,10,0,0,
  11,1,11,11,11,11,11,11, 11,11,1,11,11,11,11,11, 11,11,11,11,11,11,11,11, 11,11,1,11,11,11,11,1,
  1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,11, 1,1,1,1,1,1,1,1,
  11,1,11,1,1,1,1,11, 11,11,11,11,1,1,11,11, 1,1,1,1,11,1,1,11, 1,11,1,11,11,1,1,1,
  // 3Dxx
  37,37,37,37,37,37,37,37, 37,2,2,37,37,2,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,37,
  3,10,11,11,11,11,11,11, 11,11,0,11,11,10,0,10, 10,11,11,11,11,11,11,11, 0,11,0,11,11,10,0,0,
  11,1,11,11,11,11,11,11, 11,11,1,11,11,11,11,11, 11,11,11,11,11,11,11,11, 11,11,1,11,11,11,11,1,
  1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,11, 1,1,1,1,1,1,1,1,
  11,1,11,1,1,1,1,11, 11,11,11,11,1,1,11,11, 1,1,1,1,11,1,1,11, 1,11,1,11,11,1,1,1,
  // 3Exx
  37,37,37,37,37,37,37,37, 37,2,2,37,37,2,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,37,
  3,10,11,11,11,11,11,11, 11,11,0,11,11,10,0,10, 10,11,11,11,11,11,11,11, 0,11,0,11,11,10,0,0,
  11,1,11,11,11,11,11,11, 11,11,1,11,11,11,11,11, 11,11,11,11,11,11,11,11, 11,11,1,11,11,11,11,1,
  1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,11, 1,1,1,1,1,1,1,1,
  11,1,11,1,1,1,1,11, 11,11,11,11,1,1,11,11, 1,1,1,1,11,1,1,11, 1,11,1,11,11,1,1,1,
  // 3Fxx
  37,37,37,37,37,37,37,37, 37,2,2,37,37,2,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,37,
  3,10,11,11,11,11,11,11, 11,11,0,11,11,10,0,10, 10,11,11,11,11,11,11,11, 0,11,0,11,11,10,0,0,
  11,1,11,11,11,11,11,11, 11,11,1,11,11,11,11,11, 11,11,11,11,11,11,11,11, 11,11,1,11,11,11,11,1,
  1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,11, 1,1,1,1,1,1,1,1,
  11,1,11,1,1,1,1,11, 11,11,11,11,1,1,11,11, 1,1,1,1,11,1,1,11, 1,11,1,11,11,1,1,1,
  // 40xx
  37,37,37,37,37,37,37,37, 37,2,2,37,37,2,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,37,
  3,10,11,11,11,11,11,11, 11,11,0,11,11,10,0,10, 10,11,11,11,11,11,11,11, 0,11,0,11,11,10,0,0,
  11,1,11,11,11,11,11,11, 11,11,1,11,11,11,11,11, 11,11,11,11,11,11,11,11, 11,11,1,11,11,11,11,1,
  1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,11, 1,1,1,1,1,1,1,1,
  11,1,11,1,1,1,1,11, 11,11,11,11,1,1,11,11, 1,1,1,1,11,1,1,11, 1,11,1,11,11,1,1,1,
  // 41xx
  37,37,37,37,37,37,37,37, 37,2,2,37,37,2,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,37,
  3,10,11,11,11,11,11,11, 11,11,0,11,11,10,0,10, 10,11,11,11,11,11,11,11, 0,11,0,11,11,10,0,0,
  11,1,11,11,11,11,11,11, 11,11,1,11,11,11,11,11, 11,11,11,11,11,11,11,11, 11,11,1,11,11,11,11,1,
  1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,11, 1,1,1,1,1,1,1,1,
  11,1,11,1,1,1,1,11, 11,11,11,11,1,1,11,11, 1,1,1,1,11,1,1,11, 1,11,1,11,11,1,1,1,
  // 42xx
  37,37,37,37,37,37,37,37, 37,2,2,37,37,2,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,37,
  3,10,11,11,11,11,11,11, 11,11,0,11,11,10,0,10, 10,11,11,11,11,11,11,11, 0,11,0,11,11,10,0,0,
  11,1,11,11,11,11,11,11, 11,11,1,11,11,11,11,11, 11,11,11,11,11,11,11,11, 11,11,1,11,11,11,11,1,
  1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,11, 1,1,1,1,1,1,1,1,
  11,1,11,1,1,1,1,11, 11,11,11,11,1,1,11,11, 1,1,1,1,11,1,1,11, 1,11,1,11,11,1,1,1,
  // 43xx
  37,37,37,37,37,37,37,37, 37,2,2,37,37,2,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,37,
  3,10,11,11,11,11,11,11, 11,11,0,11,11,10,0,10, 10,11,11,11,11,11,11,11, 0,11,0,11,11,10,0,0,
  11,1,11,11,11,11,11,11, 11,11,1,11,11,11,11,11, 11,11,11,11,11,11,11,11, 11,11,1,11,11,11,11,1,
  1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,11, 1,1,1,1,1,1,1,1,
  11,1,11,1,1,1,1,11, 11,11,11,11,1,1,11,11, 1,1,1,1,11,1,1,11, 1,11,1,11,11,1,1,1,
  // 44xx
  37,37,37,37,37,37,37,37, 37,2,2,37,37,2,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,37,
  3,10,11,11,11,11,11,11, 11,11,0,11,11,10,0,10, 10,11,11,11,11,11,11,11, 0,11,0,11,11,10,0,0,
  11,1,11,11,11,11,11,11, 11,11,1,11,11,11,11,11, 11,11,11,11,11,11,11,11, 11,11,1,11,11,11,11,1,
  1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,11, 1,1,1,1,1,1,1,1,
  11,1,11,1,1,1,1,11, 11,11,11,11,1,1,11,11, 1,1,1,1,11,1,1,11, 1,11,1,11,11,1,1,1,
  // 45xx
  37,37,37,37,37,37,37,37, 37,2,2,37,37,2,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,37,
  3,10,11,11,11,11,11,11, 11,11,0,11,11,10,0,10, 10,11,11,11,11,11,11,11, 0,11,0,11,11,10,0,0,
  11,1,11,11,11,11,11,11, 11,11,1,11,11,11,11,11, 11,11,11,11,11,11,11,11, 11,11,1,11,11,11,11,1,
  1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,11, 1,1,1,1,1,1,1,1,
  11,1,11,1,1,1,1,11, 11,11,11,11,1,1,11,11, 1,1,1,1,11,1,1,11, 1,11,1,11,11,1,1,1,
  // 46xx
  37,37,37,37,37,37,37,37, 37,2,2,37,37,2,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,37,
  3,10,11,11,11,11,11,11, 11,11,0,11,11,10,0,10, 10,11,11,11,11,11,11,11, 0,11,0,11,11,10,0,0,
  11,1,11,11,11,11,11,11, 11,11,1,11,11,11,11,11, 11,11,11,11,11,11,11,11, 11,11,1,11,11,11,11,1,
  1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,11, 1,1,1,1,1,1,1,1,
  11,1,11,1,1,1,1,11, 11,11,11,11,1,1,11,11, 1,1,1,1,11,1,1,11, 1,11,1,11,11,1,1,1,
  // 47xx
  37,37,37,37,37,37,37,37, 37,2,2,37,37,2,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,37,
  3,10,11,11,11,11,11,11, 11,11,0,11,11,10,0,10, 10,11,11,11,11,11,11,11, 0,11,0,11,11,10,0,0,
  11,1,11,11,11,11,11,11, 11,11,1,11,11,11,11,11, 11,11,11,11,11,11,11,11, 11,11,1,11,11,11,11,1,
  1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,11, 1,1,1,1,1,1,1,1,
  11,1,11,1,1,1,1,11, 11,11,11,11,1,1,11,11, 1,1,1,1,11,1,1,11, 1,11,1,11,11,1,1,1,
  // 48xx
  37,37,37,37,37,37,37,37, 37,2,2,37,37,2,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,37,
  3,10,11,11,11,11,11,11, 11,11,0,11,11,10,0,10, 10,11,11,11,11,11,11,11, 0,11,0,11,11,10,0,0,
  11,1,11,11,11,11,11,11, 11,11,1,11,11,11,11,11, 11,11,11,11,11,11,11,11, 11,11,1,11,11,11,11,1,
  1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,11, 1,1,1,1,1,1,1,1,
  11,1,11,1,1,1,1,11, 11,11,11,11,1,1,11,11, 1,1,1,1,11,1,1,11, 1,11,1,11,11,1,1,1,
  // 49xx
  37,37,37,37,37,37,37,37, 37,2,2,37,37,2,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,37,
  3,10,11,11,11,11,11,11, 11,11,0,11,11,10,0,10, 10,11,11,11,11,11,11,11, 0,11,0,11,11,10,0,0,
  11,1,11,11,11,11,11,11, 11,11,1,11,11,11,11,11, 11,11,11,11,11,11,11,11, 11,11,1,11,11,11,11,1,
  1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,11, 1,1,1,1,1,1,1,1,
  11,1,11,1,1,1,1,11, 11,11,11,11,1,1,11,11, 1,1,1,1,11,1,1,11, 1,11,1,11,11,1,1,1,
  // 4Axx
  37,37,37,37,37,37,37,37, 37,2,2,37,37,2,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,37,
  3,10,11,11,11,11,11,11, 11,11,0,11,11,10,0,10, 10,11,11,11,11,11,11,11, 0,11,0,11,11,10,0,0,
  11,1,11,11,11,11,11,11, 11,11,1,11,11,11,11,11, 11,11,11,11,11,11,11,11, 11,11,1,11,11,11,11,1,
  1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,11, 1,1,1,1,1,1,1,1,
  11,1,11,1,1,1,1,11, 11,11,11,11,1,1,11,11, 1,1,1,1,11,1,1,11, 1,11,1,11,11,1,1,1,
  // 4Bxx
  37,37,37,37,37,37,37,37, 37,2,2,37,37,2,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,37,
  3,10,11,11,11,11,11,11, 11,11,0,11,11,10,0,10, 10,11,11,11,11,11,11,11, 0,11,0,11,11,10,0,0,
  11,1,11,11,11,11,11,11, 11,11,1,11,11,11,11,11, 11,11,11,11,11,11,11,11, 11,11,1,11,11,11,11,1,
  1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,11, 1,1,1,1,1,1,1,1,
  11,1,11,1,1,1,1,11, 11,11,11,11,1,1,11,11, 1,1,1,1,11,1,1,11, 1,11,1,11,11,1,1,1,
  // 4Cxx
  37,37,37,37,37,37,37,37, 37,2,2,37,37,2,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,37,
  3,10,11,11,11,11,11,11, 11,11,0,11,11,10,0,10, 10,11,11,11,11,11,11,11, 0,11,0,11,11,10,0,0,
  11,1,11,11,11,11,11,11, 11,11,1,11,11,11,11,11, 11,11,11,11,11,11,11,11, 11,11,1,11,11,11,11,1,
  1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,11, 1,1,1,1,1,1,1,1,
  11,1,11,1,1,1,1,11, 11,11,11,11,1,1,11,11, 1,1,1,1,11,1,1,11, 1,11,1,11,11,1,1,1,
  // 4Dxx
  37,37,37,37,37,37,37,37, 37,2,2,37,37,2,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,37,
  3,10,11,11,11,11,11,11, 11,11,0,11,11,10,0,10, 10,11,11,11,11,11,11,11, 0,11,0,11,11,10,0,0,
  11,1,11,11,11,11,11,11, 11,11,1,11,11,11,11,11, 11,11,11,11,11,11,11,11, 11,11,1,11,11,11,11,1,
  1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,11, 1,1,1,1,1,1,1,1,
  11,1,11,1,1,1,1,11, 11,11,11,11,1,1,11,11, 1,1,1,1,11,1,1,11, 1,11,1,11,11,1,1,1,
  // 4Exx
  37,37,37,37,37,37,37,37, 37,2,2,37,37,2,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,37,
  3,10,11,11,11,11,11,11, 11,11,0,11,11,10,0,10, 10,11,11,11,11,11,11,11, 0,11,0,11,11,10,0,0,
  11,1,11,11,11,11,11,11, 11,11,1,11,11,11,11,11, 11,11,11,11,11,11,11,11, 11,11,1,11,11,11,11,1,
  1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,11, 1,1,1,1,1,1,1,1,
  11,1,11,1,1,1,1,11, 11,11,11,11,1,1,11,11, 1,1,1,1,11,1,1,11, 1,11,1,11,11,1,1,1,
  // 4Fxx
  37,37,37,37,37,37,37,37, 37,2,2,37,37,2,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,37,
  3,10,11,11,11,11,11,11, 11,11,0,11,11,10,0,10, 10,11,11,11,11,11,11,11, 0,11,0,11,11,10,0,0,
  11,1,11,11,11,11,11,11, 11,11,1,11,11,11,11,11, 11,11,11,11,11,11,11,11, 11,11,1,11,11,11,11,1,
  1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,11, 1,1,1,1,1,1,1,1,
  11,1,11,1,1,1,1,11, 11,11,11,11,1,1,11,11, 1,1,1,1,11,1,1,11, 1,11,1,11,11,1,1,1,
  // 50xx
  37,37,37,37,37,37,37,37, 37,2,2,37,37,2,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,37,
  3,10,11,11,11,11,11,11, 11,11,0,11,11,10,0,10, 10,11,11,11,11,11,11,11, 0,11,0,11,11,10,0,0,
  11,1,11,11,11,11,11,11, 11,11,1,11,11,11,11,11, 11,11,11,11,11,11,11,11, 11,11,1,11,11,11,11,1,
  1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,11, 1,1,1,1,1,1,1,1,
  11,1,11,1,1,1,1,11, 11,11,11,11,1,1,11,11, 1,1,1,1,11,1,1,11, 1,11,1,11,11,1,1,1,
  // 51xx
  37,37,37,37,37,37,37,37, 37,2,2,37,37,2,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,37,
  3,10,11,11,11,11,11,11, 11,11,0,11,11,10,0,10, 10,11,11,11,11,11,11,11, 0,11,0,11,11,10,0,0,
  11,1,11,11,11,11,11,11, 11,11,1,11,11,11,11,11, 11,11,11,11,11,11,11,11, 11,11,1,11,11,11,11,1,
  1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,11, 1,1,1,1,1,1,1,1,
  11,1,11,1,1,1,1,11, 11,11,11,11,1,1,11,11, 1,1,1,1,11,1,1,11, 1,11,1,11,11,1,1,1,
  // 52xx
  37,37,37,37,37,37,37,37, 37,2,2,37,37,2,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,37,
  3,10,11,11,11,11,11,11, 11,11,0,11,11,10,0,10, 10,11,11,11,11,11,11,11, 0,11,0,11,11,10,0,0,
  11,1,11,11,11,11,11,11, 11,11,1,11,11,11,11,11, 11,11,11,11,11,11,11,11, 11,11,1,11,11,11,11,1,
  1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,11, 1,1,1,1,1,1,1,1,
  11,1,11,1,1,1,1,11, 11,11,11,11,1,1,11,11, 1,1,1,1,11,1,1,11, 1,11,1,11,11,1,1,1,
  // 53xx
  37,37,37,37,37,37,37,37, 37,2,2,37,37,2,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,37,
  3,10,11,11,11,11,11,11, 11,11,0,11,11,10,0,10, 10,11,11,11,11,11,11,11, 0,11,0,11,11,10,0,0,
  11,1,11,11,11,11,11,11, 11,11,1,11,11,11,11,11, 11,11,11,11,11,11,11,11, 11,11,1,11,11,11,11,1,
  1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,11, 1,1,1,1,1,1,1,1,
  11,1,11,1,1,1,1,11, 11,11,11,11,1,1,11,11, 1,1,1,1,11,1,1,11, 1,11,1,11,11,1,1,1,
  // 54xx
  37,37,37,37,37,37,37,37, 37,2,2,37,37,2,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,37,
  3,10,11,11,11,11,11,11, 11,11,0,11,11,10,0,10, 10,11,11,11,11,11,11,11, 0,11,0,11,11,10,0,0,
  11,1,11,11,11,11,11,11, 11,11,1,11,11,11,11,11, 11,11,11,11,11,11,11,11, 11,11,1,11,11,11,11,1,
  1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,11, 1,1,1,1,1,1,1,1,
  11,1,11,1,1,1,1,11, 11,11,11,11,1,1,11,11, 1,1,1,1,11,1,1,11, 1,11,1,11,11,1,1,1,
  // 55xx
  37,37,37,37,37,37,37,37, 37,2,2,37,37,2,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,37,
  3,10,11,11,11,11,11,11, 11,11,0,11,11,10,0,10, 10,11,11,11,11,11,11,11, 0,11,0,11,11,10,0,0,
  11,1,11,11,11,11,11,11, 11,11,1,11,11,11,11,11, 11,11,11,11,11,11,11,11, 11,11,1,11,11,11,11,1,
  1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,11, 1,1,1,1,1,1,1,1,
  11,1,11,1,1,1,1,11, 11,11,11,11,1,1,11,11, 1,1,1,1,11,1,1,11, 1,11,1,11,11,1,1,1,
  // 56xx
  37,37,37,37,37,37,37,37, 37,2,2,37,37,2,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,37,
  3,10,11,11,11,11,11,11, 11,11,0,11,11,10,0,10, 10,11,11,11,11,11,11,11, 0,11,0,11,11,10,0,0,
  11,1,11,11,11,11,11,11, 11,11,1,11,11,11,11,11, 11,11,11,11,11,11,11,11, 11,11,1,11,11,11,11,1,
  1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,11, 1,1,1,1,1,1,1,1,
  11,1,11,1,1,1,1,11, 11,11,11,11,1,1,11,11, 1,1,1,1,11,1,1,11, 1,11,1,11,11,1,1,1,
  // 57xx
  37,37,37,37,37,37,37,37, 37,2,2,37,37,2,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,37,
  3,10,11,11,11,11,11,11, 11,11,0,11,11,10,0,10, 10,11,11,11,11,11,11,11, 0,11,0,11,11,10,0,0,
  11,1,11,11,11,11,11,11, 11,11,1,11,11,11,11,11, 11,11,11,11,11,11,11,11, 11,11,1,11,11,11,11,1,
  1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,11, 1,1,1,1,1,1,1,1,
  11,1,11,1,1,1,1,11, 11,11,11,11,1,1,11,11, 1,1,1,1,11,1,1,11, 1,11,1,11,11,1,1,1,
  // 58xx
  37,37,37,37,37,37,37,37, 37,2,2,37,37,2,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,37,
  3,10,11,11,11,11,11,11, 11,11,0,11,11,10,0,10, 10,11,11,11,11,11,11,11, 0,11,0,11,11,10,0,0,
  11,1,11,11,11,11,11,11, 11,11,1,11,11,11,11,11, 11,11,11,11,11,11,11,11, 11,11,1,11,11,11,11,1,
  1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,11, 1,1,1,1,1,1,1,1,
  11,1,11,1,1,1,1,11, 11,11,11,11,1,1,11,11, 1,1,1,1,11,1,1,11, 1,11,1,11,11,1,1,1,
  // 59xx
  37,37,37,37,37,37,37,37, 37,2,2,37,37,2,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,37,
  3,10,11,11,11,11,11,11, 11,11,0,11,11,10,0,10, 10,11,11,11,11,11,11,11, 0,11,0,11,11,10,0,0,
  11,1,11,11,11,11,11,11, 11,11,1,11,11,11,11,11, 11,11,11,11,11,11,11,11, 11,11,1,11,11,11,11,1,
  1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,11, 1,1,1,1,1,1,1,1,
  11,1,11,1,1,1,1,11, 11,11,11,11,1,1,11,11, 1,1,1,1,11,1,1,11, 1,11,1,11,11,1,1,1,
  // 5Axx
  37,37,37,37,37,37,37,37, 37,2,2,37,37,2,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,37,
  3,10,11,11,11,11,11,11, 11,11,0,11,11,10,0,10, 10,11,11,11,11,11,11,11, 0,11,0,11,11,10,0,0,
  11,1,11,11,11,11,11,11, 11,11,1,11,11,11,11,11, 11,11,11,11,11,11,11,11, 11,11,1,11,11,11,11,1,
  1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,11, 1,1,1,1,1,1,1,1,
  11,1,11,1,1,1,1,11, 11,11,11,11,1,1,11,11, 1,1,1,1,11,1,1,11, 1,11,1,11,11,1,1,1,
  // 5Bxx
  37,37,37,37,37,37,37,37, 37,2,2,37,37,2,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,37,
  3,10,11,11,11,11,11,11, 11,11,0,11,11,10,0,10, 10,11,11,11,11,11,11,11, 0,11,0,11,11,10,0,0,
  11,1,11,11,11,11,11,11, 11,11,1,11,11,11,11,11, 11,11,11,11,11,11,11,11, 11,11,1,11,11,11,11,1,
  1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,11, 1,1,1,1,1,1,1,1,
  11,1,11,1,1,1,1,11, 11,11,11,11,1,1,11,11, 1,1,1,1,11,1,1,11, 1,11,1,11,11,1,1,1,
  // 5Cxx
  37,37,37,37,37,37,37,37, 37,2,2,37,37,2,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,37,
  3,10,11,11,11,11,11,11, 11,11,0,11,11,10,0,10, 10,11,11,11,11,11,11,11, 0,11,0,11,11,10,0,0,
  11,1,11,11,11,11,11,11, 11,11,1,11,11,11,11,11, 11,11,11,11,11,11,11,11, 11,11,1,11,11,11,11,1,
  1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,11, 1,1,1,1,1,1,1,1,
  11,1,11,1,1,1,1,11, 11,11,11,11,1,1,11,11, 1,1,1,1,11,1,1,11, 1,11,1,11,11,1,1,1,
  // 5Dxx
  37,37,37,37,37,37,37,37, 37,2,2,37,37,2,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,37,
  3,10,11,11,11,11,11,11, 11,11,0,11,11,10,0,10, 10,11,11,11,11,11,11,11, 0,11,0,11,11,10,0,0,
  11,1,11,11,11,11,11,11, 11,11,1,11,11,11,11,11, 11,11,11,11,11,11,11,11, 11,11,1,11,11,11,11,1,
  1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,11, 1,1,1,1,1,1,1,1,
  11,1,11,1,1,1,1,11, 11,11,11,11,1,1,11,11, 1,1,1,1,11,1,1,11, 1,11,1,11,11,1,1,1,
  // 5Exx
  37,37,37,37,37,37,37,37, 37,2,2,37,37,2,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,37,
  3,10,11,11,11,11,11,11, 11,11,0,11,11,10,0,10, 10,11,11,11,11,11,11,11, 0,11,0,11,11,10,0,0,
  11,1,11,11,11,11,11,11, 11,11,1,11,11,11,11,11, 11,11,11,11,11,11,11,11, 11,11,1,11,11,11,11,1,
  1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,11, 1,1,1,1,1,1,1,1,
  11,1,11,1,1,1,1,11, 11,11,11,11,1,1,11,11, 1,1,1,1,11,1,1,11, 1,11,1,11,11,1,1,1,
  // 5Fxx
  37,37,37,37,37,37,37,37, 37,2,2,37,37,2,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,37,
  3,10,11,11,11,11,11,11, 11,11,0,11,11,10,0,10, 10,11,11,11,11,11,11,11, 0,11,0,11,11,10,0,0,
  11,1,11,11,11,11,11,11, 11,11,1,11,11,11,11,11, 11,11,11,11,11,11,11,11, 11,11,1,11,11,11,11,1,
  1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,11, 1,1,1,1,1,1,1,1,
  11,1,11,1,1,1,1,11, 11,11,11,11,1,1,11,11, 1,1,1,1,11,1,1,11, 1,11,1,11,11,1,1,1,
  // 60xx
  37,37,37,37,37,37,37,37, 37,2,2,37,37,2,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,37,
  3,10,11,11,11,11,11,11, 11,11,0,11,11,10,0,10, 10,11,11,11,11,11,11,11, 0,11,0,11,11,10,0,0,
  11,1,11,11,11,11,11,11, 11,11,1,11,11,11,11,11, 11,11,11,11,11,11,11,11, 11,11,1,11,11,11,11,1,
  1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,11, 1,1,1,1,1,1,1,1,
  11,1,11,1,1,1,1,11, 11,11,11,11,1,1,11,11, 1,1,1,1,11,1,1,11, 1,11,1,11,11,1,1,1,
  // 61xx
  37,37,37,37,37,37,37,37, 37,2,2,37,37,2,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,37,
  3,10,11,11,11,11,11,11, 11,11,0,11,11,10,0,10, 10,11,11,11,11,11,11,11, 0,11,0,11,11,10,0,0,
  11,1,11,11,11,11,11,11, 11,11,1,11,11,11,11,11, 11,11,11,11,11,11,11,11, 11,11,1,11,11,11,11,1,
  1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,11, 1,1,1,1,1,1,1,1,
  11,1,11,1,1,1,1,11, 11,11,11,11,1,1,11,11, 1,1,1,1,11,1,1,11, 1,11,1,11,11,1,1,1,
  // 62xx
  37,37,37,37,37,37,37,37, 37,2,2,37,37,2,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,37,
  3,10,11,11,11,11,11,11, 11,11,0,11,11,10,0,10, 10,11,11,11,11,11,11,11, 0,11,0,11,11,10,0,0,
  11,1,11,11,11,11,11,11, 11,11,1,11,11,11,11,11, 11,11,11,11,11,11,11,11, 11,11,1,11,11,11,11,1,
  1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,11, 1,1,1,1,1,1,1,1,
  11,1,11,1,1,1,1,11, 11,11,11,11,1,1,11,11, 1,1,1,1,11,1,1,11, 1,11,1,11,11,1,1,1,
  // 63xx
  37,37,37,37,37,37,37,37, 37,2,2,37,37,2,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,37,
  3,10,11,11,11,11,11,11, 11,11,0,11,11,10,0,10, 10,11,11,11,11,11,11,11, 0,11,0,11,11,10,0,0,
  11,1,11,11,11,11,11,11, 11,11,1,11,11,11,11,11, 11,11,11,11,11,11,11,11, 11,11,1,11,11,11,11,1,
  1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,11, 1,1,1,1,1,1,1,1,
  11,1,11,1,1,1,1,11, 11,11,11,11,1,1,11,11, 1,1,1,1,11,1,1,11, 1,11,1,11,11,1,1,1,
  // 64xx
  37,37,37,37,37,37,37,37, 37,2,2,37,37,2,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,37,
  3,10,11,11,11,11,11,11, 11,11,0,11,11,10,0,10, 10,11,11,11,11,11,11,11, 0,11,0,11,11,10,0,0,
  11,1,11,11,11,11,11,11, 11,11,1,11,11,11,11,11, 11,11,11,11,11,11,11,11, 11,11,1,11,11,11,11,1,
  1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,11, 1,1,1,1,1,1,1,1,
  11,1,11,1,1,1,1,11, 11,11,11,11,1,1,11,11, 1,1,1,1,11,1,1,11, 1,11,1,11,11,1,1,1,
  // 65xx
  37,37,37,37,37,37,37,37, 37,2,2,37,37,2,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,37,
  3,10,11,11,11,11,11,11, 11,11,0,11,11,10,0,10, 10,11,11,11,11,11,11,11, 0,11,0,11,11,10,0,0,
  11,1,11,11,11,11,11,11, 11,11,1,11,11,11,11,11, 11,11,11,11,11,11,11,11, 11,11,1,11,11,11,11,1,
  1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,11, 1,1,1,1,1,1,1,1,
  11,1,11,1,1,1,1,11, 11,11,11,11,1,1,11,11, 1,1,1,1,11,1,1,11, 1,11,1,11,11,1,1,1,
  // 66xx
  37,37,37,37,37,37,37,37, 37,2,2,37,37,2,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,37,
  3,10,11,11,11,11,11,11, 11,11,0,11,11,10,0,10, 10,11,11,11,11,11,11,11, 0,11,0,11,11,10,0,0,
  11,1,11,11,11,11,11,11, 11,11,1,11,11,11,11,11, 11,11,11,11,11,11,11,11, 11,11,1,11,11,11,11,1,
  1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,11, 1,1,1,1,1,1,1,1,
  11,1,11,1,1,1,1,11, 11,11,11,11,1,1,11,11, 1,1,1,1,11,1,1,11, 1,11,1,11,11,1,1,1,
  // 67xx
  37,37,37,37,37,37,37,37, 37,2,2,37,37,2,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,37,
  3,10,11,11,11,11,11,11, 11,11,0,11,11,10,0,10, 10,11,11,11,11,11,11,11, 0,11,0,11,11,10,0,0,
  11,1,11,11,11,11,11,11, 11,11,1,11,11,11,11,11, 11,11,11,11,11,11,11,11, 11,11,1,11,11,11,11,1,
  1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,11, 1,1,1,1,1,1,1,1,
  11,1,11,1,1,1,1,11, 11,11,11,11,1,1,11,11, 1,1,1,1,11,1,1,11, 1,11,1,11,11,1,1,1,
  // 68xx
  37,37,37,37,37,37,37,37, 37,2,2,37,37,2,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,37,
  3,10,11,11,11,11,11,11, 11,11,0,11,11,10,0,10, 10,11,11,11,11,11,11,11, 0,11,0,11,11,10,0,0,
  11,1,11,11,11,11,11,11, 11,11,1,11,11,11,11,11, 11,11,11,11,11,11,11,11, 11,11,1,11,11,11,11,1,
  1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,11, 1,1,1,1,1,1,1,1,
  11,1,11,1,1,1,1,11, 11,11,11,11,1,1,11,11, 1,1,1,1,11,1,1,11, 1,11,1,11,11,1,1,1,
  // 69xx
  37,37,37,37,37,37,37,37, 37,2,2,37,37,2,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,37,
  3,10,11,11,11,11,11,11, 11,11,0,11,11,10,0,10, 10,11,11,11,11,11,11,11, 0,11,0,11,11,10,0,0,
  11,1,11,11,11,11,11,11, 11,11,1,11,11,11,11,11, 11,11,11,11,11,11,11,11, 11,11,1,11,11,11,11,1,
  1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,11, 1,1,1,1,1,1,1,1,
  11,1,11,1,1,1,1,11, 11,11,11,11,1,1,11,11, 1,1,1,1,11,1,1,11, 1,11,1,11,11,1,1,1,
  // 6Axx
  37,37,37,37,37,37,37,37, 37,2,2,37,37,2,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,37,
  3,10,11,11,11,11,11,11, 11,11,0,11,11,10,0,10, 10,11,11,11,11,11,11,11, 0,11,0,11,11,10,0,0,
  11,1,11,11,11,11,11,11, 11,11,1,11,11,11,11,11, 11,11,11,11,11,11,11,11, 11,11,1,11,11,11,11,1,
  1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,11, 1,1,1,1,1,1,1,1,
  11,1,11,1,1,1,1,11, 11,11,11,11,1,1,11,11, 1,1,1,1,11,1,1,11, 1,11,1,11,11,1,1,1,
  // 6Bxx
  37,37,37,37,37,37,37,37, 37,2,2,37,37,2,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,37,
  3,10,11,11,11,11,11,11, 11,11,0,11,11,10,0,10, 10,11,11,11,11,11,11,11, 0,11,0,11,11,10,0,0,
  11,1,11,11,11,11,11,11, 11,11,1,11,11,11,11,11, 11,11,11,11,11,11,11,11, 11,11,1,11,11,11,11,1,
  1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,11, 1,1,1,1,1,1,1,1,
  11,1,11,1,1,1,1,11, 11,11,11,11,1,1,11,11, 1,1,1,1,11,1,1,11, 1,11,1,11,11,1,1,1,
  // 6Cxx
  37,37,37,37,37,37,37,37, 37,2,2,37,37,2,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,37,
  3,10,11,11,11,11,11,11, 11,11,0,11,11,10,0,10, 10,11,11,11,11,11,11,11, 0,11,0,11,11,10,0,0,
  11,1,11,11,11,11,11,11, 11,11,1,11,11,11,11,11, 11,11,11,11,11,11,11,11, 11,11,1,11,11,11,11,1,
  1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,11, 1,1,1,1,1,1,1,1,
  11,1,11,1,1,1,1,11, 11,11,11,11,1,1,11,11, 1,1,1,1,11,1,1,11, 1,11,1,11,11,1,1,1,
  // 6Dxx
  37,37,37,37,37,37,37,37, 37,2,2,37,37,2,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,37,
  3,10,11,11,11,11,11,11, 11,11,0,11,11,10,0,10, 10,11,11,11,11,11,11,11, 0,11,0,11,11,10,0,0,
  11,1,11,11,11,11,11,11, 11,11,1,11,11,11,11,11, 11,11,11,11,11,11,11,11, 11,11,1,11,11,11,11,1,
  1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,11, 1,1,1,1,1,1,1,1,
  11,1,11,1,1,1,1,11, 11,11,11,11,1,1,11,11, 1,1,1,1,11,1,1,11, 1,11,1,11,11,1,1,1,
  // 6Exx
  37,37,37,37,37,37,37,37, 37,2,2,37,37,2,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,37,
  3,10,11,11,11,11,11,11, 11,11,0,11,11,10,0,10, 10,11,11,11,11,11,11,11, 0,11,0,11,11,10,0,0,
  11,1,11,11,11,11,11,11, 11,11,1,11,11,11,11,11, 11,11,11,11,11,11,11,11, 11,11,1,11,11,11,11,1,
  1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,11, 1,1,1,1,1,1,1,1,
  11,1,11,1,1,1,1,11, 11,11,11,11,1,1,11,11, 1,1,1,1,11,1,1,11, 1,11,1,11,11,1,1,1,
  // 6Fxx
  37,37,37,37,37,37,37,37, 37,2,2,37,37,2,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,37,
  3,10,11,11,11,11,11,11, 11,11,0,11,11,10,0,10, 10,11,11,11,11,11,11,11, 0,11,0,11,11,10,0,0,
  11,1,11,11,11,11,11,11, 11,11,1,11,11,11,11,11, 11,11,11,11,11,11,11,11, 11,11,1,11,11,11,11,1,
  1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,11, 1,1,1,1,1,1,1,1,
  11,1,11,1,1,1,1,11, 11,11,11,11,1,1,11,11, 1,1,1,1,11,1,1,11, 1,11,1,11,11,1,1,1,
  // 70xx
  37,37,37,37,37,37,37,37, 37,2,2,37,37,2,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,37,
  3,10,11,11,11,11,11,11, 11,11,0,11,11,10,0,10, 10,11,11,11,11,11,11,11, 0,11,0,11,11,10,0,0,
  11,1,11,11,11,11,11,11, 11,11,1,11,11,11,11,11, 11,11,11,11,11,11,11,11, 11,11,1,11,11,11,11,1,
  1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,11, 1,1,1,1,1,1,1,1,
  11,1,11,1,1,1,1,11, 11,11,11,11,1,1,11,11, 1,1,1,1,11,1,1,11, 1,11,1,11,11,1,1,1,
  // 71xx
  37,37,37,37,37,37,37,37, 37,2,2,37,37,2,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,37,
  3,10,11,11,11,11,11,11, 11,11,0,11,11,10,0,10, 10,11,11,11,11,11,11,11, 0,11,0,11,11,10,0,0,
  11,1,11,11,11,11,11,11, 11,11,1,11,11,11,11,11, 11,11,11,11,11,11,11,11, 11,11,1,11,11,11,11,1,
  1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,11, 1,1,1,1,1,1,1,1,
  11,1,11,1,1,1,1,11, 11,11,11,11,1,1,11,11, 1,1,1,1,11,1,1,11, 1,11,1,11,11,1,1,1,
  // 72xx
  37,37,37,37,37,37,37,37, 37,2,2,37,37,2,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,37,
  3,10,11,11,11,11,11,11, 11,11,0,11,11,10,0,10, 10,11,11,11,11,11,11,11, 0,11,0,11,11,10,0,0,
  11,1,11,11,11,11,11,11, 11,11,1,11,11,11,11,11, 11,11,11,11,11,11,11,11, 11,11,1,11,11,11,11,1,
  1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,11, 1,1,1,1,1,1,1,1,
  11,1,11,1,1,1,1,11, 11,11,11,11,1,1,11,11, 1,1,1,1,11,1,1,11, 1,11,1,11,11,1,1,1,
  // 73xx
  37,37,37,37,37,37,37,37, 37,2,2,37,37,2,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,37,
  3,10,11,11,11,11,11,11, 11,11,0,11,11,10,0,10, 10,11,11,11,11,11,11,11, 0,11,0,11,11,10,0,0,
  11,1,11,11,11,11,11,31, 11,11,1,11,11,11,11,11, 11,11,11,11,11,11,11,11, 11,11,1,11,11,11,11,1,
  1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,11, 1,1,1,1,1,1,1,1,
  11,33,11,1,1,1,1,11, 11,11,11,11,1,1,11,11, 1,1,1,1,11,1,1,11, 1,11,1,11,11,1,1,1,
  // 74xx
  37,37,37,37,37,37,37,37, 37,2,2,37,37,2,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,37,
  3,10,11,11,11,11,11,11, 11,11,0,11,11,10,0,10, 10,11,11,11,11,11,11,11, 0,11,0,11,11,10,0,0,
  11,1,11,11,11,11,11,11, 11,11,1,11,11,11,11,11, 11,11,11,11,11,11,11,11, 11,11,1,11,11,11,11,1,
  1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,11, 1,1,1,1,1,1,1,1,
  11,1,11,1,1,1,1,11, 11,11,11,11,1,1,11,11, 1,1,1,1,11,1,1,11, 1,11,1,11,11,1,1,1,
  // 75xx
  37,37,37,37,37,37,37,37, 37,2,2,37,37,2,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,37,
  3,10,11,11,11,11,11,11, 11,11,0,11,11,10,0,10, 10,11,11,11,11,11,11,11, 0,11,0,11,11,10,0,0,
  11,1,11,11,11,11,11,11, 11,11,1,11,11,11,11,11, 11,11,11,11,11,11,11,11, 11,11,1,11,11,11,11,1,
  1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,11, 1,1,1,1,1,1,1,1,
  11,1,11,1,1,1,1,11, 11,11,11,11,1,1,11,11, 1,1,1,1,11,1,1,11, 1,11,1,11,11,1,1,1,
  // 76xx
  37,37,37,37,37,37,37,37, 37,2,2,37,37,2,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,37,
  3,10,11,11,11,11,11,11, 11,11,0,11,11,10,0,10, 10,11,11,11,11,11,11,11, 0,11,0,11,11,10,0,0,
  11,1,11,11,11,11,11,11, 11,11,1,11,11,11,11,11, 11,11,11,11,11,11,11,11, 11,11,1,11,11,11,11,1,
  1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,11, 1,1,1,1,1,1,1,1,
  11,1,11,1,1,1,1,11, 11,11,11,11,1,1,11,11, 1,1,1,1,11,1,1,11, 1,11,1,11,11,1,1,1,
  // 77xx
  37,37,37,37,37,37,37,37, 37,2,2,37,37,2,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,37,
  3,10,11,11,11,11,11,11, 11,11,0,11,11,10,0,10, 10,11,11,11,11,11,11,11, 0,11,0,11,11,10,0,0,
  11,1,11,11,11,11,11,11, 11,11,1,11,11,11,11,11, 11,11,11,11,11,11,11,11, 11,11,1,11,11,11,11,1,
  1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,11, 1,1,1,1,1,1,1,1,
  11,1,11,1,1,1,1,11, 11,11,11,11,1,1,11,11, 1,1,1,1,11,1,1,11, 1,11,1,11,11,1,1,1,
  // 78xx
  37,37,37,37,37,37,37,37, 37,2,2,37,37,2,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,37,
  3,10,11,11,11,11,11,11, 11,11,0,11,11,10,0,10, 10,11,11,11,11,11,11,11, 0,11,0,11,11,10,0,0,
  11,1,11,11,11,11,11,11, 11,11,1,11,11,11,11,11, 11,11,11,11,11,11,11,11, 11,11,1,11,11,11,11,1,
  1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,11, 1,1,1,1,1,1,1,1,
  11,1,11,1,1,1,1,11, 11,11,11,11,1,1,11,11, 1,1,1,1,11,1,1,11, 1,11,1,11,11,1,1,1,
  // 79xx
  37,37,37,37,37,37,37,37, 37,2,2,37,37,2,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,37,
  3,10,11,11,11,11,11,11, 11,11,0,11,11,10,0,10, 10,11,11,11,11,11,11,11, 0,11,0,11,11,10,0,0,
  11,1,11,11,11,11,11,11, 11,11,1,11,11,11,11,11, 11,11,11,11,11,11,11,11, 11,11,1,11,11,11,11,1,
  1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,11, 1,1,1,1,1,1,1,1,
  11,1,11,1,1,1,1,11, 11,11,11,11,1,1,11,11, 1,1,1,1,11,1,1,11, 1,11,1,11,11,1,1,1,
  // 7Axx
  37,37,37,37,37,37,37,37, 37,2,2,37,37,2,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,37,
  3,10,11,11,11,11,11,11, 11,11,0,11,11,10,0,10, 10,11,11,11,11,11,11,11, 0,11,0,11,11,10,0,0,
  11,1,11,11,11,11,11,11, 11,11,1,11,11,11,11,11, 11,11,11,11,11,11,11,11, 11,11,1,11,11,11,11,1,
  1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,11, 1,1,1,1,1,1,1,1,
  11,1,11,1,1,1,1,11, 11,11,11,11,1,1,11,11, 1,1,1,1,11,1,1,11, 1,11,1,11,11,1,1,1,
  // 7Bxx
  37,37,37,37,37,37,37,37, 37,2,2,37,37,2,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,37,
  3,10,11,11,11,11,11,11, 11,11,0,11,11,10,0,10, 10,11,11,11,11,11,11,11, 0,11,0,11,11,10,0,0,
  11,1,11,11,11,11,11,11, 11,11,1,11,11,11,11,11, 11,11,11,11,11,11,11,11, 11,11,1,11,11,11,11,1,
  1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,11, 1,1,1,1,1,1,1,1,
  11,1,11,1,1,1,1,11, 11,11,11,11,1,1,11,11, 1,1,1,1,11,1,1,11, 1,11,1,11,11,1,1,1,
  // 7Cxx
  37,37,37,37,37,37,37,37, 37,2,2,37,37,2,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,37,
  3,10,11,11,11,11,11,11, 11,11,0,11,11,10,0,10, 10,11,11,11,11,11,11,11, 0,11,0,11,11,10,0,0,
  11,1,11,11,11,11,11,11, 11,11,1,11,11,11,11,11, 11,11,11,11,11,11,11,11, 11,11,1,11,11,11,11,1,
  1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,11, 1,1,1,1,1,1,1,1,
  11,1,11,1,1,1,1,11, 11,11,11,11,1,1,11,11, 1,1,1,1,11,1,1,11, 1,11,1,11,11,1,1,1,
  // 7Dxx
  37,37,37,37,37,37,37,37, 37,2,2,37,37,2,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,37,
  3,10,11,11,11,11,11,11, 11,11,0,11,11,10,0,10, 10,11,11,11,11,11,11,11, 0,11,0,11,11,10,0,0,
  11,1,11,11,11,11,11,11, 11,11,1,11,11,11,11,11, 11,11,11,11,11,11,11,11, 11,11,1,11,11,11,11,1,
  1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,11, 1,1,1,1,1,1,1,1,
  11,1,11,1,1,1,1,11, 11,11,11,11,1,1,11,11, 1,1,1,1,11,1,1,11, 1,11,1,11,11,1,1,1,
  // 7Exx
  2,39,39,39,39,39,39,39, 39,2,2,39,39,2,42,42, 39,39,39,39,39,39,39,39, 39,39,39,56,39,39,39,39,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0,
  0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,0, 0,0,0,0,0,0,0,3,
  3,3,3,3,39,39,11,11, 11,3,3,3,3,3,3,3, 3,3,3,3,3,11,3,3, 39,11,39,3,39,39,39,37,
  11,3,3,3,11,3,3,3, 3,3,3,3,3,3,11,3, 11,3,3,3,3,3,3,3, 3,3,3,3,3,3,3,3,
  2,2,2,2,2,2,2,11, 2,2,2,2,11,11,2,2, 2,11,2,11,11,2,2,2, 2,37,11,11,11,11,11,11,
  17,11,2,2,2,2,2,2, 2,2,2,2,2,2,17,2, 14,17,17,11,37,1,11,17, 16,17,11,39,37,11,39,37,
  // 7Fxx
  56,39,39,39,39,39,39,39, 39,39,39,39,39,39,39,39, 39,39,39,37,39,37,37,39, 37,39,39,39,39,37,39,39,
  39,39,39,39,39,39,39,39, 39,37,39,39,39,39,39,39, 37,39,39,39,39,39,37,39, 39,39,37,39,39,39,39,39,
  39,39,39,39,39,39,39,39, 39,39,39,39,39,39,37,39, 37,37,37,37,37,39,37,37, 39,37,39,37,37,37,39,37,
  37,39,39,37,37,37,37,39, 39,37,37,37,37,37,37,39, 37,37,37,39,37,37,37,39, 39,37,39,37,39,37,37,39,
  37,39,37,37,39,37,39,39, 37,39,37,37,39,37,37,37, 39,39,39,39,37,39,37,39, 39,37,37,39,39,39,39,39,
  39,39,39,39,37,39,39,39, 37,37,39,39,39,39,39,39, 39,39,39,39,37,39,39,39, 39,39,39,39,39,37,39,39,
  39,37,39,39,39,37,39,39, 39,39,39,39,37,39,39,39, 39,39,37,39,37,39,39,39, 56,56,39,56,56,56,56,56,
  37,39,39,39,39,39,39,39, 39,37,39,39,39,39,39,39, 37,37,39,39,39,39,39,39, 39,39,39,37,37,39,39,37,
  // 80xx
  11,37,39,37,39,37,39,39, 39,2,2,39,37,2,39,39, 37,39,39,39,39,37,39,37, 39,39,39,39,39,39,39,39,
  2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2,
  2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,11, 2,11,11,2,11,11,11,11, 11,11,11,2,11,2,2,2,
  2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,11,2,2, 2,2,2,2,2,2,2,39,
  2,2,2,2,36,2,2,36, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2,
  2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 59,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2,
  10,39,2,2,2,10,37,2, 10,2,39,2,37,39,2,2, 2,2,2,10,39,2,33,11, 2,2,2,2,37,5,37,5,
  2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 36,31,39,39,39,31,37,39, 37,39,37,39,11,37,4,37,
  // 81xx
  56,39,37,39,39,37,39,37, 37,2,2,39,39,2,39,39, 37,37,39,39,39,39,39,39, 39,37,37,39,39,39,39,39,
  2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2,
  6,6,6,6,6,6,6,6, 6,6,2,6,6,6,2,6, 6,6,6,2,2,6,6,2, 6,2,6,6,6,6,6,6,
  6,6,6,6,6,6,6,6, 6,6,6,6,6,6,6,6, 6,6,6,6,6,6,6,6, 6,6,6,6,6,6,6,37,
  36,6,2,6,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,6,2, 2,2,6,2,6,2,2,2,
  2,6,6,2,2,6,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2,
  10,39,2,2,2,10,4,11, 6,6,6,6,6,6,2,2, 2,11,11,11,11,39,12,19, 2,2,6,2,6,6,6,6,
  2,6,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 6,6,6,6,6,6,6,6, 4,4,37,39,6,19,39,37,
  // 82xx
  56,39,39,39,39,39,39,37, 37,2,2,39,39,2,42,39, 39,39,37,39,39,39,39,39, 39,39,39,39,39,39,37,39,
  2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2,
  2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,6, 6,6,6,6,6,6,6,6, 6,2,2,2,2,2,2,2,
  6,6,6,6,6,6,6,6, 6,6,6,6,6,6,6,6, 6,6,6,6,6,6,6,6, 6,6,2,2,2,2,2,37,
  2,2,2,2,2,6,2,2, 2,2,2,2,2,2,6,2, 6,36,2,2,6,6,6,6, 2,2,2,36,2,2,2,6,
  6,2,6,2,6,2,6,2, 6,6,6,6,2,6,6,2, 2,6,6,6,2,6,6,6, 2,2,2,6,6,6,6,6,
  6,6,6,6,6,6,6,6, 6,6,6,6,6,6,6,6, 6,6,6,6,6,6,6,6, 6,6,6,6,6,6,6,6,
  6,6,6,2,2,2,6,6, 6,6,6,6,6,6,6,2, 6,6,31,12,11,31,39,19, 5,19,39,19,11,33,39,37,
  // 83xx
  56,39,37,37,37,37,39,39, 39,2,2,39,39,2,37,39, 39,39,39,39,39,39,39,37, 39,39,39,39,37,39,39,39,
  2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 37,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2,
  6,6,6,6,6,6,6,6, 6,6,6,6,6,6,6,6, 6,6,6,6,6,6,6,6, 6,6,6,6,6,6,6,6,
  6,6,6,6,6,6,6,6, 6,6,6,6,6,6,6,6, 6,6,6,6,6,6,6,6, 6,6,6,6,6,6,6,37,
  6,6,6,2,6,6,2,6, 2,6,6,6,6,6,2,6, 2,2,2,6,2,2,2,2, 2,2,2,2,2,2,2,2,
  2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,6,
  6,6,59,6,6,59,59,6, 19,6,6,6,6,6,2,2, 2,2,6,6,19,6,6,11, 2,2,2,2,37,6,4,4,
  2,2,2,2,2,2,2,2, 2,2,2,2,2,2,5,2, 31,4,11,10,39,19,19,11, 37,39,39,39,39,39,39,37,
  // 84xx
  57,39,39,39,39,39,39,39, 39,2,2,39,39,2,39,39, 39,39,39,39,39,39,39,39, 37,39,39,39,39,39,39,39,
  2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2,
  2,2,2,2,11,2,2,2, 2,2,2,11,2,2,2,2, 2,2,2,11,2,2,11,11, 2,2,11,2,2,2,2,2,
  37,11,2,2,11,2,2,11, 2,2,2,11,2,2,11,2, 2,6,2,2,6,6,39,2, 6,6,11,6,2,6,2,37,
  36,2,2,2,2,36,2,2, 2,2,2,36,2,36,36,2, 2,2,2,36,2,2,2,2, 2,2,2,2,2,2,2,2,
  6,2,59,6,2,6,2,2, 36,6,6,6,6,2,36,6, 2,2,2,2,2,6,2,2, 2,2,6,5,2,5,2,2,
  10,10,2,2,2,10,10,10, 12,12,10,2,10,10,2,2, 2,2,10,19,10,19,12,2, 2,2,2,2,12,19,16,10,
  2,2,2,2,2,2,2,2, 2,2,2,2,2,2,5,2, 36,10,10,10,11,12,12,10, 12,12,12,39,11,39,16,37,
  // 85xx
  56,11,39,37,39,39,39,39, 39,2,2,39,39,2,39,11, 39,39,39,37,37,39,39,37, 37,37,37,39,39,39,39,39,
  2,2,2,2,11,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2,
  2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,39,2,2,2,2,11,11, 2,2,2,2,2,2,19,2,
  2,11,11,2,11,2,2,2, 2,11,33,2,2,2,2,11, 11,11,2,11,11,2,33,11, 11,11,33,2,2,2,2,39,
  2,2,36,2,2,2,2,2, 2,2,36,2,2,2,2,36, 2,36,2,2,2,2,36,36, 2,2,2,5,2,2,2,36,
  2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2,
  39,5,2,2,19,11,5,11, 11,12,5,2,2,37,2,2, 2,2,6,11,11,19,12,39, 2,2,2,2,11,11,11,11,
  2,2,2,2,2,2,2,2, 2,2,2,2,2,2,5,2, 36,36,4,23,23,4,19,11, 19,11,12,37,11,33,11,37,
  // 86xx
  56,39,39,11,39,39,39,37, 39,2,2,39,39,2,39,39, 39,39,39,39,39,39,39,39, 39,39,39,39,39,39,39,39,
  2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2,
  2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2,
  2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,37,
  2,6,2,2,36,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,19,2, 2,2,2,2,2,2,2,2,
  2,2,2,2,2,2,36,2, 2,2,2,2,2,5,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2,
  39,19,2,2,2,2,2,2, 11,2,19,2,19,37,2,2, 2,2,4,12,12,19,12,11, 2,2,2,2,11,12,4,6,
  2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 36,36,39,31,39,4,39,11, 4,39,39,39,11,12,19,37,
  // 87xx
  56,39,37,39,39,39,39,39, 39,2,2,39,39,2,39,37, 39,37,37,37,39,37,39,37, 37,39,39,39,37,39,39,39,
  2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2,
  6,6,6,6,6,6,6,6, 6,2,6,6,2,2,2,2, 2,11,2,2,6,6,6,6, 6,6,6,2,2,2,2,2,
  6,2,2,2,6,39,2,31, 2,11,11,2,2,2,2,6, 2,6,31,31,2,2,31,2, 11,2,37,2,2,2,2,37,
  36,6,2,2,2,36,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,5,2, 2,2,2,36,2,2,5,2,
  2,2,36,2,2,2,2,2, 2,5,2,36,2,2,11,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2,
  19,12,2,2,2,2,4,11, 4,12,4,2,19,39,2,2, 2,2,4,12,12,4,19,11, 2,2,2,2,11,11,31,19,
  2,2,2,2,2,2,2,2, 2,2,2,2,2,2,5,2, 36,4,37,11,11,11,4,11, 19,37,19,11,19,4,4,37,
  // 88xx
  57,39,39,39,39,39,39,39, 39,2,2,39,39,2,39,39, 39,39,39,39,39,39,39,39, 39,39,39,39,39,39,39,39,
  2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2,
  2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2,
  2,2,2,2,2,2,2,2, 37,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,39,
  2,36,36,36,2,36,2,2, 2,2,2,36,36,36,2,2, 2,2,2,36,2,2,2,2, 2,2,2,2,2,2,2,36,
  2,2,6,6,6,2,2,36, 2,2,2,2,2,2,6,2, 2,2,2,6,2,6,2,2, 2,2,2,2,6,2,2,2,
  6,6,2,6,6,6,6,6, 6,6,6,6,6,6,6,6, 6,6,6,6,6,6,6,6, 2,6,6,6,6,6,6,6,
  2,6,6,2,2,2,2,2, 2,2,6,2,2,2,6,2, 6,6,6,6,6,6,6,6, 6,6,6,6,6,37,39,37,
  // 89xx
  56,39,39,39,39,39,39,37, 39,2,2,39,39,2,39,39, 37,39,39,39,39,39,39,39, 39,39,39,39,39,39,39,39,
  2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 37,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2,
  6,6,6,2,2,6,6,6, 6,2,6,6,6,2,6,2, 6,6,6,6,2,2,6,2, 6,6,6,2,6,2,6,6,
  6,6,6,6,2,6,6,6, 6,6,6,6,6,2,2,2, 6,6,6,6,6,6,6,6, 6,37,6,6,6,6,6,37,
  2,2,2,6,6,6,2,2, 2,6,2,2,2,2,2,6, 6,6,36,2,6,2,6,6, 6,2,6,6,6,6,6,6,
  6,6,6,2,6,2,2,2, 6,2,6,6,2,6,6,2, 2,2,2,6,2,2,2,6, 6,6,6,6,6,6,2,6,
  6,6,6,6,6,6,6,6, 6,6,6,6,6,6,6,2, 2,6,6,6,6,6,6,6, 6,6,6,6,6,6,6,6,
  2,6,2,2,2,2,6,2, 2,2,6,2,2,2,6,6, 6,6,6,6,6,6,6,6, 6,6,6,6,6,39,39,37,
  // 8Axx
  56,39,37,39,39,39,39,39, 37,2,2,39,39,2,37,39, 39,39,39,37,39,39,39,39, 37,39,39,39,39,39,39,39,
  2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 37,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2,
  6,12,6,6,6,6,6,6, 2,12,6,6,12,2,12,6, 12,6,2,2,6,6,12,6, 6,6,6,6,2,2,6,6,
  6,12,2,2,6,6,37,6, 2,6,6,12,6,6,6,6, 6,37,6,6,6,6,6,6, 6,6,6,6,6,6,6,39,
  2,6,2,6,6,36,36,6, 6,2,2,6,6,12,36,6, 2,2,36,2,2,2,2,2, 2,6,6,2,2,2,2,2,
  2,2,2,6,2,2,6,6, 2,2,6,2,2,2,6,6, 6,2,6,6,6,2,2,6, 2,2,6,6,2,6,6,2,
  10,12,6,6,6,6,12,6, 6,6,6,6,6,12,6,6, 2,6,6,6,6,6,6,6, 6,6,6,6,6,6,6,6,
  2,6,6,2,2,2,2,2, 2,2,2,6,2,6,6,2, 6,6,6,6,6,6,6,6, 6,6,6,6,6,39,4,37,
  // 8Bxx
  56,39,37,39,39,39,39,39, 39,2,2,39,39,2,39,39, 39,39,39,39,39,39,39,39, 39,37,39,37,39,37,39,39,
  2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 37,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2,
  6,6,6,6,2,6,6,6, 6,6,2,6,6,6,6,6, 6,6,6,6,6,6,6,2, 6,6,6,2,6,2,6,6,
  6,2,2,6,2,6,2,6, 2,6,6,6,6,2,2,31, 6,6,6,6,6,6,6,2, 6,6,6,6,6,6,6,37,
  2,6,2,6,6,6,6,2, 2,2,36,6,6,2,6,6, 6,2,6,6,2,2,6,2, 2,2,5,6,2,2,6,6,
  2,2,2,6,2,2,6,2, 2,2,2,6,2,6,2,2, 2,2,6,6,6,6,6,6, 6,2,6,6,2,6,6,6,
  6,6,6,6,6,6,6,6, 6,6,6,6,6,6,6,6, 6,6,6,6,6,6,6,6, 6,6,6,6,6,6,6,6,
  6,6,6,2,2,2,2,2, 2,2,6,2,2,2,6,2, 6,6,6,6,6,6,6,6, 6,6,6,6,6,39,37,37,
  // 8Cxx
  56,39,39,37,39,39,39,39, 37,2,2,37,39,2,39,39, 37,39,39,37,39,39,39,39, 39,39,39,39,39,39,39,39,
  2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2,
  6,2,2,12,2,2,6,2, 2,6,6,6,12,2,6,2, 6,6,6,6,2,11,2,6, 6,6,6,6,6,2,6,6,
  6,6,6,6,2,6,6,6, 6,6,6,6,2,2,6,6, 6,6,2,2,2,6,6,12, 6,6,2,6,6,6,2,37,
  2,2,6,2,2,2,6,2, 6,2,6,6,6,36,6,6, 2,2,6,6,6,6,2,2, 6,6,6,6,6,6,6,6,
  6,2,6,6,6,2,2,6, 2,6,2,2,6,6,2,6, 6,6,2,6,6,6,2,6, 6,6,2,6,6,2,6,2,
  6,6,6,6,6,6,12,6, 6,6,6,6,6,6,6,6, 2,6,6,6,6,6,6,6, 2,2,2,2,6,6,6,6,
  2,6,2,2,2,2,2,2, 2,2,6,2,2,2,6,2, 6,6,6,6,6,6,6,6, 6,6,6,6,6,4,4,37,
  // 8Dxx
  57,39,39,39,39,39,39,39, 39,2,2,39,39,2,39,39, 39,39,39,39,39,39,39,39, 39,39,39,39,39,39,37,37,
  2,2,2,37,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2,
  6,12,2,6,6,6,6,6, 6,6,6,6,6,2,6,6, 6,6,6,6,6,6,2,6, 6,6,6,6,6,6,6,6,
  6,6,6,2,6,6,6,6, 6,6,6,6,6,6,2,6, 2,6,6,6,2,6,6,6, 6,6,6,2,6,6,6,39,
  6,2,6,2,6,36,6,6, 2,2,2,6,6,36,2,6, 6,6,2,2,2,2,2,2, 2,36,2,36,6,6,6,2,
  6,6,6,2,2,6,2,2, 2,6,6,2,6,6,2,2, 2,2,6,2,2,2,6,2, 6,6,2,6,2,6,2,2,
  6,12,6,6,6,6,6,6, 6,6,6,6,6,6,6,6, 6,11,6,6,6,6,6,6, 6,6,6,6,6,6,6,6,
  2,2,6,2,2,2,2,2, 2,2,2,2,6,6,6,2, 6,6,6,6,6,6,6,6, 6,6,4,6,6,39,39,37,
  // 8Exx
  56,39,39,39,39,39,39,39, 39,2,2,39,39,2,39,37, 39,39,39,37,37,37,39,39, 37,39,39,39,39,37,37,37,
  2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2,
  6,12,6,2,6,12,6,6, 2,12,2,12,2,6,12,6, 2,6,6,12,2,6,2,2, 6,6,6,2,6,6,6,6,
  6,12,6,6,6,12,6,6, 6,6,6,6,6,6,6,6, 6,6,31,6,6,6,6,6, 6,2,2,6,6,6,6,39,
  6,6,2,6,6,2,6,2, 2,2,6,2,6,6,6,6, 36,6,2,2,6,6,6,6, 2,6,6,6,6,6,6,6,
  6,6,2,7,7,2,6,2, 2,6,2,6,7,6,6,6, 2,7,2,7,7,6,7,2, 6,7,6,2,7,7,6,6,
  6,12,6,7,7,6,7,11, 7,6,6,6,7,12,6,6, 6,7,6,6,6,6,6,7, 6,7,7,7,7,7,7,7,
  2,6,2,2,2,2,2,2, 6,2,6,6,2,6,6,2, 6,6,6,6,6,6,6,6, 6,6,6,6,6,37,8,37,
  // 8Fxx
  57,39,39,37,39,39,39,39, 39,2,2,39,39,2,39,39, 39,39,39,39,37,37,39,39, 39,39,39,39,39,39,39,39,
  2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2,
  6,6,6,6,2,6,6,6, 6,6,6,6,6,6,12,6, 6,6,6,2,6,2,6,6, 6,6,6,6,6,6,2,6,
  6,6,6,6,6,6,6,2, 6,2,6,6,2,6,6,6, 6,6,6,2,6,6,6,2, 6,6,6,6,2,6,6,39,
  6,6,2,6,2,2,6,6, 6,6,6,2,2,2,36,6, 2,6,2,2,6,6,2,6, 2,2,2,2,6,6,6,6,
  2,2,6,2,6,2,2,6, 36,2,2,6,6,6,36,2, 2,2,2,6,2,6,2,2, 2,6,6,6,6,2,2,5,
  6,6,6,6,6,6,6,6, 6,6,6,6,6,6,6,6, 6,6,6,6,6,6,6,6, 6,2,6,6,6,6,6,6,
  2,6,2,2,2,2,2,2, 2,2,6,6,6,6,6,2, 6,6,6,6,6,6,6,6, 6,6,6,6,6,37,8,37,
  // 90xx
  57,37,37,37,39,39,37,39, 39,2,2,39,39,2,39,37, 39,39,37,39,37,39,39,39, 39,37,37,39,39,39,39,37,
  2,2,2,37,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2,
  6,6,6,2,6,6,6,6, 6,2,6,6,6,6,6,2, 6,6,6,6,6,6,6,6, 6,6,6,6,6,2,6,6,
  6,6,6,6,6,6,6,6, 6,6,6,6,6,6,6,6, 6,6,6,6,6,6,6,6, 6,6,6,6,6,6,6,37,
  36,6,6,2,2,6,2,6, 2,2,2,6,2,2,2,6, 2,2,36,36,6,2,2,2, 2,6,6,2,2,6,2,6,
  36,6,6,6,6,2,2,6, 6,6,6,6,6,6,6,6, 6,2,6,6,6,2,6,6, 6,6,6,6,6,2,2,6,
  6,6,6,6,6,6,6,11, 6,6,6,6,6,6,6,6, 6,6,11,6,6,6,6,2, 6,6,6,6,6,6,6,6,
  6,2,6,2,2,2,2,2, 2,2,6,2,2,6,6,2, 6,6,6,6,6,6,6,6, 6,6,6,6,6,37,39,37,
  // 91xx
  57,39,39,39,39,39,39,39, 39,2,2,39,39,2,39,39, 39,39,39,39,39,37,39,39, 39,39,39,39,39,39,37,39,
  2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2,
  6,2,6,2,6,6,6,2, 6,6,6,6,2,2,6,6, 6,6,6,6,6,6,6,11, 6,6,11,6,6,6,6,6,
  6,6,6,2,6,6,6,6, 6,2,6,6,2,6,6,6, 6,6,6,6,6,6,6,6, 6,6,6,6,6,6,2,39,
  6,6,2,6,2,36,2,6, 6,2,2,6,36,6,36,36, 6,6,36,36,6,6,6,6, 2,2,2,6,6,6,2,6,
  6,6,6,6,6,6,6,6, 2,2,6,6,6,6,6,36, 6,6,6,2,6,6,6,6, 6,6,6,2,6,6,6,6,
  10,6,2,6,6,6,2,6, 6,6,6,6,6,6,6,6, 2,6,6,6,6,6,6,6, 6,6,6,6,6,6,6,6,
  6,2,2,2,2,6,2,2, 2,2,2,2,2,2,6,2, 6,6,6,6,6,6,6,6, 6,6,6,6,6,22,22,37,
  // 92xx
  56,39,39,37,39,39,37,39, 39,2,2,39,39,2,39,39, 37,39,37,39,37,37,39,37, 37,39,39,39,37,39,37,39,
  2,2,2,2,2,2,2,2, 2,2,2,2,11,2,2,2, 39,2,2,2,2,2,2,2, 2,2,2,11,2,2,2,2,
  6,11,6,6,6,6,2,2, 11,6,6,6,2,6,6,6, 6,6,2,6,6,6,2,6, 2,6,6,6,2,6,6,6,
  6,11,6,6,11,11,6,6, 11,6,6,6,6,6,6,11, 6,6,6,11,11,6,6,6, 6,11,6,6,6,6,6,37,
  36,6,36,6,2,6,6,6, 2,6,6,6,2,6,36,2, 36,6,2,6,2,2,2,2, 6,2,6,36,36,6,2,2,
  6,6,2,6,6,6,6,6, 2,6,6,2,6,2,6,2, 2,2,6,2,6,6,6,6, 6,6,6,2,6,2,6,6,
  6,6,6,6,6,6,2,6, 6,6,6,6,6,6,6,6, 6,2,6,6,6,6,6,6, 6,6,6,6,6,6,6,6,
  6,6,6,2,2,2,2,2, 6,2,6,6,2,6,11,2, 6,6,6,6,6,6,6,6, 6,6,6,6,6,22,10,10,
  // 93xx
  56,39,39,39,37,39,39,39, 39,2,2,39,39,2,39,39, 39,39,39,39,39,39,39,37, 37,39,39,39,39,39,37,39,
  2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 37,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2,
  6,11,11,11,11,6,11,6, 11,6,6,6,11,11,6,6, 11,11,11,6,6,11,6,6, 6,6,6,2,6,6,6,6,
  6,6,2,6,6,11,6,11, 6,2,11,6,6,2,2,6, 11,6,6,6,11,2,11,6, 6,6,6,6,6,6,6,37,
  6,2,6,6,2,2,36,6, 2,2,6,6,6,2,2,6, 6,36,6,2,6,6,6,36, 36,6,6,6,2,6,6,6,
  2,2,6,2,2,2,2,6, 2,6,6,6,6,6,6,6, 6,6,2,5,6,6,6,2, 2,6,2,6,2,6,6,6,
  10,6,6,59,6,6,6,6, 6,12,6,6,10,6,2,6, 6,2,6,10,6,6,6,2, 6,2,6,6,6,6,6,6,
  6,2,2,2,2,2,2,2, 2,2,2,2,6,6,6,2, 6,6,6,6,19,6,6,6, 6,6,6,6,6,10,19,37,
  // 94xx
  56,39,39,39,39,39,39,39, 39,2,2,11,39,2,39,39, 39,39,39,39,39,39,39,37, 37,39,39,39,37,39,39,39,
  2,2,2,11,2,2,2,2, 2,2,2,2,11,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,11,2,
  6,6,2,6,6,6,6,6, 2,2,6,6,6,6,6,6, 2,2,6,6,6,2,6,2, 6,6,6,6,6,6,2,6,
  6,2,2,6,6,6,2,6, 6,2,6,2,2,2,6,6, 6,6,6,6,6,2,6,6, 6,6,6,6,6,6,6,39,
  2,2,2,6,6,2,2,2, 2,2,2,2,2,6,6,6, 2,6,6,6,2,6,6,6, 2,2,6,6,2,2,6,2,
  6,2,2,2,2,36,6,6, 2,2,6,6,2,6,2,2, 2,2,6,2,2,2,2,2, 2,2,2,2,6,6,2,2,
  6,6,6,2,6,6,6,6, 6,6,6,6,6,6,2,6, 2,6,6,6,6,6,6,2, 6,2,6,6,6,6,6,6,
  2,2,2,2,2,2,2,2, 2,2,2,2,2,2,6,6, 6,6,6,6,6,6,6,6, 6,6,6,6,6,22,37,37,
  // 95xx
  56,39,39,39,39,37,39,39, 37,11,2,39,39,11,39,39, 37,39,39,39,39,39,39,39, 39,37,39,39,39,39,39,39,
  11,2,2,39,11,2,11,2, 2,2,2,2,2,2,11,2, 2,2,2,2,2,2,2,2, 2,2,11,2,2,2,2,2,
  6,2,2,6,6,6,6,6, 6,6,2,6,11,6,11,6, 6,6,6,6,11,6,11,6, 6,6,6,6,6,6,6,6,
  6,6,6,6,6,6,6,6, 6,6,6,6,6,6,6,6, 6,6,6,6,6,6,6,6, 6,6,6,6,6,6,6,37,
  2,6,6,6,2,2,2,2, 2,6,2,6,2,6,6,2, 6,6,6,2,6,6,2,6, 2,2,6,6,2,6,6,6,
  6,6,2,2,2,6,2,6, 6,2,6,2,2,2,2,2, 2,6,6,6,2,6,6,6, 2,6,6,6,2,6,6,2,
  6,6,6,6,6,6,6,6, 6,6,6,6,6,39,6,6, 6,6,6,6,6,6,6,6, 6,6,6,6,6,6,6,6,
  2,2,6,2,2,2,2,2, 2,2,6,2,2,2,6,2, 6,6,6,6,6,6,6,6, 6,6,6,6,6,37,19,37,
  // 96xx
  57,39,39,39,39,37,39,39, 39,2,2,39,39,11,39,39, 37,39,39,39,39,39,39,39, 39,39,39,39,39,39,39,39,
  11,2,2,2,11,11,2,2, 2,2,2,2,2,2,2,2, 11,11,11,2,11,2,11,11, 11,11,2,2,2,2,11,2,
  6,6,6,6,6,6,6,6, 6,6,6,6,6,6,2,6, 6,6,6,6,6,6,6,6, 6,6,6,6,6,6,6,6,
  6,6,6,6,6,6,6,6, 6,6,6,6,6,11,6,6, 6,6,6,11,11,6,6,6, 6,6,6,6,6,6,6,39,
  2,6,6,6,2,6,2,2, 6,2,2,2,6,6,2,2, 2,2,2,2,6,6,6,2, 6,2,2,6,6,6,6,6,
  2,6,6,6,2,2,2,6, 6,2,2,2,2,2,2,6, 2,6,2,6,2,2,2,2, 2,2,6,2,6,2,6,6,
  6,6,6,6,6,6,6,11, 6,6,6,6,6,6,6,6, 2,6,6,6,6,6,6,6, 6,6,6,6,6,6,6,6,
  2,2,6,2,2,2,2,2, 2,2,2,6,6,6,6,2, 6,6,6,6,6,6,6,6, 6,6,17,6,6,39,37,37,
  // 97xx
  56,39,39,39,39,39,39,37, 39,2,2,39,39,2,39,39, 39,39,39,39,39,39,39,39, 39,39,39,39,37,39,37,39,
  2,2,2,11,11,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,11,2,
  6,6,6,6,6,6,6,6, 6,6,6,6,6,6,6,6, 6,2,6,6,6,6,6,6, 6,6,6,6,6,6,6,6,
  6,6,11,6,6,6,6,6, 6,2,6,6,6,6,6,11, 6,6,6,11,6,6,6,6, 6,6,6,6,2,6,6,39,
  36,6,6,6,2,2,2,2, 6,2,6,2,2,6,6,2, 2,6,6,6,2,2,6,6, 6,6,6,2,2,6,6,6,
  2,2,6,6,6,2,6,6, 2,2,6,6,6,2,2,6, 2,6,6,6,2,6,2,6, 2,6,2,2,6,2,6,6,
  6,6,6,6,6,6,6,6, 6,6,6,6,6,6,6,6, 2,6,6,6,6,6,6,6, 6,6,6,6,6,6,6,6,
  2,6,6,2,2,2,2,2, 2,2,6,6,6,6,6,2, 6,6,6,6,6,6,6,6, 6,6,6,6,6,39,39,37,
  // 98xx
  57,37,37,37,39,37,37,39, 37,2,2,39,39,2,39,39, 37,37,37,37,39,39,39,39, 37,39,39,39,39,39,39,39,
  2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2,
  6,6,6,6,2,2,6,2, 6,6,6,2,6,2,6,6, 2,6,6,2,2,6,6,2, 6,6,6,6,6,2,6,6,
  6,6,6,2,2,6,6,6, 6,2,6,2,6,2,2,2, 6,6,6,2,2,2,2,2, 2,2,2,2,2,2,2,39,
  2,2,2,2,2,2,2,2, 2,2,36,2,2,2,2,2, 2,2,36,2,2,2,2,2, 2,5,2,2,2,11,2,2,
  2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,5,2,2,
  11,39,2,2,2,6,39,11, 6,11,11,19,6,39,2,11, 2,11,6,11,11,39,6,2, 2,2,2,2,11,11,5,37,
  2,2,2,2,2,2,2,2, 2,2,2,2,2,2,5,2, 6,6,37,6,6,4,39,6, 6,5,6,39,6,37,37,37,
  // 99xx
  56,39,39,37,39,37,39,39, 39,2,2,39,39,2,39,39, 37,39,39,37,39,39,39,39, 37,37,39,39,39,39,37,39,
  2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2,
  2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,6,2,2,2,11,2,
  2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,6,6,2,2,2,37,
  2,2,2,2,2,2,2,2, 2,2,6,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,37,2,2,2,2,2,2,
  2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,5,2,2,2,2,2, 2,2,2,2,2,2,2,5,
  6,6,2,2,2,2,2,11, 6,6,6,2,6,6,2,2, 2,2,6,6,6,6,6,2, 2,2,19,2,11,6,11,6,
  2,2,2,2,2,2,2,2, 2,2,2,2,2,2,5,2, 6,36,6,6,6,6,6,6, 6,6,6,6,6,39,4,37,
  // 9Axx
  56,39,39,39,39,39,39,39, 39,2,2,39,39,2,37,39, 39,39,37,37,39,39,39,39, 39,37,39,37,39,39,37,39,
  2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2,
  2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2,
  2,12,2,2,2,12,2,2, 2,12,2,12,12,2,12,12, 12,6,2,2,12,12,12,2, 2,2,2,2,2,2,2,37,
  2,2,2,5,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,6,2,2,2,2,2, 2,2,12,2,2,12,6,36,
  6,6,2,2,2,5,2,2, 2,2,2,2,6,6,2,5, 2,2,5,5,2,2,2,2, 2,2,2,2,2,2,2,2,
  5,6,59,2,2,2,6,12, 6,2,6,2,4,6,2,2, 2,2,6,6,37,39,6,2, 37,2,6,6,6,19,6,6,
  2,2,2,2,2,2,2,2, 2,2,2,5,2,12,6,2, 36,6,12,10,12,6,6,6, 6,12,6,6,12,12,39,37,
  // 9Bxx
  56,39,39,39,39,39,37,39, 39,2,2,39,39,2,39,39, 39,39,39,37,39,39,39,39, 39,39,39,39,39,39,39,39,
  2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2,
  2,2,2,2,2,2,2,2, 2,6,2,2,2,2,2,2, 2,2,39,2,2,2,2,2, 5,2,6,2,2,2,2,2,
  2,2,2,2,2,2,2,6, 2,2,6,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,6,6,2,2,2,37,
  2,2,36,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,6,2,36,2,2, 2,2,2,2,2,2,2,2,
  2,2,2,2,2,5,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2,
  6,6,2,2,6,2,6,2, 4,6,6,2,2,12,2,2, 2,2,6,6,6,39,2,2, 2,2,6,6,6,6,6,6,
  2,2,2,2,2,2,2,2, 2,2,2,2,2,2,6,2, 6,6,6,6,6,6,6,11, 4,6,6,6,11,11,22,37,
  // 9Cxx
  56,39,39,11,37,37,39,39, 39,2,2,39,39,2,39,39, 37,37,39,37,39,37,39,39, 39,39,39,39,39,39,39,37,
  2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2,
  2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,6,2,2,2,
  2,2,2,12,2,2,2,2, 2,2,2,2,12,2,2,2, 2,2,12,2,2,11,2,12, 2,2,2,2,2,2,2,37,
  2,2,2,2,2,36,2,2, 2,2,2,2,2,2,2,2, 2,36,2,2,2,2,2,2, 2,2,2,2,2,2,2,2,
  2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2,
  6,4,2,2,2,2,6,2, 6,6,19,2,6,4,2,2, 2,2,2,2,6,2,2,2, 2,2,2,6,6,6,6,6,
  2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 6,12,6,10,37,4,6,6, 6,6,6,6,11,4,19,37,
  // 9Dxx
  56,39,12,4,37,39,37,39, 37,2,2,39,39,2,39,39, 37,39,37,39,37,37,39,37, 37,37,37,39,37,37,39,39,
  2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2,
  2,2,2,2,2,2,2,6, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 6,2,2,2,2,2,2,2,
  2,12,2,2,2,2,6,2, 2,2,2,2,2,2,2,12, 2,2,2,2,12,12,2,2, 2,2,2,2,6,2,2,37,
  2,2,2,2,2,5,2,2, 2,2,2,36,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2,
  2,2,2,2,2,2,2,5, 2,2,2,36,2,2,2,2, 2,2,5,2,2,2,2,2, 2,2,6,2,2,2,2,2,
  5,6,2,2,2,2,11,11, 11,6,11,2,11,11,11,11, 2,11,11,11,11,11,4,6, 2,2,11,6,6,11,11,11,
  2,2,2,2,2,2,2,2, 2,2,2,2,2,2,19,2, 6,19,6,6,6,6,6,19, 6,6,12,10,4,19,11,37,
  // 9Exx
  56,39,39,39,37,39,39,39, 39,2,2,39,39,2,39,39, 39,39,39,39,39,39,39,39, 39,39,39,39,39,39,37,37,
  12,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2,
  2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2,
  2,12,12,2,12,12,2,2, 2,12,2,12,2,2,12,12, 2,2,2,12,2,12,2,2, 2,2,2,2,2,2,2,37,
  2,2,2,2,2,2,6,2, 2,2,6,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,11,2,
  2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,5,6,2,2,2,2,2, 2,6,2,2,2,6,2,2,
  6,12,59,2,6,2,2,2, 6,6,6,6,2,5,2,6, 2,2,6,11,6,6,6,2, 11,6,6,2,6,19,37,6,
  2,2,2,2,2,2,2,2, 2,2,2,2,2,12,5,2, 12,4,12,10,6,19,39,6, 6,12,12,6,6,12,11,37,
  // 9Fxx
  56,39,39,39,4,39,39,39, 39,2,2,39,39,2,37,39, 39,39,39,37,39,37,39,39, 37,39,39,39,39,39,39,39,
  2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2,
  2,2,2,2,2,2,2,2, 2,2,2,2,2,2,6,6, 2,2,2,2,6,2,2,2, 2,2,2,2,2,2,2,2,
  2,33,2,31,12,33,2,31, 31,2,2,33,33,2,2,2, 33,6,31,2,33,33,2,12, 2,2,2,6,2,2,2,37,
  2,2,36,2,2,36,6,36, 2,2,36,2,36,36,2,2, 2,2,2,2,2,6,36,2, 2,36,2,2,2,2,36,2,
  2,33,2,2,2,2,2,2, 6,2,36,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2,
  6,4,2,2,6,2,6,6, 6,6,6,2,6,6,2,2, 2,2,1,6,6,6,4,2, 2,2,6,6,4,6,6,6,
  2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 6,12,6,6,19,6,6,6, 6,6,6,6,6,19,4,37,
  // A0xx
  7,11,37,11,37,1,37,1, 1,11,11,37,1,11,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  11,2,2,11,11,11,2,2, 11,2,2,11,2,11,2,2, 11,11,11,11,11,11,11,11, 11,11,11,11,11,11,11,2,
  2,11,11,11,11,11,11,11, 11,11,11,11,11,11,11,11, 11,11,11,11,11,11,11,11, 11,11,11,11,11,2,2,2,
  2,11,11,11,11,11,11,11, 11,11,11,11,33,11,33,11, 11,11,11,11,11,11,11,11, 11,11,11,11,11,2,2,37,
  2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,11,2,2,12,2,
  11,2,36,36,2,2,36,36, 2,36,2,36,36,2,36,2, 2,2,2,5,2,2,2,2, 2,2,2,2,2,2,2,12,
  10,10,2,2,10,11,10,11, 11,1,10,14,10,10,10,10, 2,11,11,11,11,11,1,2, 2,2,11,11,1,10,11,11,
  2,2,2,2,2,2,2,2, 2,2,2,2,2,2,36,2, 10,10,10,10,10,10,10,10, 12,17,1,10,11,33,1,10,
  // A1xx
  7,37,37,1,37,37,37,37, 37,2,2,37,37,2,37,37, 37,37,37,37,37,37,37,37, 37,37,37,3,37,37,37,37,
  11,2,2,2,2,2,11,2, 2,2,2,2,2,2,14,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,14,2,
  8,8,8,8,9,8,8,8, 8,8,8,8,8,8,8,8, 8,8,8,9,9,8,8,8, 8,1,9,2,8,8,8,2,
  8,8,33,2,33,8,8,1, 33,8,8,33,33,8,8,2, 2,8,8,33,2,8,8,8, 8,8,8,39,2,8,2,37,
  2,2,2,2,2,2,2,2, 2,2,2,39,2,2,2,2, 2,2,2,2,2,2,6,2, 2,2,2,2,2,2,2,33,
  2,3,3,3,3,7,7,7, 2,7,7,3,2,3,3,3, 3,3,7,8,8,2,3,3, 8,7,3,3,7,7,3,3,
  3,7,7,7,7,14,9,14, 7,7,7,7,7,14,7,7, 8,14,14,14,14,14,7,7, 7,7,7,7,7,7,5,7,
  2,5,5,2,2,2,2,2, 2,2,2,7,2,2,3,3, 3,3,3,7,3,7,7,7, 3,7,7,7,7,7,7,37,
  // A2xx
  7,37,37,1,37,37,37,37, 37,2,2,37,37,2,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 37,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2,
  8,8,2,2,8,2,2,8, 8,8,8,2,2,2,2,2, 2,2,2,2,2,2,2,2, 8,2,2,2,2,2,2,2,
  2,2,8,8,33,8,8,2, 8,8,2,8,2,2,2,2, 8,8,8,8,8,8,8,8, 8,8,8,8,8,8,8,37,
  2,2,6,2,2,2,2,2, 2,6,2,2,2,2,2,6, 6,2,2,2,6,6,2,2, 2,2,2,2,2,2,2,2,
  36,7,7,7,7,7,7,7, 7,7,7,2,7,7,36,8, 2,8,8,8,8,8,8,14, 5,5,5,14,2,5,5,5,
  5,5,5,5,5,14,5,14, 5,5,14,23,5,14,5,5, 8,5,14,8,8,14,14,8, 14,3,3,3,3,5,8,5,
  2,2,2,2,2,2,2,2, 2,2,8,36,8,8,8,2, 8,3,3,3,8,8,7,8, 8,3,8,8,8,8,8,37,
  // A3xx
  7,37,37,39,37,37,37,37, 37,2,2,37,4,2,9,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  2,2,2,2,11,11,2,2, 2,2,2,2,2,2,2,2, 11,11,11,11,11,11,11,11, 11,11,2,2,2,2,2,2,
  36,12,12,12,12,12,2,12, 2,2,2,12,2,12,12,12, 12,2,2,12,12,12,11,12, 8,12,12,2,8,2,11,2,
  8,12,2,2,33,12,2,8, 11,2,8,12,2,2,2,2, 2,11,2,8,2,12,8,2, 8,2,8,8,8,8,8,37,
  2,2,2,6,2,2,2,2, 3,2,2,2,2,2,2,2, 2,2,2,2,3,2,5,2, 2,2,2,2,2,2,2,2,
  3,3,3,3,3,3,3,3, 3,3,3,2,3,3,3,3, 3,3,3,3,3,3,3,3, 3,3,3,3,2,2,2,3,
  10,3,3,3,3,3,3,3, 3,3,3,3,3,3,3,3, 3,3,3,3,3,3,3,3, 3,3,3,3,5,3,3,3,
  2,2,2,2,2,2,2,2, 2,2,2,3,3,3,3,2, 3,3,3,12,3,3,3,3, 3,3,3,3,3,3,3,37,
  // A4xx
  7,37,37,11,1,37,11,39, 11,2,2,37,37,2,37,37, 37,37,37,37,37,37,37,37, 37,37,7,37,37,37,37,37,
  2,2,2,11,2,11,2,2, 2,2,11,2,2,2,2,2, 37,2,11,2,2,2,2,17, 2,17,2,2,2,2,2,2,
  8,8,8,8,8,8,8,8, 8,2,8,8,8,8,2,8, 2,8,8,8,8,8,8,8, 8,8,8,8,8,8,2,8,
  8,8,8,11,8,11,8,8, 8,8,8,8,8,2,2,8, 8,8,8,8,8,8,8,8, 8,8,8,2,2,8,8,39,
  2,2,2,2,2,2,2,2, 2,2,2,6,2,2,2,2, 2,2,2,6,6,2,2,2, 2,2,2,2,2,2,2,2,
  36,3,3,8,3,36,3,2, 3,2,3,3,3,3,3,3, 3,3,3,3,3,3,8,3, 3,3,8,3,3,3,2,3,
  3,3,14,3,3,14,3,3, 3,3,3,3,3,3,3,3, 3,8,3,3,14,3,8,3, 3,3,8,3,3,3,3,3,
  3,3,3,2,3,8,2,2, 3,3,3,3,3,3,8,3, 8,8,3,3,8,8,8,8, 11,8,8,8,8,5,8,1,
  // A5xx
  7,6,37,39,37,37,6,37, 1,2,2,37,1,2,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  2,2,2,2,2,2,2,11, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2,
  8,9,8,12,8,2,8,8, 8,8,8,8,8,2,8,8, 8,2,8,8,12,8,8,2, 8,8,8,8,8,8,8,8,
  8,8,8,2,8,8,8,2, 8,8,8,8,8,2,2,2, 2,8,2,8,2,8,8,2, 8,2,8,8,8,8,8,37,
  2,2,2,6,2,2,2,2, 2,2,2,2,2,2,6,2, 2,2,6,6,2,2,2,2, 2,2,2,2,2,2,2,2,
  2,3,3,3,3,6,3,3, 3,3,3,3,3,3,3,3, 3,3,3,3,3,3,3,3, 3,3,3,8,2,2,3,3,
  3,3,2,3,3,8,3,3, 3,3,3,3,3,8,8,3, 3,3,3,3,3,3,3,3, 8,3,3,3,3,3,3,3,
  3,3,3,2,2,2,2,2, 2,3,3,3,3,3,8,3, 8,8,8,3,8,8,3,8, 8,8,8,8,8,8,8,37,
  // A6xx
  7,37,37,39,37,37,37,1, 12,2,2,37,37,2,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  2,2,2,2,2,2,2,2, 2,2,2,2,2,6,2,2, 37,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2,
  8,8,8,9,8,33,2,2, 8,33,8,9,9,9,9,8, 8,8,9,8,8,8,8,8, 8,8,8,2,2,8,8,8,
  8,8,8,9,2,2,2,2, 8,8,8,8,9,9,8,8, 8,2,8,8,8,8,8,8, 8,8,8,8,8,8,8,37,
  2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2,
  36,8,5,8,2,36,5,2, 8,8,2,8,2,8,2,2, 2,8,2,8,2,8,8,8, 8,8,8,2,2,2,2,8,
  8,3,3,8,8,3,9,8, 8,14,8,8,8,3,2,8, 8,8,8,8,3,8,33,8, 3,8,8,8,8,8,8,8,
  2,2,8,2,2,2,8,2, 2,2,8,8,8,8,8,2, 3,8,8,8,8,8,8,8, 8,8,8,8,8,8,8,37,
  // A7xx
  7,1,1,37,37,37,37,37, 37,14,2,14,37,14,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  2,2,2,2,11,2,2,14, 2,2,11,2,2,2,2,2, 11,11,11,11,11,11,11,11, 11,11,2,2,2,2,11,2,
  8,8,8,8,8,8,2,8, 8,8,8,8,8,8,8,8, 8,8,8,2,8,8,8,2, 8,8,8,11,2,11,8,8,
  8,2,8,8,8,8,8,8, 8,8,8,8,8,2,2,8, 8,8,8,8,8,2,8,8, 8,2,2,8,8,8,2,37,
  2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,5, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2,
  36,14,14,2,14,3,2,11, 8,8,14,2,2,36,8,2, 2,2,8,5,14,14,14,14, 2,8,8,2,14,8,14,2,
  8,14,14,14,14,14,8,8, 14,8,14,14,8,14,8,8, 8,8,14,8,14,3,8,14, 2,2,8,2,8,8,8,8,
  2,14,8,2,2,2,2,2, 2,2,2,8,8,8,3,2, 8,3,8,8,8,8,8,8, 8,8,8,8,8,8,8,37,
  // A8xx
  7,11,1,39,1,37,37,37, 4,2,2,14,37,2,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  2,2,2,2,2,2,2,2, 2,2,11,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2,
  8,2,8,8,8,2,8,2, 8,8,2,8,8,8,8,3, 2,2,8,8,8,8,2,2, 2,2,8,2,2,2,8,2,
  11,8,8,8,8,8,8,8, 2,2,8,8,8,8,8,8, 8,8,3,8,8,8,8,3, 8,8,8,8,8,8,8,37,
  2,2,2,2,2,2,2,2, 2,5,2,2,2,2,2,3, 3,3,3,2,2,2,2,2, 2,2,2,2,2,2,2,2,
  2,2,7,8,8,8,7,7, 14,8,2,8,7,8,8,8, 11,2,7,2,2,8,2,14, 2,2,8,2,2,8,8,2,
  8,14,2,8,8,8,8,8, 8,8,8,14,8,14,8,8, 14,8,14,8,14,5,8,8, 14,2,8,5,5,6,5,8,
  2,2,8,2,2,2,2,2, 2,2,2,2,8,2,8,2, 5,5,8,8,8,8,5,8, 8,5,8,8,8,8,8,37,
  // A9xx
  7,1,37,39,37,37,39,37, 37,2,2,37,37,2,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  11,2,2,2,11,2,11,2, 2,2,2,11,2,2,2,2, 37,11,11,2,2,2,2,2, 2,2,2,2,2,2,2,2,
  8,9,8,8,9,9,11,8, 8,8,8,9,9,8,9,8, 8,8,8,8,8,8,9,8, 2,8,8,8,2,8,8,8,
  8,2,8,2,8,8,8,8, 8,8,8,9,8,8,2,3, 8,2,2,8,2,8,8,8, 8,8,8,8,8,8,8,39,
  2,3,6,3,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,6,2,2,3,2, 2,2,2,2,2,2,2,2,
  2,8,2,8,3,3,3,3, 8,2,3,8,2,3,3,8, 2,8,8,8,2,8,2,2, 3,8,2,2,8,3,8,2,
  3,3,2,2,3,14,8,8, 8,8,8,2,3,9,8,3, 8,14,8,8,8,14,8,2, 2,2,8,8,8,8,8,8,
  2,2,8,2,2,2,2,2, 2,2,2,2,2,2,8,2, 8,8,8,8,8,8,8,8, 8,8,8,8,8,8,8,37,
  // AAxx
  7,37,37,37,37,1,37,37, 4,2,2,1,37,2,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 37,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2,
  8,8,8,8,8,8,8,8, 2,12,2,8,8,8,2,8, 8,8,8,2,12,2,2,2, 2,8,8,8,2,2,8,8,
  8,8,2,8,8,8,8,8, 8,8,8,8,8,8,8,8, 8,8,8,2,2,8,8,8, 8,8,8,8,2,2,2,37,
  2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,5,2,2,2,2,2,2,
  36,14,8,8,2,3,8,8, 2,8,2,8,8,2,36,8, 2,8,2,5,8,5,2,5, 14,14,8,2,8,8,8,2,
  8,14,14,14,10,8,5,14, 8,1,5,5,8,14,8,8, 14,8,14,8,14,14,8,14, 14,2,8,2,8,8,8,5,
  2,8,2,2,2,2,2,2, 2,2,8,5,8,8,8,2, 8,8,8,5,8,10,8,8, 8,8,8,8,8,8,8,37,
  // ABxx
  7,1,1,37,37,37,37,37, 37,2,2,37,37,2,37,37, 37,37,37,37,6,37,37,37, 37,37,37,37,37,37,37,37,
  2,2,2,2,2,2,11,2, 2,2,2,2,2,2,2,2, 37,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2,
  2,11,8,8,8,11,8,8, 8,8,8,8,8,11,8,8, 8,8,2,11,8,8,11,8, 8,8,2,2,8,2,8,8,
  8,8,8,11,8,8,11,8, 8,8,8,8,8,2,8,8, 8,8,8,8,8,2,8,8, 8,8,8,8,8,8,8,37,
  2,2,2,2,2,2,5,2, 2,2,2,2,2,2,39,2, 5,2,2,2,2,11,2,2, 2,2,5,2,2,5,2,2,
  2,8,8,8,5,2,2,8, 36,2,2,8,8,2,36,2, 8,8,3,8,8,8,2,5, 2,2,8,2,8,8,2,2,
  8,8,8,8,8,8,5,8, 8,8,8,8,8,14,8,11, 14,8,8,8,14,14,14,8, 8,14,11,8,8,8,11,8,
  2,8,2,2,2,2,2,2, 2,2,2,2,36,8,36,2, 20,8,8,8,8,8,8,8, 8,8,8,8,8,8,8,37,
  // ACxx
  7,37,37,39,37,39,39,37, 37,2,2,37,39,2,37,39, 37,37,37,37,37,39,37,37, 39,37,37,39,37,37,37,37,
  2,2,2,11,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2,
  8,8,8,9,8,33,8,8, 2,8,8,8,9,8,9,8, 8,8,9,2,2,8,8,9, 8,8,8,8,8,2,2,8,
  8,8,2,8,8,33,8,11, 8,2,2,2,2,8,8,8, 8,8,8,8,8,8,8,8, 8,8,8,2,8,8,8,37,
  2,6,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,6,5,2,2,2,2, 2,2,2,2,2,2,2,2,
  36,8,2,8,2,36,2,2, 36,6,8,36,2,36,36,8, 8,2,2,8,2,8,8,3, 2,8,2,2,2,8,2,3,
  8,8,59,8,8,59,8,3, 8,8,3,59,8,8,3,2, 8,5,8,8,8,8,8,8, 2,8,5,8,8,8,8,8,
  2,2,59,2,2,2,2,2, 2,2,2,2,2,2,8,2, 8,8,8,8,8,8,8,8, 8,8,8,8,8,8,8,37,
  // ADxx
  7,39,37,37,39,39,37,39, 39,2,2,37,39,2,37,39, 39,37,39,39,39,39,39,37, 39,39,37,39,39,39,37,37,
  2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 37,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2,
  8,8,2,8,8,2,8,8, 2,8,8,2,8,8,2,2, 8,2,2,8,8,8,8,8, 8,8,8,8,2,8,8,2,
  8,2,8,2,11,8,8,11, 8,2,11,11,8,8,8,11, 8,8,11,8,2,8,11,8, 8,2,8,8,2,8,2,37,
  2,2,2,2,2,2,2,2, 2,2,39,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,5,2,
  36,2,2,2,2,2,2,8, 36,2,14,8,2,8,36,2, 2,8,8,2,2,8,8,8, 2,8,8,8,8,2,2,8,
  8,14,8,14,8,8,8,8, 8,8,8,8,8,8,2,2, 7,8,14,8,8,14,7,8, 2,8,8,8,8,8,8,8,
  2,8,2,2,2,2,2,2, 2,2,2,2,8,2,36,2, 8,8,8,8,8,8,8,8, 8,8,8,8,8,8,8,37,
  // AExx
  7,39,1,39,39,39,37,37, 37,2,2,37,39,2,37,39, 39,39,39,39,39,39,37,37, 39,39,37,39,39,39,37,37,
  2,2,2,2,2,2,2,2, 2,2,2,5,2,2,2,2, 37,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2,
  8,9,8,2,2,9,2,2, 8,9,8,9,8,8,9,9, 2,8,2,8,8,2,8,2, 2,2,8,2,2,8,2,8,
  8,8,8,8,8,8,2,8, 8,8,2,8,8,8,2,8, 8,8,8,2,8,8,8,8, 8,8,8,8,8,8,11,37,
  2,2,6,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2,
  2,2,2,2,2,36,8,8, 2,2,2,2,36,2,2,2, 2,2,2,2,2,2,2,8, 2,2,2,8,8,2,2,2,
  8,8,8,14,8,6,8,8, 8,8,8,14,8,9,2,2, 8,8,8,8,8,8,8,8, 8,2,8,8,8,6,8,8,
  2,36,36,2,2,2,2,2, 2,2,2,2,2,8,8,2, 8,8,8,8,8,8,8,8, 8,8,8,8,8,8,8,37,
  // AFxx
  7,9,1,1,39,39,39,37, 37,2,2,37,39,2,37,37, 37,37,37,37,37,39,37,37, 39,39,37,37,39,37,37,37,
  2,2,2,2,2,2,2,2, 2,11,2,2,2,2,2,2, 39,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2,
  2,12,8,2,12,12,8,2, 2,8,2,12,12,8,12,12, 8,8,8,8,8,2,2,2, 2,12,8,8,2,8,2,2,
  11,2,8,2,8,8,8,8, 2,2,2,8,8,2,2,12, 2,8,2,2,2,8,8,8, 8,12,8,8,2,8,2,37,
  2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2,
  36,9,2,6,2,2,2,2, 2,2,8,8,8,2,36,2, 11,2,8,8,2,8,2,2, 8,2,8,2,2,8,2,2,
  8,8,8,8,6,8,8,8, 8,8,8,6,6,10,8,8, 8,8,8,8,14,10,1,8, 2,2,2,2,8,8,8,8,
  8,2,2,2,2,2,2,2, 2,2,2,8,2,2,8,2, 8,8,8,8,8,8,8,8, 8,8,8,8,8,8,8,37,
  // B0xx
  7,39,37,37,39,4,37,37, 39,11,2,37,37,2,37,37, 39,39,37,37,39,39,37,37, 39,39,37,39,39,39,37,37,
  2,2,2,2,11,2,2,2, 2,2,11,2,2,2,2,2, 11,11,11,11,11,11,11,11, 11,11,2,2,2,2,2,2,
  8,8,2,11,8,11,11,2, 2,8,8,8,2,2,2,8, 2,8,2,2,8,8,8,2, 2,2,8,2,8,8,8,8,
  11,8,2,2,8,8,8,8, 8,2,8,8,8,11,2,11, 8,8,2,8,8,8,8,8, 8,8,2,8,8,8,11,37,
  2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2,
  11,5,5,5,2,5,7,2, 5,5,8,8,3,5,3,2, 5,2,3,5,3,8,5,8, 3,5,3,8,8,2,2,2,
  6,3,7,2,3,5,7,5, 8,3,8,5,7,5,3,8, 2,3,14,8,5,7,3,3, 3,3,5,7,5,5,7,5,
  2,5,8,2,2,2,5,2, 5,2,8,2,3,5,5,3, 5,5,8,8,3,8,8,5, 5,3,5,7,5,5,3,37,
  // B1xx
  7,11,11,39,39,37,39,37, 37,2,2,37,37,9,39,39, 37,39,37,37,39,39,37,37, 37,39,37,37,39,39,37,37,
  9,9,2,2,37,2,2,2, 2,2,2,5,9,2,9,2, 2,2,2,2,2,2,2,2, 2,2,9,9,2,2,2,9,
  8,2,8,8,2,8,8,8, 8,8,8,8,8,8,8,8, 2,2,11,2,8,8,8,8, 8,8,8,8,2,2,8,8,
  8,8,8,9,9,8,8,9, 8,8,8,9,8,8,2,8, 9,8,8,9,9,2,8,8, 8,8,8,8,2,8,2,37,
  2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,5,2, 2,2,2,2,2,2,2,2,
  2,8,7,3,5,3,3,3, 3,3,2,7,3,3,3,3, 2,3,3,5,3,8,3,8, 5,5,5,3,5,2,3,7,
  8,5,8,5,7,5,8,5, 3,7,3,3,3,5,3,3, 8,7,3,8,5,3,5,5, 3,5,8,5,3,5,5,7,
  3,8,5,2,3,2,2,2, 5,2,3,2,2,3,5,2, 3,8,5,7,5,3,3,7, 8,3,5,7,7,8,7,37,
  // B2xx
  7,39,37,39,37,37,39,1, 37,2,2,39,37,2,39,39, 39,39,37,37,39,39,37,39, 39,39,37,37,39,39,37,39,
  2,2,2,2,2,39,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2,
  8,2,8,2,8,8,2,8, 8,2,8,8,8,8,8,8, 8,8,8,2,2,8,8,8, 8,8,8,8,8,8,8,8,
  8,8,8,2,8,2,2,8, 2,8,8,8,8,8,8,8, 8,8,8,2,3,2,8,8, 8,8,8,8,2,39,8,37,
  2,2,2,2,2,2,5,5, 2,2,5,2,2,2,5,5, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2,
  2,3,3,8,8,3,7,8, 3,3,3,7,7,7,3,7, 7,8,5,8,2,3,3,2, 2,3,3,3,3,3,2,3,
  5,7,3,7,8,3,3,7, 7,3,3,3,7,3,3,7, 7,10,3,8,7,8,7,8, 3,3,7,7,3,3,5,3,
  2,3,3,2,2,2,2,2, 7,3,7,5,3,10,3,2, 7,7,7,7,5,7,5,7, 8,7,3,5,8,3,7,37,
  // B3xx
  7,39,1,39,39,37,37,37, 39,12,2,37,39,2,37,39, 39,39,37,37,39,39,39,37, 39,39,37,39,39,39,37,37,
  12,12,2,2,11,39,2,2, 2,2,2,2,12,2,12,12, 37,2,2,2,2,2,2,2, 2,2,12,12,2,2,2,12,
  8,2,8,8,8,2,2,2, 2,2,8,2,2,2,8,12, 2,8,2,8,8,8,8,8, 8,8,8,2,8,8,8,2,
  2,12,12,12,8,12,8,8, 8,2,8,12,2,12,8,8, 8,8,8,8,8,12,8,8, 8,12,8,8,2,8,8,37,
  2,2,6,2,2,2,2,2, 2,2,2,2,2,2,2,6, 2,2,2,2,2,2,2,2, 5,2,2,2,2,2,2,2,
  2,3,2,3,3,3,7,3, 2,3,5,7,3,5,8,5, 7,9,5,5,2,3,2,3, 2,8,8,5,2,8,3,3,
  7,3,3,5,5,3,3,3, 7,3,5,3,3,7,7,3, 3,5,8,7,3,8,3,5, 7,3,7,7,8,5,3,8,
  5,8,5,2,2,2,2,2, 2,2,3,5,5,5,5,2, 5,10,8,5,7,8,3,3, 8,8,5,3,8,3,3,10,
  // B4xx
  7,39,1,39,39,37,37,4, 39,14,2,14,39,2,37,39, 37,39,37,37,39,39,37,37, 37,39,37,39,37,39,39,37,
  2,2,2,11,11,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2,
  8,11,8,8,8,11,2,2, 8,8,8,8,8,8,8,8, 8,8,11,11,8,8,8,2, 8,8,8,8,2,2,8,8,
  8,11,8,8,8,11,8,8, 11,11,8,8,2,11,11,8, 11,8,8,11,11,11,11,8, 8,8,8,2,2,8,2,37,
  2,2,2,6,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2,
  2,3,2,8,8,3,3,14, 3,5,2,3,3,2,3,2, 2,7,3,7,3,8,7,14, 2,2,5,2,8,3,2,3,
  5,8,5,5,7,3,7,3, 3,5,3,3,3,14,3,5, 5,7,14,3,5,14,7,5, 7,5,8,7,5,5,5,3,
  2,14,2,2,5,2,2,5, 2,2,2,5,2,2,3,3, 3,5,3,3,7,5,5,3, 3,7,3,3,7,3,8,37,
  // B5xx
  7,39,37,39,39,37,37,37, 39,2,2,37,39,2,37,37, 37,39,39,37,37,37,37,37, 37,39,37,37,37,37,37,37,
  2,2,2,2,39,2,2,2, 2,2,2,2,2,2,14,2, 37,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2,
  8,2,33,33,33,2,11,8, 8,8,8,33,8,8,33,8, 8,39,33,8,33,8,33,8, 8,8,33,8,8,2,8,2,
  8,9,8,8,8,8,8,11, 8,8,8,8,8,11,8,8, 8,8,11,8,3,8,8,8, 8,8,8,8,8,8,2,37,
  2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,5, 2,2,2,2,2,2,2,5,
  2,7,7,3,7,3,8,8, 5,3,8,3,7,7,3,3, 2,3,8,3,8,5,5,2, 5,8,3,7,3,3,3,5,
  3,3,3,3,3,5,3,5, 3,5,5,5,3,3,5,3, 3,14,7,14,14,7,3,3, 3,7,3,8,7,3,7,7,
  2,7,5,3,3,5,2,3, 2,5,3,2,2,2,5,2, 5,5,5,8,3,3,7,3, 8,8,3,8,8,8,7,37,
  // B6xx
  7,1,4,39,37,37,37,37, 37,2,2,37,37,2,37,37, 37,37,37,37,39,37,37,37, 37,39,37,37,37,37,37,37,
  9,2,2,2,11,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,9,2,2,2,9,
  8,2,8,2,8,8,8,2, 8,2,8,8,8,2,2,8, 8,8,8,8,8,8,8,8, 8,3,8,2,2,8,8,2,
  8,30,8,9,8,8,2,8, 8,8,2,9,9,8,9,8, 9,8,9,8,8,8,8,9, 8,2,8,8,8,8,8,37,
  2,2,2,5,2,2,5,5, 2,2,2,2,2,5,2,2, 2,11,5,2,2,2,5,5, 2,2,2,2,2,2,2,2,
  2,8,8,2,8,3,7,5, 3,3,3,3,3,3,8,3, 8,5,2,2,3,7,7,3, 7,3,8,5,3,3,3,7,
  3,3,8,8,3,3,3,5, 3,7,7,3,3,3,3,3, 7,7,14,3,3,8,3,8, 7,3,7,7,8,7,6,5,
  3,7,7,2,2,2,2,2, 2,2,8,8,7,3,3,2, 8,7,5,5,5,5,5,5, 3,3,3,3,3,8,3,37,
  // B7xx
  7,39,37,14,39,37,39,37, 39,11,11,37,39,2,37,39, 39,39,37,39,39,39,39,37, 39,39,37,39,39,39,37,37,
  11,2,2,11,11,2,11,2, 2,2,11,2,2,2,11,2, 37,2,2,2,2,2,2,2, 2,2,2,2,2,2,11,2,
  8,8,8,33,33,11,8,8, 8,2,8,33,8,8,8,8, 8,8,8,8,33,8,8,8, 8,8,33,2,8,2,2,2,
  11,11,8,2,8,11,8,11, 8,8,8,8,11,8,8,8, 2,8,8,8,2,8,8,8, 8,8,8,2,8,8,8,37,
  2,6,6,2,2,2,2,2, 2,6,2,2,2,2,2,2, 2,2,2,2,2,11,2,2, 2,2,2,5,2,2,2,2,
  2,5,3,5,7,2,5,8, 3,5,7,5,3,3,5,5, 5,5,2,5,3,3,3,11, 2,5,3,5,2,3,3,3,
  3,5,5,5,5,3,3,3, 7,3,7,8,3,14,5,5, 5,3,14,8,5,14,3,7, 14,14,7,3,3,3,7,3,
  2,5,3,2,2,2,2,3, 2,2,7,7,5,8,7,7, 3,3,3,7,3,7,3,7, 3,5,7,3,5,5,3,37,
  // B8xx
  7,39,37,39,39,39,37,37, 37,2,2,37,37,2,37,1, 39,39,37,39,39,39,39,37, 39,37,37,37,39,39,39,37,
  2,2,2,2,37,2,2,2, 2,2,2,2,2,2,2,2, 37,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2,
  8,8,2,8,2,6,2,8, 8,8,8,8,8,6,8,8, 2,2,8,8,8,8,8,8, 8,8,8,2,37,2,2,8,
  8,8,2,11,8,8,8,8, 8,8,8,8,2,8,8,8, 8,8,8,8,8,8,8,8, 8,8,8,8,8,8,8,37,
  2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2,
  2,7,2,5,2,5,5,5, 8,7,2,7,2,2,5,2, 2,2,5,5,3,5,5,5, 5,8,2,5,5,3,3,5,
  7,5,5,3,3,5,5,5, 8,3,3,8,8,7,3,3, 3,8,3,5,5,5,3,3, 14,3,3,3,8,7,5,3,
  2,7,3,2,2,2,3,5, 3,5,8,2,2,5,7,3, 5,5,7,5,8,8,3,3, 3,3,3,8,3,7,7,37,
  // B9xx
  7,39,4,37,39,39,39,37, 39,14,2,14,39,14,4,37, 39,39,39,39,39,39,37,37, 39,39,37,39,39,39,39,37,
  14,2,2,2,2,2,2,14, 2,2,2,2,2,2,14,2, 37,2,2,2,2,2,2,2, 2,2,2,2,2,2,14,2,
  8,8,8,8,8,8,8,8, 8,2,8,8,8,8,2,8, 2,8,2,8,8,2,8,2, 8,8,2,2,8,8,2,8,
  8,8,12,12,12,9,2,12, 8,9,8,9,9,8,9,9, 8,8,38,8,9,8,8,8, 8,8,12,8,8,8,2,37,
  2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2,
  2,14,8,2,3,7,3,14, 14,3,8,3,3,7,5,7, 5,3,3,2,2,14,2,14, 2,3,3,3,2,7,2,2,
  3,7,5,14,3,3,7,3, 3,3,3,3,5,7,5,8, 5,14,14,14,7,14,7,5, 3,5,5,3,3,5,5,5,
  7,7,3,3,2,2,5,7, 5,5,8,2,2,3,8,8, 7,7,3,3,8,7,5,5, 5,3,3,3,5,3,7,37,
  // BAxx
  7,39,37,39,39,39,37,37, 39,2,2,37,39,2,37,39, 39,39,37,39,39,39,39,37, 39,39,37,39,39,39,37,37,
  1,2,2,2,2,1,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,14,2,2,2,2,2,
  8,8,2,8,8,2,2,8, 2,8,8,8,8,8,8,8, 8,2,8,2,2,8,8,1, 8,2,8,8,2,8,8,1,
  2,8,8,8,2,12,2,2, 8,12,8,8,2,2,8,12, 8,8,2,2,8,8,8,8, 8,8,8,8,8,8,8,37,
  2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2,
  2,8,3,3,2,5,2,2, 14,8,2,2,2,2,5,5, 5,3,2,2,5,3,2,2, 5,5,2,5,3,3,5,2,
  5,3,7,3,3,3,7,7, 3,3,8,3,7,3,5,3, 5,7,5,3,14,3,8,7, 8,7,3,3,3,7,8,7,
  7,3,8,2,2,2,2,2, 2,2,5,8,3,5,7,2, 3,5,3,3,8,3,7,7, 3,5,3,8,5,3,3,37,
  // BBxx
  7,39,1,39,39,37,14,39, 39,2,2,1,39,11,37,11, 39,39,37,37,39,39,39,37, 39,39,37,37,39,39,37,37,
  11,2,2,2,2,11,11,11, 2,2,2,2,11,2,11,2, 37,2,2,11,11,2,11,11, 11,11,2,11,11,2,14,2,
  8,8,8,2,8,8,8,8, 2,8,2,11,8,2,2,8, 8,8,8,2,8,2,11,8, 8,8,8,8,8,8,2,8,
  8,8,2,2,11,8,11,2, 8,2,11,8,2,2,8,2, 8,8,8,8,2,8,37,8, 8,8,8,8,8,8,8,37,
  2,2,6,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2,
  2,8,3,2,3,2,3,3, 3,5,3,8,2,3,3,3, 3,7,7,7,2,3,2,3, 2,3,2,3,3,2,2,2,
  7,14,7,14,7,7,3,5, 7,7,3,7,5,7,7,7, 7,14,7,3,7,3,3,7, 3,7,8,3,7,3,7,7,
  7,3,8,2,2,2,2,5, 2,2,5,7,5,2,3,7, 3,3,3,5,8,5,5,3, 3,3,3,7,3,5,7,37,
  // BCxx
  7,39,39,39,39,39,37,37, 39,2,2,37,39,2,37,37, 39,39,39,39,37,37,37,37, 37,39,37,37,37,37,37,37,
  2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2,
  8,12,8,8,2,2,8,12, 8,2,8,12,8,12,12,12, 8,8,2,12,2,12,12,8, 8,8,8,8,2,2,8,2,
  2,8,8,9,9,8,8,8, 8,8,2,9,9,9,9,8, 8,8,9,8,8,8,8,8, 8,8,8,8,2,8,8,37,
  2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2,
  2,7,2,7,3,5,3,2, 7,2,3,7,2,5,5,3, 3,5,3,2,3,2,3,2, 3,2,5,2,5,8,5,5,
  3,7,7,3,3,5,3,3, 3,8,3,8,7,3,3,8, 8,3,3,3,7,5,8,3, 3,3,8,3,3,3,3,8,
  3,3,5,2,3,2,2,2, 7,2,7,2,3,2,5,7, 7,7,3,7,3,3,5,5, 5,3,5,3,3,7,3,37,
  // BDxx
  7,39,37,11,37,37,37,37, 39,2,2,37,39,2,6,37, 37,39,37,37,39,39,37,37, 39,39,37,39,39,39,37,37,
  2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2,
  8,2,2,2,2,6,8,8, 8,2,2,8,8,8,2,2, 2,8,8,8,8,8,8,8, 8,8,8,2,8,8,2,8,
  8,8,8,8,8,2,8,8, 2,2,2,8,8,8,8,8, 2,8,2,8,8,8,8,8, 2,3,2,8,8,8,8,39,
  2,2,6,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2,
  2,3,2,3,7,2,2,8, 3,2,7,3,2,3,2,2, 2,2,3,2,5,3,3,3, 7,3,5,3,5,2,7,2,
  5,5,5,5,5,5,8,5, 5,5,5,7,3,5,5,3, 7,7,8,3,3,8,3,8, 3,3,3,3,3,3,8,8,
  3,3,3,2,2,2,2,3, 2,3,7,2,3,2,3,2, 3,7,3,3,3,3,3,7, 3,7,3,3,3,7,3,37,
  // BExx
  7,39,37,14,39,1,37,37, 39,2,2,37,37,2,37,39, 37,39,39,37,37,39,37,37, 39,39,37,39,39,37,37,39,
  2,2,2,2,2,2,2,2, 2,2,2,2,2,2,14,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,14,2,
  8,8,8,2,8,8,8,8, 8,2,8,8,8,2,2,2, 8,8,8,59,8,8,2,8, 8,8,8,2,8,2,8,8,
  8,12,9,8,8,9,2,12, 8,8,8,12,8,12,8,12, 8,8,8,12,12,12,8,8, 8,8,8,8,8,2,8,37,
  2,2,6,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2,
  2,3,3,3,2,8,7,3, 8,3,3,3,3,3,7,3, 3,3,5,3,5,3,3,2, 5,8,3,3,2,2,5,3,
  3,14,8,3,14,3,5,8, 5,8,5,5,8,3,7,5, 7,14,5,3,8,5,3,5, 5,3,7,3,7,3,3,5,
  5,8,2,2,2,2,2,5, 2,2,2,7,7,3,5,5, 7,3,7,5,8,3,3,5, 5,3,5,5,3,3,7,37,
  // BFxx
  7,11,37,37,37,12,37,1, 37,2,2,37,39,2,37,37, 37,37,37,37,39,37,37,37, 37,37,37,37,39,37,37,37,
  12,11,2,2,2,2,2,11, 2,2,2,2,2,2,2,2, 37,2,2,2,2,2,2,2, 2,2,12,2,2,2,2,1,
  8,8,1,1,8,8,1,2, 1,1,2,8,1,1,8,8, 1,1,1,1,1,8,1,8, 8,1,8,2,8,2,8,2,
  8,12,12,8,12,12,8,2, 1,8,8,12,12,2,2,12, 2,8,1,12,2,2,8,12, 2,12,8,2,8,8,2,37,
  2,2,2,2,2,2,2,2, 2,2,2,6,2,2,2,2, 2,2,2,5,2,2,2,2, 2,2,5,2,2,2,2,2,
  2,5,5,5,8,5,7,7, 3,5,3,2,5,5,7,3, 5,5,7,8,3,5,7,7, 3,5,2,2,3,2,5,7,
  5,5,5,5,8,3,3,7, 7,3,7,3,3,7,3,5, 5,14,3,3,5,3,3,3, 3,3,3,3,5,3,7,8,
  2,3,3,2,5,2,2,2, 2,8,3,5,5,3,5,5, 8,3,5,3,7,5,3,3, 5,5,8,8,5,8,5,37,
  // C0xx
  7,39,37,1,39,39,37,1, 39,10,18,1,39,10,37,39, 39,39,37,37,39,39,39,37, 39,39,7,37,39,39,37,39,
  10,10,10,10,1,10,10,10, 10,10,10,10,10,10,10,10, 37,10,10,10,10,10,10,10, 10,10,10,10,10,10,10,10,
  8,8,1,1,1,8,1,1, 8,8,8,30,1,1,1,1, 1,1,8,1,1,3,8,8, 1,8,8,8,8,8,3,3,
  8,3,8,1,8,8,8,8, 39,3,8,3,3,8,8,39, 8,8,1,8,8,8,8,8, 8,8,8,8,8,37,8,37,
  10,6,6,6,6,39,3,3, 39,6,12,6,10,6,6,6, 6,6,6,6,6,6,6,6, 6,3,37,3,3,5,37,1,
  10,3,3,8,7,5,8,5, 3,3,7,3,3,3,7,5, 5,5,5,8,3,7,3,14, 5,7,5,5,3,5,7,3,
  3,10,10,10,10,10,10,5, 3,8,10,10,5,10,5,5, 10,10,10,5,5,10,5,3, 10,8,5,5,7,10,10,7,
  3,5,10,7,3,5,3,5, 7,7,10,3,7,3,3,3, 10,3,8,10,8,7,3,3, 3,8,5,5,5,3,7,10,
  // C1xx
  7,39,1,39,39,39,37,37, 39,14,18,37,39,11,37,39, 39,39,37,39,39,39,37,39, 39,39,39,39,37,37,37,37,
  11,18,14,14,37,11,18,14, 14,11,11,14,18,18,14,11, 37,20,11,14,14,14,14,11, 37,14,11,18,11,10,18,18,
  8,8,1,1,1,1,1,1, 1,1,1,1,1,1,1,8, 1,1,1,1,1,1,1,8, 1,8,1,14,8,11,8,8,
  8,8,1,8,16,16,1,1, 1,10,8,1,1,16,8,8, 8,8,1,16,1,8,16,8, 8,8,1,8,8,8,8,37,
  39,6,6,4,39,11,3,6, 3,39,12,39,37,12,12,6, 37,6,6,6,39,3,6,6, 37,3,3,3,4,11,12,1,
  14,5,3,3,5,3,5,3, 5,5,3,3,3,3,5,5, 7,7,10,7,3,3,5,5, 5,5,3,5,3,3,5,3,
  10,5,5,14,3,10,3,7, 10,11,8,3,18,20,10,3, 3,14,14,10,18,14,5,14, 5,3,3,10,3,5,3,5,
  10,7,3,3,8,3,3,8, 14,3,7,10,3,3,5,5, 5,5,5,10,3,5,5,3, 5,3,5,10,7,5,5,10,
  // C2xx
  7,39,37,37,39,39,39,37, 39,14,14,1,39,14,37,1, 39,37,39,39,39,39,39,37, 37,37,37,39,37,39,37,39,
  10,14,14,14,39,10,14,14, 14,14,14,10,14,10,10,14, 37,10,10,14,14,10,10,10, 39,14,14,10,14,10,14,14,
  8,8,1,16,16,8,8,1, 8,8,16,16,16,1,1,8, 16,8,16,16,1,1,16,8, 8,1,8,8,8,14,8,8,
  8,8,8,8,8,8,8,1, 1,8,8,8,8,8,8,8, 8,8,8,8,8,8,8,1, 8,8,8,8,14,8,8,37,
  37,59,2,11,3,39,11,11, 2,39,10,2,2,59,11,59, 11,11,12,3,2,3,2,2, 11,11,3,12,4,59,17,11,
  2,2,7,2,2,3,7,2, 59,2,2,2,3,7,2,3, 2,2,8,7,2,2,3,3, 7,14,2,2,3,3,7,2,
  10,3,8,3,3,10,3,3, 10,3,3,10,3,10,10,18, 7,10,14,7,3,18,3,3, 7,14,10,3,8,3,3,7,
  8,11,10,11,3,7,7,7, 7,3,7,3,14,3,10,5, 3,10,3,3,3,10,3,5, 5,5,3,5,5,3,3,10,
  // C3xx
  7,1,4,39,39,37,37,1, 37,14,12,3,39,14,1,1, 39,37,39,37,37,39,37,37, 39,39,37,37,39,37,37,37,
  14,14,14,1,1,1,14,14, 14,11,11,5,14,11,14,11, 37,14,10,14,14,10,14,14, 10,10,11,14,14,11,14,1,
  8,10,8,8,8,1,8,12, 8,12,8,8,12,8,12,1, 8,8,12,1,12,12,10,10, 8,8,8,8,8,8,8,39,
  8,8,8,8,8,8,8,8, 8,8,8,8,8,8,8,8, 1,8,1,8,8,8,8,8, 8,8,8,8,14,8,8,37,
  2,2,59,59,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2,
  2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,5,2,5, 2,2,2,5,2,2,2,3,
  3,14,7,14,10,3,7,3, 11,3,5,3,11,11,10,7, 14,11,14,11,11,11,5,3, 3,14,11,11,3,11,11,5,
  5,11,5,11,11,7,3,3, 3,14,7,5,14,11,10,3, 10,3,5,11,3,11,8,3, 5,3,7,3,3,8,8,10,
  // C4xx
  7,6,1,39,37,37,37,37, 39,10,18,1,39,18,1,37, 37,37,37,37,39,37,37,37, 39,37,37,37,39,37,37,37,
  18,18,10,1,1,1,10,1, 6,10,6,3,18,10,18,10, 37,10,10,10,10,5,5,5, 5,5,18,18,10,6,10,1,
  8,8,1,1,1,1,1,1, 1,1,1,1,1,1,1,1, 1,8,1,1,1,1,1,8, 1,8,1,8,8,8,8,8,
  3,8,8,8,8,1,8,1, 1,8,8,1,8,8,1,8, 1,8,1,1,1,1,8,8, 8,8,8,8,8,8,8,37,
  2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2,
  2,5,7,3,3,7,3,5, 5,3,3,5,3,5,7,2, 2,2,8,8,7,8,3,3, 3,7,7,3,2,3,3,3,
  10,5,10,3,5,10,10,3, 10,3,10,10,7,10,10,3, 3,3,8,10,3,18,3,18, 3,18,3,10,3,8,5,10,
  10,3,3,3,3,10,8,20, 10,20,3,10,7,5,10,3, 10,3,8,7,3,8,5,5, 7,7,3,5,5,8,3,10,
  // C5xx
  7,39,37,14,39,39,37,37, 39,14,18,39,39,18,39,39, 39,39,37,37,39,39,37,39, 39,39,39,39,39,39,37,37,
  18,18,18,14,39,18,18,1, 14,18,14,3,18,18,18,14, 37,10,10,10,10,6,10,10, 39,20,18,18,18,1,18,18,
  8,8,1,3,1,1,8,1, 1,1,8,1,1,1,1,1, 1,8,1,1,1,8,1,8, 8,8,1,8,8,8,8,8,
  39,8,1,3,1,8,8,1, 1,8,12,1,1,1,8,8, 1,8,1,1,1,8,8,8, 8,8,12,3,8,8,8,37,
  2,2,2,2,2,2,2,2, 2,37,5,2,2,2,12,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2,
  2,2,2,2,3,2,3,14, 3,5,7,2,5,5,3,2, 5,2,5,2,7,3,3,7, 5,7,2,5,2,2,2,5,
  7,5,5,5,10,3,3,10, 11,10,10,10,10,5,18,11, 10,10,10,11,7,10,11,5, 5,7,11,5,8,3,11,7,
  3,11,3,8,5,8,5,7, 14,8,7,5,7,11,3,3, 5,10,7,3,3,5,7,5, 5,7,3,3,7,8,7,37,
  // C6xx
  7,37,39,1,39,39,37,37, 37,10,12,37,39,11,37,37, 39,39,37,39,39,39,39,37, 39,39,37,39,39,39,37,39,
  11,12,10,1,37,39,11,1, 37,11,11,10,12,18,10,10, 37,3,11,10,3,3,3,3, 39,3,12,39,11,6,10,12,
  37,12,1,8,1,12,8,1, 1,12,8,1,1,1,1,8, 8,8,1,1,1,8,1,8, 8,8,8,8,8,8,8,8,
  8,8,1,1,1,8,8,8, 8,12,8,8,8,12,1,12, 8,8,8,8,1,12,8,12, 8,1,8,8,8,8,8,37,
  37,6,6,6,4,3,2,4, 6,39,6,39,6,6,6,2, 6,6,2,6,37,6,39,6, 39,2,4,2,39,3,37,4,
  2,2,3,7,3,3,7,7, 7,7,3,3,3,3,5,5, 2,3,5,7,3,3,3,3, 3,3,3,7,5,3,3,3,
  3,3,3,7,5,10,3,5, 10,7,10,5,3,10,10,7, 5,11,5,10,5,3,3,3, 3,3,3,5,11,5,3,3,
  7,11,7,11,3,10,3,3, 10,7,5,3,3,5,7,3, 3,7,5,3,3,3,11,5, 3,5,5,3,7,7,7,37,
  // C7xx
  7,39,39,39,37,39,39,39, 39,11,11,14,39,11,37,39, 39,39,37,39,39,39,39,37, 39,39,37,39,39,39,39,39,
  11,11,11,11,11,11,11,11, 11,11,11,11,11,11,11,11, 37,11,11,11,11,14,11,11, 11,11,11,11,11,11,1,11,
  18,1,1,16,16,1,8,16, 1,1,16,16,1,1,16,1, 16,8,16,16,16,1,16,1, 37,1,16,11,39,11,11,11,
  39,1,1,31,16,1,8,1, 1,1,8,1,1,1,1,1, 8,8,1,1,37,1,1,3, 37,8,8,39,37,11,8,37,
  2,11,6,2,37,37,39,12, 37,6,39,3,39,2,2,39, 11,2,2,2,2,11,2,6, 11,11,2,2,2,2,11,2,
  11,11,3,3,7,5,3,3, 3,3,5,3,3,7,3,7, 5,3,5,5,14,7,3,3, 3,14,7,5,7,7,7,3,
  10,11,10,1,10,3,11,11, 11,11,11,11,11,11,11,5, 5,5,5,11,11,5,11,5, 5,5,11,11,7,11,11,11,
  5,11,5,11,11,11,11,3, 5,3,3,3,3,11,7,3, 11,7,3,3,5,5,5,3, 3,5,3,5,5,5,3,10,
  // C8xx
  7,37,37,14,37,37,39,39, 37,11,11,37,39,11,37,39, 37,37,37,37,39,37,37,39, 39,39,37,39,37,37,37,37,
  11,11,11,11,11,11,11,11, 11,11,11,11,11,11,11,11, 37,11,11,11,11,11,18,11, 18,11,11,18,11,11,10,18,
  11,12,1,1,1,12,1,1, 12,12,12,12,12,1,12,12, 12,1,1,1,1,12,1,12, 1,1,1,11,11,11,11,11,
  11,12,1,1,1,12,3,1, 12,12,12,12,12,1,1,12, 10,37,12,12,12,12,12,11, 39,12,12,11,11,11,3,37,
  39,11,6,6,3,37,2,3, 39,6,12,37,39,11,10,6, 11,6,6,6,11,11,6,6, 11,2,2,2,2,11,11,1,
  11,3,7,5,3,3,3,3, 3,7,3,3,7,5,5,5, 5,3,5,7,3,3,3,3, 5,5,3,3,3,7,3,5,
  10,10,10,3,5,10,5,11, 3,11,10,3,10,11,3,3, 10,11,10,11,11,3,7,10, 11,3,11,11,11,3,11,10,
  7,11,3,11,11,11,11,3, 7,3,5,3,10,11,10,5, 10,5,7,7,3,3,11,5, 3,5,11,5,3,3,7,37,
  // C9xx
  7,39,11,39,37,39,37,10, 39,11,18,39,37,11,37,39, 39,37,39,37,39,39,37,37, 39,39,37,39,39,39,37,39,
  11,11,11,11,11,11,11,11, 11,11,11,11,11,11,11,11, 37,11,11,11,11,11,11,11, 11,11,11,18,11,11,11,18,
  8,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,1, 1,1,1,11,11,11,11,11,
  18,8,1,1,1,8,1,1, 1,1,1,1,1,1,1,8, 1,1,1,1,1,8,1,3, 1,1,1,1,11,11,11,37,
  39,6,6,6,6,11,37,12, 6,6,12,6,6,6,12,6, 2,2,2,6,2,11,6,2, 6,2,2,2,2,11,11,2,
  11,11,3,3,7,3,7,7, 3,7,3,3,7,3,7,3, 3,3,7,3,7,3,3,3, 7,3,3,11,7,3,7,11,
  14,3,7,7,10,18,3,3, 18,18,7,3,3,10,18,3, 3,10,10,18,7,7,18,7, 7,3,18,7,3,7,18,7,
  3,7,7,3,3,7,3,3, 3,7,3,7,3,3,3,20, 7,3,3,3,7,3,3,7, 3,3,3,7,7,3,7,37,
  // CAxx
  7,6,39,37,39,37,37,37, 37,11,11,37,37,11,37,37, 37,37,37,37,39,37,37,37, 39,37,37,37,39,37,37,4,
  11,11,11,11,11,11,11,11, 11,11,11,11,11,11,11,11, 37,11,11,11,11,11,11,11, 14,11,11,18,11,11,10,18,
  11,10,12,12,12,1,1,12, 1,8,1,12,1,1,1,10, 12,1,1,1,1,1,1,12, 1,37,12,11,8,11,8,11,
  37,10,1,8,12,8,37,37, 1,1,12,12,12,12,1,8, 10,1,1,12,1,1,39,1, 1,8,12,1,11,11,11,37,
  2,2,6,2,2,11,2,2, 2,2,2,2,12,2,39,6, 11,39,2,6,2,11,6,6, 11,11,37,2,10,11,2,2,
  11,3,3,7,3,3,3,3, 3,3,7,3,7,3,3,3, 3,3,3,3,14,3,3,3, 7,3,14,3,3,3,3,3,
  10,14,3,11,7,10,3,3, 11,7,3,10,7,11,10,11, 3,11,3,11,3,3,3,3, 11,3,11,3,3,11,11,11,
  10,11,7,11,3,11,11,7, 14,3,3,10,3,11,10,3, 10,7,10,7,3,3,3,3, 3,20,7,3,7,3,7,11,
  // CBxx
  7,1,39,37,37,39,37,37, 1,11,18,37,37,11,37,37, 37,37,37,37,37,37,37,37, 37,37,7,37,37,37,37,37,
  11,18,11,10,1,39,18,11, 11,11,11,3,18,18,10,11, 37,10,10,3,10,3,10,10, 3,11,11,18,11,3,11,18,
  8,10,1,1,16,1,1,16, 1,1,16,16,1,16,1,10, 16,1,1,16,16,1,16,1, 1,39,1,37,11,10,11,11,
  37,1,1,1,1,37,1,1, 1,10,1,1,1,8,1,1, 1,1,1,1,1,37,37,37, 8,39,39,3,8,37,3,37,
  2,6,6,4,39,2,2,2, 2,2,2,2,2,2,2,37, 2,2,6,16,6,11,37,6, 2,2,2,2,2,2,2,1,
  11,7,3,3,3,3,7,3, 7,3,3,3,7,3,3,3, 3,7,10,3,7,3,3,7, 3,3,7,7,3,3,3,3,
  10,14,3,14,3,10,3,11, 10,3,10,10,7,3,3,18, 3,3,14,10,18,3,3,3, 3,3,7,10,7,8,10,3,
  3,11,7,3,3,10,3,14, 7,14,3,3,11,11,10,3, 3,3,8,10,7,3,7,3, 3,3,3,3,3,3,7,10,
  // CCxx
  7,39,39,37,39,39,39,37, 39,11,18,37,37,11,37,39, 39,39,39,37,39,39,39,37, 39,39,37,37,39,39,37,37,
  11,10,11,11,11,11,11,11, 18,11,11,10,11,10,10,11, 37,11,11,11,11,11,11,11, 11,11,11,6,11,37,6,18,
  8,1,12,12,12,16,1,1, 12,16,12,12,12,12,12,31, 12,1,12,12,12,8,12,6, 8,10,12,1,11,11,1,11,
  10,1,1,1,8,16,1,1, 1,10,1,1,1,3,1,1, 1,8,18,1,1,10,37,11, 8,10,1,8,8,11,3,39,
  2,2,6,2,2,2,2,2, 2,2,12,6,6,12,12,6, 6,6,6,6,6,6,3,6, 6,37,2,3,37,11,2,1,
  11,7,7,7,7,3,3,3, 3,3,7,3,3,3,7,3, 3,7,3,3,7,7,3,3, 3,3,3,3,3,3,7,7,
  10,18,7,3,7,10,3,11, 10,18,11,10,7,10,10,11, 11,11,11,10,3,3,3,3, 3,3,7,10,7,3,11,10,
  10,3,3,11,11,3,11,7, 10,7,3,7,3,11,10,3, 3,7,7,7,3,3,7,7, 3,3,3,3,3,3,7,10,
  // CDxx
  7,39,37,14,39,37,37,37, 37,14,18,14,37,11,14,11, 39,39,37,39,39,39,37,37, 39,37,37,37,39,37,37,37,
  11,18,11,11,11,11,14,14, 14,11,11,11,18,18,14,11, 37,14,11,14,14,14,14,14, 14,11,14,18,11,11,10,18,
  18,1,1,1,1,1,1,1, 1,10,1,1,1,1,1,1, 1,1,1,1,1,1,1,8, 1,10,1,37,37,11,37,11,
  6,1,1,1,1,16,37,1, 8,10,8,39,1,1,1,1, 37,8,1,1,1,37,10,8, 1,39,8,37,14,11,11,37,
  37,6,6,6,37,39,37,2, 6,37,12,6,6,12,12,6, 6,39,6,6,37,6,6,6, 11,37,37,3,37,11,11,1,
  1,14,7,3,14,3,3,14, 3,3,3,7,3,7,3,3, 3,3,10,3,3,3,3,3, 3,14,14,3,3,7,3,3,
  10,3,14,14,10,10,3,11, 10,11,11,3,11,10,10,11, 3,7,10,11,14,7,11,7, 3,11,3,10,11,11,11,11,
  10,11,3,11,11,10,11,3, 7,14,3,3,11,11,10,20, 7,7,3,11,3,3,3,7, 3,3,3,3,3,7,3,10,
  // CExx
  7,39,37,37,39,39,37,37, 39,11,18,37,39,11,37,37, 39,39,37,37,39,39,37,37, 39,39,3,37,39,39,37,37,
  11,10,11,11,39,10,10,10, 11,10,11,5,11,18,10,11, 37,11,10,10,11,10,10,11, 11,11,11,18,11,11,10,18,
  5,8,16,16,16,10,1,16, 1,1,16,16,1,16,1,37, 16,37,16,16,1,31,16,39, 37,1,16,37,37,8,37,10,
  8,18,37,1,8,37,10,8, 37,18,1,39,1,1,1,1, 16,8,16,16,37,37,16,37, 8,18,1,37,39,8,3,37,
  37,6,6,6,2,2,2,2, 2,2,2,6,2,6,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2,
  2,2,3,2,2,3,2,3, 7,7,3,3,3,2,2,3, 3,2,2,2,3,2,3,2, 2,2,2,3,2,2,2,2,
  3,10,10,10,3,3,10,11, 3,10,3,10,10,10,11,7, 10,10,3,7,10,3,10,10, 11,18,3,3,11,3,3,10,
  7,11,3,7,3,3,11,3, 3,3,10,10,10,11,7,3, 10,3,10,3,10,3,3,3, 7,10,3,3,3,3,3,37,
  // CFxx
  7,11,37,37,37,39,37,37, 37,11,18,11,39,11,37,37, 39,37,37,37,39,39,37,37, 39,39,37,39,37,39,37,37,
  11,11,11,6,11,11,11,11, 11,11,11,11,11,18,11,11, 37,11,11,11,11,11,11,11, 11,11,11,18,11,11,11,18,
  11,12,1,1,1,1,1,30, 1,16,30,1,1,1,1,16, 10,1,1,1,1,16,1,1, 1,1,1,11,11,11,11,11,
  8,12,37,39,12,39,8,37, 8,8,1,39,1,8,1,10, 10,8,12,8,1,12,8,8, 8,8,37,37,18,11,39,37,
  2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,37, 11,39,11,6,11,2,2,6, 11,11,37,2,2,11,2,1,
  11,11,3,3,3,3,3,3, 3,7,3,7,3,7,3,3, 3,3,3,7,3,3,3,3, 3,3,7,7,3,3,20,7,
  10,3,3,7,3,10,3,11, 3,11,3,10,18,18,10,11, 10,11,18,10,3,3,3,3, 3,3,11,10,3,11,3,3,
  3,11,3,11,3,11,11,3, 3,3,3,3,3,11,10,3, 10,3,3,3,3,11,11,3, 11,3,3,3,3,3,3,10,
  // D0xx
  7,39,1,37,39,1,39,9, 39,14,14,1,39,14,1,39, 39,39,37,37,39,39,37,37, 39,39,37,39,39,37,37,37,
  11,14,14,11,1,10,14,14, 14,14,14,14,10,10,10,11, 37,10,10,10,10,14,10,31, 10,10,11,10,14,10,10,14,
  1,13,1,13,13,13,1,1, 1,13,1,16,13,13,16,16, 16,37,13,1,16,13,16,1, 39,16,13,14,3,14,14,10,
  8,16,1,39,1,16,31,3, 37,16,1,16,16,16,16,16, 18,8,16,8,16,16,16,8, 39,16,8,8,14,8,3,37,
  10,2,6,2,2,2,2,2, 2,2,2,2,2,3,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2,
  2,3,2,3,2,2,3,3, 2,3,14,2,3,3,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2,
  10,3,3,10,3,3,10,3, 10,11,10,3,10,3,10,18, 3,10,10,10,3,3,3,3, 3,18,10,10,10,3,3,11,
  10,11,3,3,11,10,11,3, 3,3,3,3,3,3,10,20, 3,20,3,10,3,3,11,3, 3,3,11,3,3,3,3,10,
  // D1xx
  7,37,37,37,39,39,11,37, 39,11,18,37,39,11,37,39, 39,39,37,37,39,39,37,37, 39,39,37,39,39,37,37,37,
  11,11,11,10,11,11,11,11, 11,11,11,11,11,11,11,11, 37,11,11,11,11,11,11,11, 11,11,11,18,11,11,11,18,
  11,1,12,12,10,1,10,1, 10,1,1,12,8,12,37,1, 10,37,37,12,10,1,10,31, 11,3,1,37,11,11,11,11,
  39,1,3,10,31,1,37,37, 10,10,1,11,11,37,7,1, 31,11,10,37,31,1,37,31, 11,39,37,8,11,11,11,37,
  2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 11,2,2,2,2,2,2,2, 2,2,2,2,2,11,2,2,
  11,3,3,3,14,3,3,3, 3,3,3,3,3,3,3,3, 3,3,3,14,14,3,3,14, 3,3,14,3,3,3,14,3,
  10,20,10,11,11,10,11,11, 11,11,10,10,11,11,10,11, 3,10,10,11,3,3,11,3, 3,20,11,3,10,11,11,11,
  10,10,10,11,11,11,11,3, 10,3,10,3,11,11,10,10, 10,10,10,3,3,3,11,10, 3,3,11,3,3,3,3,10,
  // D2xx
  7,39,39,37,39,39,37,12, 39,14,14,39,39,14,37,37, 39,39,37,39,39,39,39,37, 39,39,37,37,39,37,37,37,
  11,10,14,14,11,11,14,14, 14,14,11,11,11,10,10,14, 37,10,14,14,10,14,11,11, 14,11,14,18,14,14,14,18,
  11,16,1,1,1,16,8,1, 31,16,31,12,1,1,1,12, 1,1,1,12,12,12,1,31, 1,31,1,11,8,14,11,11,
  10,31,31,31,31,12,31,31, 31,12,8,39,31,31,31,1, 31,31,1,31,31,16,10,31, 1,31,8,1,14,39,14,39,
  37,6,6,2,2,39,3,4, 39,39,3,6,6,12,3,6, 11,2,2,2,28,3,2,2, 11,2,2,2,2,2,39,1,
  14,14,14,3,14,3,3,14, 14,3,3,3,3,14,3,2, 3,3,3,3,14,3,3,14, 14,14,3,3,14,3,14,14,
  10,14,10,14,3,10,3,11, 10,3,14,3,11,10,10,18, 10,3,3,10,3,3,3,3, 18,18,3,10,10,3,11,3,
  10,11,3,11,3,3,3,3, 10,3,3,3,3,11,10,3, 10,3,3,10,3,3,3,3, 3,3,10,3,3,3,3,10,
  // D3xx
  7,37,37,1,39,37,37,37, 39,11,18,39,37,11,37,18, 37,39,37,39,37,39,37,37, 39,39,37,39,39,37,37,37,
  11,11,11,11,11,11,11,11, 11,11,11,11,11,18,18,11, 11,11,11,11,11,11,11,11, 11,11,11,18,11,11,31,18,
  11,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,1, 1,1,1,3,8,11,3,11,
  37,1,1,1,1,1,1,37, 1,8,1,1,1,1,1,1, 1,1,1,1,1,1,1,1, 1,8,1,11,11,8,39,37,
  10,11,6,6,2,11,3,12, 39,6,6,3,6,3,6,12, 11,2,11,3,11,6,6,2, 2,2,3,3,37,11,3,2,
  11,3,3,12,3,3,3,14, 3,2,3,3,9,3,3,3, 3,3,3,3,3,3,3,14, 20,14,14,14,14,3,3,3,
  3,10,3,3,10,3,10,11, 11,3,3,18,11,3,3,10, 3,3,3,11,18,3,3,10, 11,10,3,18,11,11,10,11,
  3,11,10,11,11,11,11,10, 3,3,3,3,11,11,3,3, 10,3,3,11,3,20,3,10, 11,3,3,3,3,3,3,10,
  // D4xx
  7,39,37,1,39,39,39,37, 39,14,18,37,39,11,37,37, 39,39,39,37,39,39,37,37, 39,39,37,37,39,37,37,37,
  11,18,18,1,11,11,11,1, 18,11,11,3,18,18,18,14, 37,11,11,10,11,11,10,11, 11,11,18,18,11,11,11,18,
  1,1,1,1,1,8,1,8, 1,1,1,1,1,1,1,1, 1,1,1,1,1,8,1,11, 1,39,1,37,37,3,11,18,
  18,33,14,31,39,20,31,1, 11,10,1,39,1,1,1,20, 18,8,8,31,1,3,31,31, 1,18,1,8,11,11,3,39,
  37,6,6,3,3,11,12,3, 37,6,3,6,6,6,12,37, 11,6,3,3,3,6,6,6, 11,3,39,3,3,11,3,4,
  11,14,3,3,3,3,3,14, 14,3,3,3,3,3,3,3, 3,3,3,5,3,3,3,14, 3,14,3,14,3,3,3,3,
  10,18,3,3,3,18,3,11, 11,18,3,3,11,11,11,18, 3,11,18,3,3,18,3,18, 3,3,3,11,11,3,11,11,
  10,11,3,11,11,11,11,3, 10,14,14,3,3,11,10,20, 3,3,3,3,3,3,3,3, 3,3,3,3,3,10,3,37,
  // D5xx
  7,39,37,39,37,39,37,37, 39,14,18,14,37,11,37,37, 39,39,37,39,39,39,37,37, 39,39,37,39,39,39,39,39,
  11,14,11,14,39,11,14,14, 14,11,11,14,18,10,18,11, 37,11,11,11,11,11,10,11, 11,11,11,18,11,11,10,18,
  1,31,1,12,12,1,12,1, 1,1,1,12,1,1,1,12, 1,37,1,12,1,1,12,37, 37,37,12,37,37,18,8,11,
  37,31,1,1,31,31,3,37, 1,1,1,1,39,31,1,31, 1,39,31,31,31,8,31,1, 1,31,37,3,14,37,8,37,
  2,2,2,2,2,2,2,2, 2,37,2,39,2,2,2,2, 2,2,2,3,2,2,2,6, 39,11,2,2,2,2,2,4,
  14,2,3,2,14,2,3,2, 3,3,3,2,3,3,3,3, 3,3,2,3,2,2,2,14, 2,3,2,14,3,3,3,3,
  10,14,3,3,3,3,3,11, 11,11,3,3,18,11,10,11, 3,11,3,18,3,3,3,3, 3,3,11,3,11,11,18,3,
  10,11,3,3,11,11,11,3, 14,14,3,3,3,11,10,3, 3,3,3,3,3,3,11,3, 11,3,11,3,3,3,3,37,
  // D6xx
  7,39,37,37,37,39,37,37, 37,11,11,37,39,11,39,39, 39,37,37,37,37,37,37,37, 39,39,39,39,39,39,37,37,
  11,11,11,1,39,11,11,1, 11,6,11,3,11,1,11,1, 37,1,11,1,11,11,11,11, 37,1,11,1,11,3,1,1,
  39,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,1, 1,1,1,37,37,11,3,11,
  8,1,1,1,1,37,1,1, 1,1,1,1,1,1,1,1, 1,8,1,1,1,1,1,1, 1,1,1,1,39,8,3,37,
  2,2,2,2,2,2,2,2, 6,2,39,12,6,4,6,37, 6,2,2,6,2,12,2,2, 6,2,2,2,2,2,2,4,
  2,14,3,3,3,3,3,3, 3,3,3,3,3,3,3,3, 3,3,3,3,3,3,2,3, 3,3,20,3,3,3,3,3,
  10,3,3,3,18,10,3,11, 10,11,3,18,3,11,3,11, 3,11,3,3,3,3,3,3, 3,3,3,10,3,3,3,8,
  3,11,10,11,11,10,11,3, 10,3,3,3,3,11,3,3, 3,3,3,3,3,3,11,3, 3,3,3,3,3,3,3,10,
  // D7xx
  7,39,37,37,39,37,37,6, 39,18,18,37,39,18,37,4, 37,37,37,39,39,39,37,37, 37,39,37,37,39,39,37,37,
  18,18,18,11,11,11,18,10, 11,18,11,3,18,10,18,10, 37,11,11,11,11,11,11,11, 11,11,18,18,18,11,10,11,
  11,10,11,3,11,10,11,11, 8,10,11,11,11,11,11,37, 11,8,3,39,11,8,11,11, 11,10,11,11,11,11,8,39,
  11,11,11,11,11,37,11,3, 11,18,11,11,11,11,11,3, 18,11,11,11,11,3,37,11, 3,18,11,11,11,11,11,37,
  2,2,2,2,2,3,11,6, 2,6,37,6,3,4,6,6, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2,
  2,2,3,3,2,3,2,2, 2,2,2,3,3,3,3,3, 3,3,3,3,3,3,3,3, 3,3,3,3,3,3,3,3,
  10,18,10,3,7,3,3,3, 10,3,3,3,3,14,18,18, 3,20,10,3,3,18,3,11, 10,18,3,3,3,3,3,3,
  10,20,3,3,3,3,3,3, 14,3,3,10,3,3,3,3, 3,3,3,3,3,3,3,3, 3,3,11,10,7,8,20,37,
  // D8xx
  7,39,39,39,39,39,39,39, 39,1,18,39,39,11,39,39, 39,39,39,39,39,39,39,39, 39,39,39,39,39,39,39,39,
  11,11,11,39,39,39,11,11, 11,11,1,3,11,1,11,1, 7,1,1,1,1,1,1,1, 1,1,11,18,11,1,18,18,
  1,12,1,12,1,12,1,1, 1,12,1,1,1,1,1,12, 1,39,1,1,1,1,1,1, 1,1,1,11,11,11,39,11,
  39,12,1,33,33,12,1,1, 33,1,33,1,33,33,1,1, 8,39,1,1,33,1,1,1, 1,1,33,8,18,11,3,39,
  39,6,6,6,39,39,39,39, 3,39,12,6,2,6,6,39, 3,6,39,3,3,6,6,6, 39,39,39,2,33,11,39,2,
  11,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,14,14,3,14,5,
  10,14,14,14,3,10,11,11, 11,11,10,10,10,12,18,10, 10,11,10,10,3,27,10,27, 11,27,11,11,11,11,11,11,
  10,11,10,11,11,11,11,20, 14,14,10,10,11,11,10,10, 10,11,10,11,3,11,11,3, 11,3,11,10,3,3,3,3,
  // D9xx
  7,12,39,1,4,39,4,39, 39,18,18,39,39,11,39,39, 39,39,39,39,39,39,39,39, 39,39,39,39,39,39,39,39,
  11,18,18,14,1,18,18,11, 6,18,14,5,18,18,18,18, 7,6,6,6,6,6,6,6, 6,6,18,18,18,18,14,18,
  8,1,12,12,12,1,3,1, 12,1,12,12,12,12,12,1, 12,3,12,12,12,1,12,7, 1,3,12,7,7,6,8,39,
  18,8,12,12,12,1,7,8, 12,1,1,12,1,12,1,8, 3,8,12,12,12,3,12,6, 1,8,39,7,3,14,5,3,
  2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,3,6,6, 6,6,3,3,3,3,39,3,
  2,14,2,5,3,5,2,14, 14,3,14,2,2,14,9,3, 6,2,3,3,14,14,3,14, 3,2,14,14,2,5,2,3,
  10,14,7,14,18,10,3,11, 10,11,18,18,18,20,10,6, 20,11,18,20,20,5,20,18, 12,3,11,18,10,11,18,11,
  14,11,3,11,11,11,11,8, 14,14,14,5,11,11,10,7, 3,3,3,11,3,11,11,3, 11,5,11,7,5,5,5,10,
  // DAxx
  7,1,39,1,39,39,39,39, 11,11,11,39,39,11,39,11, 39,39,39,39,39,39,39,39, 39,39,39,39,39,39,39,39,
  11,11,11,11,11,11,11,11, 11,11,11,5,11,11,11,11, 7,11,11,5,5,3,5,11, 11,11,11,18,11,11,11,18,
  11,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,3, 1,1,1,11,11,11,11,11,
  8,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,1, 1,8,1,1,1,1,1,3, 1,1,1,8,8,39,11,3,
  2,2,6,6,2,2,2,2, 2,2,12,6,2,12,12,2, 6,2,6,2,11,2,2,6, 2,2,2,3,2,11,12,1,
  11,11,5,3,3,8,5,3, 10,2,5,3,8,2,3,2, 6,5,6,2,3,2,3,5, 5,3,2,8,3,6,2,11,
  18,18,11,3,18,10,10,11, 11,11,11,11,11,18,18,11, 11,11,11,11,11,11,11,18, 11,11,11,3,11,11,11,10,
  27,11,27,11,11,11,11,7, 12,3,3,5,11,11,3,3, 11,11,11,11,3,11,11,3, 12,3,11,3,3,5,3,11,
  // DBxx
  7,39,39,39,39,39,39,39, 39,10,10,39,39,10,39,39, 39,39,39,39,39,39,39,39, 39,39,39,39,39,39,39,39,
  11,10,10,10,39,10,10,1, 3,11,11,5,11,10,10,31, 7,31,31,31,31,31,31,31, 31,31,10,18,10,10,10,10,
  18,12,1,16,16,12,12,16, 12,1,12,16,16,16,16,39, 16,7,16,16,1,1,12,7, 10,1,16,7,7,8,7,8,
  8,1,12,39,16,1,39,16, 31,8,8,16,8,8,1,10, 8,39,1,1,1,8,39,1, 1,8,7,39,3,8,5,39,
  2,2,6,2,39,39,2,2, 2,6,6,2,2,2,2,39, 2,6,2,2,2,2,6,6, 39,3,2,39,2,2,2,1,
  11,3,3,8,3,8,8,3, 10,2,3,3,3,3,5,3, 2,2,2,2,2,2,2,2, 2,2,3,3,3,6,5,3,
  3,18,10,10,10,10,10,11, 11,10,10,10,10,10,18,11, 11,11,10,11,18,10,11,10, 11,7,3,1,11,11,27,10,
  3,11,3,11,11,11,11,3, 3,1,7,3,11,11,27,27, 7,3,11,11,5,11,5,3, 11,7,11,3,3,3,3,3,
  // DCxx
  7,39,1,11,4,39,39,39, 39,1,11,39,39,11,39,39, 39,39,39,39,39,39,39,39, 39,39,39,39,39,39,39,39,
  11,11,11,11,11,11,11,1, 11,1,11,11,11,11,11,11, 7,11,11,11,11,11,11,11, 11,11,11,20,11,11,1,1,
  11,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,1, 1,11,1,1,1,11,1,1, 1,1,1,11,11,11,11,11,
  8,11,1,1,1,1,1,8, 1,11,11,1,1,1,1,11, 11,11,1,1,1,11,1,11, 1,1,1,11,11,3,11,3,
  39,6,6,39,6,11,11,3, 39,3,3,6,6,6,3,39, 2,11,11,6,3,11,6,6, 2,11,2,3,3,2,3,2,
  2,11,3,3,3,7,11,11, 5,11,2,3,5,3,11,6, 3,5,3,3,7,3,6,3, 3,5,11,5,3,3,3,3,
  3,10,11,11,11,10,18,11, 11,11,11,18,11,10,11,11, 13,11,11,11,18,11,11,11, 11,11,11,11,11,11,13,11,
  3,11,5,11,11,11,11,3, 20,20,20,20,11,11,3,3, 11,20,11,20,20,11,3,20, 11,3,11,3,1,5,3,11,
  // DDxx
  7,1,4,39,39,39,39,39, 39,13,11,13,39,11,4,39, 39,3,39,39,39,39,39,39, 39,39,39,39,39,39,39,39,
  11,13,11,11,11,11,13,13, 13,11,13,5,11,13,13,11, 7,11,11,11,11,11,11,13, 11,11,11,13,11,11,13,13,
  11,13,1,1,13,13,13,13, 13,1,1,13,13,13,13,13, 1,39,13,13,13,1,1,13, 13,13,13,7,13,11,3,11,
  13,13,13,13,13,13,13,13, 13,13,13,13,13,13,13,13, 13,13,13,13,13,7,13,7, 13,13,13,11,13,11,5,3,
  56,6,6,6,3,3,3,6, 39,6,12,6,6,12,12,6, 6,6,6,6,6,6,6,6, 11,6,6,3,3,11,3,3,
  11,11,8,5,5,3,7,8, 7,9,3,3,3,3,3,8, 3,3,6,1,3,7,3,3, 3,3,3,5,3,6,5,11,
  6,5,10,11,11,11,11,11, 11,11,11,10,11,11,11,11, 11,11,10,11,11,11,11,5, 11,11,11,11,11,11,11,11,
  3,11,10,11,11,11,11,13, 20,10,10,10,11,11,20,10, 10,20,10,11,20,11,11,20, 11,20,11,8,3,10,13,3,
  // DExx
  7,1,39,39,4,39,4,39, 39,11,11,39,6,11,39,39, 39,39,39,39,39,39,39,39, 39,39,39,39,39,39,39,39,
  11,11,11,11,11,11,11,11, 6,11,11,11,11,11,13,11, 7,11,11,11,11,11,11,6, 11,11,11,6,11,11,13,18,
  11,13,13,13,16,13,13,13, 13,13,1,13,13,13,16,16, 16,8,1,16,13,13,13,13, 1,13,33,11,11,11,11,1,
  13,13,13,1,1,13,13,1, 13,13,1,3,13,16,13,13, 1,3,1,8,13,13,16,1, 7,16,39,7,6,11,11,3,
  2,6,2,2,2,2,2,2, 2,2,2,2,2,2,2,6, 2,2,31,2,2,2,2,2, 6,2,2,4,2,2,2,4,
  11,11,3,3,3,3,2,2, 2,2,2,2,2,2,2,7, 2,3,6,8,5,6,6,3, 6,3,3,3,7,6,6,11,
  5,18,10,10,10,18,10,11, 11,11,11,18,10,11,18,11, 10,11,10,11,18,11,11,10, 11,10,11,18,11,11,11,11,
  27,11,10,11,11,11,11,10, 20,5,20,10,11,11,13,10, 10,20,20,11,20,11,11,7, 11,10,11,7,13,13,11,11,
  // DFxx
  7,1,39,39,7,10,39,39, 39,11,11,7,39,11,39,11, 39,39,39,39,39,39,39,39, 39,39,37,37,39,39,39,39,
  11,11,11,11,11,11,11,11, 11,11,11,6,11,1,11,11, 7,11,11,11,11,14,14,11, 14,14,11,6,11,11,11,1,
  3,11,1,1,1,1,1,1, 1,1,1,8,1,3,8,1, 1,1,1,1,1,1,1,39, 11,8,1,3,3,11,3,11,
  3,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,1, 1,39,1,1,1,1,1,1, 3,1,1,7,11,11,39,3,
  3,6,6,6,6,3,12,6, 6,6,6,6,6,6,6,6, 6,6,6,12,6,6,6,3, 39,6,6,6,6,11,3,4,
  11,11,3,5,5,6,3,5, 11,5,10,5,3,6,3,6, 6,3,5,3,3,3,3,7, 3,3,3,3,6,3,5,11,
  7,10,10,11,10,10,10,11, 11,11,11,11,10,10,11,11, 11,11,10,11,11,10,10,10, 10,10,11,11,11,11,10,11,
  27,11,10,11,11,11,11,10, 20,3,10,20,11,11,20,20, 10,10,3,11,20,11,11,3, 11,20,11,3,1,7,11,11,
  // E0xx
  7,37,37,14,1,37,37,37, 37,10,10,37,37,10,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  11,10,10,10,11,11,10,10, 10,10,10,10,10,10,10,10, 37,10,10,10,10,10,10,10, 10,10,10,10,10,10,10,10,
  10,3,11,11,33,11,6,33, 8,11,11,6,33,11,33,3, 11,39,33,8,8,11,6,33, 11,6,8,10,10,10,10,6,
  17,11,11,11,11,39,11,11, 6,11,16,30,11,11,11,11, 11,11,11,11,11,6,11,11, 11,11,39,6,17,37,3,37,
  6,6,6,6,6,11,6,6, 6,6,6,6,6,6,6,6, 6,6,6,6,10,6,6,6, 37,6,12,3,10,6,10,10,
  36,14,14,36,2,2,2,2, 14,14,14,14,36,36,2,2, 2,2,2,2,2,14,2,14, 2,2,14,14,14,2,14,14,
  14,14,14,14,17,14,17,14, 14,1,14,14,18,14,14,8, 27,3,18,27,5,27,3,3, 27,3,3,6,27,7,3,6,
  3,10,10,10,10,17,10,10, 17,17,10,10,10,10,17,10, 10,10,10,10,10,10,10,10, 17,17,17,18,5,10,10,10,
  // E1xx
  7,1,1,37,37,39,4,37, 12,1,11,20,37,11,3,11, 39,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  11,11,11,11,11,11,11,11, 11,11,11,11,11,1,10,11, 37,11,11,11,17,11,11,11, 11,11,11,1,11,11,11,1,
  11,11,1,1,1,17,1,17, 17,8,1,1,1,8,1,39, 6,39,1,6,39,1,8,11, 6,8,1,11,17,11,11,11,
  37,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,6, 1,6,1,6,11,11,11,37,
  2,2,2,2,2,2,2,6, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,6, 11,2,12,6,12,12,12,2,
  36,14,14,3,14,36,5,14, 14,6,36,36,36,36,36,36, 3,7,3,10,14,14,14,14, 2,14,2,2,2,2,14,14,
  3,14,11,11,11,11,11,11, 11,11,11,11,11,11,11,11, 11,11,11,11,11,11,11,18, 11,11,11,11,11,11,11,11,
  10,11,36,11,11,11,11,17, 10,17,20,10,11,11,10,17, 10,18,17,10,18,11,11,18, 17,17,17,10,10,18,18,10,
  // E2xx
  7,7,37,37,37,39,37,37, 37,10,10,37,4,10,1,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  10,10,10,10,11,10,10,17, 10,10,10,3,10,10,10,10, 37,10,17,10,10,10,10,10, 10,10,10,10,10,10,10,10,
  10,8,8,11,3,6,11,6, 6,6,6,6,6,6,11,6, 6,6,11,6,11,6,6,6, 6,37,6,10,17,10,37,10,
  17,10,11,16,16,10,16,11, 11,10,16,16,16,11,11,10, 16,8,16,16,11,11,16,6, 8,11,16,3,6,6,3,37,
  2,2,2,6,2,2,2,2, 2,2,2,6,2,6,2,6, 6,2,2,2,2,2,2,2, 2,2,10,6,2,2,6,6,
  36,14,36,3,14,36,14,3, 36,5,14,14,3,36,36,3, 3,3,3,10,14,14,3,14, 10,14,14,14,14,3,14,3,
  14,18,14,14,6,18,3,14, 3,18,14,14,18,14,14,18, 27,3,18,3,7,18,3,3, 27,18,27,7,20,20,27,3,
  10,18,10,17,10,10,17,10, 10,17,10,10,17,10,10,18, 10,10,10,10,17,18,10,10, 17,18,10,10,10,3,16,10,
  // E3xx
  7,37,37,37,37,37,37,1, 3,11,11,37,37,11,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,39,37,
  11,11,11,11,11,11,11,11, 11,11,11,11,11,11,10,11, 37,11,11,11,11,11,11,11, 11,11,11,10,11,11,11,17,
  10,12,1,6,6,12,6,1, 31,18,6,6,1,31,39,6, 6,37,6,39,12,1,6,6, 1,6,8,11,11,10,11,11,
  6,10,12,12,12,1,37,12, 11,12,6,6,12,12,8,1, 12,8,12,1,12,12,10,37, 37,3,12,11,17,11,11,37,
  2,2,2,2,2,2,2,6, 2,2,2,2,2,2,2,2, 11,11,6,6,6,6,6,6, 11,11,6,4,6,11,6,4,
  36,11,36,36,14,36,36,36, 14,6,14,36,36,36,7,36, 3,3,3,10,14,14,3,7, 8,14,14,11,5,3,3,11,
  5,5,11,11,11,18,11,11, 11,11,11,11,11,11,11,11, 11,11,11,11,11,11,11,18, 11,11,11,11,11,11,11,11,
  10,11,17,11,11,11,11,36, 10,17,10,10,11,11,10,20, 10,20,17,11,17,11,11,17, 17,17,11,3,3,5,3,1,
  // E4xx
  7,11,1,11,37,37,37,37, 1,11,11,4,4,11,3,11, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  11,17,11,11,11,11,11,17, 11,11,11,17,11,17,11,11, 11,11,11,11,11,11,11,11, 11,11,11,17,11,11,11,17,
  11,1,6,6,6,1,6,1, 1,17,1,1,1,1,6,11, 6,37,1,6,1,1,1,8, 6,1,1,11,11,11,11,11,
  17,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,1, 1,1,1,1,17,11,11,1,
  2,11,6,6,12,11,2,6, 6,6,6,6,6,6,11,6, 11,6,31,3,11,6,6,6, 11,11,6,3,12,12,2,6,
  36,11,14,7,14,36,8,14, 36,14,14,14,5,3,36,3, 3,3,5,10,14,14,6,14, 2,2,2,2,2,2,2,2,
  3,14,3,14,11,18,11,11, 11,11,11,14,11,11,11,11, 11,11,11,11,11,11,11,18, 11,11,11,11,11,11,11,11,
  10,17,10,11,1,10,11,17, 10,17,10,10,11,11,10,18, 10,17,17,10,17,18,17,17, 17,17,17,10,10,20,11,10,
  // E5xx
  7,10,37,37,37,39,37,4, 39,10,10,1,37,10,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  10,10,10,11,11,10,10,10, 11,11,11,10,10,10,10,10, 37,11,11,11,36,11,10,11, 11,11,10,10,11,11,10,10,
  11,6,31,31,6,31,8,1, 6,6,6,6,1,6,6,37, 8,6,1,37,31,6,1,37, 3,6,6,11,10,11,11,11,
  17,1,1,10,1,1,1,1, 1,1,6,1,1,1,1,33, 1,39,1,1,1,33,1,39, 8,6,1,11,11,11,11,37,
  2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2,
  2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2,
  17,7,11,11,11,18,18,11, 11,11,11,3,11,18,11,11, 11,11,11,18,11,5,11,18, 11,11,11,3,11,11,11,20,
  17,11,10,11,10,11,11,10, 17,10,10,10,10,10,18,17, 10,10,10,18,17,11,10,17, 17,17,17,18,5,20,10,10,
  // E6xx
  7,11,1,9,1,39,1,12, 1,12,12,11,37,11,37,11, 39,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  11,12,11,11,11,11,11,11, 11,11,11,5,12,11,12,11, 37,11,11,11,11,11,11,11, 11,11,11,12,11,11,11,12,
  6,33,11,8,11,33,6,11, 11,33,11,33,33,31,33,33, 33,6,1,6,33,33,6,11, 8,37,11,39,17,6,11,11,
  17,12,1,1,1,12,1,1, 1,12,1,1,1,1,1,12, 1,1,1,1,1,12,1,12, 6,8,1,11,6,11,6,37,
  2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2,
  2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2,
  5,11,11,11,3,11,7,11, 11,11,11,11,11,11,11,11, 11,11,11,11,11,11,11,3, 11,11,11,11,11,11,11,11,
  10,11,10,11,11,11,11,20, 10,17,10,17,11,11,17,18, 1,10,11,11,18,11,11,17, 11,20,11,10,10,3,11,10,
  // E7xx
  7,10,37,37,4,39,37,37, 37,11,10,20,4,10,10,37, 37,37,37,37,37,37,37,37, 37,37,37,3,37,37,37,37,
  10,10,10,11,11,10,11,11, 10,10,10,11,10,10,10,17, 37,10,10,11,11,11,10,17, 11,10,11,20,10,11,11,10,
  11,11,11,11,31,8,31,31, 6,11,11,11,31,8,31,6, 31,3,31,31,31,11,31,6, 11,11,31,11,17,11,6,11,
  11,11,11,16,16,11,11,16, 11,11,16,16,11,11,16,11, 16,6,16,16,16,11,16,11, 8,11,16,11,17,11,8,37,
  2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2,
  2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,14,2,2,2,2,2,2,
  1,14,14,14,18,18,1,14, 5,18,14,14,18,3,6,18, 27,3,18,7,3,18,6,3, 27,18,27,3,8,5,3,3,
  10,17,10,1,10,17,17,17, 10,17,10,10,10,10,10,18, 10,17,18,10,17,1,1,17, 17,17,17,10,11,13,10,10,
  // E8xx
  7,1,37,1,12,39,1,37, 37,10,10,10,37,10,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  10,10,10,17,11,10,10,10, 10,10,10,10,10,10,10,10, 37,14,14,14,14,14,14,14, 14,14,10,10,10,10,10,10,
  10,6,6,11,6,12,3,11, 14,6,6,12,31,6,12,6, 18,37,11,11,11,12,6,6, 8,37,6,10,10,10,10,10,
  10,12,11,11,11,12,11,11, 12,12,12,12,12,11,12,12, 12,11,11,11,12,12,11,6, 11,11,11,6,10,39,10,37,
  2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2,
  10,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,14,2,2,2,2,2,2,
  14,14,14,14,7,14,3,14, 14,3,14,14,18,14,14,18, 14,3,14,14,6,18,3,3, 5,3,27,3,20,20,20,7,
  10,10,10,10,10,10,10,10, 10,10,10,10,10,10,10,18, 10,10,10,10,10,10,10,10, 17,10,17,8,20,20,10,10,
  // E9xx
  7,1,11,4,1,37,39,37, 37,11,11,11,37,11,37,1, 37,37,37,37,37,37,37,37, 37,37,37,3,37,37,39,37,
  11,10,11,11,11,11,11,11, 11,11,11,11,11,11,11,11, 37,17,11,17,11,11,11,11, 11,11,10,10,11,10,11,11,
  11,11,11,11,11,11,3,8, 33,11,6,33,6,31,11,11, 33,6,31,31,3,11,11,11, 11,11,33,6,6,11,11,11,
  6,11,11,11,11,11,11,11, 11,11,11,11,11,11,11,11, 11,11,11,11,11,11,11,8, 11,11,11,6,17,6,17,37,
  2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2,
  2,2,2,2,2,2,2,14, 2,2,2,2,2,2,2,2, 2,2,2,2,14,14,2,14, 2,14,2,2,2,2,2,3,
  18,14,14,14,3,14,3,14, 18,3,14,14,18,14,18,3, 14,3,14,14,18,27,18,18, 3,3,18,18,20,20,20,3,
  14,17,17,17,17,17,17,17, 17,17,20,17,17,17,18,17, 17,10,10,18,18,17,17,17, 17,17,17,18,20,18,18,10,
  // EAxx
  7,37,9,37,37,39,37,9, 37,10,10,37,37,10,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  10,12,10,12,11,10,10,10, 10,10,10,10,10,10,10,10, 37,10,6,10,10,10,10,10, 10,10,10,10,10,10,10,12,
  10,31,31,31,31,31,6,12, 31,6,6,31,31,8,31,31, 3,31,37,31,31,37,31,8, 11,11,31,10,12,10,12,10,
  10,10,12,12,12,11,37,12, 11,10,11,12,11,11,11,10, 12,11,11,11,11,11,11,12, 11,6,12,8,17,6,3,37,
  6,39,6,6,2,3,2,3, 6,6,6,2,39,6,6,6, 6,3,6,6,6,6,6,6, 6,11,6,11,12,6,6,6,
  10,14,6,5,14,36,8,14, 3,3,5,5,5,5,5,3, 2,2,2,2,2,2,9,2, 2,2,2,2,2,2,2,12,
  3,3,14,3,10,5,3,3, 3,3,3,3,3,3,5,3, 14,3,14,3,3,3,5,3, 3,34,3,7,20,20,20,20,
  10,20,10,10,18,10,10,20, 10,20,10,10,10,10,10,20, 10,10,10,10,18,20,10,10, 10,20,10,18,20,20,18,10,
  // EBxx
  7,10,37,37,4,37,37,37, 37,10,10,10,37,10,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  10,10,10,10,11,10,16,10, 10,11,10,5,10,10,10,11, 37,10,17,17,17,10,10,17, 17,10,10,10,10,36,10,10,
  10,3,8,8,8,3,11,11, 3,18,14,11,8,3,31,18, 18,37,8,3,31,3,8,37, 39,18,37,3,10,10,37,39,
  17,10,11,11,16,11,11,16, 11,11,16,16,11,16,11,10, 16,11,11,16,16,11,16,11, 3,3,11,37,8,37,5,37,
  2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2,
  2,2,36,2,5,2,2,2, 2,2,2,36,2,2,2,2, 2,2,2,2,2,5,2,2, 2,2,5,2,5,2,7,2,
  5,18,14,5,3,18,3,14, 3,18,3,3,18,14,18,18, 3,3,18,18,18,18,3,3, 3,3,3,8,20,20,20,20,
  10,18,10,10,10,10,10,20, 10,18,10,10,17,10,10,18, 17,10,18,10,18,18,18,10, 17,17,17,10,10,20,10,10,
  // ECxx
  7,11,37,1,37,37,37,37, 37,14,10,14,37,11,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  11,10,11,11,11,10,10,17, 14,10,11,11,10,10,10,10, 37,11,10,10,17,11,17,11, 17,17,11,10,11,10,14,10,
  11,1,17,8,8,18,8,8, 8,8,1,8,31,17,39,11, 17,31,17,17,1,1,17,17, 17,18,3,14,17,11,11,11,
  3,1,12,12,12,16,1,1, 12,16,12,12,12,12,12,1, 12,1,12,12,12,16,12,1, 11,10,12,11,17,11,11,37,
  2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 2,2,10,2,2,2,2,2, 2,2,2,2,2,2,2,2,
  2,2,2,2,2,2,2,2, 14,2,2,3,36,36,3,2, 2,2,2,10,2,2,2,14, 2,2,2,2,2,2,14,11,
  18,14,14,14,17,3,17,14, 14,18,14,14,3,14,14,18, 3,5,3,11,7,18,3,5, 18,3,12,5,20,20,20,20,
  10,18,17,17,17,10,17,17, 10,17,10,17,10,10,10,10, 17,17,17,10,17,18,17,17, 18,17,17,10,10,10,11,10,
  // EDxx
  7,11,37,37,37,37,37,37, 4,1,1,1,37,1,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  11,1,11,17,11,11,1,1, 11,11,11,17,1,10,1,1, 37,11,11,11,11,11,11,11, 11,11,10,1,11,11,10,1,
  11,1,33,33,33,18,1,1, 33,18,1,33,8,33,1,1, 33,39,33,33,33,33,33,37, 11,37,33,11,10,11,11,11,
  17,1,1,1,1,1,1,1, 1,10,1,1,1,1,1,1, 1,1,1,1,1,1,1,8, 1,10,1,11,17,10,11,37,
  2,2,2,2,2,2,2,2, 2,2,2,2,2,2,2,2, 11,2,2,3,2,2,2,2, 2,2,2,2,2,2,2,10,
  1,11,36,3,11,3,3,3, 11,7,36,36,3,5,5,7, 7,5,3,10,7,3,3,3, 10,9,5,5,3,3,9,11,
  5,18,11,11,11,18,11,11, 11,11,11,11,11,11,11,11, 11,11,11,11,11,11,11,11, 11,11,11,11,11,11,11,11,
  10,11,36,11,11,10,11,10, 10,18,10,10,11,10,10,18, 18,10,10,10,10,11,10,10, 11,18,11,10,10,20,10,10,
  // EExx
  7,10,37,37,37,37,4,37, 37,10,10,1,37,10,10,37, 39,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  10,10,10,10,11,10,10,10, 10,10,10,10,10,10,10,10, 37,17,17,17,10,17,10,10, 10,17,10,10,10,10,10,10,
  10,3,3,31,31,18,31,31, 3,3,31,8,31,31,31,39, 8,37,31,31,31,37,31,31, 31,18,8,10,10,10,3,10,
  17,11,16,16,16,37,11,16, 8,11,16,16,11,16,11,11, 16,11,16,16,11,8,16,3, 11,11,16,37,10,3,3,37,
  2,2,6,2,2,10,3,2, 2,2,3,2,6,2,6,2, 3,6,11,2,10,6,2,2, 37,2,2,2,2,3,2,2,
  10,3,5,3,5,3,8,3, 3,3,3,3,36,10,3,5, 5,7,3,5,3,7,3,3, 10,2,10,5,10,3,3,10,
  17,18,3,10,17,18,3,8, 3,18,5,3,17,5,3,18, 3,3,3,3,3,18,3,27, 18,3,3,3,5,20,20,20,
  17,10,10,10,10,10,10,17, 10,10,10,10,10,10,18,10, 10,10,10,18,10,10,10,10, 17,17,17,18,18,10,10,10,
  // EFxx
  7,37,37,37,37,37,37,10, 37,17,20,17,37,17,37,37, 37,37,37,37,37,37,37,37, 37,37,37,3,37,37,37,37,
  17,17,17,17,17,10,18,17, 17,17,17,17,17,10,10,17, 37,10,10,17,10,10,10,17, 17,10,17,17,17,17,17,17,
  3,11,11,11,11,11,3,3, 8,18,3,11,3,31,37,37, 18,37,37,39,31,11,37,8, 8,37,11,37,17,17,37,17,
  17,12,11,11,11,11,11,30, 3,16,30,11,11,11,11,16, 10,11,11,11,11,16,11,11, 11,39,39,8,17,37,3,37,
  2,2,2,2,2,2,2,2, 2,37,2,2,2,39,2,2, 2,2,2,2,2,2,2,3, 39,3,2,2,2,2,12,2,
  17,36,36,3,2,5,5,5, 2,36,7,7,36,2,3,2, 5,3,3,10,2,3,3,5, 2,2,2,2,2,2,2,2,
  3,5,18,18,18,7,18,18, 18,3,18,18,18,18,18,3, 18,3,18,18,18,7,18,5, 3,5,3,3,20,20,18,20,
  10,5,18,20,18,10,18,18, 10,20,18,10,18,18,10,10, 10,20,18,10,18,20,18,18, 10,20,18,10,10,20,18,10,
  // F0xx
  3,37,1,37,10,37,39,39, 37,10,10,37,37,11,4,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  10,10,10,10,11,10,10,10, 10,10,11,10,10,10,10,10, 37,10,10,10,10,10,10,10, 10,10,10,10,10,10,10,1,
  10,12,3,14,1,1,1,13, 1,13,8,8,13,13,37,18, 18,37,13,39,37,13,39,11, 39,37,1,11,10,10,3,10,
  17,13,1,13,13,13,1,1, 1,13,1,16,13,13,16,16, 16,37,13,1,16,13,16,1, 1,16,13,11,10,11,14,39,
  39,6,6,6,36,11,4,36, 6,6,6,6,6,6,6,6, 2,6,6,6,6,6,6,6, 6,4,3,3,10,2,10,10,
  2,11,3,3,3,3,2,3, 5,3,3,5,5,5,3,5, 3,3,5,10,5,3,5,5, 7,5,11,11,5,3,5,11,
  3,18,5,8,3,18,3,11, 5,18,3,3,18,3,18,18, 3,18,18,18,18,18,3,7, 3,18,5,3,20,20,20,20,
  10,10,10,10,17,10,10,10, 10,17,10,17,10,10,10,18, 10,10,10,10,10,10,10,17, 10,17,17,10,10,13,10,10,
  // F1xx
  3,9,37,9,1,10,39,37, 1,12,10,37,37,10,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  10,10,10,10,10,10,10,10, 10,10,17,10,10,10,10,10, 37,10,10,10,10,10,10,10, 10,10,10,12,10,10,10,10,
  1,1,1,12,8,1,8,1, 1,1,1,8,1,1,37,1, 3,8,3,12,8,1,37,12, 8,37,8,10,1,12,8,10,
  17,1,12,12,10,1,10,1, 10,1,8,12,10,12,1,1, 10,1,10,12,10,1,37,1, 10,10,1,3,17,11,5,37,
  39,6,6,6,4,10,4,4, 6,6,37,6,6,6,6,6, 6,6,6,6,10,6,6,6, 6,37,10,3,4,3,4,4,
  10,11,36,3,3,7,5,5, 3,8,5,3,5,7,3,8, 3,3,3,10,3,3,5,3, 10,5,3,3,3,7,3,3,
  17,5,11,3,3,3,3,3, 11,3,3,18,3,18,18,11, 18,3,18,5,3,5,3,18, 3,7,18,18,20,20,20,20,
  10,20,10,17,10,10,10,20, 10,17,10,10,10,10,10,10, 10,10,10,10,17,10,10,10, 10,20,10,10,10,18,10,10,
  // F2xx
  3,1,37,1,37,37,39,37, 37,10,10,10,37,10,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  10,10,10,10,1,10,10,10, 10,10,10,10,10,10,10,10, 37,10,10,10,10,10,10,10, 10,10,10,10,10,20,10,10,
  10,31,31,17,31,12,10,1, 1,18,31,1,31,31,31,1, 31,37,8,31,31,8,39,37, 39,3,31,10,10,10,8,10,
  17,16,1,1,1,16,39,1, 12,16,1,12,1,1,1,12, 1,1,1,12,12,12,1,1, 1,10,1,1,17,11,37,37,
  37,6,6,39,4,10,3,4, 37,6,6,6,6,6,6,37, 6,6,6,6,10,6,39,6, 39,4,12,11,3,12,3,4,
  10,11,5,3,5,5,3,3, 5,5,5,3,5,8,1,3, 3,5,7,10,7,5,26,5, 10,3,3,20,3,3,3,7,
  3,18,11,3,5,18,3,11, 11,18,11,3,3,11,3,18, 3,3,1,11,3,18,11,3, 5,18,3,3,11,11,11,3,
  10,18,10,17,17,10,17,18, 10,17,10,10,17,10,10,18, 10,10,10,10,18,18,17,10, 17,18,17,10,10,10,10,10,
  // F3xx
  3,1,1,37,4,4,39,37, 1,10,10,9,37,10,37,1, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  10,10,10,10,17,10,10,10, 10,10,11,17,10,10,10,1, 37,18,1,10,10,18,10,18, 10,18,1,10,10,10,1,10,
  3,11,1,1,1,1,1,1, 18,18,31,31,31,1,1,8, 1,11,1,39,31,37,39,1, 37,37,31,37,1,10,37,1,
  39,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,1, 1,1,1,11,17,11,24,37,
  37,39,6,6,37,10,3,12, 37,37,6,6,6,6,6,6, 6,11,23,6,12,6,6,39, 37,24,10,3,12,3,10,12,
  10,11,5,3,8,8,3,3, 5,3,8,3,3,5,5,3, 3,3,3,12,3,3,9,5, 3,3,10,3,9,2,3,12,
  5,18,18,11,18,18,3,11, 11,11,11,18,18,11,11,18, 18,11,18,11,18,18,11,18, 11,11,11,11,11,11,11,11,
  10,10,10,10,10,18,10,10, 10,18,10,18,18,10,18,10, 10,10,10,18,18,10,10,10, 10,10,18,18,20,20,10,10,
  // F4xx
  3,4,37,37,37,37,39,37, 37,11,10,37,37,11,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  10,11,17,11,11,10,20,11, 11,10,11,3,10,10,18,20, 37,10,25,17,10,10,10,11, 10,10,10,11,11,17,10,10,
  17,18,11,8,11,18,11,11, 11,18,24,11,11,11,37,18, 18,37,11,37,11,11,39,11, 11,37,11,11,17,11,37,11,
  17,11,11,11,11,37,39,11, 11,11,11,11,11,11,11,11, 11,11,11,11,11,11,11,11, 11,11,11,11,11,39,3,37,
  2,39,6,6,3,24,3,3, 11,37,37,39,11,6,6,6, 37,6,23,6,3,39,6,37, 37,11,12,3,39,12,12,4,
  25,5,3,7,11,3,11,11, 8,3,8,3,3,8,3,8, 8,8,3,10,3,3,5,3, 5,5,3,5,3,3,9,8,
  18,18,3,3,3,18,3,3, 5,3,3,3,3,3,8,18, 3,5,18,3,3,18,3,18, 3,18,1,3,20,20,20,3,
  10,18,17,17,17,17,3,20, 10,17,10,10,17,10,10,18, 10,17,18,10,10,18,17,18, 17,17,17,10,20,20,20,37,
  // F5xx
  3,37,1,4,37,39,39,37, 37,10,10,37,37,10,37,39, 37,39,37,37,37,37,37,37, 39,37,37,37,37,37,37,37,
  10,10,10,10,10,10,10,10, 10,10,10,10,10,10,10,10, 10,10,10,10,10,10,10,10, 10,10,10,10,10,20,10,10,
  10,10,11,1,3,8,8,1, 10,1,1,1,1,1,37,8, 1,1,1,1,1,1,1,8, 8,1,1,1,17,10,37,39,
  17,12,1,12,12,1,12,1, 1,1,1,12,1,1,1,12, 1,3,1,12,1,1,12,1, 1,1,12,1,8,11,3,37,
  37,6,6,6,4,10,4,4, 6,6,6,6,6,39,6,6, 6,11,6,6,10,6,6,6, 4,4,4,4,3,3,3,4,
  10,11,5,3,3,3,8,8, 5,1,3,3,8,8,5,1, 8,5,3,10,3,3,5,3, 3,3,10,3,3,5,3,3,
  3,5,18,5,18,5,18,1, 11,18,11,18,3,11,3,11, 18,11,18,18,11,11,11,18, 11,11,11,11,11,11,18,11,
  10,11,18,11,18,11,11,18, 10,25,20,18,18,10,10,10, 10,1,18,18,18,10,18,18, 20,3,18,18,3,18,18,33,
  // F6xx
  3,1,37,37,37,37,39,37, 37,1,1,1,37,1,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  11,10,1,1,1,10,1,17, 3,10,11,5,10,10,10,1, 37,5,1,1,5,10,10,3, 1,1,10,10,1,11,1,1,
  5,11,1,1,1,1,1,1, 1,24,1,1,1,1,39,37, 11,37,1,1,1,37,1,37, 1,1,1,37,17,1,39,1,
  17,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,1, 1,1,1,8,1,11,5,37,
  39,6,6,4,24,24,3,3, 37,6,6,6,37,37,6,37, 6,17,17,6,39,37,39,6, 37,37,3,11,4,3,3,4,
  11,11,5,3,3,8,3,3, 3,3,3,3,3,3,3,8, 3,3,3,10,3,3,3,5, 3,5,3,3,3,5,5,5,
  3,3,3,3,3,18,11,11, 11,18,11,11,11,11,3,11, 11,11,11,11,11,18,11,3, 11,11,11,11,11,20,11,1,
  10,17,10,11,17,10,11,17, 10,17,10,18,17,11,10,20, 13,20,17,10,17,18,1,17, 17,20,17,10,10,20,13,10,
  // F7xx
  3,1,37,37,37,37,39,37, 37,10,18,6,37,10,37,37, 37,37,37,37,37,37,37,37, 37,37,7,37,37,37,37,39,
  17,17,17,17,11,10,18,17, 11,10,17,3,17,10,18,10, 11,11,11,11,11,11,11,11, 11,11,17,17,10,17,10,17,
  8,18,11,8,3,18,8,8, 3,18,11,3,11,11,39,18, 11,37,11,39,37,3,11,39, 11,37,3,11,11,8,17,17,
  17,10,37,11,11,10,11,11, 37,8,11,11,11,39,11,20, 11,11,11,11,11,11,11,11, 39,10,11,37,17,17,8,37,
  3,6,6,3,3,3,3,3, 37,6,6,39,6,37,6,39, 6,37,11,6,10,6,6,6, 4,37,3,3,10,4,12,4,
  17,3,3,3,11,3,26,3, 3,3,8,8,3,3,3,3, 3,3,3,10,8,3,26,3, 10,5,3,3,3,5,3,3,
  3,18,3,3,3,18,3,5, 3,18,3,18,18,18,18,18, 18,3,18,18,18,18,3,18, 18,18,18,3,20,20,20,20,
  10,18,10,17,17,10,17,17, 10,17,10,10,17,10,18,18, 17,17,10,18,17,18,17,18, 17,18,17,18,10,20,20,10,
  // F8xx
  3,1,37,37,37,37,39,37, 1,17,17,37,37,17,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  17,17,17,17,1,17,16,17, 17,16,17,5,17,17,17,17, 37,5,1,17,1,1,1,1, 1,10,17,17,17,17,17,17,
  10,8,3,1,1,1,12,1, 1,1,8,1,1,1,37,37, 1,37,1,39,37,8,1,8, 8,37,1,37,17,17,37,1,
  17,12,1,12,1,12,1,1, 1,12,1,1,1,1,1,12, 1,8,1,1,1,1,1,1, 1,1,1,3,37,37,5,37,
  39,6,6,39,4,16,3,17, 39,6,37,39,6,37,6,37, 37,11,6,16,16,37,16,6, 37,37,12,3,10,3,12,24,
  17,11,5,3,11,8,8,8, 5,1,8,5,8,8,1,8, 8,5,3,10,3,5,3,3, 10,5,5,3,5,5,3,5,
  5,5,3,11,24,17,17,11, 11,11,11,11,11,11,11,11, 11,11,11,11,11,11,11,3, 11,3,11,11,11,11,11,11,
  17,17,17,17,17,17,17,17, 10,17,10,17,17,12,18,17, 17,17,10,11,17,17,11,17, 17,17,17,18,10,18,10,37,
  // F9xx
  3,1,37,37,37,37,39,39, 37,12,12,1,37,12,37,39, 37,37,37,37,39,37,37,37, 37,37,37,37,37,37,37,37,
  12,17,12,17,11,11,30,12, 12,12,17,5,12,12,12,17, 37,17,17,17,17,17,10,17, 3,17,12,12,12,17,12,12,
  8,8,11,8,12,11,24,24, 24,24,11,17,11,11,11,8, 39,37,11,8,11,11,11,39, 11,11,3,39,17,12,24,11,
  17,11,12,12,12,11,12,11, 12,11,12,12,12,12,12,11, 12,37,12,12,12,11,12,24, 11,11,12,37,17,24,24,37,
  37,24,6,11,11,12,37,24, 24,24,24,37,37,24,24,24, 24,24,24,12,24,24,37,24, 24,24,12,24,24,24,12,24,
  12,5,24,3,8,5,5,24, 5,24,1,5,5,5,8,8, 5,5,3,8,3,3,5,8, 10,5,8,5,24,3,9,24,
  1,5,24,5,8,5,3,5, 3,1,5,24,17,5,5,5, 8,17,17,5,3,8,8,8, 8,8,8,5,8,8,8,8,
  10,17,17,17,17,10,17,17, 10,17,18,17,17,17,17,17, 17,20,17,20,17,7,17,17, 17,17,17,18,10,8,8,10,
  // FAxx
  3,10,1,37,37,37,37,37, 37,1,17,1,37,17,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  17,17,17,17,1,17,17,17, 17,17,17,17,17,17,17,17, 37,17,17,17,17,17,17,17, 1,17,17,17,17,17,1,17,
  8,8,1,8,8,1,1,8, 8,1,1,24,1,1,37,37, 8,39,1,37,1,1,39,39, 11,1,1,37,17,11,39,17,
  17,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,1, 1,1,1,1,1,1,1,37, 1,37,1,1,17,11,11,37,
  39,6,6,6,12,17,37,12, 37,6,37,37,6,6,6,6, 6,6,6,6,12,6,6,24, 6,37,12,12,24,12,12,4,
  17,5,3,5,5,6,8,8, 8,5,5,11,8,5,1,5, 5,3,3,3,5,3,3,5, 10,5,3,9,5,3,5,11,
  5,18,11,11,18,18,3,11, 11,18,11,11,11,11,11,11, 11,11,11,11,11,11,11,18, 11,11,11,11,11,11,11,11,
  17,18,17,11,17,17,11,17, 12,17,17,17,17,18,17,18, 17,18,17,17,17,18,17,17, 17,17,17,1,5,5,5,10,
  // FBxx
  3,37,37,37,37,37,37,14, 37,10,10,10,39,10,37,37, 37,37,37,37,37,37,37,37, 37,37,37,3,37,37,37,37,
  10,10,10,10,11,10,10,10, 10,10,10,10,10,10,10,10, 39,10,10,10,10,10,10,11, 10,10,10,10,10,10,10,10,
  10,8,11,11,8,18,8,3, 11,18,8,11,11,10,39,37, 11,39,37,37,37,37,37,37, 11,37,11,37,10,10,3,10,
  10,12,11,16,16,12,12,16, 12,8,12,16,16,16,16,11, 16,3,16,16,11,37,12,37, 10,39,16,37,39,37,37,37,
  37,6,6,6,37,10,11,39, 37,6,37,39,6,6,6,6, 3,37,37,6,6,39,6,6, 6,37,37,11,3,10,37,3,
  10,5,10,3,8,8,5,8, 8,3,5,5,5,16,11,8, 3,8,8,5,5,3,3,3, 5,3,5,3,5,3,5,5,
  5,18,5,3,3,18,3,5, 3,18,3,18,18,18,18,18, 18,3,18,3,18,18,3,18, 3,18,7,5,5,5,5,5,
  5,18,10,10,10,10,10,10, 16,10,10,10,10,10,18,18, 10,10,10,5,18,10,10,10, 10,5,5,11,11,5,5,10,
  // FCxx
  3,10,37,37,37,37,37,37, 1,11,10,37,4,10,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  10,10,10,11,11,10,10,11, 10,10,10,10,10,10,10,10, 37,10,10,10,10,10,10,10, 10,10,10,10,10,11,10,10,
  10,33,11,11,11,33,11,11, 11,33,11,11,11,11,11,33, 11,37,11,11,11,33,11,11, 11,11,11,39,10,10,37,11,
  11,11,11,11,11,11,11,11, 11,11,11,11,11,11,11,11, 11,11,11,11,11,11,11,11, 11,11,11,37,11,37,39,39,
  37,6,6,6,37,11,11,11, 6,6,6,6,39,6,6,6, 6,6,11,6,10,6,6,6, 39,11,3,11,11,4,39,4,
  10,5,5,5,5,5,8,5, 8,8,8,8,5,11,8,5, 5,5,3,5,5,33,3,5, 5,5,5,5,3,5,5,5,
  5,5,18,5,18,3,18,18, 18,3,18,18,18,18,18,5, 18,3,18,18,18,5,33,18, 5,3,18,18,5,1,5,1,
  10,10,20,10,10,10,18,11, 10,18,10,18,20,10,10,10, 13,10,10,20,18,10,10,20, 10,10,18,18,11,1,13,10,
  // FDxx
  3,37,1,37,37,37,37,37, 37,13,13,37,37,13,4,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  13,13,13,13,13,13,13,13, 13,13,13,13,13,13,13,13, 11,11,11,11,11,11,11,11, 11,11,13,13,13,13,13,13,
  13,13,13,13,13,13,13,13, 13,13,1,13,13,13,13,37, 13,37,13,13,13,13,39,37, 1,13,13,13,13,13,37,13,
  13,33,1,13,13,33,13,13, 1,33,1,13,13,13,13,13, 13,13,13,13,13,13,1,13, 13,13,13,37,13,13,3,37,
  39,37,37,37,37,22,3,3, 3,37,37,37,39,37,39,39, 39,37,22,12,22,11,37,37, 3,37,12,3,3,12,12,4,
  33,33,8,3,8,8,5,5, 5,8,8,8,8,5,8,1, 3,5,3,5,13,3,3,3, 3,9,5,3,3,3,9,3,
  5,3,3,3,3,18,3,11, 3,18,3,3,5,3,3,3, 13,3,3,3,3,3,3,3, 5,5,11,5,11,1,13,11,
  10,18,10,11,10,18,11,13, 12,18,10,10,10,10,18,10, 13,10,10,20,10,10,1,20, 18,20,5,5,5,13,13,10,
  // FExx
  3,39,39,39,37,37,39,39, 37,13,10,37,39,11,39,39, 39,39,39,39,39,37,39,37, 39,39,37,39,37,39,37,39,
  10,10,10,1,1,10,13,13, 11,10,11,13,10,10,10,13, 11,11,11,11,11,11,11,11, 11,11,13,13,13,13,13,10,
  13,13,1,11,13,13,13,13, 11,13,11,13,13,11,37,37, 11,37,37,11,13,13,39,37, 37,13,11,11,13,11,3,13,
  13,13,13,13,16,13,13,13, 13,13,1,13,13,13,16,16, 16,13,1,16,13,13,13,11, 11,13,13,11,13,11,3,37,
  3,11,37,37,4,4,4,4, 37,37,39,39,37,11,37,37, 11,37,22,16,39,11,39,37, 39,4,3,37,4,11,37,3,
  3,11,1,3,8,8,8,8, 8,8,8,11,8,13,1,1, 8,1,3,18,3,3,3,8, 3,3,10,3,3,3,3,11,
  5,18,11,11,3,18,3,11, 11,18,11,11,11,11,11,11, 11,11,11,11,18,18,18,3, 11,11,11,11,11,11,11,11,
  10,18,10,10,10,18,10,10, 10,18,10,18,10,20,18,18, 10,10,10,20,18,18,13,10, 18,10,1,18,13,13,11,37,
  // FFxx
  3,37,39,39,39,37,37,39, 37,37,10,37,37,10,37,37, 37,37,37,37,37,37,37,37, 37,37,37,37,37,37,37,37,
  10,10,10,37,37,10,10,10, 10,10,10,10,10,10,10,10, 37,36,36,37,36,36,39,39, 39,39,10,10,10,1,10,10,
  10,39,39,39,39,37,39,39, 1,39,39,36,39,39,37,39, 39,39,37,36,39,39,36,37, 36,39,36,10,10,10,37,10,
  10,2,2,2,36,2,2,2, 2,2,2,36,2,2,36,36, 36,2,2,2,2,2,36,2, 2,2,36,39,10,39,39,39,
  36,36,39,37,39,10,39,39, 4,37,36,39,36,36,36,36, 36,37,39,39,10,11,39,39, 37,39,39,39,39,37,39,33,
  10,39,10,33,39,39,36,33, 36,1,36,39,33,10,36,36, 39,39,39,18,39,39,39,39, 39,39,10,11,39,39,39,1,
  10,1,1,11,39,39,39,39, 39,39,39,39,39,39,39,39, 39,39,39,39,39,39,39,39, 3,3,3,3,11,3,3,1,
  10,10,10,10,10,10,10,10, 10,10,10,10,10,10,10,10, 10,10,10,10,10,10,10,10, 10,10,39,39,10,37,10,10,

};		// End kMostLikelyEncoding

// End of generated tables
#endif  // COMPACT_ENC_DET_COMPACT_ENC_DET_GENERATED_TABLES_H_
