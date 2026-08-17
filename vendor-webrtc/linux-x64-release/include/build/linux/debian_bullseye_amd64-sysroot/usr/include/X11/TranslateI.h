/***********************************************************

Copyright 1987, 1988, 1998  The Open Group

Permission to use, copy, modify, distribute, and sell this software and its
documentation for any purpose is hereby granted without fee, provided that
the above copyright notice appear in all copies and that both that
copyright notice and this permission notice appear in supporting
documentation.

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL THE
OPEN GROUP BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN
AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

Except as contained in this notice, the name of The Open Group shall not be
used in advertising or otherwise to promote the sale, use or other dealings
in this Software without prior written authorization from The Open Group.


Copyright 1987, 1988 by Digital Equipment Corporation, Maynard, Massachusetts.

                        All Rights Reserved

Permission to use, copy, modify, and distribute this software and its
documentation for any purpose and without fee is hereby granted,
provided that the above copyright notice appear in all copies and that
both that copyright notice and this permission notice appear in
supporting documentation, and that the name of Digital not be
used in advertising or publicity pertaining to distribution of the
software without specific, written prior permission.

DIGITAL DISCLAIMS ALL WARRANTIES WITH REGARD TO THIS SOFTWARE, INCLUDING
ALL IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS, IN NO EVENT SHALL
DIGITAL BE LIABLE FOR ANY SPECIAL, INDIRECT OR CONSEQUENTIAL DAMAGES OR
ANY DAMAGES WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS,
WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION,
ARISING OUT OF OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS
SOFTWARE.

******************************************************************/

/*
 * TranslateI.h - Header file private to translation management
 *
 * Author:	Gabe Beged-Dov, HP
 *
 * Former Author:	Charles Haynes
 * 		Digital Equipment Corporation
 * 		Western Research Laboratory
 * Date:	Sat Aug 29 1987
 */

/*#define REFCNT_TRANSLATIONS*/
#define CACHE_TRANSLATIONS

#define TM_NO_MATCH (-2)

#define _XtRStateTablePair "_XtStateTablePair"

typedef unsigned char TMByteCard;
typedef unsigned short TMShortCard;
typedef unsigned long TMLongCard;
typedef short TMShortInt;

typedef struct _TMTypeMatchRec *TMTypeMatch;
typedef struct _TMModifierMatchRec *TMModifierMatch;
typedef struct _TMEventRec *TMEventPtr;

typedef Boolean (*MatchProc)(TMTypeMatch typeMatch,
			     TMModifierMatch modMatch,
			     TMEventPtr eventSeq);

typedef struct _ModToKeysymTable {
    Modifiers mask;
    int count;
    int idx;
} ModToKeysymTable;

typedef struct _LateBindings {
    unsigned int knot:1;
    unsigned int pair:1;
    unsigned short ref_count;	/* garbage collection */
    KeySym keysym;
} LateBindings, *LateBindingsPtr;

typedef short ModifierMask;

typedef struct _ActionsRec *ActionPtr;
typedef struct _ActionsRec {
    int idx;			/* index into quarkTable to find proc */
    String *params;		/* pointer to array of params */
    Cardinal num_params;	/* number of params */
    ActionPtr next;		/* next action to perform */
} ActionRec;

typedef struct _XtStateRec *StatePtr;
typedef struct _XtStateRec {
    unsigned int	isCycleStart:1;
    unsigned int	isCycleEnd:1;
    TMShortCard		typeIndex;
    TMShortCard		modIndex;
    ActionPtr		actions;	/* rhs list of actions to perform */
    StatePtr 		nextLevel;
}StateRec;


#define XtTableReplace	0
#define XtTableAugment	1
#define XtTableOverride	2
#define XtTableUnmerge  3

typedef unsigned int _XtTranslateOp;

/*
 * New Definitions
 */
typedef struct _TMModifierMatchRec{
    TMLongCard	 modifiers;
    TMLongCard	 modifierMask;
    LateBindingsPtr lateModifiers;
    Boolean	 standard;
}TMModifierMatchRec;

