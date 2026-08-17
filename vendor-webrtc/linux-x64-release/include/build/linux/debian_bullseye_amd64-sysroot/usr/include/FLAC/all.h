/* libFLAC - Free Lossless Audio Codec library
 * Copyright (C) 2000-2009  Josh Coalson
 * Copyright (C) 2011-2016  Xiph.Org Foundation
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 *
 * - Redistributions of source code must retain the above copyright
 * notice, this list of conditions and the following disclaimer.
 *
 * - Redistributions in binary form must reproduce the above copyright
 * notice, this list of conditions and the following disclaimer in the
 * documentation and/or other materials provided with the distribution.
 *
 * - Neither the name of the Xiph.org Foundation nor the names of its
 * contributors may be used to endorse or promote products derived from
 * this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * ``AS IS'' AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL THE FOUNDATION OR
 * CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF
 * LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING
 * NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
 * SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef FLAC__ALL_H
#define FLAC__ALL_H

#include "export.h"

#include "assert.h"
#include "callback.h"
#include "format.h"
#include "metadata.h"
#include "ordinals.h"
#include "stream_decoder.h"
#include "stream_encoder.h"

/** \mainpage
 *
 * \section intro Introduction
 *
 * This is the documentation for the FLAC C and C++ APIs.  It is
 * highly interconnected; this introduction should give you a top
 * level idea of the structure and how to find the information you
 * need.  As a prerequisite you should have at least a basic
 * knowledge of the FLAC format, documented
 * <A HREF="../format.html">here</A>.
 *
 * \section c_api FLAC C API
 *
 * The FLAC C API is the interface to libFLAC, a set of structures
 * describing the components of FLAC streams, and functions for
 * encoding and decoding streams, as well as manipulating FLAC
 * metadata in files.  The public include files will be installed
 * in your include area (for example /usr/include/FLAC/...).
 *
 * By writing a little code and linking against libFLAC, it is
 * relatively easy to add FLAC support to another program.  The
 * library is licensed under <A HREF="../license.html">Xiph's BSD license</A>.
 * Complete source code of libFLAC as well as the command-line
 * encoder and plugins is available and is a useful source of
 * examples.
 *
 * Aside from encoders and decoders, libFLAC provides a powerful
 * metadata interface for manipulating metadata in FLAC files.  It
 * allows the user to add, delete, and modify FLAC metadata blocks
 * and it can automatically take advantage of PADDING blocks to avoid
 * rewriting the entire FLAC file when changing the size of the
 * metadata.
 *
 * libFLAC usually only requires the standard C library and C math
 * library. In particular, threading is not used so there is no
 * dependency on a thread library. However, libFLAC does not use
 * global variables and should be thread-safe.
 *
 * libFLAC also supports encoding to and decoding from Ogg FLAC.
 * However the metadata editing interfaces currently have limited
 * read-only support for Ogg FLAC files.
 *
 * \section cpp_api FLAC C++ API
 *
 * The FLAC C++ API is a set of classes that encapsulate the
 * structures and functions in libFLAC.  They provide slightly more
 * functionality with respect to metadata but are otherwise
 * equivalent.  For the most part, they share the same usage as
 * their counterparts in libFLAC, and the FLAC C API documentation
 * can be used as a supplement.  The public include files
 * for the C++ API will be installed in your include area (for
 * example /usr/include/FLAC++/...).
 *
 * libFLAC++ is also licensed under
 * <A HREF="../license.html">Xiph's BSD license</A>.
 *
 * \section getting_started Getting Started
 *
 * A good starting point for learning the API is to browse through
 * the <A HREF="modules.html">modules</A>.  Modules are logical
 * groupings of related functions or classes, which correspond roughly
 * to header files or sections of header files.  Each module includes a
 * detailed description of the general usage of its functions or
 * classes.
 *
 * From there you can go on to look at the documentation of
 * individual functions.  You can see different views of the individual
 * functions through the links in top bar across this page.
 *
 * If you prefer a more hands-on approach, you can jump right to some
 * <A HREF="../documentation_example_code.html">example code</A>.
 *
 * \section porting_guide Porting Guide
 *
 * Starting with FLAC 1.1.3 a \link porting Porting Guide \endlink
 * has been introduced which gives detailed instructions on how to
 * port your code to newer versions of FLAC.
 *
 * \section embedded_developers Embedded Developers
 *
 * libFLAC has grown larger over time as more functionality has been
 * included, but much of it may be unnecessary for a particular embedded
 * implementation.  Unused parts may be pruned by some simple editing of
 * src/libFLAC/Makefile.am.  In general, the decoders, encoders, and
 * metadata interface are all independent from each other.
 *
 * It is easiest to just describe the dependencies:
 *
 * - All modules depend on the \link flac_format Format \endlink module.
 * - The decoders and encoders depend on the bitbuffer.
 * - The decoder is independent of the encoder.  The encoder uses the
 *   decoder because of the verify feature, but this can be removed if
 *   not needed.
 * - Parts of the metadata interface require the stream decoder (but not
 *   the encoder).
 * - Ogg support is selectable through the compile time macro
 *   \c FLAC__HAS_OGG.
 *
 * For example, if your application only requires the stream decoder, no
 * encoder, and no metadata interface, you can remove the stream encoder
 * and the metadata interface, which will greatly reduce the size of the
 * library.
 *
 * Also, there are several places in the libFLAC code with comments marked
 * with "OPT:" where a #define can be changed to enable code that might be
 * faster on a specific platform.  Experimenting with these can yield faster
 * binaries.
 */

