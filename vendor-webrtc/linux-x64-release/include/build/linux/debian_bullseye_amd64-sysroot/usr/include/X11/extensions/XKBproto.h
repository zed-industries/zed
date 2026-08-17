/************************************************************
Copyright (c) 1993 by Silicon Graphics Computer Systems, Inc.

Permission to use, copy, modify, and distribute this
software and its documentation for any purpose and without
fee is hereby granted, provided that the above copyright
notice appear in all copies and that both that copyright
notice and this permission notice appear in supporting
documentation, and that the name of Silicon Graphics not be
used in advertising or publicity pertaining to distribution
of the software without specific prior written permission.
Silicon Graphics makes no representation about the suitability
of this software for any purpose. It is provided "as is"
without any express or implied warranty.

SILICON GRAPHICS DISCLAIMS ALL WARRANTIES WITH REGARD TO THIS
SOFTWARE, INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY
AND FITNESS FOR A PARTICULAR PURPOSE. IN NO EVENT SHALL SILICON
GRAPHICS BE LIABLE FOR ANY SPECIAL, INDIRECT OR CONSEQUENTIAL
DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM LOSS OF USE,
DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE
OR OTHER TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION  WITH
THE USE OR PERFORMANCE OF THIS SOFTWARE.

********************************************************/

#ifndef _XKBPROTO_H_
#define	_XKBPROTO_H_

#include <X11/Xmd.h>
#include <X11/extensions/XKB.h>

#define Window CARD32
#define Atom CARD32
#define Time CARD32
#define KeyCode CARD8
#define KeySym CARD32

#define	XkbPaddedSize(n)	((((unsigned int)(n)+3) >> 2) << 2)

typedef struct _xkbUseExtension {
    CARD8	reqType;
    CARD8	xkbReqType;	/* always X_KBUseExtension */
    CARD16	length;
    CARD16	wantedMajor;
    CARD16	wantedMinor;
} xkbUseExtensionReq;
#define	sz_xkbUseExtensionReq	8

typedef struct _xkbUseExtensionReply {
    BYTE	type;		/* X_Reply */
    BOOL	supported;
    CARD16	sequenceNumber;
    CARD32	length;
    CARD16	serverMajor;
    CARD16	serverMinor;
    CARD32	pad1;
    CARD32	pad2;
    CARD32	pad3;
    CARD32	pad4;
    CARD32	pad5;
} xkbUseExtensionReply;
#define	sz_xkbUseExtensionReply	32

typedef	struct _xkbSelectEvents {
    CARD8	reqType;
    CARD8	xkbReqType;	/* X_KBSelectEvents */
    CARD16	length;
    CARD16	deviceSpec;
    CARD16	affectWhich;
    CARD16	clear;
    CARD16	selectAll;
    CARD16	affectMap;
    CARD16	map;
} xkbSelectEventsReq;
#define	sz_xkbSelectEventsReq	16

typedef struct _xkbBell {
    CARD8	reqType;
    CARD8	xkbReqType;	/* X_KBBell */
    CARD16	length;
    CARD16	deviceSpec;
    CARD16	bellClass;
    CARD16	bellID;
    INT8	percent;
    BOOL	forceSound;
    BOOL	eventOnly;
    CARD8	pad1;
    INT16	pitch;
    INT16	duration;
    CARD16	pad2;
    Atom	name;
    Window	window;
} xkbBellReq;
#define	sz_xkbBellReq		28

typedef struct _xkbGetState {
	CARD8		reqType;
	CARD8		xkbReqType;	/* always X_KBGetState */
	CARD16		length;
	CARD16		deviceSpec;
	CARD16		pad;
} xkbGetStateReq;
#define	sz_xkbGetStateReq	8

typedef	struct _xkbGetStateReply {
    BYTE	type;
    BYTE	deviceID;
    CARD16	sequenceNumber;
    CARD32	length;
    CARD8	mods;
    CARD8	baseMods;
    CARD8	latchedMods;
    CARD8	lockedMods;
    CARD8	group;
    CARD8	lockedGroup;
    INT16	baseGroup;
    INT16	latchedGroup;
    CARD8	compatState;
    CARD8	grabMods;
    CARD8	compatGrabMods;
    CARD8	lookupMods;
    CARD8	compatLookupMods;
    CARD8	pad1;
    CARD16	ptrBtnState;
    CARD16	pad2;
    CARD32	pad3;
} xkbGetStateReply;
#define	sz_xkbGetStateReply	32

typedef struct _xkbLatchLockState {
    CARD8	reqType;
    CARD8	xkbReqType;	/* always X_KBLatchLockState */
    CARD16	length;
    CARD16	deviceSpec;
    CARD8	affectModLocks;
    CARD8	modLocks;
    BOOL	lockGroup;
    CARD8	groupLock;
    CARD8	affectModLatches;
    CARD8	modLatches;
    CARD8	pad;
    BOOL	latchGroup;
    INT16	groupLatch;
} xkbLatchLockStateReq;
#define	sz_xkbLatchLockStateReq		16

typedef struct _xkbGetControls {
    CARD8	reqType;
    CARD8	xkbReqType;	/* always X_KBGetControls */
    CARD16	length;
    CARD16	deviceSpec;
    CARD16	pad;
} xkbGetControlsReq;
#define	sz_xkbGetControlsReq	8

