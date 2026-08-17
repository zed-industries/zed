/*
A library to communicate a menu object set accross DBus and
track updates and maintain consistency.

Copyright 2009 Canonical Ltd.

Authors:
    Ted Gould <ted@canonical.com>

This program is free software: you can redistribute it and/or modify it 
under the terms of either or both of the following licenses:

1) the GNU Lesser General Public License version 3, as published by the 
Free Software Foundation; and/or
2) the GNU Lesser General Public License version 2.1, as published by 
the Free Software Foundation.

This program is distributed in the hope that it will be useful, but 
WITHOUT ANY WARRANTY; without even the implied warranties of 
MERCHANTABILITY, SATISFACTORY QUALITY or FITNESS FOR A PARTICULAR 
PURPOSE.  See the applicable version of the GNU Lesser General Public 
License for more details.

You should have received a copy of both the GNU Lesser General Public 
License version 3 and version 2.1 along with this program.  If not, see 
<http://www.gnu.org/licenses/>
*/

#ifndef __DBUSMENU_MENUITEM_H__
#define __DBUSMENU_MENUITEM_H__

#include <glib.h>
#include <glib-object.h>

G_BEGIN_DECLS

#define DBUSMENU_TYPE_MENUITEM            (dbusmenu_menuitem_get_type ())
#define DBUSMENU_MENUITEM(obj)            (G_TYPE_CHECK_INSTANCE_CAST ((obj), DBUSMENU_TYPE_MENUITEM, DbusmenuMenuitem))
#define DBUSMENU_MENUITEM_CLASS(klass)    (G_TYPE_CHECK_CLASS_CAST ((klass), DBUSMENU_TYPE_MENUITEM, DbusmenuMenuitemClass))
#define DBUSMENU_IS_MENUITEM(obj)         (G_TYPE_CHECK_INSTANCE_TYPE ((obj), DBUSMENU_TYPE_MENUITEM))
#define DBUSMENU_IS_MENUITEM_CLASS(klass) (G_TYPE_CHECK_CLASS_TYPE ((klass), DBUSMENU_TYPE_MENUITEM))
#define DBUSMENU_MENUITEM_GET_CLASS(obj)  (G_TYPE_INSTANCE_GET_CLASS ((obj), DBUSMENU_TYPE_MENUITEM, DbusmenuMenuitemClass))

/* ***************************************** */
/* *********  GLib Object Signals  ********* */
/* ***************************************** */
/**
 * DBUSMENU_MENUITEM_SIGNAL_PROPERTY_CHANGED:
 *
 * String to attach to signal #DbusmenuServer::property-changed
 */
#define DBUSMENU_MENUITEM_SIGNAL_PROPERTY_CHANGED    "property-changed"
/**
 * DBUSMENU_MENUITEM_SIGNAL_ITEM_ACTIVATED:
 *
 * String to attach to signal #DbusmenuServer::item-activated
 */
#define DBUSMENU_MENUITEM_SIGNAL_ITEM_ACTIVATED      "item-activated"
/**
 * DBUSMENU_MENUITEM_SIGNAL_CHILD_ADDED:
 *
 * String to attach to signal #DbusmenuServer::child-added
 */
#define DBUSMENU_MENUITEM_SIGNAL_CHILD_ADDED         "child-added"
/**
 * DBUSMENU_MENUITEM_SIGNAL_CHILD_REMOVED:
 *
 * String to attach to signal #DbusmenuServer::child-removed
 */
#define DBUSMENU_MENUITEM_SIGNAL_CHILD_REMOVED       "child-removed"
/**
 * DBUSMENU_MENUITEM_SIGNAL_CHILD_MOVED:
 *
 * String to attach to signal #DbusmenuServer::child-moved
 */
#define DBUSMENU_MENUITEM_SIGNAL_CHILD_MOVED         "child-moved"
/**
 * DBUSMENU_MENUITEM_SIGNAL_REALIZED:
 *
 * String to attach to signal #DbusmenuServer::realized
 */
#define DBUSMENU_MENUITEM_SIGNAL_REALIZED            "realized"
/**
 * DBUSMENU_MENUITEM_SIGNAL_REALIZED_ID:
 *
 * ID to attach to signal #DbusmenuServer::realized
 */
#define DBUSMENU_MENUITEM_SIGNAL_REALIZED_ID         (g_signal_lookup(DBUSMENU_MENUITEM_SIGNAL_REALIZED, DBUSMENU_TYPE_MENUITEM))
/**
 * DBUSMENU_MENUITEM_SIGNAL_SHOW_TO_USER:
 *
 * String to attach to signal #DbusmenuServer::show-to-user
 */
