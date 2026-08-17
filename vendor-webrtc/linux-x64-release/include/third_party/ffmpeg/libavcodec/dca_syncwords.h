/*
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

#ifndef AVCODEC_DCA_SYNCWORDS_H
#define AVCODEC_DCA_SYNCWORDS_H

#define    DCA_SYNCWORD_CORE_BE              0x7FFE8001U
#define    DCA_SYNCWORD_CORE_LE              0xFE7F0180U
#define    DCA_SYNCWORD_CORE_14B_BE          0x1FFFE800U
#define    DCA_SYNCWORD_CORE_14B_LE          0xFF1F00E8U
#define    DCA_SYNCWORD_XCH                  0x5A5A5A5AU
#define    DCA_SYNCWORD_XXCH                 0x47004A03U
#define    DCA_SYNCWORD_X96                  0x1D95F262U
#define    DCA_SYNCWORD_XBR                  0x655E315EU
#define    DCA_SYNCWORD_LBR                  0x0A801921U
#define    DCA_SYNCWORD_XLL                  0x41A29547U
#define    DCA_SYNCWORD_SUBSTREAM            0x64582025U
#define    DCA_SYNCWORD_SUBSTREAM_CORE       0x02B09261U
#define    DCA_SYNCWORD_REV1AUX              0x9A1105A0U

#define    DCA_SYNCWORD_XLL_X                0x02000850U
#define    DCA_SYNCWORD_XLL_X_IMAX           0xF14000D0U

#endif /* AVCODEC_DCA_SYNCWORDS_H */
