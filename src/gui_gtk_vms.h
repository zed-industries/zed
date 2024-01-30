/* vi:set ts=8 sts=4 sw=4 noet:
 *
 * VIM - Vi IMproved    by Bram Moolenaar
 *
 * Do ":help uganda"  in Vim to read copying and usage conditions.
 * Do ":help credits" in Vim to see a list of people who contributed.
 *
 * File MOTIF_REDEFINES.H originally delivers together with
 * OpenVMS Porting Library
 * http://www.openvms.compaq.com/openvms/products/ips/porting.html
 *
 * This file has been modified for Vim development.
 * Original file contains just defines that GTK for OpenVMS uses,
 * but not all functions that DECW library has. Therefore it has been expanded
 * with necessary defines for Vim on OpenVMS with GTK GUI.
 *
 * Zoltan Arpadffy <arpadffy@polarhome.com>
 */

/*
 *************************************************************************
 *									 *
 * Copyright 2000 Compaq Computer Corporation				 *
 *									 *
 * COMPAQ Registered in U.S. Patent and Trademark Office.		 *
 *									 *
 *************************************************************************
 * IMPORTANT: Carefully read the License Terms below before		 *
 * proceeding.	By use of these materials you agree to these terms.	 *
 * If you do not agree to these terms, you may not use this software or  *
 * the accompanying documentation.					 *
 *************************************************************************
 * LICENSE TERMS							 *
 * 1. GRANT								 *
 * Compaq Computer Corporation ("COMPAQ") grants you the right to use,	 *
 * modify, and distribute the following source code (the "Software")	 *
 * on any number of computers. You may use the Software as part of	 *
 * creating a software program or product intended for commercial or	 *
 * non-commercial distribution in machine-readable source code, binary,  *
 * or executable formats. You may distribute the Software as		 *
 * machine-readable source code provided this license is not removed	 *
 * from the Software and any modifications are conspicuously indicated.  *
 * 2. COPYRIGHT								 *
 * The Software is owned by COMPAQ and its suppliers and is protected by *
 * copyright laws and international treaties.  Your use of the Software  *
 * and associated documentation is subject to the applicable copyright	 *
 * laws and the express rights and restrictions of these terms.		 *
 * 3. RESTRICTIONS							 *
 * You may not remove any copyright, trademark, or other proprietary	 *
 * notices from the Software or the associated	documentation.		 *
 * You are responsible for compliance with all applicable export or	 *
 * re-export control laws and regulations if you export the Software.	 *
 * This license is governed by and is to be construed under the laws	 *
 * of the State of Texas.						 *
 *									 *
 * DISCLAIMER OF WARRANTY AND LIABILITY					 *
 * Compaq shall not be liable for technical or editorial errors or	 *
 * omissions contained herein. The information contained herein is	 *
 * subject to change without notice.					 *
 *									 *
 * THIS SOFTWARE IS PROVIDED "AS IS" WITHOUT WARRANTY OF ANY KIND.	 *
 * THE ENTIRE RISK ARISING OUT OF THE USE OF THIS SOFTWARE REMAINS WITH  *
 * RECIPIENT.  IN NO EVENT SHALL COMPAQ BE LIABLE FOR ANY DIRECT,	 *
 * CONSEQUENTIAL, INCIDENTAL, SPECIAL, PUNITIVE OR OTHER DAMAGES	 *
 * WHATSOEVER (INCLUDING WITHOUT LIMITATION DAMAGES FOR LOSS OF BUSINESS *
 * PROFITS, BUSINESS INTERRUPTION, OR LOSS OF BUSINESS INFORMATION),	 *
 * EVEN IF COMPAQ HAS BEEN ADVISED OF THE POSSIBILITY OF SUCH DAMAGES	 *
 * AND WHETHER IN AN ACTION OF CONTRACT OR TORT INCLUDING NEGLIGENCE.	 *
 *									 *
 * If you have any questions concerning this license, please contact:	 *
 * Compaq Computer Corporation, Software Business Practices, ZKO1-2/D22, *
 * 110 Spit Brook Road, Nashua, NH. 03062-2698.				 *
 *									 *
 *************************************************************************
 */

// INTRINSIC.H omits proto if XtFree is defined
// VMS_BEGIN_C_PLUS_PLUS
extern void XtFree(char*);
extern void XTFREE(char*);
// VMS_END_C_PLUS_PLUS

#define _XRegisterFilterByType _XREGISTERFILTERBYTYPE

