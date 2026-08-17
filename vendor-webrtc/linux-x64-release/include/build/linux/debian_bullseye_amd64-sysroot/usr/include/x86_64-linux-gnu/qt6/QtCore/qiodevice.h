// Copyright (C) 2020 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QIODEVICE_H
#define QIODEVICE_H

#include <QtCore/qglobal.h>
#include <QtCore/qiodevicebase.h>
#ifndef QT_NO_QOBJECT
#include <QtCore/qobject.h>
#else
#include <QtCore/qobjectdefs.h>
#include <QtCore/qscopedpointer.h>
#endif
#include <QtCore/qstring.h>

#ifdef open
#error qiodevice.h must be included before any header file that defines open
#endif

QT_BEGIN_NAMESPACE


class QByteArray;
class QIODevicePrivate;

class Q_CORE_EXPORT QIODevice
#ifndef QT_NO_QOBJECT
    : public QObject,
#else
    :
#endif
      public QIODeviceBase
{
#ifndef QT_NO_QOBJECT
    Q_OBJECT
#endif
public:
    QIODevice();
#ifndef QT_NO_QOBJECT
    explicit QIODevice(QObject *parent);
#endif
    virtual ~QIODevice();

    QIODeviceBase::OpenMode openMode() const;

    void setTextModeEnabled(bool enabled);
    bool isTextModeEnabled() const;

    bool isOpen() const;
    bool isReadable() const;
    bool isWritable() const;
    virtual bool isSequential() const;

    int readChannelCount() const;
    int writeChannelCount() const;
    int currentReadChannel() const;
    void setCurrentReadChannel(int channel);
    int currentWriteChannel() const;
    void setCurrentWriteChannel(int channel);

    virtual bool open(QIODeviceBase::OpenMode mode);
    virtual void close();

    // ### Qt 7 - QTBUG-76492: pos() and seek() should not be virtual, and
    // ### seek() should call a virtual seekData() function.
    virtual qint64 pos() const;
    virtual qint64 size() const;
    virtual bool seek(qint64 pos);
    virtual bool atEnd() const;
    virtual bool reset();

    virtual qint64 bytesAvailable() const;
    virtual qint64 bytesToWrite() const;

    qint64 read(char *data, qint64 maxlen);
    QByteArray read(qint64 maxlen);
    QByteArray readAll();
    qint64 readLine(char *data, qint64 maxlen);
    QByteArray readLine(qint64 maxlen = 0);
    virtual bool canReadLine() const;

    void startTransaction();
    void commitTransaction();
    void rollbackTransaction();
    bool isTransactionStarted() const;

    qint64 write(const char *data, qint64 len);
    qint64 write(const char *data);
    qint64 write(const QByteArray &data);

    qint64 peek(char *data, qint64 maxlen);
    QByteArray peek(qint64 maxlen);
    qint64 skip(qint64 maxSize);

    virtual bool waitForReadyRead(int msecs);
    virtual bool waitForBytesWritten(int msecs);

    void ungetChar(char c);
    bool putChar(char c);
    bool getChar(char *c);

    QString errorString() const;

#ifndef QT_NO_QOBJECT
Q_SIGNALS:
    void readyRead();
    void channelReadyRead(int channel);
    void bytesWritten(qint64 bytes);
    void channelBytesWritten(int channel, qint64 bytes);
    void aboutToClose();
    void readChannelFinished();
#endif

protected:
#ifdef QT_NO_QOBJECT
    QIODevice(QIODevicePrivate &dd);
#else
    QIODevice(QIODevicePrivate &dd, QObject *parent = nullptr);
#endif
    virtual qint64 readData(char *data, qint64 maxlen) = 0;
    virtual qint64 readLineData(char *data, qint64 maxlen);
    virtual qint64 skipData(qint64 maxSize);
    virtual qint64 writeData(const char *data, qint64 len) = 0;

    void setOpenMode(QIODeviceBase::OpenMode openMode);

    void setErrorString(const QString &errorString);

#ifdef QT_NO_QOBJECT
    QScopedPointer<QIODevicePrivate> d_ptr;
#endif

private:
    Q_DECLARE_PRIVATE(QIODevice)
    Q_DISABLE_COPY(QIODevice)
};

Q_DECLARE_OPERATORS_FOR_FLAGS(QIODevice::OpenMode)

#if !defined(QT_NO_DEBUG_STREAM)
class QDebug;
Q_CORE_EXPORT QDebug operator<<(QDebug debug, QIODevice::OpenMode modes);
#endif

QT_END_NAMESPACE

#endif // QIODEVICE_H
