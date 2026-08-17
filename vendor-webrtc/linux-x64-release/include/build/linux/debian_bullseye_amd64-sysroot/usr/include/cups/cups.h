/*
 * API definitions for CUPS.
 *
 * Copyright © 2007-2019 by Apple Inc.
 * Copyright © 1997-2007 by Easy Software Products.
 *
 * Licensed under Apache License v2.0.  See the file "LICENSE" for more
 * information.
 */

#ifndef _CUPS_CUPS_H_
#  define _CUPS_CUPS_H_

/*
 * Include necessary headers...
 */

#  include <sys/types.h>
#  if defined(_WIN32) && !defined(__CUPS_SSIZE_T_DEFINED)
#    define __CUPS_SSIZE_T_DEFINED
#    include <stddef.h>
/* Windows does not support the ssize_t type, so map it to long... */
typedef long ssize_t;			/* @private@ */
#  endif /* _WIN32 && !__CUPS_SSIZE_T_DEFINED */

#  include "file.h"
#  include "ipp.h"
#  include "language.h"
#  include "pwg.h"


/*
 * C++ magic...
 */

#  ifdef __cplusplus
extern "C" {
#  endif /* __cplusplus */


/*
 * Constants...
 */

#  define CUPS_VERSION			2.0303
#  define CUPS_VERSION_MAJOR		2
#  define CUPS_VERSION_MINOR		3
#  define CUPS_VERSION_PATCH		3

#  define CUPS_BC_FD			3
					/* Back-channel file descriptor for
					 * select/poll */
#  define CUPS_DATE_ANY			(time_t)-1
#  define CUPS_EXCLUDE_NONE		(const char *)0
#  define CUPS_FORMAT_AUTO		"application/octet-stream"
#  define CUPS_FORMAT_COMMAND		"application/vnd.cups-command"
#  define CUPS_FORMAT_JPEG		"image/jpeg"
#  define CUPS_FORMAT_PDF		"application/pdf"
#  define CUPS_FORMAT_POSTSCRIPT	"application/postscript"
#  define CUPS_FORMAT_RAW		"application/vnd.cups-raw"
#  define CUPS_FORMAT_TEXT		"text/plain"
#  define CUPS_HTTP_DEFAULT		(http_t *)0
#  define CUPS_INCLUDE_ALL		(const char *)0
#  define CUPS_JOBID_ALL		-1
#  define CUPS_JOBID_CURRENT		0
#  define CUPS_LENGTH_VARIABLE		(ssize_t)0
#  define CUPS_TIMEOUT_DEFAULT		0
#  define CUPS_WHICHJOBS_ALL		-1
#  define CUPS_WHICHJOBS_ACTIVE		0
#  define CUPS_WHICHJOBS_COMPLETED	1

/* Flags for cupsConnectDest and cupsEnumDests */
#  define CUPS_DEST_FLAGS_NONE		0x00
					/* No flags are set */
#  define CUPS_DEST_FLAGS_UNCONNECTED	0x01
					/* There is no connection */
#  define CUPS_DEST_FLAGS_MORE		0x02
					/* There are more destinations */
#  define CUPS_DEST_FLAGS_REMOVED	0x04
					/* The destination has gone away */
#  define CUPS_DEST_FLAGS_ERROR		0x08
					/* An error occurred */
#  define CUPS_DEST_FLAGS_RESOLVING	0x10
					/* The destination address is being
					 * resolved */
#  define CUPS_DEST_FLAGS_CONNECTING	0x20
					/* A connection is being established */
#  define CUPS_DEST_FLAGS_CANCELED	0x40
					/* Operation was canceled */
#  define CUPS_DEST_FLAGS_DEVICE        0x80
                                        /* For @link cupsConnectDest@: Connect to device */

/* Flags for cupsGetDestMediaByName/Size */
#  define CUPS_MEDIA_FLAGS_DEFAULT 	0x00
					/* Find the closest size supported by
					 * the printer */
#  define CUPS_MEDIA_FLAGS_BORDERLESS	0x01
					/* Find a borderless size */
#  define CUPS_MEDIA_FLAGS_DUPLEX	0x02
					/* Find a size compatible with 2-sided
					 * printing */
#  define CUPS_MEDIA_FLAGS_EXACT	0x04
					/* Find an exact match for the size */
#  define CUPS_MEDIA_FLAGS_READY	0x08
					/* If the printer supports media
					 * sensing, find the size amongst the
					 * "ready" media. */

/* Options and values */
#  define CUPS_COPIES			"copies"
#  define CUPS_COPIES_SUPPORTED		"copies-supported"

#  define CUPS_FINISHINGS		"finishings"
#  define CUPS_FINISHINGS_SUPPORTED	"finishings-supported"

#  define CUPS_FINISHINGS_BIND		"7"
#  define CUPS_FINISHINGS_COVER		"6"
#  define CUPS_FINISHINGS_FOLD		"10"
#  define CUPS_FINISHINGS_NONE		"3"
#  define CUPS_FINISHINGS_PUNCH		"5"
#  define CUPS_FINISHINGS_STAPLE	"4"
#  define CUPS_FINISHINGS_TRIM		"11"

#  define CUPS_MEDIA			"media"
#  define CUPS_MEDIA_READY		"media-ready"
#  define CUPS_MEDIA_SUPPORTED		"media-supported"

#  define CUPS_MEDIA_3X5		"na_index-3x5_3x5in"
#  define CUPS_MEDIA_4X6		"na_index-4x6_4x6in"
#  define CUPS_MEDIA_5X7		"na_5x7_5x7in"
#  define CUPS_MEDIA_8X10		"na_govt-letter_8x10in"
#  define CUPS_MEDIA_A3			"iso_a3_297x420mm"
#  define CUPS_MEDIA_A4			"iso_a4_210x297mm"
#  define CUPS_MEDIA_A5			"iso_a5_148x210mm"
#  define CUPS_MEDIA_A6			"iso_a6_105x148mm"
#  define CUPS_MEDIA_ENV10		"na_number-10_4.125x9.5in"
#  define CUPS_MEDIA_ENVDL		"iso_dl_110x220mm"
#  define CUPS_MEDIA_LEGAL		"na_legal_8.5x14in"
#  define CUPS_MEDIA_LETTER		"na_letter_8.5x11in"
#  define CUPS_MEDIA_PHOTO_L		"oe_photo-l_3.5x5in"
#  define CUPS_MEDIA_SUPERBA3		"na_super-b_13x19in"
#  define CUPS_MEDIA_TABLOID		"na_ledger_11x17in"

#  define CUPS_MEDIA_SOURCE		"media-source"
#  define CUPS_MEDIA_SOURCE_SUPPORTED	"media-source-supported"

#  define CUPS_MEDIA_SOURCE_AUTO	"auto"
#  define CUPS_MEDIA_SOURCE_MANUAL	"manual"

#  define CUPS_MEDIA_TYPE		"media-type"
#  define CUPS_MEDIA_TYPE_SUPPORTED	"media-type-supported"

#  define CUPS_MEDIA_TYPE_AUTO		"auto"
#  define CUPS_MEDIA_TYPE_ENVELOPE	"envelope"
#  define CUPS_MEDIA_TYPE_LABELS	"labels"
#  define CUPS_MEDIA_TYPE_LETTERHEAD	"stationery-letterhead"
#  define CUPS_MEDIA_TYPE_PHOTO		"photographic"
#  define CUPS_MEDIA_TYPE_PHOTO_GLOSSY	"photographic-glossy"
#  define CUPS_MEDIA_TYPE_PHOTO_MATTE	"photographic-matte"
#  define CUPS_MEDIA_TYPE_PLAIN		"stationery"
#  define CUPS_MEDIA_TYPE_TRANSPARENCY	"transparency"

#  define CUPS_NUMBER_UP		"number-up"
#  define CUPS_NUMBER_UP_SUPPORTED	"number-up-supported"

#  define CUPS_ORIENTATION		"orientation-requested"
#  define CUPS_ORIENTATION_SUPPORTED	"orientation-requested-supported"

#  define CUPS_ORIENTATION_PORTRAIT	"3"
#  define CUPS_ORIENTATION_LANDSCAPE	"4"

#  define CUPS_PRINT_COLOR_MODE		"print-color-mode"
#  define CUPS_PRINT_COLOR_MODE_SUPPORTED "print-color-mode-supported"

#  define CUPS_PRINT_COLOR_MODE_AUTO	"auto"
#  define CUPS_PRINT_COLOR_MODE_MONOCHROME "monochrome"
#  define CUPS_PRINT_COLOR_MODE_COLOR	"color"

#  define CUPS_PRINT_QUALITY		"print-quality"
#  define CUPS_PRINT_QUALITY_SUPPORTED	"print-quality-supported"

#  define CUPS_PRINT_QUALITY_DRAFT	"3"
#  define CUPS_PRINT_QUALITY_NORMAL	"4"
#  define CUPS_PRINT_QUALITY_HIGH	"5"

#  define CUPS_SIDES			"sides"
#  define CUPS_SIDES_SUPPORTED		"sides-supported"

#  define CUPS_SIDES_ONE_SIDED		"one-sided"
#  define CUPS_SIDES_TWO_SIDED_PORTRAIT	"two-sided-long-edge"
#  define CUPS_SIDES_TWO_SIDED_LANDSCAPE "two-sided-short-edge"


/*
 * Types and structures...
 */

typedef unsigned cups_ptype_t;		/* Printer type/capability bits */
enum cups_ptype_e			/* Printer type/capability bit
					 * constants */
{					/* Not a typedef'd enum so we can OR */
  CUPS_PRINTER_LOCAL = 0x0000,		/* Local printer or class */
  CUPS_PRINTER_CLASS = 0x0001,		/* Printer class */
  CUPS_PRINTER_REMOTE = 0x0002,		/* Remote printer or class */
  CUPS_PRINTER_BW = 0x0004,		/* Can do B&W printing */
  CUPS_PRINTER_COLOR = 0x0008,		/* Can do color printing */
  CUPS_PRINTER_DUPLEX = 0x0010,		/* Can do two-sided printing */
  CUPS_PRINTER_STAPLE = 0x0020,		/* Can staple output */
  CUPS_PRINTER_COPIES = 0x0040,		/* Can do copies in hardware */
  CUPS_PRINTER_COLLATE = 0x0080,	/* Can quickly collate copies */
  CUPS_PRINTER_PUNCH = 0x0100,		/* Can punch output */
  CUPS_PRINTER_COVER = 0x0200,		/* Can cover output */
  CUPS_PRINTER_BIND = 0x0400,		/* Can bind output */
  CUPS_PRINTER_SORT = 0x0800,		/* Can sort output */
  CUPS_PRINTER_SMALL = 0x1000,		/* Can print on Letter/Legal/A4-size media */
  CUPS_PRINTER_MEDIUM = 0x2000,		/* Can print on Tabloid/B/C/A3/A2-size media */
  CUPS_PRINTER_LARGE = 0x4000,		/* Can print on D/E/A1/A0-size media */
  CUPS_PRINTER_VARIABLE = 0x8000,	/* Can print on rolls and custom-size media */
  CUPS_PRINTER_IMPLICIT = 0x10000,	/* Implicit class @private@
					 * @since Deprecated@ */
  CUPS_PRINTER_DEFAULT = 0x20000,	/* Default printer on network */
  CUPS_PRINTER_FAX = 0x40000,		/* Fax queue */
  CUPS_PRINTER_REJECTING = 0x80000,	/* Printer is rejecting jobs */
  CUPS_PRINTER_DELETE = 0x100000,	/* Delete printer
					 * @deprecated@ @exclude all@ */
  CUPS_PRINTER_NOT_SHARED = 0x200000,	/* Printer is not shared
					 * @since CUPS 1.2/macOS 10.5@ */
  CUPS_PRINTER_AUTHENTICATED = 0x400000,/* Printer requires authentication
					 * @since CUPS 1.2/macOS 10.5@ */
  CUPS_PRINTER_COMMANDS = 0x800000,	/* Printer supports maintenance commands
					 * @since CUPS 1.2/macOS 10.5@ */
  CUPS_PRINTER_DISCOVERED = 0x1000000,	/* Printer was discovered @since CUPS 1.2/macOS 10.5@ */
  CUPS_PRINTER_SCANNER = 0x2000000,	/* Scanner-only device
					 * @since CUPS 1.4/macOS 10.6@ @private@ */
  CUPS_PRINTER_MFP = 0x4000000,		/* Printer with scanning capabilities
					 * @since CUPS 1.4/macOS 10.6@ @private@ */
  CUPS_PRINTER_3D = 0x8000000,		/* Printer with 3D capabilities @exclude all@ @private@ */
  CUPS_PRINTER_OPTIONS = 0x6fffc	/* ~(CLASS | REMOTE | IMPLICIT |
					 * DEFAULT | FAX | REJECTING | DELETE |
					 * NOT_SHARED | AUTHENTICATED |
					 * COMMANDS | DISCOVERED) @private@ */
};

typedef struct cups_option_s		/**** Printer Options ****/
{
  char		*name;			/* Name of option */
  char		*value;			/* Value of option */
} cups_option_t;

typedef struct cups_dest_s		/**** Destination ****/
{
  char		*name,			/* Printer or class name */
		*instance;		/* Local instance name or NULL */
  int		is_default;		/* Is this printer the default? */
  int		num_options;		/* Number of options */
  cups_option_t	*options;		/* Options */
} cups_dest_t;

typedef struct _cups_dinfo_s cups_dinfo_t;
					/* Destination capability and status
					 * information @since CUPS 1.6/macOS 10.8@ */

typedef struct cups_job_s		/**** Job ****/
{
  int		id;			/* The job ID */
  char		*dest;			/* Printer or class name */
  char		*title;			/* Title/job name */
  char		*user;			/* User that submitted the job */
  char		*format;		/* Document format */
  ipp_jstate_t	state;			/* Job state */
  int		size;			/* Size in kilobytes */
  int		priority;		/* Priority (1-100) */
  time_t	completed_time;		/* Time the job was completed */
  time_t	creation_time;		/* Time the job was created */
  time_t	processing_time;	/* Time the job was processed */
} cups_job_t;

typedef struct cups_size_s		/**** Media Size @since CUPS 1.6/macOS 10.8@ ****/
{
  char		media[128];		/* Media name to use */
  int		width,			/* Width in hundredths of millimeters */
		length,			/* Length in hundredths of
					 * millimeters */
		bottom,			/* Bottom margin in hundredths of
					 * millimeters */
		left,			/* Left margin in hundredths of
					 * millimeters */
		right,			/* Right margin in hundredths of
					 * millimeters */
		top;			/* Top margin in hundredths of
					 * millimeters */
} cups_size_t;

typedef int (*cups_client_cert_cb_t)(http_t *http, void *tls,
				     cups_array_t *distinguished_names,
				     void *user_data);
					/* Client credentials callback
					 * @since CUPS 1.5/macOS 10.7@ */

typedef int (*cups_dest_cb_t)(void *user_data, unsigned flags,
			      cups_dest_t *dest);
			      		/* Destination enumeration callback
					 * @since CUPS 1.6/macOS 10.8@ */

#  ifdef __BLOCKS__
typedef int (^cups_dest_block_t)(unsigned flags, cups_dest_t *dest);
			      		/* Destination enumeration block
					 * @since CUPS 1.6/macOS 10.8@
                                         * @exclude all@ */
#  endif /* __BLOCKS__ */

typedef const char *(*cups_password_cb_t)(const char *prompt);
					/* Password callback @exclude all@ */

typedef const char *(*cups_password_cb2_t)(const char *prompt, http_t *http,
					   const char *method,
					   const char *resource,
					   void *user_data);
					/* New password callback
					 * @since CUPS 1.4/macOS 10.6@ */

typedef int (*cups_server_cert_cb_t)(http_t *http, void *tls,
				     cups_array_t *certs, void *user_data);
					/* Server credentials callback
					 * @since CUPS 1.5/macOS 10.7@ */


/*
 * Functions...
 */

extern int		cupsCancelJob(const char *name, int job_id) _CUPS_PUBLIC;
extern ipp_t		*cupsDoFileRequest(http_t *http, ipp_t *request,
			                   const char *resource,
					   const char *filename) _CUPS_PUBLIC;
extern ipp_t		*cupsDoRequest(http_t *http, ipp_t *request,
			               const char *resource) _CUPS_PUBLIC;
extern http_encryption_t cupsEncryption(void);
extern void		cupsFreeJobs(int num_jobs, cups_job_t *jobs) _CUPS_PUBLIC;
extern int		cupsGetClasses(char ***classes) _CUPS_DEPRECATED_MSG("Use cupsEnumDests instead.");
extern const char	*cupsGetDefault(void) _CUPS_PUBLIC;
extern int		cupsGetJobs(cups_job_t **jobs, const char *name,
			            int myjobs, int whichjobs) _CUPS_PUBLIC;
extern int		cupsGetPrinters(char ***printers) _CUPS_DEPRECATED_MSG("Use cupsEnumDests instead.");
extern ipp_status_t	cupsLastError(void) _CUPS_PUBLIC;
extern int		cupsPrintFile(const char *name, const char *filename,
			              const char *title, int num_options,
				      cups_option_t *options) _CUPS_PUBLIC;
extern int		cupsPrintFiles(const char *name, int num_files,
			               const char **files, const char *title,
				       int num_options, cups_option_t *options) _CUPS_PUBLIC;
extern char		*cupsTempFile(char *filename, int len) _CUPS_DEPRECATED_MSG("Use cupsTempFd or cupsTempFile2 instead.");
extern int		cupsTempFd(char *filename, int len) _CUPS_PUBLIC;

extern int		cupsAddDest(const char *name, const char *instance,
			            int num_dests, cups_dest_t **dests) _CUPS_PUBLIC;
extern void		cupsFreeDests(int num_dests, cups_dest_t *dests) _CUPS_PUBLIC;
extern cups_dest_t	*cupsGetDest(const char *name, const char *instance,
			             int num_dests, cups_dest_t *dests) _CUPS_PUBLIC;
extern int		cupsGetDests(cups_dest_t **dests) _CUPS_PUBLIC;
extern void		cupsSetDests(int num_dests, cups_dest_t *dests) _CUPS_PUBLIC;

extern int		cupsAddOption(const char *name, const char *value,
			              int num_options, cups_option_t **options) _CUPS_PUBLIC;
extern void		cupsEncodeOptions(ipp_t *ipp, int num_options,
					  cups_option_t *options) _CUPS_PUBLIC;
extern void		cupsFreeOptions(int num_options,
			                cups_option_t *options) _CUPS_PUBLIC;
extern const char	*cupsGetOption(const char *name, int num_options,
			               cups_option_t *options) _CUPS_PUBLIC;
extern int		cupsParseOptions(const char *arg, int num_options,
			                 cups_option_t **options) _CUPS_PUBLIC;

extern const char	*cupsGetPassword(const char *prompt) _CUPS_PUBLIC;
extern const char	*cupsServer(void) _CUPS_PUBLIC;
extern void		cupsSetEncryption(http_encryption_t e) _CUPS_PUBLIC;
extern void		cupsSetPasswordCB(cups_password_cb_t cb) _CUPS_PUBLIC;
extern void		cupsSetServer(const char *server) _CUPS_PUBLIC;
extern void		cupsSetUser(const char *user) _CUPS_PUBLIC;
extern const char	*cupsUser(void) _CUPS_PUBLIC;

/**** New in CUPS 1.1.20 ****/
extern int		cupsDoAuthentication(http_t *http, const char *method,
			                     const char *resource)
			                     _CUPS_API_1_1_20;
extern http_status_t	cupsGetFile(http_t *http, const char *resource,
			            const char *filename) _CUPS_API_1_1_20;
extern http_status_t	cupsGetFd(http_t *http, const char *resource, int fd) _CUPS_API_1_1_20;
extern http_status_t	cupsPutFile(http_t *http, const char *resource,
			            const char *filename) _CUPS_API_1_1_20;
extern http_status_t	cupsPutFd(http_t *http, const char *resource, int fd)
			          _CUPS_API_1_1_20;

/**** New in CUPS 1.1.21 ****/
extern const char	*cupsGetDefault2(http_t *http) _CUPS_API_1_1_21;
extern int		cupsGetDests2(http_t *http, cups_dest_t **dests)
			              _CUPS_API_1_1_21;
extern int		cupsGetJobs2(http_t *http, cups_job_t **jobs,
			             const char *name, int myjobs,
				     int whichjobs) _CUPS_API_1_1_21;
extern int		cupsPrintFile2(http_t *http, const char *name,
			               const char *filename,
				       const char *title, int num_options,
				       cups_option_t *options) _CUPS_API_1_1_21;
extern int		cupsPrintFiles2(http_t *http, const char *name,
			                int num_files, const char **files,
					const char *title, int num_options,
					cups_option_t *options)
					_CUPS_API_1_1_21;
extern int		cupsSetDests2(http_t *http, int num_dests,
			              cups_dest_t *dests) _CUPS_API_1_1_21;

/**** New in CUPS 1.2/macOS 10.5 ****/
extern void		cupsEncodeOptions2(ipp_t *ipp, int num_options,
					   cups_option_t *options,
					   ipp_tag_t group_tag) _CUPS_API_1_2;
extern const char	*cupsLastErrorString(void) _CUPS_API_1_2;
extern char		*cupsNotifySubject(cups_lang_t *lang, ipp_t *event)
			                   _CUPS_API_1_2;
extern char		*cupsNotifyText(cups_lang_t *lang, ipp_t *event)
			                _CUPS_API_1_2;
extern int		cupsRemoveOption(const char *name, int num_options,
			                 cups_option_t **options) _CUPS_API_1_2;
extern cups_file_t	*cupsTempFile2(char *filename, int len) _CUPS_API_1_2;

/**** New in CUPS 1.3/macOS 10.5 ****/
extern ipp_t		*cupsDoIORequest(http_t *http, ipp_t *request,
			                 const char *resource, int infile,
					 int outfile) _CUPS_API_1_3;
extern int		cupsRemoveDest(const char *name,
			               const char *instance,
				       int num_dests, cups_dest_t **dests)
				       _CUPS_API_1_3;
extern void		cupsSetDefaultDest(const char *name,
			                   const char *instance,
					   int num_dests,
					   cups_dest_t *dests) _CUPS_API_1_3;

/**** New in CUPS 1.4/macOS 10.6 ****/
extern ipp_status_t	cupsCancelJob2(http_t *http, const char *name,
			               int job_id, int purge) _CUPS_API_1_4;
extern int		cupsCreateJob(http_t *http, const char *name,
				      const char *title, int num_options,
				      cups_option_t *options) _CUPS_API_1_4;
extern ipp_status_t	cupsFinishDocument(http_t *http,
			                   const char *name) _CUPS_API_1_4;
extern cups_dest_t	*cupsGetNamedDest(http_t *http, const char *name,
			                  const char *instance) _CUPS_API_1_4;
extern const char	*cupsGetPassword2(const char *prompt, http_t *http,
					  const char *method,
					  const char *resource) _CUPS_API_1_4;
extern ipp_t		*cupsGetResponse(http_t *http,
			                 const char *resource) _CUPS_API_1_4;
extern ssize_t		cupsReadResponseData(http_t *http, char *buffer,
			                     size_t length) _CUPS_API_1_4;
extern http_status_t	cupsSendRequest(http_t *http, ipp_t *request,
			                const char *resource,
					size_t length) _CUPS_API_1_4;
extern void		cupsSetPasswordCB2(cups_password_cb2_t cb,
			                   void *user_data) _CUPS_API_1_4;
extern http_status_t	cupsStartDocument(http_t *http, const char *name,
			                  int job_id, const char *docname,
					  const char *format,
					  int last_document) _CUPS_API_1_4;
extern http_status_t	cupsWriteRequestData(http_t *http, const char *buffer,
			                     size_t length) _CUPS_API_1_4;

/**** New in CUPS 1.5/macOS 10.7 ****/
extern void		cupsSetClientCertCB(cups_client_cert_cb_t cb,
					    void *user_data) _CUPS_API_1_5;
extern int		cupsSetCredentials(cups_array_t *certs) _CUPS_API_1_5;
extern void		cupsSetServerCertCB(cups_server_cert_cb_t cb,
					    void *user_data) _CUPS_API_1_5;

/**** New in CUPS 1.6/macOS 10.8 ****/
extern ipp_status_t	cupsCancelDestJob(http_t *http, cups_dest_t *dest,
			                  int job_id) _CUPS_API_1_6;
extern int		cupsCheckDestSupported(http_t *http, cups_dest_t *dest,
					       cups_dinfo_t *info,
			                       const char *option,
					       const char *value) _CUPS_API_1_6;
extern ipp_status_t	cupsCloseDestJob(http_t *http, cups_dest_t *dest,
			                 cups_dinfo_t *info, int job_id)
			                 _CUPS_API_1_6;
extern http_t		*cupsConnectDest(cups_dest_t *dest, unsigned flags,
			                 int msec, int *cancel,
					 char *resource, size_t resourcesize,
					 cups_dest_cb_t cb, void *user_data)
					 _CUPS_API_1_6;
#  ifdef __BLOCKS__
extern http_t		*cupsConnectDestBlock(cups_dest_t *dest,
					      unsigned flags, int msec,
					      int *cancel, char *resource,
					      size_t resourcesize,
					      cups_dest_block_t block)
					      _CUPS_API_1_6;
#  endif /* __BLOCKS__ */
extern int		cupsCopyDest(cups_dest_t *dest, int num_dests,
			             cups_dest_t **dests) _CUPS_API_1_6;
extern cups_dinfo_t	*cupsCopyDestInfo(http_t *http, cups_dest_t *dest)
					  _CUPS_API_1_6;
extern int		cupsCopyDestConflicts(http_t *http, cups_dest_t *dest,
					      cups_dinfo_t *info,
					      int num_options,
					      cups_option_t *options,
					      const char *new_option,
					      const char *new_value,
					      int *num_conflicts,
					      cups_option_t **conflicts,
					      int *num_resolved,
					      cups_option_t **resolved)
					      _CUPS_API_1_6;
extern ipp_status_t	cupsCreateDestJob(http_t *http, cups_dest_t *dest,
					  cups_dinfo_t *info, int *job_id,
					  const char *title, int num_options,
			                  cups_option_t *options) _CUPS_API_1_6;
extern int		cupsEnumDests(unsigned flags, int msec, int *cancel,
				      cups_ptype_t type, cups_ptype_t mask,
				      cups_dest_cb_t cb, void *user_data)
				      _CUPS_API_1_6;
#  ifdef __BLOCKS__
extern int		cupsEnumDestsBlock(unsigned flags, int msec,
					   int *cancel, cups_ptype_t type,
					   cups_ptype_t mask,
					   cups_dest_block_t block)
					   _CUPS_API_1_6;
#  endif /* __BLOCKS__ */
extern ipp_status_t	cupsFinishDestDocument(http_t *http,
			                       cups_dest_t *dest,
			                       cups_dinfo_t *info)
			                       _CUPS_API_1_6;
extern void		cupsFreeDestInfo(cups_dinfo_t *dinfo) _CUPS_API_1_6;
extern int		cupsGetDestMediaByName(http_t *http, cups_dest_t *dest,
			                       cups_dinfo_t *dinfo,
					       const char *media,
					       unsigned flags,
					       cups_size_t *size) _CUPS_API_1_6;
extern int		cupsGetDestMediaBySize(http_t *http, cups_dest_t *dest,
			                       cups_dinfo_t *dinfo,
					       int width, int length,
					       unsigned flags,
					       cups_size_t *size) _CUPS_API_1_6;
extern const char	*cupsLocalizeDestOption(http_t *http, cups_dest_t *dest,
			                        cups_dinfo_t *info,
			                        const char *option)
			                        _CUPS_API_1_6;
extern const char	*cupsLocalizeDestValue(http_t *http, cups_dest_t *dest,
					       cups_dinfo_t *info,
					       const char *option,
					       const char *value)
					       _CUPS_API_1_6;
extern http_status_t	cupsStartDestDocument(http_t *http, cups_dest_t *dest,
					      cups_dinfo_t *info, int job_id,
					      const char *docname,
					      const char *format,
					      int num_options,
					      cups_option_t *options,
					      int last_document) _CUPS_API_1_6;

/* New in CUPS 1.7 */
extern ipp_attribute_t	*cupsFindDestDefault(http_t *http, cups_dest_t *dest,
			                     cups_dinfo_t *dinfo,
			                     const char *option) _CUPS_API_1_7;
extern ipp_attribute_t	*cupsFindDestReady(http_t *http, cups_dest_t *dest,
					   cups_dinfo_t *dinfo,
					   const char *option) _CUPS_API_1_7;
extern ipp_attribute_t	*cupsFindDestSupported(http_t *http, cups_dest_t *dest,
			                       cups_dinfo_t *dinfo,
			                       const char *option)
			                       _CUPS_API_1_7;
extern int		cupsGetDestMediaByIndex(http_t *http, cups_dest_t *dest,
			                        cups_dinfo_t *dinfo, int n,
			                        unsigned flags,
			                        cups_size_t *size)
			                        _CUPS_API_1_7;
extern int		cupsGetDestMediaCount(http_t *http, cups_dest_t *dest,
			                      cups_dinfo_t *dinfo,
			                      unsigned flags) _CUPS_API_1_7;
extern int		cupsGetDestMediaDefault(http_t *http, cups_dest_t *dest,
			                        cups_dinfo_t *dinfo,
			                        unsigned flags,
			                        cups_size_t *size)
			                        _CUPS_API_1_7;
extern void		cupsSetUserAgent(const char *user_agent) _CUPS_API_1_7;
extern const char	*cupsUserAgent(void) _CUPS_API_1_7;

/* New in CUPS 2.0/macOS 10.10 */
extern cups_dest_t	*cupsGetDestWithURI(const char *name, const char *uri) _CUPS_API_2_0;
extern const char	*cupsLocalizeDestMedia(http_t *http, cups_dest_t *dest, cups_dinfo_t *info, unsigned flags, cups_size_t *size) _CUPS_API_2_0;
extern int		cupsMakeServerCredentials(const char *path, const char *common_name, int num_alt_names, const char **alt_names, time_t expiration_date) _CUPS_API_2_0;
extern int		cupsSetServerCredentials(const char *path, const char *common_name, int auto_create) _CUPS_API_2_0;

/* New in CUPS 2.2/macOS 10.12 */
extern ssize_t		cupsHashData(const char *algorithm, const void *data, size_t datalen, unsigned char *hash, size_t hashsize) _CUPS_API_2_2;

/* New in CUPS 2.2.4 */
extern int		cupsAddIntegerOption(const char *name, int value, int num_options, cups_option_t **options) _CUPS_API_2_2_4;
extern int		cupsGetIntegerOption(const char *name, int num_options, cups_option_t *options) _CUPS_API_2_2_4;

/* New in CUPS 2.2.7 */
extern const char	*cupsHashString(const unsigned char *hash, size_t hashsize, char *buffer, size_t bufsize) _CUPS_API_2_2_7;

/* New in CUPS 2.3 */
extern int		cupsAddDestMediaOptions(http_t *http, cups_dest_t *dest, cups_dinfo_t *dinfo, unsigned flags, cups_size_t *size, int num_options, cups_option_t **options) _CUPS_API_2_3;
extern ipp_attribute_t	*cupsEncodeOption(ipp_t *ipp, ipp_tag_t group_tag, const char *name, const char *value) _CUPS_API_2_3;

#  ifdef __cplusplus
}
#  endif /* __cplusplus */

#endif /* !_CUPS_CUPS_H_ */