typedef struct _TMTypeMatchRec{
    TMLongCard	 eventType;
    TMLongCard	 eventCode;
    TMLongCard	 eventCodeMask;
    MatchProc	 matchEvent;
}TMTypeMatchRec;

typedef struct _TMBranchHeadRec {
    unsigned int	isSimple:1;
    unsigned int	hasActions:1;
    unsigned int	hasCycles:1;
    unsigned int	more:13;
    TMShortCard		typeIndex;
    TMShortCard		modIndex;
}TMBranchHeadRec, *TMBranchHead;

/* NOTE: elements of this structure must match those of
 * TMComplexStateTreeRec and TMParseStateTreeRec.
 */
typedef struct _TMSimpleStateTreeRec{
    unsigned int	isSimple:1;
    unsigned int	isAccelerator:1;
    unsigned int	mappingNotifyInterest:1;
    unsigned int	refCount:13;
    TMShortCard		numBranchHeads;
    TMShortCard		numQuarks;   /* # of entries in quarkTbl */
    TMShortCard		unused;	     /* to ensure same alignment */
    TMBranchHeadRec	*branchHeadTbl;
    XrmQuark		*quarkTbl;  /* table of quarkified rhs*/
}TMSimpleStateTreeRec, *TMSimpleStateTree;

/* NOTE: elements of this structure must match those of
 * TMSimpleStateTreeRec and TMParseStateTreeRec.
 */
typedef struct _TMComplexStateTreeRec{
    unsigned int	isSimple:1;
    unsigned int	isAccelerator:1;
    unsigned int	mappingNotifyInterest:1;
    unsigned int	refCount:13;
    TMShortCard		numBranchHeads;
    TMShortCard		numQuarks;   /* # of entries in quarkTbl */
    TMShortCard		numComplexBranchHeads;
    TMBranchHeadRec	*branchHeadTbl;
    XrmQuark		*quarkTbl;  /* table of quarkified rhs*/
    StatePtr		*complexBranchHeadTbl;
}TMComplexStateTreeRec, *TMComplexStateTree;

/* NOTE: elements of this structure must match those of
 * TMSimpleStateTreeRec and TMComplexStateTreeRec.
 */
typedef struct _TMParseStateTreeRec{
    unsigned int	isSimple:1;
    unsigned int	isAccelerator:1;
    unsigned int	mappingNotifyInterest:1;
    unsigned int	isStackQuarks:1;
    unsigned int	isStackBranchHeads:1;
    unsigned int	isStackComplexBranchHeads:1;
    unsigned int	unused:10; /* to ensure correct alignment */
    TMShortCard		numBranchHeads;
    TMShortCard		numQuarks;   /* # of entries in quarkTbl */
    TMShortCard		numComplexBranchHeads;
    TMBranchHeadRec	*branchHeadTbl;
    XrmQuark		*quarkTbl;  /* table of quarkified rhs*/
    StatePtr		*complexBranchHeadTbl;
    TMShortCard		branchHeadTblSize;
    TMShortCard		quarkTblSize; /*total size of quarkTbl */
    TMShortCard		complexBranchHeadTblSize;
    StatePtr		head;
}TMParseStateTreeRec, *TMParseStateTree;

typedef union _TMStateTreeRec{
    TMSimpleStateTreeRec	simple;
    TMParseStateTreeRec		parse;
    TMComplexStateTreeRec	complex;
}*TMStateTree, **TMStateTreePtr, **TMStateTreeList;

typedef struct _TMSimpleBindProcsRec {
    XtActionProc	*procs;
}TMSimpleBindProcsRec, *TMSimpleBindProcs;

typedef struct _TMComplexBindProcsRec {
    Widget	 	widget;		/*widgetID to pass to action Proc*/
    XtTranslations	aXlations;
    XtActionProc	*procs;
}TMComplexBindProcsRec, *TMComplexBindProcs;

typedef struct _TMSimpleBindDataRec {
    unsigned int		isComplex:1;	/* must be first */
    TMSimpleBindProcsRec	bindTbl[1];	/* variable length */
}TMSimpleBindDataRec, *TMSimpleBindData;

