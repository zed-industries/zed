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

#ifndef __GDK_DND_H__
#define __GDK_DND_H__

#if !defined (__GDK_H_INSIDE__) && !defined (GDK_COMPILATION)
#error "Only <gdk/gdk.h> can be included directly."
#endif

#include <gdk/gdktypes.h>
#include <gdk/gdkdevice.h>
#include <gdk/gdkevents.h>

G_BEGIN_DECLS

#define GDK_TYPE_DRAG_CONTEXT              (gdk_drag_context_get_type ())
#define GDK_DRAG_CONTEXT(object)           (G_TYPE_CHECK_INSTANCE_CAST ((object), GDK_TYPE_DRAG_CONTEXT, GdkDragContext))
#define GDK_IS_DRAG_CONTEXT(object)        (G_TYPE_CHECK_INSTANCE_TYPE ((object), GDK_TYPE_DRAG_CONTEXT))

/**
 * GdkDragAction:
 * @GDK_ACTION_DEFAULT: Means nothing, and should not be used.
 * @GDK_ACTION_COPY: Copy the data.
 * @GDK_ACTION_MOVE: Move the data, i.e. first copy it, then delete
 *  it from the source using the DELETE target of the X selection protocol.
 * @GDK_ACTION_LINK: Add a link to the data. Note that this is only
 *  useful if source and destination agree on what it means.
 * @GDK_ACTION_PRIVATE: Special action which tells the source that the
 *  destination will do something that the source doesn’t understand.
 * @GDK_ACTION_ASK: Ask the user what to do with the data.
 *
 * Used in #GdkDragContext to indicate what the destination
 * should do with the dropped data.
 */
typedef enum
{
  GDK_ACTION_DEFAULT = 1 << 0,
  GDK_ACTION_COPY    = 1 << 1,
  GDK_ACTION_MOVE    = 1 << 2,
  GDK_ACTION_LINK    = 1 << 3,
  GDK_ACTION_PRIVATE = 1 << 4,
  GDK_ACTION_ASK     = 1 << 5
} GdkDragAction;

/**
 * GdkDragCancelReason:
 * @GDK_DRAG_CANCEL_NO_TARGET: There is no suitable drop target.
 * @GDK_DRAG_CANCEL_USER_CANCELLED: Drag cancelled by the user
 * @GDK_DRAG_CANCEL_ERROR: Unspecified error.
 *
 * Used in #GdkDragContext to the reason of a cancelled DND operation.
 *
 * Since: 3.20
 */
typedef enum {
  GDK_DRAG_CANCEL_NO_TARGET,
  GDK_DRAG_CANCEL_USER_CANCELLED,
  GDK_DRAG_CANCEL_ERROR
} GdkDragCancelReason;

/**
 * GdkDragProtocol:
 * @GDK_DRAG_PROTO_NONE: no protocol.
 * @GDK_DRAG_PROTO_MOTIF: The Motif DND protocol. No longer supported
 * @GDK_DRAG_PROTO_XDND: The Xdnd protocol.
 * @GDK_DRAG_PROTO_ROOTWIN: An extension to the Xdnd protocol for
 *  unclaimed root window drops.
 * @GDK_DRAG_PROTO_WIN32_DROPFILES: The simple WM_DROPFILES protocol.
 * @GDK_DRAG_PROTO_OLE2: The complex OLE2 DND protocol (not implemented).
 * @GDK_DRAG_PROTO_LOCAL: Intra-application DND.
 * @GDK_DRAG_PROTO_WAYLAND: Wayland DND protocol.
 *
 * Used in #GdkDragContext to indicate the protocol according to
 * which DND is done.
 */
typedef enum
{
  GDK_DRAG_PROTO_NONE = 0,
  GDK_DRAG_PROTO_MOTIF,
  GDK_DRAG_PROTO_XDND,
  GDK_DRAG_PROTO_ROOTWIN,
  GDK_DRAG_PROTO_WIN32_DROPFILES,
  GDK_DRAG_PROTO_OLE2,
  GDK_DRAG_PROTO_LOCAL,
  GDK_DRAG_PROTO_WAYLAND
} GdkDragProtocol;


GDK_AVAILABLE_IN_ALL
GType            gdk_drag_context_get_type             (void) G_GNUC_CONST;

