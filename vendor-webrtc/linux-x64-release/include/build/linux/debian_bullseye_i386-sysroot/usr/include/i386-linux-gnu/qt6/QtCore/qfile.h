// Copyright (C) 2020 The Qt Company Ltd.
// Copyright (C) 2016 Intel Corporation.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only OR GPL-2.0-only OR GPL-3.0-only

#ifndef QFILE_H
#define QFILE_H

#include <QtCore/qfiledevice.h>
#include <QtCore/qstring.h>
#include <stdio.h>

#if QT_CONFIG(cxx17_filesystem)
#include <filesystem>
#elif defined(Q_CLANG_QDOC)
namespace std {
    namespace filesystem {
        class path {
        };
    };
};
#endif

#ifdef open
#error qfile.h must be included before any header file that defines open
#endif

QT_BEGIN_NAMESPACE

#if QT_CONFIG(cxx17_filesystem)
namespace QtPrivate {
inline QString fromFilesystemPath(const std::filesystem::path &path)
{
#ifdef Q_OS_WIN
    return QString::fromStdWString(path.native());
#else
    return QString::fromStdString(path.native());
#endif
}

inline std::filesystem::path toFilesystemPath(const QString &path)
{
    return std::filesystem::path(reinterpret_cast<const char16_t *>(path.cbegin()),
                                 reinterpret_cast<const char16_t *>(path.cend()));
}

// Both std::filesystem::path and QString (without QT_NO_CAST_FROM_ASCII) can be implicitly
// constructed from string literals so we force the std::fs::path parameter to only
// accept std::fs::path with no implicit conversions.
template<typename T>
using ForceFilesystemPath = typename std::enable_if_t<std::is_same_v<std::filesystem::path, T>, int>;
}
#endif // QT_CONFIG(cxx17_filesystem)

class QTemporaryFile;
class QFilePrivate;

class Q_CORE_EXPORT QFile : public QFileDevice
{
#ifndef QT_NO_QOBJECT
    Q_OBJECT
#endif
    Q_DECLARE_PRIVATE(QFile)

public:
    QFile();
    QFile(const QString &name);
#ifdef Q_CLANG_QDOC
    QFile(const std::filesystem::path &name);
#elif QT_CONFIG(cxx17_filesystem)
    template<typename T, QtPrivate::ForceFilesystemPath<T> = 0>
    QFile(const T &name) : QFile(QtPrivate::fromFilesystemPath(name))
    {
    }
#endif // QT_CONFIG(cxx17_filesystem)

#ifndef QT_NO_QOBJECT
    explicit QFile(QObject *parent);
    QFile(const QString &name, QObject *parent);

#ifdef Q_CLANG_QDOC
    QFile(const std::filesystem::path &path, QObject *parent);
#elif QT_CONFIG(cxx17_filesystem)
    template<typename T, QtPrivate::ForceFilesystemPath<T> = 0>
    QFile(const T &path, QObject *parent) : QFile(QtPrivate::fromFilesystemPath(path), parent)
    {
    }
#endif // QT_CONFIG(cxx17_filesystem)
#endif // !QT_NO_QOBJECT
    ~QFile();