#define DBUSMENU_MENUITEM_SIGNAL_SHOW_TO_USER        "show-to-user"
/**
 * DBUSMENU_MENUITEM_SIGNAL_ABOUT_TO_SHOW:
 *
 * String to attach to signal #DbusmenuServer::about-to-show
 */
#define DBUSMENU_MENUITEM_SIGNAL_ABOUT_TO_SHOW       "about-to-show"
/**
 * DBUSMENU_MENUITEM_SIGNAL_EVENT:
 *
 * String to attach to signal #DbusmenuServer::event
 */
#define DBUSMENU_MENUITEM_SIGNAL_EVENT               "event"

/* ***************************************** */
/* *********  Menuitem Properties  ********* */
/* ***************************************** */
/**
 * DBUSMENU_MENUITEM_PROP_TYPE:
 *
 * #DbusmenuMenuitem property used to represent what type of menuitem
 * this object represents.  Type: #G_VARIANT_TYPE_STRING.
 */
#define DBUSMENU_MENUITEM_PROP_TYPE                  "type"
/**
 * DBUSMENU_MENUITEM_PROP_VISIBLE:
 *
 * #DbusmenuMenuitem property used to represent whether the menuitem
 * should be shown or not.  Type: #G_VARIANT_TYPE_BOOLEAN.
 */
#define DBUSMENU_MENUITEM_PROP_VISIBLE               "visible"
/**
 * DBUSMENU_MENUITEM_PROP_ENABLED:
 *
 * #DbusmenuMenuitem property used to represent whether the menuitem
 * is clickable or not.  Type: #G_VARIANT_TYPE_BOOLEAN.
 */
#define DBUSMENU_MENUITEM_PROP_ENABLED               "enabled"
/**
 * DBUSMENU_MENUITEM_PROP_LABEL:
 *
 * #DbusmenuMenuitem property used for the text on the menu item.
 * Type: #G_VARIANT_TYPE_STRING
 */
#define DBUSMENU_MENUITEM_PROP_LABEL                 "label"
/**
 * DBUSMENU_MENUITEM_PROP_ICON_NAME:
 *
 * #DbusmenuMenuitem property that is the name of the icon under the
 * Freedesktop.org icon naming spec.  Type: #G_VARIANT_TYPE_STRING
 */
#define DBUSMENU_MENUITEM_PROP_ICON_NAME             "icon-name"
/**
 * DBUSMENU_MENUITEM_PROP_ICON_DATA:
 *
 * #DbusmenuMenuitem property that is the raw data of a custom icon
 * used in the application.  Type: #G_VARIANT_TYPE_VARIANT
 *
 * It is recommended that this is not set directly but instead the
 * libdbusmenu-gtk library is used with the function dbusmenu_menuitem_property_set_image()
 */
#define DBUSMENU_MENUITEM_PROP_ICON_DATA             "icon-data"
/**
 * DBUSMENU_MENUITEM_PROP_ACCESSIBLE_DESC:
 *
 * #DbusmenuMenuitem property used to provide a textual description of any
 * information that the icon may convey. The contents of this property are
 * passed through to assistive technologies such as the Orca screen reader.
 * The contents of this property will not be visible in the menu item. If
 * this property is set, Orca will use this property instead of the label 
 * property.
 * Type: #G_VARIANT_TYPE_STRING
 */
#define DBUSMENU_MENUITEM_PROP_ACCESSIBLE_DESC       "accessible-desc"
/**
 * DBUSMENU_MENUITEM_PROP_TOGGLE_TYPE:
 *
 * #DbusmenuMenuitem property that says what type of toggle entry should
 * be shown in the menu.  Should be either #DBUSMENU_MENUITEM_TOGGLE_CHECK
 * or #DBUSMENU_MENUITEM_TOGGLE_RADIO.  Type: #G_VARIANT_TYPE_STRING
 */
#define DBUSMENU_MENUITEM_PROP_TOGGLE_TYPE           "toggle-type"
/**
 * DBUSMENU_MENUITEM_PROP_TOGGLE_STATE:
 *
 * #DbusmenuMenuitem property that says what state a toggle entry should
 * be shown as the menu.  Should be either #DBUSMENU_MENUITEM_TOGGLE_STATE_UNCHECKED
 * #DBUSMENU_MENUITEM_TOGGLE_STATE_CHECKED or #DBUSMENU_MENUITEM_TOGGLE_STATUE_UNKNOWN.
 * Type: #G_VARIANT_TYPE_INT32
 */
