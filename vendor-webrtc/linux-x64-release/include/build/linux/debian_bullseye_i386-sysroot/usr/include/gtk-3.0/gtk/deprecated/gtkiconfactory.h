/* GTK - The GIMP Toolkit
 * Copyright (C) 2000 Red Hat, Inc.
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 2 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.	 See the GNU
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

#ifndef __GTK_ICON_FACTORY_H__
#define __GTK_ICON_FACTORY_H__

#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gdk/gdk.h>
#include <gtk/gtkenums.h>
#include <gtk/gtktypes.h>

G_BEGIN_DECLS


#define GTK_TYPE_ICON_FACTORY              (gtk_icon_factory_get_type ())
#define GTK_ICON_FACTORY(object)           (G_TYPE_CHECK_INSTANCE_CAST ((object), GTK_TYPE_ICON_FACTORY, GtkIconFactory))
#define GTK_ICON_FACTORY_CLASS(klass)      (G_TYPE_CHECK_CLASS_CAST ((klass), GTK_TYPE_ICON_FACTORY, GtkIconFactoryClass))
#define GTK_IS_ICON_FACTORY(object)        (G_TYPE_CHECK_INSTANCE_TYPE ((object), GTK_TYPE_ICON_FACTORY))
#define GTK_IS_ICON_FACTORY_CLASS(klass)   (G_TYPE_CHECK_CLASS_TYPE ((klass), GTK_TYPE_ICON_FACTORY))
#define GTK_ICON_FACTORY_GET_CLASS(obj)    (G_TYPE_INSTANCE_GET_CLASS ((obj), GTK_TYPE_ICON_FACTORY, GtkIconFactoryClass))
#define GTK_TYPE_ICON_SET                  (gtk_icon_set_get_type ())
#define GTK_TYPE_ICON_SOURCE               (gtk_icon_source_get_type ())

typedef struct _GtkIconFactory              GtkIconFactory;
typedef struct _GtkIconFactoryPrivate       GtkIconFactoryPrivate;
typedef struct _GtkIconFactoryClass         GtkIconFactoryClass;

struct _GtkIconFactory
{
  GObject parent_instance;

  /*< private >*/
  GtkIconFactoryPrivate *priv;
};

/**
 * GtkIconFactoryClass:
 * @parent_class: The parent class.
 */
struct _GtkIconFactoryClass
{
  GObjectClass parent_class;

  /*< private >*/

  /* Padding for future expansion */
  void (*_gtk_reserved1) (void);
  void (*_gtk_reserved2) (void);
  void (*_gtk_reserved3) (void);
  void (*_gtk_reserved4) (void);
};

GDK_DEPRECATED_IN_3_10
GType           gtk_icon_factory_get_type (void) G_GNUC_CONST;
GDK_DEPRECATED_IN_3_10
GtkIconFactory* gtk_icon_factory_new      (void);
GDK_DEPRECATED_IN_3_10
void            gtk_icon_factory_add      (GtkIconFactory *factory,
                                           const gchar    *stock_id,
                                           GtkIconSet     *icon_set);
GDK_DEPRECATED_IN_3_10
GtkIconSet*     gtk_icon_factory_lookup   (GtkIconFactory *factory,
                                           const gchar    *stock_id);

/* Manage the default icon factory stack */

GDK_DEPRECATED_IN_3_10
void        gtk_icon_factory_add_default     (GtkIconFactory  *factory);
GDK_DEPRECATED_IN_3_10
void        gtk_icon_factory_remove_default  (GtkIconFactory  *factory);
GDK_DEPRECATED_IN_3_10
GtkIconSet* gtk_icon_factory_lookup_default  (const gchar     *stock_id);

/* Get preferred real size from registered semantic size.  Note that
 * themes SHOULD use this size, but they aren’t required to; for size
 * requests and such, you should get the actual pixbuf from the icon
 * set and see what size was rendered.
 *
 * This function is intended for people who are scaling icons,
 * rather than for people who are displaying already-scaled icons.
 * That is, if you are displaying an icon, you should get the
 * size from the rendered pixbuf, not from here.
 */

#ifndef GDK_MULTIHEAD_SAFE
GDK_AVAILABLE_IN_ALL
gboolean gtk_icon_size_lookup              (GtkIconSize  size,
					    gint        *width,
					    gint        *height);
#endif /* GDK_MULTIHEAD_SAFE */
GDK_DEPRECATED_IN_3_10_FOR(gtk_icon_size_lookup)
gboolean gtk_icon_size_lookup_for_settings (GtkSettings *settings,
					    GtkIconSize  size,
					    gint        *width,
					    gint        *height);

GDK_DEPRECATED_IN_3_10
GtkIconSize           gtk_icon_size_register       (const gchar *name,
                                                    gint         width,
                                                    gint         height);
GDK_DEPRECATED_IN_3_10
void                  gtk_icon_size_register_alias (const gchar *alias,
                                                    GtkIconSize  target);