#define XAllocClassHint XALLOCCLASSHINT
#define XAllocColor XALLOCCOLOR
#define XAllocColorCells XALLOCCOLORCELLS
#define XAllocSizeHints XALLOCSIZEHINTS
#define XAllocWMHints XALLOCWMHINTS
#define XAutoRepeatOff XAUTOREPEATOFF
#define XAutoRepeatOn XAUTOREPEATON
#define XBaseFontNameListOfFontSet XBASEFONTNAMELISTOFFONTSET
#define XBell XBELL
#define XBitmapPad XBITMAPPAD
#define XChangeActivePointerGrab XCHANGEACTIVEPOINTERGRAB
#define XChangeGC XCHANGEGC
#define XChangeProperty XCHANGEPROPERTY
#define XChangeWindowAttributes XCHANGEWINDOWATTRIBUTES
#define XCheckIfEvent XCHECKIFEVENT
#define XCheckMaskEvent XCHECKMASKEVENT
#define XCheckTypedEvent XCHECKTYPEDEVENT
#define XCheckTypedWindowEvent XCHECKTYPEDWINDOWEVENT
#define XCheckWindowEvent XCHECKWINDOWEVENT
#define XClearArea XCLEARAREA
#define XClearWindow XCLEARWINDOW
#define XClipBox XCLIPBOX
#define XCloseDisplay XCLOSEDISPLAY
#define XCloseIM XCLOSEIM
#define XConfigureWindow XCONFIGUREWINDOW
#define XConvertSelection XCONVERTSELECTION
#define XCopyArea XCOPYAREA
#define XCopyGC XCOPYGC
#define XCopyPlane XCOPYPLANE
#define XCreateBitmapFromData XCREATEBITMAPFROMDATA
#define XCreateColormap XCREATECOLORMAP
#define XCreateFontCursor XCREATEFONTCURSOR
#define XCreateFontSet XCREATEFONTSET
#define XCreateGC XCREATEGC
#define XCreateIC XCREATEIC
#define XCreateImage XCREATEIMAGE
#define XCreatePixmap XCREATEPIXMAP
#define XCreatePixmapCursor XCREATEPIXMAPCURSOR
#define XCreatePixmapFromBitmapData XCREATEPIXMAPFROMBITMAPDATA
#define XCreateRegion XCREATEREGION
#define XCreateSimpleWindow XCREATESIMPLEWINDOW
#define XCreateWindow XCREATEWINDOW
#define XDefaultRootWindow XDEFAULTROOTWINDOW
#define XDefaultScreenOfDisplay XDEFAULTSCREENOFDISPLAY
#define XDefineCursor XDEFINECURSOR
#define XDeleteProperty XDELETEPROPERTY
#define XDestroyIC XDESTROYIC
#define XDestroyRegion XDESTROYREGION
#define XDestroyWindow XDESTROYWINDOW
#define XDisplayName XDISPLAYNAME
#define XDisplayOfScreen XDISPLAYOFSCREEN
#define XDisplayString XDISPLAYSTRING
#define XDrawArc XDRAWARC
#define XDrawImageString XDRAWIMAGESTRING
#define XDrawImageString16 XDRAWIMAGESTRING16
#define XDrawLine XDRAWLINE
#define XDrawLines XDRAWLINES
#define XDrawPoint XDRAWPOINT
#define XDrawPoints XDRAWPOINTS
#define XDrawRectangle XDRAWRECTANGLE
#define XDrawSegments XDRAWSEGMENTS
#define XDrawString XDRAWSTRING
#define XDrawString16 XDRAWSTRING16
#define XEmptyRegion XEMPTYREGION
#define XEqualRegion XEQUALREGION
#define XEventsQueued XEVENTSQUEUED
#define XExtentsOfFontSet XEXTENTSOFFONTSET
#define XFetchBuffer XFETCHBUFFER
#define XFillArc XFILLARC
#define XFillPolygon XFILLPOLYGON
#define XFillRectangle XFILLRECTANGLE
#define XFillRectangles XFILLRECTANGLES
#define XFilterEvent XFILTEREVENT
#define XFlush XFLUSH
#define XFontsOfFontSet XFONTSOFFONTSET
#define XFree XFREE
#define XFreeColormap XFREECOLORMAP
#define XFreeColors XFREECOLORS
#define XFreeCursor XFREECURSOR
#define XFreeFont XFREEFONT
#define XFreeFontInfo XFREEFONTINFO
#define XFreeFontNames XFREEFONTNAMES
#define XFreeFontSet XFREEFONTSET
#define XFreeGC XFREEGC
#define XFreeModifiermap XFREEMODIFIERMAP
#define XFreePixmap XFREEPIXMAP
#define XFreeStringList XFREESTRINGLIST
#define XGetAtomName XGETATOMNAME
#define XGetDefault XGETDEFAULT
#define XGetErrorDatabaseText XGETERRORDATABASETEXT
#define XGetErrorText XGETERRORTEXT
#define XGetFontProperty XGETFONTPROPERTY
#define XGetGCValues XGETGCVALUES
#define XGetGeometry XGETGEOMETRY
#define XGetIconSizes XGETICONSIZES
#define XGetICValues XGETICVALUES
#define XGetIMValues XGETIMVALUES
#define XGetImage XGETIMAGE
#define XGetKeyboardControl XGETKEYBOARDCONTROL
#define XGetModifierMapping XGETMODIFIERMAPPING
#define XGetMotionEvents XGETMOTIONEVENTS
#define XGetNormalHints XGETNORMALHINTS
#define XGetSelectionOwner XGETSELECTIONOWNER
#define XGetSubImage XGETSUBIMAGE
#define XGetVisualInfo XGETVISUALINFO
#define XGetWMColormapWindows XGETWMCOLORMAPWINDOWS
#define XGetWMIconName XGETWMICONNAME
#define XGetWMProtocols XGETWMPROTOCOLS
#define XGetWMHints XGETWMHINTS
#define XGetWMName XGETWMNAME
#define XGetWMNormalHints XGETWMNORMALHINTS
#define XGetWindowAttributes XGETWINDOWATTRIBUTES
#define XGetWindowProperty XGETWINDOWPROPERTY
#define XGrabKeyboard XGRABKEYBOARD
#define XGrabPointer XGRABPOINTER
#define XGrabServer XGRABSERVER
#define XHeightOfScreen XHEIGHTOFSCREEN
#define XIconifyWindow XICONIFYWINDOW
#define XIfEvent XIFEVENT
#define XInternAtom XINTERNATOM
#define XIntersectRegion XINTERSECTREGION
#define XKeycodeToKeysym XKEYCODETOKEYSYM
#define XKeysymToKeycode XKEYSYMTOKEYCODE
#define XKeysymToString XKEYSYMTOSTRING
#define XListFonts XLISTFONTS
#define XListFontsWithInfo XLISTFONTSWITHINFO
#define XListPixmapFormats XLISTPIXMAPFORMATS
#define XListProperties XLISTPROPERTIES
#define XLoadQueryFont XLOADQUERYFONT
#define XLookupString XLOOKUPSTRING
#define XLowerWindow XLOWERWINDOW
#define XMapRaised XMAPRAISED
#define XMapWindow XMAPWINDOW
#define XMatchVisualInfo XMATCHVISUALINFO
#define XMoveResizeWindow XMOVERESIZEWINDOW
#define XMoveWindow XMOVEWINDOW
#define XNextEvent XNEXTEVENT
#define XOffsetRegion XOFFSETREGION
#define XOpenDisplay XOPENDISPLAY
#define XOpenIM XOPENIM
#define XParseColor XPARSECOLOR
#define XParseGeometry XPARSEGEOMETRY
#define XPeekEvent XPEEKEVENT
#define XPending XPENDING
#define XPointInRegion XPOINTINREGION
#define XPolygonRegion XPOLYGONREGION
#define XPutBackEvent XPUTBACKEVENT
#define XPutImage XPUTIMAGE
#define XRootWindow XROOTWINDOW
#define XQueryColor XQUERYCOLOR
#define XQueryColors XQUERYCOLORS
#define XQueryExtension XQUERYEXTENSION
#define XQueryPointer XQUERYPOINTER
#define XQueryTree XQUERYTREE
#define XRaiseWindow XRAISEWINDOW
#define XReconfigureWMWindow XRECONFIGUREWMWINDOW
#define XRectInRegion XRECTINREGION
#define XRefreshKeyboardMapping XREFRESHKEYBOARDMAPPING
#define XReparentWindow XREPARENTWINDOW
#define XResizeWindow XRESIZEWINDOW
#define XRestackWindows XRESTACKWINDOWS
#define XRootWindowOfScreen XROOTWINDOWOFSCREEN
#define XScreenNumberOfScreen XSCREENNUMBEROFSCREEN
#define XSelectAsyncEvent XSELECTASYNCEVENT
#define XSelectAsyncInput XSELECTASYNCINPUT
#define XSelectInput XSELECTINPUT
#define XSendEvent XSENDEVENT
#define XServerVendor XSERVERVENDOR
#define XSetBackground XSETBACKGROUND
#define XSetClassHint XSETCLASSHINT
#define XSetClipMask XSETCLIPMASK
#define XSetClipOrigin XSETCLIPORIGIN
#define XSetClipRectangles XSETCLIPRECTANGLES
#define XSetCloseDownMode XSETCLOSEDOWNMODE
#define XSetCommand XSETCOMMAND
#define XSetDashes XSETDASHES
#define XSetErrorHandler XSETERRORHANDLER
#define XSetFillStyle XSETFILLSTYLE
#define XSetFont XSETFONT
#define XSetForeground XSETFOREGROUND
#define XSetFunction XSETFUNCTION
#define XSetGraphicsExposures XSETGRAPHICSEXPOSURES
#define XSetICFocus XSETICFOCUS
#define XSetICValues XSETICVALUES
#define XSetIOErrorHandler XSETIOERRORHANDLER
#define XSetInputFocus XSETINPUTFOCUS
#define XSetLineAttributes XSETLINEATTRIBUTES
#define XSetLocaleModifiers XSETLOCALEMODIFIERS
#define XSetNormalHints XSETNORMALHINTS
#define XSetRegion XSETREGION
#define XSetSelectionOwner XSETSELECTIONOWNER
#define XSetStipple XSETSTIPPLE
#define XSetSubwindowMode XSETSUBWINDOWMODE
#define XSetTSOrigin XSETTSORIGIN
#define XSetTile XSETTILE
#define XSetTransientForHint XSETTRANSIENTFORHINT
#define XSetWMColormapWindows XSETWMCOLORMAPWINDOWS
#define XSetWMHints XSETWMHINTS
#define XSetWMIconName XSETWMICONNAME
#define XSetWMName XSETWMNAME
#define XSetWMNormalHints XSETWMNORMALHINTS
#define XSetWMProperties XSETWMPROPERTIES
#define XSetWMProtocols XSETWMPROTOCOLS
#define XSetWindowBackground XSETWINDOWBACKGROUND
#define XSetWindowBackgroundPixmap XSETWINDOWBACKGROUNDPIXMAP
#define XSetWindowColormap XSETWINDOWCOLORMAP
#define XShapeCombineMask XSHAPECOMBINEMASK
#define XShapeCombineRectangles XSHAPECOMBINERECTANGLES
#define XShapeGetRectangles XSHAPEGETRECTANGLES
#define XShrinkRegion XSHRINKREGION
#define XStoreBuffer XSTOREBUFFER
#define XStoreColor XSTORECOLOR
#define XStoreColors XSTORECOLORS
#define XStoreName XSTORENAME
#define XStringToKeysym XSTRINGTOKEYSYM
#define XSubtractRegion XSUBTRACTREGION
#define XSupportsLocale XSUPPORTSLOCALE
#define XSync XSYNC
#define XSynchronize XSYNCHRONIZE
#define XTextExtents XTEXTEXTENTS
#define XTextExtents16 XTEXTEXTENTS16
#define XTextWidth XTEXTWIDTH
#define XTextWidth16 XTEXTWIDTH16
#define XTranslateCoordinates XTRANSLATECOORDINATES
#define XUndefineCursor XUNDEFINECURSOR
#define XUngrabKeyboard XUNGRABKEYBOARD
#define XUngrabPointer XUNGRABPOINTER
#define XUngrabServer XUNGRABSERVER
#define XUnionRectWithRegion XUNIONRECTWITHREGION
#define XUnionRegion XUNIONREGION
#define XUnmapWindow XUNMAPWINDOW
#define _XUnregisterFilter _XUNREGISTERFILTER
#define XUnsetICFocus XUNSETICFOCUS
#define XVaCreateNestedList XVACREATENESTEDLIST
#define XVisualIDFromVisual XVISUALIDFROMVISUAL
#define XWarpPointer XWARPPOINTER
#define XWidthOfScreen XWIDTHOFSCREEN
#define XWindowEvent XWINDOWEVENT
#define XWithdrawWindow XWITHDRAWWINDOW
#define XXorRegion XXORREGION
#define XmAddProtocolCallback XMADDPROTOCOLCALLBACK
#define XmAddProtocols XMADDPROTOCOLS
#define XmChangeColor XMCHANGECOLOR
#define XmClipboardCopy XMCLIPBOARDCOPY
#define XmClipboardEndCopy XMCLIPBOARDENDCOPY
#define XmClipboardInquireLength XMCLIPBOARDINQUIRELENGTH
#define XmClipboardLock XMCLIPBOARDLOCK
#define XmClipboardRetrieve XMCLIPBOARDRETRIEVE
#define XmClipboardStartCopy XMCLIPBOARDSTARTCOPY
#define XmClipboardUnlock XMCLIPBOARDUNLOCK
#define XmCreateArrowButton XMCREATEARROWBUTTON
#define XmCreateArrowButtonGadget XMCREATEARROWBUTTONGADGET
#define XmCreateCascadeButton XMCREATECASCADEBUTTON
#define XmCreateDialogShell XMCREATEDIALOGSHELL
#define XmCreateDragIcon XMCREATEDRAGICON
#define XmCreateDrawingArea XMCREATEDRAWINGAREA
#define XmCreateDrawnButton XMCREATEDRAWNBUTTON
#define XmCreateFileSelectionBox XMCREATEFILESELECTIONBOX
#define XmCreateFileSelectionDialog XMCREATEFILESELECTIONDIALOG
#define XmCreateForm XMCREATEFORM
#define XmCreateFormDialog XMCREATEFORMDIALOG
#define XmCreateFrame XMCREATEFRAME
#define XmCreateInformationDialog XMCREATEINFORMATIONDIALOG
#define XmCreateLabelGadget XMCREATELABELGADGET
#define XmCreateMainWindow XMCREATEMAINWINDOW
#define XmCreateMenuBar XMCREATEMENUBAR
#define XmCreateMessageBox XMCREATEMESSAGEBOX
#define XmCreateMessageDialog XMCREATEMESSAGEDIALOG
#define XmCreateOptionMenu XMCREATEOPTIONMENU
#define XmCreatePanedWindow XMCREATEPANEDWINDOW
#define XmCreatePopupMenu XMCREATEPOPUPMENU
#define XmCreatePromptDialog XMCREATEPROMPTDIALOG
#define XmCreatePulldownMenu XMCREATEPULLDOWNMENU
#define XmCreatePushButton XMCREATEPUSHBUTTON
#define XmCreatePushButtonGadget XMCREATEPUSHBUTTONGADGET
#define XmCreateQuestionDialog XMCREATEQUESTIONDIALOG
#define XmCreateRadioBox XMCREATERADIOBOX
#define XmCreateRowColumn XMCREATEROWCOLUMN
#define XmCreateScale XMCREATESCALE
#define XmCreateScrollBar XMCREATESCROLLBAR
#define XmCreateScrolledList XMCREATESCROLLEDLIST
#define XmCreateScrolledText XMCREATESCROLLEDTEXT
#define XmCreateScrolledWindow XMCREATESCROLLEDWINDOW
#define XmCreateSelectionDialog XMCREATESELECTIONDIALOG
#define XmCreateSeparator XMCREATESEPARATOR
#define XmCreateSeparatorGadget XMCREATESEPARATORGADGET
#define XmCreateTemplateDialog XMCREATETEMPLATEDIALOG
#define XmCreateText XMCREATETEXT
#define XmCreateTextField XMCREATETEXTFIELD
#define XmCreateToggleButton XMCREATETOGGLEBUTTON
#define XmCreateToggleButtonGadget XMCREATETOGGLEBUTTONGADGET
#define XmDragStart XMDRAGSTART
#define XmDropSiteRegister XMDROPSITEREGISTER
#define XmDropSiteUnregister XMDROPSITEUNREGISTER
#define XmDropSiteUpdate XMDROPSITEUPDATE
#define XmDropTransferStart XMDROPTRANSFERSTART
#define XmFileSelectionBoxGetChild XMFILESELECTIONBOXGETCHILD
#define XmFileSelectionDoSearch XMFILESELECTIONDOSEARCH
#define XmFontListAppendEntry XMFONTLISTAPPENDENTRY
#define XmFontListCopy XMFONTLISTCOPY
#define XmFontListCreate XMFONTLISTCREATE
#define XmFontListEntryCreate XMFONTLISTENTRYCREATE
#define XmFontListEntryFree XMFONTLISTENTRYFREE
#define XmFontListEntryGetFont XMFONTLISTENTRYGETFONT
#define XmFontListEntryGetTag XMFONTLISTENTRYGETTAG
#define XmFontListEntryLoad XMFONTLISTENTRYLOAD
#define XmFontListFree XMFONTLISTFREE
#define XmFontListFreeFontContext XMFONTLISTFREEFONTCONTEXT
#define XmFontListGetNextFont XMFONTLISTGETNEXTFONT
#define XmFontListInitFontContext XMFONTLISTINITFONTCONTEXT
#define XmFontListNextEntry XMFONTLISTNEXTENTRY
#define XmGetColors XMGETCOLORS
#define XmGetFocusWidget XMGETFOCUSWIDGET
#define XmGetMenuCursor XMGETMENUCURSOR
#define XmGetPixmapByDepth XMGETPIXMAPBYDEPTH
#define XmGetTearOffControl XMGETTEAROFFCONTROL
#define XmGetXmDisplay XMGETXMDISPLAY
#define XmImMbLookupString XMIMMBLOOKUPSTRING
#define XmImRegister XMIMREGISTER
#define XmImSetFocusValues XMIMSETFOCUSVALUES
#define XmImSetValues XMIMSETVALUES
#define XmImUnregister XMIMUNREGISTER
#define XmImUnsetFocus XMIMUNSETFOCUS
#define XmInternAtom XMINTERNATOM
#define XmIsMotifWMRunning XMISMOTIFWMRUNNING
#define XmListAddItem XMLISTADDITEM
#define XmListAddItemUnselected XMLISTADDITEMUNSELECTED
#define XmListAddItemsUnselected XMLISTADDITEMSUNSELECTED
#define XmListDeleteAllItems XMLISTDELETEALLITEMS
#define XmListDeleteItemsPos XMLISTDELETEITEMSPOS
#define XmListDeletePos XMLISTDELETEPOS
#define XmListDeselectAllItems XMLISTDESELECTALLITEMS
#define XmListDeselectPos XMLISTDESELECTPOS
#define XmListGetKbdItemPos XMLISTGETKBDITEMPOS
#define XmListGetMatchPos XMLISTGETMATCHPOS
#define XmListGetSelectedPos XMLISTGETSELECTEDPOS
#define XmListPosSelected XMLISTPOSSELECTED
#define XmListSelectItem XMLISTSELECTITEM
#define XmListSelectPos XMLISTSELECTPOS
#define XmListSetBottomPos XMLISTSETBOTTOMPOS
#define XmListSetItem XMLISTSETITEM
#define XmListSetKbdItemPos XMLISTSETKBDITEMPOS
#define XmListSetPos XMLISTSETPOS
#define XmMainWindowSetAreas XMMAINWINDOWSETAREAS
#define XmMenuPosition XMMENUPOSITION
#define XmMessageBoxGetChild XMMESSAGEBOXGETCHILD
#define XmOptionButtonGadget XMOPTIONBUTTONGADGET
#define XmOptionLabelGadget XMOPTIONLABELGADGET
#define XmProcessTraversal XMPROCESSTRAVERSAL
#define XmQmotif XMQMOTIF
#define XmRemoveProtocolCallback XMREMOVEPROTOCOLCALLBACK
#define XmRepTypeGetId XMREPTYPEGETID
#define XmRepTypeGetRecord XMREPTYPEGETRECORD
#define XmRepTypeInstallTearOffModelCon XMREPTYPEINSTALLTEAROFFMODELCON
#define XmRepTypeRegister XMREPTYPEREGISTER
#define XmRepTypeValidValue XMREPTYPEVALIDVALUE
#define XmScrollBarSetValues XMSCROLLBARSETVALUES
#define XmScrolledWindowSetAreas XMSCROLLEDWINDOWSETAREAS
#define XmSelectionBoxGetChild XMSELECTIONBOXGETCHILD
#define XmStringByteCompare XMSTRINGBYTECOMPARE
#define XmStringCompare XMSTRINGCOMPARE
#define XmStringConcat XMSTRINGCONCAT
#define XmStringCopy XMSTRINGCOPY
#define XmStringCreate XMSTRINGCREATE
#define XmStringCreateLocalized XMSTRINGCREATELOCALIZED
#define XmStringCreateLtoR XMSTRINGCREATELTOR
#define XmStringCreateSimple XMSTRINGCREATESIMPLE
#define XmStringDraw XMSTRINGDRAW
#define XmStringDrawUnderline XMSTRINGDRAWUNDERLINE
#define XmStringExtent XMSTRINGEXTENT
#define XmStringFree XMSTRINGFREE
#define XmStringFreeContext XMSTRINGFREECONTEXT
#define XmStringGetLtoR XMSTRINGGETLTOR
#define XmStringGetNextComponent XMSTRINGGETNEXTCOMPONENT
#define XmStringGetNextSegment XMSTRINGGETNEXTSEGMENT
#define XmStringInitContext XMSTRINGINITCONTEXT
#define XmStringLength XMSTRINGLENGTH
#define XmStringLtoRCreate XMSTRINGLTORCREATE
#define XmStringNConcat XMSTRINGNCONCAT
#define XmStringSegmentCreate XMSTRINGSEGMENTCREATE
#define XmStringWidth XMSTRINGWIDTH
#define XmTextClearSelection XMTEXTCLEARSELECTION
#define XmTextFieldGetEditable XMTEXTFIELDGETEDITABLE
#define XmTextFieldGetInsertionPosition XMTEXTFIELDGETINSERTIONPOSITION
#define XmTextFieldGetLastPosition XMTEXTFIELDGETLASTPOSITION
#define XmTextFieldGetSelection XMTEXTFIELDGETSELECTION
#define XmTextFieldGetString XMTEXTFIELDGETSTRING
#define XmTextFieldInsert XMTEXTFIELDINSERT
#define XmTextFieldRemove XMTEXTFIELDREMOVE
#define XmTextFieldSetSelection XMTEXTFIELDSETSELECTION
#define XmTextFieldSetString XMTEXTFIELDSETSTRING
#define XmTextGetCursorPosition XMTEXTGETCURSORPOSITION
#define XmTextGetInsertionPosition XMTEXTGETINSERTIONPOSITION
#define XmTextGetLastPosition XMTEXTGETLASTPOSITION
#define XmTextGetMaxLength XMTEXTGETMAXLENGTH
#define XmTextGetSelection XMTEXTGETSELECTION
#define XmTextGetSelectionPosition XMTEXTGETSELECTIONPOSITION
#define XmTextGetString XMTEXTGETSTRING
#define XmTextInsert XMTEXTINSERT
#define XmTextRemove XMTEXTREMOVE
#define XmTextReplace XMTEXTREPLACE
#define XmTextSetCursorPosition XMTEXTSETCURSORPOSITION
#define XmTextSetHighlight XMTEXTSETHIGHLIGHT
#define XmTextSetInsertionPosition XMTEXTSETINSERTIONPOSITION
#define XmTextSetSelection XMTEXTSETSELECTION
#define XmTextSetString XMTEXTSETSTRING
#define XmToggleButtonGadgetGetState XMTOGGLEBUTTONGADGETGETSTATE
#define XmToggleButtonGadgetSetState XMTOGGLEBUTTONGADGETSETSTATE
#define XmToggleButtonGetState XMTOGGLEBUTTONGETSTATE
#define XmToggleButtonSetState XMTOGGLEBUTTONSETSTATE
#define XmUpdateDisplay XMUPDATEDISPLAY
#define XmVaCreateSimpleRadioBox XMVACREATESIMPLERADIOBOX
#define XmbDrawString XMBDRAWSTRING
#define XmbLookupString XMBLOOKUPSTRING
#define XmbResetIC XMBRESETIC
#define XmbSetWMProperties XMBSETWMPROPERTIES
#define XmbTextEscapement XMBTEXTESCAPEMENT
#define XmbTextExtents XMBTEXTEXTENTS
#define XmbTextListToTextProperty XMBTEXTLISTTOTEXTPROPERTY
#define XmbTextPropertyToTextList XMBTEXTPROPERTYTOTEXTLIST
#define XmuClientWindow XMUCLIENTWINDOW
#define XmuPrintDefaultErrorMessage XMUPRINTDEFAULTERRORMESSAGE
#define XrmGetDatabase XRMGETDATABASE
#define XrmGetResource XRMGETRESOURCE
#define XrmPutStringResource XRMPUTSTRINGRESOURCE
#define XrmQuarkToString XRMQUARKTOSTRING
#define XrmStringToQuark XRMSTRINGTOQUARK
#define XtAddCallback XTADDCALLBACK
#define XtAddCallbacks XTADDCALLBACKS
#define XtAddEventHandler XTADDEVENTHANDLER
#define XtAddGrab XTADDGRAB
#define XtAllocateGC XTALLOCATEGC
#define XtAppAddActions XTAPPADDACTIONS
#define XtAppAddInput XTAPPADDINPUT
#define XtAppAddTimeOut XTAPPADDTIMEOUT
#define XtAppCreateShell XTAPPCREATESHELL
#define XtAppInitialize XTAPPINITIALIZE
#define XtAppNextEvent XTAPPNEXTEVENT
#define XtAppPeekEvent XTAPPPEEKEVENT
#define XtAppPending XTAPPPENDING
#define XtAppProcessEvent XTAPPPROCESSEVENT
#define XtAppSetErrorHandler XTAPPSETERRORHANDLER
#define XtAppSetFallbackResources XTAPPSETFALLBACKRESOURCES
#define XtAppSetWarningHandler XTAPPSETWARNINGHANDLER
#define XtAppSetWarningMsgHandler XTAPPSETWARNINGMSGHANDLER
#define XtAppWarning XTAPPWARNING
#define XtCallActionProc XTCALLACTIONPROC
#define XtCallCallbackList XTCALLCALLBACKLIST
#define XtCallCallbacks XTCALLCALLBACKS
#define XtConfigureWidget XTCONFIGUREWIDGET
#define XtConvertAndStore XTCONVERTANDSTORE
#define XtCreateApplicationContext XTCREATEAPPLICATIONCONTEXT
#define XtCreateManagedWidget XTCREATEMANAGEDWIDGET
#define XtCreatePopupShell XTCREATEPOPUPSHELL
#define XtCreateWidget XTCREATEWIDGET
#define XtDatabase XTDATABASE
#define XtDestroyWidget XTDESTROYWIDGET
#define XtDisownSelection XTDISOWNSELECTION
#define XtDispatchEvent XTDISPATCHEVENT
#define XtDisplayOfObject XTDISPLAYOFOBJECT
#define XtDisplayStringConvWarning XTDISPLAYSTRINGCONVWARNING
#define XtDisplayToApplicationContext XTDISPLAYTOAPPLICATIONCONTEXT
#define XtFree XTFREE
#define XtGetActionKeysym XTGETACTIONKEYSYM
#define XtGetActionList XTGETACTIONLIST
#define XtGetApplicationNameAndClass XTGETAPPLICATIONNAMEANDCLASS
#define XtGetApplicationResources XTGETAPPLICATIONRESOURCES
#define XtGetGC XTGETGC
#define XtGetMultiClickTime XTGETMULTICLICKTIME
#define XtGetSelectionValue XTGETSELECTIONVALUE
#define XtGetSelectionValues XTGETSELECTIONVALUES
#define XtGetSubresources XTGETSUBRESOURCES
#define XtGetValues XTGETVALUES
#define XtGrabKeyboard XTGRABKEYBOARD
#define XtGrabPointer XTGRABPOINTER
#define XtHasCallbacks XTHASCALLBACKS
#define XtInitializeWidgetClass XTINITIALIZEWIDGETCLASS
#define XtInsertEventHandler XTINSERTEVENTHANDLER
#define XtIsManaged XTISMANAGED
#define XtIsObject XTISOBJECT
#define XtIsSensitive XTISSENSITIVE
#define XtIsSubclass XTISSUBCLASS
#define XtLastTimestampProcessed XTLASTTIMESTAMPPROCESSED
#define XtMakeGeometryRequest XTMAKEGEOMETRYREQUEST
#define XtMakeResizeRequest XTMAKERESIZEREQUEST
#define XtMalloc XTMALLOC
#define XtManageChild XTMANAGECHILD
#define XtManageChildren XTMANAGECHILDREN
#define XtMergeArgLists XTMERGEARGLISTS
#define XtMoveWidget XTMOVEWIDGET
#define XtName XTNAME
#define XtNameToWidget XTNAMETOWIDGET
#define XtOpenDisplay XTOPENDISPLAY
#define XtOverrideTranslations XTOVERRIDETRANSLATIONS
#define XtOwnSelection XTOWNSELECTION
#define XtParent XTPARENT
#define XtParseTranslationTable XTPARSETRANSLATIONTABLE
#define XtPopdown XTPOPDOWN
#define XtPopup XTPOPUP
#define XtQueryGeometry XTQUERYGEOMETRY
#define XtRealizeWidget XTREALIZEWIDGET
#define XtRealloc XTREALLOC
#define XtReleaseGC XTRELEASEGC
#define XtRemoveAllCallbacks XTREMOVEALLCALLBACKS
#define XtRemoveCallback XTREMOVECALLBACK
#define XtRemoveEventHandler XTREMOVEEVENTHANDLER
#define XtRemoveGrab XTREMOVEGRAB
#define XtRemoveInput XTREMOVEINPUT
#define XtRemoveTimeOut XTREMOVETIMEOUT
#define XtResizeWidget XTRESIZEWIDGET
#define XtResolvePathname XTRESOLVEPATHNAME
#define XtSetKeyboardFocus XTSETKEYBOARDFOCUS
#define XtSetMappedWhenManaged XTSETMAPPEDWHENMANAGED
#define XtSetSensitive XTSETSENSITIVE
#define XtSetTypeConverter XTSETTYPECONVERTER
#define XtSetValues XTSETVALUES
#define XtShellStrings XTSHELLSTRINGS
#define XtStrings XTSTRINGS
#define XtToolkitInitialize XTTOOLKITINITIALIZE
#define XtTranslateCoords XTTRANSLATECOORDS
#define XtTranslateKeycode XTTRANSLATEKEYCODE
#define XtUngrabKeyboard XTUNGRABKEYBOARD
#define XtUngrabPointer XTUNGRABPOINTER
#define XtUnmanageChild XTUNMANAGECHILD
#define XtUnmanageChildren XTUNMANAGECHILDREN
#define XtUnrealizeWidget XTUNREALIZEWIDGET
#define XtVaAppCreateShell XTVAAPPCREATESHELL
#define XtVaCreateManagedWidget XTVACREATEMANAGEDWIDGET
#define XtVaCreatePopupShell XTVACREATEPOPUPSHELL
#define XtVaCreateWidget XTVACREATEWIDGET
#define XtVaGetValues XTVAGETVALUES
#define XtVaSetValues XTVASETVALUES
#define XtWarning XTWARNING
#define XtWidgetToApplicationContext XTWIDGETTOAPPLICATIONCONTEXT
#define XtWindow XTWINDOW
#define XtWindowOfObject XTWINDOWOFOBJECT
#define XtWindowToWidget XTWINDOWTOWIDGET
#define XwcDrawString XWCDRAWSTRING
#define XwcFreeStringList XWCFREESTRINGLIST
#define XwcTextEscapement XWCTEXTESCAPEMENT
#define XwcTextExtents XWCTEXTEXTENTS
#define XwcTextListToTextProperty XWCTEXTLISTTOTEXTPROPERTY
#define XwcTextPropertyToTextList XWCTEXTPROPERTYTOTEXTLIST
#define _XmBottomShadowColorDefault _XMBOTTOMSHADOWCOLORDEFAULT
#define _XmClearBorder _XMCLEARBORDER
#define _XmConfigureObject _XMCONFIGUREOBJECT
#define _XmDestroyParentCallback _XMDESTROYPARENTCALLBACK
#define _XmDrawArrow _XMDRAWARROW
#define _XmDrawShadows _XMDRAWSHADOWS
#define _XmFontListGetDefaultFont _XMFONTLISTGETDEFAULTFONT
#define _XmFromHorizontalPixels _XMFROMHORIZONTALPIXELS
#define _XmFromVerticalPixels _XMFROMVERTICALPIXELS
#define _XmGetClassExtensionPtr _XMGETCLASSEXTENSIONPTR
#define _XmGetDefaultFontList _XMGETDEFAULTFONTLIST
#define _XmGetTextualDragIcon _XMGETTEXTUALDRAGICON
#define _XmGetWidgetExtData _XMGETWIDGETEXTDATA
#define _XmGrabKeyboard _XMGRABKEYBOARD
#define _XmGrabPointer _XMGRABPOINTER
#define _XmInheritClass _XMINHERITCLASS
#define _XmInputInGadget _XMINPUTINGADGET
#define _XmMakeGeometryRequest _XMMAKEGEOMETRYREQUEST
#define _XmMenuPopDown _XMMENUPOPDOWN
#define _XmMoveObject _XMMOVEOBJECT
#define _XmNavigChangeManaged _XMNAVIGCHANGEMANAGED
#define _XmOSBuildFileList _XMOSBUILDFILELIST
#define _XmOSFileCompare _XMOSFILECOMPARE
#define _XmOSFindPatternPart _XMOSFINDPATTERNPART
#define _XmOSQualifyFileSpec _XMOSQUALIFYFILESPEC
#define _XmPostPopupMenu _XMPOSTPOPUPMENU
#define _XmPrimitiveEnter _XMPRIMITIVEENTER
#define _XmPrimitiveLeave _XMPRIMITIVELEAVE
#define _XmRedisplayGadgets _XMREDISPLAYGADGETS
#define _XmShellIsExclusive _XMSHELLISEXCLUSIVE
#define _XmStringDraw _XMSTRINGDRAW
#define _XmStringGetTextConcat _XMSTRINGGETTEXTCONCAT
#define _XmStrings _XMSTRINGS
#define _XmToHorizontalPixels _XMTOHORIZONTALPIXELS
#define _XmToVerticalPixels _XMTOVERTICALPIXELS
#define _XmTopShadowColorDefault _XMTOPSHADOWCOLORDEFAULT
#define _Xm_fastPtr _XM_FASTPTR
#define _XtCheckSubclassFlag _XTCHECKSUBCLASSFLAG
#define _XtInherit _XTINHERIT
#define _XtInheritTranslations _XTINHERITTRANSLATIONS
#define applicationShellWidgetClass APPLICATIONSHELLWIDGETCLASS
#define compositeWidgetClass COMPOSITEWIDGETCLASS
#define overrideShellWidgetClass OVERRIDESHELLWIDGETCLASS
#define shellWidgetClass SHELLWIDGETCLASS
#define topLevelShellClassRec TOPLEVELSHELLCLASSREC
#define topLevelShellWidgetClass TOPLEVELSHELLWIDGETCLASS
#define transientShellWidgetClass TRANSIENTSHELLWIDGETCLASS
#define vendorShellClassRec VENDORSHELLCLASSREC
#define vendorShellWidgetClass VENDORSHELLWIDGETCLASS
#define wmShellWidgetClass WMSHELLWIDGETCLASS
#define xmArrowButtonWidgetClass XMARROWBUTTONWIDGETCLASS
#define xmCascadeButtonClassRec XMCASCADEBUTTONCLASSREC
#define xmCascadeButtonGadgetClass XMCASCADEBUTTONGADGETCLASS
#define xmCascadeButtonWidgetClass XMCASCADEBUTTONWIDGETCLASS
#define xmDialogShellWidgetClass XMDIALOGSHELLWIDGETCLASS
#define xmDrawingAreaWidgetClass XMDRAWINGAREAWIDGETCLASS
#define xmDrawnButtonWidgetClass XMDRAWNBUTTONWIDGETCLASS
#define xmFileSelectionBoxWidgetClass XMFILESELECTIONBOXWIDGETCLASS
#define xmFormWidgetClass XMFORMWIDGETCLASS
#define xmFrameWidgetClass XMFRAMEWIDGETCLASS
#define xmGadgetClass XMGADGETCLASS
#define xmLabelGadgetClass XMLABELGADGETCLASS
#define xmLabelWidgetClass XMLABELWIDGETCLASS
#define xmListWidgetClass XMLISTWIDGETCLASS
#define xmMainWindowWidgetClass XMMAINWINDOWWIDGETCLASS
#define xmManagerClassRec XMMANAGERCLASSREC
#define xmManagerWidgetClass XMMANAGERWIDGETCLASS
#define xmMenuShellWidgetClass XMMENUSHELLWIDGETCLASS
#define xmMessageBoxWidgetClass XMMESSAGEBOXWIDGETCLASS
#define xmPrimitiveClassRec XMPRIMITIVECLASSREC
#define xmPrimitiveWidgetClass XMPRIMITIVEWIDGETCLASS
#define xmPushButtonClassRec XMPUSHBUTTONCLASSREC
#define xmPushButtonGadgetClass XMPUSHBUTTONGADGETCLASS
#define xmPushButtonWidgetClass XMPUSHBUTTONWIDGETCLASS
#define xmRowColumnWidgetClass XMROWCOLUMNWIDGETCLASS
#define xmSashWidgetClass XMSASHWIDGETCLASS
#define xmScrollBarWidgetClass XMSCROLLBARWIDGETCLASS
#define xmScrolledWindowClassRec XMSCROLLEDWINDOWCLASSREC
#define xmScrolledWindowWidgetClass XMSCROLLEDWINDOWWIDGETCLASS
#define xmSeparatorGadgetClass XMSEPARATORGADGETCLASS
#define xmSeparatorWidgetClass XMSEPARATORWIDGETCLASS
#define xmTextFieldWidgetClass XMTEXTFIELDWIDGETCLASS
#define xmTextWidgetClass XMTEXTWIDGETCLASS
#define xmToggleButtonGadgetClass XMTOGGLEBUTTONGADGETCLASS
#define xmToggleButtonWidgetClass XMTOGGLEBUTTONWIDGETCLASS

/*
** XtRegisterDrawable and XtUnregisterDrawable don't exist until R6.
** So we have to fake it for R5 and earlier. It would be real nice to
** include X11.h here and then test for R6 by checking
** XlibSpecificationRelease. But including X11.h defines the symbol "None"
** and there's Mozilla code in gfx/src/gtk that defines an enum element
** named None. So for now hard code this is R5....
*/
#ifndef VMS_X11R6
// R5 or earlier
#define _XtRegisterWindow _XTREGISTERWINDOW
#define _XtUnregisterWindow _XTUNREGISTERWINDOW
#else
// R6 or later
#define XtRegisterDrawable XTREGISTERDRAWABLE
#define XtUnregisterDrawable XTUNREGISTERDRAWABLE
#endif
