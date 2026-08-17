/* PipeWire
 *
 * Copyright © 2019 Wim Taymans
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
 */

#ifndef PIPEWIRE_IMPL_H
#define PIPEWIRE_IMPL_H

#ifdef __cplusplus
extern "C" {
#endif

/**
 * \addtogroup api_pw_impl
 */

struct pw_impl_client;
struct pw_impl_module;
struct pw_global;
struct pw_node;
struct pw_impl_port;
struct pw_resource;

#include <pipewire/pipewire.h>
#include <pipewire/control.h>
#include <pipewire/impl-core.h>
#include <pipewire/impl-client.h>
#include <pipewire/impl-device.h>
#include <pipewire/impl-factory.h>
#include <pipewire/global.h>
#include <pipewire/impl-link.h>
#include <pipewire/impl-metadata.h>
#include <pipewire/impl-module.h>
#include <pipewire/impl-node.h>
#include <pipewire/impl-port.h>
#include <pipewire/resource.h>
#include <pipewire/work-queue.h>

#ifdef __cplusplus
}
#endif

#endif /* PIPEWIRE_IMPL_H */
