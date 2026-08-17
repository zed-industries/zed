/* SPDX-License-Identifier: MIT */
/*
 * Copyright © 2013 Red Hat, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to
 * deal in the Software without restriction, including without limitation the
 * rights to use, copy, modify, merge, publish, distribute, sublicense, and/or
 * sell copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice (including the next
 * paragraph) shall be included in all copies or substantial portions of the
 * Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
 * FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS
 * IN THE SOFTWARE.
 *
 */

#ifndef LIBEVDEV_H
#define LIBEVDEV_H

#ifdef __cplusplus
extern "C" {
#endif

#include <linux/input.h>
#include <stdarg.h>

#define LIBEVDEV_ATTRIBUTE_PRINTF(_format, _args) __attribute__ ((format (printf, _format, _args)))

/**
 * @mainpage
 *
 * **libevdev** is a library for handling evdev kernel devices. It abstracts
 * the \ref ioctls through type-safe interfaces and provides functions to change
 * the appearance of the device.
 *
 * Development
 * ===========
 * The git repository is available here:
 *
 *   - https://gitlab.freedesktop.org/libevdev/libevdev
 *
 * Development of libevdev is discussed on
 * [input-tools@lists.freedesktop.org](http://lists.freedesktop.org/mailman/listinfo/input-tools).
 * Please submit patches, questions or general comments there.
 *
 * Handling events and SYN_DROPPED
 * ===============================
 *
 * libevdev provides an interface for handling events, including most notably
 * `SYN_DROPPED` events. `SYN_DROPPED` events are sent by the kernel when the
 * process does not read events fast enough and the kernel is forced to drop
 * some events. This causes the device to get out of sync with the process'
 * view of it. libevdev handles this by telling the caller that a * `SYN_DROPPED`
 * has been received and that the state of the device is different to what is
 * to be expected. It then provides the delta between the previous state and
 * the actual state of the device as a set of events. See
 * libevdev_next_event() and @ref syn_dropped for more information on how
 * `SYN_DROPPED` is handled.
 *
 * Signal safety
 * =============
 *
 * libevdev is signal-safe for the majority of its operations, i.e. many of
 * its functions are safe to be called from within a signal handler.
 * Check the API documentation to make sure, unless explicitly stated a call
 * is <b>not</b> signal safe.
 *
 * Device handling
 * ===============
 *
 * A libevdev context is valid for a given file descriptor and its
 * duration. Closing the file descriptor will not destroy the libevdev device
 * but libevdev will not be able to read further events.
 *
 * libevdev does not attempt duplicate detection. Initializing two libevdev
 * devices for the same fd is valid and behaves the same as for two different
 * devices.
 *
 * libevdev does not handle the file descriptors directly, it merely uses
 * them. The caller is responsible for opening the file descriptors, setting
 * them to `O_NONBLOCK` and handling permissions. A caller should drain any
 * events pending on the file descriptor before passing it to libevdev.
 *
 * Where does libevdev sit?
 * ========================
 *
 * libevdev is essentially a `read(2)` on steroids for `/dev/input/eventX`
 * devices. It sits below the process that handles input events, in between
 * the kernel and that process. In the simplest case, e.g. an evtest-like tool
 * the stack would look like this:
 *
 *      kernel → libevdev → evtest
 *
 * For X.Org input modules, the stack would look like this:
 *
 *      kernel → libevdev → xf86-input-evdev → X server → X client
 *
 * For anything using libinput (e.g. most Wayland compositors), the stack
 * the stack would look like this:
 *
 *      kernel → libevdev → libinput → Compositor → Wayland client
 *
 * libevdev does **not** have knowledge of X clients or Wayland clients, it is
 * too low in the stack.
 *
 * Example
 * =======
 * Below is a simple example that shows how libevdev could be used. This example
 * opens a device, checks for relative axes and a left mouse button and if it
 * finds them monitors the device to print the event.
 *
 * @code
 *      struct libevdev *dev = NULL;
 *      int fd;
 *      int rc = 1;
 *
 *      fd = open("/dev/input/event0", O_RDONLY|O_NONBLOCK);
 *      rc = libevdev_new_from_fd(fd, &dev);
 *      if (rc < 0) {
 *              fprintf(stderr, "Failed to init libevdev (%s)\n", strerror(-rc));
 *              exit(1);
 *      }
 *      printf("Input device name: \"%s\"\n", libevdev_get_name(dev));
 *      printf("Input device ID: bus %#x vendor %#x product %#x\n",
 *             libevdev_get_id_bustype(dev),
 *             libevdev_get_id_vendor(dev),
 *             libevdev_get_id_product(dev));
 *      if (!libevdev_has_event_type(dev, EV_REL) ||
 *          !libevdev_has_event_code(dev, EV_KEY, BTN_LEFT)) {
 *              printf("This device does not look like a mouse\n");
 *              exit(1);
 *      }
 *
 *      do {
 *              struct input_event ev;
 *              rc = libevdev_next_event(dev, LIBEVDEV_READ_FLAG_NORMAL, &ev);
 *              if (rc == 0)
 *                      printf("Event: %s %s %d\n",
 *                             libevdev_event_type_get_name(ev.type),
 *                             libevdev_event_code_get_name(ev.type, ev.code),
 *                             ev.value);
 *      } while (rc == 1 || rc == 0 || rc == -EAGAIN);
 * @endcode
 *
 * A more complete example is available with the libevdev-events tool here:
 * https://gitlab.freedesktop.org/libevdev/libevdev/blob/master/tools/libevdev-events.c
 *
 * Backwards compatibility with older kernel
 * =========================================
 * libevdev attempts to build and run correctly on a number of kernel versions.
 * If features required are not available, libevdev attempts to work around them
 * in the most reasonable way. For more details see \ref backwardscompatibility.
 *
 * License information
 * ===================
 * libevdev is licensed under the
 * [X11 license](http://cgit.freedesktop.org/libevdev/tree/COPYING).
 *
 * Bindings
 * ===================
 * - Python: https://gitlab.freedesktop.org/libevdev/python-libevdev
 * - Haskell: http://hackage.haskell.org/package/evdev
 * - Rust: https://crates.io/crates/evdev-rs
 *
 * Reporting bugs
 * ==============
 * Please report bugs in the freedesktop.org GitLab instance:
 * https://gitlab.freedesktop.org/libevdev/libevdev/issues/
 */

/**
 * @page syn_dropped SYN_DROPPED handling
 *
 * This page describes how libevdev handles `SYN_DROPPED` events.
 *
 * Receiving `SYN_DROPPED` events
 * ==============================
 *
 * The kernel sends evdev events separated by an event of type `EV_SYN` and
 * code `SYN_REPORT`. Such an event marks the end of a frame of hardware
 * events. The number of events between `SYN_REPORT` events is arbitrary and
 * depends on the hardware. An example event sequence may look like this:
 * @code
 *  EV_ABS   ABS_X        9
 *  EV_ABS   ABS_Y        8
 *  EV_SYN   SYN_REPORT   0
 *  ------------------------
 *  EV_ABS   ABS_X        10
 *  EV_ABS   ABS_Y        10
 *  EV_KEY   BTN_TOUCH    1
 *  EV_SYN   SYN_REPORT   0
 *  ------------------------
 *  EV_ABS   ABS_X        11
 *  EV_SYN   SYN_REPORT   0
 * @endcode
 *
 * Events are handed to the client buffer as they appear, the kernel adjusts
 * the buffer size to handle at least one full event. In the normal case,
 * the client reads the event and the kernel can place the next event in the
 * buffer. If the client is not fast enough, the kernel places an event of
 * type `EV_SYN` and code `SYN_DROPPED` into the buffer, effectively notifying
 * the client that some events were lost. The above example event sequence
 * may look like this (note the missing/repeated events):
 * @code
 *  EV_ABS   ABS_X        9
 *  EV_ABS   ABS_Y        8
 *  EV_SYN   SYN_REPORT   0
 *  ------------------------
 *  EV_ABS   ABS_X        10
 *  EV_ABS   ABS_Y        10
 *  EV_SYN   SYN_DROPPED  0
 *  EV_ABS   ABS_Y        15
 *  EV_SYN   SYN_REPORT   0
 *  ------------------------
 *  EV_ABS   ABS_X        11
 *  EV_KEY   BTN_TOUCH    0
 *  EV_SYN   SYN_REPORT   0
 * @endcode
 *
 * A `SYN_DROPPED` event may be recieved at any time in the event sequence.
 * When a `SYN_DROPPED` event is received, the client must:
 * * discard all events since the last `SYN_REPORT`
 * * discard all events until including the next `SYN_REPORT`
 * These event are part of incomplete event frames.
 *
 * Synchronizing the state of the device
 * =====================================
 *
 * The handling of the device after a `SYN_DROPPED` depends on the available
 * event codes. For all event codes of type `EV_REL`, no handling is
 * necessary, there is no state attached. For all event codes of type
 * `EV_KEY`, `EV_SW`, `EV_LED` and `EV_SND`, the matching @ref ioctls retrieve the
 * current state. The caller must then compare the last-known state to the
 * retrieved state and handle the deltas accordingly.
 * libevdev simplifies this approach: if the state of the device has
 * changed, libevdev generates an event for each code with the new value and
 * passes it to the caller during libevdev_next_event() if
 * @ref LIBEVDEV_READ_FLAG_SYNC is set.
 *
 * For events of type `EV_ABS` and an event code less than `ABS_MT_SLOT`, the
 * handling of state changes is as described above. For events between
 * `ABS_MT_SLOT` and `ABS_MAX`, the event handling differs.
 * Slots are the vehicles to transport information for multiple simultaneous
 * touchpoints on a device. Slots are re-used once a touchpoint has ended.
 * The kernel sends an `ABS_MT_SLOT` event whenever the current slot
 * changes; any event in the above axis range applies only to the currently
 * active slot.
 * Thus, an event sequence from a slot-capable device may look like this:
 * @code
 *  EV_ABS   ABS_MT_POSITION_Y   10
 *  EV_ABS   ABS_MT_SLOT         1
 *  EV_ABS   ABS_MT_POSITION_X   100
 *  EV_ABS   ABS_MT_POSITION_Y   80
 *  EV_SYN   SYN_REPORT          0
 * @endcode
 * Note the lack of `ABS_MT_SLOT`: the first `ABS_MT_POSITION_Y` applies to
 * a slot opened previously, and is the only axis that changed for that
 * slot. The touchpoint in slot 1 now has position `100/80`.
 * The kernel does not provide events if a value does not change, and does
 * not send `ABS_MT_SLOT` events if the slot does not change, or none of the
 * values within a slot changes. A client must thus keep the state for each
 * slot.
 *
 * If a `SYN_DROPPED` is received,  the client must sync all slots
 * individually and update its internal state. libevdev simplifies this by
 * generating multiple events:
 * * for each slot on the device, libevdev generates an
 *   `ABS_MT_SLOT` event with the value set to the slot number
 * * for each event code between `ABS_MT_SLOT + 1` and `ABS_MAX` that changed
 *   state for this slot, libevdev generates an event for the new state
 * * libevdev sends a final `ABS_MT_SLOT` event for the current slot as
 *   seen by the kernel
 * * libevdev terminates this sequence with an `EV_SYN SYN_REPORT` event
 *
 * An example event sequence for such a sync may look like this:
 * @code
 *  EV_ABS   ABS_MT_SLOT         0
 *  EV_ABS   ABS_MT_POSITION_Y   10
 *  EV_ABS   ABS_MT_SLOT         1
 *  EV_ABS   ABS_MT_POSITION_X   100
 *  EV_ABS   ABS_MT_POSITION_Y   80
 *  EV_ABS   ABS_MT_SLOT         2
 *  EV_ABS   ABS_MT_POSITION_Y   8
 *  EV_ABS   ABS_MT_PRESSURE     12
 *  EV_ABS   ABS_MT_SLOT         1
 *  EV_SYN   SYN_REPORT          0
 * @endcode
 * Note the terminating `ABS_MT_SLOT` event, this indicates that the kernel
 * currently has slot 1 active.
 *
 * Synchronizing ABS_MT_TRACKING_ID
 * ================================
 *
 * The event code `ABS_MT_TRACKING_ID` is used to denote the start and end of
 * a touch point within a slot. An `ABS_MT_TRACKING_ID` of zero or greater
 * denotes the start of a touchpoint, an `ABS_MT_TRACKING_ID` of -1 denotes
 * the end of a touchpoint within this slot. During `SYN_DROPPED`, a touch
 * point may have ended and re-started within a slot - a client must check
 * the `ABS_MT_TRACKING_ID`. libevdev simplifies this by emulating extra
 * events if the `ABS_MT_TRACKING_ID` has changed:
 * * if the `ABS_MT_TRACKING_ID` was valid and is -1, libevdev enqueues an
 *   `ABS_MT_TRACKING_ID` event with value -1.
 * * if the `ABS_MT_TRACKING_ID` was -1 and is now a valid ID, libevdev
 *   enqueues an `ABS_MT_TRACKING_ID` event with the current value.
 * * if the `ABS_MT_TRACKING_ID` was a valid ID and is now a different valid
 *   ID, libevev enqueues an `ABS_MT_TRACKING_ID` event with value -1 and
 *   another `ABS_MT_TRACKING_ID` event with the new value.
 *
 * An example event sequence for such a sync may look like this:
 * @code
 *  EV_ABS   ABS_MT_SLOT         0
 *  EV_ABS   ABS_MT_TRACKING_ID  -1
 *  EV_ABS   ABS_MT_SLOT         2
 *  EV_ABS   ABS_MT_TRACKING_ID  -1
 *  EV_SYN   SYN_REPORT          0
 *  ------------------------
 *  EV_ABS   ABS_MT_SLOT         1
 *  EV_ABS   ABS_MT_POSITION_X   100
 *  EV_ABS   ABS_MT_POSITION_Y   80
 *  EV_ABS   ABS_MT_SLOT         2
 *  EV_ABS   ABS_MT_TRACKING_ID  45
 *  EV_ABS   ABS_MT_POSITION_Y   8
 *  EV_ABS   ABS_MT_PRESSURE     12
 *  EV_ABS   ABS_MT_SLOT         1
 *  EV_SYN   SYN_REPORT          0
 * @endcode
 * Note how the touchpoint in slot 0 was terminated, the touchpoint in slot
 * 2 was terminated and then started with a new `ABS_MT_TRACKING_ID`. The touchpoint
 * in slot 1 maintained the same `ABS_MT_TRACKING_ID` and only updated the
 * coordinates. Slot 1 is the currently active slot.
 *
 * In the case of a `SYN_DROPPED` event, a touch point may be invisible to a
 * client if it started after `SYN_DROPPED` and finished before the client
 * handles events again. The below example shows an example event sequence
 * and what libevdev sees in the case of a `SYN_DROPPED` event:
 * @code
 *
 *            kernel                  |              userspace
 *                                    |
 *  EV_ABS   ABS_MT_SLOT         0    |    EV_ABS   ABS_MT_SLOT         0
 *  EV_ABS   ABS_MT_TRACKING_ID  -1   |    EV_ABS   ABS_MT_TRACKING_ID  -1
 *  EV_SYN   SYN_REPORT          0    |    EV_SYN   SYN_REPORT          0
 *  ------------------------          |    ------------------------
 *  EV_ABS   ABS_MT_TRACKING_ID  30   |
 *  EV_ABS   ABS_MT_POSITION_X   100  |
 *  EV_ABS   ABS_MT_POSITION_Y   80   |
 *  EV_SYN   SYN_REPORT          0    |           SYN_DROPPED
 *  ------------------------          |
 *  EV_ABS   ABS_MT_TRACKING_ID  -1   |
 *  EV_SYN   SYN_REPORT          0    |
 *  ------------------------          |    ------------------------
 *  EV_ABS   ABS_MT_SLOT         1    |    EV_ABS   ABS_MT_SLOT         1
 *  EV_ABS   ABS_MT_POSITION_X   90   |    EV_ABS   ABS_MT_POSITION_X   90
 *  EV_ABS   ABS_MT_POSITION_Y   10   |    EV_ABS   ABS_MT_POSITION_Y   10
 *  EV_SYN   SYN_REPORT          0    |    EV_SYN   SYN_REPORT          0
 * @endcode
 * If such an event sequence occurs, libevdev will send all updated axes
 * during the sync process. Axis events may thus be generated for devices
 * without a currently valid `ABS_MT_TRACKING_ID`. Specifically for the above
 * example, the client would receive the following event sequence:
 * @code
 *  EV_ABS   ABS_MT_SLOT         0       ← LIBEVDEV_READ_FLAG_NORMAL
 *  EV_ABS   ABS_MT_TRACKING_ID  -1
 *  EV_SYN   SYN_REPORT          0
 *  ------------------------
 *  EV_SYN   SYN_DROPPED         0       → LIBEVDEV_READ_STATUS_SYNC
 *  ------------------------
 *  EV_ABS   ABS_MT_POSITION_X   100     ← LIBEVDEV_READ_FLAG_SYNC
 *  EV_ABS   ABS_MT_POSITION_Y   80
 *  EV_SYN   SYN_REPORT          0
 *  -----------------------------        → -EGAIN
 *  EV_ABS   ABS_MT_SLOT         1       ← LIBEVDEV_READ_FLAG_NORMAL
 *  EV_ABS   ABS_MT_POSITION_X   90
 *  EV_ABS   ABS_MT_POSITION_Y   10
 *  EV_SYN   SYN_REPORT          0
 *  -------------------
 * @endcode
 * The axis events do not reflect the position of a current touch point, a
 * client must take care not to generate a new touch point based on those
 * updates.
 *
 * Discarding events before synchronizing
 * =====================================
 *
 * The kernel implements the client buffer as a ring buffer. `SYN_DROPPED`
 * events are handled when the buffer is full and a new event is received
 * from a device. All existing events are discarded, a `SYN_DROPPED` is added
 * to the buffer followed by the actual device event. Further events will be
 * appended to the buffer until it is either read by the client, or filled
 * again, at which point the sequence repeats.
 *
 * When the client reads the buffer, the buffer will thus always consist of
 * exactly one `SYN_DROPPED` event followed by an unspecified number of real
 * events. The data the ioctls return is the current state of the device,
 * i.e. the state after all these events have been processed. For example,
 * assume the buffer contains the following sequence:
 *
 * @code
 *  EV_SYN   SYN_DROPPED
 *  EV_ABS   ABS_X               1
 *  EV_SYN   SYN_REPORT          0
 *  EV_ABS   ABS_X               2
 *  EV_SYN   SYN_REPORT          0
 *  EV_ABS   ABS_X               3
 *  EV_SYN   SYN_REPORT          0
 *  EV_ABS   ABS_X               4
 *  EV_SYN   SYN_REPORT          0
 *  EV_ABS   ABS_X               5
 *  EV_SYN   SYN_REPORT          0
 *  EV_ABS   ABS_X               6
 *  EV_SYN   SYN_REPORT          0
 * @endcode
 * An ioctl at any time in this sequence will return a value of 6 for ABS_X.
 *
 * libevdev discards all events after a `SYN_DROPPED` to ensure the events
 * during @ref LIBEVDEV_READ_FLAG_SYNC represent the last known state of the
 * device. This loses some granularity of the events especially as the time
 * between the `SYN_DROPPED` and the sync process increases. It does however
 * avoid spurious cursor movements. In the above example, the event sequence
 * by libevdev is:
 * @code
 *  EV_SYN   SYN_DROPPED
 *  EV_ABS   ABS_X               6
 *  EV_SYN   SYN_REPORT          0
 *  @endcode
 */

/**
 * @page backwardscompatibility Compatibility and Behavior across kernel versions
 *
 * This page describes libevdev's behavior when the build-time kernel and the
 * run-time kernel differ in their feature set.
 *
 * With the exception of event names, libevdev defines features that may be
 * missing on older kernels and building on such kernels will not disable
 * features. Running libevdev on a kernel that is missing some feature will
 * change libevdev's behavior. In most cases, the new behavior should be
 * obvious, but it is spelled out below in detail.
 *
 * Minimum requirements
 * ====================
 * libevdev requires a 2.6.36 kernel as minimum. Specifically, it requires
 * kernel-support for `ABS_MT_SLOT`.
 *
 * Event and input property names
 * ==============================
 * Event names and input property names are defined at build-time by the
 * linux/input.h shipped with libevdev.
 * The list of event names is compiled at build-time, any events not defined
 * at build time will not resolve. Specifically,
 * libevdev_event_code_get_name() for an undefined type or code will
 * always return `NULL`. Likewise, libevdev_property_get_name() will return NULL
 * for properties undefined at build-time.
 *
 * Input properties
 * ================
 * If the kernel does not support input properties, specifically the
 * `EVIOCGPROPS` ioctl, libevdev does not expose input properties to the caller.
 * Specifically, libevdev_has_property() will always return 0 unless the
 * property has been manually set with libevdev_enable_property().
 *
 * This also applies to the libevdev-uinput code. If uinput does not honor
 * `UI_SET_PROPBIT`, libevdev will continue without setting the properties on
 * the device.
 *
 * MT slot behavior
 * =================
 * If the kernel does not support the `EVIOCGMTSLOTS` ioctl, libevdev
 * assumes all values in all slots are 0 and continues without an error.
 *
 * SYN_DROPPED behavior
 * ====================
 * A kernel without `SYN_DROPPED` won't send such an event. libevdev_next_event()
 * will never require the switch to sync mode.
 */

/**
 * @page ioctls evdev ioctls
 *
 * This page lists the status of the evdev-specific ioctls in libevdev.
 *
 * <dl>
 * <dt>EVIOCGVERSION:</dt>
 * <dd>supported, see libevdev_get_driver_version()</dd>
 * <dt>EVIOCGID:</dt>
 * <dd>supported, see libevdev_get_id_product(), libevdev_get_id_vendor(),
 * libevdev_get_id_bustype(), libevdev_get_id_version()</dd>
 * <dt>EVIOCGREP:</dt>
 * <dd>supported, see libevdev_get_event_value())</dd>
 * <dt>EVIOCSREP:</dt>
 * <dd>supported, see libevdev_enable_event_code()</dd>
 * <dt>EVIOCGKEYCODE:</dt>
 * <dd>currently not supported</dd>
 * <dt>EVIOCGKEYCODE:</dt>
 * <dd>currently not supported</dd>
 * <dt>EVIOCSKEYCODE:</dt>
 * <dd>currently not supported</dd>
 * <dt>EVIOCSKEYCODE:</dt>
 * <dd>currently not supported</dd>
 * <dt>EVIOCGNAME:</dt>
 * <dd>supported, see libevdev_get_name()</dd>
 * <dt>EVIOCGPHYS:</dt>
 * <dd>supported, see libevdev_get_phys()</dd>
 * <dt>EVIOCGUNIQ:</dt>
 * <dd>supported, see libevdev_get_uniq()</dd>
 * <dt>EVIOCGPROP:</dt>
 * <dd>supported, see libevdev_has_property()</dd>
 * <dt>EVIOCGMTSLOTS:</dt>
 * <dd>supported, see libevdev_get_num_slots(), libevdev_get_slot_value()</dd>
 * <dt>EVIOCGKEY:</dt>
 * <dd>supported, see libevdev_has_event_code(), libevdev_get_event_value()</dd>
 * <dt>EVIOCGLED:</dt>
 * <dd>supported, see libevdev_has_event_code(), libevdev_get_event_value()</dd>
 * <dt>EVIOCGSND:</dt>
 * <dd>currently not supported</dd>
 * <dt>EVIOCGSW:</dt>
 * <dd>supported, see libevdev_has_event_code(), libevdev_get_event_value()</dd>
 * <dt>EVIOCGBIT:</dt>
 * <dd>supported, see libevdev_has_event_code(), libevdev_get_event_value()</dd>
 * <dt>EVIOCGABS:</dt>
 * <dd>supported, see libevdev_has_event_code(), libevdev_get_event_value(),
 * libevdev_get_abs_info()</dd>
 * <dt>EVIOCSABS:</dt>
 * <dd>supported, see libevdev_kernel_set_abs_info()</dd>
 * <dt>EVIOCSFF:</dt>
 * <dd>currently not supported</dd>
 * <dt>EVIOCRMFF:</dt>
 * <dd>currently not supported</dd>
 * <dt>EVIOCGEFFECTS:</dt>
 * <dd>currently not supported</dd>
 * <dt>EVIOCGRAB:</dt>
 * <dd>supported, see libevdev_grab()</dd>
 * <dt>EVIOCSCLOCKID:</dt>
 * <dd>supported, see libevdev_set_clock_id()</dd>
 * <dt>EVIOCREVOKE:</dt>
 * <dd>currently not supported, see
 * http://lists.freedesktop.org/archives/input-tools/2014-January/000688.html</dd>
 * </dl>
 *
 */

/**
 * @page kernel_header Kernel header
 *
 * libevdev provides its own copy of the Linux kernel header file and
 * compiles against the definitions define here. Event type and event code
 * names, etc. are taken from the file below:
 * @include linux/input.h
 */

/**
 * @page static_linking Statically linking libevdev
 *
 * Statically linking libevdev.a is not recommended. Symbol visibility is
 * difficult to control in a static library, so extra care must be taken to
 * only use symbols that are explicitly exported. libevdev's API stability
 * guarantee only applies to those symbols.
 *
 * If you do link libevdev statically, note that in addition to the exported
 * symbols, libevdev reserves the <b>_libevdev_*</b> namespace. Do not use
 * or create symbols with that prefix, they are subject to change at any
 * time.
 */

/**
 * @page testing libevdev-internal test suite
 *
 * libevdev's internal test suite uses the
 * [Check unit testing framework](http://check.sourceforge.net/). Tests are
 * divided into test suites and test cases. Most tests create a uinput device,
 * so you'll need to run as root, and your kernel must have
 * `CONFIG_INPUT_UINPUT` enabled.
 *
 * To run a specific suite only:
 *
 *     export CK_RUN_SUITE="suite name"
 *
 * To run a specific test case only:
 *
 *     export CK_RUN_TEST="test case name"
 *
 * To get a list of all suites or tests:
 *
 *     git grep "suite_create"
 *     git grep "tcase_create"
 *
 * By default, Check forks, making debugging harder. The test suite tries to detect
 * if it is running inside gdb and disable forking. If that doesn't work for
 * some reason, run gdb as below to avoid forking.
 *
 *     sudo CK_FORK=no CK_RUN_TEST="test case name" gdb ./test/test-libevdev
 *
 * A special target `make gcov-report.txt` exists that runs gcov and leaves a
 * `libevdev.c.gcov` file. Check that for test coverage.
 *
 * `make check` is hooked up to run the test and gcov (again, needs root).
 *
 * The test suite creates a lot of devices, very quickly. Add the following
 * xorg.conf.d snippet to avoid the devices being added as X devices (at the
 * time of writing, mutter can't handle these devices and exits after getting
 * a BadDevice error).
 *
 *     $ cat /etc/X11/xorg.conf.d/99-ignore-libevdev-devices.conf
 *     Section "InputClass"
 *             Identifier "Ignore libevdev test devices"
 *             MatchProduct "libevdev test device"
 *             Option "Ignore" "on"
 *     EndSection
 *
 */

/**
 * @defgroup init Initialization and setup
 *
 * Initialization, initial setup and file descriptor handling.
 * These functions are the main entry points for users of libevdev, usually a
 * caller will use this series of calls:
 *
 * @code
 * struct libevdev *dev;
 * int err;
 *
 * dev = libevdev_new();
 * if (!dev)
 *         return ENOMEM;
 *
 * err = libevdev_set_fd(dev, fd);
 * if (err < 0) {
 *         printf("Failed (errno %d): %s\n", -err, strerror(-err));
 *
 * libevdev_free(dev);
 * @endcode
 *
 * libevdev_set_fd() is the central call and initializes the internal structs
 * for the device at the given fd. libevdev functions will fail if called
 * before libevdev_set_fd() unless documented otherwise.
 */

/**
 * @defgroup logging Library logging facilities
 *
 * libevdev provides two methods of logging library-internal messages. The
 * old method is to provide a global log handler in
 * libevdev_set_log_function(). The new method is to provide a per-context
 * log handler in libevdev_set_device_log_function(). Developers are encouraged
 * to use the per-context logging facilities over the global log handler as
 * it provides access to the libevdev instance that caused a message, and is
 * more flexible when libevdev is used from within a shared library.
 *
 * If a caller sets both the global log handler and a per-context log
 * handler, each device with a per-context log handler will only invoke that
 * log handler.
 *
 * @note To set a context-specific log handler, a context is needed.
 * Thus developers are discouraged from using libevdev_new_from_fd() as
 * important messages from the device initialization process may get lost.
 *
 * @note A context-specific handler cannot be used for libevdev's uinput
 * devices. @ref uinput must use the global log handler.
 */

/**
 * @defgroup bits Querying device capabilities
 *
 * Abstraction functions to handle device capabilities, specifically
 * device properties such as the name of the device and the bits
 * representing the events supported by this device.
 *
 * The logical state returned may lag behind the physical state of the device.
 * libevdev queries the device state on libevdev_set_fd() and then relies on
 * the caller to parse events through libevdev_next_event(). If a caller does not
 * use libevdev_next_event(), libevdev will not update the internal state of the
 * device and thus returns outdated values.
 */

/**
 * @defgroup mt Multi-touch related functions
 * Functions for querying multi-touch-related capabilities. MT devices
 * following the kernel protocol B (using `ABS_MT_SLOT`) provide multiple touch
 * points through so-called slots on the same axis. The slots are enumerated,
 * a client reading from the device will first get an ABS_MT_SLOT event, then
 * the values of axes changed in this slot. Multiple slots may be provided in
 * before an `EV_SYN` event.
 *
 * As with @ref bits, the logical state of the device as seen by the library
 * depends on the caller using libevdev_next_event().
 *
 * The Linux kernel requires all axes on a device to have a semantic
 * meaning, matching the axis names in linux/input.h. Some devices merely
 * export a number of axes beyond the available axis list. For those
 * devices, the multitouch information is invalid. Specifically, if a device
 * provides the `ABS_MT_SLOT` axis AND also the `ABS_RESERVED` axis, the
 * device is not treated as multitouch device. No slot information is
 * available and the `ABS_MT` axis range for these devices is treated as all
 * other `EV_ABS` axes.
 *
 * Note that because of limitations in the kernel API, such fake multitouch
 * devices can not be reliably synced after a `SYN_DROPPED` event. libevdev
 * ignores all `ABS_MT` axis values during the sync process and instead
 * relies on the device to send the current axis value with the first event
 * after `SYN_DROPPED`.
 */

/**
 * @defgroup kernel Modifying the appearance or capabilities of the device
 *
 * Modifying the set of events reported by this device. By default, the
 * libevdev device mirrors the kernel device, enabling only those bits
 * exported by the kernel. This set of functions enable or disable bits as
 * seen from the caller.
 *
 * Enabling an event type or code does not affect event reporting - a
 * software-enabled event will not be generated by the physical hardware.
 * Disabling an event will prevent libevdev from routing such events to the
 * caller. Enabling and disabling event types and codes is at the library
 * level and thus only affects the caller.
 *
 * If an event type or code is enabled at kernel-level, future users of this
 * device will see this event enabled. Currently there is no option of
 * disabling an event type or code at kernel-level.
 */

/**
 * @defgroup misc Miscellaneous helper functions
 *
 * Functions for printing or querying event ranges. The list of names is
 * compiled into libevdev and is independent of the run-time kernel.
 * Likewise, the max for each event type is compiled in and does not check
 * the kernel at run-time.
 */

/**
 * @defgroup events Event handling
 *
 * Functions to handle events and fetch the current state of the event.
 * libevdev updates its internal state as the event is processed and forwarded
 * to the caller. Thus, the libevdev state of the device should always be identical
 * to the caller's state. It may however lag behind the actual state of the device.
 */

/**
 * @ingroup init
 *
 * Opaque struct representing an evdev device.
 */
struct libevdev;

/**
 * @ingroup events
 */
enum libevdev_read_flag {
	LIBEVDEV_READ_FLAG_SYNC		= 1, /**< Process data in sync mode */
	LIBEVDEV_READ_FLAG_NORMAL	= 2, /**< Process data in normal mode */
	LIBEVDEV_READ_FLAG_FORCE_SYNC	= 4, /**< Pretend the next event is a SYN_DROPPED and
					          require the caller to sync */
	LIBEVDEV_READ_FLAG_BLOCKING	= 8  /**< The fd is not in O_NONBLOCK and a read may block */
};

/**
 * @ingroup init
 *
 * Initialize a new libevdev device. This function only allocates the
 * required memory and initializes the struct to sane default values.
 * To actually hook up the device to a kernel device, use
 * libevdev_set_fd().
 *
 * Memory allocated through libevdev_new() must be released by the
 * caller with libevdev_free().
 *
 * @see libevdev_set_fd
 * @see libevdev_free
 */
struct libevdev* libevdev_new(void);

/**
 * @ingroup init
 *
 * Initialize a new libevdev device from the given fd.
 *
 * This is a shortcut for
 *
 * @code
 * int err;
 * struct libevdev *dev = libevdev_new();
 * err = libevdev_set_fd(dev, fd);
 * @endcode
 *
 * @param fd A file descriptor to the device in O_RDWR or O_RDONLY mode.
 * @param[out] dev The newly initialized evdev device.
 *
 * @return On success, 0 is returned and dev is set to the newly
 * allocated struct. On failure, a negative errno is returned and the value
 * of dev is undefined.
 *
 * @see libevdev_free
 */
int libevdev_new_from_fd(int fd, struct libevdev **dev);

/**
 * @ingroup init
 *
 * Clean up and free the libevdev struct. After completion, the <code>struct
 * libevdev</code> is invalid and must not be used.
 *
 * Note that calling libevdev_free() does not close the file descriptor
 * currently associated with this instance.
 *
 * @param dev The evdev device
 *
 * @note This function may be called before libevdev_set_fd().
 */
void libevdev_free(struct libevdev *dev);

/**
 * @ingroup logging
 */
enum libevdev_log_priority {
	LIBEVDEV_LOG_ERROR = 10,	/**< critical errors and application bugs */
	LIBEVDEV_LOG_INFO  = 20,	/**< informational messages */
	LIBEVDEV_LOG_DEBUG = 30		/**< debug information */
};

/**
 * @ingroup logging
 *
 * Logging function called by library-internal logging.
 * This function is expected to treat its input like printf would.
 *
 * @param priority Log priority of this message
 * @param data User-supplied data pointer (see libevdev_set_log_function())
 * @param file libevdev source code file generating this message
 * @param line libevdev source code line generating this message
 * @param func libevdev source code function generating this message
 * @param format printf-style format string
 * @param args List of arguments
 *
 * @see libevdev_set_log_function
 */
typedef void (*libevdev_log_func_t)(enum libevdev_log_priority priority,
				    void *data,
				    const char *file, int line,
				    const char *func,
				    const char *format, va_list args)
	LIBEVDEV_ATTRIBUTE_PRINTF(6, 0);

/**
 * @ingroup logging
 *
 * Set a printf-style logging handler for library-internal logging. The default
 * logging function is to stdout.
 *
 * @note The global log handler is only called if no context-specific log
 * handler has been set with libevdev_set_device_log_function().
 *
 * @param logfunc The logging function for this device. If NULL, the current
 * logging function is unset and no logging is performed.
 * @param data User-specific data passed to the log handler.
 *
 * @note This function may be called before libevdev_set_fd().
 *
 * @deprecated Use per-context logging instead, see
 * libevdev_set_device_log_function().
 */
void libevdev_set_log_function(libevdev_log_func_t logfunc, void *data);

/**
 * @ingroup logging
 *
 * Define the minimum level to be printed to the log handler.
 * Messages higher than this level are printed, others are discarded. This
 * is a global setting and applies to any future logging messages.
 *
 * @param priority Minimum priority to be printed to the log.
 *
 * @deprecated Use per-context logging instead, see
 * libevdev_set_device_log_function().
 */
void libevdev_set_log_priority(enum libevdev_log_priority priority);

/**
 * @ingroup logging
 *
 * Return the current log priority level. Messages higher than this level
 * are printed, others are discarded. This is a global setting.
 *
 * @return the current log level
 *
 * @deprecated Use per-context logging instead, see
 * libevdev_set_device_log_function().
 */
enum libevdev_log_priority libevdev_get_log_priority(void);

/**
 * @ingroup logging
 *
 * Logging function called by library-internal logging for a specific
 * libevdev context. This function is expected to treat its input like
 * printf would.
 *
 * @param dev The evdev device
 * @param priority Log priority of this message
 * @param data User-supplied data pointer (see libevdev_set_log_function())
 * @param file libevdev source code file generating this message
 * @param line libevdev source code line generating this message
 * @param func libevdev source code function generating this message
 * @param format printf-style format string
 * @param args List of arguments
 *
 * @see libevdev_set_log_function
 * @since 1.3
 */
typedef void (*libevdev_device_log_func_t)(const struct libevdev *dev,
					   enum libevdev_log_priority priority,
					   void *data,
					   const char *file, int line,
					   const char *func,
					   const char *format, va_list args)
	LIBEVDEV_ATTRIBUTE_PRINTF(7, 0);

/**
 * @ingroup logging
 *
 * Set a printf-style logging handler for library-internal logging for this
 * device context. The default logging function is NULL, i.e. the global log
 * handler is invoked. If a context-specific log handler is set, the global
 * log handler is not invoked for this device.
 *
 * @note This log function applies for this device context only, even if
 * another context exists for the same fd.
 *
 * @param dev The evdev device
 * @param logfunc The logging function for this device. If NULL, the current
 * logging function is unset and logging falls back to the global log
 * handler, if any.
 * @param priority Minimum priority to be printed to the log.
 * @param data User-specific data passed to the log handler.
 *
 * @note This function may be called before libevdev_set_fd().
 * @since 1.3
 */
void libevdev_set_device_log_function(struct libevdev *dev,
				      libevdev_device_log_func_t logfunc,
				      enum libevdev_log_priority priority,
				      void *data);

/**
 * @ingroup init
 */
enum libevdev_grab_mode {
	LIBEVDEV_GRAB = 3,	/**< Grab the device if not currently grabbed */
	LIBEVDEV_UNGRAB = 4	/**< Ungrab the device if currently grabbed */
};

/**
 * @ingroup init
 *
 * Grab or ungrab the device through a kernel EVIOCGRAB. This prevents other
 * clients (including kernel-internal ones such as rfkill) from receiving
 * events from this device.
 *
 * This is generally a bad idea. Don't do this.
 *
 * Grabbing an already grabbed device, or ungrabbing an ungrabbed device is
 * a noop and always succeeds.
 *
 * A grab is an operation tied to a file descriptor, not a device. If a
 * client changes the file descriptor with libevdev_change_fd(), it must
 * also re-issue a grab with libevdev_grab().
 *
 * @param dev The evdev device, already initialized with libevdev_set_fd()
 * @param grab If true, grab the device. Otherwise ungrab the device.
 *
 * @return 0 if the device was successfully grabbed or ungrabbed, or a
 * negative errno in case of failure.
 */
int libevdev_grab(struct libevdev *dev, enum libevdev_grab_mode grab);

/**
 * @ingroup init
 *
 * Set the fd for this struct and initialize internal data.
 * The fd must be in O_RDONLY or O_RDWR mode.
 *
 * This function may only be called once per device. If the device changed and
 * you need to re-read a device, use libevdev_free() and libevdev_new(). If
 * you need to change the fd after closing and re-opening the same device, use
 * libevdev_change_fd().
 *
 * A caller should ensure that any events currently pending on the fd are
 * drained before the file descriptor is passed to libevdev for
 * initialization. Due to how the kernel's ioctl handling works, the initial
 * device state will reflect the current device state *after* applying all
 * events currently pending on the fd. Thus, if the fd is not drained, the
 * state visible to the caller will be inconsistent with the events
 * immediately available on the device. This does not affect state-less
 * events like EV_REL.
 *
 * Unless otherwise specified, libevdev function behavior is undefined until
 * a successful call to libevdev_set_fd().
 *
 * @param dev The evdev device
 * @param fd The file descriptor for the device
 *
 * @return 0 on success, or a negative errno on failure
 *
 * @see libevdev_change_fd
 * @see libevdev_new
 * @see libevdev_free
 */
int libevdev_set_fd(struct libevdev* dev, int fd);

/**
 * @ingroup init
 *
 * Change the fd for this device, without re-reading the actual device. If the fd
 * changes after initializing the device, for example after a VT-switch in the
 * X.org X server, this function updates the internal fd to the newly opened.
 * No check is made that new fd points to the same device. If the device has
 * changed, libevdev's behavior is undefined.
 *
 * libevdev does not sync itself after changing the fd and keeps the current
 * device state. Use libevdev_next_event with the
 * @ref LIBEVDEV_READ_FLAG_FORCE_SYNC flag to force a re-sync.
 *
 * The example code below illustrates how to force a re-sync of the
 * library-internal state. Note that this code doesn't handle the events in
 * the caller, it merely forces an update of the internal library state.
 * @code
 *     struct input_event ev;
 *     libevdev_change_fd(dev, new_fd);
 *     libevdev_next_event(dev, LIBEVDEV_READ_FLAG_FORCE_SYNC, &ev);
 *     while (libevdev_next_event(dev, LIBEVDEV_READ_FLAG_SYNC, &ev) == LIBEVDEV_READ_STATUS_SYNC)
 *                             ; // noop
 * @endcode
 *
 * The fd may be open in O_RDONLY or O_RDWR.
 *
 * After changing the fd, the device is assumed ungrabbed and a caller must
 * call libevdev_grab() again.
 *
 * It is an error to call this function before calling libevdev_set_fd().
 *
 * @param dev The evdev device, already initialized with libevdev_set_fd()
 * @param fd The new fd
 *
 * @return 0 on success, or -1 on failure.
 *
 * @see libevdev_set_fd
 */
int libevdev_change_fd(struct libevdev* dev, int fd);

/**
 * @ingroup init
 *
 * @param dev The evdev device
 *
 * @return The previously set fd, or -1 if none had been set previously.
 * @note This function may be called before libevdev_set_fd().
 */
int libevdev_get_fd(const struct libevdev* dev);

/**
 * @ingroup events
 */
enum libevdev_read_status {
	/**
	 * libevdev_next_event() has finished without an error
	 * and an event is available for processing.
	 *
	 * @see libevdev_next_event
	 */
	LIBEVDEV_READ_STATUS_SUCCESS = 0,
	/**
	 * Depending on the libevdev_next_event() read flag:
	 * * libevdev received a SYN_DROPPED from the device, and the caller should
	 * now resync the device, or,
	 * * an event has been read in sync mode.
	 *
	 * @see libevdev_next_event
	 */
	LIBEVDEV_READ_STATUS_SYNC = 1
};

/**
 * @ingroup events
 *
 * Get the next event from the device. This function operates in two different
 * modes: normal mode or sync mode.
 *
 * In normal mode (when flags has @ref LIBEVDEV_READ_FLAG_NORMAL set), this
 * function returns @ref LIBEVDEV_READ_STATUS_SUCCESS and returns the event
 * in the argument @p ev. If no events are available at this
 * time, it returns -EAGAIN and ev is undefined.
 *
 * If the current event is an EV_SYN SYN_DROPPED event, this function returns
 * @ref LIBEVDEV_READ_STATUS_SYNC and ev is set to the EV_SYN event.
 * The caller should now call this function with the
 * @ref LIBEVDEV_READ_FLAG_SYNC flag set, to get the set of events that make up the
 * device state delta. This function returns @ref LIBEVDEV_READ_STATUS_SYNC for
 * each event part of that delta, until it returns -EAGAIN once all events
 * have been synced. For more details on what libevdev does to sync after a
 * SYN_DROPPED event, see @ref syn_dropped.
 *
 * If a device needs to be synced by the caller but the caller does not call
 * with the @ref LIBEVDEV_READ_FLAG_SYNC flag set, all events from the diff are
 * dropped after libevdev updates its internal state and event processing
 * continues as normal. Note that the current slot and the state of touch
 * points may have updated during the SYN_DROPPED event, it is strongly
 * recommended that a caller ignoring all sync events calls
 * libevdev_get_current_slot() and checks the ABS_MT_TRACKING_ID values for
 * all slots.
 *
 * If a device has changed state without events being enqueued in libevdev,
 * e.g. after changing the file descriptor, use the @ref
 * LIBEVDEV_READ_FLAG_FORCE_SYNC flag. This triggers an internal sync of the
 * device and libevdev_next_event() returns @ref LIBEVDEV_READ_STATUS_SYNC.
 * Any state changes are available as events as described above. If
 * @ref LIBEVDEV_READ_FLAG_FORCE_SYNC is set, the value of ev is undefined.
 *
 * @param dev The evdev device, already initialized with libevdev_set_fd()
 * @param flags Set of flags to determine behaviour. If @ref LIBEVDEV_READ_FLAG_NORMAL
 * is set, the next event is read in normal mode. If @ref LIBEVDEV_READ_FLAG_SYNC is
 * set, the next event is read in sync mode.
 * @param ev On success, set to the current event.
 * @return On failure, a negative errno is returned.
 * @retval LIBEVDEV_READ_STATUS_SUCCESS One or more events were read of the
 * device and ev points to the next event in the queue
 * @retval -EAGAIN No events are currently available on the device
 * @retval LIBEVDEV_READ_STATUS_SYNC A SYN_DROPPED event was received, or a
 * synced event was returned and ev points to the SYN_DROPPED event
 *
 * @note This function is signal-safe.
 */
int libevdev_next_event(struct libevdev *dev, unsigned int flags, struct input_event *ev);

/**
 * @ingroup events
 *
 * Check if there are events waiting for us. This function does not read an
 * event off the fd and may not access the fd at all. If there are events
 * queued internally this function will return non-zero. If the internal
 * queue is empty, this function will poll the file descriptor for data.
 *
 * This is a convenience function for simple processes, most complex programs
 * are expected to use select(2) or poll(2) on the file descriptor. The kernel
 * guarantees that if data is available, it is a multiple of sizeof(struct
 * input_event), and thus calling libevdev_next_event() when select(2) or
 * poll(2) return is safe. You do not need libevdev_has_event_pending() if
 * you're using select(2) or poll(2).
 *
 * @param dev The evdev device, already initialized with libevdev_set_fd()
 * @return On failure, a negative errno is returned.
 * @retval 0 No event is currently available
 * @retval 1 One or more events are available on the fd
 *
 * @note This function is signal-safe.
 */
int libevdev_has_event_pending(struct libevdev *dev);

/**
 * @ingroup bits
 *
 * Retrieve the device's name, either as set by the caller or as read from
 * the kernel. The string returned is valid until libevdev_free() or until
 * libevdev_set_name(), whichever comes earlier.
 *
 * @param dev The evdev device, already initialized with libevdev_set_fd()
 *
 * @return The device name as read off the kernel device. The name is never
 * NULL but it may be the empty string.
 *
 * @note This function is signal-safe.
 */
const char* libevdev_get_name(const struct libevdev *dev);

/**
 * @ingroup kernel
 *
 * Change the device's name as returned by libevdev_get_name(). This
 * function destroys the string previously returned by libevdev_get_name(),
 * a caller must take care that no references are kept.
 *
 * @param dev The evdev device
 * @param name The new, non-NULL, name to assign to this device.
 *
 * @note This function may be called before libevdev_set_fd(). A call to
 * libevdev_set_fd() will overwrite any previously set value.
 */
void libevdev_set_name(struct libevdev *dev, const char *name);

/**
 * @ingroup bits
 *
 * Retrieve the device's physical location, either as set by the caller or
 * as read from the kernel. The string returned is valid until
 * libevdev_free() or until libevdev_set_phys(), whichever comes earlier.
 *
 * Virtual devices such as uinput devices have no phys location.
 *
 * @param dev The evdev device, already initialized with libevdev_set_fd()
 *
 * @return The physical location of this device, or NULL if there is none
 *
 * @note This function is signal safe.
 */
const char * libevdev_get_phys(const struct libevdev *dev);

/**
 * @ingroup kernel
 *
 * Change the device's physical location as returned by libevdev_get_phys().
 * This function destroys the string previously returned by
 * libevdev_get_phys(), a caller must take care that no references are kept.
 *
 * @param dev The evdev device
 * @param phys The new phys to assign to this device.
 *
 * @note This function may be called before libevdev_set_fd(). A call to
 * libevdev_set_fd() will overwrite any previously set value.
 */
void libevdev_set_phys(struct libevdev *dev, const char *phys);

/**
 * @ingroup bits
 *
 * Retrieve the device's unique identifier, either as set by the caller or
 * as read from the kernel. The string returned is valid until
 * libevdev_free() or until libevdev_set_uniq(), whichever comes earlier.
 *
 * @param dev The evdev device, already initialized with libevdev_set_fd()
 *
 * @return The unique identifier for this device, or NULL if there is none
 *
 * @note This function is signal safe.
 */
const char * libevdev_get_uniq(const struct libevdev *dev);

/**
 * @ingroup kernel
 *
 * Change the device's unique identifier as returned by libevdev_get_uniq().
 * This function destroys the string previously returned by
 * libevdev_get_uniq(), a caller must take care that no references are kept.
 *
 * @param dev The evdev device
 * @param uniq The new uniq to assign to this device.
 *
 * @note This function may be called before libevdev_set_fd(). A call to
 * libevdev_set_fd() will overwrite any previously set value.
 */
void libevdev_set_uniq(struct libevdev *dev, const char *uniq);

/**
 * @ingroup bits
 *
 * @param dev The evdev device, already initialized with libevdev_set_fd()
 *
 * @return The device's product ID
 *
 * @note This function is signal-safe.
 */
int libevdev_get_id_product(const struct libevdev *dev);

/**
 * @ingroup kernel
 *
 * @param dev The evdev device
 * @param product_id The product ID to assign to this device
 *
 * @note This function may be called before libevdev_set_fd(). A call to
 * libevdev_set_fd() will overwrite any previously set value. Even though
 * the function accepts an int for product_id the value is truncated at 16
 * bits.
 */
void libevdev_set_id_product(struct libevdev *dev, int product_id);

/**
 * @ingroup bits
 *
 * @param dev The evdev device, already initialized with libevdev_set_fd()
 *
 * @return The device's vendor ID
 *
 * @note This function is signal-safe.
 */
int libevdev_get_id_vendor(const struct libevdev *dev);

/**
 * @ingroup kernel
 *
 * @param dev The evdev device
 * @param vendor_id The vendor ID to assign to this device
 *
 * @note This function may be called before libevdev_set_fd(). A call to
 * libevdev_set_fd() will overwrite any previously set value. Even though
 * the function accepts an int for vendor_id the value is truncated at 16
 * bits.
 */
void libevdev_set_id_vendor(struct libevdev *dev, int vendor_id);

/**
 * @ingroup bits
 *
 * @param dev The evdev device, already initialized with libevdev_set_fd()
 *
 * @return The device's bus type
 *
 * @note This function is signal-safe.
 */
int libevdev_get_id_bustype(const struct libevdev *dev);

/**
 * @ingroup kernel
 *
 * @param dev The evdev device
 * @param bustype The bustype to assign to this device
 *
 * @note This function may be called before libevdev_set_fd(). A call to
 * libevdev_set_fd() will overwrite any previously set value. Even though
 * the function accepts an int for bustype the value is truncated at 16
 * bits.
 */
void libevdev_set_id_bustype(struct libevdev *dev, int bustype);

/**
 * @ingroup bits
 *
 * @param dev The evdev device, already initialized with libevdev_set_fd()
 *
 * @return The device's firmware version
 *
 * @note This function is signal-safe.
 */
int libevdev_get_id_version(const struct libevdev *dev);

/**
 * @ingroup kernel
 *
 * @param dev The evdev device
 * @param version The version to assign to this device
 *
 * @note This function may be called before libevdev_set_fd(). A call to
 * libevdev_set_fd() will overwrite any previously set value. Even though
 * the function accepts an int for version the value is truncated at 16
 * bits.
 */
void libevdev_set_id_version(struct libevdev *dev, int version);

/**
 * @ingroup bits
 *
 * @param dev The evdev device, already initialized with libevdev_set_fd()
 *
 * @return The driver version for this device
 *
 * @note This function is signal-safe.
 */
int libevdev_get_driver_version(const struct libevdev *dev);

/**
 * @ingroup bits
 *
 * @param dev The evdev device, already initialized with libevdev_set_fd()
 * @param prop The input property to query for, one of INPUT_PROP_...
 *
 * @return 1 if the device provides this input property, or 0 otherwise.
 *
 * @note This function is signal-safe
 */
int libevdev_has_property(const struct libevdev *dev, unsigned int prop);

/**
 * @ingroup kernel
 *
 * @param dev The evdev device
 * @param prop The input property to enable, one of INPUT_PROP_...
 *
 * @return 0 on success or -1 on failure
 *
 * @note This function may be called before libevdev_set_fd(). A call to
 * libevdev_set_fd() will overwrite any previously set value.
 */
int libevdev_enable_property(struct libevdev *dev, unsigned int prop);

/**
 * @ingroup kernel
 *
 * @param dev The evdev device
 * @param prop The input property to disable, one of INPUT_PROP_...
 *
 * @return 0 on success or -1 on failure
 */
int libevdev_disable_property(struct libevdev *dev, unsigned int prop);

/**
 * @ingroup bits
 *
 * @param dev The evdev device, already initialized with libevdev_set_fd()
 * @param type The event type to query for, one of EV_SYN, EV_REL, etc.
 *
 * @return 1 if the device supports this event type, or 0 otherwise.
 *
 * @note This function is signal-safe.
 */
int libevdev_has_event_type(const struct libevdev *dev, unsigned int type);

/**
 * @ingroup bits
 *
 * @param dev The evdev device, already initialized with libevdev_set_fd()
 * @param type The event type for the code to query (EV_SYN, EV_REL, etc.)
 * @param code The event code to query for, one of ABS_X, REL_X, etc.
 *
 * @return 1 if the device supports this event type and code, or 0 otherwise.
 *
 * @note This function is signal-safe.
 */
int libevdev_has_event_code(const struct libevdev *dev, unsigned int type, unsigned int code);

/**
 * @ingroup bits
 *
 * Get the minimum axis value for the given axis, as advertised by the kernel.
 *
 * @param dev The evdev device, already initialized with libevdev_set_fd()
 * @param code The EV_ABS event code to query for, one of ABS_X, ABS_Y, etc.
 *
 * @return axis minimum for the given axis or 0 if the axis is invalid
 *
 * @note This function is signal-safe.
 */
int libevdev_get_abs_minimum(const struct libevdev *dev, unsigned int code);

/**
 * @ingroup bits
 *
 * Get the maximum axis value for the given axis, as advertised by the kernel.
 *
 * @param dev The evdev device, already initialized with libevdev_set_fd()
 * @param code The EV_ABS event code to query for, one of ABS_X, ABS_Y, etc.
 *
 * @return axis maximum for the given axis or 0 if the axis is invalid
 *
 * @note This function is signal-safe.
 */
int libevdev_get_abs_maximum(const struct libevdev *dev, unsigned int code);

/**
 * @ingroup bits
 *
 * Get the axis fuzz for the given axis, as advertised by the kernel.
 *
 * @param dev The evdev device, already initialized with libevdev_set_fd()
 * @param code The EV_ABS event code to query for, one of ABS_X, ABS_Y, etc.
 *
 * @return axis fuzz for the given axis or 0 if the axis is invalid
 *
 * @note This function is signal-safe.
 */
int libevdev_get_abs_fuzz(const struct libevdev *dev, unsigned int code);

/**
 * @ingroup bits
 *
 * Get the axis flat for the given axis, as advertised by the kernel.
 *
 * @param dev The evdev device, already initialized with libevdev_set_fd()
 * @param code The EV_ABS event code to query for, one of ABS_X, ABS_Y, etc.
 *
 * @return axis flat for the given axis or 0 if the axis is invalid
 *
 * @note This function is signal-safe.
 */
int libevdev_get_abs_flat(const struct libevdev *dev, unsigned int code);

/**
 * @ingroup bits
 *
 * Get the axis resolution for the given axis, as advertised by the kernel.
 *
 * @param dev The evdev device, already initialized with libevdev_set_fd()
 * @param code The EV_ABS event code to query for, one of ABS_X, ABS_Y, etc.
 *
 * @return axis resolution for the given axis or 0 if the axis is invalid
 *
 * @note This function is signal-safe.
 */
int libevdev_get_abs_resolution(const struct libevdev *dev, unsigned int code);

/**
 * @ingroup bits
 *
 * Get the axis info for the given axis, as advertised by the kernel.
 *
 * @param dev The evdev device, already initialized with libevdev_set_fd()
 * @param code The EV_ABS event code to query for, one of ABS_X, ABS_Y, etc.
 *
 * @return The input_absinfo for the given code, or NULL if the device does
 * not support this event code.
 *
 * @note This function is signal-safe.
 */
const struct input_absinfo* libevdev_get_abs_info(const struct libevdev *dev, unsigned int code);

/**
 * @ingroup bits
 *
 * Behaviour of this function is undefined if the device does not provide
 * the event.
 *
 * If the device supports ABS_MT_SLOT, the value returned for any ABS_MT_*
 * event code is undefined. Use libevdev_get_slot_value() instead.
 *
 * @param dev The evdev device, already initialized with libevdev_set_fd()
 * @param type The event type for the code to query (EV_SYN, EV_REL, etc.)
 * @param code The event code to query for, one of ABS_X, REL_X, etc.
 *
 * @return The current value of the event.
 *
 * @note This function is signal-safe.
 * @note The value for ABS_MT_ events is undefined, use
 * libevdev_get_slot_value() instead
 *
 * @see libevdev_get_slot_value
 */
int libevdev_get_event_value(const struct libevdev *dev, unsigned int type, unsigned int code);

/**
 * @ingroup kernel
 *
 * Set the value for a given event type and code. This only makes sense for
 * some event types, e.g. setting the value for EV_REL is pointless.
 *
 * This is a local modification only affecting only this representation of
 * this device. A future call to libevdev_get_event_value() will return this
 * value, unless the value was overwritten by an event.
 *
 * If the device supports ABS_MT_SLOT, the value set for any ABS_MT_*
 * event code is the value of the currently active slot. You should use
 * libevdev_set_slot_value() instead.
 *
 * If the device supports ABS_MT_SLOT and the type is EV_ABS and the code is
 * ABS_MT_SLOT, the value must be a positive number less then the number of
 * slots on the device. Otherwise, libevdev_set_event_value() returns -1.
 *
 * @param dev The evdev device, already initialized with libevdev_set_fd()
 * @param type The event type for the code to query (EV_SYN, EV_REL, etc.)
 * @param code The event code to set the value for, one of ABS_X, LED_NUML, etc.
 * @param value The new value to set
 *
 * @return 0 on success, or -1 on failure.
 * @retval -1
 * - the device does not have the event type or
 * - code enabled, or the code is outside the, or
 * - the code is outside the allowed limits for the given type, or
 * - the type cannot be set, or
 * - the value is not permitted for the given code.
 *
 * @see libevdev_set_slot_value
 * @see libevdev_get_event_value
 */
int libevdev_set_event_value(struct libevdev *dev, unsigned int type, unsigned int code, int value);

/**
 * @ingroup bits
 *
 * Fetch the current value of the event type. This is a shortcut for
 *
 * @code
 *   if (libevdev_has_event_type(dev, t) && libevdev_has_event_code(dev, t, c))
 *        val = libevdev_get_event_value(dev, t, c);
 * @endcode
 *
 * @param dev The evdev device, already initialized with libevdev_set_fd()
 * @param type The event type for the code to query (EV_SYN, EV_REL, etc.)
 * @param code The event code to query for, one of ABS_X, REL_X, etc.
 * @param[out] value The current value of this axis returned.
 *
 * @return If the device supports this event type and code, the return value is
 * non-zero and value is set to the current value of this axis. Otherwise,
 * 0 is returned and value is unmodified.
 *
 * @note This function is signal-safe.
 * @note The value for ABS_MT_ events is undefined, use
 * libevdev_fetch_slot_value() instead
 *
 * @see libevdev_fetch_slot_value
 */
int libevdev_fetch_event_value(const struct libevdev *dev, unsigned int type, unsigned int code, int *value);

/**
 * @ingroup mt
 *
 * Return the current value of the code for the given slot.
 *
 * The return value is undefined for a slot exceeding the available slots on
 * the device, for a code that is not in the permitted ABS_MT range or for a
 * device that does not have slots.
 *
 * @param dev The evdev device, already initialized with libevdev_set_fd()
 * @param slot The numerical slot number, must be smaller than the total number
 * of slots on this device
 * @param code The event code to query for, one of ABS_MT_POSITION_X, etc.
 *
 * @note This function is signal-safe.
 * @note The value for events other than ABS_MT_ is undefined, use
 * libevdev_fetch_value() instead
 *
 * @see libevdev_get_event_value
 */
int libevdev_get_slot_value(const struct libevdev *dev, unsigned int slot, unsigned int code);

/**
 * @ingroup kernel
 *
 * Set the value for a given code for the given slot.
 *
 * This is a local modification only affecting only this representation of
 * this device. A future call to libevdev_get_slot_value() will return this
 * value, unless the value was overwritten by an event.
 *
 * This function does not set event values for axes outside the ABS_MT range,
 * use libevdev_set_event_value() instead.
 *
 * @param dev The evdev device, already initialized with libevdev_set_fd()
 * @param slot The numerical slot number, must be smaller than the total number
 * of slots on this device
 * @param code The event code to set the value for, one of ABS_MT_POSITION_X, etc.
 * @param value The new value to set
 *
 * @return 0 on success, or -1 on failure.
 * @retval -1
 * - the device does not have the event code enabled, or
 * - the code is outside the allowed limits for multitouch events, or
 * - the slot number is outside the limits for this device, or
 * - the device does not support multitouch events.
 *
 * @see libevdev_set_event_value
 * @see libevdev_get_slot_value
 */
int libevdev_set_slot_value(struct libevdev *dev, unsigned int slot, unsigned int code, int value);

/**
 * @ingroup mt
 *
 * Fetch the current value of the code for the given slot. This is a shortcut for
 *
 * @code
 *   if (libevdev_has_event_type(dev, EV_ABS) &&
 *       libevdev_has_event_code(dev, EV_ABS, c) &&
 *       slot < device->number_of_slots)
 *       val = libevdev_get_slot_value(dev, slot, c);
 * @endcode
 *
 * @param dev The evdev device, already initialized with libevdev_set_fd()
 * @param slot The numerical slot number, must be smaller than the total number
 * of slots on this * device
 * @param[out] value The current value of this axis returned.
 *
 * @param code The event code to query for, one of ABS_MT_POSITION_X, etc.
 * @return If the device supports this event code, the return value is
 * non-zero and value is set to the current value of this axis. Otherwise, or
 * if the event code is not an ABS_MT_* event code, 0 is returned and value
 * is unmodified.
 *
 * @note This function is signal-safe.
 */
int libevdev_fetch_slot_value(const struct libevdev *dev, unsigned int slot, unsigned int code, int *value);

/**
 * @ingroup mt
 *
 * Get the number of slots supported by this device.
 *
 * @param dev The evdev device, already initialized with libevdev_set_fd()
 *
 * @return The number of slots supported, or -1 if the device does not provide
 * any slots
 *
 * @note A device may provide ABS_MT_SLOT but a total number of 0 slots. Hence
 * the return value of -1 for "device does not provide slots at all"
 */
int libevdev_get_num_slots(const struct libevdev *dev);

/**
 * @ingroup mt
 *
 * Get the currently active slot. This may differ from the value
 * an ioctl may return at this time as events may have been read off the fd
 * since changing the slot value but those events are still in the buffer
 * waiting to be processed. The returned value is the value a caller would
 * see if it were to process events manually one-by-one.
 *
 * @param dev The evdev device, already initialized with libevdev_set_fd()
 *
 * @return the currently active slot (logically)
 *
 * @note This function is signal-safe.
 */
int libevdev_get_current_slot(const struct libevdev *dev);

/**
 * @ingroup kernel
 *
 * Change the minimum for the given EV_ABS event code, if the code exists.
 * This function has no effect if libevdev_has_event_code() returns false for
 * this code.
 *
 * @param dev The evdev device, already initialized with libevdev_set_fd()
 * @param code One of ABS_X, ABS_Y, ...
 * @param val The new minimum for this axis
 */
void libevdev_set_abs_minimum(struct libevdev *dev, unsigned int code, int val);

/**
 * @ingroup kernel
 *
 * Change the maximum for the given EV_ABS event code, if the code exists.
 * This function has no effect if libevdev_has_event_code() returns false for
 * this code.
 *
 * @param dev The evdev device, already initialized with libevdev_set_fd()
 * @param code One of ABS_X, ABS_Y, ...
 * @param val The new maxium for this axis
 */
void libevdev_set_abs_maximum(struct libevdev *dev, unsigned int code, int val);

/**
 * @ingroup kernel
 *
 * Change the fuzz for the given EV_ABS event code, if the code exists.
 * This function has no effect if libevdev_has_event_code() returns false for
 * this code.
 *
 * @param dev The evdev device, already initialized with libevdev_set_fd()
 * @param code One of ABS_X, ABS_Y, ...
 * @param val The new fuzz for this axis
 */
void libevdev_set_abs_fuzz(struct libevdev *dev, unsigned int code, int val);

/**
 * @ingroup kernel
 *
 * Change the flat for the given EV_ABS event code, if the code exists.
 * This function has no effect if libevdev_has_event_code() returns false for
 * this code.
 *
 * @param dev The evdev device, already initialized with libevdev_set_fd()
 * @param code One of ABS_X, ABS_Y, ...
 * @param val The new flat for this axis
 */
void libevdev_set_abs_flat(struct libevdev *dev, unsigned int code, int val);

/**
 * @ingroup kernel
 *
 * Change the resolution for the given EV_ABS event code, if the code exists.
 * This function has no effect if libevdev_has_event_code() returns false for
 * this code.
 *
 * @param dev The evdev device, already initialized with libevdev_set_fd()
 * @param code One of ABS_X, ABS_Y, ...
 * @param val The new axis resolution
 */
void libevdev_set_abs_resolution(struct libevdev *dev, unsigned int code, int val);

/**
 * @ingroup kernel
 *
 * Change the abs info for the given EV_ABS event code, if the code exists.
 * This function has no effect if libevdev_has_event_code() returns false for
 * this code.
 *
 * @param dev The evdev device, already initialized with libevdev_set_fd()
 * @param code One of ABS_X, ABS_Y, ...
 * @param abs The new absolute axis data (min, max, fuzz, flat, resolution)
 */
void libevdev_set_abs_info(struct libevdev *dev, unsigned int code, const struct input_absinfo *abs);

/**
 * @ingroup kernel
 *
 * Forcibly enable an event type on this device, even if the underlying
 * device does not support it. While this cannot make the device actually
 * report such events, it will now return true for libevdev_has_event_type().
 *
 * This is a local modification only affecting only this representation of
 * this device.
 *
 * @param dev The evdev device, already initialized with libevdev_set_fd()
 * @param type The event type to enable (EV_ABS, EV_KEY, ...)
 *
 * @return 0 on success or -1 otherwise
 *
 * @see libevdev_has_event_type
 */
int libevdev_enable_event_type(struct libevdev *dev, unsigned int type);

/**
 * @ingroup kernel
 *
 * Forcibly disable an event type on this device, even if the underlying
 * device provides it. This effectively mutes the respective set of
 * events. libevdev will filter any events matching this type and none will
 * reach the caller. libevdev_has_event_type() will return false for this
 * type.
 *
 * In most cases, a caller likely only wants to disable a single code, not
 * the whole type. Use libevdev_disable_event_code() for that.
 *
 * Disabling EV_SYN will not work. Don't shoot yourself in the foot.
 * It hurts.
 *
 * This is a local modification only affecting only this representation of
 * this device.
 *
 * @param dev The evdev device, already initialized with libevdev_set_fd()
 * @param type The event type to disable (EV_ABS, EV_KEY, ...)
 *
 * @return 0 on success or -1 otherwise
 *
 * @see libevdev_has_event_type
 * @see libevdev_disable_event_type
 */
int libevdev_disable_event_type(struct libevdev *dev, unsigned int type);

/**
 * @ingroup kernel
 *
 * Forcibly enable an event code on this device, even if the underlying
 * device does not support it. While this cannot make the device actually
 * report such events, it will now return true for libevdev_has_event_code().
 *
 * The last argument depends on the type and code:
 * - If type is EV_ABS, data must be a pointer to a struct input_absinfo
 *   containing the data for this axis.
 * - If type is EV_REP, data must be a pointer to a int containing the data
 *   for this axis
 * - For all other types, the argument must be NULL.
 *
 * This function calls libevdev_enable_event_type() if necessary.
 *
 * This is a local modification only affecting only this representation of
 * this device.
 *
 * If this function is called with a type of EV_ABS and EV_REP on a device
 * that already has the given event code enabled, the values in data
 * overwrite the previous values.
 *
 * @param dev The evdev device, already initialized with libevdev_set_fd()
 * @param type The event type to enable (EV_ABS, EV_KEY, ...)
 * @param code The event code to enable (ABS_X, REL_X, etc.)
 * @param data If type is EV_ABS, data points to a struct input_absinfo. If type is EV_REP, data
 * points to an integer. Otherwise, data must be NULL.
 *
 * @return 0 on success or -1 otherwise
 *
 * @see libevdev_enable_event_type
 */
int libevdev_enable_event_code(struct libevdev *dev, unsigned int type, unsigned int code, const void *data);

/**
 * @ingroup kernel
 *
 * Forcibly disable an event code on this device, even if the underlying
 * device provides it. This effectively mutes the respective set of
 * events. libevdev will filter any events matching this type and code and
 * none will reach the caller. libevdev_has_event_code() will return false for
 * this code.
 *
 * Disabling all event codes for a given type will not disable the event
 * type. Use libevdev_disable_event_type() for that.
 *
 * This is a local modification only affecting only this representation of
 * this device.
 *
 * Disabling codes of type EV_SYN will not work. Don't shoot yourself in the
 * foot. It hurts.
 *
 * @param dev The evdev device, already initialized with libevdev_set_fd()
 * @param type The event type to disable (EV_ABS, EV_KEY, ...)
 * @param code The event code to disable (ABS_X, REL_X, etc.)
 *
 * @return 0 on success or -1 otherwise
 *
 * @see libevdev_has_event_code
 * @see libevdev_disable_event_type
 */
int libevdev_disable_event_code(struct libevdev *dev, unsigned int type, unsigned int code);

/**
 * @ingroup kernel
 *
 * Set the device's EV_ABS axis to the value defined in the abs
 * parameter. This will be written to the kernel.
 *
 * @param dev The evdev device, already initialized with libevdev_set_fd()
 * @param code The EV_ABS event code to modify, one of ABS_X, ABS_Y, etc.
 * @param abs Axis info to set the kernel axis to
 *
 * @return 0 on success, or a negative errno on failure
 *
 * @see libevdev_enable_event_code
 */
int libevdev_kernel_set_abs_info(struct libevdev *dev, unsigned int code, const struct input_absinfo *abs);

/**
 * @ingroup kernel
 */
enum libevdev_led_value {
	LIBEVDEV_LED_ON = 3, /**< Turn the LED on */
	LIBEVDEV_LED_OFF = 4 /**< Turn the LED off */
};

/**
 * @ingroup kernel
 *
 * Turn an LED on or off. Convenience function, if you need to modify multiple
 * LEDs simultaneously, use libevdev_kernel_set_led_values() instead.
 *
 * @note enabling an LED requires write permissions on the device's file descriptor.
 *
 * @param dev The evdev device, already initialized with libevdev_set_fd()
 * @param code The EV_LED event code to modify, one of LED_NUML, LED_CAPSL, ...
 * @param value Specifies whether to turn the LED on or off
 * @return 0 on success, or a negative errno on failure
 */
int libevdev_kernel_set_led_value(struct libevdev *dev, unsigned int code, enum libevdev_led_value value);

/**
 * @ingroup kernel
 *
 * Turn multiple LEDs on or off simultaneously. This function expects a pair
 * of LED codes and values to set them to, terminated by a -1. For example, to
 * switch the NumLock LED on but the CapsLock LED off, use:
 *
 * @code
 *     libevdev_kernel_set_led_values(dev, LED_NUML, LIBEVDEV_LED_ON,
 *                                         LED_CAPSL, LIBEVDEV_LED_OFF,
 *                                         -1);
 * @endcode
 *
 * If any LED code or value is invalid, this function returns -EINVAL and no
 * LEDs are modified.
 *
 * @note enabling an LED requires write permissions on the device's file descriptor.
 *
 * @param dev The evdev device, already initialized with libevdev_set_fd()
 * @param ... A pair of LED_* event codes and libevdev_led_value_t, followed by
 * -1 to terminate the list.
 * @return 0 on success, or a negative errno on failure
 */
int libevdev_kernel_set_led_values(struct libevdev *dev, ...);

/**
 * @ingroup kernel
 *
 * Set the clock ID to be used for timestamps. Further events from this device
 * will report an event time based on the given clock.
 *
 * This is a modification only affecting this representation of
 * this device.
 *
 * @param dev The evdev device, already initialized with libevdev_set_fd()
 * @param clockid The clock to use for future events. Permitted values
 * are CLOCK_MONOTONIC and CLOCK_REALTIME (the default).
 * @return 0 on success, or a negative errno on failure
 */
int libevdev_set_clock_id(struct libevdev *dev, int clockid);

/**
 * @ingroup misc
 *
 * Helper function to check if an event is of a specific type. This is
 * virtually the same as:
 *
 *      ev->type == type
 *
 * with the exception that some sanity checks are performed to ensure type
 * is valid.
 *
 * @note The ranges for types are compiled into libevdev. If the kernel
 * changes the max value, libevdev will not automatically pick these up.
 *
 * @param ev The input event to check
 * @param type Input event type to compare the event against (EV_REL, EV_ABS,
 * etc.)
 *
 * @return 1 if the event type matches the given type, 0 otherwise (or if
 * type is invalid)
 */
int libevdev_event_is_type(const struct input_event *ev, unsigned int type);

/**
 * @ingroup misc
 *
 * Helper function to check if an event is of a specific type and code. This
 * is virtually the same as:
 *
 *      ev->type == type && ev->code == code
 *
 * with the exception that some sanity checks are performed to ensure type and
 * code are valid.
 *
 * @note The ranges for types and codes are compiled into libevdev. If the kernel
 * changes the max value, libevdev will not automatically pick these up.
 *
 * @param ev The input event to check
 * @param type Input event type to compare the event against (EV_REL, EV_ABS,
 * etc.)
 * @param code Input event code to compare the event against (ABS_X, REL_X,
 * etc.)
 *
 * @return 1 if the event type matches the given type and code, 0 otherwise
 * (or if type/code are invalid)
 */
int libevdev_event_is_code(const struct input_event *ev, unsigned int type, unsigned int code);

/**
 * @ingroup misc
 *
 * @param type The event type to return the name for.
 *
 * @return The name of the given event type (e.g. EV_ABS) or NULL for an
 * invalid type
 *
 * @note The list of names is compiled into libevdev. If the kernel adds new
 * defines for new event types, libevdev will not automatically pick these up.
 */
const char * libevdev_event_type_get_name(unsigned int type);
/**
 * @ingroup misc
 *
 * @param type The event type for the code to query (EV_SYN, EV_REL, etc.)
 * @param code The event code to return the name for (e.g. ABS_X)
 *
 * @return The name of the given event code (e.g. ABS_X) or NULL for an
 * invalid type or code
 *
 * @note The list of names is compiled into libevdev. If the kernel adds new
 * defines for new event codes, libevdev will not automatically pick these up.
 */
const char * libevdev_event_code_get_name(unsigned int type, unsigned int code);

/**
 * @ingroup misc
 *
 * This function resolves the event value for a code.
 *
 * For almost all event codes this will return NULL as the value is just a
 * numerical value. As of kernel 4.17, the only event code that will return
 * a non-NULL value is EV_ABS/ABS_MT_TOOL_TYPE.
 *
 * @param type The event type for the value to query (EV_ABS, etc.)
 * @param code The event code for the value to query (e.g. ABS_MT_TOOL_TYPE)
 * @param value The event value to return the name for (e.g. MT_TOOL_PALM)
 *
 * @return The name of the given event value (e.g. MT_TOOL_PALM) or NULL for
 * an invalid type or code or NULL for an axis that has numerical values
 * only.
 *
 * @note The list of names is compiled into libevdev. If the kernel adds new
 * defines for new event values, libevdev will not automatically pick these up.
 */
const char * libevdev_event_value_get_name(unsigned int type,
					   unsigned int code,
					   int value);
/**
 * @ingroup misc
 *
 * @param prop The input prop to return the name for (e.g. INPUT_PROP_BUTTONPAD)
 *
 * @return The name of the given input prop (e.g. INPUT_PROP_BUTTONPAD) or NULL for an
 * invalid property
 *
 * @note The list of names is compiled into libevdev. If the kernel adds new
 * defines for new properties libevdev will not automatically pick these up.
 * @note On older kernels input properties may not be defined and
 * libevdev_property_get_name() will always return NULL
 */
const char* libevdev_property_get_name(unsigned int prop);

/**
 * @ingroup misc
 *
 * @param type The event type to return the maximum for (EV_ABS, EV_REL, etc.). No max is defined for
 * EV_SYN.
 *
 * @return The max value defined for the given event type, e.g. ABS_MAX for a type of EV_ABS, or -1
 * for an invalid type.
 *
 * @note The max value is compiled into libevdev. If the kernel changes the
 * max value, libevdev will not automatically pick these up.
 */
int libevdev_event_type_get_max(unsigned int type);

/**
 * @ingroup misc
 *
 * Look up an event-type by its name. Event-types start with "EV_" followed by
 * the name (eg., "EV_ABS"). The "EV_" prefix must be included in the name. It
 * returns the constant assigned to the event-type or -1 if not found.
 *
 * @param name A non-NULL string describing an input-event type ("EV_KEY",
 * "EV_ABS", ...), zero-terminated.
 *
 * @return The given type constant for the passed name or -1 if not found.
 *
 * @note EV_MAX is also recognized.
 */
int libevdev_event_type_from_name(const char *name);

/**
 * @ingroup misc
 *
 * Look up an event-type by its name. Event-types start with "EV_" followed by
 * the name (eg., "EV_ABS"). The "EV_" prefix must be included in the name. It
 * returns the constant assigned to the event-type or -1 if not found.
 *
 * @param name A non-NULL string describing an input-event type ("EV_KEY",
 * "EV_ABS", ...).
 * @param len The length of the passed string excluding any terminating 0
 * character.
 *
 * @return The given type constant for the passed name or -1 if not found.
 *
 * @note EV_MAX is also recognized.
 */
int libevdev_event_type_from_name_n(const char *name, size_t len);

/**
 * @ingroup misc
 *
 * Look up an event code by its type and name. Event codes start with a fixed
 * prefix followed by their name (eg., "ABS_X"). The prefix must be included in
 * the name. It returns the constant assigned to the event code or -1 if not
 * found.
 *
 * You have to pass the event type where to look for the name. For instance, to
 * resolve "ABS_X" you need to pass EV_ABS as type and "ABS_X" as string.
 * Supported event codes are codes starting with SYN_, KEY_, BTN_, REL_, ABS_,
 * MSC_, SND_, SW_, LED_, REP_, FF_.
 *
 * @param type The event type (EV_* constant) where to look for the name.
 * @param name A non-NULL string describing an input-event code ("KEY_A",
 * "ABS_X", "BTN_Y", ...), zero-terminated.
 *
 * @return The given code constant for the passed name or -1 if not found.
 */
int libevdev_event_code_from_name(unsigned int type, const char *name);

/**
 * @ingroup misc
 *
 * Look up an event code by its type and name. Event codes start with a fixed
 * prefix followed by their name (eg., "ABS_X"). The prefix must be included in
 * the name. It returns the constant assigned to the event code or -1 if not
 * found.
 *
 * You have to pass the event type where to look for the name. For instance, to
 * resolve "ABS_X" you need to pass EV_ABS as type and "ABS_X" as string.
 * Supported event codes are codes starting with SYN_, KEY_, BTN_, REL_, ABS_,
 * MSC_, SND_, SW_, LED_, REP_, FF_.
 *
 * @param type The event type (EV_* constant) where to look for the name.
 * @param name A non-NULL string describing an input-event code ("KEY_A",
 * "ABS_X", "BTN_Y", ...).
 * @param len The length of the string in @p name excluding any terminating 0
 * character.
 *
 * @return The given code constant for the name or -1 if not found.
 */
int libevdev_event_code_from_name_n(unsigned int type, const char *name,
				    size_t len);

/**
 * @ingroup misc
 *
 * Look up an event value by its type, code and name. Event values start
 * with a fixed prefix followed by their name (eg., "MT_TOOL_PALM"). The
 * prefix must be included in the name. It returns the constant assigned
 * to the event code or -1 if not found.
 *
 * You have to pass the event type and code where to look for the name. For
 * instance, to resolve "MT_TOOL_PALM" you need to pass EV_ABS as type,
 * ABS_MT_TOOL_TYPE as code and "MT_TOOL_PALM" as string.
 *
 * As of kernel 4.17, only EV_ABS/ABS_MT_TOOL_TYPE support name resolution.
 *
 * @param type The event type (EV_* constant) where to look for the name.
 * @param code The event code (ABS_* constant) where to look for the name.
 * @param name A non-NULL string describing an input-event value
 * ("MT_TOOL_TYPE", ...)
 *
 * @return The given value constant for the name or -1 if not found.
 */
int libevdev_event_value_from_name(unsigned int type, unsigned int code,
				   const char *name);

/**
 * @ingroup misc
 *
 * Look up an event type for a  event code name. For example, the name
 * "ABS_Y" returns EV_ABS. For the lookup to succeed, the name must be
 * unique, which is the case for all defines as of kernel 5.0 and likely to
 * be the case in the future.
 *
 * This is equivalent to libevdev_event_type_from_name() but takes the code
 * name instead of the type name.
 *
 * @param name A non-NULL string describing an input-event value
 * ("ABS_X", "REL_Y", "KEY_A", ...)
 *
 * @return The given event code for the name or -1 if not found.
 */
int
libevdev_event_type_from_code_name(const char *name);

/**
 * @ingroup misc
 *
 * Look up an event type for a  event code name. For example, the name
 * "ABS_Y" returns EV_ABS. For the lookup to succeed, the name must be
 * unique, which is the case for all defines as of kernel 5.0 and likely to
 * be the case in the future.
 *
 * This is equivalent to libevdev_event_type_from_name_n() but takes the code
 * name instead of the type name.
 *
 * @param name A non-NULL string describing an input-event value
 * ("ABS_X", "REL_Y", "KEY_A", ...)
 * @param len The length of the passed string excluding any terminating 0
 * character.
 *
 * @return The given event code for the name or -1 if not found.
 */
int
libevdev_event_type_from_code_name_n(const char *name, size_t len);

/**
 * @ingroup misc
 *
 * Look up an event code by its name. For example, the name "ABS_Y"
 * returns 1. For the lookup to succeed, the name must be unique, which is
 * the case for all defines as of kernel 5.0 and likely to be the case in
 * the future.
 *
 * This is equivalent to libevdev_event_code_from_name() without the need
 * for knowing the event type.
 *
 * @param name A non-NULL string describing an input-event value
 * ("ABS_X", "REL_Y", "KEY_A", ...)
 *
 * @return The given event code for the name or -1 if not found.
 */
int
libevdev_event_code_from_code_name(const char *name);

/**
 * @ingroup misc
 *
 * Look up an event code by its name. For example, the name "ABS_Y"
 * returns 1. For the lookup to succeed, the name must be unique, which is
 * the case for all defines as of kernel 5.0 and likely to be the case in
 * the future.
 *
 * This is equivalent to libevdev_event_code_from_name_n() without the need
 * for knowing the event type.
 *
 * @param name A non-NULL string describing an input-event value
 * ("ABS_X", "REL_Y", "KEY_A", ...)
 * @param len The length of the passed string excluding any terminating 0
 * character.
 *
 * @return The given event code for the name or -1 if not found.
 */
int
libevdev_event_code_from_code_name_n(const char *name, size_t len);

/**
 * @ingroup misc
 *
 * Look up an event value by its type, code and name. Event values start
 * with a fixed prefix followed by their name (eg., "MT_TOOL_PALM"). The
 * prefix must be included in the name. It returns the constant assigned
 * to the event code or -1 if not found.
 *
 * You have to pass the event type and code where to look for the name. For
 * instance, to resolve "MT_TOOL_PALM" you need to pass EV_ABS as type,
 * ABS_MT_TOOL_TYPE as code and "MT_TOOL_PALM" as string.
 *
 * As of kernel 4.17, only EV_ABS/ABS_MT_TOOL_TYPE support name resolution.
 *
 * @param type The event type (EV_* constant) where to look for the name.
 * @param code The event code (ABS_* constant) where to look for the name.
 * @param name A non-NULL string describing an input-event value
 * ("MT_TOOL_TYPE", ...)
 * @param len The length of the string in @p name excluding any terminating 0
 * character.
 *
 * @return The given value constant for the name or -1 if not found.
 */
int libevdev_event_value_from_name_n(unsigned int type, unsigned int code,
				     const char *name, size_t len);

/**
 * @ingroup misc
 *
 * Look up an input property by its name. Properties start with the fixed
 * prefix "INPUT_PROP_" followed by their name (eg., "INPUT_PROP_POINTER").
 * The prefix must be included in the name. It returns the constant assigned
 * to the property or -1 if not found.
 *
 * @param name A non-NULL string describing an input property
 *
 * @return The given code constant for the name or -1 if not found.
 */
int libevdev_property_from_name(const char *name);

/**
 * @ingroup misc
 *
 * Look up an input property by its name. Properties start with the fixed
 * prefix "INPUT_PROP_" followed by their name (eg., "INPUT_PROP_POINTER").
 * The prefix must be included in the name. It returns the constant assigned
 * to the property or -1 if not found.
 *
 * @param name A non-NULL string describing an input property
 * @param len The length of the string in @p name excluding any terminating 0
 * character.
 *
 * @return The given code constant for the name or -1 if not found.
 */
int libevdev_property_from_name_n(const char *name, size_t len);

/**
 * @ingroup bits
 *
 * Get the repeat delay and repeat period values for this device. This
 * function is a convenience function only, EV_REP is supported by
 * libevdev_get_event_value().
 *
 * @param dev The evdev device, already initialized with libevdev_set_fd()
 * @param delay If not null, set to the repeat delay value
 * @param period If not null, set to the repeat period value
 *
 * @return 0 on success, -1 if this device does not have repeat settings.
 *
 * @note This function is signal-safe
 *
 * @see libevdev_get_event_value
 */
int libevdev_get_repeat(const struct libevdev *dev, int *delay, int *period);

/********* DEPRECATED SECTION *********/
#if defined(__GNUC__) && __GNUC__ >= 4
#define LIBEVDEV_DEPRECATED __attribute__ ((deprecated))
#else
#define LIBEVDEV_DEPRECATED
#endif

#ifdef __cplusplus
}
#endif

#endif /* LIBEVDEV_H */
