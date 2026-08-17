/*
 * libthai - Thai Language Support Library
 * Copyright (C) 2001  Theppitak Karoonboonyanan <theppitak@gmail.com>
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 2.1 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with this library; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin St, Fifth Floor, Boston, MA  02110-1301  USA
 */

/*
 * thctype.h - Thai character classifications
 * Created: 2001-05-17
 * Author:  Theppitak Karoonboonyanan <theppitak@gmail.com>
 */

#ifndef THAI_THCTYPE_H
#define THAI_THCTYPE_H

#include <thai/thailib.h>

BEGIN_CDECL

/**
 * @file   thctype.h
 * @brief  Thai character classifications
 *
 * The Thai Standard Industrial Standards Institute (TIS) defined the Thai 
 * character set for using with computer named TIS-620. This character set is 
 * 8-bit encoded including both English and Thai characters. Aliases of 
 * TIS-620 are TIS620, TIS620-0, TIS620.2529-1, TIS620.2533-0 and ISO-IR-166.
 *
 * The followings are the enconding values in hexadecimal, unicode values and 
 * their names.
 *
 * <pre>
 * 0x00   &lt;U0000&gt; NULL (NUL)
 * 0x01   &lt;U0001&gt; START OF HEADING (SOH)
 * 0x02   &lt;U0002&gt; START OF TEXT (STX)
 * 0x03   &lt;U0003&gt; END OF TEXT (ETX)
 * 0x04   &lt;U0004&gt; END OF TRANSMISSION (EOT)
 * 0x05   &lt;U0005&gt; ENQUIRY (ENQ)
 * 0x06   &lt;U0006&gt; ACKNOWLEDGE (ACK)
 * 0x07   &lt;U0007&gt; BELL (BEL)
 * 0x08   &lt;U0008&gt; BACKSPACE (BS)
 * 0x09   &lt;U0009&gt; CHARACTER TABULATION (HT)
 * 0x0A   &lt;U000A&gt; LINE FEED (LF)
 * 0x0B   &lt;U000B&gt; LINE TABULATION (VT)
 * 0x0C   &lt;U000C&gt; FORM FEED (FF)
 * 0x0D   &lt;U000D&gt; CARRIAGE RETURN (CR)
 * 0x0E   &lt;U000E&gt; SHIFT OUT (SO)
 * 0x0F   &lt;U000F&gt; SHIFT IN (SI)
 * 0x10   &lt;U0010&gt; DATALINK ESCAPE (DLE)
 * 0x11   &lt;U0011&gt; DEVICE CONTROL ONE (DC1)
 * 0x12   &lt;U0012&gt; DEVICE CONTROL TWO (DC2)
 * 0x13   &lt;U0013&gt; DEVICE CONTROL THREE (DC3)
 * 0x14   &lt;U0014&gt; DEVICE CONTROL FOUR (DC4)
 * 0x15   &lt;U0015&gt; NEGATIVE ACKNOWLEDGE (NAK)
 * 0x16   &lt;U0016&gt; SYNCHRONOUS IDLE (SYN)
 * 0x17   &lt;U0017&gt; END OF TRANSMISSION BLOCK (ETB)
 * 0x18   &lt;U0018&gt; CANCEL (CAN)
 * 0x19   &lt;U0019&gt; END OF MEDIUM (EM)
 * 0x1A   &lt;U001A&gt; SUBSTITUTE (SUB)
 * 0x1B   &lt;U001B&gt; ESCAPE (ESC)
 * 0x1C   &lt;U001C&gt; FILE SEPARATOR (IS4)
 * 0x1D   &lt;U001D&gt; GROUP SEPARATOR (IS3)
 * 0x1E   &lt;U001E&gt; RECORD SEPARATOR (IS2)
 * 0x1F   &lt;U001F&gt; UNIT SEPARATOR (IS1)
 * 0x20   &lt;U0020&gt; SPACE
 * 0x21   &lt;U0021&gt; EXCLAMATION MARK
 * 0x22   &lt;U0022&gt; QUOTATION MARK
 * 0x23   &lt;U0023&gt; NUMBER SIGN
 * 0x24   &lt;U0024&gt; DOLLAR SIGN
 * 0x25   &lt;U0025&gt; PERCENT SIGN
 * 0x26   &lt;U0026&gt; AMPERSAND
 * 0x27   &lt;U0027&gt; APOSTROPHE
 * 0x28   &lt;U0028&gt; LEFT PARENTHESIS
 * 0x29   &lt;U0029&gt; RIGHT PARENTHESIS
 * 0x2A   &lt;U002A&gt; ASTERISK
 * 0x2B   &lt;U002B&gt; PLUS SIGN
 * 0x2C   &lt;U002C&gt; COMMA
 * 0x2D   &lt;U002D&gt; HYPHEN-MINUS
 * 0x2E   &lt;U002E&gt; FULL STOP
 * 0x2F   &lt;U002F&gt; SOLIDUS
 * 0x30   &lt;U0030&gt; DIGIT ZERO
 * 0x31   &lt;U0031&gt; DIGIT ONE
 * 0x32   &lt;U0032&gt; DIGIT TWO
 * 0x33   &lt;U0033&gt; DIGIT THREE
 * 0x34   &lt;U0034&gt; DIGIT FOUR
 * 0x35   &lt;U0035&gt; DIGIT FIVE
 * 0x36   &lt;U0036&gt; DIGIT SIX
 * 0x37   &lt;U0037&gt; DIGIT SEVEN
 * 0x38   &lt;U0038&gt; DIGIT EIGHT
 * 0x39   &lt;U0039&gt; DIGIT NINE
 * 0x3A   &lt;U003A&gt; COLON
 * 0x3B   &lt;U003B&gt; SEMICOLON
 * 0x3C   &lt;U003C&gt; LESS-THAN SIGN
 * 0x3D   &lt;U003D&gt; EQUALS SIGN
 * 0x3E   &lt;U003E&gt; GREATER-THAN SIGN
 * 0x3F   &lt;U003F&gt; QUESTION MARK
 * 0x40   &lt;U0040&gt; COMMERCIAL AT
 * 0x41   &lt;U0041&gt; LATIN CAPITAL LETTER A
 * 0x42   &lt;U0042&gt; LATIN CAPITAL LETTER B
 * 0x43   &lt;U0043&gt; LATIN CAPITAL LETTER C
 * 0x44   &lt;U0044&gt; LATIN CAPITAL LETTER D
 * 0x45   &lt;U0045&gt; LATIN CAPITAL LETTER E
 * 0x46   &lt;U0046&gt; LATIN CAPITAL LETTER F
 * 0x47   &lt;U0047&gt; LATIN CAPITAL LETTER G
 * 0x48   &lt;U0048&gt; LATIN CAPITAL LETTER H
 * 0x49   &lt;U0049&gt; LATIN CAPITAL LETTER I
 * 0x4A   &lt;U004A&gt; LATIN CAPITAL LETTER J
 * 0x4B   &lt;U004B&gt; LATIN CAPITAL LETTER K
 * 0x4C   &lt;U004C&gt; LATIN CAPITAL LETTER L
 * 0x4D   &lt;U004D&gt; LATIN CAPITAL LETTER M
 * 0x4E   &lt;U004E&gt; LATIN CAPITAL LETTER N
 * 0x4F   &lt;U004F&gt; LATIN CAPITAL LETTER O
 * 0x50   &lt;U0050&gt; LATIN CAPITAL LETTER P
 * 0x51   &lt;U0051&gt; LATIN CAPITAL LETTER Q
 * 0x52   &lt;U0052&gt; LATIN CAPITAL LETTER R
 * 0x53   &lt;U0053&gt; LATIN CAPITAL LETTER S
 * 0x54   &lt;U0054&gt; LATIN CAPITAL LETTER T
 * 0x55   &lt;U0055&gt; LATIN CAPITAL LETTER U
 * 0x56   &lt;U0056&gt; LATIN CAPITAL LETTER V
 * 0x57   &lt;U0057&gt; LATIN CAPITAL LETTER W
 * 0x58   &lt;U0058&gt; LATIN CAPITAL LETTER X
 * 0x59   &lt;U0059&gt; LATIN CAPITAL LETTER Y
 * 0x5A   &lt;U005A&gt; LATIN CAPITAL LETTER Z
 * 0x5B   &lt;U005B&gt; LEFT SQUARE BRACKET
 * 0x5C   &lt;U005C&gt; REVERSE SOLIDUS
 * 0x5D   &lt;U005D&gt; RIGHT SQUARE BRACKET
 * 0x5E   &lt;U005E&gt; CIRCUMFLEX ACCENT
 * 0x5F   &lt;U005F&gt; LOW LINE
 * 0x60   &lt;U0060&gt; GRAVE ACCENT
 * 0x61   &lt;U0061&gt; LATIN SMALL LETTER A
 * 0x62   &lt;U0062&gt; LATIN SMALL LETTER B
 * 0x63   &lt;U0063&gt; LATIN SMALL LETTER C
 * 0x64   &lt;U0064&gt; LATIN SMALL LETTER D
 * 0x65   &lt;U0065&gt; LATIN SMALL LETTER E
 * 0x66   &lt;U0066&gt; LATIN SMALL LETTER F
 * 0x67   &lt;U0067&gt; LATIN SMALL LETTER G
 * 0x68   &lt;U0068&gt; LATIN SMALL LETTER H
 * 0x69   &lt;U0069&gt; LATIN SMALL LETTER I
 * 0x6A   &lt;U006A&gt; LATIN SMALL LETTER J
 * 0x6B   &lt;U006B&gt; LATIN SMALL LETTER K
 * 0x6C   &lt;U006C&gt; LATIN SMALL LETTER L
 * 0x6D   &lt;U006D&gt; LATIN SMALL LETTER M
 * 0x6E   &lt;U006E&gt; LATIN SMALL LETTER N
 * 0x6F   &lt;U006F&gt; LATIN SMALL LETTER O
 * 0x70   &lt;U0070&gt; LATIN SMALL LETTER P
 * 0x71   &lt;U0071&gt; LATIN SMALL LETTER Q
 * 0x72   &lt;U0072&gt; LATIN SMALL LETTER R
 * 0x73   &lt;U0073&gt; LATIN SMALL LETTER S
 * 0x74   &lt;U0074&gt; LATIN SMALL LETTER T
 * 0x75   &lt;U0075&gt; LATIN SMALL LETTER U
 * 0x76   &lt;U0076&gt; LATIN SMALL LETTER V
 * 0x77   &lt;U0077&gt; LATIN SMALL LETTER W
 * 0x78   &lt;U0078&gt; LATIN SMALL LETTER X
 * 0x79   &lt;U0079&gt; LATIN SMALL LETTER Y
 * 0x7A   &lt;U007A&gt; LATIN SMALL LETTER Z
 * 0x7B   &lt;U007B&gt; LEFT CURLY BRACKET
 * 0x7C   &lt;U007C&gt; VERTICAL LINE
 * 0x7D   &lt;U007D&gt; RIGHT CURLY BRACKET
 * 0x7E   &lt;U007E&gt; TILDE
 * 0x7F   &lt;U007F&gt; DELETE (DEL)
 * 0xA1   &lt;U0E01&gt; THAI CHARACTER KO KAI
 * 0xA2   &lt;U0E02&gt; THAI CHARACTER KHO KHAI
 * 0xA3   &lt;U0E03&gt; THAI CHARACTER KHO KHUAT
 * 0xA4   &lt;U0E04&gt; THAI CHARACTER KHO KHWAI
 * 0xA5   &lt;U0E05&gt; THAI CHARACTER KHO KHON
 * 0xA6   &lt;U0E06&gt; THAI CHARACTER KHO RAKHANG
 * 0xA7   &lt;U0E07&gt; THAI CHARACTER NGO NGU
 * 0xA8   &lt;U0E08&gt; THAI CHARACTER CHO CHAN
 * 0xA9   &lt;U0E09&gt; THAI CHARACTER CHO CHING
 * 0xAA   &lt;U0E0A&gt; THAI CHARACTER CHO CHANG
 * 0xAB   &lt;U0E0B&gt; THAI CHARACTER SO SO
 * 0xAC   &lt;U0E0C&gt; THAI CHARACTER CHO CHOE
 * 0xAD   &lt;U0E0D&gt; THAI CHARACTER YO YING
 * 0xAE   &lt;U0E0E&gt; THAI CHARACTER DO CHADA
 * 0xAF   &lt;U0E0F&gt; THAI CHARACTER TO PATAK
 * 0xB0   &lt;U0E10&gt; THAI CHARACTER THO THAN
 * 0xB1   &lt;U0E11&gt; THAI CHARACTER THO NANGMONTHO
 * 0xB2   &lt;U0E12&gt; THAI CHARACTER THO PHUTHAO
 * 0xB3   &lt;U0E13&gt; THAI CHARACTER NO NEN
 * 0xB4   &lt;U0E14&gt; THAI CHARACTER DO DEK
 * 0xB5   &lt;U0E15&gt; THAI CHARACTER TO TAO
 * 0xB6   &lt;U0E16&gt; THAI CHARACTER THO THUNG
 * 0xB7   &lt;U0E17&gt; THAI CHARACTER THO THAHAN
 * 0xB8   &lt;U0E18&gt; THAI CHARACTER THO THONG
 * 0xB9   &lt;U0E19&gt; THAI CHARACTER NO NU
 * 0xBA   &lt;U0E1A&gt; THAI CHARACTER BO BAIMAI
 * 0xBB   &lt;U0E1B&gt; THAI CHARACTER PO PLA
 * 0xBC   &lt;U0E1C&gt; THAI CHARACTER PHO PHUNG
 * 0xBD   &lt;U0E1D&gt; THAI CHARACTER FO FA
 * 0xBE   &lt;U0E1E&gt; THAI CHARACTER PHO PHAN
 * 0xBF   &lt;U0E1F&gt; THAI CHARACTER FO FAN
 * 0xC0   &lt;U0E20&gt; THAI CHARACTER PHO SAMPHAO
 * 0xC1   &lt;U0E21&gt; THAI CHARACTER MO MA
 * 0xC2   &lt;U0E22&gt; THAI CHARACTER YO YAK
 * 0xC3   &lt;U0E23&gt; THAI CHARACTER RO RUA
 * 0xC4   &lt;U0E24&gt; THAI CHARACTER RU
 * 0xC5   &lt;U0E25&gt; THAI CHARACTER LO LING
 * 0xC6   &lt;U0E26&gt; THAI CHARACTER LU
 * 0xC7   &lt;U0E27&gt; THAI CHARACTER WO WAEN
 * 0xC8   &lt;U0E28&gt; THAI CHARACTER SO SALA
 * 0xC9   &lt;U0E29&gt; THAI CHARACTER SO RUSI
 * 0xCA   &lt;U0E2A&gt; THAI CHARACTER SO SUA
 * 0xCB   &lt;U0E2B&gt; THAI CHARACTER HO HIP
 * 0xCC   &lt;U0E2C&gt; THAI CHARACTER LO CHULA
 * 0xCD   &lt;U0E2D&gt; THAI CHARACTER O ANG
 * 0xCE   &lt;U0E2E&gt; THAI CHARACTER HO NOKHUK
 * 0xCF   &lt;U0E2F&gt; THAI CHARACTER PAIYANNOI
 * 0xD0   &lt;U0E30&gt; THAI CHARACTER SARA A
 * 0xD1   &lt;U0E31&gt; THAI CHARACTER MAI HAN-AKAT
 * 0xD2   &lt;U0E32&gt; THAI CHARACTER SARA AA
 * 0xD3   &lt;U0E33&gt; THAI CHARACTER SARA AM
 * 0xD4   &lt;U0E34&gt; THAI CHARACTER SARA I
 * 0xD5   &lt;U0E35&gt; THAI CHARACTER SARA II
 * 0xD6   &lt;U0E36&gt; THAI CHARACTER SARA UE
 * 0xD7   &lt;U0E37&gt; THAI CHARACTER SARA UEE
 * 0xD8   &lt;U0E38&gt; THAI CHARACTER SARA U
 * 0xD9   &lt;U0E39&gt; THAI CHARACTER SARA UU
 * 0xDA   &lt;U0E3A&gt; THAI CHARACTER PHINTHU
 * 0xDF   &lt;U0E3F&gt; THAI CHARACTER SYMBOL BAHT
 * 0xE0   &lt;U0E40&gt; THAI CHARACTER SARA E
 * 0xE1   &lt;U0E41&gt; THAI CHARACTER SARA AE
 * 0xE2   &lt;U0E42&gt; THAI CHARACTER SARA O
 * 0xE3   &lt;U0E43&gt; THAI CHARACTER SARA AI MAIMUAN
 * 0xE4   &lt;U0E44&gt; THAI CHARACTER SARA AI MAIMALAI
 * 0xE5   &lt;U0E45&gt; THAI CHARACTER LAKKHANGYAO
 * 0xE6   &lt;U0E46&gt; THAI CHARACTER MAIYAMOK
 * 0xE7   &lt;U0E47&gt; THAI CHARACTER MAITAIKHU
 * 0xE8   &lt;U0E48&gt; THAI CHARACTER MAI EK
 * 0xE9   &lt;U0E49&gt; THAI CHARACTER MAI THO
 * 0xEA   &lt;U0E4A&gt; THAI CHARACTER MAI TRI
 * 0xEB   &lt;U0E4B&gt; THAI CHARACTER MAI CHATTAWA
 * 0xEC   &lt;U0E4C&gt; THAI CHARACTER THANTHAKHAT
 * 0xED   &lt;U0E4D&gt; THAI CHARACTER NIKHAHIT
 * 0xEE   &lt;U0E4E&gt; THAI CHARACTER YAMAKKAN
 * 0xEF   &lt;U0E4F&gt; THAI CHARACTER FONGMAN
 * 0xF0   &lt;U0E50&gt; THAI DIGIT ZERO
 * 0xF1   &lt;U0E51&gt; THAI DIGIT ONE
 * 0xF2   &lt;U0E52&gt; THAI DIGIT TWO
 * 0xF3   &lt;U0E53&gt; THAI DIGIT THREE
 * 0xF4   &lt;U0E54&gt; THAI DIGIT FOUR
 * 0xF5   &lt;U0E55&gt; THAI DIGIT FIVE
 * 0xF6   &lt;U0E56&gt; THAI DIGIT SIX
 * 0xF7   &lt;U0E57&gt; THAI DIGIT SEVEN
 * 0xF8   &lt;U0E58&gt; THAI DIGIT EIGHT
 * 0xF9   &lt;U0E59&gt; THAI DIGIT NINE
 * 0xFA   &lt;U0E5A&gt; THAI CHARACTER ANGKHANKHU
 * 0xFB   &lt;U0E5B&gt; THAI CHARACTER KHOMUT
 * </pre>
 *
 * Thai characters consist of 44 consonants, vowels, tonemarks, diacritics and 
 * Thai digits. Thai vowels are divided into 4 groups, Leading Vowels (LV), 
 * Following Vowels (FV), Below Vowels (BV) and Above Vowels (AV). There are 4 
 * tonemarks whose position is above a consonant. Diacritics are divided into 
 * 2 groups, Above Diacritics (AD) and Below Diacritics (BD).
 *
 * Libthai has defined 4 levels for the position of a character.
 *
 *   @li  Below level: a character is placed below the consonant. 
 *        th_chlevel() will return the value -1 for these characters. 
 *
 *   @li  Base level: this includes consonants, FV and LV. A character is 
 *        placed on baseline. 
 *        th_chlevel() will return the value 0 for these characters.
 *
 *   @li  Above level: a character is placed just above the consonant.
 *        th_chlevel() will return the value 1 for these characters.
 *
 *   @li  Top level: this includes tone marks and diacritics. For plain 
 *        character cell rendering, it is safe to put these characters at 
 *        top-most level. However, some rendering engines may lower them down 
 *        on absence of character at Above level, for typographical quality.
 *        th_chlevel() will return the value 2 for these characters.
 *
 * There is an extra level value 3 for certain characters which are usually 
 * classified as characters at Above level, but are also allowed to be placed 
 * at Top level for some rare cases. Two characters fall in this category, 
 * namely MAITAIKHU and NIKHAHIT.
 *
 * MAITAIKHU can be placed at Top level when writing some minority languages 
 * such as Kuy, to shorten some syllables with compound vowels, such as Sara 
 * Ia and Sara Uea. NIKHAHIT can be placed at Top level in Pali/Sanskrit words, 
 * to represent -ng final sound above SARA I.
 *
 * The following figure illustrates a Thai word and characters' level.
 *
 * <pre>
 * --------------------------- Top(2) 
 * ------*-------------------- Top(2) 
 * ------*-------------------- Top(2) 
 * <b>---------------------------</b>
 * --------------------------- Above(1)
 * ------*---------------*---- Above(1)
 * ---****---------------*---- Above(1)
 * --------------------------- Above(1)
 * <b>---------------------------</b>
 * --------------------------- Base(0) 
 * --*---*----***-----*--*---- Base(0) 
 * -*-*-*-*--*---*---*-*-*---- Base(0) 
 * --**-*-*------*---**--*---- Base(0) 
 * ---**--*---*--*---*---*---- Base(0) 
 * ---**--*--*-*-*----*--*---- Base(0) 
 * ---*---*--**--*---*---*---- Base(0) 
 * ---*---*--*---*---*---*---- Base(0) 
 * ---*---*--*****---*****---- Base(0) 
 * <b>--------------------------- Baseline</b>
 * --------------------------- Below(-1)
 * -------------------**-*---- Below(-1)
 * --------------------***---- Below(-1)
 * --------------------------- Below(-1)
 * </pre>
 *
 * A character placed at below, above or top level is also called dead 
 * character. It is usually combined with a consonant, after a dead character 
 * is typed, the cursor will not be advanced to the next display cell. BV, BD, 
 * TONE, AD and AV are classified as dead character.
 */

