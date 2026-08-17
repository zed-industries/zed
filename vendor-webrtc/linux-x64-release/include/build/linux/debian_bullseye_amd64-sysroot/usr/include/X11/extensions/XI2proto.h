/*
 * Copyright © 2009 Red Hat, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice (including the next
 * paragraph) shall be included in all copies or substantial portions of the
 * Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
 * FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
 * DEALINGS IN THE SOFTWARE.
 *
 */

/* Conventions for this file:
 * Names:
 * structs: always typedef'd, prefixed with xXI, CamelCase
 * struct members: lower_case_with_underscores
 *        Exceptions: reqType, ReqType, repType, RepType, sequenceNumber are
 *        named as such for historical reasons.
 * request opcodes: X_XIRequestName as CamelCase
 * defines: defines used in client applications must go in XI2.h
 *          defines used only in protocol handling: XISOMENAME
 *
 * Data types: unless there is a historical name for a datatype (e.g.
 * Window), use stdint types specifying the size of the datatype.
 * historical data type names must be defined and undefined at the top and
 * end of the file.
 *
 * General:
 * spaces, not tabs.
 * structs specific to a request or reply added before the request
 *      definition. structs used in more than one request, reply or event
 *      appended to the common structs section before the definition of the
 *      first request.
 * members of structs vertically aligned on column 16 if datatypes permit.
 *      otherwise aligned on next available 8n column.
 */

/**
 * Protocol definitions for the XI2 protocol.
 * This file should not be included by clients that merely use XI2, but do not
 * need the wire protocol. Such clients should include XI2.h, or the matching
 * header from the library.
 *
 */
#ifndef _XI2PROTO_H_
#define _XI2PROTO_H_

#include <X11/Xproto.h>
#include <X11/X.h>
#include <X11/extensions/XI2.h>
#include <stdint.h>

/* make sure types have right sizes for protocol structures. */
#define Window  uint32_t
#define Time    uint32_t
#define Atom    uint32_t
#define Cursor  uint32_t
#define Barrier uint32_t

/**
 * XI2 Request opcodes
 */
#define X_XIQueryPointer                40
#define X_XIWarpPointer                 41
#define X_XIChangeCursor                42
#define X_XIChangeHierarchy             43
#define X_XISetClientPointer            44
#define X_XIGetClientPointer            45
#define X_XISelectEvents                46
#define X_XIQueryVersion                47
#define X_XIQueryDevice                 48
#define X_XISetFocus                    49
#define X_XIGetFocus                    50
#define X_XIGrabDevice                  51
#define X_XIUngrabDevice                52
#define X_XIAllowEvents                 53
#define X_XIPassiveGrabDevice           54
#define X_XIPassiveUngrabDevice         55
#define X_XIListProperties              56
#define X_XIChangeProperty              57
#define X_XIDeleteProperty              58
#define X_XIGetProperty                 59
#define X_XIGetSelectedEvents           60
#define X_XIBarrierReleasePointer       61

/** Number of XI requests */
#define XI2REQUESTS (X_XIBarrierReleasePointer - X_XIQueryPointer + 1)
/** Number of XI2 events */
#define XI2EVENTS   (XI_LASTEVENT + 1)

/*************************************************************************************
 *                                                                                   *
 *                               COMMON STRUCTS                                      *
 *                                                                                   *
 *************************************************************************************/
/** Fixed point 16.16 */
typedef int32_t FP1616;

/** Fixed point 32.32 */
typedef struct {
    int32_t     integral;
    uint32_t    frac;
} FP3232;

/**
 * Struct to describe a device.
 *
 * For a MasterPointer or a MasterKeyboard, 'attachment' specifies the
 * paired master device.
 * For a SlaveKeyboard or SlavePointer, 'attachment' specifies the master
 * device this device is attached to.
 * For a FloatingSlave, 'attachment' is undefined.
 */
typedef struct {
    uint16_t    deviceid;
    uint16_t    use;            /**< ::XIMasterPointer, ::XIMasterKeyboard,
                                     ::XISlavePointer, ::XISlaveKeyboard,
                                     ::XIFloatingSlave */
    uint16_t    attachment;     /**< Current attachment or pairing.*/
    uint16_t    num_classes;    /**< Number of classes following this struct. */
    uint16_t    name_len;       /**< Length of name in bytes. */
    uint8_t     enabled;        /**< TRUE if device is enabled. */
    uint8_t     pad;
} xXIDeviceInfo;

/**
 * Default template for a device class.
 * A device class is equivalent to a device's capabilities. Multiple classes
 * are supported per device.
 */
