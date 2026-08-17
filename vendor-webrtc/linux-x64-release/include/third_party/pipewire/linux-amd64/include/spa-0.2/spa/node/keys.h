/* Simple Plugin API
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

#ifndef SPA_NODE_KEYS_H
#define SPA_NODE_KEYS_H

#ifdef __cplusplus
extern "C" {
#endif

/**
 * \addtogroup spa_node
 * \{
 */

/** node keys */
#define SPA_KEY_NODE_NAME		"node.name"		/**< a node name */
#define SPA_KEY_NODE_LATENCY		"node.latency"		/**< the requested node latency */
#define SPA_KEY_NODE_MAX_LATENCY	"node.max-latency"	/**< maximum supported latency */

#define SPA_KEY_NODE_DRIVER		"node.driver"		/**< the node can be a driver */
#define SPA_KEY_NODE_ALWAYS_PROCESS	"node.always-process"	/**< call the process function even if
								  *  not linked. */
#define SPA_KEY_NODE_PAUSE_ON_IDLE	"node.pause-on-idle"	/**< if the node should be paused
								  *  immediately when idle. */
#define SPA_KEY_NODE_MONITOR		"node.monitor"		/**< the node has monitor ports */


/** port keys */
#define SPA_KEY_PORT_NAME		"port.name"		/**< a port name */
#define SPA_KEY_PORT_ALIAS		"port.alias"		/**< a port alias */
#define SPA_KEY_PORT_MONITOR		"port.monitor"		/**< this port is a monitor port */


/**
 * \}
 */

#ifdef __cplusplus
}  /* extern "C" */
#endif

#endif /* SPA_NODE_KEYS_H */
