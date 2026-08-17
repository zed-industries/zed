/*
***********************************************************************
* © 2016 and later: Unicode, Inc. and others.
* License & terms of use: http://www.unicode.org/copyright.html
***********************************************************************
***********************************************************************
* Copyright (c) 2002-2014, International Business Machines
* Corporation and others.  All Rights Reserved.
***********************************************************************
***********************************************************************
*/
#ifndef _CONVPERF_H
#define _CONVPERF_H

#include <mlang.h>
#include <objbase.h>
#include <stdlib.h>
#include "unicode/ucnv.h"
#include "unicode/uclean.h"
#include "unicode/ustring.h"
#include "cmemory.h" // for UPRV_LENGTHOF

#include "unicode/uperf.h"

#define CONVERSION_FLAGS (0) /*WC_DEFAULTCHAR WC_COMPOSITECHECK & WC_SEPCHARS*/
#define MAX_BUF_SIZE  3048

class ICUToUnicodePerfFunction : public UPerfFunction{
private:
    UConverter* conv;
    const char* src;
    int32_t srcLen;
    char16_t* target;
    char16_t* targetLimit;
    
public:
    ICUToUnicodePerfFunction(const char* name,  const char* source, int32_t sourceLen, UErrorCode& status){
        conv = ucnv_open(name,&status);
        src = source;
        srcLen = sourceLen;
        if(U_FAILURE(status)){
            conv = nullptr;
            return;
        }
        target = nullptr;
        targetLimit = nullptr;
        int32_t reqdLen = ucnv_toUChars(conv,   target, 0,
                                        source, srcLen, &status);
        if(status==U_BUFFER_OVERFLOW_ERROR) {
            status=U_ZERO_ERROR;
            target=(char16_t*)malloc((reqdLen) * U_SIZEOF_UCHAR*2);
            targetLimit = target + reqdLen;
            if(target == nullptr){
                status = U_MEMORY_ALLOCATION_ERROR;
                return;
            }
        }
    }
    virtual void call(UErrorCode* status){
        const char* mySrc = src;
        const char* sourceLimit = src + srcLen;
        char16_t* myTarget = target;
        ucnv_toUnicode(conv, &myTarget, targetLimit, &mySrc, sourceLimit, nullptr, true, status);
    }
    virtual long getOperationsPerIteration(){
        return srcLen;
    }
    ~ICUToUnicodePerfFunction(){
        free(target);
        ucnv_close(conv);
    }
};
class ICUFromUnicodePerfFunction : public UPerfFunction{
private:
    UConverter* conv;
    const char16_t* src;
    int32_t srcLen;
    char* target;
    char* targetLimit;
    const char* name;
    
public:
    ICUFromUnicodePerfFunction(const char* name,  const char16_t* source, int32_t sourceLen, UErrorCode& status){
        conv = ucnv_open(name,&status);
        src = source;
        srcLen = sourceLen;
        if(U_FAILURE(status)){
            conv = nullptr;
            return;
        }
        target = nullptr;
        targetLimit = nullptr;
        int32_t reqdLen = ucnv_fromUChars(conv,   target, 0,
                                          source, srcLen, &status);
        if(status==U_BUFFER_OVERFLOW_ERROR) {
            status=U_ZERO_ERROR;
            target=(char*)malloc((reqdLen*2));
            targetLimit = target + reqdLen;
            if(target == nullptr){
                status = U_MEMORY_ALLOCATION_ERROR;
                return;
            }
        }
    }
    virtual void call(UErrorCode* status){
        const char16_t* mySrc = src;
        const char16_t* sourceLimit = src + srcLen;
        char* myTarget = target;
        ucnv_fromUnicode(conv,&myTarget, targetLimit, &mySrc, sourceLimit, nullptr, true, status);
    }
    virtual long getOperationsPerIteration(){
        return srcLen;
    }
    ~ICUFromUnicodePerfFunction(){
        free(target);
        ucnv_close(conv);
    }
};