/** \defgroup porting Porting Guide for New Versions
 *
 * This module describes differences in the library interfaces from
 * version to version.  It assists in the porting of code that uses
 * the libraries to newer versions of FLAC.
 *
 * One simple facility for making porting easier that has been added
 * in FLAC 1.1.3 is a set of \c #defines in \c export.h of each
 * library's includes (e.g. \c include/FLAC/export.h).  The
 * \c #defines mirror the libraries'
 * <A HREF="http://www.gnu.org/software/libtool/manual/libtool.html#Libtool-versioning">libtool version numbers</A>,
 * e.g. in libFLAC there are \c FLAC_API_VERSION_CURRENT,
 * \c FLAC_API_VERSION_REVISION, and \c FLAC_API_VERSION_AGE.
 * These can be used to support multiple versions of an API during the
 * transition phase, e.g.
 *
 * \code
 * #if !defined(FLAC_API_VERSION_CURRENT) || FLAC_API_VERSION_CURRENT <= 7
 *   legacy code
 * #else
 *   new code
 * #endif
 * \endcode
 *
 * The source will work for multiple versions and the legacy code can
 * easily be removed when the transition is complete.
 *
 * Another available symbol is FLAC_API_SUPPORTS_OGG_FLAC (defined in
 * include/FLAC/export.h), which can be used to determine whether or not
 * the library has been compiled with support for Ogg FLAC.  This is
 * simpler than trying to call an Ogg init function and catching the
 * error.
 */

