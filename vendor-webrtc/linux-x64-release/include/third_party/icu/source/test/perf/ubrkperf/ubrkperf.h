/*
***********************************************************************
* © 2016 and later: Unicode, Inc. and others.
* License & terms of use: http://www.unicode.org/copyright.html
***********************************************************************
**********************************************************************
* Copyright (c) 2002-2011, International Business Machines
* Corporation and others.  All Rights Reserved.
**********************************************************************
**********************************************************************
*/
#ifndef _UBRKPERF_H
#define _UBRKPERF_H

#include "unicode/uperf.h"

#include <unicode/brkiter.h>

class ICUBreakFunction : public UPerfFunction {
protected:
  BreakIterator *m_brkIt_;
  const char16_t *m_file_;
  int32_t m_fileLen_;
  int32_t m_noBreaks_;
  UErrorCode m_status_;
public:
  ICUBreakFunction(const char *locale, const char *mode, const char16_t *file, int32_t file_len) :
      m_brkIt_(nullptr),
      m_file_(file),
      m_fileLen_(file_len),
      m_noBreaks_(-1),
      m_status_(U_ZERO_ERROR)
  {
    switch(mode[0]) {
    case 'c' :
      m_brkIt_ = BreakIterator::createCharacterInstance(locale, m_status_);
      break;
    case 'w' : 
      m_brkIt_ = BreakIterator::createWordInstance(locale, m_status_);
      break;
    case 'l' :
      m_brkIt_ = BreakIterator::createLineInstance(locale, m_status_);
      break;
    case 's' :
      m_brkIt_ = BreakIterator::createSentenceInstance(locale, m_status_);
      break;
    default:
      // should not happen as we already check for this in the caller
      m_status_ = U_ILLEGAL_ARGUMENT_ERROR;
      break;
    }
  }

  ~ICUBreakFunction() {  delete m_brkIt_; }
  virtual void call(UErrorCode *status) = 0;
  virtual long getOperationsPerIteration() { return m_fileLen_; }
  virtual long getEventsPerIteration() { return m_noBreaks_; }
  virtual UErrorCode getStatus() { return m_status_; }
};

class ICUIsBound : public ICUBreakFunction {
public:
  ICUIsBound(const char *locale, const char *mode, const char16_t *file, int32_t file_len) :
      ICUBreakFunction(locale, mode, file, file_len)
  {
    m_noBreaks_ = 0;
    m_brkIt_->setText(UnicodeString(m_file_, m_fileLen_));
    m_brkIt_->first();
    int32_t j = 0;
    for(j = 0; j < m_fileLen_; j++) {
      if(m_brkIt_->isBoundary(j)) {
        m_noBreaks_++;
      }
    }    
  }
  virtual void call(UErrorCode *status) 
  {
    m_noBreaks_ = 0;
    int32_t j = 0;
    for(j = 0; j < m_fileLen_; j++) {
      if(m_brkIt_->isBoundary(j)) {
        m_noBreaks_++;
      }
    }
  }
};

class ICUForward : public ICUBreakFunction {
public:
  ICUForward(const char *locale, const char *mode, const char16_t *file, int32_t file_len) :
      ICUBreakFunction(locale, mode, file, file_len)
  {
    m_noBreaks_ = 0;
    m_brkIt_->setText(UnicodeString(m_file_, m_fileLen_));
    m_brkIt_->first();
    while(m_brkIt_->next() != BreakIterator::DONE) {
      m_noBreaks_++;
    }
  }
  virtual void call(UErrorCode *status) 
  {
    m_noBreaks_ = 0;
    m_brkIt_->first();
    while(m_brkIt_->next() != BreakIterator::DONE) {
      m_noBreaks_++;
    }
  }
};

class DarwinBreakFunction : public UPerfFunction {
public:
  virtual void call(UErrorCode *status) {};
};

class BreakIteratorPerformanceTest : public UPerfTest {
private:
  const char* m_mode_;
  const char16_t* m_file_;
  int32_t m_fileLen_;

public:
  BreakIteratorPerformanceTest(int32_t argc, const char* argv[], UErrorCode& status);
  ~BreakIteratorPerformanceTest();

  virtual UPerfFunction* runIndexedTest(int32_t index, UBool exec,
    const char* &name, char* par = nullptr);

  UPerfFunction* TestICUForward();
  UPerfFunction* TestICUIsBound();

  UPerfFunction* TestDarwinForward();
  UPerfFunction* TestDarwinIsBound();

};

#endif // UBRKPERF_H
