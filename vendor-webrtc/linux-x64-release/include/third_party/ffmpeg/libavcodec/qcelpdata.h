/*
 * QCELP decoder
 * Copyright (c) 2007 Reynaldo H. Verdejo Pinochet
 *
 * This file is part of FFmpeg.
 *
 * FFmpeg is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 2.1 of the License, or (at your option) any later version.
 *
 * FFmpeg is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with FFmpeg; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA
 */

#ifndef AVCODEC_QCELPDATA_H
#define AVCODEC_QCELPDATA_H

/**
 * @file
 * Data tables for the QCELP decoder
 * @author Reynaldo H. Verdejo Pinochet
 * @remark FFmpeg merging spearheaded by Kenan Gillet
 * @remark Development mentored by Benjamin Larson
 */

#include <stddef.h>
#include <stdint.h>
#include "libavutil/common.h"

/**
 * QCELP unpacked data frame
 */
typedef struct QCELPFrame {
/// @name QCELP excitation codebook parameters
/// @{
    uint8_t cbsign[16]; ///< sign of the codebook gain for each codebook subframe
    uint8_t cbgain[16]; ///< unsigned codebook gain for each codebook subframe
    uint8_t cindex[16]; ///< codebook index for each codebook subframe
/// @}

/// @name QCELP pitch prediction parameters
/// @{
    uint8_t plag[4];    ///< pitch lag for each pitch subframe
    uint8_t pfrac[4];   ///< fractional pitch lag for each pitch subframe
    uint8_t pgain[4];   ///< pitch gain for each pitch subframe
/// @}

    /**
     * line spectral pair frequencies (LSP) for RATE_OCTAVE,
     * line spectral pair frequencies grouped into five vectors
     * of dimension two (LSPV) for other rates
     */
    uint8_t lspv[10];

    /**
     * reserved bits only present in bitrate 1, 1/4 and 1/8 packets
     */
    uint8_t reserved;
} QCELPFrame;

/**
 * Pre-calculated table for hammsinc function.
 * Only half of the table is needed because of symmetry.
 *
 * TIA/EIA/IS-733 2.4.5.2-2/3
 */
static const float qcelp_hammsinc_table[4] = { -0.006822,  0.041249, -0.143459,  0.588863};

typedef struct QCELPBitmap {
    uint8_t index;  /**< index into the QCELPContext structure */
    uint8_t bitpos; /**< position of the lowest bit in the value's byte */
    uint8_t bitlen; /**< number of bits to read */
} QCELPBitmap;

#define QCELP_OF(variable, bit, len) {offsetof(QCELPFrame, variable), bit, len}

/**
 * Bitmap unpacking tables for RATE_FULL
 *
 * TIA/EIA/IS-733 Table 2.4.7.1-1
 */
