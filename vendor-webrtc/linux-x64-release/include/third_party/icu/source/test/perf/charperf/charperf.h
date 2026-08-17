/*
**********************************************************************
* © 2016 and later: Unicode, Inc. and others.
* License & terms of use: http://www.unicode.org/copyright.html
**********************************************************************
**********************************************************************
* Copyright (c) 2002-2006, International Business Machines
* Corporation and others.  All Rights Reserved.
**********************************************************************
**********************************************************************
*/
#ifndef _CHARPERF_H
#define _CHARPERF_H

#include "unicode/uchar.h"

#include "unicode/uperf.h"
#include <stdlib.h>
#include <stdio.h>
#include <wchar.h>
#include <wctype.h>

typedef void (*CharPerfFn)(UChar32 ch);
typedef void (*StdLibCharPerfFn)(wchar_t ch);

class CharPerfFunction : public UPerfFunction
{
public:
    virtual void call(UErrorCode* status)
    {
        for (UChar32 i = MIN_; i < MAX_; i ++) {
            (*m_fn_)(i);
        }
    }

    virtual long getOperationsPerIteration()
    {
        return MAX_ - MIN_;
    }
    CharPerfFunction(CharPerfFn func, UChar32 min, UChar32 max)
    {
        m_fn_ = func;
        MIN_ = min;
        MAX_ = max;
    }   

private:
    CharPerfFn m_fn_;
    UChar32 MIN_;
    UChar32 MAX_;
}; 

class StdLibCharPerfFunction : public UPerfFunction
{
public:
    virtual void call(UErrorCode* status)
    {
        // note wchar_t is unsigned, it will revert to 0 once it reaches 
        // 65535
        for (wchar_t i = MIN_; i < MAX_; i ++) {
            (*m_fn_)(i);
        }
    }

    virtual long getOperationsPerIteration()
    {
        return MAX_ - MIN_;
    }

    StdLibCharPerfFunction(StdLibCharPerfFn func, wchar_t min, wchar_t max)
    {
        m_fn_ = func;			
        MIN_ = min;
        MAX_ = max;
    }   

    ~StdLibCharPerfFunction()
    {			
    }

private:
    StdLibCharPerfFn m_fn_;
    wchar_t MIN_;
    wchar_t MAX_;
};

class CharPerformanceTest : public UPerfTest
{
public:
    CharPerformanceTest(int32_t argc, const char *argv[], UErrorCode &status);
    ~CharPerformanceTest();
    virtual UPerfFunction* runIndexedTest(int32_t index, UBool exec,
        const char *&name, 
        char *par = nullptr);
    UPerfFunction* TestIsAlpha();
    UPerfFunction* TestIsUpper();
    UPerfFunction* TestIsLower();
    UPerfFunction* TestIsDigit();
    UPerfFunction* TestIsSpace();
    UPerfFunction* TestIsAlphaNumeric();
    UPerfFunction* TestIsPrint();
    UPerfFunction* TestIsControl();
    UPerfFunction* TestToLower();
    UPerfFunction* TestToUpper();
    UPerfFunction* TestIsWhiteSpace();
    UPerfFunction* TestStdLibIsAlpha();
    UPerfFunction* TestStdLibIsUpper();
    UPerfFunction* TestStdLibIsLower();
    UPerfFunction* TestStdLibIsDigit();
    UPerfFunction* TestStdLibIsSpace();
    UPerfFunction* TestStdLibIsAlphaNumeric();
    UPerfFunction* TestStdLibIsPrint();
    UPerfFunction* TestStdLibIsControl();
    UPerfFunction* TestStdLibToLower();
    UPerfFunction* TestStdLibToUpper();
    UPerfFunction* TestStdLibIsWhiteSpace();

private:
    UChar32 MIN_;
    UChar32 MAX_;
};

inline void isAlpha(UChar32 ch) 
{
    u_isalpha(ch);
}

inline void isUpper(UChar32 ch)
{
    u_isupper(ch);
}

inline void isLower(UChar32 ch)
{
    u_islower(ch);
}

inline void isDigit(UChar32 ch)
{
    u_isdigit(ch);
}

inline void isSpace(UChar32 ch)
{
    u_isspace(ch);
}

inline void isAlphaNumeric(UChar32 ch)
{
    u_isalnum(ch);
}

/**
* This test may be different since c lib has a type PUNCT and it is printable.
* iswgraph is not used for testing since it is a subset of iswprint with the
* exception of returning true for white spaces. no match found in icu4c.
*/
inline void isPrint(UChar32 ch)
{
    u_isprint(ch);
}

inline void isControl(UChar32 ch)
{
    u_iscntrl(ch);
}

inline void toLower(UChar32 ch)
{
    u_tolower(ch);
}

inline void toUpper(UChar32 ch)
{
    u_toupper(ch);
}

inline void isWhiteSpace(UChar32 ch)
{
    u_isWhitespace(ch);
}

inline void StdLibIsAlpha(wchar_t ch)
{
    iswalpha(ch);
}

inline void StdLibIsUpper(wchar_t ch)
{
    iswupper(ch);
}

inline void StdLibIsLower(wchar_t ch)
{
    iswlower(ch);
}

inline void StdLibIsDigit(wchar_t ch)
{
    iswdigit(ch);
}

inline void StdLibIsSpace(wchar_t ch)
{
    iswspace(ch);
}

inline void StdLibIsAlphaNumeric(wchar_t ch)
{
    iswalnum(ch);
}

/**
* This test may be different since c lib has a type PUNCT and it is printable.
* iswgraph is not used for testing since it is a subset of iswprint with the
* exception of returning true for white spaces. no match found in icu4c.
*/
inline void StdLibIsPrint(wchar_t ch)
{
    iswprint(ch);
}

inline void StdLibIsControl(wchar_t ch)
{
    iswcntrl(ch);
}

inline void StdLibToLower(wchar_t ch)
{
    towlower(ch);
}

inline void StdLibToUpper(wchar_t ch)
{
    towupper(ch);
}

inline void StdLibIsWhiteSpace(wchar_t ch)
{
    iswspace(ch);
}

#endif // CHARPERF_H
