/* GTK - The GIMP Toolkit
 * Copyright (C) 2006-2007 Async Open Source,
 *                         Johan Dahlin <jdahlin@async.com.br>
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Library General Public
 * License as published by the Free Software Foundation; either
 * version 2 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Library General Public License for more details.
 *
 * You should have received a copy of the GNU Library General Public
 * License along with this library. If not, see <http://www.gnu.org/licenses/>.
 */

#ifndef __GTK_BUILDER_H__
#define __GTK_BUILDER_H__

#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtkapplication.h>
#include <gtk/gtkwidget.h>

G_BEGIN_DECLS

#define GTK_TYPE_BUILDER                 (gtk_builder_get_type ())
#define GTK_BUILDER(obj)                 (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_BUILDER, GtkBuilder))
#define GTK_BUILDER_CLASS(klass)         (G_TYPE_CHECK_CLASS_CAST ((klass), GTK_TYPE_BUILDER, GtkBuilderClass))
#define GTK_IS_BUILDER(obj)              (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_BUILDER))
#define GTK_IS_BUILDER_CLASS(klass)      (G_TYPE_CHECK_CLASS_TYPE ((klass), GTK_TYPE_BUILDER))
#define GTK_BUILDER_GET_CLASS(obj)       (G_TYPE_INSTANCE_GET_CLASS ((obj), GTK_TYPE_BUILDER, GtkBuilderClass))

#define GTK_BUILDER_ERROR                (gtk_builder_error_quark ())

typedef struct _GtkBuilderClass   GtkBuilderClass;
typedef struct _GtkBuilderPrivate GtkBuilderPrivate;

/**
 * GtkBuilderError:
 * @GTK_BUILDER_ERROR_INVALID_TYPE_FUNCTION: A type-func attribute didn’t name
 *  a function that returns a #GType.
 * @GTK_BUILDER_ERROR_UNHANDLED_TAG: The input contained a tag that #GtkBuilder
 *  can’t handle.
 * @GTK_BUILDER_ERROR_MISSING_ATTRIBUTE: An attribute that is required by
 *  #GtkBuilder was missing.
 * @GTK_BUILDER_ERROR_INVALID_ATTRIBUTE: #GtkBuilder found an attribute that
 *  it doesn’t understand.
 * @GTK_BUILDER_ERROR_INVALID_TAG: #GtkBuilder found a tag that
 *  it doesn’t understand.
 * @GTK_BUILDER_ERROR_MISSING_PROPERTY_VALUE: A required property value was
 *  missing.
 * @GTK_BUILDER_ERROR_INVALID_VALUE: #GtkBuilder couldn’t parse
 *  some attribute value.
 * @GTK_BUILDER_ERROR_VERSION_MISMATCH: The input file requires a newer version
 *  of GTK+.
 * @GTK_BUILDER_ERROR_DUPLICATE_ID: An object id occurred twice.
 * @GTK_BUILDER_ERROR_OBJECT_TYPE_REFUSED: A specified object type is of the same type or
 *  derived from the type of the composite class being extended with builder XML.
 * @GTK_BUILDER_ERROR_TEMPLATE_MISMATCH: The wrong type was specified in a composite class’s template XML
 * @GTK_BUILDER_ERROR_INVALID_PROPERTY: The specified property is unknown for the object class.
 * @GTK_BUILDER_ERROR_INVALID_SIGNAL: The specified signal is unknown for the object class.
 * @GTK_BUILDER_ERROR_INVALID_ID: An object id is unknown
 *
 * Error codes that identify various errors that can occur while using
 * #GtkBuilder.
 */
typedef enum
{
  GTK_BUILDER_ERROR_INVALID_TYPE_FUNCTION,
  GTK_BUILDER_ERROR_UNHANDLED_TAG,
  GTK_BUILDER_ERROR_MISSING_ATTRIBUTE,
  GTK_BUILDER_ERROR_INVALID_ATTRIBUTE,
  GTK_BUILDER_ERROR_INVALID_TAG,
  GTK_BUILDER_ERROR_MISSING_PROPERTY_VALUE,
  GTK_BUILDER_ERROR_INVALID_VALUE,
  GTK_BUILDER_ERROR_VERSION_MISMATCH,
  GTK_BUILDER_ERROR_DUPLICATE_ID,
  GTK_BUILDER_ERROR_OBJECT_TYPE_REFUSED,
  GTK_BUILDER_ERROR_TEMPLATE_MISMATCH,
  GTK_BUILDER_ERROR_INVALID_PROPERTY,
  GTK_BUILDER_ERROR_INVALID_SIGNAL,
  GTK_BUILDER_ERROR_INVALID_ID
} GtkBuilderError;

GDK_AVAILABLE_IN_ALL
GQuark gtk_builder_error_quark (void);

struct _GtkBuilder
{
  GObject parent_instance;

  GtkBuilderPrivate *priv;
};

struct _GtkBuilderClass
{
  GObjectClass parent_class;
  
  GType (* get_type_from_name) (GtkBuilder *builder,
                                const char *type_name);

  /* Padding for future expansion */
  void (*_gtk_reserved1) (void);
  void (*_gtk_reserved2) (void);
  void (*_gtk_reserved3) (void);
  void (*_gtk_reserved4) (void);
  void (*_gtk_reserved5) (void);
  void (*_gtk_reserved6) (void);
  void (*_gtk_reserved7) (void);
  void (*_gtk_reserved8) (void);
};