typedef struct _xkbGetControlsReply {
    BYTE	type;		/* X_Reply */
    CARD8	deviceID;
    CARD16	sequenceNumber;
    CARD32	length;
    CARD8	mkDfltBtn;
    CARD8	numGroups;
    CARD8	groupsWrap;
    CARD8	internalMods;
    CARD8	ignoreLockMods;
    CARD8	internalRealMods;
    CARD8	ignoreLockRealMods;
    CARD8	pad1;
    CARD16	internalVMods;
    CARD16	ignoreLockVMods;
    CARD16	repeatDelay;
    CARD16	repeatInterval;
    CARD16	slowKeysDelay;
    CARD16	debounceDelay;
    CARD16	mkDelay;
    CARD16	mkInterval;
    CARD16	mkTimeToMax;
    CARD16	mkMaxSpeed;
    INT16	mkCurve;
    CARD16	axOptions;
    CARD16	axTimeout;
    CARD16	axtOptsMask;
    CARD16	axtOptsValues;
    CARD16	pad2;
    CARD32	axtCtrlsMask;
    CARD32	axtCtrlsValues;
    CARD32	enabledCtrls;
    BYTE	perKeyRepeat[XkbPerKeyBitArraySize];
} xkbGetControlsReply;
#define	sz_xkbGetControlsReply	92

typedef struct _xkbSetControls {
    CARD8	reqType;
    CARD8	xkbReqType;	/* always X_KBSetControls */
    CARD16	length;
    CARD16	deviceSpec;
    CARD8	affectInternalMods;
    CARD8	internalMods;
    CARD8	affectIgnoreLockMods;
    CARD8	ignoreLockMods;
    CARD16	affectInternalVMods;
    CARD16	internalVMods;
    CARD16	affectIgnoreLockVMods;
    CARD16	ignoreLockVMods;
    CARD8	mkDfltBtn;
    CARD8	groupsWrap;
    CARD16	axOptions;
    CARD16	pad1;
    CARD32	affectEnabledCtrls;
    CARD32	enabledCtrls;
    CARD32	changeCtrls;
    CARD16	repeatDelay;
    CARD16	repeatInterval;
    CARD16	slowKeysDelay;
    CARD16	debounceDelay;
    CARD16	mkDelay;
    CARD16	mkInterval;
    CARD16	mkTimeToMax;
    CARD16	mkMaxSpeed;
    INT16	mkCurve;
    CARD16	axTimeout;
    CARD32	axtCtrlsMask;
    CARD32	axtCtrlsValues;
    CARD16	axtOptsMask;
    CARD16	axtOptsValues;
    BYTE	perKeyRepeat[XkbPerKeyBitArraySize];
} xkbSetControlsReq;
#define	sz_xkbSetControlsReq	100

typedef	struct _xkbKTMapEntryWireDesc {
    BOOL	active;
    CARD8	mask;
    CARD8	level;
    CARD8	realMods;
    CARD16	virtualMods;
    CARD16	pad;
} xkbKTMapEntryWireDesc;
#define sz_xkbKTMapEntryWireDesc	8

typedef struct _xkbKTSetMapEntryWireDesc {
    CARD8	level;
    CARD8	realMods;
    CARD16	virtualMods;
} xkbKTSetMapEntryWireDesc;
#define	sz_xkbKTSetMapEntryWireDesc	4

typedef struct _xkbModsWireDesc {
    CARD8	mask;		/* GetMap only */
    CARD8	realMods;
    CARD16	virtualMods;
} xkbModsWireDesc;
#define	sz_xkbModsWireDesc	4

typedef struct _xkbKeyTypeWireDesc {
    CARD8	mask;
    CARD8	realMods;
    CARD16	virtualMods;
    CARD8	numLevels;
    CARD8	nMapEntries;
    BOOL	preserve;
    CARD8	pad;
} xkbKeyTypeWireDesc;
#define	sz_xkbKeyTypeWireDesc	8

typedef struct _xkbSymMapWireDesc {
    CARD8	ktIndex[XkbNumKbdGroups];
    CARD8	groupInfo;
    CARD8	width;
    CARD16	nSyms;
} xkbSymMapWireDesc;
#define	sz_xkbSymMapWireDesc	8

typedef struct _xkbVModMapWireDesc {
    KeyCode	key;
    CARD8	pad;
    CARD16	vmods;
} xkbVModMapWireDesc;
#define	sz_xkbVModMapWireDesc	4

typedef struct _xkbBehaviorWireDesc {
	CARD8	key;
	CARD8	type;
	CARD8	data;
	CARD8	pad;
} xkbBehaviorWireDesc;
#define	sz_xkbBehaviorWireDesc	4

typedef	struct _xkbActionWireDesc {
    CARD8	type;
    CARD8	data[7];
} xkbActionWireDesc;
#define	sz_xkbActionWireDesc	8

