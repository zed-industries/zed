/*
 * AAC encoder twoloop coder
 * Copyright (C) 2008-2009 Konstantin Shishkov
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

/**
 * @file
 * AAC encoder twoloop coder
 * @author Konstantin Shishkov, Claudio Freire
 */

/**
 * This file contains a template for the twoloop coder function.
 * It needs to be provided, externally, as an already included declaration,
 * the following functions from aacenc_quantization/util.h. They're not included
 * explicitly here to make it possible to provide alternative implementations:
 *  - quantize_band_cost
 *  - abs_pow34_v
 *  - find_max_val
 *  - find_min_book
 *  - find_form_factor
 */

#ifndef AVCODEC_AACCODER_TWOLOOP_H
#define AVCODEC_AACCODER_TWOLOOP_H

#include <float.h>
#include "libavutil/mathematics.h"
#include "mathops.h"
#include "avcodec.h"
#include "put_bits.h"
#include "aac.h"
#include "aacenc.h"
#include "aactab.h"
#include "aacenctab.h"

/** Frequency in Hz for lower limit of noise substitution **/
#define NOISE_LOW_LIMIT 4000

/* Reflects the cost to change codebooks */
static inline int ff_pns_bits(SingleChannelElement *sce, int w, int g)
{
    return (!g || !sce->zeroes[w*16+g-1] || !sce->can_pns[w*16+g-1]) ? 9 : 5;
}

/**
 * two-loop quantizers search taken from ISO 13818-7 Appendix C
 */
