/*
 * PWG media API definitions for CUPS.
 *
 * Copyright 2009-2017 by Apple Inc.
 *
 * Licensed under Apache License v2.0.  See the file "LICENSE" for more information.
 */

#ifndef _CUPS_PWG_H_
#  define _CUPS_PWG_H_


/*
 * C++ magic...
 */

#  ifdef __cplusplus
extern "C" {
#  endif /* __cplusplus */


/*
 * Macros...
 */

/* Convert from points to hundredths of millimeters */
#  define PWG_FROM_POINTS(n)	(int)(((n) * 2540 + 36) / 72)
/* Convert from hundredths of millimeters to points */
#  define PWG_TO_POINTS(n)	((n) * 72.0 / 2540.0)


/*
 * Types and structures...
 */

typedef struct pwg_map_s		/**** Map element - PPD to/from PWG @exclude all@ */
{
  char		*pwg,			/* PWG media keyword */
		*ppd;			/* PPD option keyword */
} pwg_map_t;

typedef struct pwg_media_s		/**** Common media size data ****/
{
  const char	*pwg,			/* PWG 5101.1 "self describing" name */
		*legacy,		/* IPP/ISO legacy name */
		*ppd;			/* Standard Adobe PPD name */
  int		width,			/* Width in 2540ths */
		length;			/* Length in 2540ths */
} pwg_media_t;

typedef struct pwg_size_s		/**** Size element - PPD to/from PWG @exclude all@ */
{
  pwg_map_t	map;			/* Map element */
  int		width,			/* Width in 2540ths */
		length,			/* Length in 2540ths */
		left,			/* Left margin in 2540ths */
		bottom,			/* Bottom margin in 2540ths */
		right,			/* Right margin in 2540ths */
		top;			/* Top margin in 2540ths */
} pwg_size_t;


/*
 * Functions...
 */

extern int		pwgFormatSizeName(char *keyword, size_t keysize,
					  const char *prefix, const char *name,
					  int width, int length,
					  const char *units) _CUPS_API_1_7;
extern int		pwgInitSize(pwg_size_t *size, ipp_t *job,
				    int *margins_set) _CUPS_API_1_7;
extern pwg_media_t	*pwgMediaForLegacy(const char *legacy) _CUPS_API_1_7;
extern pwg_media_t	*pwgMediaForPPD(const char *ppd) _CUPS_API_1_7;
extern pwg_media_t	*pwgMediaForPWG(const char *pwg) _CUPS_API_1_7;
extern pwg_media_t	*pwgMediaForSize(int width, int length) _CUPS_API_1_7;

#  ifdef __cplusplus
}
#  endif /* __cplusplus */

#endif /* !_CUPS_PWG_H_ */