GDK_AVAILABLE_IN_ALL
GType        gtk_builder_get_type                (void) G_GNUC_CONST;
GDK_AVAILABLE_IN_ALL
GtkBuilder*  gtk_builder_new                     (void);

GDK_AVAILABLE_IN_ALL
guint        gtk_builder_add_from_file           (GtkBuilder    *builder,
                                                  const gchar   *filename,
                                                  GError       **error);
GDK_AVAILABLE_IN_ALL
guint        gtk_builder_add_from_resource       (GtkBuilder    *builder,
                                                  const gchar   *resource_path,
                                                  GError       **error);
GDK_AVAILABLE_IN_ALL
guint        gtk_builder_add_from_string         (GtkBuilder    *builder,
                                                  const gchar   *buffer,
                                                  gsize          length,
                                                  GError       **error);
GDK_AVAILABLE_IN_ALL
guint        gtk_builder_add_objects_from_file   (GtkBuilder    *builder,
                                                  const gchar   *filename,
                                                  gchar        **object_ids,
                                                  GError       **error);
GDK_AVAILABLE_IN_3_4
guint        gtk_builder_add_objects_from_resource(GtkBuilder    *builder,
                                                  const gchar   *resource_path,
                                                  gchar        **object_ids,
                                                  GError       **error);
GDK_AVAILABLE_IN_ALL
guint        gtk_builder_add_objects_from_string (GtkBuilder    *builder,
                                                  const gchar   *buffer,
                                                  gsize          length,
                                                  gchar        **object_ids,
                                                  GError       **error);
GDK_AVAILABLE_IN_ALL
GObject*     gtk_builder_get_object              (GtkBuilder    *builder,
                                                  const gchar   *name);
GDK_AVAILABLE_IN_ALL
GSList*      gtk_builder_get_objects             (GtkBuilder    *builder);
GDK_AVAILABLE_IN_3_8
void         gtk_builder_expose_object           (GtkBuilder    *builder,
                                                  const gchar   *name,
                                                  GObject       *object);
GDK_AVAILABLE_IN_ALL
void         gtk_builder_connect_signals         (GtkBuilder    *builder,
						  gpointer       user_data);
GDK_AVAILABLE_IN_ALL
void         gtk_builder_connect_signals_full    (GtkBuilder    *builder,
                                                  GtkBuilderConnectFunc func,
						  gpointer       user_data);
GDK_AVAILABLE_IN_ALL
void         gtk_builder_set_translation_domain  (GtkBuilder   	*builder,
                                                  const gchar  	*domain);
GDK_AVAILABLE_IN_ALL
const gchar* gtk_builder_get_translation_domain  (GtkBuilder   	*builder);
GDK_AVAILABLE_IN_ALL
GType        gtk_builder_get_type_from_name      (GtkBuilder   	*builder,
                                                  const char   	*type_name);

GDK_AVAILABLE_IN_ALL
gboolean     gtk_builder_value_from_string       (GtkBuilder    *builder,
						  GParamSpec   	*pspec,
                                                  const gchar  	*string,
                                                  GValue       	*value,
						  GError       **error);
GDK_AVAILABLE_IN_ALL
gboolean     gtk_builder_value_from_string_type  (GtkBuilder    *builder,
						  GType        	 type,
                                                  const gchar  	*string,
                                                  GValue       	*value,
						  GError       **error);
GDK_AVAILABLE_IN_3_10
GtkBuilder * gtk_builder_new_from_file           (const gchar   *filename);
GDK_AVAILABLE_IN_3_10
GtkBuilder * gtk_builder_new_from_resource       (const gchar   *resource_path);
GDK_AVAILABLE_IN_3_10
GtkBuilder * gtk_builder_new_from_string         (const gchar   *string,
                                                  gssize         length);

GDK_AVAILABLE_IN_3_10
void         gtk_builder_add_callback_symbol     (GtkBuilder    *builder,
						  const gchar   *callback_name,
						  GCallback      callback_symbol);
GDK_AVAILABLE_IN_3_10
void         gtk_builder_add_callback_symbols    (GtkBuilder    *builder,
						  const gchar   *first_callback_name,
						  GCallback      first_callback_symbol,
						  ...) G_GNUC_NULL_TERMINATED;
GDK_AVAILABLE_IN_3_10
GCallback    gtk_builder_lookup_callback_symbol  (GtkBuilder    *builder,
						  const gchar   *callback_name);

GDK_AVAILABLE_IN_3_12
void         gtk_builder_set_application         (GtkBuilder     *builder,
                                                  GtkApplication *application);

GDK_AVAILABLE_IN_3_12
GtkApplication * gtk_builder_get_application     (GtkBuilder     *builder);


/**
 * GTK_BUILDER_WARN_INVALID_CHILD_TYPE:
 * @object: the #GtkBuildable on which the warning ocurred
 * @type: the unexpected type value
 *
 * This macro should be used to emit a warning about and unexpected @type value
 * in a #GtkBuildable add_child implementation.
 */
#define GTK_BUILDER_WARN_INVALID_CHILD_TYPE(object, type) \
  g_warning ("'%s' is not a valid child type of '%s'", type, g_type_name (G_OBJECT_TYPE (object)))

GDK_AVAILABLE_IN_3_18
guint     gtk_builder_extend_with_template  (GtkBuilder    *builder,
                                             GtkWidget     *widget,
                                             GType          template_type,                                                          const gchar   *buffer,
                                             gsize          length,
                                             GError       **error);

G_END_DECLS

#endif /* __GTK_BUILDER_H__ */