static const QCELPBitmap qcelp_rate_full_bitmap[] = {
                                // start on bit
    QCELP_OF(lspv  [ 2], 0, 3), // 265
    QCELP_OF(lspv  [ 1], 0, 7), // 262
    QCELP_OF(lspv  [ 0], 0, 6), // 255
    QCELP_OF(lspv  [ 4], 0, 6), // 249
    QCELP_OF(lspv  [ 3], 0, 6), // 243
    QCELP_OF(lspv  [ 2], 3, 4), // 237
    QCELP_OF(cbsign[ 0], 0, 1), // 233
    QCELP_OF(cbgain[ 0], 0, 4), // 232
    QCELP_OF(pfrac [ 0], 0, 1), // 228
    QCELP_OF(plag  [ 0], 0, 7), // 227
    QCELP_OF(pgain [ 0], 0, 3), // 220
    QCELP_OF(cindex[ 1], 0, 4), // 217
    QCELP_OF(cbsign[ 1], 0, 1), // 213
    QCELP_OF(cbgain[ 1], 0, 4), // 212
    QCELP_OF(cindex[ 0], 0, 7), // 208
    QCELP_OF(cbgain[ 3], 0, 1), // 201
    QCELP_OF(cindex[ 2], 0, 7), // 200
    QCELP_OF(cbsign[ 2], 0, 1), // 193
    QCELP_OF(cbgain[ 2], 0, 4), // 192
    QCELP_OF(cindex[ 1], 4, 3), // 188
    QCELP_OF(plag  [ 1], 0, 3), // 185
    QCELP_OF(pgain [ 1], 0, 3), // 182
    QCELP_OF(cindex[ 3], 0, 7), // 179
    QCELP_OF(cbsign[ 3], 0, 1), // 172
    QCELP_OF(cbgain[ 3], 1, 2), // 171
    QCELP_OF(cindex[ 4], 0, 6), // 169
    QCELP_OF(cbsign[ 4], 0, 1), // 163
    QCELP_OF(cbgain[ 4], 0, 4), // 162
    QCELP_OF(pfrac [ 1], 0, 1), // 158
    QCELP_OF(plag  [ 1], 3, 4), // 157
    QCELP_OF(cbgain[ 6], 0, 3), // 153
    QCELP_OF(cindex[ 5], 0, 7), // 150
    QCELP_OF(cbsign[ 5], 0, 1), // 143
    QCELP_OF(cbgain[ 5], 0, 4), // 142
    QCELP_OF(cindex[ 4], 6, 1), // 138
    QCELP_OF(cindex[ 7], 0, 3), // 137
    QCELP_OF(cbsign[ 7], 0, 1), // 134
    QCELP_OF(cbgain[ 7], 0, 3), // 133
    QCELP_OF(cindex[ 6], 0, 7), // 130
    QCELP_OF(cbsign[ 6], 0, 1), // 123
    QCELP_OF(cbgain[ 6], 3, 1), // 122
    QCELP_OF(cbgain[ 8], 0, 1), // 121
    QCELP_OF(pfrac [ 2], 0, 1), // 120
    QCELP_OF(plag  [ 2], 0, 7), // 119
    QCELP_OF(pgain [ 2], 0, 3), // 112
    QCELP_OF(cindex[ 7], 3, 4), // 109
    QCELP_OF(cbsign[ 9], 0, 1), // 105
    QCELP_OF(cbgain[ 9], 0, 4), // 104
    QCELP_OF(cindex[ 8], 0, 7), // 100
    QCELP_OF(cbsign[ 8], 0, 1), //  93
    QCELP_OF(cbgain[ 8], 1, 3), //  92
    QCELP_OF(cindex[10], 0, 4), //  89
    QCELP_OF(cbsign[10], 0, 1), //  85
    QCELP_OF(cbgain[10], 0, 4), //  84
    QCELP_OF(cindex[ 9], 0, 7), //  80
    QCELP_OF(pgain [ 3], 0, 2), //  73
    QCELP_OF(cindex[11], 0, 7), //  71
    QCELP_OF(cbsign[11], 0, 1), //  64
    QCELP_OF(cbgain[11], 0, 3), //  63
    QCELP_OF(cindex[10], 4, 3), //  60
    QCELP_OF(cindex[12], 0, 2), //  57
    QCELP_OF(cbsign[12], 0, 1), //  55
    QCELP_OF(cbgain[12], 0, 4), //  54
    QCELP_OF(pfrac [ 3], 0, 1), //  50
    QCELP_OF(plag  [ 3], 0, 7), //  49
    QCELP_OF(pgain [ 3], 2, 1), //  42
    QCELP_OF(cindex[13], 0, 6), //  41
    QCELP_OF(cbsign[13], 0, 1), //  35
    QCELP_OF(cbgain[13], 0, 4), //  34
    QCELP_OF(cindex[12], 2, 5), //  30
    QCELP_OF(cbgain[15], 0, 3), //  25
    QCELP_OF(cindex[14], 0, 7), //  22
    QCELP_OF(cbsign[14], 0, 1), //  15
    QCELP_OF(cbgain[14], 0, 4), //  14
    QCELP_OF(cindex[13], 6, 1), //  10
    QCELP_OF(reserved,   0, 2), //   9
    QCELP_OF(cindex[15], 0, 7), //   7
    QCELP_OF(cbsign[15], 0, 1)  //   0
};

/**
 * Bitmap unpacking tables for RATE_HALF
 *
 * TIA/EIA/IS-733 Table 2.4.7.2-1
 */
