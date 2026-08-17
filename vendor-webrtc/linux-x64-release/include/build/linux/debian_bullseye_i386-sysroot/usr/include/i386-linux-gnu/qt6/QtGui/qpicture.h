// Copyright (C) 2016 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

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

    bool load(QIODevice *dev);
    bool load(const QString &fileName);
    bool save(QIODevice *dev);
    bool save(const QString &fileName);

    QRect boundingRect() const;
    void setBoundingRect(const QRect &r);

    QPicture& operator=(const QPicture &p);
    QT_MOVE_ASSIGNMENT_OPERATOR_IMPL_VIA_PURE_SWAP(QPicture)
    inline void swap(QPicture &other) noexcept
    { d_ptr.swap(other.d_ptr); }
    void detach();
    bool isDetached() const;

    friend Q_GUI_EXPORT QDataStream &operator<<(QDataStream &in, const QPicture &p);
    friend Q_GUI_EXPORT QDataStream &operator>>(QDataStream &in, QPicture &p);

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