/** \defgroup porting_1_1_2_to_1_1_3 Porting from FLAC 1.1.2 to 1.1.3
 *  \ingroup porting
 *
 *  \brief
 *  This module describes porting from FLAC 1.1.2 to FLAC 1.1.3.
 *
 * The main change between the APIs in 1.1.2 and 1.1.3 is that they have
 * been simplified.  First, libOggFLAC has been merged into libFLAC and
 * libOggFLAC++ has been merged into libFLAC++.  Second, both the three
 * decoding layers and three encoding layers have been merged into a
 * single stream decoder and stream encoder.  That is, the functionality
 * of FLAC__SeekableStreamDecoder and FLAC__FileDecoder has been merged
 * into FLAC__StreamDecoder, and FLAC__SeekableStreamEncoder and
 * FLAC__FileEncoder into FLAC__StreamEncoder.  Only the
 * FLAC__StreamDecoder and FLAC__StreamEncoder remain.  What this means
 * is there is now a single API that can be used to encode or decode
 * streams to/from native FLAC or Ogg FLAC and the single API can work
 * on both seekable and non-seekable streams.
 *
 * Instead of creating an encoder or decoder of a certain layer, now the
 * client will always create a FLAC__StreamEncoder or
 * FLAC__StreamDecoder.  The old layers are now differentiated by the
 * initialization function.  For example, for the decoder,
 * FLAC__stream_decoder_init() has been replaced by
 * FLAC__stream_decoder_init_stream().  This init function takes
 * callbacks for the I/O, and the seeking callbacks are optional.  This
 * allows the client to use the same object for seekable and
 * non-seekable streams.  For decoding a FLAC file directly, the client
 * can use FLAC__stream_decoder_init_file() and pass just a filename
 * and fewer callbacks; most of the other callbacks are supplied
 * internally.  For situations where fopen()ing by filename is not
 * possible (e.g. Unicode filenames on Windows) the client can instead
 * open the file itself and supply the FILE* to
 * FLAC__stream_decoder_init_FILE().  The init functions now returns a
 * FLAC__StreamDecoderInitStatus instead of FLAC__StreamDecoderState.
 * Since the callbacks and client data are now passed to the init
 * function, the FLAC__stream_decoder_set_*_callback() functions and
 * FLAC__stream_decoder_set_client_data() are no longer needed.  The
 * rest of the calls to the decoder are the same as before.
 *
 * There are counterpart init functions for Ogg FLAC, e.g.
 * FLAC__stream_decoder_init_ogg_stream().  All the rest of the calls
 * and callbacks are the same as for native FLAC.
 *
 * As an example, in FLAC 1.1.2 a seekable stream decoder would have
 * been set up like so:
 *
 * \code
 * FLAC__SeekableStreamDecoder *decoder = FLAC__seekable_stream_decoder_new();
 * if(decoder == NULL) do_something;
 * FLAC__seekable_stream_decoder_set_md5_checking(decoder, true);
 * [... other settings ...]
 * FLAC__seekable_stream_decoder_set_read_callback(decoder, my_read_callback);
 * FLAC__seekable_stream_decoder_set_seek_callback(decoder, my_seek_callback);
 * FLAC__seekable_stream_decoder_set_tell_callback(decoder, my_tell_callback);
 * FLAC__seekable_stream_decoder_set_length_callback(decoder, my_length_callback);
 * FLAC__seekable_stream_decoder_set_eof_callback(decoder, my_eof_callback);
 * FLAC__seekable_stream_decoder_set_write_callback(decoder, my_write_callback);
 * FLAC__seekable_stream_decoder_set_metadata_callback(decoder, my_metadata_callback);
 * FLAC__seekable_stream_decoder_set_error_callback(decoder, my_error_callback);
 * FLAC__seekable_stream_decoder_set_client_data(decoder, my_client_data);
 * if(FLAC__seekable_stream_decoder_init(decoder) != FLAC__SEEKABLE_STREAM_DECODER_OK) do_something;
 * \endcode
 *
 * In FLAC 1.1.3 it is like this:
 *
 * \code
 * FLAC__StreamDecoder *decoder = FLAC__stream_decoder_new();
 * if(decoder == NULL) do_something;
 * FLAC__stream_decoder_set_md5_checking(decoder, true);
 * [... other settings ...]
 * if(FLAC__stream_decoder_init_stream(
 *   decoder,
 *   my_read_callback,
 *   my_seek_callback,      // or NULL
 *   my_tell_callback,      // or NULL
 *   my_length_callback,    // or NULL
 *   my_eof_callback,       // or NULL
 *   my_write_callback,
 *   my_metadata_callback,  // or NULL
 *   my_error_callback,
 *   my_client_data
 * ) != FLAC__STREAM_DECODER_INIT_STATUS_OK) do_something;
 * \endcode
 *
 * or you could do;
 *
 * \code
 * [...]
 * FILE *file = fopen("somefile.flac","rb");
 * if(file == NULL) do_somthing;
 * if(FLAC__stream_decoder_init_FILE(
 *   decoder,
 *   file,
 *   my_write_callback,
 *   my_metadata_callback,  // or NULL
 *   my_error_callback,
 *   my_client_data
 * ) != FLAC__STREAM_DECODER_INIT_STATUS_OK) do_something;
 * \endcode
 *
 * or just:
 *
 * \code
 * [...]
 * if(FLAC__stream_decoder_init_file(
 *   decoder,
 *   "somefile.flac",
 *   my_write_callback,
 *   my_metadata_callback,  // or NULL
 *   my_error_callback,
 *   my_client_data
 * ) != FLAC__STREAM_DECODER_INIT_STATUS_OK) do_something;
 * \endcode
 *
 * Another small change to the decoder is in how it handles unparseable
 * streams.  Before, when the decoder found an unparseable stream
 * (reserved for when the decoder encounters a stream from a future
 * encoder that it can't parse), it changed the state to
 * \c FLAC__STREAM_DECODER_UNPARSEABLE_STREAM.  Now the decoder instead
 * drops sync and calls the error callback with a new error code
 * \c FLAC__STREAM_DECODER_ERROR_STATUS_UNPARSEABLE_STREAM.  This is
 * more robust.  If your error callback does not discriminate on the the
 * error state, your code does not need to be changed.
 *
 * The encoder now has a new setting:
 * FLAC__stream_encoder_set_apodization().  This is for setting the
 * method used to window the data before LPC analysis.  You only need to
 * add a call to this function if the default is not suitable.   There
 * are also two new convenience functions that may be useful:
 * FLAC__metadata_object_cuesheet_calculate_cddb_id() and
 * FLAC__metadata_get_cuesheet().
 *
 * The \a bytes parameter to FLAC__StreamDecoderReadCallback,
 * FLAC__StreamEncoderReadCallback, and FLAC__StreamEncoderWriteCallback
 * is now \c size_t instead of \c uint32_t.
 */

