/*
***********************************************************************
* © 2016 and later: Unicode, Inc. and others.
* License & terms of use: http://www.unicode.org/copyright.html
***********************************************************************
***********************************************************************
* Copyright (c) 2002-2011, International Business Machines
* Corporation and others.  All Rights Reserved.
***********************************************************************
***********************************************************************
*/
#ifndef _NORMPERF_H
#define _NORMPERF_H

#include "unicode/unorm.h"
#include "unicode/ustring.h"

#include "unicode/uperf.h"
#include <stdlib.h>

//  Stubs for Windows API functions when building on UNIXes.
//
#if U_PLATFORM_USES_ONLY_WIN32_API
// do nothing
#else
#define _UNICODE
typedef int DWORD;
inline int FoldStringW(DWORD dwMapFlags, const char16_t* lpSrcStr,int cchSrc, char16_t* lpDestStr,int cchDest);
#endif

#define DEST_BUFFER_CAPACITY 6000
typedef int32_t (*NormFn)(const char16_t* src,int32_t srcLen, char16_t* dest,int32_t dstLen, int32_t options, UErrorCode* status);
typedef int32_t (*QuickCheckFn)(const char16_t* src,int32_t srcLen, UNormalizationMode mode, int32_t options, UErrorCode* status);

class QuickCheckPerfFunction : public UPerfFunction{
private:
    ULine* lines;
    int32_t numLines;
    QuickCheckFn fn;
    UNormalizationMode mode;
    int32_t retVal;
    UBool uselen;
    const char16_t* src;
    int32_t srcLen;
    UBool line_mode;
    int32_t options;

public:
    virtual void call(UErrorCode* status){
        if(line_mode){
            if(uselen){
                for(int32_t i = 0; i< numLines; i++){
                    retVal =  (*fn)(lines[i].name,lines[i].len,mode, options, status);
                }
            }else{
                for(int32_t i = 0; i< numLines; i++){
                    retVal =  (*fn)(lines[i].name,-1,mode, options, status);
                }
            }
        }else{
            if(uselen){

                retVal =  (*fn)(src,srcLen,mode, options, status);
            }else{
                retVal =  (*fn)(src,-1,mode, options, status);
            }
        }

    }
    virtual long getOperationsPerIteration(){
        if(line_mode){
            int32_t totalChars=0;
            for(int32_t i =0; i< numLines; i++){
                totalChars+= lines[i].len;
            }
            return totalChars;
        }else{
            return srcLen;
        }
    }
    QuickCheckPerfFunction(QuickCheckFn func, ULine* srcLines,int32_t srcNumLines, UNormalizationMode _mode, int32_t opts, UBool _uselen) : options(opts) {
        fn = func;
        lines = srcLines;
        numLines = srcNumLines;
        uselen = _uselen;
        mode = _mode;
        src = nullptr;
        srcLen = 0;
        line_mode = true;
    }
    QuickCheckPerfFunction(QuickCheckFn func, const char16_t* source,int32_t sourceLen, UNormalizationMode _mode, int32_t opts, UBool _uselen) : options(opts) {
        fn = func;
        lines = nullptr;
        numLines = 0;
        uselen = _uselen;
        mode = _mode;
        src = source;
        srcLen = sourceLen;
        line_mode = false;
    }
};


class NormPerfFunction : public UPerfFunction{
private:
    ULine* lines;
    int32_t numLines;
    char16_t dest[DEST_BUFFER_CAPACITY];
    char16_t* pDest;
    int32_t destLen;
    NormFn fn;
    int32_t retVal;
    UBool uselen;
    const char16_t* src;
    int32_t srcLen;
    UBool line_mode;
    int32_t options;

public:
    virtual void call(UErrorCode* status){
        if(line_mode){
            if(uselen){
                for(int32_t i = 0; i< numLines; i++){
                    retVal =  (*fn)(lines[i].name,lines[i].len,pDest,destLen, options, status);
                }
            }else{
                for(int32_t i = 0; i< numLines; i++){
                    retVal =  (*fn)(lines[i].name,-1,pDest,destLen, options, status);
                }
            }
        }else{
            if(uselen){
                retVal =  (*fn)(src,srcLen,pDest,destLen, options, status);
            }else{
                retVal =  (*fn)(src,-1,pDest,destLen, options, status);
            }
        }
    }
    virtual long getOperationsPerIteration(){
        if(line_mode){
            int32_t totalChars=0;
            for(int32_t i =0; i< numLines; i++){
                totalChars+= lines[i].len;
            }
            return totalChars;
        }else{
            return srcLen;
        }
    }
    NormPerfFunction(NormFn func, int32_t opts, ULine* srcLines,int32_t srcNumLines,UBool _uselen) : options(opts) {
        fn = func;
        lines = srcLines;
        numLines = srcNumLines;
        uselen = _uselen;
        destLen = DEST_BUFFER_CAPACITY;
        pDest = dest;
        src = nullptr;
        srcLen = 0;
        line_mode = true;
    }
    NormPerfFunction(NormFn func, int32_t opts, const char16_t* source,int32_t sourceLen,UBool _uselen) : options(opts) {
        fn = func;
        lines = nullptr;
        numLines = 0;
        uselen = _uselen;
        destLen = sourceLen*3;
        pDest = (char16_t*) malloc(destLen * U_SIZEOF_UCHAR);
        src = source;
        srcLen = sourceLen;
        line_mode = false;
    }
    ~NormPerfFunction(){
        if(dest != pDest){
            free(pDest);
        }
    }
};



