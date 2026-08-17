// Copyright (c) 2019 nyorain
// Distributed under the Boost Software License, Version 1.0.
// See accompanying file LICENSE or copy at http://www.boost.org/LICENSE_1_0.txt

#define _XOPEN_SOURCE 600
#define _POSIX_C_SOURCE 200809L
#define _WIN32_WINNT 0x0600

// Needed on windows so that we can use sprintf without warning.
#define _CRT_SECURE_NO_WARNINGS

#include <dlg/output.h>
#include <dlg/dlg.h>
#include <wchar.h>
#include <time.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

const char* const dlg_reset_sequence = "\033[0m";
const struct dlg_style dlg_default_output_styles[] = {
	{dlg_text_style_italic, dlg_color_green, dlg_color_none},
	{dlg_text_style_dim, dlg_color_gray, dlg_color_none},
	{dlg_text_style_none, dlg_color_cyan, dlg_color_none},
	{dlg_text_style_none, dlg_color_yellow, dlg_color_none},
	{dlg_text_style_none, dlg_color_red, dlg_color_none},
	{dlg_text_style_bold, dlg_color_red, dlg_color_none}
};

static void* xalloc(size_t size) {
	void* ret = calloc(size, 1);
	if(!ret) fprintf(stderr, "dlg: calloc returned NULL, probably crashing (size: %zu)\n", size);
	return ret;
}

static void* xrealloc(void* ptr, size_t size) {
	void* ret = realloc(ptr, size);
	if(!ret) fprintf(stderr, "dlg: realloc returned NULL, probably crashing (size: %zu)\n", size);
	return ret;
}

struct dlg_tag_func_pair {
	const char* tag;
	const char* func;
};

struct dlg_data {
	const char** tags; // vec
	struct dlg_tag_func_pair* pairs; // vec
	char* buffer;
	size_t buffer_size;
};

static dlg_handler g_handler = dlg_default_output;
static void* g_data = NULL;

static void dlg_free_data(void* data);
static struct dlg_data* dlg_create_data(void);

// platform-specific
#if defined(__unix__) || defined(__unix) || defined(__linux__) || defined(__APPLE__) || defined(__MACH__)
	#define DLG_OS_UNIX
	#include <unistd.h>
	#include <pthread.h>
	#include <sys/time.h>

	static pthread_key_t dlg_data_key;

	static void dlg_main_cleanup(void) {
		void* data = pthread_getspecific(dlg_data_key);
		if(data) {
			dlg_free_data(data);
			pthread_setspecific(dlg_data_key, NULL);
		}
	}

	static void init_data_key(void) {
		pthread_key_create(&dlg_data_key, dlg_free_data);
		atexit(dlg_main_cleanup);
	}

	static struct dlg_data* dlg_data(void) {
		static pthread_once_t key_once = PTHREAD_ONCE_INIT;
		pthread_once(&key_once, init_data_key);

		void* data = pthread_getspecific(dlg_data_key);
		if(!data) {
			data = dlg_create_data();
			pthread_setspecific(dlg_data_key, data);
		}

		return (struct dlg_data*) data;
	}

	static void lock_file(FILE* file) {
		flockfile(file);
	}

	static void unlock_file(FILE* file) {
		funlockfile(file);
	}

	bool dlg_is_tty(FILE* stream) {
		return isatty(fileno(stream));
	}

	static unsigned get_msecs(void) {
		struct timeval tv;
		gettimeofday(&tv, NULL);
		return tv.tv_usec;
	}