class ICUOpenAllConvertersFunction : public UPerfFunction{
private:
    UBool cleanup;
    int32_t availableConverters;
    const char **convNames;
public:
    ICUOpenAllConvertersFunction(UBool callCleanup, UErrorCode& status){
        int32_t idx;
        cleanup = callCleanup;
        availableConverters = ucnv_countAvailable();
        convNames = new const char *[availableConverters];
        for (idx = 0; idx < availableConverters; idx++) {
            convNames[idx] = ucnv_getAvailableName(idx);
        }
    }
    virtual void call(UErrorCode* status){
        int32_t idx;
        if (cleanup) {
            u_cleanup();
        }
        for (idx = 0; idx < availableConverters; idx++) {
            ucnv_close(ucnv_open(convNames[idx], status));
        }
    }
    virtual long getOperationsPerIteration(){
        return availableConverters;
    }
    ~ICUOpenAllConvertersFunction(){
        delete []convNames;
    }
};

class WinANSIToUnicodePerfFunction : public UPerfFunction{

private:
    DWORD uiCodePage;
    char* src; 
    UINT  srcLen;
    WCHAR dest[MAX_BUF_SIZE];
    UINT  dstLen;
    const char* name;
public:
    WinANSIToUnicodePerfFunction(const char* cpName, char* pszIn,UINT szLen, UErrorCode& status){
        name = cpName;
        src = pszIn;
        srcLen = szLen;
        dstLen = UPRV_LENGTHOF(dest);
        unsigned short bEnc[30]={'\0'};
        const char* tenc=name;
        for(int i=0;*tenc!='\0';i++){
            bEnc[i]=*tenc;
            tenc++;
        }
        LPMULTILANGUAGE2 pMulti;
        
        CoInitialize(nullptr);

        /* create instance of converter object*/
        CoCreateInstance(
                          __uuidof(CMultiLanguage),
                          nullptr,
                          CLSCTX_SERVER,
                          __uuidof(IMultiLanguage2),
                          (void**)&pMulti
                          );



        MIMECSETINFO mimeInfo;
        
        mimeInfo.uiCodePage = 0;
        mimeInfo.uiInternetEncoding =0;
        /* get the charset info */
        pMulti->GetCharsetInfo((wchar_t *)bEnc,&mimeInfo);
        uiCodePage = (mimeInfo.uiInternetEncoding==0)?mimeInfo.uiCodePage:mimeInfo.uiInternetEncoding;
    }
    virtual void call(UErrorCode* status){
        int winSize =MultiByteToWideChar(uiCodePage,CONVERSION_FLAGS,src,srcLen,dest,dstLen);
    }
    virtual long getOperationsPerIteration(){
        return srcLen;
    }
};

