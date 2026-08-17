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

#ifndef _XKBGEOM_H_
#define	_XKBGEOM_H_

#include <X11/extensions/XKBstr.h>

#ifdef XKB_IN_SERVER
#define XkbAddGeomKeyAlias 		SrvXkbAddGeomKeyAlias
#define XkbAddGeomColor 		SrvXkbAddGeomColor
#define XkbAddGeomDoodad		SrvXkbAddGeomDoodad
#define XkbAddGeomKey			SrvXkbAddGeomKey
#define XkbAddGeomOutline		SrvXkbAddGeomOutline
#define XkbAddGeomOverlay		SrvXkbAddGeomOverlay
#define XkbAddGeomOverlayRow		SrvXkbAddGeomOverlayRow
#define	XkbAddGeomOverlayKey		SrvXkbAddGeomOverlayKey
#define XkbAddGeomProperty		SrvXkbAddGeomProperty
#define XkbAddGeomRow			SrvXkbAddGeomRow
#define XkbAddGeomSection		SrvXkbAddGeomSection
#define XkbAddGeomShape			SrvXkbAddGeomShape
#define XkbAllocGeomKeyAliases		SrvXkbAllocGeomKeyAliases
#define XkbAllocGeomColors		SrvXkbAllocGeomColors
#define XkbAllocGeomDoodads		SrvXkbAllocGeomDoodads
#define XkbAllocGeomKeys		SrvXkbAllocGeomKeys
#define XkbAllocGeomOutlines		SrvXkbAllocGeomOutlines
#define XkbAllocGeomPoints		SrvXkbAllocGeomPoints
#define XkbAllocGeomProps		SrvXkbAllocGeomProps
#define XkbAllocGeomRows		SrvXkbAllocGeomRows
#define XkbAllocGeomSectionDoodads	SrvXkbAllocGeomSectionDoodads
#define XkbAllocGeomSections		SrvXkbAllocGeomSections
#define	XkbAllocGeomOverlays		SrvXkbAllocGeomOverlays
#define	XkbAllocGeomOverlayRows		SrvXkbAllocGeomOverlayRows
#define	XkbAllocGeomOverlayKeys		SrvXkbAllocGeomOverlayKeys
#define XkbAllocGeomShapes		SrvXkbAllocGeomShapes
#define XkbAllocGeometry		SrvXkbAllocGeometry
#define XkbFreeGeomKeyAliases		SrvXkbFreeGeomKeyAliases
#define XkbFreeGeomColors		SrvXkbFreeGeomColors
#define XkbFreeGeomDoodads		SrvXkbFreeGeomDoodads
#define XkbFreeGeomProperties		SrvXkbFreeGeomProperties
#define	XkbFreeGeomOverlayKeys		SrvXkbFreeGeomOverlayKeys
#define	XkbFreeGeomOverlayRows		SrvXkbFreeGeomOverlayRows
#define	XkbFreeGeomOverlays		SrvXkbFreeGeomOverlays
#define	XkbFreeGeomKeys			SrvXkbFreeGeomKeys
#define	XkbFreeGeomRows			SrvXkbFreeGeomRows
#define XkbFreeGeomSections		SrvXkbFreeGeomSections
#define	XkbFreeGeomPoints		SrvXkbFreeGeomPoints
#define	XkbFreeGeomOutlines		SrvXkbFreeGeomOutlines
#define XkbFreeGeomShapes		SrvXkbFreeGeomShapes
#define XkbFreeGeometry			SrvXkbFreeGeometry
#endif

typedef	struct _XkbProperty {
	char	*name;
	char	*value;
} XkbPropertyRec,*XkbPropertyPtr;

typedef struct _XkbColor {
	unsigned int 	pixel;
	char *		spec;
} XkbColorRec,*XkbColorPtr;

typedef	struct _XkbPoint {
	short	x;
	short	y;
} XkbPointRec, *XkbPointPtr;

typedef struct	_XkbBounds {
	short	x1,y1;
	short	x2,y2;
} XkbBoundsRec, *XkbBoundsPtr;
#define	XkbBoundsWidth(b)	(((b)->x2)-((b)->x1))
#define	XkbBoundsHeight(b)	(((b)->y2)-((b)->y1))

