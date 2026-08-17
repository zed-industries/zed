/*
***********************************************************************
* © 2016 and later: Unicode, Inc. and others.
* License & terms of use: http://www.unicode.org/copyright.html
***********************************************************************
***********************************************************************
* Copyright (c) 2002-2016, International Business Machines
* Corporation and others.  All Rights Reserved.
***********************************************************************
*/
#ifndef _STRINGPERF_H
#define _STRINGPERF_H

#include "cmemory.h"
#include "unicode/utypes.h"
#include "unicode/unistr.h"

#include "unicode/uperf.h"

#include <string.h>
#include <stdio.h>
#include <stdlib.h>

typedef std::wstring stlstring;	

/* Define all constants for test case operations */
#define MAXNUMLINES	40000	//Max number of lines in a test data file
#define MAXSRCLEN 20		//Max length of one line. maybe a larger number, but it need more mem
#define LOOPS 100			//Iterations
//#define LOOPS 10
#define catenate_STRLEN 2

const char16_t uTESTCHAR1 =  'a';
const wchar_t wTESTCHAR1 = 'a';
const UnicodeString uEMPTY;
const stlstring sEMPTY;
UnicodeString unistr;
stlstring stlstr;
// Simulate construction with a single-char string for basic_string
wchar_t simulate[2]={wTESTCHAR1, 0};

/* Constants for scan operation */
U_STRING_DECL(scan_STRING, "Dot. 123. Some more data.", 25);
const UnicodeString uScan_STRING=UnicodeString(scan_STRING);
const stlstring sScan_STRING=stlstring(L"Dot. 123. Some more data.");

/* global variables or constants for concatenation operation */
U_STRING_DECL(uCatenate_STR, "!!", 2);
const stlstring sCatenate_STR=stlstring(L"!!");
static UnicodeString* catICU;
static stlstring* catStd;
UBool bCatenatePrealloc;

/* type defines */
typedef struct WLine WLine;
struct  WLine {
    wchar_t   name[100];
    int32_t   len;
}; //struct to store one line of wchar_t string

enum FnType { Fn_ICU, Fn_STD };
typedef FnType FnType;
typedef void (*ICUStringPerfFn)(const char16_t* src,int32_t srcLen, UnicodeString s0);
typedef void (*StdStringPerfFn)(const wchar_t* src,int32_t srcLen, stlstring s0);


class StringPerfFunction : public UPerfFunction
{
public:

    virtual long getEventsPerIteration(){
        int loops = LOOPS;
        if (catICU) { delete catICU;}
        if (catStd) { delete catStd;}

        if (bCatenatePrealloc) {

            int to_alloc = loops * MAXNUMLINES * (MAXSRCLEN + catenate_STRLEN);
            catICU = new UnicodeString(to_alloc,'a',0);
            //catICU = new UnicodeString();

            catStd = new stlstring();
            //catStd -> reserve(loops * MAXNUMLINES * (MAXSRCLEN + catenate_STRLEN));
            catStd -> reserve(110000000);
        } else {
            catICU = new UnicodeString();
            catStd = new stlstring();
        }

        return -1;
    }

    virtual void call(UErrorCode* status)
    {
        if(line_mode_==true){
            if(uselen_){
                for(int32_t i = 0; i< numLines_; i++){
                    if (fnType_==Fn_ICU) {
                        (*fn1_)(lines_[i].name,lines_[i].len,uS0_[i]);
                    } else {
                        (*fn2_)(wlines_[i].name,wlines_[i].len,sS0_[i]);
                    }
                }
            }else{
                for(int32_t i = 0; i< numLines_; i++){
                    if (fnType_==Fn_ICU) {
                        (*fn1_)(lines_[i].name,-1,uS0_[i]);
                    } else {
                        (*fn2_)(wlines_[i].name,-1,sS0_[i]);
                    }
                }
            }
        }else{
            if(uselen_){
                if (fnType_==Fn_ICU) {
                    (*fn1_)(src_,srcLen_,*ubulk_);
                } else {
                    (*fn2_)(wsrc_,wsrcLen_,*sbulk_);
                }
            }else{
                if (fnType_==Fn_ICU) {
                    (*fn1_)(src_,-1,*ubulk_);
                } else {
                    (*fn2_)(wsrc_,-1,*sbulk_);
                }
            }
        }
    }

