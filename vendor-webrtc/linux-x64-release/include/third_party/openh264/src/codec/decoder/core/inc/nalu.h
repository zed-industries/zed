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

//nalu.h:       NAL Unit definition
#ifndef WELS_NAL_UNIT_H__
#define WELS_NAL_UNIT_H__

#include "typedefs.h"
#include "wels_common_basis.h"
#include "nal_prefix.h"
#include "bit_stream.h"

namespace WelsDec {

///////////////////////////////////NAL UNIT level///////////////////////////////////

/* NAL Unit Structure */
typedef struct TagNalUnit {
  SNalUnitHeaderExt       sNalHeaderExt;

  union {
    struct SVclNal {
      SSliceHeaderExt     sSliceHeaderExt;
      SBitStringAux       sSliceBitsRead;
      uint8_t*            pNalPos;         // save the address of slice nal for GPU function
      int32_t             iNalLength;   // save the nal length for GPU function
      bool                bSliceHeaderExtFlag;
    } sVclNal;
    SPrefixNalUnit        sPrefixNal;
  } sNalData;
  unsigned long long uiTimeStamp;
} SNalUnit, *PNalUnit;

///////////////////////////////////ACCESS Unit level///////////////////////////////////

/* Access Unit structure */
typedef struct TagAccessUnits {
  PNalUnit*               pNalUnitsList;  // list of NAL Units pointer in this AU
  uint32_t                uiAvailUnitsNum;   // Number of NAL Units available in each AU list based current bitstream,
  uint32_t                uiActualUnitsNum;       // actual number of NAL units belong to current au
// While available number exceeds count size below, need realloc extra NAL Units for list space.
  uint32_t                uiCountUnitsNum;        // Count size number of malloced NAL Units in each AU list
  uint32_t                uiStartPos;
  uint32_t                uiEndPos;
  bool                    bCompletedAuFlag;       // Indicate whether it is a completed AU
} SAccessUnit, *PAccessUnit;

} // namespace WelsDec

#endif//WELS_NAL_UNIT_H__
