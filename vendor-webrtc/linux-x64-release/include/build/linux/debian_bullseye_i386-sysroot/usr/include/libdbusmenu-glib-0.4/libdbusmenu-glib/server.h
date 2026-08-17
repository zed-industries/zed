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

#ifndef __DBUSMENU_SERVER_H__
#define __DBUSMENU_SERVER_H__

#include <glib.h>
#include <glib-object.h>

#include "menuitem.h"
#include "types.h"

G_BEGIN_DECLS

#define DBUSMENU_TYPE_SERVER            (dbusmenu_server_get_type ())
#define DBUSMENU_SERVER(obj)            (G_TYPE_CHECK_INSTANCE_CAST ((obj), DBUSMENU_TYPE_SERVER, DbusmenuServer))
#define DBUSMENU_SERVER_CLASS(klass)    (G_TYPE_CHECK_CLASS_CAST ((klass), DBUSMENU_TYPE_SERVER, DbusmenuServerClass))
#define DBUSMENU_IS_SERVER(obj)         (G_TYPE_CHECK_INSTANCE_TYPE ((obj), DBUSMENU_TYPE_SERVER))
#define DBUSMENU_IS_SERVER_CLASS(klass) (G_TYPE_CHECK_CLASS_TYPE ((klass), DBUSMENU_TYPE_SERVER))
#define DBUSMENU_SERVER_GET_CLASS(obj)  (G_TYPE_INSTANCE_GET_CLASS ((obj), DBUSMENU_TYPE_SERVER, DbusmenuServerClass))

/**
 * DBUSMENU_SERVER_SIGNAL_ID_PROP_UPDATE:
 *
 * String to attach to signal #DbusmenuServer::item-property-updated
 */
#define DBUSMENU_SERVER_SIGNAL_ID_PROP_UPDATE  "item-property-updated"
/**
 * DBUSMENU_SERVER_SIGNAL_ID_UPDATE:
 *
 * String to attach to signal #DbusmenuServer::item-updated
 */
#define DBUSMENU_SERVER_SIGNAL_ID_UPDATE       "item-updated"
/**
 * DBUSMENU_SERVER_SIGNAL_LAYOUT_UPDATED:
 *
 * String to attach to signal #DbusmenuServer::layout-updated
 */
#define DBUSMENU_SERVER_SIGNAL_LAYOUT_UPDATED  "layout-updated"
/**
 * DBUSMENU_SERVER_SIGNAL_ITEM_ACTIVATION:
 *
 * String to attach to signal #DbusmenuServer::item-activation-requested
 */
#define DBUSMENU_SERVER_SIGNAL_ITEM_ACTIVATION "item-activation-requested"
/**
 * DBUSMENU_SERVER_SIGNAL_LAYOUT_UPDATE:
 *
 * String to attach to signal #DbusmenuServer::layout-updated
 */
#define DBUSMENU_SERVER_SIGNAL_LAYOUT_UPDATE   DBUSMENU_SERVER_SIGNAL_LAYOUT_UPDATED

/**
 * DBUSMENU_SERVER_PROP_DBUS_OBJECT:
 *
 * String to access property #DbusmenuServer:dbus-object
 */
#define DBUSMENU_SERVER_PROP_DBUS_OBJECT       "dbus-object"
/**
 * DBUSMENU_SERVER_PROP_ROOT_NODE:
 *
 * String to access property #DbusmenuServer:root-node
 */
#define DBUSMENU_SERVER_PROP_ROOT_NODE         "root-node"
/**
 * DBUSMENU_SERVER_PROP_VERSION:
 *
 * String to access property #DbusmenuServer:version
 */
#define DBUSMENU_SERVER_PROP_VERSION           "version"
/**
 * DBUSMENU_SERVER_PROP_TEXT_DIRECTION:
 *
 * String to access property #DbusmenuServer:text-direction
 */
#define DBUSMENU_SERVER_PROP_TEXT_DIRECTION    "text-direction"
/**
 * DBUSMENU_SERVER_PROP_STATUS:
 *
 * String to access property #DbusmenuServer:status
 */
#define DBUSMENU_SERVER_PROP_STATUS            "status"

typedef struct _DbusmenuServerPrivate DbusmenuServerPrivate;

/**
	DbusmenuServerClass:
	@parent_class: #GObjectClass
	@id_prop_update: Slot for #DbusmenuServer::id-prop-update.
	@id_update: Slot for #DbusmenuServer::id-update.
	@layout_updated: Slot for #DbusmenuServer::layout-update.
	@item_activation: Slot for #DbusmenuServer::item-activation-requested.
	@reserved1: Reserved for future use.
	@reserved2: Reserved for future use.
	@reserved3: Reserved for future use.
	@reserved4: Reserved for future use.
	@reserved5: Reserved for future use.
	@reserved6: Reserved for future use.

	The class implementing the virtual functions for #DbusmenuServer.
*/
typedef struct _DbusmenuServerClass DbusmenuServerClass;
struct _DbusmenuServerClass {
	GObjectClass parent_class;

	/* Signals */
	void (*id_prop_update)(gint id, gchar * property, gchar * value);
	void (*id_update)(gint id);
	void (*layout_updated)(gint revision);
	void (*item_activation)(gint id, guint timestamp);

	/*< Private >*/
	void (*reserved1) (void);
	void (*reserved2) (void);
	void (*reserved3) (void);
	void (*reserved4) (void);
	void (*reserved5) (void);
	void (*reserved6) (void);
};

/**
	DbusmenuServer:

	A server which represents a sharing of a set of
	#DbusmenuMenuitems across DBus to a #DbusmenuClient.
*/
typedef struct _DbusmenuServer      DbusmenuServer;
struct _DbusmenuServer {
	/*< private >*/
	GObject parent;

	/*< Private >*/
	DbusmenuServerPrivate * priv;
};

GType                   dbusmenu_server_get_type            (void);
DbusmenuServer *        dbusmenu_server_new                 (const gchar *          object);
void                    dbusmenu_server_set_root            (DbusmenuServer *       self,
                                                             DbusmenuMenuitem *     root);
DbusmenuTextDirection   dbusmenu_server_get_text_direction  (DbusmenuServer *       server);
void                    dbusmenu_server_set_text_direction  (DbusmenuServer *       server,
                                                             DbusmenuTextDirection  dir);
DbusmenuStatus          dbusmenu_server_get_status          (DbusmenuServer *       server);
void                    dbusmenu_server_set_status          (DbusmenuServer *       server,
                                                             DbusmenuStatus         status);
GStrv                   dbusmenu_server_get_icon_paths      (DbusmenuServer *       server);
void                    dbusmenu_server_set_icon_paths      (DbusmenuServer *       server,
                                                             GStrv                  icon_paths);

/**
	SECTION:server
	@short_description: The server signals changed and
		updates on a tree of #DbusmenuMenuitem objecs.
	@stability: Unstable
	@include: libdbusmenu-glib/server.h

	A #DbusmenuServer is the object that represents the local
	tree of #DbusmenuMenuitem objects on DBus.  It watches the
	various signals that those objects emit and correctly
	represents them across DBus to a #DbusmenuClient so that
	the same tree can be maintained in another process.

	The server needs to have the root set of #DbusmenuMenuitem
	objects set via #dbusmenu_server_set_root but it will query
	all of the objects in that tree automatically.  After setting
	the root there should be no other maintence required by
	users of the server class.
*/
G_END_DECLS

#endif