    virtual long getOperationsPerIteration()
    {
        if(line_mode_==true){
            return numLines_;
        }else{
            return 1;
        }
    }

    StringPerfFunction(ICUStringPerfFn func, ULine* srcLines, int32_t srcNumLines, UBool uselen)
    {

        fn1_ = func;
        lines_=srcLines;
        wlines_=nullptr;
        numLines_=srcNumLines;
        uselen_=uselen;
        line_mode_=true;
        src_ = nullptr;
        srcLen_ = 0;
        wsrc_ = nullptr;
        wsrcLen_ = 0;
        fnType_ = Fn_ICU;

        uS0_=new UnicodeString[numLines_];
        for(int32_t i=0; i<numLines_; i++) {
            uS0_[i]=UnicodeString(lines_[i].name, lines_[i].len);
        }
        sS0_=nullptr;
        ubulk_=nullptr;
        sbulk_=nullptr;
    }

    StringPerfFunction(StdStringPerfFn func, ULine* srcLines, int32_t srcNumLines, UBool uselen)
    {

        fn2_ = func;
        lines_=srcLines;
        wlines_=nullptr;
        numLines_=srcNumLines;
        uselen_=uselen;
        line_mode_=true;
        src_ = nullptr;
        srcLen_ = 0;
        wsrc_ = nullptr;
        wsrcLen_ = 0;
        fnType_ = Fn_STD;

        uS0_=nullptr;
        ubulk_=nullptr;
        sbulk_=nullptr;

        //fillin wlines_[], sS0_[]
        prepareLinesForStd();
    }

    StringPerfFunction(ICUStringPerfFn func, char16_t* source, int32_t sourceLen, UBool uselen)
    {

        fn1_ = func;
        lines_=nullptr;
        wlines_=nullptr;
        numLines_=0;
        uselen_=uselen;
        line_mode_=false;
        src_ = new char16_t[sourceLen];
        memcpy(src_, source, sourceLen * U_SIZEOF_UCHAR);
        srcLen_ = sourceLen;
        wsrc_ = nullptr;
        wsrcLen_ = 0;
        fnType_ = Fn_ICU;

        uS0_=nullptr;
        sS0_=nullptr;
        ubulk_=new UnicodeString(src_,srcLen_);
        sbulk_=nullptr;
    }

    StringPerfFunction(StdStringPerfFn func, char16_t* source, int32_t sourceLen, UBool uselen)
    {

        fn2_ = func;
        lines_=nullptr;
        wlines_=nullptr;
        numLines_=0;
        uselen_=uselen;
        line_mode_=false;
        src_ = new char16_t[sourceLen];
        memcpy(src_, source, sourceLen * U_SIZEOF_UCHAR);
        srcLen_ = sourceLen;
        fnType_ = Fn_STD;

        uS0_=nullptr;
        sS0_=nullptr;
        ubulk_=nullptr;

        //fillin wsrc_, sbulk_
        prepareBulkForStd();

    }

    ~StringPerfFunction()
    {
        //free(src_);
        free(wsrc_);
        delete[] src_;
        delete ubulk_;
        delete sbulk_;
        delete[] uS0_;
        delete[] sS0_;
        delete[] wlines_;
    }

private:
    void prepareLinesForStd()
    {
        UErrorCode err=U_ZERO_ERROR;

        wlines_=new WLine[numLines_];
        wchar_t ws[100];
        int32_t wcap = UPRV_LENGTHOF(ws);
        int32_t wl;
        wchar_t* wcs;

        sS0_=new stlstring[numLines_];
        for(int32_t i=0; i<numLines_; i++) {
            if(uselen_) {
                wcs = u_strToWCS(ws, wcap, &wl, lines_[i].name, lines_[i].len, &err);
                memcpy(wlines_[i].name, wcs, wl * sizeof(wchar_t));
                wlines_[i].len = wl;
                sS0_[i]=stlstring(wlines_[i].name, wlines_[i].len);
            } else {
                wcs = u_strToWCS(ws, wcap, &wl, lines_[i].name, lines_[i].len-1, &err);
                memcpy(wlines_[i].name, wcs, wl*sizeof(wchar_t));
                wlines_[i].len = wl;
                sS0_[i]=stlstring(wlines_[i].name, wlines_[i].len+1);
            }

            if (U_FAILURE(err)) {
                return;
            }
        }

    }

