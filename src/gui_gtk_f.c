/* vi:set ts=8 sts=4 sw=4 noet:
 *
 * VIM - Vi IMproved		by Bram Moolenaar
 *
 * Do ":help uganda"  in Vim to read copying and usage conditions.
 * Do ":help credits" in Vim to see a list of people who contributed.
 * See README.txt for an overview of the Vim source code.
 */

/*
 * (C) 1998,1999 by Marcin Dalecki <martin@dalecki.de>
 *
 * Support for GTK+ 2 was added by:
 * (C) 2002,2003  Jason Hildebrand  <jason@peaceworks.ca>
 *		  Daniel Elstner  <daniel.elstner@gmx.net>
 *
 * This is a special purpose container widget, which manages arbitrary
 * children at arbitrary positions width arbitrary sizes.  This finally puts
 * an end on our resize problems with which we where struggling for such a
 * long time.
 *
 * Support for GTK+ 3 was added by:
 * 2016  Kazunobu Kuriyama  <kazunobu.kuriyama@gmail.com>
 */

#include "vim.h"
#include <gtk/gtk.h>	// without this it compiles, but gives errors at
			// runtime!
#include "gui_gtk_f.h"
#if !GTK_CHECK_VERSION(3,0,0)
# include <gtk/gtksignal.h>
#endif
#ifdef MSWIN
# include <gdk/gdkwin32.h>
#else
# include <gdk/gdkx.h>
#endif

typedef struct _GtkFormChild GtkFormChild;

struct _GtkFormChild
{
    GtkWidget *widget;
    GdkWindow *window;
    gint x;		// relative subwidget x position
    gint y;		// relative subwidget y position
    gint mapped;
};


static void gui_gtk_form_class_init(GtkFormClass *klass);
#if GTK_CHECK_VERSION(3,0,0)
static void gui_gtk_form_init(GtkForm *form);
#else
static void gui_gtk_form_init(GtkForm *form, void *g_class);
#endif

static void form_realize(GtkWidget *widget);
static void form_unrealize(GtkWidget *widget);
static void form_map(GtkWidget *widget);
static void form_size_request(GtkWidget *widget, GtkRequisition *requisition);
#if GTK_CHECK_VERSION(3,0,0)
static void form_get_preferred_width(GtkWidget *widget, gint *minimal_width, gint *natural_width);
static void form_get_preferred_height(GtkWidget *widget, gint *minimal_height, gint *natural_height);
#endif
static void form_size_allocate(GtkWidget *widget, GtkAllocation *allocation);
#if GTK_CHECK_VERSION(3,0,0)
static gboolean form_draw(GtkWidget *widget, cairo_t *cr);
#else
static gint form_expose(GtkWidget *widget, GdkEventExpose *event);
#endif

static void form_remove(GtkContainer *container, GtkWidget *widget);
static void form_forall(GtkContainer *container, gboolean include_internals, GtkCallback callback, gpointer callback_data);

static void form_attach_child_window(GtkForm *form, GtkFormChild *child);
static void form_realize_child(GtkForm *form, GtkFormChild *child);
static void form_position_child(GtkForm *form, GtkFormChild *child, gboolean force_allocate);
static void form_position_children(GtkForm *form);

static void form_send_configure(GtkForm *form);

static void form_child_map(GtkWidget *widget, gpointer user_data);
static void form_child_unmap(GtkWidget *widget, gpointer user_data);

#if !GTK_CHECK_VERSION(3,0,0)
static GtkWidgetClass *parent_class = NULL;
#endif

// Public interface

    GtkWidget *
gui_gtk_form_new(void)
{
    GtkForm *form;

#if GTK_CHECK_VERSION(3,0,0)
    form = g_object_new(GTK_TYPE_FORM, NULL);
#else
    form = gtk_type_new(gui_gtk_form_get_type());
#endif

    return GTK_WIDGET(form);
}

    void
