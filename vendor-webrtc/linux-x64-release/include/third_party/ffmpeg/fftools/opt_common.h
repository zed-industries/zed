/*
 * Option handlers shared between the tools.
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

#ifndef FFTOOLS_OPT_COMMON_H
#define FFTOOLS_OPT_COMMON_H

#include "config.h"

#include "cmdutils.h"

#if CONFIG_AVDEVICE
/**
 * Print a listing containing autodetected sinks of the output device.
 * Device name with options may be passed as an argument to limit results.
 */
int show_sinks(void *optctx, const char *opt, const char *arg);

/**
 * Print a listing containing autodetected sources of the input device.
 * Device name with options may be passed as an argument to limit results.
 */
int show_sources(void *optctx, const char *opt, const char *arg);
#endif

#if CONFIG_AVDEVICE
#define CMDUTILS_COMMON_OPTIONS_AVDEVICE                                                                                \
    { "sources"    , OPT_TYPE_FUNC, OPT_EXIT | OPT_FUNC_ARG | OPT_EXPERT, { .func_arg = show_sources },                 \
      "list sources of the input device", "device" },                                                                   \
    { "sinks"      , OPT_TYPE_FUNC, OPT_EXIT | OPT_FUNC_ARG | OPT_EXPERT, { .func_arg = show_sinks },                   \
      "list sinks of the output device", "device" },                                                                    \

#else
#define CMDUTILS_COMMON_OPTIONS_AVDEVICE
#endif

/**
 * Print the license of the program to stdout. The license depends on
 * the license of the libraries compiled into the program.
 * This option processing function does not utilize the arguments.
 */
int show_license(void *optctx, const char *opt, const char *arg);

/**
 * Generic -h handler common to all fftools.
 */
int show_help(void *optctx, const char *opt, const char *arg);

/**
 * Print the version of the program to stdout. The version message
 * depends on the current versions of the repository and of the libav*
 * libraries.
 * This option processing function does not utilize the arguments.
 */
int show_version(void *optctx, const char *opt, const char *arg);

/**
 * Print the build configuration of the program to stdout. The contents
 * depend on the definition of FFMPEG_CONFIGURATION.
 * This option processing function does not utilize the arguments.
 */
int show_buildconf(void *optctx, const char *opt, const char *arg);

/**
 * Print a listing containing all the formats supported by the
 * program (including devices).
 * This option processing function does not utilize the arguments.
 */
int show_formats(void *optctx, const char *opt, const char *arg);

/**
 * Print a listing containing all the muxers supported by the
 * program (including devices).
 * This option processing function does not utilize the arguments.
 */
int show_muxers(void *optctx, const char *opt, const char *arg);

/**
 * Print a listing containing all the demuxer supported by the
 * program (including devices).
 * This option processing function does not utilize the arguments.
 */
int show_demuxers(void *optctx, const char *opt, const char *arg);

/**
 * Print a listing containing all the devices supported by the
 * program.
 * This option processing function does not utilize the arguments.
 */
int show_devices(void *optctx, const char *opt, const char *arg);

/**
 * Print a listing containing all the codecs supported by the
 * program.
 * This option processing function does not utilize the arguments.
 */
int show_codecs(void *optctx, const char *opt, const char *arg);

/**
 * Print a listing containing all the decoders supported by the
 * program.
 */
int show_decoders(void *optctx, const char *opt, const char *arg);

/**
 * Print a listing containing all the encoders supported by the
 * program.
 */
int show_encoders(void *optctx, const char *opt, const char *arg);

/**
 * Print a listing containing all the bit stream filters supported by the
 * program.
 * This option processing function does not utilize the arguments.
 */
int show_bsfs(void *optctx, const char *opt, const char *arg);

/**
 * Print a listing containing all the protocols supported by the
 * program.
 * This option processing function does not utilize the arguments.
 */
int show_protocols(void *optctx, const char *opt, const char *arg);

/**
 * Print a listing containing all the filters supported by the
 * program.
 * This option processing function does not utilize the arguments.
 */
int show_filters(void *optctx, const char *opt, const char *arg);

/**
 * Print a listing containing all the pixel formats supported by the
 * program.
 * This option processing function does not utilize the arguments.
 */
int show_pix_fmts(void *optctx, const char *opt, const char *arg);

/**
 * Print a listing containing all the standard channel layouts supported by
 * the program.
 * This option processing function does not utilize the arguments.
 */
int show_layouts(void *optctx, const char *opt, const char *arg);

/**
 * Print a listing containing all the sample formats supported by the
 * program.
 */
int show_sample_fmts(void *optctx, const char *opt, const char *arg);

/**
 * Print a listing containing all supported stream dispositions.
 */
int show_dispositions(void *optctx, const char *opt, const char *arg);

/**
 * Print a listing containing all the color names and values recognized
 * by the program.
 */
int show_colors(void *optctx, const char *opt, const char *arg);

/**
 * Set the libav* libraries log level.
 */
int opt_loglevel(void *optctx, const char *opt, const char *arg);

int opt_report(void *optctx, const char *opt, const char *arg);
int init_report(const char *env, FILE **file);

int opt_max_alloc(void *optctx, const char *opt, const char *arg);

