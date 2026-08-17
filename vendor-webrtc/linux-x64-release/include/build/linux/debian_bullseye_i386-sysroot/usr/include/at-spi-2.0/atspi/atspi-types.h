/*
 * AT-SPI - Assistive Technology Service Provider Interface
 * (Gnome Accessibility Project; http://developer.gnome.org/projects/gap)
 *
 * Copyright 2002 Ximian, Inc.
 *           2002 Sun Microsystems Inc.
 *           
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
 * License along with this library; if not, write to the
 * Free Software Foundation, Inc., 51 Franklin Street, Fifth Floor,
 * Boston, MA 02110-1301, USA.
 */

#ifndef _ATSPI_TYPES_H_
#define _ATSPI_TYPES_H_

#include "glib-object.h"

#include "atspi-constants.h"

typedef struct _AtspiAccessible AtspiAccessible;
typedef struct _AtspiAction AtspiAction;
typedef struct _AtspiCollection AtspiCollection;
typedef struct _AtspiComponent AtspiComponent;
typedef struct _AtspiDocument AtspiDocument;
typedef struct _AtspiEditableText AtspiEditableText;
typedef struct _AtspiHyperlink AtspiHyperlink;
typedef struct _AtspiHypertext AtspiHypertext;
typedef struct _AtspiImage AtspiImage;
typedef struct _AtspiSelection AtspiSelection;
typedef struct _AtspiTable AtspiTable;
typedef struct _AtspiTableCell AtspiTableCell;
typedef struct _AtspiText AtspiText;
typedef struct _AtspiValue AtspiValue;

typedef guint AtspiControllerEventMask;

typedef guint AtspiKeyMaskType;

typedef guint AtspiKeyEventMask;
typedef guint AtspiDeviceEventMask;

// TODO: auto-generate the below structs
typedef struct _AtspiDeviceEvent AtspiDeviceEvent;
struct _AtspiDeviceEvent
{
  AtspiEventType type;
  guint id;
  gushort hw_code;
  gushort modifiers;
  guint timestamp;
  gchar * event_string;
  gboolean is_text;
};

typedef struct _AtspiEventListenerMode AtspiEventListenerMode;
struct _AtspiEventListenerMode
{
  gboolean synchronous;
  gboolean preemptive;
  gboolean global;
};

typedef struct _AtspiKeyDefinition AtspiKeyDefinition;
struct _AtspiKeyDefinition
{
  gint keycode;
  gint keysym;
  gchar *keystring;
  guint modifiers;
};

/**
 * ATSPI_TYPE_KEY_DEFINITION:
 * 
 * The #GType for a boxed type holding a #AtspiKeyDefinition.
 */
#define	ATSPI_TYPE_KEY_DEFINITION (atspi_key_definition_get_type ())

typedef struct _AtspiEvent AtspiEvent;
struct _AtspiEvent
{
  gchar  *type;
  AtspiAccessible  *source;
  gint         detail1;
  gint         detail2;
  GValue any_data;
  AtspiAccessible *sender;
};

/**
 * ATSPI_TYPE_DEVICE_EVENT:
 * 
 * The #GType for a boxed type holding a #AtspiDeviceEvent.
 */
#define	ATSPI_TYPE_DEVICE_EVENT (atspi_device_event_get_type ())

/**
 * ATSPI_TYPE_EVENT:
 * 
 * The #GType for a boxed type holding a #AtspiEvent.
 */
#define	ATSPI_TYPE_EVENT (atspi_event_get_type ())

typedef void AtspiKeystrokeListener;

/**
 * AtspiKeySet:
 * @keysyms:
 * @keycodes:
 * @len:
 *
 * Structure containing identifying information about a set of keycode or
 *        keysyms.
 **/
typedef struct _AtspiKeySet
{
  guint *keysyms;
  gushort *keycodes;
  gchar          **keystrings;
  gshort           len;
} AtspiKeySet;

/**
 *AtspiKeyListenerSyncType:
  * @ATSPI_KEYLISTENER_NOSYNC: Events may be delivered asynchronously, 
 * which means in some cases they may already have been delivered to the 
 * application before the AT client receives the notification.  
 * @ATSPI_KEYLISTENER_SYNCHRONOUS: Events are delivered synchronously, before the 
 * currently focussed application sees them.  
 * @ATSPI_KEYLISTENER_CANCONSUME: Events may be consumed by the AT client.  Presumes and
 * requires #ATSPI_KEYLISTENER_SYNCHRONOUS, incompatible with #ATSPI_KEYLISTENER_NOSYNC.
 * @ATSPI_KEYLISTENER_ALL_WINDOWS: Events are received not from the application toolkit layer, but
 * from the device driver or windowing system subsystem; such notifications are 'global' in the 
 * sense that they are not broken or defeated by applications that participate poorly
 * in the accessibility APIs, or not at all; however because of the intrusive nature of
 * such snooping, it can have side-effects on certain older platforms.  If unconditional
 * event notifications, even when inaccessible or "broken" applications have focus, are not
 * required, it may be best to avoid this enum value/flag.
 *
 * Specifies the type of a key listener event.
 * The values above can and should be bitwise-'OR'-ed
 * together, observing the compatibility limitations specified in the description of
 * each value.  For instance, #ATSPI_KEYLISTENER_ALL_WINDOWS | #ATSPI_KEYLISTENER_CANCONSUME is
 * a commonly used combination which gives the AT complete control over the delivery of matching
 * events.  However, such filters should be used sparingly as they may have a negative impact on 
 * system performance.
 **/
typedef enum {
  ATSPI_KEYLISTENER_NOSYNC = 0,
  ATSPI_KEYLISTENER_SYNCHRONOUS = 1 << 0,
  ATSPI_KEYLISTENER_CANCONSUME = 1 << 1,
  ATSPI_KEYLISTENER_ALL_WINDOWS = 1 << 2
} AtspiKeyListenerSyncType;
#endif	/* _ATSPI_TYPES_H_ */
