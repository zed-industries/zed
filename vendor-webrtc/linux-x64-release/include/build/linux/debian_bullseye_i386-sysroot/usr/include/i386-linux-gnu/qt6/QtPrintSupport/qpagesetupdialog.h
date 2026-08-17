// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QPAGESETUPDIALOG_H
#define QPAGESETUPDIALOG_H

#include <QtPrintSupport/qtprintsupportglobal.h>

#include <QtWidgets/qdialog.h>

QT_REQUIRE_CONFIG(printdialog);

QT_BEGIN_NAMESPACE

class QPrinter;
class QPageSetupDialogPrivate;

class Q_PRINTSUPPORT_EXPORT QPageSetupDialog : public QDialog
{
    Q_OBJECT
    Q_DECLARE_PRIVATE(QPageSetupDialog)

public:
    explicit QPageSetupDialog(QPrinter *printer, QWidget *parent = nullptr);
    explicit QPageSetupDialog(QWidget *parent = nullptr);
    ~QPageSetupDialog();

#if defined(Q_OS_MAC) || defined(Q_OS_WIN) || defined(Q_QDOC)
    void setVisible(bool visible) override;
#endif
    int exec() override;

    using QDialog::open;
    void open(QObject *receiver, const char *member);

    void done(int result) override;

    QPrinter *printer();
};

QT_END_NAMESPACE

#endif // QPAGESETUPDIALOG_H