gui_gtk_form_put(
	GtkForm	*form,
	GtkWidget	*child_widget,
	gint	x,
	gint	y)
{
    GtkFormChild *child;

    g_return_if_fail(GTK_IS_FORM(form));

    // LINTED: avoid warning: conversion to 'unsigned long'
    child = g_new(GtkFormChild, 1);
    if (child == NULL)
	return;

    child->widget = child_widget;
    child->window = NULL;
    child->x = x;
    child->y = y;
#if GTK_CHECK_VERSION(3,0,0)
    gtk_widget_set_size_request(child->widget, -1, -1);
#else
    child->widget->requisition.width = 0;
    child->widget->requisition.height = 0;
#endif
    child->mapped = FALSE;

    form->children = g_list_append(form->children, child);

    // child->window must be created and attached to the widget _before_
    // it has been realized, or else things will break with GTK2.  Note
    // that gtk_widget_set_parent() realizes the widget if it's visible
    // and its parent is mapped.
    if (gtk_widget_get_realized(GTK_WIDGET(form)))
	form_attach_child_window(form, child);

    gtk_widget_set_parent(child_widget, GTK_WIDGET(form));

    if (gtk_widget_get_realized(GTK_WIDGET(form))
	    && !gtk_widget_get_realized(child_widget))
	form_realize_child(form, child);

    form_position_child(form, child, TRUE);
}

    void
gui_gtk_form_move(
	GtkForm	*form,
	GtkWidget	*child_widget,
	gint	x,
	gint	y)
{
    GList *tmp_list;
    GtkFormChild *child;

    g_return_if_fail(GTK_IS_FORM(form));

    for (tmp_list = form->children; tmp_list; tmp_list = tmp_list->next)
    {
	child = tmp_list->data;
	if (child->widget == child_widget)
	{
	    child->x = x;
	    child->y = y;

	    form_position_child(form, child, TRUE);
	    return;
	}
    }
}

    void
gui_gtk_form_freeze(GtkForm *form)
{
    g_return_if_fail(GTK_IS_FORM(form));

    ++form->freeze_count;
}

    void
gui_gtk_form_thaw(GtkForm *form)
{
    g_return_if_fail(GTK_IS_FORM(form));

    if (!form->freeze_count)
	return;

    if (!(--form->freeze_count))
    {
	form_position_children(form);
	gtk_widget_queue_draw(GTK_WIDGET(form));
    }
}

// Basic Object handling procedures

#if GTK_CHECK_VERSION(3,0,0)
G_DEFINE_TYPE(GtkForm, gui_gtk_form, GTK_TYPE_CONTAINER)
#else
    GtkType
gui_gtk_form_get_type(void)
{
    static GtkType form_type = 0;

    if (!form_type)
    {
	GtkTypeInfo form_info;

	CLEAR_FIELD(form_info);
	form_info.type_name = "GtkForm";
	form_info.object_size = sizeof(GtkForm);
	form_info.class_size = sizeof(GtkFormClass);
	form_info.class_init_func = (GtkClassInitFunc)gui_gtk_form_class_init;
	form_info.object_init_func = (GtkObjectInitFunc)gui_gtk_form_init;

	form_type = gtk_type_unique(GTK_TYPE_CONTAINER, &form_info);
    }
    return form_type;
}
#endif // !GTK_CHECK_VERSION(3,0,0)

    static void
gui_gtk_form_class_init(GtkFormClass *klass)
{
    GtkWidgetClass *widget_class;
    GtkContainerClass *container_class;

    widget_class = (GtkWidgetClass *) klass;
    container_class = (GtkContainerClass *) klass;

#if !GTK_CHECK_VERSION(3,0,0)
    parent_class = gtk_type_class(gtk_container_get_type());
#endif

    widget_class->realize = form_realize;
    widget_class->unrealize = form_unrealize;
    widget_class->map = form_map;
#if GTK_CHECK_VERSION(3,0,0)
    widget_class->get_preferred_width = form_get_preferred_width;
    widget_class->get_preferred_height = form_get_preferred_height;
#else
    widget_class->size_request = form_size_request;
#endif
    widget_class->size_allocate = form_size_allocate;
#if GTK_CHECK_VERSION(3,0,0)
    widget_class->draw = form_draw;
#else
    widget_class->expose_event = form_expose;
#endif

    container_class->remove = form_remove;
    container_class->forall = form_forall;
}

    static void