/*
 * In the following structs, this pattern is used for dynamically sized arrays:
 * foo is an array for which sz_foo entries are allocated & num_foo are used
 */

typedef struct _XkbOutline {
	unsigned short	num_points;
	unsigned short	sz_points;
	unsigned short	corner_radius;
	XkbPointPtr	points;
} XkbOutlineRec, *XkbOutlinePtr;

typedef struct _XkbShape {
	Atom	 	 name;
	unsigned short	 num_outlines;
	unsigned short	 sz_outlines;
	XkbOutlinePtr	 outlines;
	XkbOutlinePtr	 approx;
	XkbOutlinePtr	 primary;
	XkbBoundsRec	 bounds;
} XkbShapeRec, *XkbShapePtr;
#define	XkbOutlineIndex(s,o)	((int)((o)-&(s)->outlines[0]))

typedef struct _XkbShapeDoodad {
	Atom		 name;
	unsigned char	 type;
	unsigned char	 priority;
	short		 top;
	short		 left;
	short	 	 angle;
	unsigned short	 color_ndx;
	unsigned short	 shape_ndx;
} XkbShapeDoodadRec, *XkbShapeDoodadPtr;
#define	XkbShapeDoodadColor(g,d)	(&(g)->colors[(d)->color_ndx])
#define	XkbShapeDoodadShape(g,d)	(&(g)->shapes[(d)->shape_ndx])
#define	XkbSetShapeDoodadColor(g,d,c)	((d)->color_ndx= (c)-&(g)->colors[0])
#define	XkbSetShapeDoodadShape(g,d,s)	((d)->shape_ndx= (s)-&(g)->shapes[0])

typedef struct _XkbTextDoodad {
	Atom		 name;
	unsigned char	 type;
	unsigned char	 priority;
	short	 	 top;
	short	 	 left;
	short	 	 angle;
	short	 	 width;
	short		 height;
	unsigned short	 color_ndx;
	char *		 text;
	char *		 font;
} XkbTextDoodadRec, *XkbTextDoodadPtr;
#define	XkbTextDoodadColor(g,d)	(&(g)->colors[(d)->color_ndx])
#define	XkbSetTextDoodadColor(g,d,c)	((d)->color_ndx= (c)-&(g)->colors[0])

typedef struct _XkbIndicatorDoodad {
	Atom		 name;
	unsigned char	 type;
	unsigned char	 priority;
	short	 	 top;
	short	 	 left;
	short		 angle;
	unsigned short	 shape_ndx;
	unsigned short	 on_color_ndx;
	unsigned short	 off_color_ndx;
} XkbIndicatorDoodadRec, *XkbIndicatorDoodadPtr;
#define	XkbIndicatorDoodadShape(g,d)	(&(g)->shapes[(d)->shape_ndx])
#define	XkbIndicatorDoodadOnColor(g,d)	(&(g)->colors[(d)->on_color_ndx])
#define	XkbIndicatorDoodadOffColor(g,d)	(&(g)->colors[(d)->off_color_ndx])
#define	XkbSetIndicatorDoodadOnColor(g,d,c) \
				((d)->on_color_ndx= (c)-&(g)->colors[0])
#define	XkbSetIndicatorDoodadOffColor(g,d,c) \
				((d)->off_color_ndx= (c)-&(g)->colors[0])
#define	XkbSetIndicatorDoodadShape(g,d,s) \
				((d)->shape_ndx= (s)-&(g)->shapes[0])

typedef struct _XkbLogoDoodad {
	Atom		 name;
	unsigned char	 type;
	unsigned char	 priority;
	short		 top;
	short		 left;
	short	 	 angle;
	unsigned short	 color_ndx;
	unsigned short	 shape_ndx;
	char *		 logo_name;
} XkbLogoDoodadRec, *XkbLogoDoodadPtr;
#define	XkbLogoDoodadColor(g,d)		(&(g)->colors[(d)->color_ndx])
#define	XkbLogoDoodadShape(g,d)		(&(g)->shapes[(d)->shape_ndx])
#define	XkbSetLogoDoodadColor(g,d,c)	((d)->color_ndx= (c)-&(g)->colors[0])
#define	XkbSetLogoDoodadShape(g,d,s)	((d)->shape_ndx= (s)-&(g)->shapes[0])