static const QCELPBitmap qcelp_rate_half_bitmap[] = {
                               // start on bit
    QCELP_OF(lspv  [2], 0, 3), // 123
    QCELP_OF(lspv  [1], 0, 7), // 120
    QCELP_OF(lspv  [0], 0, 6), // 113
    QCELP_OF(lspv  [4], 0, 6), // 107
    QCELP_OF(lspv  [3], 0, 6), // 101
    QCELP_OF(lspv  [2], 3, 4), //  95
    QCELP_OF(cbsign[0], 0, 1), //  91
    QCELP_OF(cbgain[0], 0, 4), //  90
    QCELP_OF(pfrac [0], 0, 1), //  86
    QCELP_OF(plag  [0], 0, 7), //  85
    QCELP_OF(pgain [0], 0, 3), //  78
    QCELP_OF(plag  [1], 0, 6), //  75
    QCELP_OF(pgain [1], 0, 3), //  69
    QCELP_OF(cindex[0], 0, 7), //  66
    QCELP_OF(pgain [2], 0, 2), //  59
    QCELP_OF(cindex[1], 0, 7), //  57
    QCELP_OF(cbsign[1], 0, 1), //  50
    QCELP_OF(cbgain[1], 0, 4), //  49
    QCELP_OF(pfrac [1], 0, 1), //  45
    QCELP_OF(plag  [1], 6, 1), //  44
    QCELP_OF(cindex[2], 0, 2), //  43
    QCELP_OF(cbsign[2], 0, 1), //  41
    QCELP_OF(cbgain[2], 0, 4), //  40
    QCELP_OF(pfrac [2], 0, 1), //  36
    QCELP_OF(plag  [2], 0, 7), //  35
    QCELP_OF(pgain [2], 2, 1), //  28
    QCELP_OF(pfrac [3], 0, 1), //  27
    QCELP_OF(plag  [3], 0, 7), //  26
    QCELP_OF(pgain [3], 0, 3), //  19
    QCELP_OF(cindex[2], 2, 5), //  16
    QCELP_OF(cindex[3], 0, 7), //  11
    QCELP_OF(cbsign[3], 0, 1), //   4
    QCELP_OF(cbgain[3], 0, 4)  //   3
};

/**
 * Bitmap unpacking tables for RATE_QUARTER
 *
 * TIA/EIA/IS-733 Table 2.4.7.3-1
 */
static const QCELPBitmap qcelp_rate_quarter_bitmap[] = {
                               // start on bit
    QCELP_OF(lspv  [2], 0, 3), // 53
    QCELP_OF(lspv  [1], 0, 7), // 50
    QCELP_OF(lspv  [0], 0, 6), // 43
    QCELP_OF(lspv  [4], 0, 6), // 37
    QCELP_OF(lspv  [3], 0, 6), // 31
    QCELP_OF(lspv  [2], 3, 4), // 25
    QCELP_OF(cbgain[3], 0, 4), // 21
    QCELP_OF(cbgain[2], 0, 4), // 17
    QCELP_OF(cbgain[1], 0, 4), // 13
    QCELP_OF(cbgain[0], 0, 4), //  9
    QCELP_OF(reserved,  0, 2), //  5
    QCELP_OF(cbgain[4], 0, 4)  //  3
};

/**
 * Bitmap unpacking tables for RATE_OCTAVE
 *
 * trick: CBSEED is written into QCELPContext.cbsign[15],
 * which is not used for RATE_OCTAVE.
 * CBSEED is only used to ensure the occurrence of random bit
 * patterns in the 16 first bits that are used as the seed.
 *
 * TIA/EIA/IS-733 Table 2.4.7.4-1
 */
static const QCELPBitmap qcelp_rate_octave_bitmap[] = {
                                // start on bit
    QCELP_OF(cbsign[15], 3, 1), // 19
    QCELP_OF(lspv   [0], 0, 1), // 18
    QCELP_OF(lspv   [1], 0, 1), // 17
    QCELP_OF(lspv   [2], 0, 1), // 16
    QCELP_OF(cbsign[15], 2, 1), // 15
    QCELP_OF(lspv   [3], 0, 1), // 14
    QCELP_OF(lspv   [4], 0, 1), // 13
    QCELP_OF(lspv   [5], 0, 1), // 12
    QCELP_OF(cbsign[15], 1, 1), // 11
    QCELP_OF(lspv   [6], 0, 1), // 10
    QCELP_OF(lspv   [7], 0, 1), //  9
    QCELP_OF(lspv   [8], 0, 1), //  8
    QCELP_OF(cbsign[15], 0, 1), //  7
    QCELP_OF(lspv   [9], 0, 1), //  6
    QCELP_OF(cbgain [0], 0, 2), //  5
    QCELP_OF(reserved,   0, 4)  //  3
};

/**
 * Bitmapping data position for each packet type in
 * the QCELPContext
 */
