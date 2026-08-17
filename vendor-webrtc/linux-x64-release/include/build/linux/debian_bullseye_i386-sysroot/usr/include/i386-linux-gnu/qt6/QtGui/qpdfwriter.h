// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

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
