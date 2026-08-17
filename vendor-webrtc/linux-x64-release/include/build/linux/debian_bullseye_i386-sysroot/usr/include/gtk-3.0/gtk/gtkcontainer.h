/* GTK - The GIMP Toolkit
 * Copyright (C) 1995-1997 Peter Mattis, Spencer Kimball and Josh MacDonald
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

#ifndef __GTK_CONTAINER_H__
#define __GTK_CONTAINER_H__


#if !defined (__GTK_H_INSIDE__) && !defined (GTK_COMPILATION)
#error "Only <gtk/gtk.h> can be included directly."
#endif

#include <gtk/gtkwidget.h>

G_BEGIN_DECLS

#define GTK_TYPE_CONTAINER              (gtk_container_get_type ())
#define GTK_CONTAINER(obj)              (G_TYPE_CHECK_INSTANCE_CAST ((obj), GTK_TYPE_CONTAINER, GtkContainer))
#define GTK_CONTAINER_CLASS(klass)      (G_TYPE_CHECK_CLASS_CAST ((klass), GTK_TYPE_CONTAINER, GtkContainerClass))
#define GTK_IS_CONTAINER(obj)           (G_TYPE_CHECK_INSTANCE_TYPE ((obj), GTK_TYPE_CONTAINER))
#define GTK_IS_CONTAINER_CLASS(klass)   (G_TYPE_CHECK_CLASS_TYPE ((klass), GTK_TYPE_CONTAINER))
#define GTK_CONTAINER_GET_CLASS(obj)    (G_TYPE_INSTANCE_GET_CLASS ((obj), GTK_TYPE_CONTAINER, GtkContainerClass))


typedef struct _GtkContainer              GtkContainer;
typedef struct _GtkContainerPrivate       GtkContainerPrivate;
typedef struct _GtkContainerClass         GtkContainerClass;

struct _GtkContainer
{
  GtkWidget widget;

  /*< private >*/
  GtkContainerPrivate *priv;
};

/**
 * GtkContainerClass:
 * @parent_class: The parent class.
 * @add: Signal emitted when a widget is added to container.
 * @remove: Signal emitted when a widget is removed from container.
 * @check_resize: Signal emitted when a size recalculation is needed.
 * @forall: Invokes callback on each child of container. The callback handler
 *    may remove the child.
 * @set_focus_child: Sets the focused child of container.
 * @child_type: Returns the type of the children supported by the container.
 * @composite_name: Gets a widget’s composite name. Deprecated: 3.10.
 * @set_child_property: Set a property on a child of container.
 * @get_child_property: Get a property from a child of container.
 * @get_path_for_child: Get path representing entire widget hierarchy
 *    from the toplevel down to and including @child.
 *
 * Base class for containers.
 */
struct _GtkContainerClass
{
  GtkWidgetClass parent_class;

  /*< public >*/

  void    (*add)       		(GtkContainer	 *container,
				 GtkWidget	 *widget);
  void    (*remove)    		(GtkContainer	 *container,
				 GtkWidget	 *widget);
  void    (*check_resize)	(GtkContainer	 *container);
  void    (*forall)    		(GtkContainer	 *container,
				 gboolean	  include_internals,
				 GtkCallback	  callback,
				 gpointer	  callback_data);
  void    (*set_focus_child)	(GtkContainer	 *container,
				 GtkWidget	 *child);
  GType   (*child_type)		(GtkContainer	 *container);
  gchar*  (*composite_name)	(GtkContainer	 *container,
				 GtkWidget	 *child);
  void    (*set_child_property) (GtkContainer    *container,
				 GtkWidget       *child,
				 guint            property_id,
				 const GValue    *value,
				 GParamSpec      *pspec);
  void    (*get_child_property) (GtkContainer    *container,
                                 GtkWidget       *child,
				 guint            property_id,
				 GValue          *value,
				 GParamSpec      *pspec);
  GtkWidgetPath * (*get_path_for_child) (GtkContainer *container,
                                         GtkWidget    *child);


  /*< private >*/