class  NormalizerPerformanceTest : public UPerfTest{
private:
    ULine* NFDFileLines;
    ULine* NFCFileLines;
    char16_t* NFDBuffer;
    char16_t* NFCBuffer;
    char16_t* origBuffer;
    int32_t origBufferLen;
    int32_t NFDBufferLen;
    int32_t NFCBufferLen;
    int32_t options;

    void normalizeInput(ULine* dest,const char16_t* src ,int32_t srcLen,UNormalizationMode mode, int32_t options);
    char16_t* normalizeInput(int32_t& len, const char16_t* src ,int32_t srcLen,UNormalizationMode mode, int32_t options);

public:

    NormalizerPerformanceTest(int32_t argc, const char* argv[], UErrorCode& status);
    ~NormalizerPerformanceTest();
    virtual UPerfFunction* runIndexedTest(int32_t index, UBool exec,const char* &name, char* par = nullptr);
    /* NFC performance */
    UPerfFunction* TestICU_NFC_NFD_Text();
    UPerfFunction* TestICU_NFC_NFC_Text();
    UPerfFunction* TestICU_NFC_Orig_Text();
    
    /* NFD performance */
    UPerfFunction* TestICU_NFD_NFD_Text();
    UPerfFunction* TestICU_NFD_NFC_Text();
    UPerfFunction* TestICU_NFD_Orig_Text();

    /* FCD performance */
    UPerfFunction* TestICU_FCD_NFD_Text();
    UPerfFunction* TestICU_FCD_NFC_Text();
    UPerfFunction* TestICU_FCD_Orig_Text();
    
    /*Win NFC performance */
    UPerfFunction* TestWin_NFC_NFD_Text();
    UPerfFunction* TestWin_NFC_NFC_Text();
    UPerfFunction* TestWin_NFC_Orig_Text();
    
    /* Win NFD performance */
    UPerfFunction* TestWin_NFD_NFD_Text();
    UPerfFunction* TestWin_NFD_NFC_Text();
    UPerfFunction* TestWin_NFD_Orig_Text();
    
    /* Quick check performance */
    UPerfFunction* TestQC_NFC_NFD_Text();
    UPerfFunction* TestQC_NFC_NFC_Text();
    UPerfFunction* TestQC_NFC_Orig_Text();

    UPerfFunction* TestQC_NFD_NFD_Text();
    UPerfFunction* TestQC_NFD_NFC_Text();
    UPerfFunction* TestQC_NFD_Orig_Text();

    UPerfFunction* TestQC_FCD_NFD_Text();
    UPerfFunction* TestQC_FCD_NFC_Text();
    UPerfFunction* TestQC_FCD_Orig_Text();

    /* IsNormalized performance */
    UPerfFunction* TestIsNormalized_NFC_NFD_Text();
    UPerfFunction* TestIsNormalized_NFC_NFC_Text();
    UPerfFunction* TestIsNormalized_NFC_Orig_Text();

    UPerfFunction* TestIsNormalized_NFD_NFD_Text();
    UPerfFunction* TestIsNormalized_NFD_NFC_Text();
    UPerfFunction* TestIsNormalized_NFD_Orig_Text();

    UPerfFunction* TestIsNormalized_FCD_NFD_Text();
    UPerfFunction* TestIsNormalized_FCD_NFC_Text();
    UPerfFunction* TestIsNormalized_FCD_Orig_Text();

};

//---------------------------------------------------------------------------------------
// Platform / ICU version specific proto-types
//---------------------------------------------------------------------------------------


#if (U_ICU_VERSION_MAJOR_NUM > 1 ) || ((U_ICU_VERSION_MAJOR_NUM == 1 )&&(U_ICU_VERSION_MINOR_NUM > 8) && (U_ICU_VERSION_PATCHLEVEL_NUM >=1))

int32_t ICUNormNFD(const char16_t* src, int32_t srcLen,char16_t* dest, int32_t dstLen, int32_t options, UErrorCode* status) {
    return unorm_normalize(src,srcLen,UNORM_NFD, options,dest,dstLen,status);
}

int32_t ICUNormNFC(const char16_t* src, int32_t srcLen,char16_t* dest, int32_t dstLen, int32_t options, UErrorCode* status) {
    return unorm_normalize(src,srcLen,UNORM_NFC, options,dest,dstLen,status);
}

int32_t ICUNormNFKD(const char16_t* src, int32_t srcLen,char16_t* dest, int32_t dstLen, int32_t options, UErrorCode* status) {
    return unorm_normalize(src,srcLen,UNORM_NFKD, options,dest,dstLen,status);
}
int32_t ICUNormNFKC(const char16_t* src, int32_t srcLen,char16_t* dest, int32_t dstLen, int32_t options, UErrorCode* status) {
    return unorm_normalize(src,srcLen,UNORM_NFKC, options,dest,dstLen,status);
}