extern int th_istis(thchar_t c);

extern int th_isthai(thchar_t c);
extern int th_iseng(thchar_t c);

/* Thai letter classification */
extern int th_isthcons(thchar_t c);
extern int th_isthvowel(thchar_t c);
extern int th_isthtone(thchar_t c);
extern int th_isthdiac(thchar_t c);
extern int th_isthdigit(thchar_t c);
extern int th_isthpunct(thchar_t c);

/* Thai consonant shapes classification */
extern int th_istaillesscons(thchar_t c);
extern int th_isovershootcons(thchar_t c);
extern int th_isundershootcons(thchar_t c);
extern int th_isundersplitcons(thchar_t c);

/* Thai vowel classification */
extern int th_isldvowel(thchar_t c);
extern int th_isflvowel(thchar_t c);
extern int th_isupvowel(thchar_t c);
extern int th_isblvowel(thchar_t c);

extern int th_chlevel(thchar_t c);

extern int th_iscombchar(thchar_t c);

/*
 * implementation parts
 */
#include <ctype.h>
#define _th_ISbit(bit)  (1 << (bit))
#define _th_bitfld(base, val)  ((val) << (base))
#define _th_bitmsk(base, bits) (~((~(unsigned)0) << (bits)) << (base))

enum {
  _th_IStis   = _th_ISbit(0),        /* TIS-620 char */

