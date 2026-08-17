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

#ifndef SPA_PARAM_PORT_CONFIG_H
#define SPA_PARAM_PORT_CONFIG_H

#ifdef __cplusplus
extern "C" {
#endif

/**
 * \addtogroup spa_param
 * \{
 */

#include <spa/param/param.h>

enum spa_param_port_config_mode {
	SPA_PARAM_PORT_CONFIG_MODE_none,	/**< no configuration */
	SPA_PARAM_PORT_CONFIG_MODE_passthrough,	/**< passthrough configuration */
	SPA_PARAM_PORT_CONFIG_MODE_convert,	/**< convert configuration */
	SPA_PARAM_PORT_CONFIG_MODE_dsp,		/**< dsp configuration, depending on the external
						  *  format. For audio, ports will be configured for
						  *  the given number of channels with F32 format. */
};

/** properties for SPA_TYPE_OBJECT_ParamPortConfig */
enum spa_param_port_config {
	SPA_PARAM_PORT_CONFIG_START,
	SPA_PARAM_PORT_CONFIG_direction,	/**< direction, input/output (Id enum spa_direction) */
	SPA_PARAM_PORT_CONFIG_mode,		/**< (Id enum spa_param_port_config_mode) mode */
	SPA_PARAM_PORT_CONFIG_monitor,		/**< (Bool) enable monitor output ports on input ports */
	SPA_PARAM_PORT_CONFIG_control,		/**< (Bool) enable control ports */
	SPA_PARAM_PORT_CONFIG_format,		/**< (Object) format filter */
};

/**
 * \}
 */

#ifdef __cplusplus
}  /* extern "C" */
#endif

#endif /* SPA_PARAM_PORT_CONFIG_H */
