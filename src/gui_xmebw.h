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

#ifndef EnhancedB_H
#define EnhancedB_H

/*
 * New resources for the Extended Pushbutton widget
 */

#ifndef XmNshift
# define XmNshift		"shift"
#endif
#ifndef XmCShift
# define XmCShift		"Shift"
#endif

#ifndef XmNlabelLocation
# define XmNlabelLocation	"labelLocation"
#endif
#ifndef XmCLocation
# define XmCLocation		"Location"
#endif

#ifndef XmNpixmapData
# define XmNpixmapData		"pixmapData"
#endif

#ifndef XmNpixmapFile
# define XmNpixmapFile		"pixmapFile"
#endif

/*
 * Constants for labelLocation.
 */
#ifdef HAVE_XM_JOINSIDET_H
# include <Xm/JoinSideT.h>
#else
# define XmLEFT	    1
# define XmRIGHT    2
# define XmTOP	    3
# define XmBOTTOM   4
#endif

#define XmIsEnhancedButton(w) XtIsSubclass(w, xmEnhancedButtonWidgetClass)

/*
 * Convenience creation function.
 */
extern Widget XgCreateEPushButtonWidget(Widget, char *, ArgList, Cardinal);

extern WidgetClass xmEnhancedButtonWidgetClass;
typedef struct _XmEnhancedButtonClassRec *XmEnhancedButtonWidgetClass;
typedef struct _XmEnhancedButtonRec *XmEnhancedButtonWidget;

#endif
