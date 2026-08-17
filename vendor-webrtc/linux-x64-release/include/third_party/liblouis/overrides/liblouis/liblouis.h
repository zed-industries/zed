/* liblouis Braille Translation and Back-Translation Library

   Based on the Linux screenreader BRLTTY, copyright (C) 1999-2006 by
   The BRLTTY Team

   Copyright (C) 2004, 2005, 2006, 2009 ViewPlus Technologies, Inc.
   www.viewplus.com and JJB Software, Inc. www.jjb-software.com

   liblouis is free software: you can redistribute it and/or modify it
   under the terms of the GNU Lesser General Public License as
   published by the Free Software Foundation, either version 3 of the
   License, or (at your option) any later version.

   liblouis is distributed in the hope that it will be useful, but
   WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
   Lesser General Public License for more details.

   You should have received a copy of the GNU Lesser General Public
   License along with this program. If not, see
   <http://www.gnu.org/licenses/>.

   Maintained by John J. Boyer john.boyer@abilitiessoft.com
   */

#ifndef __LIBLOUIS_H_
#define __LIBLOUIS_H_
#ifdef __cplusplus
extern "C"
{
#endif				/* __cplusplus */

#define widechar unsigned short int
#define formtype unsigned char

#ifdef _WIN32
#define EXPORT_CALL __stdcall
  char *EXPORT_CALL lou_getProgramPath ();
#else
#define EXPORT_CALL
#endif

  typedef enum
  {
    plain_text = 0,
    italic = 1,
    underline = 2,
    bold = 4,
    computer_braille = 8
  } typeforms;
#define comp_emph_1 italic
#define comp_emph_2 underline
#define comp_emph_3 bold

  typedef enum
  {
    noContractions = 1,
    compbrlAtCursor = 2,
    dotsIO = 4,
    comp8Dots = 8,
    pass1Only = 16,
    compbrlLeftCursor = 32,
    otherTrans = 64,
    ucBrl = 128
  } translationModes;

  char *EXPORT_CALL lou_version ();

  int EXPORT_CALL lou_charSize ();

/* Return the size of widechar */

  int EXPORT_CALL lou_translateString
    (const char *tableList,
     const widechar * inbuf,
     int *inlen,
     widechar * outbuf, int *outlen, formtype *typeform, char *spacing, 
     int 
mode);

  int EXPORT_CALL lou_translate (const char *tableList, const widechar
				 * inbuf,
				 int *inlen, widechar * outbuf, int *outlen,
				 formtype *typeform, char *spacing,
				 int *outputPos, int *inputPos,
				 int *cursorPos, int mode);

  int EXPORT_CALL lou_translatePrehyphenated (const char *tableList,
					      const widechar * inbuf,
					      int *inlen, widechar * outbuf,
					      int *outlen, formtype 
					      *typeform,
					      char *spacing, int *outputPos,
					      int *inputPos, int *cursorPos,
					      char *inputHyphens,
					      char *outputHyphens, int mode);

  int EXPORT_CALL lou_hyphenate (const char *tableList, const widechar
				 * inbuf, int inlen, char *hyphens, int mode);
  int EXPORT_CALL lou_dotsToChar (const char *tableList, widechar * inbuf,
				  widechar * outbuf, int length, int mode);
  int EXPORT_CALL lou_charToDots (const char *tableList, const widechar
				  * inbuf,
				  widechar * outbuf, int length, int mode);
  int EXPORT_CALL lou_backTranslateString (const char *tableList,
					   const widechar * inbuf,
					   int *inlen,
					   widechar * outbuf,
					   int *outlen, formtype 
					   *typeform, char
					   *spacing, int mode);

  int EXPORT_CALL lou_backTranslate (const char *tableList, const widechar
				     * inbuf,
				     int *inlen, widechar * outbuf,
				     int *outlen, formtype *typeform,
				     char *spacing, int *outputPos,
				     int *inputPos, int *cursorPos, int mode);
  void EXPORT_CALL lou_logPrint (const char *format, ...);
/* Prints error messages to a file
   @deprecated As of 2.6.0, applications using liblouis should implement
               their own logging system. */

  void EXPORT_CALL lou_logFile (const char *filename);
/* Specifies the name of the file to be used by lou_logPrint. If it is 
* not used, this file is stderr*/

  int EXPORT_CALL lou_readCharFromFile (const char *fileName, int *mode);
/*Read a character from a file, whether big-encian, little-endian or 
* ASCII8, and return it as an integer. EOF at end of file. Mode = 1 on 
* first call, any other value thereafter*/

  void EXPORT_CALL lou_logEnd ();
  /* Closes the log file so it can be read by other functions. */

  void *EXPORT_CALL lou_getTable (const char *tableList);
/* This function checks a table for errors. If none are found it loads 
* the table into memory and returns a pointer to it. if errors are found 
* it returns a null pointer. It is called by lou_translateString and 
* lou_backTranslateString and also by functions in liblouisxml
*/

void EXPORT_CALL lou_registerTableResolver (char ** (* resolver) (const char *table, const char *base));
/* Register a new table resolver. Overrides the default resolver. */

  int EXPORT_CALL lou_compileString (const char *tableList, const char
				     *inString);
  char *EXPORT_CALL lou_setDataPath (char *path);
  /* Set the path used for searching for tables and liblouisutdml files. 
   * Overrides the installation path. */

  char *EXPORT_CALL lou_getDataPath ();
  /* Get the path set in the previous function. */

//  char EXPORT_CALL * lou_getTablePaths ();
  /* Get a list of paths actually used in seraching for tables */

typedef void (*logcallback)(int level, const char *message);
  void EXPORT_CALL lou_registerLogCallback(logcallback callback);
/* Register logging callbacks
 * Set to NULL for default callback.
 */

  typedef enum
  {
    LOG_ALL = -2147483647 - 1,
    LOG_DEBUG = 10000,
    LOG_INFO = 20000,
    LOG_WARN = 30000,
    LOG_ERROR = 40000,
    LOG_FATAL = 50000,
    LOG_OFF = 2147483647
  } logLevels;
  void EXPORT_CALL lou_setLogLevel(logLevels level);
/* Set the level for logging callback to be called at */
  void EXPORT_CALL lou_free ();
/* This function should be called at the end of 
* the application to free all memory allocated by liblouis. */

#ifdef __cplusplus
}
#endif				/* __cplusplus */
#endif				/*LibLOUIS_H_ */