  unsigned int _handle_border_width : 1;

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


/**
 * GtkResizeMode:
 * @GTK_RESIZE_PARENT: Pass resize request to the parent
 * @GTK_RESIZE_QUEUE: Queue resizes on this widget
 * @GTK_RESIZE_IMMEDIATE: Resize immediately. Deprecated.
 */
typedef enum
{
  GTK_RESIZE_PARENT,
  GTK_RESIZE_QUEUE,
  GTK_RESIZE_IMMEDIATE
} GtkResizeMode;


/* Application-level methods */

GDK_AVAILABLE_IN_ALL
GType   gtk_container_get_type		 (void) G_GNUC_CONST;
GDK_AVAILABLE_IN_ALL
void    gtk_container_set_border_width	 (GtkContainer	   *container,
					  guint		    border_width);
GDK_AVAILABLE_IN_ALL
guint   gtk_container_get_border_width   (GtkContainer     *container);
GDK_AVAILABLE_IN_ALL
void    gtk_container_add		 (GtkContainer	   *container,
					  GtkWidget	   *widget);
GDK_AVAILABLE_IN_ALL
void    gtk_container_remove		 (GtkContainer	   *container,
					  GtkWidget	   *widget);

GDK_DEPRECATED_IN_3_12
void    gtk_container_set_resize_mode    (GtkContainer     *container,
					  GtkResizeMode     resize_mode);
GDK_DEPRECATED_IN_3_12
GtkResizeMode gtk_container_get_resize_mode (GtkContainer     *container);

GDK_AVAILABLE_IN_ALL
void    gtk_container_check_resize       (GtkContainer     *container);

GDK_AVAILABLE_IN_ALL
void     gtk_container_foreach      (GtkContainer       *container,
				     GtkCallback         callback,
				     gpointer            callback_data);
GDK_AVAILABLE_IN_ALL
GList*   gtk_container_get_children     (GtkContainer       *container);

GDK_AVAILABLE_IN_ALL
void     gtk_container_propagate_draw   (GtkContainer   *container,
					 GtkWidget      *child,
					 cairo_t        *cr);

GDK_DEPRECATED_IN_3_24
void     gtk_container_set_focus_chain  (GtkContainer   *container,
                                         GList          *focusable_widgets);
GDK_DEPRECATED_IN_3_24
gboolean gtk_container_get_focus_chain  (GtkContainer   *container,
					 GList         **focusable_widgets);
GDK_DEPRECATED_IN_3_24
void     gtk_container_unset_focus_chain (GtkContainer  *container);

#define GTK_IS_RESIZE_CONTAINER(widget) (GTK_IS_CONTAINER (widget) && \
                                        (gtk_container_get_resize_mode (GTK_CONTAINER (widget)) != GTK_RESIZE_PARENT))

/* Widget-level methods */

GDK_DEPRECATED_IN_3_14
void   gtk_container_set_reallocate_redraws (GtkContainer    *container,
					     gboolean         needs_redraws);
GDK_AVAILABLE_IN_ALL
void   gtk_container_set_focus_child	   (GtkContainer     *container,
					    GtkWidget	     *child);
GDK_AVAILABLE_IN_ALL
GtkWidget *
       gtk_container_get_focus_child	   (GtkContainer     *container);
GDK_AVAILABLE_IN_ALL
void   gtk_container_set_focus_vadjustment (GtkContainer     *container,
					    GtkAdjustment    *adjustment);
GDK_AVAILABLE_IN_ALL
GtkAdjustment *gtk_container_get_focus_vadjustment (GtkContainer *container);
GDK_AVAILABLE_IN_ALL
void   gtk_container_set_focus_hadjustment (GtkContainer     *container,
					    GtkAdjustment    *adjustment);
GDK_AVAILABLE_IN_ALL
GtkAdjustment *gtk_container_get_focus_hadjustment (GtkContainer *container);

GDK_DEPRECATED_IN_3_10
void    gtk_container_resize_children      (GtkContainer     *container);

GDK_AVAILABLE_IN_ALL
GType   gtk_container_child_type	   (GtkContainer     *container);


