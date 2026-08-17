// © 2016 and later: Unicode, Inc. and others.
// License & terms of use: http://www.unicode.org/copyright.html
/********************************************************************
 * COPYRIGHT: 
 * Copyright (c) 1997-2003, International Business Machines Corporation and
 * others. All Rights Reserved.
 ********************************************************************/

#ifndef SFDWCHIT_H
#define SFDWCHIT_H

#include "unicode/chariter.h"
#include "intltest.h"

class SimpleFwdCharIterator : public ForwardCharacterIterator {
public:
    // not used -- SimpleFwdCharIterator(const UnicodeString& s);
    SimpleFwdCharIterator(char16_t *s, int32_t len, UBool adopt = false);

    virtual ~SimpleFwdCharIterator();

  /**
   * Returns true when both iterators refer to the same
   * character in the same character-storage object.  
   */
  // not used -- virtual bool operator==(const ForwardCharacterIterator& that) const;
        
  /**
   * Generates a hash code for this iterator.  
   */
  virtual int32_t hashCode() const override;
        
  /**
   * Returns a UClassID for this ForwardCharacterIterator ("poor man's
   * RTTI").<P> Despite the fact that this function is public,
   * DO NOT CONSIDER IT PART OF CHARACTERITERATOR'S API!  
   */
  virtual UClassID getDynamicClassID() const override;

  /**
   * Gets the current code unit for returning and advances to the next code unit
   * in the iteration range
   * (toward endIndex()).  If there are
   * no more code units to return, returns DONE.
   */
  virtual char16_t      nextPostInc() override;
        
  /**
   * Gets the current code point for returning and advances to the next code point
   * in the iteration range
   * (toward endIndex()).  If there are
   * no more code points to return, returns DONE.
   */
  virtual UChar32       next32PostInc() override;
        
  /**
   * Returns false if there are no more code units or code points
   * at or after the current position in the iteration range.
   * This is used with nextPostInc() or next32PostInc() in forward
   * iteration.
   */
  virtual UBool        hasNext() override;

protected:
    SimpleFwdCharIterator() {}
    SimpleFwdCharIterator(const SimpleFwdCharIterator &other)
        : ForwardCharacterIterator(other) {}
    SimpleFwdCharIterator &operator=(const SimpleFwdCharIterator&) { return *this; }
private:
    static const int32_t            kInvalidHashCode;
    static const int32_t            kEmptyHashCode;

    char16_t *fStart, *fEnd, *fCurrent;
    int32_t fLen;
    UBool fBogus;
    int32_t fHashCode;
};

#endif
