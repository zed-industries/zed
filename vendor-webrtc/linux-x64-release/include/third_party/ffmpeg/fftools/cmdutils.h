/*
 * Various utilities for command line tools
 * copyright (c) 2003 Fabrice Bellard
 *
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

#ifndef FFTOOLS_CMDUTILS_H
#define FFTOOLS_CMDUTILS_H

#include <stdint.h>

#include "config.h"
#include "libavcodec/avcodec.h"
#include "libavfilter/avfilter.h"
#include "libavformat/avformat.h"
#include "libswscale/swscale.h"

#ifdef _WIN32
#undef main /* We don't want SDL to override our main() */
#endif

/**
 * program name, defined by the program for show_version().
 */
extern const char program_name[];

/**
 * program birth year, defined by the program for show_banner()
 */
extern const int program_birth_year;

extern AVDictionary *sws_dict;
extern AVDictionary *swr_opts;
extern AVDictionary *format_opts, *codec_opts;
extern int hide_banner;

/**
 * Initialize dynamic library loading
 */
void init_dynload(void);

/**
 * Uninitialize the cmdutils option system, in particular
 * free the *_opts contexts and their contents.
 */
void uninit_opts(void);

/**
 * Trivial log callback.
 * Only suitable for opt_help and similar since it lacks prefix handling.
 */
void log_callback_help(void* ptr, int level, const char* fmt, va_list vl);

/**
 * Fallback for options that are not explicitly handled, these will be
 * parsed through AVOptions.
 */
int opt_default(void *optctx, const char *opt, const char *arg);

/**
 * Limit the execution time.
 */
int opt_timelimit(void *optctx, const char *opt, const char *arg);

enum OptionType {
    OPT_TYPE_FUNC,
    OPT_TYPE_BOOL,
    OPT_TYPE_STRING,
    OPT_TYPE_INT,
    OPT_TYPE_INT64,
    OPT_TYPE_FLOAT,
    OPT_TYPE_DOUBLE,
    OPT_TYPE_TIME,
};

/**
 * Parse a string and return its corresponding value as a double.
 *
 * @param context the context of the value to be set (e.g. the
 * corresponding command line option name)
 * @param numstr the string to be parsed
 * @param type the type (OPT_INT64 or OPT_FLOAT) as which the
 * string should be parsed
 * @param min the minimum valid accepted value
 * @param max the maximum valid accepted value
 */
int parse_number(const char *context, const char *numstr, enum OptionType type,
                 double min, double max, double *dst);

enum StreamList {
    STREAM_LIST_ALL,
    STREAM_LIST_STREAM_ID,
    STREAM_LIST_PROGRAM,
    STREAM_LIST_GROUP_ID,
    STREAM_LIST_GROUP_IDX,
};

typedef struct StreamSpecifier {
    // trailing stream index - pick idx-th stream that matches
    // all the other constraints; -1 when not present
    int                  idx;

    // which stream list to consider
    enum StreamList      stream_list;

    // STREAM_LIST_STREAM_ID: stream ID
    // STREAM_LIST_GROUP_IDX: group index
    // STREAM_LIST_GROUP_ID:  group ID
    // STREAM_LIST_PROGRAM:   program ID
    int64_t              list_id;

    // when not AVMEDIA_TYPE_UNKNOWN, consider only streams of this type
    enum AVMediaType     media_type;
    uint8_t              no_apic;

    uint8_t              usable_only;

    int                  disposition;

    char                *meta_key;
    char                *meta_val;

    char                *remainder;
} StreamSpecifier;

/**
 * Parse a stream specifier string into a form suitable for matching.
 *
 * @param ss Parsed specifier will be stored here; must be uninitialized
 *           with stream_specifier_uninit() when no longer needed.
 * @param spec String containing the stream specifier to be parsed.
 * @param allow_remainder When 1, the part of spec that is left after parsing
 *                        the stream specifier is stored into ss->remainder.
 *                        When 0, any remainder will cause parsing to fail.
 */
int stream_specifier_parse(StreamSpecifier *ss, const char *spec,
                           int allow_remainder, void *logctx);

/**
 * @return 1 if st matches the parsed specifier, 0 if it does not
 */
unsigned stream_specifier_match(const StreamSpecifier *ss,
                                const AVFormatContext *s, const AVStream *st,
                                void *logctx);

