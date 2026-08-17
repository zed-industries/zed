// Copyright (c) 2019 nyorain
// Distributed under the Boost Software License, Version 1.0.
// See accompanying file LICENSE or copy at http://www.boost.org/LICENSE_1_0.txt

#ifndef INC_DLG_DLG_H_
#define INC_DLG_DLG_H_

#include <stdbool.h>
#include <stddef.h>
#include <stdlib.h>
#include <stdarg.h>
#include <stdio.h>

// Hosted at https://github.com/nyorain/dlg.
// There are examples and documentation.
// Issue reports and contributions appreciated.

// - CONFIG -
// Define this macro to make all dlg macros have no effect at all
// #define DLG_DISABLE

// the log/assertion levels below which logs/assertions are ignored
// defaulted depending on the NDEBUG macro
#ifndef DLG_LOG_LEVEL
	#ifdef NDEBUG
		#define DLG_LOG_LEVEL dlg_level_warn
	#else
		#define DLG_LOG_LEVEL dlg_level_trace
	#endif
#endif

#ifndef DLG_ASSERT_LEVEL
	#ifdef NDEBUG
		#define DLG_ASSERT_LEVEL dlg_level_warn
	#else
		#define DLG_ASSERT_LEVEL dlg_level_trace
	#endif
#endif

// the assert level of dlg_assert
#ifndef DLG_DEFAULT_ASSERT
	#define DLG_DEFAULT_ASSERT dlg_level_error
#endif

// evaluated to the 'file' member in dlg_origin
#ifndef DLG_FILE
	#define DLG_FILE dlg__strip_root_path(__FILE__, DLG_BASE_PATH)

	// the base path stripped from __FILE__. If you don't override DLG_FILE set this to
	// the project root to make 'main.c' from '/some/bullshit/main.c'
	#ifndef DLG_BASE_PATH
		#define DLG_BASE_PATH ""
	#endif
#endif

// Default tags applied to all logs/assertions (in the defining file).
// Must be in format ```#define DLG_DEFAULT_TAGS "tag1", "tag2"```
// or just nothing (as defaulted here)
#ifndef DLG_DEFAULT_TAGS
	#define DLG_DEFAULT_TAGS_TERM NULL
#else
	#define DLG_DEFAULT_TAGS_TERM DLG_DEFAULT_TAGS, NULL
#endif

// The function used for formatting. Can have any signature, but must be callable with
// the arguments the log/assertions macros are called with. Must return a const char*
// that will not be freed by dlg, the formatting function must keep track of it.
// The formatting function might use dlg_thread_buffer or a custom owned buffer.
// The returned const char* has to be valid until the dlg log/assertion ends.
// Usually a c function with ... (i.e. using va_list) or a variadic c++ template do
// allow formatting.
#ifndef DLG_FMT_FUNC
	#define DLG_FMT_FUNC dlg__printf_format
#endif

// Only overwrite (i.e. predefine) this if you know what you are doing.
// On windows this is used to add the dllimport specified.
// If you are using the static version of dlg (on windows) define
// DLG_STATIC before including dlg.h
#ifndef DLG_API
 	#if (defined(_WIN32) || defined(__CYGWIN__)) && !defined(DLG_STATIC)
		#define DLG_API __declspec(dllimport)
	#else
		#define DLG_API
	#endif
#endif

// This macro is used when an assertion fails. It gets the source expression
// and can return an alternative (that must stay alive).
// Mainly useful to execute something on failed assertion.
#ifndef DLG_FAILED_ASSERTION_TEXT
	#define DLG_FAILED_ASSERTION_TEXT(x) x
#endif

// - utility -
// two methods needed since cplusplus does not support compound literals
// and c does not support uniform initialization/initializer lists
#ifdef __cplusplus
	#include <initializer_list>
	#define DLG_CREATE_TAGS(...) std::initializer_list<const char*> \
		{DLG_DEFAULT_TAGS_TERM, __VA_ARGS__, NULL}.begin()
#else
	#define DLG_CREATE_TAGS(...) (const char* const[]) {DLG_DEFAULT_TAGS_TERM, __VA_ARGS__, NULL}
#endif

#ifdef __GNUC__
	#define DLG_PRINTF_ATTRIB(a, b) __attribute__ ((format (printf, a, b)))
#else
	#define DLG_PRINTF_ATTRIB(a, b)
#endif

