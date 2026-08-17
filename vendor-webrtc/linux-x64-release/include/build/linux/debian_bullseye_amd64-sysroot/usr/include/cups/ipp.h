/*
 * Internet Printing Protocol definitions for CUPS.
 *
 * Copyright © 2007-2018 by Apple Inc.
 * Copyright © 1997-2006 by Easy Software Products.
 *
 * Licensed under Apache License v2.0.  See the file "LICENSE" for more
 * information.
 */

#ifndef _CUPS_IPP_H_
#  define _CUPS_IPP_H_

/*
 * Include necessary headers...
 */

#  include "http.h"
#  include <stdarg.h>


/*
 * C++ magic...
 */

#  ifdef __cplusplus
extern "C" {
#  endif /* __cplusplus */


/*
 * IPP version string...
 */

#  define IPP_VERSION		"\002\001"

/*
 * IPP registered port number...
 *
 * Note: Applications should never use IPP_PORT, but instead use the
 * ippPort() function to allow overrides via the IPP_PORT environment
 * variable and services file if needed!
 */

#  define IPP_PORT		631

/*
 * Common limits...
 */

#  define IPP_MAX_CHARSET	64	/* Maximum length of charset values w/nul */
#  define IPP_MAX_KEYWORD	256	/* Maximum length of keyword values w/nul */
#  define IPP_MAX_LANGUAGE	64	/* Maximum length of naturalLanguage values w/nul */
#  define IPP_MAX_LENGTH	32767	/* Maximum size of any single value */
#  define IPP_MAX_MIMETYPE	256	/* Maximum length of mimeMediaType values w/nul */
#  define IPP_MAX_NAME		256	/* Maximum length of common name values w/nul */
#  define IPP_MAX_OCTETSTRING	1023	/* Maximum length of octetString values w/o nul */
#  define IPP_MAX_TEXT		1024	/* Maximum length of text values w/nul */
#  define IPP_MAX_URI		1024	/* Maximum length of uri values w/nul */
#  define IPP_MAX_URISCHEME	64	/* Maximum length of uriScheme values w/nul */
#  define IPP_MAX_VALUES	8	/* Power-of-2 allocation increment */

/*
 * Macro to flag a text string attribute as "const" (static storage) vs.
 * allocated.
 */

#  define IPP_CONST_TAG(x) (ipp_tag_t)(IPP_TAG_CUPS_CONST | (x))


/*
 * Types and structures...
 */

typedef enum ipp_dstate_e		/**** Document states @exclude all@ ****/
{
  IPP_DSTATE_PENDING = 3,		/* Document is pending */
  IPP_DSTATE_PROCESSING = 5,		/* Document is processing */
  IPP_DSTATE_CANCELED = 7,		/* Document is canceled */
  IPP_DSTATE_ABORTED,			/* Document is aborted */
  IPP_DSTATE_COMPLETED			/* Document is completed */

#  ifndef _CUPS_NO_DEPRECATED
#    define IPP_DOCUMENT_PENDING	IPP_DSTATE_PENDING
#    define IPP_DOCUMENT_PROCESSING	IPP_DSTATE_PROCESSING
#    define IPP_DOCUMENT_CANCELED	IPP_DSTATE_CANCELED
#    define IPP_DOCUMENT_ABORTED	IPP_DSTATE_ABORTED
#    define IPP_DOCUMENT_COMPLETED	IPP_DSTATE_COMPLETED
#  endif /* !_CUPS_NO_DEPRECATED */
} ipp_dstate_t;

typedef enum ipp_finishings_e		/**** Finishings values ****/
{
  IPP_FINISHINGS_NONE = 3,		/* No finishing */
  IPP_FINISHINGS_STAPLE,		/* Staple (any location/method) */
  IPP_FINISHINGS_PUNCH,			/* Punch (any location/count) */
  IPP_FINISHINGS_COVER,			/* Add cover */
  IPP_FINISHINGS_BIND,			/* Bind */
  IPP_FINISHINGS_SADDLE_STITCH,		/* Staple interior */
  IPP_FINISHINGS_EDGE_STITCH,		/* Stitch along any side */
  IPP_FINISHINGS_FOLD,			/* Fold (any type) */
  IPP_FINISHINGS_TRIM,			/* Trim (any type) */
  IPP_FINISHINGS_BALE,			/* Bale (any type) */
  IPP_FINISHINGS_BOOKLET_MAKER,		/* Fold to make booklet */
  IPP_FINISHINGS_JOG_OFFSET,		/* Offset for binding (any type) */
  IPP_FINISHINGS_COAT,			/* Apply protective liquid or powder coating */
  IPP_FINISHINGS_LAMINATE,		/* Apply protective (solid) material */
  IPP_FINISHINGS_STAPLE_TOP_LEFT = 20,	/* Staple top left corner */
  IPP_FINISHINGS_STAPLE_BOTTOM_LEFT,	/* Staple bottom left corner */
  IPP_FINISHINGS_STAPLE_TOP_RIGHT,	/* Staple top right corner */
  IPP_FINISHINGS_STAPLE_BOTTOM_RIGHT,	/* Staple bottom right corner */
  IPP_FINISHINGS_EDGE_STITCH_LEFT,	/* Stitch along left side */
  IPP_FINISHINGS_EDGE_STITCH_TOP,	/* Stitch along top edge */
  IPP_FINISHINGS_EDGE_STITCH_RIGHT,	/* Stitch along right side */
  IPP_FINISHINGS_EDGE_STITCH_BOTTOM,	/* Stitch along bottom edge */
  IPP_FINISHINGS_STAPLE_DUAL_LEFT,	/* Two staples on left */
  IPP_FINISHINGS_STAPLE_DUAL_TOP,	/* Two staples on top */
  IPP_FINISHINGS_STAPLE_DUAL_RIGHT,	/* Two staples on right */
  IPP_FINISHINGS_STAPLE_DUAL_BOTTOM,	/* Two staples on bottom */
  IPP_FINISHINGS_STAPLE_TRIPLE_LEFT,	/* Three staples on left */
  IPP_FINISHINGS_STAPLE_TRIPLE_TOP,	/* Three staples on top */
  IPP_FINISHINGS_STAPLE_TRIPLE_RIGHT,	/* Three staples on right */
  IPP_FINISHINGS_STAPLE_TRIPLE_BOTTOM,	/* Three staples on bottom */
  IPP_FINISHINGS_BIND_LEFT = 50,	/* Bind on left */
  IPP_FINISHINGS_BIND_TOP,		/* Bind on top */
  IPP_FINISHINGS_BIND_RIGHT,		/* Bind on right */
  IPP_FINISHINGS_BIND_BOTTOM,		/* Bind on bottom */
  IPP_FINISHINGS_TRIM_AFTER_PAGES = 60,	/* Trim output after each page */
  IPP_FINISHINGS_TRIM_AFTER_DOCUMENTS,	/* Trim output after each document */
  IPP_FINISHINGS_TRIM_AFTER_COPIES,	/* Trim output after each copy */
  IPP_FINISHINGS_TRIM_AFTER_JOB,	/* Trim output after job */
  IPP_FINISHINGS_PUNCH_TOP_LEFT = 70,	/* Punch 1 hole top left */
  IPP_FINISHINGS_PUNCH_BOTTOM_LEFT,	/* Punch 1 hole bottom left */
  IPP_FINISHINGS_PUNCH_TOP_RIGHT,	/* Punch 1 hole top right */
  IPP_FINISHINGS_PUNCH_BOTTOM_RIGHT,	/* Punch 1 hole bottom right */
  IPP_FINISHINGS_PUNCH_DUAL_LEFT,	/* Punch 2 holes left side */
  IPP_FINISHINGS_PUNCH_DUAL_TOP,	/* Punch 2 holes top edge */
  IPP_FINISHINGS_PUNCH_DUAL_RIGHT,	/* Punch 2 holes right side */
  IPP_FINISHINGS_PUNCH_DUAL_BOTTOM,	/* Punch 2 holes bottom edge */
  IPP_FINISHINGS_PUNCH_TRIPLE_LEFT,	/* Punch 3 holes left side */
  IPP_FINISHINGS_PUNCH_TRIPLE_TOP,	/* Punch 3 holes top edge */
  IPP_FINISHINGS_PUNCH_TRIPLE_RIGHT,	/* Punch 3 holes right side */
  IPP_FINISHINGS_PUNCH_TRIPLE_BOTTOM,	/* Punch 3 holes bottom edge */
  IPP_FINISHINGS_PUNCH_QUAD_LEFT,	/* Punch 4 holes left side */
  IPP_FINISHINGS_PUNCH_QUAD_TOP,	/* Punch 4 holes top edge */
  IPP_FINISHINGS_PUNCH_QUAD_RIGHT,	/* Punch 4 holes right side */
  IPP_FINISHINGS_PUNCH_QUAD_BOTTOM,	/* Punch 4 holes bottom edge */
  IPP_FINISHINGS_PUNCH_MULTIPLE_LEFT,	/* Punch multiple holes left side */
  IPP_FINISHINGS_PUNCH_MULTIPLE_TOP,	/* Punch multiple holes top edge */
  IPP_FINISHINGS_PUNCH_MULTIPLE_RIGHT,	/* Punch multiple holes right side */
  IPP_FINISHINGS_PUNCH_MULTIPLE_BOTTOM,	/* Punch multiple holes bottom edge */
  IPP_FINISHINGS_FOLD_ACCORDION = 90,	/* Accordion-fold the paper vertically into four sections */
  IPP_FINISHINGS_FOLD_DOUBLE_GATE,	/* Fold the top and bottom quarters of the paper towards the midline, then fold in half vertically */
  IPP_FINISHINGS_FOLD_GATE,		/* Fold the top and bottom quarters of the paper towards the midline */
  IPP_FINISHINGS_FOLD_HALF,		/* Fold the paper in half vertically */
  IPP_FINISHINGS_FOLD_HALF_Z,		/* Fold the paper in half horizontally, then Z-fold the paper vertically */
  IPP_FINISHINGS_FOLD_LEFT_GATE,	/* Fold the top quarter of the paper towards the midline */
  IPP_FINISHINGS_FOLD_LETTER,		/* Fold the paper into three sections vertically; sometimes also known as a C fold*/
  IPP_FINISHINGS_FOLD_PARALLEL,		/* Fold the paper in half vertically two times, yielding four sections */
  IPP_FINISHINGS_FOLD_POSTER,		/* Fold the paper in half horizontally and vertically; sometimes also called a cross fold */
  IPP_FINISHINGS_FOLD_RIGHT_GATE,	/* Fold the bottom quarter of the paper towards the midline */
  IPP_FINISHINGS_FOLD_Z,		/* Fold the paper vertically into three sections, forming a Z */
  IPP_FINISHINGS_FOLD_ENGINEERING_Z,	/* Fold the paper vertically into two small sections and one larger, forming an elongated Z */

  /* CUPS extensions for finishings (pre-standard versions of values above) */
  IPP_FINISHINGS_CUPS_PUNCH_TOP_LEFT = 0x40000046,
					/* Punch 1 hole top left @exclude all@ */
  IPP_FINISHINGS_CUPS_PUNCH_BOTTOM_LEFT,/* Punch 1 hole bottom left @exclude all@ */
  IPP_FINISHINGS_CUPS_PUNCH_TOP_RIGHT,	/* Punch 1 hole top right @exclude all@ */
  IPP_FINISHINGS_CUPS_PUNCH_BOTTOM_RIGHT,
					/* Punch 1 hole bottom right @exclude all@ */
  IPP_FINISHINGS_CUPS_PUNCH_DUAL_LEFT,	/* Punch 2 holes left side @exclude all@ */
  IPP_FINISHINGS_CUPS_PUNCH_DUAL_TOP,	/* Punch 2 holes top edge @exclude all@ */
  IPP_FINISHINGS_CUPS_PUNCH_DUAL_RIGHT,	/* Punch 2 holes right side @exclude all@ */
  IPP_FINISHINGS_CUPS_PUNCH_DUAL_BOTTOM,/* Punch 2 holes bottom edge @exclude all@ */
  IPP_FINISHINGS_CUPS_PUNCH_TRIPLE_LEFT,/* Punch 3 holes left side @exclude all@ */
  IPP_FINISHINGS_CUPS_PUNCH_TRIPLE_TOP,	/* Punch 3 holes top edge @exclude all@ */
  IPP_FINISHINGS_CUPS_PUNCH_TRIPLE_RIGHT,
					/* Punch 3 holes right side @exclude all@ */
  IPP_FINISHINGS_CUPS_PUNCH_TRIPLE_BOTTOM,
					/* Punch 3 holes bottom edge @exclude all@ */
  IPP_FINISHINGS_CUPS_PUNCH_QUAD_LEFT,	/* Punch 4 holes left side @exclude all@ */
  IPP_FINISHINGS_CUPS_PUNCH_QUAD_TOP,	/* Punch 4 holes top edge @exclude all@ */
  IPP_FINISHINGS_CUPS_PUNCH_QUAD_RIGHT,	/* Punch 4 holes right side @exclude all@ */
  IPP_FINISHINGS_CUPS_PUNCH_QUAD_BOTTOM,/* Punch 4 holes bottom edge @exclude all@ */

  IPP_FINISHINGS_CUPS_FOLD_ACCORDION = 0x4000005A,
					/* Accordion-fold the paper vertically into four sections @exclude all@ */
  IPP_FINISHINGS_CUPS_FOLD_DOUBLE_GATE,	/* Fold the top and bottom quarters of the paper towards the midline, then fold in half vertically @exclude all@ */
  IPP_FINISHINGS_CUPS_FOLD_GATE,	/* Fold the top and bottom quarters of the paper towards the midline @exclude all@ */
  IPP_FINISHINGS_CUPS_FOLD_HALF,	/* Fold the paper in half vertically @exclude all@ */
  IPP_FINISHINGS_CUPS_FOLD_HALF_Z,	/* Fold the paper in half horizontally, then Z-fold the paper vertically @exclude all@ */
  IPP_FINISHINGS_CUPS_FOLD_LEFT_GATE,	/* Fold the top quarter of the paper towards the midline @exclude all@ */
  IPP_FINISHINGS_CUPS_FOLD_LETTER,	/* Fold the paper into three sections vertically; sometimes also known as a C fold @exclude all@ */
  IPP_FINISHINGS_CUPS_FOLD_PARALLEL,	/* Fold the paper in half vertically two times, yielding four sections @exclude all@ */
  IPP_FINISHINGS_CUPS_FOLD_POSTER,	/* Fold the paper in half horizontally and vertically; sometimes also called a cross fold @exclude all@ */
  IPP_FINISHINGS_CUPS_FOLD_RIGHT_GATE,	/* Fold the bottom quarter of the paper towards the midline @exclude all@ */
  IPP_FINISHINGS_CUPS_FOLD_Z		/* Fold the paper vertically into three sections, forming a Z @exclude all@ */
} ipp_finishings_t;
#  ifndef _CUPS_NO_DEPRECATED
#    define IPP_FINISHINGS_CUPS_FOLD_ACCORDIAN IPP_FINISHINGS_CUPS_FOLD_ACCORDION
#    define IPP_FINISHINGS_FOLD_ACCORDIAN IPP_FINISHINGS_FOLD_ACCORDION
#    define IPP_FINISHINGS_JOB_OFFSET	IPP_FINISHINGS_JOG_OFFSET
					/* Long-time misspellings... */
typedef enum ipp_finishings_e ipp_finish_t;
#  endif /* !_CUPS_NO_DEPRECATED */

typedef enum ipp_jcollate_e		/**** Job collation types @deprecated@ @exclude all@ ****/
{
  IPP_JCOLLATE_UNCOLLATED_SHEETS = 3,
  IPP_JCOLLATE_COLLATED_DOCUMENTS,
  IPP_JCOLLATE_UNCOLLATED_DOCUMENTS

#  ifndef _CUPS_NO_DEPRECATED
#    define IPP_JOB_UNCOLLATED_SHEETS		IPP_JCOLLATE_UNCOLLATED_SHEETS
#    define IPP_JOB_COLLATED_DOCUMENTS		IPP_JCOLLATE_COLLATED_DOCUMENTS
#    define IPP_JOB_UNCOLLATED_DOCUMENTS	IPP_JCOLLATE_UNCOLLATED_DOCUMENTS
#  endif /* !_CUPS_NO_DEPRECATED */
} ipp_jcollate_t;

typedef enum ipp_jstate_e		/**** Job states ****/
{
  IPP_JSTATE_PENDING = 3,		/* Job is waiting to be printed */
  IPP_JSTATE_HELD,			/* Job is held for printing */
  IPP_JSTATE_PROCESSING,		/* Job is currently printing */
  IPP_JSTATE_STOPPED,			/* Job has been stopped */
  IPP_JSTATE_CANCELED,			/* Job has been canceled */
  IPP_JSTATE_ABORTED,			/* Job has aborted due to error */
  IPP_JSTATE_COMPLETED			/* Job has completed successfully */

#  ifndef _CUPS_NO_DEPRECATED
#    define IPP_JOB_PENDING	IPP_JSTATE_PENDING
#    define IPP_JOB_HELD	IPP_JSTATE_HELD
#    define IPP_JOB_PROCESSING	IPP_JSTATE_PROCESSING
#    define IPP_JOB_STOPPED	IPP_JSTATE_STOPPED
#    define IPP_JOB_CANCELED	IPP_JSTATE_CANCELED
#    define IPP_JOB_ABORTED	IPP_JSTATE_ABORTED
#    define IPP_JOB_COMPLETED	IPP_JSTATE_COMPLETED
  /* Legacy name for canceled state */
#    define IPP_JOB_CANCELLED	IPP_JSTATE_CANCELED
#  endif /* !_CUPS_NO_DEPRECATED */
} ipp_jstate_t;

typedef enum ipp_op_e			/**** IPP operations ****/
{
  IPP_OP_CUPS_INVALID = -1,		/* Invalid operation name for @link ippOpValue@ */
  IPP_OP_CUPS_NONE = 0,			/* No operation @private@ */
  IPP_OP_PRINT_JOB = 0x0002,		/* Print-Job: Print a single file */
  IPP_OP_PRINT_URI,			/* Print-URI: Print a single URL @exclude all@ */
  IPP_OP_VALIDATE_JOB,			/* Validate-Job: Validate job values prior to submission */
  IPP_OP_CREATE_JOB,			/* Create-Job: Create an empty print job */
  IPP_OP_SEND_DOCUMENT,			/* Send-Document: Add a file to a job */
  IPP_OP_SEND_URI,			/* Send-URI: Add a URL to a job @exclude all@ */
  IPP_OP_CANCEL_JOB,			/* Cancel-Job: Cancel a job */
  IPP_OP_GET_JOB_ATTRIBUTES,		/* Get-Job-Attribute: Get information about a job */
  IPP_OP_GET_JOBS,			/* Get-Jobs: Get a list of jobs */
  IPP_OP_GET_PRINTER_ATTRIBUTES,	/* Get-Printer-Attributes: Get information about a printer */
  IPP_OP_HOLD_JOB,			/* Hold-Job: Hold a job for printing */
  IPP_OP_RELEASE_JOB,			/* Release-Job: Release a job for printing */
  IPP_OP_RESTART_JOB,			/* Restart-Job: Reprint a job @deprecated@ */
  IPP_OP_PAUSE_PRINTER = 0x0010,	/* Pause-Printer: Stop a printer */
  IPP_OP_RESUME_PRINTER,		/* Resume-Printer: Start a printer */
  IPP_OP_PURGE_JOBS,			/* Purge-Jobs: Delete all jobs @deprecated@ @exclude all@ */
  IPP_OP_SET_PRINTER_ATTRIBUTES,	/* Set-Printer-Attributes: Set printer values */
  IPP_OP_SET_JOB_ATTRIBUTES,		/* Set-Job-Attributes: Set job values */
  IPP_OP_GET_PRINTER_SUPPORTED_VALUES,	/* Get-Printer-Supported-Values: Get supported values */
  IPP_OP_CREATE_PRINTER_SUBSCRIPTIONS,	/* Create-Printer-Subscriptions: Create one or more printer subscriptions @since CUPS 1.2/macOS 10.5@ */
  IPP_OP_CREATE_JOB_SUBSCRIPTIONS,	/* Create-Job-Subscriptions: Create one of more job subscriptions @since CUPS 1.2/macOS 10.5@ */
  IPP_OP_GET_SUBSCRIPTION_ATTRIBUTES,	/* Get-Subscription-Attributes: Get subscription information @since CUPS 1.2/macOS 10.5@ */
  IPP_OP_GET_SUBSCRIPTIONS,		/* Get-Subscriptions: Get list of subscriptions @since CUPS 1.2/macOS 10.5@ */
  IPP_OP_RENEW_SUBSCRIPTION,		/* Renew-Subscription: Renew a printer subscription @since CUPS 1.2/macOS 10.5@ */
  IPP_OP_CANCEL_SUBSCRIPTION,		/* Cancel-Subscription: Cancel a subscription @since CUPS 1.2/macOS 10.5@ */
  IPP_OP_GET_NOTIFICATIONS,		/* Get-Notifications: Get notification events @since CUPS 1.2/macOS 10.5@ */
  IPP_OP_SEND_NOTIFICATIONS,		/* Send-Notifications: Send notification events @private@ */
  IPP_OP_GET_RESOURCE_ATTRIBUTES,	/* Get-Resource-Attributes: Get resource information @private@ */
  IPP_OP_GET_RESOURCE_DATA,		/* Get-Resource-Data: Get resource data @private@ @deprecated@ */
  IPP_OP_GET_RESOURCES,			/* Get-Resources: Get list of resources @private@ */
  IPP_OP_GET_PRINT_SUPPORT_FILES,	/* Get-Printer-Support-Files: Get printer support files @private@ */
  IPP_OP_ENABLE_PRINTER,		/* Enable-Printer: Accept new jobs for a printer */
  IPP_OP_DISABLE_PRINTER,		/* Disable-Printer: Reject new jobs for a printer */
  IPP_OP_PAUSE_PRINTER_AFTER_CURRENT_JOB,
					/* Pause-Printer-After-Current-Job: Stop printer after the current job */
  IPP_OP_HOLD_NEW_JOBS,			/* Hold-New-Jobs: Hold new jobs */
  IPP_OP_RELEASE_HELD_NEW_JOBS,		/* Release-Held-New-Jobs: Release new jobs that were previously held */
  IPP_OP_DEACTIVATE_PRINTER,		/* Deactivate-Printer: Stop a printer and do not accept jobs @deprecated@ @exclude all@ */
  IPP_OP_ACTIVATE_PRINTER,		/* Activate-Printer: Start a printer and accept jobs @deprecated@ @exclude all@ */
  IPP_OP_RESTART_PRINTER,		/* Restart-Printer: Restart a printer @exclude all@ */
  IPP_OP_SHUTDOWN_PRINTER,		/* Shutdown-Printer: Turn a printer off @exclude all@ */
  IPP_OP_STARTUP_PRINTER,		/* Startup-Printer: Turn a printer on @exclude all@ */
  IPP_OP_REPROCESS_JOB,			/* Reprocess-Job: Reprint a job @deprecated@ @exclude all@*/
  IPP_OP_CANCEL_CURRENT_JOB,		/* Cancel-Current-Job: Cancel the current job */
  IPP_OP_SUSPEND_CURRENT_JOB,		/* Suspend-Current-Job: Suspend the current job */
  IPP_OP_RESUME_JOB,			/* Resume-Job: Resume the current job */
  IPP_OP_PROMOTE_JOB,			/* Promote-Job: Promote a job to print sooner */
  IPP_OP_SCHEDULE_JOB_AFTER,		/* Schedule-Job-After: Schedule a job to print after another */
  IPP_OP_CANCEL_DOCUMENT = 0x0033,	/* Cancel-Document: Cancel a document @exclude all@ */
  IPP_OP_GET_DOCUMENT_ATTRIBUTES,	/* Get-Document-Attributes: Get document information @exclude all@ */
  IPP_OP_GET_DOCUMENTS,			/* Get-Documents: Get a list of documents in a job @exclude all@ */
  IPP_OP_DELETE_DOCUMENT,		/* Delete-Document: Delete a document @deprecated@  @exclude all@ */
  IPP_OP_SET_DOCUMENT_ATTRIBUTES,	/* Set-Document-Attributes: Set document values @exclude all@ */
  IPP_OP_CANCEL_JOBS,			/* Cancel-Jobs: Cancel all jobs (administrative) */
  IPP_OP_CANCEL_MY_JOBS,		/* Cancel-My-Jobs: Cancel a user's jobs */
  IPP_OP_RESUBMIT_JOB,			/* Resubmit-Job: Copy and reprint a job @exclude all@ */
  IPP_OP_CLOSE_JOB,			/* Close-Job: Close a job and start printing */
  IPP_OP_IDENTIFY_PRINTER,		/* Identify-Printer: Make the printer beep, flash, or display a message for identification */
  IPP_OP_VALIDATE_DOCUMENT,		/* Validate-Document: Validate document values prior to submission @exclude all@ */
  IPP_OP_ADD_DOCUMENT_IMAGES,		/* Add-Document-Images: Add image(s) from the specified scanner source @exclude all@ */
  IPP_OP_ACKNOWLEDGE_DOCUMENT,		/* Acknowledge-Document: Acknowledge processing of a document @exclude all@ */
  IPP_OP_ACKNOWLEDGE_IDENTIFY_PRINTER,	/* Acknowledge-Identify-Printer: Acknowledge action on an Identify-Printer request @exclude all@ */
  IPP_OP_ACKNOWLEDGE_JOB,		/* Acknowledge-Job: Acknowledge processing of a job @exclude all@ */
  IPP_OP_FETCH_DOCUMENT,		/* Fetch-Document: Fetch a document for processing @exclude all@ */
  IPP_OP_FETCH_JOB,			/* Fetch-Job: Fetch a job for processing @exclude all@ */
  IPP_OP_GET_OUTPUT_DEVICE_ATTRIBUTES,	/* Get-Output-Device-Attributes: Get printer information for a specific output device @exclude all@ */
  IPP_OP_UPDATE_ACTIVE_JOBS,		/* Update-Active-Jobs: Update the list of active jobs that a proxy has processed @exclude all@ */
  IPP_OP_DEREGISTER_OUTPUT_DEVICE,	/* Deregister-Output-Device: Remove an output device @exclude all@ */
  IPP_OP_UPDATE_DOCUMENT_STATUS,	/* Update-Document-Status: Update document values @exclude all@ */
  IPP_OP_UPDATE_JOB_STATUS,		/* Update-Job-Status: Update job values @exclude all@ */
  IPP_OP_UPDATE_OUTPUT_DEVICE_ATTRIBUTES,
					/* Update-Output-Device-Attributes: Update output device values @exclude all@ */
  IPP_OP_GET_NEXT_DOCUMENT_DATA,	/* Get-Next-Document-Data: Scan more document data @exclude all@ */
  IPP_OP_ALLOCATE_PRINTER_RESOURCES,    /* Allocate-Printer-Resources: Use resources for a printer. */
  IPP_OP_CREATE_PRINTER,                /* Create-Printer: Create a new service. */
  IPP_OP_DEALLOCATE_PRINTER_RESOURCES,  /* Deallocate-Printer-Resources: Stop using resources for a printer. */
  IPP_OP_DELETE_PRINTER,                /* Delete-Printer: Delete an existing service. */
  IPP_OP_GET_PRINTERS,                  /* Get-Printers: Get a list of services. */
  IPP_OP_SHUTDOWN_ONE_PRINTER,          /* Shutdown-One-Printer: Shutdown a service. */
  IPP_OP_STARTUP_ONE_PRINTER,           /* Startup-One-Printer: Start a service. */
  IPP_OP_CANCEL_RESOURCE,               /* Cancel-Resource: Uninstall a resource. */
  IPP_OP_CREATE_RESOURCE,               /* Create-Resource: Create a new (empty) resource. */
  IPP_OP_INSTALL_RESOURCE,              /* Install-Resource: Install a resource. */
  IPP_OP_SEND_RESOURCE_DATA,            /* Send-Resource-Data: Upload the data for a resource. */
  IPP_OP_SET_RESOURCE_ATTRIBUTES,       /* Set-Resource-Attributes: Set resource object  attributes. */
  IPP_OP_CREATE_RESOURCE_SUBSCRIPTIONS, /* Create-Resource-Subscriptions: Create event subscriptions for a resource. */
  IPP_OP_CREATE_SYSTEM_SUBSCRIPTIONS,   /* Create-System-Subscriptions: Create event subscriptions for a system. */
  IPP_OP_DISABLE_ALL_PRINTERS,          /* Disable-All-Printers: Stop accepting new jobs on all services. */
  IPP_OP_ENABLE_ALL_PRINTERS,           /* Enable-All-Printers: Start accepting new jobs on all services. */
  IPP_OP_GET_SYSTEM_ATTRIBUTES,         /* Get-System-Attributes: Get system object attributes. */
  IPP_OP_GET_SYSTEM_SUPPORTED_VALUES,   /* Get-System-Supported-Values: Get supported values for system object attributes. */
  IPP_OP_PAUSE_ALL_PRINTERS,            /* Pause-All-Printers: Stop all services immediately. */
  IPP_OP_PAUSE_ALL_PRINTERS_AFTER_CURRENT_JOB,
                                        /* Pause-All-Printers-After-Current-Job: Stop all services after processing the current jobs. */
  IPP_OP_REGISTER_OUTPUT_DEVICE,        /* Register-Output-Device: Register a remote service. */
  IPP_OP_RESTART_SYSTEM,                /* Restart-System: Restart all services. */
  IPP_OP_RESUME_ALL_PRINTERS,           /* Resume-All-Printers: Start job processing on all services. */
  IPP_OP_SET_SYSTEM_ATTRIBUTES,         /* Set-System-Attributes: Set system object attributes. */
  IPP_OP_SHUTDOWN_ALL_PRINTERS,         /* Shutdown-All-Printers: Shutdown all services. */
  IPP_OP_STARTUP_ALL_PRINTERS,          /* Startup-All-Printers: Startup all services. */

  IPP_OP_PRIVATE = 0x4000,		/* Reserved @private@ */
  IPP_OP_CUPS_GET_DEFAULT,		/* CUPS-Get-Default: Get the default printer */
  IPP_OP_CUPS_GET_PRINTERS,		/* CUPS-Get-Printers: Get a list of printers and/or classes */
  IPP_OP_CUPS_ADD_MODIFY_PRINTER,	/* CUPS-Add-Modify-Printer: Add or modify a printer */
  IPP_OP_CUPS_DELETE_PRINTER,		/* CUPS-Delete-Printer: Delete a printer */
  IPP_OP_CUPS_GET_CLASSES,		/* CUPS-Get-Classes: Get a list of classes @deprecated@ @exclude all@ */
  IPP_OP_CUPS_ADD_MODIFY_CLASS,		/* CUPS-Add-Modify-Class: Add or modify a class */
  IPP_OP_CUPS_DELETE_CLASS,		/* CUPS-Delete-Class: Delete a class */
  IPP_OP_CUPS_ACCEPT_JOBS,		/* CUPS-Accept-Jobs: Accept new jobs on a printer @exclude all@ */
  IPP_OP_CUPS_REJECT_JOBS,		/* CUPS-Reject-Jobs: Reject new jobs on a printer @exclude all@ */
  IPP_OP_CUPS_SET_DEFAULT,		/* CUPS-Set-Default: Set the default printer */
  IPP_OP_CUPS_GET_DEVICES,		/* CUPS-Get-Devices: Get a list of supported devices @deprecated@ */
  IPP_OP_CUPS_GET_PPDS,			/* CUPS-Get-PPDs: Get a list of supported drivers @deprecated@ */
  IPP_OP_CUPS_MOVE_JOB,			/* CUPS-Move-Job: Move a job to a different printer */
  IPP_OP_CUPS_AUTHENTICATE_JOB,		/* CUPS-Authenticate-Job: Authenticate a job @since CUPS 1.2/macOS 10.5@ */
  IPP_OP_CUPS_GET_PPD,			/* CUPS-Get-PPD: Get a PPD file @deprecated@ */
  IPP_OP_CUPS_GET_DOCUMENT = 0x4027,	/* CUPS-Get-Document: Get a document file @since CUPS 1.4/macOS 10.6@ */
  IPP_OP_CUPS_CREATE_LOCAL_PRINTER	/* CUPS-Create-Local-Printer: Create a local (temporary) printer @since CUPS 2.2@ */

#  ifndef _CUPS_NO_DEPRECATED
#    define IPP_PRINT_JOB			IPP_OP_PRINT_JOB
#    define IPP_PRINT_URI			IPP_OP_PRINT_URI
#    define IPP_VALIDATE_JOB			IPP_OP_VALIDATE_JOB
#    define IPP_CREATE_JOB			IPP_OP_CREATE_JOB
#    define IPP_SEND_DOCUMENT			IPP_OP_SEND_DOCUMENT
#    define IPP_SEND_URI			IPP_OP_SEND_URI
#    define IPP_CANCEL_JOB			IPP_OP_CANCEL_JOB
#    define IPP_GET_JOB_ATTRIBUTES		IPP_OP_GET_JOB_ATTRIBUTES
#    define IPP_GET_JOBS			IPP_OP_GET_JOBS
#    define IPP_GET_PRINTER_ATTRIBUTES		IPP_OP_GET_PRINTER_ATTRIBUTES
#    define IPP_HOLD_JOB			IPP_OP_HOLD_JOB
#    define IPP_RELEASE_JOB			IPP_OP_RELEASE_JOB
#    define IPP_RESTART_JOB			IPP_OP_RESTART_JOB
#    define IPP_PAUSE_PRINTER			IPP_OP_PAUSE_PRINTER
#    define IPP_RESUME_PRINTER			IPP_OP_RESUME_PRINTER
#    define IPP_PURGE_JOBS			IPP_OP_PURGE_JOBS
#    define IPP_SET_PRINTER_ATTRIBUTES		IPP_OP_SET_PRINTER_ATTRIBUTES
#    define IPP_SET_JOB_ATTRIBUTES		IPP_OP_SET_JOB_ATTRIBUTES
#    define IPP_GET_PRINTER_SUPPORTED_VALUES	IPP_OP_GET_PRINTER_SUPPORTED_VALUES
#    define IPP_CREATE_PRINTER_SUBSCRIPTION	IPP_OP_CREATE_PRINTER_SUBSCRIPTIONS
#    define IPP_CREATE_JOB_SUBSCRIPTION		IPP_OP_CREATE_JOB_SUBSCRIPTIONS
#    define IPP_OP_CREATE_PRINTER_SUBSCRIPTION	IPP_OP_CREATE_PRINTER_SUBSCRIPTIONS
#    define IPP_OP_CREATE_JOB_SUBSCRIPTION	IPP_OP_CREATE_JOB_SUBSCRIPTIONS
#    define IPP_GET_SUBSCRIPTION_ATTRIBUTES	IPP_OP_GET_SUBSCRIPTION_ATTRIBUTES
#    define IPP_GET_SUBSCRIPTIONS		IPP_OP_GET_SUBSCRIPTIONS
#    define IPP_RENEW_SUBSCRIPTION		IPP_OP_RENEW_SUBSCRIPTION
#    define IPP_CANCEL_SUBSCRIPTION		IPP_OP_CANCEL_SUBSCRIPTION
#    define IPP_GET_NOTIFICATIONS		IPP_OP_GET_NOTIFICATIONS
#    define IPP_SEND_NOTIFICATIONS		IPP_OP_SEND_NOTIFICATIONS
#    define IPP_GET_RESOURCE_ATTRIBUTES		IPP_OP_GET_RESOURCE_ATTRIBUTES
#    define IPP_GET_RESOURCE_DATA		IPP_OP_GET_RESOURCE_DATA
#    define IPP_GET_RESOURCES			IPP_OP_GET_RESOURCES
#    define IPP_GET_PRINT_SUPPORT_FILES		IPP_OP_GET_PRINT_SUPPORT_FILES
#    define IPP_ENABLE_PRINTER			IPP_OP_ENABLE_PRINTER
#    define IPP_DISABLE_PRINTER			IPP_OP_DISABLE_PRINTER
#    define IPP_PAUSE_PRINTER_AFTER_CURRENT_JOB	IPP_OP_PAUSE_PRINTER_AFTER_CURRENT_JOB
#    define IPP_HOLD_NEW_JOBS			IPP_OP_HOLD_NEW_JOBS
#    define IPP_RELEASE_HELD_NEW_JOBS		IPP_OP_RELEASE_HELD_NEW_JOBS
#    define IPP_DEACTIVATE_PRINTER		IPP_OP_DEACTIVATE_PRINTER
#    define IPP_ACTIVATE_PRINTER		IPP_OP_ACTIVATE_PRINTER
#    define IPP_RESTART_PRINTER			IPP_OP_RESTART_PRINTER
#    define IPP_SHUTDOWN_PRINTER		IPP_OP_SHUTDOWN_PRINTER
#    define IPP_STARTUP_PRINTER			IPP_OP_STARTUP_PRINTER
#    define IPP_REPROCESS_JOB			IPP_OP_REPROCESS_JOB
#    define IPP_CANCEL_CURRENT_JOB		IPP_OP_CANCEL_CURRENT_JOB
#    define IPP_SUSPEND_CURRENT_JOB		IPP_OP_SUSPEND_CURRENT_JOB
#    define IPP_RESUME_JOB			IPP_OP_RESUME_JOB
#    define IPP_PROMOTE_JOB			IPP_OP_PROMOTE_JOB
#    define IPP_SCHEDULE_JOB_AFTER		IPP_OP_SCHEDULE_JOB_AFTER
#    define IPP_CANCEL_DOCUMENT			IPP_OP_CANCEL_DOCUMENT
#    define IPP_GET_DOCUMENT_ATTRIBUTES		IPP_OP_GET_DOCUMENT_ATTRIBUTES
#    define IPP_GET_DOCUMENTS			IPP_OP_GET_DOCUMENTS
#    define IPP_DELETE_DOCUMENT			IPP_OP_DELETE_DOCUMENT
#    define IPP_SET_DOCUMENT_ATTRIBUTES		IPP_OP_SET_DOCUMENT_ATTRIBUTES
#    define IPP_CANCEL_JOBS			IPP_OP_CANCEL_JOBS
#    define IPP_CANCEL_MY_JOBS			IPP_OP_CANCEL_MY_JOBS
#    define IPP_RESUBMIT_JOB			IPP_OP_RESUBMIT_JOB
#    define IPP_CLOSE_JOB			IPP_OP_CLOSE_JOB
#    define IPP_IDENTIFY_PRINTER		IPP_OP_IDENTIFY_PRINTER
#    define IPP_VALIDATE_DOCUMENT		IPP_OP_VALIDATE_DOCUMENT
#    define IPP_OP_SEND_HARDCOPY_DOCUMENT	IPP_OP_ADD_DOCUMENT_IMAGES
#    define IPP_PRIVATE				IPP_OP_PRIVATE
#    define CUPS_GET_DEFAULT			IPP_OP_CUPS_GET_DEFAULT
#    define CUPS_GET_PRINTERS			IPP_OP_CUPS_GET_PRINTERS
#    define CUPS_ADD_MODIFY_PRINTER		IPP_OP_CUPS_ADD_MODIFY_PRINTER
#    define CUPS_DELETE_PRINTER			IPP_OP_CUPS_DELETE_PRINTER
#    define CUPS_GET_CLASSES			IPP_OP_CUPS_GET_CLASSES
#    define CUPS_ADD_MODIFY_CLASS		IPP_OP_CUPS_ADD_MODIFY_CLASS
#    define CUPS_DELETE_CLASS			IPP_OP_CUPS_DELETE_CLASS
#    define CUPS_ACCEPT_JOBS			IPP_OP_CUPS_ACCEPT_JOBS
#    define CUPS_REJECT_JOBS			IPP_OP_CUPS_REJECT_JOBS
#    define CUPS_SET_DEFAULT			IPP_OP_CUPS_SET_DEFAULT
#    define CUPS_GET_DEVICES			IPP_OP_CUPS_GET_DEVICES
#    define CUPS_GET_PPDS			IPP_OP_CUPS_GET_PPDS
#    define CUPS_MOVE_JOB			IPP_OP_CUPS_MOVE_JOB
#    define CUPS_AUTHENTICATE_JOB		IPP_OP_CUPS_AUTHENTICATE_JOB
#    define CUPS_GET_PPD			IPP_OP_CUPS_GET_PPD
#    define CUPS_GET_DOCUMENT			IPP_OP_CUPS_GET_DOCUMENT
     /* Legacy names */
#    define CUPS_ADD_PRINTER			IPP_OP_CUPS_ADD_MODIFY_PRINTER
#    define CUPS_ADD_CLASS			IPP_OP_CUPS_ADD_MODIFY_CLASS
#  endif /* !_CUPS_NO_DEPRECATED */
} ipp_op_t;

typedef enum ipp_orient_e		/**** Orientation values ****/
{
  IPP_ORIENT_PORTRAIT = 3,		/* No rotation */
  IPP_ORIENT_LANDSCAPE,			/* 90 degrees counter-clockwise */
  IPP_ORIENT_REVERSE_LANDSCAPE,		/* 90 degrees clockwise */
  IPP_ORIENT_REVERSE_PORTRAIT,		/* 180 degrees */
  IPP_ORIENT_NONE			/* No rotation */

#  ifndef _CUPS_NO_DEPRECATED
#    define IPP_PORTRAIT		IPP_ORIENT_PORTRAIT
#    define IPP_LANDSCAPE		IPP_ORIENT_LANDSCAPE
#    define IPP_REVERSE_LANDSCAPE	IPP_ORIENT_REVERSE_LANDSCAPE
#    define IPP_REVERSE_PORTRAIT	IPP_ORIENT_REVERSE_PORTRAIT
#  endif /* !_CUPS_NO_DEPRECATED */
} ipp_orient_t;

typedef enum ipp_pstate_e		/**** Printer state values ****/
{
  IPP_PSTATE_IDLE = 3,			/* Printer is idle */
  IPP_PSTATE_PROCESSING,		/* Printer is working */
  IPP_PSTATE_STOPPED			/* Printer is stopped */

#  ifndef _CUPS_NO_DEPRECATED
#    define IPP_PRINTER_IDLE		IPP_PSTATE_IDLE
#    define IPP_PRINTER_PROCESSING	IPP_PSTATE_PROCESSING
#    define IPP_PRINTER_STOPPED		IPP_PSTATE_STOPPED
#  endif /* _CUPS_NO_DEPRECATED */
} ipp_pstate_t;

typedef enum ipp_quality_e		/**** Print quality values ****/
{
  IPP_QUALITY_DRAFT = 3,		/* Draft quality */
  IPP_QUALITY_NORMAL,			/* Normal quality */
  IPP_QUALITY_HIGH			/* High quality */
} ipp_quality_t;

typedef enum ipp_res_e			/**** Resolution units ****/
{
  IPP_RES_PER_INCH = 3,			/* Pixels per inch */
  IPP_RES_PER_CM			/* Pixels per centimeter */
} ipp_res_t;

typedef enum ipp_rstate_e		/**** resource-state values ****/
{
  IPP_RSTATE_PENDING = 3,		/* Resource is created but has no data yet. */
  IPP_RSTATE_AVAILABLE,			/* Resource is available for installation. */
  IPP_RSTATE_INSTALLED,			/* Resource is installed.  */
  IPP_RSTATE_CANCELED,			/* Resource has been canceled and is pending deletion. */
  IPP_RSTATE_ABORTED			/* Resource has been aborted and is pending deletion. */
} ipp_rstate_t;

typedef enum ipp_sstate_e		/**** system-state values ****/
{
  IPP_SSTATE_IDLE = 3,			/* At least one printer is idle and none are processing a job. */
  IPP_SSTATE_PROCESSING,		/* At least one printer is processing a job. */
  IPP_SSTATE_STOPPED			/* All printers are stopped. */
} ipp_sstate_t;

typedef enum ipp_state_e		/**** ipp_t state values ****/
{
  IPP_STATE_ERROR = -1,			/* An error occurred */
  IPP_STATE_IDLE,			/* Nothing is happening/request completed */
  IPP_STATE_HEADER,			/* The request header needs to be sent/received */
  IPP_STATE_ATTRIBUTE,			/* One or more attributes need to be sent/received */
  IPP_STATE_DATA			/* IPP request data needs to be sent/received */

#  ifndef _CUPS_NO_DEPRECATED
#    define IPP_ERROR		IPP_STATE_ERROR
#    define IPP_IDLE		IPP_STATE_IDLE
#    define IPP_HEADER		IPP_STATE_HEADER
#    define IPP_ATTRIBUTE	IPP_STATE_ATTRIBUTE
#    define IPP_DATA		IPP_STATE_DATA
#  endif /* !_CUPS_NO_DEPRECATED */
} ipp_state_t;

typedef enum ipp_status_e		/**** IPP status code values ****/
{
  IPP_STATUS_CUPS_INVALID = -1,		/* Invalid status name for @link ippErrorValue@ */
  IPP_STATUS_OK = 0x0000,		/* successful-ok */
  IPP_STATUS_OK_IGNORED_OR_SUBSTITUTED,	/* successful-ok-ignored-or-substituted-attributes */
  IPP_STATUS_OK_CONFLICTING,		/* successful-ok-conflicting-attributes */
  IPP_STATUS_OK_IGNORED_SUBSCRIPTIONS,	/* successful-ok-ignored-subscriptions */
  IPP_STATUS_OK_IGNORED_NOTIFICATIONS,	/* successful-ok-ignored-notifications @private@ */
  IPP_STATUS_OK_TOO_MANY_EVENTS,	/* successful-ok-too-many-events */
  IPP_STATUS_OK_BUT_CANCEL_SUBSCRIPTION,/* successful-ok-but-cancel-subscription @private@ */
  IPP_STATUS_OK_EVENTS_COMPLETE,	/* successful-ok-events-complete */
  IPP_STATUS_REDIRECTION_OTHER_SITE = 0x0200,
					/* redirection-other-site @private@ */
  IPP_STATUS_CUPS_SEE_OTHER = 0x0280,	/* cups-see-other @private@ */
  IPP_STATUS_ERROR_BAD_REQUEST = 0x0400,/* client-error-bad-request */
  IPP_STATUS_ERROR_FORBIDDEN,		/* client-error-forbidden */
  IPP_STATUS_ERROR_NOT_AUTHENTICATED,	/* client-error-not-authenticated */
  IPP_STATUS_ERROR_NOT_AUTHORIZED,	/* client-error-not-authorized */
  IPP_STATUS_ERROR_NOT_POSSIBLE,	/* client-error-not-possible */
  IPP_STATUS_ERROR_TIMEOUT,		/* client-error-timeout */
  IPP_STATUS_ERROR_NOT_FOUND,		/* client-error-not-found */
  IPP_STATUS_ERROR_GONE,		/* client-error-gone */
  IPP_STATUS_ERROR_REQUEST_ENTITY,	/* client-error-request-entity-too-large */
  IPP_STATUS_ERROR_REQUEST_VALUE,	/* client-error-request-value-too-long */
  IPP_STATUS_ERROR_DOCUMENT_FORMAT_NOT_SUPPORTED,
					/* client-error-document-format-not-supported */
  IPP_STATUS_ERROR_ATTRIBUTES_OR_VALUES,/* client-error-attributes-or-values-not-supported */
  IPP_STATUS_ERROR_URI_SCHEME,		/* client-error-uri-scheme-not-supported */
  IPP_STATUS_ERROR_CHARSET,		/* client-error-charset-not-supported */
  IPP_STATUS_ERROR_CONFLICTING,		/* client-error-conflicting-attributes */
  IPP_STATUS_ERROR_COMPRESSION_NOT_SUPPORTED,
					/* client-error-compression-not-supported */
  IPP_STATUS_ERROR_COMPRESSION_ERROR,	/* client-error-compression-error */
  IPP_STATUS_ERROR_DOCUMENT_FORMAT_ERROR,
					/* client-error-document-format-error */
  IPP_STATUS_ERROR_DOCUMENT_ACCESS,	/* client-error-document-access-error */
  IPP_STATUS_ERROR_ATTRIBUTES_NOT_SETTABLE,
					/* client-error-attributes-not-settable */
  IPP_STATUS_ERROR_IGNORED_ALL_SUBSCRIPTIONS,
					/* client-error-ignored-all-subscriptions */
  IPP_STATUS_ERROR_TOO_MANY_SUBSCRIPTIONS,
					/* client-error-too-many-subscriptions */
  IPP_STATUS_ERROR_IGNORED_ALL_NOTIFICATIONS,
					/* client-error-ignored-all-notifications @private@ */
  IPP_STATUS_ERROR_PRINT_SUPPORT_FILE_NOT_FOUND,
					/* client-error-print-support-file-not-found @private@ */
  IPP_STATUS_ERROR_DOCUMENT_PASSWORD,	/* client-error-document-password-error */
  IPP_STATUS_ERROR_DOCUMENT_PERMISSION,	/* client-error-document-permission-error */
  IPP_STATUS_ERROR_DOCUMENT_SECURITY,	/* client-error-document-security-error */
  IPP_STATUS_ERROR_DOCUMENT_UNPRINTABLE,/* client-error-document-unprintable-error */
  IPP_STATUS_ERROR_ACCOUNT_INFO_NEEDED,	/* client-error-account-info-needed */
  IPP_STATUS_ERROR_ACCOUNT_CLOSED,	/* client-error-account-closed */
  IPP_STATUS_ERROR_ACCOUNT_LIMIT_REACHED,
					/* client-error-account-limit-reached */
  IPP_STATUS_ERROR_ACCOUNT_AUTHORIZATION_FAILED,
					/* client-error-account-authorization-failed */
  IPP_STATUS_ERROR_NOT_FETCHABLE,	/* client-error-not-fetchable */

  /* Legacy status codes for paid printing */
  IPP_STATUS_ERROR_CUPS_ACCOUNT_INFO_NEEDED = 0x049C,
					/* cups-error-account-info-needed @deprecated@ */
  IPP_STATUS_ERROR_CUPS_ACCOUNT_CLOSED,	/* cups-error-account-closed @deprecate@ */
  IPP_STATUS_ERROR_CUPS_ACCOUNT_LIMIT_REACHED,
					/* cups-error-account-limit-reached @deprecated@ */
  IPP_STATUS_ERROR_CUPS_ACCOUNT_AUTHORIZATION_FAILED,
					/* cups-error-account-authorization-failed @deprecated@ */

  IPP_STATUS_ERROR_INTERNAL = 0x0500,	/* server-error-internal-error */
  IPP_STATUS_ERROR_OPERATION_NOT_SUPPORTED,
					/* server-error-operation-not-supported */
  IPP_STATUS_ERROR_SERVICE_UNAVAILABLE,	/* server-error-service-unavailable */
  IPP_STATUS_ERROR_VERSION_NOT_SUPPORTED,
					/* server-error-version-not-supported */
  IPP_STATUS_ERROR_DEVICE,		/* server-error-device-error */
  IPP_STATUS_ERROR_TEMPORARY,		/* server-error-temporary-error */
  IPP_STATUS_ERROR_NOT_ACCEPTING_JOBS,	/* server-error-not-accepting-jobs */
  IPP_STATUS_ERROR_BUSY,		/* server-error-busy */
  IPP_STATUS_ERROR_JOB_CANCELED,	/* server-error-job-canceled */
  IPP_STATUS_ERROR_MULTIPLE_JOBS_NOT_SUPPORTED,
					/* server-error-multiple-document-jobs-not-supported */
  IPP_STATUS_ERROR_PRINTER_IS_DEACTIVATED,
					/* server-error-printer-is-deactivated */
  IPP_STATUS_ERROR_TOO_MANY_JOBS,	/* server-error-too-many-jobs */
  IPP_STATUS_ERROR_TOO_MANY_DOCUMENTS,	/* server-error-too-many-documents */

  /* These are internal and never sent over the wire... */
  IPP_STATUS_ERROR_CUPS_AUTHENTICATION_CANCELED = 0x1000,
					/* cups-authentication-canceled - Authentication canceled by user @since CUPS 1.5/macOS 10.7@ */
  IPP_STATUS_ERROR_CUPS_PKI,		/* cups-pki-error - Error negotiating a secure connection @since CUPS 1.5/macOS 10.7@ */
  IPP_STATUS_ERROR_CUPS_UPGRADE_REQUIRED/* cups-upgrade-required - TLS upgrade required @since CUPS 1.5/macOS 10.7@ */

#  ifndef _CUPS_NO_DEPRECATED
#    define IPP_OK				IPP_STATUS_OK
#    define IPP_OK_SUBST			IPP_STATUS_OK_IGNORED_OR_SUBSTITUTED
#    define IPP_OK_CONFLICT			IPP_STATUS_OK_CONFLICTING
#    define IPP_OK_IGNORED_SUBSCRIPTIONS	IPP_STATUS_OK_IGNORED_SUBSCRIPTIONS
#    define IPP_OK_IGNORED_NOTIFICATIONS	IPP_STATUS_OK_IGNORED_NOTIFICATIONS
#    define IPP_OK_TOO_MANY_EVENTS		IPP_STATUS_OK_TOO_MANY_EVENTS
#    define IPP_OK_BUT_CANCEL_SUBSCRIPTION	IPP_STATUS_OK_BUT_CANCEL_SUBSCRIPTION
#    define IPP_OK_EVENTS_COMPLETE		IPP_STATUS_OK_EVENTS_COMPLETE
#    define IPP_REDIRECTION_OTHER_SITE		IPP_STATUS_REDIRECTION_OTHER_SITE
#    define CUPS_SEE_OTHER			IPP_STATUS_CUPS_SEE_OTHER
#    define IPP_BAD_REQUEST			IPP_STATUS_ERROR_BAD_REQUEST
#    define IPP_FORBIDDEN			IPP_STATUS_ERROR_FORBIDDEN
#    define IPP_NOT_AUTHENTICATED		IPP_STATUS_ERROR_NOT_AUTHENTICATED
#    define IPP_NOT_AUTHORIZED			IPP_STATUS_ERROR_NOT_AUTHORIZED
#    define IPP_NOT_POSSIBLE			IPP_STATUS_ERROR_NOT_POSSIBLE
#    define IPP_TIMEOUT				IPP_STATUS_ERROR_TIMEOUT
#    define IPP_NOT_FOUND			IPP_STATUS_ERROR_NOT_FOUND
#    define IPP_GONE				IPP_STATUS_ERROR_GONE
#    define IPP_REQUEST_ENTITY			IPP_STATUS_ERROR_REQUEST_ENTITY
#    define IPP_REQUEST_VALUE			IPP_STATUS_ERROR_REQUEST_VALUE
#    define IPP_DOCUMENT_FORMAT			IPP_STATUS_ERROR_DOCUMENT_FORMAT_NOT_SUPPORTED
#    define IPP_ATTRIBUTES			IPP_STATUS_ERROR_ATTRIBUTES_OR_VALUES
#    define IPP_URI_SCHEME			IPP_STATUS_ERROR_URI_SCHEME
#    define IPP_CHARSET				IPP_STATUS_ERROR_CHARSET
#    define IPP_CONFLICT			IPP_STATUS_ERROR_CONFLICTING
#    define IPP_COMPRESSION_NOT_SUPPORTED	IPP_STATUS_ERROR_COMPRESSION_NOT_SUPPORTED
#    define IPP_COMPRESSION_ERROR		IPP_STATUS_ERROR_COMPRESSION_ERROR
#    define IPP_DOCUMENT_FORMAT_ERROR		IPP_STATUS_ERROR_DOCUMENT_FORMAT_ERROR
#    define IPP_DOCUMENT_ACCESS_ERROR		IPP_STATUS_ERROR_DOCUMENT_ACCESS
#    define IPP_ATTRIBUTES_NOT_SETTABLE		IPP_STATUS_ERROR_ATTRIBUTES_NOT_SETTABLE
#    define IPP_IGNORED_ALL_SUBSCRIPTIONS	IPP_STATUS_ERROR_IGNORED_ALL_SUBSCRIPTIONS
#    define IPP_TOO_MANY_SUBSCRIPTIONS		IPP_STATUS_ERROR_TOO_MANY_SUBSCRIPTIONS
#    define IPP_IGNORED_ALL_NOTIFICATIONS	IPP_STATUS_ERROR_IGNORED_ALL_NOTIFICATIONS
#    define IPP_PRINT_SUPPORT_FILE_NOT_FOUND	IPP_STATUS_ERROR_PRINT_SUPPORT_FILE_NOT_FOUND
#    define IPP_DOCUMENT_PASSWORD_ERROR		IPP_STATUS_ERROR_DOCUMENT_PASSWORD
#    define IPP_DOCUMENT_PERMISSION_ERROR	IPP_STATUS_ERROR_DOCUMENT_PERMISSION
#    define IPP_DOCUMENT_SECURITY_ERROR		IPP_STATUS_ERROR_DOCUMENT_SECURITY
#    define IPP_DOCUMENT_UNPRINTABLE_ERROR	IPP_STATUS_ERROR_DOCUMENT_UNPRINTABLE
#    define IPP_INTERNAL_ERROR			IPP_STATUS_ERROR_INTERNAL
#    define IPP_OPERATION_NOT_SUPPORTED		IPP_STATUS_ERROR_OPERATION_NOT_SUPPORTED
#    define IPP_SERVICE_UNAVAILABLE		IPP_STATUS_ERROR_SERVICE_UNAVAILABLE
#    define IPP_VERSION_NOT_SUPPORTED		IPP_STATUS_ERROR_VERSION_NOT_SUPPORTED
#    define IPP_DEVICE_ERROR			IPP_STATUS_ERROR_DEVICE
#    define IPP_TEMPORARY_ERROR			IPP_STATUS_ERROR_TEMPORARY
#    define IPP_NOT_ACCEPTING			IPP_STATUS_ERROR_NOT_ACCEPTING_JOBS
#    define IPP_PRINTER_BUSY			IPP_STATUS_ERROR_BUSY
#    define IPP_ERROR_JOB_CANCELED		IPP_STATUS_ERROR_JOB_CANCELED
#    define IPP_MULTIPLE_JOBS_NOT_SUPPORTED	IPP_STATUS_ERROR_MULTIPLE_JOBS_NOT_SUPPORTED
#    define IPP_PRINTER_IS_DEACTIVATED		IPP_STATUS_ERROR_PRINTER_IS_DEACTIVATED
#    define IPP_TOO_MANY_JOBS			IPP_STATUS_ERROR_TOO_MANY_JOBS
#    define IPP_TOO_MANY_DOCUMENTS		IPP_STATUS_ERROR_TOO_MANY_DOCUMENTS
#    define IPP_AUTHENTICATION_CANCELED		IPP_STATUS_ERROR_CUPS_AUTHENTICATION_CANCELED
#    define IPP_PKI_ERROR			IPP_STATUS_ERROR_CUPS_PKI
#    define IPP_UPGRADE_REQUIRED		IPP_STATUS_ERROR_CUPS_UPGRADE_REQUIRED
     /* Legacy name for canceled status */
#    define IPP_ERROR_JOB_CANCELLED		IPP_STATUS_ERROR_JOB_CANCELED
#  endif /* _CUPS_NO_DEPRECATED */
} ipp_status_t;

typedef enum ipp_tag_e			/**** Value and group tag values for attributes ****/
{
  IPP_TAG_CUPS_INVALID = -1,		/* Invalid tag name for @link ippTagValue@ */
  IPP_TAG_ZERO = 0x00,			/* Zero tag - used for separators */
  IPP_TAG_OPERATION,			/* Operation group */
  IPP_TAG_JOB,				/* Job group */
  IPP_TAG_END,				/* End-of-attributes */
  IPP_TAG_PRINTER,			/* Printer group */
  IPP_TAG_UNSUPPORTED_GROUP,		/* Unsupported attributes group */
  IPP_TAG_SUBSCRIPTION,			/* Subscription group */
  IPP_TAG_EVENT_NOTIFICATION,		/* Event group */
  IPP_TAG_RESOURCE,			/* Resource group */
  IPP_TAG_DOCUMENT,			/* Document group */
  IPP_TAG_SYSTEM,                       /* System group */
  IPP_TAG_UNSUPPORTED_VALUE = 0x10,	/* Unsupported value */
  IPP_TAG_DEFAULT,			/* Default value */
  IPP_TAG_UNKNOWN,			/* Unknown value */
  IPP_TAG_NOVALUE,			/* No-value value */
  IPP_TAG_NOTSETTABLE = 0x15,		/* Not-settable value */
  IPP_TAG_DELETEATTR,			/* Delete-attribute value */
  IPP_TAG_ADMINDEFINE,			/* Admin-defined value */
  IPP_TAG_INTEGER = 0x21,		/* Integer value */
  IPP_TAG_BOOLEAN,			/* Boolean value */
  IPP_TAG_ENUM,				/* Enumeration value */
  IPP_TAG_STRING = 0x30,		/* Octet string value */
  IPP_TAG_DATE,				/* Date/time value */
  IPP_TAG_RESOLUTION,			/* Resolution value */
  IPP_TAG_RANGE,			/* Range value */
  IPP_TAG_BEGIN_COLLECTION,		/* Beginning of collection value @exclude all@ */
  IPP_TAG_TEXTLANG,			/* Text-with-language value */
  IPP_TAG_NAMELANG,			/* Name-with-language value */
  IPP_TAG_END_COLLECTION,		/* End of collection value @exclude all@ */
  IPP_TAG_TEXT = 0x41,			/* Text value */
  IPP_TAG_NAME,				/* Name value */
  IPP_TAG_RESERVED_STRING,		/* Reserved for future string value @private@ */
  IPP_TAG_KEYWORD,			/* Keyword value */
  IPP_TAG_URI,				/* URI value */
  IPP_TAG_URISCHEME,			/* URI scheme value */
  IPP_TAG_CHARSET,			/* Character set value */
  IPP_TAG_LANGUAGE,			/* Language value */
  IPP_TAG_MIMETYPE,			/* MIME media type value */
  IPP_TAG_MEMBERNAME,			/* Collection member name value @exclude all@ */
  IPP_TAG_EXTENSION = 0x7f,		/* Extension point for 32-bit tags @exclude all@ */
  IPP_TAG_CUPS_MASK = 0x7fffffff,	/* Mask for copied attribute values @private@ */
  /* The following expression is used to avoid compiler warnings with +/-0x80000000 */
  IPP_TAG_CUPS_CONST = -0x7fffffff-1	/* Bitflag for copied/const attribute values @private@ */

#  ifndef _CUPS_NO_DEPRECATED
#    define IPP_TAG_MASK		IPP_TAG_CUPS_MASK
#    define IPP_TAG_COPY		IPP_TAG_CUPS_CONST
#  endif /* !_CUPS_NO_DEPRECATED */
} ipp_tag_t;

typedef unsigned char ipp_uchar_t;	/**** Unsigned 8-bit integer/character @exclude all@ ****/
typedef struct _ipp_s ipp_t;		/**** IPP request/response data ****/
typedef struct _ipp_attribute_s ipp_attribute_t;
					/**** IPP attribute ****/

/**** New in CUPS 1.2/macOS 10.5 ****/
typedef ssize_t	(*ipp_iocb_t)(void *context, ipp_uchar_t *buffer, size_t bytes);
					/**** ippReadIO/ippWriteIO callback function @since CUPS 1.2/macOS 10.5@ ****/

/**** New in CUPS 1.6/macOS 10.8 ****/
typedef int (*ipp_copycb_t)(void *context, ipp_t *dst, ipp_attribute_t *attr);
                                        /**** ippCopyAttributes callback function @since CUPS 1.6/macOS 10.8 ****/


/*
 * Prototypes...
 */

extern ipp_attribute_t	*ippAddBoolean(ipp_t *ipp, ipp_tag_t group,
			               const char *name, char value) _CUPS_PUBLIC;
extern ipp_attribute_t	*ippAddBooleans(ipp_t *ipp, ipp_tag_t group,
			                const char *name, int num_values,
					const char *values) _CUPS_PUBLIC;
extern ipp_attribute_t	*ippAddDate(ipp_t *ipp, ipp_tag_t group,
			            const char *name, const ipp_uchar_t *value) _CUPS_PUBLIC;
extern ipp_attribute_t	*ippAddInteger(ipp_t *ipp, ipp_tag_t group,
			               ipp_tag_t value_tag, const char *name,
				       int value) _CUPS_PUBLIC;
extern ipp_attribute_t	*ippAddIntegers(ipp_t *ipp, ipp_tag_t group,
			                ipp_tag_t value_tag, const char *name,
					int num_values, const int *values) _CUPS_PUBLIC;
extern ipp_attribute_t	*ippAddRange(ipp_t *ipp, ipp_tag_t group,
			             const char *name, int lower, int upper) _CUPS_PUBLIC;
extern ipp_attribute_t	*ippAddRanges(ipp_t *ipp, ipp_tag_t group,
			              const char *name, int num_values,
				      const int *lower, const int *upper) _CUPS_PUBLIC;
extern ipp_attribute_t	*ippAddResolution(ipp_t *ipp, ipp_tag_t group,
			                  const char *name, ipp_res_t units,
					  int xres, int yres) _CUPS_PUBLIC;
extern ipp_attribute_t	*ippAddResolutions(ipp_t *ipp, ipp_tag_t group,
			                   const char *name, int num_values,
					   ipp_res_t units, const int *xres,
					   const int *yres) _CUPS_PUBLIC;
extern ipp_attribute_t	*ippAddSeparator(ipp_t *ipp) _CUPS_PUBLIC;
extern ipp_attribute_t	*ippAddString(ipp_t *ipp, ipp_tag_t group,
			              ipp_tag_t value_tag, const char *name,
				      const char *language, const char *value) _CUPS_PUBLIC;
extern ipp_attribute_t	*ippAddStrings(ipp_t *ipp, ipp_tag_t group,
			               ipp_tag_t value_tag, const char *name,
				       int num_values, const char *language,
				       const char * const *values) _CUPS_PUBLIC;
extern time_t		ippDateToTime(const ipp_uchar_t *date) _CUPS_PUBLIC;
extern void		ippDelete(ipp_t *ipp) _CUPS_PUBLIC;
extern const char	*ippErrorString(ipp_status_t error) _CUPS_PUBLIC;
extern ipp_attribute_t	*ippFindAttribute(ipp_t *ipp, const char *name,
			                  ipp_tag_t value_tag) _CUPS_PUBLIC;
extern ipp_attribute_t	*ippFindNextAttribute(ipp_t *ipp, const char *name,
			                      ipp_tag_t value_tag) _CUPS_PUBLIC;
extern size_t		ippLength(ipp_t *ipp) _CUPS_PUBLIC;
extern ipp_t		*ippNew(void) _CUPS_PUBLIC;
extern ipp_state_t	ippRead(http_t *http, ipp_t *ipp) _CUPS_PUBLIC;
extern const ipp_uchar_t *ippTimeToDate(time_t t) _CUPS_PUBLIC;
extern ipp_state_t	ippWrite(http_t *http, ipp_t *ipp) _CUPS_PUBLIC;
extern int		ippPort(void) _CUPS_PUBLIC;
extern void		ippSetPort(int p) _CUPS_PUBLIC;

/**** New in CUPS 1.1.19 ****/
extern ipp_attribute_t	*ippAddCollection(ipp_t *ipp, ipp_tag_t group,
			                  const char *name, ipp_t *value) _CUPS_API_1_1_19;
extern ipp_attribute_t	*ippAddCollections(ipp_t *ipp, ipp_tag_t group,
			                   const char *name, int num_values,
					   const ipp_t **values) _CUPS_API_1_1_19;
extern void		ippDeleteAttribute(ipp_t *ipp, ipp_attribute_t *attr) _CUPS_API_1_1_19;
extern ipp_state_t	ippReadFile(int fd, ipp_t *ipp) _CUPS_API_1_1_19;
extern ipp_state_t	ippWriteFile(int fd, ipp_t *ipp) _CUPS_API_1_1_19;

/**** New in CUPS 1.2/macOS 10.5 ****/
extern ipp_attribute_t	*ippAddOctetString(ipp_t *ipp, ipp_tag_t group,
			                   const char *name,
					   const void *data, int datalen) _CUPS_API_1_2;
extern ipp_status_t	ippErrorValue(const char *name) _CUPS_API_1_2;
extern ipp_t		*ippNewRequest(ipp_op_t op) _CUPS_API_1_2;
extern const char	*ippOpString(ipp_op_t op) _CUPS_API_1_2;
extern ipp_op_t		ippOpValue(const char *name) _CUPS_API_1_2;
extern ipp_state_t	ippReadIO(void *src, ipp_iocb_t cb, int blocking,
			          ipp_t *parent, ipp_t *ipp) _CUPS_API_1_2;
extern ipp_state_t	ippWriteIO(void *dst, ipp_iocb_t cb, int blocking,
			           ipp_t *parent, ipp_t *ipp) _CUPS_API_1_2;

/**** New in CUPS 1.4/macOS 10.6 ****/
extern const char	*ippTagString(ipp_tag_t tag) _CUPS_API_1_4;
extern ipp_tag_t	ippTagValue(const char *name) _CUPS_API_1_4;

/**** New in CUPS 1.6/macOS 10.8 ****/
extern ipp_attribute_t	*ippAddOutOfBand(ipp_t *ipp, ipp_tag_t group,
			                 ipp_tag_t value_tag, const char *name)
			                 _CUPS_API_1_6;
extern size_t		ippAttributeString(ipp_attribute_t *attr, char *buffer,
			                   size_t bufsize) _CUPS_API_1_6;
extern ipp_attribute_t	*ippCopyAttribute(ipp_t *dst, ipp_attribute_t *attr,
			                 int quickcopy) _CUPS_API_1_6;
extern int		ippCopyAttributes(ipp_t *dst, ipp_t *src,
			                  int quickcopy, ipp_copycb_t cb,
			                  void *context) _CUPS_API_1_6;
extern int		ippDeleteValues(ipp_t *ipp, ipp_attribute_t **attr,
			                int element, int count) _CUPS_API_1_6;
extern const char	*ippEnumString(const char *attrname, int enumvalue)
			               _CUPS_API_1_6;
extern int		ippEnumValue(const char *attrname,
			             const char *enumstring) _CUPS_API_1_6;
extern ipp_attribute_t	*ippFirstAttribute(ipp_t *ipp) _CUPS_API_1_6;
extern int		ippGetBoolean(ipp_attribute_t *attr, int element)
			              _CUPS_API_1_6;
extern ipp_t		*ippGetCollection(ipp_attribute_t *attr,
			                  int element) _CUPS_API_1_6;
extern int		ippGetCount(ipp_attribute_t *attr) _CUPS_API_1_6;
extern const ipp_uchar_t *ippGetDate(ipp_attribute_t *attr, int element)
			             _CUPS_API_1_6;
extern ipp_tag_t	ippGetGroupTag(ipp_attribute_t *attr) _CUPS_API_1_6;
extern int		ippGetInteger(ipp_attribute_t *attr, int element)
			              _CUPS_API_1_6;
extern const char	*ippGetName(ipp_attribute_t *attr) _CUPS_API_1_6;
extern ipp_op_t		ippGetOperation(ipp_t *ipp) _CUPS_API_1_6;
extern int		ippGetRange(ipp_attribute_t *attr, int element,
			            int *upper) _CUPS_API_1_6;
extern int		ippGetRequestId(ipp_t *ipp) _CUPS_API_1_6;
extern int		ippGetResolution(ipp_attribute_t *attr, int element,
			                 int *yres, ipp_res_t *units)
			                 _CUPS_API_1_6;
extern ipp_state_t	ippGetState(ipp_t *ipp) _CUPS_API_1_6;
extern ipp_status_t	ippGetStatusCode(ipp_t *ipp) _CUPS_API_1_6;
extern const char	*ippGetString(ipp_attribute_t *attr, int element,
				      const char **language) _CUPS_API_1_6;
extern ipp_tag_t	ippGetValueTag(ipp_attribute_t *attr) _CUPS_API_1_6;
extern int		ippGetVersion(ipp_t *ipp, int *minor) _CUPS_API_1_6;
extern ipp_attribute_t	*ippNextAttribute(ipp_t *ipp) _CUPS_API_1_6;
extern int		ippSetBoolean(ipp_t *ipp, ipp_attribute_t **attr,
			              int element, int boolvalue) _CUPS_API_1_6;
extern int		ippSetCollection(ipp_t *ipp, ipp_attribute_t **attr,
			                 int element, ipp_t *colvalue)
			                 _CUPS_API_1_6;
extern int		ippSetDate(ipp_t *ipp, ipp_attribute_t **attr,
			            int element, const ipp_uchar_t *datevalue)
				    _CUPS_API_1_6;
extern int		ippSetGroupTag(ipp_t *ipp, ipp_attribute_t **attr,
			               ipp_tag_t group_tag) _CUPS_API_1_6;
extern int		ippSetInteger(ipp_t *ipp, ipp_attribute_t **attr,
			              int element, int intvalue) _CUPS_API_1_6;
extern int		ippSetName(ipp_t *ipp, ipp_attribute_t **attr,
			            const char *name) _CUPS_API_1_6;
extern int		ippSetOperation(ipp_t *ipp, ipp_op_t op) _CUPS_API_1_6;
extern int		ippSetRange(ipp_t *ipp, ipp_attribute_t **attr,
			            int element, int lowervalue, int uppervalue)
			            _CUPS_API_1_6;
extern int		ippSetRequestId(ipp_t *ipp, int request_id)
			                _CUPS_API_1_6;
extern int		ippSetResolution(ipp_t *ipp, ipp_attribute_t **attr,
			                 int element, ipp_res_t unitsvalue,
			                 int xresvalue, int yresvalue)
			                 _CUPS_API_1_6;
extern int		ippSetState(ipp_t *ipp, ipp_state_t state)
			            _CUPS_API_1_6;
extern int		ippSetStatusCode(ipp_t *ipp, ipp_status_t status)
			                 _CUPS_API_1_6;
extern int		ippSetString(ipp_t *ipp, ipp_attribute_t **attr,
			             int element, const char *strvalue)
			             _CUPS_API_1_6;
extern int		ippSetValueTag(ipp_t *ipp, ipp_attribute_t **attr,
			               ipp_tag_t value_tag) _CUPS_API_1_6;
extern int		ippSetVersion(ipp_t *ipp, int major, int minor)
			              _CUPS_API_1_6;

/**** New in CUPS 1.7 ****/
extern ipp_attribute_t	*ippAddStringf(ipp_t *ipp, ipp_tag_t group,
			               ipp_tag_t value_tag, const char *name,
			               const char *language, const char *format,
			               ...) _CUPS_API_1_7;
extern ipp_attribute_t	*ippAddStringfv(ipp_t *ipp, ipp_tag_t group,
					ipp_tag_t value_tag, const char *name,
					const char *language,
					const char *format, va_list ap)
					_CUPS_API_1_7;
extern int		ippContainsInteger(ipp_attribute_t *attr, int value)
			                   _CUPS_API_1_7;
extern int		ippContainsString(ipp_attribute_t *attr,
			                  const char *value) _CUPS_API_1_7;
extern cups_array_t	*ippCreateRequestedArray(ipp_t *request) _CUPS_API_1_7;
extern void		*ippGetOctetString(ipp_attribute_t *attr, int element,
			                   int *datalen) _CUPS_API_1_7;
extern ipp_t		*ippNewResponse(ipp_t *request) _CUPS_API_1_7;
extern int		ippSetOctetString(ipp_t *ipp, ipp_attribute_t **attr,
			                  int element, const void *data,
					  int datalen) _CUPS_API_1_7;
extern int		ippSetStringf(ipp_t *ipp, ipp_attribute_t **attr,
			              int element, const char *format,
				      ...) _CUPS_API_1_7;
extern int		ippSetStringfv(ipp_t *ipp, ipp_attribute_t **attr,
			               int element, const char *format,
				       va_list ap) _CUPS_API_1_7;
extern int		ippValidateAttribute(ipp_attribute_t *attr)
			                     _CUPS_API_1_7;
extern int		ippValidateAttributes(ipp_t *ipp) _CUPS_API_1_7;


/**** New in CUPS 2.0 ****/
extern const char	*ippStateString(ipp_state_t state) _CUPS_API_2_0;


/*
 * C++ magic...
 */

#  ifdef __cplusplus
}
#  endif /* __cplusplus */
#endif /* !_CUPS_IPP_H_ */
