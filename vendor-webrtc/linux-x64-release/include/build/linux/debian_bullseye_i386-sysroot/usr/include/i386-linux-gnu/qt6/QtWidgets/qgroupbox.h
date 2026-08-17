// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QGROUPBOX_H
#define QGROUPBOX_H

#include <QtWidgets/qtwidgetsglobal.h>
#include <QtWidgets/qframe.h>

QT_REQUIRE_CONFIG(groupbox);

QT_BEGIN_NAMESPACE

class QGroupBoxPrivate;
class QStyleOptionGroupBox;
class Q_WIDGETS_EXPORT QGroupBox : public QWidget
{
    Q_OBJECT

    Q_PROPERTY(QString title READ title WRITE setTitle)
    Q_PROPERTY(Qt::Alignment alignment READ alignment WRITE setAlignment)
    Q_PROPERTY(bool flat READ isFlat WRITE setFlat)
    Q_PROPERTY(bool checkable READ isCheckable WRITE setCheckable)
    Q_PROPERTY(bool checked READ isChecked WRITE setChecked NOTIFY toggled USER true)
public:
    explicit QGroupBox(QWidget *parent = nullptr);
    explicit QGroupBox(const QString &title, QWidget *parent = nullptr);
    ~QGroupBox();

    QString title() const;
    void setTitle(const QString &title);

    Qt::Alignment alignment() const;
    void setAlignment(int alignment);

    QSize minimumSizeHint() const override;

    bool isFlat() const;
    void setFlat(bool flat);
    bool isCheckable() const;
    void setCheckable(bool checkable);
    bool isChecked() const;

public Q_SLOTS:
    void setChecked(bool checked);

Q_SIGNALS:
    void clicked(bool checked = false);
    void toggled(bool);

protected:
    bool event(QEvent *event) override;
    void childEvent(QChildEvent *event) override;
    void resizeEvent(QResizeEvent *event) override;
    void paintEvent(QPaintEvent *event) override;
    void focusInEvent(QFocusEvent *event) override;
    void changeEvent(QEvent *event) override;
    void mousePressEvent(QMouseEvent *event) override;
    void mouseMoveEvent(QMouseEvent *event) override;
    void mouseReleaseEvent(QMouseEvent *event) override;
    virtual void initStyleOption(QStyleOptionGroupBox *option) const;


private:
    Q_DISABLE_COPY(QGroupBox)
    Q_DECLARE_PRIVATE(QGroupBox)
    Q_PRIVATE_SLOT(d_func(), void _q_setChildrenEnabled(bool b))
};

QT_END_NAMESPACE

#endif // QGROUPBOX_H
