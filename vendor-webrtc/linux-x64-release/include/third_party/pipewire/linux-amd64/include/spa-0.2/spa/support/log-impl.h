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

#ifndef SPA_LOG_IMPL_H
#define SPA_LOG_IMPL_H

#ifdef __cplusplus
extern "C" {
#endif

#include <stdio.h>

#include <spa/utils/type.h>
#include <spa/support/log.h>

/**
 * \addtogroup spa_log
 * \{
 */

static inline SPA_PRINTF_FUNC(7, 0) void spa_log_impl_logtv(void *object,
				     enum spa_log_level level,
				     const struct spa_log_topic *topic,
				     const char *file,
				     int line,
				     const char *func,
				     const char *fmt,
				     va_list args)
{
	static const char * const levels[] = { "-", "E", "W", "I", "D", "T" };

	const char *basename = strrchr(file, '/');
	char text[512], location[1024];
	char topicstr[32] = {0};

	if (basename)
		basename += 1; /* skip '/' */
	else
		basename = file; /* use whole string if no '/' is found */

	if (topic && topic->topic)
		snprintf(topicstr, sizeof(topicstr), " %s ", topic->topic);
	vsnprintf(text, sizeof(text), fmt, args);
	snprintf(location, sizeof(location), "[%s]%s[%s:%i %s()] %s\n",
		 levels[level],
		 topicstr,
		 basename, line, func, text);
	fputs(location, stderr);
}

static inline SPA_PRINTF_FUNC(7,8) void spa_log_impl_logt(void *object,
				    enum spa_log_level level,
				    const struct spa_log_topic *topic,
				    const char *file,
				    int line,
				    const char *func,
				    const char *fmt, ...)
{
	va_list args;
	va_start(args, fmt);
	spa_log_impl_logtv(object, level, topic, file, line, func, fmt, args);
	va_end(args);
}

static inline SPA_PRINTF_FUNC(6, 0) void spa_log_impl_logv(void *object,
				     enum spa_log_level level,
				     const char *file,
				     int line,
				     const char *func,
				     const char *fmt,
				     va_list args)
{

	spa_log_impl_logtv(object, level, NULL, file, line, func, fmt, args);
}

static inline SPA_PRINTF_FUNC(6,7) void spa_log_impl_log(void *object,
				    enum spa_log_level level,
				    const char *file,
				    int line,
				    const char *func,
				    const char *fmt, ...)
{
	va_list args;
	va_start(args, fmt);
	spa_log_impl_logv(object, level, file, line, func, fmt, args);
	va_end(args);
}

static inline void spa_log_impl_topic_init(void *object, struct spa_log_topic *topic)
{
	/* noop */
}

#define SPA_LOG_IMPL_DEFINE(name)		\
struct {					\
	struct spa_log log;			\
	struct spa_log_methods methods;		\
} name

#define SPA_LOG_IMPL_INIT(name)				\
	{ { { SPA_TYPE_INTERFACE_Log, SPA_VERSION_LOG,	\
	      SPA_CALLBACKS_INIT(&name.methods, &name) },	\
	    SPA_LOG_LEVEL_INFO,	},			\
	  { SPA_VERSION_LOG_METHODS,			\
	    spa_log_impl_log,				\
	    spa_log_impl_logv,				\
	    spa_log_impl_logt,				\
	    spa_log_impl_logtv,				\
	  } }

#define SPA_LOG_IMPL(name)			\
        SPA_LOG_IMPL_DEFINE(name) = SPA_LOG_IMPL_INIT(name)

/**
 * \}
 */

#ifdef __cplusplus
}  /* extern "C" */
#endif
#endif /* SPA_LOG_IMPL_H */