typedef struct _xkbGetMap {
    CARD8	reqType;
    CARD8	xkbReqType;	/* always X_KBGetMap */
    CARD16	length;
    CARD16	deviceSpec;
    CARD16	full;
    CARD16	partial;
    CARD8	firstType;
    CARD8	nTypes;
    KeyCode	firstKeySym;
    CARD8	nKeySyms;
    KeyCode	firstKeyAct;
    CARD8	nKeyActs;
    KeyCode	firstKeyBehavior;
    CARD8	nKeyBehaviors;
    CARD16	virtualMods;
    KeyCode	firstKeyExplicit;
    CARD8	nKeyExplicit;
    KeyCode	firstModMapKey;
    CARD8	nModMapKeys;
    KeyCode	firstVModMapKey;
    CARD8	nVModMapKeys;
    CARD16	pad1;
} xkbGetMapReq;
#define	sz_xkbGetMapReq	28

typedef struct _xkbGetMapReply {
    CARD8	type;		/* always X_Reply */
    CARD8	deviceID;
    CARD16	sequenceNumber;
    CARD32	length;
    CARD16	pad1;
    KeyCode	minKeyCode;
    KeyCode	maxKeyCode;
    CARD16	present;
    CARD8	firstType;
    CARD8	nTypes;
    CARD8	totalTypes;
    KeyCode	firstKeySym;
    CARD16	totalSyms;
    CARD8	nKeySyms;
    KeyCode	firstKeyAct;
    CARD16	totalActs;
    CARD8	nKeyActs;
    KeyCode	firstKeyBehavior;
    CARD8	nKeyBehaviors;
    CARD8	totalKeyBehaviors;
    KeyCode	firstKeyExplicit;
    CARD8	nKeyExplicit;
    CARD8	totalKeyExplicit;
    KeyCode	firstModMapKey;
    CARD8	nModMapKeys;
    CARD8	totalModMapKeys;
    KeyCode	firstVModMapKey;
    CARD8	nVModMapKeys;
    CARD8	totalVModMapKeys;
    CARD8	pad2;
    CARD16	virtualMods;
} xkbGetMapReply;
#define	sz_xkbGetMapReply		40

#define	XkbSetMapResizeTypes		(1L<<0)
#define	XkbSetMapRecomputeActions	(1L<<1)
#define	XkbSetMapAllFlags		(0x3)

typedef struct _xkbSetMap {
    CARD8	reqType;
    CARD8	xkbReqType;	/* always X_KBSetMap */
    CARD16	length;
    CARD16	deviceSpec;
    CARD16	present;
    CARD16	flags;
    KeyCode	minKeyCode;
    KeyCode	maxKeyCode;
    CARD8	firstType;
    CARD8	nTypes;
    KeyCode	firstKeySym;
    CARD8	nKeySyms;
    CARD16	totalSyms;
    KeyCode	firstKeyAct;
    CARD8	nKeyActs;
    CARD16	totalActs;
    KeyCode	firstKeyBehavior;
    CARD8	nKeyBehaviors;
    CARD8	totalKeyBehaviors;
    KeyCode	firstKeyExplicit;
    CARD8	nKeyExplicit;
    CARD8	totalKeyExplicit;
    KeyCode	firstModMapKey;
    CARD8	nModMapKeys;
    CARD8	totalModMapKeys;
    KeyCode	firstVModMapKey;
    CARD8	nVModMapKeys;
    CARD8	totalVModMapKeys;
    CARD16	virtualMods;
} xkbSetMapReq;
#define	sz_xkbSetMapReq	36

typedef struct _xkbSymInterpretWireDesc {
    CARD32		sym;
    CARD8		mods;
    CARD8		match;
    CARD8		virtualMod;
    CARD8		flags;
    xkbActionWireDesc	act;
} xkbSymInterpretWireDesc;
#define	sz_xkbSymInterpretWireDesc	16

typedef struct _xkbGetCompatMap {
    CARD8	reqType;
    CARD8	xkbReqType;	/* always X_KBGetCompatMap */
    CARD16	length;
    CARD16	deviceSpec;
    CARD8	groups;
    BOOL	getAllSI;
    CARD16	firstSI;
    CARD16	nSI;
} xkbGetCompatMapReq;
#define	sz_xkbGetCompatMapReq	12

typedef struct _xkbGetCompatMapReply {
    CARD8	type;		/* always X_Reply */
    CARD8	deviceID;
    CARD16	sequenceNumber;
    CARD32	length;
    CARD8	groups;
    CARD8	pad1;
    CARD16	firstSI;
    CARD16	nSI;
    CARD16	nTotalSI;
    CARD32	pad2;
    CARD32	pad3;
    CARD32	pad4;
    CARD32	pad5;
} xkbGetCompatMapReply;
#define	sz_xkbGetCompatMapReply		32

typedef struct _xkbSetCompatMap {
    CARD8	reqType;
    CARD8	xkbReqType;	/* always X_KBSetCompatMap */
    CARD16	length;
    CARD16	deviceSpec;
    CARD8	pad1;
    BOOL	recomputeActions;
    BOOL	truncateSI;
    CARD8	groups;
    CARD16	firstSI;
    CARD16	nSI;
    CARD16	pad2;
} xkbSetCompatMapReq;
#define	sz_xkbSetCompatMapReq	16

typedef struct _xkbGetIndicatorState {
    CARD8	reqType;
    CARD8	xkbReqType;	/* always X_KBGetIndicatorState */
    CARD16	length;
    CARD16	deviceSpec;
    CARD16	pad1;
} xkbGetIndicatorStateReq;
#define	sz_xkbGetIndicatorStateReq	8

