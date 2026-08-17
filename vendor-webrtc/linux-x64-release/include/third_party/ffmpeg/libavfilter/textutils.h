/*
 * This file is part of FFmpeg.
 *
 * FFmpeg is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 2.1 of the License, or (at your option) any later version.
 *
 * FFmpeg is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with FFmpeg; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA
 */

/**
 * @file
 * text utilities
 */

#ifndef AVFILTER_TEXTUTILS_H
#define AVFILTER_TEXTUTILS_H

#include "libavutil/bprint.h"
#include "libavutil/eval.h"
#include "libavutil/log.h"
#include "libavutil/parseutils.h"

/**
 * Function used to expand a template sequence in the format
 * %{FUNCTION_NAME[:PARAMS]}, defined in the TextExpander object.
 */
typedef struct FFExpandTextFunction {
    /**
     * name of the function
     */
    const char *name;

    /**
     * minimum and maximum number of arguments accepted by the
     * function in the PARAMS
     */
    unsigned argc_min, argc_max;

    /**
     * actual function used to perform the expansion
     */
    int (*func)(void *ctx, AVBPrint *bp, const char *function_name, unsigned argc, char **args);
} FFExpandTextFunction;

/**
 * Text expander context, used to encapsulate the logic to expand a
 * given text template.
 *
 * A backslash character @samp{\} in a text template, followed by any
 * character, always expands to the second character.
 * Sequences of the form %{FUNCTION_NAME[:PARAMS]} are expanded using a
 * function defined in the object. The text between the braces is a
 * function name, possibly followed by arguments separated by ':'. If
 * the arguments contain special characters or delimiters (':' or
 * '}'), they should be escaped.
 */
typedef struct FFExpandTextContext {
    /**
     * log context to pass to the function, used for logging and for
     * accessing the context for the function
     */
    void *log_ctx;

    /**
     * list of functions to use to expand sequences in the format
     * FUNCTION_NAME{PARAMS}
     */
    const FFExpandTextFunction *functions;

    /**
     * number of functions
     */
    unsigned int functions_nb;
} FFExpandTextContext;

/**
 * Expand text template.
 *
 * Expand text template defined in text using the logic defined in a text
 * expander object.
 *
 * @param expand_text text expansion context used to expand the text
 * @param text template text to expand
 * @param bp   BPrint object where the expanded text is written to
 * @return negative value corresponding to an AVERROR error code in case of
 * errors, a non-negative value otherwise
 */
int ff_expand_text(FFExpandTextContext *expand_text, const char *text, AVBPrint *bp);

/**
 * Print PTS representation to an AVBPrint object.
 *
 * @param log_ctx pointer to av_log object
 * @param bp  AVBPrint object where the PTS textual representation is written to
 * @param pts PTS value expressed as a double to represent
 * @param delta delta time parsed by av_parse_time(), added to the PTS
 * @param fmt string representing the format to use for printing, can be
 *        "flt" - use a float representation with 6 decimal digits,
 *        "hms" - use HH:MM:SS.MMM format,
 *        "hms24hh" - same as "hms" but wraps the hours in 24hh format
 *        (so that it is expressed in the range 00-23),
 *        "localtime" or "gmtime" - expand the PTS according to the
 *        @code{strftime()} function rules, using either the corresponding
 *        @code{localtime()} or @code{gmtime()} time
 * @param strftime_fmt: @code{strftime()} format to use to represent the PTS in
 *       case the format "localtime" or "gmtime" was selected, if not specified
 *       defaults to "%Y-%m-%d %H:%M:%S"
 * @return negative value corresponding to an AVERROR error code in case of
 * errors, a non-negative value otherwise
 */
int ff_print_pts(void *log_ctx, AVBPrint *bp, double pts, const char *delta,
                 const char *fmt, const char *strftime_fmt);