typedef struct {
    uint16_t    type;           /**< One of *class */
    uint16_t    length;         /**< Length in 4 byte units */
    uint16_t    sourceid;       /**< source device for this class */
    uint16_t    pad;
} xXIAnyInfo;

/**
 * Denotes button capability on a device.
 * Struct is followed by a button bit-mask (padded to four byte chunks) and
 * then num_buttons * Atom that names the buttons in the device-native setup
 * (i.e.  ignoring button mappings).
 */
typedef struct {
    uint16_t    type;           /**< Always ButtonClass */
    uint16_t    length;         /**< Length in 4 byte units */
    uint16_t    sourceid;       /**< source device for this class */
    uint16_t    num_buttons;    /**< Number of buttons provided */
} xXIButtonInfo;

/**
 * Denotes key capability on a device.
 * Struct is followed by num_keys * CARD32 that lists the keycodes available
 * on the device.
 */
typedef struct {
    uint16_t    type;           /**< Always KeyClass */
    uint16_t    length;         /**< Length in 4 byte units */
    uint16_t    sourceid;       /**< source device for this class */
    uint16_t    num_keycodes;   /**< Number of keys provided */
} xXIKeyInfo;

/**
 * Denotes an valuator capability on a device.
 * One XIValuatorInfo describes exactly one valuator (axis) on the device.
 */
typedef struct {
    uint16_t    type;           /**< Always ValuatorClass       */
    uint16_t    length;         /**< Length in 4 byte units */
    uint16_t    sourceid;       /**< source device for this class */
    uint16_t    number;         /**< Valuator number            */
    Atom        label;          /**< Axis label                 */
    FP3232      min;            /**< Min value                  */
    FP3232      max;            /**< Max value                  */
    FP3232      value;          /**< Last published value       */
    uint32_t    resolution;     /**< Resolutions in units/m     */
    uint8_t     mode;           /**< ModeRelative or ModeAbsolute */
    uint8_t     pad1;
    uint16_t    pad2;
} xXIValuatorInfo;

/***
 * Denotes a scroll valuator on a device.
 * One XIScrollInfo describes exactly one scroll valuator that must have a
 * XIValuatorInfo struct.
 */
typedef struct {
    uint16_t    type;           /**< Always ValuatorClass         */
    uint16_t    length;         /**< Length in 4 byte units       */
    uint16_t    sourceid;       /**< source device for this class */
    uint16_t    number;         /**< Valuator number              */
    uint16_t    scroll_type;    /**< ::XIScrollTypeVertical, ::XIScrollTypeHorizontal */
    uint16_t    pad0;
    uint32_t    flags;          /**< ::XIScrollFlagEmulate, ::XIScrollFlagPreferred   */
    FP3232      increment;      /**< Increment for one unit of scrolling              */
} xXIScrollInfo;

/**
 * Denotes multitouch capability on a device.
 */
typedef struct {
    uint16_t    type;           /**< Always TouchClass */
    uint16_t    length;         /**< Length in 4 byte units */
    uint16_t    sourceid;       /**< source device for this class */
    uint8_t     mode;           /**< DirectTouch or DependentTouch */
    uint8_t     num_touches;    /**< Maximum number of touches (0==unlimited) */
} xXITouchInfo;

/**
 * Used to select for events on a given window.
 * Struct is followed by (mask_len * CARD8), with each bit set representing
 * the event mask for the given type. A mask bit represents an event type if
 * (mask == (1 << type)).
 */
typedef struct {
    uint16_t    deviceid;       /**< Device id to select for        */
    uint16_t    mask_len;       /**< Length of mask in 4 byte units */
} xXIEventMask;

/**
 * XKB modifier information.
 * The effective modifier is a binary mask of base, latched, and locked
 * modifiers.
 */
typedef struct
{
    uint32_t    base_mods;              /**< Logically pressed modifiers */
    uint32_t    latched_mods;           /**< Logically latched modifiers */
    uint32_t    locked_mods;            /**< Logically locked modifiers */
    uint32_t    effective_mods;         /**< Effective modifiers */
} xXIModifierInfo;

/**
 * XKB group information.
 * The effective group is the mathematical sum of base, latched, and locked
 * group after group wrapping is taken into account.
 */
typedef struct
{
    uint8_t     base_group;             /**< Logically "pressed" group */
    uint8_t     latched_group;          /**< Logically latched group */
    uint8_t     locked_group;           /**< Logically locked group */
    uint8_t     effective_group;        /**< Effective group */
} xXIGroupInfo;


/*************************************************************************************
 *                                                                                   *
 *                                   REQUESTS                                        *
 *                                                                                   *
 *************************************************************************************/

/**
 * Query the server for the supported X Input extension version.
 */