    void prepareBulkForStd()
    {
        UErrorCode err=U_ZERO_ERROR;

        const char16_t* uSrc = src_;
        int32_t uSrcLen = srcLen_;
        wchar_t* wDest = nullptr;
        int32_t wDestLen = 0;
        int32_t reqLen= 0 ;

        if(uselen_) {
            /* pre-flight*/
            u_strToWCS(wDest,wDestLen,&reqLen,uSrc,uSrcLen,&err);

            if(err == U_BUFFER_OVERFLOW_ERROR){
                err=U_ZERO_ERROR;
                wDest =(wchar_t*) malloc(sizeof(wchar_t) * (reqLen));
                wDestLen = reqLen;
                u_strToWCS(wDest,wDestLen,&reqLen,uSrc,uSrcLen,&err);
            }

            if (U_SUCCESS(err)) {
                wsrc_ = wDest;
                wsrcLen_ = wDestLen;
                sbulk_=new stlstring(wsrc_,wsrcLen_);
            }

        } else {
            /* pre-flight*/
            u_strToWCS(wDest,wDestLen,&reqLen,uSrc,uSrcLen-1,&err);

            if(err == U_BUFFER_OVERFLOW_ERROR){
                err=U_ZERO_ERROR;
                wDest =(wchar_t*) malloc(sizeof(wchar_t) * (reqLen+1));
                wDestLen = reqLen+1;
                u_strToWCS(wDest,wDestLen,&reqLen,uSrc,uSrcLen-1,&err);
            }

            if (U_SUCCESS(err)) {
                wsrc_ = wDest;
                wsrcLen_ = wDestLen;
                sbulk_=new stlstring(wsrc_);
            }
        }

        //free(wDest);
    }


private:
    ICUStringPerfFn fn1_;
    StdStringPerfFn fn2_;

    ULine* lines_;
    WLine* wlines_;
    int32_t numLines_;

    UBool uselen_;
    char16_t* src_;
    int32_t srcLen_;
    wchar_t* wsrc_;
    int32_t wsrcLen_;
    UBool line_mode_;

    //added for preparing testing data
    UnicodeString* uS0_;
    stlstring* sS0_;
    UnicodeString* ubulk_;
    stlstring* sbulk_;
    FnType fnType_;
};


class StringPerformanceTest : public UPerfTest
{
public:
    StringPerformanceTest(int32_t argc, const char *argv[], UErrorCode &status);
    ~StringPerformanceTest();
    virtual UPerfFunction* runIndexedTest(int32_t index, UBool exec,
                                          const char *&name,
                                          char *par = nullptr);
    UPerfFunction* TestCtor();
    UPerfFunction* TestCtor1();
    UPerfFunction* TestCtor2();
    UPerfFunction* TestCtor3();
    UPerfFunction* TestAssign();
    UPerfFunction* TestAssign1();
    UPerfFunction* TestAssign2();
    UPerfFunction* TestGetch();
    UPerfFunction* TestCatenate();
    UPerfFunction* TestScan();
    UPerfFunction* TestScan1();
    UPerfFunction* TestScan2();

    UPerfFunction* TestStdLibCtor();
    UPerfFunction* TestStdLibCtor1();
    UPerfFunction* TestStdLibCtor2();
    UPerfFunction* TestStdLibCtor3();
    UPerfFunction* TestStdLibAssign();
    UPerfFunction* TestStdLibAssign1();
    UPerfFunction* TestStdLibAssign2();
    UPerfFunction* TestStdLibGetch();
    UPerfFunction* TestStdLibCatenate();
    UPerfFunction* TestStdLibScan();
    UPerfFunction* TestStdLibScan1();
    UPerfFunction* TestStdLibScan2();

private:
    long COUNT_;
    ULine* filelines_;
    char16_t* StrBuffer;
    int32_t StrBufferLen;

};


inline void ctor(const char16_t* src,int32_t srcLen, UnicodeString s0)
{
    UnicodeString a;
}

inline void ctor1(const char16_t* src,int32_t srcLen, UnicodeString s0)
{
    UnicodeString b(uTESTCHAR1);
}

inline void ctor2(const char16_t* src,int32_t srcLen, UnicodeString s0)
{
    UnicodeString c(uEMPTY);
}