class WinANSIFromUnicodePerfFunction : public UPerfFunction{

private:
    DWORD uiCodePage;
    WCHAR* src; 
    UINT  srcLen;
    char dest[MAX_BUF_SIZE];
    UINT  dstLen;
    const char* name;
    BOOL lpUsedDefaultChar;

public:
    WinANSIFromUnicodePerfFunction(const char* cpName,  WCHAR* pszIn,UINT szLen, UErrorCode& status){
        name = cpName;
        src = pszIn;
        srcLen = szLen;
        dstLen = UPRV_LENGTHOF(dest);
        lpUsedDefaultChar=false;
        unsigned short bEnc[30]={'\0'};
        const char* tenc=name;
        for(int i=0;*tenc!='\0';i++){
            bEnc[i]=*tenc;
            tenc++;
        }
        LPMULTILANGUAGE2 pMulti;
        
        CoInitialize(nullptr);

        /* create instance of converter object*/
        CoCreateInstance(
                          __uuidof(CMultiLanguage),
                          nullptr,
                          CLSCTX_SERVER,
                          __uuidof(IMultiLanguage2),
                          (void**)&pMulti
                          );



        MIMECSETINFO mimeInfo;
        mimeInfo.uiCodePage = 0;
        mimeInfo.uiInternetEncoding =0;
        /* get the charset info */
        pMulti->GetCharsetInfo((wchar_t *)bEnc,&mimeInfo);
        uiCodePage = (mimeInfo.uiInternetEncoding==0)?mimeInfo.uiCodePage:mimeInfo.uiInternetEncoding;
    }
    virtual void call(UErrorCode* status){
        BOOL* pUsedDefaultChar =(uiCodePage==CP_UTF8)?nullptr:&lpUsedDefaultChar;
        int winSize = WideCharToMultiByte(uiCodePage,CONVERSION_FLAGS,src,srcLen,dest,dstLen,nullptr, pUsedDefaultChar);
    }
    virtual long getOperationsPerIteration(){
        return srcLen;
    }
};
static inline void getErr(HRESULT err, UErrorCode& status){

    switch (err){

    case S_OK:
        //printf("Operation %s successful\n",operation);
        break;
    case S_FALSE:
        status = U_INTERNAL_PROGRAM_ERROR;
        break;
    case E_FAIL:
        status = U_ILLEGAL_CHAR_FOUND;
    }
}
class WinIMultiLanguageToUnicodePerfFunction : public UPerfFunction{

private:
    LPMULTILANGUAGE2 pMulti;
    LPMLANGCONVERTCHARSET pConvToUni;
    char* src; 
    UINT  srcLen;
    WCHAR dst[MAX_BUF_SIZE];
    UINT  dstLen;
    const char* cpName;

public:
    WinIMultiLanguageToUnicodePerfFunction(const char* name,char* source, UINT sourceLen, UErrorCode& status){
             
        CoInitialize(nullptr);

        /* create instance of converter object*/
        CoCreateInstance(
                          __uuidof(CMultiLanguage),
                          nullptr,
                          CLSCTX_SERVER,
                          __uuidof(IMultiLanguage2),
                          (void**)&pMulti
                          );



        MIMECSETINFO mimeInfo;
        mimeInfo.uiCodePage = 0;
        mimeInfo.uiInternetEncoding =0;
        HRESULT err=S_OK;
        unsigned short bEnc[30]={'\0'};
        const char* tenc=name;
        for(int i=0;*tenc!='\0';i++){
            bEnc[i]=*tenc;
            tenc++;
        }
        /* get the charset info */
        pMulti->GetCharsetInfo((wchar_t *)bEnc,&mimeInfo);
        pMulti->CreateConvertCharset(mimeInfo.uiCodePage, 1200 /*unicode*/, (DWORD)0,&pConvToUni);
        getErr(err,status);
        src = source;
        srcLen = sourceLen;
        dstLen = UPRV_LENGTHOF(dst);
        cpName = name;
    }

    virtual void call(UErrorCode* status){
        HRESULT err= pConvToUni->DoConversionToUnicode(src,&srcLen,dst, &dstLen);
        getErr(err,*status);
    }
    virtual long getOperationsPerIteration(){
        return srcLen;
    }
};

class WinIMultiLanguageFromUnicodePerfFunction : public UPerfFunction{

private:
    LPMULTILANGUAGE2 pMulti;
    LPMLANGCONVERTCHARSET pConvFromUni;
    WCHAR* src; 
    UINT  srcLen;
    char dst[MAX_BUF_SIZE];
    UINT  dstLen;
    const char* cpName;

public:
    WinIMultiLanguageFromUnicodePerfFunction(const char* name,WCHAR* source, UINT sourceLen, UErrorCode& status){
             
        CoInitialize(nullptr);

        /* create instance of converter object*/
        CoCreateInstance(
                          __uuidof(CMultiLanguage),
                          nullptr,
                          CLSCTX_SERVER,
                          __uuidof(IMultiLanguage2),
                          (void**)&pMulti
                          );



        MIMECSETINFO mimeInfo;
        mimeInfo.uiCodePage = 0;
        mimeInfo.uiInternetEncoding =0;
        HRESULT err=S_OK;
        unsigned short bEnc[30]={'\0'};
        const char* tenc=name;
        for(int i=0;*tenc!='\0';i++){
            bEnc[i]=*tenc;
            tenc++;
        }
        /* get the charset info */
        pMulti->GetCharsetInfo((wchar_t *)bEnc,&mimeInfo);
        pMulti->CreateConvertCharset(1200 /*unicode*/, mimeInfo.uiCodePage, (DWORD)0,&pConvFromUni);
        getErr(err,status);
        src = source;
        srcLen = sourceLen;
        dstLen = UPRV_LENGTHOF(dst);
        cpName = name;

    }

    virtual void call(UErrorCode* status){
        HRESULT err= pConvFromUni->DoConversionFromUnicode(src,&srcLen,dst, &dstLen);
        getErr(err,*status);
    }
    virtual long getOperationsPerIteration(){
        return srcLen;
    }
};