/**
 * Print time representation to an AVBPrint object.
 *
 * @param log_ctx pointer to av_log object
 * @param bp AVBPrint object where the time textual representation is written to
 * @param strftime_fmt: strftime() format to use to represent the time in case
 *        if not specified defaults to "%Y-%m-%d %H:%M:%S". The format string is
 *        extended to support the %[1-6]N after %S which prints fractions of the
 *        second with optionally specified number of digits, if not specified
 *        defaults to 3.
 * @param localtime use local time to compute the time if non-zero, otherwise
 *        use UTC
 * @return negative value corresponding to an AVERROR error code in case of
 * errors, a non-negative value otherwise
 */
int ff_print_time(void *log_ctx, AVBPrint *bp, const char *strftime_fmt, char localtime);

typedef double (*ff_eval_func2)(void *, double a, double b);

/**
 * Evaluate and print expression to an AVBprint object.
 * The output is written as a double representation.
 *
 * This is a wrapper around av_expr_parse_and_eval() and following the
 * same rules.
 *
 * @param log_ctx pointer to av_log object
 * @param bp AVBPrint object where the evaluated expression is written to
 * @param expr the expression to be evaluated
 * @param fun_names names of the ff_eval_func2 functions used to evaluate the expression
 * @param fun_values values of the ff_eval_func2 functions used to evaluate the expression
 * @param var_names names of the variables used in the expression
 * @param var_values values of the variables used in the expression
 * @param eval_ctx evaluation context to be passed to some functions
 *
 * @return negative value corresponding to an AVERROR error code in case of
 * errors, a non-negative value otherwise
 */
int ff_print_eval_expr(void *log_ctx, AVBPrint *bp,
                       const char *expr,
                       const char * const *fun_names, const ff_eval_func2 *fun_values,
                       const char * const *var_names, const double *var_values,
                       void *eval_ctx);

/**
 * Evaluate and print expression to an AVBprint object, using the
 * specified format.
 *
 * This is a wrapper around av_expr_parse_and_eval() and following the
 * same rules.
 *
 * The format is specified as a printf format character, optionally
 * preceded by the positions numbers for zero-padding.
 *
 * The following formats are accepted:
 * - x: use lowercase hexadecimal representation
 * - X: use uppercase hexadecimal representation
 * - d: use decimal representation
 * - u: use unsigned decimal representation
 *
 * @param log_ctx pointer to av_log object
 * @param bp AVBPrint object where the evaluated expression is written to
 * @param expr the expression to be evaluated
 * @param fun_names names of the ff_eval_func2 functions used to evaluate the expression
 * @param fun_values values of the ff_eval_func2 functions used to evaluate the expression
 * @param var_names names of the variables used in the expression
 * @param var_values values of the variables used in the expression
 * @param eval_ctx evaluation context to be passed to some functions
 * @param format a character representing the format, to be chosen in xXdu
 * @param positions final size of the value representation with 0-padding
 * @return negative value corresponding to an AVERROR error code in case of
 * errors, a non-negative value otherwise
 */
int ff_print_formatted_eval_expr(void *log_ctx, AVBPrint *bp,
                                 const char *expr,
                                 const char * const *fun_names, const ff_eval_func2 *fun_values,
                                 const char * const *var_names, const double *var_values,
                                 void *eval_ctx,
                                 const char format, int positions);

/**
 * Check if the character is a newline.
 *
 * @param c character to check
 * @return non-negative value in case c is a newline, 0 otherwise
 */
static inline int ff_is_newline(uint32_t c)
{
    return c == '\n' || c == '\r' || c == '\f' || c == '\v';
}

/**
 * Load text file into the buffer pointed by text.
 *
 * @param log_ctx   pointer to av_log object
 * @param textfile  filename containing the text to load
 * @param text      pointer to the text buffer where the loaded text will be
 *                  loaded
 * @param text_size pointer to the value to set with the loaded text data,
 *                  including the terminating 0 character
 * @return negative value corresponding to an AVERROR error code in case of
 * errors, a non-negative value otherwise
 */
int ff_load_textfile(void *log_ctx, const char *textfile,
                     unsigned char **text, size_t *text_size);

#endif /* AVFILTER_TEXTUTILS__H */