void stream_specifier_uninit(StreamSpecifier *ss);

typedef struct SpecifierOpt {
    // original specifier or empty string
    char            *specifier;
    // parsed specifier for OPT_FLAG_PERSTREAM options
    StreamSpecifier  stream_spec;

    union {
        uint8_t *str;
        int        i;
        int64_t  i64;
        uint64_t ui64;
        float      f;
        double   dbl;
    } u;
} SpecifierOpt;

typedef struct SpecifierOptList {
    SpecifierOpt    *opt;
    int           nb_opt;

    /* Canonical option definition that was parsed into this list. */
    const struct OptionDef *opt_canon;
    /* Type corresponding to the field that should be used from SpecifierOpt.u.
     * May not match the option type, e.g. OPT_TYPE_BOOL options are stored as
     * int, so this field would be OPT_TYPE_INT for them */
    enum OptionType type;
} SpecifierOptList;

typedef struct OptionDef {
    const char *name;
    enum OptionType type;
    int flags;

/* The OPT_TYPE_FUNC option takes an argument.
 * Must not be used with other option types, as for those it holds:
 * - OPT_TYPE_BOOL do not take an argument
 * - all other types do
 */
#define OPT_FUNC_ARG    (1 << 0)
/* Program will immediately exit after processing this option */
#define OPT_EXIT        (1 << 1)
/* Option is intended for advanced users. Only affects
 * help output.
 */
#define OPT_EXPERT      (1 << 2)
#define OPT_VIDEO       (1 << 3)
#define OPT_AUDIO       (1 << 4)
#define OPT_SUBTITLE    (1 << 5)
#define OPT_DATA        (1 << 6)
/* The option is per-file (currently ffmpeg-only). At least one of OPT_INPUT,
 * OPT_OUTPUT, OPT_DECODER must be set when this flag is in use.
   */
#define OPT_PERFILE     (1 << 7)

/* Option is specified as an offset in a passed optctx.
 * Always use as OPT_OFFSET in option definitions. */
#define OPT_FLAG_OFFSET (1 << 8)
#define OPT_OFFSET      (OPT_FLAG_OFFSET | OPT_PERFILE)

/* Option is to be stored in a SpecifierOptList.
   Always use as OPT_SPEC in option definitions. */
#define OPT_FLAG_SPEC   (1 << 9)
#define OPT_SPEC        (OPT_FLAG_SPEC | OPT_OFFSET)

/* Option applies per-stream (implies OPT_SPEC). */
#define OPT_FLAG_PERSTREAM  (1 << 10)
#define OPT_PERSTREAM   (OPT_FLAG_PERSTREAM | OPT_SPEC)

/* ffmpeg-only - specifies whether an OPT_PERFILE option applies to input,
 * output, or both. */
#define OPT_INPUT       (1 << 11)
#define OPT_OUTPUT      (1 << 12)

/* This option is a "canonical" form, to which one or more alternatives
 * exist. These alternatives are listed in u1.names_alt. */
#define OPT_HAS_ALT     (1 << 13)
/* This option is an alternative form of some other option, whose
 * name is stored in u1.name_canon */
#define OPT_HAS_CANON   (1 << 14)

/* ffmpeg-only - OPT_PERFILE may apply to standalone decoders */
#define OPT_DECODER     (1 << 15)

     union {
        void *dst_ptr;
        int (*func_arg)(void *, const char *, const char *);
        size_t off;
    } u;
    const char *help;
    const char *argname;

    union {
        /* Name of the canonical form of this option.
         * Is valid when OPT_HAS_CANON is set. */
        const char *name_canon;
        /* A NULL-terminated list of alternate forms of this option.
         * Is valid when OPT_HAS_ALT is set. */
        const char * const *names_alt;
    } u1;
} OptionDef;

/**
 * Print help for all options matching specified flags.
 *
 * @param options a list of options
 * @param msg title of this group. Only printed if at least one option matches.
 * @param req_flags print only options which have all those flags set.
 * @param rej_flags don't print options which have any of those flags set.
 */
void show_help_options(const OptionDef *options, const char *msg, int req_flags,
                       int rej_flags);

/**
 * Show help for all options with given flags in class and all its
 * children.
 */
void show_help_children(const AVClass *class, int flags);

/**
 * Per-fftool specific help handler. Implemented in each
 * fftool, called by show_help().
 */
