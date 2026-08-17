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

#ifndef __GTK_BUILDABLE_H__
#define __GTK_BUILDABLE_H__

#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtkbuilder.h>

G_BEGIN_DECLS

#define GTK_TYPE_BUILDABLE            (gtk_buildable_get_type ())
#define GTK_BUILDABLE(obj)            (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_BUILDABLE, GtkBuildable))
#define GTK_IS_BUILDABLE(obj)         (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_BUILDABLE))
#define GTK_BUILDABLE_GET_IFACE(obj)  (G_TYPE_INSTANCE_GET_INTERFACE ((obj), GTK_TYPE_BUILDABLE, GtkBuildableIface))

typedef struct _GtkBuildable      GtkBuildable; /* Dummy typedef */
typedef struct _GtkBuildableIface GtkBuildableIface;

typedef struct _GtkBuildableParseContext      GtkBuildableParseContext;
typedef struct _GtkBuildableParser GtkBuildableParser;

/**
 * GtkBuildableParseContext:
 *
 * An opaque context struct for `GtkBuildableParser`.
 */

/**
 * GtkBuildableParser:
 * @start_element: function called for open elements
 * @end_element: function called for close elements
 * @text: function called for character data
 * @error: function called on error
 *
 * A sub-parser for `GtkBuildable` implementations.
 */
struct _GtkBuildableParser
{
  /* Called for open tags <foo bar="baz"> */
  void (*start_element)  (GtkBuildableParseContext *context,
                          const char               *element_name,
                          const char              **attribute_names,
                          const char              **attribute_values,
                          gpointer                  user_data,
                          GError                  **error);

  /* Called for close tags </foo> */
  void (*end_element)    (GtkBuildableParseContext *context,
                          const char               *element_name,
                          gpointer                  user_data,
                          GError                  **error);

  /* Called for character data */
  /* text is not nul-terminated */
  void (*text)           (GtkBuildableParseContext *context,
                          const char               *text,
                          gsize                     text_len,
                          gpointer                  user_data,
                          GError                  **error);

  /* Called on error, including one set by other
   * methods in the vtable. The GError should not be freed.
   */
  void (*error)          (GtkBuildableParseContext *context,
                          GError                   *error,
                          gpointer                 user_data);

  /*< private >*/
  gpointer padding[4];
};

/**
 * GtkBuildableIface:
 * @g_iface: the parent class
 * @set_id: Stores the id attribute given in the `GtkBuilder` UI definition.
 *   `GtkWidget` stores the name as object data. Implement this method if your
 *   object has some notion of “ID” and it makes sense to map the XML id
 *   attribute to it.
 * @get_id: The getter corresponding to @set_id. Implement this
 *   if you implement @set_id.
 * @add_child: Adds a child. The @type parameter can be used to
 *   differentiate the kind of child. `GtkWidget` implements this
 *   to add event controllers to the widget, `GtkNotebook` uses
 *   the @type to distinguish between page labels (of type "page-label")
 *   and normal children.
 * @set_buildable_property: Sets a property of a buildable object.
 *  It is normally not necessary to implement this, g_object_set_property()
 *  is used by default. `GtkWindow` implements this to delay showing itself
 *  (i.e. setting the [property@GtkWidget:visible] property) until the whole
 *  interface is created.
 * @construct_child: Constructs a child of a buildable that has been
 *  specified as “constructor” in the UI definition. This can be used to
 *  reference a widget created in a <ui> tag which is outside
 *  of the normal GtkBuilder UI definition hierarchy.  A reference to the
 *  constructed object is returned and becomes owned by the caller.
 * @custom_tag_start: Implement this if the buildable needs to parse
 *  content below <child>. To handle an element, the implementation
 *  must fill in the @parser and @user_data and return %TRUE.
 *  `GtkWidget` implements this to parse accessible attributes specified
 *  in <accessibility> elements.
 *  Note that @user_data must be freed in @custom_tag_end or @custom_finished.
 * @custom_tag_end: Called for the end tag of each custom element that is
 *  handled by the buildable (see @custom_tag_start).
 * @custom_finished: Called for each custom tag handled by the buildable
 *  when the builder finishes parsing (see @custom_tag_start)
 * @parser_finished: Called when a builder finishes the parsing
 *  of a UI definition. It is normally not necessary to implement this,
 *  unless you need to perform special cleanup actions. `GtkWindow` sets
 *  the `GtkWidget:visible` property here.
 * @get_internal_child: Returns an internal child of a buildable.
 *  `GtkDialog` implements this to give access to its @vbox, making
 *  it possible to add children to the vbox in a UI definition.
 *  Implement this if the buildable has internal children that may
 *  need to be accessed from a UI definition.
 *
 * The `GtkBuildableIface` interface contains methods that are
 * necessary to allow `GtkBuilder` to construct an object from
 * a `GtkBuilder` UI definition.
 */