    QString fileName() const override;
#if QT_CONFIG(cxx17_filesystem) || defined(Q_CLANG_QDOC)
    std::filesystem::path filesystemFileName() const
    { return QtPrivate::toFilesystemPath(fileName()); }
#endif
    void setFileName(const QString &name);
#ifdef Q_CLANG_QDOC
    void setFileName(const std::filesystem::path &name);
#elif QT_CONFIG(cxx17_filesystem)
    template<typename T, QtPrivate::ForceFilesystemPath<T> = 0>
    void setFileName(const T &name)
    {
        setFileName(QtPrivate::fromFilesystemPath(name));
    }
#endif // QT_CONFIG(cxx17_filesystem)

#if defined(Q_OS_DARWIN)
    // Mac always expects filenames in UTF-8... and decomposed...
    static inline QByteArray encodeName(const QString &fileName)
    {
        return fileName.normalized(QString::NormalizationForm_D).toUtf8();
    }
    static QString decodeName(const QByteArray &localFileName)
    {
        // note: duplicated in qglobal.cpp (qEnvironmentVariable)
        return QString::fromUtf8(localFileName).normalized(QString::NormalizationForm_C);
    }
    static inline QString decodeName(const char *localFileName)
    {
        return QString::fromUtf8(localFileName).normalized(QString::NormalizationForm_C);
    }
#else
    static inline QByteArray encodeName(const QString &fileName)
    {
        return fileName.toLocal8Bit();
    }
    static QString decodeName(const QByteArray &localFileName)
    {
        return QString::fromLocal8Bit(localFileName);
    }
    static inline QString decodeName(const char *localFileName)
    {
        return QString::fromLocal8Bit(localFileName);
    }
#endif

    bool exists() const;
    static bool exists(const QString &fileName);
#ifdef Q_CLANG_QDOC
    static bool exists(const std::filesystem::path &fileName);
#elif QT_CONFIG(cxx17_filesystem)
    template<typename T, QtPrivate::ForceFilesystemPath<T> = 0>
    static bool exists(const T &fileName)
    {
        return exists(QtPrivate::fromFilesystemPath(fileName));
    }
#endif // QT_CONFIG(cxx17_filesystem)

    QString symLinkTarget() const;
    static QString symLinkTarget(const QString &fileName);
#ifdef Q_CLANG_QDOC
    std::filesystem::path filesystemSymLinkTarget() const;
    static std::filesystem::path filesystemSymLinkTarget(const std::filesystem::path &fileName);
#elif QT_CONFIG(cxx17_filesystem)
    std::filesystem::path filesystemSymLinkTarget() const
    {
        return QtPrivate::toFilesystemPath(symLinkTarget());
    }
    template<typename T, QtPrivate::ForceFilesystemPath<T> = 0>
    static std::filesystem::path filesystemSymLinkTarget(const T &fileName)
    {
        return QtPrivate::toFilesystemPath(symLinkTarget(QtPrivate::fromFilesystemPath(fileName)));
    }
#endif // QT_CONFIG(cxx17_filesystem)

    bool remove();
    static bool remove(const QString &fileName);
#ifdef Q_CLANG_QDOC
    static bool remove(const std::filesystem::path &fileName);
#elif QT_CONFIG(cxx17_filesystem)
    template<typename T, QtPrivate::ForceFilesystemPath<T> = 0>
    static bool remove(const T &fileName)
    {
        return remove(QtPrivate::fromFilesystemPath(fileName));
    }
#endif // QT_CONFIG(cxx17_filesystem)

    bool moveToTrash();
    static bool moveToTrash(const QString &fileName, QString *pathInTrash = nullptr);
#ifdef Q_CLANG_QDOC
    static bool moveToTrash(const std::filesystem::path &fileName, QString *pathInTrash = nullptr);
#elif QT_CONFIG(cxx17_filesystem)
    template<typename T, QtPrivate::ForceFilesystemPath<T> = 0>
    static bool moveToTrash(const T &fileName, QString *pathInTrash = nullptr)
    {
        return moveToTrash(QtPrivate::fromFilesystemPath(fileName), pathInTrash);
    }
#endif // QT_CONFIG(cxx17_filesystem)

    bool rename(const QString &newName);
    static bool rename(const QString &oldName, const QString &newName);
#ifdef Q_CLANG_QDOC
    bool rename(const std::filesystem::path &newName);
    static bool rename(const std::filesystem::path &oldName,
                       const std::filesystem::path &newName);
#elif QT_CONFIG(cxx17_filesystem)
    template<typename T, QtPrivate::ForceFilesystemPath<T> = 0>
    bool rename(const T &newName)
    {
        return rename(QtPrivate::fromFilesystemPath(newName));
    }
    template<typename T, QtPrivate::ForceFilesystemPath<T> = 0>
    static bool rename(const T &oldName, const T &newName)
    {
        return rename(QtPrivate::fromFilesystemPath(oldName),
                      QtPrivate::fromFilesystemPath(newName));
    }
#endif // QT_CONFIG(cxx17_filesystem)

