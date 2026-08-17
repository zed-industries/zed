/* GTK - The GIMP Toolkit
 * Copyright (C) 2010 Carlos Garnacho <carlosg@gnome.org>
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

#ifndef __GTK_STYLE_PROPERTIES_H__
#define __GTK_STYLE_PROPERTIES_H__

#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <glib-object.h>
#include <gdk/gdk.h>
#include <gtk/gtkenums.h>

G_BEGIN_DECLS

#define GTK_TYPE_STYLE_PROPERTIES         (gtk_style_properties_get_type ())
#define GTK_STYLE_PROPERTIES(o)           (G_TYPE_CHECK_INSTANCE_CAST ((o), GTK_TYPE_STYLE_PROPERTIES, GtkStyleProperties))
#define GTK_STYLE_PROPERTIES_CLASS(c)     (G_TYPE_CHECK_CLASS_CAST    ((c), GTK_TYPE_STYLE_PROPERTIES, GtkStylePropertiesClass))
#define GTK_IS_STYLE_PROPERTIES(o)        (G_TYPE_CHECK_INSTANCE_TYPE ((o), GTK_TYPE_STYLE_PROPERTIES))
#define GTK_IS_STYLE_PROPERTIES_CLASS(c)  (G_TYPE_CHECK_CLASS_TYPE    ((c), GTK_TYPE_STYLE_PROPERTIES))
#define GTK_STYLE_PROPERTIES_GET_CLASS(o) (G_TYPE_INSTANCE_GET_CLASS  ((o), GTK_TYPE_STYLE_PROPERTIES, GtkStylePropertiesClass))

typedef struct _GtkStyleProperties GtkStyleProperties;
typedef struct _GtkStylePropertiesClass GtkStylePropertiesClass;
typedef struct _GtkStylePropertiesPrivate GtkStylePropertiesPrivate;

typedef struct _GtkSymbolicColor GtkSymbolicColor;
typedef struct _GtkGradient GtkGradient;

struct _GtkStyleProperties
{
  /*< private >*/
  GObject parent_object;
  GtkStylePropertiesPrivate *priv;
};

struct _GtkStylePropertiesClass
{
  /*< private >*/
  GObjectClass parent_class;

  /* Padding for future expansion */
  void (*_gtk_reserved1) (void);
  void (*_gtk_reserved2) (void);
  void (*_gtk_reserved3) (void);
  void (*_gtk_reserved4) (void);
};

typedef gboolean (* GtkStylePropertyParser) (const gchar  *string,
                                             GValue       *value,
                                             GError      **error);

GDK_DEPRECATED_IN_3_16
GType gtk_style_properties_get_type (void) G_GNUC_CONST;

/* Next 2 are implemented in gtkcsscustomproperty.c */
GDK_DEPRECATED_IN_3_8
void     gtk_style_properties_register_property (GtkStylePropertyParser  parse_func,
                                                 GParamSpec             *pspec);
GDK_DEPRECATED_IN_3_8
gboolean gtk_style_properties_lookup_property   (const gchar             *property_name,
                                                 GtkStylePropertyParser  *parse_func,
                                                 GParamSpec             **pspec);

GDK_DEPRECATED_IN_3_16
GtkStyleProperties * gtk_style_properties_new (void);

GDK_DEPRECATED_IN_3_8
void               gtk_style_properties_map_color    (GtkStyleProperties *props,
                                                      const gchar        *name,
                                                      GtkSymbolicColor   *color);
GDK_DEPRECATED_IN_3_8
GtkSymbolicColor * gtk_style_properties_lookup_color (GtkStyleProperties *props,
                                                      const gchar        *name);

GDK_DEPRECATED_IN_3_16
void     gtk_style_properties_set_property (GtkStyleProperties *props,
                                            const gchar        *property,
                                            GtkStateFlags       state,
                                            const GValue       *value);
GDK_DEPRECATED_IN_3_16
void     gtk_style_properties_set_valist   (GtkStyleProperties *props,
                                            GtkStateFlags       state,
                                            va_list             args);
GDK_DEPRECATED_IN_3_16
void     gtk_style_properties_set          (GtkStyleProperties *props,
                                            GtkStateFlags       state,
                                            ...) G_GNUC_NULL_TERMINATED;

GDK_DEPRECATED_IN_3_16
gboolean gtk_style_properties_get_property (GtkStyleProperties *props,
                                            const gchar        *property,
                                            GtkStateFlags       state,
                                            GValue             *value);
GDK_DEPRECATED_IN_3_16
void     gtk_style_properties_get_valist   (GtkStyleProperties *props,
                                            GtkStateFlags       state,
                                            va_list             args);
GDK_DEPRECATED_IN_3_16
void     gtk_style_properties_get          (GtkStyleProperties *props,
                                            GtkStateFlags       state,
                                            ...) G_GNUC_NULL_TERMINATED;

GDK_DEPRECATED_IN_3_16
void     gtk_style_properties_unset_property (GtkStyleProperties *props,
                                              const gchar        *property,
                                              GtkStateFlags       state);

GDK_DEPRECATED_IN_3_16
void     gtk_style_properties_clear          (GtkStyleProperties  *props);

GDK_DEPRECATED_IN_3_16
void     gtk_style_properties_merge          (GtkStyleProperties       *props,
                                              const GtkStyleProperties *props_to_merge,
                                              gboolean                  replace);

G_END_DECLS

#endif /* __GTK_STYLE_PROPERTIES_H__ */
