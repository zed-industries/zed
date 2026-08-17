/* GtkToolPalette -- A tool palette with categories and DnD support
 * Copyright (C) 2008  Openismus GmbH
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 2.1 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with this library. If not, see <http://www.gnu.org/licenses/>.
 *
 * Authors:
 *      Mathias Hasselmann
 */

#ifndef __GTK_TOOL_PALETTE_H__
#define __GTK_TOOL_PALETTE_H__

#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtkcontainer.h>
#include <gtk/gtkdnd.h>
#include <gtk/gtktoolitem.h>

G_BEGIN_DECLS

#define GTK_TYPE_TOOL_PALETTE           (gtk_tool_palette_get_type ())
#define GTK_TOOL_PALETTE(obj)           (G_TYPE_CHECK_INSTANCE_CAST (obj, GTK_TYPE_TOOL_PALETTE, GtkToolPalette))
#define GTK_TOOL_PALETTE_CLASS(cls)     (G_TYPE_CHECK_CLASS_CAST (cls, GTK_TYPE_TOOL_PALETTE, GtkToolPaletteClass))
#define GTK_IS_TOOL_PALETTE(obj)        (G_TYPE_CHECK_INSTANCE_TYPE (obj, GTK_TYPE_TOOL_PALETTE))
#define GTK_IS_TOOL_PALETTE_CLASS(obj)  (G_TYPE_CHECK_CLASS_TYPE (obj, GTK_TYPE_TOOL_PALETTE))
#define GTK_TOOL_PALETTE_GET_CLASS(obj) (G_TYPE_INSTANCE_GET_CLASS ((obj), GTK_TYPE_TOOL_PALETTE, GtkToolPaletteClass))

typedef struct _GtkToolPalette           GtkToolPalette;
typedef struct _GtkToolPaletteClass      GtkToolPaletteClass;
typedef struct _GtkToolPalettePrivate    GtkToolPalettePrivate;

/**
 * GtkToolPaletteDragTargets:
 * @GTK_TOOL_PALETTE_DRAG_ITEMS: Support drag of items.
 * @GTK_TOOL_PALETTE_DRAG_GROUPS: Support drag of groups.
 *
 * Flags used to specify the supported drag targets.
 */
typedef enum /*< flags >*/
{
  GTK_TOOL_PALETTE_DRAG_ITEMS = (1 << 0),
  GTK_TOOL_PALETTE_DRAG_GROUPS = (1 << 1)
}
GtkToolPaletteDragTargets;

/**
 * GtkToolPalette:
 *
 * This should not be accessed directly. Use the accessor functions below.
 */
struct _GtkToolPalette
{
  GtkContainer parent_instance;
  GtkToolPalettePrivate *priv;
};

/**
 * GtkToolPaletteClass:
 * @parent_class: The parent class.
 */
struct _GtkToolPaletteClass
{
  GtkContainerClass parent_class;

  /*< private >*/

  /* Padding for future expansion */
  void (*_gtk_reserved1) (void);
  void (*_gtk_reserved2) (void);
  void (*_gtk_reserved3) (void);
  void (*_gtk_reserved4) (void);
};

GDK_AVAILABLE_IN_ALL
GType                          gtk_tool_palette_get_type              (void) G_GNUC_CONST;
GDK_AVAILABLE_IN_ALL
GtkWidget*                     gtk_tool_palette_new                   (void);

GDK_AVAILABLE_IN_ALL
void                           gtk_tool_palette_set_group_position    (GtkToolPalette            *palette,
                                                                       GtkToolItemGroup          *group,
                                                                       gint                       position);
GDK_AVAILABLE_IN_ALL
void                           gtk_tool_palette_set_exclusive         (GtkToolPalette            *palette,
                                                                       GtkToolItemGroup          *group,
                                                                       gboolean                   exclusive);
GDK_AVAILABLE_IN_ALL
void                           gtk_tool_palette_set_expand            (GtkToolPalette            *palette,
                                                                       GtkToolItemGroup          *group,
                                                                       gboolean                   expand);

GDK_AVAILABLE_IN_ALL
gint                           gtk_tool_palette_get_group_position    (GtkToolPalette            *palette,
                                                                       GtkToolItemGroup          *group);
GDK_AVAILABLE_IN_ALL
gboolean                       gtk_tool_palette_get_exclusive         (GtkToolPalette            *palette,
                                                                       GtkToolItemGroup          *group);
GDK_AVAILABLE_IN_ALL
gboolean                       gtk_tool_palette_get_expand            (GtkToolPalette            *palette,
                                                                       GtkToolItemGroup          *group);

GDK_AVAILABLE_IN_ALL
void                           gtk_tool_palette_set_icon_size         (GtkToolPalette            *palette,
                                                                       GtkIconSize                icon_size);
GDK_AVAILABLE_IN_ALL
void                           gtk_tool_palette_unset_icon_size       (GtkToolPalette            *palette);
GDK_AVAILABLE_IN_ALL
void                           gtk_tool_palette_set_style             (GtkToolPalette            *palette,
                                                                       GtkToolbarStyle            style);
GDK_AVAILABLE_IN_ALL
void                           gtk_tool_palette_unset_style           (GtkToolPalette            *palette);

GDK_AVAILABLE_IN_ALL
GtkIconSize                    gtk_tool_palette_get_icon_size         (GtkToolPalette            *palette);
GDK_AVAILABLE_IN_ALL
GtkToolbarStyle                gtk_tool_palette_get_style             (GtkToolPalette            *palette);

GDK_AVAILABLE_IN_ALL
GtkToolItem*                   gtk_tool_palette_get_drop_item         (GtkToolPalette            *palette,
                                                                       gint                       x,
                                                                       gint                       y);
GDK_AVAILABLE_IN_ALL
GtkToolItemGroup*              gtk_tool_palette_get_drop_group        (GtkToolPalette            *palette,
                                                                       gint                       x,
                                                                       gint                       y);
GDK_AVAILABLE_IN_ALL
GtkWidget*                     gtk_tool_palette_get_drag_item         (GtkToolPalette            *palette,
                                                                       const GtkSelectionData    *selection);

GDK_AVAILABLE_IN_ALL
void                           gtk_tool_palette_set_drag_source       (GtkToolPalette            *palette,
                                                                       GtkToolPaletteDragTargets  targets);
GDK_AVAILABLE_IN_ALL
void                           gtk_tool_palette_add_drag_dest         (GtkToolPalette            *palette,
                                                                       GtkWidget                 *widget,
                                                                       GtkDestDefaults            flags,
                                                                       GtkToolPaletteDragTargets  targets,
                                                                       GdkDragAction              actions);


GDK_DEPRECATED_IN_3_0_FOR(gtk_scrollable_get_hadjustment)
GtkAdjustment*                 gtk_tool_palette_get_hadjustment       (GtkToolPalette            *palette);
GDK_DEPRECATED_IN_3_0_FOR(gtk_scrollable_get_vadjustment)
GtkAdjustment*                 gtk_tool_palette_get_vadjustment       (GtkToolPalette            *palette);

GDK_AVAILABLE_IN_ALL
const GtkTargetEntry*          gtk_tool_palette_get_drag_target_item  (void) G_GNUC_CONST;
GDK_AVAILABLE_IN_ALL
const GtkTargetEntry*          gtk_tool_palette_get_drag_target_group (void) G_GNUC_CONST;


G_END_DECLS

#endif /* __GTK_TOOL_PALETTE_H__ */