typedef struct _TMComplexBindDataRec {
    unsigned int		isComplex:1;	/* must be first */
    struct _ATranslationData	*accel_context;	/* for GetValues */
    TMComplexBindProcsRec	bindTbl[1]; 	/* variable length */
}TMComplexBindDataRec, *TMComplexBindData;

typedef union _TMBindDataRec{
    TMSimpleBindDataRec		simple;
    TMComplexBindDataRec	complex;
}*TMBindData;

typedef struct _TranslationData{
    unsigned char		hasBindings;	/* must be first */
    unsigned char		operation; /*replace,augment,override*/
    TMShortCard			numStateTrees;
    struct _TranslationData    	*composers[2];
    EventMask			eventMask;
    TMStateTree			stateTreeTbl[1]; /* variable length */
}TranslationData;

/*
 * ATranslations is returned by GetValues for translations that contain
 * accelerators.  The TM can differentiate between this and TranslationData
 * (that don't have a bindTbl) by looking at the first field (hasBindings)
 * of either structure.  All ATranslationData structures associated with a
 * widget are chained off the BindData record of the widget.
 */
typedef struct _ATranslationData{
    unsigned char		hasBindings;	/* must be first */
    unsigned char		operation;
    struct _TranslationData	*xlations;  /* actual translations */
    struct _ATranslationData	*next;      /* chain the contexts together */
    TMComplexBindProcsRec	bindTbl[1]; /* accelerator bindings */
}ATranslationData, *ATranslations;

typedef struct _TMConvertRec {
    XtTranslations	old; /* table to merge into */
    XtTranslations	new; /* table to merge from */
} TMConvertRec;

#define _XtEventTimerEventType ((TMLongCard)~0L)
#define KeysymModMask		(1L<<27) /* private to TM */
#define AnyButtonMask		(1L<<28) /* private to TM */

typedef struct _EventRec {
    TMLongCard modifiers;
    TMLongCard modifierMask;
    LateBindingsPtr lateModifiers;
    TMLongCard eventType;
    TMLongCard eventCode;
    TMLongCard eventCodeMask;
    MatchProc matchEvent;
    Boolean standard;
} Event;

typedef struct _EventSeqRec *EventSeqPtr;
typedef struct _EventSeqRec {
    Event event;	/* X event description */
    StatePtr state;	/* private to state table builder */
    EventSeqPtr next;	/* next event on line */
    ActionPtr actions;	/* r.h.s.   list of actions to perform */
} EventSeqRec;

typedef EventSeqRec EventRec;
typedef EventSeqPtr EventPtr;

typedef struct _TMEventRec {
    XEvent *xev;
    Event event;
}TMEventRec;

typedef struct _ActionHookRec {
    struct _ActionHookRec* next; /* must remain first */
    XtAppContext app;
    XtActionHookProc proc;
    XtPointer closure;
} ActionHookRec, *ActionHook;

/* choose a number between 2 and 8 */
#define TMKEYCACHELOG2 6
#define TMKEYCACHESIZE (1<<TMKEYCACHELOG2)

typedef struct _KeyCacheRec {
    unsigned char modifiers_return[256]; /* constant per KeyCode, key proc */
    KeyCode keycode[TMKEYCACHESIZE];
    unsigned char modifiers[TMKEYCACHESIZE];
    KeySym keysym[TMKEYCACHESIZE];
} TMKeyCache;

typedef struct _TMKeyContextRec {
    XEvent *event;
    unsigned long serial;
    KeySym keysym;
    Modifiers modifiers;
    TMKeyCache keycache;  /* keep this last, to keep offsets to others small */
} TMKeyContextRec, *TMKeyContext;

