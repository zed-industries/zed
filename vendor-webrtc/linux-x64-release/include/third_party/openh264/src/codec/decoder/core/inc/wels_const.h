/*!
 * \copy
 *     Copyright (c)  2013, Cisco Systems
 *     All rights reserved.
 *
 *     Redistribution and use in source and binary forms, with or without
 *     modification, are permitted provided that the following conditions
 *     are met:
 *
 *        * Redistributions of source code must retain the above copyright
 *          notice, this list of conditions and the following disclaimer.
 *
 *        * Redistributions in binary form must reproduce the above copyright
 *          notice, this list of conditions and the following disclaimer in
 *          the documentation and/or other materials provided with the
 *          distribution.
 *
 *     THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 *     "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 *     LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS
 *     FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE
 *     COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT,
 *     INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING,
 *     BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
 *     LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
 *     CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
 *     LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN
 *     ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
 *     POSSIBILITY OF SUCH DAMAGE.
 *
 */

//wels_const.h
#ifndef WELS_CONST_H__
#define WELS_CONST_H__

#include "wels_const_common.h"

/* Some list size */

#define MB_SUB_PARTITION_SIZE           4       // Sub partition size in a 8x8 sub-block
#define NAL_UNIT_HEADER_EXT_SIZE        3       // Size of NAL unit header for extension in byte
#define MAX_PPS_COUNT                   256     // Count number of PPS

#define MAX_REF_PIC_COUNT               16              // MAX Short + Long reference pictures
#define MIN_REF_PIC_COUNT               1               // minimal count number of reference pictures, 1 short + 2 key reference based?
#define MAX_SHORT_REF_COUNT             16              // maximal count number of short reference pictures
#define MAX_LONG_REF_COUNT              16              // maximal count number of long reference pictures
#define MAX_DPB_COUNT                   (MAX_REF_PIC_COUNT + 1) // 1 additional position for re-order and other process

#define MAX_MMCO_COUNT                  66

#define MAX_SLICEGROUP_IDS              8       // Count number of Slice Groups

#define MAX_LAYER_NUM                   8

#define LAYER_NUM_EXCHANGEABLE          1

#define MAX_NAL_UNIT_NUM_IN_AU          32      // predefined maximal number of NAL Units in an access unit
#define MIN_ACCESS_UNIT_CAPACITY        1048576 // Min AU capacity in bytes: (1<<20) = 1024 KB predefined
#define MAX_BUFFERED_NUM 3 //mamixum stored number of AU|packet to prevent overwrite
#define MAX_ACCESS_UNIT_CAPACITY 7077888 //Maximum AU size in bytes for level 5.2 for single frame
#define MAX_MACROBLOCK_CAPACITY 5000 //Maximal legal MB capacity, 15000 bits is enough

#endif//WELS_CONST_H__
