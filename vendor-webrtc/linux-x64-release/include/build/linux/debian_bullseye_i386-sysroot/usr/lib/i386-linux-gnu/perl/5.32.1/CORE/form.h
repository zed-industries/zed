/*    form.h
 *
 *    Copyright (C) 1991, 1992, 1993, 2000, 2004, 2011 by Larry Wall and others
 *
 *    You may distribute under the terms of either the GNU General Public
 *    License or the Artistic License, as specified in the README file.
 *
 */

#define FF_END          0  /* tidy up, then return */
#define FF_LINEMARK     1  /* start (or end) of a line */
#define FF_LITERAL      2  /* append <arg> literal chars */
#define FF_SKIP         3  /* skip <arg> chars in format */
#define FF_FETCH        4  /* get next item and set field size to <arg> */
#define FF_CHECKNL      5  /* find max len of item (up to \n) that fits field */
#define FF_CHECKCHOP    6  /* like CHECKNL, but up to highest split point */
#define FF_SPACE        7  /* append padding space (diff of field, item size) */
#define FF_HALFSPACE    8  /* like FF_SPACE, but only append half as many */
#define FF_ITEM         9  /* append a text item, while blanking ctrl chars */
#define FF_CHOP         10 /* (for ^*) chop the current item */
#define FF_LINEGLOB     11 /* process @*  */
#define FF_DECIMAL      12 /* do @##, ^##, where <arg>=(precision|flags) */
#define FF_NEWLINE      13 /* delete trailing spaces, then append \n */
#define FF_BLANK        14 /* for arg==0: do '~'; for arg>0 : do '~~' */
#define FF_MORE         15 /* replace long end of string with '...' */
#define FF_0DECIMAL     16 /* like FF_DECIMAL but for 0### */
#define FF_LINESNGL     17 /* process ^*  */