  _th_IScons  = _th_ISbit(1),        /* Thai consonant */
  _th_CClassMsk = _th_bitmsk(1, 3),  /*   Thai consonant shape masks */
  _th_CCtailless   = _th_bitfld(2, 0)|_th_IScons,   /* tailless cons */
  _th_CCovershoot  = _th_bitfld(2, 1)|_th_IScons,   /* overshoot cons */
  _th_CCundershoot = _th_bitfld(2, 2)|_th_IScons,   /* undershoot cons */
  _th_CCundersplit = _th_bitfld(2, 3)|_th_IScons,   /* undersplit cons */
  _th_ISvowel = _th_ISbit(4),        /* Thai vowel */
  _th_VClassMsk = _th_bitmsk(4, 3),  /*   Thai vowel class masks */
  _th_VCflvowel = _th_bitfld(5, 0)|_th_ISvowel,  /*   Thai following vowel */
  _th_VCldvowel = _th_bitfld(5, 1)|_th_ISvowel,  /*   Thai leading vowel */
  _th_VCupvowel = _th_bitfld(5, 2)|_th_ISvowel,  /*   Thai upper vowel */
  _th_VCblvowel = _th_bitfld(5, 3)|_th_ISvowel,  /*   Thai below vowel */
  _th_IStone  = _th_ISbit(7),        /* Thai tone mark */
  _th_ISdiac  = _th_ISbit(8),        /* Thai diacritic */
  _th_ISdigit = _th_ISbit(9),        /* digit */
  _th_ISpunct = _th_ISbit(10)        /* punctuation */
};