typedef struct {
    uint8_t     reqType;                /**< Input extension major code */
    uint8_t     ReqType;                /**< Always ::X_XIQueryVersion */
    uint16_t    length;                 /**< Length in 4 byte units */
    uint16_t    major_version;
    uint16_t    minor_version;
} xXIQueryVersionReq;
#define sz_xXIQueryVersionReq                     8

typedef struct {
    uint8_t     repType;                /**< ::X_Reply */
    uint8_t     RepType;                /**< Always ::X_XIQueryVersion */
    uint16_t    sequenceNumber;
    uint32_t    length;
    uint16_t    major_version;
    uint16_t    minor_version;
    uint32_t    pad1;
    uint32_t    pad2;
    uint32_t    pad3;
    uint32_t    pad4;
    uint32_t    pad5;
} xXIQueryVersionReply;
#define sz_xXIQueryVersionReply             32

/**
 * Query the server for information about a specific device or all input
 * devices.
 */
typedef struct {
    uint8_t     reqType;                /**< Input extension major code */
    uint8_t     ReqType;                /**< Always ::X_XIQueryDevice */
    uint16_t    length;                 /**< Length in 4 byte units */
    uint16_t    deviceid;
    uint16_t    pad;
} xXIQueryDeviceReq;
#define sz_xXIQueryDeviceReq                    8

typedef struct {
    uint8_t     repType;                /**< ::X_Reply */
    uint8_t     RepType;                /**< Always ::X_XIQueryDevice */
    uint16_t    sequenceNumber;
    uint32_t    length;
    uint16_t    num_devices;
    uint16_t    pad0;
    uint32_t    pad1;
    uint32_t    pad2;
    uint32_t    pad3;
    uint32_t    pad4;
    uint32_t    pad5;
} xXIQueryDeviceReply;
#define sz_xXIQueryDeviceReply                  32

/**
 * Select for events on a given window.
 */
typedef struct {
    uint8_t     reqType;                /**< Input extension major code */
    uint8_t     ReqType;                /**< Always ::X_XISelectEvents */
    uint16_t    length;                 /**< Length in 4 byte units */
    Window      win;
    uint16_t    num_masks;
    uint16_t    pad;
} xXISelectEventsReq;
#define sz_xXISelectEventsReq                  12

/**
 * Query for selected events on a given window.
 */
typedef struct {
    uint8_t     reqType;                /**< Input extension major code */
    uint8_t     ReqType;                /**< Always ::X_XIGetSelectedEvents */
    uint16_t    length;                 /**< Length in 4 byte units */
    Window      win;
} xXIGetSelectedEventsReq;
#define sz_xXIGetSelectedEventsReq              8

typedef struct {
    uint8_t     repType;                /**< Input extension major opcode */
    uint8_t     RepType;                /**< Always ::X_XIGetSelectedEvents */
    uint16_t    sequenceNumber;
    uint32_t    length;
    uint16_t    num_masks;              /**< Number of xXIEventMask structs
                                             trailing the reply */
    uint16_t    pad0;
    uint32_t    pad1;
    uint32_t    pad2;
    uint32_t    pad3;
    uint32_t    pad4;
    uint32_t    pad5;
} xXIGetSelectedEventsReply;
#define sz_xXIGetSelectedEventsReply            32

/**
 * Query the given device's screen/window coordinates.
 */

typedef struct {
    uint8_t     reqType;                /**< Input extension major code */
    uint8_t     ReqType;                /**< Always ::X_XIQueryPointer */
    uint16_t    length;                 /**< Length in 4 byte units */
    Window      win;
    uint16_t    deviceid;
    uint16_t    pad1;
} xXIQueryPointerReq;
#define sz_xXIQueryPointerReq                   12


typedef struct {
    uint8_t     repType;                /**< Input extension major opcode */
    uint8_t     RepType;                /**< Always ::X_XIQueryPointer */
    uint16_t    sequenceNumber;
    uint32_t    length;
    Window      root;
    Window      child;
    FP1616      root_x;
    FP1616      root_y;
    FP1616      win_x;
    FP1616      win_y;
    uint8_t     same_screen;
    uint8_t     pad0;
    uint16_t    buttons_len;
    xXIModifierInfo mods;
    xXIGroupInfo group;
} xXIQueryPointerReply;
#define sz_xXIQueryPointerReply                 56

/**
 * Warp the given device's pointer to the specified position.
 */