struct _GtkBuildableIface
{
  GTypeInterface g_iface;

  /* virtual table */
  void          (* set_id)                 (GtkBuildable       *buildable,
                                            const char         *id);
  const char *  (* get_id)                 (GtkBuildable       *buildable);

  /**
   * GtkBuildableIface::add_child:
   * @buildable: a `GtkBuildable`
   * @builder: a `GtkBuilder`
   * @child: child to add
   * @type: (nullable): kind of child or %NULL
   *
   * Adds a child to @buildable. @type is an optional string
   * describing how the child should be added.
   */
  void          (* add_child)              (GtkBuildable       *buildable,
                                            GtkBuilder         *builder,
                                            GObject            *child,
                                            const char         *type);
  void          (* set_buildable_property) (GtkBuildable       *buildable,
                                            GtkBuilder         *builder,
                                            const char         *name,
                                            const GValue       *value);
  GObject *     (* construct_child)        (GtkBuildable       *buildable,
                                            GtkBuilder         *builder,
                                            const char         *name);

  /**
   * GtkBuildableIface::custom_tag_start:
   * @buildable: a `GtkBuildable`
   * @builder: a `GtkBuilder` used to construct this object
   * @child: (nullable): child object or %NULL for non-child tags
   * @tagname: name of tag
   * @parser: (out): a `GtkBuildableParser` to fill in
   * @data: (out): return location for user data that will be passed in
   *   to parser functions
   *
   * Called for each unknown element under `<child>`.
   *
   * Returns: %TRUE if an object has a custom implementation, %FALSE
   *   if it doesn't.
   */
  gboolean      (* custom_tag_start)       (GtkBuildable       *buildable,
                                            GtkBuilder         *builder,
                                            GObject            *child,
                                            const char         *tagname,
                                            GtkBuildableParser *parser,
                                            gpointer           *data);
  /**
   * GtkBuildableIface::custom_tag_end:
   * @buildable: A `GtkBuildable`
   * @builder: `GtkBuilder` used to construct this object
   * @child: (nullable): child object or %NULL for non-child tags
   * @tagname: name of tag
   * @data: user data that will be passed in to parser functions
   *
   * Called at the end of each custom element handled by
   * the buildable.
   */
  void          (* custom_tag_end)         (GtkBuildable       *buildable,
                                            GtkBuilder         *builder,
                                            GObject            *child,
                                            const char         *tagname,
                                            gpointer            data);
   /**
    * GtkBuildableIface::custom_finished:
    * @buildable: a `GtkBuildable`
    * @builder: a `GtkBuilder`
    * @child: (nullable): child object or %NULL for non-child tags
    * @tagname: the name of the tag
    * @data: user data created in custom_tag_start
    *
    * Similar to gtk_buildable_parser_finished() but is
    * called once for each custom tag handled by the @buildable.
    */
  void          (* custom_finished)        (GtkBuildable       *buildable,
                                            GtkBuilder         *builder,
                                            GObject            *child,
                                            const char         *tagname,
                                            gpointer            data);
  void          (* parser_finished)        (GtkBuildable       *buildable,
                                            GtkBuilder         *builder);

  /**
   * GtkBuildableIface::get_internal_child:
   * @buildable: a `GtkBuildable`
   * @builder: a `GtkBuilder`
   * @childname: name of child
   *
   * Retrieves the internal child called @childname of the @buildable object.
   *
   * Returns: (transfer none): the internal child of the buildable object
   */
  GObject *     (* get_internal_child)     (GtkBuildable       *buildable,
                                            GtkBuilder         *builder,
                                            const char         *childname);
};


GDK_AVAILABLE_IN_ALL
GType     gtk_buildable_get_type               (void) G_GNUC_CONST;

GDK_AVAILABLE_IN_ALL
const char * gtk_buildable_get_buildable_id    (GtkBuildable        *buildable);

GDK_AVAILABLE_IN_ALL
void          gtk_buildable_parse_context_push              (GtkBuildableParseContext *context,
                                                             const GtkBuildableParser *parser,
                                                             gpointer                  user_data);
GDK_AVAILABLE_IN_ALL
gpointer      gtk_buildable_parse_context_pop               (GtkBuildableParseContext *context);
GDK_AVAILABLE_IN_ALL
const char *  gtk_buildable_parse_context_get_element       (GtkBuildableParseContext *context);
GDK_AVAILABLE_IN_ALL
GPtrArray    *gtk_buildable_parse_context_get_element_stack (GtkBuildableParseContext *context);
GDK_AVAILABLE_IN_ALL
void          gtk_buildable_parse_context_get_position      (GtkBuildableParseContext *context,
                                                             int                      *line_number,
                                                             int                      *char_number);

G_DEFINE_AUTOPTR_CLEANUP_FUNC(GtkBuildable, g_object_unref)

G_END_DECLS

#endif /* __GTK_BUILDABLE_H__ */