extern const unsigned short _th_ctype_tbl[];

#define _th_isctype(c, type)      (_th_ctype_tbl[c] & (type))
#define _th_isbits(c, mask, val)  ((_th_ctype_tbl[c] & (mask)) == (val))

#define th_istis(c)         _th_isctype((c), _th_IStis)

#define th_isthai(c)        (th_istis(c) && ((c) & 0x80))
#define th_iseng(c)         (!((c) & 0x80))

/* Thai letter classification */
#define th_isthcons(c)      _th_isctype((c), _th_IScons)
#define th_isthvowel(c)     _th_isctype((c), _th_ISvowel)
#define th_isthtone(c)      _th_isctype((c), _th_IStone)
#define th_isthdiac(c)      _th_isctype((c), _th_ISdiac)
#define th_isthdigit(c)     _th_isctype((c), _th_ISdigit)
#define th_isthpunct(c)     _th_isctype((c), _th_ISpunct)

/* Thai consonant shapes classification */
#define th_istaillesscons(c)   _th_isbits((c), _th_CClassMsk, _th_CCtailless)
#define th_isovershootcons(c)  _th_isbits((c), _th_CClassMsk, _th_CCovershoot)
#define th_isundershootcons(c) _th_isbits((c), _th_CClassMsk, _th_CCundershoot)
#define th_isundersplitcons(c) _th_isbits((c), _th_CClassMsk, _th_CCundersplit)

/* Thai vowel classification */
#define th_isldvowel(c)     _th_isbits((c), _th_VClassMsk, _th_VCldvowel)
#define th_isflvowel(c)     _th_isbits((c), _th_VClassMsk, _th_VCflvowel)
#define th_isupvowel(c)     _th_isbits((c), _th_VClassMsk, _th_VCupvowel)
#define th_isblvowel(c)     _th_isbits((c), _th_VClassMsk, _th_VCblvowel)

extern const int            _th_chlevel_tbl[];

#define th_chlevel(c)       (_th_chlevel_tbl[c])
#define th_iscombchar(c)    (th_chlevel(c) != 0)

END_CDECL

#endif  /* THAI_THCTYPE_H */

