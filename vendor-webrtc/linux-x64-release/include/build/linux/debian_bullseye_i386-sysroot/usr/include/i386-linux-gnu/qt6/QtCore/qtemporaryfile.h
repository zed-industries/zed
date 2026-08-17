// Copyright (C) 2020 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QTEMPORARYFILE_H
#define QTEMPORARYFILE_H

#include <QtCore/qiodevice.h>
#include <QtCore/qfile.h>

#ifdef open
#error qtemporaryfile.h must be included before any header file that defines open
#endif

QT_BEGIN_NAMESPACE


#ifndef QT_NO_TEMPORARYFILE

class QTemporaryFilePrivate;
class QLockFilePrivate;

class Q_CORE_EXPORT QTemporaryFile : public QFile
{
#ifndef QT_NO_QOBJECT
    Q_OBJECT
#endif
    Q_DECLARE_PRIVATE(QTemporaryFile)

public:
    QTemporaryFile();
    explicit QTemporaryFile(const QString &templateName);
#ifndef QT_NO_QOBJECT
    explicit QTemporaryFile(QObject *parent);
    QTemporaryFile(const QString &templateName, QObject *parent);
#endif
    ~QTemporaryFile();

    bool autoRemove() const;
    void setAutoRemove(bool b);

    // ### Hides open(flags)
    bool open() { return open(QIODevice::ReadWrite); }

    QString fileName() const override;
    QString fileTemplate() const;
    void setFileTemplate(const QString &name);

    // Hides QFile::rename
    bool rename(const QString &newName);

    inline static QTemporaryFile *createNativeFile(const QString &fileName)
        { QFile file(fileName); return createNativeFile(file); }
    static QTemporaryFile *createNativeFile(QFile &file);

protected:
    bool open(OpenMode flags) override;

private:
    friend class QFile;
    friend class QLockFilePrivate;
    Q_DISABLE_COPY(QTemporaryFile)
};

#endif // QT_NO_TEMPORARYFILE

QT_END_NAMESPACE

#endif // QTEMPORARYFILE_H