#define DBUSMENU_MENUITEM_PROP_TOGGLE_STATE          "toggle-state"
/**
 * DBUSMENU_MENUITEM_PROP_SHORTCUT:
 *
 * #DbusmenuMenuitem property that is the entries that represent a shortcut
 * to activate the menuitem.  It is an array of arrays of strings.
 * Type: #G_VARIANT_TYPE_ARRAY
 *
 * It is recommended that this is not set directly but instead the
 * libdbusmenu-gtk library is used with the function dbusmenu_menuitem_property_set_shortcut()
 */
#define DBUSMENU_MENUITEM_PROP_SHORTCUT              "shortcut"
/**
 * DBUSMENU_MENUITEM_PROP_CHILD_DISPLAY:
 *
 * #DbusmenuMenuitem property that tells how the children of this menuitem
 * should be displayed.  Most likely this will be unset or of the value
 * #DBUSMENU_MENUITEM_CHILD_DISPLAY_SUBMENU.  Type: #G_VARIANT_TYPE_STRING
 */
#define DBUSMENU_MENUITEM_PROP_CHILD_DISPLAY         "children-display"
/**
 * DBUSMENU_MENUITEM_PROP_DISPOSITION:
 *
 * #DbusmenuMenuitem property to tell what type of information that the
 * menu item is displaying to the user.  Type: #G_VARIANT_TYPE_STRING
 */
#define DBUSMENU_MENUITEM_PROP_DISPOSITION           "disposition"

/* ***************************************** */
/* *********    Toggle Values      ********* */
/* ***************************************** */
/**
 * DBUSMENU_MENUITEM_TOGGLE_CHECK:
 *
 * Used to set #DBUSMENU_MENUITEM_PROP_TOGGLE_TYPE to be a standard
 * check mark item.
 */
#define DBUSMENU_MENUITEM_TOGGLE_CHECK               "checkmark"
/**
 * DBUSMENU_MENUITEM_TOGGLE_RADIO:
 *
 * Used to set #DBUSMENU_MENUITEM_PROP_TOGGLE_TYPE to be a standard
 * radio item.
 */
#define DBUSMENU_MENUITEM_TOGGLE_RADIO               "radio"

/* ***************************************** */
/* *********    Toggle States      ********* */
/* ***************************************** */
/**
 * DBUSMENU_MENUITEM_TOGGLE_STATE_UNCHECKED:
 *
 * Used to set #DBUSMENU_MENUITEM_PROP_TOGGLE_STATE so that the menu's
 * toggle item is empty.
 */
#define DBUSMENU_MENUITEM_TOGGLE_STATE_UNCHECKED     0
/**
 * DBUSMENU_MENUITEM_TOGGLE_STATE_CHECKED:
 *
 * Used to set #DBUSMENU_MENUITEM_PROP_TOGGLE_STATE so that the menu's
 * toggle item is filled.
 */
#define DBUSMENU_MENUITEM_TOGGLE_STATE_CHECKED       1
/**
 * DBUSMENU_MENUITEM_TOGGLE_STATE_UNKNOWN:
 *
 * Used to set #DBUSMENU_MENUITEM_PROP_TOGGLE_STATE so that the menu's
 * toggle item is undecided.
 */
#define DBUSMENU_MENUITEM_TOGGLE_STATE_UNKNOWN       -1

/* ***************************************** */
/* *********    Icon specials      ********* */
/* ***************************************** */
/**
 * DBUSMENU_MENUITEM_ICON_NAME_BLANK:
 *
 * Used to set #DBUSMENU_MENUITEM_PROP_TOGGLE_STATE so that the menu's
 * toggle item is undecided.
 */
#define DBUSMENU_MENUITEM_ICON_NAME_BLANK            "blank-icon"

/* ***************************************** */
/* *********  Shortcut Modifiers   ********* */
/* ***************************************** */
/**
 * DBUSMENU_MENUITEM_SHORTCUT_CONTROL:
 *
 * Used in #DBUSMENU_MENUITEM_PROP_SHORTCUT to represent the
 * control key.
 */
#define DBUSMENU_MENUITEM_SHORTCUT_CONTROL           "Control"
/**
 * DBUSMENU_MENUITEM_SHORTCUT_ALT:
 *
 * Used in #DBUSMENU_MENUITEM_PROP_SHORTCUT to represent the
 * alternate key.
 */
