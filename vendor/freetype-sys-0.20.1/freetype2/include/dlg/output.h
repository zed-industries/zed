// Copyright (c) 2019 nyorain
// Distributed under the Boost Software License, Version 1.0.
// See accompanying file LICENSE or copy at http://www.boost.org/LICENSE_1_0.txt

#ifndef INC_DLG_OUTPUT_H_
#define INC_DLG_OUTPUT_H_

#include <dlg/dlg.h>
#include <stdio.h>

#ifdef __cplusplus
extern "C" {
#endif

// Text style
enum dlg_text_style {
	dlg_text_style_reset     = 0,
	dlg_text_style_bold      = 1,
	dlg_text_style_dim       = 2,
	dlg_text_style_italic    = 3,
	dlg_text_style_underline = 4,
	dlg_text_style_blink     = 5,
	dlg_text_style_rblink    = 6,
	dlg_text_style_reversed  = 7,
	dlg_text_style_conceal   = 8,
	dlg_text_style_crossed   = 9,
	dlg_text_style_none,
};

// Text color
enum dlg_color {
	dlg_color_black = 0,
	dlg_color_red,
	dlg_color_green,
	dlg_color_yellow,
	dlg_color_blue,
	dlg_color_magenta,
	dlg_color_cyan,
	dlg_color_gray,
	dlg_color_reset = 9,

	dlg_color_black2 = 60,
	dlg_color_red2,
	dlg_color_green2,
	dlg_color_yellow2,
	dlg_color_blue2,
	dlg_color_magenta2,
	dlg_color_cyan2,
	dlg_color_gray2,

	dlg_color_none = 69,
};

struct dlg_style {
	enum dlg_text_style style;
	enum dlg_color fg;
	enum dlg_color bg;
};

// Like fprintf but fixes utf-8 output to console on windows.
// On non-windows sytems just uses the corresponding standard library
// functions. On windows, if dlg was compiled with the win_console option,
// will first try to output it in a way that allows the default console
// to display utf-8. If that fails, will fall back to the standard
// library functions.
DLG_API int dlg_fprintf(FILE* stream, const char* format, ...) DLG_PRINTF_ATTRIB(2, 3);
DLG_API int dlg_vfprintf(FILE* stream, const char* format, va_list list);

// Like dlg_printf, but also applies the given style to this output.
// The style will always be applied (using escape sequences), independent of the given stream.
// On windows escape sequences don't work out of the box, see dlg_win_init_ansi().
DLG_API int dlg_styled_fprintf(FILE* stream, struct dlg_style style,
	const char* format, ...) DLG_PRINTF_ATTRIB(3, 4);

// Features to output from the generic output handler.
// Some features might have only an effect in the specializations.
enum dlg_output_feature {
	dlg_output_tags = 1, // output tags list
	dlg_output_time = 2, // output time of log call (hour:minute:second)
	dlg_output_style = 4, // whether to use the supplied styles
	dlg_output_func = 8, // output function
	dlg_output_file_line = 16, // output file:line,
	dlg_output_newline = 32, // output a newline at the end
	dlg_output_threadsafe = 64, // locks stream before printing
	dlg_output_time_msecs = 128 // output micro seconds (ms on windows)
};

// The default level-dependent output styles. The array values represent the styles
// to be used for the associated level (i.e. [0] for trace level).
DLG_API extern const struct dlg_style dlg_default_output_styles[6];

// Generic output function. Used by the default output handler and might be useful
// for custom output handlers (that don't want to manually format the output).
// Will call the given output func with the given data (and format + args to print)
// for everything it has to print in printf format.
// See also the *_stream and *_buf specializations for common usage.
// The given output function must not be NULL.
typedef void(*dlg_generic_output_handler)(void* data, const char* format, ...);
DLG_API void dlg_generic_output(dlg_generic_output_handler output, void* data,
		unsigned int features, const struct dlg_origin* origin, const char* string,
		const struct dlg_style styles[6]);

// Generic output function, using a format string instead of feature flags.
// Use following conversion characters:
// %h - output the time in H:M:S format
// %m - output the time in milliseconds
// %t - output the full list of tags, comma separated
// %f - output the function name noted in the origin
// %o - output the file:line of the origin
// %s - print the appropriate style escape sequence.
// %r - print the escape sequence to reset the style.
// %c - The content of the log/assert
// %% - print the '%' character
// Only the above specified conversion characters are valid, the rest are
// written as it is.
DLG_API void dlg_generic_outputf(dlg_generic_output_handler output, void* data,
		const char* format_string, const struct dlg_origin* origin,
		const char* string, const struct dlg_style styles[6]);

// Generic output function. Used by the default output handler and might be useful
// for custom output handlers (that don't want to manually format the output).
// If stream is NULL uses stdout.
// Automatically uses dlg_fprintf to assure correct utf-8 even on windows consoles.
// Locks the stream (i.e. assures threadsafe access) when the associated feature
// is passed (note that stdout/stderr might still mix from multiple threads).
DLG_API void dlg_generic_output_stream(FILE* stream, unsigned int features,
	const struct dlg_origin* origin, const char* string,
	const struct dlg_style styles[6]);
DLG_API void dlg_generic_outputf_stream(FILE* stream, const char* format_string,
	const struct dlg_origin* origin, const char* string,
	const struct dlg_style styles[6], bool lock_stream);

// Generic output function (see dlg_generic_output) that uses a buffer instead of
// a stream. buf must at least point to *size bytes. Will set *size to the number
// of bytes written (capped to the given size), if buf == NULL will set *size
// to the needed size. The size parameter must not be NULL.
DLG_API void dlg_generic_output_buf(char* buf, size_t* size, unsigned int features,
	const struct dlg_origin* origin, const char* string,
	const struct dlg_style styles[6]);
DLG_API void dlg_generic_outputf_buf(char* buf, size_t* size, const char* format_string,
	const struct dlg_origin* origin, const char* string,
	const struct dlg_style styles[6]);

// Returns if the given stream is a tty. Useful for custom output handlers
// e.g. to determine whether to use color.
// NOTE: Due to windows limitations currently returns false for wsl ttys.
DLG_API bool dlg_is_tty(FILE* stream);

// Returns the null-terminated escape sequence for the given style into buf.
// Undefined behvaiour if any member of style has a value outside its enum range (will
// probably result in a buffer overflow or garbage being printed).
// If all member of style are 'none' will simply nullterminate the first buf char.
DLG_API void dlg_escape_sequence(struct dlg_style style, char buf[12]);

// The reset style escape sequence.
DLG_API extern const char* const dlg_reset_sequence;

// Just returns true without other effect on non-windows systems or if dlg
// was compiled without the win_console option.
// On windows tries to set the console mode to ansi to make escape sequences work.
// This works only on newer windows 10 versions. Returns false on error.
// Only the first call to it will have an effect, following calls just return the result.
// The function is threadsafe. Automatically called by the default output handler.
// This will only be able to set the mode for the stdout and stderr consoles, so
// other streams to consoles will still not work.
DLG_API bool dlg_win_init_ansi(void);

#ifdef __cplusplus
} // extern "C"
#endif

#endif // header guard