GDK_AVAILABLE_IN_ALL
void             gdk_drag_context_set_device           (GdkDragContext *context,
                                                        GdkDevice      *device);
GDK_AVAILABLE_IN_ALL
GdkDevice *      gdk_drag_context_get_device           (GdkDragContext *context);

GDK_AVAILABLE_IN_ALL
GList           *gdk_drag_context_list_targets         (GdkDragContext *context);
GDK_AVAILABLE_IN_ALL
GdkDragAction    gdk_drag_context_get_actions          (GdkDragContext *context);
GDK_AVAILABLE_IN_ALL
GdkDragAction    gdk_drag_context_get_suggested_action (GdkDragContext *context);
GDK_AVAILABLE_IN_ALL
GdkDragAction    gdk_drag_context_get_selected_action  (GdkDragContext *context);

GDK_AVAILABLE_IN_ALL
GdkWindow       *gdk_drag_context_get_source_window    (GdkDragContext *context);
GDK_AVAILABLE_IN_ALL
GdkWindow       *gdk_drag_context_get_dest_window      (GdkDragContext *context);
GDK_AVAILABLE_IN_ALL
GdkDragProtocol  gdk_drag_context_get_protocol         (GdkDragContext *context);

/* Destination side */

GDK_AVAILABLE_IN_ALL
void             gdk_drag_status        (GdkDragContext   *context,
                                         GdkDragAction     action,
                                         guint32           time_);
GDK_AVAILABLE_IN_ALL
void             gdk_drop_reply         (GdkDragContext   *context,
                                         gboolean          accepted,
                                         guint32           time_);
GDK_AVAILABLE_IN_ALL
void             gdk_drop_finish        (GdkDragContext   *context,
                                         gboolean          success,
                                         guint32           time_);
GDK_AVAILABLE_IN_ALL
GdkAtom          gdk_drag_get_selection (GdkDragContext   *context);

/* Source side */

GDK_AVAILABLE_IN_ALL
GdkDragContext * gdk_drag_begin            (GdkWindow      *window,
                                            GList          *targets);

GDK_AVAILABLE_IN_ALL
GdkDragContext * gdk_drag_begin_for_device (GdkWindow      *window,
                                            GdkDevice      *device,
                                            GList          *targets);
GDK_AVAILABLE_IN_3_20
GdkDragContext * gdk_drag_begin_from_point  (GdkWindow      *window,
                                             GdkDevice      *device,
                                             GList          *targets,
                                             gint            x_root,
                                             gint            y_root);

GDK_AVAILABLE_IN_ALL
void    gdk_drag_find_window_for_screen   (GdkDragContext   *context,
                                           GdkWindow        *drag_window,
                                           GdkScreen        *screen,
                                           gint              x_root,
                                           gint              y_root,
                                           GdkWindow       **dest_window,
                                           GdkDragProtocol  *protocol);

GDK_AVAILABLE_IN_ALL
gboolean        gdk_drag_motion      (GdkDragContext *context,
                                      GdkWindow      *dest_window,
                                      GdkDragProtocol protocol,
                                      gint            x_root,
                                      gint            y_root,
                                      GdkDragAction   suggested_action,
                                      GdkDragAction   possible_actions,
                                      guint32         time_);
GDK_AVAILABLE_IN_ALL
void            gdk_drag_drop        (GdkDragContext *context,
                                      guint32         time_);
GDK_AVAILABLE_IN_ALL
void            gdk_drag_abort       (GdkDragContext *context,
                                      guint32         time_);
GDK_AVAILABLE_IN_ALL
gboolean        gdk_drag_drop_succeeded (GdkDragContext *context);

GDK_AVAILABLE_IN_3_20
void            gdk_drag_drop_done   (GdkDragContext *context,
                                      gboolean        success);

GDK_AVAILABLE_IN_3_20
GdkWindow      *gdk_drag_context_get_drag_window (GdkDragContext *context);

GDK_AVAILABLE_IN_3_20
void            gdk_drag_context_set_hotspot (GdkDragContext *context,
                                              gint            hot_x,
                                              gint            hot_y);

GDK_AVAILABLE_IN_3_20
gboolean        gdk_drag_context_manage_dnd (GdkDragContext *context,
                                             GdkWindow      *ipc_window,
                                             GdkDragAction   actions);
G_END_DECLS

#endif /* __GDK_DND_H__ */