#define DBUSMENU_MENUITEM_SHORTCUT_ALT               "Alt"
/**
 * DBUSMENU_MENUITEM_SHORTCUT_SHIFT:
 *
 * Used in #DBUSMENU_MENUITEM_PROP_SHORTCUT to represent the
 * shift key.
 */
#define DBUSMENU_MENUITEM_SHORTCUT_SHIFT             "Shift"
/**
 * DBUSMENU_MENUITEM_SHORTCUT_SUPER:
 *
 * Used in #DBUSMENU_MENUITEM_PROP_SHORTCUT to represent the
 * super key.
 */
#define DBUSMENU_MENUITEM_SHORTCUT_SUPER             "Super"

/* ***************************************** */
/* *********  Child Display Types  ********* */
/* ***************************************** */
/**
 * DBUSMENU_MENUITEM_CHILD_DISPLAY_SUBMENU:
 *
 * Used in #DBUSMENU_MENUITEM_PROP_CHILD_DISPLAY to have the
 * subitems displayed as a submenu.
 */
#define DBUSMENU_MENUITEM_CHILD_DISPLAY_SUBMENU      "submenu"

/* ***************************************** */
/* ********* Menuitem Dispositions ********* */
/* ***************************************** */
/**
 * DBUSMENU_MENUITEM_DISPOSITION_NORMAL:
 *
 * Used in #DBUSMENU_MENUITEM_PROP_DISPOSITION to have a menu
 * item displayed in the normal manner.  Default value.
 */
#define DBUSMENU_MENUITEM_DISPOSITION_NORMAL         "normal"
/**
 * DBUSMENU_MENUITEM_DISPOSITION_INFORMATIVE:
 *
 * Used in #DBUSMENU_MENUITEM_PROP_DISPOSITION to have a menu
 * item displayed in a way that conveys it's giving additional
 * information to the user.
 */
#define DBUSMENU_MENUITEM_DISPOSITION_INFORMATIVE    "informative"
/**
 * DBUSMENU_MENUITEM_DISPOSITION_WARNING:
 *
 * Used in #DBUSMENU_MENUITEM_PROP_DISPOSITION to have a menu
 * item displayed in a way that conveys it's giving a warning
 * to the user.
 */
#define DBUSMENU_MENUITEM_DISPOSITION_WARNING        "warning"
/**
 * DBUSMENU_MENUITEM_DISPOSITION_ALERT:
 *
 * Used in #DBUSMENU_MENUITEM_PROP_DISPOSITION to have a menu
 * item displayed in a way that conveys it's giving an alert
 * to the user.
 */
#define DBUSMENU_MENUITEM_DISPOSITION_ALERT          "alert"

/* ***************************************** */
/* *********   Dbusmenu Events     ********* */
/* ***************************************** */
/**
 * DBUSMENU_MENUITEM_EVENT_ACTIVATED:
 *
 * String for the event identifier when a menu item is clicked
 * on by the user.
 */
#define DBUSMENU_MENUITEM_EVENT_ACTIVATED            "clicked"

/**
 * DBUSMENU_MENUITEM_EVENT_OPENED:
 *
 * String for the event identifier when a menu is opened and
 * displayed to the user.  Only valid for items that contain
 * submenus.
 */
#define DBUSMENU_MENUITEM_EVENT_OPENED               "opened"

/**
 * DBUSMENU_MENUITEM_EVENT_CLOSED:
 *
 * String for the event identifier when a menu is closed and
 * displayed to the user.  Only valid for items that contain
 * submenus.
 */
#define DBUSMENU_MENUITEM_EVENT_CLOSED               "closed"

typedef struct _DbusmenuMenuitemPrivate DbusmenuMenuitemPrivate;

/**
 * DbusmenuMenuitem:
 * @parent: Parent object
 * @priv: Private data
 * 
 * This is the #GObject based object that represents a menu
 * item.  It gets created the same on both the client and
 * the server side and libdbusmenu-glib does the work of making
 * this object model appear on both sides of DBus.  Simple
 * really, though through updates and people coming on and off
 * the bus it can lead to lots of fun complex scenarios.
 */
typedef struct _DbusmenuMenuitem      DbusmenuMenuitem;
struct _DbusmenuMenuitem
{
	GObject parent;

	/*< Private >*/
	DbusmenuMenuitemPrivate * priv;
};