GDK_AVAILABLE_IN_ALL
void         gtk_container_class_install_child_property (GtkContainerClass *cclass,
							 guint		    property_id,
							 GParamSpec	   *pspec);
GDK_AVAILABLE_IN_3_18
void         gtk_container_class_install_child_properties (GtkContainerClass *cclass,
                                                           guint              n_pspecs,
                                                           GParamSpec       **pspecs);
GDK_AVAILABLE_IN_ALL
GParamSpec*  gtk_container_class_find_child_property	(GObjectClass	   *cclass,
							 const gchar	   *property_name);
GDK_AVAILABLE_IN_ALL
GParamSpec** gtk_container_class_list_child_properties	(GObjectClass	   *cclass,
							 guint		   *n_properties);
GDK_AVAILABLE_IN_ALL
void         gtk_container_add_with_properties		(GtkContainer	   *container,
							 GtkWidget	   *widget,
							 const gchar	   *first_prop_name,
							 ...) G_GNUC_NULL_TERMINATED;
GDK_AVAILABLE_IN_ALL
void         gtk_container_child_set			(GtkContainer	   *container,
							 GtkWidget	   *child,
							 const gchar	   *first_prop_name,
							 ...) G_GNUC_NULL_TERMINATED;
GDK_AVAILABLE_IN_ALL
void         gtk_container_child_get			(GtkContainer	   *container,
							 GtkWidget	   *child,
							 const gchar	   *first_prop_name,
							 ...) G_GNUC_NULL_TERMINATED;
GDK_AVAILABLE_IN_ALL
void         gtk_container_child_set_valist		(GtkContainer	   *container,
							 GtkWidget	   *child,
							 const gchar	   *first_property_name,
							 va_list	    var_args);
GDK_AVAILABLE_IN_ALL
void         gtk_container_child_get_valist		(GtkContainer	   *container,
							 GtkWidget	   *child,
							 const gchar	   *first_property_name,
							 va_list	    var_args);
GDK_AVAILABLE_IN_ALL
void	     gtk_container_child_set_property		(GtkContainer	   *container,
							 GtkWidget	   *child,
							 const gchar	   *property_name,
							 const GValue	   *value);
GDK_AVAILABLE_IN_ALL
void	     gtk_container_child_get_property		(GtkContainer	   *container,
							 GtkWidget	   *child,
							 const gchar	   *property_name,
	                                                 GValue		   *value);

GDK_AVAILABLE_IN_3_2
void gtk_container_child_notify (GtkContainer *container,
                                 GtkWidget    *child,
                                 const gchar  *child_property);

GDK_AVAILABLE_IN_3_18
void gtk_container_child_notify_by_pspec (GtkContainer *container,
                                          GtkWidget    *child,
                                          GParamSpec   *pspec);

/**
 * GTK_CONTAINER_WARN_INVALID_CHILD_PROPERTY_ID:
 * @object: the #GObject on which set_child_property() or get_child_property()
 *  was called
 * @property_id: the numeric id of the property
 * @pspec: the #GParamSpec of the property
 *
 * This macro should be used to emit a standard warning about unexpected
 * properties in set_child_property() and get_child_property() implementations.
 */
#define GTK_CONTAINER_WARN_INVALID_CHILD_PROPERTY_ID(object, property_id, pspec) \
    G_OBJECT_WARN_INVALID_PSPEC ((object), "child property", (property_id), (pspec))


GDK_AVAILABLE_IN_ALL
void    gtk_container_forall		     (GtkContainer *container,
					      GtkCallback   callback,
					      gpointer	    callback_data);

GDK_AVAILABLE_IN_ALL
void    gtk_container_class_handle_border_width (GtkContainerClass *klass);

GDK_AVAILABLE_IN_ALL
GtkWidgetPath * gtk_container_get_path_for_child (GtkContainer      *container,
                                                  GtkWidget         *child);

G_DEFINE_AUTOPTR_CLEANUP_FUNC(GtkContainer, g_object_unref)

G_END_DECLS

#endif /* __GTK_CONTAINER_H__ */
