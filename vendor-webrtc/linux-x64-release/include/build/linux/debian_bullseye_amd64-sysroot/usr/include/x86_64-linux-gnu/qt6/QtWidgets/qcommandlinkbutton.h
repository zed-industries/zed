// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QCOMMANDLINKBUTTON_H
#define QCOMMANDLINKBUTTON_H

#include <QtWidgets/qtwidgetsglobal.h>
#include <QtWidgets/qpushbutton.h>

QT_REQUIRE_CONFIG(commandlinkbutton);

QT_BEGIN_NAMESPACE


class QCommandLinkButtonPrivate;

class Q_WIDGETS_EXPORT QCommandLinkButton: public QPushButton
{
    Q_OBJECT

    Q_PROPERTY(QString description READ description WRITE setDescription)
    Q_PROPERTY(bool flat READ isFlat WRITE setFlat DESIGNABLE false)

public:
    explicit QCommandLinkButton(QWidget *parent = nullptr);
    explicit QCommandLinkButton(const QString &text, QWidget *parent = nullptr);
    explicit QCommandLinkButton(const QString &text, const QString &description, QWidget *parent = nullptr);
    ~QCommandLinkButton();

    QString description() const;
    void setDescription(const QString &description);

    QSize sizeHint() const override;
    int heightForWidth(int) const override;
    QSize minimumSizeHint() const override;
    void initStyleOption(QStyleOptionButton *option) const override;

protected:
    bool event(QEvent *e) override;
    void paintEvent(QPaintEvent *) override;

private:
    Q_DISABLE_COPY(QCommandLinkButton)
    Q_DECLARE_PRIVATE(QCommandLinkButton)
};

QT_END_NAMESPACE

#endif // QCOMMANDLINKBUTTON
