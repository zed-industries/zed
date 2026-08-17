
/*--------------------------------------------------------------------*/
/*--- A simple sequence matching facility.                         ---*/
/*---                                          pub_tool_seqmatch.h ---*/
/*--------------------------------------------------------------------*/

/*
   This file is part of Valgrind, a dynamic binary instrumentation
   framework.

   Copyright (C) 2008-2017 OpenWorks Ltd
      info@open-works.co.uk

   This program is free software; you can redistribute it and/or
   modify it under the terms of the GNU General Public License as
   published by the Free Software Foundation; either version 2 of the
   License, or (at your option) any later version.

   This program is distributed in the hope that it will be useful, but
   WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
   General Public License for more details.

   You should have received a copy of the GNU General Public License
   along with this program; if not, see <http://www.gnu.org/licenses/>.

   The GNU General Public License is contained in the file COPYING.
*/

#ifndef __PUB_TOOL_SEQMATCH_H
#define __PUB_TOOL_SEQMATCH_H

#include "pub_tool_basics.h"   // UWord

/* Perform totally abstractified sequence matching, of an input
   sequence against a pattern sequence.  The pattern sequence may
   include '*' elements (matches any number of anything) and '?'
   elements (matches exactly one element).  '*' patterns are matched
   frugally, meaning that they are "assigned" the minimum amount of
   input needed to make the match work.

   This routine is recursive.  The recursion depth is equal to the
   number of '*' elements in the pattern.  There is no guard against
   excessive recursion.  This function has no global state and so is
   thread-safe and re-entrant.  (It needs to be, since m_errormgr will
   effectively construct two simultaneous calls to it, once to match
   at the frame level, and whilst that is happening, once at the
   function/object-name level.)

   When matchAll is True, the entire input sequence must match the
   pattern, else the match fails.  When False, it's ok for some tail
   of the input sequence to be unused -- so we're only matching a
   prefix.

   The pattern array is starts at 'patt' and consists of 'nPatt'
   elements each of size 'szbPatt'.  For the initial call, pass a
   value of zero to 'ixPatt'.

   The input sequence can be similarly described using
   input/nInput/szbInput/ixInput.
   Alternatively, the input can be lazily constructed using an
   inputCompleter. When using an inputCompleter, input/nInput/szbInput
   are unused.

   pIsStar should return True iff the pointed-to pattern element is
   conceptually a '*'.

   pIsQuery should return True iff the pointed-to-pattern element is
   conceptually a '?'.

   pattEQinp takes a pointer to a pattern element and a pointer to an
   input element.  It should return True iff they are considered
   equal.  Note that the pattern element is guaranteed to be neither
   (conceptually) '*' nor '?', so it must be a literal (in the sense
   that all the input sequence elements are literal).

   If inputCompleter is not NULL, the input will be lazily constructed
   when pattEQinp is called.
   For lazily constructing the input element, the two last arguments
   of pattEQinp are the inputCompleter and the index of the input
   element to complete.
   VG_(generic_match) calls (*haveInputInpC)(inputCompleter,ixInput) to
   check if there is an element ixInput in the input sequence.
*/
Bool VG_(generic_match) ( 
        Bool matchAll,
        const void* patt,  SizeT szbPatt,  UWord nPatt,  UWord ixPatt,
        const void* input, SizeT szbInput, UWord nInput, UWord ixInput,
        Bool (*pIsStar)(const void*),
        Bool (*pIsQuery)(const void*),
        Bool (*pattEQinp)(const void*,const void*,void*,UWord),
        void* inputCompleter,
        Bool (*haveInputInpC)(void*,UWord)
     );

/* Mini-regexp function.  Searches for 'pat' in 'str'.  Supports
   meta-symbols '*' and '?'.  There is no way to escape meta-symbols
   in the pattern. */
Bool VG_(string_match) ( const HChar* pat, const HChar* str );

#endif   // __PUB_TOOL_SEQMATCH_H

/*--------------------------------------------------------------------*/
/*--- end                                      pub_tool_seqmatch.h ---*/
/*--------------------------------------------------------------------*/
