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
 * License along with this library; if not, write to the
 * Free Software Foundation, Inc., 59 Temple Place - Suite 330,
 * Boston, MA 02111-1307, USA.
 */

/*
 * Modified by the GTK+ Team and others 1997-2000.  See the AUTHORS
 * file for a list of people on the GTK+ Team.  See the ChangeLog
 * files for a list of changes.  These files are distributed with
 * GTK+ at ftp://ftp.gtk.org/pub/gtk/.
 */

#ifndef __GTK_ICON_FACTORY_H__
#define __GTK_ICON_FACTORY_H__

#if defined(GTK_DISABLE_SINGLE_INCLUDES) && !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gdk/gdk.h>
#include <gtk/gtkrc.h>

G_BEGIN_DECLS

typedef struct _GtkIconFactoryClass GtkIconFactoryClass;

#define GTK_TYPE_ICON_FACTORY              (gtk_icon_factory_get_type ())
#define GTK_ICON_FACTORY(object)           (G_TYPE_CHECK_INSTANCE_CAST ((object), GTK_TYPE_ICON_FACTORY, GtkIconFactory))
#define GTK_ICON_FACTORY_CLASS(klass)      (G_TYPE_CHECK_CLASS_CAST ((klass), GTK_TYPE_ICON_FACTORY, GtkIconFactoryClass))
#define GTK_IS_ICON_FACTORY(object)        (G_TYPE_CHECK_INSTANCE_TYPE ((object), GTK_TYPE_ICON_FACTORY))
#define GTK_IS_ICON_FACTORY_CLASS(klass)   (G_TYPE_CHECK_CLASS_TYPE ((klass), GTK_TYPE_ICON_FACTORY))
#define GTK_ICON_FACTORY_GET_CLASS(obj)    (G_TYPE_INSTANCE_GET_CLASS ((obj), GTK_TYPE_ICON_FACTORY, GtkIconFactoryClass))
#define GTK_TYPE_ICON_SET                  (gtk_icon_set_get_type ())
#define GTK_TYPE_ICON_SOURCE               (gtk_icon_source_get_type ())

struct _GtkIconFactory
{
  GObject parent_instance;

  GHashTable *GSEAL (icons);
};

struct _GtkIconFactoryClass
{
  GObjectClass parent_class;

  /* Padding for future expansion */
  void (*_gtk_reserved1) (void);
  void (*_gtk_reserved2) (void);
  void (*_gtk_reserved3) (void);
  void (*_gtk_reserved4) (void);
};

#ifdef G_OS_WIN32
/* Reserve old names for DLL ABI backward compatibility */
#define gtk_icon_source_set_filename gtk_icon_source_set_filename_utf8
#define gtk_icon_source_get_filename gtk_icon_source_get_filename_utf8
#endif

GType           gtk_icon_factory_get_type (void) G_GNUC_CONST;
GtkIconFactory* gtk_icon_factory_new      (void);
void            gtk_icon_factory_add      (GtkIconFactory *factory,
                                           const gchar    *stock_id,
                                           GtkIconSet     *icon_set);
GtkIconSet*     gtk_icon_factory_lookup   (GtkIconFactory *factory,
                                           const gchar    *stock_id);

/* Manage the default icon factory stack */

void        gtk_icon_factory_add_default     (GtkIconFactory  *factory);
void        gtk_icon_factory_remove_default  (GtkIconFactory  *factory);
GtkIconSet* gtk_icon_factory_lookup_default  (const gchar     *stock_id);

/* Get preferred real size from registered semantic size.  Note that
 * themes SHOULD use this size, but they aren't required to; for size
 * requests and such, you should get the actual pixbuf from the icon
 * set and see what size was rendered.
 *
 * This function is intended for people who are scaling icons,
 * rather than for people who are displaying already-scaled icons.
 * That is, if you are displaying an icon, you should get the
 * size from the rendered pixbuf, not from here.
 */

#ifndef GDK_MULTIHEAD_SAFE
gboolean gtk_icon_size_lookup              (GtkIconSize  size,
					    gint        *width,
					    gint        *height);