int32_t ICUNormFCD(const char16_t* src, int32_t srcLen,char16_t* dest, int32_t dstLen, int32_t options, UErrorCode* status) {
    return unorm_normalize(src,srcLen,UNORM_FCD, options,dest,dstLen,status);
}

int32_t ICUQuickCheck(const char16_t* src,int32_t srcLen, UNormalizationMode mode, int32_t options, UErrorCode* status){
#if (U_ICU_VERSION_MAJOR_NUM > 2 ) || ((U_ICU_VERSION_MAJOR_NUM == 2 )&&(U_ICU_VERSION_MINOR_NUM >= 6))
    return unorm_quickCheckWithOptions(src,srcLen,mode, options, status);
#else
    return unorm_quickCheck(src,srcLen,mode,status);
#endif
}
int32_t ICUIsNormalized(const char16_t* src,int32_t srcLen, UNormalizationMode mode, int32_t options, UErrorCode* status){
    return unorm_isNormalized(src,srcLen,mode,status);
}


#else

int32_t ICUNormNFD(const char16_t* src, int32_t srcLen,char16_t* dest, int32_t dstLen, int32_t options, UErrorCode* status) {
    return unorm_normalize(src,srcLen,UCOL_DECOMP_CAN, options,dest,dstLen,status);
}

int32_t ICUNormNFC(const char16_t* src, int32_t srcLen,char16_t* dest, int32_t dstLen, int32_t options, UErrorCode* status) {
    return unorm_normalize(src,srcLen,UCOL_COMPOSE_CAN, options,dest,dstLen,status);
}

int32_t ICUNormNFKD(const char16_t* src, int32_t srcLen,char16_t* dest, int32_t dstLen, int32_t options, UErrorCode* status) {
    return unorm_normalize(src,srcLen,UCOL_DECOMP_COMPAT, options,dest,dstLen,status);
}
int32_t ICUNormNFKC(const char16_t* src, int32_t srcLen,char16_t* dest, int32_t dstLen, int32_t options, UErrorCode* status) {
    return unorm_normalize(src,srcLen,UCOL_COMPOSE_COMPAT, options,dest,dstLen,status);
}

int32_t ICUNormFCD(const char16_t* src, int32_t srcLen,char16_t* dest, int32_t dstLen, int32_t options, UErrorCode* status) {
    return unorm_normalize(src,srcLen,UNORM_FCD, options,dest,dstLen,status);
}

int32_t ICUQuickCheck(const char16_t* src,int32_t srcLen, UNormalizationMode mode, int32_t options, UErrorCode* status){
    return unorm_quickCheck(src,srcLen,mode,status);
}

int32_t ICUIsNormalized(const char16_t* src,int32_t srcLen, UNormalizationMode mode, int32_t options, UErrorCode* status){
    return 0;
}
#endif

#if U_PLATFORM_HAS_WIN32_API

int32_t WinNormNFD(const char16_t* src, int32_t srcLen, char16_t* dest, int32_t dstLen, int32_t options, UErrorCode* status) {
    return FoldStringW(MAP_COMPOSITE, toOldUCharPtr(src),srcLen, toOldUCharPtr(dest),dstLen);
}

int32_t WinNormNFC(const char16_t* src, int32_t srcLen, char16_t* dest, int32_t dstLen, int32_t options, UErrorCode* status) {
    return FoldStringW(MAP_PRECOMPOSED, toOldUCharPtr(src),srcLen, toOldUCharPtr(dest),dstLen);
}

int32_t WinNormNFKD(const char16_t* src, int32_t srcLen, char16_t* dest, int32_t dstLen, int32_t options, UErrorCode* status) {
    return FoldStringW(MAP_COMPOSITE+MAP_FOLDCZONE, toOldUCharPtr(src),srcLen, toOldUCharPtr(dest),dstLen);
}
int32_t WinNormNFKC(const char16_t* src, int32_t srcLen, char16_t* dest, int32_t dstLen, int32_t options, UErrorCode* status) {
    return FoldStringW(MAP_FOLDCZONE, toOldUCharPtr(src),srcLen, toOldUCharPtr(dest),dstLen);
}
#else
int32_t WinNormNFD(const char16_t* src, int32_t srcLen, char16_t* dest, int32_t dstLen, int32_t options, UErrorCode* status) {
    return 0 ;
}

int32_t WinNormNFC(const char16_t* src, int32_t srcLen, char16_t* dest, int32_t dstLen, int32_t options, UErrorCode* status) {
    return 0;
}

int32_t WinNormNFKD(const char16_t* src, int32_t srcLen, char16_t* dest, int32_t dstLen, int32_t options, UErrorCode* status) {
    return 0;
}
int32_t WinNormNFKC(const char16_t* src, int32_t srcLen, char16_t* dest, int32_t dstLen, int32_t options, UErrorCode* status) {
    return 0;
}
#endif


#endif // NORMPERF_H

