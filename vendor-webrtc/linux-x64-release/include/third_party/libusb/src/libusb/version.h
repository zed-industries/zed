/*
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
 * License along with this library; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA
 */

/* This file is parsed by m4 and windres and RC.EXE so please keep it simple. */
#include "version_nano.h"
#ifndef LIBUSB_MAJOR
#define LIBUSB_MAJOR 1
#endif
#ifndef LIBUSB_MINOR
#define LIBUSB_MINOR 0
#endif
#ifndef LIBUSB_MICRO
#define LIBUSB_MICRO 17
#endif
#ifndef LIBUSB_NANO
#define LIBUSB_NANO 0
#endif
/* LIBUSB_RC is the release candidate suffix. Should normally be empty. */
#ifndef LIBUSB_RC
#define LIBUSB_RC ""
#endif