static void search_for_quantizers_twoloop(AVCodecContext *avctx,
                                          AACEncContext *s,
                                          SingleChannelElement *sce,
                                          const float lambda)
{
    int start = 0, i, w, w2, g, recomprd;
    int destbits = avctx->bit_rate * 1024.0 / avctx->sample_rate
        / ((avctx->flags & AV_CODEC_FLAG_QSCALE) ? 2.0f : avctx->ch_layout.nb_channels)
        * (lambda / 120.f);
    int refbits = destbits;
    int toomanybits, toofewbits;
    char nzs[128];
    uint8_t nextband[128];
    int maxsf[128], minsf[128];
    float dists[128] = { 0 }, qenergies[128] = { 0 }, uplims[128], euplims[128], energies[128];
    float maxvals[128], spread_thr_r[128];
    float min_spread_thr_r, max_spread_thr_r;

    /**
     * rdlambda controls the maximum tolerated distortion. Twoloop
     * will keep iterating until it fails to lower it or it reaches
     * ulimit * rdlambda. Keeping it low increases quality on difficult
     * signals, but lower it too much, and bits will be taken from weak
     * signals, creating "holes". A balance is necessary.
     * rdmax and rdmin specify the relative deviation from rdlambda
     * allowed for tonality compensation
     */
    float rdlambda = av_clipf(2.0f * 120.f / lambda, 0.0625f, 16.0f);
    const float nzslope = 1.5f;
    float rdmin = 0.03125f;
    float rdmax = 1.0f;

    /**
     * sfoffs controls an offset of optmium allocation that will be
     * applied based on lambda. Keep it real and modest, the loop
     * will take care of the rest, this just accelerates convergence
     */
    float sfoffs = av_clipf(log2f(120.0f / lambda) * 4.0f, -5, 10);

    int fflag, minscaler, nminscaler;
    int its  = 0;
    int maxits = 30;
    int allz = 0;
    int tbits;
    int cutoff = 1024;
    int pns_start_pos;
    int prev;

    /**
     * zeroscale controls a multiplier of the threshold, if band energy
     * is below this, a zero is forced. Keep it lower than 1, unless
     * low lambda is used, because energy < threshold doesn't mean there's
     * no audible signal outright, it's just energy. Also make it rise
     * slower than rdlambda, as rdscale has due compensation with
     * noisy band depriorization below, whereas zeroing logic is rather dumb
     */
    float zeroscale;
    if (lambda > 120.f) {
        zeroscale = av_clipf(powf(120.f / lambda, 0.25f), 0.0625f, 1.0f);
    } else {
        zeroscale = 1.f;
    }

    if (s->psy.bitres.alloc >= 0) {
        /**
         * Psy granted us extra bits to use, from the reservoire
         * adjust for lambda except what psy already did
         */
        destbits = s->psy.bitres.alloc
            * (lambda / (avctx->global_quality ? avctx->global_quality : 120));
    }

    if (avctx->flags & AV_CODEC_FLAG_QSCALE) {
        /**
         * Constant Q-scale doesn't compensate MS coding on its own
         * No need to be overly precise, this only controls RD
         * adjustment CB limits when going overboard
         */
        if (s->options.mid_side && s->cur_type == TYPE_CPE)
            destbits *= 2;

        /**
         * When using a constant Q-scale, don't adjust bits, just use RD
         * Don't let it go overboard, though... 8x psy target is enough
         */
        toomanybits = 5800;
        toofewbits = destbits / 16;

        /** Don't offset scalers, just RD */
        sfoffs = sce->ics.num_windows - 1;
        rdlambda = sqrtf(rdlambda);

        /** search further */
        maxits *= 2;
    } else {
        /* When using ABR, be strict, but a reasonable leeway is
         * critical to allow RC to smoothly track desired bitrate
         * without sudden quality drops that cause audible artifacts.
         * Symmetry is also desirable, to avoid systematic bias.
         */
        toomanybits = destbits + destbits/8;
        toofewbits = destbits - destbits/8;

        sfoffs = 0;
        rdlambda = sqrtf(rdlambda);
    }

    /** and zero out above cutoff frequency */
    {
        int wlen = 1024 / sce->ics.num_windows;
        int bandwidth;

        /**
         * Scale, psy gives us constant quality, this LP only scales
         * bitrate by lambda, so we save bits on subjectively unimportant HF
         * rather than increase quantization noise. Adjust nominal bitrate
         * to effective bitrate according to encoding parameters,
         * AAC_CUTOFF_FROM_BITRATE is calibrated for effective bitrate.
         */
        float rate_bandwidth_multiplier = 1.5f;
        int frame_bit_rate = (avctx->flags & AV_CODEC_FLAG_QSCALE)
            ? (refbits * rate_bandwidth_multiplier * avctx->sample_rate / 1024)
            : (avctx->bit_rate / avctx->ch_layout.nb_channels);

        /** Compensate for extensions that increase efficiency */
        if (s->options.pns || s->options.intensity_stereo)
            frame_bit_rate *= 1.15f;

        if (avctx->cutoff > 0) {
            bandwidth = avctx->cutoff;
        } else {
            bandwidth = FFMAX(3000, AAC_CUTOFF_FROM_BITRATE(frame_bit_rate, 1, avctx->sample_rate));
            s->psy.cutoff = bandwidth;
        }

        cutoff = bandwidth * 2 * wlen / avctx->sample_rate;
        pns_start_pos = NOISE_LOW_LIMIT * 2 * wlen / avctx->sample_rate;
    }

    /**
     * for values above this the decoder might end up in an endless loop
     * due to always having more bits than what can be encoded.
     */
    destbits = FFMIN(destbits, 5800);
    toomanybits = FFMIN(toomanybits, 5800);
    toofewbits = FFMIN(toofewbits, 5800);
    /**
     * XXX: some heuristic to determine initial quantizers will reduce search time
     * determine zero bands and upper distortion limits
     */
    min_spread_thr_r = -1;
    max_spread_thr_r = -1;
    for (w = 0; w < sce->ics.num_windows; w += sce->ics.group_len[w]) {
        for (g = start = 0;  g < sce->ics.num_swb; start += sce->ics.swb_sizes[g++]) {
            int nz = 0;
            float uplim = 0.0f, energy = 0.0f, spread = 0.0f;
            for (w2 = 0; w2 < sce->ics.group_len[w]; w2++) {
                FFPsyBand *band = &s->psy.ch[s->cur_channel].psy_bands[(w+w2)*16+g];
                if (start >= cutoff || band->energy <= (band->threshold * zeroscale) || band->threshold == 0.0f) {
                    sce->zeroes[(w+w2)*16+g] = 1;
                    continue;
                }
                nz = 1;
            }
            if (!nz) {
                uplim = 0.0f;
            } else {
                nz = 0;
                for (w2 = 0; w2 < sce->ics.group_len[w]; w2++) {
                    FFPsyBand *band = &s->psy.ch[s->cur_channel].psy_bands[(w+w2)*16+g];
                    if (band->energy <= (band->threshold * zeroscale) || band->threshold == 0.0f)
                        continue;
                    uplim += band->threshold;
                    energy += band->energy;
                    spread += band->spread;
                    nz++;
                }
            }
            uplims[w*16+g] = uplim;
            energies[w*16+g] = energy;
            nzs[w*16+g] = nz;
            sce->zeroes[w*16+g] = !nz;
            allz |= nz;
            if (nz && sce->can_pns[w*16+g]) {
                spread_thr_r[w*16+g] = energy * nz / (uplim * spread);
                if (min_spread_thr_r < 0) {
                    min_spread_thr_r = max_spread_thr_r = spread_thr_r[w*16+g];
                } else {
                    min_spread_thr_r = FFMIN(min_spread_thr_r, spread_thr_r[w*16+g]);
                    max_spread_thr_r = FFMAX(max_spread_thr_r, spread_thr_r[w*16+g]);
                }
            }
        }
    }

    /** Compute initial scalers */
    minscaler = 65535;
    for (w = 0; w < sce->ics.num_windows; w += sce->ics.group_len[w]) {
        for (g = 0;  g < sce->ics.num_swb; g++) {
            if (sce->zeroes[w*16+g]) {
                sce->sf_idx[w*16+g] = SCALE_ONE_POS;
                continue;
            }
            /**
             * log2f-to-distortion ratio is, technically, 2 (1.5db = 4, but it's power vs level so it's 2).
             * But, as offsets are applied, low-frequency signals are too sensitive to the induced distortion,
             * so we make scaling more conservative by choosing a lower log2f-to-distortion ratio, and thus
             * more robust.
             */
            sce->sf_idx[w*16+g] = av_clip(
                SCALE_ONE_POS
                    + 1.75*log2f(FFMAX(0.00125f,uplims[w*16+g]) / sce->ics.swb_sizes[g])
                    + sfoffs,
                60, SCALE_MAX_POS);
            minscaler = FFMIN(minscaler, sce->sf_idx[w*16+g]);
        }
    }

    /** Clip */
    minscaler = av_clip(minscaler, SCALE_ONE_POS - SCALE_DIV_512, SCALE_MAX_POS - SCALE_DIV_512);
    for (w = 0; w < sce->ics.num_windows; w += sce->ics.group_len[w])
        for (g = 0;  g < sce->ics.num_swb; g++)
            if (!sce->zeroes[w*16+g])
                sce->sf_idx[w*16+g] = av_clip(sce->sf_idx[w*16+g], minscaler, minscaler + SCALE_MAX_DIFF - 1);

    if (!allz)
        return;
    s->aacdsp.abs_pow34(s->scoefs, sce->coeffs, 1024);
    ff_quantize_band_cost_cache_init(s);

    for (i = 0; i < sizeof(minsf) / sizeof(minsf[0]); ++i)
        minsf[i] = 0;
    for (w = 0; w < sce->ics.num_windows; w += sce->ics.group_len[w]) {
        start = w*128;
        for (g = 0;  g < sce->ics.num_swb; g++) {
            const float *scaled = s->scoefs + start;
            int minsfidx;
            maxvals[w*16+g] = find_max_val(sce->ics.group_len[w], sce->ics.swb_sizes[g], scaled);
            if (maxvals[w*16+g] > 0) {
                minsfidx = coef2minsf(maxvals[w*16+g]);
                for (w2 = 0; w2 < sce->ics.group_len[w]; w2++)
                    minsf[(w+w2)*16+g] = minsfidx;
            }
            start += sce->ics.swb_sizes[g];
        }
    }

    /**
     * Scale uplims to match rate distortion to quality
     * bu applying noisy band depriorization and tonal band priorization.
     * Maxval-energy ratio gives us an idea of how noisy/tonal the band is.
     * If maxval^2 ~ energy, then that band is mostly noise, and we can relax
     * rate distortion requirements.
     */
    memcpy(euplims, uplims, sizeof(euplims));
    for (w = 0; w < sce->ics.num_windows; w += sce->ics.group_len[w]) {
        /** psy already priorizes transients to some extent */
        float de_psy_factor = (sce->ics.num_windows > 1) ? 8.0f / sce->ics.group_len[w] : 1.0f;
        start = w*128;
        for (g = 0;  g < sce->ics.num_swb; g++) {
            if (nzs[g] > 0) {
                float cleanup_factor = ff_sqrf(av_clipf(start / (cutoff * 0.75f), 1.0f, 2.0f));
                float energy2uplim = find_form_factor(
                    sce->ics.group_len[w], sce->ics.swb_sizes[g],
                    uplims[w*16+g] / (nzs[g] * sce->ics.swb_sizes[w]),
                    sce->coeffs + start,
                    nzslope * cleanup_factor);
                energy2uplim *= de_psy_factor;
                if (!(avctx->flags & AV_CODEC_FLAG_QSCALE)) {
                    /** In ABR, we need to priorize less and let rate control do its thing */
                    energy2uplim = sqrtf(energy2uplim);
                }
                energy2uplim = FFMAX(0.015625f, FFMIN(1.0f, energy2uplim));
                uplims[w*16+g] *= av_clipf(rdlambda * energy2uplim, rdmin, rdmax)
                                  * sce->ics.group_len[w];

                energy2uplim = find_form_factor(
                    sce->ics.group_len[w], sce->ics.swb_sizes[g],
                    uplims[w*16+g] / (nzs[g] * sce->ics.swb_sizes[w]),
                    sce->coeffs + start,
                    2.0f);
                energy2uplim *= de_psy_factor;
                if (!(avctx->flags & AV_CODEC_FLAG_QSCALE)) {
                    /** In ABR, we need to priorize less and let rate control do its thing */
                    energy2uplim = sqrtf(energy2uplim);
                }
                energy2uplim = FFMAX(0.015625f, FFMIN(1.0f, energy2uplim));
                euplims[w*16+g] *= av_clipf(rdlambda * energy2uplim * sce->ics.group_len[w],
                    0.5f, 1.0f);
            }
            start += sce->ics.swb_sizes[g];
        }
    }

    for (i = 0; i < sizeof(maxsf) / sizeof(maxsf[0]); ++i)
        maxsf[i] = SCALE_MAX_POS;

    //perform two-loop search
    //outer loop - improve quality
    do {
        //inner loop - quantize spectrum to fit into given number of bits
        int overdist;
        int qstep = its ? 1 : 32;
        do {
            int changed = 0;
            prev = -1;
            recomprd = 0;
            tbits = 0;
            for (w = 0; w < sce->ics.num_windows; w += sce->ics.group_len[w]) {
                start = w*128;
                for (g = 0;  g < sce->ics.num_swb; g++) {
                    const float *coefs = &sce->coeffs[start];
                    const float *scaled = &s->scoefs[start];
                    int bits = 0;
                    int cb;
                    float dist = 0.0f;
                    float qenergy = 0.0f;

                    if (sce->zeroes[w*16+g] || sce->sf_idx[w*16+g] >= 218) {
                        start += sce->ics.swb_sizes[g];
                        if (sce->can_pns[w*16+g]) {
                            /** PNS isn't free */
                            tbits += ff_pns_bits(sce, w, g);
                        }
                        continue;
                    }
                    cb = find_min_book(maxvals[w*16+g], sce->sf_idx[w*16+g]);
                    for (w2 = 0; w2 < sce->ics.group_len[w]; w2++) {
                        int b;
                        float sqenergy;
                        dist += quantize_band_cost_cached(s, w + w2, g, coefs + w2*128,
                                                   scaled + w2*128,
                                                   sce->ics.swb_sizes[g],
                                                   sce->sf_idx[w*16+g],
                                                   cb,
                                                   1.0f,
                                                   INFINITY,
                                                   &b, &sqenergy,
                                                   0);
                        bits += b;
                        qenergy += sqenergy;
                    }
                    dists[w*16+g] = dist - bits;
                    qenergies[w*16+g] = qenergy;
                    if (prev != -1) {
                        int sfdiff = av_clip(sce->sf_idx[w*16+g] - prev + SCALE_DIFF_ZERO, 0, 2*SCALE_MAX_DIFF);
                        bits += ff_aac_scalefactor_bits[sfdiff];
                    }
                    tbits += bits;
                    start += sce->ics.swb_sizes[g];
                    prev = sce->sf_idx[w*16+g];
                }
            }
            if (tbits > toomanybits) {
                recomprd = 1;
                for (i = 0; i < 128; i++) {
                    if (sce->sf_idx[i] < (SCALE_MAX_POS - SCALE_DIV_512)) {
                        int maxsf_i = (tbits > 5800) ? SCALE_MAX_POS : maxsf[i];
                        int new_sf = FFMIN(maxsf_i, sce->sf_idx[i] + qstep);
                        if (new_sf != sce->sf_idx[i]) {
                            sce->sf_idx[i] = new_sf;
                            changed = 1;
                        }
                    }
                }
            } else if (tbits < toofewbits) {
                recomprd = 1;
                for (i = 0; i < 128; i++) {
                    if (sce->sf_idx[i] > SCALE_ONE_POS) {
                        int new_sf = FFMAX3(minsf[i], SCALE_ONE_POS, sce->sf_idx[i] - qstep);
                        if (new_sf != sce->sf_idx[i]) {
                            sce->sf_idx[i] = new_sf;
                            changed = 1;
                        }
                    }
                }
            }
            qstep >>= 1;
            if (!qstep && tbits > toomanybits && sce->sf_idx[0] < 217 && changed)
                qstep = 1;
        } while (qstep);

        overdist = 1;
        fflag = tbits < toofewbits;
        for (i = 0; i < 2 && (overdist || recomprd); ++i) {
            if (recomprd) {
                /** Must recompute distortion */
                prev = -1;
                tbits = 0;
                for (w = 0; w < sce->ics.num_windows; w += sce->ics.group_len[w]) {
                    start = w*128;
                    for (g = 0;  g < sce->ics.num_swb; g++) {
                        const float *coefs = sce->coeffs + start;
                        const float *scaled = s->scoefs + start;
                        int bits = 0;
                        int cb;
                        float dist = 0.0f;
                        float qenergy = 0.0f;

                        if (sce->zeroes[w*16+g] || sce->sf_idx[w*16+g] >= 218) {
                            start += sce->ics.swb_sizes[g];
                            if (sce->can_pns[w*16+g]) {
                                /** PNS isn't free */
                                tbits += ff_pns_bits(sce, w, g);
                            }
                            continue;
                        }
                        cb = find_min_book(maxvals[w*16+g], sce->sf_idx[w*16+g]);
                        for (w2 = 0; w2 < sce->ics.group_len[w]; w2++) {
                            int b;
                            float sqenergy;
                            dist += quantize_band_cost_cached(s, w + w2, g, coefs + w2*128,
                                                    scaled + w2*128,
                                                    sce->ics.swb_sizes[g],
                                                    sce->sf_idx[w*16+g],
                                                    cb,
                                                    1.0f,
                                                    INFINITY,
                                                    &b, &sqenergy,
                                                    0);
                            bits += b;
                            qenergy += sqenergy;
                        }
                        dists[w*16+g] = dist - bits;
                        qenergies[w*16+g] = qenergy;
                        if (prev != -1) {
                            int sfdiff = av_clip(sce->sf_idx[w*16+g] - prev + SCALE_DIFF_ZERO, 0, 2*SCALE_MAX_DIFF);
                            bits += ff_aac_scalefactor_bits[sfdiff];
                        }
                        tbits += bits;
                        start += sce->ics.swb_sizes[g];
                        prev = sce->sf_idx[w*16+g];
                    }
                }
            }
            if (!i && s->options.pns && its > maxits/2 && tbits > toofewbits) {
                float maxoverdist = 0.0f;
                float ovrfactor = 1.f+(maxits-its)*16.f/maxits;
                overdist = recomprd = 0;
                for (w = 0; w < sce->ics.num_windows; w += sce->ics.group_len[w]) {
                    for (g = start = 0;  g < sce->ics.num_swb; start += sce->ics.swb_sizes[g++]) {
                        if (!sce->zeroes[w*16+g] && sce->sf_idx[w*16+g] > SCALE_ONE_POS && dists[w*16+g] > uplims[w*16+g]*ovrfactor) {
                            float ovrdist = dists[w*16+g] / FFMAX(uplims[w*16+g],euplims[w*16+g]);
                            maxoverdist = FFMAX(maxoverdist, ovrdist);
                            overdist++;
                        }
                    }
                }
                if (overdist) {
                    /* We have overdistorted bands, trade for zeroes (that can be noise)
                     * Zero the bands in the lowest 1.25% spread-energy-threshold ranking
                     */
                    float minspread = max_spread_thr_r;
                    float maxspread = min_spread_thr_r;
                    float zspread;
                    int zeroable = 0;
                    int zeroed = 0;
                    int maxzeroed, zloop;
                    for (w = 0; w < sce->ics.num_windows; w += sce->ics.group_len[w]) {
                        for (g = start = 0;  g < sce->ics.num_swb; start += sce->ics.swb_sizes[g++]) {
                            if (start >= pns_start_pos && !sce->zeroes[w*16+g] && sce->can_pns[w*16+g]) {
                                minspread = FFMIN(minspread, spread_thr_r[w*16+g]);
                                maxspread = FFMAX(maxspread, spread_thr_r[w*16+g]);
                                zeroable++;
                            }
                        }
                    }
                    zspread = (maxspread-minspread) * 0.0125f + minspread;
                    /* Don't PNS everything even if allowed. It suppresses bit starvation signals from RC,
                     * and forced the hand of the later search_for_pns step.
                     * Instead, PNS a fraction of the spread_thr_r range depending on how starved for bits we are,
                     * and leave further PNSing to search_for_pns if worthwhile.
                     */
                    zspread = FFMIN3(min_spread_thr_r * 8.f, zspread,
                        ((toomanybits - tbits) * min_spread_thr_r + (tbits - toofewbits) * max_spread_thr_r) / (toomanybits - toofewbits + 1));
                    maxzeroed = FFMIN(zeroable, FFMAX(1, (zeroable * its + maxits - 1) / (2 * maxits)));
                    for (zloop = 0; zloop < 2; zloop++) {
                        /* Two passes: first distorted stuff - two birds in one shot and all that,
                         * then anything viable. Viable means not zero, but either CB=zero-able
                         * (too high SF), not SF <= 1 (that means we'd be operating at very high
                         * quality, we don't want PNS when doing VHQ), PNS allowed, and within
                         * the lowest ranking percentile.
                         */
                        float loopovrfactor = (zloop) ? 1.0f : ovrfactor;
                        int loopminsf = (zloop) ? (SCALE_ONE_POS - SCALE_DIV_512) : SCALE_ONE_POS;
                        int mcb;
                        for (g = sce->ics.num_swb-1; g > 0 && zeroed < maxzeroed; g--) {
                            if (sce->ics.swb_offset[g] < pns_start_pos)
                                continue;
                            for (w = 0; w < sce->ics.num_windows; w += sce->ics.group_len[w]) {
                                if (!sce->zeroes[w*16+g] && sce->can_pns[w*16+g] && spread_thr_r[w*16+g] <= zspread
                                    && sce->sf_idx[w*16+g] > loopminsf
                                    && (dists[w*16+g] > loopovrfactor*uplims[w*16+g] || !(mcb = find_min_book(maxvals[w*16+g], sce->sf_idx[w*16+g]))
                                        || (mcb <= 1 && dists[w*16+g] > FFMIN(uplims[w*16+g], euplims[w*16+g]))) ) {
                                    sce->zeroes[w*16+g] = 1;
                                    sce->band_type[w*16+g] = 0;
                                    zeroed++;
                                }
                            }
                        }
                    }
                    if (zeroed)
                        recomprd = fflag = 1;
                } else {
                    overdist = 0;
                }
            }
        }

        minscaler = SCALE_MAX_POS;
        for (w = 0; w < sce->ics.num_windows; w += sce->ics.group_len[w]) {
            for (g = 0;  g < sce->ics.num_swb; g++) {
                if (!sce->zeroes[w*16+g]) {
                    minscaler = FFMIN(minscaler, sce->sf_idx[w*16+g]);
                }
            }
        }

        minscaler = nminscaler = av_clip(minscaler, SCALE_ONE_POS - SCALE_DIV_512, SCALE_MAX_POS - SCALE_DIV_512);
        prev = -1;
        for (w = 0; w < sce->ics.num_windows; w += sce->ics.group_len[w]) {
            /** Start with big steps, end up fine-tunning */
            int depth = (its > maxits/2) ? ((its > maxits*2/3) ? 1 : 3) : 10;
            int edepth = depth+2;
            float uplmax = its / (maxits*0.25f) + 1.0f;
            uplmax *= (tbits > destbits) ? FFMIN(2.0f, tbits / (float)FFMAX(1,destbits)) : 1.0f;
            start = w * 128;
            for (g = 0; g < sce->ics.num_swb; g++) {
                int prevsc = sce->sf_idx[w*16+g];
                if (prev < 0 && !sce->zeroes[w*16+g])
                    prev = sce->sf_idx[0];
                if (!sce->zeroes[w*16+g]) {
                    const float *coefs = sce->coeffs + start;
                    const float *scaled = s->scoefs + start;
                    int cmb = find_min_book(maxvals[w*16+g], sce->sf_idx[w*16+g]);
                    int mindeltasf = FFMAX(0, prev - SCALE_MAX_DIFF);
                    int maxdeltasf = FFMIN(SCALE_MAX_POS - SCALE_DIV_512, prev + SCALE_MAX_DIFF);
                    if ((!cmb || dists[w*16+g] > uplims[w*16+g]) && sce->sf_idx[w*16+g] > FFMAX(mindeltasf, minsf[w*16+g])) {
                        /* Try to make sure there is some energy in every nonzero band
                         * NOTE: This algorithm must be forcibly imbalanced, pushing harder
                         *  on holes or more distorted bands at first, otherwise there's
                         *  no net gain (since the next iteration will offset all bands
                         *  on the opposite direction to compensate for extra bits)
                         */
                        for (i = 0; i < edepth && sce->sf_idx[w*16+g] > mindeltasf; ++i) {
                            int cb, bits;
                            float dist, qenergy;
                            int mb = find_min_book(maxvals[w*16+g], sce->sf_idx[w*16+g]-1);
                            cb = find_min_book(maxvals[w*16+g], sce->sf_idx[w*16+g]);
                            dist = qenergy = 0.f;
                            bits = 0;
                            if (!cb) {
                                maxsf[w*16+g] = FFMIN(sce->sf_idx[w*16+g]-1, maxsf[w*16+g]);
                            } else if (i >= depth && dists[w*16+g] < euplims[w*16+g]) {
                                break;
                            }
                            /* !g is the DC band, it's important, since quantization error here
                             * applies to less than a cycle, it creates horrible intermodulation
                             * distortion if it doesn't stick to what psy requests
                             */
                            if (!g && sce->ics.num_windows > 1 && dists[w*16+g] >= euplims[w*16+g])
                                maxsf[w*16+g] = FFMIN(sce->sf_idx[w*16+g], maxsf[w*16+g]);
                            for (w2 = 0; w2 < sce->ics.group_len[w]; w2++) {
                                int b;
                                float sqenergy;
                                dist += quantize_band_cost_cached(s, w + w2, g, coefs + w2*128,
                                                        scaled + w2*128,
                                                        sce->ics.swb_sizes[g],
                                                        sce->sf_idx[w*16+g]-1,
                                                        cb,
                                                        1.0f,
                                                        INFINITY,
                                                        &b, &sqenergy,
                                                        0);
                                bits += b;
                                qenergy += sqenergy;
                            }
                            sce->sf_idx[w*16+g]--;
                            dists[w*16+g] = dist - bits;
                            qenergies[w*16+g] = qenergy;
                            if (mb && (sce->sf_idx[w*16+g] < mindeltasf || (
                                    (dists[w*16+g] < FFMIN(uplmax*uplims[w*16+g], euplims[w*16+g]))
                                    && (fabsf(qenergies[w*16+g]-energies[w*16+g]) < euplims[w*16+g])
                                ) )) {
                                break;
                            }
                        }
                    } else if (tbits > toofewbits && sce->sf_idx[w*16+g] < FFMIN(maxdeltasf, maxsf[w*16+g])
                            && (dists[w*16+g] < FFMIN(euplims[w*16+g], uplims[w*16+g]))
                            && (fabsf(qenergies[w*16+g]-energies[w*16+g]) < euplims[w*16+g])
                        ) {
                        /** Um... over target. Save bits for more important stuff. */
                        for (i = 0; i < depth && sce->sf_idx[w*16+g] < maxdeltasf; ++i) {
                            int cb, bits;
                            float dist, qenergy;
                            cb = find_min_book(maxvals[w*16+g], sce->sf_idx[w*16+g]+1);
                            if (cb > 0) {
                                dist = qenergy = 0.f;
                                bits = 0;
                                for (w2 = 0; w2 < sce->ics.group_len[w]; w2++) {
                                    int b;
                                    float sqenergy;
                                    dist += quantize_band_cost_cached(s, w + w2, g, coefs + w2*128,
                                                            scaled + w2*128,
                                                            sce->ics.swb_sizes[g],
                                                            sce->sf_idx[w*16+g]+1,
                                                            cb,
                                                            1.0f,
                                                            INFINITY,
                                                            &b, &sqenergy,
                                                            0);
                                    bits += b;
                                    qenergy += sqenergy;
                                }
                                dist -= bits;
                                if (dist < FFMIN(euplims[w*16+g], uplims[w*16+g])) {
                                    sce->sf_idx[w*16+g]++;
                                    dists[w*16+g] = dist;
                                    qenergies[w*16+g] = qenergy;
                                } else {
                                    break;
                                }
                            } else {
                                maxsf[w*16+g] = FFMIN(sce->sf_idx[w*16+g], maxsf[w*16+g]);
                                break;
                            }
                        }
                    }
                    prev = sce->sf_idx[w*16+g] = av_clip(sce->sf_idx[w*16+g], mindeltasf, maxdeltasf);
                    if (sce->sf_idx[w*16+g] != prevsc)
                        fflag = 1;
                    nminscaler = FFMIN(nminscaler, sce->sf_idx[w*16+g]);
                    sce->band_type[w*16+g] = find_min_book(maxvals[w*16+g], sce->sf_idx[w*16+g]);
                }
                start += sce->ics.swb_sizes[g];
            }
        }

        /** SF difference limit violation risk. Must re-clamp. */
        prev = -1;
        for (w = 0; w < sce->ics.num_windows; w += sce->ics.group_len[w]) {
            for (g = 0; g < sce->ics.num_swb; g++) {
                if (!sce->zeroes[w*16+g]) {
                    int prevsf = sce->sf_idx[w*16+g];
                    if (prev < 0)
                        prev = prevsf;
                    sce->sf_idx[w*16+g] = av_clip(sce->sf_idx[w*16+g], prev - SCALE_MAX_DIFF, prev + SCALE_MAX_DIFF);
                    sce->band_type[w*16+g] = find_min_book(maxvals[w*16+g], sce->sf_idx[w*16+g]);
                    prev = sce->sf_idx[w*16+g];
                    if (!fflag && prevsf != sce->sf_idx[w*16+g])
                        fflag = 1;
                }
            }
        }

        its++;
    } while (fflag && its < maxits);

    /** Scout out next nonzero bands */
    ff_init_nextband_map(sce, nextband);

    prev = -1;
    for (w = 0; w < sce->ics.num_windows; w += sce->ics.group_len[w]) {
        /** Make sure proper codebooks are set */
        for (g = 0; g < sce->ics.num_swb; g++) {
            if (!sce->zeroes[w*16+g]) {
                sce->band_type[w*16+g] = find_min_book(maxvals[w*16+g], sce->sf_idx[w*16+g]);
                if (sce->band_type[w*16+g] <= 0) {
                    if (!ff_sfdelta_can_remove_band(sce, nextband, prev, w*16+g)) {
                        /** Cannot zero out, make sure it's not attempted */
                        sce->band_type[w*16+g] = 1;
                    } else {
                        sce->zeroes[w*16+g] = 1;
                        sce->band_type[w*16+g] = 0;
                    }
                }
            } else {
                sce->band_type[w*16+g] = 0;
            }
            /** Check that there's no SF delta range violations */
            if (!sce->zeroes[w*16+g]) {
                if (prev != -1) {
                    av_unused int sfdiff = sce->sf_idx[w*16+g] - prev + SCALE_DIFF_ZERO;
                    av_assert1(sfdiff >= 0 && sfdiff <= 2*SCALE_MAX_DIFF);
                } else if (sce->zeroes[0]) {
                    /** Set global gain to something useful */
                    sce->sf_idx[0] = sce->sf_idx[w*16+g];
                }
                prev = sce->sf_idx[w*16+g];
            }
        }
    }
}

#endif /* AVCODEC_AACCODER_TWOLOOP_H */
