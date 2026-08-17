/*

Copyright 1987, 1998  The Open Group

Permission to use, copy, modify, distribute, and sell this software and its
documentation for any purpose is hereby granted without fee, provided that
the above copyright notice appear in all copies and that both that
copyright notice and this permission notice appear in supporting
documentation.

The above copyright notice and this permission notice shall be included
in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS
OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
IN NO EVENT SHALL THE OPEN GROUP BE LIABLE FOR ANY CLAIM, DAMAGES OR
OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
OTHER DEALINGS IN THE SOFTWARE.

Except as contained in this notice, the name of The Open Group shall
not be used in advertising or otherwise to promote the sale, use or
other dealings in this Software without prior written authorization
from The Open Group.

Copyright 1987 by Digital Equipment Corporation, Maynard, Massachusetts,

                        All Rights Reserved

Permission to use, copy, modify, and distribute this software and its
documentation for any purpose and without fee is hereby granted,
provided that the above copyright notice appear in all copies and that
both that copyright notice and this permission notice appear in
supporting documentation, and that the names of Hewlett Packard
or Digital not be
used in advertising or publicity pertaining to distribution of the
software without specific, written prior permission.

DIGITAL DISCLAIMS ALL WARRANTIES WITH REGARD TO THIS SOFTWARE, INCLUDING
ALL IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS, IN NO EVENT SHALL
DIGITAL BE LIABLE FOR ANY SPECIAL, INDIRECT OR CONSEQUENTIAL DAMAGES OR
ANY DAMAGES WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS,
WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION,
ARISING OUT OF OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS
SOFTWARE.

HEWLETT-PACKARD MAKES NO WARRANTY OF ANY KIND WITH REGARD
TO THIS SOFWARE, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
PURPOSE.  Hewlett-Packard shall not be liable for errors
contained herein or direct, indirect, special, incidental or
consequential damages in connection with the furnishing,
performance, or use of this material.

*/

#ifndef _HPKEYSYM_H

#define _HPKEYSYM_H

#define hpXK_ClearLine		0x1000FF6F
#define hpXK_InsertLine		0x1000FF70
#define hpXK_DeleteLine		0x1000FF71
#define hpXK_InsertChar		0x1000FF72
#define hpXK_DeleteChar		0x1000FF73
#define hpXK_BackTab		0x1000FF74
#define hpXK_KP_BackTab		0x1000FF75
#define hpXK_Modelock1		0x1000FF48
#define hpXK_Modelock2		0x1000FF49
#define hpXK_Reset		0x1000FF6C
#define hpXK_System		0x1000FF6D
#define hpXK_User		0x1000FF6E
#define hpXK_mute_acute		0x100000A8
#define hpXK_mute_grave		0x100000A9
#define hpXK_mute_asciicircum	0x100000AA
#define hpXK_mute_diaeresis	0x100000AB
#define hpXK_mute_asciitilde	0x100000AC
#define hpXK_lira		0x100000AF
#define hpXK_guilder		0x100000BE
#define hpXK_Ydiaeresis		0x100000EE
#define hpXK_IO			0x100000EE
#define hpXK_longminus		0x100000F6
#define hpXK_block		0x100000FC


#ifndef _OSF_Keysyms
#define _OSF_Keysyms

#define osfXK_Copy		0x1004FF02
#define osfXK_Cut		0x1004FF03
#define osfXK_Paste		0x1004FF04
#define osfXK_BackTab		0x1004FF07
#define osfXK_BackSpace		0x1004FF08
#define osfXK_Clear		0x1004FF0B
#define osfXK_Escape		0x1004FF1B
#define osfXK_AddMode		0x1004FF31
#define osfXK_PrimaryPaste	0x1004FF32
#define osfXK_QuickPaste	0x1004FF33
#define osfXK_PageLeft		0x1004FF40
#define osfXK_PageUp		0x1004FF41
#define osfXK_PageDown		0x1004FF42
#define osfXK_PageRight		0x1004FF43
#define osfXK_Activate		0x1004FF44
#define osfXK_MenuBar		0x1004FF45
#define osfXK_Left		0x1004FF51
#define osfXK_Up		0x1004FF52
#define osfXK_Right		0x1004FF53
#define osfXK_Down		0x1004FF54
#define osfXK_EndLine		0x1004FF57
#define osfXK_BeginLine		0x1004FF58
#define osfXK_EndData		0x1004FF59
#define osfXK_BeginData		0x1004FF5A
#define osfXK_PrevMenu		0x1004FF5B
#define osfXK_NextMenu		0x1004FF5C
#define osfXK_PrevField		0x1004FF5D
#define osfXK_NextField		0x1004FF5E
#define osfXK_Select		0x1004FF60
#define osfXK_Insert		0x1004FF63
#define osfXK_Undo		0x1004FF65
#define osfXK_Menu		0x1004FF67
#define osfXK_Cancel		0x1004FF69
#define osfXK_Help		0x1004FF6A
#define osfXK_SelectAll		0x1004FF71
#define osfXK_DeselectAll	0x1004FF72
#define osfXK_Reselect		0x1004FF73
#define osfXK_Extend		0x1004FF74
#define osfXK_Restore		0x1004FF78
#define osfXK_Delete		0x1004FFFF

#endif /* _OSF_Keysyms */


/**************************************************************
 * The use of the following macros is deprecated.
 * They are listed below only for backwards compatibility.
 */
#define XK_Reset                0x1000FF6C
#define XK_System               0x1000FF6D
#define XK_User                 0x1000FF6E
#define XK_ClearLine            0x1000FF6F
#define XK_InsertLine           0x1000FF70
#define XK_DeleteLine           0x1000FF71
#define XK_InsertChar           0x1000FF72
#define XK_DeleteChar           0x1000FF73
#define XK_BackTab              0x1000FF74
#define XK_KP_BackTab           0x1000FF75
#define XK_Ext16bit_L           0x1000FF76
#define XK_Ext16bit_R           0x1000FF77
#define XK_mute_acute           0x100000a8
#define XK_mute_grave           0x100000a9
#define XK_mute_asciicircum     0x100000aa
#define XK_mute_diaeresis       0x100000ab
#define XK_mute_asciitilde      0x100000ac
#define XK_lira                 0x100000af
#define XK_guilder              0x100000be
#ifndef XK_Ydiaeresis
#define XK_Ydiaeresis           0x100000ee
#endif
#define XK_IO                   0x100000ee
#define XK_longminus            0x100000f6
#define XK_block                0x100000fc

#endif /* _HPKEYSYM_H */
