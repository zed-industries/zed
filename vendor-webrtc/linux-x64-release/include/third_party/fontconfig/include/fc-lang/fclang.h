/*
 * fontconfig/fc-lang/fclang.tmpl.h
 *
 * Copyright © 2002 Keith Packard
 *
 * Permission to use, copy, modify, distribute, and sell this software and its
 * documentation for any purpose is hereby granted without fee, provided that
 * the above copyright notice appear in all copies and that both that
 * copyright notice and this permission notice appear in supporting
 * documentation, and that the name of the author(s) not be used in
 * advertising or publicity pertaining to distribution of the software without
 * specific, written prior permission.  The authors make no
 * representations about the suitability of this software for any purpose.  It
 * is provided "as is" without express or implied warranty.
 *
 * THE AUTHOR(S) DISCLAIMS ALL WARRANTIES WITH REGARD TO THIS SOFTWARE,
 * INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS, IN NO
 * EVENT SHALL THE AUTHOR(S) BE LIABLE FOR ANY SPECIAL, INDIRECT OR
 * CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM LOSS OF USE,
 * DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR OTHER
 * TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR
 * PERFORMANCE OF THIS SOFTWARE.
 */

/* total size: 1655 unique leaves: 739 */

#define LEAF0       (279 * sizeof (FcLangCharSet))
#define OFF0        (LEAF0 + 739 * sizeof (FcCharLeaf))
#define NUM0        (OFF0 + 883 * sizeof (uintptr_t))
#define SET(n)      (n * sizeof (FcLangCharSet) + offsetof (FcLangCharSet, charset))
#define OFF(s,o)    (OFF0 + o * sizeof (uintptr_t) - SET(s))
#define NUM(s,n)    (NUM0 + n * sizeof (FcChar16) - SET(s))
#define LEAF(o,l)   (LEAF0 + l * sizeof (FcCharLeaf) - (OFF0 + o * sizeof (intptr_t)))
#define fcLangCharSets (fcLangData.langCharSets)
#define fcLangCharSetIndices (fcLangData.langIndices)
#define fcLangCharSetIndicesInv (fcLangData.langIndicesInv)