typedef struct {
    uint8_t     reqType;                /**< Input extension major code */
    uint8_t     ReqType;                /**< Always ::X_XIWarpPointer   */
    uint16_t    length;                 /**< Length in 4 byte units */
    Window      src_win;
    Window      dst_win;
    FP1616      src_x;
    FP1616      src_y;
    uint16_t    src_width;
    uint16_t    src_height;
    FP1616      dst_x;
    FP1616      dst_y;
    uint16_t    deviceid;
    uint16_t    pad1;
} xXIWarpPointerReq;
#define sz_xXIWarpPointerReq                    36

/**
 * Change the given device's sprite to the given cursor.
 */

typedef struct {
    uint8_t     reqType;                /**< Input extension major code */
    uint8_t     ReqType;                /**< Always ::X_XIChangeCursor  */
    uint16_t    length;                 /**< Length in 4 byte units */
    Window      win;
    Cursor      cursor;
    uint16_t    deviceid;
    uint16_t    pad1;
} xXIChangeCursorReq;
#define sz_xXIChangeCursorReq                           16

/**
 * Modify the device hierarchy.
 */

typedef struct {
    uint8_t     reqType;                /**< Input extension major code */
    uint8_t     ReqType;                /**< Always ::X_XIChangeHierarchy */
    uint16_t    length;                 /**< Length in 4 byte units */
    uint8_t     num_changes;
    uint8_t     pad0;
    uint16_t    pad1;
} xXIChangeHierarchyReq;
#define sz_xXIChangeHierarchyReq                        8

/**
 * Generic header for any hierarchy change.
 */
typedef struct {
    uint16_t    type;
    uint16_t    length;                 /**< Length in 4 byte units */
} xXIAnyHierarchyChangeInfo;

/**
 * Create a new master device.
 * Name of new master follows struct (4-byte padded)
 */
typedef struct {
    uint16_t    type;                   /**< Always ::XIAddMaster */
    uint16_t    length;                 /**< 2 + (namelen + padding)/4 */
    uint16_t    name_len;
    uint8_t     send_core;
    uint8_t     enable;
} xXIAddMasterInfo;

/**
 * Delete a master device. Will automatically delete the master device paired
 * with the given master device.
 */
typedef struct {
    uint16_t    type;            /**< Always ::XIRemoveMaster */
    uint16_t    length;          /**< 3 */
    uint16_t    deviceid;
    uint8_t     return_mode;     /**< ::XIAttachToMaster, ::XIFloating */
    uint8_t     pad;
    uint16_t    return_pointer;  /**< Pointer to attach slave ptr devices to */
    uint16_t    return_keyboard; /**< keyboard to attach slave keybd devices to*/
} xXIRemoveMasterInfo;

/**
 * Attach an SD to a new device.
 * NewMaster has to be of same type (pointer->pointer, keyboard->keyboard);
 */
typedef struct {
    uint16_t    type;           /**< Always ::XIAttachSlave */
    uint16_t    length;         /**< 2 */
    uint16_t    deviceid;
    uint16_t    new_master;     /**< id of new master device */
} xXIAttachSlaveInfo;

/**
 * Detach an SD from its current master device.
 */
typedef struct {
    uint16_t    type;           /**< Always ::XIDetachSlave */
    uint16_t    length;         /**< 2 */
    uint16_t    deviceid;
    uint16_t    pad;
} xXIDetachSlaveInfo;


/**
 * Set the window/client's ClientPointer.
 */
typedef struct {
    uint8_t     reqType;
    uint8_t     ReqType;                /**< Always ::X_XISetClientPointer */
    uint16_t    length;                 /**< Length in 4 byte units */
    Window      win;
    uint16_t    deviceid;
    uint16_t    pad1;
} xXISetClientPointerReq;
#define sz_xXISetClientPointerReq                 12

/**
 * Query the given window/client's ClientPointer setting.
 */
typedef struct {
    uint8_t     reqType;
    uint8_t     ReqType;                /**< Always ::X_GetClientPointer */
    uint16_t    length;                 /**< Length in 4 byte units */
    Window      win;
} xXIGetClientPointerReq;
#define sz_xXIGetClientPointerReq                 8

typedef struct {
    uint8_t     repType;                /**< Input extension major opcode */
    uint8_t     RepType;                /**< Always ::X_GetClientPointer */
    uint16_t    sequenceNumber;
    uint32_t    length;
    BOOL        set;                    /**< client pointer is set? */
    uint8_t     pad0;
    uint16_t    deviceid;
    uint32_t    pad1;
    uint32_t    pad2;
    uint32_t    pad3;
    uint32_t    pad4;
    uint32_t    pad5;
} xXIGetClientPointerReply;
#define sz_xXIGetClientPointerReply               32

/**
 * Set the input focus to the specified window.
 */
