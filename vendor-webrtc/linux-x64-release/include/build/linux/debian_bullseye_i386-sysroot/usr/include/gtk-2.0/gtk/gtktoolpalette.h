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
 * License along with this library; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin St, Fifth Floor, Boston, MA  02110-1301  USA
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

struct _GtkToolPaletteClass
{
  GtkContainerClass parent_class;

  void (*set_scroll_adjustments) (GtkWidget     *widget,
                                  GtkAdjustment *hadjustment,
                                  GtkAdjustment *vadjustment);

  /* Padding for future expansion */
  void (*_gtk_reserved1) (void);
  void (*_gtk_reserved2) (void);
  void (*_gtk_reserved3) (void);
  void (*_gtk_reserved4) (void);
  void (*_gtk_reserved5) (void);
  void (*_gtk_reserved6) (void);
};

GType                          gtk_tool_palette_get_type              (void) G_GNUC_CONST;
GtkWidget*                     gtk_tool_palette_new                   (void);

void                           gtk_tool_palette_set_group_position    (GtkToolPalette            *palette,
                                                                       GtkToolItemGroup          *group,
                                                                       gint                       position);
void                           gtk_tool_palette_set_exclusive         (GtkToolPalette            *palette,
                                                                       GtkToolItemGroup          *group,
                                                                       gboolean                   exclusive);
void                           gtk_tool_palette_set_expand            (GtkToolPalette            *palette,
                                                                       GtkToolItemGroup          *group,
                                                                       gboolean                   expand);

gint                           gtk_tool_palette_get_group_position    (GtkToolPalette            *palette,
                                                                       GtkToolItemGroup          *group);
gboolean                       gtk_tool_palette_get_exclusive         (GtkToolPalette            *palette,
                                                                       GtkToolItemGroup          *group);
gboolean                       gtk_tool_palette_get_expand            (GtkToolPalette            *palette,
                                                                       GtkToolItemGroup          *group);

void                           gtk_tool_palette_set_icon_size         (GtkToolPalette            *palette,
                                                                       GtkIconSize                icon_size);
void                           gtk_tool_palette_unset_icon_size       (GtkToolPalette            *palette);
void                           gtk_tool_palette_set_style             (GtkToolPalette            *palette,
                                                                       GtkToolbarStyle            style);
void                           gtk_tool_palette_unset_style           (GtkToolPalette            *palette);

GtkIconSize                    gtk_tool_palette_get_icon_size         (GtkToolPalette            *palette);
GtkToolbarStyle                gtk_tool_palette_get_style             (GtkToolPalette            *palette);

GtkToolItem*                   gtk_tool_palette_get_drop_item         (GtkToolPalette            *palette,
                                                                       gint                       x,
                                                                       gint                       y);
GtkToolItemGroup*              gtk_tool_palette_get_drop_group        (GtkToolPalette            *palette,
                                                                       gint                       x,
                                                                       gint                       y);
GtkWidget*                     gtk_tool_palette_get_drag_item         (GtkToolPalette            *palette,
                                                                       const GtkSelectionData    *selection);

void                           gtk_tool_palette_set_drag_source       (GtkToolPalette            *palette,
                                                                       GtkToolPaletteDragTargets  targets);
void                           gtk_tool_palette_add_drag_dest         (GtkToolPalette            *palette,
                                                                       GtkWidget                 *widget,
                                                                       GtkDestDefaults            flags,
                                                                       GtkToolPaletteDragTargets  targets,
                                                                       GdkDragAction              actions);

GtkAdjustment*                 gtk_tool_palette_get_hadjustment       (GtkToolPalette            *palette);
GtkAdjustment*                 gtk_tool_palette_get_vadjustment       (GtkToolPalette            *palette);

const GtkTargetEntry *         gtk_tool_palette_get_drag_target_item  (void) G_GNUC_CONST;
const GtkTargetEntry *         gtk_tool_palette_get_drag_target_group (void) G_GNUC_CONST;


G_END_DECLS

#endif /* __GTK_TOOL_PALETTE_H__ */