typedef struct _TMGlobalRec{
    TMTypeMatchRec 		**typeMatchSegmentTbl;
    TMShortCard			numTypeMatches;
    TMShortCard			numTypeMatchSegments;
    TMShortCard			typeMatchSegmentTblSize;
    TMModifierMatchRec 		**modMatchSegmentTbl;
    TMShortCard			numModMatches;
    TMShortCard			numModMatchSegments;
    TMShortCard			modMatchSegmentTblSize;
    Boolean			newMatchSemantics;
#ifdef TRACE_TM
    XtTranslations		*tmTbl;
    TMShortCard			numTms;
    TMShortCard			tmTblSize;
    struct _TMBindCacheRec	**bindCacheTbl;
    TMShortCard			numBindCache;
    TMShortCard			bindCacheTblSize;
    TMShortCard			numLateBindings;
    TMShortCard			numBranchHeads;
    TMShortCard			numComplexStates;
    TMShortCard			numComplexActions;
#endif /* TRACE_TM */
}TMGlobalRec;

_XFUNCPROTOBEGIN

extern TMGlobalRec _XtGlobalTM;

#define TM_MOD_SEGMENT_SIZE 	16
#define TM_TYPE_SEGMENT_SIZE 	16

#define TMGetTypeMatch(idx) \
  ((TMTypeMatch) \
   &((_XtGlobalTM.typeMatchSegmentTbl[((idx) >> 4)])[(idx) & 15]))
#define TMGetModifierMatch(idx) \
  ((TMModifierMatch) \
   &((_XtGlobalTM.modMatchSegmentTbl[(idx) >> 4])[(idx) & 15]))

/* Useful Access Macros */
#define TMNewMatchSemantics() (_XtGlobalTM.newMatchSemantics)
#define TMBranchMore(branch) (branch->more)
#define TMComplexBranchHead(tree, br) \
  (((TMComplexStateTree)tree)->complexBranchHeadTbl[TMBranchMore(br)])

#define TMGetComplexBindEntry(bindData, idx) \
  ((TMComplexBindProcs)&(((TMComplexBindData)bindData)->bindTbl[idx]))

#define TMGetSimpleBindEntry(bindData, idx) \
  ((TMSimpleBindProcs)&(((TMSimpleBindData)bindData)->bindTbl[idx]))


#define _InitializeKeysymTables(dpy, pd) \
    if (pd->keysyms == NULL) \
        _XtBuildKeysymTables(dpy, pd)

/*
 * Internal Functions
 */

extern void _XtPopup(
    Widget      /* widget */,
    XtGrabKind  /* grab_kind */,
    _XtBoolean	/* spring_loaded */
);

extern _XtString _XtPrintXlations(
    Widget		/* w */,
    XtTranslations 	/* xlations */,
    Widget		/* accelWidget */,
    _XtBoolean		/* includeRHS */
);

extern void _XtRegisterGrabs(
    Widget	/* widget */
);

extern XtPointer _XtInitializeActionData(
    struct _XtActionsRec *	/* actions */,
    Cardinal 			/* count */,
    _XtBoolean			/* inPlace */
);

extern void _XtAddEventSeqToStateTree(
    EventSeqPtr		/* eventSeq */,
    TMParseStateTree	/* stateTree */
);

extern Boolean _XtMatchUsingStandardMods(
    TMTypeMatch		/* typeMatch */,
    TMModifierMatch	/* modMatch */,
    TMEventPtr		/* eventSeq */
);

extern Boolean _XtMatchUsingDontCareMods(
    TMTypeMatch		/* typeMatch */,
    TMModifierMatch	/* modMatch */,
    TMEventPtr		/* eventSeq */
);

extern Boolean _XtRegularMatch(
    TMTypeMatch		/* typeMatch */,
    TMModifierMatch	/* modMatch */,
    TMEventPtr		/* eventSeq */
);

extern Boolean _XtMatchAtom(
    TMTypeMatch		/* typeMatch */,
    TMModifierMatch	/* modMatch */,
    TMEventPtr		/* eventSeq */
);

extern void _XtTranslateEvent(
    Widget		/* widget */,
    XEvent*		/* event */
);

#include "CallbackI.h"
#include "EventI.h"
#include "HookObjI.h"
#include "PassivGraI.h"
#include "ThreadsI.h"
#include "InitialI.h"
#include "ResourceI.h"
#include "StringDefs.h"

extern void _XtBuildKeysymTables(Display *dpy, XtPerDisplay pd);

