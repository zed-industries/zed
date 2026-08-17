// Copyright (C) 2016 The Qt Company Ltd.
// Copyright (C) 2013 Ivan Komissarov.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QKEYSEQUENCEEDIT_H
#define QKEYSEQUENCEEDIT_H

#include <QtWidgets/qtwidgetsglobal.h>
#include <QtWidgets/qwidget.h>

QT_REQUIRE_CONFIG(keysequenceedit);

QT_BEGIN_NAMESPACE

class QKeySequenceEditPrivate;
class Q_WIDGETS_EXPORT QKeySequenceEdit : public QWidget
{
    Q_OBJECT
    Q_PROPERTY(QKeySequence keySequence READ keySequence WRITE setKeySequence
               NOTIFY keySequenceChanged USER true)
    Q_PROPERTY(bool clearButtonEnabled READ isClearButtonEnabled WRITE setClearButtonEnabled)

public:
    explicit QKeySequenceEdit(QWidget *parent = nullptr);
    explicit QKeySequenceEdit(const QKeySequence &keySequence, QWidget *parent = nullptr);
    ~QKeySequenceEdit();

    QKeySequence keySequence() const;

    void setClearButtonEnabled(bool enable);
    bool isClearButtonEnabled() const;

public Q_SLOTS:
    void setKeySequence(const QKeySequence &keySequence);
    void clear();

Q_SIGNALS:
    void editingFinished();
    void keySequenceChanged(const QKeySequence &keySequence);

protected:
    QKeySequenceEdit(QKeySequenceEditPrivate &d, QWidget *parent, Qt::WindowFlags f);

    bool event(QEvent *) override;
    void keyPressEvent(QKeyEvent *) override;
    void keyReleaseEvent(QKeyEvent *) override;
    void timerEvent(QTimerEvent *) override;
    void focusOutEvent(QFocusEvent *) override;

private:
    Q_DISABLE_COPY(QKeySequenceEdit)
    Q_DECLARE_PRIVATE(QKeySequenceEdit)
};

QT_END_NAMESPACE

#endif // QKEYSEQUENCEEDIT_H
