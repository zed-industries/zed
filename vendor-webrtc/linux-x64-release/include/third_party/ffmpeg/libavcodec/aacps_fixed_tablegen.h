/*
 * Header file for hardcoded Parametric Stereo tables
 *
 * Copyright (c) 2010 Alex Converse <alex.converse@gmail.com>
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
 *
 * Note: Rounding-to-nearest used unless otherwise stated
 *
 */

#ifndef AVCODEC_AACPS_FIXED_TABLEGEN_H
#define AVCODEC_AACPS_FIXED_TABLEGEN_H

#include <math.h>
#include <stdint.h>

#if CONFIG_HARDCODED_TABLES
#define ps_tableinit()
#define TABLE_CONST const
#include "libavcodec/aacps_fixed_tables.h"
#else
#include "libavutil/common.h"
#include "libavutil/mathematics.h"
#ifdef BUILD_TABLES
#undef DECLARE_ALIGNED
#define DECLARE_ALIGNED(align, type, variable) type variable
#else
#include "libavutil/mem_internal.h"
#endif

#include "aac_defines.h"
#include "libavutil/softfloat.h"
#define NR_ALLPASS_BANDS20 30
#define NR_ALLPASS_BANDS34 50
#define PS_AP_LINKS 3
#define TABLE_CONST
static int pd_re_smooth[8*8*8];
static int pd_im_smooth[8*8*8];
static int HA[46][8][4];
static int HB[46][8][4];
static DECLARE_ALIGNED(16, int, f20_0_8) [ 8][8][2];
static DECLARE_ALIGNED(16, int, f34_0_12)[12][8][2];
static DECLARE_ALIGNED(16, int, f34_1_8) [ 8][8][2];
static DECLARE_ALIGNED(16, int, f34_2_4) [ 4][8][2];
static TABLE_CONST DECLARE_ALIGNED(16, int, Q_fract_allpass)[2][50][3][2];
static DECLARE_ALIGNED(16, int, phi_fract)[2][50][2];

static const int g0_Q8[] = {
    Q31(0.00746082949812f), Q31(0.02270420949825f), Q31(0.04546865930473f), Q31(0.07266113929591f),
    Q31(0.09885108575264f), Q31(0.11793710567217f), Q31(0.125f)
};

static const int g0_Q12[] = {
    Q31(0.04081179924692f), Q31(0.03812810994926f), Q31(0.05144908135699f), Q31(0.06399831151592f),
    Q31(0.07428313801106f), Q31(0.08100347892914f), Q31(0.08333333333333f)
};

static const int g1_Q8[] = {
    Q31(0.01565675600122f), Q31(0.03752716391991f), Q31(0.05417891378782f), Q31(0.08417044116767f),
    Q31(0.10307344158036f), Q31(0.12222452249753f), Q31(0.125f)
};

static const int g2_Q4[] = {
    Q31(-0.05908211155639f), Q31(-0.04871498374946f), Q31(0.0f),   Q31(0.07778723915851f),
    Q31( 0.16486303567403f), Q31( 0.23279856662996f), Q31(0.25f)
};

static const int sintbl_4[4]   = {           0,  1073741824,           0, -1073741824 };
static const int costbl_4[4]   = {  1073741824,           0, -1073741824,           0 };
static const int sintbl_8[8]   = {           0,   759250125,  1073741824,   759250125,
                                             0,  -759250125, -1073741824,  -759250125 };
static const int costbl_8[8]   = {  1073741824,   759250125,           0,  -759250125,
                                   -1073741824,  -759250125,           0,   759250125 };
static const int sintbl_12[12] = {           0,   536870912,   929887697,  1073741824,
                                     929887697,   536870912,           0,  -536870912,
                                    -929887697, -1073741824,  -929887697,  -536870912 };
static const int costbl_12[12] = {  1073741824,   929887697,   536870912,           0,
                                    -536870912,  -929887697, -1073741824,  -929887697,
                                    -536870912,           0,   536870912,   929887697 };

static void make_filters_from_proto(int (*filter)[8][2], const int *proto, int bands)
{

    const int *sinptr, *cosptr;
    int s, c, sinhalf, coshalf;
    int q, n;

    if (bands == 4) {
        sinptr = sintbl_4;
        cosptr = costbl_4;
        sinhalf = 759250125;
        coshalf = 759250125;
    } else if (bands == 8) {
        sinptr = sintbl_8;
        cosptr = costbl_8;
        sinhalf = 410903207;
        coshalf = 992008094;
    } else {
        sinptr = sintbl_12;
        cosptr = costbl_12;
        sinhalf = 277904834;
        coshalf = 1037154959;
    }

    for (q = 0; q < bands; q++) {
        for (n = 0; n < 7; n++) {
            int theta = (q*(n-6) + (n>>1) - 3) % bands;

            if (theta < 0)
                theta += bands;
            s = sinptr[theta];
            c = cosptr[theta];

            if (n & 1) {
                theta = (int)(((int64_t)c * coshalf - (int64_t)s * sinhalf + 0x20000000) >> 30);
                s = (int)(((int64_t)s * coshalf + (int64_t)c * sinhalf + 0x20000000) >> 30);
                c = theta;
            }
            filter[q][n][0] = (int)(((int64_t)proto[n] * c + 0x20000000) >> 30);
            filter[q][n][1] = -(int)(((int64_t)proto[n] * s + 0x20000000) >> 30);
        }
    }
}