typedef struct _xkbGetIndicatorStateReply {
    CARD8	type;		/* always X_Reply */
    CARD8	deviceID;
    CARD16	sequenceNumber;
    CARD32	length;
    CARD32	state;
    CARD32	pad1;
    CARD32	pad2;
    CARD32	pad3;
    CARD32	pad4;
    CARD32	pad5;
} xkbGetIndicatorStateReply;
#define	sz_xkbGetIndicatorStateReply	32

typedef struct _xkbGetIndicatorMap {
    CARD8	reqType;
    CARD8	xkbReqType;	/* always X_KBGetIndicatorMap */
    CARD16	length;
    CARD16	deviceSpec;
    CARD16	pad;
    CARD32	which;
} xkbGetIndicatorMapReq;
#define	sz_xkbGetIndicatorMapReq	12

typedef struct _xkbGetIndicatorMapReply {
    CARD8	type;		/* always X_Reply */
    CARD8	deviceID;
    CARD16	sequenceNumber;
    CARD32	length;
    CARD32	which;
    CARD32	realIndicators;
    CARD8	nIndicators;
    CARD8	pad1;
    CARD16	pad2;
    CARD32	pad3;
    CARD32	pad4;
    CARD32	pad5;
} xkbGetIndicatorMapReply;
#define	sz_xkbGetIndicatorMapReply	32

typedef struct _xkbIndicatorMapWireDesc {
    CARD8	flags;
    CARD8	whichGroups;
    CARD8	groups;
    CARD8	whichMods;
    CARD8	mods;
    CARD8	realMods;
    CARD16	virtualMods;
    CARD32	ctrls;
} xkbIndicatorMapWireDesc;
#define	sz_xkbIndicatorMapWireDesc	12

typedef struct _xkbSetIndicatorMap {
    CARD8	reqType;
    CARD8	xkbReqType;	/* always X_KBSetIndicatorMap */
    CARD16	length;
    CARD16	deviceSpec;
    CARD16	pad1;
    CARD32	which;
} xkbSetIndicatorMapReq;
#define	sz_xkbSetIndicatorMapReq	12

typedef struct _xkbGetNamedIndicator {
    CARD8	reqType;
    CARD8	xkbReqType;	/* X_KBGetNamedIndicator */
    CARD16	length;
    CARD16	deviceSpec;
    CARD16	ledClass;
    CARD16	ledID;
    CARD16	pad1;
    Atom	indicator;
} xkbGetNamedIndicatorReq;
#define	sz_xkbGetNamedIndicatorReq		16

typedef	struct _xkbGetNamedIndicatorReply {
    BYTE	type;
    BYTE	deviceID;
    CARD16	sequenceNumber;
    CARD32	length;
    Atom	indicator;
    BOOL	found;
    BOOL	on;
    BOOL	realIndicator;
    CARD8	ndx;
    CARD8	flags;
    CARD8	whichGroups;
    CARD8	groups;
    CARD8	whichMods;
    CARD8	mods;
    CARD8	realMods;
    CARD16	virtualMods;
    CARD32	ctrls;
    BOOL	supported;
    CARD8	pad1;
    CARD16	pad2;
} xkbGetNamedIndicatorReply;
#define	sz_xkbGetNamedIndicatorReply	32

typedef struct _xkbSetNamedIndicator {
    CARD8	reqType;
    CARD8	xkbReqType;	/* X_KBSetNamedIndicator */
    CARD16	length;
    CARD16	deviceSpec;
    CARD16	ledClass;
    CARD16	ledID;
    CARD16	pad1;
    Atom	indicator;
    BOOL	setState;
    BOOL	on;
    BOOL	setMap;
    BOOL	createMap;
    CARD8	pad2;
    CARD8	flags;
    CARD8	whichGroups;
    CARD8	groups;
    CARD8	whichMods;
    CARD8	realMods;
    CARD16	virtualMods;
    CARD32	ctrls;
} xkbSetNamedIndicatorReq;
#define	sz_xkbSetNamedIndicatorReq	32

typedef struct _xkbGetNames {
    CARD8	reqType;
    CARD8	xkbReqType;	/* always X_KBGetNames */
    CARD16	length;
    CARD16	deviceSpec;
    CARD16	pad;
    CARD32	which;
} xkbGetNamesReq;
#define	sz_xkbGetNamesReq		12

typedef	struct _xkbGetNamesReply {
    BYTE	type;
    BYTE	deviceID;
    CARD16	sequenceNumber;
    CARD32	length;
    CARD32	which;
    KeyCode	minKeyCode;
    KeyCode	maxKeyCode;
    CARD8	nTypes;
    CARD8	groupNames;
    CARD16	virtualMods;
    KeyCode	firstKey;
    CARD8	nKeys;
    CARD32	indicators;
    CARD8	nRadioGroups;
    CARD8	nKeyAliases;
    CARD16	nKTLevels;
    CARD32	pad3;
} xkbGetNamesReply;
#define	sz_xkbGetNamesReply	32

