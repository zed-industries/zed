/****************************************************************************
**
** Copyright (C) 2016 The Qt Company Ltd.
** Contact: https://www.qt.io/licensing/
**
** This file is part of the QtGui module of the Qt Toolkit.
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

#ifndef QPDFWRITER_H
#define QPDFWRITER_H

#include <QtGui/qtguiglobal.h>

#ifndef QT_NO_PDF

#include <QtCore/qobject.h>
#include <QtGui/qpagedpaintdevice.h>
#include <QtGui/qpagelayout.h>

QT_BEGIN_NAMESPACE

class QIODevice;
class QPdfWriterPrivate;

class Q_GUI_EXPORT QPdfWriter : public QObject, public QPagedPaintDevice
{
    Q_OBJECT
public:
    explicit QPdfWriter(const QString &filename);
    explicit QPdfWriter(QIODevice *device);
    ~QPdfWriter();

    void setPdfVersion(PdfVersion version);
    PdfVersion pdfVersion() const;

    QString title() const;
    void setTitle(const QString &title);

    QString creator() const;
    void setCreator(const QString &creator);

    bool newPage() override;

    void setResolution(int resolution);
    int resolution() const;

    void setDocumentXmpMetadata(const QByteArray &xmpMetadata);
    QByteArray documentXmpMetadata() const;

    void addFileAttachment(const QString &fileName, const QByteArray &data, const QString &mimeType = QString());

#ifdef Q_QDOC
    bool setPageLayout(const QPageLayout &pageLayout);
    bool setPageSize(const QPageSize &pageSize);
    bool setPageOrientation(QPageLayout::Orientation orientation);
    bool setPageMargins(const QMarginsF &margins);
    bool setPageMargins(const QMarginsF &margins, QPageLayout::Unit units);
    QPageLayout pageLayout() const;
#else
    using QPagedPaintDevice::setPageSize;
#endif

#if QT_DEPRECATED_SINCE(5, 14)
    QT_DEPRECATED_X("Use setPageSize(QPageSize(id)) instead")
    void setPageSize(PageSize size) override;
    QT_DEPRECATED_X("Use setPageSize(QPageSize(size, QPageSize::Millimeter)) instead")
    void setPageSizeMM(const QSizeF &size) override;
    QT_DEPRECATED_X("Use setPageMargins(QMarginsF(l, t, r, b), QPageLayout::Millimeter) instead")
    void setMargins(const Margins &m) override;
#endif

protected:
    QPaintEngine *paintEngine() const override;
    int metric(PaintDeviceMetric id) const override;

private:
    Q_DISABLE_COPY(QPdfWriter)
    Q_DECLARE_PRIVATE(QPdfWriter)
};

QT_END_NAMESPACE

#endif // QT_NO_PDF

#endif