static const struct {
    FcLangCharSet  langCharSets[279];
    FcCharLeaf     leaves[739];
    uintptr_t      leaf_offsets[883];
    FcChar16       numbers[883];
    FcChar16        langIndices[279];
    FcChar16        langIndicesInv[279];
} fcLangData = {
{
    { "aa",  { FC_REF_CONSTANT, 1, OFF(0,0), NUM(0,0) } }, /* 0 */
    { "ab",  { FC_REF_CONSTANT, 1, OFF(1,1), NUM(1,1) } }, /* 1 */
    { "af",  { FC_REF_CONSTANT, 2, OFF(2,2), NUM(2,2) } }, /* 2 */
    { "agr",  { FC_REF_CONSTANT, 1, OFF(3,4), NUM(3,4) } }, /* 3 */
    { "ak",  { FC_REF_CONSTANT, 5, OFF(4,5), NUM(4,5) } }, /* 4 */
    { "am",  { FC_REF_CONSTANT, 2, OFF(5,10), NUM(5,10) } }, /* 5 */
    { "an",  { FC_REF_CONSTANT, 1, OFF(6,12), NUM(6,12) } }, /* 6 */
    { "anp",  { FC_REF_CONSTANT, 1, OFF(7,13), NUM(7,13) } }, /* 7 */
    { "ar",  { FC_REF_CONSTANT, 1, OFF(8,14), NUM(8,14) } }, /* 8 */
    { "as",  { FC_REF_CONSTANT, 1, OFF(9,15), NUM(9,15) } }, /* 9 */
    { "ast",  { FC_REF_CONSTANT, 2, OFF(10,16), NUM(10,16) } }, /* 10 */
    { "av",  { FC_REF_CONSTANT, 1, OFF(11,18), NUM(11,18) } }, /* 11 */
    { "ay",  { FC_REF_CONSTANT, 1, OFF(12,19), NUM(12,19) } }, /* 12 */
    { "ayc",  { FC_REF_CONSTANT, 1, OFF(13,20), NUM(13,20) } }, /* 13 */
    { "az-az",  { FC_REF_CONSTANT, 3, OFF(14,21), NUM(14,21) } }, /* 14 */
    { "az-ir",  { FC_REF_CONSTANT, 1, OFF(15,24), NUM(15,24) } }, /* 15 */
    { "ba",  { FC_REF_CONSTANT, 1, OFF(16,25), NUM(16,25) } }, /* 16 */
    { "be",  { FC_REF_CONSTANT, 1, OFF(17,26), NUM(17,26) } }, /* 17 */
    { "bem",  { FC_REF_CONSTANT, 1, OFF(18,27), NUM(18,27) } }, /* 18 */
    { "ber-dz",  { FC_REF_CONSTANT, 4, OFF(19,28), NUM(19,28) } }, /* 19 */
    { "ber-ma",  { FC_REF_CONSTANT, 1, OFF(20,32), NUM(20,32) } }, /* 20 */
    { "bg",  { FC_REF_CONSTANT, 1, OFF(21,33), NUM(21,33) } }, /* 21 */
    { "bh",  { FC_REF_CONSTANT, 1, OFF(22,13), NUM(22,13) } }, /* 22 */
    { "bhb",  { FC_REF_CONSTANT, 1, OFF(23,13), NUM(23,13) } }, /* 23 */
    { "bho",  { FC_REF_CONSTANT, 1, OFF(24,13), NUM(24,13) } }, /* 24 */
    { "bi",  { FC_REF_CONSTANT, 1, OFF(25,34), NUM(25,34) } }, /* 25 */
    { "bin",  { FC_REF_CONSTANT, 3, OFF(26,35), NUM(26,35) } }, /* 26 */
    { "bm",  { FC_REF_CONSTANT, 3, OFF(27,38), NUM(27,38) } }, /* 27 */
    { "bn",  { FC_REF_CONSTANT, 1, OFF(28,41), NUM(28,41) } }, /* 28 */
    { "bo",  { FC_REF_CONSTANT, 1, OFF(29,42), NUM(29,42) } }, /* 29 */
    { "br",  { FC_REF_CONSTANT, 1, OFF(30,43), NUM(30,43) } }, /* 30 */
    { "brx",  { FC_REF_CONSTANT, 1, OFF(31,44), NUM(31,44) } }, /* 31 */
    { "bs",  { FC_REF_CONSTANT, 2, OFF(32,45), NUM(32,45) } }, /* 32 */
    { "bua",  { FC_REF_CONSTANT, 1, OFF(33,47), NUM(33,47) } }, /* 33 */
    { "byn",  { FC_REF_CONSTANT, 2, OFF(34,48), NUM(34,48) } }, /* 34 */
    { "ca",  { FC_REF_CONSTANT, 2, OFF(35,50), NUM(35,50) } }, /* 35 */
    { "ce",  { FC_REF_CONSTANT, 1, OFF(36,18), NUM(36,18) } }, /* 36 */
    { "ch",  { FC_REF_CONSTANT, 1, OFF(37,52), NUM(37,52) } }, /* 37 */
    { "chm",  { FC_REF_CONSTANT, 1, OFF(38,53), NUM(38,53) } }, /* 38 */
    { "chr",  { FC_REF_CONSTANT, 1, OFF(39,54), NUM(39,54) } }, /* 39 */
    { "ckb",  { FC_REF_CONSTANT, 1, OFF(40,55), NUM(40,55) } }, /* 40 */
    { "cmn",  { FC_REF_CONSTANT, 83, OFF(41,56), NUM(41,56) } }, /* 41 */
    { "co",  { FC_REF_CONSTANT, 2, OFF(42,139), NUM(42,139) } }, /* 42 */
    { "crh",  { FC_REF_CONSTANT, 2, OFF(43,141), NUM(43,141) } }, /* 43 */
    { "cs",  { FC_REF_CONSTANT, 2, OFF(44,143), NUM(44,143) } }, /* 44 */
    { "csb",  { FC_REF_CONSTANT, 2, OFF(45,145), NUM(45,145) } }, /* 45 */
    { "cu",  { FC_REF_CONSTANT, 1, OFF(46,147), NUM(46,147) } }, /* 46 */
    { "cv",  { FC_REF_CONSTANT, 2, OFF(47,148), NUM(47,148) } }, /* 47 */
    { "cy",  { FC_REF_CONSTANT, 3, OFF(48,150), NUM(48,150) } }, /* 48 */
    { "da",  { FC_REF_CONSTANT, 1, OFF(49,153), NUM(49,153) } }, /* 49 */
    { "de",  { FC_REF_CONSTANT, 1, OFF(50,154), NUM(50,154) } }, /* 50 */
    { "doi",  { FC_REF_CONSTANT, 1, OFF(51,155), NUM(51,155) } }, /* 51 */
    { "dsb",  { FC_REF_CONSTANT, 2, OFF(52,143), NUM(52,143) } }, /* 52 */
    { "dv",  { FC_REF_CONSTANT, 1, OFF(53,156), NUM(53,156) } }, /* 53 */
    { "dz",  { FC_REF_CONSTANT, 1, OFF(54,42), NUM(54,42) } }, /* 54 */
    { "ee",  { FC_REF_CONSTANT, 4, OFF(55,157), NUM(55,157) } }, /* 55 */
    { "el",  { FC_REF_CONSTANT, 1, OFF(56,161), NUM(56,161) } }, /* 56 */
    { "en",  { FC_REF_CONSTANT, 1, OFF(57,162), NUM(57,162) } }, /* 57 */
    { "eo",  { FC_REF_CONSTANT, 2, OFF(58,163), NUM(58,163) } }, /* 58 */
    { "es",  { FC_REF_CONSTANT, 1, OFF(59,12), NUM(59,12) } }, /* 59 */
    { "et",  { FC_REF_CONSTANT, 2, OFF(60,165), NUM(60,165) } }, /* 60 */
    { "eu",  { FC_REF_CONSTANT, 1, OFF(61,167), NUM(61,167) } }, /* 61 */
    { "fa",  { FC_REF_CONSTANT, 1, OFF(62,24), NUM(62,24) } }, /* 62 */
    { "fat",  { FC_REF_CONSTANT, 5, OFF(63,5), NUM(63,5) } }, /* 63 */
    { "ff",  { FC_REF_CONSTANT, 3, OFF(64,168), NUM(64,168) } }, /* 64 */
    { "fi",  { FC_REF_CONSTANT, 2, OFF(65,171), NUM(65,171) } }, /* 65 */
    { "fil",  { FC_REF_CONSTANT, 1, OFF(66,173), NUM(66,173) } }, /* 66 */
    { "fj",  { FC_REF_CONSTANT, 1, OFF(67,27), NUM(67,27) } }, /* 67 */
    { "fo",  { FC_REF_CONSTANT, 1, OFF(68,174), NUM(68,174) } }, /* 68 */
    { "fr",  { FC_REF_CONSTANT, 2, OFF(69,139), NUM(69,139) } }, /* 69 */
    { "fur",  { FC_REF_CONSTANT, 1, OFF(70,175), NUM(70,175) } }, /* 70 */
    { "fy",  { FC_REF_CONSTANT, 1, OFF(71,176), NUM(71,176) } }, /* 71 */
    { "ga",  { FC_REF_CONSTANT, 3, OFF(72,177), NUM(72,177) } }, /* 72 */
    { "gd",  { FC_REF_CONSTANT, 1, OFF(73,180), NUM(73,180) } }, /* 73 */
    { "gez",  { FC_REF_CONSTANT, 2, OFF(74,181), NUM(74,181) } }, /* 74 */
    { "gl",  { FC_REF_CONSTANT, 1, OFF(75,12), NUM(75,12) } }, /* 75 */
    { "gn",  { FC_REF_CONSTANT, 3, OFF(76,183), NUM(76,183) } }, /* 76 */
    { "gu",  { FC_REF_CONSTANT, 1, OFF(77,186), NUM(77,186) } }, /* 77 */
    { "gv",  { FC_REF_CONSTANT, 1, OFF(78,187), NUM(78,187) } }, /* 78 */
    { "ha",  { FC_REF_CONSTANT, 3, OFF(79,188), NUM(79,188) } }, /* 79 */
    { "hak",  { FC_REF_CONSTANT, 83, OFF(80,56), NUM(80,56) } }, /* 80 */
    { "haw",  { FC_REF_CONSTANT, 3, OFF(81,191), NUM(81,191) } }, /* 81 */
    { "he",  { FC_REF_CONSTANT, 1, OFF(82,194), NUM(82,194) } }, /* 82 */
    { "hi",  { FC_REF_CONSTANT, 1, OFF(83,13), NUM(83,13) } }, /* 83 */
    { "hif",  { FC_REF_CONSTANT, 1, OFF(84,13), NUM(84,13) } }, /* 84 */
    { "hne",  { FC_REF_CONSTANT, 1, OFF(85,13), NUM(85,13) } }, /* 85 */
    { "ho",  { FC_REF_CONSTANT, 1, OFF(86,27), NUM(86,27) } }, /* 86 */
    { "hr",  { FC_REF_CONSTANT, 2, OFF(87,45), NUM(87,45) } }, /* 87 */
    { "hsb",  { FC_REF_CONSTANT, 2, OFF(88,195), NUM(88,195) } }, /* 88 */
    { "ht",  { FC_REF_CONSTANT, 1, OFF(89,197), NUM(89,197) } }, /* 89 */
    { "hu",  { FC_REF_CONSTANT, 2, OFF(90,198), NUM(90,198) } }, /* 90 */
    { "hy",  { FC_REF_CONSTANT, 1, OFF(91,200), NUM(91,200) } }, /* 91 */
    { "hz",  { FC_REF_CONSTANT, 3, OFF(92,201), NUM(92,201) } }, /* 92 */
    { "ia",  { FC_REF_CONSTANT, 1, OFF(93,27), NUM(93,27) } }, /* 93 */
    { "id",  { FC_REF_CONSTANT, 1, OFF(94,204), NUM(94,204) } }, /* 94 */
    { "ie",  { FC_REF_CONSTANT, 1, OFF(95,205), NUM(95,205) } }, /* 95 */
    { "ig",  { FC_REF_CONSTANT, 2, OFF(96,206), NUM(96,206) } }, /* 96 */
    { "ii",  { FC_REF_CONSTANT, 5, OFF(97,208), NUM(97,208) } }, /* 97 */
    { "ik",  { FC_REF_CONSTANT, 1, OFF(98,213), NUM(98,213) } }, /* 98 */
    { "io",  { FC_REF_CONSTANT, 1, OFF(99,27), NUM(99,27) } }, /* 99 */
    { "is",  { FC_REF_CONSTANT, 1, OFF(100,214), NUM(100,214) } }, /* 100 */
    { "it",  { FC_REF_CONSTANT, 1, OFF(101,215), NUM(101,215) } }, /* 101 */
    { "iu",  { FC_REF_CONSTANT, 3, OFF(102,216), NUM(102,216) } }, /* 102 */
    { "ja",  { FC_REF_CONSTANT, 83, OFF(103,219), NUM(103,219) } }, /* 103 */
    { "jv",  { FC_REF_CONSTANT, 1, OFF(104,302), NUM(104,302) } }, /* 104 */
    { "ka",  { FC_REF_CONSTANT, 1, OFF(105,303), NUM(105,303) } }, /* 105 */
    { "kaa",  { FC_REF_CONSTANT, 1, OFF(106,304), NUM(106,304) } }, /* 106 */
    { "kab",  { FC_REF_CONSTANT, 4, OFF(107,28), NUM(107,28) } }, /* 107 */
    { "ki",  { FC_REF_CONSTANT, 2, OFF(108,305), NUM(108,305) } }, /* 108 */
    { "kj",  { FC_REF_CONSTANT, 1, OFF(109,27), NUM(109,27) } }, /* 109 */
    { "kk",  { FC_REF_CONSTANT, 1, OFF(110,307), NUM(110,307) } }, /* 110 */
    { "kl",  { FC_REF_CONSTANT, 2, OFF(111,308), NUM(111,308) } }, /* 111 */
    { "km",  { FC_REF_CONSTANT, 1, OFF(112,310), NUM(112,310) } }, /* 112 */
    { "kn",  { FC_REF_CONSTANT, 1, OFF(113,311), NUM(113,311) } }, /* 113 */
    { "ko",  { FC_REF_CONSTANT, 45, OFF(114,312), NUM(114,312) } }, /* 114 */
    { "kok",  { FC_REF_CONSTANT, 1, OFF(115,13), NUM(115,13) } }, /* 115 */
    { "kr",  { FC_REF_CONSTANT, 3, OFF(116,357), NUM(116,357) } }, /* 116 */
    { "ks",  { FC_REF_CONSTANT, 1, OFF(117,360), NUM(117,360) } }, /* 117 */
    { "ku-am",  { FC_REF_CONSTANT, 2, OFF(118,361), NUM(118,361) } }, /* 118 */
    { "ku-iq",  { FC_REF_CONSTANT, 1, OFF(119,55), NUM(119,55) } }, /* 119 */
    { "ku-ir",  { FC_REF_CONSTANT, 1, OFF(120,55), NUM(120,55) } }, /* 120 */
    { "ku-tr",  { FC_REF_CONSTANT, 2, OFF(121,363), NUM(121,363) } }, /* 121 */
    { "kum",  { FC_REF_CONSTANT, 1, OFF(122,365), NUM(122,365) } }, /* 122 */
    { "kv",  { FC_REF_CONSTANT, 1, OFF(123,366), NUM(123,366) } }, /* 123 */
    { "kw",  { FC_REF_CONSTANT, 3, OFF(124,367), NUM(124,367) } }, /* 124 */
    { "kwm",  { FC_REF_CONSTANT, 1, OFF(125,27), NUM(125,27) } }, /* 125 */
    { "ky",  { FC_REF_CONSTANT, 1, OFF(126,370), NUM(126,370) } }, /* 126 */
    { "la",  { FC_REF_CONSTANT, 2, OFF(127,371), NUM(127,371) } }, /* 127 */
    { "lah",  { FC_REF_CONSTANT, 1, OFF(128,373), NUM(128,373) } }, /* 128 */
    { "lb",  { FC_REF_CONSTANT, 1, OFF(129,374), NUM(129,374) } }, /* 129 */
    { "lez",  { FC_REF_CONSTANT, 1, OFF(130,18), NUM(130,18) } }, /* 130 */
    { "lg",  { FC_REF_CONSTANT, 2, OFF(131,375), NUM(131,375) } }, /* 131 */
    { "li",  { FC_REF_CONSTANT, 1, OFF(132,377), NUM(132,377) } }, /* 132 */
    { "lij",  { FC_REF_CONSTANT, 1, OFF(133,378), NUM(133,378) } }, /* 133 */
    { "ln",  { FC_REF_CONSTANT, 4, OFF(134,379), NUM(134,379) } }, /* 134 */
    { "lo",  { FC_REF_CONSTANT, 1, OFF(135,383), NUM(135,383) } }, /* 135 */
    { "lt",  { FC_REF_CONSTANT, 2, OFF(136,384), NUM(136,384) } }, /* 136 */
    { "lv",  { FC_REF_CONSTANT, 2, OFF(137,386), NUM(137,386) } }, /* 137 */
    { "lzh",  { FC_REF_CONSTANT, 83, OFF(138,56), NUM(138,56) } }, /* 138 */
    { "mag",  { FC_REF_CONSTANT, 1, OFF(139,13), NUM(139,13) } }, /* 139 */
    { "mai",  { FC_REF_CONSTANT, 1, OFF(140,13), NUM(140,13) } }, /* 140 */
    { "mfe",  { FC_REF_CONSTANT, 2, OFF(141,139), NUM(141,139) } }, /* 141 */
    { "mg",  { FC_REF_CONSTANT, 1, OFF(142,388), NUM(142,388) } }, /* 142 */
    { "mh",  { FC_REF_CONSTANT, 2, OFF(143,389), NUM(143,389) } }, /* 143 */
    { "mhr",  { FC_REF_CONSTANT, 1, OFF(144,365), NUM(144,365) } }, /* 144 */
    { "mi",  { FC_REF_CONSTANT, 3, OFF(145,391), NUM(145,391) } }, /* 145 */
    { "miq",  { FC_REF_CONSTANT, 3, OFF(146,394), NUM(146,394) } }, /* 146 */
    { "mjw",  { FC_REF_CONSTANT, 1, OFF(147,162), NUM(147,162) } }, /* 147 */
    { "mk",  { FC_REF_CONSTANT, 1, OFF(148,397), NUM(148,397) } }, /* 148 */
    { "ml",  { FC_REF_CONSTANT, 1, OFF(149,398), NUM(149,398) } }, /* 149 */
    { "mn-cn",  { FC_REF_CONSTANT, 1, OFF(150,399), NUM(150,399) } }, /* 150 */
    { "mn-mn",  { FC_REF_CONSTANT, 1, OFF(151,400), NUM(151,400) } }, /* 151 */
    { "mni",  { FC_REF_CONSTANT, 1, OFF(152,401), NUM(152,401) } }, /* 152 */
    { "mnw",  { FC_REF_CONSTANT, 1, OFF(153,402), NUM(153,402) } }, /* 153 */
    { "mo",  { FC_REF_CONSTANT, 4, OFF(154,403), NUM(154,403) } }, /* 154 */
    { "mr",  { FC_REF_CONSTANT, 1, OFF(155,13), NUM(155,13) } }, /* 155 */
    { "ms",  { FC_REF_CONSTANT, 1, OFF(156,27), NUM(156,27) } }, /* 156 */
    { "mt",  { FC_REF_CONSTANT, 2, OFF(157,407), NUM(157,407) } }, /* 157 */
    { "my",  { FC_REF_CONSTANT, 1, OFF(158,402), NUM(158,402) } }, /* 158 */
    { "na",  { FC_REF_CONSTANT, 2, OFF(159,409), NUM(159,409) } }, /* 159 */
    { "nan",  { FC_REF_CONSTANT, 84, OFF(160,411), NUM(160,411) } }, /* 160 */
    { "nb",  { FC_REF_CONSTANT, 1, OFF(161,495), NUM(161,495) } }, /* 161 */
    { "nds",  { FC_REF_CONSTANT, 1, OFF(162,154), NUM(162,154) } }, /* 162 */
    { "ne",  { FC_REF_CONSTANT, 1, OFF(163,496), NUM(163,496) } }, /* 163 */
    { "ng",  { FC_REF_CONSTANT, 1, OFF(164,27), NUM(164,27) } }, /* 164 */
    { "nhn",  { FC_REF_CONSTANT, 2, OFF(165,497), NUM(165,497) } }, /* 165 */
    { "niu",  { FC_REF_CONSTANT, 2, OFF(166,499), NUM(166,499) } }, /* 166 */
    { "nl",  { FC_REF_CONSTANT, 1, OFF(167,501), NUM(167,501) } }, /* 167 */
    { "nn",  { FC_REF_CONSTANT, 1, OFF(168,502), NUM(168,502) } }, /* 168 */
    { "no",  { FC_REF_CONSTANT, 1, OFF(169,495), NUM(169,495) } }, /* 169 */
    { "nqo",  { FC_REF_CONSTANT, 1, OFF(170,503), NUM(170,503) } }, /* 170 */
    { "nr",  { FC_REF_CONSTANT, 1, OFF(171,27), NUM(171,27) } }, /* 171 */
    { "nso",  { FC_REF_CONSTANT, 2, OFF(172,504), NUM(172,504) } }, /* 172 */
    { "nv",  { FC_REF_CONSTANT, 4, OFF(173,506), NUM(173,506) } }, /* 173 */
    { "ny",  { FC_REF_CONSTANT, 2, OFF(174,510), NUM(174,510) } }, /* 174 */
    { "oc",  { FC_REF_CONSTANT, 1, OFF(175,512), NUM(175,512) } }, /* 175 */
    { "om",  { FC_REF_CONSTANT, 1, OFF(176,27), NUM(176,27) } }, /* 176 */
    { "or",  { FC_REF_CONSTANT, 1, OFF(177,513), NUM(177,513) } }, /* 177 */
    { "os",  { FC_REF_CONSTANT, 1, OFF(178,365), NUM(178,365) } }, /* 178 */
    { "ota",  { FC_REF_CONSTANT, 1, OFF(179,514), NUM(179,514) } }, /* 179 */
    { "pa",  { FC_REF_CONSTANT, 1, OFF(180,515), NUM(180,515) } }, /* 180 */
    { "pa-pk",  { FC_REF_CONSTANT, 1, OFF(181,373), NUM(181,373) } }, /* 181 */
    { "pap-an",  { FC_REF_CONSTANT, 1, OFF(182,516), NUM(182,516) } }, /* 182 */
    { "pap-aw",  { FC_REF_CONSTANT, 1, OFF(183,517), NUM(183,517) } }, /* 183 */
    { "pl",  { FC_REF_CONSTANT, 2, OFF(184,518), NUM(184,518) } }, /* 184 */
    { "ps-af",  { FC_REF_CONSTANT, 1, OFF(185,520), NUM(185,520) } }, /* 185 */
    { "ps-pk",  { FC_REF_CONSTANT, 1, OFF(186,521), NUM(186,521) } }, /* 186 */
    { "pt",  { FC_REF_CONSTANT, 1, OFF(187,522), NUM(187,522) } }, /* 187 */
    { "qu",  { FC_REF_CONSTANT, 2, OFF(188,523), NUM(188,523) } }, /* 188 */
    { "quz",  { FC_REF_CONSTANT, 2, OFF(189,523), NUM(189,523) } }, /* 189 */
    { "raj",  { FC_REF_CONSTANT, 1, OFF(190,13), NUM(190,13) } }, /* 190 */
    { "rif",  { FC_REF_CONSTANT, 4, OFF(191,525), NUM(191,525) } }, /* 191 */
    { "rm",  { FC_REF_CONSTANT, 1, OFF(192,529), NUM(192,529) } }, /* 192 */
    { "rn",  { FC_REF_CONSTANT, 1, OFF(193,27), NUM(193,27) } }, /* 193 */
    { "ro",  { FC_REF_CONSTANT, 3, OFF(194,530), NUM(194,530) } }, /* 194 */
    { "ru",  { FC_REF_CONSTANT, 1, OFF(195,365), NUM(195,365) } }, /* 195 */
    { "rw",  { FC_REF_CONSTANT, 1, OFF(196,27), NUM(196,27) } }, /* 196 */
    { "sa",  { FC_REF_CONSTANT, 1, OFF(197,13), NUM(197,13) } }, /* 197 */
    { "sah",  { FC_REF_CONSTANT, 1, OFF(198,533), NUM(198,533) } }, /* 198 */
    { "sat",  { FC_REF_CONSTANT, 1, OFF(199,534), NUM(199,534) } }, /* 199 */
    { "sc",  { FC_REF_CONSTANT, 1, OFF(200,535), NUM(200,535) } }, /* 200 */
    { "sco",  { FC_REF_CONSTANT, 3, OFF(201,536), NUM(201,536) } }, /* 201 */
    { "sd",  { FC_REF_CONSTANT, 1, OFF(202,539), NUM(202,539) } }, /* 202 */
    { "se",  { FC_REF_CONSTANT, 2, OFF(203,540), NUM(203,540) } }, /* 203 */
    { "sel",  { FC_REF_CONSTANT, 1, OFF(204,365), NUM(204,365) } }, /* 204 */
    { "sg",  { FC_REF_CONSTANT, 1, OFF(205,542), NUM(205,542) } }, /* 205 */
    { "sgs",  { FC_REF_CONSTANT, 3, OFF(206,543), NUM(206,543) } }, /* 206 */
    { "sh",  { FC_REF_CONSTANT, 3, OFF(207,546), NUM(207,546) } }, /* 207 */
    { "shn",  { FC_REF_CONSTANT, 1, OFF(208,402), NUM(208,402) } }, /* 208 */
    { "shs",  { FC_REF_CONSTANT, 2, OFF(209,549), NUM(209,549) } }, /* 209 */
    { "si",  { FC_REF_CONSTANT, 1, OFF(210,551), NUM(210,551) } }, /* 210 */
    { "sid",  { FC_REF_CONSTANT, 2, OFF(211,552), NUM(211,552) } }, /* 211 */
    { "sk",  { FC_REF_CONSTANT, 2, OFF(212,554), NUM(212,554) } }, /* 212 */
    { "sl",  { FC_REF_CONSTANT, 2, OFF(213,45), NUM(213,45) } }, /* 213 */
    { "sm",  { FC_REF_CONSTANT, 2, OFF(214,556), NUM(214,556) } }, /* 214 */
    { "sma",  { FC_REF_CONSTANT, 1, OFF(215,558), NUM(215,558) } }, /* 215 */
    { "smj",  { FC_REF_CONSTANT, 1, OFF(216,559), NUM(216,559) } }, /* 216 */
    { "smn",  { FC_REF_CONSTANT, 2, OFF(217,560), NUM(217,560) } }, /* 217 */
    { "sms",  { FC_REF_CONSTANT, 3, OFF(218,562), NUM(218,562) } }, /* 218 */
    { "sn",  { FC_REF_CONSTANT, 1, OFF(219,27), NUM(219,27) } }, /* 219 */
    { "so",  { FC_REF_CONSTANT, 1, OFF(220,27), NUM(220,27) } }, /* 220 */
    { "sq",  { FC_REF_CONSTANT, 1, OFF(221,565), NUM(221,565) } }, /* 221 */
    { "sr",  { FC_REF_CONSTANT, 1, OFF(222,566), NUM(222,566) } }, /* 222 */
    { "ss",  { FC_REF_CONSTANT, 1, OFF(223,27), NUM(223,27) } }, /* 223 */
    { "st",  { FC_REF_CONSTANT, 1, OFF(224,27), NUM(224,27) } }, /* 224 */
    { "su",  { FC_REF_CONSTANT, 1, OFF(225,204), NUM(225,204) } }, /* 225 */
    { "sv",  { FC_REF_CONSTANT, 1, OFF(226,567), NUM(226,567) } }, /* 226 */
    { "sw",  { FC_REF_CONSTANT, 1, OFF(227,27), NUM(227,27) } }, /* 227 */
    { "syr",  { FC_REF_CONSTANT, 1, OFF(228,568), NUM(228,568) } }, /* 228 */
    { "szl",  { FC_REF_CONSTANT, 2, OFF(229,569), NUM(229,569) } }, /* 229 */
    { "ta",  { FC_REF_CONSTANT, 1, OFF(230,571), NUM(230,571) } }, /* 230 */
    { "tcy",  { FC_REF_CONSTANT, 1, OFF(231,311), NUM(231,311) } }, /* 231 */
    { "te",  { FC_REF_CONSTANT, 1, OFF(232,572), NUM(232,572) } }, /* 232 */
    { "tg",  { FC_REF_CONSTANT, 1, OFF(233,573), NUM(233,573) } }, /* 233 */
    { "th",  { FC_REF_CONSTANT, 1, OFF(234,574), NUM(234,574) } }, /* 234 */
    { "the",  { FC_REF_CONSTANT, 1, OFF(235,13), NUM(235,13) } }, /* 235 */
    { "ti-er",  { FC_REF_CONSTANT, 2, OFF(236,48), NUM(236,48) } }, /* 236 */
    { "ti-et",  { FC_REF_CONSTANT, 2, OFF(237,552), NUM(237,552) } }, /* 237 */
    { "tig",  { FC_REF_CONSTANT, 2, OFF(238,575), NUM(238,575) } }, /* 238 */
    { "tk",  { FC_REF_CONSTANT, 2, OFF(239,577), NUM(239,577) } }, /* 239 */
    { "tl",  { FC_REF_CONSTANT, 1, OFF(240,173), NUM(240,173) } }, /* 240 */
    { "tn",  { FC_REF_CONSTANT, 2, OFF(241,504), NUM(241,504) } }, /* 241 */
    { "to",  { FC_REF_CONSTANT, 2, OFF(242,556), NUM(242,556) } }, /* 242 */
    { "tpi",  { FC_REF_CONSTANT, 1, OFF(243,162), NUM(243,162) } }, /* 243 */
    { "tr",  { FC_REF_CONSTANT, 2, OFF(244,579), NUM(244,579) } }, /* 244 */
    { "ts",  { FC_REF_CONSTANT, 1, OFF(245,27), NUM(245,27) } }, /* 245 */
    { "tt",  { FC_REF_CONSTANT, 1, OFF(246,581), NUM(246,581) } }, /* 246 */
    { "tw",  { FC_REF_CONSTANT, 5, OFF(247,5), NUM(247,5) } }, /* 247 */
    { "ty",  { FC_REF_CONSTANT, 3, OFF(248,582), NUM(248,582) } }, /* 248 */
    { "tyv",  { FC_REF_CONSTANT, 1, OFF(249,370), NUM(249,370) } }, /* 249 */
    { "ug",  { FC_REF_CONSTANT, 1, OFF(250,585), NUM(250,585) } }, /* 250 */
    { "uk",  { FC_REF_CONSTANT, 1, OFF(251,586), NUM(251,586) } }, /* 251 */
    { "und-zmth",  { FC_REF_CONSTANT, 12, OFF(252,587), NUM(252,587) } }, /* 252 */
    { "und-zsye",  { FC_REF_CONSTANT, 12, OFF(253,599), NUM(253,599) } }, /* 253 */
    { "unm",  { FC_REF_CONSTANT, 1, OFF(254,162), NUM(254,162) } }, /* 254 */
    { "ur",  { FC_REF_CONSTANT, 1, OFF(255,373), NUM(255,373) } }, /* 255 */
    { "uz",  { FC_REF_CONSTANT, 1, OFF(256,27), NUM(256,27) } }, /* 256 */
    { "ve",  { FC_REF_CONSTANT, 2, OFF(257,611), NUM(257,611) } }, /* 257 */
    { "vi",  { FC_REF_CONSTANT, 4, OFF(258,613), NUM(258,613) } }, /* 258 */
    { "vo",  { FC_REF_CONSTANT, 1, OFF(259,617), NUM(259,617) } }, /* 259 */
    { "vot",  { FC_REF_CONSTANT, 2, OFF(260,618), NUM(260,618) } }, /* 260 */
    { "wa",  { FC_REF_CONSTANT, 1, OFF(261,620), NUM(261,620) } }, /* 261 */
    { "wae",  { FC_REF_CONSTANT, 1, OFF(262,154), NUM(262,154) } }, /* 262 */
    { "wal",  { FC_REF_CONSTANT, 2, OFF(263,552), NUM(263,552) } }, /* 263 */
    { "wen",  { FC_REF_CONSTANT, 2, OFF(264,621), NUM(264,621) } }, /* 264 */
    { "wo",  { FC_REF_CONSTANT, 2, OFF(265,623), NUM(265,623) } }, /* 265 */
    { "xh",  { FC_REF_CONSTANT, 1, OFF(266,27), NUM(266,27) } }, /* 266 */
    { "yap",  { FC_REF_CONSTANT, 1, OFF(267,625), NUM(267,625) } }, /* 267 */
    { "yi",  { FC_REF_CONSTANT, 1, OFF(268,194), NUM(268,194) } }, /* 268 */
    { "yo",  { FC_REF_CONSTANT, 4, OFF(269,626), NUM(269,626) } }, /* 269 */
    { "yue",  { FC_REF_CONSTANT, 171, OFF(270,630), NUM(270,630) } }, /* 270 */
    { "yuw",  { FC_REF_CONSTANT, 1, OFF(271,162), NUM(271,162) } }, /* 271 */
    { "za",  { FC_REF_CONSTANT, 1, OFF(272,27), NUM(272,27) } }, /* 272 */
    { "zh-cn",  { FC_REF_CONSTANT, 82, OFF(273,801), NUM(273,801) } }, /* 273 */
    { "zh-hk",  { FC_REF_CONSTANT, 171, OFF(274,630), NUM(274,630) } }, /* 274 */
    { "zh-mo",  { FC_REF_CONSTANT, 171, OFF(275,630), NUM(275,630) } }, /* 275 */
    { "zh-sg",  { FC_REF_CONSTANT, 82, OFF(276,801), NUM(276,801) } }, /* 276 */
    { "zh-tw",  { FC_REF_CONSTANT, 83, OFF(277,56), NUM(277,56) } }, /* 277 */
    { "zu",  { FC_REF_CONSTANT, 1, OFF(278,27), NUM(278,27) } }, /* 278 */
},
{
    { { /* 0 */
    0x00000000, 0x00000000, 0x07fffffe, 0x07fffffe,
    0x00000000, 0x00000000, 0x08104404, 0x08104404,
    } },
    { { /* 1 */
    0xffff8002, 0xffffffff, 0x8002ffff, 0x00000000,
    0xc0000000, 0xf0fc33c0, 0x03000000, 0x00000003,
    } },
    { { /* 2 */
    0x00000000, 0x00000000, 0x07fffffe, 0x07fffffe,
    0x00000000, 0x00000000, 0x0810cf00, 0x0810cf00,
    } },
    { { /* 3 */
    0x00000000, 0x00000000, 0x00000200, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 4 */
    0x00000000, 0x00000000, 0x07fffffe, 0x07fffffe,
    0x00000000, 0x00000000, 0x00000000, 0x04000000,
    } },
    { { /* 5 */
    0x00000000, 0x00000000, 0x07fffffe, 0x07fffffe,
    0x00000000, 0x00000000, 0x00220008, 0x00220008,
    } },
    { { /* 6 */
    0x00000000, 0x00000300, 0x00000000, 0x00000300,
    0x00010040, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 7 */
    0x00000000, 0x00000000, 0x08100000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 8 */
    0x00000048, 0x00000200, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 9 */
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x30000000, 0x00000000, 0x03000000,
    } },
    { { /* 10 */
    0xff7fff7f, 0xff01ff7f, 0x00003d7f, 0xffff7fff,
    0xffff3d7f, 0x003d7fff, 0xff7f7f00, 0x00ff7fff,
    } },
    { { /* 11 */
    0x003d7fff, 0xffffffff, 0x007fff7f, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 12 */
    0x00000000, 0x00000000, 0x07fffffe, 0x07fffffe,
    0x00000000, 0x00000000, 0x140a2202, 0x140a2202,
    } },
    { { /* 13 */
    0xffffffe0, 0x83ffffff, 0x00003fff, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 14 */
    0x00000000, 0x07fffffe, 0x000007fe, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 15 */
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0xfff99fee, 0xd3c4fdff, 0xb000399f, 0x00030000,
    } },
    { { /* 16 */
    0x00000000, 0x00c00030, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 17 */
    0xffff0042, 0xffffffff, 0x0002ffff, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 18 */
    0x00000000, 0x00000000, 0x07fffffe, 0x07fffffe,
    0x00000000, 0x00000000, 0x10028010, 0x10028010,
    } },
    { { /* 19 */
    0x00000000, 0x00000000, 0x07fffffe, 0x07fffffe,
    0x00000000, 0x00000000, 0x00000000, 0x10028010,
    } },
    { { /* 20 */
    0x00000000, 0x00000000, 0x07fffffe, 0x07fffffe,
    0x00000000, 0x00000000, 0x10400080, 0x10400080,
    } },
    { { /* 21 */
    0xc0000000, 0x00030000, 0xc0000000, 0x00000000,
    0x00008000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 22 */
    0x00000000, 0x00000000, 0x02000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 23 */
    0x00000000, 0x07ffffde, 0x001009f6, 0x40000000,
    0x01000040, 0x00008200, 0x00001000, 0x00000000,
    } },
    { { /* 24 */
    0xffff0000, 0xffffffff, 0x0000ffff, 0x00000000,
    0x030c0000, 0x0c00cc0f, 0x03000000, 0x00000300,
    } },
    { { /* 25 */
    0xffff4040, 0xffffffff, 0x4040ffff, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 26 */
    0x00000000, 0x00000000, 0x07fffffe, 0x07fffffe,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 27 */
    0x00003000, 0x00000000, 0x00000000, 0x00000000,
    0x00110000, 0x00000000, 0x00000000, 0x000000c0,
    } },
    { { /* 28 */
    0x00000000, 0x00000000, 0x08000000, 0x00000008,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 29 */
    0x00003000, 0x00000030, 0x00000000, 0x0000300c,
    0x000c0000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 30 */
    0x00000000, 0x3a8b0000, 0x9e78e6b9, 0x0000802e,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 31 */
    0xffff0000, 0xffffd7ff, 0x0000d7ff, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 32 */
    0x00000000, 0x00000000, 0x07fffffe, 0x07fffffe,
    0x00000000, 0x00000000, 0x10008200, 0x10008200,
    } },
    { { /* 33 */
    0x00000000, 0x00000000, 0x07fffffe, 0x07fffffe,
    0x00000000, 0x00000000, 0x060c3303, 0x060c3303,
    } },
    { { /* 34 */
    0x00000003, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 35 */
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x03000000, 0x00003000, 0x00000000,
    } },
    { { /* 36 */
    0x00000000, 0x00000000, 0x00000c00, 0x00000000,
    0x20010040, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 37 */
    0x00000000, 0x00000000, 0x08100000, 0x00040000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 38 */
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0xfff99fee, 0xd3c5fdff, 0xb000399f, 0x00000000,
    } },
    { { /* 39 */
    0x00000000, 0x00000000, 0xfffffeff, 0x3d7e03ff,
    0xfeff0003, 0x03ffffff, 0x00000000, 0x00000000,
    } },
    { { /* 40 */
    0x00000000, 0x00000000, 0x07fffffe, 0x07fffffe,
    0x00000000, 0x00000000, 0x12120404, 0x12120404,
    } },
    { { /* 41 */
    0xfff99fee, 0xf3e5fdff, 0x0007399f, 0x0001ffff,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 42 */
    0x000330c0, 0x00000000, 0x00000000, 0x60000003,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 43 */
    0xffff0002, 0xffffffff, 0x0002ffff, 0x00000000,
    0x00000000, 0x0c00c000, 0x00000000, 0x00000000,
    } },
    { { /* 44 */
    0xff7fff7f, 0xff01ff00, 0x3d7f3d7f, 0xffff7fff,
    0xffff0000, 0x003d7fff, 0xff7f7f3d, 0x00ff7fff,
    } },
    { { /* 45 */
    0x003d7fff, 0xffffffff, 0x007fff00, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 46 */
    0x00000000, 0x00000000, 0x07fffffe, 0x07fffffe,
    0x00000000, 0x00000000, 0x140ca381, 0x140ca381,
    } },
    { { /* 47 */
    0x00000000, 0x80000000, 0x00000001, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 48 */
    0x00000000, 0x00000000, 0x07fffffe, 0x07fffffe,
    0x00000000, 0x00000000, 0x10020004, 0x10020004,
    } },
    { { /* 49 */
    0xffff0002, 0xffffffff, 0x0002ffff, 0x00000000,
    0x00000000, 0x00000030, 0x000c0000, 0x030300c0,
    } },
    { { /* 50 */
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0xffffffff, 0xffffffff, 0x001fffff,
    } },
    { { /* 51 */
    0x00000000, 0x061ef5c0, 0x000001f6, 0x40000000,
    0x01040040, 0x00208210, 0x00005040, 0x00000000,
    } },
    { { /* 52 */
    0xc373ff8b, 0x1b0f6840, 0xf34ce9ac, 0xc0080200,
    0xca3e795c, 0x06487976, 0xf7f02fdf, 0xa8ff033a,
    } },
    { { /* 53 */
    0x233fef37, 0xfd59b004, 0xfffff3ca, 0xfff9de9f,
    0x7df7abff, 0x8eecc000, 0xffdbeebf, 0x45fad003,
    } },
    { { /* 54 */
    0xdffefae1, 0x10abbfef, 0xfcaaffeb, 0x24fdef3f,
    0x7f7678ad, 0xedfff00c, 0x2cfacff6, 0xeb6bf7f9,
    } },
    { { /* 55 */
    0x95bf1ffd, 0xbfbf6677, 0xfeb43bfb, 0x11e27bae,
    0x41bea681, 0x72c31435, 0x71917d70, 0x276b0003,
    } },
    { { /* 56 */
    0x70cf57cb, 0x0def4732, 0xfc747eda, 0xbdb4fe06,
    0x8bca3f9f, 0x58007e49, 0xebec228f, 0xddbb8a5c,
    } },
    { { /* 57 */
    0xb6e7ef60, 0xf293a40f, 0x549e37bb, 0x9bafd04b,
    0xf7d4c414, 0x0a1430b0, 0x88d02f08, 0x192fff7e,
    } },
    { { /* 58 */
    0xfb07ffda, 0x7beb7ff1, 0x0010c5ef, 0xfdff99ff,
    0x056779d7, 0xfdcbffe7, 0x4040c3ff, 0xbd8e6ff7,
    } },
    { { /* 59 */
    0x0497dffa, 0x5bfff4c0, 0xd0e7ed7b, 0xf8e0047e,
    0xb73eff9f, 0x882e7dfe, 0xbe7ffffd, 0xf6c483fe,
    } },
    { { /* 60 */
    0xb8fdf357, 0xef7dd680, 0x47885767, 0xc3dfff7d,
    0x37a9f0ff, 0x70fc7de0, 0xec9a3f6f, 0x86814cb3,
    } },
    { { /* 61 */
    0xdd5c3f9e, 0x4819f70d, 0x0007fea3, 0x38ffaf56,
    0xefb8980d, 0xb760403d, 0x9035d8ce, 0x3fff72bf,
    } },
    { { /* 62 */
    0x7a117ff7, 0xabfff7bb, 0x6fbeff00, 0xfe72a93c,
    0xf11bcfef, 0xf40adb6b, 0xef7ec3e6, 0xf6109b9c,
    } },
    { { /* 63 */
    0x16f4f048, 0x5182feb5, 0x15bbc7b1, 0xfbdf6e87,
    0x63cde43f, 0x7e7ec1ff, 0x7d5ffdeb, 0xfcfe777b,
    } },
    { { /* 64 */
    0xdbea960b, 0x53e86229, 0xfdef37df, 0xbd8136f5,
    0xfcbddc18, 0xffffd2e4, 0xffe03fd7, 0xabf87f6f,
    } },
    { { /* 65 */
    0x6ed99bae, 0xf115f5fb, 0xbdfb79a9, 0xadaf5a3c,
    0x1facdbba, 0x837971fc, 0xc35f7cf7, 0x0567dfff,
    } },
    { { /* 66 */
    0x8467ff9a, 0xdf8b1534, 0x3373f9f3, 0x5e1af7bd,
    0xa03fbf40, 0x01ebffff, 0xcfdddfc0, 0xabd37500,
    } },
    { { /* 67 */
    0xeed6f8c3, 0xb7ff43fd, 0x42275eaf, 0xf6869bac,
    0xf6bc27d7, 0x35b7f787, 0xe176aacd, 0xe29f49e7,
    } },
    { { /* 68 */
    0xaff2545c, 0x61d82b3f, 0xbbb8fc3b, 0x7b7dffcf,
    0x1ce0bf95, 0x43ff7dfd, 0xfffe5ff6, 0xc4ced3ef,
    } },
    { { /* 69 */
    0xadbc8db6, 0x11eb63dc, 0x23d0df59, 0xf3dbbeb4,
    0xdbc71fe7, 0xfae4ff63, 0x63f7b22b, 0xadbaed3b,
    } },
    { { /* 70 */
    0x7efffe01, 0x02bcfff7, 0xef3932ff, 0x8005fffc,
    0xbcf577fb, 0xfff7010d, 0xbf3afffb, 0xdfff0057,
    } },
    { { /* 71 */
    0xbd7def7b, 0xc8d4db88, 0xed7cfff3, 0x56ff5dee,
    0xac5f7e0d, 0xd57fff96, 0xc1403fee, 0xffe76ff9,
    } },
    { { /* 72 */
    0x8e77779b, 0xe45d6ebf, 0x5f1f6fcf, 0xfedfe07f,
    0x01fed7db, 0xfb7bff00, 0x1fdfffd4, 0xfffff800,
    } },
    { { /* 73 */
    0x007bfb8f, 0x7f5cbf00, 0x07f3ffff, 0x3de7eba0,
    0xfbd7f7bf, 0x6003ffbf, 0xbfedfffd, 0x027fefbb,
    } },
    { { /* 74 */
    0xddfdfe40, 0xe2f9fdff, 0xfb1f680b, 0xaffdfbe3,
    0xf7ed9fa4, 0xf80f7a7d, 0x0fd5eebe, 0xfd9fbb5d,
    } },
    { { /* 75 */
    0x3bf9f2db, 0xebccfe7f, 0x73fa876a, 0x9ffc95fc,
    0xfaf7109f, 0xbbcdddb7, 0xeccdf87e, 0x3c3ff366,
    } },
    { { /* 76 */
    0xb03ffffd, 0x067ee9f7, 0xfe0696ae, 0x5fd7d576,
    0xa3f33fd1, 0x6fb7cf07, 0x7f449fd1, 0xd3dd7b59,
    } },
    { { /* 77 */
    0xa9bdaf3b, 0xff3a7dcf, 0xf6ebfbe0, 0xffffb401,
    0xb7bf7afa, 0x0ffdc000, 0xff1fff7f, 0x95fffefc,
    } },
    { { /* 78 */
    0xb5dc0000, 0x3f3eef63, 0x001bfb7f, 0xfbf6e800,
    0xb8df9eef, 0x003fff9f, 0xf5ff7bd0, 0x3fffdfdb,
    } },
    { { /* 79 */
    0x00bffdf0, 0xbbbd8420, 0xffdedf37, 0x0ff3ff6d,
    0x5efb604c, 0xfafbfffb, 0x0219fe5e, 0xf9de79f4,
    } },
    { { /* 80 */
    0xebfaa7f7, 0xff3401eb, 0xef73ebd3, 0xc040afd7,
    0xdcff72bb, 0x2fd8f17f, 0xfe0bb8ec, 0x1f0bdda3,
    } },
    { { /* 81 */
    0x47cf8f1d, 0xffdeb12b, 0xda737fee, 0xcbc424ff,
    0xcbf2f75d, 0xb4edecfd, 0x4dddbff9, 0xfb8d99dd,
    } },
    { { /* 82 */
    0xaf7bbb7f, 0xc959ddfb, 0xfab5fc4f, 0x6d5fafe3,
    0x3f7dffff, 0xffdb7800, 0x7effb6ff, 0x022ffbaf,
    } },
    { { /* 83 */
    0xefc7ff9b, 0xffffffa5, 0xc7000007, 0xfff1f7ff,
    0x01bf7ffd, 0xfdbcdc00, 0xffffbff5, 0x3effff7f,
    } },
    { { /* 84 */
    0xbe000029, 0xff7ff9ff, 0xfd7e6efb, 0x039ecbff,
    0xfbdde300, 0xf6dfccff, 0x117fffff, 0xfbf6f800,
    } },
    { { /* 85 */
    0xd73ce7ef, 0xdfeffeef, 0xedbfc00b, 0xfdcdfedf,
    0x40fd7bf5, 0xb75fffff, 0xf930ffdf, 0xdc97fbdf,
    } },
    { { /* 86 */
    0xbff2fef3, 0xdfbf8fdf, 0xede6177f, 0x35530f7f,
    0x877e447c, 0x45bbfa12, 0x779eede0, 0xbfd98017,
    } },
    { { /* 87 */
    0xde897e55, 0x0447c16f, 0xf75d7ade, 0x290557ff,
    0xfe9586f7, 0xf32f97b3, 0x9f75cfff, 0xfb1771f7,
    } },
    { { /* 88 */
    0xee1934ee, 0xef6137cc, 0xef4c9fd6, 0xfbddd68f,
    0x6def7b73, 0xa431d7fe, 0x97d75e7f, 0xffd80f5b,
    } },
    { { /* 89 */
    0x7bce9d83, 0xdcff22ec, 0xef87763d, 0xfdeddfe7,
    0xa0fc4fff, 0xdbfc3b77, 0x7fdc3ded, 0xf5706fa9,
    } },
    { { /* 90 */
    0x2c403ffb, 0x847fff7f, 0xdeb7ec57, 0xf22fe69c,
    0xd5b50feb, 0xede7afeb, 0xfff08c2f, 0xe8f0537f,
    } },
    { { /* 91 */
    0xb5ffb99d, 0xe78fff66, 0xbe10d981, 0xe3c19c7c,
    0x27339cd1, 0xff6d0cbc, 0xefb7fcb7, 0xffffa0df,
    } },
    { { /* 92 */
    0xfe7bbf0b, 0x353fa3ff, 0x97cd13cc, 0xfb277637,
    0x7e6ccfd6, 0xed31ec50, 0xfc1c677c, 0x5fbff6fa,
    } },
    { { /* 93 */
    0xae2f0fba, 0x7ffea3ad, 0xde74fcf0, 0xf200ffef,
    0xfea2fbbf, 0xbcff3daf, 0x5fb9f694, 0x3f8ff3ad,
    } },
    { { /* 94 */
    0xa01ff26c, 0x01bfffef, 0x70057728, 0xda03ff35,
    0xc7fad2f9, 0x5c1d3fbf, 0xec33ff3a, 0xfe9cb7af,
    } },
    { { /* 95 */
    0x7a9f5236, 0xe722bffa, 0xfcff9ff7, 0xb61d2fbb,
    0x1dfded06, 0xefdf7dd7, 0xf166eb23, 0x0dc07ed9,
    } },
    { { /* 96 */
    0xdfbf3d3d, 0xba83c945, 0x9dd07dd1, 0xcf737b87,
    0xc3f59ff3, 0xc5fedf0d, 0x83020cb3, 0xaec0e879,
    } },
    { { /* 97 */
    0x6f0fc773, 0x093ffd7d, 0x0157fff1, 0x01ff62fb,
    0x3bf3fdb4, 0x43b2b013, 0xff305ed3, 0xeb9f0fff,
    } },
    { { /* 98 */
    0xf203feef, 0xfb893fef, 0x9e9937a9, 0xa72cdef9,
    0xc1f63733, 0xfe3e812e, 0xf2f75d20, 0x69d7d585,
    } },
    { { /* 99 */
    0xffffffff, 0xff6fdb07, 0xd97fc4ff, 0xbe0fefce,
    0xf05ef17b, 0xffb7f6cf, 0xef845ef7, 0x0edfd7cb,
    } },
    { { /* 100 */
    0xfcffff08, 0xffffee3f, 0xd7ff13ff, 0x7ffdaf0f,
    0x1ffabdc7, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 101 */
    0x00000000, 0xe7400000, 0xf933bd38, 0xfeed7feb,
    0x7c767fe8, 0xffefb3f7, 0xd8b7feaf, 0xfbbfff6f,
    } },
    { { /* 102 */
    0xdbf7f8fb, 0xe2f91752, 0x754785c8, 0xe3ef9090,
    0x3f6d9ef4, 0x0536ee2e, 0x7ff3f7bc, 0x7f3fa07b,
    } },
    { { /* 103 */
    0xeb600567, 0x6601babe, 0x583ffcd8, 0x87dfcaf7,
    0xffa0bfcd, 0xfebf5bcd, 0xefa7b6fd, 0xdf9c77ef,
    } },
    { { /* 104 */
    0xf8773fb7, 0xb7fc9d27, 0xdfefcab5, 0xf1b6fb5a,
    0xef1fec39, 0x7ffbfbbf, 0xdafe000d, 0x4e7fbdfb,
    } },
    { { /* 105 */
    0x5ac033ff, 0x9ffebff5, 0x005fffbf, 0xfdf80000,
    0x6ffdffca, 0xa001cffd, 0xfbf2dfff, 0xff7fdfbf,
    } },
    { { /* 106 */
    0x080ffeda, 0xbfffba08, 0xeed77afd, 0x67f9fbeb,
    0xff93e044, 0x9f57df97, 0x08dffef7, 0xfedfdf80,
    } },
    { { /* 107 */
    0xf7feffc5, 0x6803fffb, 0x6bfa67fb, 0x5fe27fff,
    0xff73ffff, 0xe7fb87df, 0xf7a7ebfd, 0xefc7bf7e,
    } },
    { { /* 108 */
    0xdf821ef3, 0xdf7e76ff, 0xda7d79c9, 0x1e9befbe,
    0x77fb7ce0, 0xfffb87be, 0xffdb1bff, 0x4fe03f5c,
    } },
    { { /* 109 */
    0x5f0e7fff, 0xddbf77ff, 0xfffff04f, 0x0ff8ffff,
    0xfddfa3be, 0xfffdfc1c, 0xfb9e1f7d, 0xdedcbdff,
    } },
    { { /* 110 */
    0xbafb3f6f, 0xfbefdf7f, 0x2eec7d1b, 0xf2f7af8e,
    0xcfee7b0f, 0x77c61d96, 0xfff57e07, 0x7fdfd982,
    } },
    { { /* 111 */
    0xc7ff5ee6, 0x79effeee, 0xffcf9a56, 0xde5efe5f,
    0xf9e8896e, 0xe6c4f45e, 0xbe7c0001, 0xdddf3b7f,
    } },
    { { /* 112 */
    0xe9efd59d, 0xde5334ac, 0x4bf7f573, 0x9eff7b4f,
    0x476eb8fe, 0xff450dfb, 0xfbfeabfd, 0xddffe9d7,
    } },
    { { /* 113 */
    0x7fffedf7, 0x7eebddfd, 0xb7ffcfe7, 0xef91bde9,
    0xd77c5d75, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 114 */
    0x00000000, 0xfa800000, 0xb4f1ffee, 0x2fefbf76,
    0x77bfb677, 0xfffd9fbf, 0xf6ae95bf, 0x7f3b75ff,
    } },
    { { /* 115 */
    0x0af9a7f5, 0x00000000, 0x00000000, 0x2bddfbd0,
    0x9a7ff633, 0xd6fcfdab, 0xbfebf9e6, 0xf41fdfdf,
    } },
    { { /* 116 */
    0xffffa6fd, 0xf37b4aff, 0xfef97fb7, 0x1d5cb6ff,
    0xe5ff7ff6, 0x24041f7b, 0xf99ebe05, 0xdff2dbe3,
    } },
    { { /* 117 */
    0xfdff6fef, 0xcbfcd679, 0xefffebfd, 0x0000001f,
    0x98000000, 0x8017e148, 0x00fe6a74, 0xfdf16d7f,
    } },
    { { /* 118 */
    0xfef3b87f, 0xf176e01f, 0x7b3fee96, 0xfffdeb8d,
    0xcbb3adff, 0xe17f84ef, 0xbff04daa, 0xfe3fbf3f,
    } },
    { { /* 119 */
    0xffd7ebff, 0xcf7fffdf, 0x85edfffb, 0x07bcd73f,
    0xfe0faeff, 0x76bffdaf, 0x37bbfaef, 0xa3ba7fdc,
    } },
    { { /* 120 */
    0x56f7b6ff, 0xe7df60f8, 0x4cdfff61, 0xff45b0fb,
    0x3ffa7ded, 0x18fc1fff, 0xe3afffff, 0xdf83c7d3,
    } },
    { { /* 121 */
    0xef7dfb57, 0x1378efff, 0x5ff7fec0, 0x5ee334bb,
    0xeff6f70d, 0x00bfd7fe, 0xf7f7f59d, 0xffe051de,
    } },
    { { /* 122 */
    0x037ffec9, 0xbfef5f01, 0x60a79ff1, 0xf1ffef1d,
    0x0000000f, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 123 */
    0x00000000, 0x00000000, 0x00000000, 0x3c800000,
    0xd91ffb4d, 0xfee37b3a, 0xdc7f3fe9, 0x0000003f,
    } },
    { { /* 124 */
    0x50000000, 0xbe07f51f, 0xf91bfc1d, 0x71ffbc1e,
    0x5bbe6ff9, 0x9b1b5796, 0xfffc7fff, 0xafe7872e,
    } },
    { { /* 125 */
    0xf34febf5, 0xe725dffd, 0x5d440bdc, 0xfddd5747,
    0x7790ed3f, 0x8ac87d7f, 0xf3f9fafa, 0xef4b202a,
    } },
    { { /* 126 */
    0x79cff5ff, 0x0ba5abd3, 0xfb8ff77a, 0x001f8ebd,
    0x00000000, 0xfd4ef300, 0x88001a57, 0x7654aeac,
    } },
    { { /* 127 */
    0xcdff17ad, 0xf42fffb2, 0xdbff5baa, 0x00000002,
    0x73c00000, 0x2e3ff9ea, 0xbbfffa8e, 0xffd376bc,
    } },
    { { /* 128 */
    0x7e72eefe, 0xe7f77ebd, 0xcefdf77f, 0x00000ff5,
    0x00000000, 0xdb9ba900, 0x917fa4c7, 0x7ecef8ca,
    } },
    { { /* 129 */
    0xc7e77d7a, 0xdcaecbbd, 0x8f76fd7e, 0x7cf391d3,
    0x4c2f01e5, 0xa360ed77, 0x5ef807db, 0x21811df7,
    } },
    { { /* 130 */
    0x309c6be0, 0xfade3b3a, 0xc3f57f53, 0x07ba61cd,
    0x00000000, 0x00000000, 0x00000000, 0xbefe26e0,
    } },
    { { /* 131 */
    0xebb503f9, 0xe9cbe36d, 0xbfde9c2f, 0xabbf9f83,
    0xffd51ff7, 0xdffeb7df, 0xffeffdae, 0xeffdfb7e,
    } },
    { { /* 132 */
    0x6ebfaaff, 0x00000000, 0x00000000, 0xb6200000,
    0xbe9e7fcd, 0x58f162b3, 0xfd7bf10d, 0xbefde9f1,
    } },
    { { /* 133 */
    0x5f6dc6c3, 0x69ffff3d, 0xfbf4ffcf, 0x4ff7dcfb,
    0x11372000, 0x00000015, 0x00000000, 0x00000000,
    } },
    { { /* 134 */
    0x00003000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 135 */
    0x00000000, 0x00000000, 0x07fffffe, 0x07fffffe,
    0x00000000, 0x00000000, 0x1a10cfc5, 0x9a10cfc5,
    } },
    { { /* 136 */
    0x00000000, 0x00000000, 0x000c0000, 0x01000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 137 */
    0x00000000, 0x00000000, 0x07fffffe, 0x07fffffe,
    0x00000000, 0x00000000, 0x10420084, 0x10420084,
    } },
    { { /* 138 */
    0xc0000000, 0x00030000, 0xc0000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 139 */
    0x00000000, 0x00000000, 0x07fffffe, 0x07fffffe,
    0x00000000, 0x00000000, 0x24082202, 0x24082202,
    } },
    { { /* 140 */
    0x0c00f000, 0x00000000, 0x03000180, 0x6000c033,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 141 */
    0x00000000, 0x00000000, 0x07fffffe, 0x07fffffe,
    0x00000000, 0x00000000, 0x021c0a08, 0x021c0a08,
    } },
    { { /* 142 */
    0x00000030, 0x00000000, 0x0000001e, 0x18000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 143 */
    0xfdffa966, 0xffffdfff, 0xa965dfff, 0x03ffffff,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 144 */
    0x0000000c, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 145 */
    0xffff0002, 0xffffffff, 0x0002ffff, 0x00000000,
    0x00000000, 0x00000c00, 0x00c00000, 0x000c0000,
    } },
    { { /* 146 */
    0x00000000, 0x00000000, 0x07fffffe, 0x07fffffe,
    0x00000000, 0x00000000, 0x0010c604, 0x8010c604,
    } },
    { { /* 147 */
    0x00000000, 0x00000000, 0x00000000, 0x01f00000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 148 */
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x0000003f, 0x00000000, 0x00000000, 0x000c0000,
    } },
    { { /* 149 */
    0x00000000, 0x00000000, 0x07fffffe, 0x07fffffe,
    0x00000000, 0x00000000, 0x25082262, 0x25082262,
    } },
    { { /* 150 */
    0x00000000, 0x00000000, 0x07fffffe, 0x07fffffe,
    0x00000000, 0x00000000, 0x90400010, 0x10400010,
    } },
    { { /* 151 */
    0xfff99fec, 0xf3e5fdff, 0xf807399f, 0x0000ffff,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 152 */
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0xffffffff, 0x0001ffff, 0x00000000, 0x00000000,
    } },
    { { /* 153 */
    0x0c000000, 0x00000000, 0x00000c00, 0x00000000,
    0x00170240, 0x00040000, 0x001fe000, 0x00000000,
    } },
    { { /* 154 */
    0x00000000, 0x00000000, 0x08500000, 0x00000008,
    0x00000800, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 155 */
    0x00001003, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 156 */
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0xffffd740, 0xfffffffb, 0x00007fff, 0x00000000,
    } },
    { { /* 157 */
    0x00000000, 0x00000000, 0x07fffffe, 0x07fffffe,
    0x00000000, 0x00000000, 0x00528f81, 0x00528f81,
    } },
    { { /* 158 */
    0x30000300, 0x00300030, 0x30000000, 0x00003000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 159 */
    0x00000000, 0x00000000, 0x07fffffe, 0x07fffffe,
    0x00000000, 0x00000000, 0x10600010, 0x10600010,
    } },
    { { /* 160 */
    0x00000000, 0x00000000, 0x00000000, 0x60000003,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 161 */
    0x00000000, 0x00000000, 0x07fffffe, 0x07fffffe,
    0x00000000, 0x00000000, 0x10020000, 0x10020000,
    } },
    { { /* 162 */
    0x00000000, 0x00000000, 0x00000c00, 0x00000000,
    0x20000402, 0x00180000, 0x00000000, 0x00000000,
    } },
    { { /* 163 */
    0x00000000, 0x00000000, 0x00880000, 0x00040000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 164 */
    0x00000000, 0x00000000, 0x07fffffe, 0x07fffffe,
    0x00000000, 0x00000000, 0x00400030, 0x00400030,
    } },
    { { /* 165 */
    0x00000000, 0x00000000, 0x07fffffe, 0x07fffffe,
    0x00000000, 0x00000000, 0x0e1e7707, 0x0e1e7707,
    } },
    { { /* 166 */
    0x00000000, 0x00000000, 0x07fffffe, 0x07fffffe,
    0x00000000, 0x00000000, 0x25092042, 0x25092042,
    } },
    { { /* 167 */
    0x00000000, 0x00000000, 0x07fffffe, 0x07fffffe,
    0x00000000, 0x00000000, 0x02041107, 0x02041107,
    } },
    { { /* 168 */
    0x00000000, 0x00000000, 0x07fffffe, 0x07fffffe,
    0x00000000, 0x00000000, 0x9c508e14, 0x1c508e14,
    } },
    { { /* 169 */
    0x00000000, 0x00000000, 0x07fffffe, 0x07fffffe,
    0x00000000, 0x00000000, 0x04082202, 0x04082202,
    } },
    { { /* 170 */
    0x00000c00, 0x00000003, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 171 */
    0xc0000c0c, 0x00000000, 0x00c00003, 0x00000c03,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 172 */
    0x00000000, 0x00000000, 0x07fffffe, 0x07fffffe,
    0x00000000, 0x00000000, 0x020c1383, 0x020c1383,
    } },
    { { /* 173 */
    0xff7fff7f, 0xff01ff7f, 0x00003d7f, 0x00ff00ff,
    0x00ff3d7f, 0x003d7fff, 0xff7f7f00, 0x00ff7f00,
    } },
    { { /* 174 */
    0x003d7f00, 0xffff01ff, 0x007fff7f, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 175 */
    0x00000000, 0x00000000, 0x07fffffe, 0x07fffffe,
    0x00000000, 0x00000000, 0x040a2202, 0x042a220a,
    } },
    { { /* 176 */
    0x00000000, 0x00000200, 0x00000000, 0x00000200,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 177 */
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x20000000, 0x00000000, 0x02000000,
    } },
    { { /* 178 */
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0xfffbafee, 0xf3edfdff, 0x00013bbf, 0x00000001,
    } },
    { { /* 179 */
    0x00000000, 0x00000000, 0x07fffffe, 0x07fffffe,
    0x00000000, 0x00000000, 0x00000080, 0x00000080,
    } },
    { { /* 180 */
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x03000402, 0x00180000, 0x00000000, 0x00000000,
    } },
    { { /* 181 */
    0x00000000, 0x00000000, 0x00880000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 182 */
    0x000c0003, 0x00000c00, 0x00003000, 0x00000c00,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 183 */
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x08000000, 0x00000000, 0x00000000,
    } },
    { { /* 184 */
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0xffff0000, 0x000007ff,
    } },
    { { /* 185 */
    0x00000000, 0x00000000, 0x07fffffe, 0x07fffffe,
    0x00000000, 0x00000000, 0x00080000, 0x00080000,
    } },
    { { /* 186 */
    0x0c0030c0, 0x00000000, 0x0300001e, 0x66000003,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 187 */
    0x00000000, 0x00000000, 0x07fffffe, 0x07fffffe,
    0x00000000, 0x00000000, 0x00040100, 0x00040100,
    } },
    { { /* 188 */
    0x00000000, 0x00000000, 0x07fffffe, 0x07fffffe,
    0x00000000, 0x00000000, 0x14482202, 0x14482202,
    } },
    { { /* 189 */
    0x00000000, 0x00000000, 0x00030000, 0x00030000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 190 */
    0x00000000, 0xfffe0000, 0x007fffff, 0xfffffffe,
    0x000000ff, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 191 */
    0x00000000, 0x00008000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 192 */
    0x000c0000, 0x00000000, 0x00000c00, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 193 */
    0x00000000, 0x00000000, 0x07fffffe, 0x07fffffe,
    0x00000000, 0x00000000, 0x00000200, 0x00000200,
    } },
    { { /* 194 */
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00003c00, 0x00000030,
    } },
    { { /* 195 */
    0xffffffff, 0xffffffff, 0xffffffff, 0xffffffff,
    0xffffffff, 0xffffffff, 0xffffffff, 0xffffffff,
    } },
    { { /* 196 */
    0xffffffff, 0xffffffff, 0xffffffff, 0xffffffff,
    0x00001fff, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 197 */
    0xffff4002, 0xffffffff, 0x4002ffff, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 198 */
    0x00000000, 0x00000000, 0x07fffffe, 0x07fffffe,
    0x00000000, 0x00000000, 0x64092242, 0x64092242,
    } },
    { { /* 199 */
    0x00000000, 0x00000000, 0x07fffffe, 0x07fffffe,
    0x00000000, 0x00000000, 0x060cb301, 0x060cb301,
    } },
    { { /* 200 */
    0x00000c7e, 0x031f8000, 0x0063f200, 0x000df840,
    0x00037e08, 0x08000dfa, 0x0df901bf, 0x5437e400,
    } },
    { { /* 201 */
    0x00000025, 0x40006fc0, 0x27f91be4, 0xdee00000,
    0x007ff83f, 0x00007f7f, 0x00000000, 0x00000000,
    } },
    { { /* 202 */
    0x00000000, 0x00000000, 0x00000000, 0x007f8000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 203 */
    0x000000a7, 0x00000000, 0xfffffffe, 0xffffffff,
    0x780fffff, 0xfffffffe, 0xffffffff, 0x787fffff,
    } },
    { { /* 204 */
    0x03506f8b, 0x1b042042, 0x62808020, 0x400a0000,
    0x10341b41, 0x04003812, 0x03608c02, 0x08454038,
    } },
    { { /* 205 */
    0x2403c002, 0x15108000, 0x1229e040, 0x80280000,
    0x28002800, 0x8060c002, 0x2080040c, 0x05284002,
    } },
    { { /* 206 */
    0x82042a00, 0x02000818, 0x10008200, 0x20700020,
    0x03022000, 0x40a41000, 0x0420a020, 0x00000080,
    } },
    { { /* 207 */
    0x80040011, 0x00000400, 0x04012b78, 0x11a23920,
    0x02842460, 0x00c01021, 0x20002050, 0x07400042,
    } },
    { { /* 208 */
    0x208205c9, 0x0fc10230, 0x08402480, 0x00258018,
    0x88000080, 0x42120609, 0xa32002a8, 0x40040094,
    } },
    { { /* 209 */
    0x00c00024, 0x8e000001, 0x059e058a, 0x013b0001,
    0x85000010, 0x08080000, 0x02d07d04, 0x018d9838,
    } },
    { { /* 210 */
    0x8803f310, 0x03000840, 0x00000704, 0x30080500,
    0x00001000, 0x20040000, 0x00000003, 0x04040002,
    } },
    { { /* 211 */
    0x000100d0, 0x40028000, 0x00088040, 0x00000000,
    0x34000210, 0x00400e00, 0x00000020, 0x00000008,
    } },
    { { /* 212 */
    0x00000040, 0x00060000, 0x00000000, 0x00100100,
    0x00000080, 0x00000000, 0x4c000000, 0x240d0009,
    } },
    { { /* 213 */
    0x80048000, 0x00010180, 0x00020484, 0x00000400,
    0x00000804, 0x00000008, 0x80004800, 0x16800000,
    } },
    { { /* 214 */
    0x00200065, 0x00120410, 0x44920403, 0x40000200,
    0x10880008, 0x40080100, 0x00001482, 0x00074800,
    } },
    { { /* 215 */
    0x14608200, 0x00024e84, 0x00128380, 0x20184520,
    0x0240041c, 0x0a001120, 0x00180a00, 0x88000800,
    } },
    { { /* 216 */
    0x01000002, 0x00008001, 0x04000040, 0x80000040,
    0x08040000, 0x00000000, 0x00001202, 0x00000002,
    } },
    { { /* 217 */
    0x00000000, 0x00000004, 0x21910000, 0x00000858,
    0xbf8013a0, 0x8279401c, 0xa8041054, 0xc5004282,
    } },
    { { /* 218 */
    0x0402ce56, 0xfc020000, 0x40200d21, 0x00028030,
    0x00010000, 0x01081202, 0x00000000, 0x00410003,
    } },
    { { /* 219 */
    0x00404080, 0x00000200, 0x00010000, 0x00000000,
    0x00000000, 0x00000000, 0x60000000, 0x480241ea,
    } },
    { { /* 220 */
    0x2000104c, 0x2109a820, 0x00200020, 0x7b1c0008,
    0x10a0840a, 0x01c028c0, 0x00000608, 0x04c00000,
    } },
    { { /* 221 */
    0x80398412, 0x40a200e0, 0x02080000, 0x12030a04,
    0x008d1833, 0x02184602, 0x13803028, 0x00200801,
    } },
    { { /* 222 */
    0x20440000, 0x000005a1, 0x00050800, 0x0020a328,
    0x80100000, 0x10040649, 0x10020020, 0x00090180,
    } },
    { { /* 223 */
    0x8c008202, 0x00000000, 0x00205910, 0x0041410c,
    0x00004004, 0x40441290, 0x00010080, 0x01040000,
    } },
    { { /* 224 */
    0x04070000, 0x89108040, 0x00282a81, 0x82420000,
    0x51a20411, 0x32220800, 0x2b0d2220, 0x40c83003,
    } },
    { { /* 225 */
    0x82020082, 0x80008900, 0x10a00200, 0x08004100,
    0x09041108, 0x000405a6, 0x0c018000, 0x04104002,
    } },
    { { /* 226 */
    0x00002000, 0x44003000, 0x01000004, 0x00008200,
    0x00000008, 0x00044010, 0x00002002, 0x00001040,
    } },
    { { /* 227 */
    0x00000000, 0xca008000, 0x02828020, 0x00b1100c,
    0x12824280, 0x22013030, 0x00808820, 0x040013e4,
    } },
    { { /* 228 */
    0x801840c0, 0x1000a1a1, 0x00000004, 0x0050c200,
    0x00c20082, 0x00104840, 0x10400080, 0xa3140000,
    } },
    { { /* 229 */
    0xa8a02301, 0x24123d00, 0x80030200, 0xc0028022,
    0x34a10000, 0x00408005, 0x00190010, 0x882a0000,
    } },
    { { /* 230 */
    0x00080018, 0x33000402, 0x9002010a, 0x00000000,
    0x00800020, 0x00010100, 0x84040810, 0x04004000,
    } },
    { { /* 231 */
    0x10006020, 0x00000000, 0x00000000, 0x30a02000,
    0x00000004, 0x00000000, 0x01000800, 0x20000000,
    } },
    { { /* 232 */
    0x02000000, 0x02000602, 0x80000800, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 233 */
    0x00000010, 0x44040083, 0x00081000, 0x0818824c,
    0x00400e00, 0x8c300000, 0x08146001, 0x00000000,
    } },
    { { /* 234 */
    0x00828000, 0x41900000, 0x84804006, 0x24010001,
    0x02400108, 0x9b080006, 0x00201602, 0x0009012e,
    } },
    { { /* 235 */
    0x40800800, 0x48000420, 0x10000032, 0x01904440,
    0x02000100, 0x10048000, 0x00020000, 0x08820802,
    } },
    { { /* 236 */
    0x08080ba0, 0x00009242, 0x00400000, 0xc0008080,
    0x20410001, 0x04400000, 0x60020820, 0x00100000,
    } },
    { { /* 237 */
    0x00108046, 0x01001805, 0x90100000, 0x00014010,
    0x00000010, 0x00000000, 0x0000000b, 0x00008800,
    } },
    { { /* 238 */
    0x00000000, 0x00001000, 0x00000000, 0x20018800,
    0x00004600, 0x06002000, 0x00000100, 0x00000000,
    } },
    { { /* 239 */
    0x00000000, 0x10400042, 0x02004000, 0x00004280,
    0x80000400, 0x00020000, 0x00000008, 0x00000020,
    } },
    { { /* 240 */
    0x00000040, 0x20600400, 0x0a000180, 0x02040280,
    0x00000000, 0x00409001, 0x02000004, 0x00003200,
    } },
    { { /* 241 */
    0x88000000, 0x80404800, 0x00000010, 0x00040008,
    0x00000a90, 0x00000200, 0x00002000, 0x40002001,
    } },
    { { /* 242 */
    0x00000048, 0x00100000, 0x00000000, 0x00000001,
    0x00000008, 0x20010080, 0x00000000, 0x00400040,
    } },
    { { /* 243 */
    0x85000000, 0x0c8f0108, 0x32129000, 0x80090420,
    0x00024000, 0x40040800, 0x092000a0, 0x00100204,
    } },
    { { /* 244 */
    0x00002000, 0x00000000, 0x00440004, 0x6c000000,
    0x000000d0, 0x80004000, 0x88800440, 0x41144018,
    } },
    { { /* 245 */
    0x80001a02, 0x14000001, 0x00000001, 0x0000004a,
    0x00000000, 0x00083000, 0x08000000, 0x0008a024,
    } },
    { { /* 246 */
    0x00300004, 0x00140000, 0x20000000, 0x00001800,
    0x00020002, 0x04000000, 0x00000002, 0x00000100,
    } },
    { { /* 247 */
    0x00004002, 0x54000000, 0x60400300, 0x00002120,
    0x0000a022, 0x00000000, 0x81060803, 0x08010200,
    } },
    { { /* 248 */
    0x04004800, 0xb0044000, 0x0000a005, 0x04500800,
    0x800c000a, 0x0000c000, 0x10000800, 0x02408021,
    } },
    { { /* 249 */
    0x08020000, 0x00001040, 0x00540a40, 0x00000000,
    0x00800880, 0x01020002, 0x00000211, 0x00000010,
    } },
    { { /* 250 */
    0x00000000, 0x80000002, 0x00002000, 0x00080001,
    0x09840a00, 0x40000080, 0x00400000, 0x49000080,
    } },
    { { /* 251 */
    0x0e102831, 0x06098807, 0x40011014, 0x02620042,
    0x06000000, 0x88062000, 0x04068400, 0x08108301,
    } },
    { { /* 252 */
    0x08000012, 0x40004840, 0x00300402, 0x00012000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 253 */
    0x00000000, 0x00400000, 0x00000000, 0x00a54400,
    0x40004420, 0x20000310, 0x00041002, 0x18000000,
    } },
    { { /* 254 */
    0x00a1002a, 0x00080000, 0x40400000, 0x00900000,
    0x21401200, 0x04048626, 0x40005048, 0x21100000,
    } },
    { { /* 255 */
    0x040005a4, 0x000a0000, 0x00214000, 0x07010800,
    0x34000000, 0x00080100, 0x00080040, 0x10182508,
    } },
    { { /* 256 */
    0xc0805100, 0x02c01400, 0x00000080, 0x00448040,
    0x20000800, 0x210a8000, 0x08800000, 0x00020060,
    } },
    { { /* 257 */
    0x00004004, 0x00400100, 0x01040200, 0x00800000,
    0x00000000, 0x00000000, 0x10081400, 0x00008000,
    } },
    { { /* 258 */
    0x00004000, 0x20000000, 0x08800200, 0x00001000,
    0x00000000, 0x01000000, 0x00000810, 0x00000000,
    } },
    { { /* 259 */
    0x00020000, 0x20200000, 0x00000000, 0x00000000,
    0x00000010, 0x00001c40, 0x00002000, 0x08000210,
    } },
    { { /* 260 */
    0x00000000, 0x00000000, 0x54014000, 0x02000800,
    0x00200400, 0x00000000, 0x00002080, 0x00004000,
    } },
    { { /* 261 */
    0x10000004, 0x00000000, 0x00000000, 0x00000000,
    0x00002000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 262 */
    0x00000000, 0x00000000, 0x28881041, 0x0081010a,
    0x00400800, 0x00000800, 0x10208026, 0x61000000,
    } },
    { { /* 263 */
    0x00050080, 0x00000000, 0x80000000, 0x80040000,
    0x044088c2, 0x00080480, 0x00040000, 0x00000048,
    } },
    { { /* 264 */
    0x8188410d, 0x141a2400, 0x40310000, 0x000f4249,
    0x41283280, 0x80053011, 0x00400880, 0x410060c0,
    } },
    { { /* 265 */
    0x2a004013, 0x02000002, 0x11000000, 0x00850040,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 266 */
    0x00000000, 0x00800000, 0x04000440, 0x00000402,
    0x60001000, 0x99909f87, 0x5808049d, 0x10002445,
    } },
    { { /* 267 */
    0x00000100, 0x00000000, 0x00000000, 0x00910050,
    0x00000420, 0x00080008, 0x20000000, 0x00288002,
    } },
    { { /* 268 */
    0x00008400, 0x00000400, 0x00000000, 0x00100000,
    0x00002000, 0x00000800, 0x80043400, 0x21000004,
    } },
    { { /* 269 */
    0x20000208, 0x01000600, 0x00000010, 0x00000000,
    0x48000000, 0x14060008, 0x00124020, 0x20812800,
    } },
    { { /* 270 */
    0xa419804b, 0x01064009, 0x10386ca4, 0x85a0620b,
    0x00000010, 0x01000448, 0x00004400, 0x20a02102,
    } },
    { { /* 271 */
    0x00000000, 0x00000000, 0x00147000, 0x01a01404,
    0x10040000, 0x01000000, 0x3002f180, 0x00000008,
    } },
    { { /* 272 */
    0x00002000, 0x00100000, 0x08000010, 0x00020004,
    0x01000029, 0x00002000, 0x00000000, 0x10082000,
    } },
    { { /* 273 */
    0x00000000, 0x0004d041, 0x08000800, 0x00200000,
    0x00401000, 0x00004000, 0x00000000, 0x00000002,
    } },
    { { /* 274 */
    0x01000000, 0x00000000, 0x00020000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 275 */
    0x00000000, 0x00000000, 0x00000000, 0x00800000,
    0x000a0a01, 0x0004002c, 0x01000080, 0x00000000,
    } },
    { { /* 276 */
    0x10000000, 0x08040400, 0x08012010, 0x2569043c,
    0x1a10c460, 0x08800009, 0x000210f0, 0x08c5050c,
    } },
    { { /* 277 */
    0x10000481, 0x00040080, 0x42040000, 0x00100204,
    0x00000000, 0x00000000, 0x00080000, 0x88080000,
    } },
    { { /* 278 */
    0x010f016c, 0x18002000, 0x41307000, 0x00000080,
    0x00000000, 0x00000100, 0x88000000, 0x70048004,
    } },
    { { /* 279 */
    0x00081420, 0x00000100, 0x00000000, 0x00000000,
    0x02400000, 0x00001000, 0x00050070, 0x00000000,
    } },
    { { /* 280 */
    0x000c4000, 0x00010000, 0x04000000, 0x00000000,
    0x00000000, 0x01000100, 0x01000010, 0x00000400,
    } },
    { { /* 281 */
    0x00000000, 0x10020000, 0x04100024, 0x00000000,
    0x00000000, 0x00004000, 0x00000000, 0x00000100,
    } },
    { { /* 282 */
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00100020,
    } },
    { { /* 283 */
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00008000, 0x00100000, 0x00000000, 0x00000000,
    } },
    { { /* 284 */
    0x00000000, 0x00000000, 0x00000000, 0x80000000,
    0x00880000, 0x0c000040, 0x02040010, 0x00000000,
    } },
    { { /* 285 */
    0x00080000, 0x08000000, 0x00000000, 0x00000004,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 286 */
    0x00000000, 0x00000000, 0x07fffffe, 0x07fffffe,
    0x00000000, 0x00000000, 0x00000300, 0x00000300,
    } },
    { { /* 287 */
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0xffff0000, 0x0001ffff,
    } },
    { { /* 288 */
    0xffff0002, 0xffffffff, 0x0002ffff, 0x00000000,
    0x0c0c0000, 0x000cc00c, 0x03000000, 0x00000000,
    } },
    { { /* 289 */
    0x00000000, 0x00000300, 0x00000000, 0x00000300,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 290 */
    0xffff0000, 0xffffffff, 0x0040ffff, 0x00000000,
    0x0c0c0000, 0x0c00000c, 0x03000000, 0x00000300,
    } },
    { { /* 291 */
    0x00000000, 0x00000000, 0x07fffffe, 0x07fffffe,
    0x00000000, 0x00000000, 0x0d10646e, 0x0d10646e,
    } },
    { { /* 292 */
    0x00000000, 0x01000300, 0x00000000, 0x00000300,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 293 */
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x9fffffff, 0xffcffee7, 0x0000003f, 0x00000000,
    } },
    { { /* 294 */
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0xfffddfec, 0xc3effdff, 0x40603ddf, 0x00000003,
    } },
    { { /* 295 */
    0x00000000, 0xfffe0000, 0xffffffff, 0xffffffef,
    0x00007fff, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 296 */
    0x3eff0793, 0x1303b011, 0x11102801, 0x05930000,
    0xb0111e7b, 0x3b019703, 0x00a01112, 0x306b9593,
    } },
    { { /* 297 */
    0x1102b051, 0x11303201, 0x011102b0, 0xb879300a,
    0x30011306, 0x00800010, 0x100b0113, 0x93000011,
    } },
    { { /* 298 */
    0x00102b03, 0x05930000, 0xb051746b, 0x3b011323,
    0x00001030, 0x70000000, 0x1303b011, 0x11102900,
    } },
    { { /* 299 */
    0x00012180, 0xb0153000, 0x3001030e, 0x02000030,
    0x10230111, 0x13000000, 0x10106b81, 0x01130300,
    } },
    { { /* 300 */
    0x30111013, 0x00000100, 0x22b85530, 0x30000000,
    0x9702b011, 0x113afb07, 0x011303b0, 0x00000021,
    } },
    { { /* 301 */
    0x3b0d1b00, 0x03b01138, 0x11330113, 0x13000001,
    0x111c2b05, 0x00000100, 0xb0111000, 0x2a011300,
    } },
    { { /* 302 */
    0x02b01930, 0x10100001, 0x11000000, 0x10300301,
    0x07130230, 0x0011146b, 0x2b051300, 0x8fb8f974,
    } },
    { { /* 303 */
    0x103b0113, 0x00000000, 0xd9700000, 0x01134ab0,
    0x0011103b, 0x00001103, 0x2ab15930, 0x10000111,
    } },
    { { /* 304 */
    0x11010000, 0x00100b01, 0x01130000, 0x0000102b,
    0x20000101, 0x02a01110, 0x30210111, 0x0102b059,
    } },
    { { /* 305 */
    0x19300000, 0x011307b0, 0xb011383b, 0x00000003,
    0x00000000, 0x383b0d13, 0x0103b011, 0x00001000,
    } },
    { { /* 306 */
    0x01130000, 0x00101020, 0x00000100, 0x00000110,
    0x30000000, 0x00021811, 0x00100000, 0x01110000,
    } },
    { { /* 307 */
    0x00000023, 0x0b019300, 0x00301110, 0x302b0111,
    0x13c7b011, 0x01303b01, 0x00000280, 0xb0113000,
    } },
    { { /* 308 */
    0x2b011383, 0x03b01130, 0x300a0011, 0x1102b011,
    0x00002000, 0x01110100, 0xa011102b, 0x2b011302,
    } },
    { { /* 309 */
    0x01000010, 0x30000001, 0x13029011, 0x11302b01,
    0x000066b0, 0xb0113000, 0x6b07d302, 0x07b0113a,
    } },
    { { /* 310 */
    0x00200103, 0x13000000, 0x11386b05, 0x011303b0,
    0x000010b8, 0x2b051b00, 0x03000110, 0x10000000,
    } },
    { { /* 311 */
    0x1102a011, 0x79700a01, 0x0111a2b0, 0x0000100a,
    0x00011100, 0x00901110, 0x00090111, 0x93000000,
    } },
    { { /* 312 */
    0xf9f2bb05, 0x011322b0, 0x2001323b, 0x00000000,
    0x06b05930, 0x303b0193, 0x1123a011, 0x11700000,
    } },
    { { /* 313 */
    0x001102b0, 0x00001010, 0x03011301, 0x00000110,
    0x162b0793, 0x01010010, 0x11300000, 0x01110200,
    } },
    { { /* 314 */
    0xb0113029, 0x00000000, 0x0eb05130, 0x383b0513,
    0x0303b011, 0x00000100, 0x01930000, 0x00001039,
    } },
    { { /* 315 */
    0x3b000302, 0x00000000, 0x00230113, 0x00000000,
    0x00100000, 0x00010000, 0x90113020, 0x00000002,
    } },
    { { /* 316 */
    0x00000000, 0x10000000, 0x11020000, 0x00000301,
    0x01130000, 0xb079b02b, 0x3b011323, 0x02b01130,
    } },
    { { /* 317 */
    0xf0210111, 0x1343b0d9, 0x11303b01, 0x011103b0,
    0xb0517020, 0x20011322, 0x01901110, 0x300b0111,
    } },
    { { /* 318 */
    0x9302b011, 0x0016ab01, 0x01130100, 0xb0113021,
    0x29010302, 0x02b03130, 0x30000000, 0x1b42b819,
    } },
    { { /* 319 */
    0x11383301, 0x00000330, 0x00000020, 0x33051300,
    0x00001110, 0x00000000, 0x93000000, 0x01302305,
    } },
    { { /* 320 */
    0x00010100, 0x30111010, 0x00000100, 0x02301130,
    0x10100001, 0x11000000, 0x00000000, 0x85130200,
    } },
    { { /* 321 */
    0x10111003, 0x2b011300, 0x63b87730, 0x303b0113,
    0x11a2b091, 0x7b300201, 0x011357f0, 0xf0d1702b,
    } },
    { { /* 322 */
    0x1b0111e3, 0x0ab97130, 0x303b0113, 0x13029001,
    0x11302b01, 0x071302b0, 0x3011302b, 0x23011303,
    } },
    { { /* 323 */
    0x02b01130, 0x30ab0113, 0x11feb411, 0x71300901,
    0x05d347b8, 0xb011307b, 0x21015303, 0x00001110,
    } },
    { { /* 324 */
    0x306b0513, 0x1102b011, 0x00103301, 0x05130000,
    0xa01038eb, 0x30000102, 0x02b01110, 0x30200013,
    } },
    { { /* 325 */
    0x0102b071, 0x00101000, 0x01130000, 0x1011100b,
    0x2b011300, 0x00000000, 0x366b0593, 0x1303b095,
    } },
    { { /* 326 */
    0x01103b01, 0x00000200, 0xb0113000, 0x20000103,
    0x01000010, 0x30000000, 0x030ab011, 0x00101001,
    } },
    { { /* 327 */
    0x01110100, 0x00000003, 0x23011302, 0x03000010,
    0x10000000, 0x01000000, 0x00100000, 0x00000290,
    } },
    { { /* 328 */
    0x30113000, 0x7b015386, 0x03b01130, 0x00210151,
    0x13000000, 0x11303b01, 0x001102b0, 0x00011010,
    } },
    { { /* 329 */
    0x2b011302, 0x02001110, 0x10000000, 0x0102b011,
    0x11300100, 0x000102b0, 0x00011010, 0x2b011100,
    } },
    { { /* 330 */
    0x02101110, 0x002b0113, 0x93000000, 0x11302b03,
    0x011302b0, 0x0000303b, 0x00000002, 0x03b01930,
    } },
    { { /* 331 */
    0x102b0113, 0x0103b011, 0x11300000, 0x011302b0,
    0x00001021, 0x00010102, 0x00000010, 0x102b0113,
    } },
    { { /* 332 */
    0x01020011, 0x11302000, 0x011102b0, 0x30113001,
    0x00000002, 0x02b01130, 0x303b0313, 0x0103b011,
    } },
    { { /* 333 */
    0x00002000, 0x05130000, 0xb011303b, 0x10001102,
    0x00000110, 0x142b0113, 0x01000001, 0x01100000,
    } },
    { { /* 334 */
    0x00010280, 0xb0113000, 0x10000102, 0x00000010,
    0x10230113, 0x93021011, 0x11100b05, 0x01130030,
    } },
    { { /* 335 */
    0xb051702b, 0x3b011323, 0x00000030, 0x30000000,
    0x1303b011, 0x11102b01, 0x01010330, 0xb011300a,
    } },
    { { /* 336 */
    0x20000102, 0x00000000, 0x10000011, 0x9300a011,
    0x00102b05, 0x00000200, 0x90111000, 0x29011100,
    } },
    { { /* 337 */
    0x00b01110, 0x30000000, 0x1302b011, 0x11302b21,
    0x000103b0, 0x00000020, 0x2b051300, 0x02b01130,
    } },
    { { /* 338 */
    0x103b0113, 0x13002011, 0x11322b21, 0x00130280,
    0xa0113028, 0x0a011102, 0x02921130, 0x30210111,
    } },
    { { /* 339 */
    0x13020011, 0x11302b01, 0x03d30290, 0x3011122b,
    0x2b011302, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 340 */
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00004000, 0x00000000, 0x20000000, 0x00000000,
    } },
    { { /* 341 */
    0x00000000, 0x00000000, 0x00003000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 342 */
    0x00000000, 0x040001df, 0x80800176, 0x420c0000,
    0x01020140, 0x44008200, 0x00041018, 0x00000000,
    } },
    { { /* 343 */
    0xffff0000, 0xffff27bf, 0x000027bf, 0x00000000,
    0x00000000, 0x0c000000, 0x03000000, 0x000000c0,
    } },
    { { /* 344 */
    0x3c000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 345 */
    0x00000000, 0x00000000, 0x07fffffe, 0x07fffffe,
    0x00000000, 0x00000000, 0x08004480, 0x08004480,
    } },
    { { /* 346 */
    0x00000000, 0x00000000, 0xc0000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 347 */
    0xffff0002, 0xffffffff, 0x0002ffff, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 348 */
    0xffff0042, 0xffffffff, 0x0042ffff, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x000000c0,
    } },
    { { /* 349 */
    0x00000000, 0x000c0000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 350 */
    0xffff0002, 0xffffffff, 0x0002ffff, 0x00000000,
    0x00000000, 0x0000c00c, 0x00000000, 0x00000000,
    } },
    { { /* 351 */
    0x000c0003, 0x00003c00, 0x0000f000, 0x00003c00,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 352 */
    0x00000000, 0x040001de, 0x00000176, 0x42000000,
    0x01020140, 0x44008200, 0x00041008, 0x00000000,
    } },
    { { /* 353 */
    0x00000000, 0x00000000, 0x07fffffe, 0x07fffffe,
    0x00000000, 0x00000000, 0x98504f14, 0x18504f14,
    } },
    { { /* 354 */
    0x00000000, 0x00000000, 0x00000c00, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 355 */
    0x00000000, 0x00000000, 0x07fffffe, 0x07fffffe,
    0x00000000, 0x00000000, 0x00480910, 0x00480910,
    } },
    { { /* 356 */
    0x00000000, 0x00000000, 0x07fffffe, 0x07fffffe,
    0x00000000, 0x00000000, 0x060cb301, 0x060eb3d5,
    } },
    { { /* 357 */
    0x00000000, 0x00000000, 0x07fffffe, 0x07fffffe,
    0x00000000, 0x00000000, 0x0c186606, 0x0c186606,
    } },
    { { /* 358 */
    0x0c000000, 0x00000000, 0x00000000, 0x00000000,
    0x00010040, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 359 */
    0x00001006, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 360 */
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0xfef02596, 0x3bffecae, 0x30003f5f, 0x00000000,
    } },
    { { /* 361 */
    0x03c03030, 0x0000c000, 0x00000000, 0x600c0c03,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 362 */
    0x000c3003, 0x18c00c0c, 0x00c03060, 0x60000c03,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 363 */
    0x00000000, 0x00000000, 0x07fffffe, 0x07fffffe,
    0x00000000, 0x00000000, 0x00100002, 0x00100002,
    } },
    { { /* 364 */
    0x00000003, 0x18000000, 0x00003060, 0x00000c00,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 365 */
    0x00000000, 0x00300000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 366 */
    0x00000000, 0x00000000, 0x07fffffe, 0x07fffffe,
    0x00000000, 0x00000000, 0x140a2202, 0x142a220a,
    } },
    { { /* 367 */
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x20000000, 0x00000000, 0x00000000,
    } },
    { { /* 368 */
    0xfdffb729, 0x000001ff, 0xb7290000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 369 */
    0xfffddfec, 0xc3fffdff, 0x00803dcf, 0x00000003,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 370 */
    0x00000000, 0xffffffff, 0xffffffff, 0x00ffffff,
    0xffffffff, 0x000003ff, 0x00000000, 0x00000000,
    } },
    { { /* 371 */
    0xffff0002, 0xffffffff, 0x0002ffff, 0x00000000,
    0x00000000, 0x0000c000, 0x00000000, 0x00000300,
    } },
    { { /* 372 */
    0x00000000, 0x00000000, 0x00000000, 0x00000010,
    0xfff99fee, 0xf3c5fdff, 0xb000798f, 0x0002ffc0,
    } },
    { { /* 373 */
    0xffffffff, 0x0007f6fb, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 374 */
    0x00000000, 0x00000000, 0x07fffffe, 0x07fffffe,
    0x00000000, 0x00000000, 0x00004004, 0x00004004,
    } },
    { { /* 375 */
    0x0f000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 376 */
    0x00000000, 0x00000000, 0x07fffffe, 0x07fffffe,
    0x00000000, 0x00000000, 0x02045101, 0x02045101,
    } },
    { { /* 377 */
    0x00000c00, 0x000000c3, 0x00000000, 0x18000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 378 */
    0x00000000, 0x00000000, 0x00000000, 0x00000300,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 379 */
    0x00000000, 0x00000000, 0x07fffffe, 0x07fffffe,
    0x00000000, 0x00000000, 0x011c0661, 0x011c0661,
    } },
    { { /* 380 */
    0xfff98fee, 0xc3e5fdff, 0x0001398f, 0x0001fff0,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 381 */
    0x00000002, 0x00000000, 0x00002000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 382 */
    0x00080002, 0x00000800, 0x00002000, 0x00000800,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 383 */
    0x00000000, 0x00000000, 0x07fffffe, 0x07fffffe,
    0x00000000, 0x00000000, 0x1c58af16, 0x1c58af16,
    } },
    { { /* 384 */
    0x00000000, 0x00000000, 0x07fffffe, 0x07fffffe,
    0x00000000, 0x00000000, 0x115c0671, 0x115c0671,
    } },
    { { /* 385 */
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0xffffffff, 0x07ffffff,
    } },
    { { /* 386 */
    0x00000000, 0x00000000, 0x07fffffe, 0x07fffffe,
    0x00000000, 0x00000000, 0x00100400, 0x00100400,
    } },
    { { /* 387 */
    0x00000000, 0x00000000, 0x00000000, 0x00000003,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 388 */
    0x00000000, 0x00000000, 0x07fffffe, 0x07fffffe,
    0x00000000, 0x00000000, 0x00082202, 0x00082202,
    } },
    { { /* 389 */
    0x03000030, 0x0000c000, 0x00000006, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000c00,
    } },
    { { /* 390 */
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x10000000, 0x00000000, 0x00000000,
    } },
    { { /* 391 */
    0x00000002, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 392 */
    0x00000000, 0x00000000, 0x00000000, 0x00300000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 393 */
    0x00000000, 0x00000000, 0x07fffffe, 0x07fffffe,
    0x00000000, 0x00000000, 0x040c2383, 0x040c2383,
    } },
    { { /* 394 */
    0xfff99fee, 0xf3cdfdff, 0xb0c0398f, 0x00000003,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 395 */
    0x00000000, 0x07ffffc6, 0x000001fe, 0x40000000,
    0x01000040, 0x0000a000, 0x00001000, 0x00000000,
    } },
    { { /* 396 */
    0xfff987e0, 0xd36dfdff, 0x1e003987, 0x001f0000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 397 */
    0x00000000, 0x00000000, 0x07fffffe, 0x07fffffe,
    0x00000000, 0x00000000, 0x160e2302, 0x160e2302,
    } },
    { { /* 398 */
    0x00000000, 0x00000000, 0x07fffffe, 0x07fffffe,
    0x00000000, 0x00000000, 0x00020000, 0x00020000,
    } },
    { { /* 399 */
    0x030000f0, 0x00000000, 0x0c00001e, 0x1e000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 400 */
    0x00000000, 0x07ffffde, 0x000005f6, 0x50000000,
    0x05480262, 0x10000a00, 0x00013000, 0x00000000,
    } },
    { { /* 401 */
    0x00000000, 0x07ffffde, 0x000005f6, 0x50000000,
    0x05480262, 0x10000a00, 0x00052000, 0x00000000,
    } },
    { { /* 402 */
    0x00000000, 0x00000000, 0x07fffffe, 0x07fffffe,
    0x00000000, 0x00000000, 0x143c278f, 0x143c278f,
    } },
    { { /* 403 */
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000100, 0x00000000,
    } },
    { { /* 404 */
    0x00002000, 0x00000000, 0x02000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000080,
    } },
    { { /* 405 */
    0x00002000, 0x00000020, 0x08000000, 0x00002008,
    0x00080000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 406 */
    0x00000000, 0x00000000, 0x07fffffe, 0x07fffffe,
    0x00000000, 0x00000000, 0x02045301, 0x02045301,
    } },
    { { /* 407 */
    0xffff0002, 0xffffffff, 0x0002ffff, 0x00000000,
    0x00300000, 0x0c00c030, 0x03000000, 0x00000000,
    } },
    { { /* 408 */
    0xfff987ee, 0xf325fdff, 0x00013987, 0x0001fff0,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 409 */
    0x00000000, 0x00000000, 0x07fffffe, 0x07fffffe,
    0x00000000, 0x00000000, 0x02041101, 0x02041101,
    } },
    { { /* 410 */
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00800000, 0x00000000, 0x00000000,
    } },
    { { /* 411 */
    0x30000000, 0x00000000, 0x00000000, 0x00000000,
    0x00040000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 412 */
    0x00000000, 0x07fffdd6, 0x000005f6, 0xec000000,
    0x0200b4d9, 0x480a8640, 0x00000000, 0x00000000,
    } },
    { { /* 413 */
    0x00000000, 0x00000000, 0x07fffffe, 0x07fffffe,
    0x00000000, 0x00000000, 0x00000002, 0x00000002,
    } },
    { { /* 414 */
    0x00033000, 0x00000000, 0x00000c00, 0x600000c3,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 415 */
    0x00000000, 0x00000000, 0x07fffffe, 0x07fffffe,
    0x00000000, 0x00000000, 0x1850cc14, 0x1850cc14,
    } },
    { { /* 416 */
    0x00000000, 0x00000000, 0x07fffffe, 0x07fffffe,
    0x00000000, 0x00000000, 0x00000000, 0x00200000,
    } },
    { { /* 417 */
    0x03c83032, 0x0000c800, 0x00002000, 0x600c0c03,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 418 */
    0x00000010, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 419 */
    0xffff8f04, 0xffffffff, 0x8f04ffff, 0x00000000,
    0x030c0000, 0x0c00cc0f, 0x03000000, 0x00000300,
    } },
    { { /* 420 */
    0x00000000, 0x00800000, 0x03bffbaa, 0x03bffbaa,
    0x00000000, 0x00000000, 0x00002202, 0x00002202,
    } },
    { { /* 421 */
    0x00080000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 422 */
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0xfc7e3fec, 0x2ffbffbf, 0x7f5f847f, 0x00040000,
    } },
    { { /* 423 */
    0xff7fff7f, 0xff01ff7f, 0x3d7f3d7f, 0xffff7fff,
    0xffff3d7f, 0x003d7fff, 0xff7f7f3d, 0x00ff7fff,
    } },
    { { /* 424 */
    0x00000000, 0x00000000, 0x07fffffe, 0x07fffffe,
    0x00000000, 0x00000000, 0x24182212, 0x24182212,
    } },
    { { /* 425 */
    0x0000f000, 0x66000000, 0x00300180, 0x60000033,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 426 */
    0x00000000, 0x00000000, 0x07fffffe, 0x07fffffe,
    0x00000000, 0x00000000, 0x00408030, 0x00408030,
    } },
    { { /* 427 */
    0x00000000, 0x00000000, 0x07fffffe, 0x07fffffe,
    0x00000000, 0x00000000, 0x00020032, 0x00020032,
    } },
    { { /* 428 */
    0x00000000, 0x00000000, 0x07fffffe, 0x07fffffe,
    0x00000000, 0x00000000, 0x00000016, 0x00000016,
    } },
    { { /* 429 */
    0x00033000, 0x00000000, 0x00000c00, 0x60000003,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 430 */
    0x00000000, 0x00000000, 0x07fffffe, 0x07fffffe,
    0x00000000, 0x00000000, 0x00200034, 0x00200034,
    } },
    { { /* 431 */
    0x00033000, 0x00000000, 0x00000c00, 0x60000003,
    0x00000000, 0x00800000, 0x00000000, 0x0000c3f0,
    } },
    { { /* 432 */
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00040000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 433 */
    0x00000000, 0x00000000, 0x07fffffe, 0x07fffffe,
    0x00000000, 0x00000000, 0x00000880, 0x00000880,
    } },
    { { /* 434 */
    0xfdff8f04, 0xfdff01ff, 0x8f0401ff, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 435 */
    0x00000000, 0x00000000, 0x07fffffe, 0x07fffffe,
    0x00000000, 0x00000000, 0x10400a33, 0x10400a33,
    } },
    { { /* 436 */
    0xffff0000, 0xffff1fff, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 437 */
    0x00000000, 0x00000000, 0x07fffffe, 0x07fffffe,
    0x00000000, 0x00000000, 0x00380008, 0x00080000,
    } },
    { { /* 438 */
    0x030000f0, 0x00000000, 0x0c00501e, 0x1e004000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 439 */
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0xd63dc7e8, 0xc3bfc718, 0x00803dc7, 0x00000000,
    } },
    { { /* 440 */
    0xfffddfee, 0xc3effdff, 0x00603ddf, 0x00000003,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 441 */
    0xffff0002, 0xffffffff, 0x0002ffff, 0x00000000,
    0x0c0c0000, 0x00cc0000, 0x00000000, 0x0000c00c,
    } },
    { { /* 442 */
    0xfffffffe, 0x87ffffff, 0x00007fff, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 443 */
    0xff7fff7f, 0xff01ff00, 0x00003d7f, 0xffff7fff,
    0x00ff0000, 0x003d7f7f, 0xff7f7f00, 0x00ff7f00,
    } },
    { { /* 444 */
    0x00000000, 0x00000000, 0x07fffffe, 0x07fffffe,
    0x00000000, 0x00000000, 0x30400090, 0x30400090,
    } },
    { { /* 445 */
    0x00000000, 0x00000000, 0xc0000180, 0x60000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 446 */
    0x00000000, 0x00000000, 0x07fffffe, 0x07fffffe,
    0x00000000, 0x00000000, 0x18404084, 0x18404084,
    } },
    { { /* 447 */
    0xffff0002, 0xffffffff, 0x0002ffff, 0x00000000,
    0x00c00000, 0x0c00c00c, 0x03000000, 0x00000000,
    } },
    { { /* 448 */
    0x00000000, 0x00000000, 0x07fffffe, 0x07fffffe,
    0x00000000, 0x00000000, 0x00008000, 0x00008000,
    } },
    { { /* 449 */
    0x00000000, 0x041ed5c0, 0x0000077e, 0x40000000,
    0x01000040, 0x4000a000, 0x002109c0, 0x00000000,
    } },
    { { /* 450 */
    0xffff00d0, 0xffffffff, 0x00d0ffff, 0x00000000,
    0x00030000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 451 */
    0x00000000, 0xffffff7b, 0x7fffffff, 0x7ffffffe,
    0x00000000, 0x80e310fe, 0x00800000, 0x00800000,
    } },
    { { /* 452 */
    0x00000000, 0x00020000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 453 */
    0x00001500, 0x01000000, 0x00000000, 0x00000000,
    0xfffe0000, 0xfffe03db, 0x006003fb, 0x00030000,
    } },
    { { /* 454 */
    0x00400000, 0x00000047, 0x00800010, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000002,
    } },
    { { /* 455 */
    0x3f2fc004, 0x00000010, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 456 */
    0xe3ffbfff, 0xfff007ff, 0x00000001, 0x00000000,
    0xfffff000, 0x0000003f, 0x0000e10f, 0x00000000,
    } },
    { { /* 457 */
    0x00000f00, 0x0000000c, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 458 */
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000003, 0x00000000, 0x00000000,
    } },
    { { /* 459 */
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x000003c0,
    } },
    { { /* 460 */
    0xffffffff, 0xffffffff, 0xffdfffff, 0xffffffff,
    0xdfffffff, 0x00001e64, 0x00000000, 0x00000000,
    } },
    { { /* 461 */
    0x00000000, 0x78000000, 0x0001fc5f, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 462 */
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000030, 0x00000000, 0x00000000,
    } },
    { { /* 463 */
    0x0c000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00091e00,
    } },
    { { /* 464 */
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x60000000,
    } },
    { { /* 465 */
    0x00300000, 0x00000000, 0x000fff00, 0x80000000,
    0x00080000, 0x60000c02, 0x00104030, 0x242c0400,
    } },
    { { /* 466 */
    0x00000c20, 0x00000100, 0x00b85000, 0x00000000,
    0x00e00000, 0x80010000, 0x00000000, 0x00000000,
    } },
    { { /* 467 */
    0x18000000, 0x00000000, 0x00210000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 468 */
    0x00000010, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00008000, 0x00000000,
    } },
    { { /* 469 */
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x07fe4000, 0x00000000, 0x00000000, 0xffffffc0,
    } },
    { { /* 470 */
    0x04000002, 0x077c8000, 0x00030000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 471 */
    0xffffffff, 0xffbf0001, 0xffffffff, 0x1fffffff,
    0x000fffff, 0xffffffff, 0x000007df, 0x0001ffff,
    } },
    { { /* 472 */
    0x00000000, 0x00000000, 0xfffffffd, 0xffffffff,
    0xffffffff, 0xffffffff, 0xffffffff, 0x1effffff,
    } },
    { { /* 473 */
    0xffffffff, 0x3fffffff, 0xffff0000, 0x000000ff,
    0x00000000, 0x00000000, 0x00000000, 0xf8000000,
    } },
    { { /* 474 */
    0x755dfffe, 0xffef2f3f, 0x0000ffe1, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 475 */
    0x000c0000, 0x30000000, 0x00000c30, 0x00030000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 476 */
    0x00000000, 0x00000000, 0x07fffffe, 0x07fffffe,
    0x00000000, 0x00000000, 0x263c370f, 0x263c370f,
    } },
    { { /* 477 */
    0x0003000c, 0x00000300, 0x00000000, 0x00000300,
    0x00000000, 0x00018003, 0x00000000, 0x00000000,
    } },
    { { /* 478 */
    0x0800024f, 0x00000008, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 479 */
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0xffffffff, 0xffffffff, 0x03ffffff,
    } },
    { { /* 480 */
    0x00000000, 0x00000000, 0x077dfffe, 0x077dfffe,
    0x00000000, 0x00000000, 0x10400010, 0x10400010,
    } },
    { { /* 481 */
    0x00000000, 0x00000000, 0x07fffffe, 0x07fffffe,
    0x00000000, 0x00000000, 0x10400010, 0x10400010,
    } },
    { { /* 482 */
    0x00000000, 0x00000000, 0x07fffffe, 0x07fffffe,
    0x00000000, 0x00000000, 0x081047a4, 0x081047a4,
    } },
    { { /* 483 */
    0x0c0030c0, 0x00000000, 0x0f30001e, 0x66000003,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 484 */
    0x00000000, 0x00000000, 0x07fffffe, 0x07fffffe,
    0x00000000, 0x00000000, 0x000a0a09, 0x000a0a09,
    } },
    { { /* 485 */
    0x00000000, 0x00000000, 0x07fffffe, 0x07fffffe,
    0x00000000, 0x00000000, 0x00400810, 0x00400810,
    } },
    { { /* 486 */
    0x00000000, 0x00000000, 0x07fffffe, 0x07fffffe,
    0x00000000, 0x00000000, 0x0e3c770f, 0x0e3c770f,
    } },
    { { /* 487 */
    0x0c000000, 0x00000300, 0x00000018, 0x00000300,
    0x00000000, 0x00000000, 0x001fe000, 0x03000000,
    } },
    { { /* 488 */
    0x0000100f, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 489 */
    0x00000000, 0xc0000000, 0x00000000, 0x0000000c,
    0x00000000, 0x33000000, 0x00003000, 0x00000000,
    } },
    { { /* 490 */
    0x00000080, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 491 */
    0x00000000, 0x00000000, 0x00001000, 0x64080010,
    0x00480000, 0x10000020, 0x80000102, 0x08000010,
    } },
    { { /* 492 */
    0x00000040, 0x40000000, 0x00020000, 0x01852002,
    0x00800010, 0x80002022, 0x084444a2, 0x480e0000,
    } },
    { { /* 493 */
    0x04000200, 0x02202008, 0x80004380, 0x04000000,
    0x00000002, 0x12231420, 0x2058003a, 0x00200060,
    } },
    { { /* 494 */
    0x10002508, 0x040d0028, 0x00000009, 0x00008004,
    0x00800000, 0x42000001, 0x00000000, 0x09040000,
    } },
    { { /* 495 */
    0x02008000, 0x01402001, 0x00000000, 0x00000008,
    0x00000000, 0x00000001, 0x00021008, 0x04000000,
    } },
    { { /* 496 */
    0x00100100, 0x80040080, 0x00002000, 0x00000008,
    0x08040601, 0x01000012, 0x10000000, 0x49001024,
    } },
    { { /* 497 */
    0x0180004a, 0x00100600, 0x50840800, 0x000000c0,
    0x00800000, 0x20000800, 0x40000000, 0x08050000,
    } },
    { { /* 498 */
    0x02004000, 0x02000804, 0x01000004, 0x18060001,
    0x02400001, 0x40000002, 0x20800014, 0x000c1000,
    } },
    { { /* 499 */
    0x00222000, 0x00000000, 0x00100000, 0x00000000,
    0x00000000, 0x00000000, 0x10422800, 0x00000800,
    } },
    { { /* 500 */
    0x20080000, 0x00040000, 0x80025040, 0x20208604,
    0x00028020, 0x80102020, 0x080820c0, 0x10880800,
    } },
    { { /* 501 */
    0x00000000, 0x00000000, 0x00200109, 0x00100000,
    0x00000000, 0x81022700, 0x40c21404, 0x84010882,
    } },
    { { /* 502 */
    0x00004010, 0x00000000, 0x03000000, 0x00000008,
    0x00080000, 0x00000000, 0x10800001, 0x06002020,
    } },
    { { /* 503 */
    0x00000010, 0x02000000, 0x00880020, 0x00008424,
    0x00000000, 0x88000000, 0x81000100, 0x04000000,
    } },
    { { /* 504 */
    0x00004218, 0x00040000, 0x00000000, 0x80005080,
    0x00010000, 0x00040000, 0x08008000, 0x02008000,
    } },
    { { /* 505 */
    0x00020000, 0x00000000, 0x00000001, 0x04000401,
    0x00100000, 0x12200004, 0x00000000, 0x18100000,
    } },
    { { /* 506 */
    0x00000000, 0x00000800, 0x00000000, 0x00004000,
    0x00800000, 0x04000000, 0x82000002, 0x00042000,
    } },
    { { /* 507 */
    0x00080006, 0x00000000, 0x00000000, 0x04000000,
    0x80008000, 0x00810001, 0xa0000000, 0x00100410,
    } },
    { { /* 508 */
    0x00400218, 0x88084080, 0x00260008, 0x00800404,
    0x00000020, 0x00000000, 0x00000000, 0x00000200,
    } },
    { { /* 509 */
    0x00a08048, 0x00000000, 0x08000000, 0x04000000,
    0x00000000, 0x00000000, 0x00018000, 0x00200000,
    } },
    { { /* 510 */
    0x01000000, 0x00000000, 0x00000000, 0x10000000,
    0x00000000, 0x00000000, 0x00200000, 0x00102000,
    } },
    { { /* 511 */
    0x00000801, 0x00000000, 0x00000000, 0x00020000,
    0x08000000, 0x00002000, 0x20010000, 0x04002000,
    } },
    { { /* 512 */
    0x40000040, 0x50202400, 0x000a0020, 0x00040420,
    0x00000200, 0x00000080, 0x80000000, 0x00000020,
    } },
    { { /* 513 */
    0x20008000, 0x00200010, 0x00000000, 0x00000000,
    0x00400000, 0x01100000, 0x00020000, 0x80000010,
    } },
    { { /* 514 */
    0x02000000, 0x00801000, 0x00000000, 0x48058000,
    0x20c94000, 0x60000000, 0x00000001, 0x00000000,
    } },
    { { /* 515 */
    0x00004090, 0x48000000, 0x08000000, 0x28802000,
    0x00000002, 0x00014000, 0x00002000, 0x00002002,
    } },
    { { /* 516 */
    0x00010200, 0x00100000, 0x00000000, 0x00800000,
    0x10020000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 517 */
    0x00000010, 0x00000402, 0x0c000000, 0x01000400,
    0x01000021, 0x00000000, 0x00004000, 0x00004000,
    } },
    { { /* 518 */
    0x00000000, 0x00800000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x02000020,
    } },
    { { /* 519 */
    0x00000100, 0x08000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00002000, 0x00000000,
    } },
    { { /* 520 */
    0x00006000, 0x00000000, 0x00000000, 0x00000400,
    0x04000040, 0x003c0180, 0x00000200, 0x00102000,
    } },
    { { /* 521 */
    0x00000800, 0x101000c0, 0x00800000, 0x00000000,
    0x00008000, 0x02200000, 0x00020020, 0x00000000,
    } },
    { { /* 522 */
    0x00000000, 0x01000000, 0x00000000, 0x20100000,
    0x00080000, 0x00000141, 0x02001002, 0x40400001,
    } },
    { { /* 523 */
    0x00580000, 0x00000002, 0x00003000, 0x00002400,
    0x00988000, 0x00040010, 0x00002800, 0x00000008,
    } },
    { { /* 524 */
    0x40080004, 0x00000020, 0x20080000, 0x02060a00,
    0x00010040, 0x14010200, 0x40800000, 0x08031000,
    } },
    { { /* 525 */
    0x40020020, 0x0000202c, 0x2014a008, 0x00000000,
    0x80040200, 0x82020012, 0x00400000, 0x20000000,
    } },
    { { /* 526 */
    0x00000000, 0x00000000, 0x00000004, 0x04000000,
    0x00000000, 0x00000000, 0x40800100, 0x00000000,
    } },
    { { /* 527 */
    0x00000008, 0x04000040, 0x00000001, 0x000c0200,
    0x00000000, 0x08000400, 0x00000000, 0x080c0001,
    } },
    { { /* 528 */
    0x00000400, 0x00000000, 0x00000000, 0x00200000,
    0x80000000, 0x00001000, 0x00000200, 0x01000800,
    } },
    { { /* 529 */
    0x00000000, 0x00000800, 0x00000000, 0x40000000,
    0x00000000, 0x00000000, 0x00000000, 0x04040000,
    } },
    { { /* 530 */
    0x00000000, 0x00000000, 0x00000040, 0x00002000,
    0xa0000000, 0x00000000, 0x08000008, 0x00080000,
    } },
    { { /* 531 */
    0x00000020, 0x00000000, 0x40000400, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00008000,
    } },
    { { /* 532 */
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000800, 0x00000000, 0x00000000, 0x00200000,
    } },
    { { /* 533 */
    0x00000000, 0x00000000, 0x00000000, 0x04000000,
    0x00000008, 0x00000000, 0x00010000, 0x1b000000,
    } },
    { { /* 534 */
    0x00007000, 0x00000000, 0x10000000, 0x00000000,
    0x00000000, 0x00000080, 0x80000000, 0x00000000,
    } },
    { { /* 535 */
    0x00000000, 0x00020000, 0x00000000, 0x00200000,
    0x40000000, 0x00000010, 0x00800000, 0x00000008,
    } },
    { { /* 536 */
    0x00000000, 0x00000000, 0x02000000, 0x20000010,
    0x00000080, 0x00000000, 0x00010000, 0x00000000,
    } },
    { { /* 537 */
    0x00000000, 0x02000000, 0x00000000, 0x00000000,
    0x20000000, 0x00000040, 0x00200028, 0x00000000,
    } },
    { { /* 538 */
    0x00000000, 0x00020000, 0x00000000, 0x02000000,
    0x00000000, 0x02000000, 0x40020000, 0x51000040,
    } },
    { { /* 539 */
    0x00000080, 0x04040000, 0x00000000, 0x10000000,
    0x00022000, 0x00100000, 0x20000000, 0x00000082,
    } },
    { { /* 540 */
    0x40000000, 0x00010000, 0x00002000, 0x00000000,
    0x00000240, 0x00000000, 0x00000000, 0x00000008,
    } },
    { { /* 541 */
    0x00000000, 0x00010000, 0x00000810, 0x00080880,
    0x00004000, 0x00000000, 0x00000000, 0x00020000,
    } },
    { { /* 542 */
    0x00000000, 0x00400020, 0x00000000, 0x00000082,
    0x00000000, 0x00020001, 0x00000000, 0x00000000,
    } },
    { { /* 543 */
    0x40000018, 0x00000004, 0x00000000, 0x00000000,
    0x01000000, 0x00400000, 0x00000000, 0x00000000,
    } },
    { { /* 544 */
    0x00000001, 0x00400000, 0x00000000, 0x00080002,
    0x00000400, 0x00040000, 0x00000000, 0x00000000,
    } },
    { { /* 545 */
    0x00000800, 0x00000800, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000100, 0x00000000,
    } },
    { { /* 546 */
    0x00000000, 0x00200000, 0x00000000, 0x04108000,
    0x00000000, 0x00000000, 0x00000000, 0x00000002,
    } },
    { { /* 547 */
    0x00000000, 0x02800000, 0x04000000, 0x00000000,
    0x00000000, 0x00000004, 0x00000000, 0x00000400,
    } },
    { { /* 548 */
    0x00000000, 0x00000000, 0x10000000, 0x00040000,
    0x00400000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 549 */
    0x00200000, 0x00000200, 0x00000000, 0x10000000,
    0x00000000, 0x00000000, 0x2a000000, 0x00000000,
    } },
    { { /* 550 */
    0x00400000, 0x00000000, 0x00400000, 0x00000000,
    0x00000002, 0x40000000, 0x00000000, 0x00400000,
    } },
    { { /* 551 */
    0x40000000, 0x00001000, 0x00000000, 0x00000000,
    0x00000202, 0x02000000, 0x80000000, 0x00020000,
    } },
    { { /* 552 */
    0x00000020, 0x00000800, 0x00020421, 0x00020000,
    0x00000000, 0x00000000, 0x00000000, 0x00400000,
    } },
    { { /* 553 */
    0x00200000, 0x00000000, 0x00000001, 0x00000000,
    0x00000084, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 554 */
    0x00000000, 0x00004400, 0x00000002, 0x00100000,
    0x00000000, 0x00000000, 0x00008200, 0x00000000,
    } },
    { { /* 555 */
    0x00000000, 0x12000000, 0x00000100, 0x00000001,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 556 */
    0x00000020, 0x08100000, 0x000a0400, 0x00000081,
    0x00006000, 0x00120000, 0x00000000, 0x00000000,
    } },
    { { /* 557 */
    0x00000004, 0x08000000, 0x00004000, 0x044000c0,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 558 */
    0x40001000, 0x00000000, 0x01000001, 0x05000000,
    0x00080000, 0x02000000, 0x00000800, 0x00000000,
    } },
    { { /* 559 */
    0x00000100, 0x00000000, 0x00000000, 0x00000000,
    0x00002002, 0x01020000, 0x00800000, 0x00000000,
    } },
    { { /* 560 */
    0x00000040, 0x00004000, 0x01000000, 0x00000004,
    0x00020000, 0x00000000, 0x00000010, 0x00000000,
    } },
    { { /* 561 */
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00080000, 0x00010000, 0x30000300, 0x00000400,
    } },
    { { /* 562 */
    0x00000800, 0x02000000, 0x00000000, 0x00008000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 563 */
    0x00200000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x000040c0, 0x00002200, 0x12002000,
    } },
    { { /* 564 */
    0x00000000, 0x00000020, 0x20000000, 0x00000000,
    0x00000200, 0x00080800, 0x1000a000, 0x00000000,
    } },
    { { /* 565 */
    0x00000000, 0x00000000, 0x00000000, 0x00004000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 566 */
    0x00000000, 0x00000000, 0x00004280, 0x01000000,
    0x00800000, 0x00000008, 0x00000000, 0x00000000,
    } },
    { { /* 567 */
    0x00000000, 0x00000000, 0x00000000, 0x00000002,
    0x00000000, 0x20400000, 0x00000040, 0x00000000,
    } },
    { { /* 568 */
    0x00800080, 0x00800000, 0x00000000, 0x00000000,
    0x00000000, 0x00400020, 0x00000000, 0x00008000,
    } },
    { { /* 569 */
    0x01000000, 0x00000040, 0x00000000, 0x00400000,
    0x00000000, 0x00000440, 0x00000000, 0x00800000,
    } },
    { { /* 570 */
    0x01000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00080000, 0x00000000,
    } },
    { { /* 571 */
    0x01000000, 0x00000001, 0x00000000, 0x00020000,
    0x00000000, 0x20002000, 0x00000000, 0x00000004,
    } },
    { { /* 572 */
    0x00000008, 0x00100000, 0x00000000, 0x00010000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 573 */
    0x00000004, 0x00008000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00008000,
    } },
    { { /* 574 */
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000040, 0x00000000, 0x00004000, 0x00000000,
    } },
    { { /* 575 */
    0x00000010, 0x00002000, 0x40000040, 0x00000000,
    0x10000000, 0x00000000, 0x00008080, 0x00000000,
    } },
    { { /* 576 */
    0x00000000, 0x00000000, 0x00000080, 0x00000000,
    0x00100080, 0x000000a0, 0x00000000, 0x00000000,
    } },
    { { /* 577 */
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00100000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 578 */
    0x00000000, 0x00000000, 0x00001000, 0x00000000,
    0x0001000a, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 579 */
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x08002000, 0x00000000,
    } },
    { { /* 580 */
    0x00000808, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 581 */
    0x00004000, 0x00002400, 0x00008000, 0x40000000,
    0x00000001, 0x00002000, 0x04000000, 0x00040004,
    } },
    { { /* 582 */
    0x00000000, 0x00002000, 0x00000000, 0x00000000,
    0x00000000, 0x1c200000, 0x00000000, 0x02000000,
    } },
    { { /* 583 */
    0x00000000, 0x00080000, 0x00400000, 0x00000002,
    0x00000000, 0x00000100, 0x00000000, 0x00000000,
    } },
    { { /* 584 */
    0x00000000, 0x00000000, 0x00000000, 0x00400000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 585 */
    0x00004100, 0x00000400, 0x20200010, 0x00004004,
    0x00000000, 0x42000000, 0x00000000, 0x00000000,
    } },
    { { /* 586 */
    0x00000080, 0x00000000, 0x00000121, 0x00000200,
    0x000000b0, 0x80002000, 0x00000000, 0x00010000,
    } },
    { { /* 587 */
    0x00000010, 0x000000c0, 0x08100000, 0x00000020,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 588 */
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x02000000, 0x00000404, 0x00000000, 0x00000000,
    } },
    { { /* 589 */
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00400000, 0x00000008, 0x00000000, 0x00000000,
    } },
    { { /* 590 */
    0x00000000, 0x00000002, 0x00020000, 0x00002000,
    0x00000000, 0x00000000, 0x00000000, 0x00204000,
    } },
    { { /* 591 */
    0x00000000, 0x00100000, 0x00000000, 0x00000000,
    0x00000000, 0x00800000, 0x00000100, 0x00000001,
    } },
    { { /* 592 */
    0x10000000, 0x01000000, 0x00002400, 0x00000004,
    0x00000000, 0x00000000, 0x00000020, 0x00000002,
    } },
    { { /* 593 */
    0x00010000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 594 */
    0x00000000, 0x00002400, 0x00000000, 0x00000000,
    0x00004802, 0x00000000, 0x00000000, 0x80022000,
    } },
    { { /* 595 */
    0x00001004, 0x04208000, 0x20000020, 0x00040000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 596 */
    0x00000000, 0x00100000, 0x40010000, 0x00000000,
    0x00080000, 0x00000000, 0x00100211, 0x00000000,
    } },
    { { /* 597 */
    0x00001400, 0x00000000, 0x00000000, 0x00000000,
    0x00610000, 0x80008c00, 0x00000000, 0x00000000,
    } },
    { { /* 598 */
    0x00000100, 0x00000040, 0x00000000, 0x00000004,
    0x00004000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 599 */
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000400, 0x00000000,
    } },
    { { /* 600 */
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000210, 0x00000000, 0x00000000,
    } },
    { { /* 601 */
    0x00000000, 0x00000020, 0x00000002, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 602 */
    0x00004000, 0x00000000, 0x00000000, 0x02000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 603 */
    0x00000000, 0x00000000, 0x00080002, 0x01000020,
    0x00400000, 0x00200000, 0x00008000, 0x00000000,
    } },
    { { /* 604 */
    0x00000000, 0x00020000, 0x00000000, 0xc0020000,
    0x10000000, 0x00000080, 0x00000000, 0x00000000,
    } },
    { { /* 605 */
    0x00000210, 0x00000000, 0x00001000, 0x04480000,
    0x20000000, 0x00000004, 0x00800000, 0x02000000,
    } },
    { { /* 606 */
    0x00000000, 0x08006000, 0x00001000, 0x00000000,
    0x00000000, 0x00100000, 0x00000000, 0x00000400,
    } },
    { { /* 607 */
    0x00100000, 0x00000000, 0x10000000, 0x08608000,
    0x00000000, 0x00000000, 0x00080002, 0x00000000,
    } },
    { { /* 608 */
    0x00000000, 0x20000000, 0x00008020, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 609 */
    0x00000000, 0x00000000, 0x00000000, 0x10000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 610 */
    0x00000000, 0x00100000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 611 */
    0x00000000, 0x00000400, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 612 */
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x02000000,
    } },
    { { /* 613 */
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000080, 0x00000000,
    } },
    { { /* 614 */
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000002, 0x00000000, 0x00000000,
    } },
    { { /* 615 */
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00008000, 0x00000000,
    } },
    { { /* 616 */
    0x00000000, 0x00000000, 0x00000008, 0x00000000,
    0x00000000, 0x00000000, 0x00000400, 0x00000000,
    } },
    { { /* 617 */
    0x00000000, 0x00000000, 0x00220000, 0x00000004,
    0x00000000, 0x00040000, 0x00000004, 0x00000000,
    } },
    { { /* 618 */
    0x00000000, 0x00000000, 0x00001000, 0x00000080,
    0x00002000, 0x00000000, 0x00000000, 0x00004000,
    } },
    { { /* 619 */
    0x00000000, 0x00000000, 0x00000000, 0x00100000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 620 */
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00200000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 621 */
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x04000000, 0x00000000, 0x00000000,
    } },
    { { /* 622 */
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000200, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 623 */
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000001, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 624 */
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00080000, 0x00000000,
    } },
    { { /* 625 */
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x01000000, 0x00000000, 0x00000400,
    } },
    { { /* 626 */
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000080, 0x00000000, 0x00000000,
    } },
    { { /* 627 */
    0x00000000, 0x00000800, 0x00000100, 0x40000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 628 */
    0x00000000, 0x00200000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 629 */
    0x00000000, 0x00000000, 0x01000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 630 */
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x04000000, 0x00000000,
    } },
    { { /* 631 */
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00001000, 0x00000000,
    } },
    { { /* 632 */
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000400, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 633 */
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x04040000,
    } },
    { { /* 634 */
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000020, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 635 */
    0x00000000, 0x00000000, 0x00800000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 636 */
    0x00000000, 0x00200000, 0x40000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 637 */
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x20000000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 638 */
    0x00000000, 0x00000000, 0x00000000, 0x04000000,
    0x00000000, 0x00000001, 0x00000000, 0x00000000,
    } },
    { { /* 639 */
    0x00000000, 0x40000000, 0x02000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 640 */
    0x00000000, 0x00000000, 0x00000000, 0x00080000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 641 */
    0x00000000, 0x00000010, 0x00000000, 0x00000000,
    0x00000000, 0x20000000, 0x00000000, 0x00000000,
    } },
    { { /* 642 */
    0x00000000, 0x00000000, 0x20000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 643 */
    0x00000080, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000004,
    } },
    { { /* 644 */
    0x00000000, 0x00000000, 0x00000000, 0x00002000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 645 */
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x10000001, 0x00000000,
    } },
    { { /* 646 */
    0x00008000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 647 */
    0x00000000, 0x00000000, 0x00004040, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 648 */
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00042400, 0x00000000,
    } },
    { { /* 649 */
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x02000000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 650 */
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000080,
    } },
    { { /* 651 */
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000020,
    } },
    { { /* 652 */
    0x00000000, 0x00000001, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 653 */
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00020000, 0x00000000,
    } },
    { { /* 654 */
    0x00000000, 0x00000000, 0x00002000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 655 */
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x01000000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 656 */
    0x00000000, 0x00040000, 0x08000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 657 */
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000280, 0x00000000,
    } },
    { { /* 658 */
    0x7f7b7f8b, 0xef553db4, 0xf35dfba8, 0x400b0243,
    0x8d3efb40, 0x8c2c7bf7, 0xe3fa6eff, 0xa8ed1d3a,
    } },
    { { /* 659 */
    0xcf83e602, 0x35558cf5, 0xffabe048, 0xd85992b9,
    0x2892ab18, 0x8020d7e9, 0xf583c438, 0x450ae74a,
    } },
    { { /* 660 */
    0x9714b000, 0x54007762, 0x1420d188, 0xc8c01020,
    0x00002121, 0x0c0413a8, 0x04408000, 0x082870c0,
    } },
    { { /* 661 */
    0x000408c0, 0x80000002, 0x14722b7b, 0x3bfb7924,
    0x1ae43327, 0x38ef9835, 0x28029ad1, 0xbf69a813,
    } },
    { { /* 662 */
    0x2fc665cf, 0xafc96b11, 0x5053340f, 0xa00486a2,
    0xe8090106, 0xc00e3f0f, 0x81450a88, 0xc6010010,
    } },
    { { /* 663 */
    0x26e1a161, 0xce00444b, 0xd4eec7aa, 0x85bbcadf,
    0xa5203a74, 0x8840436c, 0x8bd23f06, 0x3befff79,
    } },
    { { /* 664 */
    0xe8eff75a, 0x5b36fbcb, 0x1bfd0d49, 0x39ee0154,
    0x2e75d855, 0xa91abfd8, 0xf6bff3d7, 0xb40c67e0,
    } },
    { { /* 665 */
    0x081382c2, 0xd08bd49d, 0x1061065a, 0x59e074f2,
    0xb3128f9f, 0x6aaa0080, 0xb05e3230, 0x60ac9d7a,
    } },
    { { /* 666 */
    0xc900d303, 0x8a563098, 0x13907000, 0x18421f14,
    0x0008c060, 0x10808008, 0xec900400, 0xe6332817,
    } },
    { { /* 667 */
    0x90000758, 0x4e09f708, 0xfc83f485, 0x18c8af53,
    0x080c187c, 0x01146adf, 0xa734c80c, 0x2710a011,
    } },
    { { /* 668 */
    0x422228c5, 0x00210413, 0x41123010, 0x40001820,
    0xc60c022b, 0x10000300, 0x00220022, 0x02495810,
    } },
    { { /* 669 */
    0x9670a094, 0x1792eeb0, 0x05f2cb96, 0x23580025,
    0x42cc25de, 0x4a04cf38, 0x359f0c40, 0x8a001128,
    } },
    { { /* 670 */
    0x910a13fa, 0x10560229, 0x04200641, 0x84f00484,
    0x0c040000, 0x412c0400, 0x11541206, 0x00020a4b,
    } },
    { { /* 671 */
    0x00c00200, 0x00940000, 0xbfbb0001, 0x242b167c,
    0x7fa89bbb, 0xe3790c7f, 0xe00d10f4, 0x9f014132,
    } },
    { { /* 672 */
    0x35728652, 0xff1210b4, 0x4223cf27, 0x8602c06b,
    0x1fd33106, 0xa1aa3a0c, 0x02040812, 0x08012572,
    } },
    { { /* 673 */
    0x485040cc, 0x601062d0, 0x29001c80, 0x00109a00,
    0x22000004, 0x00800000, 0x68002020, 0x609ecbe6,
    } },
    { { /* 674 */
    0x3f73916e, 0x398260c0, 0x48301034, 0xbd5c0006,
    0xd6fb8cd1, 0x43e820e1, 0x084e0600, 0xc4d00500,
    } },
    { { /* 675 */
    0x89aa8d1f, 0x1602a6e1, 0x21ed0001, 0x1a8b3656,
    0x13a51fb7, 0x30a06502, 0x23c7b278, 0xe9226c93,
    } },
    { { /* 676 */
    0x3a74e47f, 0x98208fe3, 0x2625280e, 0xbf49bf9c,
    0xac543218, 0x1916b949, 0xb5220c60, 0x0659fbc1,
    } },
    { { /* 677 */
    0x8420e343, 0x800008d9, 0x20225500, 0x00a10184,
    0x20104800, 0x40801380, 0x00160d04, 0x80200040,
    } },
    { { /* 678 */
    0x8de7fd40, 0xe0985436, 0x091e7b8b, 0xd249fec8,
    0x8dee0611, 0xba221937, 0x9fdd77f4, 0xf0daf3ec,
    } },
    { { /* 679 */
    0xec424386, 0x26048d3f, 0xc021fa6c, 0x0cc2628e,
    0x0145d785, 0x559977ad, 0x4045e250, 0xa154260b,
    } },
    { { /* 680 */
    0x58199827, 0xa4103443, 0x411405f2, 0x07002280,
    0x426600b4, 0x15a17210, 0x41856025, 0x00000054,
    } },
    { { /* 681 */
    0x01040201, 0xcb70c820, 0x6a629320, 0x0095184c,
    0x9a8b1880, 0x3201aab2, 0x00c4d87a, 0x04c3f3e5,
    } },
    { { /* 682 */
    0xa238d44d, 0x5072a1a1, 0x84fc980a, 0x44d1c152,
    0x20c21094, 0x42104180, 0x3a000000, 0xd29d0240,
    } },
    { { /* 683 */
    0xa8b12f01, 0x2432bd40, 0xd04bd34d, 0xd0ada723,
    0x75a10a92, 0x01e9adac, 0x771f801a, 0xa01b9225,
    } },
    { { /* 684 */
    0x20cadfa1, 0x738c0602, 0x003b577f, 0x00d00bff,
    0x0088806a, 0x0029a1c4, 0x05242a05, 0x16234009,
    } },
    { { /* 685 */
    0x80056822, 0xa2112011, 0x64900004, 0x13824849,
    0x193023d5, 0x08922980, 0x88115402, 0xa0042001,
    } },
    { { /* 686 */
    0x81800400, 0x60228502, 0x0b010090, 0x12020022,
    0x00834011, 0x00001a01, 0x00000000, 0x00000000,
    } },
    { { /* 687 */
    0x00000000, 0x4684009f, 0x020012c8, 0x1a0004fc,
    0x0c4c2ede, 0x80b80402, 0x0afca826, 0x22288c02,
    } },
    { { /* 688 */
    0x8f7ba0e0, 0x2135c7d6, 0xf8b106c7, 0x62550713,
    0x8a19936e, 0xfb0e6efa, 0x48f91630, 0x7debcd2f,
    } },
    { { /* 689 */
    0x4e845892, 0x7a2e4ca0, 0x561eedea, 0x1190c649,
    0xe83a5324, 0x8124cfdb, 0x634218f1, 0x1a8a5853,
    } },
    { { /* 690 */
    0x24d37420, 0x0514aa3b, 0x89586018, 0xc0004800,
    0x91018268, 0x2cd684a4, 0xc4ba8886, 0x02100377,
    } },
    { { /* 691 */
    0x00388244, 0x404aae11, 0x510028c0, 0x15146044,
    0x10007310, 0x02480082, 0x40060205, 0x0000c003,
    } },
    { { /* 692 */
    0x0c020000, 0x02200008, 0x40009000, 0xd161b800,
    0x32744621, 0x3b8af800, 0x8b00050f, 0x2280bbd0,
    } },
    { { /* 693 */
    0x07690600, 0x00438040, 0x50005420, 0x250c41d0,
    0x83108410, 0x02281101, 0x00304008, 0x020040a1,
    } },
    { { /* 694 */
    0x20000040, 0xabe31500, 0xaa443180, 0xc624c2c6,
    0x8004ac13, 0x03d1b000, 0x4285611e, 0x1d9ff303,
    } },
    { { /* 695 */
    0x78e8440a, 0xc3925e26, 0x00852000, 0x4000b001,
    0x88424a90, 0x0c8dca04, 0x4203a705, 0x000422a1,
    } },
    { { /* 696 */
    0x0c018668, 0x10795564, 0xdea00002, 0x40c12000,
    0x5001488b, 0x04000380, 0x50040000, 0x80d0c05d,
    } },
    { { /* 697 */
    0x970aa010, 0x4dafbb20, 0x1e10d921, 0x83140460,
    0xa6d68848, 0x733fd83b, 0x497427bc, 0x92130ddc,
    } },
    { { /* 698 */
    0x8ba1142b, 0xd1392e75, 0x50503009, 0x69008808,
    0x024a49d4, 0x80164010, 0x89d7e564, 0x5316c020,
    } },
    { { /* 699 */
    0x86002b92, 0x15e0a345, 0x0c03008b, 0xe200196e,
    0x80067031, 0xa82916a5, 0x18802000, 0xe1487aac,
    } },
    { { /* 700 */
    0xb5d63207, 0x5f9132e8, 0x20e550a1, 0x10807c00,
    0x9d8a7280, 0x421f00aa, 0x02310e22, 0x04941100,
    } },
    { { /* 701 */
    0x40080022, 0x5c100010, 0xfcc80343, 0x0580a1a5,
    0x04008433, 0x6e080080, 0x81262a4b, 0x2901aad8,
    } },
    { { /* 702 */
    0x4490684d, 0xba880009, 0x00820040, 0x87d10000,
    0xb1e6215b, 0x80083161, 0xc2400800, 0xa600a069,
    } },
    { { /* 703 */
    0x4a328d58, 0x550a5d71, 0x2d579aa0, 0x4aa64005,
    0x30b12021, 0x01123fc6, 0x260a10c2, 0x50824462,
    } },
    { { /* 704 */
    0x80409880, 0x810004c0, 0x00002003, 0x38180000,
    0xf1a60200, 0x720e4434, 0x92e035a2, 0x09008101,
    } },
    { { /* 705 */
    0x00000400, 0x00008885, 0x00000000, 0x00804000,
    0x00000000, 0x00004040, 0x00000000, 0x00000000,
    } },
    { { /* 706 */
    0x00000000, 0x08000000, 0x00000082, 0x00000000,
    0x88000004, 0xe7efbfff, 0xffbfffff, 0xfdffefef,
    } },
    { { /* 707 */
    0xbffefbff, 0x057fffff, 0x85b30034, 0x42164706,
    0xe4105402, 0xb3058092, 0x81305422, 0x180b4263,
    } },
    { { /* 708 */
    0x13f5387b, 0xa9ea07e5, 0x05143c4c, 0x80020600,
    0xbd481ad9, 0xf496ee37, 0x7ec0705f, 0x355fbfb2,
    } },
    { { /* 709 */
    0x455fe644, 0x41469000, 0x063b1d40, 0xfe1362a1,
    0x39028505, 0x0c080548, 0x0000144f, 0x58183488,
    } },
    { { /* 710 */
    0xd8153077, 0x4bfbbd0e, 0x85008a90, 0xe61dc100,
    0xb386ed14, 0x639bff72, 0xd9befd92, 0x0a92887b,
    } },
    { { /* 711 */
    0x1cb2d3fe, 0x177ab980, 0xdc1782c9, 0x3980fffb,
    0x590c4260, 0x37df0f01, 0xb15094a3, 0x23070623,
    } },
    { { /* 712 */
    0x3102f85a, 0x310201f0, 0x1e820040, 0x056a3a0a,
    0x12805b84, 0xa7148002, 0xa04b2612, 0x90011069,
    } },
    { { /* 713 */
    0x848a1000, 0x3f801802, 0x42400708, 0x4e140110,
    0x180080b0, 0x0281c510, 0x10298202, 0x88000210,
    } },
    { { /* 714 */
    0x00420020, 0x11000280, 0x4413e000, 0xfe025804,
    0x30283c07, 0x04739798, 0xcb13ced1, 0x431f6210,
    } },
    { { /* 715 */
    0x55ac278d, 0xc892422e, 0x02885380, 0x78514039,
    0x8088292c, 0x2428b900, 0x080e0c41, 0x42004421,
    } },
    { { /* 716 */
    0x08680408, 0x12040006, 0x02903031, 0xe0855b3e,
    0x10442936, 0x10822814, 0x83344266, 0x531b013c,
    } },
    { { /* 717 */
    0x0e0d0404, 0x00510c22, 0xc0000012, 0x88000040,
    0x0000004a, 0x00000000, 0x5447dff6, 0x00088868,
    } },
    { { /* 718 */
    0x00000081, 0x40000000, 0x00000100, 0x02000000,
    0x00080600, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 719 */
    0x00000080, 0x00000040, 0x00000000, 0x00001040,
    0x00000000, 0xf7fdefff, 0xfffeff7f, 0xfffffbff,
    } },
    { { /* 720 */
    0xbffffdff, 0x00ffffff, 0x042012c2, 0x07080c06,
    0x01101624, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 721 */
    0xe0000000, 0xfffffffe, 0x7f79ffff, 0x00f928df,
    0x80120c32, 0xd53a0008, 0xecc2d858, 0x2fa89d18,
    } },
    { { /* 722 */
    0xe0109620, 0x2622d60c, 0x02060f97, 0x9055b240,
    0x501180a2, 0x04049800, 0x00004000, 0x00000000,
    } },
    { { /* 723 */
    0x00000000, 0x00000000, 0x00000000, 0xfffffbc0,
    0xdffbeffe, 0x62430b08, 0xfb3b41b6, 0x23896f74,
    } },
    { { /* 724 */
    0xecd7ae7f, 0x5960e047, 0x098fa096, 0xa030612c,
    0x2aaa090d, 0x4f7bd44e, 0x388bc4b2, 0x6110a9c6,
    } },
    { { /* 725 */
    0x42000014, 0x0202800c, 0x6485fe48, 0xe3f7d63e,
    0x0c073aa0, 0x0430e40c, 0x1002f680, 0x00000000,
    } },
    { { /* 726 */
    0x00000000, 0x00000000, 0x00000000, 0x00100000,
    0x00004000, 0x00004000, 0x00000100, 0x00000000,
    } },
    { { /* 727 */
    0x00000000, 0x40000000, 0x00000000, 0x00000400,
    0x00008000, 0x00000000, 0x00400400, 0x00000000,
    } },
    { { /* 728 */
    0x00000000, 0x40000000, 0x00000000, 0x00000800,
    0xfebdffe0, 0xffffffff, 0xfbe77f7f, 0xf7ffffbf,
    } },
    { { /* 729 */
    0xefffffff, 0xdff7ff7e, 0xfbdff6f7, 0x804fbffe,
    0x00000000, 0x00000000, 0x00000000, 0x7fffef00,
    } },
    { { /* 730 */
    0xb6f7ff7f, 0xb87e4406, 0x88313bf5, 0x00f41796,
    0x1391a960, 0x72490080, 0x0024f2f3, 0x42c88701,
    } },
    { { /* 731 */
    0x5048e3d3, 0x43052400, 0x4a4c0000, 0x10580227,
    0x01162820, 0x0014a809, 0x00000000, 0x00683ec0,
    } },
    { { /* 732 */
    0x00000000, 0x00000000, 0x00000000, 0xffe00000,
    0xfddbb7ff, 0x000000f7, 0xc72e4000, 0x00000180,
    } },
    { { /* 733 */
    0x00012000, 0x00004000, 0x00300000, 0xb4f7ffa8,
    0x03ffadf3, 0x00000120, 0x00000000, 0x00000000,
    } },
    { { /* 734 */
    0x00000000, 0x00000000, 0x00000000, 0xfffbf000,
    0xfdcf9df7, 0x15c301bf, 0x810a1827, 0x0a00a842,
    } },
    { { /* 735 */
    0x80088108, 0x18048008, 0x0012a3be, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000,
    } },
    { { /* 736 */
    0x00000000, 0x00000000, 0x00000000, 0x90000000,
    0xdc3769e6, 0x3dff6bff, 0xf3f9fcf8, 0x00000004,
    } },
    { { /* 737 */
    0x80000000, 0xe7eebf6f, 0x5da2dffe, 0xc00b3fd8,
    0xa00c0984, 0x69100040, 0xb912e210, 0x5a0086a5,
    } },
    { { /* 738 */
    0x02896800, 0x6a809005, 0x00030010, 0x80000000,
    0x8e001ff9, 0x00000001, 0x00000000, 0x00000000,
    } },
},
{
    /* aa */
    LEAF(  0,  0),
    /* ab */
    LEAF(  1,  1),
    /* af */
    LEAF(  2,  2), LEAF(  2,  3),
    /* agr */
    LEAF(  4,  4),
    /* ak */
    LEAF(  5,  5), LEAF(  5,  6), LEAF(  5,  7), LEAF(  5,  8),
    LEAF(  5,  9),
    /* am */
    LEAF( 10, 10), LEAF( 10, 11),
    /* an */
    LEAF( 12, 12),
    /* anp */
    LEAF( 13, 13),
    /* ar */
    LEAF( 14, 14),
    /* as */
    LEAF( 15, 15),
    /* ast */
    LEAF( 16, 12), LEAF( 16, 16),
    /* av */
    LEAF( 18, 17),
    /* ay */
    LEAF( 19, 18),
    /* ayc */
    LEAF( 20, 19),
    /* az_az */
    LEAF( 21, 20), LEAF( 21, 21), LEAF( 21, 22),
    /* az_ir */
    LEAF( 24, 23),
    /* ba */
    LEAF( 25, 24),
    /* be */
    LEAF( 26, 25),
    /* bem */
    LEAF( 27, 26),
    /* ber_dz */
    LEAF( 28, 26), LEAF( 28, 27), LEAF( 28, 28), LEAF( 28, 29),
    /* ber_ma */
    LEAF( 32, 30),
    /* bg */
    LEAF( 33, 31),
    /* bi */
    LEAF( 34, 32),
    /* bin */
    LEAF( 35, 33), LEAF( 35, 34), LEAF( 35, 35),
    /* bm */
    LEAF( 38, 26), LEAF( 38, 36), LEAF( 38, 37),
    /* bn */
    LEAF( 41, 38),
    /* bo */
    LEAF( 42, 39),
    /* br */
    LEAF( 43, 40),
    /* brx */
    LEAF( 44, 41),
    /* bs */
    LEAF( 45, 26), LEAF( 45, 42),
    /* bua */
    LEAF( 47, 43),
    /* byn */
    LEAF( 48, 44), LEAF( 48, 45),
    /* ca */
    LEAF( 50, 46), LEAF( 50, 47),
    /* ch */
    LEAF( 52, 48),
    /* chm */
    LEAF( 53, 49),
    /* chr */
    LEAF( 54, 50),
    /* ckb */
    LEAF( 55, 51),
    /* cmn */
    LEAF( 56, 52), LEAF( 56, 53), LEAF( 56, 54), LEAF( 56, 55),
    LEAF( 56, 56), LEAF( 56, 57), LEAF( 56, 58), LEAF( 56, 59),
    LEAF( 56, 60), LEAF( 56, 61), LEAF( 56, 62), LEAF( 56, 63),
    LEAF( 56, 64), LEAF( 56, 65), LEAF( 56, 66), LEAF( 56, 67),
    LEAF( 56, 68), LEAF( 56, 69), LEAF( 56, 70), LEAF( 56, 71),
    LEAF( 56, 72), LEAF( 56, 73), LEAF( 56, 74), LEAF( 56, 75),
    LEAF( 56, 76), LEAF( 56, 77), LEAF( 56, 78), LEAF( 56, 79),
    LEAF( 56, 80), LEAF( 56, 81), LEAF( 56, 82), LEAF( 56, 83),
    LEAF( 56, 84), LEAF( 56, 85), LEAF( 56, 86), LEAF( 56, 87),
    LEAF( 56, 88), LEAF( 56, 89), LEAF( 56, 90), LEAF( 56, 91),
    LEAF( 56, 92), LEAF( 56, 93), LEAF( 56, 94), LEAF( 56, 95),
    LEAF( 56, 96), LEAF( 56, 97), LEAF( 56, 98), LEAF( 56, 99),
    LEAF( 56,100), LEAF( 56,101), LEAF( 56,102), LEAF( 56,103),
    LEAF( 56,104), LEAF( 56,105), LEAF( 56,106), LEAF( 56,107),
    LEAF( 56,108), LEAF( 56,109), LEAF( 56,110), LEAF( 56,111),
    LEAF( 56,112), LEAF( 56,113), LEAF( 56,114), LEAF( 56,115),
    LEAF( 56,116), LEAF( 56,117), LEAF( 56,118), LEAF( 56,119),
    LEAF( 56,120), LEAF( 56,121), LEAF( 56,122), LEAF( 56,123),
    LEAF( 56,124), LEAF( 56,125), LEAF( 56,126), LEAF( 56,127),
    LEAF( 56,128), LEAF( 56,129), LEAF( 56,130), LEAF( 56,131),
    LEAF( 56,132), LEAF( 56,133), LEAF( 56,134),
    /* co */
    LEAF(139,135), LEAF(139,136),
    /* crh */
    LEAF(141,137), LEAF(141,138),
    /* cs */
    LEAF(143,139), LEAF(143,140),
    /* csb */
    LEAF(145,141), LEAF(145,142),
    /* cu */
    LEAF(147,143),
    /* cv */
    LEAF(148,144), LEAF(148,145),
    /* cy */
    LEAF(150,146), LEAF(150,147), LEAF(150,148),
    /* da */
    LEAF(153,149),
    /* de */
    LEAF(154,150),
    /* doi */
    LEAF(155,151),
    /* dv */
    LEAF(156,152),
    /* ee */
    LEAF(157, 33), LEAF(157,153), LEAF(157,154), LEAF(157,155),
    /* el */
    LEAF(161,156),
    /* en */
    LEAF(162,157),
    /* eo */
    LEAF(163, 26), LEAF(163,158),
    /* et */
    LEAF(165,159), LEAF(165,160),
    /* eu */
    LEAF(167,161),
    /* ff */
    LEAF(168, 26), LEAF(168,162), LEAF(168,163),
    /* fi */
    LEAF(171,164), LEAF(171,160),
    /* fil */
    LEAF(173,165),
    /* fo */
    LEAF(174,166),
    /* fur */
    LEAF(175,167),
    /* fy */
    LEAF(176,168),
    /* ga */
    LEAF(177,169), LEAF(177,170), LEAF(177,171),
    /* gd */
    LEAF(180,172),
    /* gez */
    LEAF(181,173), LEAF(181,174),
    /* gn */
    LEAF(183,175), LEAF(183,176), LEAF(183,177),
    /* gu */
    LEAF(186,178),
    /* gv */
    LEAF(187,179),
    /* ha */
    LEAF(188, 26), LEAF(188,180), LEAF(188,181),
    /* haw */
    LEAF(191, 26), LEAF(191,182), LEAF(191,183),
    /* he */
    LEAF(194,184),
    /* hsb */
    LEAF(195,185), LEAF(195,186),
    /* ht */
    LEAF(197,187),
    /* hu */
    LEAF(198,188), LEAF(198,189),
    /* hy */
    LEAF(200,190),
    /* hz */
    LEAF(201, 26), LEAF(201,191), LEAF(201,192),
    /* id */
    LEAF(204,193),
    /* ie */
    LEAF(205,139),
    /* ig */
    LEAF(206, 26), LEAF(206,194),
    /* ii */
    LEAF(208,195), LEAF(208,195), LEAF(208,195), LEAF(208,195),
    LEAF(208,196),
    /* ik */
    LEAF(213,197),
    /* is */
    LEAF(214,198),
    /* it */
    LEAF(215,199),
    /* iu */
    LEAF(216,200), LEAF(216,201), LEAF(216,202),
    /* ja */
    LEAF(219,203), LEAF(219,204), LEAF(219,205), LEAF(219,206),
    LEAF(219,207), LEAF(219,208), LEAF(219,209), LEAF(219,210),
    LEAF(219,211), LEAF(219,212), LEAF(219,213), LEAF(219,214),
    LEAF(219,215), LEAF(219,216), LEAF(219,217), LEAF(219,218),
    LEAF(219,219), LEAF(219,220), LEAF(219,221), LEAF(219,222),
    LEAF(219,223), LEAF(219,224), LEAF(219,225), LEAF(219,226),
    LEAF(219,227), LEAF(219,228), LEAF(219,229), LEAF(219,230),
    LEAF(219,231), LEAF(219,232), LEAF(219,233), LEAF(219,234),
    LEAF(219,235), LEAF(219,236), LEAF(219,237), LEAF(219,238),
    LEAF(219,239), LEAF(219,240), LEAF(219,241), LEAF(219,242),
    LEAF(219,243), LEAF(219,244), LEAF(219,245), LEAF(219,246),
    LEAF(219,247), LEAF(219,248), LEAF(219,249), LEAF(219,250),
    LEAF(219,251), LEAF(219,252), LEAF(219,253), LEAF(219,254),
    LEAF(219,255), LEAF(219,256), LEAF(219,257), LEAF(219,258),
    LEAF(219,259), LEAF(219,260), LEAF(219,261), LEAF(219,262),
    LEAF(219,263), LEAF(219,264), LEAF(219,265), LEAF(219,266),
    LEAF(219,267), LEAF(219,268), LEAF(219,269), LEAF(219,270),
    LEAF(219,271), LEAF(219,272), LEAF(219,273), LEAF(219,274),
    LEAF(219,275), LEAF(219,276), LEAF(219,277), LEAF(219,278),
    LEAF(219,279), LEAF(219,280), LEAF(219,281), LEAF(219,282),
    LEAF(219,283), LEAF(219,284), LEAF(219,285),
    /* jv */
    LEAF(302,286),
    /* ka */
    LEAF(303,287),
    /* kaa */
    LEAF(304,288),
    /* ki */
    LEAF(305, 26), LEAF(305,289),
    /* kk */
    LEAF(307,290),
    /* kl */
    LEAF(308,291), LEAF(308,292),
    /* km */
    LEAF(310,293),
    /* kn */
    LEAF(311,294),
    /* ko */
    LEAF(312,295), LEAF(312,296), LEAF(312,297), LEAF(312,298),
    LEAF(312,299), LEAF(312,300), LEAF(312,301), LEAF(312,302),
    LEAF(312,303), LEAF(312,304), LEAF(312,305), LEAF(312,306),
    LEAF(312,307), LEAF(312,308), LEAF(312,309), LEAF(312,310),
    LEAF(312,311), LEAF(312,312), LEAF(312,313), LEAF(312,314),
    LEAF(312,315), LEAF(312,316), LEAF(312,317), LEAF(312,318),
    LEAF(312,319), LEAF(312,320), LEAF(312,321), LEAF(312,322),
    LEAF(312,323), LEAF(312,324), LEAF(312,325), LEAF(312,326),
    LEAF(312,327), LEAF(312,328), LEAF(312,329), LEAF(312,330),
    LEAF(312,331), LEAF(312,332), LEAF(312,333), LEAF(312,334),
    LEAF(312,335), LEAF(312,336), LEAF(312,337), LEAF(312,338),
    LEAF(312,339),
    /* kr */
    LEAF(357, 26), LEAF(357,340), LEAF(357,341),
    /* ks */
    LEAF(360,342),
    /* ku_am */
    LEAF(361,343), LEAF(361,344),
    /* ku_tr */
    LEAF(363,345), LEAF(363,346),
    /* kum */
    LEAF(365,347),
    /* kv */
    LEAF(366,348),
    /* kw */
    LEAF(367, 26), LEAF(367,182), LEAF(367,349),
    /* ky */
    LEAF(370,350),
    /* la */
    LEAF(371, 26), LEAF(371,351),
    /* lah */
    LEAF(373,352),
    /* lb */
    LEAF(374,353),
    /* lg */
    LEAF(375, 26), LEAF(375,354),
    /* li */
    LEAF(377,355),
    /* lij */
    LEAF(378,356),
    /* ln */
    LEAF(379,357), LEAF(379,358), LEAF(379,  7), LEAF(379,359),
    /* lo */
    LEAF(383,360),
    /* lt */
    LEAF(384, 26), LEAF(384,361),
    /* lv */
    LEAF(386, 26), LEAF(386,362),
    /* mg */
    LEAF(388,363),
    /* mh */
    LEAF(389, 26), LEAF(389,364),
    /* mi */
    LEAF(391, 26), LEAF(391,182), LEAF(391,365),
    /* miq */
    LEAF(394,366), LEAF(394,176), LEAF(394,367),
    /* mk */
    LEAF(397,368),
    /* ml */
    LEAF(398,369),
    /* mn_cn */
    LEAF(399,370),
    /* mn_mn */
    LEAF(400,371),
    /* mni */
    LEAF(401,372),
    /* mnw */
    LEAF(402,373),
    /* mo */
    LEAF(403,374), LEAF(403,144), LEAF(403,375), LEAF(403,347),
    /* mt */
    LEAF(407,376), LEAF(407,377),
    /* na */
    LEAF(409,  5), LEAF(409,378),
    /* nan */
    LEAF(411,157), LEAF(411, 52), LEAF(411, 53), LEAF(411, 54),
    LEAF(411, 55), LEAF(411, 56), LEAF(411, 57), LEAF(411, 58),
    LEAF(411, 59), LEAF(411, 60), LEAF(411, 61), LEAF(411, 62),
    LEAF(411, 63), LEAF(411, 64), LEAF(411, 65), LEAF(411, 66),
    LEAF(411, 67), LEAF(411, 68), LEAF(411, 69), LEAF(411, 70),
    LEAF(411, 71), LEAF(411, 72), LEAF(411, 73), LEAF(411, 74),
    LEAF(411, 75), LEAF(411, 76), LEAF(411, 77), LEAF(411, 78),
    LEAF(411, 79), LEAF(411, 80), LEAF(411, 81), LEAF(411, 82),
    LEAF(411, 83), LEAF(411, 84), LEAF(411, 85), LEAF(411, 86),
    LEAF(411, 87), LEAF(411, 88), LEAF(411, 89), LEAF(411, 90),
    LEAF(411, 91), LEAF(411, 92), LEAF(411, 93), LEAF(411, 94),
    LEAF(411, 95), LEAF(411, 96), LEAF(411, 97), LEAF(411, 98),
    LEAF(411, 99), LEAF(411,100), LEAF(411,101), LEAF(411,102),
    LEAF(411,103), LEAF(411,104), LEAF(411,105), LEAF(411,106),
    LEAF(411,107), LEAF(411,108), LEAF(411,109), LEAF(411,110),
    LEAF(411,111), LEAF(411,112), LEAF(411,113), LEAF(411,114),
    LEAF(411,115), LEAF(411,116), LEAF(411,117), LEAF(411,118),
    LEAF(411,119), LEAF(411,120), LEAF(411,121), LEAF(411,122),
    LEAF(411,123), LEAF(411,124), LEAF(411,125), LEAF(411,126),
    LEAF(411,127), LEAF(411,128), LEAF(411,129), LEAF(411,130),
    LEAF(411,131), LEAF(411,132), LEAF(411,133), LEAF(411,134),
    /* nb */
    LEAF(495,379),
    /* ne */
    LEAF(496,380),
    /* nhn */
    LEAF(497, 12), LEAF(497,381),
    /* niu */
    LEAF(499,157), LEAF(499,382),
    /* nl */
    LEAF(501,383),
    /* nn */
    LEAF(502,384),
    /* nqo */
    LEAF(503,385),
    /* nso */
    LEAF(504,386), LEAF(504,387),
    /* nv */
    LEAF(506,388), LEAF(506,389), LEAF(506,390), LEAF(506,391),
    /* ny */
    LEAF(510, 26), LEAF(510,392),
    /* oc */
    LEAF(512,393),
    /* or */
    LEAF(513,394),
    /* ota */
    LEAF(514,395),
    /* pa */
    LEAF(515,396),
    /* pap_an */
    LEAF(516,397),
    /* pap_aw */
    LEAF(517,398),
    /* pl */
    LEAF(518,185), LEAF(518,399),
    /* ps_af */
    LEAF(520,400),
    /* ps_pk */
    LEAF(521,401),
    /* pt */
    LEAF(522,402),
    /* qu */
    LEAF(523,398), LEAF(523,403),
    /* rif */
    LEAF(525,157), LEAF(525,404), LEAF(525, 28), LEAF(525,405),
    /* rm */
    LEAF(529,406),
    /* ro */
    LEAF(530,374), LEAF(530,144), LEAF(530,375),
    /* sah */
    LEAF(533,407),
    /* sat */
    LEAF(534,408),
    /* sc */
    LEAF(535,409),
    /* sco */
    LEAF(536, 26), LEAF(536,410), LEAF(536,411),
    /* sd */
    LEAF(539,412),
    /* se */
    LEAF(540,413), LEAF(540,414),
    /* sg */
    LEAF(542,415),
    /* sgs */
    LEAF(543,416), LEAF(543,417), LEAF(543,418),
    /* sh */
    LEAF(546, 26), LEAF(546, 42), LEAF(546,419),
    /* shs */
    LEAF(549,420), LEAF(549,421),
    /* si */
    LEAF(551,422),
    /* sid */
    LEAF(552,423), LEAF(552, 11),
    /* sk */
    LEAF(554,424), LEAF(554,425),
    /* sm */
    LEAF(556, 26), LEAF(556,183),
    /* sma */
    LEAF(558,426),
    /* smj */
    LEAF(559,427),
    /* smn */
    LEAF(560,428), LEAF(560,429),
    /* sms */
    LEAF(562,430), LEAF(562,431), LEAF(562,432),
    /* sq */
    LEAF(565,433),
    /* sr */
    LEAF(566,434),
    /* sv */
    LEAF(567,435),
    /* syr */
    LEAF(568,436),
    /* szl */
    LEAF(569,437), LEAF(569,438),
    /* ta */
    LEAF(571,439),
    /* te */
    LEAF(572,440),
    /* tg */
    LEAF(573,441),
    /* th */
    LEAF(574,442),
    /* tig */
    LEAF(575,443), LEAF(575, 45),
    /* tk */
    LEAF(577,444), LEAF(577,445),
    /* tr */
    LEAF(579,446), LEAF(579,138),
    /* tt */
    LEAF(581,447),
    /* ty */
    LEAF(582,448), LEAF(582,182), LEAF(582,390),
    /* ug */
    LEAF(585,449),
    /* uk */
    LEAF(586,450),
    /* und_zmth */
    LEAF(587,451), LEAF(587,452), LEAF(587,453), LEAF(587,454),
    LEAF(587,455), LEAF(587,456), LEAF(587,457), LEAF(587,458),
    LEAF(587,459), LEAF(587,460), LEAF(587,461), LEAF(587,462),
    /* und_zsye */
    LEAF(599,463), LEAF(599,464), LEAF(599,465), LEAF(599,466),
    LEAF(599,467), LEAF(599,468), LEAF(599,469), LEAF(599,470),
    LEAF(599,471), LEAF(599,472), LEAF(599,473), LEAF(599,474),
    /* ve */
    LEAF(611, 26), LEAF(611,475),
    /* vi */
    LEAF(613,476), LEAF(613,477), LEAF(613,478), LEAF(613,479),
    /* vo */
    LEAF(617,480),
    /* vot */
    LEAF(618,481), LEAF(618,160),
    /* wa */
    LEAF(620,482),
    /* wen */
    LEAF(621,185), LEAF(621,483),
    /* wo */
    LEAF(623,484), LEAF(623,354),
    /* yap */
    LEAF(625,485),
    /* yo */
    LEAF(626,486), LEAF(626,487), LEAF(626,488), LEAF(626,489),
    /* yue */
    LEAF(630,490), LEAF(630,491), LEAF(630,492), LEAF(630,493),
    LEAF(630,494), LEAF(630,495), LEAF(630,496), LEAF(630,497),
    LEAF(630,498), LEAF(630,499), LEAF(630,500), LEAF(630,501),
    LEAF(630,502), LEAF(630,503), LEAF(630,504), LEAF(630,505),
    LEAF(630,506), LEAF(630,507), LEAF(630,508), LEAF(630,509),
    LEAF(630,510), LEAF(630,511), LEAF(630,512), LEAF(630,513),
    LEAF(630,514), LEAF(630,515), LEAF(630,516), LEAF(630,517),
    LEAF(630,518), LEAF(630,519), LEAF(630,520), LEAF(630,521),
    LEAF(630,522), LEAF(630,523), LEAF(630,524), LEAF(630,525),
    LEAF(630,526), LEAF(630,527), LEAF(630,528), LEAF(630,529),
    LEAF(630,530), LEAF(630,531), LEAF(630,532), LEAF(630,533),
    LEAF(630,534), LEAF(630,535), LEAF(630,536), LEAF(630,537),
    LEAF(630,538), LEAF(630,539), LEAF(630,540), LEAF(630,541),
    LEAF(630,542), LEAF(630,543), LEAF(630,544), LEAF(630,545),
    LEAF(630,546), LEAF(630,547), LEAF(630,548), LEAF(630,549),
    LEAF(630,550), LEAF(630,551), LEAF(630,552), LEAF(630,553),
    LEAF(630,554), LEAF(630,555), LEAF(630,556), LEAF(630,557),
    LEAF(630,558), LEAF(630,559), LEAF(630,560), LEAF(630,561),
    LEAF(630,562), LEAF(630,563), LEAF(630,564), LEAF(630,565),
    LEAF(630,566), LEAF(630,567), LEAF(630,568), LEAF(630,569),
    LEAF(630,570), LEAF(630,571), LEAF(630,572), LEAF(630,573),
    LEAF(630,574), LEAF(630,575), LEAF(630,576), LEAF(630,577),
    LEAF(630,578), LEAF(630,579), LEAF(630,580), LEAF(630,581),
    LEAF(630,582), LEAF(630,583), LEAF(630,584), LEAF(630,585),
    LEAF(630,586), LEAF(630,587), LEAF(630,588), LEAF(630,589),
    LEAF(630,590), LEAF(630,591), LEAF(630,592), LEAF(630,593),
    LEAF(630,594), LEAF(630,595), LEAF(630,596), LEAF(630,597),
    LEAF(630,598), LEAF(630,599), LEAF(630,600), LEAF(630,601),
    LEAF(630,602), LEAF(630,603), LEAF(630,604), LEAF(630,605),
    LEAF(630,606), LEAF(630,607), LEAF(630,608), LEAF(630,609),
    LEAF(630,610), LEAF(630,611), LEAF(630,612), LEAF(630,613),
    LEAF(630,614), LEAF(630,615), LEAF(630,616), LEAF(630,617),
    LEAF(630,618), LEAF(630,619), LEAF(630,452), LEAF(630,620),
    LEAF(630,621), LEAF(630,410), LEAF(630,622), LEAF(630,623),
    LEAF(630,624), LEAF(630,625), LEAF(630,626), LEAF(630,627),
    LEAF(630,628), LEAF(630,  3), LEAF(630,629), LEAF(630,630),
    LEAF(630,631), LEAF(630,632), LEAF(630,633), LEAF(630,634),
    LEAF(630,619), LEAF(630,635), LEAF(630,636), LEAF(630,637),
    LEAF(630,638), LEAF(630,639), LEAF(630,640), LEAF(630,641),
    LEAF(630,642), LEAF(630,643), LEAF(630,644), LEAF(630,645),
    LEAF(630,646), LEAF(630,647), LEAF(630,648), LEAF(630,649),
    LEAF(630,650), LEAF(630,651), LEAF(630,652), LEAF(630,653),
    LEAF(630,654), LEAF(630,655), LEAF(630,656),
    /* zh_cn */
    LEAF(801,657), LEAF(801,658), LEAF(801,659), LEAF(801,660),
    LEAF(801,661), LEAF(801,662), LEAF(801,663), LEAF(801,664),
    LEAF(801,665), LEAF(801,666), LEAF(801,667), LEAF(801,668),
    LEAF(801,669), LEAF(801,670), LEAF(801,671), LEAF(801,672),
    LEAF(801,673), LEAF(801,674), LEAF(801,675), LEAF(801,676),
    LEAF(801,677), LEAF(801,678), LEAF(801,679), LEAF(801,680),
    LEAF(801,681), LEAF(801,682), LEAF(801,683), LEAF(801,684),
    LEAF(801,685), LEAF(801,686), LEAF(801,687), LEAF(801,688),
    LEAF(801,689), LEAF(801,690), LEAF(801,691), LEAF(801,692),
    LEAF(801,693), LEAF(801,694), LEAF(801,695), LEAF(801,696),
    LEAF(801,697), LEAF(801,698), LEAF(801,699), LEAF(801,700),
    LEAF(801,701), LEAF(801,702), LEAF(801,703), LEAF(801,704),
    LEAF(801,705), LEAF(801,706), LEAF(801,707), LEAF(801,708),
    LEAF(801,709), LEAF(801,710), LEAF(801,711), LEAF(801,712),
    LEAF(801,713), LEAF(801,714), LEAF(801,715), LEAF(801,716),
    LEAF(801,717), LEAF(801,718), LEAF(801,719), LEAF(801,720),
    LEAF(801,721), LEAF(801,722), LEAF(801,723), LEAF(801,724),
    LEAF(801,725), LEAF(801,726), LEAF(801,727), LEAF(801,728),
    LEAF(801,729), LEAF(801,730), LEAF(801,731), LEAF(801,732),
    LEAF(801,733), LEAF(801,734), LEAF(801,735), LEAF(801,736),
    LEAF(801,737), LEAF(801,738),
},
{
    /* aa */
    0x0000,
    /* ab */
    0x0004,
    /* af */
    0x0000, 0x0001,
    /* agr */
    0x0000,
    /* ak */
    0x0000, 0x0001, 0x0002, 0x0003, 0x001e,
    /* am */
    0x0012, 0x0013,
    /* an */
    0x0000,
    /* anp */
    0x0009,
    /* ar */
    0x0006,
    /* as */
    0x0009,
    /* ast */
    0x0000, 0x001e,
    /* av */
    0x0004,
    /* ay */
    0x0000,
    /* ayc */
    0x0000,
    /* az_az */
    0x0000, 0x0001, 0x0002,
    /* az_ir */
    0x0006,
    /* ba */
    0x0004,
    /* be */
    0x0004,
    /* bem */
    0x0000,
    /* ber_dz */
    0x0000, 0x0001, 0x0002, 0x001e,
    /* ber_ma */
    0x002d,
    /* bg */
    0x0004,
    /* bi */
    0x0000,
    /* bin */
    0x0000, 0x0003, 0x001e,
    /* bm */
    0x0000, 0x0001, 0x0002,
    /* bn */
    0x0009,
    /* bo */
    0x000f,
    /* br */
    0x0000,
    /* brx */
    0x0009,
    /* bs */
    0x0000, 0x0001,
    /* bua */
    0x0004,
    /* byn */
    0x0012, 0x0013,
    /* ca */
    0x0000, 0x0001,
    /* ch */
    0x0000,
    /* chm */
    0x0004,
    /* chr */
    0x0013,
    /* ckb */
    0x0006,
    /* cmn */
    0x004e, 0x004f, 0x0050, 0x0051, 0x0052, 0x0053, 0x0054, 0x0055,
    0x0056, 0x0057, 0x0058, 0x0059, 0x005a, 0x005b, 0x005c, 0x005d,
    0x005e, 0x005f, 0x0060, 0x0061, 0x0062, 0x0063, 0x0064, 0x0065,
    0x0066, 0x0067, 0x0068, 0x0069, 0x006a, 0x006b, 0x006c, 0x006d,
    0x006e, 0x006f, 0x0070, 0x0071, 0x0072, 0x0073, 0x0074, 0x0075,
    0x0076, 0x0077, 0x0078, 0x0079, 0x007a, 0x007b, 0x007c, 0x007d,
    0x007e, 0x007f, 0x0080, 0x0081, 0x0082, 0x0083, 0x0084, 0x0085,
    0x0086, 0x0087, 0x0088, 0x0089, 0x008a, 0x008b, 0x008c, 0x008d,
    0x008e, 0x008f, 0x0090, 0x0091, 0x0092, 0x0093, 0x0094, 0x0095,
    0x0096, 0x0097, 0x0098, 0x0099, 0x009a, 0x009b, 0x009c, 0x009d,
    0x009e, 0x009f, 0x00fa,
    /* co */
    0x0000, 0x0001,
    /* crh */
    0x0000, 0x0001,
    /* cs */
    0x0000, 0x0001,
    /* csb */
    0x0000, 0x0001,
    /* cu */
    0x0004,
    /* cv */
    0x0001, 0x0004,
    /* cy */
    0x0000, 0x0001, 0x001e,
    /* da */
    0x0000,
    /* de */
    0x0000,
    /* doi */
    0x0009,
    /* dv */
    0x0007,
    /* ee */
    0x0000, 0x0001, 0x0002, 0x0003,
    /* el */
    0x0003,
    /* en */
    0x0000,
    /* eo */
    0x0000, 0x0001,
    /* et */
    0x0000, 0x0001,
    /* eu */
    0x0000,
    /* ff */
    0x0000, 0x0001, 0x0002,
    /* fi */
    0x0000, 0x0001,
    /* fil */
    0x0000,
    /* fo */
    0x0000,
    /* fur */
    0x0000,
    /* fy */
    0x0000,
    /* ga */
    0x0000, 0x0001, 0x001e,
    /* gd */
    0x0000,
    /* gez */
    0x0012, 0x0013,
    /* gn */
    0x0000, 0x0001, 0x001e,
    /* gu */
    0x000a,
    /* gv */
    0x0000,
    /* ha */
    0x0000, 0x0001, 0x0002,
    /* haw */
    0x0000, 0x0001, 0x0002,
    /* he */
    0x0005,
    /* hsb */
    0x0000, 0x0001,
    /* ht */
    0x0000,
    /* hu */
    0x0000, 0x0001,
    /* hy */
    0x0005,
    /* hz */
    0x0000, 0x0003, 0x001e,
    /* id */
    0x0000,
    /* ie */
    0x0000,
    /* ig */
    0x0000, 0x001e,
    /* ii */
    0x00a0, 0x00a1, 0x00a2, 0x00a3, 0x00a4,
    /* ik */
    0x0004,
    /* is */
    0x0000,
    /* it */
    0x0000,
    /* iu */
    0x0014, 0x0015, 0x0016,
    /* ja */
    0x0030, 0x004e, 0x004f, 0x0050, 0x0051, 0x0052, 0x0053, 0x0054,
    0x0055, 0x0056, 0x0057, 0x0058, 0x0059, 0x005a, 0x005b, 0x005c,
    0x005d, 0x005e, 0x005f, 0x0060, 0x0061, 0x0062, 0x0063, 0x0064,
    0x0065, 0x0066, 0x0067, 0x0068, 0x0069, 0x006a, 0x006b, 0x006c,
    0x006d, 0x006e, 0x006f, 0x0070, 0x0071, 0x0072, 0x0073, 0x0074,
    0x0075, 0x0076, 0x0077, 0x0078, 0x0079, 0x007a, 0x007b, 0x007c,
    0x007d, 0x007e, 0x007f, 0x0080, 0x0081, 0x0082, 0x0083, 0x0084,
    0x0085, 0x0086, 0x0087, 0x0088, 0x0089, 0x008a, 0x008b, 0x008c,
    0x008d, 0x008e, 0x008f, 0x0090, 0x0091, 0x0092, 0x0093, 0x0094,
    0x0095, 0x0096, 0x0097, 0x0098, 0x0099, 0x009a, 0x009b, 0x009c,
    0x009d, 0x009e, 0x009f,
    /* jv */
    0x0000,
    /* ka */
    0x0010,
    /* kaa */
    0x0004,
    /* ki */
    0x0000, 0x0001,
    /* kk */
    0x0004,
    /* kl */
    0x0000, 0x0001,
    /* km */
    0x0017,
    /* kn */
    0x000c,
    /* ko */
    0x0031, 0x00ac, 0x00ad, 0x00ae, 0x00af, 0x00b0, 0x00b1, 0x00b2,
    0x00b3, 0x00b4, 0x00b5, 0x00b6, 0x00b7, 0x00b8, 0x00b9, 0x00ba,
    0x00bb, 0x00bc, 0x00bd, 0x00be, 0x00bf, 0x00c0, 0x00c1, 0x00c2,
    0x00c3, 0x00c4, 0x00c5, 0x00c6, 0x00c7, 0x00c8, 0x00c9, 0x00ca,
    0x00cb, 0x00cc, 0x00cd, 0x00ce, 0x00cf, 0x00d0, 0x00d1, 0x00d2,
    0x00d3, 0x00d4, 0x00d5, 0x00d6, 0x00d7,
    /* kr */
    0x0000, 0x0001, 0x0002,
    /* ks */
    0x0006,
    /* ku_am */
    0x0004, 0x0005,
    /* ku_tr */
    0x0000, 0x0001,
    /* kum */
    0x0004,
    /* kv */
    0x0004,
    /* kw */
    0x0000, 0x0001, 0x0002,
    /* ky */
    0x0004,
    /* la */
    0x0000, 0x0001,
    /* lah */
    0x0006,
    /* lb */
    0x0000,
    /* lg */
    0x0000, 0x0001,
    /* li */
    0x0000,
    /* lij */
    0x0000,
    /* ln */
    0x0000, 0x0001, 0x0002, 0x0003,
    /* lo */
    0x000e,
    /* lt */
    0x0000, 0x0001,
    /* lv */
    0x0000, 0x0001,
    /* mg */
    0x0000,
    /* mh */
    0x0000, 0x0001,
    /* mi */
    0x0000, 0x0001, 0x001e,
    /* miq */
    0x0000, 0x0001, 0x001e,
    /* mk */
    0x0004,
    /* ml */
    0x000d,
    /* mn_cn */
    0x0018,
    /* mn_mn */
    0x0004,
    /* mni */
    0x0009,
    /* mnw */
    0x0010,
    /* mo */
    0x0000, 0x0001, 0x0002, 0x0004,
    /* mt */
    0x0000, 0x0001,
    /* na */
    0x0000, 0x0001,
    /* nan */
    0x0000, 0x004e, 0x004f, 0x0050, 0x0051, 0x0052, 0x0053, 0x0054,
    0x0055, 0x0056, 0x0057, 0x0058, 0x0059, 0x005a, 0x005b, 0x005c,
    0x005d, 0x005e, 0x005f, 0x0060, 0x0061, 0x0062, 0x0063, 0x0064,
    0x0065, 0x0066, 0x0067, 0x0068, 0x0069, 0x006a, 0x006b, 0x006c,
    0x006d, 0x006e, 0x006f, 0x0070, 0x0071, 0x0072, 0x0073, 0x0074,
    0x0075, 0x0076, 0x0077, 0x0078, 0x0079, 0x007a, 0x007b, 0x007c,
    0x007d, 0x007e, 0x007f, 0x0080, 0x0081, 0x0082, 0x0083, 0x0084,
    0x0085, 0x0086, 0x0087, 0x0088, 0x0089, 0x008a, 0x008b, 0x008c,
    0x008d, 0x008e, 0x008f, 0x0090, 0x0091, 0x0092, 0x0093, 0x0094,
    0x0095, 0x0096, 0x0097, 0x0098, 0x0099, 0x009a, 0x009b, 0x009c,
    0x009d, 0x009e, 0x009f, 0x00fa,
    /* nb */
    0x0000,
    /* ne */
    0x0009,
    /* nhn */
    0x0000, 0x0001,
    /* niu */
    0x0000, 0x0001,
    /* nl */
    0x0000,
    /* nn */
    0x0000,
    /* nqo */
    0x0007,
    /* nso */
    0x0000, 0x0001,
    /* nv */
    0x0000, 0x0001, 0x0002, 0x0003,
    /* ny */
    0x0000, 0x0001,
    /* oc */
    0x0000,
    /* or */
    0x000b,
    /* ota */
    0x0006,
    /* pa */
    0x000a,
    /* pap_an */
    0x0000,
    /* pap_aw */
    0x0000,
    /* pl */
    0x0000, 0x0001,
    /* ps_af */
    0x0006,
    /* ps_pk */
    0x0006,
    /* pt */
    0x0000,
    /* qu */
    0x0000, 0x0002,
    /* rif */
    0x0000, 0x0001, 0x0002, 0x001e,
    /* rm */
    0x0000,
    /* ro */
    0x0000, 0x0001, 0x0002,
    /* sah */
    0x0004,
    /* sat */
    0x0009,
    /* sc */
    0x0000,
    /* sco */
    0x0000, 0x0001, 0x0002,
    /* sd */
    0x0006,
    /* se */
    0x0000, 0x0001,
    /* sg */
    0x0000,
    /* sgs */
    0x0000, 0x0001, 0x0003,
    /* sh */
    0x0000, 0x0001, 0x0004,
    /* shs */
    0x0000, 0x0003,
    /* si */
    0x000d,
    /* sid */
    0x0012, 0x0013,
    /* sk */
    0x0000, 0x0001,
    /* sm */
    0x0000, 0x0002,
    /* sma */
    0x0000,
    /* smj */
    0x0000,
    /* smn */
    0x0000, 0x0001,
    /* sms */
    0x0000, 0x0001, 0x0002,
    /* sq */
    0x0000,
    /* sr */
    0x0004,
    /* sv */
    0x0000,
    /* syr */
    0x0007,
    /* szl */
    0x0000, 0x0001,
    /* ta */
    0x000b,
    /* te */
    0x000c,
    /* tg */
    0x0004,
    /* th */
    0x000e,
    /* tig */
    0x0012, 0x0013,
    /* tk */
    0x0000, 0x0001,
    /* tr */
    0x0000, 0x0001,
    /* tt */
    0x0004,
    /* ty */
    0x0000, 0x0001, 0x0002,
    /* ug */
    0x0006,
    /* uk */
    0x0004,
    /* und_zmth */
    0x0000, 0x0001, 0x0003, 0x0020, 0x0021, 0x0022, 0x0023, 0x0025,
    0x0027, 0x01d4, 0x01d5, 0x01d6,
    /* und_zsye */
    0x0023, 0x0025, 0x0026, 0x0027, 0x002b, 0x01f0, 0x01f1, 0x01f2,
    0x01f3, 0x01f4, 0x01f5, 0x01f6,
    /* ve */
    0x0000, 0x001e,
    /* vi */
    0x0000, 0x0001, 0x0003, 0x001e,
    /* vo */
    0x0000,
    /* vot */
    0x0000, 0x0001,
    /* wa */
    0x0000,
    /* wen */
    0x0000, 0x0001,
    /* wo */
    0x0000, 0x0001,
    /* yap */
    0x0000,
    /* yo */
    0x0000, 0x0001, 0x0003, 0x001e,
    /* yue */
    0x0030, 0x0034, 0x0035, 0x0036, 0x0037, 0x0038, 0x0039, 0x003a,
    0x003b, 0x003c, 0x003d, 0x003e, 0x003f, 0x0040, 0x0041, 0x0042,
    0x0043, 0x0044, 0x0045, 0x0046, 0x0047, 0x0048, 0x0049, 0x004a,
    0x004b, 0x004c, 0x004d, 0x004e, 0x004f, 0x0050, 0x0051, 0x0052,
    0x0053, 0x0054, 0x0055, 0x0056, 0x0057, 0x0058, 0x0059, 0x005a,
    0x005b, 0x005c, 0x005d, 0x005e, 0x005f, 0x0060, 0x0061, 0x0062,
    0x0063, 0x0064, 0x0065, 0x0066, 0x0067, 0x0068, 0x0069, 0x006a,
    0x006b, 0x006c, 0x006d, 0x006e, 0x006f, 0x0070, 0x0071, 0x0072,
    0x0073, 0x0074, 0x0075, 0x0076, 0x0077, 0x0078, 0x0079, 0x007a,
    0x007b, 0x007c, 0x007d, 0x007e, 0x007f, 0x0080, 0x0081, 0x0082,
    0x0083, 0x0084, 0x0085, 0x0086, 0x0087, 0x0088, 0x0089, 0x008a,
    0x008b, 0x008c, 0x008d, 0x008e, 0x008f, 0x0090, 0x0091, 0x0092,
    0x0093, 0x0094, 0x0095, 0x0096, 0x0097, 0x0098, 0x0099, 0x009a,
    0x009b, 0x009c, 0x009d, 0x009e, 0x009f, 0x0200, 0x0201, 0x0203,
    0x0207, 0x020c, 0x020d, 0x020e, 0x020f, 0x0210, 0x0211, 0x0219,
    0x021a, 0x021c, 0x021d, 0x0220, 0x0221, 0x022a, 0x022b, 0x022c,
    0x022d, 0x022f, 0x0232, 0x0235, 0x0236, 0x023c, 0x023e, 0x023f,
    0x0244, 0x024d, 0x024e, 0x0251, 0x0255, 0x025e, 0x0262, 0x0266,
    0x0267, 0x0268, 0x0269, 0x0272, 0x0275, 0x0276, 0x0277, 0x0278,
    0x0279, 0x027a, 0x027d, 0x0280, 0x0281, 0x0282, 0x0283, 0x0289,
    0x028a, 0x028b, 0x028c, 0x028d, 0x028e, 0x0294, 0x0297, 0x0298,
    0x029a, 0x029d, 0x02a6,
    /* zh_cn */
    0x0002, 0x004e, 0x004f, 0x0050, 0x0051, 0x0052, 0x0053, 0x0054,
    0x0055, 0x0056, 0x0057, 0x0058, 0x0059, 0x005a, 0x005b, 0x005c,
    0x005d, 0x005e, 0x005f, 0x0060, 0x0061, 0x0062, 0x0063, 0x0064,
    0x0065, 0x0066, 0x0067, 0x0068, 0x0069, 0x006a, 0x006b, 0x006c,
    0x006d, 0x006e, 0x006f, 0x0070, 0x0071, 0x0072, 0x0073, 0x0074,
    0x0075, 0x0076, 0x0077, 0x0078, 0x0079, 0x007a, 0x007b, 0x007c,
    0x007d, 0x007e, 0x007f, 0x0080, 0x0081, 0x0082, 0x0083, 0x0084,
    0x0085, 0x0086, 0x0087, 0x0088, 0x0089, 0x008a, 0x008b, 0x008c,
    0x008d, 0x008e, 0x008f, 0x0090, 0x0091, 0x0092, 0x0093, 0x0094,
    0x0095, 0x0096, 0x0097, 0x0098, 0x0099, 0x009a, 0x009b, 0x009c,
    0x009e, 0x009f,
},
{
    0, /* aa */
    1, /* ab */
    2, /* af */
    252, /* agr */
    190, /* ak */
    3, /* am */
    191, /* an */
    246, /* anp */
    4, /* ar */
    5, /* as */
    6, /* ast */
    7, /* av */
    8, /* ay */
    253, /* ayc */
    9, /* az_az */
    10, /* az_ir */
    11, /* ba */
    13, /* be */
    254, /* bem */
    192, /* ber_dz */
    193, /* ber_ma */
    14, /* bg */
    15, /* bh */
    247, /* bhb */
    16, /* bho */
    17, /* bi */
    18, /* bin */
    12, /* bm */
    19, /* bn */
    20, /* bo */
    21, /* br */
    240, /* brx */
    22, /* bs */
    23, /* bua */
    194, /* byn */
    24, /* ca */
    25, /* ce */
    26, /* ch */
    27, /* chm */
    28, /* chr */
    255, /* ckb */
    256, /* cmn */
    29, /* co */
    195, /* crh */
    30, /* cs */
    196, /* csb */
    31, /* cu */
    32, /* cv */
    33, /* cy */
    34, /* da */
    35, /* de */
    242, /* doi */
    257, /* dsb */
    197, /* dv */
    36, /* dz */
    198, /* ee */
    37, /* el */
    38, /* en */
    39, /* eo */
    40, /* es */
    41, /* et */
    42, /* eu */
    43, /* fa */
    199, /* fat */
    48, /* ff */
    44, /* fi */
    200, /* fil */
    45, /* fj */
    46, /* fo */
    47, /* fr */
    49, /* fur */
    50, /* fy */
    51, /* ga */
    52, /* gd */
    53, /* gez */
    54, /* gl */
    55, /* gn */
    56, /* gu */
    57, /* gv */
    58, /* ha */
    258, /* hak */
    59, /* haw */
    60, /* he */
    61, /* hi */
    248, /* hif */
    201, /* hne */
    62, /* ho */
    63, /* hr */
    202, /* hsb */
    203, /* ht */
    64, /* hu */
    65, /* hy */
    204, /* hz */
    66, /* ia */
    68, /* id */
    69, /* ie */
    67, /* ig */
    205, /* ii */
    70, /* ik */
    71, /* io */
    72, /* is */
    73, /* it */
    74, /* iu */
    75, /* ja */
    206, /* jv */
    76, /* ka */
    77, /* kaa */
    207, /* kab */
    78, /* ki */
    208, /* kj */
    79, /* kk */
    80, /* kl */
    81, /* km */
    82, /* kn */
    83, /* ko */
    84, /* kok */
    209, /* kr */
    85, /* ks */
    86, /* ku_am */
    210, /* ku_iq */
    87, /* ku_ir */
    211, /* ku_tr */
    88, /* kum */
    89, /* kv */
    90, /* kw */
    212, /* kwm */
    91, /* ky */
    92, /* la */
    238, /* lah */
    93, /* lb */
    94, /* lez */
    213, /* lg */
    214, /* li */
    259, /* lij */
    95, /* ln */
    96, /* lo */
    97, /* lt */
    98, /* lv */
    260, /* lzh */
    249, /* mag */
    215, /* mai */
    261, /* mfe */
    99, /* mg */
    100, /* mh */
    262, /* mhr */
    101, /* mi */
    263, /* miq */
    264, /* mjw */
    102, /* mk */
    103, /* ml */
    104, /* mn_cn */
    216, /* mn_mn */
    243, /* mni */
    265, /* mnw */
    105, /* mo */
    106, /* mr */
    217, /* ms */
    107, /* mt */
    108, /* my */
    218, /* na */
    266, /* nan */
    109, /* nb */
    110, /* nds */
    111, /* ne */
    219, /* ng */
    267, /* nhn */
    268, /* niu */
    112, /* nl */
    113, /* nn */
    114, /* no */
    239, /* nqo */
    115, /* nr */
    116, /* nso */
    220, /* nv */
    117, /* ny */
    118, /* oc */
    119, /* om */
    120, /* or */
    121, /* os */
    221, /* ota */
    122, /* pa */
    222, /* pa_pk */
    223, /* pap_an */
    224, /* pap_aw */
    123, /* pl */
    124, /* ps_af */
    125, /* ps_pk */
    126, /* pt */
    225, /* qu */
    226, /* quz */
    250, /* raj */
    269, /* rif */
    127, /* rm */
    227, /* rn */
    128, /* ro */
    129, /* ru */
    228, /* rw */
    130, /* sa */
    131, /* sah */
    241, /* sat */
    229, /* sc */
    132, /* sco */
    230, /* sd */
    133, /* se */
    134, /* sel */
    231, /* sg */
    270, /* sgs */
    135, /* sh */
    271, /* shn */
    136, /* shs */
    137, /* si */
    232, /* sid */
    138, /* sk */
    139, /* sl */
    140, /* sm */
    141, /* sma */
    142, /* smj */
    143, /* smn */
    144, /* sms */
    233, /* sn */
    145, /* so */
    146, /* sq */
    147, /* sr */
    148, /* ss */
    149, /* st */
    234, /* su */
    150, /* sv */
    151, /* sw */
    152, /* syr */
    272, /* szl */
    153, /* ta */
    273, /* tcy */
    154, /* te */
    155, /* tg */
    156, /* th */
    251, /* the */
    157, /* ti_er */
    158, /* ti_et */
    159, /* tig */
    160, /* tk */
    161, /* tl */
    162, /* tn */
    163, /* to */
    274, /* tpi */
    164, /* tr */
    165, /* ts */
    166, /* tt */
    167, /* tw */
    235, /* ty */
    168, /* tyv */
    169, /* ug */
    170, /* uk */
    245, /* und_zmth */
    244, /* und_zsye */
    275, /* unm */
    171, /* ur */
    172, /* uz */
    173, /* ve */
    174, /* vi */
    175, /* vo */
    176, /* vot */
    177, /* wa */
    276, /* wae */
    236, /* wal */
    178, /* wen */
    179, /* wo */
    180, /* xh */
    181, /* yap */
    182, /* yi */
    183, /* yo */
    277, /* yue */
    278, /* yuw */
    237, /* za */
    184, /* zh_cn */
    185, /* zh_hk */
    186, /* zh_mo */
    187, /* zh_sg */
    188, /* zh_tw */
    189, /* zu */
},
{
    0, /* aa */
    1, /* ab */
    2, /* af */
    5, /* am */
    8, /* ar */
    9, /* as */
    10, /* ast */
    11, /* av */
    12, /* ay */
    14, /* az_az */
    15, /* az_ir */
    16, /* ba */
    27, /* bm */
    17, /* be */
    21, /* bg */
    22, /* bh */
    24, /* bho */
    25, /* bi */
    26, /* bin */
    28, /* bn */
    29, /* bo */
    30, /* br */
    32, /* bs */
    33, /* bua */
    35, /* ca */
    36, /* ce */
    37, /* ch */
    38, /* chm */
    39, /* chr */
    42, /* co */
    44, /* cs */
    46, /* cu */
    47, /* cv */
    48, /* cy */
    49, /* da */
    50, /* de */
    54, /* dz */
    56, /* el */
    57, /* en */
    58, /* eo */
    59, /* es */
    60, /* et */
    61, /* eu */
    62, /* fa */
    65, /* fi */
    67, /* fj */
    68, /* fo */
    69, /* fr */
    64, /* ff */
    70, /* fur */
    71, /* fy */
    72, /* ga */
    73, /* gd */
    74, /* gez */
    75, /* gl */
    76, /* gn */
    77, /* gu */
    78, /* gv */
    79, /* ha */
    81, /* haw */
    82, /* he */
    83, /* hi */
    86, /* ho */
    87, /* hr */
    90, /* hu */
    91, /* hy */
    93, /* ia */
    96, /* ig */
    94, /* id */
    95, /* ie */
    98, /* ik */
    99, /* io */
    100, /* is */
    101, /* it */
    102, /* iu */
    103, /* ja */
    105, /* ka */
    106, /* kaa */
    108, /* ki */
    110, /* kk */
    111, /* kl */
    112, /* km */
    113, /* kn */
    114, /* ko */
    115, /* kok */
    117, /* ks */
    118, /* ku_am */
    120, /* ku_ir */
    122, /* kum */
    123, /* kv */
    124, /* kw */
    126, /* ky */
    127, /* la */
    129, /* lb */
    130, /* lez */
    134, /* ln */
    135, /* lo */
    136, /* lt */
    137, /* lv */
    142, /* mg */
    143, /* mh */
    145, /* mi */
    148, /* mk */
    149, /* ml */
    150, /* mn_cn */
    154, /* mo */
    155, /* mr */
    157, /* mt */
    158, /* my */
    161, /* nb */
    162, /* nds */
    163, /* ne */
    167, /* nl */
    168, /* nn */
    169, /* no */
    171, /* nr */
    172, /* nso */
    174, /* ny */
    175, /* oc */
    176, /* om */
    177, /* or */
    178, /* os */
    180, /* pa */
    184, /* pl */
    185, /* ps_af */
    186, /* ps_pk */
    187, /* pt */
    192, /* rm */
    194, /* ro */
    195, /* ru */
    197, /* sa */
    198, /* sah */
    201, /* sco */
    203, /* se */
    204, /* sel */
    207, /* sh */
    209, /* shs */
    210, /* si */
    212, /* sk */
    213, /* sl */
    214, /* sm */
    215, /* sma */
    216, /* smj */
    217, /* smn */
    218, /* sms */
    220, /* so */
    221, /* sq */
    222, /* sr */
    223, /* ss */
    224, /* st */
    226, /* sv */
    227, /* sw */
    228, /* syr */
    230, /* ta */
    232, /* te */
    233, /* tg */
    234, /* th */
    236, /* ti_er */
    237, /* ti_et */
    238, /* tig */
    239, /* tk */
    240, /* tl */
    241, /* tn */
    242, /* to */
    244, /* tr */
    245, /* ts */
    246, /* tt */
    247, /* tw */
    249, /* tyv */
    250, /* ug */
    251, /* uk */
    255, /* ur */
    256, /* uz */
    257, /* ve */
    258, /* vi */
    259, /* vo */
    260, /* vot */
    261, /* wa */
    264, /* wen */
    265, /* wo */
    266, /* xh */
    267, /* yap */
    268, /* yi */
    269, /* yo */
    273, /* zh_cn */
    274, /* zh_hk */
    275, /* zh_mo */
    276, /* zh_sg */
    277, /* zh_tw */
    278, /* zu */
    4, /* ak */
    6, /* an */
    19, /* ber_dz */
    20, /* ber_ma */
    34, /* byn */
    43, /* crh */
    45, /* csb */
    53, /* dv */
    55, /* ee */
    63, /* fat */
    66, /* fil */
    85, /* hne */
    88, /* hsb */
    89, /* ht */
    92, /* hz */
    97, /* ii */
    104, /* jv */
    107, /* kab */
    109, /* kj */
    116, /* kr */
    119, /* ku_iq */
    121, /* ku_tr */
    125, /* kwm */
    131, /* lg */
    132, /* li */
    140, /* mai */
    151, /* mn_mn */
    156, /* ms */
    159, /* na */
    164, /* ng */
    173, /* nv */
    179, /* ota */
    181, /* pa_pk */
    182, /* pap_an */
    183, /* pap_aw */
    188, /* qu */
    189, /* quz */
    193, /* rn */
    196, /* rw */
    200, /* sc */
    202, /* sd */
    205, /* sg */
    211, /* sid */
    219, /* sn */
    225, /* su */
    248, /* ty */
    263, /* wal */
    272, /* za */
    128, /* lah */
    170, /* nqo */
    31, /* brx */
    199, /* sat */
    51, /* doi */
    152, /* mni */
    253, /* und_zsye */
    252, /* und_zmth */
    7, /* anp */
    23, /* bhb */
    84, /* hif */
    139, /* mag */
    190, /* raj */
    235, /* the */
    3, /* agr */
    13, /* ayc */
    18, /* bem */
    40, /* ckb */
    41, /* cmn */
    52, /* dsb */
    80, /* hak */
    133, /* lij */
    138, /* lzh */
    141, /* mfe */
    144, /* mhr */
    146, /* miq */
    147, /* mjw */
    153, /* mnw */
    160, /* nan */
    165, /* nhn */
    166, /* niu */
    191, /* rif */
    206, /* sgs */
    208, /* shn */
    229, /* szl */
    231, /* tcy */
    243, /* tpi */
    254, /* unm */
    262, /* wae */
    270, /* yue */
    271, /* yuw */
}
};