static const QCELPBitmap * const qcelp_unpacking_bitmaps_per_rate[5] = {
    NULL,                     ///< for SILENCE rate
    qcelp_rate_octave_bitmap,
    qcelp_rate_quarter_bitmap,
    qcelp_rate_half_bitmap,
    qcelp_rate_full_bitmap,
};

static const uint16_t qcelp_unpacking_bitmaps_lengths[5] = {
    0, ///< for SILENCE rate
    FF_ARRAY_ELEMS(qcelp_rate_octave_bitmap),
    FF_ARRAY_ELEMS(qcelp_rate_quarter_bitmap),
    FF_ARRAY_ELEMS(qcelp_rate_half_bitmap),
    FF_ARRAY_ELEMS(qcelp_rate_full_bitmap),
};

typedef uint16_t qcelp_vector[2];

/**
 * LSP vector quantization tables in x*10000 form
 *
 * TIA/EIA/IS-733 tables 2.4.3.2.6.3-1 through 2.4.3.2.6.3-5
 */

static const qcelp_vector qcelp_lspvq1[64]= {
{ 327, 118},{ 919, 111},{ 427, 440},{1327, 185},
{ 469,  50},{1272,  91},{ 892,  59},{1771, 193},
{ 222, 158},{1100, 127},{ 827,  55},{ 978, 791},
{ 665,  47},{ 700,1401},{ 670, 859},{1913,1048},
{ 471, 215},{1046, 125},{ 645, 298},{1599, 160},
{ 593,  39},{1187, 462},{ 749, 341},{1520, 511},
{ 290, 792},{ 909, 362},{ 753,  81},{1111,1058},
{ 519, 253},{ 828, 839},{ 685, 541},{1421,1258},
{ 386, 130},{ 962, 119},{ 542, 387},{1431, 185},
{ 526,  51},{1175, 260},{ 831, 167},{1728, 510},
{ 273, 437},{1172, 113},{ 771, 144},{1122, 751},
{ 619, 119},{ 492,1276},{ 658, 695},{1882, 615},
{ 415, 200},{1018,  88},{ 681, 339},{1436, 325},
{ 555, 122},{1042, 485},{ 826, 345},{1374, 743},
{ 383,1018},{1005, 358},{ 704,  86},{1301, 586},
{ 597, 241},{ 832, 621},{ 555, 573},{1504, 839}};

static const qcelp_vector qcelp_lspvq2[128]= {
{ 255, 293},{ 904, 219},{ 151,1211},{1447, 498},
{ 470, 253},{1559, 177},{1547, 994},{2394, 242},
{  91, 813},{ 857, 590},{ 934,1326},{1889, 282},
{ 813, 472},{1057,1494},{ 450,3315},{2163,1895},
{ 538, 532},{1399, 218},{ 146,1552},{1755, 626},
{ 822, 202},{1299, 663},{ 706,1732},{2656, 401},
{ 418, 745},{ 762,1038},{ 583,1748},{1746,1285},
{ 527,1169},{1314, 830},{ 556,2116},{1073,2321},
{ 297, 570},{ 981, 403},{ 468,1103},{1740, 243},
{ 725, 179},{1255, 474},{1374,1362},{1922, 912},
{ 285, 947},{ 930, 700},{ 593,1372},{1909, 576},
{ 588, 916},{1110,1116},{ 224,2719},{1633,2220},
{ 402, 520},{1061, 448},{ 402,1352},{1499, 775},
{ 664, 589},{1081, 727},{ 801,2206},{2165,1157},
{ 566, 802},{ 911,1116},{ 306,1703},{1792, 836},
{ 655, 999},{1061,1038},{ 298,2089},{1110,1753},
{ 361, 311},{ 970, 239},{ 265,1231},{1495, 573},
{ 566, 262},{1569, 293},{1341,1144},{2271, 544},
{ 214, 877},{ 847, 719},{ 794,1384},{2067, 274},
{ 703, 688},{1099,1306},{ 391,2947},{2024,1670},
{ 471, 525},{1245, 290},{ 264,1557},{1568, 807},
{ 718, 399},{1193, 685},{ 883,1594},{2729, 764},
{ 500, 754},{ 809,1108},{ 541,1648},{1523,1385},
{ 614,1196},{1209, 847},{ 345,2242},{1442,1747},
{ 199, 560},{1092, 194},{ 349,1253},{1653, 507},
{ 625, 354},{1376, 431},{1187,1465},{2164, 872},
{ 360, 974},{1008, 698},{ 704,1346},{2114, 452},
{ 720, 816},{1240,1089},{ 439,2475},{1498,2040},
{ 336, 718},{1213, 187},{ 451,1450},{1368, 885},
{ 592, 578},{1131, 531},{ 861,1855},{1764,1500},
{ 444, 970},{ 935, 903},{ 424,1687},{1633,1102},
{ 793, 897},{1060, 897},{ 185,2011},{1205,1855}};