typedef struct {
    uint8_t     reqType;
    uint8_t     ReqType;                /**< Always ::X_XISetFocus */
    uint16_t    length;                 /**< Length in 4 byte units */
    Window      focus;
    Time        time;
    uint16_t    deviceid;
    uint16_t    pad0;
} xXISetFocusReq;
#define sz_xXISetFocusReq                       16

/**
 * Query the current input focus.
 */
typedef struct {
    uint8_t     reqType;
    uint8_t     ReqType;                /**< Always ::X_XIGetDeviceFocus */
    uint16_t    length;                 /**< Length in 4 byte units */
    uint16_t    deviceid;
    uint16_t    pad0;
} xXIGetFocusReq;
#define sz_xXIGetFocusReq                       8

typedef struct {
    uint8_t     repType;                /**< Input extension major opcode */
    uint8_t     RepType;                /**< Always ::X_XIGetFocus */
    uint16_t    sequenceNumber;
    uint32_t    length;
    Window      focus;
    uint32_t    pad1;
    uint32_t    pad2;
    uint32_t    pad3;
    uint32_t    pad4;
    uint32_t    pad5;
} xXIGetFocusReply;
#define sz_xXIGetFocusReply                     32


/**
 * Grab the given device.
 */
typedef struct {
    uint8_t     reqType;
    uint8_t     ReqType;                /**< Always ::X_XIGrabDevice */
    uint16_t    length;                 /**< Length in 4 byte units */
    Window      grab_window;
    Time        time;
    Cursor      cursor;
    uint16_t    deviceid;
    uint8_t     grab_mode;
    uint8_t     paired_device_mode;
    uint8_t     owner_events;
    uint8_t     pad;
    uint16_t    mask_len;
} xXIGrabDeviceReq;
#define sz_xXIGrabDeviceReq                     24

/**
 * Return codes from a XIPassiveGrabDevice request.
 */
typedef struct {
    uint32_t    modifiers;              /**< Modifier state */
    uint8_t     status;                 /**< Grab status code */
    uint8_t     pad0;
    uint16_t    pad1;
} xXIGrabModifierInfo;

typedef struct {
    uint8_t     repType;                /**< Input extension major opcode */
    uint8_t     RepType;                /**< Always ::X_XIGrabDevice */
    uint16_t    sequenceNumber;
    uint32_t    length;
    uint8_t     status;
    uint8_t     pad0;
    uint16_t    pad1;
    uint32_t    pad2;
    uint32_t    pad3;
    uint32_t    pad4;
    uint32_t    pad5;
    uint32_t    pad6;
} xXIGrabDeviceReply;
#define sz_xXIGrabDeviceReply                  32

/**
 * Ungrab the specified device.
 *
 */
typedef struct {
    uint8_t     reqType;
    uint8_t     ReqType;                /**< Always ::X_XIUngrabDevice */
    uint16_t    length;                 /**< Length in 4 byte units */
    Time        time;
    uint16_t    deviceid;
    uint16_t    pad;
} xXIUngrabDeviceReq;
#define sz_xXIUngrabDeviceReq                   12


/**
 * Allow or replay events on the specified grabbed device.
 */
typedef struct {
    uint8_t     reqType;
    uint8_t     ReqType;                /**< Always ::X_XIAllowEvents */
    uint16_t    length;                 /**< Length in 4 byte units */
    Time        time;
    uint16_t    deviceid;
    uint8_t     mode;
    uint8_t     pad;
} xXIAllowEventsReq;
#define sz_xXIAllowEventsReq                   12

/**
 * Allow or replay events on the specified grabbed device.
 * Since XI 2.2
 */
typedef struct {
    uint8_t     reqType;
    uint8_t     ReqType;                /**< Always ::X_XIAllowEvents */
    uint16_t    length;                 /**< Length in 4 byte units */
    Time        time;
    uint16_t    deviceid;
    uint8_t     mode;
    uint8_t     pad;
    uint32_t    touchid;                /**< Since XI 2.2 */
    Window      grab_window;            /**< Since XI 2.2 */
} xXI2_2AllowEventsReq;
#define sz_xXI2_2AllowEventsReq                20


/**
 * Passively grab the device.
 */
typedef struct {
    uint8_t     reqType;
    uint8_t     ReqType;                /**< Always ::X_XIPassiveGrabDevice */
    uint16_t    length;                 /**< Length in 4 byte units */
    Time        time;
    Window      grab_window;
    Cursor      cursor;
    uint32_t    detail;
    uint16_t    deviceid;
    uint16_t    num_modifiers;
    uint16_t    mask_len;
    uint8_t     grab_type;
    uint8_t     grab_mode;
    uint8_t     paired_device_mode;
    uint8_t     owner_events;
    uint16_t    pad1;
} xXIPassiveGrabDeviceReq;
#define sz_xXIPassiveGrabDeviceReq              32

