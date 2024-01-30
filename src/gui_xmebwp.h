/* vi:set ts=8 sts=4 sw=4 noet:
 *
 * VIM - Vi IMproved	by Bram Moolenaar
 *
 * Do ":help uganda"  in Vim to read copying and usage conditions.
 * Do ":help credits" in Vim to see a list of people who contributed.
 * See README.txt for an overview of the Vim source code.
 */
/*
 *
 * (C) 2002,2005 by Marcin Dalecki <martin@dalecki.de>
 *
 * MARCIN DALECKI ASSUMES NO RESPONSIBILITY FOR THE USE OR INABILITY TO USE ANY
 * OF THIS SOFTWARE . THIS SOFTWARE IS PROVIDED "AS IS" WITHOUT WARRANTY OF ANY
 * KIND, AND MARCIN DALECKI EXPRESSLY DISCLAIMS ALL IMPLIED WARRANTIES,
 * INCLUDING BUT NOT LIMITED TO THE IMPLIED WARRANTIES OF MERCHANTABILITY AND
 * FITNESS FOR A PARTICULAR PURPOSE.
 */

#ifndef EnhancedBP_H
#define EnhancedBP_H

#include <Xm/PushBP.h>

#include "gui_xmebw.h"


/*
 * EnhancedButton class structure.
 */
typedef struct _XmEnhancedButtonClassPart
{
    Pixmap stipple_bitmap;
} XmEnhancedButtonClassPart;

/*
 * Full class record declaration for EnhancedButton class.
 */
typedef struct
{
    CoreClassPart core_class;
    XmPrimitiveClassPart primitive_class;
    XmLabelClassPart label_class;
    XmPushButtonClassPart pushbutton_class;
    XmEnhancedButtonClassPart enhancedbutton_class;
} XmEnhancedButtonClassRec;


extern XmEnhancedButtonClassRec xmEnhancedButtonClassRec;

/*
 * EnhancedButton instance record.
 */
typedef struct _XmEnhancedButtonPart
{
    // public resources
    String pixmap_data;
    String pixmap_file;
    Dimension spacing;
    int label_location;

    // private resources
    int pixmap_depth;
    Dimension pixmap_width;
    Dimension pixmap_height;
    Pixmap normal_pixmap;
    Pixmap armed_pixmap;
    Pixmap insensitive_pixmap;
    Pixmap highlight_pixmap;

    int doing_setvalues;
    int doing_destroy;
} XmEnhancedButtonPart;


/*
 * Full instance record declaration.
 */
typedef struct _XmEnhancedButtonRec
{
    CorePart core;
    XmPrimitivePart primitive;
    XmLabelPart label;
    XmPushButtonPart pushbutton;
    XmEnhancedButtonPart enhancedbutton;
} XmEnhancedButtonRec;

#endif
