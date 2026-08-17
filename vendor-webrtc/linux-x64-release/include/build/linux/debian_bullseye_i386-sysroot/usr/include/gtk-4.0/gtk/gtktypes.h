/* GDK - The GIMP Drawing Kit
 * Copyright (C) 1995-1997 Peter Mattis, Spencer Kimball and Josh MacDonald
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 2 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with this library. If not, see <http://www.gnu.org/licenses/>.
 */

/*
 * Modified by the GTK+ Team and others 1997-2000.  See the AUTHORS
 * file for a list of people on the GTK+ Team.  See the ChangeLog
 * files for a list of changes.  These files are distributed with
 * GTK+ at ftp://ftp.gtk.org/pub/gtk/.
 */

#ifndef __GTK_TYPES_H__
#define __GTK_TYPES_H__

#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gdk/gdk.h>

G_BEGIN_DECLS

typedef struct _GtkAdjustment          GtkAdjustment;
typedef struct _GtkBitset              GtkBitset;
typedef struct _GtkBuilder             GtkBuilder;
typedef struct _GtkBuilderScope        GtkBuilderScope;
typedef struct _GtkCssStyleChange      GtkCssStyleChange;
typedef struct _GtkEventController     GtkEventController;
typedef struct _GtkGesture             GtkGesture;
typedef struct _GtkLayoutManager       GtkLayoutManager;
typedef struct _GtkListItem            GtkListItem;
typedef struct _GtkListItemFactory     GtkListItemFactory;
typedef struct _GtkNative              GtkNative;
typedef struct _GtkRequisition	       GtkRequisition;
typedef struct _GtkRoot  	       GtkRoot;
typedef struct _GtkSettings            GtkSettings;
typedef struct _GtkShortcut            GtkShortcut;
typedef struct _GtkShortcutAction      GtkShortcutAction;
typedef struct _GtkShortcutTrigger     GtkShortcutTrigger;
typedef GdkSnapshot                    GtkSnapshot;
typedef struct _GtkStyleContext        GtkStyleContext;
typedef struct _GtkTooltip             GtkTooltip;
typedef struct _GtkWidget              GtkWidget;
typedef struct _GtkWindow              GtkWindow;

/**
 * GTK_INVALID_LIST_POSITION:
 *
 * The value used to refer to a guaranteed invalid position
 * in a `GListModel`.
 *
 * This value may be returned from some functions, others may
 * accept it as input. Its interpretation may differ for different
 * functions.
 *
 * Refer to each function's documentation for if this value is
 * allowed and what it does.
 */
#define GTK_INVALID_LIST_POSITION ((guint) 0xffffffff)

G_END_DECLS

#endif /* __GTK_TYPES_H__ */
