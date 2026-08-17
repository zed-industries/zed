/* GTK - The GIMP Toolkit
 * Copyright (C) 2007  Openismus GmbH
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
 * License along with this library; if not, write to the
 * Free Software Foundation, Inc., 59 Temple Place - Suite 330,
 * Boston, MA 02111-1307, USA.
 *
 * Author:
 *   Mathias Hasselmann
 */

#ifndef __GTK_TOOL_SHELL_H__
#define __GTK_TOOL_SHELL_H__


#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtkenums.h>
#include <pango/pango.h>
#include <gtk/gtksizegroup.h>


G_BEGIN_DECLS

#define GTK_TYPE_TOOL_SHELL            (gtk_tool_shell_get_type ())
#define GTK_TOOL_SHELL(obj)            (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_TOOL_SHELL, GtkToolShell))
#define GTK_IS_TOOL_SHELL(obj)         (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_TOOL_SHELL))
#define GTK_TOOL_SHELL_GET_IFACE(obj)  (G_TYPE_INSTANCE_GET_INTERFACE ((obj), GTK_TYPE_TOOL_SHELL, GtkToolShellIface))

typedef struct _GtkToolShell           GtkToolShell; /* dummy typedef */
typedef struct _GtkToolShellIface      GtkToolShellIface;

/**
 * GtkToolShellIface:
 * @get_icon_size:        mandatory implementation of gtk_tool_shell_get_icon_size().
 * @get_orientation:      mandatory implementation of gtk_tool_shell_get_orientation().
 * @get_style:            mandatory implementation of gtk_tool_shell_get_style().
 * @get_relief_style:     optional implementation of gtk_tool_shell_get_relief_style().
 * @rebuild_menu:         optional implementation of gtk_tool_shell_rebuild_menu().
 * @get_text_orientation: optional implementation of gtk_tool_shell_get_text_orientation().
 * @get_text_alignment:   optional implementation of gtk_tool_shell_get_text_alignment().
 * @get_ellipsize_mode:   optional implementation of gtk_tool_shell_get_ellipsize_mode().
 * @get_text_size_group:  optional implementation of gtk_tool_shell_get_text_size_group().
 *
 * Virtual function table for the #GtkToolShell interface.
 */
struct _GtkToolShellIface
{
  /*< private >*/
  GTypeInterface g_iface;

  /*< public >*/
  GtkIconSize        (*get_icon_size)        (GtkToolShell *shell);
  GtkOrientation     (*get_orientation)      (GtkToolShell *shell);
  GtkToolbarStyle    (*get_style)            (GtkToolShell *shell);
  GtkReliefStyle     (*get_relief_style)     (GtkToolShell *shell);
  void               (*rebuild_menu)         (GtkToolShell *shell);
  GtkOrientation     (*get_text_orientation) (GtkToolShell *shell);
  gfloat             (*get_text_alignment)   (GtkToolShell *shell);
  PangoEllipsizeMode (*get_ellipsize_mode)   (GtkToolShell *shell);
  GtkSizeGroup *     (*get_text_size_group)  (GtkToolShell *shell);
};

GType              gtk_tool_shell_get_type             (void) G_GNUC_CONST;

GtkIconSize        gtk_tool_shell_get_icon_size        (GtkToolShell *shell);
GtkOrientation     gtk_tool_shell_get_orientation      (GtkToolShell *shell);
GtkToolbarStyle    gtk_tool_shell_get_style            (GtkToolShell *shell);
GtkReliefStyle     gtk_tool_shell_get_relief_style     (GtkToolShell *shell);
void               gtk_tool_shell_rebuild_menu         (GtkToolShell *shell);
GtkOrientation     gtk_tool_shell_get_text_orientation (GtkToolShell *shell);
gfloat             gtk_tool_shell_get_text_alignment   (GtkToolShell *shell);
PangoEllipsizeMode gtk_tool_shell_get_ellipsize_mode   (GtkToolShell *shell);
GtkSizeGroup *     gtk_tool_shell_get_text_size_group  (GtkToolShell *shell);

G_END_DECLS

#endif /* __GTK_TOOL_SHELL_H__ */