typedef struct _XkbAnyDoodad {
	Atom		 name;
	unsigned char	 type;
	unsigned char	 priority;
	short	 	 top;
	short	 	 left;
	short		 angle;
} XkbAnyDoodadRec, *XkbAnyDoodadPtr;

typedef union _XkbDoodad {
	XkbAnyDoodadRec		any;
	XkbShapeDoodadRec	shape;
	XkbTextDoodadRec	text;
	XkbIndicatorDoodadRec	indicator;
	XkbLogoDoodadRec	logo;
} XkbDoodadRec, *XkbDoodadPtr;

#define	XkbUnknownDoodad	0
#define	XkbOutlineDoodad	1
#define	XkbSolidDoodad		2
#define	XkbTextDoodad		3
#define	XkbIndicatorDoodad	4
#define	XkbLogoDoodad		5

typedef struct _XkbKey {
	XkbKeyNameRec	 name;
	short		 gap;
	unsigned char	 shape_ndx;
	unsigned char	 color_ndx;
} XkbKeyRec, *XkbKeyPtr;
#define	XkbKeyShape(g,k)	(&(g)->shapes[(k)->shape_ndx])
#define	XkbKeyColor(g,k)	(&(g)->colors[(k)->color_ndx])
#define	XkbSetKeyShape(g,k,s)	((k)->shape_ndx= (s)-&(g)->shapes[0])
#define	XkbSetKeyColor(g,k,c)	((k)->color_ndx= (c)-&(g)->colors[0])

typedef struct _XkbRow {
	short	 	top;
	short	 	left;
	unsigned short	num_keys;
	unsigned short	sz_keys;
	int		vertical;
	XkbKeyPtr	keys;
	XkbBoundsRec	bounds;
} XkbRowRec, *XkbRowPtr;

typedef struct _XkbSection {
	Atom		 name;
	unsigned char	 priority;
	short	 	 top;
	short	 	 left;
	unsigned short	 width;
	unsigned short	 height;
	short	 	 angle;
	unsigned short	 num_rows;
	unsigned short	 num_doodads;
	unsigned short	 num_overlays;
	unsigned short	 sz_rows;
	unsigned short	 sz_doodads;
	unsigned short	 sz_overlays;
	XkbRowPtr	 rows;
	XkbDoodadPtr	 doodads;
	XkbBoundsRec	 bounds;
	struct _XkbOverlay *overlays;
} XkbSectionRec, *XkbSectionPtr;

typedef	struct _XkbOverlayKey {
	XkbKeyNameRec	over;
	XkbKeyNameRec	under;
} XkbOverlayKeyRec,*XkbOverlayKeyPtr;

typedef struct _XkbOverlayRow {
	unsigned short		row_under;
	unsigned short		num_keys;
	unsigned short		sz_keys;
	XkbOverlayKeyPtr	keys;
} XkbOverlayRowRec,*XkbOverlayRowPtr;

typedef struct _XkbOverlay {
	Atom			name;
	XkbSectionPtr		section_under;
	unsigned short		num_rows;
	unsigned short		sz_rows;
	XkbOverlayRowPtr	rows;
	XkbBoundsPtr		bounds;
} XkbOverlayRec,*XkbOverlayPtr;

typedef struct _XkbGeometry {
	Atom		 name;
	unsigned short	 width_mm;
	unsigned short	 height_mm;
	char *		 label_font;
	XkbColorPtr	 label_color;
	XkbColorPtr	 base_color;
	unsigned short	 sz_properties;
	unsigned short	 sz_colors;
	unsigned short	 sz_shapes;
	unsigned short   sz_sections;
	unsigned short	 sz_doodads;
	unsigned short	 sz_key_aliases;
	unsigned short	 num_properties;
	unsigned short	 num_colors;
	unsigned short	 num_shapes;
	unsigned short	 num_sections;
	unsigned short	 num_doodads;
	unsigned short	 num_key_aliases;
	XkbPropertyPtr	 properties;
	XkbColorPtr	 colors;
	XkbShapePtr	 shapes;
	XkbSectionPtr	 sections;
	XkbDoodadPtr	 doodads;
	XkbKeyAliasPtr	 key_aliases;
} XkbGeometryRec;
#define	XkbGeomColorIndex(g,c)	((int)((c)-&(g)->colors[0]))