gui_gtk_form_init(GtkForm *form
#if !GTK_CHECK_VERSION(3,0,0)
	, void *g_class UNUSED
#endif
	)
{
#if GTK_CHECK_VERSION(3,0,0)
    gtk_widget_set_has_window(GTK_WIDGET(form), TRUE);
#endif
    form->children = NULL;
    form->bin_window = NULL;
    form->freeze_count = 0;
}

/*
 * Widget methods
 */

    static void
form_realize(GtkWidget *widget)
{
    GList *tmp_list;
    GtkForm *form;
    GdkWindowAttr attributes;
    gint attributes_mask;
    GtkAllocation allocation;

    g_return_if_fail(GTK_IS_FORM(widget));

    form = GTK_FORM(widget);
    gtk_widget_set_realized(widget, TRUE);

    gtk_widget_get_allocation(widget, &allocation);
    attributes.window_type = GDK_WINDOW_CHILD;
    attributes.x = allocation.x;
    attributes.y = allocation.y;
    attributes.width = allocation.width;
    attributes.height = allocation.height;
    attributes.wclass = GDK_INPUT_OUTPUT;
    attributes.visual = gtk_widget_get_visual(widget);
#if GTK_CHECK_VERSION(3,0,0)
    attributes.event_mask = GDK_EXPOSURE_MASK;
#else
    attributes.colormap = gtk_widget_get_colormap(widget);
    attributes.event_mask = GDK_VISIBILITY_NOTIFY_MASK;
#endif

#if GTK_CHECK_VERSION(3,0,0)
    attributes_mask = GDK_WA_X | GDK_WA_Y | GDK_WA_VISUAL;
#else
    attributes_mask = GDK_WA_X | GDK_WA_Y | GDK_WA_VISUAL | GDK_WA_COLORMAP;
#endif

    gtk_widget_set_window(widget,
			  gdk_window_new(gtk_widget_get_parent_window(widget),
					 &attributes, attributes_mask));
    gdk_window_set_user_data(gtk_widget_get_window(widget), widget);

    attributes.x = 0;
    attributes.y = 0;
    attributes.event_mask = gtk_widget_get_events(widget);

    form->bin_window = gdk_window_new(gtk_widget_get_window(widget),
				      &attributes, attributes_mask);
    gdk_window_set_user_data(form->bin_window, widget);

#if GTK_CHECK_VERSION(3,0,0)
    {
	GtkStyleContext * const sctx = gtk_widget_get_style_context(widget);

	gtk_style_context_add_class(sctx, "gtk-form");
	gtk_style_context_set_state(sctx, GTK_STATE_FLAG_NORMAL);
# if !GTK_CHECK_VERSION(3,18,0)
	gtk_style_context_set_background(sctx, gtk_widget_get_window(widget));
	gtk_style_context_set_background(sctx, form->bin_window);
# endif
    }
#else
    widget->style = gtk_style_attach(widget->style, widget->window);
    gtk_style_set_background(widget->style, widget->window, GTK_STATE_NORMAL);
    gtk_style_set_background(widget->style, form->bin_window, GTK_STATE_NORMAL);
#endif

    for (tmp_list = form->children; tmp_list; tmp_list = tmp_list->next)
    {
	GtkFormChild *child = tmp_list->data;

	form_attach_child_window(form, child);

	if (gtk_widget_get_visible(child->widget))
	    form_realize_child(form, child);
    }
}