    bool link(const QString &newName);
    static bool link(const QString &fileName, const QString &newName);
#ifdef Q_CLANG_QDOC
    bool link(const std::filesystem::path &newName);
    static bool link(const std::filesystem::path &fileName,
                     const std::filesystem::path &newName);
#elif QT_CONFIG(cxx17_filesystem)
    template<typename T, QtPrivate::ForceFilesystemPath<T> = 0>
    bool link(const T &newName)
    {
        return link(QtPrivate::fromFilesystemPath(newName));
    }
    template<typename T, QtPrivate::ForceFilesystemPath<T> = 0>
    static bool link(const T &fileName, const T &newName)
    {
        return link(QtPrivate::fromFilesystemPath(fileName),
                    QtPrivate::fromFilesystemPath(newName));
    }
#endif // QT_CONFIG(cxx17_filesystem)

    bool copy(const QString &newName);
    static bool copy(const QString &fileName, const QString &newName);
#ifdef Q_CLANG_QDOC
    bool copy(const std::filesystem::path &newName);
    static bool copy(const std::filesystem::path &fileName,
                     const std::filesystem::path &newName);
#elif QT_CONFIG(cxx17_filesystem)
    template<typename T, QtPrivate::ForceFilesystemPath<T> = 0>
    bool copy(const T &newName)
    {
        return copy(QtPrivate::fromFilesystemPath(newName));
    }
    template<typename T, QtPrivate::ForceFilesystemPath<T> = 0>
    static bool copy(const T &fileName, const T &newName)
    {
        return copy(QtPrivate::fromFilesystemPath(fileName),
                    QtPrivate::fromFilesystemPath(newName));
    }
#endif // QT_CONFIG(cxx17_filesystem)

    bool open(OpenMode flags) override;
    bool open(OpenMode flags, Permissions permissions);
    bool open(FILE *f, OpenMode ioFlags, FileHandleFlags handleFlags=DontCloseHandle);
    bool open(int fd, OpenMode ioFlags, FileHandleFlags handleFlags=DontCloseHandle);

    qint64 size() const override;

    bool resize(qint64 sz) override;
    static bool resize(const QString &filename, qint64 sz);

    Permissions permissions() const override;
    static Permissions permissions(const QString &filename);
    bool setPermissions(Permissions permissionSpec) override;
    static bool setPermissions(const QString &filename, Permissions permissionSpec);
#ifdef Q_CLANG_QDOC
    static Permissions permissions(const std::filesystem::path &filename);
    static bool setPermissions(const std::filesystem::path &filename, Permissions permissionSpec);
#elif QT_CONFIG(cxx17_filesystem)
    template<typename T,  QtPrivate::ForceFilesystemPath<T> = 0>
    static Permissions permissions(const T &filename)
    {
        return permissions(QtPrivate::fromFilesystemPath(filename));
    }
    template<typename T, QtPrivate::ForceFilesystemPath<T> = 0>
    static bool setPermissions(const T &filename, Permissions permissionSpec)
    {
        return setPermissions(QtPrivate::fromFilesystemPath(filename), permissionSpec);
    }
#endif // QT_CONFIG(cxx17_filesystem)

protected:
#ifdef QT_NO_QOBJECT
    QFile(QFilePrivate &dd);
#else
    QFile(QFilePrivate &dd, QObject *parent = nullptr);
#endif

private:
    friend class QTemporaryFile;
    Q_DISABLE_COPY(QFile)
};

QT_END_NAMESPACE

#endif // QFILE_H