#define NUM_LANG_CHAR_SET	279
#define NUM_LANG_SET_MAP	9

static const FcChar32 fcLangCountrySets[][NUM_LANG_SET_MAP] = {
    { 0x00000600, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, }, /* az */
    { 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000003, 0x00000000, 0x00000000, }, /* ber */
    { 0x00000000, 0x00000000, 0x00c00000, 0x00000000, 0x00000000, 0x00000000, 0x000c0000, 0x00000000, 0x00000000, }, /* ku */
    { 0x00000000, 0x00000000, 0x00000000, 0x00000100, 0x00000000, 0x00000000, 0x01000000, 0x00000000, 0x00000000, }, /* mn */
    { 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x40000000, 0x00000000, 0x00000000, }, /* pa */
    { 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x80000000, 0x00000001, 0x00000000, }, /* pap */
    { 0x00000000, 0x00000000, 0x00000000, 0x30000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, }, /* ps */
    { 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x60000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, }, /* ti */
    { 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00300000, 0x00000000, }, /* und */
    { 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x1f000000, 0x00000000, 0x00000000, 0x00000000, }, /* zh */
};

#define NUM_COUNTRY_SET 10

static const FcLangCharSetRange  fcLangCharSetRanges[] = {

    { 0, 15 }, /* a */
    { 16, 34 }, /* b */
    { 35, 48 }, /* c */
    { 49, 54 }, /* d */
    { 55, 61 }, /* e */
    { 62, 71 }, /* f */
    { 72, 78 }, /* g */
    { 79, 92 }, /* h */
    { 93, 102 }, /* i */
    { 103, 104 }, /* j */
    { 105, 126 }, /* k */
    { 127, 138 }, /* l */
    { 139, 158 }, /* m */
    { 159, 174 }, /* n */
    { 175, 179 }, /* o */
    { 180, 187 }, /* p */
    { 188, 189 }, /* q */
    { 190, 196 }, /* r */
    { 197, 229 }, /* s */
    { 230, 249 }, /* t */
    { 250, 256 }, /* u */
    { 257, 260 }, /* v */
    { 261, 265 }, /* w */
    { 266, 266 }, /* x */
    { 267, 271 }, /* y */
    { 272, 278 }, /* z */
};