// After reading the documentation at
// http://developer.gnome.org/doc/API/2.0/gtk/gtk-changes-2-0.html
// I think it should be possible to remove this function when compiling
// against gtk-2.0.  It doesn't seem to cause problems, though.
//
// Well, I reckon at least the gdk_window_show(form->bin_window)
// is necessary.  GtkForm is anything but a usual container widget.
    static void
form_map(GtkWidget *widget)
{
    GList *tmp_list;
    GtkForm *form;

    g_return_if_fail(GTK_IS_FORM(widget));

    form = GTK_FORM(widget);

    gtk_widget_set_mapped(widget, TRUE);

    gdk_window_show(gtk_widget_get_window(widget));
    gdk_window_show(form->bin_window);

    for (tmp_list = form->children; tmp_list; tmp_list = tmp_list->next)
    {
	GtkFormChild *child = tmp_list->data;

	if (gtk_widget_get_visible(child->widget)
		&& !gtk_widget_get_mapped(child->widget))
	    gtk_widget_map(child->widget);
    }
}

    static void
form_unrealize(GtkWidget *widget)
{
    GList *tmp_list;
    GtkForm *form;

    g_return_if_fail(GTK_IS_FORM(widget));

    form = GTK_FORM(widget);

    tmp_list = form->children;

    gdk_window_set_user_data(form->bin_window, NULL);
    gdk_window_destroy(form->bin_window);
    form->bin_window = NULL;

    while (tmp_list)
    {
	GtkFormChild *child = tmp_list->data;

	if (child->window != NULL)
	{
	    g_signal_handlers_disconnect_by_func(G_OBJECT(child->widget),
		    FUNC2GENERIC(form_child_map),
		    child);
	    g_signal_handlers_disconnect_by_func(G_OBJECT(child->widget),
		    FUNC2GENERIC(form_child_unmap),
		    child);

	    gdk_window_set_user_data(child->window, NULL);
	    gdk_window_destroy(child->window);

	    child->window = NULL;
	}

	tmp_list = tmp_list->next;
    }

#if GTK_CHECK_VERSION(3,0,0)
    if (GTK_WIDGET_CLASS (gui_gtk_form_parent_class)->unrealize)
	 (* GTK_WIDGET_CLASS (gui_gtk_form_parent_class)->unrealize) (widget);
#else
    if (GTK_WIDGET_CLASS (parent_class)->unrealize)
	 (* GTK_WIDGET_CLASS (parent_class)->unrealize) (widget);
#endif
}

    static void
form_size_request(GtkWidget *widget, GtkRequisition *requisition)
{
    g_return_if_fail(GTK_IS_FORM(widget));
    g_return_if_fail(requisition != NULL);

    requisition->width = 1;
    requisition->height = 1;
}

#if GTK_CHECK_VERSION(3,0,0)
    static void
form_get_preferred_width(GtkWidget *widget,
			     gint      *minimal_width,
			     gint      *natural_width)
{
    GtkRequisition requisition;

    form_size_request(widget, &requisition);

    *minimal_width = requisition.width;
    *natural_width = requisition.width;
}

    static void
form_get_preferred_height(GtkWidget *widget,
			      gint	*minimal_height,
			      gint	*natural_height)
{
    GtkRequisition requisition;

    form_size_request(widget, &requisition);

    *minimal_height = requisition.height;
    *natural_height = requisition.height;
}
#endif // GTK_CHECK_VERSION(3,0,0)

    static void