static const qcelp_vector qcelp_lspvq3[128]= {
{ 225, 283},{1296, 355},{ 543, 343},{2073, 274},
{ 204,1099},{1562, 523},{1388, 161},{2784, 274},
{ 112, 849},{1870, 175},{1189, 160},{1490,1088},
{ 969,1115},{ 659,3322},{1158,1073},{3183,1363},
{ 517, 223},{1740, 223},{ 704, 387},{2637, 234},
{ 692,1005},{1287,1610},{ 952, 532},{2393, 646},
{ 490, 552},{1619, 657},{ 845, 670},{1784,2280},
{ 191,1775},{ 272,2868},{ 942, 952},{2628,1479},
{ 278, 579},{1565, 218},{ 814, 180},{2379, 187},
{ 276,1444},{1199,1223},{1200, 349},{3009, 307},
{ 312, 844},{1898, 306},{ 863, 470},{1685,1241},
{ 513,1727},{ 711,2233},{1085, 864},{3398, 527},
{ 414, 440},{1356, 612},{ 964, 147},{2173, 738},
{ 465,1292},{ 877,1749},{1104, 689},{2105,1311},
{ 580, 864},{1895, 752},{ 652, 609},{1485,1699},
{ 514,1400},{ 386,2131},{ 933, 798},{2473, 986},
{ 334, 360},{1375, 398},{ 621, 276},{2183, 280},
{ 311,1114},{1382, 807},{1284, 175},{2605, 636},
{ 230, 816},{1739, 408},{1074, 176},{1619,1120},
{ 784,1371},{ 448,3050},{1189, 880},{3039,1165},
{ 424, 241},{1672, 186},{ 815, 333},{2432, 324},
{ 584,1029},{1137,1546},{1015, 585},{2198, 995},
{ 574, 581},{1746, 647},{ 733, 740},{1938,1737},
{ 347,1710},{ 373,2429},{ 787,1061},{2439,1438},
{ 185, 536},{1489, 178},{ 703, 216},{2178, 487},
{ 154,1421},{1414, 994},{1103, 352},{3072, 473},
{ 408, 819},{2055, 168},{ 998, 354},{1917,1140},
{ 665,1799},{ 993,2213},{1234, 631},{3003, 762},
{ 373, 620},{1518, 425},{ 913, 300},{1966, 836},
{ 402,1185},{ 948,1385},{1121, 555},{1802,1509},
{ 474, 886},{1888, 610},{ 739, 585},{1231,2379},
{ 661,1335},{ 205,2211},{ 823, 822},{2480,1179}};

static const qcelp_vector qcelp_lspvq4[64]= {
{ 348, 311},{ 812,1145},{ 552, 461},{1826, 263},
{ 601, 675},{1730, 172},{1523, 193},{2449, 277},
{ 334, 668},{ 805,1441},{1319, 207},{1684, 910},
{ 582,1318},{1403,1098},{ 979, 832},{2700,1359},
{ 624, 228},{1292, 979},{ 800, 195},{2226, 285},
{ 730, 862},{1537, 601},{1115, 509},{2720, 354},
{ 218,1167},{1212,1538},{1074, 247},{1674,1710},
{ 322,2142},{1263, 777},{ 981, 556},{2119,1710},
{ 193, 596},{1035, 957},{ 694, 397},{1997, 253},
{ 743, 603},{1584, 321},{1346, 346},{2221, 708},
{ 451, 732},{1040,1415},{1184, 230},{1853, 919},
{ 310,1661},{1625, 706},{ 856, 843},{2902, 702},
{ 467, 348},{1108,1048},{ 859, 306},{1964, 463},
{ 560,1013},{1425, 533},{1142, 634},{2391, 879},
{ 397,1084},{1345,1700},{ 976, 248},{1887,1189},
{ 644,2087},{1262, 603},{ 877, 550},{2203,1307}};

