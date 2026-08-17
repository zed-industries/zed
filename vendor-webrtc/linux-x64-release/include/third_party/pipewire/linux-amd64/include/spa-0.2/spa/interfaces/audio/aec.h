/* PipeWire
 *
 * Copyright © 2021 Wim Taymans <wim.taymans@gmail.com>
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


#include <spa/utils/dict.h>
#include <spa/utils/hook.h>
#include <spa/param/audio/raw.h>

#ifndef SPA_AUDIO_AEC_H
#define SPA_AUDIO_AEC_H

#ifdef __cplusplus
extern "C" {
#endif

#define SPA_TYPE_INTERFACE_AUDIO_AEC SPA_TYPE_INFO_INTERFACE_BASE "Audio:AEC"

#define SPA_VERSION_AUDIO_AEC   0
struct spa_audio_aec {
	struct spa_interface iface;
	const char *name;
	const struct spa_dict *info;
	const char *latency;
};

struct spa_audio_aec_info {
#define SPA_AUDIO_AEC_CHANGE_MASK_PROPS	(1u<<0)
        uint64_t change_mask;

	const struct spa_dict *props;
};

struct spa_audio_aec_events {
#define SPA_VERSION_AUDIO_AEC_EVENTS	0
        uint32_t version;       /**< version of this structure */

	/** Emitted when info changes */
	void (*info) (void *data, const struct spa_audio_aec_info *info);
};

struct spa_audio_aec_methods {
#define SPA_VERSION_AUDIO_AEC_METHODS	0
        uint32_t version;

	int (*add_listener) (void *object,
			struct spa_hook *listener,
			const struct spa_audio_aec_events *events,
			void *data);

	int (*init) (void *data, const struct spa_dict *args, const struct spa_audio_info_raw *info);
	int (*run) (void *data, const float *rec[], const float *play[], float *out[], uint32_t n_samples);
	int (*set_props) (void *data, const struct spa_dict *args);
};

#define spa_audio_aec_method(o,method,version,...)			\
({									\
	int _res = -ENOTSUP;						\
	struct spa_audio_aec *_o = o;					\
	spa_interface_call_res(&_o->iface,				\
			struct spa_audio_aec_methods, _res,		\
			method, version, ##__VA_ARGS__);		\
	_res;								\
})

#define spa_audio_aec_add_listener(o,...)	spa_audio_aec_method(o, add_listener, 0, __VA_ARGS__)
#define spa_audio_aec_init(o,...)		spa_audio_aec_method(o, init, 0, __VA_ARGS__)
#define spa_audio_aec_run(o,...)		spa_audio_aec_method(o, run, 0, __VA_ARGS__)
#define spa_audio_aec_set_props(o,...)		spa_audio_aec_method(o, set_props, 0, __VA_ARGS__)

#ifdef __cplusplus
}  /* extern "C" */
#endif

#endif /* SPA_AUDIO_AEC_H */