#ifdef __cplusplus
extern "C" {
#endif


// Represents the importance of a log/assertion call.
enum dlg_level {
	dlg_level_trace = 0, // temporary used debug, e.g. to check if control reaches function
	dlg_level_debug, // general debugging, prints e.g. all major events
	dlg_level_info, // general useful information
	dlg_level_warn, // warning, something went wrong but might have no (really bad) side effect
	dlg_level_error, // something really went wrong; expect serious issues
	dlg_level_fatal // critical error; application is likely to crash/exit
};

// Holds various information associated with a log/assertion call.
// Forwarded to the output handler.
struct dlg_origin {
	const char* file;
	unsigned int line;
	const char* func;
	enum dlg_level level;
	const char** tags; // null-terminated
	const char* expr; // assertion expression, otherwise null
};

// Type of the output handler, see dlg_set_handler.
typedef void(*dlg_handler)(const struct dlg_origin* origin, const char* string, void* data);

#ifndef DLG_DISABLE
	// Tagged/Untagged logging with variable level
	// Tags must always be in the format `("tag1", "tag2")` (including brackets)
	// Example usages:
	//   dlg_log(dlg_level_warning, "test 1")
	//   dlg_logt(("tag1, "tag2"), dlg_level_debug, "test %d", 2)
	#define dlg_log(level, ...) if(level >= DLG_LOG_LEVEL) \
		dlg__do_log(level, DLG_CREATE_TAGS(NULL), DLG_FILE, __LINE__, __func__,  \
		DLG_FMT_FUNC(__VA_ARGS__), NULL)
	#define dlg_logt(level, tags, ...) if(level >= DLG_LOG_LEVEL) \
		dlg__do_log(level, DLG_CREATE_TAGS tags, DLG_FILE, __LINE__, __func__, \
		DLG_FMT_FUNC(__VA_ARGS__), NULL)

	// Dynamic level assert macros in various versions for additional arguments
	// Example usages:
	//   dlg_assertl(dlg_level_warning, data != nullptr);
	//   dlg_assertlt(("tag1, "tag2"), dlg_level_trace, data != nullptr);
	//   dlg_asserttlm(("tag1), dlg_level_warning, data != nullptr, "Data must not be null");
	//   dlg_assertlm(dlg_level_error, data != nullptr, "Data must not be null");
	#define dlg_assertl(level, expr) if(level >= DLG_ASSERT_LEVEL && !(expr)) \
		dlg__do_log(level, DLG_CREATE_TAGS(NULL), DLG_FILE, __LINE__, __func__, NULL, \
			DLG_FAILED_ASSERTION_TEXT(#expr))
	#define dlg_assertlt(level, tags, expr) if(level >= DLG_ASSERT_LEVEL && !(expr)) \
		dlg__do_log(level, DLG_CREATE_TAGS tags, DLG_FILE, __LINE__, __func__, NULL, \
			DLG_FAILED_ASSERTION_TEXT(#expr))
	#define dlg_assertlm(level, expr, ...) if(level >= DLG_ASSERT_LEVEL && !(expr)) \
		dlg__do_log(level, DLG_CREATE_TAGS(NULL), DLG_FILE, __LINE__, __func__,  \
			DLG_FMT_FUNC(__VA_ARGS__), DLG_FAILED_ASSERTION_TEXT(#expr))
	#define dlg_assertltm(level, tags, expr, ...) if(level >= DLG_ASSERT_LEVEL && !(expr)) \
		dlg__do_log(level, DLG_CREATE_TAGS tags, DLG_FILE, __LINE__,  \
			__func__, DLG_FMT_FUNC(__VA_ARGS__), DLG_FAILED_ASSERTION_TEXT(#expr))

	#define dlg__assert_or(level, tags, expr, code, msg) if(!(expr)) {\
			if(level >= DLG_ASSERT_LEVEL) \
				dlg__do_log(level, tags, DLG_FILE, __LINE__, __func__, msg, \
					DLG_FAILED_ASSERTION_TEXT(#expr)); \
			code; \
		} (void) NULL

	// - Private interface: not part of the abi/api but needed in macros -
	// Formats the given format string and arguments as printf would, uses the thread buffer.
	DLG_API const char* dlg__printf_format(const char* format, ...) DLG_PRINTF_ATTRIB(1, 2);
	DLG_API void dlg__do_log(enum dlg_level lvl, const char* const*, const char*, int,
		const char*, const char*, const char*);
	DLG_API const char* dlg__strip_root_path(const char* file, const char* base);

#else // DLG_DISABLE

	#define dlg_log(level, ...)
	#define dlg_logt(level, tags, ...)

	#define dlg_assertl(level, expr) // assert without tags/message
	#define dlg_assertlt(level, tags, expr) // assert with tags
	#define dlg_assertlm(level, expr, ...) // assert with message
	#define dlg_assertltm(level, tags, expr, ...) // assert with tags & message

	#define dlg__assert_or(level, tags, expr, code, msg) if(!(expr)) { code; } (void) NULL
#endif // DLG_DISABLE

// The API below is independent from DLG_DISABLE

// Sets the handler that is responsible for formatting and outputting log calls.
// This function is not thread safe and the handler is set globally.
// The handler itself must not change dlg tags or call a dlg macro (if it
// does so, the provided string or tags array in 'origin' might get invalid).
// The handler can also be used for various other things such as dealing
// with failed assertions or filtering calls based on the passed tags.
// The default handler is dlg_default_output (see its doc for more info).
// If using c++ make sure the registered handler cannot throw e.g. by
// wrapping everything into a try-catch blog.
DLG_API void dlg_set_handler(dlg_handler handler, void* data);

// The default output handler.
// Only use this to reset the output handler, prefer to use
// dlg_generic_output (from output.h) which this function simply calls.
// It also flushes the stream used and correctly outputs even from multiple threads.
DLG_API void dlg_default_output(const struct dlg_origin*, const char* string, void*);

// Returns the currently active dlg handler and sets `data` to
// its user data pointer. `data` must not be NULL.
// Useful to create handler chains.
// This function is not threadsafe, i.e. retrieving the handler while
// changing it from another thread is unsafe.
// See `dlg_set_handler`.
DLG_API dlg_handler dlg_get_handler(void** data);

// Adds the given tag associated with the given function to the thread specific list.
// If func is not NULL the tag will only applied to calls from the same function.
// Remove the tag again calling dlg_remove_tag (with exactly the same pointers!).
// Does not check if the tag is already present.
DLG_API void dlg_add_tag(const char* tag, const char* func);

// Removes a tag added with dlg_add_tag (has no effect for tags no present).
// The pointers must be exactly the same pointers that were supplied to dlg_add_tag,
// this function will not check using strcmp. When the same tag/func combination
// is added multiple times, this function remove exactly one candidate, it is
// undefined which. Returns whether a tag was found (and removed).
DLG_API bool dlg_remove_tag(const char* tag, const char* func);

// Returns the thread-specific buffer and its size for dlg.
// The buffer should only be used by formatting functions.
// The buffer can be reallocated and the size changed, just make sure
// to update both values correctly.
DLG_API char** dlg_thread_buffer(size_t** size);

// Untagged leveled logging
#define dlg_trace(...) dlg_log(dlg_level_trace, __VA_ARGS__)
#define dlg_debug(...) dlg_log(dlg_level_debug, __VA_ARGS__)
#define dlg_info(...) dlg_log(dlg_level_info, __VA_ARGS__)
#define dlg_warn(...) dlg_log(dlg_level_warn, __VA_ARGS__)
#define dlg_error(...) dlg_log(dlg_level_error, __VA_ARGS__)
#define dlg_fatal(...) dlg_log(dlg_level_fatal, __VA_ARGS__)

// Tagged leveled logging
#define dlg_tracet(tags, ...) dlg_logt(dlg_level_trace, tags, __VA_ARGS__)
#define dlg_debugt(tags, ...) dlg_logt(dlg_level_debug, tags, __VA_ARGS__)
#define dlg_infot(tags, ...) dlg_logt(dlg_level_info, tags, __VA_ARGS__)
#define dlg_warnt(tags, ...) dlg_logt(dlg_level_warn, tags, __VA_ARGS__)
#define dlg_errort(tags, ...) dlg_logt(dlg_level_error, tags, __VA_ARGS__)
#define dlg_fatalt(tags, ...) dlg_logt(dlg_level_fatal, tags, __VA_ARGS__)

// Assert macros useing DLG_DEFAULT_ASSERT as level
#define dlg_assert(expr) dlg_assertl(DLG_DEFAULT_ASSERT, expr)
#define dlg_assertt(tags, expr) dlg_assertlt(DLG_DEFAULT_ASSERT, tags, expr)
#define dlg_assertm(expr, ...) dlg_assertlm(DLG_DEFAULT_ASSERT, expr, __VA_ARGS__)
#define dlg_asserttm(tags, expr, ...) dlg_assertltm(DLG_DEFAULT_ASSERT, tags, expr, __VA_ARGS__)

// If (expr) does not evaluate to true, always executes 'code' (no matter what
// DLG_ASSERT_LEVEL is or if dlg is disabled or not).
// When dlg is enabled and the level is greater or equal to DLG_ASSERT_LEVEL,
// logs the failed assertion.
// Example usages:
//   dlg_assertl_or(dlg_level_warn, data != nullptr, return);
//   dlg_assertlm_or(dlg_level_fatal, data != nullptr, return, "Data must not be null");
//   dlg_assert_or(data != nullptr, logError(); return false);
#define dlg_assertltm_or(level, tags, expr, code, ...) dlg__assert_or(level, \
		DLG_CREATE_TAGS tags, expr, code, DLG_FMT_FUNC(__VA_ARGS__))
#define dlg_assertlm_or(level, expr, code, ...) dlg__assert_or(level, \
		DLG_CREATE_TAGS(NULL), expr, code, DLG_FMT_FUNC(__VA_ARGS__))
#define dlg_assertl_or(level, expr, code) dlg__assert_or(level, \
		DLG_CREATE_TAGS(NULL), expr, code, NULL)

#define dlg_assert_or(expr, code) dlg_assertl_or(DLG_DEFAULT_ASSERT, expr, code)
#define dlg_assertm_or(expr, code, ...) dlg_assertlm_or(DLG_DEFAULT_ASSERT, expr, code, __VA_ARGS__)

#ifdef __cplusplus
}
#endif

#endif // header guard