form_size_allocate(GtkWidget *widget, GtkAllocation *allocation)
{
    GList *tmp_list;
    GtkForm *form;
    gboolean need_reposition;
    GtkAllocation cur_alloc;

    g_return_if_fail(GTK_IS_FORM(widget));

    gtk_widget_get_allocation(widget, &cur_alloc);

    if (cur_alloc.x == allocation->x
	    && cur_alloc.y == allocation->y
	    && cur_alloc.width == allocation->width
	    && cur_alloc.height == allocation->height)
	return;

    need_reposition = cur_alloc.width != allocation->width
		   || cur_alloc.height != allocation->height;
    form = GTK_FORM(widget);

    if (need_reposition)
    {
	tmp_list = form->children;

	while (tmp_list)
	{
	    GtkFormChild *child = tmp_list->data;
	    form_position_child(form, child, TRUE);

	    tmp_list = tmp_list->next;
	}
    }

    if (gtk_widget_get_realized(widget))
    {
	gdk_window_move_resize(gtk_widget_get_window(widget),
			       allocation->x, allocation->y,
			       allocation->width, allocation->height);
	gdk_window_move_resize(GTK_FORM(widget)->bin_window,
			       0, 0,
			       allocation->width, allocation->height);
    }
    gtk_widget_set_allocation(widget, allocation);
    if (need_reposition)
	form_send_configure(form);
}

#if GTK_CHECK_VERSION(3,0,0)
    static void
gtk_form_render_background(GtkWidget *widget, cairo_t *cr)
{
    gtk_render_background(gtk_widget_get_style_context(widget), cr,
			  0, 0,
			  gtk_widget_get_allocated_width(widget),
			  gtk_widget_get_allocated_height(widget));
}

    static gboolean
form_draw(GtkWidget *widget, cairo_t *cr)
{
    GList   *tmp_list = NULL;
    GtkForm *form     = NULL;

    g_return_val_if_fail(GTK_IS_FORM(widget), FALSE);

    gtk_form_render_background(widget, cr);

    form = GTK_FORM(widget);
    for (tmp_list = form->children; tmp_list; tmp_list = tmp_list->next)
    {
	GtkFormChild * const formchild = tmp_list->data;

	if (!gtk_widget_get_has_window(formchild->widget) &&
		gtk_cairo_should_draw_window(cr, formchild->window))
	{
	    // To get gtk_widget_draw() to work, it is required to call
	    // gtk_widget_size_allocate() in advance with a well-posed
	    // allocation for a given child widget in order to set a
	    // certain private GtkWidget variable, called
	    // widget->priv->alloc_need, to the proper value; otherwise,
	    // gtk_widget_draw() fails and the relevant scrollbar won't
	    // appear on the screen.
	    //
	    // Calling form_position_child() like this is one of ways
	    // to make sure of that.
	    form_position_child(form, formchild, TRUE);

	    gtk_form_render_background(formchild->widget, cr);
	}
    }

    return GTK_WIDGET_CLASS(gui_gtk_form_parent_class)->draw(widget, cr);
}
#else // !GTK_CHECK_VERSION(3,0,0)
    static gint
form_expose(GtkWidget *widget, GdkEventExpose *event)
{
    GList   *tmp_list;
    GtkForm *form;

    g_return_val_if_fail(GTK_IS_FORM(widget), FALSE);

    form = GTK_FORM(widget);

    if (event->window == form->bin_window)
	return FALSE;

    for (tmp_list = form->children; tmp_list; tmp_list = tmp_list->next)
	gtk_container_propagate_expose(GTK_CONTAINER(widget),
		GTK_WIDGET(((GtkFormChild *)tmp_list->data)->widget),
		event);

    return FALSE;
}
#endif // !GTK_CHECK_VERSION(3,0,0)

// Container method
    static void