#define	XkbGeomPropertiesMask	(1<<0)
#define	XkbGeomColorsMask	(1<<1)
#define	XkbGeomShapesMask	(1<<2)
#define	XkbGeomSectionsMask	(1<<3)
#define	XkbGeomDoodadsMask	(1<<4)
#define	XkbGeomKeyAliasesMask	(1<<5)
#define	XkbGeomAllMask		(0x3f)

typedef struct _XkbGeometrySizes {
	unsigned int	which;
	unsigned short	num_properties;
	unsigned short	num_colors;
	unsigned short	num_shapes;
	unsigned short	num_sections;
	unsigned short	num_doodads;
	unsigned short	num_key_aliases;
} XkbGeometrySizesRec,*XkbGeometrySizesPtr;

_XFUNCPROTOBEGIN

extern	XkbPropertyPtr
XkbAddGeomProperty(
    XkbGeometryPtr	/* geom */,
    char *		/* name */,
    char *		/* value */
);

extern	XkbKeyAliasPtr
XkbAddGeomKeyAlias(
    XkbGeometryPtr	/* geom */,
    char *		/* alias */,
    char *		/* real */
);

extern	XkbColorPtr
XkbAddGeomColor(
    XkbGeometryPtr	/* geom */,
    char *		/* spec */,
    unsigned int	/* pixel */
);

extern	XkbOutlinePtr
XkbAddGeomOutline(
    XkbShapePtr		/* shape */,
    int			/* sz_points */
);

extern XkbShapePtr
XkbAddGeomShape(
    XkbGeometryPtr	/* geom */,
    Atom		/* name */,
    int			/* sz_outlines */
);

extern XkbKeyPtr
XkbAddGeomKey(
    XkbRowPtr		/* row */
);

extern XkbRowPtr
XkbAddGeomRow(
    XkbSectionPtr	/* section */,
    int			/* sz_keys */
);

extern XkbSectionPtr
XkbAddGeomSection(
    XkbGeometryPtr	/* geom */,
    Atom		/* name */,
    int			/* sz_rows */,
    int			/* sz_doodads */,
    int			/* sz_overlays */
);

extern XkbOverlayPtr
XkbAddGeomOverlay(
    XkbSectionPtr	/* section */,
    Atom		/* name */,
    int			/* sz_rows */
);

extern XkbOverlayRowPtr
XkbAddGeomOverlayRow(
    XkbOverlayPtr	/* overlay */,
    int			/* row_under */,
    int			/* sz_keys */
);

extern XkbOverlayKeyPtr
XkbAddGeomOverlayKey(
    XkbOverlayPtr	/* overlay */,
    XkbOverlayRowPtr	/* row */,
    char *		/* over */,
    char *		/* under */
);

extern XkbDoodadPtr
XkbAddGeomDoodad(
    XkbGeometryPtr	/* geom */,
    XkbSectionPtr	/* section */,
    Atom		/* name */
);


extern void
XkbFreeGeomKeyAliases(
    XkbGeometryPtr	/* geom */,
    int			/* first */,
    int			/* count */,
    Bool		/* freeAll */
);

extern void
XkbFreeGeomColors(
    XkbGeometryPtr	/* geom */,
    int			/* first */,
    int			/* count */,
    Bool		/* freeAll */
);

extern void
XkbFreeGeomDoodads(
    XkbDoodadPtr	/* doodads */,
    int			/* nDoodads */,
    Bool		/* freeAll */
);


extern void
XkbFreeGeomProperties(
    XkbGeometryPtr	/* geom */,
    int			/* first */,
    int			/* count */,
    Bool		/* freeAll */
);

extern void
XkbFreeGeomOverlayKeys(
    XkbOverlayRowPtr	/* row */,
    int			/* first */,
    int			/* count */,
    Bool		/* freeAll */
);

extern void
XkbFreeGeomOverlayRows(
    XkbOverlayPtr	/* overlay */,
    int			/* first */,
    int			/* count */,
    Bool		/* freeAll */
);

extern void
XkbFreeGeomOverlays(
    XkbSectionPtr	/* section */,
    int			/* first */,
    int			/* count */,
    Bool		/* freeAll */
);

extern void
XkbFreeGeomKeys(
    XkbRowPtr		/* row */,
    int			/* first */,
    int			/* count */,
    Bool		/* freeAll */
);