/** \defgroup porting_1_1_3_to_1_1_4 Porting from FLAC 1.1.3 to 1.1.4
 *  \ingroup porting
 *
 *  \brief
 *  This module describes porting from FLAC 1.1.3 to FLAC 1.1.4.
 *
 * There were no changes to any of the interfaces from 1.1.3 to 1.1.4.
 * There was a slight change in the implementation of
 * FLAC__stream_encoder_set_metadata(); the function now makes a copy
 * of the \a metadata array of pointers so the client no longer needs
 * to maintain it after the call.  The objects themselves that are
 * pointed to by the array are still not copied though and must be
 * maintained until the call to FLAC__stream_encoder_finish().
 */

/** \defgroup porting_1_1_4_to_1_2_0 Porting from FLAC 1.1.4 to 1.2.0
 *  \ingroup porting
 *
 *  \brief
 *  This module describes porting from FLAC 1.1.4 to FLAC 1.2.0.
 *
 * There were only very minor changes to the interfaces from 1.1.4 to 1.2.0.
 * In libFLAC, \c FLAC__format_sample_rate_is_subset() was added.
 * In libFLAC++, \c FLAC::Decoder::Stream::get_decode_position() was added.
 *
 * Finally, value of the constant \c FLAC__FRAME_HEADER_RESERVED_LEN
 * has changed to reflect the conversion of one of the reserved bits
 * into active use.  It used to be \c 2 and now is \c 1.  However the
 * FLAC frame header length has not changed, so to skip the proper
 * number of bits, use \c FLAC__FRAME_HEADER_RESERVED_LEN +
 * \c FLAC__FRAME_HEADER_BLOCKING_STRATEGY_LEN
 */

/** \defgroup flac FLAC C API
 *
 * The FLAC C API is the interface to libFLAC, a set of structures
 * describing the components of FLAC streams, and functions for
 * encoding and decoding streams, as well as manipulating FLAC
 * metadata in files.
 *
 * You should start with the format components as all other modules
 * are dependent on it.
 */

#endif