void show_help_default(const char *opt, const char *arg);

/**
 * Parse the command line arguments.
 *
 * @param optctx an opaque options context
 * @param argc   number of command line arguments
 * @param argv   values of command line arguments
 * @param options Array with the definitions required to interpret every
 * option of the form: -option_name [argument]
 * @param parse_arg_function Name of the function called to process every
 * argument without a leading option name flag. NULL if such arguments do
 * not have to be processed.
 */
int parse_options(void *optctx, int argc, char **argv, const OptionDef *options,
                  int (* parse_arg_function)(void *optctx, const char*));

/**
 * Parse one given option.
 *
 * @return on success 1 if arg was consumed, 0 otherwise; negative number on error
 */
int parse_option(void *optctx, const char *opt, const char *arg,
                 const OptionDef *options);

/**
 * An option extracted from the commandline.
 * Cannot use AVDictionary because of options like -map which can be
 * used multiple times.
 */
typedef struct Option {
    const OptionDef  *opt;
    const char       *key;
    const char       *val;
} Option;

typedef struct OptionGroupDef {
    /** group name */
    const char *name;
    /**
     * Option to be used as group separator. Can be NULL for groups which
     * are terminated by a non-option argument (e.g. ffmpeg output files)
     */
    const char *sep;
    /**
     * Option flags that must be set on each option that is
     * applied to this group
     */
    int flags;
} OptionGroupDef;

typedef struct OptionGroup {
    const OptionGroupDef *group_def;
    const char *arg;

    Option *opts;
    int  nb_opts;

    AVDictionary *codec_opts;
    AVDictionary *format_opts;
    AVDictionary *sws_dict;
    AVDictionary *swr_opts;
} OptionGroup;

/**
 * A list of option groups that all have the same group type
 * (e.g. input files or output files)
 */
typedef struct OptionGroupList {
    const OptionGroupDef *group_def;

    OptionGroup *groups;
    int       nb_groups;
} OptionGroupList;

typedef struct OptionParseContext {
    OptionGroup global_opts;

    OptionGroupList *groups;
    int           nb_groups;

    /* parsing state */
    OptionGroup cur_group;
} OptionParseContext;

/**
 * Parse an options group and write results into optctx.
 *
 * @param optctx an app-specific options context. NULL for global options group
 */
int parse_optgroup(void *optctx, OptionGroup *g, const OptionDef *defs);

/**
 * Split the commandline into an intermediate form convenient for further
 * processing.
 *
 * The commandline is assumed to be composed of options which either belong to a
 * group (those with OPT_SPEC, OPT_OFFSET or OPT_PERFILE) or are global
 * (everything else).
 *
 * A group (defined by an OptionGroupDef struct) is a sequence of options
 * terminated by either a group separator option (e.g. -i) or a parameter that
 * is not an option (doesn't start with -). A group without a separator option
 * must always be first in the supplied groups list.
 *
 * All options within the same group are stored in one OptionGroup struct in an
 * OptionGroupList, all groups with the same group definition are stored in one
 * OptionGroupList in OptionParseContext.groups. The order of group lists is the
 * same as the order of group definitions.
 */
int split_commandline(OptionParseContext *octx, int argc, char *argv[],
                      const OptionDef *options,
                      const OptionGroupDef *groups, int nb_groups);

/**
 * Free all allocated memory in an OptionParseContext.
 */
void uninit_parse_context(OptionParseContext *octx);

/**
 * Find the '-loglevel' option in the command line args and apply it.
 */
void parse_loglevel(int argc, char **argv, const OptionDef *options);

/**
 * Return index of option opt in argv or 0 if not found.
 */
int locate_option(int argc, char **argv, const OptionDef *options,
                  const char *optname);

/**
 * Check if the given stream matches a stream specifier.
 *
 * @param s  Corresponding format context.
 * @param st Stream from s to be checked.
 * @param spec A stream specifier of the [v|a|s|d]:[\<stream index\>] form.
 *
 * @return 1 if the stream matches, 0 if it doesn't, <0 on error
 */
int check_stream_specifier(AVFormatContext *s, AVStream *st, const char *spec);

