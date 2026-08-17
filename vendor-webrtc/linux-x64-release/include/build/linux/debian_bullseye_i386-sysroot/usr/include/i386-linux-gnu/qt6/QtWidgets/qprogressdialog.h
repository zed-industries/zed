// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QPROGRESSDIALOG_H
#define QPROGRESSDIALOG_H

#include <QtWidgets/qtwidgetsglobal.h>

#include <QtWidgets/qdialog.h>

QT_REQUIRE_CONFIG(progressdialog);

QT_BEGIN_NAMESPACE

class QPushButton;
class QLabel;
class QProgressBar;
class QTimer;
class QProgressDialogPrivate;

class Q_WIDGETS_EXPORT QProgressDialog : public QDialog
{
    Q_OBJECT
    Q_DECLARE_PRIVATE(QProgressDialog)
    Q_PROPERTY(bool wasCanceled READ wasCanceled)
    Q_PROPERTY(int minimum READ minimum WRITE setMinimum)
    Q_PROPERTY(int maximum READ maximum WRITE setMaximum)
    Q_PROPERTY(int value READ value WRITE setValue)
    Q_PROPERTY(bool autoReset READ autoReset WRITE setAutoReset)
    Q_PROPERTY(bool autoClose READ autoClose WRITE setAutoClose)
    Q_PROPERTY(int minimumDuration READ minimumDuration WRITE setMinimumDuration)
    Q_PROPERTY(QString labelText READ labelText WRITE setLabelText)

public:
    explicit QProgressDialog(QWidget *parent = nullptr, Qt::WindowFlags flags = Qt::WindowFlags());
    QProgressDialog(const QString &labelText, const QString &cancelButtonText,
                    int minimum, int maximum, QWidget *parent = nullptr,
                    Qt::WindowFlags flags = Qt::WindowFlags());
    ~QProgressDialog();

    void setLabel(QLabel *label);
    void setCancelButton(QPushButton *button);
    void setBar(QProgressBar *bar);

    bool wasCanceled() const;

    int minimum() const;
    int maximum() const;

    int value() const;

    QSize sizeHint() const override;

    QString labelText() const;
    int minimumDuration() const;

    void setAutoReset(bool reset);
    bool autoReset() const;
    void setAutoClose(bool close);
    bool autoClose() const;

    using QDialog::open;
    void open(QObject *receiver, const char *member);

public Q_SLOTS:
    void cancel();
    void reset();
    void setMaximum(int maximum);
    void setMinimum(int minimum);
    void setRange(int minimum, int maximum);
    void setValue(int progress);
    void setLabelText(const QString &text);
    void setCancelButtonText(const QString &text);
    void setMinimumDuration(int ms);

Q_SIGNALS:
    void canceled();

protected:
    void resizeEvent(QResizeEvent *event) override;
    void closeEvent(QCloseEvent *event) override;
    void changeEvent(QEvent *event) override;
    void showEvent(QShowEvent *event) override;

protected Q_SLOTS:
    void forceShow();

private:
    Q_DISABLE_COPY(QProgressDialog)

    Q_PRIVATE_SLOT(d_func(), void _q_disconnectOnClose())
};

QT_END_NAMESPACE

#endif // QPROGRESSDIALOG_H