/**
 * Override the cpuflags.
 */
int opt_cpuflags(void *optctx, const char *opt, const char *arg);

/**
 * Override the cpucount.
 */
int opt_cpucount(void *optctx, const char *opt, const char *arg);

#define CMDUTILS_COMMON_OPTIONS                                                                                         \
    { "L",            OPT_TYPE_FUNC, OPT_EXIT,              { .func_arg = show_license },     "show license" },                          \
    { "h",            OPT_TYPE_FUNC, OPT_EXIT,              { .func_arg = show_help },        "show help", "topic" },                    \
    { "?",            OPT_TYPE_FUNC, OPT_EXIT | OPT_EXPERT, { .func_arg = show_help },        "show help", "topic" },                    \
    { "help",         OPT_TYPE_FUNC, OPT_EXIT | OPT_EXPERT, { .func_arg = show_help },        "show help", "topic" },                    \
    { "-help",        OPT_TYPE_FUNC, OPT_EXIT | OPT_EXPERT, { .func_arg = show_help },        "show help", "topic" },                    \
    { "version",      OPT_TYPE_FUNC, OPT_EXIT,              { .func_arg = show_version },     "show version" },                          \
    { "buildconf",    OPT_TYPE_FUNC, OPT_EXIT | OPT_EXPERT, { .func_arg = show_buildconf },   "show build configuration" },              \
    { "formats",      OPT_TYPE_FUNC, OPT_EXIT | OPT_EXPERT, { .func_arg = show_formats },     "show available formats" },                \
    { "muxers",       OPT_TYPE_FUNC, OPT_EXIT,              { .func_arg = show_muxers },      "show available muxers" },                 \
    { "demuxers",     OPT_TYPE_FUNC, OPT_EXIT,              { .func_arg = show_demuxers },    "show available demuxers" },               \
    { "devices",      OPT_TYPE_FUNC, OPT_EXIT,              { .func_arg = show_devices },     "show available devices" },                \
    { "codecs",       OPT_TYPE_FUNC, OPT_EXIT | OPT_EXPERT, { .func_arg = show_codecs },      "show available codecs" },                 \
    { "decoders",     OPT_TYPE_FUNC, OPT_EXIT,              { .func_arg = show_decoders },    "show available decoders" },               \
    { "encoders",     OPT_TYPE_FUNC, OPT_EXIT,              { .func_arg = show_encoders },    "show available encoders" },               \
    { "bsfs",         OPT_TYPE_FUNC, OPT_EXIT | OPT_EXPERT, { .func_arg = show_bsfs },        "show available bit stream filters" },     \
    { "protocols",    OPT_TYPE_FUNC, OPT_EXIT | OPT_EXPERT, { .func_arg = show_protocols },   "show available protocols" },              \
    { "filters",      OPT_TYPE_FUNC, OPT_EXIT,              { .func_arg = show_filters },     "show available filters" },                \
    { "pix_fmts",     OPT_TYPE_FUNC, OPT_EXIT,              { .func_arg = show_pix_fmts },    "show available pixel formats" },          \
    { "layouts",      OPT_TYPE_FUNC, OPT_EXIT,              { .func_arg = show_layouts },     "show standard channel layouts" },         \
    { "sample_fmts",  OPT_TYPE_FUNC, OPT_EXIT,              { .func_arg = show_sample_fmts }, "show available audio sample formats" },   \
    { "dispositions", OPT_TYPE_FUNC, OPT_EXIT | OPT_EXPERT, { .func_arg = show_dispositions}, "show available stream dispositions" },    \
    { "colors",       OPT_TYPE_FUNC, OPT_EXIT | OPT_EXPERT, { .func_arg = show_colors },      "show available color names" },            \
    { "loglevel",     OPT_TYPE_FUNC, OPT_FUNC_ARG | OPT_EXPERT, { .func_arg = opt_loglevel },     "set logging level", "loglevel" },         \
    { "v",            OPT_TYPE_FUNC, OPT_FUNC_ARG,          { .func_arg = opt_loglevel },     "set logging level", "loglevel" },         \
    { "report",       OPT_TYPE_FUNC, OPT_EXPERT,            { .func_arg = opt_report },       "generate a report" },                     \
    { "max_alloc",    OPT_TYPE_FUNC, OPT_FUNC_ARG | OPT_EXPERT, { .func_arg = opt_max_alloc },    "set maximum size of a single allocated block", "bytes" }, \
    { "cpuflags",     OPT_TYPE_FUNC, OPT_FUNC_ARG | OPT_EXPERT, { .func_arg = opt_cpuflags },     "force specific cpu flags", "flags" },     \
    { "cpucount",     OPT_TYPE_FUNC, OPT_FUNC_ARG | OPT_EXPERT, { .func_arg = opt_cpucount },     "force specific cpu count", "count" },     \
    { "hide_banner",  OPT_TYPE_BOOL, OPT_EXPERT,            {&hide_banner},                   "do not show program banner", "hide_banner" }, \
    CMDUTILS_COMMON_OPTIONS_AVDEVICE                                                                                    \

#endif /* FFTOOLS_OPT_COMMON_H */
