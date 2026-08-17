/****************************************************************************
**
** Copyright (C) 2016 The Qt Company Ltd.
** Contact: https://www.qt.io/licensing/
**
** This file is part of the QtWidgets module of the Qt Toolkit.
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

#ifndef QABSTRACTSPINBOX_H
#define QABSTRACTSPINBOX_H

#include <QtWidgets/qtwidgetsglobal.h>
#include <QtWidgets/qwidget.h>
#include <QtGui/qvalidator.h>

QT_REQUIRE_CONFIG(spinbox);

QT_BEGIN_NAMESPACE

class QLineEdit;

class QAbstractSpinBoxPrivate;
class QStyleOptionSpinBox;

class Q_WIDGETS_EXPORT QAbstractSpinBox : public QWidget
{
    Q_OBJECT

    Q_PROPERTY(bool wrapping READ wrapping WRITE setWrapping)
    Q_PROPERTY(bool frame READ hasFrame WRITE setFrame)
    Q_PROPERTY(Qt::Alignment alignment READ alignment WRITE setAlignment)
    Q_PROPERTY(bool readOnly READ isReadOnly WRITE setReadOnly)
    Q_PROPERTY(ButtonSymbols buttonSymbols READ buttonSymbols WRITE setButtonSymbols)
    Q_PROPERTY(QString specialValueText READ specialValueText WRITE setSpecialValueText)
    Q_PROPERTY(QString text READ text)
    Q_PROPERTY(bool accelerated READ isAccelerated WRITE setAccelerated)
    Q_PROPERTY(CorrectionMode correctionMode READ correctionMode WRITE setCorrectionMode)
    Q_PROPERTY(bool acceptableInput READ hasAcceptableInput)
    Q_PROPERTY(bool keyboardTracking READ keyboardTracking WRITE setKeyboardTracking)
    Q_PROPERTY(bool showGroupSeparator READ isGroupSeparatorShown WRITE setGroupSeparatorShown)
public:
    explicit QAbstractSpinBox(QWidget *parent = nullptr);
    ~QAbstractSpinBox();

    enum StepEnabledFlag { StepNone = 0x00, StepUpEnabled = 0x01,
                           StepDownEnabled = 0x02 };
    Q_DECLARE_FLAGS(StepEnabled, StepEnabledFlag)

    enum ButtonSymbols { UpDownArrows, PlusMinus, NoButtons };
    Q_ENUM(ButtonSymbols)

    ButtonSymbols buttonSymbols() const;
    void setButtonSymbols(ButtonSymbols bs);

    enum CorrectionMode  { CorrectToPreviousValue, CorrectToNearestValue };
    Q_ENUM(CorrectionMode)

    void setCorrectionMode(CorrectionMode cm);
    CorrectionMode correctionMode() const;

    bool hasAcceptableInput() const;
    QString text() const;

    QString specialValueText() const;
    void setSpecialValueText(const QString &txt);

    bool wrapping() const;
    void setWrapping(bool w);

    void setReadOnly(bool r);
    bool isReadOnly() const;

    void setKeyboardTracking(bool kt);
    bool keyboardTracking() const;

    void setAlignment(Qt::Alignment flag);
    Qt::Alignment alignment() const;

    void setFrame(bool);
    bool hasFrame() const;

    void setAccelerated(bool on);
    bool isAccelerated() const;

    void setGroupSeparatorShown(bool shown);
    bool isGroupSeparatorShown() const;

    QSize sizeHint() const override;
    QSize minimumSizeHint() const override;
    void interpretText();
    bool event(QEvent *event) override;

    QVariant inputMethodQuery(Qt::InputMethodQuery) const override;

    virtual QValidator::State validate(QString &input, int &pos) const;
    virtual void fixup(QString &input) const;

    virtual void stepBy(int steps);

    enum StepType {
        DefaultStepType,
        AdaptiveDecimalStepType
    };
    Q_ENUM(StepType)

public Q_SLOTS:
    void stepUp();
    void stepDown();
    void selectAll();
    virtual void clear();
protected:
    void resizeEvent(QResizeEvent *event) override;
    void keyPressEvent(QKeyEvent *event) override;
    void keyReleaseEvent(QKeyEvent *event) override;
#if QT_CONFIG(wheelevent)
    void wheelEvent(QWheelEvent *event) override;
#endif
    void focusInEvent(QFocusEvent *event) override;
    void focusOutEvent(QFocusEvent *event) override;
#if QT_CONFIG(contextmenu)
    void contextMenuEvent(QContextMenuEvent *event) override;
#endif
    void changeEvent(QEvent *event) override;
    void closeEvent(QCloseEvent *event) override;
    void hideEvent(QHideEvent *event) override;
    void mousePressEvent(QMouseEvent *event) override;
    void mouseReleaseEvent(QMouseEvent *event) override;
    void mouseMoveEvent(QMouseEvent *event) override;
    void timerEvent(QTimerEvent *event) override;
    void paintEvent(QPaintEvent *event) override;
    void showEvent(QShowEvent *event) override;
    void initStyleOption(QStyleOptionSpinBox *option) const;

    QLineEdit *lineEdit() const;
    void setLineEdit(QLineEdit *edit);

    virtual StepEnabled stepEnabled() const;
Q_SIGNALS:
    void editingFinished();
protected:
    QAbstractSpinBox(QAbstractSpinBoxPrivate &dd, QWidget *parent = nullptr);

private:
    Q_PRIVATE_SLOT(d_func(), void _q_editorTextChanged(const QString &))
    Q_PRIVATE_SLOT(d_func(), void _q_editorCursorPositionChanged(int, int))

    Q_DECLARE_PRIVATE(QAbstractSpinBox)
    Q_DISABLE_COPY(QAbstractSpinBox)
    friend class QAccessibleAbstractSpinBox;
};
Q_DECLARE_OPERATORS_FOR_FLAGS(QAbstractSpinBox::StepEnabled)

QT_END_NAMESPACE

#endif // QABSTRACTSPINBOX_H