// platform switch -- end unix
#elif defined(WIN32) || defined(_WIN32) || defined(_WIN64)
	#define DLG_OS_WIN
	#define WIN32_LEAN_AND_MEAN
	#define DEFINE_CONSOLEV2_PROPERTIES
	#include <windows.h>
	#include <io.h>

	// thanks for nothing, microsoft
	#ifndef ENABLE_VIRTUAL_TERMINAL_PROCESSING
	#define ENABLE_VIRTUAL_TERMINAL_PROCESSING 0x0004
	#endif

	// the max buffer size we will convert on the stack
	#define DLG_MAX_STACK_BUF_SIZE 1024

	static void WINAPI dlg_fls_destructor(void* data) {
		dlg_free_data(data);
	}

	// TODO: error handling
	static BOOL CALLBACK dlg_init_fls(PINIT_ONCE io, void* param, void** lpContext) {
		(void) io;
		(void) param;
		**((DWORD**) lpContext) = FlsAlloc(dlg_fls_destructor);
		return true;
	}

	static struct dlg_data* dlg_data(void) {
		static INIT_ONCE init_once = INIT_ONCE_STATIC_INIT;
		static DWORD fls = 0;
		void* flsp = (void*) &fls;
		InitOnceExecuteOnce(&init_once, dlg_init_fls, NULL, &flsp);
		void* data = FlsGetValue(fls);
		if(!data) {
			data = dlg_create_data();
			FlsSetValue(fls, data);
		}

		return (struct dlg_data*) data;
	}

	static void lock_file(FILE* file) {
		_lock_file(file);
	}

	static void unlock_file(FILE* file) {
		_unlock_file(file);
	}

	bool dlg_is_tty(FILE* stream) {
		return _isatty(_fileno(stream));
	}

#ifdef DLG_WIN_CONSOLE
	static bool init_ansi_console(void) {
		HANDLE out = GetStdHandle(STD_OUTPUT_HANDLE);
		HANDLE err = GetStdHandle(STD_ERROR_HANDLE);
		if(out == INVALID_HANDLE_VALUE || err == INVALID_HANDLE_VALUE)
			return false;

		DWORD outMode, errMode;
		if(!GetConsoleMode(out, &outMode) || !GetConsoleMode(err, &errMode))
		   return false;

		outMode |= ENABLE_VIRTUAL_TERMINAL_PROCESSING;
		errMode |= ENABLE_VIRTUAL_TERMINAL_PROCESSING;
		if(!SetConsoleMode(out, outMode) || !SetConsoleMode(out, errMode))
			return false;

		return true;
	}

	static bool win_write_heap(void* handle, int needed, const char* format, va_list args) {
		char* buf1 = xalloc(3 * needed + 3 + (needed % 2));
		wchar_t* buf2 = (wchar_t*) (buf1 + needed + 1 + (needed % 2));
		vsnprintf(buf1, needed + 1, format, args);
	    needed = MultiByteToWideChar(CP_UTF8, 0, buf1, needed, buf2, needed + 1);
		bool ret = (needed != 0 && WriteConsoleW(handle, buf2, needed, NULL, NULL) != 0);
		free(buf1);
		return ret;
	}

	static bool win_write_stack(void* handle, int needed, const char* format, va_list args) {
		char buf1[DLG_MAX_STACK_BUF_SIZE];
		wchar_t buf2[DLG_MAX_STACK_BUF_SIZE];
		vsnprintf(buf1, needed + 1, format, args);
	    needed = MultiByteToWideChar(CP_UTF8, 0, buf1, needed, buf2, needed + 1);
		return (needed != 0 && WriteConsoleW(handle, buf2, needed, NULL, NULL) != 0);
	}
#endif // DLG_WIN_CONSOLE

	static unsigned get_msecs() {
		SYSTEMTIME st;
		GetSystemTime(&st);
		return st.wMilliseconds;
	}

#else // platform switch -- end windows
	#error Cannot determine platform (needed for color and utf-8 and stuff)
#endif

// general
void dlg_escape_sequence(struct dlg_style style, char buf[12]) {
	int nums[3];
	unsigned int count = 0;

	if(style.fg != dlg_color_none) {
		nums[count++] = style.fg + 30;
	}

	if(style.bg != dlg_color_none) {
		nums[count++] = style.fg + 40;
	}

	if(style.style != dlg_text_style_none) {
		nums[count++] = style.style;
	}

	switch(count) {
		case 1: snprintf(buf, 12, "\033[%dm", nums[0]); break;
		case 2: snprintf(buf, 12, "\033[%d;%dm", nums[0], nums[1]); break;
		case 3: snprintf(buf, 12, "\033[%d;%d;%dm", nums[0], nums[1], nums[2]); break;
		default: buf[0] = '\0'; break;
	}
}