inline void ctor3(const char16_t* src,int32_t srcLen, UnicodeString s0)
{
    UnicodeString d(src,srcLen);
}

inline UnicodeString icu_assign_helper(const char16_t* src,int32_t srcLen)
{
    if (srcLen==-1) { return src;}
    else { return UnicodeString(src, srcLen);}
}

inline void assign(const char16_t* src,int32_t srcLen, UnicodeString s0)
{
    unistr = icu_assign_helper(src,srcLen);
}

inline void assign1(const char16_t* src,int32_t srcLen, UnicodeString s0)
{
    unistr.setTo(src, srcLen);
}

inline void assign2(const char16_t* src,int32_t srcLen, UnicodeString s0)
{
    unistr = s0;
}

inline void getch(const char16_t* src,int32_t srcLen, UnicodeString s0)
{
    s0.charAt(0);
}


inline void catenate(const char16_t* src,int32_t srcLen, UnicodeString s0)
{
    UTimer mystart, mystop;
    utimer_getTime(&mystart);

    *catICU += s0;

    utimer_getTime(&mystop);
    double mytime = utimer_getDeltaSeconds(&mystart,&mystop);

    *catICU += uCatenate_STR;
}

volatile int scan_idx;
U_STRING_DECL(SCAN1, "123", 3);

inline void scan(const char16_t* src,int32_t srcLen, UnicodeString s0)
{
    char16_t c='.';
    scan_idx = uScan_STRING.indexOf(c);
}

inline void scan1(const char16_t* src,int32_t srcLen, UnicodeString s0)
{
    scan_idx = uScan_STRING.indexOf(SCAN1,3);
}

inline void scan2(const char16_t* src,int32_t srcLen, UnicodeString s0)
{
    char16_t c1='s';
    char16_t c2='m';
    scan_idx = uScan_STRING.indexOf(c1);
    scan_idx = uScan_STRING.indexOf(c2);
}


inline void StdLibCtor(const wchar_t* src,int32_t srcLen, stlstring s0)
{
    stlstring a;
}

inline void StdLibCtor1(const wchar_t* src,int32_t srcLen, stlstring s0)
{
    stlstring b(simulate);
}

inline void StdLibCtor2(const wchar_t* src,int32_t srcLen, stlstring s0)
{
    stlstring c(sEMPTY);
}

inline void StdLibCtor3(const wchar_t* src,int32_t srcLen, stlstring s0)
{
    if (srcLen==-1) {
        stlstring d(src);
    }else {
        stlstring d(src, srcLen);
    }
}

inline stlstring stl_assign_helper(const wchar_t* src,int32_t srcLen)
{
    if (srcLen==-1) { return src;}
    else { return stlstring(src, srcLen);}
}

inline void StdLibAssign(const wchar_t* src,int32_t srcLen, stlstring s0)
{
    stlstr = stl_assign_helper(src,srcLen);
}

inline void StdLibAssign1(const wchar_t* src,int32_t srcLen, stlstring s0)
{
    if (srcLen==-1) { stlstr=src;}
    else { stlstr.assign(src, srcLen);}
}

inline void StdLibAssign2(const wchar_t* src,int32_t srcLen, stlstring s0)
{
    stlstr=s0;
}

inline void StdLibGetch(const wchar_t* src,int32_t srcLen, stlstring s0)
{
    s0.at(0);
}

inline void StdLibCatenate(const wchar_t* src,int32_t srcLen, stlstring s0)
{
    UTimer mystart, mystop;
    utimer_getTime(&mystart);

    *catStd += s0;
    *catStd += sCatenate_STR;

    utimer_getTime(&mystop);
    double mytime = utimer_getDeltaSeconds(&mystart,&mystop);

}

inline void StdLibScan(const wchar_t* src,int32_t srcLen, stlstring s0)
{
    scan_idx = (int) sScan_STRING.find('.');	
}

inline void StdLibScan1(const wchar_t* src,int32_t srcLen, stlstring s0)
{
    scan_idx = (int) sScan_STRING.find(L"123");
}

inline void StdLibScan2(const wchar_t* src,int32_t srcLen, stlstring s0)
{	
    scan_idx = (int) sScan_STRING.find_first_of(L"sm");
}

#endif // STRINGPERF_H

