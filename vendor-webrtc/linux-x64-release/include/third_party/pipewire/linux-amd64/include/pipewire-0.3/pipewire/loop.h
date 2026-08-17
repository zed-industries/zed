/* PipeWire
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

#ifndef PIPEWIRE_LOOP_H
#define PIPEWIRE_LOOP_H

#ifdef __cplusplus
extern "C" {
#endif

#include <spa/support/loop.h>
#include <spa/utils/dict.h>

/** \defgroup pw_loop Loop
 *
 * PipeWire loop object provides an implementation of
 * the spa loop interfaces. It can be used to implement various
 * event loops.
 */

/**
 * \addtogroup pw_loop
 * \{
 */

struct pw_loop {
	struct spa_system *system;		/**< system utils */
	struct spa_loop *loop;			/**< wrapped loop */
	struct spa_loop_control *control;	/**< loop control */
	struct spa_loop_utils *utils;		/**< loop utils */
};

struct pw_loop *
pw_loop_new(const struct spa_dict *props);

void
pw_loop_destroy(struct pw_loop *loop);

#define pw_loop_add_source(l,...)	spa_loop_add_source((l)->loop,__VA_ARGS__)
#define pw_loop_update_source(l,...)	spa_loop_update_source((l)->loop,__VA_ARGS__)
#define pw_loop_remove_source(l,...)	spa_loop_remove_source((l)->loop,__VA_ARGS__)
#define pw_loop_invoke(l,...)		spa_loop_invoke((l)->loop,__VA_ARGS__)

#define pw_loop_get_fd(l)		spa_loop_control_get_fd((l)->control)
#define pw_loop_add_hook(l,...)		spa_loop_control_add_hook((l)->control,__VA_ARGS__)
#define pw_loop_enter(l)		spa_loop_control_enter((l)->control)
#define pw_loop_iterate(l,...)		spa_loop_control_iterate((l)->control,__VA_ARGS__)
#define pw_loop_leave(l)		spa_loop_control_leave((l)->control)

#define pw_loop_add_io(l,...)		spa_loop_utils_add_io((l)->utils,__VA_ARGS__)
#define pw_loop_update_io(l,...)	spa_loop_utils_update_io((l)->utils,__VA_ARGS__)
#define pw_loop_add_idle(l,...)		spa_loop_utils_add_idle((l)->utils,__VA_ARGS__)
#define pw_loop_enable_idle(l,...)	spa_loop_utils_enable_idle((l)->utils,__VA_ARGS__)
#define pw_loop_add_event(l,...)	spa_loop_utils_add_event((l)->utils,__VA_ARGS__)
#define pw_loop_signal_event(l,...)	spa_loop_utils_signal_event((l)->utils,__VA_ARGS__)
#define pw_loop_add_timer(l,...)	spa_loop_utils_add_timer((l)->utils,__VA_ARGS__)
#define pw_loop_update_timer(l,...)	spa_loop_utils_update_timer((l)->utils,__VA_ARGS__)
#define pw_loop_add_signal(l,...)	spa_loop_utils_add_signal((l)->utils,__VA_ARGS__)
#define pw_loop_destroy_source(l,...)	spa_loop_utils_destroy_source((l)->utils,__VA_ARGS__)

/**
 * \}
 */

#ifdef __cplusplus
}
#endif

#endif /* PIPEWIRE_LOOP_H */