/**
 * dbusmenu_menuitem_about_to_show_cb:
 * @mi: Menu item that should be shown
 * @user_data: (closure): Extra user data sent with the function
 * 
 * Callback prototype for a callback that is called when the
 * menu should be shown.
 */
typedef void (*dbusmenu_menuitem_about_to_show_cb) (DbusmenuMenuitem * mi, gpointer user_data);

/**
 * dbusmenu_menuitem_buildvariant_slot_t:
 * @mi: (in): Menu item that should be built from
 * @properties: (allow-none): A list of properties that should be the only ones in the resulting variant structure
 * 
 * This is the function that is called to represent this menu item
 * as a variant.  Should call its own children.
 *
 * Return value: (transfer full): A variant representing this item and its children
 */
typedef GVariant * (*dbusmenu_menuitem_buildvariant_slot_t) (DbusmenuMenuitem * mi, gchar ** properties);

/**
 * DbusmenuMenuitemClass:
 * @parent_class: Functions and signals from our parent
 * @property_changed: Slot for #DbusmenuMenuitem::property-changed.
 * @item_activated: Slot for #DbusmenuMenuitem::item-activated.
 * @child_added: Slot for #DbusmenuMenuitem::child-added.
 * @child_removed: Slot for #DbusmenuMenuitem::child-removed.
 * @child_moved: Slot for #DbusmenuMenuitem::child-moved.
 * @realized: Slot for #DbusmenuMenuitem::realized.
 * @about_to_show: Slot for #DbusmenuMenuitem::about-to-show.
 * @buildvariant: Virtual function that appends the strings required to represent this menu item in the menu variant.
 * @handle_event: This function is to override how events are handled by subclasses.  Look at #dbusmenu_menuitem_handle_event for lots of good information.
 * @send_about_to_show: Virtual function that notifies server that the client is about to show a menu.
 * @show_to_user: Slot for #DbusmenuMenuitem::show-to-user.
 * @event: Slot for #DbsumenuMenuitem::event.
 * @reserved1: Reserved for future use.
 * @reserved2: Reserved for future use.
 * @reserved3: Reserved for future use.
 * @reserved4: Reserved for future use.
 * @reserved5: Reserved for future use.
 *
 * Functions and signals that every menuitem should know something 
 * about.
 */
typedef struct _DbusmenuMenuitemClass DbusmenuMenuitemClass;
struct _DbusmenuMenuitemClass
{
	GObjectClass parent_class;

	/* Signals */
	void (*property_changed) (gchar * property, GVariant * value);
	void (*item_activated) (guint timestamp);
	void (*child_added) (DbusmenuMenuitem * child, guint position);
	void (*child_removed) (DbusmenuMenuitem * child);
	void (*child_moved) (DbusmenuMenuitem * child, guint newpos, guint oldpos);
	void (*realized) (void);

	/* Virtual functions */
	dbusmenu_menuitem_buildvariant_slot_t buildvariant;
	void (*handle_event) (DbusmenuMenuitem * mi, const gchar * name, GVariant * variant, guint timestamp);
	void (*send_about_to_show) (DbusmenuMenuitem * mi, dbusmenu_menuitem_about_to_show_cb cb, gpointer cb_data);

	void (*show_to_user) (DbusmenuMenuitem * mi, guint timestamp, gpointer cb_data);
	gboolean (*about_to_show) (void);

	void (*event) (const gchar * name, GVariant * value, guint timestamp);

	/*< Private >*/
	void (*reserved1) (void);
	void (*reserved2) (void);
	void (*reserved3) (void);
	void (*reserved4) (void);
	void (*reserved5) (void);
};

GType dbusmenu_menuitem_get_type (void);

DbusmenuMenuitem * dbusmenu_menuitem_new (void) G_GNUC_WARN_UNUSED_RESULT;
DbusmenuMenuitem * dbusmenu_menuitem_new_with_id (gint id) G_GNUC_WARN_UNUSED_RESULT;
gint dbusmenu_menuitem_get_id (DbusmenuMenuitem * mi);

GList * dbusmenu_menuitem_get_children (DbusmenuMenuitem * mi);
GList * dbusmenu_menuitem_take_children (DbusmenuMenuitem * mi) G_GNUC_WARN_UNUSED_RESULT;
guint dbusmenu_menuitem_get_position (DbusmenuMenuitem * mi, DbusmenuMenuitem * parent);
guint dbusmenu_menuitem_get_position_realized (DbusmenuMenuitem * mi, DbusmenuMenuitem * parent);

