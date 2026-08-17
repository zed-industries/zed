// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QABSTRACTPRINTDIALOG_H
#define QABSTRACTPRINTDIALOG_H

#include <QtPrintSupport/qtprintsupportglobal.h>

#include <QtWidgets/qdialog.h>

QT_REQUIRE_CONFIG(printdialog);

QT_BEGIN_NAMESPACE

class QAbstractPrintDialogPrivate;
class QPrinter;

class Q_PRINTSUPPORT_EXPORT QAbstractPrintDialog : public QDialog
{
    Q_DECLARE_PRIVATE(QAbstractPrintDialog)
    Q_OBJECT

public:
    // Keep in sync with QPrinter::PrintRange
    enum PrintRange {
        AllPages,
        Selection,
        PageRange,
        CurrentPage
    };

    enum PrintDialogOption {
        PrintToFile             = 0x0001,
        PrintSelection          = 0x0002,
        PrintPageRange          = 0x0004,
        PrintShowPageSize       = 0x0008,
        PrintCollateCopies      = 0x0010,
        PrintCurrentPage        = 0x0040
    };
    Q_ENUM(PrintDialogOption)

    Q_DECLARE_FLAGS(PrintDialogOptions, PrintDialogOption)
    Q_FLAG(PrintDialogOptions)

    explicit QAbstractPrintDialog(QPrinter *printer, QWidget *parent = nullptr);
    ~QAbstractPrintDialog();

    void setOptionTabs(const QList<QWidget*> &tabs);

    void setPrintRange(PrintRange range);
    PrintRange printRange() const;

    void setMinMax(int min, int max);
    int minPage() const;
    int maxPage() const;

    void setFromTo(int fromPage, int toPage);
    int fromPage() const;
    int toPage() const;

    QPrinter *printer() const;

protected:
    QAbstractPrintDialog(QAbstractPrintDialogPrivate &ptr, QPrinter *printer, QWidget *parent = nullptr);

private:
    Q_DISABLE_COPY(QAbstractPrintDialog)

};

Q_DECLARE_OPERATORS_FOR_FLAGS(QAbstractPrintDialog::PrintDialogOptions)

QT_END_NAMESPACE

#endif // QABSTRACTPRINTDIALOG_H