class WinIMultiLanguage2ToUnicodePerfFunction : public UPerfFunction{

private:
    LPMULTILANGUAGE2 pMulti;
    char* src; 
    UINT  srcLen;
    WCHAR dst[MAX_BUF_SIZE];
    UINT  dstLen;
    const char* cpName;
    DWORD dwEnc;
public:
    WinIMultiLanguage2ToUnicodePerfFunction(const char* name,char* source, UINT sourceLen, UErrorCode& status){
             
        CoInitialize(nullptr);

        /* create instance of converter object*/
        CoCreateInstance(
                          __uuidof(CMultiLanguage),
                          nullptr,
                          CLSCTX_SERVER,
                          __uuidof(IMultiLanguage2),
                          (void**)&pMulti
                          );

        src = source;
        srcLen = sourceLen;
        dstLen = UPRV_LENGTHOF(dst);
        cpName = name;
        unsigned short bEnc[30]={'\0'};
        const char* tenc=name;
        for(int i=0;*tenc!='\0';i++){
            bEnc[i]=*tenc;
            tenc++;
        }
        /* get the charset info */
        MIMECSETINFO mimeInfo;
        mimeInfo.uiCodePage = 0;
        mimeInfo.uiInternetEncoding =0;
        pMulti->GetCharsetInfo((wchar_t *)bEnc,&mimeInfo);
        dwEnc = (mimeInfo.uiInternetEncoding==0)?mimeInfo.uiCodePage:mimeInfo.uiInternetEncoding;
    }

    virtual void call(UErrorCode* status){
        DWORD dwMode=0;
        HRESULT err=  pMulti->ConvertStringToUnicode(&dwMode,dwEnc,(char*)src,&srcLen,dst, &dstLen);
        getErr(err,*status);
    }
    virtual long getOperationsPerIteration(){
        return srcLen;
    }
};

class WinIMultiLanguage2FromUnicodePerfFunction : public UPerfFunction{

private:
    LPMULTILANGUAGE2 pMulti;
    LPMLANGCONVERTCHARSET pConvFromUni;
    WCHAR* src; 
    UINT  srcLen;
    char dst[MAX_BUF_SIZE];
    UINT  dstLen;
    const char* cpName;
    DWORD dwEnc;

public:
    WinIMultiLanguage2FromUnicodePerfFunction(const char* name,WCHAR* source, UINT sourceLen, UErrorCode& status){
             
        CoInitialize(nullptr);

        /* create instance of converter object*/
        CoCreateInstance(
                          __uuidof(CMultiLanguage),
                          nullptr,
                          CLSCTX_SERVER,
                          __uuidof(IMultiLanguage2),
                          (void**)&pMulti
                          );


        unsigned short bEnc[30]={'\0'};
        const char* tenc=name;
        for(int i=0;*tenc!='\0';i++){
            bEnc[i]=*tenc;
            tenc++;
        }
        src = source;
        srcLen = sourceLen;
        dstLen = UPRV_LENGTHOF(dst);
        cpName = name;
        /* get the charset info */
        MIMECSETINFO mimeInfo;
        mimeInfo.uiCodePage = 0;
        mimeInfo.uiInternetEncoding =0;

        pMulti->GetCharsetInfo((wchar_t *)bEnc,&mimeInfo);
        dwEnc = (mimeInfo.uiInternetEncoding==0)?mimeInfo.uiCodePage:mimeInfo.uiInternetEncoding;
    }

    virtual void call(UErrorCode* status){
        DWORD dwMode=0;
        HRESULT err= pMulti->ConvertStringFromUnicode(&dwMode,dwEnc,src,&srcLen,dst, &dstLen);
        getErr(err,*status);
    }
    virtual long getOperationsPerIteration(){
        return srcLen;
    }
};

class  ConverterPerformanceTest : public UPerfTest{

public:

    ConverterPerformanceTest(int32_t argc, const char* argv[], UErrorCode& status);
    ~ConverterPerformanceTest();
    virtual UPerfFunction* runIndexedTest(int32_t index, UBool exec,const char* &name, char* par = nullptr);
    
    UPerfFunction* TestICU_CleanOpenAllConverters();
    UPerfFunction* TestICU_OpenAllConverters();