GDK_DEPRECATED_IN_3_10
GtkIconSize           gtk_icon_size_from_name      (const gchar *name);
GDK_DEPRECATED_IN_3_10
const gchar*          gtk_icon_size_get_name       (GtkIconSize  size);

/* Icon sets */

GDK_DEPRECATED_IN_3_10
GType       gtk_icon_set_get_type        (void) G_GNUC_CONST;
GDK_DEPRECATED_IN_3_10
GtkIconSet* gtk_icon_set_new             (void);
GDK_DEPRECATED_IN_3_10
GtkIconSet* gtk_icon_set_new_from_pixbuf (GdkPixbuf       *pixbuf);

GDK_DEPRECATED_IN_3_10
GtkIconSet* gtk_icon_set_ref             (GtkIconSet      *icon_set);
GDK_DEPRECATED_IN_3_10
void        gtk_icon_set_unref           (GtkIconSet      *icon_set);
GDK_DEPRECATED_IN_3_10
GtkIconSet* gtk_icon_set_copy            (GtkIconSet      *icon_set);

GDK_DEPRECATED_IN_3_0_FOR(gtk_icon_set_render_icon_pixbuf)
GdkPixbuf*  gtk_icon_set_render_icon     (GtkIconSet      *icon_set,
                                          GtkStyle        *style,
                                          GtkTextDirection direction,
                                          GtkStateType     state,
                                          GtkIconSize      size,
                                          GtkWidget       *widget,
                                          const gchar     *detail);

GDK_DEPRECATED_IN_3_10
void           gtk_icon_set_add_source   (GtkIconSet          *icon_set,
                                          const GtkIconSource *source);

GDK_DEPRECATED_IN_3_10
void           gtk_icon_set_get_sizes    (GtkIconSet          *icon_set,
                                          GtkIconSize        **sizes,
                                          gint                *n_sizes);

GDK_DEPRECATED_IN_3_10
GType          gtk_icon_source_get_type                 (void) G_GNUC_CONST;
GDK_DEPRECATED_IN_3_10
GtkIconSource* gtk_icon_source_new                      (void);
GDK_DEPRECATED_IN_3_10
GtkIconSource* gtk_icon_source_copy                     (const GtkIconSource *source);
GDK_DEPRECATED_IN_3_10
void           gtk_icon_source_free                     (GtkIconSource       *source);

GDK_DEPRECATED_IN_3_10
void           gtk_icon_source_set_filename             (GtkIconSource       *source,
                                                         const gchar         *filename);
GDK_DEPRECATED_IN_3_10
void           gtk_icon_source_set_icon_name            (GtkIconSource       *source,
                                                         const gchar         *icon_name);
GDK_DEPRECATED_IN_3_10
void           gtk_icon_source_set_pixbuf               (GtkIconSource       *source,
                                                         GdkPixbuf           *pixbuf);

GDK_DEPRECATED_IN_3_10
const gchar *    gtk_icon_source_get_filename             (const GtkIconSource *source);
GDK_DEPRECATED_IN_3_10
const gchar *    gtk_icon_source_get_icon_name            (const GtkIconSource *source);
GDK_DEPRECATED_IN_3_10
GdkPixbuf*       gtk_icon_source_get_pixbuf               (const GtkIconSource *source);

GDK_DEPRECATED_IN_3_10
void             gtk_icon_source_set_direction_wildcarded (GtkIconSource       *source,
                                                           gboolean             setting);
GDK_DEPRECATED_IN_3_10
void             gtk_icon_source_set_state_wildcarded     (GtkIconSource       *source,
                                                           gboolean             setting);
GDK_DEPRECATED_IN_3_10
void             gtk_icon_source_set_size_wildcarded      (GtkIconSource       *source,
                                                           gboolean             setting);
GDK_DEPRECATED_IN_3_10
gboolean         gtk_icon_source_get_size_wildcarded      (const GtkIconSource *source);
GDK_DEPRECATED_IN_3_10
gboolean         gtk_icon_source_get_state_wildcarded     (const GtkIconSource *source);
GDK_DEPRECATED_IN_3_10
gboolean         gtk_icon_source_get_direction_wildcarded (const GtkIconSource *source);
GDK_DEPRECATED_IN_3_10
void             gtk_icon_source_set_direction            (GtkIconSource       *source,
                                                           GtkTextDirection     direction);
GDK_DEPRECATED_IN_3_10
void             gtk_icon_source_set_state                (GtkIconSource       *source,
                                                           GtkStateType         state);
GDK_DEPRECATED_IN_3_10
void             gtk_icon_source_set_size                 (GtkIconSource       *source,
                                                           GtkIconSize          size);
GDK_DEPRECATED_IN_3_10
GtkTextDirection gtk_icon_source_get_direction            (const GtkIconSource *source);
GDK_DEPRECATED_IN_3_10
GtkStateType     gtk_icon_source_get_state                (const GtkIconSource *source);
GDK_DEPRECATED_IN_3_10
GtkIconSize      gtk_icon_source_get_size                 (const GtkIconSource *source);

G_END_DECLS

#endif /* __GTK_ICON_FACTORY_H__ */