typedef struct _xkbSetNames {
    CARD8	reqType;
    CARD8	xkbReqType;	/* always X_KBSetNames */
    CARD16	length;
    CARD16	deviceSpec;
    CARD16	virtualMods;
    CARD32	which;
    CARD8	firstType;
    CARD8	nTypes;
    CARD8	firstKTLevel;
    CARD8	nKTLevels;
    CARD32	indicators;
    CARD8	groupNames;
    CARD8	nRadioGroups;
    KeyCode	firstKey;
    CARD8	nKeys;
    CARD8	nKeyAliases;
    CARD8	pad1;
    CARD16	totalKTLevelNames;
} xkbSetNamesReq;
#define	sz_xkbSetNamesReq	28

typedef struct _xkbPointWireDesc {
    INT16	x;
    INT16	y;
} xkbPointWireDesc;
#define	sz_xkbPointWireDesc	4

typedef struct _xkbOutlineWireDesc {
    CARD8	nPoints;
    CARD8	cornerRadius;
    CARD16	pad;
} xkbOutlineWireDesc;
#define	sz_xkbOutlineWireDesc	4

typedef struct _xkbShapeWireDesc {
    Atom	name;
    CARD8	nOutlines;
    CARD8	primaryNdx;
    CARD8	approxNdx;
    CARD8	pad;
} xkbShapeWireDesc;
#define	sz_xkbShapeWireDesc	8

typedef struct _xkbSectionWireDesc {
    Atom	name;
    INT16	top;
    INT16	left;
    CARD16	width;
    CARD16	height;
    INT16	angle;
    CARD8	priority;
    CARD8	nRows;
    CARD8	nDoodads;
    CARD8	nOverlays;
    CARD16	pad;
} xkbSectionWireDesc;
#define	sz_xkbSectionWireDesc	20

typedef struct _xkbRowWireDesc {
    INT16	top;
    INT16	left;
    CARD8	nKeys;
    BOOL	vertical;
    CARD16	pad;
} xkbRowWireDesc;
#define	sz_xkbRowWireDesc	8

typedef struct _xkbKeyWireDesc {
    CARD8	name[XkbKeyNameLength];
    INT16	gap;
    CARD8	shapeNdx;
    CARD8	colorNdx;
} xkbKeyWireDesc;
#define	sz_xkbKeyWireDesc	8

typedef struct _xkbOverlayWireDesc {
    Atom	name;
    CARD8	nRows;
    CARD8	pad1;
    CARD16	pad2;
} xkbOverlayWireDesc;
#define	sz_xkbOverlayWireDesc	8

typedef struct _xkbOverlayRowWireDesc {
   CARD8	rowUnder;
   CARD8	nKeys;
   CARD16	pad1;
} xkbOverlayRowWireDesc;
#define	sz_xkbOverlayRowWireDesc	4

typedef struct _xkbOverlayKeyWireDesc {
   CARD8	over[XkbKeyNameLength];
   CARD8	under[XkbKeyNameLength];
} xkbOverlayKeyWireDesc;
#define	sz_xkbOverlayKeyWireDesc	8

typedef struct _xkbShapeDoodadWireDesc {
    Atom	name;
    CARD8	type;
    CARD8	priority;
    INT16	top;
    INT16	left;
    INT16	angle;
    CARD8	colorNdx;
    CARD8	shapeNdx;
    CARD16	pad1;
    CARD32	pad2;
} xkbShapeDoodadWireDesc;
#define	sz_xkbShapeDoodadWireDesc	20

typedef struct _xkbTextDoodadWireDesc {
    Atom	name;
    CARD8	type;
    CARD8	priority;
    INT16	top;
    INT16	left;
    INT16	angle;
    CARD16	width;
    CARD16	height;
    CARD8	colorNdx;
    CARD8	pad1;
    CARD16	pad2;
} xkbTextDoodadWireDesc;
#define	sz_xkbTextDoodadWireDesc	20

typedef struct _xkbIndicatorDoodadWireDesc {
    Atom	name;
    CARD8	type;
    CARD8	priority;
    INT16	top;
    INT16	left;
    INT16	angle;
    CARD8	shapeNdx;
    CARD8	onColorNdx;
    CARD8	offColorNdx;
    CARD8	pad1;
    CARD32	pad2;
} xkbIndicatorDoodadWireDesc;
#define	sz_xkbIndicatorDoodadWireDesc	20

typedef struct _xkbLogoDoodadWireDesc {
    Atom	name;
    CARD8	type;
    CARD8	priority;
    INT16	top;
    INT16	left;
    INT16	angle;
    CARD8	colorNdx;
    CARD8	shapeNdx;
    CARD16	pad1;
    CARD32	pad2;
} xkbLogoDoodadWireDesc;
#define	sz_xkbLogoDoodadWireDesc	20

typedef struct _xkbAnyDoodadWireDesc {
    Atom	name;
    CARD8	type;
    CARD8	priority;
    INT16	top;
    INT16	left;
    INT16	angle;
    CARD32	pad2;
    CARD32	pad3;
} xkbAnyDoodadWireDesc;
#define	sz_xkbAnyDoodadWireDesc	20

typedef union _xkbDoodadWireDesc {
    xkbAnyDoodadWireDesc	any;
    xkbShapeDoodadWireDesc	shape;
    xkbTextDoodadWireDesc	text;
    xkbIndicatorDoodadWireDesc	indicator;
    xkbLogoDoodadWireDesc	logo;
} xkbDoodadWireDesc;
#define	sz_xkbDoodadWireDesc	20