typedef struct {
    uint8_t     repType;                /**< Input extension major opcode */
    uint8_t     RepType;                /**< Always ::X_XIPassiveGrabDevice */
    uint16_t    sequenceNumber;
    uint32_t    length;
    uint16_t    num_modifiers;
    uint16_t    pad1;
    uint32_t    pad2;
    uint32_t    pad3;
    uint32_t    pad4;
    uint32_t    pad5;
    uint32_t    pad6;
} xXIPassiveGrabDeviceReply;
#define sz_xXIPassiveGrabDeviceReply            32

/**
 * Delete a passive grab for the given device.
 */
typedef struct {
    uint8_t     reqType;
    uint8_t     ReqType;                /**< Always ::X_XIPassiveUngrabDevice */
    uint16_t    length;                 /**< Length in 4 byte units */
    Window      grab_window;
    uint32_t    detail;
    uint16_t    deviceid;
    uint16_t    num_modifiers;
    uint8_t     grab_type;
    uint8_t     pad0;
    uint16_t    pad1;
} xXIPassiveUngrabDeviceReq;
#define sz_xXIPassiveUngrabDeviceReq            20

/**
 * List all device properties on the specified device.
 */
typedef struct {
    uint8_t     reqType;
    uint8_t     ReqType;                /**< Always ::X_XIListProperties */
    uint16_t    length;                 /**< Length in 4 byte units */
    uint16_t    deviceid;
    uint16_t    pad;
} xXIListPropertiesReq;
#define sz_xXIListPropertiesReq                 8

typedef struct {
    uint8_t     repType;                /**< Input extension major opcode */
    uint8_t     RepType;                /**< Always ::X_XIListProperties */
    uint16_t    sequenceNumber;
    uint32_t    length;
    uint16_t    num_properties;
    uint16_t    pad0;
    uint32_t    pad1;
    uint32_t    pad2;
    uint32_t    pad3;
    uint32_t    pad4;
    uint32_t    pad5;
} xXIListPropertiesReply;
#define sz_xXIListPropertiesReply               32

/**
 * Change a property on the specified device.
 */
typedef struct {
    uint8_t     reqType;
    uint8_t     ReqType;                /**< Always ::X_XIChangeProperty */
    uint16_t    length;                 /**< Length in 4 byte units */
    uint16_t    deviceid;
    uint8_t     mode;
    uint8_t     format;
    Atom        property;
    Atom        type;
    uint32_t    num_items;
} xXIChangePropertyReq;
#define sz_xXIChangePropertyReq                 20

/**
 * Delete the specified property.
 */
typedef struct {
    uint8_t     reqType;
    uint8_t     ReqType;                /**< Always X_XIDeleteProperty */
    uint16_t    length;                 /**< Length in 4 byte units */
    uint16_t    deviceid;
    uint16_t    pad0;
    Atom        property;
} xXIDeletePropertyReq;
#define sz_xXIDeletePropertyReq                 12

/**
 * Query the specified property's values.
 */
typedef struct {
    uint8_t     reqType;
    uint8_t     ReqType;                /**< Always X_XIGetProperty */
    uint16_t    length;                 /**< Length in 4 byte units */
    uint16_t    deviceid;
#if defined(__cplusplus) || defined(c_plusplus)
    uint8_t     c_delete;
#else
    uint8_t     delete;
#endif
    uint8_t     pad0;
    Atom        property;
    Atom        type;
    uint32_t    offset;
    uint32_t    len;
} xXIGetPropertyReq;
#define sz_xXIGetPropertyReq                    24

typedef struct {
    uint8_t     repType;                /**< Input extension major opcode */
    uint8_t     RepType;                /**< Always X_XIGetProperty */
    uint16_t    sequenceNumber;
    uint32_t    length;
    Atom        type;
    uint32_t    bytes_after;
    uint32_t    num_items;
    uint8_t     format;
    uint8_t     pad0;
    uint16_t    pad1;
    uint32_t    pad2;
    uint32_t    pad3;
} xXIGetPropertyReply;
#define sz_xXIGetPropertyReply               32

typedef struct {
    uint16_t    deviceid;
    uint16_t    pad;
    Barrier     barrier;
    uint32_t    eventid;
} xXIBarrierReleasePointerInfo;

typedef struct {
    uint8_t     reqType;                /**< Input extension major opcode */
    uint8_t     ReqType;                /**< Always X_XIBarrierReleasePointer */
    uint16_t    length;
    uint32_t    num_barriers;
    /* array of xXIBarrierReleasePointerInfo */
} xXIBarrierReleasePointerReq;
#define sz_xXIBarrierReleasePointerReq       8