form_remove(GtkContainer *container, GtkWidget *widget)
{
    GList *tmp_list;
    GtkForm *form;
    GtkFormChild *child = NULL;	    // init for gcc

    g_return_if_fail(GTK_IS_FORM(container));

    form = GTK_FORM(container);

    tmp_list = form->children;
    while (tmp_list)
    {
	child = tmp_list->data;
	if (child->widget == widget)
	    break;
	tmp_list = tmp_list->next;
    }

    if (tmp_list == NULL)
	return;

#if GTK_CHECK_VERSION(3,0,0)
    const gboolean was_visible = gtk_widget_get_visible(widget);
#endif
    if (child->window)
    {
	g_signal_handlers_disconnect_by_func(G_OBJECT(child->widget),
		FUNC2GENERIC(&form_child_map), child);
	g_signal_handlers_disconnect_by_func(G_OBJECT(child->widget),
		FUNC2GENERIC(&form_child_unmap), child);

	// FIXME: This will cause problems for reparenting NO_WINDOW
	// widgets out of a GtkForm
	gdk_window_set_user_data(child->window, NULL);
	gdk_window_destroy(child->window);
    }
    gtk_widget_unparent(widget);
#if GTK_CHECK_VERSION(3,0,0)
    if (was_visible)
	gtk_widget_queue_resize(GTK_WIDGET(container));
#endif
    form->children = g_list_remove_link(form->children, tmp_list);
    g_list_free_1(tmp_list);
    g_free(child);
}

    static void
form_forall(GtkContainer	*container,
		gboolean	include_internals UNUSED,
		GtkCallback	callback,
		gpointer	callback_data)
{
    GtkForm *form;
    GtkFormChild *child;
    GList *tmp_list;

    g_return_if_fail(GTK_IS_FORM(container));
    g_return_if_fail(callback != NULL);

    form = GTK_FORM(container);

    tmp_list = form->children;
    while (tmp_list)
    {
	child = tmp_list->data;
	tmp_list = tmp_list->next;

	(*callback) (child->widget, callback_data);
    }
}

// Operations on children

    static void
form_attach_child_window(GtkForm *form, GtkFormChild *child)
{
    if (child->window != NULL)
	return; // been there, done that

    if (!gtk_widget_get_has_window(child->widget))
    {
	GtkWidget	*widget;
	GdkWindowAttr	attributes;
	gint		attributes_mask;
	GtkRequisition	requisition;

	widget = GTK_WIDGET(form);

#if GTK_CHECK_VERSION(3,0,0)
	gtk_widget_get_preferred_size(child->widget, &requisition, NULL);
#else
	requisition = child->widget->requisition;
#endif
	attributes.window_type = GDK_WINDOW_CHILD;
	attributes.x = child->x;
	attributes.y = child->y;
	attributes.width = requisition.width;
	attributes.height = requisition.height;
	attributes.wclass = GDK_INPUT_OUTPUT;
	attributes.visual = gtk_widget_get_visual(widget);
#if !GTK_CHECK_VERSION(3,0,0)
	attributes.colormap = gtk_widget_get_colormap(widget);
#endif
	attributes.event_mask = GDK_EXPOSURE_MASK;

#if GTK_CHECK_VERSION(3,0,0)
	attributes_mask = GDK_WA_X | GDK_WA_Y | GDK_WA_VISUAL;
#else
	attributes_mask = GDK_WA_X | GDK_WA_Y | GDK_WA_VISUAL | GDK_WA_COLORMAP;
#endif
	child->window = gdk_window_new(form->bin_window,
				       &attributes, attributes_mask);
	gdk_window_set_user_data(child->window, widget);

#if GTK_CHECK_VERSION(3,0,0)
	{
	    GtkStyleContext * const sctx = gtk_widget_get_style_context(widget);

	    gtk_style_context_set_state(sctx, GTK_STATE_FLAG_NORMAL);
# if !GTK_CHECK_VERSION(3,18,0)
	    gtk_style_context_set_background(sctx, child->window);
# endif
	}
#else
	gtk_style_set_background(widget->style,
				 child->window,
				 GTK_STATE_NORMAL);
#endif

	gtk_widget_set_parent_window(child->widget, child->window);
	/*
	 * Install signal handlers to map/unmap child->window
	 * alongside with the actual widget.
	 */
	g_signal_connect(G_OBJECT(child->widget), "map",
			 G_CALLBACK(&form_child_map), child);
	g_signal_connect(G_OBJECT(child->widget), "unmap",
			 G_CALLBACK(&form_child_unmap), child);
    }
    else if (!gtk_widget_get_realized(child->widget))
    {
	gtk_widget_set_parent_window(child->widget, form->bin_window);
    }
}

    static void