typedef struct _xkbGetGeometry {
    CARD8	reqType;
    CARD8	xkbReqType;	/* always X_KBGetGeometry */
    CARD16	length;
    CARD16	deviceSpec;
    CARD16	pad;
    Atom	name;
} xkbGetGeometryReq;
#define	sz_xkbGetGeometryReq	12

typedef struct _xkbGetGeometryReply {
    CARD8	type;		/* always X_Reply */
    CARD8	deviceID;
    CARD16	sequenceNumber;
    CARD32	length;
    Atom	name;
    BOOL	found;
    CARD8	pad;
    CARD16	widthMM;
    CARD16	heightMM;
    CARD16	nProperties;
    CARD16	nColors;
    CARD16	nShapes;
    CARD16	nSections;
    CARD16	nDoodads;
    CARD16	nKeyAliases;
    CARD8	baseColorNdx;
    CARD8	labelColorNdx;
} xkbGetGeometryReply;
#define	sz_xkbGetGeometryReply	32

typedef struct _xkbSetGeometry {
    CARD8	reqType;
    CARD8	xkbReqType;	/* always X_KBSetGeometry */
    CARD16	length;
    CARD16	deviceSpec;
    CARD8	nShapes;
    CARD8	nSections;
    Atom	name;
    CARD16	widthMM;
    CARD16	heightMM;
    CARD16	nProperties;
    CARD16	nColors;
    CARD16	nDoodads;
    CARD16	nKeyAliases;
    CARD8	baseColorNdx;
    CARD8	labelColorNdx;
    CARD16	pad;
} xkbSetGeometryReq;
#define	sz_xkbSetGeometryReq	28

typedef struct _xkbPerClientFlags {
    CARD8	reqType;
    CARD8	xkbReqType;/* always X_KBPerClientFlags */
    CARD16	length;
    CARD16	deviceSpec;
    CARD16	pad1;
    CARD32	change;
    CARD32	value;
    CARD32	ctrlsToChange;
    CARD32	autoCtrls;
    CARD32	autoCtrlValues;
} xkbPerClientFlagsReq;
#define	sz_xkbPerClientFlagsReq	28

typedef struct _xkbPerClientFlagsReply {
    CARD8	type;		/* always X_Reply */
    CARD8	deviceID;
    CARD16	sequenceNumber;
    CARD32	length;
    CARD32	supported;
    CARD32	value;
    CARD32	autoCtrls;
    CARD32	autoCtrlValues;
    CARD32	pad1;
    CARD32	pad2;
} xkbPerClientFlagsReply;
#define	sz_xkbPerClientFlagsReply	32

typedef struct _xkbListComponents {
    CARD8	reqType;
    CARD8	xkbReqType;	/* always X_KBListComponents */
    CARD16	length;
    CARD16	deviceSpec;
    CARD16	maxNames;
} xkbListComponentsReq;
#define	sz_xkbListComponentsReq	8

typedef struct _xkbListComponentsReply {
    CARD8	type;		/* always X_Reply */
    CARD8	deviceID;
    CARD16	sequenceNumber;
    CARD32	length;
    CARD16	nKeymaps;
    CARD16	nKeycodes;
    CARD16	nTypes;
    CARD16	nCompatMaps;
    CARD16	nSymbols;
    CARD16	nGeometries;
    CARD16	extra;
    CARD16	pad1;
    CARD32	pad2;
    CARD32	pad3;
} xkbListComponentsReply;
#define	sz_xkbListComponentsReply	32

typedef struct _xkbGetKbdByName {
    CARD8	reqType;
    CARD8	xkbReqType;	/* always X_KBGetKbdByName */
    CARD16	length;
    CARD16	deviceSpec;
    CARD16	need;		/* combination of XkbGBN_* */
    CARD16	want;		/* combination of XkbGBN_* */
    BOOL	load;
    CARD8	pad;
} xkbGetKbdByNameReq;
#define	sz_xkbGetKbdByNameReq	12

typedef struct _xkbGetKbdByNameReply {
    CARD8	type;		/* always X_Reply */
    CARD8	deviceID;
    CARD16	sequenceNumber;
    CARD32	length;
    KeyCode	minKeyCode;
    KeyCode	maxKeyCode;
    BOOL	loaded;
    BOOL	newKeyboard;
    CARD16	found;		/* combination of XkbGBN_* */
    CARD16	reported;	/* combination of XkbAllComponents */
    CARD32	pad1;
    CARD32	pad2;
    CARD32	pad3;
    CARD32	pad4;
} xkbGetKbdByNameReply;
#define	sz_xkbGetKbdByNameReply	32

typedef	struct _xkbDeviceLedsWireDesc {
    CARD16	ledClass;
    CARD16	ledID;
    CARD32	namesPresent;
    CARD32	mapsPresent;
    CARD32	physIndicators;
    CARD32	state;
} xkbDeviceLedsWireDesc;
#define sz_xkbDeviceLedsWireDesc	20

typedef struct _xkbGetDeviceInfo {
    CARD8	reqType;
    CARD8	xkbReqType;	/* always X_KBGetDeviceInfo */
    CARD16	length;
    CARD16	deviceSpec;
    CARD16	wanted;
    BOOL	allBtns;
    CARD8	firstBtn;
    CARD8	nBtns;
    CARD8	pad;
    CARD16	ledClass;
    CARD16	ledID;
} xkbGetDeviceInfoReq;
#define	sz_xkbGetDeviceInfoReq	16