extern void
XkbFreeGeomRows(
    XkbSectionPtr	/* section */,
    int			/* first */,
    int			/* count */,
    Bool		/* freeAll */
);

extern void
XkbFreeGeomSections(
    XkbGeometryPtr	/* geom */,
    int			/* first */,
    int			/* count */,
    Bool		/* freeAll */
);


extern void
XkbFreeGeomPoints(
    XkbOutlinePtr	/* outline */,
    int			/* first */,
    int			/* count */,
    Bool		/* freeAll */
);

extern void
XkbFreeGeomOutlines(
    XkbShapePtr		/* shape */,
    int			/* first */,
    int			/* count */,
    Bool		/* freeAll */
);

extern void
XkbFreeGeomShapes(
    XkbGeometryPtr	/* geom */,
    int			/* first */,
    int			/* count */,
    Bool		/* freeAll */
);

extern void
XkbFreeGeometry(
    XkbGeometryPtr	/* geom */,
    unsigned int	/* which */,
    Bool		/* freeMap */
);

extern Status
XkbAllocGeomProps(
    XkbGeometryPtr	/* geom */,
    int			/* nProps */
);

extern Status
XkbAllocGeomKeyAliases(
    XkbGeometryPtr	/* geom */,
    int			/* nAliases */
);

extern Status
XkbAllocGeomColors(
    XkbGeometryPtr	/* geom */,
    int			/* nColors */
);

extern Status
XkbAllocGeomShapes(
    XkbGeometryPtr	/* geom */,
    int			/* nShapes */
);

extern Status
XkbAllocGeomSections(
    XkbGeometryPtr	/* geom */,
    int			/* nSections */
);

extern Status
XkbAllocGeomOverlays(
    XkbSectionPtr	/* section */,
    int			/* num_needed */
);

extern Status
XkbAllocGeomOverlayRows(
    XkbOverlayPtr	/* overlay */,
    int			/* num_needed */
);

extern Status
XkbAllocGeomOverlayKeys(
    XkbOverlayRowPtr	/* row */,
    int			/* num_needed */
);

extern Status
XkbAllocGeomDoodads(
    XkbGeometryPtr	/* geom */,
    int			/* nDoodads */
);

extern Status
XkbAllocGeomSectionDoodads(
    XkbSectionPtr	/* section */,
    int			/* nDoodads */
);

extern Status
XkbAllocGeomOutlines(
    XkbShapePtr		/* shape */,
    int			/* nOL */
);

extern Status
XkbAllocGeomRows(
    XkbSectionPtr	/* section */,
    int			/* nRows */
);

extern Status
XkbAllocGeomPoints(
    XkbOutlinePtr	/* ol */,
    int			/* nPts */
);

extern Status
XkbAllocGeomKeys(
    XkbRowPtr		/* row */,
    int			/* nKeys */
);

extern	Status
XkbAllocGeometry(
	XkbDescPtr		/* xkb */,
	XkbGeometrySizesPtr	/* sizes */
);

extern	Status
XkbSetGeometry(
	Display *		/* dpy */,
	unsigned		/* deviceSpec */,
	XkbGeometryPtr		/* geom */
);

extern	Bool
XkbComputeShapeTop(
	XkbShapePtr		/* shape */,
	XkbBoundsPtr		/* bounds */
);

extern	Bool
XkbComputeShapeBounds(
	XkbShapePtr		/* shape */
);

extern	Bool
XkbComputeRowBounds(
	XkbGeometryPtr		/* geom */,
	XkbSectionPtr		/* section */,
	XkbRowPtr		/* row */
);

extern	Bool
XkbComputeSectionBounds(
	XkbGeometryPtr		/* geom */,
	XkbSectionPtr		/* section */
);

extern	char *
XkbFindOverlayForKey(
	XkbGeometryPtr		/* geom */,
	XkbSectionPtr		/* wanted */,
	char *			/* under */
);

extern	Status
XkbGetGeometry(
    Display *			/* dpy */,
    XkbDescPtr			/* xkb */
);

extern	Status
XkbGetNamedGeometry(
    Display *			/* dpy */,
    XkbDescPtr			/* xkb */,
    Atom			/* name */
);

_XFUNCPROTOEND

#endif /* _XKBSTR_H_ */
