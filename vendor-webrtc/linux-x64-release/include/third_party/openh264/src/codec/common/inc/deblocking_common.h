#ifndef WELS_DEBLOCKING_COMMON_H__
#define WELS_DEBLOCKING_COMMON_H__
#include "typedefs.h"
void DeblockLumaLt4V_c (uint8_t* pPixY, int32_t iStride, int32_t iAlpha, int32_t iBeta, int8_t* pTc);
void DeblockLumaEq4V_c (uint8_t* pPixY, int32_t iStride, int32_t iAlpha, int32_t iBeta);

void DeblockLumaLt4H_c (uint8_t* pPixY, int32_t iStride, int32_t iAlpha, int32_t iBeta, int8_t* pTc);
void DeblockLumaEq4H_c (uint8_t* pPixY, int32_t iStride, int32_t iAlpha, int32_t iBeta);

void DeblockChromaLt4V_c (uint8_t* pPixCb, uint8_t* pPixCr, int32_t iStride, int32_t iAlpha, int32_t iBeta,
                          int8_t* pTc);
void DeblockChromaEq4V_c (uint8_t* pPixCb, uint8_t* pPixCr, int32_t iStride, int32_t iAlpha, int32_t iBeta);

void DeblockChromaLt4H_c (uint8_t* pPixCb, uint8_t* pPixCr, int32_t iStride, int32_t iAlpha, int32_t iBeta,
                          int8_t* pTc);
void DeblockChromaEq4H_c (uint8_t* pPixCb, uint8_t* pPixCr, int32_t iStride, int32_t iAlpha, int32_t iBeta);

void DeblockChromaLt4V2_c (uint8_t* pPixCbCr, int32_t iStride, int32_t iAlpha, int32_t iBeta,
                          int8_t* pTc);
void DeblockChromaEq4V2_c (uint8_t* pPixCbCr, int32_t iStride, int32_t iAlpha, int32_t iBeta);

void DeblockChromaLt4H2_c (uint8_t* pPixCbCr, int32_t iStride, int32_t iAlpha, int32_t iBeta,
                          int8_t* pTc);
void DeblockChromaEq4H2_c (uint8_t* pPixCbCr,int32_t iStride, int32_t iAlpha, int32_t iBeta);

void WelsNonZeroCount_c (int8_t* pNonZeroCount);

