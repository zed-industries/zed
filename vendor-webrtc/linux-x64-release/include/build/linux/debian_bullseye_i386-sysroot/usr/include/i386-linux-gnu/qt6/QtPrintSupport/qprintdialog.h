// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QPRINTDIALOG_H
#define QPRINTDIALOG_H

#include <QtPrintSupport/qtprintsupportglobal.h>

#include <QtPrintSupport/qabstractprintdialog.h>

QT_REQUIRE_CONFIG(printdialog);

QT_BEGIN_NAMESPACE

class QPrintDialogPrivate;
class QPushButton;
class QPrinter;

class Q_PRINTSUPPORT_EXPORT QPrintDialog : public QAbstractPrintDialog
{
    Q_OBJECT
    Q_DECLARE_PRIVATE(QPrintDialog)
    Q_PROPERTY(PrintDialogOptions options READ options WRITE setOptions)

public:
    explicit QPrintDialog(QPrinter *printer, QWidget *parent = nullptr);
    explicit QPrintDialog(QWidget *parent = nullptr);
    ~QPrintDialog();

    int exec() override;
#if defined (Q_OS_UNIX) && !defined(Q_OS_MAC)
    virtual void accept() override;
#endif
    void done(int result) override;

    void setOption(PrintDialogOption option, bool on = true);
    bool testOption(PrintDialogOption option) const;
    void setOptions(PrintDialogOptions options);
    PrintDialogOptions options() const;

#if defined(Q_OS_UNIX) || defined(Q_OS_WIN)
    void setVisible(bool visible) override;
#endif

    using QDialog::open;
    void open(QObject *receiver, const char *member);

#ifdef Q_QDOC
    QPrinter *printer();
#endif

    using QDialog::accepted;

Q_SIGNALS:
    void accepted(QPrinter *printer);

private:
#if defined (Q_OS_UNIX) && !defined(Q_OS_MAC)
    Q_PRIVATE_SLOT(d_func(), void _q_togglePageSetCombo(bool))
    Q_PRIVATE_SLOT(d_func(), void _q_collapseOrExpandDialog())
#if QT_CONFIG(messagebox)
    Q_PRIVATE_SLOT(d_func(), void _q_checkFields())
#endif // QT_CONFIG(messagebox)
    friend class QUnixPrintWidget;
# endif // Q_OS_UNIX
};

QT_END_NAMESPACE

#endif // QPRINTDIALOG_H
