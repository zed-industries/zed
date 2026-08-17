// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

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
