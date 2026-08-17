// Copyright (C) 2018 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

//
//  W A R N I N G
//  -------------
//
// This file is not part of the Qt API.  It exists purely as an
// implementation detail.  This header file may change from version to
// version without notice, or even be removed.
//
// We mean it.
//
// Despite its file name, this really is not a public header.
// It is an implementation detail of the private bootstrap library.
//

#if 0
// silence syncqt warnings
#pragma qt_sync_skip_header_check
#pragma qt_sync_stop_processing
#endif

#ifdef QT_BOOTSTRAPPED

#ifndef QT_NO_EXCEPTIONS
#define QT_NO_EXCEPTIONS
#endif

#define QT_NO_USING_NAMESPACE
#define QT_NO_DEPRECATED

// Keep feature-test macros in alphabetic order by feature name:
#define QT_FEATURE_alloca 1
#define QT_FEATURE_alloca_h -1
#ifdef _WIN32
# define QT_FEATURE_alloca_malloc_h 1
#else
# define QT_FEATURE_alloca_malloc_h -1
#endif
#define QT_FEATURE_cborstreamreader -1
#define QT_FEATURE_cborstreamwriter 1
#define QT_CRYPTOGRAPHICHASH_ONLY_SHA1
#define QT_FEATURE_cxx11_random (__has_include(<random>) ? 1 : -1)
#define QT_FEATURE_cxx17_filesystem -1
#define QT_NO_DATASTREAM
#define QT_FEATURE_datestring 1
#define QT_FEATURE_datetimeparser -1
#define QT_FEATURE_easingcurve -1
#define QT_FEATURE_etw -1
#if defined(__linux__) || defined(__GLIBC__)
#define QT_FEATURE_getauxval (__has_include(<sys/auxv.h>) ? 1 : -1)
#else
#define QT_FEATURE_getauxval -1
#endif
#define QT_FEATURE_getentropy -1
#define QT_NO_GEOM_VARIANT
#define QT_FEATURE_hijricalendar -1
#define QT_FEATURE_icu -1
#define QT_FEATURE_islamiccivilcalendar -1
#define QT_FEATURE_jalalicalendar -1
#define QT_FEATURE_journald -1
#define QT_FEATURE_futimens -1
#define QT_FEATURE_futimes -1
#define QT_FEATURE_future -1
#define QT_FEATURE_itemmodel -1
#define QT_FEATURE_library -1
#ifdef __linux__
# define QT_FEATURE_linkat 1
#else
# define QT_FEATURE_linkat -1
#endif
#define QT_FEATURE_lttng -1
#define QT_NO_QOBJECT
#define QT_FEATURE_process -1
#define QT_FEATURE_regularexpression 1
#ifdef __GLIBC_PREREQ
# define QT_FEATURE_renameat2 (__GLIBC_PREREQ(2, 28) ? 1 : -1)
#else
# define QT_FEATURE_renameat2 -1
#endif
#define QT_FEATURE_shortcut -1
#define QT_FEATURE_signaling_nan -1
#define QT_FEATURE_slog2 -1
#ifdef __GLIBC_PREREQ
# define QT_FEATURE_statx (__GLIBC_PREREQ(2, 28) ? 1 : -1)
#else
# define QT_FEATURE_statx -1
#endif
#define QT_FEATURE_syslog -1
#define QT_NO_SYSTEMLOCALE
#define QT_FEATURE_temporaryfile 1
#define QT_FEATURE_textdate 1
#define QT_FEATURE_thread -1
#define QT_FEATURE_timezone -1
#define QT_FEATURE_topleveldomain -1
#define QT_NO_TRANSLATION
#define QT_FEATURE_translation -1

#define QT_NO_COMPRESS

// rcc.pro will DEFINES+= this
#ifndef QT_FEATURE_zstd
#define QT_FEATURE_zstd -1
#endif

#define QT_FEATURE_commandlineparser 1
#define QT_FEATURE_settings -1

#define QT_NO_TEMPORARYFILE

#endif // QT_BOOTSTRAPPED