gboolean dbusmenu_menuitem_child_append (DbusmenuMenuitem * mi, DbusmenuMenuitem * child);
gboolean dbusmenu_menuitem_child_prepend (DbusmenuMenuitem * mi, DbusmenuMenuitem * child);
gboolean dbusmenu_menuitem_child_delete (DbusmenuMenuitem * mi, DbusmenuMenuitem * child);
gboolean dbusmenu_menuitem_child_add_position (DbusmenuMenuitem * mi, DbusmenuMenuitem * child, guint position);
gboolean dbusmenu_menuitem_child_reorder (DbusmenuMenuitem * mi, DbusmenuMenuitem * child, guint position);
DbusmenuMenuitem * dbusmenu_menuitem_child_find (DbusmenuMenuitem * mi, gint id);
DbusmenuMenuitem * dbusmenu_menuitem_find_id (DbusmenuMenuitem * mi, gint id);

gboolean dbusmenu_menuitem_set_parent (DbusmenuMenuitem * mi, DbusmenuMenuitem * parent);
gboolean dbusmenu_menuitem_unparent (DbusmenuMenuitem *mi);
DbusmenuMenuitem * dbusmenu_menuitem_get_parent (DbusmenuMenuitem * mi);

gboolean dbusmenu_menuitem_property_set (DbusmenuMenuitem * mi, const gchar * property, const gchar * value);
gboolean dbusmenu_menuitem_property_set_variant (DbusmenuMenuitem * mi, const gchar * property, GVariant * value);
gboolean dbusmenu_menuitem_property_set_bool (DbusmenuMenuitem * mi, const gchar * property, const gboolean value);
gboolean dbusmenu_menuitem_property_set_int (DbusmenuMenuitem * mi, const gchar * property, const gint value);
gboolean dbusmenu_menuitem_property_set_byte_array (DbusmenuMenuitem * mi, const gchar * property, const guchar * value, gsize nelements);
const gchar * dbusmenu_menuitem_property_get (const DbusmenuMenuitem * mi, const gchar * property);
GVariant * dbusmenu_menuitem_property_get_variant (const DbusmenuMenuitem * mi, const gchar * property);
gboolean dbusmenu_menuitem_property_get_bool (const DbusmenuMenuitem * mi, const gchar * property);
gint dbusmenu_menuitem_property_get_int (const DbusmenuMenuitem * mi, const gchar * property);
const guchar * dbusmenu_menuitem_property_get_byte_array (const DbusmenuMenuitem * mi, const gchar * property, gsize * nelements);
gboolean dbusmenu_menuitem_property_exist (const DbusmenuMenuitem * mi, const gchar * property);
GList * dbusmenu_menuitem_properties_list (DbusmenuMenuitem * mi) G_GNUC_WARN_UNUSED_RESULT;
GHashTable * dbusmenu_menuitem_properties_copy (DbusmenuMenuitem * mi);
void dbusmenu_menuitem_property_remove (DbusmenuMenuitem * mi, const gchar * property);

void dbusmenu_menuitem_set_root (DbusmenuMenuitem * mi, gboolean root);
gboolean dbusmenu_menuitem_get_root (DbusmenuMenuitem * mi);

void dbusmenu_menuitem_foreach (DbusmenuMenuitem * mi, void (*func) (DbusmenuMenuitem * mi, gpointer data), gpointer data);
void dbusmenu_menuitem_handle_event (DbusmenuMenuitem * mi, const gchar * name, GVariant * variant, guint timestamp);
void dbusmenu_menuitem_send_about_to_show (DbusmenuMenuitem * mi, void (*cb) (DbusmenuMenuitem * mi, gpointer user_data), gpointer cb_data);

void dbusmenu_menuitem_show_to_user (DbusmenuMenuitem * mi, guint timestamp);

/**
 * SECTION:menuitem
 * @short_description: A lowlevel represenation of a menuitem
 * @stability: Unstable
 * @include: libdbusmenu-glib/menuitem.h
 * 
 * A #DbusmenuMenuitem is the lowest level of represenation of a
 * single item in a menu.  It gets created on the server side
 * and copied over to the client side where it gets rendered.  As
 * the server starts to change it, and grow it, and do all kinds
 * of fun stuff that information is transfered over DBus and the
 * client updates its understanding of the object model.
 * 
 * Most people using either the client or the server should be
 * able to deal mostly with #DbusmenuMenuitem objects.  These
 * are simple, but then they can be attached to more complex
 * objects and handled appropriately.
 */

G_END_DECLS

#endif