#if defined(__cplusplus)
extern "C" {
#endif//__cplusplus

#ifdef  X86_ASM
void DeblockLumaLt4V_ssse3 (uint8_t* pPixY, int32_t iStride, int32_t iAlpha, int32_t iBeta, int8_t* pTc);
void DeblockLumaEq4V_ssse3 (uint8_t* pPixY, int32_t iStride, int32_t iAlpha, int32_t iBeta);
void DeblockLumaTransposeH2V_sse2 (uint8_t* pPixY, int32_t iStride, uint8_t* pDst);
void DeblockLumaTransposeV2H_sse2 (uint8_t* pPixY, int32_t iStride, uint8_t* pSrc);
void DeblockLumaLt4H_ssse3 (uint8_t* pPixY, int32_t iStride, int32_t iAlpha, int32_t iBeta, int8_t* pTc);
void DeblockLumaEq4H_ssse3 (uint8_t* pPixY, int32_t iStride, int32_t iAlpha, int32_t iBeta);
void DeblockChromaEq4V_ssse3 (uint8_t* pPixCb, uint8_t* pPixCr, int32_t iStride, int32_t iAlpha, int32_t iBeta);
void DeblockChromaLt4V_ssse3 (uint8_t* pPixCb, uint8_t* pPixCr, int32_t iStride, int32_t iAlpha, int32_t iBeta,
                              int8_t* pTC);
void DeblockChromaEq4H_ssse3 (uint8_t* pPixCb, uint8_t* pPixCr, int32_t iStride, int32_t iAlpha, int32_t iBeta);
void DeblockChromaLt4H_ssse3 (uint8_t* pPixCb, uint8_t* pPixCr, int32_t iStride, int32_t iAlpha, int32_t iBeta,
                              int8_t* pTC);
void WelsNonZeroCount_sse2 (int8_t* pNonZeroCount);
#endif

#if defined(HAVE_NEON)
void DeblockLumaLt4V_neon (uint8_t* pPixY, int32_t iStride, int32_t iAlpha, int32_t iBeta, int8_t* pTc);
void DeblockLumaEq4V_neon (uint8_t* pPixY, int32_t iStride, int32_t iAlpha, int32_t iBeta);

void DeblockLumaLt4H_neon (uint8_t* pPixY, int32_t iStride, int32_t iAlpha, int32_t iBeta, int8_t* pTc);
void DeblockLumaEq4H_neon (uint8_t* pPixY, int32_t iStride, int32_t iAlpha, int32_t iBeta);

void DeblockChromaLt4V_neon (uint8_t* pPixCb, uint8_t* pPixCr, int32_t iStride, int32_t iAlpha, int32_t iBeta,
                             int8_t* pTC);
void DeblockChromaEq4V_neon (uint8_t* pPixCb, uint8_t* pPixCr, int32_t iStride, int32_t iAlpha, int32_t iBeta);

void DeblockChromaLt4H_neon (uint8_t* pPixCb, uint8_t* pPixCr, int32_t iStride, int32_t iAlpha, int32_t iBeta,
                             int8_t* pTC);
void DeblockChromaEq4H_neon (uint8_t* pPixCb, uint8_t* pPixCr, int32_t iStride, int32_t iAlpha, int32_t iBeta);
void WelsNonZeroCount_neon (int8_t* pNonZeroCount);
#endif

#if defined(HAVE_NEON_AARCH64)
void DeblockLumaLt4V_AArch64_neon (uint8_t* pPixY, int32_t iStride, int32_t iAlpha, int32_t iBeta, int8_t* pTc);
void DeblockLumaEq4V_AArch64_neon (uint8_t* pPixY, int32_t iStride, int32_t iAlpha, int32_t iBeta);
void DeblockLumaLt4H_AArch64_neon (uint8_t* pPixY, int32_t iStride, int32_t iAlpha, int32_t iBeta, int8_t* pTc);
void DeblockLumaEq4H_AArch64_neon (uint8_t* pPixY, int32_t iStride, int32_t iAlpha, int32_t iBeta);
void DeblockChromaLt4V_AArch64_neon (uint8_t* pPixCb, uint8_t* pPixCr, int32_t iStride, int32_t iAlpha, int32_t iBeta,
                                     int8_t* pTC);
void DeblockChromaEq4V_AArch64_neon (uint8_t* pPixCb, uint8_t* pPixCr, int32_t iStride, int32_t iAlpha, int32_t iBeta);
void DeblockChromaLt4H_AArch64_neon (uint8_t* pPixCb, uint8_t* pPixCr, int32_t iStride, int32_t iAlpha, int32_t iBeta,
                                     int8_t* pTC);
void DeblockChromaEq4H_AArch64_neon (uint8_t* pPixCb, uint8_t* pPixCr, int32_t iStride, int32_t iAlpha, int32_t iBeta);
void WelsNonZeroCount_AArch64_neon (int8_t* pNonZeroCount);
#endif

#if defined(HAVE_MMI)
void DeblockLumaLt4V_mmi (uint8_t* pPixY, int32_t iStride, int32_t iAlpha, int32_t iBeta, int8_t* pTc);
void DeblockLumaEq4V_mmi (uint8_t* pPixY, int32_t iStride, int32_t iAlpha, int32_t iBeta);
void DeblockLumaTransposeH2V_mmi (uint8_t* pPixY, int32_t iStride, uint8_t* pDst);
void DeblockLumaTransposeV2H_mmi (uint8_t* pPixY, int32_t iStride, uint8_t* pSrc);
void DeblockLumaLt4H_mmi (uint8_t* pPixY, int32_t iStride, int32_t iAlpha, int32_t iBeta, int8_t* pTc);
void DeblockLumaEq4H_mmi (uint8_t* pPixY, int32_t iStride, int32_t iAlpha, int32_t iBeta);
void DeblockChromaEq4V_mmi (uint8_t* pPixCb, uint8_t* pPixCr, int32_t iStride, int32_t iAlpha, int32_t iBeta);
void DeblockChromaLt4V_mmi (uint8_t* pPixCb, uint8_t* pPixCr, int32_t iStride, int32_t iAlpha, int32_t iBeta,
                            int8_t* pTC);
void DeblockChromaEq4H_mmi (uint8_t* pPixCb, uint8_t* pPixCr, int32_t iStride, int32_t iAlpha, int32_t iBeta);
void DeblockChromaLt4H_mmi (uint8_t* pPixCb, uint8_t* pPixCr, int32_t iStride, int32_t iAlpha, int32_t iBeta,
                            int8_t* pTC);
void WelsNonZeroCount_mmi (int8_t* pNonZeroCount);
#endif//HAVE_MMI

#if defined(HAVE_MSA)
void DeblockLumaLt4V_msa (uint8_t* pPixY, int32_t iStride, int32_t iAlpha, int32_t iBeta, int8_t* pTc);
void DeblockLumaEq4V_msa (uint8_t* pPixY, int32_t iStride, int32_t iAlpha, int32_t iBeta);
void DeblockLumaLt4H_msa (uint8_t* pPixY, int32_t iStride, int32_t iAlpha, int32_t iBeta, int8_t* pTc);
void DeblockLumaEq4H_msa (uint8_t* pPixY, int32_t iStride, int32_t iAlpha, int32_t iBeta);
void DeblockChromaEq4V_msa (uint8_t* pPixCb, uint8_t* pPixCr, int32_t iStride, int32_t iAlpha, int32_t iBeta);
void DeblockChromaLt4V_msa (uint8_t* pPixCb, uint8_t* pPixCr, int32_t iStride, int32_t iAlpha, int32_t iBeta,
                            int8_t* pTC);
void DeblockChromaEq4H_msa (uint8_t* pPixCb, uint8_t* pPixCr, int32_t iStride, int32_t iAlpha, int32_t iBeta);
void DeblockChromaLt4H_msa (uint8_t* pPixCb, uint8_t* pPixCr, int32_t iStride, int32_t iAlpha, int32_t iBeta,
                            int8_t* pTC);
void WelsNonZeroCount_msa (int8_t* pNonZeroCount);
#endif//HAVE_MSA

#if defined(HAVE_LSX)
void DeblockLumaLt4V_lsx (uint8_t* pPixY, int32_t iStride, int32_t iAlpha, int32_t iBeta, int8_t* pTc);
void DeblockLumaLt4H_lsx (uint8_t* pPixY, int32_t iStride, int32_t iAlpha, int32_t iBeta, int8_t* pTc);
void DeblockLumaEq4V_lsx (uint8_t* pPixY, int32_t iStride, int32_t iAlpha, int32_t iBeta);
void DeblockLumaEq4H_lsx (uint8_t* pPixY, int32_t iStride, int32_t iAlpha, int32_t iBeta);
void DeblockChromaEq4H_lsx (uint8_t* pPixCb, uint8_t* pPixCr, int32_t iStride, int32_t iAlpha, int32_t iBeta);
void DeblockChromaLt4V_lsx (uint8_t* pPixCb, uint8_t* pPixCr, int32_t iStride, int32_t iAlpha, int32_t iBeta,
                            int8_t* pTC);
void DeblockChromaLt4H_lsx (uint8_t* pPixCb, uint8_t* pPixCr, int32_t iStride, int32_t iAlpha, int32_t iBeta,
                            int8_t* pTC);
#endif//HAVE_LSX
#if defined(__cplusplus)
}
#endif//__cplusplus

#endif //WELS_DEBLOCKING_COMMON_H__