typedef struct _xkbGetDeviceInfoReply {
    CARD8	type;		/* always X_Reply */
    CARD8	deviceID;
    CARD16	sequenceNumber;
    CARD32	length;
    CARD16	present;
    CARD16	supported;
    CARD16	unsupported;
    CARD16	nDeviceLedFBs;
    CARD8	firstBtnWanted;
    CARD8	nBtnsWanted;
    CARD8	firstBtnRtrn;
    CARD8	nBtnsRtrn;
    CARD8	totalBtns;
    BOOL	hasOwnState;
    CARD16	dfltKbdFB;
    CARD16	dfltLedFB;
    CARD16	pad;
    Atom	devType;
} xkbGetDeviceInfoReply;
#define	sz_xkbGetDeviceInfoReply	32

typedef struct _xkbSetDeviceInfo {
    CARD8	reqType;
    CARD8	xkbReqType;	/* always X_KBSetDeviceInfo */
    CARD16	length;
    CARD16	deviceSpec;
    CARD8	firstBtn;
    CARD8	nBtns;
    CARD16	change;
    CARD16	nDeviceLedFBs;
} xkbSetDeviceInfoReq;
#define	sz_xkbSetDeviceInfoReq	12

typedef struct _xkbSetDebuggingFlags {
    CARD8	reqType;
    CARD8	xkbReqType;	/* always X_KBSetDebuggingFlags */
    CARD16	length;
    CARD16	msgLength;
    CARD16	pad;
    CARD32	affectFlags;
    CARD32	flags;
    CARD32	affectCtrls;
    CARD32	ctrls;
} xkbSetDebuggingFlagsReq;
#define	sz_xkbSetDebuggingFlagsReq	24

typedef struct _xkbSetDebuggingFlagsReply {
    BYTE	type;		/* X_Reply */
    CARD8	pad0;
    CARD16	sequenceNumber;
    CARD32	length;
    CARD32	currentFlags;
    CARD32	currentCtrls;
    CARD32	supportedFlags;
    CARD32	supportedCtrls;
    CARD32	pad1;
    CARD32	pad2;
} xkbSetDebuggingFlagsReply;
#define	sz_xkbSetDebuggingFlagsReply	32

	/*
	 * X KEYBOARD EXTENSION EVENT STRUCTURES
	 */

typedef struct _xkbAnyEvent {
    BYTE	type;
    BYTE	xkbType;
    CARD16	sequenceNumber;
    Time	time;
    CARD8	deviceID;
    CARD8	pad1;
    CARD16	pad2;
    CARD32	pad3;
    CARD32	pad4;
    CARD32	pad5;
    CARD32	pad6;
    CARD32	pad7;
} xkbAnyEvent;
#define	sz_xkbAnyEvent 32

typedef	struct _xkbNewKeyboardNotify {
    BYTE	type;
    BYTE	xkbType;
    CARD16	sequenceNumber;
    Time	time;
    CARD8	deviceID;
    CARD8	oldDeviceID;
    KeyCode	minKeyCode;
    KeyCode	maxKeyCode;
    KeyCode	oldMinKeyCode;
    KeyCode	oldMaxKeyCode;
    CARD8	requestMajor;
    CARD8	requestMinor;
    CARD16	changed;
    CARD8	detail;
    CARD8	pad1;
    CARD32	pad2;
    CARD32	pad3;
    CARD32	pad4;
} xkbNewKeyboardNotify;
#define	sz_xkbNewKeyboardNotify	32

typedef	struct _xkbMapNotify {
    BYTE	type;
    BYTE	xkbType;
    CARD16	sequenceNumber;
    Time	time;
    CARD8	deviceID;
    CARD8	ptrBtnActions;
    CARD16	changed;
    KeyCode	minKeyCode;
    KeyCode	maxKeyCode;
    CARD8	firstType;
    CARD8	nTypes;
    KeyCode	firstKeySym;
    CARD8	nKeySyms;
    KeyCode	firstKeyAct;
    CARD8	nKeyActs;
    KeyCode	firstKeyBehavior;
    CARD8	nKeyBehaviors;
    KeyCode	firstKeyExplicit;
    CARD8	nKeyExplicit;
    KeyCode	firstModMapKey;
    CARD8	nModMapKeys;
    KeyCode	firstVModMapKey;
    CARD8	nVModMapKeys;
    CARD16	virtualMods;
    CARD16	pad1;
} xkbMapNotify;
#define	sz_xkbMapNotify	32

typedef	struct _xkbStateNotify {
    BYTE	type;
    BYTE	xkbType;
    CARD16	sequenceNumber;
    Time	time;
    CARD8	deviceID;
    CARD8	mods;
    CARD8	baseMods;
    CARD8	latchedMods;
    CARD8	lockedMods;
    CARD8	group;
    INT16	baseGroup;
    INT16	latchedGroup;
    CARD8	lockedGroup;
    CARD8	compatState;
    CARD8	grabMods;
    CARD8	compatGrabMods;
    CARD8	lookupMods;
    CARD8	compatLookupMods;
    CARD16	ptrBtnState;
    CARD16	changed;
    KeyCode	keycode;
    CARD8	eventType;
    CARD8	requestMajor;
    CARD8	requestMinor;
} xkbStateNotify;
#define	sz_xkbStateNotify	32