static const qcelp_vector qcelp_lspvq5[64]= {
{ 360, 222},{ 820,1097},{ 601, 319},{1656, 198},
{ 604, 513},{1552, 141},{1391, 155},{2474, 261},
{ 269, 785},{1463, 646},{1123, 191},{2015, 223},
{ 785, 844},{1202,1011},{ 980, 807},{3014, 793},
{ 570, 180},{1135,1382},{ 778, 256},{1901, 179},
{ 807, 622},{1461, 458},{1231, 178},{2028, 821},
{ 387, 927},{1496,1004},{ 888, 392},{2246, 341},
{ 295,1462},{1156, 694},{1022, 473},{2226,1364},
{ 210, 478},{1029,1020},{ 722, 181},{1730, 251},
{ 730, 488},{1465, 293},{1303, 326},{2595, 387},
{ 458, 584},{1569, 742},{1029, 173},{1910, 495},
{ 605,1159},{1268, 719},{ 973, 646},{2872, 428},
{ 443, 334},{ 835,1465},{ 912, 138},{1716, 442},
{ 620, 778},{1316, 450},{1186, 335},{1446,1665},
{ 486,1050},{1675,1019},{ 880, 278},{2214, 202},
{ 539,1564},{1142, 533},{ 984, 391},{2130,1089}};

static const qcelp_vector * const qcelp_lspvq[5] = {
    qcelp_lspvq1,
    qcelp_lspvq2,
    qcelp_lspvq3,
    qcelp_lspvq4,
    qcelp_lspvq5
};

/**
 * The final gain scalefactor before clipping into a usable output float
 */
#define QCELP_SCALE 8192.

/**
 * Table for computing Ga (decoded linear codebook gain magnitude)
 *
 * @note The table could fit in int16_t in x*8 form, but it seems
 *       to be slower on x86
 *
 * TIA/EIA/IS-733 2.4.6.2.1-3
 */

static const float qcelp_g12ga[61] = {
    1.000/QCELP_SCALE,   1.125/QCELP_SCALE,   1.250/QCELP_SCALE,   1.375/QCELP_SCALE,
    1.625/QCELP_SCALE,   1.750/QCELP_SCALE,   2.000/QCELP_SCALE,   2.250/QCELP_SCALE,
    2.500/QCELP_SCALE,   2.875/QCELP_SCALE,   3.125/QCELP_SCALE,   3.500/QCELP_SCALE,
    4.000/QCELP_SCALE,   4.500/QCELP_SCALE,   5.000/QCELP_SCALE,   5.625/QCELP_SCALE,
    6.250/QCELP_SCALE,   7.125/QCELP_SCALE,   8.000/QCELP_SCALE,   8.875/QCELP_SCALE,
   10.000/QCELP_SCALE,  11.250/QCELP_SCALE,  12.625/QCELP_SCALE,  14.125/QCELP_SCALE,
   15.875/QCELP_SCALE,  17.750/QCELP_SCALE,  20.000/QCELP_SCALE,  22.375/QCELP_SCALE,
   25.125/QCELP_SCALE,  28.125/QCELP_SCALE,  31.625/QCELP_SCALE,  35.500/QCELP_SCALE,
   39.750/QCELP_SCALE,  44.625/QCELP_SCALE,  50.125/QCELP_SCALE,  56.250/QCELP_SCALE,
   63.125/QCELP_SCALE,  70.750/QCELP_SCALE,  79.375/QCELP_SCALE,  89.125/QCELP_SCALE,
  100.000/QCELP_SCALE, 112.250/QCELP_SCALE, 125.875/QCELP_SCALE, 141.250/QCELP_SCALE,
  158.500/QCELP_SCALE, 177.875/QCELP_SCALE, 199.500/QCELP_SCALE, 223.875/QCELP_SCALE,
  251.250/QCELP_SCALE, 281.875/QCELP_SCALE, 316.250/QCELP_SCALE, 354.875/QCELP_SCALE,
  398.125/QCELP_SCALE, 446.625/QCELP_SCALE, 501.125/QCELP_SCALE, 562.375/QCELP_SCALE,
  631.000/QCELP_SCALE, 708.000/QCELP_SCALE, 794.375/QCELP_SCALE, 891.250/QCELP_SCALE,
 1000.000/QCELP_SCALE};

/**
 * Circular codebook for rate 1 frames in x*100 form
 *
 * TIA/EIA/IS-733 2.4.6.1-2
 */