#endif /* GDK_MULTIHEAD_SAFE */
gboolean gtk_icon_size_lookup_for_settings (GtkSettings *settings,
					    GtkIconSize  size,
					    gint        *width,
					    gint        *height);

GtkIconSize           gtk_icon_size_register       (const gchar *name,
                                                    gint         width,
                                                    gint         height);
void                  gtk_icon_size_register_alias (const gchar *alias,
                                                    GtkIconSize  target);
GtkIconSize           gtk_icon_size_from_name      (const gchar *name);
const gchar *         gtk_icon_size_get_name       (GtkIconSize  size);

/* Icon sets */

GType       gtk_icon_set_get_type        (void) G_GNUC_CONST;
GtkIconSet* gtk_icon_set_new             (void);
GtkIconSet* gtk_icon_set_new_from_pixbuf (GdkPixbuf       *pixbuf);

GtkIconSet* gtk_icon_set_ref             (GtkIconSet      *icon_set);
void        gtk_icon_set_unref           (GtkIconSet      *icon_set);
GtkIconSet* gtk_icon_set_copy            (GtkIconSet      *icon_set);

/* Get one of the icon variants in the set, creating the variant if
 * necessary.
 */
GdkPixbuf*  gtk_icon_set_render_icon     (GtkIconSet      *icon_set,
                                          GtkStyle        *style,
                                          GtkTextDirection direction,
                                          GtkStateType     state,
                                          GtkIconSize      size,
                                          GtkWidget       *widget,
                                          const char      *detail);


void           gtk_icon_set_add_source   (GtkIconSet          *icon_set,
                                          const GtkIconSource *source);

void           gtk_icon_set_get_sizes    (GtkIconSet          *icon_set,
                                          GtkIconSize        **sizes,
                                          gint                *n_sizes);

GType          gtk_icon_source_get_type                 (void) G_GNUC_CONST;
GtkIconSource* gtk_icon_source_new                      (void);
GtkIconSource* gtk_icon_source_copy                     (const GtkIconSource *source);
void           gtk_icon_source_free                     (GtkIconSource       *source);

void           gtk_icon_source_set_filename             (GtkIconSource       *source,
                                                         const gchar         *filename);
void           gtk_icon_source_set_icon_name            (GtkIconSource       *source,
                                                         const gchar         *icon_name);
void           gtk_icon_source_set_pixbuf               (GtkIconSource       *source,
                                                         GdkPixbuf           *pixbuf);

const gchar* gtk_icon_source_get_filename  (const GtkIconSource *source);
const gchar* gtk_icon_source_get_icon_name (const GtkIconSource *source);
GdkPixbuf*            gtk_icon_source_get_pixbuf    (const GtkIconSource *source);

void             gtk_icon_source_set_direction_wildcarded (GtkIconSource       *source,
                                                           gboolean             setting);
void             gtk_icon_source_set_state_wildcarded     (GtkIconSource       *source,
                                                           gboolean             setting);
void             gtk_icon_source_set_size_wildcarded      (GtkIconSource       *source,
                                                           gboolean             setting);
gboolean         gtk_icon_source_get_size_wildcarded      (const GtkIconSource *source);
gboolean         gtk_icon_source_get_state_wildcarded     (const GtkIconSource *source);
gboolean         gtk_icon_source_get_direction_wildcarded (const GtkIconSource *source);
void             gtk_icon_source_set_direction            (GtkIconSource       *source,
                                                           GtkTextDirection     direction);
void             gtk_icon_source_set_state                (GtkIconSource       *source,
                                                           GtkStateType         state);
void             gtk_icon_source_set_size                 (GtkIconSource       *source,
                                                           GtkIconSize          size);
GtkTextDirection gtk_icon_source_get_direction            (const GtkIconSource *source);
GtkStateType     gtk_icon_source_get_state                (const GtkIconSource *source);
GtkIconSize      gtk_icon_source_get_size                 (const GtkIconSource *source);


/* ignore this */
void _gtk_icon_set_invalidate_caches (void);
GList* _gtk_icon_factory_list_ids (void);
void _gtk_icon_factory_ensure_default_icons (void);

G_END_DECLS

#endif /* __GTK_ICON_FACTORY_H__ */
