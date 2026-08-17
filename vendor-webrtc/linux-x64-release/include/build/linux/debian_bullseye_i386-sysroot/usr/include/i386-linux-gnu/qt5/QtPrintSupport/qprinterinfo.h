/****************************************************************************
**
** Copyright (C) 2016 The Qt Company Ltd.
** Contact: https://www.qt.io/licensing/
**
** This file is part of the QtGui module of the Qt Toolkit.
**
** $QT_BEGIN_LICENSE:LGPL$
** Commercial License Usage
** Licensees holding valid commercial Qt licenses may use this file in
** accordance with the commercial license agreement provided with the
** Software or, alternatively, in accordance with the terms contained in
** a written agreement between you and The Qt Company. For licensing terms
** and conditions see https://www.qt.io/terms-conditions. For further
** information use the contact form at https://www.qt.io/contact-us.
**
** GNU Lesser General Public License Usage
** Alternatively, this file may be used under the terms of the GNU Lesser
** General Public License version 3 as published by the Free Software
** Foundation and appearing in the file LICENSE.LGPL3 included in the
** packaging of this file. Please review the following information to
** ensure the GNU Lesser General Public License version 3 requirements
** will be met: https://www.gnu.org/licenses/lgpl-3.0.html.
**
** GNU General Public License Usage
** Alternatively, this file may be used under the terms of the GNU
** General Public License version 2.0 or (at your option) the GNU General
** Public license version 3 or any later version approved by the KDE Free
** Qt Foundation. The licenses are as published by the Free Software
** Foundation and appearing in the file LICENSE.GPL2 and LICENSE.GPL3
** included in the packaging of this file. Please review the following
** information to ensure the GNU General Public License requirements will
** be met: https://www.gnu.org/licenses/gpl-2.0.html and
** https://www.gnu.org/licenses/gpl-3.0.html.
**
** $QT_END_LICENSE$
**
****************************************************************************/

#ifndef QPRINTERINFO_H
#define QPRINTERINFO_H

#include <QtPrintSupport/qtprintsupportglobal.h>
#include <QtPrintSupport/qprinter.h>

#include <QtCore/QList>
#include <QtCore/QPair>
#include <QtGui/qpagesize.h>

QT_BEGIN_NAMESPACE


#ifndef QT_NO_PRINTER
class QPrinterInfoPrivate;
class QPrinterInfoPrivateDeleter;
class QDebug;
class Q_PRINTSUPPORT_EXPORT QPrinterInfo
{
public:
    QPrinterInfo();
    QPrinterInfo(const QPrinterInfo &other);
    explicit QPrinterInfo(const QPrinter &printer);
    ~QPrinterInfo();

    QPrinterInfo &operator=(const QPrinterInfo &other);

    QString printerName() const;
    QString description() const;
    QString location() const;
    QString makeAndModel() const;

    bool isNull() const;
    bool isDefault() const;
    bool isRemote() const;

    QPrinter::PrinterState state() const;

    QList<QPageSize> supportedPageSizes() const;
    QPageSize defaultPageSize() const;

    bool supportsCustomPageSizes() const;

    QPageSize minimumPhysicalPageSize() const;
    QPageSize maximumPhysicalPageSize() const;

#if QT_DEPRECATED_SINCE(5,3)
    QT_DEPRECATED QList<QPrinter::PaperSize> supportedPaperSizes() const;
    QT_DEPRECATED QList<QPair<QString, QSizeF> > supportedSizesWithNames() const;
#endif // QT_DEPRECATED_SINCE(5,3)

    QList<int> supportedResolutions() const;

    QPrinter::DuplexMode defaultDuplexMode() const;
    QList<QPrinter::DuplexMode> supportedDuplexModes() const;

    QPrinter::ColorMode defaultColorMode() const;
    QList<QPrinter::ColorMode> supportedColorModes() const;

    static QStringList availablePrinterNames();
    static QList<QPrinterInfo> availablePrinters();

    static QString defaultPrinterName();
    static QPrinterInfo defaultPrinter();

    static QPrinterInfo printerInfo(const QString &printerName);

private:
    explicit QPrinterInfo(const QString &name);

private:
    friend class QPlatformPrinterSupport;
#  ifndef QT_NO_DEBUG_STREAM
    friend Q_PRINTSUPPORT_EXPORT QDebug operator<<(QDebug debug, const QPrinterInfo &);
#  endif
    Q_DECLARE_PRIVATE(QPrinterInfo)
    QScopedPointer<QPrinterInfoPrivate, QPrinterInfoPrivateDeleter> d_ptr;
};

#endif // QT_NO_PRINTER

QT_END_NAMESPACE

#endif // QPRINTERINFO_H