form_realize_child(GtkForm *form, GtkFormChild *child)
{
    form_attach_child_window(form, child);
    gtk_widget_realize(child->widget);
}

    static void
form_position_child(GtkForm *form, GtkFormChild *child, gboolean force_allocate)
{
    gint x;
    gint y;

    x = child->x;
    y = child->y;

    if ((x >= G_MINSHORT) && (x <= G_MAXSHORT) &&
	(y >= G_MINSHORT) && (y <= G_MAXSHORT))
    {
	if (!child->mapped)
	{
	    if (gtk_widget_get_mapped(GTK_WIDGET(form))
		    && gtk_widget_get_visible(child->widget))
	    {
		if (!gtk_widget_get_mapped(child->widget))
		    gtk_widget_map(child->widget);

		child->mapped = TRUE;
		force_allocate = TRUE;
	    }
	}

	if (force_allocate)
	{
	    GtkAllocation allocation;
	    GtkRequisition requisition;

#if GTK_CHECK_VERSION(3,0,0)
	    gtk_widget_get_preferred_size(child->widget, &requisition, NULL);
#else
	    requisition = child->widget->requisition;
#endif

	    if (!gtk_widget_get_has_window(child->widget))
	    {
		if (child->window)
		{
		    gdk_window_move_resize(child->window,
			    x, y,
			    requisition.width,
			    requisition.height);
		}

		allocation.x = 0;
		allocation.y = 0;
	    }
	    else
	    {
		allocation.x = x;
		allocation.y = y;
	    }

	    allocation.width = requisition.width;
	    allocation.height = requisition.height;

	    gtk_widget_size_allocate(child->widget, &allocation);
	}
    }
    else
    {
	if (child->mapped)
	{
	    child->mapped = FALSE;

	    if (gtk_widget_get_mapped(child->widget))
		gtk_widget_unmap(child->widget);
	}
    }
}

    static void
form_position_children(GtkForm *form)
{
    GList *tmp_list;

    for (tmp_list = form->children; tmp_list; tmp_list = tmp_list->next)
	form_position_child(form, tmp_list->data, FALSE);
}

    void
gui_gtk_form_move_resize(GtkForm *form, GtkWidget *widget,
		     gint x, gint y, gint w, gint h)
{
#if GTK_CHECK_VERSION(3,0,0)
    gtk_widget_set_size_request(widget, w, h);
#else
    widget->requisition.width  = w;
    widget->requisition.height = h;
#endif

    gui_gtk_form_move(form, widget, x, y);
}

    static void
form_send_configure(GtkForm *form)
{
    GtkWidget *widget;
    GdkEventConfigure event;
    GtkAllocation allocation;

    widget = GTK_WIDGET(form);

    gtk_widget_get_allocation(widget, &allocation);
    event.type = GDK_CONFIGURE;
    event.window = gtk_widget_get_window(widget);
    event.x = allocation.x;
    event.y = allocation.y;
    event.width = allocation.width;
    event.height = allocation.height;

    gtk_main_do_event((GdkEvent*)&event);
}

    static void
form_child_map(GtkWidget *widget UNUSED, gpointer user_data)
{
    GtkFormChild *child;

    child = (GtkFormChild *)user_data;

    child->mapped = TRUE;
    gdk_window_show(child->window);
}

    static void
form_child_unmap(GtkWidget *widget UNUSED, gpointer user_data)
{
    GtkFormChild *child;

    child = (GtkFormChild *)user_data;

    child->mapped = FALSE;
    gdk_window_hide(child->window);
}
