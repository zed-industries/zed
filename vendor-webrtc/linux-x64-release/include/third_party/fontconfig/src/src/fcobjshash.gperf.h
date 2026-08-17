/*
 * fontconfig/src/fcobjshash.gperf.h
 *
 * Copyright © 2000 Keith Packard
 *
 * Permission to use, copy, modify, distribute, and sell this software and its
 * documentation for any purpose is hereby granted without fee, provided that
 * the above copyright notice appear in all copies and that both that
 * copyright notice and this permission notice appear in supporting
 * documentation, and that the name of the author(s) not be used in
 * advertising or publicity pertaining to distribution of the software without
 * specific, written prior permission.  The authors make no
 * representations about the suitability of this software for any purpose.  It
 * is provided "as is" without express or implied warranty.
 *
 * THE AUTHOR(S) DISCLAIMS ALL WARRANTIES WITH REGARD TO THIS SOFTWARE,
 * INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS, IN NO
 * EVENT SHALL THE AUTHOR(S) BE LIABLE FOR ANY SPECIAL, INDIRECT OR
 * CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM LOSS OF USE,
 * DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR OTHER
 * TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR
 * PERFORMANCE OF THIS SOFTWARE.
 */
%{
CUT_OUT_BEGIN
#include <fontconfig/fontconfig.h>
CUT_OUT_END
%}
%struct-type
%language=ANSI-C
%includes
%enum
%readonly-tables
%define slot-name name
%define hash-function-name FcObjectTypeHash
%define lookup-function-name FcObjectTypeLookup

%pic
%define string-pool-name FcObjectTypeNamePool

struct FcObjectTypeInfo {
	int name;
	int id;
};

%%
#define FC_OBJECT(NAME, Type, Cmp) FC_##NAME, FC_##NAME##_OBJECT
#include "fcobjs.h"
#undef FC_OBJECT