int dlg_vfprintf(FILE* stream, const char* format, va_list args) {
#if defined(DLG_OS_WIN) && defined(DLG_WIN_CONSOLE)
	void* handle = NULL;
	if(stream == stdout) {
		handle = GetStdHandle(STD_OUTPUT_HANDLE);
	} else if(stream == stderr) {
		handle = GetStdHandle(STD_ERROR_HANDLE);
	}

	if(handle) {
		va_list args_copy;
		va_copy(args_copy, args);
		int needed = vsnprintf(NULL, 0, format, args_copy);
		va_end(args_copy);

		if(needed < 0) {
			return needed;
		}

		// We don't allocate too much on the stack
		// but we also don't want to call alloc every logging call
		// or use another cached buffer
		if(needed >= DLG_MAX_STACK_BUF_SIZE) {
			if(win_write_heap(handle, needed, format, args)) {
				return needed;
			}
		} else {
			if(win_write_stack(handle, needed, format, args)) {
				return needed;
			}
		}
	}
#endif

	return vfprintf(stream, format, args);
}

int dlg_fprintf(FILE* stream, const char* format, ...) {
	va_list args;
	va_start(args, format);
	int ret = dlg_vfprintf(stream, format, args);
	va_end(args);
	return ret;
}

int dlg_styled_fprintf(FILE* stream, struct dlg_style style, const char* format, ...) {
	char buf[12];
	dlg_escape_sequence(style, buf);

	fprintf(stream, "%s", buf);
	va_list args;
	va_start(args, format);
	int ret = dlg_vfprintf(stream, format, args);
	va_end(args);
	fprintf(stream, "%s", dlg_reset_sequence);
	return ret;
}

void dlg_generic_output(dlg_generic_output_handler output, void* data,
		unsigned int features, const struct dlg_origin* origin, const char* string,
		const struct dlg_style styles[6]) {
	// We never print any dynamic content below so we can be sure at compile
	// time that a buffer of size 64 is large enough.
	char format_buf[64];
	char* format = format_buf;

	if(features & dlg_output_style) {
		format += sprintf(format, "%%s");
	}

	if(features & (dlg_output_time | dlg_output_file_line | dlg_output_tags | dlg_output_func)) {
		format += sprintf(format, "[");
	}

	bool first_meta = true;
	if(features & dlg_output_time) {
		format += sprintf(format, "%%h");
		first_meta = false;
	}

	if(features & dlg_output_time_msecs) {
		if(!first_meta) {
			format += sprintf(format, ":");
		}

		format += sprintf(format, "%%m");
		first_meta = false;
	}

	if(features & dlg_output_file_line) {
		if(!first_meta) {
			format += sprintf(format, " ");
		}

		format += sprintf(format, "%%o");
		first_meta = false;
	}

	if(features & dlg_output_func) {
		if(!first_meta) {
			format += sprintf(format, " ");
		}

		format += sprintf(format, "%%f");
		first_meta = false;
	}

	if(features & dlg_output_tags) {
		if(!first_meta) {
			format += sprintf(format, " ");
		}

		format += sprintf(format, "{%%t}");
		first_meta = false;
	}

	if(features & (dlg_output_time | dlg_output_file_line | dlg_output_tags | dlg_output_func)) {
		format += sprintf(format, "] ");
	}

	format += sprintf(format, "%%c");

	if(features & dlg_output_newline) {
		format += sprintf(format, "\n");
	}

	*format = '\0';
	dlg_generic_outputf(output, data, format_buf, origin, string, styles);
}