    UPerfFunction* TestICU_UTF8_ToUnicode();
    UPerfFunction* TestICU_UTF8_FromUnicode();
    UPerfFunction* TestWinANSI_UTF8_ToUnicode();
    UPerfFunction* TestWinANSI_UTF8_FromUnicode();
    UPerfFunction* TestWinIML2_UTF8_ToUnicode();
    UPerfFunction* TestWinIML2_UTF8_FromUnicode();
        
    UPerfFunction* TestICU_Latin1_ToUnicode();
    UPerfFunction* TestICU_Latin1_FromUnicode();
    UPerfFunction* TestWinANSI_Latin1_ToUnicode();
    UPerfFunction* TestWinANSI_Latin1_FromUnicode();
    UPerfFunction* TestWinIML2_Latin1_ToUnicode();
    UPerfFunction* TestWinIML2_Latin1_FromUnicode();

    UPerfFunction* TestICU_EBCDIC_Arabic_ToUnicode();
    UPerfFunction* TestICU_EBCDIC_Arabic_FromUnicode();
    UPerfFunction* TestWinANSI_EBCDIC_Arabic_ToUnicode();
    UPerfFunction* TestWinANSI_EBCDIC_Arabic_FromUnicode();
    UPerfFunction* TestWinIML2_EBCDIC_Arabic_ToUnicode();
    UPerfFunction* TestWinIML2_EBCDIC_Arabic_FromUnicode();

    UPerfFunction* TestICU_Latin8_ToUnicode();
    UPerfFunction* TestICU_Latin8_FromUnicode();
    UPerfFunction* TestWinANSI_Latin8_ToUnicode();
    UPerfFunction* TestWinANSI_Latin8_FromUnicode();
    UPerfFunction* TestWinIML2_Latin8_ToUnicode();
    UPerfFunction* TestWinIML2_Latin8_FromUnicode();

    
    UPerfFunction* TestICU_SJIS_ToUnicode();
    UPerfFunction* TestICU_SJIS_FromUnicode();
    UPerfFunction* TestWinANSI_SJIS_ToUnicode();
    UPerfFunction* TestWinANSI_SJIS_FromUnicode();
    UPerfFunction* TestWinIML2_SJIS_ToUnicode();
    UPerfFunction* TestWinIML2_SJIS_FromUnicode();

    UPerfFunction* TestICU_EUCJP_ToUnicode();
    UPerfFunction* TestICU_EUCJP_FromUnicode();
    UPerfFunction* TestWinANSI_EUCJP_ToUnicode();
    UPerfFunction* TestWinANSI_EUCJP_FromUnicode();
    UPerfFunction* TestWinIML2_EUCJP_ToUnicode();
    UPerfFunction* TestWinIML2_EUCJP_FromUnicode();

    UPerfFunction* TestICU_GB2312_ToUnicode();
    UPerfFunction* TestICU_GB2312_FromUnicode();
    UPerfFunction* TestWinANSI_GB2312_ToUnicode();
    UPerfFunction* TestWinANSI_GB2312_FromUnicode();
    UPerfFunction* TestWinIML2_GB2312_ToUnicode();
    UPerfFunction* TestWinIML2_GB2312_FromUnicode();


    UPerfFunction* TestICU_ISO2022KR_ToUnicode();
    UPerfFunction* TestICU_ISO2022KR_FromUnicode();
    UPerfFunction* TestWinANSI_ISO2022KR_ToUnicode();
    UPerfFunction* TestWinANSI_ISO2022KR_FromUnicode();
    UPerfFunction* TestWinIML2_ISO2022KR_ToUnicode();
    UPerfFunction* TestWinIML2_ISO2022KR_FromUnicode();   

    UPerfFunction* TestICU_ISO2022JP_ToUnicode();
    UPerfFunction* TestICU_ISO2022JP_FromUnicode();
    UPerfFunction* TestWinANSI_ISO2022JP_ToUnicode();
    UPerfFunction* TestWinANSI_ISO2022JP_FromUnicode();
    UPerfFunction* TestWinIML2_ISO2022JP_ToUnicode();
    UPerfFunction* TestWinIML2_ISO2022JP_FromUnicode(); 

};

#endif