static void ps_tableinit(void)
{
    static const int ipdopd_sin[] = { Q30(0), Q30(M_SQRT1_2), Q30(1), Q30( M_SQRT1_2), Q30( 0), Q30(-M_SQRT1_2), Q30(-1), Q30(-M_SQRT1_2) };
    static const int ipdopd_cos[] = { Q30(1), Q30(M_SQRT1_2), Q30(0), Q30(-M_SQRT1_2), Q30(-1), Q30(-M_SQRT1_2), Q30( 0), Q30( M_SQRT1_2) };
    int pd0, pd1, pd2;
    int idx;

    static const int alpha_tab[] =
    {
      Q30(1.5146213770f/M_PI), Q30(1.5181334019f/M_PI), Q30(1.5234849453f/M_PI), Q30(1.5369486809f/M_PI), Q30(1.5500687361f/M_PI), Q30(1.5679757595f/M_PI),
      Q30(1.4455626011f/M_PI), Q30(1.4531552792f/M_PI), Q30(1.4648091793f/M_PI), Q30(1.4945238829f/M_PI), Q30(1.5239057541f/M_PI), Q30(1.5644006729f/M_PI),
      Q30(1.3738563061f/M_PI), Q30(1.3851221800f/M_PI), Q30(1.4026404619f/M_PI), Q30(1.4484288692f/M_PI), Q30(1.4949874878f/M_PI), Q30(1.5604078770f/M_PI),
      Q30(1.2645189762f/M_PI), Q30(1.2796478271f/M_PI), Q30(1.3038636446f/M_PI), Q30(1.3710125685f/M_PI), Q30(1.4443849325f/M_PI), Q30(1.5532352924f/M_PI),
      Q30(1.1507037878f/M_PI), Q30(1.1669205427f/M_PI), Q30(1.1938756704f/M_PI), Q30(1.2754167318f/M_PI), Q30(1.3761177063f/M_PI), Q30(1.5429240465f/M_PI),
      Q30(1.0079245567f/M_PI), Q30(1.0208238363f/M_PI), Q30(1.0433073044f/M_PI), Q30(1.1208510399f/M_PI), Q30(1.2424604893f/M_PI), Q30(1.5185726881f/M_PI),
      Q30(0.8995233774f/M_PI), Q30(0.9069069624f/M_PI), Q30(0.9201194048f/M_PI), Q30(0.9698365927f/M_PI), Q30(1.0671583414f/M_PI), Q30(1.4647934437f/M_PI),
      Q30(0.7853981853f/M_PI), Q30(0.7853981853f/M_PI), Q30(0.7853981853f/M_PI), Q30(0.7853981853f/M_PI), Q30(0.7853981853f/M_PI), Q30(0.7853981853f/M_PI),
      Q30(0.6712729335f/M_PI), Q30(0.6638893485f/M_PI), Q30(0.6506769061f/M_PI), Q30(0.6009597182f/M_PI), Q30(0.5036380291f/M_PI), Q30(0.1060028747f/M_PI),
      Q30(0.5628717542f/M_PI), Q30(0.5499725342f/M_PI), Q30(0.5274890065f/M_PI), Q30(0.4499453008f/M_PI), Q30(0.3283358216f/M_PI), Q30(0.0522236861f/M_PI),
      Q30(0.4200925827f/M_PI), Q30(0.4038758278f/M_PI), Q30(0.3769206405f/M_PI), Q30(0.2953795493f/M_PI), Q30(0.1946786791f/M_PI), Q30(0.0278722942f/M_PI),
      Q30(0.3062773645f/M_PI), Q30(0.2911485136f/M_PI), Q30(0.2669326365f/M_PI), Q30(0.1997837722f/M_PI), Q30(0.1264114529f/M_PI), Q30(0.0175609849f/M_PI),
      Q30(0.1969399750f/M_PI), Q30(0.1856741160f/M_PI), Q30(0.1681558639f/M_PI), Q30(0.1223674342f/M_PI), Q30(0.0758088827f/M_PI), Q30(0.0103884479f/M_PI),
      Q30(0.1252337098f/M_PI), Q30(0.1176410317f/M_PI), Q30(0.1059871912f/M_PI), Q30(0.0762724727f/M_PI), Q30(0.0468905345f/M_PI), Q30(0.0063956482f/M_PI),
      Q30(0.0561749674f/M_PI), Q30(0.0526629239f/M_PI), Q30(0.0473113805f/M_PI), Q30(0.0338476151f/M_PI), Q30(0.0207276177f/M_PI), Q30(0.0028205961f/M_PI),
      Q30(1.5676341057f/M_PI), Q30(1.5678333044f/M_PI), Q30(1.5681363344f/M_PI), Q30(1.5688960552f/M_PI), Q30(1.5696337223f/M_PI), Q30(1.5706381798f/M_PI),
      Q30(1.5651730299f/M_PI), Q30(1.5655272007f/M_PI), Q30(1.5660660267f/M_PI), Q30(1.5674170256f/M_PI), Q30(1.5687289238f/M_PI), Q30(1.5705151558f/M_PI),
      Q30(1.5607966185f/M_PI), Q30(1.5614265203f/M_PI), Q30(1.5623844862f/M_PI), Q30(1.5647867918f/M_PI), Q30(1.5671195984f/M_PI), Q30(1.5702962875f/M_PI),
      Q30(1.5530153513f/M_PI), Q30(1.5541347265f/M_PI), Q30(1.5558375120f/M_PI), Q30(1.5601085424f/M_PI), Q30(1.5642569065f/M_PI), Q30(1.5699069500f/M_PI),
      Q30(1.5391840935f/M_PI), Q30(1.5411708355f/M_PI), Q30(1.5441943407f/M_PI), Q30(1.5517836809f/M_PI), Q30(1.5591609478f/M_PI), Q30(1.5692136288f/M_PI),
      Q30(1.5146213770f/M_PI), Q30(1.5181334019f/M_PI), Q30(1.5234849453f/M_PI), Q30(1.5369486809f/M_PI), Q30(1.5500687361f/M_PI), Q30(1.5679757595f/M_PI),
      Q30(1.4915299416f/M_PI), Q30(1.4964480400f/M_PI), Q30(1.5039558411f/M_PI), Q30(1.5229074955f/M_PI), Q30(1.5414420366f/M_PI), Q30(1.5667995214f/M_PI),
      Q30(1.4590617418f/M_PI), Q30(1.4658898115f/M_PI), Q30(1.4763505459f/M_PI), Q30(1.5029321909f/M_PI), Q30(1.5291173458f/M_PI), Q30(1.5651149750f/M_PI),
      Q30(1.4136143923f/M_PI), Q30(1.4229322672f/M_PI), Q30(1.4373078346f/M_PI), Q30(1.4743183851f/M_PI), Q30(1.5113102198f/M_PI), Q30(1.5626684427f/M_PI),
      Q30(1.3505556583f/M_PI), Q30(1.3628427982f/M_PI), Q30(1.3820509911f/M_PI), Q30(1.4327841997f/M_PI), Q30(1.4850014448f/M_PI), Q30(1.5590143204f/M_PI),
      Q30(1.2645189762f/M_PI), Q30(1.2796478271f/M_PI), Q30(1.3038636446f/M_PI), Q30(1.3710125685f/M_PI), Q30(1.4443849325f/M_PI), Q30(1.5532352924f/M_PI),
      Q30(1.1919227839f/M_PI), Q30(1.2081253529f/M_PI), Q30(1.2346779108f/M_PI), Q30(1.3123005629f/M_PI), Q30(1.4034168720f/M_PI), Q30(1.5471596718f/M_PI),
      Q30(1.1061993837f/M_PI), Q30(1.1219338179f/M_PI), Q30(1.1484941244f/M_PI), Q30(1.2320860624f/M_PI), Q30(1.3421301842f/M_PI), Q30(1.5373806953f/M_PI),
      Q30(1.0079245567f/M_PI), Q30(1.0208238363f/M_PI), Q30(1.0433073044f/M_PI), Q30(1.1208510399f/M_PI), Q30(1.2424604893f/M_PI), Q30(1.5185726881f/M_PI),
      Q30(0.8995233774f/M_PI), Q30(0.9069069624f/M_PI), Q30(0.9201194048f/M_PI), Q30(0.9698365927f/M_PI), Q30(1.0671583414f/M_PI), Q30(1.4647934437f/M_PI),
      Q30(0.7853981853f/M_PI), Q30(0.7853981853f/M_PI), Q30(0.7853981853f/M_PI), Q30(0.7853981853f/M_PI), Q30(0.7853981853f/M_PI), Q30(0.7853981853f/M_PI),
      Q30(0.6712729335f/M_PI), Q30(0.6638893485f/M_PI), Q30(0.6506769061f/M_PI), Q30(0.6009597182f/M_PI), Q30(0.5036380291f/M_PI), Q30(0.1060028747f/M_PI),
      Q30(0.5628717542f/M_PI), Q30(0.5499725342f/M_PI), Q30(0.5274890065f/M_PI), Q30(0.4499453008f/M_PI), Q30(0.3283358216f/M_PI), Q30(0.0522236861f/M_PI),
      Q30(0.4645969570f/M_PI), Q30(0.4488625824f/M_PI), Q30(0.4223022461f/M_PI), Q30(0.3387103081f/M_PI), Q30(0.2286661267f/M_PI), Q30(0.0334156826f/M_PI),
      Q30(0.3788735867f/M_PI), Q30(0.3626709878f/M_PI), Q30(0.3361184299f/M_PI), Q30(0.2584958076f/M_PI), Q30(0.1673794836f/M_PI), Q30(0.0236366931f/M_PI),
      Q30(0.3062773645f/M_PI), Q30(0.2911485136f/M_PI), Q30(0.2669326365f/M_PI), Q30(0.1997837722f/M_PI), Q30(0.1264114529f/M_PI), Q30(0.0175609849f/M_PI),
      Q30(0.2202406377f/M_PI), Q30(0.2079535723f/M_PI), Q30(0.1887452900f/M_PI), Q30(0.1380121708f/M_PI), Q30(0.0857949182f/M_PI), Q30(0.0117820343f/M_PI),
      Q30(0.1571819335f/M_PI), Q30(0.1478640437f/M_PI), Q30(0.1334884763f/M_PI), Q30(0.0964778885f/M_PI), Q30(0.0594860613f/M_PI), Q30(0.0081279324f/M_PI),
      Q30(0.1117345318f/M_PI), Q30(0.1049065739f/M_PI), Q30(0.0944457650f/M_PI), Q30(0.0678641573f/M_PI), Q30(0.0416790098f/M_PI), Q30(0.0056813755f/M_PI),
      Q30(0.0792663917f/M_PI), Q30(0.0743482932f/M_PI), Q30(0.0668405443f/M_PI), Q30(0.0478888862f/M_PI), Q30(0.0293543357f/M_PI), Q30(0.0039967746f/M_PI),
      Q30(0.0561749674f/M_PI), Q30(0.0526629239f/M_PI), Q30(0.0473113805f/M_PI), Q30(0.0338476151f/M_PI), Q30(0.0207276177f/M_PI), Q30(0.0028205961f/M_PI),
      Q30(0.0316122435f/M_PI), Q30(0.0296254847f/M_PI), Q30(0.0266019460f/M_PI), Q30(0.0190126132f/M_PI), Q30(0.0116353342f/M_PI), Q30(0.0015827164f/M_PI),
      Q30(0.0177809205f/M_PI), Q30(0.0166615788f/M_PI), Q30(0.0149587989f/M_PI), Q30(0.0106877899f/M_PI), Q30(0.0065393616f/M_PI), Q30(0.0008894200f/M_PI),
      Q30(0.0099996664f/M_PI), Q30(0.0093698399f/M_PI), Q30(0.0084118480f/M_PI), Q30(0.0060095116f/M_PI), Q30(0.0036767013f/M_PI), Q30(0.0005000498f/M_PI),
      Q30(0.0056233541f/M_PI), Q30(0.0052691097f/M_PI), Q30(0.0047303112f/M_PI), Q30(0.0033792770f/M_PI), Q30(0.0020674451f/M_PI), Q30(0.0002811795f/M_PI),
      Q30(0.0031622672f/M_PI), Q30(0.0029630491f/M_PI), Q30(0.0026600463f/M_PI), Q30(0.0019002859f/M_PI), Q30(0.0011625893f/M_PI), Q30(0.0001581155f/M_PI)
    };

    static const int gamma_tab[] =
    {
      Q30(0.0000000000f/M_PI), Q30(0.0195873566f/M_PI), Q30(0.0303316917f/M_PI), Q30(0.0448668823f/M_PI), Q30(0.0522258915f/M_PI), Q30(0.0561044961f/M_PI),
      Q30(0.0000000000f/M_PI), Q30(0.0433459543f/M_PI), Q30(0.0672172382f/M_PI), Q30(0.0997167900f/M_PI), Q30(0.1162951663f/M_PI), Q30(0.1250736862f/M_PI),
      Q30(0.0000000000f/M_PI), Q30(0.0672341362f/M_PI), Q30(0.1045235619f/M_PI), Q30(0.1558904350f/M_PI), Q30(0.1824723780f/M_PI), Q30(0.1966800541f/M_PI),
      Q30(0.0000000000f/M_PI), Q30(0.1011129096f/M_PI), Q30(0.1580764502f/M_PI), Q30(0.2387557179f/M_PI), Q30(0.2820728719f/M_PI), Q30(0.3058380187f/M_PI),
      Q30(0.0000000000f/M_PI), Q30(0.1315985769f/M_PI), Q30(0.2072522491f/M_PI), Q30(0.3188187480f/M_PI), Q30(0.3825501204f/M_PI), Q30(0.4193951190f/M_PI),
      Q30(0.0000000000f/M_PI), Q30(0.1603866369f/M_PI), Q30(0.2549437582f/M_PI), Q30(0.4029446840f/M_PI), Q30(0.4980689585f/M_PI), Q30(0.5615641475f/M_PI),
      Q30(0.0000000000f/M_PI), Q30(0.1736015975f/M_PI), Q30(0.2773745656f/M_PI), Q30(0.4461984038f/M_PI), Q30(0.5666890144f/M_PI), Q30(0.6686112881f/M_PI),
      Q30(0.0000000000f/M_PI), Q30(0.1784276664f/M_PI), Q30(0.2856673002f/M_PI), Q30(0.4630723596f/M_PI), Q30(0.5971632004f/M_PI), Q30(0.7603877187f/M_PI),
      Q30(0.0000000000f/M_PI), Q30(0.1736015975f/M_PI), Q30(0.2773745656f/M_PI), Q30(0.4461984038f/M_PI), Q30(0.5666890144f/M_PI), Q30(0.6686112881f/M_PI),
      Q30(0.0000000000f/M_PI), Q30(0.1603866369f/M_PI), Q30(0.2549437582f/M_PI), Q30(0.4029446840f/M_PI), Q30(0.4980689585f/M_PI), Q30(0.5615641475f/M_PI),
      Q30(0.0000000000f/M_PI), Q30(0.1315985769f/M_PI), Q30(0.2072522491f/M_PI), Q30(0.3188187480f/M_PI), Q30(0.3825501204f/M_PI), Q30(0.4193951190f/M_PI),
      Q30(0.0000000000f/M_PI), Q30(0.1011129096f/M_PI), Q30(0.1580764502f/M_PI), Q30(0.2387557179f/M_PI), Q30(0.2820728719f/M_PI), Q30(0.3058380187f/M_PI),
      Q30(0.0000000000f/M_PI), Q30(0.0672341362f/M_PI), Q30(0.1045235619f/M_PI), Q30(0.1558904350f/M_PI), Q30(0.1824723780f/M_PI), Q30(0.1966800541f/M_PI),
      Q30(0.0000000000f/M_PI), Q30(0.0433459543f/M_PI), Q30(0.0672172382f/M_PI), Q30(0.0997167900f/M_PI), Q30(0.1162951663f/M_PI), Q30(0.1250736862f/M_PI),
      Q30(0.0000000000f/M_PI), Q30(0.0195873566f/M_PI), Q30(0.0303316917f/M_PI), Q30(0.0448668823f/M_PI), Q30(0.0522258915f/M_PI), Q30(0.0561044961f/M_PI),
      Q30(0.0000000000f/M_PI), Q30(0.0011053939f/M_PI), Q30(0.0017089852f/M_PI), Q30(0.0025254129f/M_PI), Q30(0.0029398468f/M_PI), Q30(0.0031597170f/M_PI),
      Q30(0.0000000000f/M_PI), Q30(0.0019607407f/M_PI), Q30(0.0030395309f/M_PI), Q30(0.0044951206f/M_PI), Q30(0.0052305623f/M_PI), Q30(0.0056152637f/M_PI),
      Q30(0.0000000000f/M_PI), Q30(0.0034913034f/M_PI), Q30(0.0054070661f/M_PI), Q30(0.0079917293f/M_PI), Q30(0.0092999367f/M_PI), Q30(0.0099875759f/M_PI),
      Q30(0.0000000000f/M_PI), Q30(0.0062100487f/M_PI), Q30(0.0096135242f/M_PI), Q30(0.0142110568f/M_PI), Q30(0.0165348612f/M_PI), Q30(0.0177587029f/M_PI),
      Q30(0.0000000000f/M_PI), Q30(0.0110366223f/M_PI), Q30(0.0170863140f/M_PI), Q30(0.0252620988f/M_PI), Q30(0.0293955617f/M_PI), Q30(0.0315726399f/M_PI),
      Q30(0.0000000000f/M_PI), Q30(0.0195873566f/M_PI), Q30(0.0303316917f/M_PI), Q30(0.0448668823f/M_PI), Q30(0.0522258915f/M_PI), Q30(0.0561044961f/M_PI),
      Q30(0.0000000000f/M_PI), Q30(0.0275881495f/M_PI), Q30(0.0427365713f/M_PI), Q30(0.0632618815f/M_PI), Q30(0.0736731067f/M_PI), Q30(0.0791663304f/M_PI),
      Q30(0.0000000000f/M_PI), Q30(0.0387469754f/M_PI), Q30(0.0600636788f/M_PI), Q30(0.0890387669f/M_PI), Q30(0.1037906483f/M_PI), Q30(0.1115923747f/M_PI),
      Q30(0.0000000000f/M_PI), Q30(0.0541138873f/M_PI), Q30(0.0839984417f/M_PI), Q30(0.1248718798f/M_PI), Q30(0.1458375156f/M_PI), Q30(0.1569785923f/M_PI),
      Q30(0.0000000000f/M_PI), Q30(0.0747506917f/M_PI), Q30(0.1163287833f/M_PI), Q30(0.1738867164f/M_PI), Q30(0.2038587779f/M_PI), Q30(0.2199459076f/M_PI),
      Q30(0.0000000000f/M_PI), Q30(0.1011129096f/M_PI), Q30(0.1580764502f/M_PI), Q30(0.2387557179f/M_PI), Q30(0.2820728719f/M_PI), Q30(0.3058380187f/M_PI),
      Q30(0.0000000000f/M_PI), Q30(0.1212290376f/M_PI), Q30(0.1903949380f/M_PI), Q30(0.2907958031f/M_PI), Q30(0.3466993868f/M_PI), Q30(0.3782821596f/M_PI),
      Q30(0.0000000000f/M_PI), Q30(0.1418247074f/M_PI), Q30(0.2240308374f/M_PI), Q30(0.3474813402f/M_PI), Q30(0.4202919006f/M_PI), Q30(0.4637607038f/M_PI),
      Q30(0.0000000000f/M_PI), Q30(0.1603866369f/M_PI), Q30(0.2549437582f/M_PI), Q30(0.4029446840f/M_PI), Q30(0.4980689585f/M_PI), Q30(0.5615641475f/M_PI),
      Q30(0.0000000000f/M_PI), Q30(0.1736015975f/M_PI), Q30(0.2773745656f/M_PI), Q30(0.4461984038f/M_PI), Q30(0.5666890144f/M_PI), Q30(0.6686112881f/M_PI),
      Q30(0.0000000000f/M_PI), Q30(0.1784276664f/M_PI), Q30(0.2856673002f/M_PI), Q30(0.4630723596f/M_PI), Q30(0.5971632004f/M_PI), Q30(0.7603877187f/M_PI),
      Q30(0.0000000000f/M_PI), Q30(0.1736015975f/M_PI), Q30(0.2773745656f/M_PI), Q30(0.4461984038f/M_PI), Q30(0.5666890144f/M_PI), Q30(0.6686112881f/M_PI),
      Q30(0.0000000000f/M_PI), Q30(0.1603866369f/M_PI), Q30(0.2549437582f/M_PI), Q30(0.4029446840f/M_PI), Q30(0.4980689585f/M_PI), Q30(0.5615641475f/M_PI),
      Q30(0.0000000000f/M_PI), Q30(0.1418247074f/M_PI), Q30(0.2240308374f/M_PI), Q30(0.3474813402f/M_PI), Q30(0.4202919006f/M_PI), Q30(0.4637607038f/M_PI),
      Q30(0.0000000000f/M_PI), Q30(0.1212290376f/M_PI), Q30(0.1903949380f/M_PI), Q30(0.2907958031f/M_PI), Q30(0.3466993868f/M_PI), Q30(0.3782821596f/M_PI),
      Q30(0.0000000000f/M_PI), Q30(0.1011129096f/M_PI), Q30(0.1580764502f/M_PI), Q30(0.2387557179f/M_PI), Q30(0.2820728719f/M_PI), Q30(0.3058380187f/M_PI),
      Q30(0.0000000000f/M_PI), Q30(0.0747506917f/M_PI), Q30(0.1163287833f/M_PI), Q30(0.1738867164f/M_PI), Q30(0.2038587779f/M_PI), Q30(0.2199459076f/M_PI),
      Q30(0.0000000000f/M_PI), Q30(0.0541138873f/M_PI), Q30(0.0839984417f/M_PI), Q30(0.1248718798f/M_PI), Q30(0.1458375156f/M_PI), Q30(0.1569785923f/M_PI),
      Q30(0.0000000000f/M_PI), Q30(0.0387469754f/M_PI), Q30(0.0600636788f/M_PI), Q30(0.0890387669f/M_PI), Q30(0.1037906483f/M_PI), Q30(0.1115923747f/M_PI),
      Q30(0.0000000000f/M_PI), Q30(0.0275881495f/M_PI), Q30(0.0427365713f/M_PI), Q30(0.0632618815f/M_PI), Q30(0.0736731067f/M_PI), Q30(0.0791663304f/M_PI),
      Q30(0.0000000000f/M_PI), Q30(0.0195873566f/M_PI), Q30(0.0303316917f/M_PI), Q30(0.0448668823f/M_PI), Q30(0.0522258915f/M_PI), Q30(0.0561044961f/M_PI),
      Q30(0.0000000000f/M_PI), Q30(0.0110366223f/M_PI), Q30(0.0170863140f/M_PI), Q30(0.0252620988f/M_PI), Q30(0.0293955617f/M_PI), Q30(0.0315726399f/M_PI),
      Q30(0.0000000000f/M_PI), Q30(0.0062100487f/M_PI), Q30(0.0096135242f/M_PI), Q30(0.0142110568f/M_PI), Q30(0.0165348612f/M_PI), Q30(0.0177587029f/M_PI),
      Q30(0.0000000000f/M_PI), Q30(0.0034913034f/M_PI), Q30(0.0054070661f/M_PI), Q30(0.0079917293f/M_PI), Q30(0.0092999367f/M_PI), Q30(0.0099875759f/M_PI),
      Q30(0.0000000000f/M_PI), Q30(0.0019607407f/M_PI), Q30(0.0030395309f/M_PI), Q30(0.0044951206f/M_PI), Q30(0.0052305623f/M_PI), Q30(0.0056152637f/M_PI),
      Q30(0.0000000000f/M_PI), Q30(0.0011053939f/M_PI), Q30(0.0017089852f/M_PI), Q30(0.0025254129f/M_PI), Q30(0.0029398468f/M_PI), Q30(0.0031597170f/M_PI)
    };

    static const int iid_par_dequant_c1[] = {
        //iid_par_dequant_default
        Q30(1.41198278375959f), Q30(1.40313815268360f), Q30(1.38687670404960f), Q30(1.34839972492648f),
        Q30(1.29124937110028f), Q30(1.19603741667993f), Q30(1.10737240362323f), Q30(1),
        Q30(0.87961716655242f), Q30(0.75464859232732f), Q30(0.57677990744575f), Q30(0.42640143271122f),
        Q30(0.27671828230984f), Q30(0.17664462766713f), Q30(0.07940162697653f),
        //iid_par_dequant_fine
        Q30(1.41420649135832f), Q30(1.41419120222364f), Q30(1.41414285699784f), Q30(1.41399000859438f),
        Q30(1.41350698548044f), Q30(1.41198278375959f), Q30(1.40977302262355f), Q30(1.40539479488545f),
        Q30(1.39677960498402f), Q30(1.38005309967827f), Q30(1.34839972492648f), Q30(1.31392017367631f),
        Q30(1.26431008149654f), Q30(1.19603741667993f), Q30(1.10737240362323f), Q30(1),
        Q30(0.87961716655242f), Q30(0.75464859232732f), Q30(0.63365607219232f), Q30(0.52308104267543f),
        Q30(0.42640143271122f), Q30(0.30895540465965f), Q30(0.22137464873077f), Q30(0.15768788954414f),
        Q30(0.11198225164225f), Q30(0.07940162697653f), Q30(0.04469901562677f), Q30(0.02514469318284f),
        Q30(0.01414142856998f), Q30(0.00795258154731f), Q30(0.00447211359449f),
    };

    static const int acos_icc_invq[] = {
        Q31(0), Q31(0.178427635f/M_PI), Q31(0.28566733f/M_PI), Q31(0.46307236f/M_PI), Q31(0.59716315f/M_PI), Q31(0.78539816f/M_PI), Q31(1.10030855f/M_PI), Q31(1.57079633f/M_PI)
    };
    int iid, icc;

    int k, m;
    static const int8_t f_center_20[] = {
        -3, -1, 1, 3, 5, 7, 10, 14, 18, 22,
    };
    static const int32_t f_center_34[] = {
      Q31(  2/768.0),Q31(  6/768.0),Q31(10/768.0),Q31(14/768.0),Q31( 18/768.0),Q31( 22/768.0),Q31( 26/768.0),Q31(30/768.0),
      Q31( 34/768.0),Q31(-10/768.0),Q31(-6/768.0),Q31(-2/768.0),Q31( 51/768.0),Q31( 57/768.0),Q31( 15/768.0),Q31(21/768.0),
      Q31( 27/768.0),Q31( 33/768.0),Q31(39/768.0),Q31(45/768.0),Q31( 54/768.0),Q31( 66/768.0),Q31( 78/768.0),Q31(42/768.0),
      Q31(102/768.0),Q31( 66/768.0),Q31(78/768.0),Q31(90/768.0),Q31(102/768.0),Q31(114/768.0),Q31(126/768.0),Q31(90/768.0)
    };
    static const int fractional_delay_links[] = { Q31(0.43f), Q31(0.75f), Q31(0.347f) };
    const int fractional_delay_gain = Q31(0.39f);

    for (pd0 = 0; pd0 < 8; pd0++) {
        int pd0_re = (ipdopd_cos[pd0]+2)>>2;
        int pd0_im = (ipdopd_sin[pd0]+2)>>2;
        for (pd1 = 0; pd1 < 8; pd1++) {
            int pd1_re = ipdopd_cos[pd1] >> 1;
            int pd1_im = ipdopd_sin[pd1] >> 1;
            for (pd2 = 0; pd2 < 8; pd2++) {
                int shift, round;
                int pd2_re = ipdopd_cos[pd2];
                int pd2_im = ipdopd_sin[pd2];
                int re_smooth = pd0_re + pd1_re + pd2_re;
                int im_smooth = pd0_im + pd1_im + pd2_im;

                SoftFloat pd_mag = av_int2sf(((ipdopd_cos[(pd0-pd1)&7]+8)>>4) + ((ipdopd_cos[(pd0-pd2)&7]+4)>>3) +
                                               ((ipdopd_cos[(pd1-pd2)&7]+2)>>2) + 0x15000000, 28);
                pd_mag = av_div_sf(FLOAT_1, av_sqrt_sf(pd_mag));
                shift = 30 - pd_mag.exp;
                round = 1 << (shift-1);
                pd_re_smooth[pd0*64+pd1*8+pd2] = (int)(((int64_t)re_smooth * pd_mag.mant + round) >> shift);
                pd_im_smooth[pd0*64+pd1*8+pd2] = (int)(((int64_t)im_smooth * pd_mag.mant + round) >> shift);
            }
        }
    }

    idx = 0;
    for (iid = 0; iid < 46; iid++) {
        int c1, c2;

        c1 = iid_par_dequant_c1[iid];
        if (iid < 15)
          c2 = iid_par_dequant_c1[14-iid];
        else
          c2 = iid_par_dequant_c1[60-iid];

        for (icc = 0; icc < 8; icc++) {
            /*if (PS_BASELINE || ps->icc_mode < 3)*/{
                int alpha, beta;
                int ca, sa, cb, sb;

                alpha = acos_icc_invq[icc];
                beta = (int)(((int64_t)alpha * 1518500250 + 0x40000000) >> 31);
                alpha >>= 1;
                beta = (int)(((int64_t)beta * (c1 - c2) + 0x40000000) >> 31);
                av_sincos_sf(beta + alpha, &sa, &ca);
                av_sincos_sf(beta - alpha, &sb, &cb);

                HA[iid][icc][0] = (int)(((int64_t)c2 * ca + 0x20000000) >> 30);
                HA[iid][icc][1] = (int)(((int64_t)c1 * cb + 0x20000000) >> 30);
                HA[iid][icc][2] = (int)(((int64_t)c2 * sa + 0x20000000) >> 30);
                HA[iid][icc][3] = (int)(((int64_t)c1 * sb + 0x20000000) >> 30);
            } /* else */ {
                int alpha_int, gamma_int;
                int alpha_c_int, alpha_s_int, gamma_c_int, gamma_s_int;

                alpha_int = alpha_tab[idx];
                gamma_int = gamma_tab[idx];

                av_sincos_sf(alpha_int, &alpha_s_int, &alpha_c_int);
                av_sincos_sf(gamma_int, &gamma_s_int, &gamma_c_int);

                alpha_c_int = (int)(((int64_t)alpha_c_int * 1518500250 + 0x20000000) >> 30);
                alpha_s_int = (int)(((int64_t)alpha_s_int * 1518500250 + 0x20000000) >> 30);

                HB[iid][icc][0] = (int)(((int64_t)alpha_c_int * gamma_c_int + 0x20000000) >> 30);
                HB[iid][icc][1] = (int)(((int64_t)alpha_s_int * gamma_c_int + 0x20000000) >> 30);
                HB[iid][icc][2] = -(int)(((int64_t)alpha_s_int * gamma_s_int + 0x20000000) >> 30);
                HB[iid][icc][3] = (int)(((int64_t)alpha_c_int * gamma_s_int + 0x20000000) >> 30);
            }

            if (icc < 5 || icc > 6)
              idx++;
        }
    }

    for (k = 0; k < NR_ALLPASS_BANDS20; k++) {
        int theta;
        int64_t f_center;
        int c, s;

        if (k < FF_ARRAY_ELEMS(f_center_20))
          f_center = f_center_20[k];
        else
          f_center = (k << 3) - 52;

        for (m = 0; m < PS_AP_LINKS; m++) {
            theta = (int)(((int64_t)fractional_delay_links[m] * f_center + 8) >> 4);
            av_sincos_sf(-theta, &s, &c);
            Q_fract_allpass[0][k][m][0] = c;
            Q_fract_allpass[0][k][m][1] = s;
        }

        theta = (int)(((int64_t)fractional_delay_gain * f_center + 8) >> 4);
        av_sincos_sf(-theta, &s, &c);
        phi_fract[0][k][0] = c;
        phi_fract[0][k][1] = s;
    }

    for (k = 0; k < NR_ALLPASS_BANDS34; k++) {
        int theta, f_center;
        int c, s;

        if (k < FF_ARRAY_ELEMS(f_center_34))
            f_center = f_center_34[k];
        else
            f_center = ((int64_t)k << 26) - (53 << 25);

        for (m = 0; m < PS_AP_LINKS; m++) {
            theta = (int)(((int64_t)fractional_delay_links[m] * f_center + 0x10000000) >> 27);
            av_sincos_sf(-theta, &s, &c);
            Q_fract_allpass[1][k][m][0] = c;
            Q_fract_allpass[1][k][m][1] = s;
        }

        theta = (int)(((int64_t)fractional_delay_gain * f_center + 0x10000000) >> 27);
        av_sincos_sf(-theta, &s, &c);
        phi_fract[1][k][0] = c;
        phi_fract[1][k][1] = s;
    }

    make_filters_from_proto(f20_0_8,  g0_Q8,   8);
    make_filters_from_proto(f34_0_12, g0_Q12, 12);
    make_filters_from_proto(f34_1_8,  g1_Q8,   8);
    make_filters_from_proto(f34_2_4,  g2_Q4,   4);
}
#endif /* CONFIG_HARDCODED_TABLES */

#endif /* AVCODEC_AACPS_FIXED_TABLEGEN_H */