typedef struct _xkbControlsNotify {
    BYTE	type;
    BYTE	xkbType;
    CARD16	sequenceNumber;
    Time	time;
    CARD8	deviceID;
    CARD8	numGroups;
    CARD16	pad1;
    CARD32	changedControls;
    CARD32	enabledControls;
    CARD32	enabledControlChanges;
    KeyCode	keycode;
    CARD8	eventType;
    CARD8	requestMajor;
    CARD8	requestMinor;
    CARD32	pad2;
} xkbControlsNotify;
#define	sz_xkbControlsNotify	32

typedef struct _xkbIndicatorNotify {
    BYTE	type;
    BYTE	xkbType;
    CARD16	sequenceNumber;
    Time	time;
    CARD8	deviceID;
    CARD8	pad1;
    CARD16	pad2;
    CARD32	state;
    CARD32	changed;
    CARD32	pad3;
    CARD32	pad4;
    CARD32	pad5;
} xkbIndicatorNotify;
#define	sz_xkbIndicatorNotify	32

typedef struct _xkbNamesNotify {
    BYTE	type;
    BYTE	xkbType;
    CARD16	sequenceNumber;
    Time	time;
    CARD8	deviceID;
    CARD8	pad1;
    CARD16	changed;
    CARD8	firstType;
    CARD8	nTypes;
    CARD8	firstLevelName;
    CARD8	nLevelNames;
    CARD8	pad2;
    CARD8	nRadioGroups;
    CARD8	nAliases;
    CARD8	changedGroupNames;
    CARD16	changedVirtualMods;
    CARD8	firstKey;
    CARD8	nKeys;
    CARD32	changedIndicators;
    CARD32	pad3;
} xkbNamesNotify;
#define	sz_xkbNamesNotify	32

typedef struct _xkbCompatMapNotify {
    BYTE	type;
    BYTE	xkbType;
    CARD16	sequenceNumber;
    Time	time;
    CARD8	deviceID;
    CARD8	changedGroups;
    CARD16	firstSI;
    CARD16	nSI;
    CARD16	nTotalSI;
    CARD32	pad1;
    CARD32	pad2;
    CARD32	pad3;
    CARD32	pad4;
} xkbCompatMapNotify;
#define sz_xkbCompatMapNotify	32

typedef struct _xkbBellNotify {
    BYTE	type;
    BYTE	xkbType;
    CARD16	sequenceNumber;
    Time	time;
    CARD8	deviceID;
    CARD8	bellClass;
    CARD8	bellID;
    CARD8	percent;
    CARD16	pitch;
    CARD16	duration;
    Atom	name;
    Window	window;
    BOOL	eventOnly;
    CARD8	pad1;
    CARD16	pad2;
    CARD32	pad3;
} xkbBellNotify;
#define	sz_xkbBellNotify	32

typedef struct _xkbActionMessage {
    BYTE	type;
    BYTE	xkbType;
    CARD16	sequenceNumber;
    Time	time;
    CARD8	deviceID;
    KeyCode	keycode;
    BOOL	press;
    BOOL	keyEventFollows;
    CARD8	mods;
    CARD8	group;
    CARD8	message[8];
    CARD16	pad1;
    CARD32	pad2;
    CARD32	pad3;
} xkbActionMessage;
#define	sz_xkbActionMessage		32

typedef struct _xkbAccessXNotify {
    BYTE	type;
    BYTE	xkbType;
    CARD16	sequenceNumber;
    Time	time;
    CARD8	deviceID;
    KeyCode	keycode;
    CARD16	detail;
    CARD16	slowKeysDelay;
    CARD16	debounceDelay;
    CARD32	pad1;
    CARD32	pad2;
    CARD32	pad3;
    CARD32	pad4;
} xkbAccessXNotify;
#define	sz_xkbAccessXNotify	32

typedef struct _xkbExtensionDeviceNotify {
    BYTE	type;
    BYTE	xkbType;
    CARD16	sequenceNumber;
    Time	time;
    CARD8	deviceID;
    CARD8	pad1;
    CARD16	reason;
    CARD16	ledClass;
    CARD16	ledID;
    CARD32	ledsDefined;
    CARD32	ledState;
    CARD8	firstBtn;
    CARD8	nBtns;
    CARD16	supported;
    CARD16	unsupported;
    CARD16	pad3;
} xkbExtensionDeviceNotify;
#define	sz_xkbExtensionDeviceNotify		32

typedef struct _xkbEvent {
    union {
	xkbAnyEvent		any;
	xkbNewKeyboardNotify	new_kbd;
	xkbMapNotify		map;
	xkbStateNotify		state;
	xkbControlsNotify	ctrls;
	xkbIndicatorNotify	indicators;
	xkbNamesNotify		names;
	xkbCompatMapNotify	compat;
	xkbBellNotify		bell;
	xkbActionMessage	message;
	xkbAccessXNotify	accessx;
	xkbExtensionDeviceNotify device;
    } u;
} xkbEvent;
#define sz_xkbEvent	32

#undef Window
#undef Atom
#undef Time
#undef KeyCode
#undef KeySym

#endif /* _XKBPROTO_H_ */
