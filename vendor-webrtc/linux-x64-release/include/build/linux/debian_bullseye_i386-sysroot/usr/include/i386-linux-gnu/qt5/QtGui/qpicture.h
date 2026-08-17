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

#ifndef QPICTURE_H
#define QPICTURE_H

#include <QtGui/qtguiglobal.h>
#include <QtCore/qiodevice.h>
#include <QtCore/qstringlist.h>
#include <QtCore/qsharedpointer.h>
#include <QtGui/qpaintdevice.h>

QT_BEGIN_NAMESPACE


#ifndef QT_NO_PICTURE

class QPicturePrivate;
class Q_GUI_EXPORT QPicture : public QPaintDevice
{
    Q_DECLARE_PRIVATE(QPicture)
public:
    explicit QPicture(int formatVersion = -1);
    QPicture(const QPicture &);
    ~QPicture();

    bool isNull() const;

    int devType() const override;
    uint size() const;
    const char* data() const;
    virtual void setData(const char* data, uint size);

    bool play(QPainter *p);

    bool load(QIODevice *dev, const char *format = nullptr);
    bool load(const QString &fileName, const char *format = nullptr);
    bool save(QIODevice *dev, const char *format = nullptr);
    bool save(const QString &fileName, const char *format = nullptr);

    QRect boundingRect() const;
    void setBoundingRect(const QRect &r);

    QPicture& operator=(const QPicture &p);
    inline QPicture &operator=(QPicture &&other) noexcept
    { qSwap(d_ptr, other.d_ptr); return *this; }
    inline void swap(QPicture &other) noexcept
    { d_ptr.swap(other.d_ptr); }
    void detach();
    bool isDetached() const;

    friend Q_GUI_EXPORT QDataStream &operator<<(QDataStream &in, const QPicture &p);
    friend Q_GUI_EXPORT QDataStream &operator>>(QDataStream &in, QPicture &p);

#if QT_DEPRECATED_SINCE(5, 10)
    static QT_DEPRECATED const char* pictureFormat(const QString &fileName);
    static QT_DEPRECATED QList<QByteArray> inputFormats();
    static QT_DEPRECATED QList<QByteArray> outputFormats();
    static QT_DEPRECATED QStringList inputFormatList();
    static QT_DEPRECATED QStringList outputFormatList();
#endif // QT_DEPRECATED_SINCE(5, 10)

    QPaintEngine *paintEngine() const override;

protected:
    QPicture(QPicturePrivate &data);

    int metric(PaintDeviceMetric m) const override;

private:
    bool exec(QPainter *p, QDataStream &ds, int i);

    QExplicitlySharedDataPointer<QPicturePrivate> d_ptr;
    friend class QPicturePaintEngine;
    friend class QAlphaPaintEngine;
    friend class QPreviewPaintEngine;

public:
    typedef QExplicitlySharedDataPointer<QPicturePrivate> DataPtr;
    inline DataPtr &data_ptr() { return d_ptr; }
};

Q_DECLARE_SHARED(QPicture)


#ifndef QT_NO_PICTUREIO
class QIODevice;
class QPictureIO;
typedef void (*picture_io_handler)(QPictureIO *); // picture IO handler

struct QPictureIOData;

class Q_GUI_EXPORT QPictureIO
{
public:
    QPictureIO();
    QPictureIO(QIODevice *ioDevice, const char *format);
    QPictureIO(const QString &fileName, const char *format);
    ~QPictureIO();

    const QPicture &picture() const;
    int status() const;
    const char *format() const;
    QIODevice *ioDevice() const;
    QString fileName() const;
    int quality() const;
    QString description() const;
    const char *parameters() const;
    float gamma() const;

    void setPicture(const QPicture &);
    void setStatus(int);
    void setFormat(const char *);
    void setIODevice(QIODevice *);
    void setFileName(const QString &);
    void setQuality(int);
    void setDescription(const QString &);
    void setParameters(const char *);
    void setGamma(float);

    bool read();
    bool write();

    static QByteArray pictureFormat(const QString &fileName);
    static QByteArray pictureFormat(QIODevice *);
    static QList<QByteArray> inputFormats();
    static QList<QByteArray> outputFormats();

    static void defineIOHandler(const char *format,
                                const char *header,
                                const char *flags,
                                picture_io_handler read_picture,
                                picture_io_handler write_picture);

private:
    Q_DISABLE_COPY(QPictureIO)

    void init();

    QPictureIOData *d;
};

#endif //QT_NO_PICTUREIO


/*****************************************************************************
  QPicture stream functions
 *****************************************************************************/

#ifndef QT_NO_DATASTREAM
Q_GUI_EXPORT QDataStream &operator<<(QDataStream &, const QPicture &);
Q_GUI_EXPORT QDataStream &operator>>(QDataStream &, QPicture &);
#endif

#endif // QT_NO_PICTURE

QT_END_NAMESPACE

#endif // QPICTURE_H
