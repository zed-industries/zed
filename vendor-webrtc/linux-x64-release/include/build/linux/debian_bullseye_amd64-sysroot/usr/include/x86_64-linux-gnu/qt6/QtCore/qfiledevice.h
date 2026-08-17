// Copyright (C) 2020 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QFILEDEVICE_H
#define QFILEDEVICE_H

#include <QtCore/qiodevice.h>
#include <QtCore/qstring.h>

QT_BEGIN_NAMESPACE

class QDateTime;
class QFileDevicePrivate;

class Q_CORE_EXPORT QFileDevice : public QIODevice
{
#ifndef QT_NO_QOBJECT
    Q_OBJECT
#endif
    Q_DECLARE_PRIVATE(QFileDevice)

public:
    enum FileError {
        NoError = 0,
        ReadError = 1,
        WriteError = 2,
        FatalError = 3,
        ResourceError = 4,
        OpenError = 5,
        AbortError = 6,
        TimeOutError = 7,
        UnspecifiedError = 8,
        RemoveError = 9,
        RenameError = 10,
        PositionError = 11,
        ResizeError = 12,
        PermissionsError = 13,
        CopyError = 14
    };

    enum FileTime {
        FileAccessTime,
        FileBirthTime,
        FileMetadataChangeTime,
        FileModificationTime
    };

    enum Permission {
        ReadOwner = 0x4000, WriteOwner = 0x2000, ExeOwner = 0x1000,
        ReadUser  = 0x0400, WriteUser  = 0x0200, ExeUser  = 0x0100,
        ReadGroup = 0x0040, WriteGroup = 0x0020, ExeGroup = 0x0010,
        ReadOther = 0x0004, WriteOther = 0x0002, ExeOther = 0x0001
    };
    Q_DECLARE_FLAGS(Permissions, Permission)

    enum FileHandleFlag {
        AutoCloseHandle = 0x0001,
        DontCloseHandle = 0
    };
    Q_DECLARE_FLAGS(FileHandleFlags, FileHandleFlag)

    ~QFileDevice();

    FileError error() const;
    void unsetError();

    void close() override;

    bool isSequential() const override;

    int handle() const;
    virtual QString fileName() const;

    qint64 pos() const override;
    bool seek(qint64 offset) override;
    bool atEnd() const override;
    bool flush();

    qint64 size() const override;

    virtual bool resize(qint64 sz);
    virtual Permissions permissions() const;
    virtual bool setPermissions(Permissions permissionSpec);

    enum MemoryMapFlag {
        NoOptions = 0,
        MapPrivateOption = 0x0001
    };
    Q_DECLARE_FLAGS(MemoryMapFlags, MemoryMapFlag)

    uchar *map(qint64 offset, qint64 size, MemoryMapFlags flags = NoOptions);
    bool unmap(uchar *address);

    QDateTime fileTime(QFileDevice::FileTime time) const;
    bool setFileTime(const QDateTime &newDate, QFileDevice::FileTime fileTime);

protected:
    QFileDevice();
#ifdef QT_NO_QOBJECT
    QFileDevice(QFileDevicePrivate &dd);
#else
    explicit QFileDevice(QObject *parent);
    QFileDevice(QFileDevicePrivate &dd, QObject *parent = nullptr);
#endif

    qint64 readData(char *data, qint64 maxlen) override;
    qint64 writeData(const char *data, qint64 len) override;
    qint64 readLineData(char *data, qint64 maxlen) override;

private:
    Q_DISABLE_COPY(QFileDevice)
};

Q_DECLARE_OPERATORS_FOR_FLAGS(QFileDevice::Permissions)
Q_DECLARE_OPERATORS_FOR_FLAGS(QFileDevice::FileHandleFlags)
Q_DECLARE_OPERATORS_FOR_FLAGS(QFileDevice::MemoryMapFlags)

QT_END_NAMESPACE

#endif // QFILEDEVICE_H
