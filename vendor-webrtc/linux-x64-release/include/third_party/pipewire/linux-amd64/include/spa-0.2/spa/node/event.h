/* Simple Plugin API
 *
 * Copyright © 2018 Wim Taymans
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

#ifndef SPA_EVENT_NODE_H
#define SPA_EVENT_NODE_H

#ifdef __cplusplus
extern "C" {
#endif

/**
 * \addtogroup spa_node
 * \{
 */

#include <spa/pod/event.h>

/* object id of SPA_TYPE_EVENT_Node */
enum spa_node_event {
	SPA_NODE_EVENT_Error,
	SPA_NODE_EVENT_Buffering,
	SPA_NODE_EVENT_RequestRefresh,
	SPA_NODE_EVENT_RequestProcess,		/*< Ask the driver to start processing
						 *  the graph */
};

#define SPA_NODE_EVENT_ID(ev)	SPA_EVENT_ID(ev, SPA_TYPE_EVENT_Node)
#define SPA_NODE_EVENT_INIT(id) SPA_EVENT_INIT(SPA_TYPE_EVENT_Node, id)

/* properties for SPA_TYPE_EVENT_Node */
enum spa_event_node {
	SPA_EVENT_NODE_START,
};

/**
 * \}
 */

#ifdef __cplusplus
}  /* extern "C" */
#endif

#endif /* SPA_EVENT_NODE_H */