/**
 * Filter out options for given codec.
 *
 * Create a new options dictionary containing only the options from
 * opts which apply to the codec with ID codec_id.
 *
 * @param opts     dictionary to place options in
 * @param codec_id ID of the codec that should be filtered for
 * @param s Corresponding format context.
 * @param st A stream from s for which the options should be filtered.
 * @param codec The particular codec for which the options should be filtered.
 *              If null, the default one is looked up according to the codec id.
 * @param dst a pointer to the created dictionary
 * @param opts_used if non-NULL, every option stored in dst is also stored here,
 *                  with specifiers preserved
 * @return a non-negative number on success, a negative error code on failure
 */
int filter_codec_opts(const AVDictionary *opts, enum AVCodecID codec_id,
                      AVFormatContext *s, AVStream *st, const AVCodec *codec,
                      AVDictionary **dst, AVDictionary **opts_used);

/**
 * Setup AVCodecContext options for avformat_find_stream_info().
 *
 * Create an array of dictionaries, one dictionary for each stream
 * contained in s.
 * Each dictionary will contain the options from codec_opts which can
 * be applied to the corresponding stream codec context.
 */
int setup_find_stream_info_opts(AVFormatContext *s,
                                AVDictionary *codec_opts,
                                AVDictionary ***dst);

/**
 * Print an error message to stderr, indicating filename and a human
 * readable description of the error code err.
 *
 * If strerror_r() is not available the use of this function in a
 * multithreaded application may be unsafe.
 *
 * @see av_strerror()
 */
static inline void print_error(const char *filename, int err)
{
    av_log(NULL, AV_LOG_ERROR, "%s: %s\n", filename, av_err2str(err));
}

/**
 * Print the program banner to stderr. The banner contents depend on the
 * current version of the repository and of the libav* libraries used by
 * the program.
 */
void show_banner(int argc, char **argv, const OptionDef *options);

/**
 * Return a positive value if a line read from standard input
 * starts with [yY], otherwise return 0.
 */
int read_yesno(void);

/**
 * Get a file corresponding to a preset file.
 *
 * If is_path is non-zero, look for the file in the path preset_name.
 * Otherwise search for a file named arg.ffpreset in the directories
 * $FFMPEG_DATADIR (if set), $HOME/.ffmpeg, and in the datadir defined
 * at configuration time or in a "ffpresets" folder along the executable
 * on win32, in that order. If no such file is found and
 * codec_name is defined, then search for a file named
 * codec_name-preset_name.avpreset in the above-mentioned directories.
 *
 * @param filename buffer where the name of the found filename is written
 * @param filename_size size in bytes of the filename buffer
 * @param preset_name name of the preset to search
 * @param is_path tell if preset_name is a filename path
 * @param codec_name name of the codec for which to look for the
 * preset, may be NULL
 */
FILE *get_preset_file(char *filename, size_t filename_size,
                      const char *preset_name, int is_path, const char *codec_name);

/**
 * Realloc array to hold new_size elements of elem_size.
 *
 * @param array pointer to the array to reallocate, will be updated
 *              with a new pointer on success
 * @param elem_size size in bytes of each element
 * @param size new element count will be written here
 * @param new_size number of elements to place in reallocated array
 * @return a non-negative number on success, a negative error code on failure
 */
int grow_array(void **array, int elem_size, int *size, int new_size);

/**
 * Atomically add a new element to an array of pointers, i.e. allocate
 * a new entry, reallocate the array of pointers and make the new last
 * member of this array point to the newly allocated buffer.
 *
 * @param array     array of pointers to reallocate
 * @param elem_size size of the new element to allocate
 * @param nb_elems  pointer to the number of elements of the array array;
 *                  *nb_elems will be incremented by one by this function.
 * @return pointer to the newly allocated entry or NULL on failure
 */
void *allocate_array_elem(void *array, size_t elem_size, int *nb_elems);

#define GROW_ARRAY(array, nb_elems)\
    grow_array((void**)&array, sizeof(*array), &nb_elems, nb_elems + 1)

double get_rotation(const int32_t *displaymatrix);

/* read file contents into a string */
char *file_read(const char *filename);

/* Remove keys in dictionary b from dictionary a */
void remove_avoptions(AVDictionary **a, AVDictionary *b);

/* Check if any keys exist in dictionary m */
int check_avoptions(AVDictionary *m);

int cmdutils_isalnum(char c);

#endif /* FFTOOLS_CMDUTILS_H */
