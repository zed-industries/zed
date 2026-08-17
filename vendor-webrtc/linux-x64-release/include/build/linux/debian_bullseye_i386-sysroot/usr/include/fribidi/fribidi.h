/* FriBidi
 * fribidi.h - Unicode bidirectional and Arabic joining/shaping algorithms
 *
 * Author:
 *   Behdad Esfahbod, 2004
 *
 * Copyright (C) 2004 Sharif FarsiWeb, Inc
 * 
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 2.1 of the License, or (at your option) any later version.
 * 
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 * 
 * You should have received a copy of the GNU Lesser General Public License
 * along with this library, in a file named COPYING; if not, write to the
 * Free Software Foundation, Inc., 51 Franklin Street, Fifth Floor,
 * Boston, MA 02110-1301, USA
 * 
 * For licensing issues, contact <fribidi.license@gmail.com>.
 */
#ifndef _FRIBIDI_H
#define _FRIBIDI_H

#include "fribidi-common.h"

#include "fribidi-unicode.h"
#include "fribidi-types.h"
#include "fribidi-flags.h"
#include "fribidi-bidi-types.h"
#include "fribidi-bidi.h"
#include "fribidi-joining-types.h"
#include "fribidi-joining.h"
#include "fribidi-mirroring.h"
#include "fribidi-brackets.h"
#include "fribidi-arabic.h"
#include "fribidi-shape.h"
#include "fribidi-char-sets.h"

#include "fribidi-begindecls.h"

/* fribidi_remove_bidi_marks - remove bidi marks out of an string
 *
 * This function removes the bidi and boundary-neutral marks out of an string
 * and the accompanying lists.  It implements rule X9 of the Unicode
 * Bidirectional Algorithm available at
 * http://www.unicode.org/reports/tr9/#X9, with the exception that it removes
 * U+200E LEFT-TO-RIGHT MARK and U+200F RIGHT-TO-LEFT MARK too.
 *
 * If any of the input lists are NULL, the list is skipped.  If str is the
 * visual string, then positions_to_this is  positions_L_to_V and
 * position_from_this_list is positions_V_to_L;  if str is the logical
 * string, the other way. Moreover, the position maps should be filled with
 * valid entries.
 * 
 * A position map pointing to a removed character is filled with \-1. By the
 * way, you should not use embedding_levels if str is visual string.
 * 
 * For best results this function should be run on a whole paragraph, not
 * lines; but feel free to do otherwise if you know what you are doing.
 *
 * Returns: New length of the string, or \-1 if an error occurred (memory
 * allocation failure most probably).
 */
FRIBIDI_ENTRY FriBidiStrIndex
fribidi_remove_bidi_marks (
  FriBidiChar *str,		/* input string to clean */
  const FriBidiStrIndex len,	/* input string length */
  FriBidiStrIndex *positions_to_this,	/* list mapping positions to the
					   order used in str */
  FriBidiStrIndex *position_from_this_list,	/* list mapping positions from the
						   order used in str */
  FriBidiLevel *embedding_levels	/* list of embedding levels */
);


/* fribidi_log2vis - get visual string
 *
 * This function converts the logical input string to the visual output
 * strings as specified by the Unicode Bidirectional Algorithm.  As a side
 * effect it also generates mapping lists between the two strings, and the
 * list of embedding levels as defined by the algorithm.
 *
 * If NULL is passed as any of the the lists, the list is ignored and not
 * filled.
 *
 * Note that this function handles one-line paragraphs. For multi-
 * paragraph texts it is necessary to first split the text into
 * separate paragraphs and then carry over the resolved pbase_dir
 * between the subsequent invocations.
 *
 * Returns: Maximum level found plus one, or zero if any error occurred
 * (memory allocation failure most probably).
 */
FRIBIDI_ENTRY FriBidiLevel fribidi_log2vis (
  const FriBidiChar *str,	/* input logical string */
  const FriBidiStrIndex len,	/* input string length */
  FriBidiParType *pbase_dir,	/* requested and resolved paragraph
				 * base direction */
  FriBidiChar *visual_str,	/* output visual string */
  FriBidiStrIndex *positions_L_to_V,	/* output mapping from logical to 
					 * visual string positions */
  FriBidiStrIndex *positions_V_to_L,	/* output mapping from visual string
					 * back to the logical string
					 * positions */
  FriBidiLevel *embedding_levels	/* output list of embedding levels */
);

#ifdef FRIBIDI_NO_DEPRECATED
#else
# include "fribidi-deprecated.h"
#endif				/* !FRIBIDI_NO_DEPRECATED */


/* An string containing the version information of the library. */
FRIBIDI_ENTRY const char *fribidi_version_info;

#include "fribidi-enddecls.h"

#endif /* !_FRIBIDI_H */
/* Editor directions:
 * vim:textwidth=78:tabstop=8:shiftwidth=2:autoindent:cindent
 */