/*************************************************************************************
 *                                                                                   *
 *                                      EVENTS                                       *
 *                                                                                   *
 *************************************************************************************/

/**
 * Generic XI2 event header. All XI2 events use the same header.
 */
typedef struct
{
    uint8_t     type;
    uint8_t     extension;              /**< XI extension offset */
    uint16_t    sequenceNumber;
    uint32_t    length;
    uint16_t    evtype;
    uint16_t    deviceid;
    Time        time;
} xXIGenericDeviceEvent;

/**
 * Device hierarchy information.
 */
typedef struct
{
    uint16_t    deviceid;
    uint16_t    attachment;             /**< ID of master or paired device */
    uint8_t     use;                    /**< ::XIMasterKeyboard,
                                             ::XIMasterPointer,
                                             ::XISlaveKeyboard,
                                             ::XISlavePointer,
                                             ::XIFloatingSlave */
    BOOL        enabled;                /**< TRUE if the device is enabled */
    uint16_t    pad;
    uint32_t    flags;                  /**< ::XIMasterAdded, ::XIMasterRemoved,
                                             ::XISlaveAttached, ::XISlaveDetached,
                                             ::XISlaveAdded, ::XISlaveRemoved,
                                             ::XIDeviceEnabled, ::XIDeviceDisabled */
} xXIHierarchyInfo;

/**
 * The device hierarchy has been modified. This event includes the device
 * hierarchy after the modification has been applied.
 */
typedef struct
{
    uint8_t     type;                   /**< Always GenericEvent */
    uint8_t     extension;              /**< XI extension offset */
    uint16_t    sequenceNumber;
    uint32_t    length;                 /**< Length in 4 byte units */
    uint16_t    evtype;                 /**< ::XI_Hierarchy */
    uint16_t    deviceid;
    Time        time;
    uint32_t    flags;                  /**< ::XIMasterAdded, ::XIMasterDeleted,
                                             ::XISlaveAttached, ::XISlaveDetached,
                                             ::XISlaveAdded, ::XISlaveRemoved,
                                             ::XIDeviceEnabled, ::XIDeviceDisabled */
    uint16_t    num_info;
    uint16_t    pad0;
    uint32_t    pad1;
    uint32_t    pad2;
} xXIHierarchyEvent;

/**
 * A device has changed capabilities.
 */
typedef struct
{
    uint8_t     type;                   /**< Always GenericEvent */
    uint8_t     extension;              /**< XI extension offset */
    uint16_t    sequenceNumber;
    uint32_t    length;                 /**< Length in 4 byte units */
    uint16_t    evtype;                 /**< XI_DeviceChanged */
    uint16_t    deviceid;               /**< Device that has changed */
    Time        time;
    uint16_t    num_classes;            /**< Number of classes that have changed */
    uint16_t    sourceid;               /**< Source of the new classes */
    uint8_t     reason;                 /**< ::XISlaveSwitch, ::XIDeviceChange */
    uint8_t     pad0;
    uint16_t    pad1;
    uint32_t    pad2;
    uint32_t    pad3;
} xXIDeviceChangedEvent;

/**
 * The owner of a touch stream has passed on ownership to another client.
 */
typedef struct
{
    uint8_t     type;               /**< Always GenericEvent */
    uint8_t     extension;          /**< XI extension offset */
    uint16_t    sequenceNumber;
    uint32_t    length;             /**< Length in 4 byte units */
    uint16_t    evtype;             /**< XI_TouchOwnership */
    uint16_t    deviceid;           /**< Device that has changed */
    Time        time;
    uint32_t    touchid;
    Window      root;
    Window      event;
    Window      child;
/* └──────── 32 byte boundary ────────┘ */
    uint16_t    sourceid;
    uint16_t    pad0;
    uint32_t    flags;
    uint32_t    pad1;
    uint32_t    pad2;
} xXITouchOwnershipEvent;

/**
 * Default input event for pointer, keyboard or touch input.
 */
typedef struct
{
    uint8_t     type;                   /**< Always GenericEvent */
    uint8_t     extension;              /**< XI extension offset */
    uint16_t    sequenceNumber;
    uint32_t    length;                 /**< Length in 4 byte uints */
    uint16_t    evtype;
    uint16_t    deviceid;
    Time        time;
    uint32_t    detail;                 /**< Keycode or button */
    Window      root;
    Window      event;
    Window      child;
/* └──────── 32 byte boundary ────────┘ */
    FP1616      root_x;                 /**< Always screen coords, 16.16 fixed point */
    FP1616      root_y;
    FP1616      event_x;                /**< Always screen coords, 16.16 fixed point */
    FP1616      event_y;
    uint16_t    buttons_len;            /**< Len of button flags in 4 b units */
    uint16_t    valuators_len;          /**< Len of val. flags in 4 b units */
    uint16_t    sourceid;               /**< The source device */
    uint16_t    pad0;
    uint32_t    flags;                  /**< ::XIKeyRepeat */
    xXIModifierInfo     mods;
    xXIGroupInfo        group;
} xXIDeviceEvent;