#ifndef NO_MIT_HACKS
extern void  _XtDisplayTranslations(
    Widget		/* widget */,
    XEvent*		/* event */,
    String*		/* params */,
    Cardinal*		/* num_params */
);

extern void  _XtDisplayAccelerators(
    Widget		/* widget */,
    XEvent*		/* event */,
    String*		/* params */,
    Cardinal*		/* num_params */
);

extern void _XtDisplayInstalledAccelerators(
    Widget		/* widget */,
    XEvent*		/* event */,
    String*		/* params */,
    Cardinal*		/* num_params */
);
#endif /* ifndef NO_MIT_HACKS */

extern void _XtPopupInitialize(
    XtAppContext	/* app_context */
);

extern void _XtBindActions(
    Widget	/* widget */,
    XtTM 	/* tm_rec */
);

extern Boolean _XtComputeLateBindings(
    Display*		/* dpy */,
    LateBindingsPtr	/* lateModifiers */,
    Modifiers*		/* computed */,
    Modifiers*		/* computedMask */
);

extern XtTranslations _XtCreateXlations(
    TMStateTree *	/* stateTrees */,
    TMShortCard		/* numStateTrees */,
    XtTranslations 	/* first */,
    XtTranslations	/* second */
);

extern Boolean _XtCvtMergeTranslations(
    Display*	/* dpy */,
    XrmValuePtr	/* args */,
    Cardinal*	/* num_args */,
    XrmValuePtr	/* from */,
    XrmValuePtr	/* to */,
    XtPointer*	/* closure_ret */
);

void _XtRemoveStateTreeByIndex(
    XtTranslations	/* xlations */,
    TMShortCard	/* i */);

void _XtFreeTranslations(
    XtAppContext	/* app */,
    XrmValuePtr		/* toVal */,
    XtPointer		/* closure */,
    XrmValuePtr		/* args */,
    Cardinal*		/* num_args */
);

extern TMShortCard _XtGetModifierIndex(
    Event*	/* event */
);

extern TMShortCard _XtGetQuarkIndex(
    TMParseStateTree	/* stateTreePtr */,
    XrmQuark		/* quark */
);

extern XtTranslations _XtGetTranslationValue(
    Widget		/* widget */
);

extern TMShortCard _XtGetTypeIndex(
    Event*	/* event */
);

extern void _XtGrabInitialize(
    XtAppContext	/* app */
);

extern void _XtInstallTranslations(
    Widget		/* widget */
);

extern void _XtRemoveTranslations(
    Widget		/* widget */
);

extern void _XtDestroyTMData(
    Widget		/* widget */
);

extern void _XtMergeTranslations(
    Widget		/* widget */,
    XtTranslations	/* newXlations */,
    _XtTranslateOp	/* operation */
);

extern void _XtActionInitialize(
    XtAppContext	/* app */
);

extern TMStateTree _XtParseTreeToStateTree(
    TMParseStateTree 	/* parseTree */
);

extern String _XtPrintActions(
    ActionRec*	/* actions */,
    XrmQuark*	/* quarkTbl */
);

extern String _XtPrintState(
    TMStateTree	/* stateTree */,
    TMBranchHead /* branchHead */);

extern String _XtPrintEventSeq(
    EventSeqPtr	/* eventSeq */,
    Display*	/* dpy */
);

typedef Boolean (*_XtTraversalProc)(
    StatePtr	/* state */,
    XtPointer	/* data */
);

extern void _XtTraverseStateTree(
    TMStateTree		/* tree */,
    _XtTraversalProc	/* func */,
    XtPointer		/* data */
);

extern void _XtTranslateInitialize(
    void
);

extern void _XtAddTMConverters(
    ConverterTable	/* table */
);

extern void _XtUnbindActions(
    Widget		/* widget */,
    XtTranslations	/* xlations */,
    TMBindData		/* bindData */
);

extern void _XtUnmergeTranslations(
    Widget		/* widget */,
    XtTranslations 	/* xlations */
);

/* TMKey.c */
extern void _XtAllocTMContext(XtPerDisplay pd);

_XFUNCPROTOEND
