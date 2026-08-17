/****************************************************************************
 *
 * afadjust.h
 *
 *   Auto-fitter routines to adjust components based on charcode (header).
 *
 * Copyright (C) 2023-2024 by
 * David Turner, Robert Wilhelm, and Werner Lemberg.
 *
 * Written by Craig White <gerzytet@gmail.com>.
 *
 * This file is part of the FreeType project, and may only be used,
 * modified, and distributed under the terms of the FreeType project
 * license, LICENSE.TXT.  By continuing to use, modify, or distribute
 * this file you indicate that you have read the license and
 * understand and accept it fully.
 *
 */


#ifndef AFADJUST_H_
#define AFADJUST_H_

#include <freetype/fttypes.h>

#include "afglobal.h"
#include "aftypes.h"


FT_BEGIN_HEADER


  /* The type of adjustment that should be done to prevent cases where   */
  /* two parts of a character stacked vertically merge, even though they */
  /* should be separate.                                                 */
  typedef enum  AF_VerticalSeparationAdjustmentType_
  {
    /* This means that the hinter should find the topmost contour and push */
    /* it up until its lowest point is one pixel above the highest point   */
    /* not part of that contour.                                           */
    AF_VERTICAL_ADJUSTMENT_TOP_CONTOUR_UP,

    /* This is the opposite direction.  The hinter should find the         */
    /* bottommost contour and push it down until there is a one-pixel gap. */
    AF_VERTICAL_ADJUSTMENT_BOTTOM_CONTOUR_DOWN,

    AF_VERTICAL_ADJUSTMENT_NONE

  } AF_VerticalSeparationAdjustmentType;


  typedef struct  AF_AdjustmentDatabaseEntry_
  {
    FT_UInt32                            codepoint;
    AF_VerticalSeparationAdjustmentType  vertical_separation_adjustment_type;
    FT_Bool                              apply_tilde;

  } AF_AdjustmentDatabaseEntry;


  FT_LOCAL( const AF_AdjustmentDatabaseEntry* )
  af_adjustment_database_lookup( FT_UInt32  codepoint );

  FT_LOCAL( const AF_ReverseMapEntry* )
  af_reverse_character_map_lookup( AF_ReverseCharacterMap  map,
                                   FT_Int                  glyph_index );

  /* Allocate and populate the reverse character map, */
  /* using the character map within the face.         */
  FT_LOCAL( FT_Error )
  af_reverse_character_map_new( AF_ReverseCharacterMap  *map,
                                AF_FaceGlobals           globals );

  /* Free the reverse character map. */
  FT_LOCAL( FT_Error )
  af_reverse_character_map_done( AF_ReverseCharacterMap  map,
                                 FT_Memory               memory );


FT_END_HEADER

#endif /* AFADJUST_H_ */


/* END */