/**
 * Sent when an input event is generated. RawEvents include valuator
 * information in both device-specific data (i.e. unaccelerated) and
 * processed data (i.e. accelerated, if applicable).
 */
typedef struct
{
    uint8_t     type;                   /**< Always GenericEvent */
    uint8_t     extension;              /**< XI extension offset */
    uint16_t    sequenceNumber;
    uint32_t    length;                 /**< Length in 4 byte uints */
    uint16_t    evtype;                 /**< ::XI_RawEvent */
    uint16_t    deviceid;
    Time        time;
    uint32_t    detail;
    uint16_t    sourceid;               /**< The source device (XI 2.1) */
    uint16_t    valuators_len;          /**< Length of trailing valuator
                                             mask in 4 byte units */
    uint32_t    flags;                  /**< ::XIKeyRepeat */
    uint32_t    pad2;
} xXIRawEvent;

/**
 * Note that the layout of root, event, child, root_x, root_y, event_x,
 * event_y must be identical to the xXIDeviceEvent.
 */
typedef struct
{
    uint8_t     type;                   /**< Always GenericEvent */
    uint8_t     extension;              /**< XI extension offset */
    uint16_t    sequenceNumber;
    uint32_t    length;                 /**< Length in 4 byte uints */
    uint16_t    evtype;                 /**< ::XI_Enter */
    uint16_t    deviceid;
    Time        time;
    uint16_t    sourceid;
    uint8_t     mode;
    uint8_t     detail;
    Window      root;
    Window      event;
    Window      child;
/* └──────── 32 byte boundary ────────┘ */
    FP1616      root_x;
    FP1616      root_y;
    FP1616      event_x;
    FP1616      event_y;
    BOOL        same_screen;
    BOOL        focus;
    uint16_t    buttons_len;            /**< Length of trailing button mask
                                             in 4 byte units */
    xXIModifierInfo     mods;
    xXIGroupInfo        group;
} xXIEnterEvent;

typedef xXIEnterEvent xXILeaveEvent;
typedef xXIEnterEvent xXIFocusInEvent;
typedef xXIEnterEvent xXIFocusOutEvent;

/**
 * Sent when a device property is created, modified or deleted. Does not
 * include property data, the client is required to query the data.
 */
typedef struct
{
    uint8_t     type;                   /**< Always GenericEvent */
    uint8_t     extension;              /**< XI extension offset */
    uint16_t    sequenceNumber;
    uint32_t    length;                 /**< Length in 4 byte units */
    uint16_t    evtype;                 /**< ::XI_PropertyEvent */
    uint16_t    deviceid;
    Time        time;
    Atom        property;
    uint8_t     what;                   /**< ::XIPropertyDeleted,
                                             ::XIPropertyCreated,
                                             ::XIPropertyMotified */
    uint8_t     pad0;
    uint16_t    pad1;
    uint32_t    pad2;
    uint32_t    pad3;
} xXIPropertyEvent;

typedef struct
{
    uint8_t     type;                   /**< Always GenericEvent */
    uint8_t     extension;              /**< XI extension offset */
    uint16_t    sequenceNumber;
    uint32_t    length;                 /**< Length in 4 byte units */
    uint16_t    evtype;                 /**< ::XI_BarrierHit or ::XI_BarrierLeave */
    uint16_t    deviceid;
    Time        time;
    uint32_t    eventid;
    Window      root;
    Window      event;
    Barrier     barrier;
/* └──────── 32 byte boundary ────────┘ */
    uint32_t    dtime;
    uint32_t    flags;                  /**< ::XIBarrierPointerReleased
                                             ::XIBarrierDeviceIsGrabbed */
    uint16_t    sourceid;
    int16_t     pad;
    FP1616      root_x;
    FP1616      root_y;
    FP3232      dx;
    FP3232      dy;
} xXIBarrierEvent;

typedef xXIBarrierEvent xXIBarrierHitEvent;
typedef xXIBarrierEvent xXIBarrierPointerReleasedEvent;
typedef xXIBarrierEvent xXIBarrierLeaveEvent;

#undef Window
#undef Time
#undef Atom
#undef Cursor
#undef Barrier

#endif /* _XI2PROTO_H_ */