void dlg_generic_outputf(dlg_generic_output_handler output, void* data,
		const char* format_string, const struct dlg_origin* origin, const char* string,
		const struct dlg_style styles[6]) {
	bool reset_style = false;
	for(const char* it = format_string; *it; it++) {
		if(*it != '%') {
			output(data, "%c", *it);
			continue;
		}

		char next = *(it + 1); // must be valid since *it is not '\0'
		if(next == 'h') {
			time_t t = time(NULL);
			struct tm tm_info;

	#ifdef DLG_OS_WIN
			if(localtime_s(&tm_info, &t)) {
	#else
			if(!localtime_r(&t, &tm_info)) {
	#endif
				output(data, "<DATE ERROR>");
			} else {
				char timebuf[32];
				strftime(timebuf, sizeof(timebuf), "%H:%M:%S", &tm_info);
				output(data, "%s", timebuf);
			}
			it++;
		} else if(next == 'm') {
			output(data, "%06d", get_msecs());
			it++;
		} else if(next == 't') {
			bool first_tag = true;
			for(const char** tags = origin->tags; *tags; ++tags) {
				if(!first_tag) {
					output(data, ", ");
				}

				output(data, "%s", *tags);
				first_tag = false;
			}
			++it;
		} else if(next == 'f') {
			output(data, "%s", origin->func);
			++it;
		} else if(next == 'o') {
			output(data, "%s:%u", origin->file, origin->line);
			++it;
		} else if(next == 's') {
			char buf[12];
			dlg_escape_sequence(styles[origin->level], buf);
			output(data, "%s", buf);
			reset_style = true;
			++it;
		} else if(next == 'r') {
			output(data, "%s", dlg_reset_sequence);
			reset_style = false;
			++it;
		} else if(next == 'c') {
			if(origin->expr && string) {
				output(data, "assertion '%s' failed: '%s'", origin->expr, string);
			} else if(origin->expr) {
				output(data, "assertion '%s' failed", origin->expr);
			} else if(string) {
				output(data, "%s", string);
			}
			++it;
		} else if(next == '%') {
			output(data, "%s", "%");
			++it;
		} else {
			// in this case it's a '%' without known format specifier following
			output(data, "%s", "%");
		}
	}

	if(reset_style) {
		output(data, "%s", dlg_reset_sequence);
	}
}

struct buf {
	char* buf;
	size_t* size;
};

static void print_size(void* size, const char* format, ...) {
	va_list args;
	va_start(args, format);

	int ret = vsnprintf(NULL, 0, format, args);
	va_end(args);

	if(ret > 0) {
		*((size_t*) size) += ret;
	}
}

static void print_buf(void* dbuf, const char* format, ...) {
	struct buf* buf = (struct buf*) dbuf;
	va_list args;
	va_start(args, format);

	int printed = vsnprintf(buf->buf, *buf->size, format, args);
	va_end(args);

	if(printed > 0) {
		*buf->size -= printed;
		buf->buf += printed;
	}
}

void dlg_generic_output_buf(char* buf, size_t* size, unsigned int features,
		const struct dlg_origin* origin, const char* string,
		const struct dlg_style styles[6]) {
	if(buf) {
		struct buf mbuf;
		mbuf.buf = buf;
		mbuf.size = size;
		dlg_generic_output(print_buf, &mbuf, features, origin, string, styles);
	} else {
		*size = 0;
		dlg_generic_output(print_size, size, features, origin, string, styles);
	}
}

void dlg_generic_outputf_buf(char* buf, size_t* size, const char* format_string,
		const struct dlg_origin* origin, const char* string,
		const struct dlg_style styles[6]) {
	if(buf) {
		struct buf mbuf;
		mbuf.buf = buf;
		mbuf.size = size;
		dlg_generic_outputf(print_buf, &mbuf, format_string, origin, string, styles);
	} else {
		*size = 0;
		dlg_generic_outputf(print_size, size, format_string, origin, string, styles);
	}
}

static void print_stream(void* stream, const char* format, ...) {
	va_list args;
	va_start(args, format);
	dlg_vfprintf((FILE*) stream, format, args);
	va_end(args);
}

void dlg_generic_output_stream(FILE* stream, unsigned int features,
		const struct dlg_origin* origin, const char* string,
		const struct dlg_style styles[6]) {
	stream = stream ? stream : stdout;
	if(features & dlg_output_threadsafe) {
		lock_file(stream);
	}

	dlg_generic_output(print_stream, stream, features, origin, string, styles);
	if(features & dlg_output_threadsafe) {
		unlock_file(stream);
	}
}

void dlg_generic_outputf_stream(FILE* stream, const char* format_string,
		const struct dlg_origin* origin, const char* string,
		const struct dlg_style styles[6], bool lock_stream) {
	stream = stream ? stream : stdout;
	if(lock_stream) {
		lock_file(stream);
	}

	dlg_generic_outputf(print_stream, stream, format_string, origin, string, styles);
	if(lock_stream) {
		unlock_file(stream);
	}
}

void dlg_default_output(const struct dlg_origin* origin, const char* string, void* data) {
	FILE* stream = data ? (FILE*) data : stdout;
	unsigned int features = dlg_output_file_line |
		dlg_output_newline |
		dlg_output_threadsafe;

#ifdef DLG_DEFAULT_OUTPUT_ALWAYS_COLOR
	dlg_win_init_ansi();
	features |= dlg_output_style;
#else
	if(dlg_is_tty(stream) && dlg_win_init_ansi()) {
		features |= dlg_output_style;
	}
#endif

	dlg_generic_output_stream(stream, features, origin, string, dlg_default_output_styles);
	fflush(stream);
}

bool dlg_win_init_ansi(void) {
#if defined(DLG_OS_WIN) && defined(DLG_WIN_CONSOLE)
	// TODO: use init once
	static volatile LONG status = 0;
	LONG res = InterlockedCompareExchange(&status, 1, 0);
	if(res == 0) { // not initialized
		InterlockedExchange(&status, 3 + init_ansi_console());
	}

	while(status == 1); // currently initialized in another thread, spinlock
	return (status == 4);
#else
	return true;
#endif
}

// small dynamic vec/array implementation
// Since the macros vec_init and vec_add[c]/vec_push might
// change the pointers value it must not be referenced somewhere else.
#define vec__raw(vec) (((unsigned int*) vec) - 2)

static void* vec_do_create(unsigned int typesize, unsigned int cap, unsigned int size) {
	unsigned long a = (size > cap) ? size : cap;
	void* ptr = xalloc(2 * sizeof(unsigned int) + a * typesize);
	unsigned int* begin = (unsigned int*) ptr;
	begin[0] = size * typesize;
	begin[1] = a * typesize;
	return begin + 2;
}

// NOTE: can be more efficient if we are allowed to reorder vector
static void vec_do_erase(void* vec, unsigned int pos, unsigned int size) {
	unsigned int* begin = vec__raw(vec);
	begin[0] -= size;
	char* buf = (char*) vec;
	memcpy(buf + pos, buf + pos + size, size);
}

static void* vec_do_add(void** vec, unsigned int size) {
	unsigned int* begin = vec__raw(*vec);
	unsigned int needed = begin[0] + size;
	if(needed >= begin[1]) {
		void* ptr = xrealloc(begin, sizeof(unsigned int) * 2 + needed * 2);
		begin = (unsigned int*) ptr;
		begin[1] = needed * 2;
		(*vec) = begin + 2;
	}

	void* ptr = ((char*) (*vec)) + begin[0];
	begin[0] += size;
	return ptr;
}

#define vec_create(type, size) (type*) vec_do_create(sizeof(type), size * 2, size)
#define vec_create_reserve(type, size, capacity) (type*) vec_do_create(sizeof(type), capcity, size)
#define vec_init(array, size) array = vec_do_create(sizeof(*array), size * 2, size)
#define vec_init_reserve(array, size, capacity) *((void**) &array) = vec_do_create(sizeof(*array), capacity, size)
#define vec_free(vec) (free((vec) ? vec__raw(vec) : NULL), vec = NULL)
#define vec_erase_range(vec, pos, count) vec_do_erase(vec, pos * sizeof(*vec), count * sizeof(*vec))
#define vec_erase(vec, pos) vec_do_erase(vec, pos * sizeof(*vec), sizeof(*vec))
#define vec_size(vec) (vec__raw(vec)[0] / sizeof(*vec))
#define vec_capacity(vec) (vec_raw(vec)[1] / sizeof(*vec))
#define vec_add(vec) vec_do_add((void**) &vec, sizeof(*vec))
#define vec_addc(vec, count) (vec_do_add((void**) &vec, sizeof(*vec) * count))
#define vec_push(vec, value) (vec_do_add((void**) &vec, sizeof(*vec)), vec_last(vec) = (value))
#define vec_pop(vec) (vec__raw(vec)[0] -= sizeof(*vec))
#define vec_popc(vec, count) (vec__raw(vec)[0] -= sizeof(*vec) * count)
#define vec_clear(vec) (vec__raw(vec)[0] = 0)
#define vec_last(vec) (vec[vec_size(vec) - 1])

static struct dlg_data* dlg_create_data(void) {
	struct dlg_data* data = (struct dlg_data*) xalloc(sizeof(struct dlg_data));
	vec_init_reserve(data->tags, 0, 20);
	vec_init_reserve(data->pairs, 0, 20);
	data->buffer_size = 100;
	data->buffer = (char*) xalloc(data->buffer_size);
	return data;
}

static void dlg_free_data(void* ddata) {
	struct dlg_data* data = (struct dlg_data*) ddata;
	if(data) {
		vec_free(data->pairs);
		vec_free(data->tags);
		free(data->buffer);
		free(data);
	}
}

void dlg_add_tag(const char* tag, const char* func) {
	struct dlg_data* data = dlg_data();
	struct dlg_tag_func_pair* pair =
		(struct dlg_tag_func_pair*) vec_add(data->pairs);
	pair->tag = tag;
	pair->func = func;
}

bool dlg_remove_tag(const char* tag, const char* func) {
	struct dlg_data* data = dlg_data();
	for(unsigned int i = 0; i < vec_size(data->pairs); ++i) {
		if(data->pairs[i].func == func && data->pairs[i].tag == tag) {
			vec_erase(data->pairs, i);
			return true;
		}
	}

	return false;
}

char** dlg_thread_buffer(size_t** size) {
	struct dlg_data* data = dlg_data();
	if(size) {
		*size = &data->buffer_size;
	}
	return &data->buffer;
}

void dlg_set_handler(dlg_handler handler, void* data) {
	g_handler = handler;
	g_data = data;
}

dlg_handler dlg_get_handler(void** data) {
	*data = g_data;
	return g_handler;
}

const char* dlg__printf_format(const char* str, ...) {
	va_list vlist;
	va_start(vlist, str);

	va_list vlistcopy;
	va_copy(vlistcopy, vlist);
	int needed = vsnprintf(NULL, 0, str, vlist);
	if(needed < 0) {
		printf("dlg__printf_format: invalid format given\n");
		va_end(vlist);
		va_end(vlistcopy);
		return NULL;
	}

	va_end(vlist);

	size_t* buf_size;
	char** buf = dlg_thread_buffer(&buf_size);
	if(*buf_size <= (unsigned int) needed) {
		*buf_size = (needed + 1) * 2;
		*buf = (char*) xrealloc(*buf, *buf_size);
	}

	vsnprintf(*buf, *buf_size, str, vlistcopy);
	va_end(vlistcopy);

	return *buf;
}

void dlg__do_log(enum dlg_level lvl, const char* const* tags, const char* file, int line,
		const char* func, const char* string, const char* expr) {
	struct dlg_data* data = dlg_data();
	unsigned int tag_count = 0;

	// push default tags
	while(tags[tag_count]) {
		vec_push(data->tags, tags[tag_count++]);
	}

	// push current global tags
	for(size_t i = 0; i < vec_size(data->pairs); ++i) {
		const struct dlg_tag_func_pair pair = data->pairs[i];
		if(pair.func == NULL || !strcmp(pair.func, func)) {
			vec_push(data->tags, pair.tag);
		}
	}

	// push call-specific tags, skip first terminating NULL
	++tag_count;
	while(tags[tag_count]) {
		vec_push(data->tags, tags[tag_count++]);
	}

	vec_push(data->tags, NULL); // terminating NULL
	struct dlg_origin origin;
	origin.level = lvl;
	origin.file = file;
	origin.line = line;
	origin.func = func;
	origin.expr = expr;
	origin.tags = data->tags;

	g_handler(&origin, string, g_data);
	vec_clear(data->tags);
}

#ifdef _MSC_VER
// shitty msvc compatbility
// meson gives us sane paths (separated by '/') while on MSVC,
// __FILE__ contains a '\\' separator.
static bool path_same(char a, char b) {
	return (a == b) ||
		(a == '/' && b == '\\') ||
		(a == '\\' && b == '/');
}
#else

static inline bool path_same(char a, char b) {
	return a == b;
}

#endif

const char* dlg__strip_root_path(const char* file, const char* base) {
	if(!file) {
		return NULL;
	}

	const char* saved = file;
	if(*file == '.') { // relative path detected
		while(*(++file) == '.' || *file == '/' || *file == '\\');
		if(*file == '\0') { // weird case: purely relative path without file
			return saved;
		}

		return file;
	}

	// strip base from file if it is given
	if(base) {
		char fn = *file;
		char bn = *base;
		while(bn != '\0' && path_same(fn, bn)) {
			fn = *(++file);
			bn = *(++base);
		}

		if(fn == '\0' || bn != '\0') { // weird case: base isn't prefix of file
			return saved;
		}
	}

	return file;
}