static const int16_t qcelp_rate_full_codebook[128] = {
     10,  -65,  -59,   12,  110,   34, -134,  157,
    104,  -84,  -34, -115,   23, -101,    3,   45,
   -101,  -16,  -59,   28,  -45,  134,  -67,   22,
     61,  -29,  226,  -26,  -55, -179,  157,  -51,
   -220,  -93,  -37,   60,  118,   74,  -48,  -95,
   -181,  111,   36,  -52, -215,   78, -112,   39,
    -17,  -47, -223,   19,   12,  -98, -142,  130,
     54, -127,   21,  -12,   39,  -48,   12,  128,
      6, -167,   82, -102,  -79,   55,  -44,   48,
    -20,  -53,    8,  -61,   11,  -70, -157, -168,
     20,  -56,  -74,   78,   33,  -63, -173,   -2,
    -75,  -53, -146,   77,   66,  -29,    9,  -75,
     65,  119,  -43,   76,  233,   98,  125, -156,
    -27,   78,   -9,  170,  176,  143, -148,   -7,
     27, -136,    5,   27,   18,  139,  204,    7,
   -184, -197,   52,   -3,   78, -189,    8,  -65
};
#define QCELP_RATE_FULL_CODEBOOK_RATIO .01

/**
 * Circular codebook for rate 1/2 frames in x*2 form
 *
 * TIA/EIA/IS-733 2.4.6.1-1
 */
static const int8_t qcelp_rate_half_codebook[128] = {
     0, -4,  0, -3,  0,  0,  0,  0,
     0,  0,  0,  0,  0,  0,  0,  0,
     0, -3, -2,  0,  0,  0,  0,  0,
     0,  0,  0,  0,  0,  0,  0,  5,
     0,  0,  0,  0,  0,  0,  4,  0,
     0,  3,  2,  0,  3,  4,  0,  0,
     0,  0,  0,  0,  0,  0,  0,  0,
     0,  0,  0,  0,  0,  3,  0,  0,
    -3,  3,  0,  0, -2,  0,  3,  0,
     0,  0,  0,  0,  0,  0, -5,  0,
     0,  0,  0,  3,  0,  0,  0,  3,
     0,  0,  0,  0,  0,  0,  0,  4,
     0,  0,  0,  0,  0,  0,  0,  0,
     0,  3,  6, -3, -4,  0, -3, -3,
     3, -3,  0,  0,  0,  0,  0,  0,
     0,  0,  0,  0,  0,  0,  0,  0
};
#define QCELP_RATE_HALF_CODEBOOK_RATIO 0.5

/**
 * sqrt(1.887) is the maximum of the pseudorandom
 * white sequence used to generate the scaled codebook
 * vector for bitrate 1/4.
 *
 * TIA/EIA/IS-733 2.4.8.1.2
 */
#define QCELP_SQRT1887 1.373681186

/**
 * Table for impulse response of BPF used to filter
 * the white excitation for bitrate 1/4 synthesis
 *
 * Only half the tables are needed because of symmetry.
 *
 * TIA/EIA/IS-733 2.4.8.1.2-1.1
 */
static const double qcelp_rnd_fir_coefs[11] = {
  -1.344519e-1, 1.735384e-2, -6.905826e-2, 2.434368e-2,
  -8.210701e-2, 3.041388e-2, -9.251384e-2, 3.501983e-2,
  -9.918777e-2, 3.749518e-2,  8.985137e-1
};

/**
 * This spread factor is used, for bitrate 1/8 and I_F_Q,
 * to force LSP frequencies to be at least 80 Hz apart.
 *
 * TIA/EIA/IS-733 2.4.3.3.2
 */
#define QCELP_LSP_SPREAD_FACTOR 0.02

/**
 * Predictor coefficient for the conversion of LSP codes
 * to LSP frequencies for 1/8 and I_F_Q
 *
 * TIA/EIA/IS-733 2.4.3.2.7-2
 */
#define QCELP_LSP_OCTAVE_PREDICTOR 29.0/32

/**
 * Initial coefficient to perform bandwidth expansion on LPC
 *
 * @note: 0.9883 looks like an approximation of 253/256.
 *
 * TIA/EIA/IS-733 2.4.3.3.6 6
 */
#define QCELP_BANDWIDTH_EXPANSION_COEFF 0.9883

#endif /* AVCODEC_QCELPDATA_H */
