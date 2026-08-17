// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QOPENGLWIDGET_H
#define QOPENGLWIDGET_H

#include <QtOpenGLWidgets/qtopenglwidgetsglobal.h>

#include <QtWidgets/QWidget>
#include <QtGui/QSurfaceFormat>
#include <QtGui/qopengl.h>

QT_BEGIN_NAMESPACE

class QOpenGLWidgetPrivate;

class Q_OPENGLWIDGETS_EXPORT QOpenGLWidget : public QWidget
{
    Q_OBJECT
    Q_DECLARE_PRIVATE(QOpenGLWidget)

public:
    enum UpdateBehavior {
        NoPartialUpdate,
        PartialUpdate
    };

    explicit QOpenGLWidget(QWidget* parent = nullptr, Qt::WindowFlags f = Qt::WindowFlags());
    ~QOpenGLWidget();

    void setUpdateBehavior(UpdateBehavior updateBehavior);
    UpdateBehavior updateBehavior() const;

    void setFormat(const QSurfaceFormat &format);
    QSurfaceFormat format() const;

    GLenum textureFormat() const;
    void setTextureFormat(GLenum texFormat);

    bool isValid() const;

    void makeCurrent();
    void doneCurrent();

    QOpenGLContext *context() const;
    GLuint defaultFramebufferObject() const;

    QImage grabFramebuffer();

Q_SIGNALS:
    void aboutToCompose();
    void frameSwapped();
    void aboutToResize();
    void resized();

protected:
    virtual void initializeGL();
    virtual void resizeGL(int w, int h);
    virtual void paintGL();

    void paintEvent(QPaintEvent *e) override;
    void resizeEvent(QResizeEvent *e) override;
    bool event(QEvent *e) override;

    int metric(QPaintDevice::PaintDeviceMetric metric) const override;
    QPaintDevice *redirected(QPoint *p) const override;
    QPaintEngine *paintEngine() const override;

private:
    Q_DISABLE_COPY(QOpenGLWidget)
};

QT_END_NAMESPACE

#endif // QOPENGLWIDGET_H
