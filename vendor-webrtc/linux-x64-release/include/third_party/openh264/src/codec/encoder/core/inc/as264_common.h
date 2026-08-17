/*!
 * \copy
 *     Copyright (c)  2009-2013, Cisco Systems
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
 *
 * \file    common.h
 *
 * \brief   common flag definitions
 *
 * \date    7/6/2009 Created
 *
 *************************************************************************************
 */

#ifndef AS264_COMMON_H_
#define AS264_COMMON_H_

/* debug setting for console
$(TargetPath)
-iper 60 -lqp 26 -frin 2 -rc 1 -cf 4 -org desktop.bgra -sw 800 -sh 592 -bf desktop.h264
.\..\..\..\..\bin
*/

/****************************************************************************
 * Options for algorithm, usually change bitrate
 ****************************************************************************/
#define DISABLE_FMO_FEATURE             //

/****************************************************************************
 * Options for optimization, not change bitrate
 ****************************************************************************/
//#undef        X86_ASM             // X86_ASM is included in project preprocessor definitions, undef it when need to disable asm code
#define SINGLE_REF_FRAME            // need to disable it when use multi-reference


#if defined(WELS_TESTBED)               // for SGE testing
#define ENABLE_FRAME_DUMP

#ifdef FRAME_INFO_OUTPUT
#undef FRAME_INFO_OUTPUT
#endif//FRAME_INFO_OUTPUT
#endif//WELS_TESTBED


#if defined(__UNITTEST__)               // for unittest
#ifndef ENABLE_FRAME_DUMP
#define ENABLE_FRAME_DUMP
#endif//ENABLE_FRAME_DUMP
#endif//__UNITTEST__

//#define STAT_OUTPUT
//#define MB_TYPES_CHECK
//
//#define FRAME_INFO_OUTPUT
//#define LAYER_INFO_OUTPUT
//#define SLICE_INFO_OUTPUT             // useful in multiple slice coding track
//#define MB_TYPES_INFO_OUTPUT


/* macros dependencies check */
//@if !FRAME_INFO_OUTPUT
#if !defined(FRAME_INFO_OUTPUT)

//#if defined(STAT_OUTPUT)
//#undef STAT_OUTPUT
//#endif//STAT_OUTPUT

#if defined(LAYER_INFO_OUTPUT)
#undef LAYER_INFO_OUTPUT
#endif//LAYER_INFO_OUTPUT

#if defined(SLICE_INFO_OUTPUT)
#undef SLICE_INFO_OUTPUT
#endif//SLICE_INFO_OUTPUT

#if defined(MB_TYPES_INFO_OUTPUT)
#undef MB_TYPES_INFO_OUTPUT
#endif//MB_TYPES_INFO_OUTPUT

#endif//FRAME_INFO_OUTPUT

//@if SLICE_INFO_OUTPUT
#if defined(SLICE_INFO_OUTPUT)

#if !defined(FRAME_INFO_OUTPUT)
#define FRAME_INFO_OUTPUT
#endif//FRAME_INFO_OUTPUT

#if !defined(LAYER_INFO_OUTPUT)
#define LAYER_INFO_OUTPUT
#endif//LAYER_INFO_OUTPUT

#endif//SLICE_INFO_OUTPUT

#if defined(LAYER_INFO_OUTPUT)

#if !defined(FRAME_INFO_OUTPUT)
#define FRAME_INFO_OUTPUT
#endif//!FRAME_INFO_OUTPUT

#endif//LAYER_INFO_OUTPUT

//@if MB_TYPES_INFO_OUTPUT
#if defined(MB_TYPES_INFO_OUTPUT)

#if !defined(MB_TYPES_CHECK)
#define MB_TYPES_CHECK
#endif//MB_TYPES_CHECK
#endif//MB_TYPES_INFO_OUTPUT

#endif // AS264_COMMON_H_

