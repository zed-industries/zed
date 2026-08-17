// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QWINDOWDEFS_H
#define QWINDOWDEFS_H

#include <QtGui/qtguiglobal.h>
#include <QtCore/qobjectdefs.h>
#include <QtCore/qnamespace.h>
#include <QtCore/qcontainerfwd.h>

QT_BEGIN_NAMESPACE


// Class forward definitions

class QPaintDevice;
class QWidget;
class QWindow;
class QDialog;
class QColor;
class QPalette;
class QCursor;
class QPoint;
class QSize;
class QRect;
class QPolygon;
class QPainter;
class QRegion;
class QFont;
class QFontMetrics;
class QFontInfo;
class QPen;
class QBrush;
class QPixmap;
class QBitmap;
class QMovie;
class QImage;
class QPicture;
class QTimer;
class QTime;
class QClipboard;
class QString;
class QByteArray;
class QApplication;

typedef QList<QWidget *> QWidgetList;
typedef QList<QWindow *> QWindowList;

QT_END_NAMESPACE

// Window system dependent definitions


#if defined(Q_OS_WIN) || defined(Q_QDOC)
#  include <QtGui/qwindowdefs_win.h>
#endif // Q_OS_WIN




typedef QT_PREPEND_NAMESPACE(quintptr) WId;



QT_BEGIN_NAMESPACE

typedef QHash<WId, QWidget *> QWidgetMapper;
typedef QSet<QWidget *> QWidgetSet;

QT_END_NAMESPACE

#if defined(QT_NEEDS_QMAIN)
#define main qMain
#endif

// Global platform-independent types and functions

#endif // QWINDOWDEFS_H
